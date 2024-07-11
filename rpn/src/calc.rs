use std::{fmt, mem};

use crate::Num;
use crate::NumDisplay::{self, *};
use pretty::pretty;

#[derive(Copy, Clone, Debug, Default, PartialEq)] #[rustfmt::skip]
pub enum DispState { #[default] DispStart, DispFix, DispSci, DispHex }
use DispState::*;

#[derive(Copy, Clone, Debug, Default, PartialEq)] #[rustfmt::skip]
pub enum State { #[default] Base, Alt, Inv, Sto, Rcl, Disp(DispState) }
use State::*;

#[derive(Clone, Debug, PartialEq)]
pub struct JsCalls {
  pub log: fn(&str),
  pub lang: fn(&str, &str) -> String,
  pub key: fn(&str) -> String,
}

#[derive(Clone, PartialEq)]
pub struct Calc {
  pub t: Num,
  pub z: Num,
  pub y: Num,
  pub x: Num,
  pub input: String,
  pub state: State,
  pub disp: NumDisplay,
  pub text: String,
  pub scroll: usize,
  pub js_calls: JsCalls,
}

impl fmt::Debug for Calc {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let Calc { t, z, y, x, input, state: meta, disp, text, scroll, .. } = self;
    let stack = format!("t {t:?} z {z:?} y {y:?} x {x:?}");
    let text = pretty(text.as_bytes(), 30);
    let text = format!("{meta:?} {disp:?} `{input}` `{text}` scroll {scroll}");

    write!(f, "Calc {{\n  {stack}\n  {text}\n}}")
  }
}

impl Calc {
  pub fn new(js_calls: JsCalls) -> Self {
    let zero = Num::default();
    Calc {
      t: zero,
      z: zero,
      y: zero,
      x: zero,
      input: String::new(),
      state: Base,
      disp: Std,
      text: "".into(),
      scroll: 0,
      js_calls,
    }
  }

  pub fn display(&self) -> String {
    self.log(&format!("{self:?}"));

    let meta = match self.state {
      Alt => "ALT",
      Inv => "INV",
      Sto => "STO",
      Rcl => "RCL",
      _ => "    ",
    };

    let lines = self.text.split('\n').collect::<Vec<_>>();
    let lines_n = lines.len();
    let scroll = self.scroll.min(lines_n - 1);

    let disp = self.disp;
    let shows = |i| scroll >= i && lines_n > i;
    let check_line_len = |s: &str| {
      let n = s.chars().count();
      if n > 35 {
        self.log(&format!("Warning: `{s}` has len {n}, max. 35 is ok"));
      }
    };

    let t = format!("{: <29} {meta}", self.t.disp(disp));
    let t = if shows(4) { lines[scroll - 4] } else { &t };
    check_line_len(t);

    let z = format!("{: <33}", self.z.disp(disp));
    let z = if shows(3) { lines[scroll - 3] } else { &z };
    check_line_len(z);

    let y = format!("{: <33}", self.y.disp(disp));
    let y = if shows(2) { lines[scroll - 2] } else { &y };
    check_line_len(z);

    let empty = self.input.is_empty();
    let i = &self.input;
    let i = format!("{i}_");
    let x = self.x.disp(disp);
    let x = if empty { format!("{x: <33}") } else { format!("› {i:33}") };
    let x = if shows(1) { lines[scroll - 1] } else { &x };
    check_line_len(x);

    let s = if shows(0) { lines[scroll] } else { "" };

    format!("{t}\n{z}\n{y}\n{x}\n{s}")
  }

  pub fn up_with_x(&mut self, x: Num) {
    self.t = self.z;
    self.z = self.y;
    self.y = self.x;
    self.x = x;
  }

  pub fn down_with_x(&mut self, x: Num) {
    self.x = x;
    self.y = self.z;
    self.z = self.t;
  }

  pub fn add_input(&mut self, input: &str) {
    if self.input.is_empty() {
      self.up_with_x(Num::ZERO)
    }
    self.input.push_str(input);
  }

