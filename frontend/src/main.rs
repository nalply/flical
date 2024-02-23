use console_error_panic_hook::set_once as init_panic_hook;
use js_sys::JsString;
use wasm_bindgen::prelude::*;
use web_sys::console;

pub fn log(msg: &str) {
  console::log_1(&msg.into());
}

fn main() {
  init_panic_hook();
  log("Rust panic hook initialized");

  let js_calls = rpn::JsCalls { log, lang, key };
  putFlicalSingleton(Flical(rpn::Calc::new(js_calls)));
}

#[wasm_bindgen(inline_js = "
export function key(key) { 
  return window.flical?.keys?.[key] ?? ''
}
")]
extern "C" {
  pub fn key(key: &str) -> String;
}

#[wasm_bindgen(inline_js = "
export function lang(lang, id) { 
  return window.flical?.lang?.[lang]?.[id] ?? ''
}
")]
extern "C" {
  pub fn lang(lang: &str, id: &str) -> String;
}

#[wasm_bindgen(inline_js = "
export function updateScreen(contents) {
  document.querySelector('#screen').innerText = contents
}
")]
extern "C" {
  pub fn updateScreen(contents: JsString);
}

#[wasm_bindgen(inline_js = "
export function flashScreen() {
  let screen = window.document.querySelector('#screen')
  screen.classList.add('dim')
  window.setTimeout(_ => screen.classList.remove('dim'), 100)
}
")]
extern "C" {
  pub fn flashScreen();
}

// putFlicalSingleton() and takeFlicalSingleton() are helpers to avoid OnceCell
// on Rust side. The Flical JavaScript class is an opaque wrapper around the
// state of the flical calculator.
#[wasm_bindgen(inline_js = "
export function putFlicalSingleton(flical) {
  if (!window.flical) window.flical = {}
  window.flical.singleton = flical
}
")]
extern "C" {
  pub fn putFlicalSingleton(singleton: Flical);
}

// Take out the singleton. Don't forget to use putFilcalGingleton() to put it
// back. If you don't you get something like a null pointer exception, even
// if you still see the object window.flical in JavaScript.
#[wasm_bindgen(inline_js = "
export function takeFlicalSingleton() {
  return window.flical.singleton  
}
")]
extern "C" {
  pub fn takeFlicalSingleton() -> Flical;
}

// The Flical singleton (handled by JavaScript)
#[wasm_bindgen]
struct Flical(rpn::Calc);

#[wasm_bindgen]
pub fn flical_translate_button_press(index: u8, long: bool) -> JsString {
  let flical = takeFlicalSingleton();
  let command = flical.0.translate_button_press(index, long);
  putFlicalSingleton(flical);

  command.into()
}

#[wasm_bindgen]
pub fn flical_translate_key_press(key: JsString) -> JsString {
  let flical = takeFlicalSingleton();
  let command = flical.0.translate_key_press(key.into());
  putFlicalSingleton(flical);

  command.into()
}

#[wasm_bindgen]
pub fn flical_command(command: String) {
  if !command.is_empty() {
    let mut flical = takeFlicalSingleton();
    if flical.0.command(&command) {
      flashScreen();
    }
    updateScreen(flical.0.display().into());
    putFlicalSingleton(flical);
  }
}

// Copyright see AUTHORS & LICENSE; SPDX-License-Identifier: ISC+
