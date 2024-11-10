use std::ptr::{null, null_mut};
use crate::gk::err_check;
use gl::types::{GLchar, GLsizei, GLuint};

pub(crate) struct Material {
	prog: GLuint,
	frag: GLuint,
	vert: GLuint,
}

impl Material {
	pub fn new(frag_source: &str, vert_source: &str) -> Self {
		let prog = unsafe {
			gl::CreateProgram()
		};

		let frag = unsafe {
			gl::CreateShader(gl::FRAGMENT_SHADER)
		};

		let vert = unsafe {
			gl::CreateShader(gl::VERTEX_SHADER)
		};

		unsafe fn ensure_shader_status(shader: GLuint) {
			let mut success = 0;
			let mut info_log = ['\0'; 1024];
			gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
			if success != gl::TRUE as i32 {
				gl::GetShaderInfoLog(shader, info_log.len() as GLsizei, null_mut(), info_log.as_mut_ptr() as *mut GLchar);
				err_check();
				panic!("OpenGL error: {success} \"{}\"", String::from_iter(info_log));
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
			err_check();
			gl::AttachShader(prog, vert);
			err_check();
			gl::LinkProgram(prog);
			err_check();
			{
				let mut success = 0;
				let mut info_log = ['\0'; 512];
				gl::GetProgramiv(prog, gl::LINK_STATUS, &mut success);
				err_check();
				if success != gl::TRUE as i32 {
					gl::GetProgramInfoLog(prog, 512, null_mut(), info_log.as_mut_ptr() as *mut GLchar);
					err_check();
					panic!("OpenGL error: {success} {}", String::from_iter(info_log));
				}
			}
		}

		Self {
			prog,
			frag,
			vert,
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
			gl::Uniform2f(
				u_pos_loc,
				x,
				y,
			);
		}
	}
}