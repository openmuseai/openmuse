#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use std::collections::HashSet;
use std::sync::OnceLock;

// Java field: String WINGDINGS
pub const WINGDINGS: &str = "Wingdings";
// Java field: String WINGDINGS2
pub const WINGDINGS2: &str = "Wingdings 2";
// Java field: String WINGDINGS3
pub const WINGDINGS3: &str = "Wingdings 3";
// Java field: String EXTRA
pub const EXTRA: &str = "MT Extra";
// Java field: String SYMBOL
pub const SYMBOL: &str = "Symbol";
// Java field: String WEBDINGS
pub const WEBDINGS: &str = "Webdings";
// Java field: String CAMBRIA
pub const CAMBRIA: &str = "Cambria";
// Java field: String CAMBRIA_MATH
pub const CAMBRIA_MATH: &str = "Cambria Math";
// Java field: String IMPACT
pub const IMPACT: &str = "Impact";
// Java field: String KINGSOFT_CONFETTI
pub const KINGSOFT_CONFETTI: &str = "Kingsoft Confetti";
// Java field: String KINGSOFT_EXTRA
pub const KINGSOFT_EXTRA: &str = "Kingsoft Extra";
// Java field: String KINGSOFT_SIGN
pub const KINGSOFT_SIGN: &str = "Kingsoft Sign";
// Java field: String KINGSOFT_MATH
pub const KINGSOFT_MATH: &str = "Kingsoft Math";
// Java field: String KINGSOFT_MARK
pub const KINGSOFT_MARK: &str = "Kingsoft Mark";
// Java field: String KINGSOFT_STRESS
pub const KINGSOFT_STRESS: &str = "Kingsoft Stress";
// Java field: String FZ_FSJW
pub const FZ_FSJW: &str = "\u{65B9}\u{6B63}\u{4EFF}\u{5B8B}\u{7B80}\u{4F53}";
// Java field: String FZ_FSK
pub const FZ_FSK: &str = "\u{65B9}\u{6B63}\u{4EFF}\u{5B8B}_GBK";
// Java field: String FZ_HTJW
pub const FZ_HTJW: &str = "\u{65B9}\u{6B63}\u{9ED1}\u{4F53}\u{7B80}\u{4F53}";
// Java field: String FZ_HTK
pub const FZ_HTK: &str = "\u{65B9}\u{6B63}\u{9ED1}\u{4F53}_GBK";
// Java field: String FZ_KTJW
pub const FZ_KTJW: &str = "\u{65B9}\u{6B63}\u{6977}\u{4F53}\u{7B80}\u{4F53}";
// Java field: String FZ_KTK
pub const FZ_KTK: &str = "\u{65B9}\u{6B63}\u{6977}\u{4F53}_GBK";
// Java field: String FZ_XBSJW
pub const FZ_XBSJW: &str = "\u{65B9}\u{6B63}\u{5C0F}\u{6807}\u{5B8B}\u{7B80}\u{4F53}";
// Java field: String FZ_XBSK
pub const FZ_XBSK: &str = "\u{65B9}\u{6B63}\u{5C0F}\u{6807}\u{5B8B}_GBK";
// Java field: String HY_SHUSONGERJ
pub const HY_SHUSONGERJ: &str = "\u{6C49}\u{4EEA}\u{4E66}\u{5B8B}\u{4E8C}\u{7B80}";
// Java field: String HY_ZHONGHEIJ
pub const HY_ZHONGHEIJ: &str = "\u{6C49}\u{4EEA}\u{4E2D}\u{9ED1}\u{7B80}";
// Java field: String HY_FANGSONGJ
pub const HY_FANGSONGJ: &str = "\u{6C49}\u{4EEA}\u{4EFF}\u{5B8B}\u{7B80}";
// Java field: String HY_ZHONGKAIJ
pub const HY_ZHONGKAIJ: &str = "\u{6C49}\u{4EEA}\u{4E2D}\u{6977}\u{7B80}";
// Java field: String HY_ZHONGLISHUJ
pub const HY_ZHONGLISHUJ: &str = "\u{6C49}\u{4EEA}\u{4E2D}\u{96B6}\u{4E66}\u{7B80}";
// Java field: String SPECIAL1
pub const SPECIAL1: &str = "WPS Special 1";
// Java field: String SPECIAL2
pub const SPECIAL2: &str = "WPS Special 2";
// Java field: String SPECIAL3
pub const SPECIAL3: &str = "WPS Special 3";
// Java field: String WPS_WINGDINGS
pub const WPS_WINGDINGS: &str = "Wingdings";
// Java field: String WPS_EXTRA
pub const WPS_EXTRA: &str = "MT Extra";
// Java field: String WPS_SYMBOL
pub const WPS_SYMBOL: &str = "Symbol";
// Java field: String WPS_WEBDINGS
pub const WPS_WEBDINGS: &str = "Webdings";
// Java field: Set<String> SPECIAL_FONT_SET
static SPECIAL_FONT_SET: OnceLock<HashSet<&str>> = OnceLock::new();
// Java field: _EXTRA_CODEPOINTS: &[i32]
const EXTRA_CODEPOINTS: [i32; 24] = [35,36,37,38,40,41,176,177,178,181,182,183,186,187,188,192,193,194,195,209,210,211,213,214];
// Java field: _KINGSOFT_CONFETTI_CODEPOINTS: &[i32]
const KINGSOFT_CONFETTI_CODEPOINTS: [i32; 28] = [40,67,69,74,75,76,81,108,110,110,111,112,115,117,118,159,161,167,168,171,178,216,223,224,243,250,252,254];
// Java field: _KINGSOFT_MARK_CODEPOINTS: &[i32]
const KINGSOFT_MARK_CODEPOINTS: [i32; 27] = [32,36,69,95,97,98,99,101,114,116,118,119,122,134,152,154,157,166,168,183,200,201,228,232,234,241,252];

