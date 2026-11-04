#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Java field: int decimal
pub(crate) const decimal: i32 = 0;
// Java field: int upperRoman
pub(crate) const upperRoman: i32 = 1;
// Java field: int lowerRoman
pub(crate) const lowerRoman: i32 = 2;
// Java field: int upperLetter
pub(crate) const upperLetter: i32 = 3;
// Java field: int lowerLetter
pub(crate) const lowerLetter: i32 = 4;
// Java field: int ordinal
pub(crate) const ordinal: i32 = 5;
// Java field: int cardinalText
pub(crate) const cardinalText: i32 = 6;
// Java field: int ordinalText
pub(crate) const ordinalText: i32 = 7;
// Java field: int hex
pub(crate) const hex: i32 = 8;
// Java field: int chicago
pub(crate) const chicago: i32 = 9;
// Java field: int ideographDigital
pub(crate) const ideographDigital: i32 = 10;
// Java field: int japaneseCounting
pub(crate) const japaneseCounting: i32 = 11;
// Java field: int aiueo
pub(crate) const aiueo: i32 = 12;
// Java field: int iroha
pub(crate) const iroha: i32 = 13;
// Java field: int decimalFullWidth
pub(crate) const decimalFullWidth: i32 = 14;
// Java field: int decimalHalfWidth
pub(crate) const decimalHalfWidth: i32 = 15;
// Java field: int japaneseLegal
pub(crate) const japaneseLegal: i32 = 16;
// Java field: int japaneseDigitalTenThousand
pub(crate) const japaneseDigitalTenThousand: i32 = 17;
// Java field: int decimalEnclosedCircle
pub(crate) const decimalEnclosedCircle: i32 = 18;
// Java field: int decimalFullWidth2
pub(crate) const decimalFullWidth2: i32 = 19;
// Java field: int aiueoFullWidth
pub(crate) const aiueoFullWidth: i32 = 20;
// Java field: int irohaFullWidth
pub(crate) const irohaFullWidth: i32 = 21;
// Java field: int decimalZero
pub(crate) const decimalZero: i32 = 22;
// Java field: int bullet
pub(crate) const bullet: i32 = 23;
// Java field: int ganada
pub(crate) const ganada: i32 = 24;
// Java field: int chosung
pub(crate) const chosung: i32 = 25;
// Java field: int decimalEnclosedFullstop
pub(crate) const decimalEnclosedFullstop: i32 = 26;
// Java field: int decimalEnclosedParen
pub(crate) const decimalEnclosedParen: i32 = 27;
// Java field: int decimalEnclosedCircleChinese
pub(crate) const decimalEnclosedCircleChinese: i32 = 28;
// Java field: int ideographEnclosedCircle
pub(crate) const ideographEnclosedCircle: i32 = 29;
// Java field: int ideographTraditional
pub(crate) const ideographTraditional: i32 = 30;
// Java field: int ideographZodiac
pub(crate) const ideographZodiac: i32 = 31;
// Java field: int ideographZodiacTraditional
pub(crate) const ideographZodiacTraditional: i32 = 32;
// Java field: int taiwaneseCounting
pub(crate) const taiwaneseCounting: i32 = 33;
// Java field: int ideographLegalTraditional
pub(crate) const ideographLegalTraditional: i32 = 34;
// Java field: int taiwaneseCountingThousand
pub(crate) const taiwaneseCountingThousand: i32 = 35;
// Java field: int taiwaneseDigital
pub(crate) const taiwaneseDigital: i32 = 36;
// Java field: int chineseCounting
pub(crate) const chineseCounting: i32 = 37;
// Java field: int chineseLegalSimplified
pub(crate) const chineseLegalSimplified: i32 = 38;
// Java field: int chineseCountingThousand
pub(crate) const chineseCountingThousand: i32 = 39;
// Java field: int applicationDefined
pub(crate) const applicationDefined: i32 = 40;
// Java field: int koreanDigital
pub(crate) const koreanDigital: i32 = 41;
// Java field: int koreanCounting
pub(crate) const koreanCounting: i32 = 42;
// Java field: int koreanLegal
pub(crate) const koreanLegal: i32 = 43;
// Java field: int koreanDigital2
pub(crate) const koreanDigital2: i32 = 44;
// Java field: int hebrew1
pub(crate) const hebrew1: i32 = 45;
// Java field: int arabicAlpha
pub(crate) const arabicAlpha: i32 = 46;
// Java field: int hebrew2
pub(crate) const hebrew2: i32 = 47;
// Java field: int arabicAbjad
pub(crate) const arabicAbjad: i32 = 48;
// Java field: int hindiVowels
pub(crate) const hindiVowels: i32 = 49;
// Java field: int hindiConsonants
pub(crate) const hindiConsonants: i32 = 50;
// Java field: int hindiNumbers
pub(crate) const hindiNumbers: i32 = 51;
// Java field: int hindiCounting
pub(crate) const hindiCounting: i32 = 52;
// Java field: int thaiLetters
pub(crate) const thaiLetters: i32 = 53;
// Java field: int thaiNumbers
pub(crate) const thaiNumbers: i32 = 54;
// Java field: int thaiCounting
pub(crate) const thaiCounting: i32 = 55;
// Java field: int vietnameseCounting
pub(crate) const vietnameseCounting: i32 = 56;
// Java field: int numberInDash
pub(crate) const numberInDash: i32 = 57;
// Java field: int russianLower
pub(crate) const russianLower: i32 = 58;
// Java field: int russianUpper
pub(crate) const russianUpper: i32 = 59;
// Java field: int totalCount
pub(crate) const totalCount: i32 = 60;
// Java field: String[] NUMBERING_TYPE
static NUMBERING_TYPE: OnceLock<Vec<String>> = OnceLock::new();
// Java field: HashMap<String, Integer> NfcMap
static NFC_MAP: OnceLock<Mutex<HashMap<String, i32>>> = OnceLock::new();

// Java: public class NFCDefine
pub struct NFCDefine {
}

// Java: public class NFCDefine
impl NFCDefine {
    // Java: NFCDefine()
    pub fn new() -> Self {
        // Java: super();
        NFCDefine {}
    }

    // Java: static int getNfc(String type)
    pub fn getNfc(_type_str: &str) -> i32 {
        // Java: Integer nfc = (Integer)NfcMap.get(type);
        // Java: return nfc != null ? nfc : 0;
        todo!("NFCDefine::getNfc")
    }
}
