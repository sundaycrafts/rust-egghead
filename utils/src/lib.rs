extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
  fn appendStringToBody(s: &str);
}

#[wasm_bindgen]
pub extern fn run() {
  appendStringToBody("Hello World");
}
