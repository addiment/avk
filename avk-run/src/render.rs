//! This module uses a lot of hardcoded silliness in order to work with AVK.

mod material;
mod mesh;
mod texture;

use crate::backend::AvkBackend;
use avk_types::prelude::{Image, Palette};
use avk_types::{u16_to_rgba, MAX_IMAGES, MAX_PALETTES, RESOLUTION_HEIGHT, RESOLUTION_WIDTH};
use gl::types::{GLchar, GLenum, GLfloat, GLint, GLsizei, GLuint, GLushort};
use log::error;
use std::ffi::{c_void, CStr};
use std::ptr::null;

use crate::render::material::Material;
use crate::render::mesh::Mesh;
use crate::render::texture::Texture;

const UNIT_MESH: [GLfloat; 8] = [
	0.0, 0.0,
	1.0, 0.0,
	1.0, 1.0,
	0.0, 1.0
];
const VIEWPORT_MESH: [GLfloat; 8] = [
	-1.0, -1.0,
	1.0, -1.0,
	1.0, 1.0,
	-1.0, 1.0
];
const SQUARE_MESH_ELEMENTS: [GLushort; 6] = [
	0, 1, 2,
	2, 3, 0
];

const QUAD_VERT_SOURCE: &str = concat!(include_str!("shaders/quad_vert.glsl"), "\0");
const QUAD_FRAG_SOURCE: &str = concat!(include_str!("shaders/quad_frag.glsl"), "\0");
const VIEW_VERT_SOURCE: &str = concat!(include_str!("shaders/view_vert.glsl"), "\0");
const VIEW_FRAG_SOURCE: &str = concat!(include_str!("shaders/view_frag.glsl"), "\0");

#[inline(always)]
pub fn gl_err_check() {
	unsafe {
		let err = gl::GetError();
		if err != gl::NO_ERROR {
			panic!("GL error: {err}")
		}
	}
}

#[derive(Clone)]
pub(crate) struct AvkRenderManager {
	textures: [Texture; MAX_IMAGES],
	unit_quad: Mesh,
	unit_prog: Material,
	viewport_quad: Mesh,
	viewport_prog: Material,
	fbo: GLuint,
	fbt: GLuint,
}

/// Called by OpenGL whenever an error occurs.
/// Prints a message to stderr containing some brief information regarding the error.
extern "system" fn opengl_error_hook(
	source: GLenum,
	gl_type: GLenum,
	_id: GLuint,
	_severity: GLenum,
	_length: GLsizei,
	message: *const GLchar,
	_user_param: *mut c_void,
) {
	unsafe {
		let cstr = CStr::from_ptr(message).to_str().unwrap().to_owned();
		error!(
			"!!! OpenGL error !!!\nsource: {source}, gl_type: {gl_type}\n\"{}\"",
			cstr
		);
	}
}

