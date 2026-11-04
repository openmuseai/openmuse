#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unused_mut, unused_unsafe, static_mut_refs)]

// Java: import cn.wps.base.assertion.Assert
use crate::cn::wps::base::assertion::Assert::Assert;
// Java: import java.util.Arrays
use std::sync::OnceLock;

// Java field: char THAI_RANGE_START
pub const THAI_RANGE_START: char = '\u{0e00}';
// Java field: char THAI_RANGE_END
pub const THAI_RANGE_END: char = '\u{0e7f}';
// Java field: char THAI_CHARACTER_SARA_A
pub const THAI_CHARACTER_SARA_A: char = '\u{0e30}';
// Java field: char THAI_CHARACTER_SARA_AA
pub const THAI_CHARACTER_SARA_AA: char = '\u{0e32}';
// Java field: char THAI_CHARACTER_RU
pub const THAI_CHARACTER_RU: char = '\u{0e24}';
// Java field: char THAI_CHARACTER_LU
pub const THAI_CHARACTER_LU: char = '\u{0e26}';
// Java field: char THAI_CHARACTER_SARA_AM
pub const THAI_CHARACTER_SARA_AM: char = '\u{0e33}';
// Java field: char THAI_CHARACTER_SARA_I
pub const THAI_CHARACTER_SARA_I: char = '\u{0e34}';
// Java field: char THAI_CHARACTER_SARA_II
pub const THAI_CHARACTER_SARA_II: char = '\u{0e35}';
// Java field: char THAI_CHARACTER_SARA_UEE
pub const THAI_CHARACTER_SARA_UEE: char = '\u{0e37}';
// Java field: char THAI_CHARACTER_SARA_U
pub const THAI_CHARACTER_SARA_U: char = '\u{0e38}';
// Java field: char THAI_CHARACTER_SARA_E
pub const THAI_CHARACTER_SARA_E: char = '\u{0e40}';
// Java field: char THAI_CHARACTER_SARA_AE
pub const THAI_CHARACTER_SARA_AE: char = '\u{0e41}';
// Java field: char THAI_CHARACTER_SARA_O
pub const THAI_CHARACTER_SARA_O: char = '\u{0e42}';
// Java field: char THAI_CHARACTER_LAKKHANGYAO
pub const THAI_CHARACTER_LAKKHANGYAO: char = '\u{0e45}';
// Java field: char THAI_CHARACTER_MAITAIKHU
pub const THAI_CHARACTER_MAITAIKHU: char = '\u{0e47}';
// Java field: char THAI_CHARACTER_THANTHAKHAT
pub const THAI_CHARACTER_THANTHAKHAT: char = '\u{0e4c}';
// Java field: char THAI_CHARACTER_NIKHAHIT
pub const THAI_CHARACTER_NIKHAHIT: char = '\u{0e4d}';
// Java field: char THAI_CHARACTER_YAMAKKAN
pub const THAI_CHARACTER_YAMAKKAN: char = '\u{0e4e}';
// Java field: ThaiCharType[] s_charType
static S_CHAR_TYPE: OnceLock<[ThaiCharType; 128]> = OnceLock::new();
// Java field: Unit mask
static mut MASK: Unit = Unit {
    vowelLeft: '\0',
    consonant: '\0',
    tone: '\0',
    vowel1: '\0',
    vowel2: '\0',
    valid: false,
    split: false,
    srcLen: 0,
};

// Java: public class ThaiCorrectionTool
pub struct ThaiCorrectionTool;

// Java: public enum ThaiCharType
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThaiCharType {
    Invalid,
    Consonant,
    ConsonantVowel,
    VowelLeft,
    VowelRight,
    VowelTop,
    VowelBottom,
    VowelLengthSign,
    ToneMark,
    Sign,
    RepetitionMark,
    CurrencySymbol,
    Digit,
    None,
}

// Java: public enum Action
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    insert,
    overwrite,
    forbid,
    split,
}

// Java: public class Unit
pub struct Unit {
    // Java field: char vowelLeft
    pub vowelLeft: char,
    // Java field: char consonant
    pub consonant: char,
    // Java field: char tone
    pub tone: char,
    // Java field: char vowel1
    pub vowel1: char,
    // Java field: char vowel2
    pub vowel2: char,
    // Java field: boolean valid
    pub valid: bool,
    // Java field: boolean split
    pub split: bool,
    // Java field: int srcLen
    pub srcLen: i32,
}

// Java: public class ThaiCorrectionTool
impl ThaiCorrectionTool {
    // Java: ThaiCorrectionTool()
    pub fn new() -> Self {
        // Java: super();
        ThaiCorrectionTool
    }

