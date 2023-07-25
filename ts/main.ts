import { Calc, type Meta } from './calc.js'
import { type Digit, type Dbg, _, _disabled } from './nums.js'

const calc = (window as any).calc = new Calc()

const screen = document.getElementById('screen')

if (screen) calc.changed = function() {
  const annunc = calc.meta === 'alt' || calc.meta === 'inv' ? calc.meta : ''
  
  screen.innerText = ''
    + calc.info.padEnd(35).substring(0, 35 - annunc.length) + annunc
    + '\n' + calc.y.calcString()
    + '\n' + calc.x.calcString()
    + '\n' + calc.hints
}
calc.changed()

const digitMap: ( Digit | null )[] = [
  "a", "b", "c", "d", "e", "f",
  null, null, null, null,
  null, 7, 8, 9,
  null, 4, 5, 6,
  null, 1, 2, 3,
  null, 0, null, null,
]

let suppressMetaOnce = false
function meta(meta?: Meta): any {
  console.log("meta()", "meta", meta, "calc.meta", calc.meta,
    "suppressMetaOnce", suppressMetaOnce)
  
  if (suppressMetaOnce) return suppressMetaOnce = false
    
  if (meta === 'help') suppressMetaOnce = true
  calc.doMeta(meta)
}

// Bind i as closure variable: run(0) is a function!
const run
  = (i: number | undefined) => function(): void | 'ignored'
{
  i ??= NaN
  
  if (i === 25) return meta()

  if (calc.meta === 'base') {
    if (i === 6) return calc.enter()
    if (i === 7) return calc.sto()
    if (i === 8) return calc.rcl()
    if (i === 9) return calc.left()
    if (i === 10) return calc.add()
    if (i === 14) return calc.sub()
    if (i === 18) return calc.mul()
    if (i === 22) return calc.div()
    if (i === 24) return calc.dot()
    
    if (i % 1 === 0 && i < 25)
      return calc.input(digitMap[i] ?? null)()
  }

  if (i > 25) {
    calc.meta = 'alt'
    i -= 25
  }

  if (calc.meta === 'alt') {
    if (i === 6) return calc.disp()
    if (i === 7) return calc.xy()
    if (i === 8) return calc.rotDown()
    if (i === 24) return calc.i()
  }

  if (calc.meta === 'inv') {
    if (i === 6) return calc.doLastX()
    if (i === 7) return calc.xz()
    if (i === 8) return calc.rotUp()
    if (i === 24) return calc.abs()
  }

  return 'ignored'
}

const buttons 
  = [...document.getElementsByTagName('button')]

buttons.map((button: HTMLElement, i: number) =>
    button.addEventListener('click', run(i))
  )

const metaButton
  = buttons.at(-1) ?? document.createElement("p")
onLongPress(
  metaButton,
  600,
  (): void => meta('help')
)

function onLongPress(el: HTMLElement, duration:number, cb: () => void) {
  const _: Dbg = _disabled // comment out to enable debug log
  
  let timer = NaN
  const longPressDone = () => 
    _("longPressDone", cb(), "meta", calc.meta)
    
  const start = (_ev: Event) => timer = isNaN(timer)
    ? _("start: setTimeout", +setTimeout(longPressDone, duration))
    : _("start: timer", timer)
  const cancel = (ev: Event) =>
    _("cancel", (ev.preventDefault(), timer = +clearTimeout(timer)))
  
   // isNaN(timer)
   //  ? _("cancel: preventDefault", ev.preventDefault())
   //  : _("cancel: clearTimeout", timer = +clearTimeout(timer))

  const noisy
    = (what: string, handler: (ev: Event) => void) =>
      (ev: Event) =>
        _(what, handler(ev),
         "timer", timer, "meta", calc.meta)
  
  el.addEventListener('touchstart', noisy("touchstart", start))
  el.addEventListener('mousedown', noisy("mousedown", start))
  el.addEventListener('touchend', noisy("touchend", cancel))
  el.addEventListener('mouseup', noisy("mouseup", cancel))  
}

const keyMap: Record<string, number> = {
//  a: 0, b: 1, c: 2, d: 3, e: 4, f: 5,
  Enter: 6, s: 7, r: 8, Backspace: 9,
  "+": 10, 7: 11, 8: 12, 9: 13,
  "-": 14, 4: 15, 5: 16, 6: 17,
  "*": 18, 1: 19, 2: 20, 3: 21,
  "/": 22, 0: 23, ".": 24, "m": 25,
  "i": 49,
}
const keydown = (ev: any) =>
  run(
  _("run after keydown", keyMap[ev.key], "ev", ev)
  )() === 'ignored' ? null : ev.preventDefault()


document.addEventListener('keydown', keydown)

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+
