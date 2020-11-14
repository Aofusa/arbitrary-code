use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn sh(s: &str) -> String;
}

#[wasm_bindgen]
pub fn entry_point(_arg: &str) -> String {
    let user = sh(&format!("{}", "whoami"));
    sh(&format!("echo hello, {} !", user))
}