    // Java: static boolean isThai(char ch)
    pub fn isThai(ch: char) -> bool {
        // Java: return ch >= 3584 && ch <= 3711;
        ch as u32 >= 3584 && ch as u32 <= 3711
    }

    // Java: static boolean isAlone(char ch)
    pub fn isAlone(ch: char) -> bool {
        // Java: ThaiCharType type = getThaiCharType(ch);
        let t = Self::getThaiCharType(ch);
        // Java: return type == ThaiCorrectionTool.ThaiCharType.Invalid || type == ThaiCorrectionTool.ThaiCharType.ConsonantVowel || type == ThaiCorrectionTool.ThaiCharType.Digit || type == ThaiCorrectionTool.ThaiCharType.None;
        t == ThaiCharType::Invalid || t == ThaiCharType::ConsonantVowel || t == ThaiCharType::Digit || t == ThaiCharType::None
    }

    // Java: static boolean cannotSplit(char ch)
    pub fn cannotSplit(ch: char) -> bool {
        let ch_u = ch as u32;
        // Java: if (ch != 3655 && ch != 3660 && ch != 3661 && ch != 3662)
        if ch_u != 3655 && ch_u != 3660 && ch_u != 3661 && ch_u != 3662 {
            // Java: ThaiCharType type = getThaiCharType(ch);
            let t = Self::getThaiCharType(ch);
            // Java: return type == ThaiCorrectionTool.ThaiCharType.VowelTop || type == ThaiCorrectionTool.ThaiCharType.VowelBottom || type == ThaiCorrectionTool.ThaiCharType.ToneMark;
            t == ThaiCharType::VowelTop || t == ThaiCharType::VowelBottom || t == ThaiCharType::ToneMark
        // Java: else
        } else {
            // Java: return true;
            true
        }
    }

    // Java: static ThaiCharType getThaiCharType(char ch)
    pub fn getThaiCharType(ch: char) -> ThaiCharType {
        // Java: if (null == s_charType)
        let s_char_type = S_CHAR_TYPE.get_or_init(|| {
            // Java: s_charType = new ThaiCharType[]{ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.VowelRight, ThaiCorrectionTool.ThaiCharType.VowelLeft, ThaiCorrectionTool.ThaiCharType.Digit, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.VowelTop, ThaiCorrectionTool.ThaiCharType.VowelLeft, ThaiCorrectionTool.ThaiCharType.Digit, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.VowelRight, ThaiCorrectionTool.ThaiCharType.VowelLeft, ThaiCorrectionTool.ThaiCharType.Digit, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.VowelRight, ThaiCorrectionTool.ThaiCharType.VowelLeft, ThaiCorrectionTool.ThaiCharType.Digit, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.ConsonantVowel, ThaiCorrectionTool.ThaiCharType.VowelTop, ThaiCorrectionTool.ThaiCharType.VowelLeft, ThaiCorrectionTool.ThaiCharType.Digit, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.VowelTop, ThaiCorrectionTool.ThaiCharType.VowelLengthSign, ThaiCorrectionTool.ThaiCharType.Digit, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.ConsonantVowel, ThaiCorrectionTool.ThaiCharType.VowelTop, ThaiCorrectionTool.ThaiCharType.RepetitionMark, ThaiCorrectionTool.ThaiCharType.Digit, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.VowelTop, ThaiCorrectionTool.ThaiCharType.VowelTop, ThaiCorrectionTool.ThaiCharType.Digit, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.VowelBottom, ThaiCorrectionTool.ThaiCharType.ToneMark, ThaiCorrectionTool.ThaiCharType.Digit, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.VowelBottom, ThaiCorrectionTool.ThaiCharType.ToneMark, ThaiCorrectionTool.ThaiCharType.Digit, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.VowelBottom, ThaiCorrectionTool.ThaiCharType.ToneMark, ThaiCorrectionTool.ThaiCharType.Sign, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.ToneMark, ThaiCorrectionTool.ThaiCharType.Sign, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Sign, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Sign, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Sign, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Consonant, ThaiCorrectionTool.ThaiCharType.Sign, ThaiCorrectionTool.ThaiCharType.CurrencySymbol, ThaiCorrectionTool.ThaiCharType.Sign, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None, ThaiCorrectionTool.ThaiCharType.None};
            [
                ThaiCharType::None, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::VowelRight, ThaiCharType::VowelLeft, ThaiCharType::Digit, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::VowelTop, ThaiCharType::VowelLeft, ThaiCharType::Digit, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::VowelRight, ThaiCharType::VowelLeft, ThaiCharType::Digit, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::VowelRight, ThaiCharType::VowelLeft, ThaiCharType::Digit, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::ConsonantVowel, ThaiCharType::VowelTop, ThaiCharType::VowelLeft, ThaiCharType::Digit, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::VowelTop, ThaiCharType::VowelLengthSign, ThaiCharType::Digit, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::ConsonantVowel, ThaiCharType::VowelTop, ThaiCharType::RepetitionMark, ThaiCharType::Digit, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::VowelTop, ThaiCharType::VowelTop, ThaiCharType::Digit, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::VowelBottom, ThaiCharType::ToneMark, ThaiCharType::Digit, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::VowelBottom, ThaiCharType::ToneMark, ThaiCharType::Digit, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::VowelBottom, ThaiCharType::ToneMark, ThaiCharType::Sign, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::None, ThaiCharType::ToneMark, ThaiCharType::Sign, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::None, ThaiCharType::Sign, ThaiCharType::None, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::None, ThaiCharType::Sign, ThaiCharType::None, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::None, ThaiCharType::Sign, ThaiCharType::None, ThaiCharType::None, ThaiCharType::None,
                ThaiCharType::Consonant, ThaiCharType::Consonant, ThaiCharType::Sign, ThaiCharType::CurrencySymbol, ThaiCharType::Sign, ThaiCharType::None, ThaiCharType::None, ThaiCharType::None,
            ]
        });
        // Java: if (ch >= 3584 && ch <= 3711)
        let ch_u = ch as u32;
        if ch_u >= 3584 && ch_u <= 3711 {
            // Java: int offset = ch - 3584;
            let offset = (ch_u - 3584) as usize;
            // Java: offset = offset % 16 * 8 + offset / 16;
            let offset = offset % 16 * 8 + offset / 16;
            // Java: return s_charType[offset];
            s_char_type[offset]
        // Java: else
        } else {
            // Java: return ThaiCorrectionTool.ThaiCharType.Invalid;
            ThaiCharType::Invalid
        }
    }

