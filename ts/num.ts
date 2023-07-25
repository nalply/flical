// type Grow<T, N extends number, A extends T[]> =
//   A['length'] extends N ? A : Grow<T, N, [...A, T]>
//
// type FixedArray<T, N extends number> = Grow<T, N, []>

export type Parts = [ number ] | [ number, number ]

export type DecDigit = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
export type HexDigit = 'a' | 'b' | 'c' | 'd' | 'e' | 'f'
export type Digit = DecDigit | HexDigit

export interface Num {
  isValid(): this is Num
  assertValid(): this
  add(y: Num): this
  mul(y: Num): this
  sub(y: Num): this
  div(y: Num): this
  parse(s: string): this
  calcString(): string
  abs(): this
  int(): this
  frac(): this
  get re(): number
  get im(): number
  get complex(): Parts
  get num(): number
  get den(): number
  get simple(): number
  get fraction(): Parts
  get parts(): Parts
}

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+

