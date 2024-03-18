use crate::native::types::*;
use crate::native::Native::*;
use crate::native::NativeError;
use crate::Native;
use crate::Repr;
use num_complex::ComplexFloat;
use num_traits::Signed;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Num(Repr);

impl Num {
  pub const ZERO: Self = Self(Repr::ZERO);
  pub const P_INF: Self = Self(Repr::P_INF);
  pub const M_INF: Self = Self(Repr::M_INF);
}

impl Num {
  pub fn pow(self, exp: Self) -> Self {
    let x: C = self.into();
    let exp: C = exp.into();
    x.powc(exp).into()
  }

  pub fn root(self, root: Self) -> Self {
    let x: C = self.into();
    let root: C = root.into();
    x.powc(1.0 / root).into()
  }

  pub fn decode(s: &str) -> Self {
    Num::from_str(s).unwrap_or_else(|err| panic!("{}", err.to_string()))
  }
}

impl Deref for Num {
  type Target = Repr;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl FromStr for Num {
  type Err = NativeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(s.parse::<Native>()?.into())
  }
}

impl From<Native> for Num {
  fn from(native: Native) -> Self {
    Num(native.fix().repr())
  }
}

impl Default for Num {
  fn default() -> Self {
    Num::ZERO
  }
}

impl fmt::Debug for Num {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&format!("Num({:?})", self.0))
  }
}

macro_rules! impl_num_methods {
  (
    $(
      $method:ident:
      $z:pat => $z_expr:expr ,
      $r:pat => $r_expr:expr ,
      $q:pat => $q_expr:expr ,
      $c:pat => $c_expr:expr ;
    )+
  ) => {
    impl Num {
      $(
        pub fn $method(self) -> Self {
          match self.0.to_native() {
            Integer($z) => ($z_expr).into(),
            Real($r) => ($r_expr).into(),
            Quotient($q) => ($q_expr).into(),
            Complex($c) => ($c_expr).into(),
          }
        }
      )+
    }
  }
}

impl_num_methods! {
  // re() -> R { r => r_to_repr(r), q => q2r(q), c => c.re }
  // im() -> R { _ => 0f64, _ => 0f64, c => c.im }
  // inti() -> Z { r => r2z(r), q => r2z(q2r(q)), c => r2z(c.abs()) }
  // numer() -> Z { r => r2z(r), q => *q.numer(), c => r2z(c.abs()) }
  // denom() -> Z { _ => 1i64, q => *q.denom(), _ => 1i64 }
  chs: z => -z, r => -r, q => -q, c => -c;
  recip: z => z, r => 1.0 / r, q => q.recip(), c => 1.0 / c;
  frac: _ => 0, r => r.fract(), q => q.fract(), c => c.abs().fract();
  int: z => z, r => r.trunc(), q => q.trunc(), c => c.abs().trunc();
  abs: z => z.abs(), r => r.abs(), q => q.abs(), c => c.abs();
  round: z => z, r => r.round(), q => q.round(), c => c.abs().round();
}

macro_rules! impl_binary_ops {
  (
    $( pub fn $name:ident(self, rhs: Self) -> Self { $op:tt } )+
  ) => {
    impl Num {
      $(
        pub fn $name(self, rhs: Self) -> Self {
          if self.is_c() || rhs.is_c() {
            let x: C = self.into();
            let rhs: C = rhs.into();
            (x $op rhs).into()
          } else if self.is_q() || rhs.is_q() {
            let x: Q = self.into();
            let rhs: Q = rhs.into();
            (x $op rhs).into()
          } else {
            let x: R = self.into();
            let rhs: R = rhs.into();
            (x $op rhs).into()
          }
        }
      )+
    }
  }
}

impl_binary_ops! {
  pub fn add_num(self, rhs: Self) -> Self { + }
  pub fn sub_num(self, rhs: Self) -> Self { - }
  pub fn mul_num(self, rhs: Self) -> Self { * }
  pub fn div_num(self, rhs: Self) -> Self { / }
}

macro_rules! impl_conversions {
  ( $( $native:ident( $( $param:ident: $ty:ty ),+ ) )+ ) => {
    $(
      paste::paste! {
        impl From<Num> for [< $native:upper >] {
          fn from(value: Num) -> Self {
            value.0.[< as_ $native >]()
          }
        }

        impl From< [< $native:upper >] > for Num {
          fn from(value: [< $native:upper >]) -> Self {
            Num(Native::from(value).fix().repr())
            // Num([< repr_ $native >](value))
          }
        }

        impl Num {
          pub fn [< from_ $native >]( $( $param: $ty, )+ ) -> Self {
            // let name = stringify!($native);
            // let params = stringify!( $( $param ),+ );
            // eprintln!("from_{name}({params})");
            // dbg!(
              Self(Native::$native( $( $param, )+ ).fix().repr())
            // )
          }

          pub fn [< to_ $native >](&self) -> Self {
            Self(self.0.[< to_ $native >]())
         }
        }
      }
    )+
  }
}

impl_conversions! { z(z: Z) r(r: R) q(numer: Z, denom: Z) c(re: R, im: R) }

macro_rules! impl_is {
  ( $( pub fn $is:ident(self) -> bool { $pat:pat } )+ ) => {
    impl Num {
      $( pub fn $is(self) -> bool { matches!(self.0.to_native(), $pat) } )+
    }
  }
}

impl_is! {
  pub fn is_z(self) -> bool { Integer(_) }
  pub fn is_r(self) -> bool { Real(_) }
  pub fn is_q(self) -> bool { Quotient(_) }
  pub fn is_c(self) -> bool { Complex(_) }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Disp::*;

  fn convert(s: &str, f: impl Fn(Native) -> Native) -> String {
    f(Num::decode(s).to_native()).disp(Internal)
  }

  #[test]
  fn test_conversions() {
    assert_eq!(convert("1/2", |n| n.to_r()), "5.0000000000000e-1");
    assert_eq!(convert("-1/4", |n| n.to_r()), "-2.5000000000000e-1");

    assert_eq!(convert("8.25", |n| n.to_q()), "33/4");
    assert_eq!(convert("-1.5", |n| n.to_q()), "-3/2");
    assert_eq!(convert("4/1", |n| n.to_q()), "H0000000000000004");

    assert_eq!(convert("1", |n| n.to_c()), "H0000000000000001");
    assert_eq!(convert("1/16", |n| n.to_c()), "6.2500000000000e-2");
    assert_eq!(convert("1i1", |n| n.to_r()), "1.4142135623731e0");
    assert_eq!(convert("3i3", |n| n.to_z()), "H0000000000000004");

    assert_eq!(convert("+oo", |n| n.to_z()), "H7fffffffffffffff");
    assert_eq!(convert("42.499", |n| n.to_z()), "H000000000000002a");
    assert_eq!(convert("42.5", |n| n.to_z()), "H000000000000002b");

    // Flical's internal precision is 14 decimal digits
    assert_eq!(convert("44.4999999999994", |n| n.to_z()), "H000000000000002c");
    assert_eq!(convert("45.4999999999995", |n| n.to_z()), "H000000000000002e");
  }

  #[test]
  fn test_round() {
    assert_eq!(Num::decode("+oo").round(), Num::P_INF);
    assert_eq!(Num::from_r(12.5).round(), Num::from_z(13));
    assert_eq!(Num::from_r(-4.9).round(), Num::from_r(-5.0));

    // todo fraction and complex
  }

  // todo abs, int, frac
}

// Copyright see AUTHORS & LICENSE; SPDX-License-Identifier: ISC+
