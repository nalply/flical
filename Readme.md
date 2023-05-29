# Flical (FLIpped CALculator)

## Introduction

**This is a work in progress. Only adding works and only partially.**

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

- TypeScript
- ESM modules
- No bundling
- Development quick roundtrip
- Not too complicated
- Possibility to reuse configuration for similar projects

## Development environment

- use relative import paths ending in `.js`
- use [@web/dev-server](
  https://modern-web.dev/guides/dev-server/typescript-and-jsx/#tsc)
- dev-server tasks:
  - live reload on save (builtin)
  - compile typescript on the fly (builtin)
  - resolve *.js imports to typescript if *.ts exists (middleware)
  - transform CommonJS to ESM (middleware)
  - open one tab for development (builtin, but not good enough)
  - web automation (middleware)

### complex.js and fraction.js as ESM

These are UMD scripts. I have to esmify them such that they fit my development 
workflow. I forked the repositories and did the change myself. They are now
git submodules under deps (to make JSPM esmify them is a bit too much yak
shaving).

### Production

- bundle
- create app with Tauri
- upload to webserver

