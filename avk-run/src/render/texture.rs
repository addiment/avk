use crate::render::gl_err_check;
use avk_types::prelude::Image;
use avk_types::{IMAGE_SIZE, MAX_IMAGES};
use gl::types::{GLint, GLsizei, GLuint};
use std::array::from_fn;
use std::ffi::c_void;

#[derive(Copy, Clone)]
pub(crate) struct Texture {
	texture_handle: GLuint,
}

impl Texture {
	/// Loads all the images from an array of AVK indexed-images
	pub fn new_bulk(data: &mut [Image; MAX_IMAGES]) -> [Self; MAX_IMAGES] {
		let mut texture_ids: [GLuint; MAX_IMAGES] = [0; MAX_IMAGES];
		unsafe {
			gl::CreateTextures(
				gl::TEXTURE_2D,
				MAX_IMAGES as GLsizei,
				texture_ids.as_mut_ptr(),
			);
			gl_err_check();
		}

		let textures = from_fn(|i| {
			unsafe {
				gl::BindTexture(gl::TEXTURE_2D, texture_ids[i]);
				gl::TexImage2D(
					gl::TEXTURE_2D,
					0,
					// one u8 red channel
					gl::R8 as GLint,
					IMAGE_SIZE as GLsizei,
					IMAGE_SIZE as GLsizei,
					0,
					// grayscale red
					gl::RED,
					// u8
					gl::UNSIGNED_BYTE,
					data[i].0.as_mut_ptr() as *mut c_void,
				);
				gl_err_check();
				// image won't show up without this. damn you, OpenGL!
				gl::GenerateMipmap(gl::TEXTURE_2D);
				gl_err_check();
			}
			Texture {
				texture_handle: texture_ids[i],
			}
		});

		textures
	}
	pub fn bind(&self) {
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
		}
	}
}
