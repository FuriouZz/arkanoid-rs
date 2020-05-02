#![allow(dead_code)]
pub type KeyCode = u32;

pub const BACKSPACE: KeyCode = 8;
pub const TAB: KeyCode = 9;
pub const ENTER: KeyCode = 13;
pub const SHIFT: KeyCode = 16;
pub const CTRL: KeyCode = 17;
pub const ALT: KeyCode = 18;
pub const ESCAPE: KeyCode = 27;
pub const SPACE: KeyCode = 32;
pub const PGUP: KeyCode = 33;
pub const PGDOWN: KeyCode = 34;
pub const END: KeyCode = 35;
pub const HOME: KeyCode = 36;
pub const LEFT: KeyCode = 37;
pub const UP: KeyCode = 38;
pub const RIGHT: KeyCode = 39;
pub const DOWN: KeyCode = 40;
pub const INSERT: KeyCode = 45;
pub const DELETE: KeyCode = 46;

pub const QWERTY_EQUALS: KeyCode = 187;
pub const QWERTY_MINUS: KeyCode = 189;
pub const QWERTY_TILDE: KeyCode = 192;
pub const QWERTY_BRACKET_LEFT: KeyCode = 219;
pub const QWERTY_BRACKET_RIGHT: KeyCode = 221;
pub const QWERTY_SEMICOLON: KeyCode = 186;
pub const QWERTY_QUOTE: KeyCode = 222;
pub const QWERTY_BACKSLASH: KeyCode = 220;
pub const QWERTY_COMMA: KeyCode = 188;
pub const QWERTY_PERIOD: KeyCode = 190;
pub const QWERTY_SLASH: KeyCode = 191;
pub const INTL_BACKSLASH: KeyCode = 226; // Backslash located next to left shift on some keyboards. Warning: Not available on HLSDL.
pub const LEFT_WINDOW_KEY: KeyCode = 91;
pub const RIGHT_WINDOW_KEY: KeyCode = 92;
pub const CONTEXT_MENU: KeyCode = 93;
// pub const PRINT_SCREEN = // Only available on SDL

pub const PAUSE_BREAK: KeyCode = 19;
pub const CAPS_LOCK: KeyCode = 20;
pub const NUM_LOCK: KeyCode = 144;
pub const SCROLL_LOCK: KeyCode = 145;

pub const NUMBER_0: KeyCode = 48;
pub const NUMBER_1: KeyCode = 49;
pub const NUMBER_2: KeyCode = 50;
pub const NUMBER_3: KeyCode = 51;
pub const NUMBER_4: KeyCode = 52;
pub const NUMBER_5: KeyCode = 53;
pub const NUMBER_6: KeyCode = 54;
pub const NUMBER_7: KeyCode = 55;
pub const NUMBER_8: KeyCode = 56;
pub const NUMBER_9: KeyCode = 57;

pub const NUMPAD_0: KeyCode = 96;
pub const NUMPAD_1: KeyCode = 97;
pub const NUMPAD_2: KeyCode = 98;
pub const NUMPAD_3: KeyCode = 99;
pub const NUMPAD_4: KeyCode = 100;
pub const NUMPAD_5: KeyCode = 101;
pub const NUMPAD_6: KeyCode = 102;
pub const NUMPAD_7: KeyCode = 103;
pub const NUMPAD_8: KeyCode = 104;
pub const NUMPAD_9: KeyCode = 105;

pub const A: KeyCode = 65;
pub const B: KeyCode = 66;
pub const C: KeyCode = 67;
pub const D: KeyCode = 68;
pub const E: KeyCode = 69;
pub const F: KeyCode = 70;
pub const G: KeyCode = 71;
pub const H: KeyCode = 72;
pub const I: KeyCode = 73;
pub const J: KeyCode = 74;
pub const K: KeyCode = 75;
pub const L: KeyCode = 76;
pub const M: KeyCode = 77;
pub const N: KeyCode = 78;
pub const O: KeyCode = 79;
pub const P: KeyCode = 80;
pub const Q: KeyCode = 81;
pub const R: KeyCode = 82;
pub const S: KeyCode = 83;
pub const T: KeyCode = 84;
pub const U: KeyCode = 85;
pub const V: KeyCode = 86;
pub const W: KeyCode = 87;
pub const X: KeyCode = 88;
pub const Y: KeyCode = 89;
pub const Z: KeyCode = 90;

pub const F1: KeyCode = 112;
pub const F2: KeyCode = 113;
pub const F3: KeyCode = 114;
pub const F4: KeyCode = 115;
pub const F5: KeyCode = 116;
pub const F6: KeyCode = 117;
pub const F7: KeyCode = 118;
pub const F8: KeyCode = 119;
pub const F9: KeyCode = 120;
pub const F10: KeyCode = 121;
pub const F11: KeyCode = 122;
pub const F12: KeyCode = 123;
// Extended F keys
pub const F13: KeyCode = 124;
pub const F14: KeyCode = 125;
pub const F15: KeyCode = 126;
pub const F16: KeyCode = 127;
pub const F17: KeyCode = 128;
pub const F18: KeyCode = 129;
pub const F19: KeyCode = 130;
pub const F20: KeyCode = 131;
pub const F21: KeyCode = 132;
pub const F22: KeyCode = 133;
pub const F23: KeyCode = 134;
pub const F24: KeyCode = 135;

pub const NUMPAD_MULT: KeyCode = 106;
pub const NUMPAD_ADD: KeyCode = 107;
pub const NUMPAD_ENTER: KeyCode = 108;
pub const NUMPAD_SUB: KeyCode = 109;
pub const NUMPAD_DOT: KeyCode = 110;
pub const NUMPAD_DIV: KeyCode = 111;

pub const MOUSE_LEFT: KeyCode = 0;

/* The opposite on the web */
//pub const MOUSE_RIGHT: KeyCode      = 1;
//pub const MOUSE_MIDDLE: KeyCode     = 2;
pub const MOUSE_RIGHT: KeyCode = 2;
pub const MOUSE_MIDDLE: KeyCode = 1;

pub const MOUSE_BACK: KeyCode = 3;
pub const MOUSE_FORWARD: KeyCode = 4;
pub const MOUSE_WHEEL_UP: KeyCode = 5;
pub const MOUSE_WHEEL_DOWN: KeyCode = 6;

/** a bit that is set for left keys **/
pub const LOC_LEFT: KeyCode = 256;
/** a bit that is set for right keys **/
pub const LOC_RIGHT: KeyCode = 512;
