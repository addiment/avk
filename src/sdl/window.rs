use std::ffi::{c_char, c_int, c_void};
// use std::num::NonZeroIsize;
// use std::ptr::NonNull;
// use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle, Win32WindowHandle, WindowHandle, WindowsDisplayHandle, XlibDisplayHandle, XlibWindowHandle};
use crate::sdl::sys::*;

#[derive(Copy, Clone)]
pub struct Window {
	pub(super) sdl_window: *mut SDL_Window,
	pub(super) native_window: NativeWindow,
	pub(super) width: u32,
	pub(super) height: u32,
}

#[derive(Copy, Clone)]
#[non_exhaustive]
pub enum NativeWindow {
	Wayland { window: *mut c_void, display: *mut c_void, },
	X11 { window: i64, display: *mut c_void, screen: i64 },
	Win32 { hwnd: *mut c_void },
}

impl Window {
	pub fn show(&mut self) {
		unsafe {
			SDL_ShowWindow(self.sdl_window);
		}
	}

	pub fn set_width(&mut self, w: u32) {
		self.width = w;
		unsafe {
			SDL_SetWindowSize(self.sdl_window,self.width as c_int, self.height as c_int);
		}
	}

	pub fn set_height(&mut self, h: u32) {
		self.height = h;
		unsafe {
			SDL_SetWindowSize(self.sdl_window,self.width as c_int, self.height as c_int);
		}
	}
	
	pub fn set_title(&mut self, title: impl Into<String>) {
		unsafe {
			SDL_SetWindowTitle(self.sdl_window, title.into().as_ptr() as *const c_char);
		}
	}

	// pub unsafe fn raw_display_handle(&self) -> Result<RawDisplayHandle, HandleError> {
	// 	#[allow(unreachable_patterns)]
	// 	Ok(match self.native_window {
	// 		NativeWindow::Wayland { display, .. } => RawDisplayHandle::Wayland(WaylandDisplayHandle::new(NonNull::new(display).expect("Got null when retrieving Wayland display handle!"))),
	// 		NativeWindow::X11 { display, screen, .. } => RawDisplayHandle::Xlib(XlibDisplayHandle::new(Some(NonNull::new(display).expect("Got null when retrieving X11 display handle!")), screen as c_int)),
	// 		// NativeWindow::Win32 { hwnd, .. } => RawDisplayHandle::Windows(WindowsDisplayHandle::new()),
	// 		_ => return Err(HandleError::NotSupported)
	// 	})
	// }
	//
	// pub unsafe fn raw_window_handle(&self) -> Result<RawWindowHandle, HandleError> {
	// 	#[allow(unreachable_patterns)]
	// 	Ok(match self.native_window {
	// 		NativeWindow::Wayland { window, .. } => RawWindowHandle::Wayland(WaylandWindowHandle::new(NonNull::new(window).expect("Got null when retrieving Wayland window handle!"))),
	// 		NativeWindow::X11 { window, .. } => RawWindowHandle::Xlib(XlibWindowHandle::new(window as c_ulong)),
	// 		NativeWindow::Win32 { hwnd, .. } => RawWindowHandle::Win32(Win32WindowHandle::new(NonZeroIsize::new_unchecked(hwnd as isize))),
	// 		_ => return Err(HandleError::NotSupported)
	// 	})
	// }
}

// impl HasDisplayHandle for Window {
// 	fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
// 		unsafe {
// 			Ok(DisplayHandle::borrow_raw(self.raw_display_handle()?))
// 		}
// 	}
// }
//
// impl HasWindowHandle for Window {
// 	fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
// 		unsafe {
// 			Ok(WindowHandle::borrow_raw(self.raw_window_handle()?))
// 		}
// 	}
// }