fn get_special_font_set() -> &'static HashSet<&'static str> {
    SPECIAL_FONT_SET.get_or_init(|| {
        let mut set = HashSet::new();
        set.insert("Wingdings");
        set.insert("MT Extra");
        set.insert("Symbol");
        set.insert("Kingsoft Confetti");
        set.insert("Kingsoft Extra");
        set.insert("Kingsoft Sign");
        set
    })
}

// Java: public class FontName
pub struct FontName;

// Java: public class FontName
impl FontName {
    // Java: FontName()
    pub fn new() -> Self {
        // Java: super();
        FontName
    }

    // Java: static boolean isSpecialFontName(String name)
    pub fn isSpecialFontName(name: &str) -> bool {
        // Java: return SPECIAL_FONT_SET.contains(name);
        get_special_font_set().contains(name)
    }

    // Java: static boolean isNormalChar(char c)
    pub fn isNormalChar(c: char) -> bool {
        // Java: return c >= '!' && c <= '~' || c >= 161 && c <= 172 || c >= 174 && c <= 255;
        let cv = c as u32;
        (cv >= 0x21 && cv <= 0x7E) || (cv >= 161 && cv <= 172) || (cv >= 174 && cv <= 255)
    }

    // Java: static boolean isWingdingsCodePoint(int codePoint)
    pub fn isWingdingsCodePoint(code_point: i32) -> bool {
        // Java: int low = codePoint & 255;
        let low = code_point & 255;
        // Java: if (low < 33)
        if low < 33 {
            // Java: return false;
            return false;
        }
        // Java: else if (low <= 126)
        else if low <= 126 {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return low >= 128 && low <= 255;
            return low >= 128 && low <= 255;
        }
    }

    // Java: static boolean isWebdingsCodePoint(int codePoint)
    pub fn isWebdingsCodePoint(code_point: i32) -> bool {
        // Java: int low = codePoint & 255;
        let low = code_point & 255;
        // Java: if (low < 33)
        if low < 33 {
            // Java: return false;
            return false;
        }
        // Java: else if (low <= 126)
        else if low <= 126 {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return low >= 128 && low <= 255;
            return low >= 128 && low <= 255;
        }
    }

    // Java: static boolean isSymbolCodePoint(int codePoint)
    pub fn isSymbolCodePoint(code_point: i32) -> bool {
        // Java: int low = codePoint & 255;
        let low = code_point & 255;
        // Java: if (low < 33)
        if low < 33 {
            // Java: return false;
            return false;
        }
        // Java: else if (low <= 126)
        else if low <= 126 {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return low >= 161 && low <= 254;
            return low >= 161 && low <= 254;
        }
    }

