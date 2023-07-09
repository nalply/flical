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
var _a;
Object.defineProperty(exports, "__esModule", { value: true });
var calc_js_1 = require("./calc.js");
var calc = window.calc = new calc_js_1.Calc();
var screen = document.getElementById('screen');
if (screen)
    calc.changed = function () {
        var annunc = calc.meta === 'alt' || calc.meta === 'inv' ? calc.meta : '';
        screen.innerText = ''
            + calc.info.padEnd(35).substring(0, 35 - annunc.length) + annunc
            + '\n' + calc.y
            + '\n' + calc.x
            + '\n' + calc.hints;
    };
calc.changed();
var digitMap = [
    "a", "b", "c", "d", "e", "f",
    null, null, null, null,
    null, 7, 8, 9,
    null, 4, 5, 6,
    null, 1, 2, 3,
    null, 0, null, null,
];
var suppressMetaOnce = false;
function meta(meta) {
    console.log("meta()", "meta", meta, "calc.meta", calc.meta, "suppressMetaOnce", suppressMetaOnce);
    if (suppressMetaOnce)
        return suppressMetaOnce = false;
    if (meta === 'help')
        suppressMetaOnce = true;
    calc.doMeta(meta);
}
var run = function (i) { return function () {
    var _a;
    if (i == null || i < 0 || i > 26 || isNaN(i))
        return (console.log("invalid i", i), 'ignored');
    if (i === 25)
        return meta();
    switch (calc.meta) {
        case 'base': switch (i) {
            case 6: return calc.enter();
            case 7: return calc.sto();
            case 8: return calc.rcl();
            case 9: return calc.left();
            case 10: return calc.add();
            case 14: return calc.sub();
            case 18: return calc.mul();
            case 22: return calc.div();
            case 24: return calc.dot();
            default: return calc.input((_a = digitMap[i !== null && i !== void 0 ? i : NaN]) !== null && _a !== void 0 ? _a : null)();
        }
        case 'alt':
            switch (i) {
                case 24: return calc.i();
            }
            return 'ignored';
    }
}; };
var buttons = __spreadArray([], document.getElementsByTagName('button'), true);
buttons.map(function (button, i) { return button.addEventListener('click', run(i)); });
var keyMap = {
    a: 0, b: 1, c: 2, d: 3, e: 4, f: 5,
    Enter: 6, s: 7, r: 8, Backspace: 9,
    "+": 10, 7: 11, 8: 12, 9: 13,
    "-": 14, 4: 15, 5: 16, 6: 17,
    "*": 18, 1: 19, 2: 20, 3: 21,
    "/": 22, 0: 23, ".": 24, "m": 25,
};
document.addEventListener('keydown', function (ev) { return run(keyMap[ev.key])() === 'ignored' ? null : ev.preventDefault(); });
var metaButton = (_a = buttons.at(-1)) !== null && _a !== void 0 ? _a : document.createElement("p");
onLongPress(metaButton, 600, function () { return meta('help'); });
function onLongPress(el, duration, cb) {
    function _(_topic, result) {
        var _args = [];
        for (var _i = 2; _i < arguments.length; _i++) {
            _args[_i - 2] = arguments[_i];
        }
        //console.debug(_topic + ":", result, ..._args)
        return result;
    }
    var timer = NaN;
    var longPressDone = function () { return _("longPressDone", cb(), "meta", calc.meta); };
    var start = function (_ev) { return timer = isNaN(timer)
        ? _("start: setTimeout", +setTimeout(longPressDone, duration))
        : _("start: timer", timer); };
    var cancel = function (ev) {
        return _("cancel", (ev.preventDefault(), timer = +clearTimeout(timer)));
    };
    // isNaN(timer)
    //  ? _("cancel: preventDefault", ev.preventDefault())
    //  : _("cancel: clearTimeout", timer = +clearTimeout(timer))
    var noisy = function (what, handler) {
        return function (ev) { return _(what, handler(ev), "timer", timer, "meta", calc.meta); };
    };
    el.addEventListener('touchstart', noisy("touchstart", start));
    el.addEventListener('mousedown', noisy("mousedown", start));
    el.addEventListener('touchend', noisy("touchend", cancel));
    el.addEventListener('mouseup', noisy("mouseup", cancel));
}
// Copyright: 2023 Daniel Ly; SPDX-License-Identifier: ISC+
