const width = 35

function reflowEntry([key, text]) {
  const words = text.replace(/\s+/g, " ").split(" ")
  const lines = []
  let currentLine = "";

  for (let word of words) {
    if (currentLine.length + word.length <= width) {
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
  ENTER: "Separate two entered numbers",
  ENTER_long: `
    With ENTER you separate the input of numbers. For example to add 3 and 4,
    you hit 3, then ENTER, then 4, then add. To read more about RPN, long-press
    ALT.
  `,

  ADD: "Addition: x = x + y",
  ADD_long: `
    The addition operator. Add number y to x.
    Both numbers can be a simple or a complex number or a fractional and are
    converted as needed.
    The result is moved to x and the stack is shifted down.
    The previous value of x is moved to LAST_X.
  `,

  SUB: "Subtraction: x = y - x",
  SUB_long: `
    The subtraction operator. Subtract y from x.
    Both numbers can be a simple or a complex number or a fractional and are
    converted as needed.
    The result is moved to x and the stack is shifted down.
    The previous value of x is moved to LAST_X.
  `,

  MUL: "Multiplication: x = x * y",
  MUL_long: `
    The multiplication operator. Multiply x with y.
    Both numbers can be a simple or a complex number or a fractional and are
    converted as needed.
    The result is moved to x and the stack is shifted down.
    The previous value of x is moved to LAST_X.
  `,

  DIV: "Division: x = y / x",
  DIV_long: `
    The division operator. Divide y by x.
    Both numbers can be a simple or a complex number or a fractional and are
    converted as needed.
    The result is moved to x and the stack is shifted down.
    The previous value of x is moved to LAST_X.
  `,

  DOT: "Decimal point/fraction slash",
  DOT_long: `
    Enter entry mode if not already and enters the decimal point. If
    there's already a decimal point, a fraction will be entered. The first
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
})

// ================================ <-- this is the max. width of 35 columns.
window.flical.lang.en.ALT_long = `\
1. What is ALT / INV?
2. What is RPN?

Exit this help: Hit ENTER  or <-
Scroll down:        Hit ÷  or  0
Scroll up:          Hit ×  or  2

1. What is ALT / INV?

All buttons have three functions.
Press ALT then the button for the
second function (red). Press ALT
twice for the third one (purple).

2. What is RPN?

RPN is Reverse Polish Notation.

https://www.flical.ch/what-is-rpn

RPN is an efficient calculator
notation without parentheses. An
addition looks like this: 

    3 ENTER 4 +

Do you find this weird? But know
this: Once you've got it, there's
no turning back. You'll like it.

Let's try a more complicated exam-
ple. The length of a 3D vector:
Square root of x² + y² + z².

Calculate it for (3, 4, 5).

First input 3 then hit the x² but-
ton (ALT ALT 2). You should have 9
at x. Input 4 and repeat. Input 5
amd repeat. Now you have numbers 9,
16 and 25 in the stack. Add twice.
You now have the sum 50. Hit the
square root button √x (ALT 2). 
Aaand done!

    3 x² 4 x² 5 x² + + √x

No parentheses. Different but a lot
simpler. Right? Try a few more cal-
culations to get a taste of RPN.

To learn more, exit the help (ENTER
or <-) then long press another but-
ton. There's always quick help and
for the longer text scroll down.

For help about the ALT and INV
functions press ALT once or twice
first, then long press the button.
` // ============================== <-- this is the max. width of 35 columns.

// Copyright see AUTHORS & LICENSE; SPDX-License-Identifier: ISC+
