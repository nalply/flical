document.addEventListener("keydown", handleKey);
window.addEventListener("load", measureScreenLetterSpacing);

function measureScreenLetterSpacing() {
  let screen =  document.querySelector("#screen")
  console.log("measureScreenLetterSpacing: screen", screen)
  
  if (!screen) return

  // Hardcoded: outer width of #screen
  let screenWidth = 273

  let style = "style=letter-spacing:0;margin:0;padding:0"
  let text = "x".repeat(35)
  screen.insertAdjacentHTML("afterbegin", `<span ${style}>${text}</span>`)
  let testSpan = document.querySelector("#screen span")
  let testWidth = testSpan.getBoundingClientRect().width;
  let letterSpacing = ((screenWidth - testWidth) / 35) + "px"
  testSpan.remove()
  console.log(testSpan, "width", testWidth, "letter-spacing", letterSpacing)

  screen.style.letterSpacing = letterSpacing
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
    button.addEventListener("touchstart", touch)
    button.addEventListener("mousedown", touch)
    let lift = _ => lifted(button, index)
    button.addEventListener("touchend", lift)
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

function lifted(button, index) {
if (running && lastIndex === index) {
    clearTimeout(running)
    flicalExecute(index, false)
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