    // Java: static void parse(char ch, int sel, _pText: &[char])
    pub fn parse(ch: char, sel: i32, p_text: &[char]) {
        // Java: parse(ch, sel, pText, false);
        Self::parse_full(ch, sel, p_text, false);
    }

    // Java: static void parse(char ch, int sel, _pText: &[char], boolean needSplit)
    pub fn parse_full(ch: char, sel: i32, p_text: &[char], need_split: bool) {
        // Java: ThaiCharType cType = getThaiCharType(ch);
        let c_type = Self::getThaiCharType(ch);
        // Java: init();
        Self::init();
        // Java: switch (cType)
        match c_type {
            // Java: case Digit:
            // Java: case RepetitionMark:
            // Java: case CurrencySymbol:
            // Java: case ConsonantVowel:
            ThaiCharType::Digit | ThaiCharType::RepetitionMark | ThaiCharType::CurrencySymbol | ThaiCharType::ConsonantVowel => {
                // Java: if (sel <= 0 || getThaiCharType(pText[sel - 1]) != ThaiCorrectionTool.ThaiCharType.VowelLeft)
                if sel <= 0 || Self::getThaiCharType(p_text[(sel - 1) as usize]) != ThaiCharType::VowelLeft {
                    // Java: setValid(true);
                    Self::setValid(true);
                    // Java: setConsonant(ch);
                    Self::setConsonant(ch);
                }
            }
            // Java: break;
            // Java: case None:
            // Java: case Invalid:
            // Java: case Consonant:
            ThaiCharType::None | ThaiCharType::Invalid | ThaiCharType::Consonant => {
                // Java: setValid(true);
                Self::setValid(true);
                // Java: setConsonant(ch);
                Self::setConsonant(ch);
            }
            // Java: break;
            // Java: case ToneMark:
            ThaiCharType::ToneMark => {
                // Java: parseSyllable(pText, sel);
                Self::parseSyllable(p_text, sel);
                // Java: if (valid())
                if Self::valid() {
                    // Java: inputTone(ch);
                    Self::inputTone(ch);
                }
            }
            // Java: break;
            // Java: case VowelLengthSign:
            ThaiCharType::VowelLengthSign => {
                // Java: if (sel > 0 && getThaiCharType(pText[sel - 1]) == ThaiCorrectionTool.ThaiCharType.ConsonantVowel)
                if sel > 0 && Self::getThaiCharType(p_text[(sel - 1) as usize]) == ThaiCharType::ConsonantVowel {
                    // Java: setValid(true);
                    Self::setValid(true);
                    // Java: setVowel1(ch);
                    Self::setVowel1(ch);
                // Java: else if (sel > 1 && pText[sel - 1] == 3634 && getThaiCharType(pText[sel - 2]) == ThaiCorrectionTool.ThaiCharType.ConsonantVowel)
                } else if sel > 1 && p_text[(sel - 1) as usize] as u32 == 3634 && Self::getThaiCharType(p_text[(sel - 2) as usize]) == ThaiCharType::ConsonantVowel {
                    // Java: setValid(true);
                    Self::setValid(true);
                    // Java: setVowel1(ch);
                    Self::setVowel1(ch);
                    // Java: setSrcLen(1);
                    Self::setSrcLen(1);
                }
            }
            // Java: break;
            // Java: case Sign:
            ThaiCharType::Sign => {
                // Java: correctSign(pText, sel, ch);
                Self::correctSign(p_text, sel, ch);
            }
            // Java: break;
            // Java: case VowelLeft:
            ThaiCharType::VowelLeft => {
                // Java: int len = 0;
                let mut len: i32 = 0;
                // Java: char dst = ch;
                let mut dst = ch;
                // Java: int prev = sel - 1;
                let prev = sel - 1;
                // Java: if (prev >= 0)
                if prev >= 0 {
                    // Java: ThaiCharType prevType = getThaiCharType(pText[prev]);
                    let prev_type = Self::getThaiCharType(p_text[prev as usize]);
                    // Java: if (prevType == ThaiCorrectionTool.ThaiCharType.VowelLeft)
                    if prev_type == ThaiCharType::VowelLeft {
                        // Java: len = 1;
                        len = 1;
                        // Java: if (pText[prev] == 3648 && ch == pText[prev])
                        if p_text[prev as usize] as u32 == 3648 && ch == p_text[prev as usize] {
                            // Java: dst = 3649;
                            dst = char::from_u32(3649).unwrap();
                        }
                    }
                }
                // Java: setValid(true);
                Self::setValid(true);
                // Java: setSrcLen(len);
                Self::setSrcLen(len);
                // Java: setVowelLeft(dst);
                Self::setVowelLeft(dst);
            }
            // Java: break;
            // Java: case VowelTop:
            // Java: case VowelRight:
            // Java: case VowelBottom:
            ThaiCharType::VowelTop | ThaiCharType::VowelRight | ThaiCharType::VowelBottom => {
                // Java: parseSyllable(pText, sel);
                Self::parseSyllable(p_text, sel);
                // Java: if (valid())
                if Self::valid() {
                    // Java: if (needSplit)
                    if need_split {
                        // Java: if (ch == 3635)
                        if ch as u32 == 3635 {
                            // Java: setSplit(true);
                            Self::setSplit(true);
                            // Java: setVowel1('ํ');
                            Self::setVowel1('\u{0e4d}');
                            // Java: removeVowel2();
                            Self::removeVowel2();
                            // Java: removeVowelLeft();
                            Self::removeVowelLeft();
                            // Java: inputVowel('า');
                            Self::inputVowel('\u{0e32}');
                        // Java: else
                        } else {
                            // Java: inputVowel(ch);
                            Self::inputVowel(ch);
                        }
                    // Java: else if (mask.vowel1 != 0 && mask.vowel1 == 3661 && ch == 3634)
                    } else if unsafe { MASK.vowel1 } as u32 != 0 && unsafe { MASK.vowel1 } as u32 == 3661 && ch as u32 == 3634 {
                        // Java: inputVowel('ำ');
                        Self::inputVowel('\u{0e33}');
                    // Java: else
                    } else {
                        // Java: inputVowel(ch);
                        Self::inputVowel(ch);
                    }
                // Java: else if (ch == 3634)
                } else if ch as u32 == 3634 {
                    // Java: if (sel > 0 && getThaiCharType(pText[sel - 1]) == ThaiCorrectionTool.ThaiCharType.ConsonantVowel)
                    if sel > 0 && Self::getThaiCharType(p_text[(sel - 1) as usize]) == ThaiCharType::ConsonantVowel {
                        // Java: setValid(true);
                        Self::setValid(true);
                        // Java: setVowel1(ch);
                        Self::setVowel1(ch);
                    // Java: else if (sel > 1 && pText[sel - 1] == 3653 && getThaiCharType(pText[sel - 2]) == ThaiCorrectionTool.ThaiCharType.ConsonantVowel)
                    } else if sel > 1 && p_text[(sel - 1) as usize] as u32 == 3653 && Self::getThaiCharType(p_text[(sel - 2) as usize]) == ThaiCharType::ConsonantVowel {
                        // Java: setValid(true);
                        Self::setValid(true);
                        // Java: setVowel1(ch);
                        Self::setVowel1(ch);
                        // Java: setSrcLen(1);
                        Self::setSrcLen(1);
                    }
                }
            }
            // Java: break;
            // Java: default:
            _ => {
                // Java: Assert.assertTrue(false);
                Assert::assertTrue(false);
            }
        }
    }

