use crate::render::gl_err_check;
use gl::types::{GLchar, GLsizei, GLuint};
use std::ptr::{null, null_mut};

#[derive(Copy, Clone)]
pub(crate) struct Material {
	prog: GLuint,
	// frag: GLuint,
	// vert: GLuint,
}

impl Material {
	pub fn new(frag_source: &str, vert_source: &str) -> Self {
		let prog = unsafe { gl::CreateProgram() };

		let frag = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

		let vert = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };

		fn ensure_shader_status(shader: GLuint) {
			unsafe {
				let mut success = 0;
				let mut info_log = ['\0'; 1024];
				gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
				if success != gl::TRUE as i32 {
					gl::GetShaderInfoLog(
						shader,
						info_log.len() as GLsizei,
						null_mut(),
						info_log.as_mut_ptr() as *mut GLchar,
					);
					panic!(
						"Failed to compile error: {success} \"{}\"",
						String::from_iter(info_log)
					);
				}
			}
		}

		fn ensure_program_status(prog: GLuint) {
			unsafe {
				let mut success = 0;
				let mut info_log = ['\0'; 512];
				gl::GetProgramiv(prog, gl::LINK_STATUS, &mut success);
				if success != gl::TRUE as i32 {
					gl::GetProgramInfoLog(
						prog,
						info_log.len() as GLsizei,
						null_mut(),
						info_log.as_mut_ptr() as *mut GLchar,
					);
					panic!(
						"OpenGL error: {success} \"{}\"",
						String::from_iter(info_log)
					);
				}
			}
		}

		unsafe {
			gl::ShaderSource(frag, 1, &(frag_source.as_ptr() as *const GLchar), null());
			gl::CompileShader(frag);
			ensure_shader_status(frag);

			gl::ShaderSource(vert, 1, &(vert_source.as_ptr() as *const GLchar), null());
			gl::CompileShader(vert);
			ensure_shader_status(vert);

			gl::AttachShader(prog, frag);
			gl_err_check();
			gl::AttachShader(prog, vert);
			gl_err_check();
			gl::LinkProgram(prog);
			gl_err_check();
			ensure_program_status(prog);

			// cleanup
			gl::DetachShader(prog, frag);
			gl::DetachShader(prog, vert);
			gl::DeleteShader(frag);
			gl::DeleteShader(vert);
		}

		Self {
			prog,
			// frag,
			// vert,
		}
	}

	pub fn bind(&self) {
		unsafe {
			gl::UseProgram(self.prog);
		}
	}

	pub fn set_uniform_vec2(&mut self, name: impl Into<String>, x: f32, y: f32) {
		unsafe {
			let string = name.into() + "\0";
			let u_pos_loc = gl::GetUniformLocation(self.prog, string.as_ptr() as *const GLchar);
			gl::Uniform2f(u_pos_loc, x, y);
		}
	}

	pub fn set_uniform_vec4(&mut self, name: impl Into<String>, x: f32, y: f32, z: f32, w: f32) {
		unsafe {
			let string = name.into() + "\0";
			let u_pos_loc = gl::GetUniformLocation(self.prog, string.as_ptr() as *const GLchar);
			gl::Uniform4f(u_pos_loc, x, y, z, w);
		}
	}
}
