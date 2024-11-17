#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod window;

mod audio;
mod sys;

use crate::sdl::sys::*;
use crate::sdl::window::Window;
use avk_types::{AvkGamepadInput, Player};
use log::{debug, warn};
use std::array::from_fn;
use std::collections::HashMap;
use std::ffi::*;
use std::mem;
use std::ptr::*;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub(super) enum Keycode {
	Unknown,
	Return,
	Escape,
	Backspace,
	Tab,
	Space,
	Comma,
	Period,
	Num0,
	Num1,
	Num2,
	Num3,
	Num4,
	Num5,
	Num6,
	Num7,
	Num8,
	Num9,
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	Delete,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	PrintScreen,
	ScrollLock,
	PauseBreak,
	Insert,
	Home,
	PageUp,
	End,
	PageDown,
}

/// Converts an SDL Keycode to the binding enumerated Keycode type.
pub(super) fn sdl_keycode_to_keycode(value: SDL_Keycode) -> Keycode {
	match value {
		SDLK_RETURN => Keycode::Return,
		SDLK_ESCAPE => Keycode::Escape,
		SDLK_BACKSPACE => Keycode::Backspace,
		SDLK_TAB => Keycode::Tab,
		SDLK_SPACE => Keycode::Space,
		SDLK_COMMA => Keycode::Comma,
		SDLK_PERIOD => Keycode::Period,
		SDLK_0 => Keycode::Num0,
		SDLK_1 => Keycode::Num1,
		SDLK_2 => Keycode::Num2,
		SDLK_3 => Keycode::Num3,
		SDLK_4 => Keycode::Num4,
		SDLK_5 => Keycode::Num5,
		SDLK_6 => Keycode::Num6,
		SDLK_7 => Keycode::Num7,
		SDLK_8 => Keycode::Num8,
		SDLK_9 => Keycode::Num9,
		SDLK_A => Keycode::A,
		SDLK_B => Keycode::B,
		SDLK_C => Keycode::C,
		SDLK_D => Keycode::D,
		SDLK_E => Keycode::E,
		SDLK_F => Keycode::F,
		SDLK_G => Keycode::G,
		SDLK_H => Keycode::H,
		SDLK_I => Keycode::I,
		SDLK_J => Keycode::J,
		SDLK_K => Keycode::K,
		SDLK_L => Keycode::L,
		SDLK_M => Keycode::M,
		SDLK_N => Keycode::N,
		SDLK_O => Keycode::O,
		SDLK_P => Keycode::P,
		SDLK_Q => Keycode::Q,
		SDLK_R => Keycode::R,
		SDLK_S => Keycode::S,
		SDLK_T => Keycode::T,
		SDLK_U => Keycode::U,
		SDLK_V => Keycode::V,
		SDLK_W => Keycode::W,
		SDLK_X => Keycode::X,
		SDLK_Y => Keycode::Y,
		SDLK_Z => Keycode::Z,
		SDLK_PRINTSCREEN => Keycode::PrintScreen,
		SDLK_PAUSE => Keycode::PauseBreak,
		_ => Keycode::Unknown,
	}
}

/// Panics and prints the contents of SDL_GetError to the console.
unsafe fn panic_sdl_error(format_string: &str) -> ! {
	let err = SDL_GetError();
	panic!(
		"{} {}",
		format_string,
		if err == null() {
			"No further information."
		} else {
			CStr::from_ptr(err as *mut c_char).to_str().unwrap()
		}
	);
}

enum SdlProperty<'a> {
	Pointer(*mut c_void),
	String(&'a str),
	Number(i64),
	Float(f32),
	Bool(bool),
}

unsafe fn set_sdl_prop(props: SDL_PropertiesID, key: &[u8], value: SdlProperty) {
	match value {
		SdlProperty::Pointer(p) => {
			SDL_SetPointerProperty(props, key.as_ptr() as *const c_char, p);
		}
		SdlProperty::String(s) => {
			SDL_SetStringProperty(
				props,
				key.as_ptr() as *const c_char,
				s.as_ptr() as *const c_char,
			);
		}
		SdlProperty::Number(i) => {
			SDL_SetNumberProperty(props, key.as_ptr() as *const c_char, i);
		}
		SdlProperty::Float(f) => {
			SDL_SetFloatProperty(props, key.as_ptr() as *const c_char, f);
		}
		SdlProperty::Bool(b) => {
			SDL_SetBooleanProperty(props, key.as_ptr() as *const c_char, b);
		}
	}
}

