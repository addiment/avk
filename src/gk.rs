use std::array::from_fn;
use std::ffi::{c_char, c_void, CStr, CString, OsStr};
use std::ptr::{null, null_mut};
use gl::types::{GLchar, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint, GLushort};
use avk_types::{CANVAS_WIDTH, IMAGE_SIZE, MAX_IMAGES, MAX_PALETTES, RESOLUTION_SIZE};
use crate::prelude::*;

const SQUARE_MESH: [GLfloat; 8] = [
	0.0, 0.0,
	1.0, 0.0,
	1.0, 1.0,
	0.0, 1.0,
];
const SQUARE_MESH_ELEMENTS: [GLushort; 6] = [
	0, 1, 2,
	2, 3, 0
];

const FRAG_SOURCE: &str = concat!(include_str!("shaders/frag.glsl"), "\0");
const VERT_SOURCE: &str = concat!(include_str!("shaders/vert.glsl"), "\0");

#[derive(Copy, Clone)]
pub(crate) struct Texture {
	// data: [Image; Image::PIXEL_COUNT],
	texture_handle: GLuint,
}

impl Texture {
	fn new_bulk(data: &mut [Image; MAX_IMAGES]) -> [Self; MAX_IMAGES] {
		let mut texture_ids: [GLuint; MAX_IMAGES] = [0; MAX_IMAGES];
		unsafe {
			gl::CreateTextures(gl::TEXTURE_2D, MAX_IMAGES as GLsizei, texture_ids.as_mut_ptr());
			let err = gl::GetError();
			if err != gl::NO_ERROR {
				let err_str = gl::GetString(err);
				println!("{}", err_str as usize);
				// println!("{}", CStr::from_ptr( as *const c_char).to_str().unwrap());
				panic!("GL error: {err}")
			}
		}


		let textures = from_fn(|i| {
			unsafe {
				gl::BindTexture(gl::TEXTURE_2D, texture_ids[i]);
				gl::TexImage2D(
					gl::TEXTURE_2D,
					0,
					gl::RGBA4 as GLint,
					IMAGE_SIZE as GLsizei,
					IMAGE_SIZE as GLsizei,
					0,
					gl::R8,
					gl::UNSIGNED_BYTE,
					data[i].0.as_mut_ptr() as *mut c_void
				);
			}
			Texture { texture_handle: texture_ids[i] }
		});

		textures
	}
}

pub(crate) struct Material {
	prog: GLuint,
	frag: GLuint,
	vert: GLuint,
}

#[inline(always)]
fn err_check() {
	unsafe {
		let err = gl::GetError();
		if err != gl::NO_ERROR {
			panic!("GL error: {err}")
		}
	}
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
			gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success as *mut GLint);
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
				gl::GetProgramiv(prog, gl::LINK_STATUS, &mut success as *mut GLint);
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
}

pub(crate) struct Mesh {
	vertex_count: usize,
	vbo: GLuint,
	ebo: GLuint
}

impl Mesh {
	pub fn new(vertex_count: usize, vertex_data: &[GLfloat], element_data: &[GLushort]) -> Self {
		let mut buffers = [0; 2];
		unsafe {
			gl::GenBuffers(buffers.len() as GLsizei, &mut buffers as *mut GLuint);
		}
		err_check();
		let vao = unsafe {
			let mut vao = 0;
			gl::GenVertexArrays(1, &mut vao as *mut GLuint);
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
			vertex_count,
			vbo,
			ebo,
		}
	}
}

pub(crate) struct GirlsKissing {
	textures: [Texture; MAX_IMAGES],
	shared_mesh: Mesh,
	shared_prog: Material,
	// pub framebuffers:
	// pub shaders:
}

extern "system" fn girl_kisser_alert(
	source: GLenum,
	gl_type: GLenum,
	id: GLuint,
	severity: GLenum,
	length: GLsizei,
	message: *const GLchar,
	user_param: *mut c_void
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
		unsafe {
			gl::Enable(gl::DEBUG_OUTPUT);
			gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
			gl::DebugMessageCallback(Some(girl_kisser_alert), null());
		}

		Self {
			// textures: Texture::new_bulk(images)
			textures: [Texture { texture_handle: 0 }; MAX_IMAGES ],
			shared_mesh: Mesh::new(4, &SQUARE_MESH, &SQUARE_MESH_ELEMENTS),
			shared_prog: Material::new(FRAG_SOURCE, VERT_SOURCE),
		}
	}

	pub fn update(&mut self) {
		unsafe {
			gl::Viewport(0, 0, avk_types::RESOLUTION_WIDTH as GLsizei, avk_types::RESOLUTION_HEIGHT as GLsizei);
			// gl::Viewport(-16, -16, 32, 32);
			// gl::Viewport(0, 0, 512, 384);
			gl::ClearColor(0.933333333, 0.4, 0.133333333, 1.0);
			// gl::ClearColor(0.0, 0.0, 0.0, 1.0);
			err_check();
			gl::Clear(gl::COLOR_BUFFER_BIT);
			err_check();

			// gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

			err_check();
			gl::BindBuffer(gl::ARRAY_BUFFER, self.shared_mesh.vbo);
			err_check();
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.shared_mesh.ebo);
			err_check();
			gl::UseProgram(self.shared_prog.prog);

			let u_pos_loc = gl::GetUniformLocation(self.shared_prog.prog, "pos\0".as_ptr() as *const GLchar);
			gl::Uniform2f(
				u_pos_loc,
				// ((std::time::SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs_f32() / 2.0)) * (RESOLUTION_SIZE - IMAGE_SIZE as usize) as GLfloat,
				0.0,
				0.0,
			);

			err_check();
			// gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, self.shared_mesh.elements.as_ptr() as *const c_void);
			gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, null());
			// gl::DrawArrays(gl::LINE_LOOP, 0, 4);
			err_check();

			// let err = gl::GetError();
			// if err != gl::NO_ERROR {
			// 	panic!("OpenGL error: {err}");
			// 	// panic!("OpenGL error: {err} {:?}", CStr::from_ptr(gl::GetString(err) as *const c_char));
			// }
		}
	}
}