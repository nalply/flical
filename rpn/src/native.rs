use crate::disp::{disp_c, disp_q, disp_r, disp_z};
use crate::repr::{repr_c, repr_q, repr_r, repr_z};
use crate::Disp;
use crate::Repr;
use core::fmt;
use num_complex::ComplexFloat;
use num_traits::cast::FromPrimitive;
use std::backtrace::Backtrace;
use std::{
  error::Error,
  num::{ParseFloatError, ParseIntError},
  str::FromStr,
};
use types::*;
use Disp::*;
use Native::*;

/// The native types underlying the numbers: `Z` is the set of integers, R the
/// set of reals, Q the set of quotients (also called as ratios, rationals,
/// fractions and fractionals), and C the set of complex numbers. Of course
/// these sets being infinite cannot be represented faithfully by native types
/// because they are finite. And another mismatch: types are not really sets,
/// for example the value 42.0 is of type R but is integer, too. Or an integer
/// 43 · 10⁶⁰ which is not exactly representable in the type R but in Z, and
/// mathematics say it is an integer number but because the set of the real
/// numbers is a superset, it is also a real number.
///
/// So let's consider these limitations and quirks:
///
/// - Values which are elements of a subset are fixed to that type.
///   Example: `Native::c(1, 0).fix()` becomes `Native::r(1)`.
//
/// - Except if they exceed 1e52 (because of precision already lost).
///
/// - NaN is not supported, for example because I find 0 + NaN · i an
///   abomination. `Native::r(R::NAN).check()` returns an error. The calculator
///   should then refuse the calculation with the message: "Undefined number".
///
/// - Infinities (+oo and -oo) are supported. They are not numbers but useful.
pub mod types {
  /// Corresponds to the set of integers
  pub type Z = i64;

  /// Corresponds to the set of reals
  pub type R = f64;

  /// Corresponds to the set of quotients
  pub type Q = num_rational::Ratio<Z>;

  /// Corresponds to the set of complex numbers
  pub type C = num_complex::Complex<R>;
}

/// Enum over the native number types in the calculator
#[derive(Clone, Copy, PartialEq)] #[rustfmt::skip]
pub enum Native { Integer(Z), Real(R), Quotient(Q), Complex(C) }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeError(pub String);

impl fmt::Display for NativeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.0)
  }
}

impl NativeError {
  fn from_error<E: Error>(err: E) -> Self {
    let stack = Backtrace::force_capture();
    NativeError(format!("{err}\n{stack}"))
  }
}

impl Error for NativeError {}

impl From<ParseFloatError> for NativeError {
  fn from(error: ParseFloatError) -> Self {
    Self::from_error(error)
  }
}

impl From<ParseIntError> for NativeError {
  fn from(error: ParseIntError) -> Self {
    Self::from_error(error)
  }
}

impl fmt::Debug for Native {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&format!("‹{}›", self.disp(Internal)))
  }
}

impl fmt::Display for Native {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.disp(Std))
  }
}

impl FromStr for Native {
  type Err = NativeError;

  /// Parse a string as a native calculator number type.
  ///
  /// This relies on the fact that quotients are displayed with a slash like
  /// this `"2/3"`, and that complex numbers contain the letter `i` in their
  /// string representation and that integers don't have the decimal point.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // todo: parse more things like i = 0 + i or decimal integers
    // todo: write a test

    // Positive Infinity
    if s == "+oo" {
      Real(f64::INFINITY)
    }
    // Negative Infinity
    else if s == "-oo" {
      Real(-f64::INFINITY)
    }
    // Complex has i
    else if let Some(pos) = s.find('i') {
      let (re_s, im_s) = s.split_at(pos);
      let im_s = &im_s[1..];
      Complex(C::new(re_s.parse()?, im_s.parse()?))
    }
    // Quotient has /
    else if let Some(pos_slash) = s.find('/') {
      let (numer_s, denom_s) = s.split_at(pos_slash);
      let denom_s = &denom_s[1..];
      Quotient(Q::new(numer_s.parse()?, denom_s.parse()?))
    }
    // Integer in hex
    else if s.find('H') == Some(0) {
      // first parse as unsigned then cast to signed
      Integer(u64::from_str_radix(&s[1..], 16)? as Z)
    }
    // Real has `.`, `e` or `E`
    else if s.find(['.', 'e', 'E']).is_some() {
      Real(s.parse()?)
    }
    // Anything else try integer
    else {
      Integer(s.parse()?)
    }
    // f64::from_str() might have produced a NaN, check for it.
    .check()
  }
}

impl From<Z> for Native {
  fn from(z: Z) -> Self {
    Integer(z)
  }
}

impl From<R> for Native {
  fn from(r: R) -> Self {
    Real(r)
  }
}

impl From<Q> for Native {
  fn from(q: Q) -> Self {
    Quotient(q)
  }
}

impl From<C> for Native {
  fn from(c: C) -> Self {
    Complex(c)
  }
}

impl Native {
  pub fn disp(&self, disp: Disp) -> String {
    match *self {
      Integer(z) => disp_z(z, disp),
      Real(r) => disp_r(r, disp),
      Quotient(q) => disp_q(q, disp),
      Complex(c) => disp_c(c, disp),
    }
  }

  pub fn z(z: Z) -> Self {
    Integer(z).fix()
  }

  pub fn r(r: R) -> Self {
    Real(r).fix()
  }

  pub fn c(re: R, im: R) -> Self {
    Complex(C { re, im }).fix()
  }

