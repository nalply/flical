import { type Num } from  './num.js'
import { Fraction as FractionJs } from 'fraction'

export class Fraction implements Num {
  readonly x: FractionJs

  private constructor(x: FractionJs) {
    this.x = x
  }

  static from(num: Num = 0, den: Num = 1) {
    return new Fraction(new FractionJs(
      num.num * den.den,
      den.num * num.den,
    ))
  }

  static parse(s: string) {
    return new Fraction(new FractionJs(s))
  }

  calcString() {
    return this.x.toString()
  }

  add(y: Num) {
    return new Fraction(this.x.add(y.num, y.den)) as this
  }

  mul(y:  Num) {
    return new Fraction(this.x.add(y.num, y.den)) as this
  }

  sub(y: Num) {
    return new Fraction(this.x.sub(y.num, y.den)) as this
  }

  div(y: Num) {
    return new Fraction(this.x.div(y.num, y.den)) as this
  }

  abs() {
     return new Fraction(this.x.abs()) as this
  }

  int() {
    return new Fraction(this.x.floor()) as this
  }

  frac() {
    return new Fraction(this.x.sub(this.x.floor())) as this
  }

  get re() {
    return this.x.valueOf()
  }

  get im() {
    return 0
  }

  get num() {
    return this.x.n
  }

  get den() {
    return this.x.d
  }

  get complex(): [ number ] {
    return [ this.x.valueOf() ]
  }

  get fraction(): [ number, number ]  {
    return [ this.num, this.den ]
  }

  get parts(): [ number, number ] {
    return [ this.num, this.den ]
  }
}

Object.assign(window, { Fraction })

/// Is the value a Fraction?
export const isFraction
  = (x: unknown): x is Fraction => x instanceof Fraction

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+

