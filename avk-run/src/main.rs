use std::ffi::c_void;
use std::mem;
use dlopen2::raw::Library;
use crate::c_binds::{
    avk_init,
    avk_drop,
    avk_update,
    avk_get_time,
    avk_get_input,
};

mod sdl;
mod gk;
mod logchamp;
mod c_binds;
mod avk_backend;

fn main() {
    logchamp::init().unwrap();
    let lib = Library::open("./libpong.so").unwrap();
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