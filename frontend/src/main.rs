use console_error_panic_hook::set_once as init_panic_hook;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{console, window, Event};

fn main() {
  init_panic_hook();

  console::log_1(&"Rust WASM main() started".into());

  let document = window().and_then(|win| win.document()).unwrap();
  let buttons = document.query_selector_all("button").unwrap();
  for i in 0..buttons.length() {
    let closure = Closure::wrap(
      Box::new(move |_: Event| handle_button_click(i)) as Box<dyn FnMut(_)>,
    );
    let button_click = closure.as_ref().unchecked_ref();
    let button = buttons.get(i).unwrap();
    button.add_event_listener_with_callback("click", button_click).unwrap();
    closure.forget();
  }
}

fn handle_button_click(index: u32) {
  console::log_2(&"button no".into(), &JsValue::from_f64(index as f64));
}
