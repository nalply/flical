document.addEventListener("keydown", handleKey);
window.addEventListener("load", _ => {
  scaleCalculatorToViewport()

  let screen =  document.querySelector("#screen")
  if (!screen) return
  
  measureLetterSpacing(screen)
  replaceUnderscores(screen)
})
//window.addEventListener("resize", scaleCalculatorToViewport)

function scaleCalculatorToViewport() {
  let main = document.querySelector("main");
  if (!main) return
  
  let { width, height } = main.getBoundingClientRect()
  let vpHeight = window.innerHeight
  let vpWidth = window.innerWidth
  console.log("vp", vpWidth, "x", vpHeight, "main", width, "x", height)

  let scaleWidth = vpWidth / width
  let scaleHeight = vpHeight / height
  let scale = Math.min(scaleWidth, scaleHeight)
  console.log("scale width", scaleWidth, "height", scaleHeight, "scale", scale)

  if (scaleCalculatorToViewport.scale == scale) return

  scaleCalculatorToViewport.scale = scale
  document.querySelector("meta[name=viewport]").setAttribute("content",
    `width=device-width,initial-scale=${scale}>`
  )
}

scaleCalculatorToViewport.scale = 0
const columns = 35

function measureLetterSpacing(screen) {
  // Hardcoded: width of #screen
  let screenWidth = 274

  let style = "style=letter-spacing:0;margin:0;padding:0"
  let text = "x".repeat(columns)
  screen.insertAdjacentHTML("afterbegin", `<span ${style}>${text}</span>`)
  let testSpan = document.querySelector("#screen span")
  let testWidth = testSpan.getBoundingClientRect().width;
  let letterSpacing = ((screenWidth - testWidth) / columns) + "px"
  testSpan.remove()
  console.log(testSpan, "width", testWidth, "letter-spacing", letterSpacing)

  screen.style.letterSpacing = letterSpacing
}

function replaceUnderscores(screen) {
  screen.innerText = screen.innerText.replaceAll("_", " ")
}

function handleKey(ev) {
  let prefixes = (ev.ctrlKey ? "C-" : "") + (ev.altKey ? "A-" : "")
  let key = prefixes + ev.key.replace(/^Arrow/, "").replace("/^Escape$", "Esc")
  wasmBindings.flical_command(wasmBindings.flical_translate_key_press(key))
  ev.preventDefault()
}

Array.from(document.querySelectorAll("button")).map(
  (button, index) => {
    let touch = _ => touched(button, index)
    button.addEventListener("mousedown", touch)
    let lift = ev => lifted(button, index, ev.button == 1)
    button.addEventListener("mouseup", lift)
  }
)

const longTime = 500
let running = null
let lastIndex = null

function touched(button, index) {
  if (running) return
  
  navigator.vibrate?.([200, 50, 50, 50, 150])
  running = setTimeout(timeout, longTime)
  lastIndex = index

  function timeout() {
    flicalExecute(index, true)
    running = null

    // make button jump back to not active
    button.classList.add('deactivated')

    // TODO if mouse moves out without mouseup then lifted() is never called
    // and the button stays showing deactivated. Perhaps there are other corner
    // cases. It's not a big problem, just click the button again and it shows
    // activation again. Whatever.
  }
}

function lifted(button, index, middle) {
if (running && lastIndex === index) {
    clearTimeout(running)
    flicalExecute(index, middle)
  }

  navigator.vibrate?.(0)
  lastIndex = null
  running = null
  button.classList.remove('deactivated')
}

function flicalExecute(index, long) {
  let command = wasmBindings.flical_translate_button_press(index, long)
  wasmBindings.flical_command(command)
}

// Copyright see AUTHORS & LICENSE; SPDX-License-Identifier: ISC+