    // Java: static Action getAction(char ch, _pText: &[char], StringBuffer dest)
    pub fn getAction(ch: char, p_text: &[char], dest: &mut String) -> Action {
        // Java: return getAction(ch, pText, dest, false);
        Self::getAction_full(ch, p_text, dest, false)
    }

    // Java: static Action getAction(char ch, _pText: &[char], StringBuffer dest, boolean needSplit)
    pub fn getAction_full(ch: char, p_text: &[char], dest: &mut String, need_split: bool) -> Action {
        // Java: parse(ch, pText.length, pText, needSplit);
        Self::parse_full(ch, p_text.len() as i32, p_text, need_split);
        // Java: if (valid())
        if Self::valid() {
            // Java: value(dest);
            Self::value(dest);
            // Java: if (dest.length() == 0)
            if dest.len() == 0 {
                // Java: return ThaiCorrectionTool.Action.forbid;
                return Action::forbid;
            // Java: else if (getSrcLen() == 0)
            } else if Self::getSrcLen() == 0 {
                // Java: return ThaiCorrectionTool.Action.insert;
                return Action::insert;
            // Java: else
            } else {
                // Java: return isSplit() ? ThaiCorrectionTool.Action.split : ThaiCorrectionTool.Action.overwrite;
                return if Self::isSplit() { Action::split } else { Action::overwrite };
            }
        // Java: else
        } else {
            // Java: return ThaiCorrectionTool.Action.forbid;
            return Action::forbid;
        }
    }

