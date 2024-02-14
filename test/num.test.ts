import { expect } from '@esm-bundle/chai'

import {
  isComplex, Complex, isFraction, Fraction, isNumber,
  ty, isNum, isSimple, isInteger, isFractional, isWithImaginary, kind,
  type Num,
} from '../ts/nums.js'

// todo: same as global.d.ts, don't know why global.d.ts didn't work here
declare global {
  export interface Number extends Num {}
}

const notNumber = "0"
const ordNumber = -1.5
const intNumber = 1
const ordComplex = Complex.from(2, 4)
const intComplex  = Complex.from(-2)
const realComplex = Complex.from(0.5)
const ordFraction = Fraction.from(3, 5)
const intFraction = Fraction.from(6)

function createInvalidComplex(re: any, im: any) {
  const c = Complex.from()
  c.x.re = re
  c.x.im = im
  return c
}

const nanComplex1 = createInvalidComplex(NaN, 0)
const nanComplex2 = createInvalidComplex(-1, NaN)
const infComplex1 = createInvalidComplex(Infinity, 1)
const infComplex2 = createInvalidComplex(0, -Infinity)

it('Complex Infinity and NaN throwing', () => {
  expect(() => Complex.from(NaN)).to.throw()
  expect(() => Complex.from(-1, NaN)).to.throw()
  expect(() => Complex.from(Infinity, 1)).to.throw()
  expect(() => Complex.from(0, -Infinity)).to.throw()
})

it('Fraction Infinity and NaN throwing', () => {
  expect(() => Fraction.from(NaN)).to.throw()
  expect(() => Fraction.from(1, Infinity)).to.throw()
})

it('type properties', () => {
  expect(ty(ordNumber)).to.equal('number')
  expect(ty(ordComplex)).to.equal('Complex')
  expect(ty(ordFraction)).to.equal('Fraction')
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

  expect(ordNumber.parts).to.deep.equal([ -1.5 ])
  expect(ordComplex.parts).to.deep.equal([ 2, 4 ])
  expect(ordFraction.parts).to.deep.equal([ 3, 5 ])

  expect(isNum(ordNumber)).to.be.true
  expect(isNum(realComplex)).to.be.true
  expect(isNum(ordFraction)).to.be.true
  expect(isNum(notNumber)).to.be.false
  expect(isNum(Infinity)).to.be.false
  expect(isNum(-Infinity)).to.be.false
  expect(isNum(NaN)).to.be.false
  expect(isNum(nanComplex1)).to.be.false
  expect(isNum(nanComplex2)).to.be.false
  expect(isNum(infComplex1)).to.be.false
  expect(isNum(infComplex2)).to.be.false
  
  expect(() => ordNumber.assertValid()).not.to.throw
  expect(() => ordComplex.assertValid()).not.to.throw
  expect(() => ordFraction.assertValid()).not.to.throw
  expect(() => infComplex1.assertValid()).to.throw
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
  const n = ordNumber, i = intNumber, f = ordFraction
  const c = intComplex, f2 = intFraction
  expect(n.add( i).parts).to.deep.equal([ -0.5 ])
  expect(n.add( c).parts).to.deep.equal([ -3.5, 0 ])
  expect(n.add( f).parts).to.deep.equal([ -0.9 ])
  expect(i.add( f).parts).to.deep.equal([ 8, 5 ])
  expect(f.add( f2).parts).to.deep.equal([ 33, 5 ])
  expect(n.mul( i).parts).to.deep.equal([ -1.5 ])
  expect(n.mul( c).parts).to.deep.equal([ 3, 0 ])
  expect(n.mul( f).parts[0]).to.be.closeTo(-0.9, 0.001)
  expect(i.mul( f).parts).to.deep.equal([ 3, 5 ])
  expect(i.mul( f2).parts).to.deep.equal([ 18, 5 ])

  // sub div

})


// appendDigit

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+
