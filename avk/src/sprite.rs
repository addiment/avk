/// An element of the foreground layer.
// tile_id, palette_id, x, y
// TODO: pleeeeeeeeeeeeease fix the alignment and visibility...
//      also, crop?
#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Sprite {
	/// The tile ID.
	// Every bit is needed, as there are 256 images max
	pub image_id: u8,
	// TODO: this is bad! icky! these should not need functions to be convenient...
	//       this is a fantasy console! we can pay for a cache miss.
	/// padding (2 bits) | flip-X (1 bit) | flip-Y (1 bit) | Palette ID (4 bits)
	pub palette_transform: u8,
	pub x: i16,
	pub y: i16,
}

impl Sprite {
	pub const FLIP_X_MASK: u8 = 0b0010_0000;
	pub const FLIP_Y_MASK: u8 = 0b0001_0000;
	pub const PALETTE_MASK: u8 = 0b1111;

	pub fn get_palette_id(&self) -> u8 {
		self.palette_transform & Self::PALETTE_MASK
	}

	pub fn get_flip_x(&self) -> bool {
		self.palette_transform & Self::FLIP_X_MASK != 0
	}

	pub fn get_flip_y(&self) -> bool {
		self.palette_transform & Self::FLIP_Y_MASK != 0
	}

	pub fn set_flip_x(&mut self, flip: bool) {
		if flip {
			self.palette_transform |= Self::FLIP_X_MASK;
		} else {
			self.palette_transform &= !Self::FLIP_X_MASK;
		}
	}

	pub fn set_flip_y(&mut self, flip: bool) {
		if flip {
			self.palette_transform |= Self::FLIP_Y_MASK;
		} else {
			self.palette_transform &= !Self::FLIP_Y_MASK;
		}
	}
}