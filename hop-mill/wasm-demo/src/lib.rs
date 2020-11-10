// use wasm-pack build --target web to compile
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-demo!");
}

#[wasm_bindgen]
// TODO: eventually take in user's name and display Thanks {name} ...
// confirm(name: &str)
// alert(&format!("... {} ...", name));
pub fn confirm() {
    alert("Thanks! We will contact you as soon as possible!");
}
