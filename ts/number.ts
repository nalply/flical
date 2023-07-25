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
    return +this
  }
  
  get re() {
    return +this
  }

  get im() {
    return 0
  }

  get num() {
    return Math.floor(+this)
  }

  get den() {
    return 1
  }

  get complex(): [ number ] {
    return [ +this ]
  }

  get fraction(): [ number ] {
    return [ +this ]
  }

  get parts(): [ number ] {
    return [ +this ]
  }
}

const descs 
  = Object.getOwnPropertyDescriptors(NumberExt.prototype)
console.debug("descriptors", descs)

const numProto = Number.prototype as any
for (const [ name, desc ] of Object.entries(descs)) {
  numProto[name] = desc.value

  const get = desc.get
  if (get) Object.defineProperty(numProto, name, { get })
}
console.debug("number now has NumberExt methods")

Object.assign(window, { NumberExt })

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+

