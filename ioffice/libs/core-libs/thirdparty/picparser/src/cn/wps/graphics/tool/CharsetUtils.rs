#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_assignments, unused_mut, unused_imports)]

// Java: import java.util.Set

use std::collections::HashSet;
use std::sync::Mutex;

// Java field: int ANSI_CHARSET
pub(crate) const ANSI_CHARSET: i32 = 0;
// Java field: int DEFAULT_CHARSET
pub(crate) const DEFAULT_CHARSET: i32 = 1;
// Java field: int SYMBOL_CHARSET
pub(crate) const SYMBOL_CHARSET: i32 = 2;
// Java field: int SHIFTJIS_CHARSET
pub(crate) const SHIFTJIS_CHARSET: i32 = 128;
// Java field: int HANGEUL_CHARSET
pub(crate) const HANGEUL_CHARSET: i32 = 129;
// Java field: int HANGUL_CHARSET
pub(crate) const HANGUL_CHARSET: i32 = 129;
// Java field: int GB2312_CHARSET
pub(crate) const GB2312_CHARSET: i32 = 134;
// Java field: int CHINESEBIG5_CHARSET
pub(crate) const CHINESEBIG5_CHARSET: i32 = 136;
// Java field: int OEM_CHARSET
pub(crate) const OEM_CHARSET: i32 = 255;
// Java field: int JOHAB_CHARSET
pub(crate) const JOHAB_CHARSET: i32 = 130;
// Java field: int HEBREW_CHARSET
pub(crate) const HEBREW_CHARSET: i32 = 177;
// Java field: int ARABIC_CHARSET
pub(crate) const ARABIC_CHARSET: i32 = 178;
// Java field: int GREEK_CHARSET
pub(crate) const GREEK_CHARSET: i32 = 161;
// Java field: int TURKISH_CHARSET
pub(crate) const TURKISH_CHARSET: i32 = 162;
// Java field: int VIETNAMESE_CHARSET
pub(crate) const VIETNAMESE_CHARSET: i32 = 163;
// Java field: int THAI_CHARSET
pub(crate) const THAI_CHARSET: i32 = 222;
// Java field: int EASTEUROPE_CHARSET
pub(crate) const EASTEUROPE_CHARSET: i32 = 238;
// Java field: int RUSSIAN_CHARSET
pub(crate) const RUSSIAN_CHARSET: i32 = 204;
// Java field: int MAC_CHARSET
pub(crate) const MAC_CHARSET: i32 = 77;
// Java field: int BALTIC_CHARSET
pub(crate) const BALTIC_CHARSET: i32 = 186;
// Java field: String DEFAULT
pub(crate) const DEFAULT: &str = "DEFAULT";
// Java field: String GB2312
pub(crate) const GB2312: &str = "GB2312";
// Java field: String GBK
pub(crate) const GBK: &str = "GBK";
// Java field: String ISO1
pub(crate) const ISO1: &str = "ISO-8859-1";
// Java field: int gb2312Count
static gb2312Count: Mutex<i32> = Mutex::new(0);
// Java field: int iso1Count
static iso1Count: Mutex<i32> = Mutex::new(0);
// Java field: String recentlyCharset
static recentlyCharset: Mutex<Option<String>> = Mutex::new(None);

// Java: public class CharsetUtils
pub struct CharsetUtils {
}

// Java: public class Guess
pub struct Guess {
}

// Java: public class CharsetUtils
impl CharsetUtils {
    // Java: CharsetUtils()
    pub fn new() -> Self {
        // Java: super();
        CharsetUtils {}
    }

    // Java: static String getCharSetText(byte code)
    pub fn getCharSetText(code: i8) -> String {
        // Java: switch (code & 255)
        match (code as i32) & 255 {
            // Java: case 0:
            0 => {
                // Java: ++CharsetUtils.Guess.iso1Count;
                *iso1Count.lock().unwrap() += 1;
                // Java: CharsetUtils.Guess.recentlyCharset = "ISO-8859-1";
                *recentlyCharset.lock().unwrap() = Some("ISO-8859-1".to_string());
                // Java: return "ISO-8859-1";
                "ISO-8859-1".to_string()
            }
            // Java: case 1:
            1 => {
                // Java: return "DEFAULT";
                "DEFAULT".to_string()
            }
            // Java: case 2:
            2 => {
                // Java: return "SYMBOL";
                "SYMBOL".to_string()
            }
            // Java: case 128:
            128 => {
                // Java: return "shift_jis";
                "shift_jis".to_string()
            }
            // Java: case 129:
            129 => {
                // Java: return "GB18030";
                "GB18030".to_string()
            }
            // Java: case 134:
            134 => {
                // Java: ++CharsetUtils.Guess.gb2312Count;
                *gb2312Count.lock().unwrap() += 1;
                // Java: CharsetUtils.Guess.recentlyCharset = "GB2312";
                *recentlyCharset.lock().unwrap() = Some("GB2312".to_string());
                // Java: return "GB2312";
                "GB2312".to_string()
            }
            // Java: case 136:
            136 => {
                // Java: return "BIG5";
                "BIG5".to_string()
            }
            // Java: case 161:
            161 => {
                // Java: return "ISO-8859-7";
                "ISO-8859-7".to_string()
            }
            // Java: case 204:
            204 => {
                // Java: return "windows-1251";
                "windows-1251".to_string()
            }
            // Java: default:
            _ => {
                // Java: return "";
                String::new()
            }
        }
    }

    // Java: static String guessCharset(Set<String> affectCharsetSet, String keyword)
    pub fn guessCharset(affectCharsetSet: &HashSet<String>, keyword: &str) -> String {
        let gb_count = *gb2312Count.lock().unwrap();
        let iso_count = *iso1Count.lock().unwrap();
        // Java: if (CharsetUtils.Guess.gb2312Count > 0 && CharsetUtils.Guess.iso1Count > 0)
        if gb_count > 0 && iso_count > 0 {
            // Java: CharsetUtils.Guess.recentlyCharset = CharsetUtils.Guess.gb2312Count > CharsetUtils.Guess.iso1Count ? "GB2312" : "ISO-8859-1";
            let recent = if gb_count > iso_count { "GB2312" } else { "ISO-8859-1" };
            *recentlyCharset.lock().unwrap() = Some(recent.to_string());
        }
        // Java: String charset = null;
        let charset: String;
        let recent = recentlyCharset.lock().unwrap().clone();
        // Java: if (!affectCharsetSet.contains(keyword) && !"GB2312".equals(CharsetUtils.Guess.recentlyCharset))
        if !affectCharsetSet.contains(keyword) && recent.as_deref() != Some("GB2312") {
            // Java: if ("ISO-8859-1".equals(CharsetUtils.Guess.recentlyCharset))
            if recent.as_deref() == Some("ISO-8859-1") {
                // Java: charset = "ISO-8859-1";
                charset = "ISO-8859-1".to_string();
            }
            // Java: else
            else {
                // Java: charset = "ISO-8859-1";
                charset = "ISO-8859-1".to_string();
            }
        }
        // Java: else
        else {
            // Java: charset = "GB2312";
            charset = "GB2312".to_string();
        }
        // Java: CharsetUtils.Guess.reset();
        Guess::reset();
        // Java: return charset;
        charset
    }
}

// Java: public class Guess
impl Guess {
    // Java: static void reset()
    pub fn reset() {
        // Java: gb2312Count = 0;
        *gb2312Count.lock().unwrap() = 0;
        // Java: iso1Count = 0;
        *iso1Count.lock().unwrap() = 0;
        // Java: recentlyCharset = null;
        *recentlyCharset.lock().unwrap() = None;
    }
}
