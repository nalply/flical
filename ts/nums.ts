import { type Num } from './num.js'
import { Complex } from './complex.js'
import { Fraction } from './fraction.js'
import { NumberExt } from './number.js'

export { type Num, type Digit } from './num.js'
export { Complex } from './complex.js'
export { Fraction } from './fraction.js'
export { NumberExt } from './number.js'


export type Dbg = (topic: any, result: any, ...args: any[]) => any

/// Like `console.debug()` but return second argument, too.
/// @param topic the topic of the log
/// @param result the return value of this function
/// @param ...args the rest of the arguments, optional
export const dbg: Dbg = (topic: any, result: any, ...args: any[]): any => {
  console.debug(topic + ":", result, ...args)
  return result
}

/// Same as `dbg()` but with a shorter name.
export const _: Dbg = dbg

/// To disable _() do `const _ = _disabled` in scope.
export const _disabled: Dbg = (_topic: any, result: any, _args: any[]): any =>
  result

export type Ty = 'number' | 'Fraction' | 'Complex' 

/// Type string for valid Num or null
export function ty(x: unknown): Ty | null {
  return typeof x === 'number' && x.isValid() ? 'number'
    : x instanceof Complex && x.isValid() ? 'Complex'
    : x instanceof Fraction && x.isValid() ? 'Fraction'
    : null
}

/// Type: Is the value a valid (no Infinity nor NaN) Complex instance?
export function isComplex(x: unknown): x is Complex {
  return ty(x) === 'Complex'
}

/// Type: Is the value a valid Fraction instance?
export function isFraction(x: unknown): x is Fraction {
  return ty(x) === 'Fraction'
}

/// Type: Is the value a valid (no Infinity nor NaN) JavaScript number?
export function isNumber(x: unknown): x is number {
  return ty(x) === 'number'
}

/// Kind: Is the value one of the valid Num instances?
export function isNum(x: unknown): x is Num {
  return ty(x) !== null
}

/// Kind: Is the value a Num with imaginary part zero or a non-fraction?
export function isSimple(x: unknown): x is Num {
  return isNum(x) && x.im === 0 && x.den === 1
}

/// Kind: Is the value a simple safe integer Num?
export function isInteger(x: unknown): x is Num {
  return isSimple(x) && Number.isSafeInteger(x.simple)
}

/// Kind: Is the value fractional, i. e. a Fraction with denominator not 1?
export function isFractional(x: unknown): x is Fraction {
  return isFraction(x) && x.den !== 1
}

/// Kind: Is the value with an imaginary part?
export function isWithImaginary(x: unknown): x is Complex {
  return isComplex(x) && x.im !== 0
}

/// The four kinds of Num, see kind()
export type Kind = 'simple' | 'integer' | 'fractional' | 'withImaginary'

/// A type is what in a programming language a value is about, a kind is more
/// mathematical. A value of type Complex can be integer, for example the
/// value '1+i0'. This distinction is neccessary because in JavaScript we
/// have two different but mathematical equal values: '1+i0' and '1', both
/// are same and of kind 'integer', but have the types 'Complex' and 'number'.
export function kind(x: unknown): Kind | null {
  return isInteger(x) ? 'integer'
    : isSimple(x) ? 'simple'
    : isFractional(x) ? 'fractional'
    : isWithImaginary(x) ? 'withImaginary'
    : null
}

  /// Coerce to second argument, if neccessary
export function coerceToSecond(x: Num, y: Num): Num {
  return isComplex(y) ? Complex.from(x)
    : isFraction(y) ? Fraction.fromFloat(x)
    : x
}

console.debug("imported NumberExt", NumberExt)

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+
