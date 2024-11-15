#![no_std]

mod avk;
mod image;
mod palette;
pub mod prelude;
mod sprite;

pub use avk::AvkRaw;

/// Square pixel size of sprites and tiles.
pub const IMAGE_SIZE: i16 = 16; // px

/// Tiles per row of the canvas.
pub const CANVAS_WIDTH: i16 = 16; // tiles/screen
/// Tiles per column of the canvas.
pub const CANVAS_HEIGHT: i16 = 12; // tiles/screen
/// The total tile count of the canvas.
pub const CANVAS_SIZE: usize = CANVAS_WIDTH as usize * CANVAS_HEIGHT as usize; // 192

// TODO: figure out how to name CANVAS, BACKGROUND_CANVAS, and RESOLUTION in a way that makes sense

// the canvas including scroll padding
pub const BACKGROUND_CANVAS_WIDTH: i16 = CANVAS_WIDTH + 2;
pub const BACKGROUND_CANVAS_HEIGHT: i16 = CANVAS_HEIGHT + 2;
pub const BACKGROUND_CANVAS_SIZE: usize =
	BACKGROUND_CANVAS_WIDTH as usize * BACKGROUND_CANVAS_HEIGHT as usize;

/// Pixels per row of the canvas.
pub const RESOLUTION_WIDTH: i16 = IMAGE_SIZE * CANVAS_WIDTH;
/// Pixels per column of the canvas
pub const RESOLUTION_HEIGHT: i16 = IMAGE_SIZE * CANVAS_HEIGHT;
/// The total pixel count of the canvas.
pub const RESOLUTION_SIZE: usize = RESOLUTION_WIDTH as usize * RESOLUTION_HEIGHT as usize;

pub const MAX_IMAGES: usize = 256;
pub const MAX_PALETTES: usize = 16;
pub const MAX_SPRITES: usize = 96;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum Player {
	Alpha,
	Bravo,
	Charlie,
	Delta,
}

impl Player {
	pub fn index(&self) -> usize {
		match self {
			Player::Alpha => 0,
			Player::Bravo => 1,
			Player::Charlie => 2,
			Player::Delta => 3,
		}
	}
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Tile {
	pub image_id: u8,
	/// lower bits are palette, upper bits are flip
	pub palette_id: u8,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
#[repr(C)]
pub enum AvkGamepadInput {
	DirUp,
	DirRight,
	DirDown,
	DirLeft,

	FaceUp,
	FaceRight,
	FaceDown,
	FaceLeft,

	TriggerLeft,
	TriggerRight,

	Menu,
}

pub fn rgba_to_u16(mut rgba: [u8; 4]) -> u16 {
	if rgba[3] > 7 {
		rgba[3] = 15;
		// red
		(rgba[0] as u16) << 12 |
            // green
            (rgba[1] as u16) << 8 |
            // blue
            (rgba[2] as u16) << 4 |
            // alpha
            (rgba[3] as u16)
	} else {
		0
	}
}

pub const fn u16_to_rgba(color: u16) -> [u8; 4] {
	[
		// red
		((color & 0b1111_0000_0000_0000) >> 12) as u8,
		// green
		((color & 0b0000_1111_0000_0000) >> 8) as u8,
		// blue
		((color & 0b0000_0000_1111_0000) >> 4) as u8,
		// alpha
		(color & 0b0000_0000_0000_1111) as u8,
	]
}
