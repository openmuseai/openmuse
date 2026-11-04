#![allow(non_snake_case, dead_code, unused_variables)]

// Java field: int TYPE_CONSONANT
pub(crate) const TYPE_CONSONANT: i32 = 1;
// Java field: int TYPE_VOWEL
pub(crate) const TYPE_VOWEL: i32 = 2;
// Java field: int TYPE_VIRAMA
pub(crate) const TYPE_VIRAMA: i32 = 3;
// Java field: int TYPE_SIGN_MARK
pub(crate) const TYPE_SIGN_MARK: i32 = 4;
// Java field: int TYPE_UNKNOWN
pub(crate) const TYPE_UNKNOWN: i32 = 5;
// Java field: _CONSONANTS: &[char]
const CONSONANTS: &[char] = &[
    '\u{0915}', '\u{0916}', '\u{0917}', '\u{0918}', '\u{0919}',
    '\u{091A}', '\u{091B}', '\u{091C}', '\u{091D}', '\u{091E}',
    '\u{091F}', '\u{0920}', '\u{0921}', '\u{0922}', '\u{0923}',
    '\u{0924}', '\u{0925}', '\u{0926}', '\u{0927}', '\u{0928}',
    '\u{0929}', '\u{092A}', '\u{092B}', '\u{092C}', '\u{092D}',
    '\u{092E}', '\u{092F}', '\u{0930}', '\u{0931}', '\u{0932}',
    '\u{0933}', '\u{0934}', '\u{0935}', '\u{0936}', '\u{0937}',
    '\u{0938}', '\u{0939}', '\u{0958}', '\u{0959}', '\u{095A}',
    '\u{095B}', '\u{095C}', '\u{095D}', '\u{095E}', '\u{095F}',
    '\u{0979}', '\u{097A}', '\u{097B}', '\u{097C}', '\u{097E}',
    '\u{097F}',
];
// Java field: _VOWELS: &[char]
const VOWELS: &[char] = &[
    '\u{093A}', '\u{093B}', '\u{093E}', '\u{093F}', '\u{0940}',
    '\u{0941}', '\u{0942}', '\u{0943}', '\u{0944}', '\u{0945}',
    '\u{0946}', '\u{0947}', '\u{0948}', '\u{0949}', '\u{094A}',
    '\u{094B}', '\u{094C}', '\u{094E}', '\u{094F}',
];
// Java field: _SIGNS_MARKS: &[char]
const SIGNS_MARKS: &[char] = &[
    '\u{0900}', '\u{0901}', '\u{0902}', '\u{0903}', '\u{093C}',
    '\u{0951}', '\u{0952}', '\u{0953}', '\u{0954}', '\u{0955}',
    '\u{0956}', '\u{0957}',
];
// Java field: char VIRAMA
pub(crate) const VIRAMA: char = '\u{094D}';

// Java: public class Hindi
pub struct Hindi;

// Java: public class Hindi
impl Hindi {
    // Java: Hindi()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static boolean needSplit(char last, char cur)
    pub fn needSplit(last: char, cur: char) -> bool {
        // Java: int curType = getCharType(cur);
        let cur_type = Self::getCharType(cur);
        // Java: if (5 == curType)
        if 5 == cur_type
        {
            // Java: return true;
            return true;
        }
        // Java: else if (4 == curType)
        else if 4 == cur_type
        {
            // Java: return false;
            return false;
        }
        // Java: else
        else
        {
            // Java: int lastType = getCharType(last);
            let last_type = Self::getCharType(last);
            // Java: if (5 == lastType)
            if 5 == last_type
            {
                // Java: return true;
                return true;
            }
            // Java: else if (1 == curType && 3 != lastType)
            else if 1 == cur_type && 3 != last_type
            {
                // Java: return true;
                return true;
            }
            // Java: else
            else
            {
                // Java: return 2 == curType && 2 == lastType;
                return 2 == cur_type && 2 == last_type;
            }
        }
    }

    // Java: static int getCharType(char c)
    fn getCharType(c: char) -> i32 {
        // Java: if (c == 2381)
        if c == '\u{094D}'
        {
            // Java: return 3;
            return 3;
        }
        // Java: else
        else
        {
            // Java: int result = Arrays.binarySearch(CONSONANTS, c);
            let result = CONSONANTS.binary_search(&c);
            // Java: if (result >= 0)
            if result.is_ok()
            {
                // Java: return 1;
                return 1;
            }
            // Java: else
            else
            {
                // Java: result = Arrays.binarySearch(VOWELS, c);
                let result = VOWELS.binary_search(&c);
                // Java: if (result >= 0)
                if result.is_ok()
                {
                    // Java: return 2;
                    return 2;
                }
                // Java: else
                else
                {
                    // Java: result = Arrays.binarySearch(SIGNS_MARKS, c);
                    let result = SIGNS_MARKS.binary_search(&c);
                    // Java: return result >= 0 ? 4 : 5;
                    return if result.is_ok() { 4 } else { 5 };
                }
            }
        }
    }
}
