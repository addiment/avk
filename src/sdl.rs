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
use crate::sdl::window::{NativeWindow, Window};

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
	pub girls_context: SDL_GLContext
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

impl <'a> Gman<> {
	pub fn new(
		game_name: impl Into<String>,
		game_version: impl Into<String>,
		game_identifier: impl Into<String>
	) -> Self {
		unsafe {
			let width: i32 = avk_types::RESOLUTION_WIDTH as i32 * 4;
			let height: i32 = avk_types::RESOLUTION_HEIGHT as i32 * 4;

			// called before init
			SDL_SetMainReady();
			// SDL_SetLogPriorities(SDL_LogPriority_SDL_LOG_PRIORITY_TRACE);
			{
				let game_name = game_name.into() + "\0";
				let game_version = game_version.into() + "\0";
				let game_identifier = game_identifier.into() + "\0";
				SDL_SetAppMetadata(
					game_name.as_ptr() as *const c_char,
					game_version.as_ptr() as *const c_char,
					game_identifier.as_ptr() as *const c_char
				);
			}

			if !SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_GAMEPAD) {
				panic_sdl_error("Failed to initialize SDL!");
			}

			SDL_GL_SetAttribute(
				SDL_GLattr_SDL_GL_CONTEXT_PROFILE_MASK,
				SDL_GLprofile_SDL_GL_CONTEXT_PROFILE_CORE as c_int
			);
			SDL_GL_SetAttribute(SDL_GLattr_SDL_GL_CONTEXT_MAJOR_VERSION, 4);
			SDL_GL_SetAttribute(SDL_GLattr_SDL_GL_CONTEXT_MINOR_VERSION, 4);

			// SDL_GL_SetAttribute(SDL_GLattr_SDL_GL_DOUBLEBUFFER, 1);
			// SDL_GL_SetAttribute(SDL_GLattr_SDL_GL_DEPTH_SIZE, 16);

			let window_props = SDL_CreateProperties();
			set_sdl_prop(window_props, SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER, SdlProperty::Number(width as i64));
			set_sdl_prop(window_props, SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER, SdlProperty::Number(height as i64));
			set_sdl_prop(window_props, SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN, SdlProperty::Bool(false));
			set_sdl_prop(window_props, SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN, SdlProperty::Bool(true));
			// set_sdl_prop(window_props, SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN, SdlProperty::Bool(true));
			set_sdl_prop(window_props, SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN, SdlProperty::Bool(true));
			set_sdl_prop(window_props, SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN, SdlProperty::Bool(true));

			let sdl_window = SDL_CreateWindowWithProperties(window_props);
			if sdl_window == null_mut() {
				panic_sdl_error("Failed to create window!");
			}

			// SDL_SetWindowMinimumSize(sdl_window, width, height);
			let girls = SDL_GL_CreateContext(sdl_window);
			if girls == null_mut() {
				panic!("no bitches?");
			}
			SDL_GL_SetSwapInterval(1);

			let window_props = SDL_GetWindowProperties(sdl_window);

			let native_window= if cfg!(target_os = "linux") {
				let driver = CStr::from_ptr(SDL_GetCurrentVideoDriver() as *mut c_char).to_str().unwrap();
				if driver == "x11" {
					NativeWindow::X11 {
						window: SDL_GetNumberProperty(window_props, SDL_PROP_WINDOW_X11_WINDOW_NUMBER.as_ptr() as *const c_char, 0),
						display: SDL_GetPointerProperty(window_props, SDL_PROP_WINDOW_X11_DISPLAY_POINTER.as_ptr() as *const c_char, null_mut()),
						screen: SDL_GetNumberProperty(window_props, SDL_PROP_WINDOW_X11_SCREEN_NUMBER.as_ptr() as *const c_char, 0),
					}
				} else if driver == "wayland" {
					NativeWindow::Wayland {
						window: SDL_GetPointerProperty(window_props, SDL_PROP_WINDOW_WAYLAND_SURFACE_POINTER.as_ptr() as *const c_char, null_mut()),
						display: SDL_GetPointerProperty(window_props, SDL_PROP_WINDOW_WAYLAND_DISPLAY_POINTER.as_ptr() as *const c_char, null_mut()),
					}
				} else {
					panic!("Unknown Linux video driver \"{driver}\"!");
				}
			} else if cfg!(target_os = "windows") {
				NativeWindow::Win32 {
					hwnd: SDL_GetPointerProperty(window_props, SDL_PROP_WINDOW_WIN32_HWND_POINTER.as_ptr() as *const c_char, null_mut()),
				}
			} else {
				panic!("Unimplemented window support for current platform!");
			};

			Self {
				girls_context: girls,
				window: Window {
					sdl_window,
					native_window,
					width: width as u32,
					height: height as u32,
				},
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

				SDL_GamepadButton_SDL_GAMEPAD_BUTTON_LEFT_SHOULDER => Some((GamepadInput::BumperLeft, down)),
				SDL_GamepadButton_SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER => Some((GamepadInput::BumperRight, down)),

				SDL_GamepadButton_SDL_GAMEPAD_BUTTON_BACK => Some((GamepadInput::Select, down)),
				SDL_GamepadButton_SDL_GAMEPAD_BUTTON_START => Some((GamepadInput::Start, down)),

				_ => None,
			}
		}

		loop {
			let (res, event) = unsafe {
				let mut event: SDL_Event = mem::zeroed();
				(SDL_PollEvent(&mut event as *mut SDL_Event), event)
			};

			// swap the girls kissing
			unsafe {
				SDL_GL_SwapWindow(self.window.sdl_window);
			}

			if res {
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

	/// Returns the number of nanoseconds elapsed since the engine start.
	#[inline(always)]
	pub fn get_ticks_ns(&self) -> u64 {
		unsafe {
			SDL_GetTicksNS()
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