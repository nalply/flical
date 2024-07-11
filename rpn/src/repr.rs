use crate::disp::{disp_c, disp_q, disp_r, disp_z};
use crate::native::types::*;
use crate::native::Native;
use crate::NumDisplay::{self, *};
use core::fmt;

pub fn repr_z(z: Z) -> Repr {
  Repr::from_bytes(disp_z(z, Raw))
}

pub fn repr_r(r: R) -> Repr {
  Repr::from_bytes(disp_r(r, Raw))
}

pub fn repr_q(q: Q) -> Repr {
  Repr::from_bytes(disp_q(q, Raw))
}

pub fn repr_c(c: C) -> Repr {
  Repr::from_bytes(disp_c(c, Raw))
}

/// The repr is the internal representation of numbers as strings. For
/// calculations they are converted into one of the native types Z, R, Q or C.
/// Z and Q use 64-bit integers, R and C use 64-bit IEEE754 numbers (which have
/// 15.95 decimal digits, see wikipedia.org/wiki/IEEE_754). The calculator
/// uses 15 decimal digits but displays only 10 decimal digits, because the
/// display is only 35 columns wide: -0.123456789e-123i-0.123456789e-123 uses
/// the whole width of the display!
///
/// This representation will be converted to a native number for calculation
/// then back to the internal string representation. This is not efficient but
/// calculator numbers are not native and have different ranges and precisions.
///
/// Calculator numbers:
///
/// - 64 bit signed integer
/// - Floating point numbers with 15 decimal digits precision where only 10
/// are visible and with exponent between -300 and 300 exclusive, and also
/// infinities +oo and -oo, but not NaNs.
/// - Reduced quotients with numerator and denominator not more than 6 decimal
/// digits
/// - Complex numbers with two floating point numbers as defined abote, one for
/// the real and the other for the imaginary part and also the four infinities
/// +oo, -oo, +ioo and -ioo, this means numbers like +oo+i are not supported
///
/// The longest repr is for a complex number with a negative part with 15 digits
/// precision and exponent below -99 both in the real and imaginary part: 41
/// chars. Example: `"-.123456789012345E-100I-.123456789012345E-100"`.
///
/// Note that if a number is element of a subset it will be represented as
/// being in the subset. Example: 0.0 is mathematically an integer, so it will
/// be represented as `"0"`.
///
/// Grammar:
///
/// - Z: decimal number
/// - Q: Z then `/` then Z (reduced quotients only!)
/// - R: scientific notation with leading '.' and at most 15 decimal digits
/// in mantissa and exponent between -299 and 299 inclusive with letter 'E'
/// - Z: R then 'I' then R; and also only these complex infinities
/// `"+.0E+0I+oo"` and `"+.0E+0I-oo"`
///
/// Examples: `"0"`, `"42"`, `"-1"`, `"1/2"`, `"-1/3"` `".1E+0"`, `"-.2E-12"`,
/// `-.42E-3`, `".12345678901234E-299"` and `"+oo"`
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Repr([u8; REPR_LEN]);

pub const REPR_LEN: usize = 41;

impl Default for Repr {
  fn default() -> Self {
    Repr::ZERO
  }
}

impl fmt::Debug for Repr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = self.to_str().trim();
    f.write_str(&format!("Repr {s}"))
  }
}

impl fmt::Display for Repr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.to_str())
  }
}

impl Repr {
  pub const ZERO: Repr = Repr(*b"0                                        ");

  pub const P_INF: Repr = Repr(*b"+oo                                      ");

  pub const M_INF: Repr = Repr(*b"-oo                                      ");

  // not a valid representation but used to start a representation.
  const EMPTY: Repr = Repr(*b"                                         ");

  /// Convert to native. Panic if not a valid representation
  pub fn to_native(&self) -> Native {
    let s = std::str::from_utf8(&self.0).expect("invalid UTF-8").trim();
    s.parse().unwrap_or_else(|err| panic!("invalid repr {s}: {err}\n{}", err))
  }

  pub fn as_z(&self) -> Z {
    self.to_native().as_z()
  }

  pub fn as_r(&self) -> R {
    self.to_native().as_r()
  }

  pub fn as_q(&self) -> Q {
    self.to_native().as_q()
  }

  pub fn as_c(&self) -> C {
    self.to_native().as_c()
  }

  pub fn to_z(&self) -> Self {
    self.to_native().to_z().repr()
  }

  pub fn to_r(&self) -> Self {
    self.to_native().to_r().repr()
  }

  pub fn to_q(&self) -> Self {
    self.to_native().to_q().repr()
  }

  pub fn to_c(&self) -> Self {
    self.to_native().to_c().repr()
  }

  pub fn copy_from_bytes<I: ToBytes>(mut self, from: I) -> Self {
    let from = from.to_bytes();
    self.0[..from.len()].copy_from_slice(from);
    self
  }

  pub fn from_bytes<I: ToBytes>(from: I) -> Repr {
    Self::EMPTY.copy_from_bytes(from)
  }

  pub fn to_str(&self) -> &str {
    to_str(self.0.as_slice()).trim()
  }

  pub fn disp(&self, disp: NumDisplay) -> String {
    self.to_native().disp(disp)
  }
}

fn to_str(bytes: &[u8]) -> &str {
  std::str::from_utf8(bytes).expect("invalid UTF-8")
}

pub trait ToBytes {
  fn to_bytes(&self) -> &[u8];
}

impl ToBytes for String {
  fn to_bytes(&self) -> &[u8] {
    self.as_bytes()
  }
}

