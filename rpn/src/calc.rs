use std::fmt;

use crate::num::Disp::{self, *};
use crate::num::Number::{self, *};
use pretty::pretty;
use Meta::*;
use Mode::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Meta {
  #[default]
  Base,
  Alt,
  Inv,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)] #[rustfmt::skip]
pub enum Mode { #[default] Main, E, Help, SelectDisp }

#[derive(Clone, Debug, PartialEq)]
pub struct JsCalls {
  pub log: fn(&str),
  pub lang: fn(&str, &str) -> String,
  pub key: fn(&str) -> String,
}

#[derive(Clone, PartialEq)]
pub struct Calc {
  pub t: Number,
  pub z: Number,
  pub y: Number,
  pub x: Number,
  pub last_x: Number,
  pub input: String,
  pub meta: Meta,
  pub mode: Mode,
  pub disp: Disp,
  pub text: String,
  pub scroll: usize,
  pub js_calls: JsCalls,
}

impl fmt::Debug for Calc {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let Calc {
      t, z, y, x, last_x, input, meta, mode, disp, text, scroll, ..
    } = self;
    let stack = format!("t {t} z {z} y {y} x {x} last_x {last_x}");
    let flags = format!("meta {meta:?} mode {mode:?} disp {disp:?}");
    let text = pretty(text.as_bytes(), 30);
    let text = format!("input `{input}` text `{text}` scroll {scroll}");

    write!(f, "Calc {{\n  {stack}\n  {flags}\n  {text}\n}}")
  }
}

impl Calc {
  pub fn new(js_calls: JsCalls) -> Self {
    let zero = Number::default();
    Calc {
      t: zero,
      z: zero,
      y: zero,
      x: zero,
      last_x: zero,
      input: String::new(),
      meta: Base,
      mode: Main,
      disp: Std,
      text: "".into(),
      scroll: 0,
      js_calls,
    }
  }

  pub fn display(&self) -> String {
    self.log(&format!("{self:?}"));

    let meta = match self.meta {
      Base => "   ",
      Alt => "ALT",
      Inv => "INV",
    };

    let lines = self.text.split('\n').collect::<Vec<_>>();
    let lines_n = lines.len();
    let scroll = self.scroll;
    let disp = self.disp;
    let shows = |i| scroll >= i && lines_n >= i;
    let check_line_len = |s: &str| {
      let n = s.chars().count();
      if n > 35 {
        self.log(&format!("Warning: `{s}` has len {n}, max. 35 is ok"));
      }
    };

    let t = format!("t {: <29} {meta}", self.t.display(disp));
    let t = if shows(5) { lines[scroll - 5] } else { &t };
    check_line_len(t);

    let z = format!("z {: <33}", self.z.display(disp));
    let z = if shows(4) { lines[scroll - 4] } else { &z };
    check_line_len(z);

    let y = format!("y {: <33}", self.y.display(disp));
    let y = if shows(3) { lines[scroll - 3] } else { &y };
    check_line_len(z);

    let empty = self.input.is_empty();
    let i = &self.input;
    let i = format!("{i}_");
    let x = self.x.display(Std);
    let x = if empty { format!("x {x: <33}") } else { format!("› {i:33}") };
    let x = if shows(2) { lines[scroll - 2] } else { &x };
    check_line_len(x);

    let s = if shows(1) { lines[scroll - 1] } else { "" };

    format!("{t}\n{z}\n{y}\n{x}\n{s}")
  }

  pub fn up_with_x(&mut self, x: Number) {
    self.t = self.z;
    self.z = self.y;
    self.y = self.x;
    self.x = x;
  }

  pub fn down_with_x(&mut self, x: Number) {
    self.x = x;
    self.y = self.z;
    self.z = self.t;
  }

  pub fn add_input(&mut self, input: &str) {
    if self.input.is_empty() {
      self.up_with_x(Simple(0.0))
    }
    self.input.push_str(input);
  }

  /// Handle command, return true to flash
  pub fn command(&mut self, command: &str) -> bool {
    self.log(&format!("Command `{command}`"));

    // On help scroll down, up or exit help
    if self.scroll > 0 {
      let scroll = &mut self.scroll;
      let lines_n = self.text.split('\n').count();
      match command {
        "ENTER" | "DEL" => *scroll = 0,
        "2" => *scroll = (*scroll).max(2) - 1,
        "0" => *scroll = (*scroll + 1).min(lines_n),
        _ => (),
      }
      self.log(&format!("scroll {}", self.scroll));
      return false;
    }

    if command.ends_with("_long") {
      self.text = (self.js_calls.lang)("en", command);
      self.scroll = if command == "ENTER_long" { 5 } else { 1 };
      return true;
    }

    // For commands in '0' ... '9' handle here, saves space in COMMANDS
    if let Some(c @ '0'..='9') = command.chars().next() {
      self.add_input(&c.to_string());
      return true;
    }

    // Else just get the command implementation fn and invoke it
    if let Some(command) = COMMANDS.get(command) {
      command(self);
      return true;
    }

    // Command not found: ignore and don't flash
    false
  }

