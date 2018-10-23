extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

extern {
    fn appendNumberToBody(x: u32);
    fn alert(x: u32);
}

#[wasm_bindgen]
pub extern fn add_one(x: u32) -> u32 {
    x + 1
}

#[wasm_bindgen]
pub extern fn run() {
    unsafe {
        appendNumberToBody(42);
        alert(4);
    }
}
