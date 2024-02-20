use num_complex::ComplexFloat;
use num_traits::cast::FromPrimitive;
use std::fmt;
use Number::*;

#[derive(Copy, Clone, Debug, Default, PartialEq)] #[rustfmt::skip]
pub enum Disp { #[default] Std, Fix(u8), Eng(u8), Sci(u8), }

pub type Fraction = num_rational::Ratio<i64>;

pub type Complex = num_complex::Complex<f64>;

#[derive(Clone, Copy, Debug, PartialEq)] #[rustfmt::skip]
pub enum Number { Simple(f64), Fraction(Fraction), Complex(Complex) }

fn f2s(f: Fraction) -> f64 {
  p(*f.numer() as f64 / *f.denom() as f64)
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

// round to 12 decimal digits of precision and have at most +/-299 as exponent
fn p(x: f64) -> f64 {
  if x.is_nan() {
    return f64::NAN;
  }
  if x.is_infinite() {
    return x;
  }

  // Convert f64 to string with with 12 decimal digits precision
  let s: &str = &format!("{x:.11e}");
  let inf = f64::INFINITY.copysign(x.signum());

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

fn display(f: f64, disp: Disp) -> String {
  if f.is_nan() {
    return "? (not a number)".into();
  }
  if f.is_infinite() {
    return format!("{}oo (infinity)", if f < 0.0 { "-" } else { "" });
  }

  match disp {
    Disp::Std => {
      let f = p(f);
      let s = format!("{f}");
      if s.len() > 14 {
        format!("{f:e}")
      } else {
        s
      }
    }
    _ => todo!(),
  }
}

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

impl_number! {
  simple() -> f64 { s => p(s), f => f2s(f), c => c.abs() }
  fraction() -> Fraction { s => s2f(s), f => f, c => s2f(c.abs()) }
  complex() -> Complex { s => s2c(s), f => s2c(f2s(f)), c => c }
  re() -> f64 { s => p(s), f => f2s(f), c => c.re }
  im() -> f64 { _ => 0f64, _ => 0f64, c => c.im }
  int() -> i64 { s => int(s), f => int(f2s(f)), c => int(c.abs()) }
  numer() -> i64 { s => int(s), f => *f.numer(), c => int(c.re) }
  denom() -> i64 { _ => 1i64, f => *f.denom(), _ => 1i64 }
  add_number(rhs: Self) -> Self {
    s => Simple(p(s + rhs.simple())),
    f => Fraction(f + rhs.fraction()),
    c => Complex(c + rhs.complex()),
  }
  sub_number(rhs: Self) -> Self {
    s => Simple(p(s - rhs.simple())),
    f => Fraction(f - rhs.fraction()),
    c => Complex(c - rhs.complex()),
  }
  mul_number(rhs: Self) -> Self {
    s => Simple(p(s * rhs.simple())),
    f => Fraction(f * rhs.fraction()),
    c => Complex(c * rhs.complex()),
  }
  div_number(rhs: Self) -> Self {
    s => Simple(p(s / rhs.simple())),
    f => Fraction(f / rhs.fraction()),
    c => Complex(c / rhs.complex()),
  }
  display(disp: Disp) -> String {
    s => display(s, disp),
    f => format!("{f}"),
    c => format!("{c}"),
  }
}

impl Number {
  pub fn as_complex(re: f64, im: f64) -> Number {
    Complex(num_complex::Complex { re, im })
  }

  pub fn as_fraction(numer: i64, denom: i64) -> Number {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_p() {
    let x = 0.123456789012; // todo not sure, should I add 49 to the right?
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
      inf,
      -inf,
      inf,
      -inf,
    ];
    assert_eq!(numbers.len(), numbers2.len());
    for i in 0..numbers.len() {
      assert_eq!(p(numbers[i]), numbers2[i], "#{i}: {:e}", numbers[i]);
    }

    assert!(p(f64::NAN).is_nan());
  }
}
