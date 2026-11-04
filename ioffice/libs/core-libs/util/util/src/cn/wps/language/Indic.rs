#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use std::sync::atomic::{AtomicU32, Ordering};

// Java field: int TYPE_CONSONANT
pub(crate) const TYPE_CONSONANT: i32 = 1;
// Java field: int TYPE_DEPENDENT
pub(crate) const TYPE_DEPENDENT: i32 = 2;
// Java field: int TYPE_VIRAMA
pub(crate) const TYPE_VIRAMA: i32 = 3;
// Java field: int TYPE_UNKNOWN
pub(crate) const TYPE_UNKNOWN: i32 = 4;
// Java field: char virama
static VIRAMA: AtomicU32 = AtomicU32::new(2509);

fn java_char_get_type(c: char) -> i32 {
    let cp = c as u32;
    if c.is_alphabetic() {
        if c.is_uppercase() { return 1; }
        if c.is_lowercase() { return 2; }
        return 5;
    }
    if is_non_spacing_mark(cp) { return 6; }
    if is_enclosing_mark(cp) { return 7; }
    if is_combining_spacing_mark(cp) { return 8; }
    0
}

fn is_non_spacing_mark(cp: u32) -> bool {
    matches!(cp,
        0x0300..=0x036F |
        0x0901..=0x0902 | 0x093C | 0x0941..=0x0948 | 0x094D |
        0x0951..=0x0957 | 0x0962..=0x0963 |
        0x0981 | 0x09BC | 0x09C1..=0x09C4 | 0x09CD | 0x09E2..=0x09E3 |
        0x0A01..=0x0A02 | 0x0A3C | 0x0A41..=0x0A42 | 0x0A47..=0x0A48 |
        0x0A4B..=0x0A4D | 0x0A70..=0x0A71 |
        0x0A81..=0x0A82 | 0x0ABC | 0x0AC1..=0x0AC5 | 0x0AC7..=0x0AC8 | 0x0ACD |
        0x0B01 | 0x0B3C | 0x0B3F | 0x0B41..=0x0B43 | 0x0B4D | 0x0B56 |
        0x0B82 | 0x0BC0 | 0x0BCD |
        0x0C3E..=0x0C40 | 0x0C46..=0x0C48 | 0x0C4A..=0x0C4D | 0x0C55..=0x0C56 |
        0x0CBF | 0x0CC6 | 0x0CCC..=0x0CCD |
        0x0D41..=0x0D43 | 0x0D4D |
        0x0DCA |
        0x0DD2..=0x0DD4 | 0x0DD6 |
        0x17C6 | 0x17C9..=0x17D3 | 0x17D2
    )
}

fn is_combining_spacing_mark(cp: u32) -> bool {
    matches!(cp,
        0x0903 | 0x093E..=0x0940 | 0x0949..=0x094C |
        0x0982..=0x0983 | 0x09BE..=0x09C0 | 0x09C7..=0x09C8 | 0x09CB..=0x09CC |
        0x0A03 | 0x0A3E..=0x0A40 |
        0x0A83 | 0x0ABE..=0x0AC0 | 0x0AC9 | 0x0ACB..=0x0ACC |
        0x0B02..=0x0B03 | 0x0B3E | 0x0B40 | 0x0B47..=0x0B48 | 0x0B4B..=0x0B4C |
        0x0BBE..=0x0BBF | 0x0BC1..=0x0BC2 | 0x0BC6..=0x0BC8 | 0x0BCA..=0x0BCC |
        0x0C01..=0x0C03 | 0x0C41..=0x0C44 |
        0x0C82..=0x0C83 | 0x0CBE | 0x0CC0..=0x0CC4 | 0x0CC7..=0x0CC8 | 0x0CCA..=0x0CCB |
        0x0D02..=0x0D03 | 0x0D3E..=0x0D40 | 0x0D46..=0x0D48 | 0x0D4A..=0x0D4C |
        0x0D82..=0x0D83 | 0x0DCF..=0x0DD1 | 0x0DD8..=0x0DDF |
        0x17BE..=0x17C5 | 0x17C7..=0x17C8
    )
}

fn is_enclosing_mark(cp: u32) -> bool {
    matches!(cp, 0x0488..=0x0489 | 0x20DD..=0x20E0 | 0x20E2..=0x20E4 | 0xA670..=0xA672)
}

// Java: public class Indic
pub struct Indic;

// Java: public class Indic
impl Indic {
    // Java: Indic()
    pub fn new() -> Self {
        // Java: super();
        Indic
    }

