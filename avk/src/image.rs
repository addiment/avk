use crate::*;

/// Image data usable by any tile or sprite.
#[derive(Copy, Clone)]
pub struct Image(pub [u8; Image::PIXEL_COUNT]);

impl Image {
	pub const PIXEL_COUNT: usize = IMAGE_SIZE as usize * IMAGE_SIZE as usize;

	pub const fn empty() -> Self {
		Self([0; Image::PIXEL_COUNT])
	}

	/// Creates a new image from the given avkres data.
	/// 4 bpp (2 pixels/byte), indexed color bitmap.
	pub const fn from_resource(avk_res: &[u8; 128]) -> Self {
		let mut this = [0; Image::PIXEL_COUNT];
		let mut i = 0;
		while i < 128 {
			this[i * 2] = avk_res[i] >> 4;
			this[i * 2 + 1] = avk_res[i] & 0b1111;
			i += 1;
		}
		Self(this)
	}
}
