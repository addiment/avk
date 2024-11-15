use crate::render::gl_err_check;
use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint};
use std::ffi::c_void;
use std::ptr::null;

/// A mesh made up of triangles.
/// Currently, this abstraction doesn't have support for any attributes other than position.
#[derive(Copy, Clone)]
pub(crate) struct Mesh {
	_vertex_count: usize,
	tri_count: usize,
	/// The Vertex Array Object (VAO).
	/// State-wise, this holds the VBO and EBO.
	vao: GLuint,
	/// The Vertex Buffer Object (VBO).
	/// Contains the actual vertex data.
	vbo: GLuint,
	/// The Element Buffer Object (EBO).
	/// Groups the vertices into sets of 3 to draw as a triangle.
	ebo: GLuint,
}

impl Mesh {
	pub fn new(vertex_count: usize, vertex_data: &[f32], element_data: &[u16]) -> Self {
		if vertex_data.len() % 2 != 0 {
			panic!("The provided vertex data must be in pairs of XY (length % 2 != 0)")
		}
		if element_data.len() % 3 != 0 {
			panic!("The provided element data can't be used to draw triangles (length % 3 != 0)")
		}

		let mut buffers = [0; 2];
		unsafe {
			gl::GenBuffers(buffers.len() as GLsizei, &mut buffers as *mut GLuint);
		}
		gl_err_check();
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
			gl::BufferData(
				gl::ARRAY_BUFFER,
				// this is measured in BYTES, not ELEMENTS. damn you, OpenGL!
				(vertex_data.len() * size_of::<GLfloat>()) as GLsizeiptr,
				vertex_data.as_ptr() as *const c_void,
				gl::STATIC_DRAW,
			);
			gl_err_check();
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::VertexAttribPointer(
				0,
				// Each vertex is two floats- X and Y.
				// this is measured in ELEMENTS, not BYTES.
				2,
				gl::FLOAT,
				gl::FALSE,
				(2 * size_of::<GLfloat>()) as GLsizei,
				null(),
			);
			gl_err_check();
			gl::EnableVertexAttribArray(0);
			gl_err_check();

			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				// BYTES not ELEMENTS
				(element_data.len() * size_of::<GLfloat>()) as GLsizeiptr,
				element_data.as_ptr() as *const c_void,
				gl::STATIC_DRAW,
			);
			gl_err_check();
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
			gl::DrawElements(
				gl::TRIANGLES,
				(self.tri_count * 3) as GLsizei,
				gl::UNSIGNED_SHORT,
				null(),
			);
		}
	}
}
