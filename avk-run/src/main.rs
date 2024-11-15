use crate::c_binds::{avk_drop, avk_get_input, avk_get_time, avk_init, avk_update};
use dlopen2::raw::Library;
use std::env::args;
use std::ffi::c_void;
use std::mem;
use std::path::Path;

mod backend;
mod c_binds;
mod logchamp;
mod render;
mod sdl;

fn main() {
	logchamp::init().unwrap();
	let args = args().collect::<Vec<String>>();
	let so_path_arg = &args[1];
	let so_path = Path::new(so_path_arg).canonicalize().unwrap();
	let lib = Library::open(so_path).unwrap();
	unsafe {
		// load the external function pointers
		let ext_avk_init = lib.symbol::<*const c_void>("AVK_INIT").unwrap();
		let ext_avk_drop = lib.symbol::<*const c_void>("AVK_DROP").unwrap();
		let ext_avk_update = lib.symbol::<*const c_void>("AVK_UPDATE").unwrap();
		let ext_avk_get_time = lib.symbol::<*const c_void>("AVK_GET_TIME").unwrap();
		let ext_avk_get_input = lib.symbol::<*const c_void>("AVK_GET_INPUT").unwrap();
		let ext_avk_main = lib.symbol::<*const c_void>("avk_main").unwrap();

		// write the actual in-memory function pointers to the destinations
		*(ext_avk_init as *mut *const c_void) = avk_init as *const c_void;
		*(ext_avk_drop as *mut *const c_void) = avk_drop as *const c_void;
		*(ext_avk_update as *mut *const c_void) = avk_update as *const c_void;
		*(ext_avk_get_time as *mut *const c_void) = avk_get_time as *const c_void;
		*(ext_avk_get_input as *mut *const c_void) = avk_get_input as *const c_void;

		// call the external main function
		let main = mem::transmute::<*const c_void, fn()>(ext_avk_main);
		main()
	}
}
