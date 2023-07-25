import { expect } from '@esm-bundle/chai'

import { 
  ty, isNumber, isComplex, isFraction, part1, part2, isNum, assertNum,
  isSimple, isInteger, isFractional, isWithImaginary, kind,
  coerce, complex, fraction,
  add, mul,
  type Num,
} from '../ts/num.js'

import { Complex } from 'complex'
import { Fraction } from 'fraction'

const notNumber = "0"
const ordNumber = -1.5
const intNumber = 1
const ordComplex = new Complex(2, 4)
const intComplex = new Complex(-2)
const realComplex = new Complex(0.5)
const nanComplex1 = new Complex(NaN, 0)
const nanComplex2 = new Complex(-1, NaN)
const infComplex1 = new Complex(Infinity, 1)
const infComplex2 = new Complex(0, -Infinity)
const ordFraction = new Fraction(3, 5)
const intFraction = new Fraction(6, 1)
// Fraction seems to disallow non-integer parts, so no tests for that

it('type properties', () => {
  expect(ty(ordNumber)).to.equal('number')
  expect(ty(ordComplex)).to.equal('complex')
  expect(ty(ordFraction)).to.equal('fraction')
  expect(ty(notNumber)).to.be.null

  expect(isNumber(ordNumber)).to.be.true
  expect(isNumber(ordComplex)).to.be.false
  expect(isNumber(ordFraction)).to.be.false
  expect(isNumber(notNumber)).to.be.false

  expect(isComplex(ordNumber)).to.be.false
  expect(isComplex(ordComplex)).to.be.true
  expect(isComplex(ordFraction)).to.be.false
  expect(isComplex(notNumber)).to.be.false

  expect(isFraction(ordNumber)).to.be.false
  expect(isFraction(ordNumber)).to.be.false
  expect(isFraction(ordFraction)).to.be.true
  expect(isFraction(notNumber)).to.be.false

  expect(part1(ordNumber)).to.equal(-1.5)
  expect(part1(ordComplex)).to.equal(2)
  expect(part1(ordFraction)).to.equal(3)
  expect(part1(notNumber)).to.be.NaN

  expect(part2(ordNumber)).to.be.NaN
  expect(part2(ordComplex)).to.equal(4)
  expect(part2(ordFraction)).to.equal(5)
  expect(part2(notNumber)).to.be.NaN

  expect(isNum(ordNumber)).to.be.true
  expect(isNum(ordComplex)).to.be.true
  expect(isNum(ordFraction)).to.be.true
  expect(isNum(notNumber)).to.be.false
  expect(isNum(Infinity)).to.be.false
  expect(isNum(-Infinity)).to.be.false
  expect(isNum(NaN)).to.be.false
  expect(isNum(nanComplex1)).to.be.false
  expect(isNum(nanComplex2)).to.be.false
  expect(isNum(infComplex1)).to.be.false
  expect(isNum(infComplex2)).to.be.false
  
  expect(() => assertNum(ordNumber)).not.to.throw
  expect(() => assertNum(ordComplex)).not.to.throw
  expect(() => assertNum(ordFraction)).not.to.throw
  expect(() => assertNum(notNumber)).to.throw
  expect(() => assertNum(infComplex1)).to.throw
})

