use std::array::from_fn;
use std::ffi::c_void;
use gl::types::{GLint, GLsizei, GLuint};
use avk_types::{IMAGE_SIZE, MAX_IMAGES};
use avk_types::prelude::Image;

#[derive(Copy, Clone)]
pub(crate) struct Texture {
	// data: [Image; Image::PIXEL_COUNT],
	texture_handle: GLuint,
}

impl Texture {
	pub fn new_bulk(data: &mut [Image; MAX_IMAGES]) -> [Self; MAX_IMAGES] {
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
					gl::R8 as GLint,
					IMAGE_SIZE as GLsizei,
					IMAGE_SIZE as GLsizei,
					0,
					gl::RED,
					gl::UNSIGNED_BYTE,
					data[i].0.as_mut_ptr() as *mut c_void
				);
				gl::GenerateMipmap(gl::TEXTURE_2D);
			}
			Texture { texture_handle: texture_ids[i] }
		});

		textures
	}
	pub fn bind(&self) {
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
		}
	}
}
