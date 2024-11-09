use std::array;
use std::collections::HashMap;
use std::ops::Index;
use std::ptr::{null,};
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
use crate::gk::GirlsKissing;

mod sdl;
mod input;
mod tests;
pub mod prelude;
mod gk;

/// The instance struct.
pub struct Avk {
	palettes: [Palette; MAX_PALETTES],
	images:	[Image; MAX_IMAGES],

	pub background: [Tile; CANVAS_SIZE],
	pub foreground: [Sprite; MAX_SPRITES],

	input_state: [HashMap<GamepadInput, bool>; 4],

	gman: Gman,
	girls_kissing: GirlsKissing,
}

impl Avk {
	pub fn new(mut images: [Image; MAX_IMAGES], mut palettes: [Palette; MAX_PALETTES]) -> Self {

		let gman = Gman::new(
			"AK Virtual Console",
			"1.0.0",
			"computer.living.ak",
		);
		let girls_kissing = GirlsKissing::init(&mut images, &mut palettes, Gman::girls_loader);
		unsafe {
			let err = gl::GetError();
			if err != gl::NO_ERROR {
				panic!("GL error: {err}")
			}
		}
		Self {
			palettes,
			images,

			background: [Default::default(); CANVAS_SIZE],
			foreground: [Default::default(); MAX_SPRITES],

			// input_state: [],
			input_state: array::from_fn(|_| { HashMap::with_capacity(16) }),
			gman,
			girls_kissing,
		}
	}

	pub fn update(&mut self) -> bool {
		self.girls_kissing.update();
		let should_not_quit = self.gman.update();
		should_not_quit
	}

	pub fn get_input(&self, player: Player) -> &HashMap<GamepadInput, bool> {
		&self.input_state[player.index()]
	}

	/// Returns the current time, in milliseconds.
	pub fn get_time(&self) -> u64 {
		self.gman.get_ticks_ms()
	}

	pub fn get_palette(&self, index: usize) -> Palette {
		self.palettes[index]
	}

	pub fn get_image(&self, index: usize) -> Image {
		self.images[index]
	}
}