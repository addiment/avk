use std::ffi::c_void;
use std::mem;
use std::slice::from_raw_parts;
use dlopen2::raw::Library;
use avk_run::Avk;
use avk_types::{AvkRaw, GamepadInput, Player, BACKGROUND_CANVAS_SIZE, MAX_SPRITES};
use avk_types::prelude::{Image, Palette};

#[no_mangle]
extern "C" fn avk_init(images: *const Image, palettes: *const Palette) -> *mut AvkRaw {
    unsafe {
        let mut avk: Box<Avk> = Box::new(Avk::init(
            mem::transmute(images),
            mem::transmute(palettes)
        ));
        let mut raw = Box::new(AvkRaw {
            internal: (avk.as_ref() as *const Avk) as *mut c_void,
            background: [Default::default(); BACKGROUND_CANVAS_SIZE],
            foreground: [Default::default(); MAX_SPRITES],
        });
        avk.raw = raw.as_mut() as *mut AvkRaw;
        Box::leak::<'static>(avk);
        Box::leak::<'static>(raw)
    }
}

extern "C" fn avk_drop(avk: *mut AvkRaw) {
    unsafe {
        // TODO: free the think
        drop(Box::from_raw(avk));
    }
}

extern "C" fn avk_update(avk: *mut AvkRaw) -> bool {
    unsafe { // println!("avk update! {:x}", avk as usize);
        let avk = &mut *((*avk).internal as *mut Avk);
        avk.update()
    }
}

extern "C" fn avk_get_time(avk: *const AvkRaw) -> u64 {
    // println!("avk get time! {:x}", avk as usize);
    unsafe {
        let avk = &*((*avk).internal as *const Avk);
        avk.get_time()
    }
}

extern "C" fn avk_get_input(avk: *const AvkRaw, player: Player, input: GamepadInput) -> bool {
    println!("avk get input! {:x}", avk as usize);
    false
}

fn main() {
    let lib = Library::open("./libpong.so").unwrap();
    unsafe {
        let ext_avk_init = lib.symbol::<*const c_void>("AVK_INIT").unwrap();
        let ext_avk_drop = lib.symbol::<*const c_void>("AVK_DROP").unwrap();
        let ext_avk_update = lib.symbol::<*const c_void>("AVK_UPDATE").unwrap();
        let ext_avk_get_time = lib.symbol::<*const c_void>("AVK_GET_TIME").unwrap();
        let ext_avk_get_input = lib.symbol::<*const c_void>("AVK_GET_INPUT").unwrap();
        let ext_avk_main = lib.symbol::<*const c_void>("avk_main").unwrap();

        *(ext_avk_init as *mut *const c_void) = avk_init as *const c_void;
        *(ext_avk_drop as *mut *const c_void) = avk_drop as *const c_void;
        *(ext_avk_update as *mut *const c_void) = avk_update as *const c_void;
        *(ext_avk_get_time as *mut *const c_void) = avk_get_time as *const c_void;
        *(ext_avk_get_input as *mut *const c_void) = avk_get_input as *const c_void;

        let main = mem::transmute::<*const c_void, fn()>(ext_avk_main);
        main()
    }
}