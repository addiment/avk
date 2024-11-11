mod material;
mod mesh;
mod texture;

use std::ffi::{c_void, CStr};
use std::ptr::{null};
use gl::types::{GLchar, GLenum, GLfloat, GLint, GLsizei,GLuint, GLushort};
use avk_types::{u16_to_rgba, MAX_IMAGES, MAX_PALETTES, RESOLUTION_HEIGHT, RESOLUTION_WIDTH};
use crate::gk::material::Material;
use crate::gk::mesh::Mesh;
use crate::gk::texture::Texture;
use crate::prelude::*;

const UNIT_MESH: [GLfloat; 8] = [
	0.0, 0.0,
	1.0, 0.0,
	1.0, 1.0,
	0.0, 1.0,
];
const VIEWPORT_MESH: [GLfloat; 8] = [
	-1.0, -1.0,
	1.0, -1.0,
	1.0, 1.0,
	-1.0, 1.0,
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
pub fn err_check() {
	unsafe {
		let err = gl::GetError();
		if err != gl::NO_ERROR {
			panic!("GL error: {err}")
		}
	}
}

#[derive(Clone)]
pub(crate) struct GirlsKissing {
	textures: [Texture; MAX_IMAGES],
	unit_quad: Mesh,
	unit_prog: Material,
	viewport_quad: Mesh,
	viewport_prog: Material,
	fbo: GLuint,
	fbt: GLuint,
}

extern "system" fn girl_kisser_alert(
	source: GLenum,
	gl_type: GLenum,
	_id: GLuint,
	_severity: GLenum,
	_length: GLsizei,
	message: *const GLchar,
	_user_param: *mut c_void
) {
	unsafe {
		let cstr = CStr::from_ptr(message).to_str().unwrap().to_owned();
		eprintln!("!!! OPENGL ERROR !!!\nsource: {source}, gl_type: {gl_type}\n\"{}\"", cstr);
	}
}

impl GirlsKissing {
	// TODO: generate a texture using the palettes so we can use two samplers instead of 8 vec4s
	pub fn init(images: &mut [Image; MAX_IMAGES], _palettes: &[Palette; MAX_PALETTES], loader: fn(&'static str) -> *const c_void) -> Self {
		gl::load_with(loader);
		err_check();

		unsafe {
			gl::Enable(gl::DEBUG_OUTPUT);
			gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
			gl::DebugMessageCallback(Some(girl_kisser_alert), null());

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
					null()
				);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
				// attach the texture to the framebuffer
				gl::FramebufferTexture2D(
					gl::FRAMEBUFFER,
					gl::COLOR_ATTACHMENT0,
					gl::TEXTURE_2D,
					fbt,
					0
				);
				gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
				gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, RESOLUTION_WIDTH as GLsizei, RESOLUTION_HEIGHT as GLsizei); // use a single renderbuffer object for both a depth AND stencil buffer.
				gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo); // now actually attach it
				// now that we actually created the framebuffer and added all attachments we want to check if it is actually complete now
				if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
					panic!("Framebuffer is not complete!");
				}
				gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
			}

			Self {
				fbo,
				fbt,
				textures: Texture::new_bulk(images),

				unit_quad: Mesh::new(4, &UNIT_MESH, &SQUARE_MESH_ELEMENTS),
				unit_prog: Material::new(QUAD_FRAG_SOURCE, QUAD_VERT_SOURCE),

				viewport_quad: Mesh::new(4, &VIEWPORT_MESH, &SQUARE_MESH_ELEMENTS),
				viewport_prog: Material::new(VIEW_FRAG_SOURCE, VIEW_VERT_SOURCE),
			}
		}
	}

	pub fn update(&mut self, avk: *mut Avk, window_width: u32, window_height: u32,) {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);

			gl::Viewport(0, 0, RESOLUTION_WIDTH as GLsizei, RESOLUTION_HEIGHT as GLsizei);
			gl::ClearColor(0.933333333, 0.4, 0.133333333, 1.0);
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
				self.unit_prog.set_uniform_vec2("pos", sprite.x as f32, sprite.y as f32);
				self.unit_prog.set_uniform_vec2(
					"flip",
					if sprite.get_flip_x() { -1.0 } else { 1.0 },
					if sprite.get_flip_y() { -1.0 } else { 1.0 }
				);

				let c0 = u16_to_rgba(palette[0]);
				let c1 = u16_to_rgba(palette[1]);
				let c2 = u16_to_rgba(palette[2]);
				let c3 = u16_to_rgba(palette[3]);
				let c4 = u16_to_rgba(palette[4]);
				let c5 = u16_to_rgba(palette[5]);
				let c6 = u16_to_rgba(palette[6]);
				let c7 = u16_to_rgba(palette[7]);

				fn dry_peppers(material: &mut Material, name: &str, arr: &[u8; 4]) {
					material.set_uniform_vec4(
						name,
						arr[0] as f32 / 15.0, arr[1] as f32 / 15.0, arr[2] as f32 / 15.0, arr[3] as f32 / 15.0,
					);
				}

				dry_peppers(&mut self.unit_prog, "palette_0", &c0);
				dry_peppers(&mut self.unit_prog, "palette_1", &c1);
				dry_peppers(&mut self.unit_prog, "palette_2", &c2);
				dry_peppers(&mut self.unit_prog, "palette_3", &c3);
				dry_peppers(&mut self.unit_prog, "palette_4", &c4);
				dry_peppers(&mut self.unit_prog, "palette_5", &c5);
				dry_peppers(&mut self.unit_prog, "palette_6", &c6);
				dry_peppers(&mut self.unit_prog, "palette_7", &c7);

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

			err_check();
		}
	}
}