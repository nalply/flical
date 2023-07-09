"use strict";
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.Calc = void 0;
var num_js_1 = require("./num.js");
function _(topic, result) {
    var args = [];
    for (var _i = 2; _i < arguments.length; _i++) {
        args[_i - 2] = arguments[_i];
    }
    console.debug.apply(console, __spreadArray([topic + ":", result], args, false));
    return result;
}
var Calc = /** @class */ (function () {
    function Calc() {
        this.x = 0;
        this.y = 0;
        this.z = 0;
        this.t = 0;
        this.lastX = 0;
        this.info = "  Long-press ALT key for help      ";
        this.hints = "< A > < B > < C > < D > < E > < F >";
        this.mode = 'push';
        this.meta = 'base';
        this.changed = function () { return null; };
    }
    Calc.prototype.dump = function () {
        return "stack " + this.x + "·" + this.y + "·" + this.z + "·" + this.t
            + " lastX " + this.lastX
            + " mode " + this.mode
            + " meta " + this.meta;
    };
    /// push up the stack with X a new number or same
    Calc.prototype.push = function (newX, mode) {
        this.t = this.z;
        this.z = this.y;
        this.y = this.x;
        if (newX)
            this.x = newX;
        if (mode)
            this.mode = mode;
        this.changed();
        _("push", this.dump());
    };
    /// down a level with X a new number or from Y
    Calc.prototype.down = function (newX) {
        newX !== null && newX !== void 0 ? newX : (newX = this.y);
        this.x = newX;
        this.y = this.z;
        this.z = this.t;
        this.mode = 'push';
        this.changed();
        _("down", this.dump());
    };
    /// replace X with a digit
    Calc.prototype.replace = function (digit, mode) {
        this.lastX = this.x;
        this.x = +digit;
        if (mode)
            this.mode = mode;
        this.changed();
        _("replace", this.dump());
    };
    /// append a digit to X
    Calc.prototype.append = function (digit) {
        this.x = (0, num_js_1.appendDigit)(this.x, digit);
        this.changed();
        _("append", this.dump());
    };
    Calc.prototype.nop = function () { };
    Calc.prototype.input = function (digit) {
        var _this = this;
        return function () {
            if (digit)
                switch (_this.mode) {
                    case 'push': return _this.push(+digit, 'append');
                    case 'replace': return _this.replace(digit, 'append');
                    case 'append': return _this.append(digit);
                }
        };
    };
    Calc.prototype.enter = function () {
        this.push(null, 'replace');
    };
    Calc.prototype.add = function () {
        this.down((0, num_js_1.add)(this.x, this.y));
    };
    Calc.prototype.sub = function () {
        console.log("sub");
    };
    Calc.prototype.mul = function () {
        //this.push(m.multiply(this.x, this.y))
    };
    Calc.prototype.div = function () {
        console.log("div");
    };
    // todo bug help then meta == alt but expected base
    Calc.prototype.doMeta = function (meta) {
        var state = { base: 'alt', alt: 'inv', inv: 'base', help: 'inv' };
        this.meta = meta !== null && meta !== void 0 ? meta : state[this.meta];
        if (meta)
            this.meta = meta;
        if (meta === 'help') {
            this.info = 'help';
        }
        this.changed();
    };
    Calc.prototype.sto = function () {
        console.log("sto");
    };
    Calc.prototype.rcl = function () {
        console.log("rcl");
    };
    Calc.prototype.left = function () {
        console.log("left");
    };
    Calc.prototype.dot = function () {
        console.log("dot");
    };
    Calc.prototype.i = function () {
        this.x = (0, num_js_1.complex)(this.x);
    };
    return Calc;
}());
exports.Calc = Calc;
// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+
