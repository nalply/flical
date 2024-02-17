// some_string == "" is shorter than some_string.is_empty()
#![allow(clippy::comparison_to_empty)]

use num_complex::ComplexFloat;
use num_traits::cast::FromPrimitive;
use std::fmt;
use Disp::*;
use Meta::*;
use Number::*;

pub type Fraction = num_rational::Ratio<i64>;

pub type Complex = num_complex::Complex<f64>;

#[derive(Clone, Copy, Debug, PartialEq)] #[rustfmt::skip]
pub enum Number { Simple(f64), Fraction(Fraction), Complex(Complex) }

macro_rules! impl_number {
  (
    $(
      $method:ident( $( $param:ident: $param_ty:ty ),* ) $( -> $ty:ty )? {
        $s:pat => $s_expr:expr ,
        $f:pat => $f_expr:expr ,
        $c:pat => $c_expr:expr $(,)?
      }
    )+
  ) => {
    impl Number {
      $(
        pub fn $method(self $(, $param: $param_ty)* ) $( -> $ty )? {
          match self {
            Simple($s) => $s_expr,
            Fraction($f) => $f_expr,
            Complex($c) => $c_expr,
          }
        }
      )+
    }
  }
}

fn f2s(f: Fraction) -> f64 {
  *f.numer() as f64 / *f.denom() as f64
}

fn s2f(s: f64) -> Fraction {
  Fraction::from_f64(s).unwrap_or(Fraction::from_integer(s as i64))
}

fn s2c(s: f64) -> Complex {
  Complex { re: s, im: 0f64 }
}

fn int(s: f64) -> i64 {
  s.trunc() as i64
}

impl_number! {
  simple() -> f64 { s => s, f => f2s(f), c => c.abs() }
  fraction() -> Fraction { s => s2f(s), f => f, c => s2f(c.abs()) }
  complex() -> Complex { s => s2c(s), f => s2c(f2s(f)), c => c }
  re() -> f64 { s => s, f => f2s(f), c => c.re }
  im() -> f64 { _ => 0f64, _ => 0f64, c => c.im }
  int() -> i64 { s => int(s), f => int(f2s(f)), c => int(c.abs()) }
  numer() -> i64 { s => int(s), f => *f.numer(), c => int(c.re) }
  denom() -> i64 { _ => 1i64, f => *f.denom(), _ => 1i64 }
  add_number(rhs: Self) -> Self {
    s => Simple(s + rhs.simple()),
    f => Fraction(f + rhs.fraction()),
    c => Complex(c + rhs.complex()),
  }
}

impl Number {
  fn from_complex(re: f64, im: f64) -> Number {
    Complex(num_complex::Complex { re, im })
  }

  fn from_fraction(numer: i64, denom: i64) -> Number {
    Fraction(num_rational::Ratio::<i64>::new(numer, denom))
  }
}

impl Default for Number {
  fn default() -> Self {
    Simple(0f64)
  }
}

impl fmt::Display for Number {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Simple(simple) => write!(f, "{simple}"),
      Fraction(fraction) => write!(f, "{fraction}"),
      Complex(complex) => write!(f, "{complex}"),
    }
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)] #[rustfmt::skip]
pub enum Disp { #[default] Std, Fix(u8), Eng(u8), Sci(u8), }

#[derive(Copy, Clone, Debug, Default, PartialEq)] #[rustfmt::skip]
pub enum Meta { #[default] Base, Alt, Inv }

#[rustfmt::skip]
pub static BASE_KEYS: &[&str] = &[
  // ------- ---------- ---------- ---------- ---------- ----------
  "A",       "B",       "C",       "D",       "E",       "F",
  "ENTER",   "STO",     "RCL",     "DEL",
  "ADD",     "7",       "8",       "9",
  "SUB",     "4",       "5",       "6",
  "MUL",     "1",       "2",       "3",
  "DIV",     ".",       "0",       "ALT",
];

#[derive(Clone, Debug, PartialEq)]
pub struct State {
  pub t: Number,
  pub z: Number,
  pub y: Number,
  pub x: Number,
  pub last_x: Number,
  pub input: String,
  pub meta: Meta,
  pub disp: Disp,
  pub log: fn(&str),
}

impl State {
  pub fn new(logger: fn(&str)) -> Self {
    let zero = Number::default();
    State {
      t: zero,
      z: zero,
      y: zero,
      x: zero,
      last_x: zero,
      input: String::new(),
      meta: Base,
      disp: Std,
      log: logger,
    }
  }

  pub fn translate_button_press(&self, index: u8, long: bool) -> &'static str {
    if long {
      return "LONG";
    }
    BASE_KEYS.get(index as usize).copied().unwrap_or_default()
  }

  pub fn log(&self, msg: &str) {
    (self.log)(msg)
  }

  pub fn display(&self) -> String {
    self.log(&format!("{self:?}"));

    let empty = self.input.is_empty();
    let x = self.x;
    let input = &self.input;
    let x = if empty { format!("x {x}") } else { format!("› {input}_") };
    format!(
      "t {}\nz {}\ny {}\n{x}\n{}",
      self.t, self.z, self.y, "[ A ] [ B ] [ C ] [ D ] [ E ] [ F ]"
    )
  }

  pub fn push(&mut self, x: Number) {
    self.t = self.z;
    self.z = self.y;
    self.y = self.x;
    self.x = x;
  }

  pub fn input(&mut self, command: String) {
    if self.input == "" {
      self.push(Simple(0.0))
    }
    self.input.push_str(&command);
  }

  pub fn dot(&mut self) {
    if self.input == "" {
      self.push(Simple(0.0))
    }
    self.input.push('.');
  }

  pub fn use_input(&mut self) {
    self.x = if self.input == "" { self.x } else { parse(&self.input) };
    self.input.clear()
  }

  pub fn enter(&mut self) {
    if self.input == "" {
      self.push(self.x)
    } else {
      self.use_input();
    }
  }

  pub fn add(&mut self) {
    self.use_input();
    self.last_x = self.x;
    self.x = self.x.add_number(self.y);
    self.y = self.z;
    self.z = self.t;
  }

  // pub fn handle_press(&mut self, index: u8, _long: bool) {
  //   let key = Key::try_from(index).unwrap();

  //   match (key, self.meta) {
  //     (D0 | D1 | D2 | D3 | D4, Base) => self.digit(key),
  //     (D5 | D6 | D7 | D8 | D9, Base) => self.digit(key),
  //     (DOT, Base) => self.dot(),
  //     (ENTER, Base) => self.enter(),
  //     (ADD, Base) => self.add(),
  //     _ => (),
  //   }
  // }

  pub fn command(&mut self, command: String) {
    if let Some(c @ '0'..='9') = command.chars().next() {
      return self.input(c.to_string());
    }

    match command.as_str() {
      "." => self.dot(),
      "ENTER" => self.enter(),
      "ADD" => self.add(),
      _ => (),
    }
  }
}

pub fn parse(input: &str) -> Number {
  if let Some(i_pos) = input.find('i') {
    let (re, im) = input.split_at(i_pos);
    let re = re.parse().unwrap();
    let im = im.parse().unwrap();
    Number::from_complex(re, im)
  } else if let Some(slash_pos) = input.find('/') {
    let (numer, denom) = input.split_at(slash_pos);
    let numer = numer.parse::<i64>().unwrap();
    let denom = denom.parse::<i64>().unwrap();
    Number::from_fraction(numer, denom)
  } else {
    Number::Simple(input.parse().unwrap())
  }
}
