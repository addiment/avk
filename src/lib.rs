use std::array;
use std::collections::HashMap;
use crate::sdl::Gman;

pub use avk_types::{
	IMAGE_SIZE,

	CANVAS_SIZE,
	CANVAS_WIDTH,
	CANVAS_HEIGHT,

	BACKGROUND_CANVAS_SIZE,

	RESOLUTION_SIZE,
	RESOLUTION_WIDTH,
	RESOLUTION_HEIGHT,

	MAX_IMAGES,
	MAX_PALETTES,
	MAX_SPRITES
};
use avk_types::prelude::*;
use crate::gk::GirlsKissing;

pub mod prelude;
mod sdl;
mod gk;

/// The instance struct.
pub struct Avk {
	palettes: [Palette; MAX_PALETTES],
	boot_images: [Image; 4],
	images:	[Image; MAX_IMAGES],

	pub background: [Tile; BACKGROUND_CANVAS_SIZE],
	pub foreground: [Sprite; MAX_SPRITES],

	pub pan_x: f32,
	pub pan_y: f32,

	input_state: [HashMap<GamepadInput, bool>; 4],

	gman: Gman,
	girls_kissing: GirlsKissing,
}

impl Avk {
	pub fn init(mut images: [Image; MAX_IMAGES], mut palettes: [Palette; MAX_PALETTES]) -> Self {

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

			boot_images: [
				Image::from_resource(*include_bytes!("icon0.avkres")),
				Image::from_resource(*include_bytes!("icon1.avkres")),
				Image::empty(),
				Image::empty(),
			],

			background: [Default::default(); BACKGROUND_CANVAS_SIZE],
			foreground: [Default::default(); MAX_SPRITES],

			// input_state: [],
			pan_x: 0.0,
			pan_y: 0.0,

			input_state: array::from_fn(|_| { HashMap::with_capacity(16) }),
			gman,
			girls_kissing,
		}
	}

	pub fn update(&mut self) -> bool {
		let this = self as *mut Self;
		self.girls_kissing.update(
			this,
			self.gman.window.get_width(),
			self.gman.window.get_height()
		);

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
}