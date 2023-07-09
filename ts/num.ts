import { Complex } from 'complex'
import { Fraction } from 'fraction'

Object.assign(window, { Complex, Fraction })

export type Digit = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
  | 'a' | 'b' | 'c' | 'd' | 'e' | 'f'

/// Flical num
export type Num = number | Fraction | Complex

type Ty = 'n' | 'f' | 'c' 

function ty(x: unknown): Ty | null {
  return typeof x === 'number' && 'n'
    || x instanceof Complex && 'c'
    || x instanceof Fraction && 'f'
    || null
}

/// Is the value an IEE 754 instance?
export const isNumber = (x: unknown): x is number => ty(x) === 'n'

/// Is the value a Complex.js instance?
export const isComplex = (x: unknown): x is Complex => ty(x) === 'c'

/// Is the value a Fraction.js instance?
export const isFraction = (x: unknown): x is Fraction => ty(x) === 'f'

/// Extract the first part of the Num, or NaN if not available
export function part1(x: unknown): number {
  return isNumber(x) ? x
    : isComplex(x) ? x.re
    : isFraction(x) ? x.n
    : NaN
}

/// Extract the second part of the Num, or NaN if not available
export function part2(x: unknown): number {
  return isNumber(x) ? NaN
    : isComplex(x) ? x.im
    : isFraction(x) ? x.d
    : NaN
}

/// Is the value regular (all parts neither NaN nor infinite)?
export function isRegular(x: unknown): boolean {
  const x1 = part1(x)
  const x2 = isNumber(x) ? 1 : part2(x)
  return Number.isFinite(x1) && Number.isFinite(x2) 
    && !Number.isNaN(x1) && !Number.isNaN(x2)
}

export function isNum(x: unknown): x is Num {
  return isRegular(x) && ty(x) !== null
}

export function assertNum(x: unknown): asserts x is Num {
  if (!isNum(x)) throw new TypeError("not a Num value")
}

// Is the value a Num with imaginary part zero or a non-fraction?
export function isSimple(x: unknown): x is Num {
  return isRegular(x) && (isNumber(x) 
    || isComplex(x) && x.im === 0 
    || isFraction(x) && x.d === 1
  )
}

export function isInteger(x: unknown): x is Num {
  return isSimple(x) && Number.isSafeInteger(part1(x))
}

export function isFractional(x: unknown): x is Fraction {
  return isRegular(x) && isFraction(x) && x.d !== 1
}

export function isWithImaginary(x: unknown): x is Complex {
  return isRegular(x) && isComplex(x) && x.im !== 0
}  

export type Kind = 'simple' | 'integer' | 'fractional' | 'withImaginary'

export function kind(x: unknown): Kind | null {
  return isSimple(x) ? 'simple'
    : isInteger(x) ? 'integer'
    : isFractional(x) ? 'fractional'
    : isWithImaginary(x) ? 'withImaginary'
    : null
}

Object.assign(Number.prototype, {
  add(y: number): number {
    return +this + y
  }
})

function coercePair(x: Num, y: Num): [ Num, Num ] {
  assertNum(x)
  assertNum(y)

  const xTy = ty(x), yTy = ty(y)
  if (xTy === yTy) return [ x, y ]
  if (xTy === 'c') y = complex(y)
  if (yTy === 'c') x = complex(x)
  if (xTy === 'f') y = new Fraction(+y)
  if (yTy === 'f') x = new Fraction(+x)

  return [ x, y ]
}

function binaryOp(op: string, x: Num, y: Num): Num {
  [ x, y ] = coercePair(x, y)
  return (x as any)[op](y) as Num
}

const unimplemented = () => new TypeError("unimplemented")

export function add(x: Num, y: Num): Num {
  return binaryOp('add', x, y)
}

export function appendDigit(x: Num, d: Digit): Num {
  if (isNumber(x)) return +(x.toString() + d)

  throw unimplemented()
}

export function complex(x: Num): Complex {
  return new Complex(+x)
}

type ToPrimitiveHint = 'string' | 'number' | 'default'
type ToPrimitiveResult = string | number

(Complex.prototype as any)[Symbol.toPrimitive] = 
  function(hint: ToPrimitiveHint): ToPrimitiveResult {
    return hint === 'number' ? this.abs() : this.re + " " + this.im + "𝑖"
  }

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+
