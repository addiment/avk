use crate::backend::AvkBackend;
use avk_types::prelude::{Image, Palette};
use avk_types::{AvkGamepadInput, AvkRaw, Player, BACKGROUND_CANVAS_SIZE, MAX_SPRITES};
use std::ffi::c_void;
use std::mem;

#[no_mangle]
pub extern "C" fn avk_init(images: *const Image, palettes: *const Palette) -> *mut AvkRaw {
	// this function should probably undergo SERIOUS review...
	unsafe {
		let mut avk: Box<AvkBackend> = Box::new(AvkBackend::init(
			mem::transmute(images),
			mem::transmute(palettes),
		));
		let mut raw = Box::new(AvkRaw {
			internal: (avk.as_ref() as *const AvkBackend) as *mut c_void,
			background: [Default::default(); BACKGROUND_CANVAS_SIZE],
			foreground: [Default::default(); MAX_SPRITES],
			pan_x: 0,
			pan_y: 0,
		});
		avk.raw = raw.as_mut() as *mut AvkRaw;
		Box::leak::<'static>(avk);
		Box::leak::<'static>(raw)
	}
}

pub extern "C" fn avk_drop(avk: *mut AvkRaw) {
	unsafe {
		// TODO: make sure this actually frees the object...
		drop(Box::from_raw(avk));
	}
}

pub extern "C" fn avk_update(avk: *mut AvkRaw) -> bool {
	unsafe {
		let avk = &mut *((*avk).internal as *mut AvkBackend);
		avk.update()
	}
}

pub extern "C" fn avk_get_time(avk: *const AvkRaw) -> u64 {
	unsafe {
		let avk = &*((*avk).internal as *const AvkBackend);
		avk.get_time()
	}
}

pub extern "C" fn avk_get_input(
	avk: *const AvkRaw,
	player: Player,
	input: AvkGamepadInput,
) -> bool {
	unsafe {
		let avk = &*((*avk).internal as *const AvkBackend);
		avk.get_input(player, input)
	}
}