    // Java: static void value(StringBuffer txt)
    fn value(txt: &mut String) {
        unsafe {
            // Java: if (mask.vowelLeft != 0)
            if MASK.vowelLeft != '\0' {
                // Java: txt.append(mask.vowelLeft);
                txt.push(MASK.vowelLeft);
            }
            // Java: if (mask.consonant != 0)
            if MASK.consonant != '\0' {
                // Java: txt.append(mask.consonant);
                txt.push(MASK.consonant);
            }
            // Java: if (mask.vowel1 != 0)
            if MASK.vowel1 != '\0' {
                // Java: if (getThaiCharType(mask.vowel1) == ThaiCorrectionTool.ThaiCharType.VowelRight)
                if Self::getThaiCharType(MASK.vowel1) == ThaiCharType::VowelRight {
                    // Java: if (mask.tone != 0)
                    if MASK.tone != '\0' {
                        // Java: txt.append(mask.tone);
                        txt.push(MASK.tone);
                    }
                    // Java: txt.append(mask.vowel1);
                    txt.push(MASK.vowel1);
                // Java: else
                } else {
                    // Java: txt.append(mask.vowel1);
                    txt.push(MASK.vowel1);
                    // Java: if (mask.tone != 0)
                    if MASK.tone != '\0' {
                        // Java: txt.append(mask.tone);
                        txt.push(MASK.tone);
                    }
                }
                // Java: if (mask.vowel2 != 0)
                if MASK.vowel2 != '\0' {
                    // Java: txt.append(mask.vowel2);
                    txt.push(MASK.vowel2);
                }
            // Java: else if (mask.tone != 0)
            } else if MASK.tone != '\0' {
                // Java: txt.append(mask.tone);
                txt.push(MASK.tone);
            }
        }
    }

