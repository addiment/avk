use std::array::from_fn;
use std::ffi::c_void;
use gl::types::{GLint, GLsizei, GLuint};
use avk_types::{IMAGE_SIZE, MAX_IMAGES, MAX_PALETTES};
use crate::prelude::*;

pub(crate) struct Texture {
	// data: [Image; Image::PIXEL_COUNT],
	texture_handle: GLuint,
}

impl Texture {
	fn new_bulk(data: &mut [Image; MAX_IMAGES]) -> [Self; MAX_IMAGES] {
		let mut texture_ids: [GLuint; MAX_IMAGES] = [0; MAX_IMAGES];
		unsafe {
			gl::CreateTextures(gl::TEXTURE_2D, MAX_IMAGES as GLsizei, texture_ids.as_mut_ptr());
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

pub(crate) struct GirlsKissing {
	pub textures: [Texture; MAX_IMAGES],
	// pub framebuffers:
	// pub shaders:
}

impl GirlsKissing {
	fn init(images: &mut [Image; MAX_IMAGES], palettes: &[Palette; MAX_PALETTES]) -> Self {
		Self {
			textures: Texture::new_bulk(images)
		}
	}
}