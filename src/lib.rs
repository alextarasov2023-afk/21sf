// yaya Horizon native mod — overlay GUI каркас.

mod overlay;

use overlay::Overlay;

static mut OVERLAY: Option<Overlay> = None;

#[no_mangle]
pub extern "C" fn mod_init() {
    unsafe { OVERLAY = Some(Overlay::new()); }
}

#[no_mangle]
pub extern "C" fn mod_tick() {
    // overlay работает в отдельном потоке
}

#[no_mangle]
pub extern "C" fn mod_toggle_gui() {
    unsafe {
        if let Some(o) = &mut OVERLAY {
            o.toggle();
        }
    }
}