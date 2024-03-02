use num_complex::ComplexFloat;
use num_traits::cast::FromPrimitive;
use num_traits::Signed;
use std::fmt;
use Num::*;

#[derive(Copy, Clone, Debug, Default, PartialEq)] #[rustfmt::skip]
pub enum Disp { #[default] Std, Fix(u8), Sci(u8), }

pub type Z = i64;

pub type R = f64;

pub type Q = num_rational::Ratio<Z>;

pub type C = num_complex::Complex<R>;

#[derive(Clone, Copy, Debug, PartialEq)] #[rustfmt::skip]
pub enum Num { Real(R), Quotient(Q), Complex(C) }

pub const ZERO: Num = Real(0.0);

fn q2r(q: Q) -> R {
  p(*q.numer() as f64 / *q.denom() as f64)
}

fn r2q(r: R) -> Q {
  Q::from_f64(r).unwrap_or(Q::from_integer(r as i64))
}

fn r2c(r: R) -> C {
  C { re: r, im: 0f64 }
}

fn inti(r: R) -> Z {
  r.trunc() as i64
}

// round to 12 decimal digits of precision and have at most +/-299 as exponent
fn p(r: R) -> R {
  if r.is_nan() {
    return f64::NAN;
  }
  if r.is_infinite() {
    return r;
  }

  // Convert f64 to string with with 12 decimal digits precision
  let s: &str = &format!("{r:.11e}");
  let inf = f64::INFINITY.copysign(r.signum());

  // If exponent >= 300, positive infinity, if <= -300, negative infinity
  if let Some(e_pos) = s.find('e') {
    let (_, exponent) = s.split_at(e_pos + 1);
    if exponent.len() >= 3 {
      let mut chars = exponent.chars();
      match chars.next() {
        Some('3') => return inf,
        Some('-') => {
          if let Some('3') = chars.next() {
            return 0.0;
          }
        } // no -0.0
        _ => (),
      }
    }
  }

  // Convert back to f64
  s.parse().unwrap()
}

fn display(r: R, disp: Disp) -> String {
  if r.is_nan() {
    return "? (not a number)".into();
  }
  if r.is_infinite() {
    return format!("{}oo (infinity)", if r < 0.0 { "-" } else { "" });
  }

  match disp {
    Disp::Std => {
      let r = p(r);
      let with_negative = (r < 0.0) as usize;
      let with_frac = 2 * (r.fract().abs() > 0.0) as usize;
      let s = format!("{r}");
      if s.len() > 12 + with_negative + with_frac {
        format!("{r:e}")
      } else {
        s
      }
    }
    _ => todo!(),
  }
}

macro_rules! impl_num_methods {
  (
    $(
      $method:ident( $( $param:ident: $param_ty:ty ),* ) $( -> $ty:ty )? {
        $r:pat => $r_expr:expr ,
        $q:pat => $q_expr:expr ,
        $c:pat => $c_expr:expr $(,)?
      }
    )+
  ) => {
    impl Num {
      $(
        pub fn $method(self $(, $param: $param_ty)* ) $( -> $ty )? {
          match self {
            Real($r) => $r_expr,
            Quotient($q) => $q_expr,
            Complex($c) => $c_expr,
          }
        }
      )+
    }
  }
}

impl_num_methods! {
  as_r() -> R { r => p(r), q => q2r(q), c => c.abs() }
  as_q() -> Q { r => r2q(r), q => q, c => r2q(c.abs()) }
  as_c() -> C { r => r2c(r), q => r2c(q2r(q)), c => c }
  re() -> R { r => p(r), q => q2r(q), c => c.re }
  im() -> R { _ => 0f64, _ => 0f64, c => c.im }
  inti() -> Z { r => inti(r), q => inti(q2r(q)), c => inti(c.abs()) }
  numer() -> Z { r => inti(r), q => *q.numer(), c => inti(c.abs()) }
  denom() -> Z { _ => 1i64, q => *q.denom(), _ => 1i64 }
  is_c() -> bool { _ => false, _ => false, c => c.im() != 0.0 }
  is_q() -> bool { _ => false, q => *q.denom() != 1, _ => false }
  display(disp: Disp) -> String {
    r => display(r, disp),
    q => format!("{q}"),
    c => format!("{c}"),
  }
  frac() -> Self {
    r => Real(r.fract()),
    q => Quotient(q.fract()),
    c => Real(c.abs().fract()),
  }
  int() -> Self {
    r => Real(r.trunc()),
    q => Quotient(q.trunc()),
    c => Real(c.abs().trunc()),
  }
  abs() -> Self {
    r => Real(r.abs()),
    q => Quotient(q.abs()),
    c => Real(c.abs()),
  }
  round() -> Self {
    r => Real(r.round()),
    q => Quotient(q.round()),
    c => Real(c.abs().round()),
  }
  is_nan() -> bool {
    r => r.is_nan(),
    _ => false,
    c => c.is_nan(),
  }
}

macro_rules! impl_binary_ops_with_fix {
  (
    $( pub fn $name:ident(self, rhs: Self) -> Self { $op:tt } )+
  ) => {
    impl Num {
      $(
        pub fn $name(self, rhs: Self) -> Self {
          if self.is_c() || rhs.is_c() {
            Complex(self.as_c() $op rhs.as_c()).fix()
          } else if self.is_q() || rhs.is_q() {
            Quotient(self.as_q() $op rhs.as_q()).fix()
          } else {
            Real(self.as_r() $op rhs.as_r())
          }
        }
      )+
    }
  }
}

impl_binary_ops_with_fix! {
  pub fn add_num(self, rhs: Self) -> Self { + }
  pub fn sub_num(self, rhs: Self) -> Self { - }
  pub fn mul_num(self, rhs: Self) -> Self { * }
  pub fn div_num(self, rhs: Self) -> Self { / }
}

impl Num {
  pub fn complex(re: f64, im: f64) -> Self {
    Complex(num_complex::Complex { re, im })
  }

