import { type Num } from './num.js'
import { isComplex, Complex } from './complex.js'
import { isFraction, Fraction } from './fraction.js'
import { NumberExt } from './number.js'

export { type Num, type Digit } from './num.js'
export { Complex, isComplex } from './complex.js'
export { Fraction,isFraction } from './fraction.js'
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

/// Type string for for Num or null
export function ty(x: unknown): Ty | null {
  return typeof x === 'number' && 'number'
    || x instanceof Complex && 'Complex'
    || x instanceof Fraction && 'Fraction'
    || null
}

export function isNum(x: unknown): x is Num {
  return ty(x) !== null
}

// Is the value a Num with imaginary part zero or a non-fraction?
export function isSimple(x: unknown): x is Num {
  return isNum(x) && x.im === 0 && x.den === 1
}

export function isInteger(x: unknown): x is Num {
  return isSimple(x) && Number.isSafeInteger(x.simple)
}

export function isFractional(x: unknown): x is Fraction {
  return isFraction(x) && x.den !== 1
}

export function isWithImaginary(x: unknown): x is Complex {
  return isComplex(x) && x.im !== 0
}

export type Kind = 'simple' | 'integer' | 'fractional' | 'withImaginary'

export function kind(x: unknown): Kind | null {
  return isInteger(x) ? 'integer'
    : isSimple(x) ? 'simple'
    : isFractional(x) ? 'fractional'
    : isWithImaginary(x) ? 'withImaginary'
    : null
}

console.debug("imported NumberExt", NumberExt)

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+
