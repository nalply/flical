import { Calc, type Meta } from './calc.js'
import { type Digit } from './num.js'

const calc = (window as any).calc = new Calc()

const screen = document.getElementById('screen')

if (screen) calc.changed = function() {
  const annunc = calc.meta === 'alt' || calc.meta === 'inv' ? calc.meta : ''
  
  screen.innerText = ''
    + calc.info.padEnd(35).substring(0, 35 - annunc.length) + annunc
    + '\n' + calc.y
    + '\n' + calc.x
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
 
const run = (i?: number) => function(): void | 'ignored' {
  if (i == null || i < 0 || i > 26 || isNaN(i))
    return (console.log("invalid i", i), 'ignored')

  if (i === 25) return meta()

  switch (calc.meta) {
    case 'base': switch (i) {
      case 6: return calc.enter()
      case 7: return calc.sto()
      case 8: return calc.rcl()
      case 9: return calc.left()
      case 10: return calc.add()
      case 14: return calc.sub()
      case 18: return calc.mul()
      case 22: return calc.div()
      case 24: return calc.dot()
      default: return calc.input(digitMap[i ?? NaN] ?? null)()
    }
    case 'alt': switch (i) {
      case 24: return calc.i()
    }
    return 'ignored'
  }
}
let buttons = [...document.getElementsByTagName('button')]
buttons.map((button, i) => button.addEventListener('click', run(i)))

const keyMap: Record<string, number> = {
  a: 0, b: 1, c: 2, d: 3, e: 4, f: 5,
  Enter: 6, s: 7, r: 8, Backspace: 9,
  "+": 10, 7: 11, 8: 12, 9: 13,
  "-": 14, 4: 15, 5: 16, 6: 17,
  "*": 18, 1: 19, 2: 20, 3: 21,
  "/": 22, 0: 23, ".": 24, "m": 25,
}
document.addEventListener('keydown',
  ev => run(keyMap[ev.key])() === 'ignored' ? null : ev.preventDefault()
)

const metaButton = buttons.at(-1) ?? document.createElement("p")
onLongPress(metaButton, 600, () => meta('help'))

function onLongPress(el: HTMLElement, duration: number, cb: () => void) {
  function _(_topic: any, result: any, ..._args: any[]): any {
    //console.debug(_topic + ":", result, ..._args)
    return result
  }
  
  let timer = NaN
  const longPressDone = () => _("longPressDone", cb(), "meta", calc.meta)
    
  const start = (_ev: Event) => timer = isNaN(timer)
    ? _("start: setTimeout", +setTimeout(longPressDone, duration))
    : _("start: timer", timer)
  const cancel = (ev: Event) =>
    _("cancel", (ev.preventDefault(), timer = +clearTimeout(timer)))
  
   // isNaN(timer)
   //  ? _("cancel: preventDefault", ev.preventDefault())
   //  : _("cancel: clearTimeout", timer = +clearTimeout(timer))

  const noisy = (what: string, handler: (ev: Event) => void) =>
    (ev: Event) => _(what, handler(ev), "timer", timer, "meta", calc.meta)
  
  el.addEventListener('touchstart', noisy("touchstart", start))
  el.addEventListener('mousedown', noisy("mousedown", start))
  el.addEventListener('touchend', noisy("touchend", cancel))
  el.addEventListener('mouseup', noisy("mouseup", cancel))  
}

// Copyright: 2023 Daniel Ly; SPDX-License-Identifier: ISC+
