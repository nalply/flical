"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.add = void 0;
var fraction_js_1 = require("fraction.js");
function add(x, y) {
    return new fraction_js_1.default(+x).add(new fraction_js_1.default(+y));
}
exports.add = add;
/*
const unimplemented = () => new TypeError("unimplemented")

const _isReal = (n: number) => isNaN(n) || !Number.isFinite(n)


export function real(n: number | Digit): number {
  return +n
}

/// Create a fraction
export function fraction(num: number, den: number): _m.Fraction {
  return m.fraction(num, den)
}

/// Create a complex
export function complex(re: number, im: number): _m.Complex {
  return m.complex(re, im)
}

/*
  /// Absolute value
  /// Convert fractional to real, id for complex
  unfraction(): _Num {
    return new _Num()
    //return this.isFractional()
    //  ? Num.real(this.num / this.den)
    //  : this
  }

  /// Create a new number by appending a digit
  appendDigit(_digit: Digit) {
  }

  toString(): string {
    if (this.isSimple())
      return this.num.toString()
    
    throw unimplemented()
  }

  valueOf(): number {
    return 0
  }
}
*/
