import { type Num } from 'num.js'

export class NumberExt implements Num {
  private constructor() { }

  isValid(): this is Num {
    return typeof this == 'number'
      && Number.isFinite(this)
      && !Number.isNaN(this)
  }

  assertValid(): this {
    if (!this.isValid()) throw new TypeError("invalid number " + this)
    return this
  }
  
  parse(s: string) {
    return +s as unknown as this
  }

  calcString() {
    return this.toString()
  }

  add(y: Num) {
    return (+this + y.simple as unknown as this).assertValid()
  }

  mul(y: Num) {
    return (+this * y.simple as unknown as this).assertValid()
  }

  sub(y: Num) {
    return (+this - y.simple as unknown as this).assertValid()
  }

  div(y: Num) {
    return (+this / y.simple as unknown as this).assertValid()
  }

  abs() {
    return (Math.abs(+this) as unknown as this).assertValid()
  }

  int() {
    console.log(this)
    return (Math.floor(+this) as unknown as this).assertValid()
  }

  frac(): this {
    return (+this - +this.int() as unknown as this).assertValid()
  }

  get simple() {
    return +this.assertValid()
  }
  
  get re() {
    return +this.assertValid()
  }

  get im() {
    return this.assertValid(), 0
  }

  get num() {
    return Math.floor(+this.assertValid())
  }

  get den() {
    return this.assertValid(), 1
  }

  get complex(): [ number ] {
    return [ +this.assertValid() ]
  }

  get fraction(): [ number ] {
    return [ +this.assertValid() ]
  }

  get parts(): [ number ] {
    return [ +this.assertValid() ]
  }
}

const numProto = Number.prototype as any
const descs 
  = Object.getOwnPropertyDescriptors(NumberExt.prototype)

let names = []

for (const [ name, desc ] of Object.entries(descs)) {
  const { value, get } = desc
  const attributes 
    = get ? { get } : { value }
  Object.defineProperty(numProto, name, attributes)

  names.push((get ? "get " : "") + name)
}
console.debug("number now has NumberExt methods",
  names.join(" "))

Object.assign(window, { NumberExt })

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+

