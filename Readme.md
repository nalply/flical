# Flical (FLIpped CALculator)

## A work in progress

If you like, fill out this short questionnaire on
[Google forms](https://forms.gle/wUQL45x5wCq9Eodx5) with four questions
about calculators and RPN. This helps me to find out what people care
about. Thank you! I plan to publish the results after some time.

<img alt=Screenshot src=flical.png width=250>

## The state of things

- Basics work
  - reals, fractionals and complex numbers
  - simple arithmetic operations, frac, int, round, abs
  - inbuilt documentation
  - key mapping
  - mode ALT and INV
- AppImage bundling with Tauri works
- Todo
  - more operations (trigonometrics, roots, powers, logic, etc.)
  - stack management
  - more testing
  - configuration by user
  - extensions
  - home page

## Goals

- Small and simple but complete online RPN calculator
- Extensions for the A-F buttons by external libraries, for example:
  - A => scientific constants
  - E => statistics
  - F => boolean operations for 64 bit integers (fraction with denom 1)

## Features

- 26 keys
  - Basic arithmetic operators
  - Stack management and registers
  - Backspace and undo
  - Elementary mathematical functions
  - Display, percent, random number, decimal time, truncating, factorial
  - Hexadecimal letters A to F, also used as function keys
  - Mode key (ALT and INV)
- Stack with four places (x, y, z, t)
- Sixteen registers
- Display with four rows of 35 columns
- Number types
  - double precision IEEE 754 numbers
  - fractions on 64 bit integers
  - complex numbers on double precision IEEE 754 numbers

## Development

- Rust
- Trunk
- Webassembly
- Tauri
- No framework

<sup>Copyright see AUTHORS & LICENSE; SPDX-License-Identifier: ISC+</sup>