    // Java: static boolean isExtraCodePoint(int codePoint)
    pub fn isExtraCodePoint(code_point: i32) -> bool {
        // Java: int lowPoint = codePoint & 255;
        let low_point = code_point & 255;
        // Java: if (lowPoint >= 49 && lowPoint <= 56)
        if low_point >= 49 && low_point <= 56 {
            // Java: return true;
            return true;
        }
        // Java: else if (lowPoint >= 58 && lowPoint <= 83)
        else if low_point >= 58 && low_point <= 83 {
            // Java: return true;
            return true;
        }
        // Java: else if (lowPoint >= 85 && lowPoint <= 120)
        else if low_point >= 85 && low_point <= 120 {
            // Java: return true;
            return true;
        }
        // Java: else if (lowPoint >= 123 && lowPoint <= 126)
        else if low_point >= 123 && low_point <= 126 {
            // Java: return true;
            return true;
        }
        // Java: else if (lowPoint >= 128 && lowPoint <= 136)
        else if low_point >= 128 && low_point <= 136 {
            // Java: return true;
            return true;
        }
        // Java: else if (lowPoint >= 160 && lowPoint <= 165)
        else if low_point >= 160 && low_point <= 165 {
            // Java: return true;
            return true;
        }
        // Java: else if (lowPoint >= 167 && lowPoint <= 174)
        else if low_point >= 167 && low_point <= 174 {
            // Java: return true;
            return true;
        }
        // Java: else if (lowPoint >= 231 && lowPoint <= 255)
        else if low_point >= 231 && low_point <= 255 {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: int low = 0;
            let mut low: usize = 0;
            // Java: int high = EXTRA_CODEPOINTS.length - 1;
            let mut high: usize = EXTRA_CODEPOINTS.len() - 1;
            // Java: if (lowPoint >= EXTRA_CODEPOINTS[low] && lowPoint <= EXTRA_CODEPOINTS[high])
            if low_point >= EXTRA_CODEPOINTS[low] && low_point <= EXTRA_CODEPOINTS[high] {
                // Java: while(low <= high)
                while low <= high {
                    // Java: int mid = (high + low) / 2;
                    let mid = (high + low) / 2;
                    // Java: int midVal = EXTRA_CODEPOINTS[mid];
                    let mid_val = EXTRA_CODEPOINTS[mid];
                    // Java: if (lowPoint < midVal)
                    if low_point < mid_val {
                        // Java: high = mid - 1;
                        if mid == 0 { break; }
                        high = mid - 1;
                    }
                    // Java: else
                    else {
                        // Java: if (lowPoint <= midVal)
                        if low_point <= mid_val {
                            // Java: return true;
                            return true;
                        }
                        // Java: low = mid + 1;
                        low = mid + 1;
                    }
                }
                // Java: return false;
                return false;
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
    }

    // Java: static boolean isKingsoftSignCodePoint(int codePoint)
    pub fn isKingsoftSignCodePoint(code_point: i32) -> bool {
        // Java: return isSymbolCodePoint(codePoint);
        Self::isSymbolCodePoint(code_point)
    }

    // Java: static boolean isKingsoftExtraCodePoint(int codePoint)
    pub fn isKingsoftExtraCodePoint(code_point: i32) -> bool {
        // Java: return isExtraCodePoint(codePoint);
        Self::isExtraCodePoint(code_point)
    }

    // Java: static boolean isKingsoftConfettiCodePoint(int codePoint)
    pub fn isKingsoftConfettiCodePoint(code_point: i32) -> bool {
        // Java: if (codePoint >= 128 && codePoint <= 149)
        if code_point >= 128 && code_point <= 149 {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return Arrays.binarySearch(KINGSOFT_CONFETTI_CODEPOINTS, codePoint) >= 0;
            return KINGSOFT_CONFETTI_CODEPOINTS.binary_search(&code_point).is_ok();
        }
    }

    // Java: static boolean isKingsoftMarkCodePoint(int codePoint)
    pub fn isKingsoftMarkCodePoint(code_point: i32) -> bool {
        // Java: if (codePoint >= 48 && codePoint <= 52)
        if code_point >= 48 && code_point <= 52 {
            // Java: return true;
            return true;
        }
        // Java: else if (codePoint >= 55 && codePoint <= 61)
        else if code_point >= 55 && code_point <= 61 {
            // Java: return true;
            return true;
        }
        // Java: else if (codePoint >= 71 && codePoint <= 75)
        else if code_point >= 71 && code_point <= 75 {
            // Java: return true;
            return true;
        }
        // Java: else if (codePoint >= 80 && codePoint <= 82)
        else if code_point >= 80 && code_point <= 82 {
            // Java: return true;
            return true;
        }
        // Java: else if (codePoint >= 84 && codePoint <= 90)
        else if code_point >= 84 && code_point <= 90 {
            // Java: return true;
            return true;
        }
        // Java: else if (codePoint >= 103 && codePoint <= 109)
        else if code_point >= 103 && code_point <= 109 {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return Arrays.binarySearch(KINGSOFT_MARK_CODEPOINTS, codePoint) >= 0;
            return KINGSOFT_MARK_CODEPOINTS.binary_search(&code_point).is_ok();
        }
    }
}