    // Java: static void parseSyllable(_pText: &[char], int sel)
    fn parseSyllable(p_text: &[char], sel: i32) {
        // Java: if (sel > 0)
        if sel > 0 {
            // Java: int idx = sel;
            let mut idx = sel;
            // Java: int len = 0;
            let mut len: i32 = 0;
            // Java: while(true)
            loop {
                // Java: --idx;
                idx -= 1;
                // Java: if (idx < 0)
                if idx < 0 {
                    // Java: break;
                    break;
                }
                // Java: ThaiCharType type = getThaiCharType(pText[idx]);
                let t = Self::getThaiCharType(p_text[idx as usize]);
                // Java: if (type == ThaiCorrectionTool.ThaiCharType.VowelLeft)
                if t == ThaiCharType::VowelLeft {
                    // Java: if (mask.vowelLeft == 0)
                    if unsafe { MASK.vowelLeft } == '\0' {
                        // Java: setVowelLeft(pText[idx]);
                        Self::setVowelLeft(p_text[idx as usize]);
                        // Java: ++len;
                        len += 1;
                    }
                    // Java: break;
                    break;
                }
                // Java: if (valid())
                if Self::valid() {
                    // Java: break;
                    break;
                }
                // Java: if (type == ThaiCorrectionTool.ThaiCharType.Consonant)
                if t == ThaiCharType::Consonant {
                    // Java: setConsonant(pText[idx]);
                    Self::setConsonant(p_text[idx as usize]);
                    // Java: setValid(true);
                    Self::setValid(true);
                // Java: else if (type == ThaiCorrectionTool.ThaiCharType.ToneMark)
                } else if t == ThaiCharType::ToneMark {
                    // Java: if (mask.tone != 0)
                    if unsafe { MASK.tone } != '\0' {
                        // Java: break;
                        break;
                    }
                    // Java: setTone(pText[idx]);
                    Self::setTone(p_text[idx as usize]);
                // Java: else if (type != ThaiCorrectionTool.ThaiCharType.VowelLeft && type != ThaiCorrectionTool.ThaiCharType.VowelRight && type != ThaiCorrectionTool.ThaiCharType.VowelTop && type != ThaiCorrectionTool.ThaiCharType.VowelBottom)
                } else if t != ThaiCharType::VowelLeft && t != ThaiCharType::VowelRight && t != ThaiCharType::VowelTop && t != ThaiCharType::VowelBottom {
                    // Java: if (mask.tone == 0 && pText[idx] == 3660)
                    if unsafe { MASK.tone } == '\0' && p_text[idx as usize] as u32 == 3660 {
                        // Java: setTone(pText[idx]);
                        Self::setTone(p_text[idx as usize]);
                    // Java: else
                    } else {
                        // Java: if (mask.vowel1 != 0 || pText[idx] != 3661 && pText[idx] != 3662)
                        if unsafe { MASK.vowel1 } != '\0' || (p_text[idx as usize] as u32 != 3661 && p_text[idx as usize] as u32 != 3662) {
                            // Java: break;
                            break;
                        }
                        // Java: setVowel1(pText[idx]);
                        Self::setVowel1(p_text[idx as usize]);
                    }
                // Java: else if (mask.vowel1 == 0)
                } else if unsafe { MASK.vowel1 } == '\0' {
                    // Java: setVowel1(pText[idx]);
                    Self::setVowel1(p_text[idx as usize]);
                // Java: else
                } else {
                    // Java: if (idx != sel - 2 || pText[idx] != 3634 || pText[idx + 1] != 3632)
                    if idx != sel - 2 || p_text[idx as usize] as u32 != 3634 || p_text[(idx + 1) as usize] as u32 != 3632 {
                        // Java: break;
                        break;
                    }
                    // Java: setVowel1(pText[idx]);
                    Self::setVowel1(p_text[idx as usize]);
                    // Java: setVowel2(pText[idx + 1]);
                    Self::setVowel2(p_text[(idx + 1) as usize]);
                }
                // Java: ++len;
                len += 1;
            }
            // Java: setSrcLen(len);
            Self::setSrcLen(len);
        }
    }

    // Java: static void correctSign(_pText: &[char], int sel, char ch)
    fn correctSign(p_text: &[char], sel: i32, ch: char) {
        // Java: switch (ch)
        match ch {
            // Java: case '์':
            '\u{0e4c}' => {
                // Java: parseSyllable(pText, sel);
                Self::parseSyllable(p_text, sel);
                // Java: if (!valid())
                if !Self::valid() {
                    // Java: return;
                    return;
                }
                // Java: setTone(ch);
                Self::setTone(ch);
                // Java: if (mask.vowel1 != 0 && mask.vowel1 != 3640 && mask.vowel1 != 3636)
                if unsafe { MASK.vowel1 } != '\0' && unsafe { MASK.vowel1 } as u32 != 3640 && unsafe { MASK.vowel1 } as u32 != 3636 {
                    // Java: removeVowel1();
                    Self::removeVowel1();
                }
                // Java: removeVowel2();
                Self::removeVowel2();
                // Java: removeVowelLeft();
                Self::removeVowelLeft();
            }
            // Java: break;
            // Java: case 'ํ':
            // Java: case '๎':
            '\u{0e4d}' | '\u{0e4e}' => {
                // Java: parseSyllable(pText, sel);
                Self::parseSyllable(p_text, sel);
                // Java: if (!valid())
                if !Self::valid() {
                    // Java: return;
                    return;
                }
                // Java: setVowel1(ch);
                Self::setVowel1(ch);
                // Java: removeTone();
                Self::removeTone();
                // Java: removeVowel2();
                Self::removeVowel2();
                // Java: removeVowelLeft();
                Self::removeVowelLeft();
            }
            // Java: break;
            // Java: default:
            _ => {
                // Java: if (sel > 0 && getThaiCharType(pText[sel - 1]) == ThaiCorrectionTool.ThaiCharType.VowelLeft)
                if sel > 0 && Self::getThaiCharType(p_text[(sel - 1) as usize]) == ThaiCharType::VowelLeft {
                    // Java: return;
                    return;
                }
                // Java: setValid(true);
                Self::setValid(true);
                // Java: setConsonant(ch);
                Self::setConsonant(ch);
            }
        }
    }