impl ToBytes for &str {
  fn to_bytes(&self) -> &[u8] {
    self.as_bytes()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_repr_r_to_str() {
    let x = 0.1234567890123449; // the last two digits 49 should be truncated
    let ten = 10.0f64;
    let inf = f64::INFINITY;
    let nums = [
      0.1,
      1.1 - 1.0,
      1.0 / 3.0,
      1234567890123.0,
      1234567890123.4,
      12345678901234.0,
      123456789012345.0,
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
    let strs = [
      "1.0000000000000e-1",
      "1.0000000000000e-1",
      "3.3333333333333e-1",
      "1234567890123",
      "1.2345678901234e12",
      "12345678901234",
      "1.2345678901234e14", // todo should be 5 before e instead of 4?
      "1.2345678901234e10",
      "1.2345678901234e11",
      "1.2345678901234e199",
      "1.2345678901234e299",
      "+oo",
      "-oo",
      "1.2345678901234e-5",
      "1.2345678901234e-13",
      "1.2345678901234e-201",
      "1.2345678901234e-299",
      "0",
      "0",
      "+oo",
      "-oo",
    ];
    assert_eq!(nums.len(), strs.len());
    for i in 0..nums.len() {
      assert_eq!(repr_r(nums[i]).to_str(), strs[i], "#{i}: {:e}", nums[i]);
    }
  }

  #[test]
  fn test_native_repr_to_str() {
    assert_eq!(Native::q(1, 4).repr().to_str(), "1/4");
    assert_eq!(
      Native::c(0.0, 1.234567890123449).repr().to_str(),
      "0i1.2345678901234e0"
    );
  }

  #[test]
  fn test_r_display_std() {
    fn r_std(r: R) -> String {
      Native::r(r).repr().disp(Std)
    }

    assert_eq!(r_std(0.1234567890123), "1.2345678901230e-1");
    assert_eq!(r_std(0.12345678901234), "1.2345678901234e-1");
    assert_eq!(r_std(-0.12345678901234), "-1.2345678901234e-1");
    assert_eq!(r_std(1.2345678901234), "1.2345678901234e0");
    assert_eq!(r_std(-1.2345678901234), "-1.2345678901234e0");

    assert_eq!(r_std(1.234567890123e15), "1.2345678901230e15");
    assert_eq!(r_std(1.2345678901234e15), "1.2345678901234e15");
    assert_eq!(r_std(-1.2345678901234e15), "-1.2345678901234e15");
    assert_eq!(r_std(1.2345678901234e-2), "1.2345678901234e-2");
    assert_eq!(r_std(-1.2345678901234e-2), "-1.2345678901234e-2");

    // Round down / up for negative
    assert_eq!(r_std(0.123456789012341), "1.2345678901234e-1");
    assert_eq!(r_std(-0.123456789012341), "-1.2345678901234e-1");
    assert_eq!(r_std(1.23456789012341), "1.2345678901234e0");
    assert_eq!(r_std(-1.23456789012341), "-1.2345678901234e0");

    // Round to even
    assert_eq!(r_std(0.123456789012345), "1.2345678901234e-1");
    assert_eq!(r_std(-0.123456789012345), "-1.2345678901234e-1");
    assert_eq!(r_std(0.123456789012335), "1.2345678901234e-1");
    assert_eq!(r_std(-0.123456789012335), "-1.2345678901234e-1");

    // Todo why does this not round to even?
    assert_eq!(r_std(1.23456789012345), "1.2345678901235e0");
    assert_eq!(r_std(-1.23456789012345), "-1.2345678901235e0");

    // Round up / down for negative
    assert_eq!(r_std(0.123456789012351), "1.2345678901235e-1");
    // assert_eq!(r_std(-0.12345678901251), "-0.123456789013");
    // assert_eq!(r_std(0.12345678901299), "0.123456789013");
    // assert_eq!(r_std(-0.12345678901299), "-0.123456789013");
    // assert_eq!(r_std(1.2345678901251), "1.23456789013");
    // assert_eq!(r_std(-1.2345678901251), "-1.23456789013");
    // assert_eq!(r_std(1.2345678901299), "1.23456789013");
    // assert_eq!(r_std(-1.2345678901299), "-1.23456789013");

    // // Sci to Std
    // assert_eq!(r_std(1.23456789012e0), "1.23456789012");
    // assert_eq!(r_std(1.23456789012e11), "123456789012");
    // assert_eq!(r_std(1.23456789012e-1), "0.123456789012");
    // assert_eq!(r_std(-1.23456789012e0), "-1.23456789012");
    // assert_eq!(r_std(-1.23456789012e11), "-123456789012");
    // assert_eq!(r_std(-1.23456789012e-1), "-0.123456789012");
  }

  #[test]
  fn test_disp_infinity() {
    assert_eq!(Native::r(f64::INFINITY).disp(Std), "+oo");
    assert_eq!(Native::r(f64::INFINITY).disp(Fix(4)), "+oo");
    assert_eq!(Native::r(f64::INFINITY).disp(Sci(4)), "+oo");
    assert_eq!(Native::r(f64::INFINITY).disp(HexU), "+oo");
    assert_eq!(Native::r(f64::INFINITY).disp(HexL), "+oo");
    assert_eq!(Native::r(f64::INFINITY).disp(Raw), "+oo");
    assert_eq!(Native::r(-f64::INFINITY).disp(Std), "-oo");
    assert_eq!(Native::r(-f64::INFINITY).disp(Fix(4)), "-oo");
    assert_eq!(Native::r(-f64::INFINITY).disp(Sci(4)), "-oo");
    assert_eq!(Native::r(f64::INFINITY).disp(HexU), "+oo");
    assert_eq!(Native::r(-f64::INFINITY).disp(HexL), "-oo");
    assert_eq!(Native::r(-f64::INFINITY).disp(Raw), "-oo");
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
