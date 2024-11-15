use crate::sdl::sys::*;
use crate::sdl::{panic_sdl_error, set_sdl_prop, SdlProperty};
use std::ffi::{c_char, c_int};
use std::ptr::null_mut;

#[derive(Copy, Clone)]
pub struct Window {
	pub(super) sdl_window: *mut SDL_Window,
	pub(super) width: u32,
	pub(super) height: u32,
	pub(super) _gl_context: SDL_GLContext,
}

impl Window {
	pub fn init(width: i32, height: i32) -> Self {
		unsafe {
			let window_props = SDL_CreateProperties();
			set_sdl_prop(
				window_props,
				SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER,
				SdlProperty::Number(width as i64),
			);
			set_sdl_prop(
				window_props,
				SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER,
				SdlProperty::Number(height as i64),
			);
			set_sdl_prop(
				window_props,
				SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN,
				SdlProperty::Bool(false),
			);
			set_sdl_prop(
				window_props,
				SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN,
				SdlProperty::Bool(true),
			);
			set_sdl_prop(
				window_props,
				SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN,
				SdlProperty::Bool(true),
			);
			set_sdl_prop(
				window_props,
				SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN,
				SdlProperty::Bool(true),
			);

			let sdl_window = SDL_CreateWindowWithProperties(window_props);
			if sdl_window == null_mut() {
				panic_sdl_error("Failed to create window!");
			}

			// Create OpenGL context for window
			let gl_context = SDL_GL_CreateContext(sdl_window);
			if gl_context == null_mut() {
				panic_sdl_error("Failed to create OpenGL context!");
			}

			// from the docs:
			// 0 for immediate updates
			// 1 for updates synchronized with the vertical retrace
			// -1 for adaptive vsync
			SDL_GL_SetSwapInterval(1);

			Window {
				sdl_window,
				_gl_context: gl_context,
				width: width as u32,
				height: height as u32,
			}
		}
	}

	#[inline]
	pub fn show(&mut self) {
		unsafe {
			SDL_ShowWindow(self.sdl_window);
		}
	}

	#[inline]
	pub fn set_width(&mut self, w: u32) {
		self.width = w;
		unsafe {
			SDL_SetWindowSize(self.sdl_window, self.width as c_int, self.height as c_int);
		}
	}

	#[inline]
	pub fn get_width(&self) -> u32 {
		self.width
	}

	#[inline]
	pub fn set_height(&mut self, h: u32) {
		self.height = h;
		unsafe {
			SDL_SetWindowSize(self.sdl_window, self.width as c_int, self.height as c_int);
		}
	}

	#[inline]
	pub fn get_height(&self) -> u32 {
		self.height
	}

	#[inline]
	pub fn set_title(&mut self, title: impl Into<String>) {
		unsafe {
			SDL_SetWindowTitle(self.sdl_window, title.into().as_ptr() as *const c_char);
		}
	}
}
