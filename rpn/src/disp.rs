use crate::native::is_disp_as_z;
use crate::native::types::*;
use Disp::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)] #[rustfmt::skip]
pub enum Disp { #[default] Std, Internal, Fix(u8), Sci(u8), HexU, HexL }

pub fn disp_z(z: Z, disp: Disp) -> String {
  match disp {
    Std => format!("{z}"),
    Internal => format!("H{z:016x}"),
    HexU => format!("{z:X}"),
    HexL => format!("{z:x}"),
    _ => todo!("for disp={disp:?}"),
  }
}

pub fn disp_r(r: R, _disp: Disp) -> String {
  if r.is_nan() {
    panic!("unexpected NaN")
  }

  let inf = if r > 0.0 { "+oo" } else { "-oo" }.into();
  if r.is_infinite() {
    return inf;
  }

  let s = if is_disp_as_z(r) { format!("{r}") } else { format!("{r:.013e}") };

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

// Round to 14 decimprecision and have at most +/-299 as exponent
// Regex: \d\.\d{14}e-?[0-2]?\d?\d
pub fn disp_q(q: Q, _disp: Disp) -> String {
  // todo: omit / if denom == 1
  let numer = q.numer();
  let denom = q.denom();
  format!("{numer}/{denom}")
}

pub fn disp_c(c: C, disp: Disp) -> String {
  if c.im == 0.0 {
    disp_r(c.re, disp)
  } else {
    format!("{}i{}", disp_r(c.re, disp), disp_r(c.im, disp))
  }
}

#[cfg(test)]
mod tests {
  // use super::*;

  #[test]
  fn test_disp() {
    // todo
  }
}