  /// DISP spans up a state machine consisting of Dsp* states.
  /// Handle DISP by switching between these states, for example in DispStart
  /// A sets display mode to Std, B switches to DspFix state, etc.
  pub fn handle_disp(&mut self, command: &str) -> bool {
    let disp_state = if let Disp(disp_state) = self.state {
      disp_state
    } else {
      // Not DISP? Return false to continue command handling in parent function
      return false;
    };
    let command_bytes = command.as_bytes();
    let digit = command_bytes[0].max(b'0') - b'0';

    #[derive(Clone, Copy, Debug, PartialEq)] #[rustfmt::skip]
    enum Action { Stay, Current, Error, Set }
    use Action::*;

    let mut action = Stay;
    let mut set_display = |display| {
      self.disp = display;
      action = Set;
    };

    match (disp_state, command_bytes) {
      (DispStart, b"A") => set_display(Std),
      (DispStart, b"B") => self.state = Disp(DispFix),
      (DispStart, b"C") => self.state = Disp(DispSci),
      (DispStart, b"D") => self.state = Disp(DispHex),
      (DispStart, b"E") => set_display(Raw),
      (DispFix, [b'0'..=b'9']) => set_display(Fix(digit)),
      (DispSci, [b'0'..=b'9']) => set_display(Sci(digit)),
      (DispHex, b"A") => set_display(HexU),
      (DispHex, b"B") => set_display(HexL),
      (_, b"F") => action = Current,
      (_, _) => action = Error,
    }

    self.log(&format!("state {:?} action {:?}", disp_state, action));

    let disp_state = if let Disp(d) = self.state { d } else { DispStart };
    let status = match (disp_state, action) {
      (DispFix | DispSci, Stay) => "Enter digit for precision      Show".into(),
      (DispHex, Stay) => "Upper Lower                    Show".into(),
      (_, Set) => format!("Display set to: {:?}", self.disp),
      (_, Current) => format!("Display is: {:?}", self.disp),
      (_, Error) => format!("Bad key? Display is: {:?}", self.disp),
      (_, _) => unreachable!(),
    };
    self.status(&status);

    if action != Stay {
      self.state = Base;
    }

    true
  }