unsafe fn process_gamepad_button(event: &SDL_Event) -> Option<AvkGamepadInput> {
	match event.gbutton.button as c_int {
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_NORTH => Some(AvkGamepadInput::FaceUp),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_EAST => Some(AvkGamepadInput::FaceRight),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_SOUTH => Some(AvkGamepadInput::FaceDown),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_WEST => Some(AvkGamepadInput::FaceLeft),

		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_DPAD_UP => Some(AvkGamepadInput::DirUp),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_DPAD_RIGHT => Some(AvkGamepadInput::DirRight),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_DPAD_DOWN => Some(AvkGamepadInput::DirDown),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_DPAD_LEFT => Some(AvkGamepadInput::DirLeft),

		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_LEFT_SHOULDER => Some(AvkGamepadInput::TriggerLeft),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER => Some(AvkGamepadInput::TriggerRight),

		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_BACK => Some(AvkGamepadInput::Menu),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_START => Some(AvkGamepadInput::Menu),

		_ => None,
	}
}

pub struct SdlManager {
	pub window: Window,
	gamepads: Vec<(SDL_JoystickID, *mut SDL_Gamepad)>,
	// TODO: fix joystick support by tracking previous state... grumble grumble
	pub action_state_gp: [HashMap<AvkGamepadInput, bool>; 4],
	pub action_state_kb: [HashMap<AvkGamepadInput, bool>; 4],
}

