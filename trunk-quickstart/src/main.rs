use console_error_panic_hook::set_once;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "
  export function consoleTest() {
    console.log('hello from JS FFI');
  }
")]
extern "C" {
  fn consoleTest();
}

fn main() {
  set_once();
  consoleTest();
}
