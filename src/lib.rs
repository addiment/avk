use std::array;
use std::collections::HashMap;
use std::ops::Index;
use crate::input::GamepadInput;
use crate::sdl::Gman;

pub use avk_types::{
	IMAGE_SIZE,

	CANVAS_SIZE,
	CANVAS_WIDTH,
	CANVAS_HEIGHT,

	RESOLUTION_SIZE,
	RESOLUTION_WIDTH,
	RESOLUTION_HEIGHT,

	MAX_IMAGES,
	MAX_PALETTES,
	MAX_SPRITES
};
use avk_types::prelude::*;
mod sdl;
mod input;
mod tests;
pub mod prelude;
mod gl;

/// The instance struct.
pub struct Avk {
	palettes: [Palette; MAX_PALETTES],
	images:	[Image; MAX_IMAGES],

	pub background: [Tile; CANVAS_SIZE],
	pub foreground: [Sprite; MAX_SPRITES],

	input_state: [HashMap<GamepadInput, bool>; 4],

	gman: Gman
}

impl Avk {
	pub fn new(palettes: [Palette; MAX_PALETTES], images: [Image; MAX_IMAGES]) -> Self {
		Self {
			palettes,
			images,

			background: [Default::default(); CANVAS_SIZE],
			foreground: [Default::default(); MAX_SPRITES],

			// input_state: [],
			input_state: array::from_fn(|_| { HashMap::with_capacity(16) }),
			gman: Gman::new(
				"AK Virtual Console",
				"1.0.0",
				"computer.living.ak",
			),
		}
	}

	pub fn update(&mut self) -> bool {
		let should_not_quit = self.gman.update();
		should_not_quit
	}

	pub fn get_input(&self, player: Player) -> &HashMap<GamepadInput, bool> {
		&self.input_state[player.index()]
	}

	pub fn get_ms(&self) -> u64 {
		self.gman.get_ticks_ms()
	}

	pub fn get_palette(&self, index: usize) -> Palette {
		self.palettes[index]
	}

	pub fn get_image(&self, index: usize) -> Image {
		self.images[index]
	}
}