  pub fn status(&mut self, status: &str) {
    self.scroll = 1;
    self.text = status.into();
  }

  pub fn translate_button_press(&self, index: u8, long: bool) -> String {
    let index = index as usize;
    let command = match self.meta {
      Base => BASE_BUTTONS.get(index).copied().unwrap_or_default(),
      Alt => ALT_BUTTONS.get(index).copied().unwrap_or_default(),
      Inv => INV_BUTTONS.get(index).copied().unwrap_or_default(),
    };

    format!("{command}{}", if long { "_long" } else { "" })
  }

  pub fn translate_key_press(&self, key: String) -> String {
    (self.js_calls.key)(&key)
  }

  pub fn log(&self, msg: &str) {
    (self.js_calls.log)(msg)
  }
}

pub fn parse(input: &str) -> Number {
  if input.is_empty() {
    return Simple(0.0);
  }

  if let Some(i_pos) = input.find('i') {
    if input == "i" {
      return Number::as_complex(0.0, 1.0);
    }

    let (re, im) = input.split_at(i_pos);
    let re = if re.is_empty() { 0f64 } else { re.parse().unwrap() };
    let im = &im[1..];
    let im = if im.is_empty() { 1f64 } else { im.parse().unwrap() };

    return Number::as_complex(re, im);
  }

  if let Some(slash_pos) = input.find('/') {
    let (numer, denom) = input.split_at(slash_pos);
    let numer = numer.parse().unwrap();
    let denom = denom[1..].parse().unwrap();
    return Number::as_fraction(numer, denom);
  }

  Number::Simple(input.parse().unwrap())
}

#[rustfmt::skip]
pub static BASE_BUTTONS: &[&str] = &[
  "A",       "B",       "C",       "D",       "E",       "F",
  "ENTER",   "STO",     "RCL",     "DEL",
  "ADD",     "7",       "8",       "9",
  "SUB",     "4",       "5",       "6",
  "MUL",     "1",       "2",       "3",
  "DIV",     "DOT",     "0",       "ALT",
];

#[rustfmt::skip]
pub static ALT_BUTTONS: &[&str] = &[
  "ALT_A",   "ALT_B",   "ALT_C",   "ALT_D",   "ALT_E",   "ALT_F",
  "LAST_X",  "XY",      "R_DOWN",  "UNDO",
  "E",       "SIN",     "COS",     "TAN",
  "CHS",     "LOG",     "LB",      "LN",
  "FAC",     "ROOT",    "SQRT",    "TO_HMS",
  "PERC",    "INT",     "I",       "INV",
];

#[rustfmt::skip]
pub static INV_BUTTONS: &[&str] = &[
  "INV_A",   "INV_B",   "INV_C",   "INV_D",   "INV_E",   "INV_F",
  "DISP",    "XY",      "R_DOWN",  "UNDO",
  "E",       "SIN",     "COS",     "TAN",
  "CHS",     "LOG",     "LB",      "LN",
  "FAC",     "ROOT",    "SQRT",    "TO_HMS",
  "PERC",    "INT",     "I",       "BASE",
];

// I tried to use stringify!() or paste!() to avoid duplication like this:
// "ENTER" => fn enter(...) but stringify!() is not evaluated before phf_map!
// gets it. It's a limitation of the Rust macro system and a workaround would be
// complicated. A proc macro stringify_eager!{} which replaces stringify!()
// macros by its stringified contents or something, not even clear if it is
// doable at all. Made this note not to forget this and have a useless retry.
macro_rules! commands {
  (
    $(
      $cmd:literal => fn $fn:ident($calc:ident: &mut Calc) $( $pre:ident )* {
        $( $tt:tt )+
      }
    )+
  ) => {
    {
      $(
        fn $fn($calc: &mut Calc) {
          $( $pre($calc); )*
          $( $tt )+
        }
      )+

      phf::phf_map! { $( $cmd => $fn, )+ }
    }
  }
}

