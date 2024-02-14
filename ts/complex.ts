import { type Num } from './num.js'
import { Complex as ComplexJs } from 'complex'

export class Complex implements Num {
  readonly x: ComplexJs

  private constructor(x: ComplexJs) {
    this.x = x
  }

  /** @internal */ static ComplexJs = ComplexJs
    
  static from(re: Num = 0, im: Num = 0) {
    return new Complex(new ComplexJs(re.re - im.im, re.im + im.re))
      .assertValid()
  }

  isValid(): this is Complex {
    return this.x.isFinite() && !this.x.isNaN()
  }

  assertValid(): this {
    if (this.isValid()) return this

    throw new TypeError("invalid complex: " + this)
  }

  parse(s: string) {
    return new Complex(new ComplexJs(s)) as this
  }
 
  calcString() {
    const op = this.x.im >= 0 ? "+" : "-"
    return this.x.re + op + "i" + this.x.im
  }

  add(y: Num) {
    return new Complex(this.x.add(y.re, y.im)) as this
  }

  mul(y: Num) {
    return new Complex(this.x.mul(y.re, y.im)) as this
  }

  sub(y: Num) {
    return new Complex(this.x.sub(y.re, y.im)) as this
  }

  div(y: Num) {
    return new Complex(this.x.div(y.re, y.im)) as this
  }

  abs() {
    return  Complex.from(this.x.abs()) as this
  }

  int() {
    return Complex.from(Math.floor(this.x.abs())) as this
  }

  frac(): this {
    return this.abs().sub(this)
  }

  get simple() {
    return this.abs().re
  }
  
  get re() {
    return this.x.re
  }

  get im() {
    return this.x.im
  }

  get num() {
    return this.int().re
  }

  get den() {
    return 1
  }

  get complex(): [ number, number ] {
    return [ this.re, this.im ]
  }

  get fraction(): [ number ] {
    return [ this.int().re ]
  }

  get parts(): [ number, number ] {
    return [ this.re, this.im ]
  }
}

Object.assign(window, { Complex })

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+

