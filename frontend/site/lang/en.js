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
  ALT_long: `
    Many buttons have three functions. One is printed in bright yellow, one
    in red for the ALT function and one in purple for the INV. function. Press
    ALT for the ALT function and press ALT twice for the INV function. The
    screen shows ALT or INV in the top right corner.
  `,
})

// ================================ <-- this is the max. width of 35 columns.
window.flical.lang.en.ENTER_long = `\
Exit help screen: ENTER or <-
Scroll down: ÷ or 0      up: × or 2

There's help screen for all buttons
by long press.

== An Introduction: What is RPN? ==

RPN is Reverse Polish Notation. RPN
is an easy and efficient calculator
notation without parentheses.

Add 3 and 4:   3  ENTER  4  +

Weird? Let me tell you: Once you
get it you like it. It's a good
skill. Take a moment to learn it!

Rule: Numbers, then the operation!
Try it out! Close this help with
ENTER and then return back here to
learn more.

Welcome back!

Did you try a more complicated cal-
culation? The trick is: Put inter-
mediate results on the stack! Did
you see the letters x, y, z and t
to the left? That's the stack!

Let's use it!

Task: Length of a 3D vector. It's
the square root of x² + y² + z².

What's the length of (3, 4, 5)?

Input 3 then hit the x² button (ALT
ALT 2). Now you have 9 at x. Input
4 and repeat. Input 5 amd repeat.
Now you have 9, 16 and 25. Add
twice by pressing + twice. Then hit
the square root button √x (ALT 2). 
Aaand done!

  3  x²  4  x²  5  x²  +  +  √x

No parentheses. Different but a lot
simpler. Right? Try a few more cal-
culations to get a taste of RPN.

Have fun!
` // ============================== <-- this is the max. width of 35 columns.

// Copyright see AUTHORS & LICENSE; SPDX-License-Identifier: ISC+
