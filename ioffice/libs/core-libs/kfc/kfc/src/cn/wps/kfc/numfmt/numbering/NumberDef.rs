#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code)]

use std::sync::OnceLock;

// Java field: char DIGIT_0
pub(crate) const DIGIT_0: char = '0';
// Java field: char DIGIT_1
pub(crate) const DIGIT_1: char = '1';
// Java field: char DIGIT_2
pub(crate) const DIGIT_2: char = '2';
// Java field: char DIGIT_3
pub(crate) const DIGIT_3: char = '3';
// Java field: char DIGIT_4
pub(crate) const DIGIT_4: char = '4';
// Java field: char DIGIT_5
pub(crate) const DIGIT_5: char = '5';
// Java field: char DIGIT_6
pub(crate) const DIGIT_6: char = '6';
// Java field: char DIGIT_7
pub(crate) const DIGIT_7: char = '7';
// Java field: char DIGIT_8
pub(crate) const DIGIT_8: char = '8';
// Java field: char DIGIT_9
pub(crate) const DIGIT_9: char = '9';
// Java field: String[] DECIMAL
static DECIMAL: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] DECIMAL_FULL_WIDTH
static DECIMAL_FULL_WIDTH: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] IDEOGRAPH_DIGITAL
static IDEOGRAPH_DIGITAL: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] KOREAN_DIGITAL2
static KOREAN_DIGITAL2: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] KOREAN_DIGITAL
static KOREAN_DIGITAL: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] THAI_NUMBERS
static THAI_NUMBERS: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] HINDI_NUMBERS
static HINDI_NUMBERS: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] HEX
static HEX: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] LOWER_LETTER
static LOWER_LETTER: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] UPPER_LETTER
static UPPER_LETTER: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] CHICAGO
static CHICAGO: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] RUSSIAN_LOWER
static RUSSIAN_LOWER: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] RUSSIAN_UPPER
static RUSSIAN_UPPER: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] THAI_LETTERS
static THAI_LETTERS: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] ARABIC_ALPHA
static ARABIC_ALPHA: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] ARABIC_ABJAD
static ARABIC_ABJAD: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] HINDI_CONSONANTS
static HINDI_CONSONANTS: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] HINDI_VOWELS
static HINDI_VOWELS: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] IDEOGRAPH_TRADITIONAL
static IDEOGRAPH_TRADITIONAL: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] IDEOGRAPH_ZODIAC
static IDEOGRAPH_ZODIAC: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] AIUEO
static AIUEO: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] AIUEO_FULL_WIDTH
static AIUEO_FULL_WIDTH: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] IROHA
static IROHA: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] IROHA_FULL_WIDTH
static IROHA_FULL_WIDTH: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] DECIMAL_ENCLOSED_CIRCLE_CHINESE
static DECIMAL_ENCLOSED_CIRCLE_CHINESE: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] DECIMAL_ENCLOSED_CIRCLE
static DECIMAL_ENCLOSED_CIRCLE: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] DECIMAL_ENCLOSED_FULL_STOP
static DECIMAL_ENCLOSED_FULL_STOP: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] DECIMAL_ENCLOSED_PAREN
static DECIMAL_ENCLOSED_PAREN: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] IDEOGRAPH_ENCLOSED_CIRCLE
static IDEOGRAPH_ENCLOSED_CIRCLE: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] TAIWANESE_COUNTING
static TAIWANESE_COUNTING: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] TAIWANESE_COUNTING_THOUSAND
static TAIWANESE_COUNTING_THOUSAND: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] IDEOGRAPH_LEGAL_TRADITIONANL
static IDEOGRAPH_LEGAL_TRADITIONANL: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] CHINESE_COUNTING_THOUSAND
static CHINESE_COUNTING_THOUSAND: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] CHINESE_LEGAL_SIMPLIFIED
static CHINESE_LEGAL_SIMPLIFIED: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] KOREAN_COUNTING
static KOREAN_COUNTING: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] VIETNAMESE_COUNTING
static VIETNAMESE_COUNTING: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] JAPANESE_LEGAL
static JAPANESE_LEGAL: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] JAPANESE_COUNTING
static JAPANESE_COUNTING: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] GANADA
static GANADA: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] CHOSUNG
static CHOSUNG: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] HEBREW2
static HEBREW2: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] HEBREW1_X
static HEBREW1_X: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] HEBREW1_X0
static HEBREW1_X0: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] HEBREW1_X00
static HEBREW1_X00: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] UPPER_ROMAS
static UPPER_ROMAS: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] LOWER_ROMAS
static LOWER_ROMAS: OnceLock<Vec<String>> = OnceLock::new();

// Java: public class NumberDef
pub struct NumberDef {
}

// Java: public class NumberDef
impl NumberDef {
    // Java: NumberDef()
    pub fn new() -> Self {
        // Java: super();
        NumberDef {}
    }
}
