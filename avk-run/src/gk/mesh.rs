use std::ffi::c_void;
use std::ptr::null;
use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint, GLushort};
use crate::gk::err_check;

#[derive(Copy, Clone)]
pub(crate) struct Mesh {
	_vertex_count: usize,
	tri_count: usize,
	vao: GLuint,
	vbo: GLuint,
	ebo: GLuint,
}

impl Mesh {
	pub fn new(vertex_count: usize, vertex_data: &[GLfloat], element_data: &[GLushort]) -> Self {
		if element_data.len() % 3 != 0 {
			panic!("vertex element data can't be used to draw triangles (length % 3 != 0)")
		}

		let mut buffers = [0; 2];
		unsafe {
			gl::GenBuffers(buffers.len() as GLsizei, &mut buffers as *mut GLuint);
		}
		err_check();
		let vao = unsafe {
			let mut vao = 0;
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);
			vao
		};
		let vbo = buffers[0];
		let ebo = buffers[1];

		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			err_check();
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(vertex_data.len() * size_of::<GLfloat>()) as GLsizeiptr,
				vertex_data.as_ptr() as *const c_void,
				gl::STATIC_DRAW
			);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::VertexAttribPointer(
				0,
				2,
				gl::FLOAT,
				gl::FALSE,
				(2 * size_of::<GLfloat>()) as GLsizei,
				null()
			);
			gl::EnableVertexAttribArray(0);
			err_check();

			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
			err_check();
			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				(element_data.len() * size_of::<GLfloat>()) as GLsizeiptr,
				element_data.as_ptr() as *const c_void,
				gl::STATIC_DRAW
			);
		}

		Self {
			_vertex_count: vertex_count,
			tri_count: element_data.len() / 3,
			vao,
			vbo,
			ebo,
		}
	}

	pub fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
			gl::DrawElements(gl::TRIANGLES, (self.tri_count * 3) as GLsizei, gl::UNSIGNED_SHORT, null());
		}
	}
}
