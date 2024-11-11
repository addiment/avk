#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod window;

mod sys;

use std::ffi::*;
use std::mem;
use std::ptr::*;
use log::debug;
use avk_types::prelude::*;
use crate::sdl::sys::*;
use crate::sdl::window::{Window};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub(super) enum Keycode {
	Unknown,
	Return,
	Escape,
	Backspace,
	Tab,
	Space,
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

pub(super) fn sdl_keycode_to_keycode(value: SDL_Keycode) -> Keycode {
	match value {
		SDLK_RETURN => Keycode::Return,
		SDLK_ESCAPE => Keycode::Escape,
		SDLK_BACKSPACE => Keycode::Backspace,
		SDLK_TAB => Keycode::Tab,
		SDLK_SPACE => Keycode::Space,
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

#[allow(dead_code)]
unsafe fn panic_sdl_error(format_string: &str) {
	let err = SDL_GetError();
	panic!("{} {}", format_string, if err == null() { String::from("No further information.") } else { CString::from_raw(err as *mut c_char).into_string().unwrap() });
}

pub struct Gman {
	pub window: Window,
	gamepads: Vec<(SDL_JoystickID, *mut SDL_Gamepad)>,
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
			SDL_SetPointerProperty(
				props,
				key.as_ptr() as *const c_char,
				p
			);
		}
		SdlProperty::String(s) => {
			SDL_SetStringProperty(
				props,
				key.as_ptr() as *const c_char,
				s.as_ptr() as *const c_char
			);
		}
		SdlProperty::Number(i) => {
			SDL_SetNumberProperty(
				props,
				key.as_ptr() as *const c_char,
				i
			);
		}
		SdlProperty::Float(f) => {
			SDL_SetFloatProperty(
				props,
				key.as_ptr() as *const c_char,
				f
			);
		}
		SdlProperty::Bool(b) => {
			SDL_SetBooleanProperty(
				props,
				key.as_ptr() as *const c_char,
				b
			);
		}
	}
}

unsafe fn process_gamepad_button(event: &SDL_Event, down: bool) -> Option<(GamepadInput, bool)> {
	match event.gbutton.button as c_int {
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_NORTH => Some((GamepadInput::FaceUp, down)),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_EAST  => Some((GamepadInput::FaceRight, down)),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_SOUTH => Some((GamepadInput::FaceDown, down)),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_WEST  => Some((GamepadInput::FaceLeft, down)),

		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_DPAD_UP => Some((GamepadInput::DirUp, down)),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_DPAD_RIGHT => Some((GamepadInput::DirRight, down)),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_DPAD_DOWN => Some((GamepadInput::DirDown, down)),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_DPAD_LEFT => Some((GamepadInput::DirLeft, down)),

		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_LEFT_SHOULDER => Some((GamepadInput::TriggerLeft, down)),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER => Some((GamepadInput::TriggerRight, down)),

		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_BACK => Some((GamepadInput::Menu, down)),
		SDL_GamepadButton_SDL_GAMEPAD_BUTTON_START => Some((GamepadInput::Menu, down)),

		_ => None,
	}
}

impl <'a> Gman<> {
	pub fn new(
		app_name: impl Into<String>,
		app_version: impl Into<String>,
		app_identifier: impl Into<String>
	) -> Self {
		unsafe {
			let width: i32 = avk_types::RESOLUTION_WIDTH as i32 * 2;
			let height: i32 = avk_types::RESOLUTION_HEIGHT as i32 * 2;

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
					app_identifier.as_ptr() as *const c_char
				);
			}

			if !SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_GAMEPAD) {
				panic_sdl_error("Failed to initialize SDL!");
			}

			// set OpenGL attributes
			SDL_GL_SetAttribute(SDL_GLattr_SDL_GL_CONTEXT_PROFILE_MASK, SDL_GLprofile_SDL_GL_CONTEXT_PROFILE_CORE as c_int);
			SDL_GL_SetAttribute(SDL_GLattr_SDL_GL_CONTEXT_MAJOR_VERSION, 4);
			SDL_GL_SetAttribute(SDL_GLattr_SDL_GL_CONTEXT_MINOR_VERSION, 4);
			SDL_GL_SetAttribute(SDL_GLattr_SDL_GL_DOUBLEBUFFER, 0);

			let window = Window::init(width, height);

			Self {
				window,
				gamepads: Vec::new()
			}
		}
	}

	pub fn girls_loader(proc: &'static str) -> *const c_void {
		// null-terminate it
		let proc = String::from(proc) + "\0";
		let res = unsafe {
			SDL_GL_GetProcAddress(proc.as_ptr() as *const c_char)
				.unwrap_unchecked()
				as *const c_void
		};
		// println!("{proc} @ {:#x}", res as usize);
		res
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

						SDL_EventType_SDL_EVENT_GAMEPAD_REMOVED => {
							let gamepad = self.gamepads.iter().position(|(j_id, _gamepad)| *j_id == event.gdevice.which);
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
						},

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
						},

						// SDL_EventType_SDL_EVENT_GAMEPAD_AXIS_MOTION => {
						// 	if let Some(axis) = match event.gaxis.axis as c_int {
						// 		SDL_GamepadAxis_SDL_GAMEPAD_AXIS_LEFTX => Some(GamepadInput::StickLeftX),
						// 		SDL_GamepadAxis_SDL_GAMEPAD_AXIS_LEFTY => Some(GamepadInput::StickLeftY),
						// 		_ => None,
						// 	} {
						// 		Some(BackendEvent::Gamepad(GamepadInputData::Analog {
						// 			input: axis,
						// 			x: ((event.gaxis.value as f32) + 0.5) / 32767.5,
						// 		}))
						// 	} else {
						// 		None
						// 	}
						// },

						SDL_EventType_SDL_EVENT_GAMEPAD_BUTTON_DOWN =>  { let _ = process_gamepad_button(&event, true); }
						SDL_EventType_SDL_EVENT_GAMEPAD_BUTTON_UP =>  { let _ = process_gamepad_button(&event, false); }

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
		unsafe {
			SDL_GetTicks()
		}
	}
}

impl <'a> Drop for Gman<> {
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