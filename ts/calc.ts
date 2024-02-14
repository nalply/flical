import { type Num, Complex, type Digit, coerceToSecond, _ } from './nums.js'

export type Meta = 'base' | 'alt' | 'inv' | 'help' 
export type Mode = 'push' | 'replace' | 'append'

// Todo: Complex wins 0+i4 + 8 => 8+i4 and not 12

export class Calc {
  x: Num = 0
  y: Num = 0
  z: Num = 0
  t: Num = 0
  lastX: Num = 0 
  info =  "  Long-press ALT key for help      "
  hints = "< A > < B > < C > < D > < E > < F >"
  mode: Mode = 'push'
  meta: Meta = 'base'
  changed: () => void = () => null

  dump() {
    return "stack " + this.x + "·" + this.y + "·" + this.z + "·" + this.t
      + " lastX " + this.lastX
      + " mode " + this.mode
      + " meta " + this.meta
  }

  /// push up the stack with X a new number or same
  push(newX?: Num | null, mode?: Mode) {
    this.t = this.z
    this.z = this.y
    this.y = this.x
    if (newX) this.x = newX
    if (mode) this.mode = mode
    this.changed()
    _("push", this.dump())
  }

  /// down a level with X a new number or from Y
  down(newX?: Num) {
    newX ??= this.y
    this.lastX = this.x
    this.x = newX
    this.y = this.z
    this.z = this.t
    this.mode = 'push'
    this.changed()
    _("down", this.dump())
  }
  
  /// replace X with a digit
  replace(digit: Digit, mode?: Mode) {
    this.x = +digit
    if (mode) this.mode = mode
    this.changed()
    _("replace", this.dump())
  }

  /// append a digit to X
  append(digit: Digit) {
    this.x = this.x.parse(this.x.calcString() + digit)
    this.changed()
    _("append", this.dump())
  }

  nop() {}

  input(digit: Digit | null) {
    return () => {
      if (digit != null) switch (this.mode) {
        case 'push': return this.push(+digit, 'append')
        case 'replace': return this.replace(digit, 'append')
        case 'append': return this.append(digit)
      }
    }
  }

  enter() {
    this.push(null, 'replace')
  }

  disp() {
    console.log("disp")
    this.meta = 'base'
  }

  doLastX() {
    this.x = this.lastX
    this.meta = 'base'
    this.changed()
  }

  add() {
    this.down(coerceToSecond(this.x, this.y).add(this.y))
  }

  sub() {
    console.log("sub")
  }

  mul() {
    //this.push(m.multiply(this.x, this.y))
  }

  div() {
    console.log("div")
  }

  int() {
    this.x = this.x.int()
    this.changed()
  }

  frac() {
    this.x = this.x.frac()
  }

  // todo bug help then meta == alt but expected base
  doMeta(meta?: Meta) {
    const state = { base: 'alt', alt: 'inv', inv: 'base', help: 'inv' }
    this.meta = meta ?? state[this.meta] as Meta
    if (meta) this.meta = meta
    if (meta === 'help') {
      this.info = 'help'
      this.meta = 'base'
    }
    this.changed()
  }

  sto() {
    console.log("sto")
  }

  xy() {
    [ this.x, this.y ] = [ this.y, this.x ]
    this.meta = 'base'
    this.changed()
  }

  xz() {
    [ this.x, this.z ] = [ this.z, this. x]
    this.meta = 'base'
    this.changed()
  }

  rcl() {
    console.log("rcl")
  }

  rotDown() {
    [ this.x, this.y, this.z, this.t ] = [ this.y, this.z, this.t, this.x ]
    this.meta = 'base'
    this.changed()
  }

  rotUp() {
    [ this.x, this.y, this.z, this.t ] = [ this.t, this.x, this.y, this.z ]
    this.meta = 'base'
    this.changed()
  }

  left() {
    console.log("left")
  }

  dot() {
    console.log("dot")
  }

  i() {
    console.log("i")
    this.x = Complex.from(this.x)
    this.meta = 'base'
    this.mode = 'append'
    this.changed()
  }

  abs() {
    this.x = this.x.abs()
    this.meta = 'base'
    this.changed()
  }  
}


// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+