  pub fn fraction(numer: i64, denom: i64) -> Self {
    Quotient(num_rational::Ratio::<i64>::new(numer, denom))
  }

  pub fn pow(self, exp: Self) -> Self {
    Complex(self.as_c().powc(exp.as_c())).fix()
  }

  pub fn root(self, root: Self) -> Self {
    Complex(self.as_c().powc(1.0 / root.as_c())).fix()
  }

  // If a complex has im == 0 or fraction denom == 1 then convert to Simple
  pub fn fix(self) -> Self {
    if self.im() == 0.0 && self.denom() == 1 {
      Real(self.re())
    } else {
      self
    }
  }
}

impl Default for Num {
  fn default() -> Self {
    Real(0f64)
  }
}

impl fmt::Display for Num {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Real(simple) => write!(f, "{simple}"),
      Quotient(fraction) => write!(f, "{fraction}"),
      Complex(complex) => write!(f, "{complex}"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fn_p() {
    let x = 0.12345678901249; // the last two digits 49 should be truncated
    let ten = 10.0f64;
    let inf = f64::INFINITY;
    let numbers = [
      0.1,
      1.1 - 1.0,
      1.0 / 3.0,
      x * ten.powf(11.),
      x * ten.powf(12.),
      x * ten.powf(200.),
      x * ten.powf(300.),
      x * ten.powf(301.),
      -x * ten.powf(305.),
      x * ten.powf(-4.),
      x * ten.powf(-12.),
      x * ten.powf(-200.),
      x * ten.powf(-298.),
      x * ten.powf(-299.),
      -x * ten.powf(-304.),
      inf,
      -inf,
    ];
    let numbers2 = [
      0.1,
      0.1,
      0.333333333333,
      12345678901.2,
      1.23456789012e11,
      1.23456789012e199,
      1.23456789012e299,
      inf,
      -inf,
      1.23456789012e-5,
      1.23456789012e-13,
      1.23456789012e-201,
      1.23456789012e-299,
      0.0,
      0.0,
      inf,
      -inf,
    ];
    assert_eq!(numbers.len(), numbers2.len());
    for i in 0..numbers.len() {
      assert_eq!(p(numbers[i]), numbers2[i], "#{i}: {:e}", numbers[i]);
    }

    assert!(p(f64::NAN).is_nan());
  }

  #[test]
  fn test_precision_12_digits_display() {
    // Unchanged Std
    assert_eq!(display(0.123456789012, Disp::Std), "0.123456789012");
    assert_eq!(display(-0.123456789012, Disp::Std), "-0.123456789012");
    assert_eq!(display(1.23456789012, Disp::Std), "1.23456789012");
    assert_eq!(display(-1.23456789012, Disp::Std), "-1.23456789012");

    // Unchanged Sci
    assert_eq!(display(1.23456789012e12, Disp::Std), "1.23456789012e12");
    assert_eq!(display(-1.23456789012e12, Disp::Std), "-1.23456789012e12");
    assert_eq!(display(1.23456789012e-2, Disp::Std), "1.23456789012e-2");
    assert_eq!(display(-1.23456789012e-2, Disp::Std), "-1.23456789012e-2");

    // Round down / up for negative
    assert_eq!(display(0.1234567890121, Disp::Std), "0.123456789012");
    assert_eq!(display(-0.1234567890121, Disp::Std), "-0.123456789012");
    assert_eq!(display(1.234567890121, Disp::Std), "1.23456789012");
    assert_eq!(display(-1.234567890121, Disp::Std), "-1.23456789012");

    // Round to even
    assert_eq!(display(0.1234567890125, Disp::Std), "0.123456789012");
    assert_eq!(display(-0.1234567890125, Disp::Std), "-0.123456789012");
    assert_eq!(display(0.1234567890135, Disp::Std), "0.123456789014");
    assert_eq!(display(-0.1234567890135, Disp::Std), "-0.123456789014");
    assert_eq!(display(1.234567890125, Disp::Std), "1.23456789012");
    assert_eq!(display(-1.234567890125, Disp::Std), "-1.23456789012");

    // Todo: why does this round down instead to even?
    assert_eq!(display(1.234567890135, Disp::Std), "1.23456789013");
    assert_eq!(display(-1.234567890135, Disp::Std), "-1.23456789013");

    // Round up / down for negative
    assert_eq!(display(0.12345678901251, Disp::Std), "0.123456789013");
    assert_eq!(display(-0.12345678901251, Disp::Std), "-0.123456789013");
    assert_eq!(display(0.12345678901299, Disp::Std), "0.123456789013");
    assert_eq!(display(-0.12345678901299, Disp::Std), "-0.123456789013");
    assert_eq!(display(1.2345678901251, Disp::Std), "1.23456789013");
    assert_eq!(display(-1.2345678901251, Disp::Std), "-1.23456789013");
    assert_eq!(display(1.2345678901299, Disp::Std), "1.23456789013");
    assert_eq!(display(-1.2345678901299, Disp::Std), "-1.23456789013");

    // Sci to Std
    assert_eq!(display(1.23456789012e0, Disp::Std), "1.23456789012");
    assert_eq!(display(1.23456789012e11, Disp::Std), "123456789012");
    assert_eq!(display(1.23456789012e-1, Disp::Std), "0.123456789012");
    assert_eq!(display(-1.23456789012e0, Disp::Std), "-1.23456789012");
    assert_eq!(display(-1.23456789012e11, Disp::Std), "-123456789012");
    assert_eq!(display(-1.23456789012e-1, Disp::Std), "-0.123456789012");
  }

  #[test]
  fn test_conversions() {
    // This is a simple test: num-rational and num-complex do the hard work

    // Only test power-of-two rationals to avoid floating point errors
    assert_eq!(q2r(Q::new(1, 2)), 0.5);
    assert_eq!(q2r(Q::new(1, -4)), -0.25);

    assert_eq!(r2q(8.25), Q::new(33, 4));
    assert_eq!(r2q(-1.5), Q::new(-3, 2));

    assert_eq!(r2c(0.0), C::new(0.0, 0.0));
    assert_eq!(r2c(-1.0e10), C::new(-1.0e10, 0.0));

    assert_eq!(inti(42.99), 42i64);
    assert_eq!(inti(f64::INFINITY), i64::MAX);
    assert_eq!(inti(f64::NAN), 0i64);
  }

  #[test]
  fn test_display_nan_and_infinity() {
    assert_eq!(&display(f64::NAN, Disp::Std), "? (not a number)");
    assert_eq!(&display(f64::NAN, Disp::Fix(4)), "? (not a number)");
    assert_eq!(&display(f64::NAN, Disp::Sci(4)), "? (not a number)");
    assert_eq!(&display(f64::INFINITY, Disp::Std), "oo (infinity)");
    assert_eq!(&display(f64::INFINITY, Disp::Fix(4)), "oo (infinity)");
    assert_eq!(&display(f64::INFINITY, Disp::Sci(4)), "oo (infinity)");
    assert_eq!(&display(-f64::INFINITY, Disp::Std), "-oo (infinity)");
    assert_eq!(&display(-f64::INFINITY, Disp::Fix(4)), "-oo (infinity)");
    assert_eq!(&display(-f64::INFINITY, Disp::Sci(4)), "-oo (infinity)");
  }

  #[test]
  fn test_round() {
    assert!(Real(f64::NAN).round().is_nan());
    assert_eq!(Real(f64::INFINITY).round(), Real(f64::INFINITY));
    assert_eq!(Real(12.5).round(), Real(13.0));
    assert_eq!(Real(-4.9).round(), Real(-5.0));

    // todo fraction and complex
  }

  // todo abs, int, frac
}

// Copyright see AUTHORS & LICENSE; SPDX-License-Identifier: ISC+
