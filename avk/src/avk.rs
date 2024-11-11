use core::ffi::c_void;
use core::mem;
use core::ptr::null;
use crate::{BACKGROUND_CANVAS_SIZE, MAX_IMAGES, MAX_PALETTES, MAX_SPRITES};
use crate::prelude::*;

#[repr(C)]
pub struct AvkRaw {
	pub internal: *mut c_void,
	pub background: [Tile; BACKGROUND_CANVAS_SIZE],
	pub foreground: [Sprite; MAX_SPRITES],
}

/// Rust wrapper around the C ABI to use the raw Rust library.
pub struct Avk {
	raw: *mut AvkRaw,
}

// #[no_mangle]
// extern "C" static mut AVK_INIT_FP

#[no_mangle]
pub static mut AVK_INIT: *const c_void = null();
#[no_mangle]
pub static mut AVK_DROP: *const c_void = null();
#[no_mangle]
pub static mut AVK_UPDATE: *const c_void = null();
#[no_mangle]
pub static mut AVK_GET_TIME: *const c_void = null();
#[no_mangle]
pub static mut AVK_GET_INPUT: *const c_void = null();

impl Avk {
	pub fn init(images: [Image; MAX_IMAGES], palettes: [Palette; MAX_PALETTES]) -> Self {
		unsafe {
			let avk_init = mem::transmute::<*const c_void, fn(images: *const [u8; Image::PIXEL_COUNT], palettes: *const [u16; 16]) -> *mut AvkRaw>(AVK_INIT);
			let raw = avk_init(
				images.map(|e| e.0).as_mut_ptr(),
				palettes.map(|e| e.0).as_mut_ptr()
			);
			Self { raw }
		}
	}

	pub fn update(&mut self) -> bool {
		unsafe {
			mem::transmute::<*const c_void, extern "C" fn(avk: *mut AvkRaw) -> bool>(AVK_UPDATE)(self.raw)
			// avk_update(self.raw)
		}
	}

	pub fn get_input(&self, player: Player, input: GamepadInput) -> bool {
		unsafe {
			mem::transmute::<*const c_void, extern "C" fn(avk: *const AvkRaw, player: Player, input: GamepadInput) -> bool>(AVK_GET_INPUT)(self.raw, player, input)
			// avk_get_input(self.raw, player, input)
		}
	}

	/// Returns the current time, in milliseconds.
	pub fn get_time(&self) -> u64 {
		unsafe {
			mem::transmute::<*const c_void, extern "C" fn(avk: *const AvkRaw) -> u64>(AVK_GET_TIME)(self.raw)
			// avk_get_time(self.raw)
		}
	}

	pub fn get_foreground(&mut self) -> &mut [Sprite; MAX_SPRITES] {
		unsafe {
			&mut (*self.raw).foreground
		}
	}

	pub fn get_background(&mut self) -> &mut [Tile; BACKGROUND_CANVAS_SIZE] {
		unsafe {
			&mut (*self.raw).background
		}
	}
}

impl Drop for Avk {
	fn drop(&mut self) {
		unsafe {
			mem::transmute::<*const c_void, extern "C" fn(avk: *mut AvkRaw)>(AVK_DROP)(self.raw)
			// avk_drop(self.raw);
		}
	}
}