    // Java: static void inputVowel(char ch)
    fn inputVowel(ch: char) {
        // Java: Assert.assertTrue(getThaiCharType(ch) == ThaiCorrectionTool.ThaiCharType.VowelRight || getThaiCharType(ch) == ThaiCorrectionTool.ThaiCharType.VowelTop || getThaiCharType(ch) == ThaiCorrectionTool.ThaiCharType.VowelBottom);
        Assert::assertTrue(Self::getThaiCharType(ch) == ThaiCharType::VowelRight || Self::getThaiCharType(ch) == ThaiCharType::VowelTop || Self::getThaiCharType(ch) == ThaiCharType::VowelBottom);
        unsafe {
            // Java: if (mask.vowel1 == 0 || mask.vowel1 != 3634 || ch != 3632 || mask.tone != 0 && (mask.vowelLeft == 0 || mask.vowelLeft != 3648))
            if MASK.vowel1 == '\0' || MASK.vowel1 as u32 != 3634 || ch as u32 != 3632 || (MASK.tone != '\0' && (MASK.vowelLeft == '\0' || MASK.vowelLeft as u32 != 3648)) {
                // Java: if (mask.vowel1 == 3661 && ch == 3634)
                if MASK.vowel1 as u32 == 3661 && ch as u32 == 3634 {
                    // Java: setVowel2(ch);
                    Self::setVowel2(ch);
                // Java: else
                } else {
                    // Java: setVowel1(ch);
                    Self::setVowel1(ch);
                    // Java: removeVowel2();
                    Self::removeVowel2();
                    // Java: if (mask.tone != 0 && (ch == 3655 || mask.tone == 3660 && ch != 3636 && ch != 3640))
                    if MASK.tone != '\0' && (ch as u32 == 3655 || (MASK.tone as u32 == 3660 && ch as u32 != 3636 && ch as u32 != 3640)) {
                        // Java: removeTone();
                        Self::removeTone();
                    }
                }
            // Java: else
            } else {
                // Java: if (mask.vowel2 != 0)
                if MASK.vowel2 != '\0' {
                    // Java: setValid(false);
                    Self::setValid(false);
                    // Java: return;
                    return;
                }
                // Java: setVowel2(ch);
                Self::setVowel2(ch);
            }
            // Java: if (mask.vowelLeft != 0 && !reserveLeftVowel(ch, mask.vowelLeft))
            if MASK.vowelLeft != '\0' && !Self::reserveLeftVowel(ch, MASK.vowelLeft) {
                // Java: removeVowelLeft();
                Self::removeVowelLeft();
            }
        }
    }

    // Java: static void inputTone(char ch)
    fn inputTone(ch: char) {
        // Java: Assert.assertTrue(getThaiCharType(ch) == ThaiCorrectionTool.ThaiCharType.ToneMark);
        Assert::assertTrue(Self::getThaiCharType(ch) == ThaiCharType::ToneMark);
        // Java: setTone(ch);
        Self::setTone(ch);
        unsafe {
            // Java: if (mask.vowel1 != 0)
            if MASK.vowel1 != '\0' {
                // Java: if (mask.vowel1 != 3655 && getThaiCharType(mask.vowel1) != ThaiCorrectionTool.ThaiCharType.Sign)
                if MASK.vowel1 as u32 != 3655 && Self::getThaiCharType(MASK.vowel1) != ThaiCharType::Sign {
                    // Java: if (mask.vowel2 != 0 && (mask.vowelLeft == 0 || mask.vowelLeft != 3648))
                    if MASK.vowel2 != '\0' && (MASK.vowelLeft == '\0' || MASK.vowelLeft as u32 != 3648) {
                        // Java: removeVowel2();
                        Self::removeVowel2();
                    }
                // Java: else
                } else {
                    // Java: removeVowel1();
                    Self::removeVowel1();
                }
            }
        }
    }

    // Java: static boolean reserveLeftVowel(char v, char lv)
    fn reserveLeftVowel(v: char, lv: char) -> bool {
        let v_u = v as u32;
        let lv_u = lv as u32;
        // Java: if (v == 3632)
        if v_u == 3632 {
            // Java: return lv == 3649 || lv == 3650 || lv == 3648;
            lv_u == 3649 || lv_u == 3650 || lv_u == 3648
        // Java: else if (v != 3655)
        } else if v_u != 3655 {
            // Java: if (v != 3634 && v != 3636 && v != 3637 && v != 3639)
            if v_u != 3634 && v_u != 3636 && v_u != 3637 && v_u != 3639 {
                // Java: return false;
                false
            // Java: else
            } else {
                // Java: return lv == 3648;
                lv_u == 3648
            }
        // Java: else
        } else {
            // Java: return lv == 3648 || lv == 3649;
            lv_u == 3648 || lv_u == 3649
        }
    }

