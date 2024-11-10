mod material;
mod mesh;
mod texture;

use std::ffi::{c_void, CStr};
use std::ptr::{null};
use gl::types::{GLchar, GLenum, GLfloat, GLint, GLsizei,GLuint, GLushort};
use avk_types::{MAX_IMAGES, MAX_PALETTES, RESOLUTION_HEIGHT, RESOLUTION_WIDTH};
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

const VIEW_FRAG_SOURCE: &str = concat!(include_str!("shaders/viewport_frag.glsl"), "\0");
const FRAG_SOURCE: &str = concat!(include_str!("shaders/frag.glsl"), "\0");
const VERT_SOURCE: &str = concat!(include_str!("shaders/vert.glsl"), "\0");

#[inline(always)]
fn err_check() {
	unsafe {
		let err = gl::GetError();
		if err != gl::NO_ERROR {
			panic!("GL error: {err}")
		}
	}
}

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
	pub fn init(images: &mut [Image; MAX_IMAGES], palettes: &[Palette; MAX_PALETTES], loader: fn(&'static str) -> *const c_void) -> Self {
		gl::load_with(loader);
		err_check();

		println!("{:#x?}", images[0].0);
		unsafe {
			gl::Enable(gl::DEBUG_OUTPUT);
			gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
			gl::DebugMessageCallback(Some(girl_kisser_alert), null());

			let mut fbo = 0;
			gl::GenFramebuffers(1, &mut fbo);
			gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
			// if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE {
			//
			// }
			let mut rbo = 0;
			gl::GenRenderbuffers(1, &mut rbo);
			gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
			// gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, 800, 600);
			// gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);

			let mut fbt = 0;
			gl::GenTextures(1, &mut fbt);
			gl::BindTexture(gl::TEXTURE_2D, fbt);
			// some magic?
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
			let mut rbo = 0;
			gl::GenRenderbuffers(1, &mut rbo);
			gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
			gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, RESOLUTION_WIDTH as GLsizei, RESOLUTION_HEIGHT as GLsizei); // use a single renderbuffer object for both a depth AND stencil buffer.
			gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo); // now actually attach it
			// now that we actually created the framebuffer and added all attachments we want to check if it is actually complete now
			if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
				panic!("Framebuffer is not complete!");
			}
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);


			Self {
				fbo,
				fbt,
				textures: Texture::new_bulk(images),
				// textures: [Texture { texture_handle: 0 }; MAX_IMAGES ],

				unit_quad: Mesh::new(4, &UNIT_MESH, &SQUARE_MESH_ELEMENTS),
				unit_prog: Material::new(FRAG_SOURCE, VERT_SOURCE),

				viewport_quad: Mesh::new(4, &VIEWPORT_MESH, &SQUARE_MESH_ELEMENTS),
				viewport_prog: Material::new(VIEW_FRAG_SOURCE, VERT_SOURCE),
			}
		}
	}

	pub fn update(&mut self, window_width: u32, window_height: u32,) {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);

			gl::Viewport(0, 0, RESOLUTION_WIDTH as GLsizei, RESOLUTION_HEIGHT as GLsizei);
			gl::ClearColor(0.933333333, 0.4, 0.133333333, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			// gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

			self.unit_prog.bind();
			self.textures[0].bind();
			self.unit_prog.set_uniform_vec2("pos", -1.0, -1.0);
			self.unit_prog.set_uniform_vec2("scale", 0.125, 0.125 * 4.0 / 3.0);
			self.unit_quad.draw();

			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

			gl::Viewport(0, 0, window_width as GLsizei, window_height as GLsizei);
			gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			self.viewport_prog.bind();
			gl::BindTexture(gl::TEXTURE_2D, self.fbt);
			self.viewport_prog.set_uniform_vec2("pos", 0.0, 0.0);
			self.viewport_prog.set_uniform_vec2("scale", 1.0, 1.0);
			self.viewport_quad.draw();

			err_check();
		}
	}
}