# Flical (FLIpped CALculator)

## Introduction

**This is a work in progress.**

This is a [RPN calculator
](https://en.wikipedia.org/wiki/Reverse_Polish_notation).

<img alt=Screenshot src=flical.png width=250>

If you don't know what RPN means, please read the [RPN Wikipedia
page](https://en.wikipedia.org/wiki/Reverse_Polish_notation) first
before continuing. 

A quick example: To calculate a vector length, `sqrt(x² + y² + z²)`,
you do:

- Enter first number then square
- Enter second number then quare, then add
- Enter third number, then square, then add, then square-root

If you are used to RPN you will find this a lot easier than with the more
common algebraic calculators, especially you don't need parentheses.

## Goals

- Simplicity
- Completeness for typical RPN usage for math and computer science students
- External scripting in a Forth-like language

## Planned Features

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
- Display with four rows of 35 cells
- Number types
  - double precision IEEE 754 numbers
  - fractions on 64 bit integers
  - complex numbers on double precision IEEE 754 numbers
  - 64 bit integers in hexadecimal, decimal, octal and binary display

## Development Goals

- Rust
- Seed
- Webassembly

## Development environment

<small>Copyright see AUTHORS & LICENSE; SPDX-License-Identifier: ISC+</small>
