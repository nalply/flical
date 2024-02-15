use console_error_panic_hook::set_once;
use wasm_bindgen::prelude::*;

fn main() {
  set_once();
  panic!("test panic");
}