  pub fn q(numer: Z, denom: Z) -> Self {
    Quotient(Q::new(numer, denom)).fix()
  }

  pub fn check(self) -> Result<Self, NativeError> {
    Ok(match self {
      Integer(_) => self,
      Quotient(q) => Quotient(Self::check_q(q)?),
      Real(r) => Real(Self::check_r(r)?),
      Complex(c) => Complex(Self::check_c(c)?),
    })
  }

  // n / 1 -> n,  x + 0i -> x, yyyy.0 -> yyyy, -0.0 -> 0.0, round to 14 digits
  pub fn fix(self) -> Self {
    fn is_denom_1(q: Q) -> bool {
      *q.denom() == 1
    }
    fn is_real(c: C) -> bool {
      c.im == 0.0
    }

    // to string and back
    let native = self.repr_raw().to_native();

    match native {
      Quotient(q) if is_denom_1(q) => Integer(q.to_integer()),
      Real(r) if is_disp_as_z(r) => Integer(r as Z),
      Complex(c) if is_real(c) && is_disp_as_z(c.re) => Integer(c.re as Z),
      Complex(c) if is_real(c) => Real(c.re),
      _ => native,
    }
  }

  pub fn repr_raw(self) -> Repr {
    match self {
      Integer(z) => repr_z(z),
      Real(r) => repr_r(r),
      Quotient(q) => repr_q(q),
      Complex(c) => repr_c(c),
    }
  }

  pub fn repr(self) -> Repr {
    self.check().expect("invalid number").fix().repr_raw()
  }

  /// No NaN. Subnormal and negative zero get rounded to zero.
  pub fn check_r(r: R) -> Result<R, NativeError> {
    if r.is_nan() {
      Err(NativeError("number is NaN".into()))
    } else {
      Ok(r)
    }
  }

  /// No NaN, subnormal values and -0.0 both in re and im
  pub fn check_c(c: C) -> Result<C, NativeError> {
    Self::check_r(c.re)?;
    Self::check_r(c.im)?;

    Ok(c)
  }

  pub fn check_q(q: Q) -> Result<Q, NativeError> {
    if *q.numer() > 999_999 || *q.denom() > 999_999 {
      Err(NativeError("numerator or denominator beyond 999999".into()))
    } else {
      Ok(q)
    }
  }

  pub fn as_z(self) -> Z {
    match self.check().expect("invalid number") {
      Integer(z) => z,
      Real(r) => r.round() as Z,
      Quotient(q) => *q.round().numer(),
      Complex(c) => c.abs().round() as Z,
    }
  }

  pub fn to_z(self) -> Self {
    Integer(self.as_z()).fix()
  }

  pub fn as_r(self) -> R {
    match self.check().expect("invalid number") {
      Integer(z) => z as R,
      Real(r) => r,
      Quotient(q) => *q.numer() as f64 / *q.denom() as f64,
      Complex(c) => c.abs(),
    }
  }

  pub fn to_r(self) -> Self {
    Real(self.as_r()).fix()
  }

  pub fn as_q(self) -> Q {
    match self.check().expect("invalid number") {
      Integer(z) => Q::new(z, 1),
      Real(r) => Q::from_f64(r).unwrap_or_default(),
      Quotient(q) => q,
      Complex(c) => Q::from_f64(c.abs()).unwrap_or_default(),
    }
  }

  pub fn to_q(self) -> Self {
    Quotient(self.as_q()).fix()
  }

  pub fn as_c(self) -> C {
    match self.check().expect("invalid number") {
      Integer(z) => C { re: z as f64, im: 0.0 },
      Real(re) => C { re, im: 0.0 },
      Quotient(q) => C { re: *q.numer() as f64 / *q.denom() as f64, im: 0.0 },
      Complex(c) => c,
    }
  }

  pub fn to_c(self) -> Self {
    Complex(self.as_c()).fix()
  }
}

pub fn exponent(r: R) -> i16 {
  const MINUS_INF: R = -R::INFINITY;

  // https://github.com/rust-lang/rust/issues/41620#issuecomment-1726984085
  #[allow(illegal_floating_point_literal_pattern)]
  match r.abs().log10() {
    MINUS_INF => 0,
    exp => exp.floor() as i16,
  }
}

pub fn is_disp_as_z(r: R) -> bool {
  r.fract() == 0.0 && exponent(r) < 14
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_log10_as_exponent() {
    fn assert_exponents(x: f64) {
      let exponent = exponent(x);
      let s = format!("{x:e}");
      let (_, e) = s.split_at(s.find('e').unwrap());
      let exponent_by_format = e[1..].parse().unwrap();

      assert_eq!(exponent, exponent_by_format, "for {x}");
    }

    for x in [
      0.0, 0.1, 0.02, -0.003, 4e10, 5e99, -6e-10, 7e99, 8e100, 9e-200, 1.2e298,
      3.4e-298, -5.6e299, 7.8e-299, 9.1e300, -2.3e-300,
    ] {
      assert_exponents(x);
    }
  }

  #[test]
  #[should_panic]
  fn test_nan_panic() {
    let _ = Native::r(f64::NAN);
  }

  #[test]
  fn test_weird_zeros() {
    let subnormal = f64::MIN_POSITIVE / 2.0;
    assert!(subnormal.is_subnormal(), "subormality sanity check");
    assert_eq!(Native::r(subnormal).disp(Std), "0");

    let neg_zero = -0.0f64;
    assert!(neg_zero.is_sign_negative(), "negative zero sanity check");
    assert_eq!(Native::r(neg_zero).disp(Std), "0");
  }
}
