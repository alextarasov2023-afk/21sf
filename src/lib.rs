// yaya Horizon native mod — каркас с GUI.

mod gui;

use gui::ClickGui;

static mut GUI: Option<ClickGui> = None;

#[no_mangle]
pub extern "C" fn mod_init() {
    unsafe { GUI = Some(ClickGui::new()); }
}

#[no_mangle]
pub extern "C" fn mod_tick() {
    // обновление GUI
    unsafe {
        if let Some(g) = &mut GUI {
            g.update(1.0 / 20.0);
        }
    }
}

#[no_mangle]
pub extern "C" fn mod_render() {
    unsafe {
        if let Some(g) = &GUI {
            g.render();
        }
    }
}