it('math properties', () => {
  expect(isSimple(ordNumber)).to.be.true
  expect(isSimple(ordComplex)).to.be.false
  expect(isSimple(ordFraction)).to.be.false
  expect(isSimple(notNumber)).to.be.false
  expect(isSimple(NaN)).to.be.false
  expect(isSimple(Infinity)).to.be.false
  expect(isSimple(nanComplex1)).to.be.false
  expect(isSimple(NaN)).to.be.false
  expect(isSimple(Infinity)).to.be.false
  expect(isSimple(nanComplex1)).to.be.false
  expect(isSimple(nanComplex2)).to.be.false
  expect(isSimple(infComplex1)).to.be.false
  expect(isSimple(infComplex2)).to.be.false
  expect(isSimple(intNumber)).to.be.true
  expect(isSimple(intComplex)).to.be.true
  expect(isSimple(intFraction)).to.be.true

  expect(isInteger(ordNumber)).to.be.false
  expect(isInteger(ordComplex)).to.be.false
  expect(isInteger(ordFraction)).to.be.false
  expect(isInteger(notNumber)).to.be.false
  expect(isInteger(NaN)).to.be.false
  expect(isInteger(Infinity)).to.be.false
  expect(isInteger(nanComplex1)).to.be.false
  expect(isInteger(NaN)).to.be.false
  expect(isInteger(Infinity)).to.be.false
  expect(isInteger(nanComplex1)).to.be.false
  expect(isInteger(nanComplex2)).to.be.false
  expect(isInteger(infComplex1)).to.be.false
  expect(isInteger(infComplex2)).to.be.false
  expect(isInteger(intNumber)).to.be.true
  expect(isInteger(intComplex)).to.be.true
  expect(isInteger(intFraction)).to.be.true

  expect(isFractional(ordNumber)).to.be.false
  expect(isFractional(ordComplex)).to.be.false
  expect(isFractional(ordFraction)).to.be.true
  expect(isFractional(notNumber)).to.be.false
  expect(isFractional(NaN)).to.be.false
  expect(isFractional(Infinity)).to.be.false
  expect(isFractional(nanComplex1)).to.be.false
  expect(isFractional(nanComplex2)).to.be.false
  expect(isFractional(infComplex1)).to.be.false
  expect(isFractional(infComplex2)).to.be.false
  expect(isFractional(intNumber)).to.be.false
  expect(isFractional(intComplex)).to.be.false
  expect(isFractional(intFraction)).to.be.false

  expect(isWithImaginary(ordNumber)).to.be.false
  expect(isWithImaginary(ordComplex)).to.be.true
  expect(isWithImaginary(ordFraction)).to.be.false
  expect(isWithImaginary(notNumber)).to.be.false
  expect(isWithImaginary(NaN)).to.be.false
  expect(isWithImaginary(Infinity)).to.be.false
  expect(isWithImaginary(nanComplex1)).to.be.false
  expect(isWithImaginary(nanComplex2)).to.be.false
  expect(isWithImaginary(infComplex1)).to.be.false
  expect(isWithImaginary(infComplex2)).to.be.false
  expect(isWithImaginary(intNumber)).to.be.false
  expect(isWithImaginary(intComplex)).to.be.false
  expect(isWithImaginary(intFraction)).to.be.false

  expect(kind(ordNumber)).to.be.a.string('simple')
  expect(kind(ordComplex)).to.be.a.string('withImaginary')
  expect(kind(ordFraction)).to.be.a.string('fractional')
  expect(kind(notNumber)).to.be.null
  expect(kind(NaN)).to.be.null
  expect(kind(Infinity)).to.be.null
  expect(kind(nanComplex1)).to.be.null
  expect(kind(nanComplex2)).to.be.null
  expect(kind(infComplex1)).to.be.null
  expect(kind(infComplex2)).to.be.null
  expect(kind(intNumber)).to.be.a.string('integer')
  expect(kind(intComplex)).to.be.a.string('integer')
  expect(kind(intFraction)).to.be.a.string('integer')
})

it('coerce', () => {
  const n1 = ordNumber, n2 = intNumber
  const c1 = ordComplex, c2 = intComplex
  const f1 = ordFraction, f2 = intFraction
  
  expect(coerce(n1, n2)).to.deep.equal([ n1, n2 ])
  expect(coerce(c1, c2)).to.deep.equal([ c1, c2 ])
  expect(coerce(f1, f2)).to.deep.equal([ f1, f2 ])
  expect(coerce(n1, c1)).to.deep.equal([ complex(n1), c1 ])
  expect(coerce(c1, n1)).to.deep.equal([ c1, complex(n1) ])
  expect(coerce(f1, c1)).to.deep.equal([ complex(f1), c1 ])
  expect(coerce(c1, f1)).to.deep.equal([ c1, complex(f1) ])
  expect(coerce(f1, n2)).to.deep.equal([ f1, fraction(n2) ])
  expect(coerce(n2, f1)).to.deep.equal([ fraction(n2), f1 ])
})

it('binary ops', () => {
  expect(add(ordNumber, intNumber)).to.equal(-0.5)
  expect(add(ordNumber, intComplex)).to.deep.equal(complex(-3.5))
  expect(add(ordNumber, ordFraction)).to.equal(-0.9)
  expect(add(intNumber, ordFraction)).to.deep.equal(fraction(8, 5))
  expect(add(ordFraction, intFraction)).to.deep.equal(fraction(33, 5))
  expect(mul(ordNumber, intNumber)).to.equal(-1.5)
  expect(mul(ordNumber, intComplex)).to.deep.equal(complex(3))
  expect(mul(ordNumber, ordFraction)).to.be.closeTo(-0.9, 0.001)
  expect(mul(intNumber, ordFraction)).to.deep.equal(fraction(3, 5))
  expect(mul(ordFraction, intFraction)).to.deep.equal(fraction(18, 5))

  // sub div

})


// appendDigit

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+