impl<'a> SdlManager {
	pub fn new(
		app_name: impl Into<String>,
		app_version: impl Into<String>,
		app_identifier: impl Into<String>,
	) -> Self {
		unsafe {
			let width: i32 = avk_types::RESOLUTION_WIDTH as i32 * 4;
			let height: i32 = avk_types::RESOLUTION_HEIGHT as i32 * 4;

			// called before init
			SDL_SetMainReady();
			// SDL_SetLogPriorities(SDL_LogPriority_SDL_LOG_PRIORITY_TRACE);

			{
				let app_name = app_name.into() + "\0";
				let app_version = app_version.into() + "\0";
				let app_identifier = app_identifier.into() + "\0";
				SDL_SetAppMetadata(
					app_name.as_ptr() as *const c_char,
					app_version.as_ptr() as *const c_char,
					app_identifier.as_ptr() as *const c_char,
				);
			}

			if !SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_GAMEPAD) {
				panic_sdl_error("Failed to initialize SDL!");
			}

			// set OpenGL attributes
			SDL_GL_SetAttribute(
				SDL_GLAttr_SDL_GL_CONTEXT_PROFILE_MASK,
				SDL_GL_CONTEXT_PROFILE_CORE as c_int,
			);
			SDL_GL_SetAttribute(SDL_GLAttr_SDL_GL_CONTEXT_MAJOR_VERSION, 4);
			SDL_GL_SetAttribute(SDL_GLAttr_SDL_GL_CONTEXT_MINOR_VERSION, 2);
			SDL_GL_SetAttribute(SDL_GLAttr_SDL_GL_DOUBLEBUFFER, 0);

			let window = Window::init(width, height);

			Self {
				window,
				gamepads: Vec::new(),
				action_state_gp: from_fn(|_| HashMap::with_capacity(4)),
				action_state_kb: from_fn(|_| HashMap::with_capacity(4)),
			}
		}
	}

	/// Loads an OpenGL function by name.
	/// Used by the Rust OpenGL crate.
	pub fn gl_loader(proc: &'static str) -> *const c_void {
		// null-terminate it
		let proc = String::from(proc) + "\0";
		let res = unsafe {
			SDL_GL_GetProcAddress(proc.as_ptr() as *const c_char).unwrap() as *const c_void
		};
		// println!("{proc} @ {:#x}", res as usize);
		res
	}

	/// Updates the
	fn keyboard_update(&mut self, event: SDL_KeyboardEvent) {
		let player: Player;
		let button: AvkGamepadInput;
		match sdl_keycode_to_keycode(event.key) {
			Keycode::W => {
				player = Player::Alpha;
				button = AvkGamepadInput::DirUp
			}
			Keycode::A => {
				player = Player::Alpha;
				button = AvkGamepadInput::DirLeft
			}
			Keycode::S => {
				player = Player::Alpha;
				button = AvkGamepadInput::DirDown
			}
			Keycode::D => {
				player = Player::Alpha;
				button = AvkGamepadInput::DirRight
			}
			Keycode::Q => {
				player = Player::Alpha;
				button = AvkGamepadInput::TriggerLeft
			}
			Keycode::E => {
				player = Player::Alpha;
				button = AvkGamepadInput::TriggerRight
			}
			Keycode::Z => {
				player = Player::Alpha;
				button = AvkGamepadInput::FaceLeft
			}
			Keycode::C => {
				player = Player::Alpha;
				button = AvkGamepadInput::FaceRight
			}
			Keycode::X => {
				player = Player::Alpha;
				button = AvkGamepadInput::FaceUp
			}
			Keycode::V => {
				player = Player::Alpha;
				button = AvkGamepadInput::FaceDown
			}

			Keycode::I => {
				player = Player::Bravo;
				button = AvkGamepadInput::DirUp
			}
			Keycode::J => {
				player = Player::Bravo;
				button = AvkGamepadInput::DirLeft
			}
			Keycode::K => {
				player = Player::Bravo;
				button = AvkGamepadInput::DirDown
			}
			Keycode::L => {
				player = Player::Bravo;
				button = AvkGamepadInput::DirRight
			}
			Keycode::U => {
				player = Player::Bravo;
				button = AvkGamepadInput::TriggerLeft
			}
			Keycode::O => {
				player = Player::Bravo;
				button = AvkGamepadInput::TriggerRight
			}
			Keycode::N => {
				player = Player::Bravo;
				button = AvkGamepadInput::FaceLeft
			}
			Keycode::M => {
				player = Player::Bravo;
				button = AvkGamepadInput::FaceRight
			}
			Keycode::Comma => {
				player = Player::Bravo;
				button = AvkGamepadInput::FaceUp
			}
			Keycode::Period => {
				player = Player::Bravo;
				button = AvkGamepadInput::FaceDown
			}
			_ => return,
		}

		if !event.repeat {
			// println!("player {player:?} button {button:?} state {}", event.down);
			self.action_state_kb[player.index()].insert(button, event.down);
		}
	}

	pub fn update(&mut self) -> bool {
		unsafe {
			SDL_ShowWindow(self.window.sdl_window);
		}

		// poll events
		loop {
			// poll
			let (has_event, event) = unsafe {
				let mut event: SDL_Event = mem::zeroed();
				(SDL_PollEvent(&mut event as *mut SDL_Event), event)
			};

			// swap the framebuffer
			unsafe {
				SDL_GL_SwapWindow(self.window.sdl_window);
			}

			if has_event {
				unsafe {
					match event.type_ {
						SDL_EventType_SDL_EVENT_QUIT => {
							return false;
						}

						SDL_EventType_SDL_EVENT_KEY_DOWN | SDL_EventType_SDL_EVENT_KEY_UP => {
							self.keyboard_update(event.key)
						}

						SDL_EventType_SDL_EVENT_GAMEPAD_REMOVED => {
							let gamepad = self
								.gamepads
								.iter()
								.position(|(j_id, _gamepad)| *j_id == event.gdevice.which);
							if let Some(index) = gamepad {
								let (_joy_id, gamepad) = self.gamepads[index];

								{
									let name = SDL_GetGamepadName(gamepad) as *mut c_char;
									if name.is_null() {
										debug!(
											"Gamepad disconnected (@ {:?})",
											SDL_GetGamepadPlayerIndex(gamepad),
										);
									} else {
										debug!(
											"Gamepad disconnected ({:?} @ {:?})",
											CString::from_raw(name),
											SDL_GetGamepadPlayerIndex(gamepad),
										);
									};
								}

								SDL_CloseGamepad(gamepad);
								self.gamepads.remove(index);
							}
						}

						SDL_EventType_SDL_EVENT_GAMEPAD_ADDED => {
							let gamepad = SDL_OpenGamepad(event.gdevice.which);
							if gamepad.is_null() {
								panic!("Failed to open gamepad!");
							}
							self.gamepads.push((event.gdevice.which, gamepad));

							{
								let name = SDL_GetGamepadName(gamepad) as *mut c_char;
								if name.is_null() {
									debug!(
										"Gamepad connected (@ {:?})",
										SDL_GetGamepadPlayerIndex(gamepad),
									);
								} else {
									debug!(
										"Gamepad connected ({:?} @ {:?})",
										CString::from_raw(name),
										SDL_GetGamepadPlayerIndex(gamepad),
									);
								};
							}
						}

						SDL_EventType_SDL_EVENT_GAMEPAD_AXIS_MOTION => {
							let g_axis = event.gaxis;
							let pos = self
								.gamepads
								.iter()
								.position(|e| g_axis.which == e.0)
								.expect("Received a gamepad event from an invalid gamepad!");
							if pos > 3 {
								warn!("Input from extra gamepad discarded!");
							} else {
								let hm = self
									.action_state_gp
									.get_mut(pos)
									.expect("Failed to get hashmap!");
								match g_axis.axis as c_int {
									SDL_GamepadAxis_SDL_GAMEPAD_AXIS_LEFTX => {
										if g_axis.value > i16::MAX / 4 {
											hm.insert(AvkGamepadInput::DirRight, true);
											hm.insert(AvkGamepadInput::DirLeft, false);
										} else if g_axis.value < i16::MIN / 4 {
											hm.insert(AvkGamepadInput::DirRight, false);
											hm.insert(AvkGamepadInput::DirLeft, true);
										} else {
											hm.insert(AvkGamepadInput::DirRight, false);
											hm.insert(AvkGamepadInput::DirLeft, false);
										}
									}
									SDL_GamepadAxis_SDL_GAMEPAD_AXIS_LEFTY => {
										if g_axis.value > i16::MAX / 4 {
											hm.insert(AvkGamepadInput::DirUp, false);
											hm.insert(AvkGamepadInput::DirDown, true);
										} else if g_axis.value < i16::MIN / 4 {
											hm.insert(AvkGamepadInput::DirUp, true);
											hm.insert(AvkGamepadInput::DirDown, false);
										} else {
											hm.insert(AvkGamepadInput::DirUp, false);
											hm.insert(AvkGamepadInput::DirDown, false);
										}
									}
									_ => (),
								};
							}
						}

						SDL_EventType_SDL_EVENT_GAMEPAD_BUTTON_DOWN => {
							let g_button = event.gbutton;
							let pos = self
								.gamepads
								.iter()
								.position(|e| g_button.which == e.0)
								.expect("Received a gamepad event from an invalid gamepad!");

							if pos > 3 {
								warn!("Input from extra gamepad discarded!");
							} else if let Some(input) = process_gamepad_button(&event) {
								let hm = self
									.action_state_gp
									.get_mut(pos)
									.expect("Failed to get hashmap!");
								hm.insert(input, true);
							}
						}
						SDL_EventType_SDL_EVENT_GAMEPAD_BUTTON_UP => {
							let g_button = event.gbutton;
							let pos = self
								.gamepads
								.iter()
								.position(|e| g_button.which == e.0)
								.expect("Received a gamepad event from an invalid gamepad!");

							if pos > 3 {
								warn!("Input from extra gamepad discarded!");
							} else if let Some(input) = process_gamepad_button(&event) {
								let hm = self
									.action_state_gp
									.get_mut(pos)
									.expect("Failed to get hashmap!");
								hm.insert(input, false);
							}
						}

						// SDL_EventType_SDL_EVENT_WINDOW_RESIZED => Some(BackendEvent::WindowResized {
						// 	w: event.window.data1,
						// 	h: event.window.data2,
						// }),
						_ => {}
					};
				}
			} else {
				return true;
			}
		}
	}

	/// Returns the number of milliseconds elapsed since the engine start.
	#[inline(always)]
	pub fn get_ticks_ms(&self) -> u64 {
		unsafe { SDL_GetTicks() }
	}
}

impl<'a> Drop for SdlManager {
	fn drop(&mut self) {
		// Destroy the window
		if self.window.sdl_window != null_mut() {
			unsafe {
				SDL_DestroyWindow(self.window.sdl_window);
			}
		}

		// De-init SDL
		unsafe {
			SDL_Quit();
		}
	}
}
