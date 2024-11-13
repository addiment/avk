use std::array;
use std::collections::HashMap;
use std::ptr::null_mut;
use avk_types::prelude::*;
use avk_types::{
	AvkRaw,
	MAX_IMAGES,
	MAX_PALETTES
};

use crate::gk::GirlsKissing;
use crate::sdl::Gman;

pub struct AvkBackend {
	pub raw: *mut AvkRaw,
	pub palettes: [Palette; MAX_PALETTES],
	pub boot_images: [Image; 4],
	pub images:	[Image; MAX_IMAGES],

	pub input_state: [HashMap<GamepadInput, bool>; 4],

	gman: Gman,
	girls_kissing: GirlsKissing,
}

impl AvkBackend {
	pub fn init(images: &[Image; MAX_IMAGES], palettes: &[Palette; MAX_PALETTES]) -> Self {
		let mut images = images.clone();
		let mut palettes = palettes.clone();
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
				Image::from_resource(include_bytes!("icon0.avkres")),
				Image::from_resource(include_bytes!("icon1.avkres")),
				Image::empty(),
				Image::empty(),
			],

			raw: null_mut(),

			input_state: array::from_fn(|_| {
				let mut hm = HashMap::with_capacity(16);
				for e in [
					GamepadInput::DirUp,
					GamepadInput::DirDown,
					GamepadInput::DirLeft,
					GamepadInput::DirRight,
					GamepadInput::FaceUp,
					GamepadInput::FaceDown,
					GamepadInput::FaceLeft,
					GamepadInput::FaceRight,
					GamepadInput::TriggerLeft,
					GamepadInput::TriggerRight,
					GamepadInput::Menu,
				] {
					hm.insert(e, false);
				}

				hm
			}),
			gman,
			girls_kissing,
		}
	}

	fn update_input_state(&mut self) {
		for player in [Player::Alpha, Player::Bravo, Player::Charlie, Player::Delta] {
			let idx = player.index();
			let kb = &self.gman.action_state_kb[idx];
			let gp = &self.gman.action_state_gp[idx];

			for input in [
				GamepadInput::DirUp,
				GamepadInput::DirDown,
				GamepadInput::DirLeft,
				GamepadInput::DirRight,
				GamepadInput::FaceUp,
				GamepadInput::FaceDown,
				GamepadInput::FaceLeft,
				GamepadInput::FaceRight,
				GamepadInput::TriggerLeft,
				GamepadInput::TriggerRight,
				GamepadInput::Menu,
			] {
				let state = *kb.get(&input).unwrap_or(&false)
					|| *gp.get(&input).unwrap_or(&false);
				self.input_state[idx].insert(input, state);
			}
		}
	}

	pub fn update(&mut self) -> bool {
		// silly!!! breaking mutability rules!!! I don't care!!!
		let this = self as *mut Self;
		self.girls_kissing.update(
			this,
			self.gman.window.get_width(),
			self.gman.window.get_height()
		);
		let should_not_quit = self.gman.update();

		self.update_input_state();

		should_not_quit
	}

	pub fn get_input(&self, player: Player, input: GamepadInput) -> bool {
		let p_input = self.input_state.get(player.index());
		if let Some(p_input) = p_input {
			if let Some(p_input) = p_input.get(&input) {
				return *p_input;
			}
		}
		false
	}

	/// Returns the current time, in milliseconds.
	pub fn get_time(&self) -> u64 {
		self.gman.get_ticks_ms()
	}
}