impl AvkRenderManager {
	/// Initializes the OpenGL state related to AVK.
	// TODO: generate a texture using the palettes so we can use two samplers instead of 8 vec4s
	pub fn init(
		images: &mut [Image; MAX_IMAGES],
		_palettes: &[Palette; MAX_PALETTES],
		loader: fn(&'static str) -> *const c_void,
	) -> Self {
		gl::load_with(loader);
		gl_err_check();

		unsafe {
			// TODO: only call this when we're willing to take the perf hit
			{
				// Setup OpenGL to use all the fancy debugging hooks it has
				gl::Enable(gl::DEBUG_OUTPUT);
				gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
				gl::DebugMessageCallback(Some(opengl_error_hook), null());
			}

			let mut rbo = 0;
			let mut fbo = 0;
			let mut fbt = 0;

			{
				gl::GenFramebuffers(1, &mut fbo);
				gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
				gl::GenRenderbuffers(1, &mut rbo);
				gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);

				gl::GenTextures(1, &mut fbt);
				gl::BindTexture(gl::TEXTURE_2D, fbt);
				gl::TexImage2D(
					gl::TEXTURE_2D,
					0,
					gl::RGB as GLint,
					RESOLUTION_WIDTH as GLsizei,
					RESOLUTION_HEIGHT as GLsizei,
					0,
					gl::RGB,
					gl::UNSIGNED_BYTE,
					null(),
				);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
				// attach the texture to the framebuffer
				gl::FramebufferTexture2D(
					gl::FRAMEBUFFER,
					gl::COLOR_ATTACHMENT0,
					gl::TEXTURE_2D,
					fbt,
					0,
				);
				gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
				gl::RenderbufferStorage(
					gl::RENDERBUFFER,
					gl::DEPTH24_STENCIL8,
					RESOLUTION_WIDTH as GLsizei,
					RESOLUTION_HEIGHT as GLsizei,
				);
				gl::FramebufferRenderbuffer(
					gl::FRAMEBUFFER,
					gl::DEPTH_STENCIL_ATTACHMENT,
					gl::RENDERBUFFER,
					rbo,
				);
				if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
					panic!("Framebuffer is not complete!");
				}
				gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
			}

			Self {
				fbo,
				fbt,
				// create ALL THE TEXTURES!!!
				textures: Texture::new_bulk(images),

				unit_quad: Mesh::new(4, &UNIT_MESH, &SQUARE_MESH_ELEMENTS),
				unit_prog: Material::new(QUAD_FRAG_SOURCE, QUAD_VERT_SOURCE),

				viewport_quad: Mesh::new(4, &VIEWPORT_MESH, &SQUARE_MESH_ELEMENTS),
				viewport_prog: Material::new(VIEW_FRAG_SOURCE, VIEW_VERT_SOURCE),
			}
		}
	}

	/// Updates the OpenGL rendering backend.
	pub fn update(&mut self, avk: *mut AvkBackend, window_width: u32, window_height: u32) {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);

			gl::Viewport(
				0,
				0,
				RESOLUTION_WIDTH as GLsizei,
				RESOLUTION_HEIGHT as GLsizei,
			);
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			gl::Enable(gl::BLEND);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

			// gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

			for sprite in (*(*avk).raw).foreground {
				// let image_id = sprite.image_id;
				let palette_id = sprite.get_palette_id();

				let palette = (*avk).palettes[palette_id as usize].0;

				self.unit_prog.bind();
				self.textures[sprite.image_id as usize].bind();
				self.unit_prog
					.set_uniform_vec2("pos", sprite.x as f32, sprite.y as f32);
				self.unit_prog.set_uniform_vec2(
					"flip",
					if sprite.get_flip_x() { -1.0 } else { 1.0 },
					if sprite.get_flip_y() { -1.0 } else { 1.0 },
				);

				// helper function, because the code for sending palettes to the GPU is pretty bad
				fn set_color(material: &mut Material, name: &str, arr: &[u8; 4]) {
					material.set_uniform_vec4(
						name,
						arr[0] as f32 / 15.0,
						arr[1] as f32 / 15.0,
						arr[2] as f32 / 15.0,
						arr[3] as f32 / 15.0,
					);
				}

				set_color(&mut self.unit_prog, "palette_0", &u16_to_rgba(palette[0]));
				set_color(&mut self.unit_prog, "palette_1", &u16_to_rgba(palette[1]));
				set_color(&mut self.unit_prog, "palette_2", &u16_to_rgba(palette[2]));
				set_color(&mut self.unit_prog, "palette_3", &u16_to_rgba(palette[3]));
				set_color(&mut self.unit_prog, "palette_4", &u16_to_rgba(palette[4]));
				set_color(&mut self.unit_prog, "palette_5", &u16_to_rgba(palette[5]));
				set_color(&mut self.unit_prog, "palette_6", &u16_to_rgba(palette[6]));
				set_color(&mut self.unit_prog, "palette_7", &u16_to_rgba(palette[7]));

				self.unit_quad.draw();
			}

			// draw to the actual window framebuffer
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

			gl::Viewport(0, 0, window_width as GLsizei, window_height as GLsizei);
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			self.viewport_prog.bind();
			gl::BindTexture(gl::TEXTURE_2D, self.fbt);
			self.viewport_quad.draw();

			gl::Finish();
			// gl::Flush();

			gl_err_check();
		}
	}
}
