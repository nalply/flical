import {
  type Num, type Digit,
  add, appendDigit, complex, 
} from './num.js'

export type Meta = 'base' | 'alt' | 'inv' | 'help' 
export type Mode = 'push' | 'replace' | 'append'

function _<T>(topic: string, result: T, ...args: any[]): T {
  console.debug(topic + ":", result , ...args)
  return result
}

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
    this.x = newX
    this.y = this.z
    this.z = this.t
    this.mode = 'push'
    this.changed()
    _("down", this.dump())
  }
  
  /// replace X with a digit
  replace(digit: Digit, mode?: Mode) {
    this.lastX = this.x
    this.x = +digit
    if (mode) this.mode = mode
    this.changed()
    _("replace", this.dump())
  }

  /// append a digit to X
  append(digit: Digit) {
    this.x = appendDigit(this.x, digit)
    this.changed()
    _("append", this.dump())
  }

  nop() {}

  input(digit: Digit | null) {
    return () => {
      if (digit) switch (this.mode) {
        case 'push': return this.push(+digit, 'append')
        case 'replace': return this.replace(digit, 'append')
        case 'append': return this.append(digit)
      }
    }
  }

  enter() {
    this.push(null, 'replace')
  }

  add() {
    this.down(add(this.x, this.y))
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

  // todo bug help then meta == alt but expected base
  doMeta(meta?: Meta) {
    const state = { base: 'alt', alt: 'inv', inv: 'base', help: 'inv' }
    this.meta = meta ?? state[this.meta] as Meta
    if (meta) this.meta = meta
    if (meta === 'help') {
      this.info = 'help'
    }
    this.changed()
  }

  sto() {
    console.log("sto")
  }

  rcl() {
    console.log("rcl")
  }

  left() {
    console.log("left")
  }

  dot() {
    console.log("dot")
  }

  i() {
    this.x = complex(this.x)
  }
}