// The prologue are identifiers name1 name2 ... namen and they get translated
// to name1(calc); ... namen(calc); invocations bevore the main code.
pub static COMMANDS: phf::Map<&str, fn(&mut Calc)> = commands! {
  "ENTER" => fn enter(calc: &mut Calc) base {
    if calc.input.is_empty() {
      calc.up_with_x(calc.x);
    } else {
      input_x(calc);
    }
  }

  // todo y complex then result complex
  "ADD" => fn add(calc: &mut Calc) input_x base set_last_x {
    calc.down_with_x(calc.x.add_number(calc.y));
  }

  "SUB" => fn sub(calc: &mut Calc) input_x base set_last_x {
    calc.down_with_x(calc.y.sub_number(calc.x));
  }

  "MUL" => fn mul(calc: &mut Calc) input_x base set_last_x {
    calc.down_with_x(calc.x.mul_number(calc.y));
  }

  "DIV" => fn div(calc: &mut Calc) input_x base set_last_x {
    calc.down_with_x(calc.y.div_number(calc.x));
  }

  "DOT" => fn dot(calc: &mut Calc) base {
    if let Some(dot_pos) = calc.input.find('.') {
      let (input, empty) = calc.input.split_at(dot_pos);
      if empty[1..].is_empty() {
        if input.len() > 5 {
          calc.status("Numerator too large");
          return;
        }
        calc.input = format!("{input}/");
        return;
      } else {
        calc.status("Integer part not supported");
        return;
      }
    }

    if calc.input.find('/').is_some() {
      calc.status("Already a fraction");
      return;
    }

    calc.add_input(".")
  }

  "I" => fn i(calc: &mut Calc) base { calc.add_input("i") }

  "E" => fn e(calc: &mut Calc) base {
    if calc.input.contains('/') {
      calc.status("Error: No 'e' for fractions");
      return;
    }
    let pos = if let Some(i_pos) = calc.input.find('i') { i_pos } else { 0 };
    if calc.input[pos..].contains('e') {
      calc.status("Error: Duplicate 'e'");
      return;
    }
    calc.add_input("e")
  }

  "RUP" => fn rup(calc: &mut Calc) {
    let t = calc.t;
    calc.t = calc.z;
    calc.z = calc.y;
    calc.y = calc.x;
    calc.x = t;
  }

  "RDOWN" => fn rdown(calc: &mut Calc) {
    let x = calc.x;
    calc.x = calc.y;
    calc.y = calc.z;
    calc.z = calc.t;
    calc.t = x;
  }

  "SET_LAST_X" => fn set_last_x(calc: &mut Calc) { calc.last_x = calc.x }

  "LAST_X" => fn last_x(calc: &mut Calc) base { calc.up_with_x(calc.last_x) }

  "ALT" => fn alt(calc: &mut Calc) { calc.meta = Alt }

  "INV" => fn inv(calc: &mut Calc) { calc.meta = Inv }

  "BASE" => fn base(calc: &mut Calc) { calc.meta = Base }

  "_INPUT_X" => fn input_x(calc: &mut Calc) {
    if !calc.input.is_empty() {
      calc.x = parse(&calc.input);
      calc.input.clear();
    }
  }

  "META" => fn meta(calc: &mut Calc) {
    calc.meta = match calc.meta {
      Base => Alt, Alt => Inv, Inv => Base,
    }
  }
};

#[cfg(test)]
mod tests {
  use super::*;

  fn log(s: &str) {
    println!("{s}");
  }
  fn lang(_: &str, _: &str) -> String {
    "".into()
  }
  fn key(_: &str) -> String {
    "".into()
  }

  const JS_CALLS: JsCalls = JsCalls { log, lang, key };

  #[test]
  fn test_arithmetic_simple() {
    let mut calc = Calc::new(JS_CALLS);

    calc.y = Simple(-1.0);
    calc.x = Simple(1.23456789012);
    calc.command("ADD");

    assert_eq!(calc.y, Simple(0.0));
    assert_eq!(calc.x, Simple(0.23456789012));
    assert_eq!(calc.last_x, Simple(1.23456789012));

    calc.y = Simple(2.0);
    calc.command("SUB");
    assert_eq!(calc.y, Simple(0.0));
    assert_eq!(calc.x, Simple(1.76543210988)); // todo? float cancellation?
    assert_eq!(calc.last_x, Simple(0.23456789012));

    calc.y = Simple(2.0);
    calc.command("MUL");
    assert_eq!(calc.y, Simple(0.0));
    assert_eq!(calc.x, Simple(3.53086421976));
    assert_eq!(calc.last_x, Simple(1.76543210988));

    calc.y = Simple(-3.0);
    calc.command("DIV");
    assert_eq!(calc.y, Simple(0.0));
    assert_eq!(calc.x, Simple(-0.849650344301));
    assert_eq!(calc.last_x, Simple(3.53086421976));
  }
}
