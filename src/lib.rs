// yaya Horizon native mod — overlay GUI.

mod overlay;

use overlay::Overlay;
use std::thread;

static mut OVERLAY: Option<Overlay> = None;
static mut OVERLAY_THREAD: Option<thread::JoinHandle<()>> = None;

#[no_mangle]
pub extern "C" fn mod_init() {
    unsafe {
        let mut overlay = Overlay::new();
        overlay.open = true;
        let handle = thread::spawn(move || {
            overlay.run();
        });
        OVERLAY = Some(overlay);
        OVERLAY_THREAD = Some(handle);
    }
}

#[no_mangle]
pub extern "C" fn mod_toggle_gui() {
    unsafe {
        if let Some(o) = &mut OVERLAY {
            o.toggle();
        }
    }
}