    // Java: static boolean needSplit(char last, char cur, int unicodeType)
    pub fn needSplit(last: char, cur: char, unicode_type: i32) -> bool {
        // Java: if (cur != 8205 && cur != 8204)
        if cur != '\u{200D}' && cur != '\u{200C}' {
            // Java: if (last != 8205 && last != 8204)
            if last != '\u{200D}' && last != '\u{200C}' {
                // Java: switch (unicodeType)
                match unicode_type {
                    // Java: case 21:
                    21 => {
                        // Java: virama = 2381;
                        VIRAMA.store(2381, Ordering::Relaxed);
                        // Java: break;
                    }
                    // Java: case 22:
                    22 => {
                        // Java: virama = 2509;
                        VIRAMA.store(2509, Ordering::Relaxed);
                        // Java: break;
                    }
                    // Java: case 23:
                    23 => {
                        // Java: virama = 2637;
                        VIRAMA.store(2637, Ordering::Relaxed);
                        // Java: break;
                    }
                    // Java: case 24:
                    24 => {
                        // Java: virama = 2765;
                        VIRAMA.store(2765, Ordering::Relaxed);
                        // Java: break;
                    }
                    // Java: case 25:
                    25 => {
                        // Java: virama = 2893;
                        VIRAMA.store(2893, Ordering::Relaxed);
                        // Java: break;
                    }
                    // Java: case 26:
                    26 => {
                        // Java: virama = 3021;
                        VIRAMA.store(3021, Ordering::Relaxed);
                        // Java: break;
                    }
                    // Java: case 27:
                    27 => {
                        // Java: virama = 3149;
                        VIRAMA.store(3149, Ordering::Relaxed);
                        // Java: break;
                    }
                    // Java: case 28:
                    28 => {
                        // Java: virama = 3277;
                        VIRAMA.store(3277, Ordering::Relaxed);
                        // Java: break;
                    }
                    // Java: case 29:
                    29 => {
                        // Java: virama = 3405;
                        VIRAMA.store(3405, Ordering::Relaxed);
                        // Java: break;
                    }
                    // Java: case 30:
                    30 => {
                        // Java: virama = 3530;
                        VIRAMA.store(3530, Ordering::Relaxed);
                        // Java: break;
                    }
                    // Java: case 31:
                    // Java: case 32:
                    // Java: case 34:
                    // Java: case 35:
                    // Java: case 36:
                    // Java: case 37:
                    // Java: case 38:
                    // Java: case 39:
                    // Java: case 40:
                    // Java: case 41:
                    // Java: case 42:
                    // Java: case 43:
                    // Java: case 44:
                    // Java: case 45:
                    // Java: case 46:
                    // Java: default:
                    31 | 32 | 34..=46 => {
                        // Java: return false;
                        return false;
                    }
                    // Java: case 33:
                    33 => {
                        // Java: virama = 3972;
                        VIRAMA.store(3972, Ordering::Relaxed);
                        // Java: break;
                    }
                    // Java: case 47:
                    47 => {
                        // Java: virama = 6098;
                        VIRAMA.store(6098, Ordering::Relaxed);
                    }
                    _ => {
                        return false;
                    }
                }
                // Java: return needSpe(last, cur);
                return Self::needSpe(last, cur);
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            return false;
        }
    }

    // Java: static boolean needSpe(char last, char cur)
    fn needSpe(last: char, cur: char) -> bool {
        // Java: int curType = getCharType(cur);
        let cur_type = Self::getCharType(cur);
        // Java: if (4 == curType)
        if 4 == cur_type {
            // Java: return true;
            return true;
        }
        // Java: else if (2 == curType)
        else if 2 == cur_type {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: int lastType = getCharType(last);
            let last_type = Self::getCharType(last);
            // Java: if (4 == lastType)
            if 4 == last_type {
                // Java: return true;
                return true;
            }
            // Java: else
            else {
                // Java: return 1 == curType && 3 != lastType;
                return 1 == cur_type && 3 != last_type;
            }
        }
    }

    // Java: static int getCharType(char c)
    fn getCharType(c: char) -> i32 {
        let virama_val = VIRAMA.load(Ordering::Relaxed);
        // Java: if (c == virama)
        if c as u32 == virama_val {
            // Java: return 3;
            return 3;
        }
        // Java: else
        else {
            // Java: int charType = Character.getType(c);
            let char_type = java_char_get_type(c);
            // Java: switch (charType)
            match char_type {
                // Java: case 5:
                5 => {
                    // Java: return 1;
                    return 1;
                }
                // Java: case 6:
                // Java: case 8:
                6 | 8 => {
                    // Java: return 2;
                    return 2;
                }
                // Java: case 7:
                // Java: default:
                _ => {
                    // Java: return 4;
                    return 4;
                }
            }
        }
    }

    // Java: static boolean isIndic(int unicodeType)
    pub fn isIndic(unicode_type: i32) -> bool {
        // Java: switch (unicodeType)
        match unicode_type {
            // Java: case 21:
            // Java: case 22:
            // Java: case 23:
            // Java: case 24:
            // Java: case 25:
            // Java: case 26:
            // Java: case 27:
            // Java: case 28:
            // Java: case 29:
            // Java: case 30:
            // Java: case 33:
            // Java: case 47:
            21..=30 | 33 | 47 => {
                // Java: return true;
                true
            }
            // Java: case 31:
            // Java: case 32:
            // Java: case 34:
            // Java: case 35:
            // Java: case 36:
            // Java: case 37:
            // Java: case 38:
            // Java: case 39:
            // Java: case 40:
            // Java: case 41:
            // Java: case 42:
            // Java: case 43:
            // Java: case 44:
            // Java: case 45:
            // Java: case 46:
            // Java: default:
            _ => {
                // Java: return false;
                false
            }
        }
    }
}