    // Java: static boolean isCorrectInput(String str)
    pub fn isCorrectInput(str_val: Option<&str>) -> bool {
        // Java: if (null != str)
        if let Some(s) = str_val {
            // Java: _pText: &[char] = str.toCharArray();
            let mut p_text: Vec<char> = s.chars().collect();
            // Java: boolean skip = false;
            let mut skip = false;
            // Java: for(int i = 0; i < pText.length; ++i)
            let mut i: usize = 0;
            while i < p_text.len() {
                // Java: char ch = pText[i];
                let ch = p_text[i];
                // Java: if (isThai(ch))
                if Self::isThai(ch) {
                    // Java: if (skip)
                    if skip {
                        // Java: pText = Arrays.copyOfRange(pText, i, pText.length);
                        p_text = p_text[i..].to_vec();
                        // Java: i = 0;
                        i = 0;
                        // Java: skip = false;
                        skip = false;
                    }
                    // Java: parse(ch, i, pText);
                    Self::parse(ch, i as i32, &p_text);
                    // Java: if (!valid())
                    if !Self::valid() {
                        // Java: return false;
                        return false;
                    }
                // Java: else
                } else {
                    // Java: skip = true;
                    skip = true;
                }
                i += 1;
            }
        }
        // Java: return true;
        true
    }

    // Java: static void init()
    fn init() {
        unsafe {
            // Java: mask = new Unit();
            MASK = Unit {
                vowelLeft: '\0',
                consonant: '\0',
                tone: '\0',
                vowel1: '\0',
                vowel2: '\0',
                valid: false,
                split: false,
                srcLen: 0,
            };
            // Java: mask.srcLen = 0;
            MASK.srcLen = 0;
        }
    }

    // Java: static void setConsonant(char ch)
    fn setConsonant(ch: char) {
        // Java: mask.consonant = ch;
        unsafe { MASK.consonant = ch; }
    }

    // Java: static void setTone(char ch)
    fn setTone(ch: char) {
        // Java: mask.tone = ch;
        unsafe { MASK.tone = ch; }
    }

    // Java: static void setVowelLeft(char ch)
    fn setVowelLeft(ch: char) {
        // Java: mask.vowelLeft = ch;
        unsafe { MASK.vowelLeft = ch; }
    }

    // Java: static void setVowel1(char ch)
    fn setVowel1(ch: char) {
        // Java: mask.vowel1 = ch;
        unsafe { MASK.vowel1 = ch; }
    }

    // Java: static void setVowel2(char ch)
    fn setVowel2(ch: char) {
        // Java: mask.vowel2 = ch;
        unsafe { MASK.vowel2 = ch; }
    }

    // Java: static void setSrcLen(int len)
    fn setSrcLen(len: i32) {
        // Java: mask.srcLen = len;
        unsafe { MASK.srcLen = len; }
    }

    // Java: static int getSrcLen()
    pub fn getSrcLen() -> i32 {
        // Java: return mask.valid ? mask.srcLen : 0;
        unsafe { if MASK.valid { MASK.srcLen } else { 0 } }
    }

    // Java: static void setValid(boolean valid)
    fn setValid(v: bool) {
        // Java: mask.valid = valid;
        unsafe { MASK.valid = v; }
    }

    // Java: static void removeTone()
    fn removeTone() {
        // Java: mask.tone = 0;
        unsafe { MASK.tone = '\0'; }
    }

    // Java: static void removeVowelLeft()
    fn removeVowelLeft() {
        // Java: mask.vowelLeft = 0;
        unsafe { MASK.vowelLeft = '\0'; }
    }

    // Java: static void removeVowel1()
    fn removeVowel1() {
        // Java: mask.vowel1 = 0;
        unsafe { MASK.vowel1 = '\0'; }
    }

    // Java: static void removeVowel2()
    fn removeVowel2() {
        // Java: mask.vowel2 = 0;
        unsafe { MASK.vowel2 = '\0'; }
    }

    // Java: static void removeConsonant()
    fn removeConsonant() {
        // Java: mask.consonant = 0;
        unsafe { MASK.consonant = '\0'; }
    }

    // Java: static boolean valid()
    fn valid() -> bool {
        // Java: return mask.valid;
        unsafe { MASK.valid }
    }

    // Java: static void setSplit(boolean split)
    fn setSplit(s: bool) {
        // Java: mask.split = split;
        unsafe { MASK.split = s; }
    }

    // Java: static boolean isSplit()
    pub fn isSplit() -> bool {
        // Java: return mask.split;
        unsafe { MASK.split }
    }
}

// Java: public enum ThaiCharType
impl ThaiCharType {
    // Java: ThaiCharType()
    pub fn new() -> Self {
        ThaiCharType::Invalid
    }
}

// Java: public enum Action
impl Action {
    // Java: Action()
    pub fn new() -> Self {
        Action::forbid
    }
}

// Java: public class Unit
impl Unit {}
