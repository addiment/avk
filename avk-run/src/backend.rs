use avk_types::prelude::*;
use avk_types::{AvkRaw, MAX_IMAGES, MAX_PALETTES};
use std::array;
use std::collections::HashMap;
use std::ptr::null_mut;

use crate::render::AvkRenderManager;
use crate::sdl::SdlManager;

pub struct AvkBackend {
	pub raw: *mut AvkRaw,
	pub palettes: [Palette; MAX_PALETTES],
	// TODO: add the boot screen!
	pub boot_images: [Image; 4],
	pub images: [Image; MAX_IMAGES],

	pub input_state: [HashMap<AvkGamepadInput, bool>; 4],

	sdl_manager: SdlManager,
	render_manager: AvkRenderManager,
}

impl AvkBackend {
	pub fn init(images: &[Image; MAX_IMAGES], palettes: &[Palette; MAX_PALETTES]) -> Self {
		let mut images = images.clone();
		let mut palettes = palettes.clone();
		let gman = SdlManager::new("AK Virtual Console", "1.0.0", "computer.living.ak");
		let girls_kissing =
			AvkRenderManager::init(&mut images, &mut palettes, SdlManager::gl_loader);

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
					AvkGamepadInput::DirUp,
					AvkGamepadInput::DirDown,
					AvkGamepadInput::DirLeft,
					AvkGamepadInput::DirRight,
					AvkGamepadInput::FaceUp,
					AvkGamepadInput::FaceDown,
					AvkGamepadInput::FaceLeft,
					AvkGamepadInput::FaceRight,
					AvkGamepadInput::TriggerLeft,
					AvkGamepadInput::TriggerRight,
					AvkGamepadInput::Menu,
				] {
					hm.insert(e, false);
				}

				hm
			}),
			sdl_manager: gman,
			render_manager: girls_kissing,
		}
	}

	fn update_input_state(&mut self) {
		for player in [Player::Alpha, Player::Bravo, Player::Charlie, Player::Delta] {
			let idx = player.index();
			let kb = &self.sdl_manager.action_state_kb[idx];
			let gp = &self.sdl_manager.action_state_gp[idx];

			for input in [
				AvkGamepadInput::DirUp,
				AvkGamepadInput::DirDown,
				AvkGamepadInput::DirLeft,
				AvkGamepadInput::DirRight,
				AvkGamepadInput::FaceUp,
				AvkGamepadInput::FaceDown,
				AvkGamepadInput::FaceLeft,
				AvkGamepadInput::FaceRight,
				AvkGamepadInput::TriggerLeft,
				AvkGamepadInput::TriggerRight,
				AvkGamepadInput::Menu,
			] {
				let state = *kb.get(&input).unwrap_or(&false) || *gp.get(&input).unwrap_or(&false);
				self.input_state[idx].insert(input, state);
			}
		}
	}

	pub fn update(&mut self) -> bool {
		// silly!!! breaking mutability rules!!! I don't care!!!
		let this = self as *mut Self;
		self.render_manager.update(
			this,
			self.sdl_manager.window.get_width(),
			self.sdl_manager.window.get_height(),
		);
		let should_not_quit = self.sdl_manager.update();

		self.update_input_state();

		should_not_quit
	}

	pub fn get_input(&self, player: Player, input: AvkGamepadInput) -> bool {
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
		self.sdl_manager.get_ticks_ms()
	}
}
