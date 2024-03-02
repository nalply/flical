const width = 35

function reflowEntry([key, text]) {
  const words = text.replace(/\s+/g, " ").split(" ")
  const lines = []
  let currentLine = "";

  for (let word of words) {
    if (currentLine.length + word.length < width) {
      currentLine += (currentLine === "" ? "" : " ") + word
    } else {
      lines.push(currentLine)
      currentLine = word.length > width ? word.substring(0, width) : word
    }
  }
  if (currentLine) lines.push(currentLine)

  return [key, lines.join('\n')]
}

function reflow(obj) {
  return Object.fromEntries(Object.entries(obj).map(reflowEntry))
}

if (!window.flical) window.flical = {}
if (!window.flical.lang) window.flical.lang = {}

window.flical.lang.en = reflow({
  ENTER: "Enter another number",

  ADD: "Addition: x = x + y",
  ADD_long: `
    The addition operator. Add number y to x.
    Both numbers can be a simple or a complex number or a quotient and are
    converted as needed.
    The result is moved to x and the stack is shifted down.
    The previous value of x is moved to LAST x.
  `,

  SUB: "Subtraction: x = y - x",
  SUB_long: `
    The subtraction operator. Subtract y from x.
    Both numbers can be a simple or a complex number or a quotient and are
    converted as needed.
    The result is moved to x and the stack is shifted down.
    The previous value of x is moved to LAST x.
  `,

  MUL: "Multiplication: x = x * y",
  MUL_long: `
    The multiplication operator. Multiply x with y.
    Both numbers can be a simple or a complex number or a quotient and are
    converted as needed.
    The result is moved to x and the stack is shifted down.
    The previous value of x is moved to LAST x.
  `,

  DIV: "Division: x = y / x",
  DIV_long: `
    The division operator. Divide y by x.
    Both numbers can be a simple or a complex number or a quotient and are
    converted as needed.
    The result is moved to x and the stack is shifted down.
    The previous value of x is moved to LAST x.
  `,

  DOT: "Decimal point/quotient slash",
  DOT_long: `
    Enter entry mode if not already and enters the decimal point. If
    there's already a decimal point, a quotient will be entered. The first
    decimal point separates the integer part from te numerator and the second
    decimal point is the slash separating the numerator and denominator.
    Examples: 2 . 3 . 8 gives 2 3/8, 2 . . 5 or . 2 . 5 gives 2 / 5.
  `,

  I: "Imaginary unit i",
  I_long: `
    Enter the imaginary unit. It serves as a separator between the
    real and the imaginary part of a comple number. By itself it is the
    imaginary unit (the square root of -1).
  `,

  E: "Exponent/Hex digit E",
  E_long: `
    Enter the exponent of a simple number or of the real or imaginary part
    of a complex number. In hex mode E enters the hex digit E instead.
  `,

  UP: "Shift up the stack",
  UP_long: `
    Shift up the stack, this means duplicate x and move all stack elements one
    step up.
  `,

  ALT: "Second and third function",
  ALT_long: `
    Many buttons have three functions. One is printed in bright yellow, one
    in red for the ALT function and one in purple for the INV function. Press
    ALT once for ALT and twice for INV. An indicator ALT or INV is shwn in the
    top right corner.
  `,
})

// ================================ <-- this is the max. width of 35 columns.
window.flical.lang.en.ENTER_long = `\
     Button help:   Long press
   Scroll down: 0   Scroll up: 2
   
==  A Tiny Teaser: What is RPN?  ==

RPN is Reverse Polish Notation. RPN
is an easy and efficient calculator
notation without parentheses.

Add 3 and 4:      3 ENTER 4 +

A simple rule: Numbers first, then
the operation. To learn more go to

       www.flical.com/tutor

Have fun!
` // ============================== <-- this is the max. width of 35 columns.

// Copyright see AUTHORS & LICENSE; SPDX-License-Identifier: ISC+
