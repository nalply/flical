use std::num::ParseIntError;

use crate::native::types::*;

const ENABLE_DEBUG: bool = true;
// const ENABLE_DEBUG: bool = false;

macro_rules! d {
  ( $fmt:literal $(, $( $arg:tt )* )? ) => {
    if ENABLE_DEBUG { eprintln!( $fmt $( , $( $arg )* )? ) }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)] #[rustfmt::skip]
pub enum NumDisplay { #[default] Std, Raw, Fix(u8), Sci(u8), HexU, HexL }
use NumDisplay::*;

pub fn disp_z(z: Z, disp: NumDisplay) -> String {
  match disp {
    Std | Fix(..) | Sci(..) | Raw => format!("{z}"),
    HexU => format!("{z:X}"),
    HexL => format!("{z:x}"),
  }
}

fn disp_sci(r: R, prec: u8) -> String {
  let inf = if r > 0.0 { "+oo" } else { "-oo" }.into();
  let prec = prec as usize;
  let s = format!("{r:.prec$e}");

  // If exponent >= 300, positive infinity, if <= -300, negative infinity
  if let Some(e_pos) = s.find('e') {
    let (_, exponent) = s.split_at(e_pos + 1);
    if exponent.len() >= 3 {
      let mut chars = exponent.chars();
      match chars.next() {
        Some('3') => return inf,
        Some('-') => {
          if let Some('3') = chars.next() {
            return "0".into();
          }
        } // no -0.0
        _ => (),
      }
    }
  }

  s
}

/// Exponent in scientific notation, +/-INFINITY => i16::MAX, 0.0 or NaN => 0
pub fn exponent(r: R) -> i16 {
  const MINUS_INF: R = -R::INFINITY;

  match r.abs().log10() {
    MINUS_INF => 0,
    exp => exp.floor() as i16,
  }
}

/// Should the real be displayed like an integer?
pub fn is_disp_as_z(r: R) -> bool {
  !r.is_nan() && r.fract() == 0.0 && (0..10).contains(&exponent(r))
}

pub fn disp_raw(r: R) -> String {
  fn to_i_1(s: &str) -> Result<i32, ParseIntError> {
    d!("to_i_1 {s}");
    s[1..].parse()
  }
  fn trim_end_0(s: &str) -> &str {
    s.trim_end_matches('0')
  }

  const RAW_PRECISION: u8 = 14u8;

  let s = disp_sci(r, RAW_PRECISION);
  if let Some(dot_pos) = s.find('.') {
    let (int, rest) = s.split_at(dot_pos);
    d!("int {int} rest {rest}");
    if let Some(e_pos) = rest.find('e') {
      let (frac, exp) = rest.split_at(e_pos);
      let frac = &trim_end_0(frac)[1..];
      d!("frac {frac} exp {exp}");
      if let Ok(exp) = to_i_1(exp) {
        let sign = if int.as_bytes()[0] == b'-' { "-" } else { "" };
        let int = int.trim_start_matches('-');
        let exp = exp + 1;
        return format!("{sign}.{int}{frac}E{exp}");
      }
    }
  }

  unreachable!("exp not i32 or missing . or e: '{s}'");
}

/// Format r in standard notation when |r| between 1e-6 and 1e12 exclusive,
/// rounding to 12 decimal digits precision, in scientific notation with
/// 12 digits precision but trailing zeroes removed from mantissa. If the
/// display gets wider than 17 chars, reduce precision.
pub fn disp_std(r: R) -> String {
  const DISP_MAX_LEN: usize = 17;

  // Format then trim zeroes from number or mantissa respectively. If the
  // formatted results doesn't fit within DISP_MAX_LEN retry recursively
  // with lower precision.
  fn fmt_r<F: Fn(R, usize) -> String>(r: R, prec: usize, fmt: F) -> String {
    let s = fmt(r, prec);
    assert!(s.is_ascii(), "fmt() did not return an ASCII-only string");
    let n = s.len();

    // trim end zeroes from timmee (mantissa / number), tail is exp or empty
    let pos = s.find('e').unwrap_or(n);
    let (trimmee, tail) = s.split_at(pos);
    let trimmee = trimmee.trim_end_matches('0').trim_end_matches('.');
    let s_trimmed = [trimmee, tail].concat();
    d!("fmt({r:e}, {prec}): s {s} n {n} pos {pos} s_trimmed {s_trimmed}");

    let n_trimmed = s_trimmed.len();
    if n_trimmed > DISP_MAX_LEN {
      fmt_r(r, prec - (n_trimmed - DISP_MAX_LEN), fmt)
    } else {
      s_trimmed.to_owned()
    }
  }

  // Display standard notation without exponent if not too small nor too large,
  // else use scientific notation but don't exceed DISP_MAX_LEN (see fmt_r())
  if (1e-6..1e13).contains(&r.abs()) {
    d!("standard notation");
    fmt_r(r, 12, |r, prec| format!("{r:.prec$}"))
  } else {
    d!("scientific notation");
    fmt_r(r, 12, |r, prec| format!("{r:.prec$e}"))
  }

  // Todo: small bug, numbers like 99999999999.432 display too many digits of
  // precision because format!() understands the precision as the number of
  // digits after the point, and Flical understands the precision as the number
  // of digits.
}

// Format the number in standard notation with a fixed precision. If it displays
// more than 12 chars plus the count of . and +, switch to scientific notation
// with fixed precision.
pub fn disp_fix(r: R, prec: u8) -> String {
  let s = format!("{r:.0$}", prec as usize);
  let max_len = 12 + s.matches(['.', '-']).count();

  d!("s {s} #{} max_len {max_len}", s.len());

  // s is guaranteed to be ASCII only
  if s.len() <= max_len {
    return s;
  }

  format!("{r:.0$e}", prec as usize)
}

pub fn disp_r(r: R, disp: NumDisplay) -> String {
  if r.is_nan() {
    panic!("unexpected NaN")
  }

  if is_disp_as_z(r) {
    return disp_z(r as Z, disp);
  }

  if r.is_infinite() {
    return if r > 0.0 { "+oo" } else { "-oo" }.into();
  }

  match disp {
    Std | HexL | HexU => disp_std(r),
    Fix(prec) => disp_fix(r, prec),
    Sci(prec) => disp_sci(r, prec),
    Raw => disp_raw(r),
  }
}

pub fn disp_q(q: Q, _disp: NumDisplay) -> String {
  // todo: omit / if denom == 1
  let numer = q.numer();
  let denom = q.denom();
  format!("{numer}/{denom}")
}

pub fn disp_c(c: C, disp: NumDisplay) -> String {
  if c.im == 0.0 {
    disp_r(c.re, disp)
  } else {
    format!("{}i{}", disp_r(c.re, disp), disp_r(c.im, disp))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_disp_r() {
    #[track_caller]
    fn modi(r: R, nums: &str) {
      let mut disps = [Std, Raw, Fix(0), Fix(4), Sci(6), HexL, HexU].iter();
      for num in nums.split(' ') {
        let disp = disps.next().unwrap();
        assert_eq!(disp_r(r, *disp), num, "with disp {disp:?}");
      }
    }

    modi(0.0, "0 0 0 0 0 0 0");
    modi(42.0, "42 42 42 42 42 2a 2A");
    modi(-1.0, "-1 -1 -1 -1 -1 ffffffffffffffff FFFFFFFFFFFFFFFF");
    modi(0.1, "0.1 .1E0 0 0.1000 1.000000e-1 0.1 0.1");
    modi(-2.1, "-2.1 -.21E1 -2 -2.1000 -2.100000e0 -2.1 -2.1");
    modi(8e200, "8e200 .8E201 8e200 8.0000e200 8.000000e200 8e200 8e200");
    modi(f64::INFINITY, "+oo +oo +oo +oo +oo +oo +oo");
    modi(-f64::INFINITY, "-oo -oo -oo -oo -oo -oo -oo");
  }

  // Test the precision of 15 decimal digits with rounding
  #[test]
  fn test_r_raw_precision() {
    assert_eq!(disp_r(1.23456789012345, Raw), ".123456789012345E1");
    assert_eq!(disp_r(1.234567890123456, Raw), ".123456789012346E1");

    assert_eq!(disp_r(444.444444444423, Raw), ".444444444444423E3");
    assert_eq!(disp_r(444.4444444444423, Raw), ".444444444444442E3");
    assert_eq!(disp_r(444.4444444444465, Raw), ".444444444444447E3");

    assert_eq!(disp_r(0.222222222222221e-99, Raw), ".222222222222221E-99");
    assert_eq!(disp_r(0.2222222222222234e-99, Raw), ".222222222222223E-99");
    assert_eq!(disp_r(0.2222222222222277e-99, Raw), ".222222222222228E-99");
  }

  #[test]
  fn test_r_std_precision() {
    assert_eq!(disp_r(0.45, Std), "0.45");
    assert_eq!(disp_r(0.123456781234, Std), "0.123456781234");
    assert_eq!(disp_r(0.1234567812365, Std), "0.123456781237");
    assert_eq!(disp_r(-9999999999999.4, Std), "-9999999999999.4");
    assert_eq!(disp_r(-9999999999999.42, Std), "-9999999999999.42");
    assert_eq!(disp_r(12345678901234.0, Std), "1.234567890123e13");
    assert_eq!(disp_r(-1.2345678901e-200, Std), "-1.23456789e-200");
    assert_eq!(disp_r(-1.2345678916e-200, Std), "-1.234567892e-200");

    // small bug: if Std uses sci, should route through disp_sci() instead
    // assert_eq!(disp_r(1e300, Std), "+oo");
    // assert_eq!(disp_r(1e-300, Std), "+oo");
    // todo something like 0.9999999e300
  }

  #[test]
  #[should_panic(expected = "unexpected NaN")]
  fn test_panic_on_nan() {
    disp_r(f64::NAN, Raw);
  }

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
  fn test_weird_zeros() {
    let subnormal = f64::MIN_POSITIVE / 2.0;
    assert!(subnormal.is_subnormal(), "subormality sanity check");
    // assert_eq!(disp_r(subnormal, Std), "0"); todo Std should use disp_sci()

    let neg_zero = -0.0f64;
    assert!(neg_zero.is_sign_negative(), "negative zero sanity check");
    assert_eq!(disp_r(neg_zero, Std), "0");
  }
}