  /// Handle command, return true to flash
  pub fn handle_command(&mut self, command: &str) -> bool {
    self.log(&format!("Command `{command}`"));

    if self.handle_disp(command) {
      return true;
    }

    // On help scroll down, up or exit help
    let lines_n = self.text.split('\n').count();
    if lines_n > 1 {
      match command {
        "2" | "R_UP" => self.scroll = self.scroll.max(1) - 1,
        "0" | "R_DOWN" => self.scroll = self.scroll.min(lines_n.max(2) - 2) + 1,
        _ => self.status(""), // todo Esc key
      }
    }

    // Exit handling if showing help longer than one line
    if self.text.split('\n').count() > 1 {
      return false;
    }

    self.status("");

    if command.ends_with("_long") {
      self.text = (self.js_calls.lang)("en", command);
      self.scroll = if command == "ENTER_long" { 4 } else { 1 };
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

    self.log(&format!("Ignoring command {command}"));
    false
  }

  pub fn status(&mut self, status: &str) {
    self.scroll = 0;
    self.text = status.into();
  }

  pub fn button_from(buttons: &[&'static str], index: u8) -> &'static str {
    buttons.get(index as usize).copied().unwrap_or_default()
  }

  pub fn translate_button_press(&self, index: u8, long: bool) -> String {
    let command = match &self.state {
      Alt => Self::button_from(ALT_BUTTONS, index),
      Inv => Self::button_from(INV_BUTTONS, index),
      _ => Self::button_from(BASE_BUTTONS, index),
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
  "DISP",    "XY",      "R_DOWN",  "UNDO",
  "EDATA",   "SIN",     "COS",     "TAN",
  "CHS",     "LN",      "LD",      "LOG",
  "FAC",     "ROOT",    "SQRT",    "TO_HMS",
  "RECIP",    "INT",     "I",       "INV",
];

#[rustfmt::skip]
pub static INV_BUTTONS: &[&str] = &[
  "INV_A",   "INV_B",   "INV_C",   "INV_D",   "INV_E",   "INV_F",
  "MACRO",   "XZ",      "R_UP",    "REDO",
  "RAND",    "ASIN",    "ACOS",    "ATAN",
  "ABS",     "EXP",     "LB",      "H",
  "DPERC",   "POW",     "SQR",     "TO_H",
  "PERC",    "FRAC",    "ABS",     "BASE",
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
  "E" => fn e(calc: &mut Calc) base {
    if calc.input.contains('/') {
      calc.status("Error: No 'e' for fractions");
      return;
    }
    let pos = calc.input.find('i').unwrap_or_default();
    if calc.input[pos..].contains('e') {
      calc.status("Error: Duplicate 'e'");
      return;
    }
    calc.add_input("e")
  }

  "ENTER" => fn enter(calc: &mut Calc) base {
    if calc.input.is_empty() {
      calc.up_with_x(calc.x);
    } else {
      input_x(calc);
    }
  }

  "DISP" => fn disp(calc: &mut Calc) {
    calc.state = Disp(DispStart);
    calc.status(" Std   Fix   Sci   Hex   Raw   Show");
  }

  "STO" => fn sto(calc: &mut Calc) {
    calc.state = Sto;
  }

  "XY" => fn xy(calc: &mut Calc) input_x base {
    mem::swap(&mut calc.x, &mut calc.y);
  }

  "XZ" => fn xz(calc: &mut Calc) input_x base {
    mem::swap(&mut calc.x, &mut calc.z);
  }

  "RCL" => fn rcl(calc: &mut Calc) {
    calc.state = Rcl;
  }

  "R_UP" => fn rup(calc: &mut Calc) input_x base {
    mem::swap(&mut calc.t, &mut calc.z);
    mem::swap(&mut calc.z, &mut calc.y);
    mem::swap(&mut calc.y, &mut calc.x);
  }

  "R_DOWN" => fn rdown(calc: &mut Calc) input_x base {
    mem::swap(&mut calc.y, &mut calc.x);
    mem::swap(&mut calc.z, &mut calc.y);
    mem::swap(&mut calc.t, &mut calc.z);
  }

  "DEL" => fn del(calc: &mut Calc) {
    if calc.input.is_empty() && calc.x != Num::ZERO {
      calc.x = Num::ZERO;
    } else {
      let rev_take = calc.input.chars().rev().skip(1).collect::<String>();
      calc.input = rev_take.chars().rev().collect();
    }
  }

  "ADD" => fn add(calc: &mut Calc) input_x base {
    calc.down_with_x(calc.y.add_num(calc.x));
  }

  "SIN" => fn sin(calc: &mut Calc) input_x base {
    calc.x = calc.x.sin();
  }

  "ASIN" => fn asin(calc: &mut Calc) input_x base {
    calc.x = calc.x.asin();
  }

  "COS" => fn cos(calc: &mut Calc) input_x base {
    calc.x = calc.x.cos();
  }

  "ACOS" => fn acos(calc: &mut Calc) input_x base {
    calc.x = calc.x.acos();
  }

  "TAN" => fn tan(calc: &mut Calc) input_x base {
    calc.x = calc.x.tan();
  }

  "ATAN" => fn atan(calc: &mut Calc) input_x base {
    calc.x = calc.x.atan();
  }

  "SUB" => fn sub(calc: &mut Calc) input_x base {
    calc.down_with_x(calc.y.sub_num(calc.x));
  }

  "CHS" => fn chs(calc: &mut Calc) input_x base {
    calc.x = calc.x.chs();
  }

  "ABS" => fn abs(calc: &mut Calc) input_x base {
    calc.x = calc.x.abs();
  }

  "LD" => fn ld(calc: &mut Calc) input_x base {
    calc.x = calc.x.ld();
  }

  "LOG" => fn log(calc: &mut Calc) input_x base {
    calc.down_with_x(calc.y.log(calc.x));
  }

  "MUL" => fn mul(calc: &mut Calc) input_x base {
    calc.down_with_x(calc.x.mul_num(calc.y));
  }

  "LB" => fn lb(calc: &mut Calc) input_x base {
    calc.x = calc.x.lb();
  }

  "POW" => fn pow(calc: &mut Calc) input_x base {
    calc.down_with_x(calc.y.pow(calc.x));
  }

  "RECIP" => fn recip(calc: &mut Calc) input_x base {
    calc.x = calc.x.recip();
  }

  "ROOT" => fn root(calc: &mut Calc) input_x base {
    calc.down_with_x(calc.y.root(calc.x));
  }

  "DIV" => fn div(calc: &mut Calc) input_x base {
    calc.down_with_x(calc.y.div_num(calc.x));
  }

  "INT" => fn int(calc: &mut Calc) input_x base {
    calc.x = calc.x.int();
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

  "FRAC" => fn frac(calc: &mut Calc) input_x base {
    calc.x = calc.x.frac();
  }

  "I" => fn i(calc: &mut Calc) base { calc.add_input("i") }

  "ROUND" => fn round(calc: &mut Calc) input_x base {
    calc.x = calc.x.round();
  }

  "ALT" => fn alt(calc: &mut Calc) { calc.state = Alt }

  "INV" => fn inv(calc: &mut Calc) { calc.state = Inv }

  "BASE" => fn base(calc: &mut Calc) { calc.state = Base }

  "_INPUT_X" => fn input_x(calc: &mut Calc) {
    if !calc.input.is_empty() {
      calc.x = Num::decode(&calc.input);
      calc.input.clear();
    }
  }

  "META" => fn meta(calc: &mut Calc) {
    calc.state = match calc.state {
      Base => Alt, Alt => Inv, _ => Base,
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

    calc.y = Num::from_r(-1.0);
    calc.x = Num::from_r(1.23456789012);
    calc.handle_command("ADD");

    assert_eq!(calc.y, Num::from_r(0.0));
    assert_eq!(calc.x, Num::from_r(0.23456789012));

    calc.y = Num::from_r(2.0);
    calc.handle_command("SUB");
    assert_eq!(calc.y, Num::from_r(0.0));
    assert_eq!(calc.x, Num::from_r(1.76543210988)); // todo? float cancellation?

    calc.y = Num::from_r(2.0);
    calc.handle_command("MUL");
    assert_eq!(calc.y, Num::from_r(0.0));
    assert_eq!(calc.x, Num::from_r(3.53086421976));

    calc.y = Num::from_r(-3.0);
    calc.handle_command("DIV");
    assert_eq!(calc.y, Num::from_r(0.0));
    assert_eq!(calc.x, Num::from_r(-0.84965034430124));
  }
}
