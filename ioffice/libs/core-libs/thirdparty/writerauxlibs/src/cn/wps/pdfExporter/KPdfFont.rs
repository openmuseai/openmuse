#![allow(dead_code, non_snake_case, unused_variables, unused_mut, unreachable_code)]

// Java: import android.graphics.Paint
// (android.graphics.Paint not available in Rust)
// Java: import android.graphics.RectF
// (android.graphics.RectF not available in Rust)
// Java: import cn.wps.core.runtime.Platform
// (util_adapter crate not available)
// Java: import cn.wps.font.FontHost
// (thirdparty_font crate not available)
// Java: import cn.wps.font.FreeTypeJNI
// (thirdparty_font crate not available)
// Java: import cn.wps.font.IFontFamily
// (thirdparty_font crate not available)
// Java: import cn.wps.font.IFontTable
// (thirdparty_font crate not available)
// Java: import cn.wps.font.impl.TTCParser
// (thirdparty_font crate not available)
// Java: import cn.wps.moffice.define.UILanguage
// (util_adapter crate not available)
// Java: import cn.wps.moffice.kfs.File
// (kfs_kfs_app crate not available)
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Locale
// Java: import java.util.Map
// Java: import java.util.Set
// Java: import java.util.Vector
use std::sync::{Mutex, OnceLock};

use super::KPdfObject::KPdfObject;
use super::KStream::KStream;
use super::KPdfArray::KPdfArray;
use super::KPdfTypeface::{KPdfTypeface, IFontFamily};

pub type Paint = ();
pub type KPdfStream = ();
pub type KPdfType0Font = ();
pub type KAdvancedTypefaceMetrics = ();
pub type KPdfDict = ();

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UILanguage {
    UILanguage_chinese,
    UILanguage_japan,
    UILanguage_korean,
    UILanguage_taiwan,
    UILanguage_hongkong,
    UILanguage_other,
}

struct Platform;
impl Platform {
    fn getLanguage() -> UILanguage { UILanguage::UILanguage_chinese }
}

struct FontHost;
impl FontHost {
    fn getFontTable() -> FontTable { FontTable }
}

struct FontTable;
impl FontTable {
    fn getCJKName(&self) -> Vec<String> { Vec::new() }
}

#[derive(Clone)]
pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

// Java field: int MAXFONTSIZE
pub const MAXFONTSIZE: i32 = 409600;
// Java field: IFontFamily mFontFamily
// (static, managed externally)
// Java field: Vector<KPdfFont> mFonts
static M_FONTS: OnceLock<Mutex<Vec<KPdfFont>>> = OnceLock::new();
// Java field: Map<String, Map<Integer, Integer>> sUsedFontMap
static S_USED_FONT_MAP: OnceLock<Mutex<HashMap<String, HashMap<i32, i32>>>> = OnceLock::new();
// Java field: Map<String, KPdfStream> sFontMap
// (placeholder - KPdfStream not fully defined)
// Java field: Vector<String> mBackFontNames
static M_BACK_FONT_NAMES: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
// Java field: int LATIN
pub const LATIN: i32 = 1;
// Java field: int LATIN_EXTENED
pub const LATIN_EXTENED: i32 = 2;
// Java field: int IPA_EXT
pub const IPA_EXT: i32 = 3;
// Java field: int SPACING_MOD
pub const SPACING_MOD: i32 = 4;
// Java field: int COMBINGING_DIACRITICS_MARKS
pub const COMBINGING_DIACRITICS_MARKS: i32 = 5;
// Java field: int GREEK
pub const GREEK: i32 = 6;
// Java field: int CYRILLIC
pub const CYRILLIC: i32 = 7;
// Java field: int ARMENIAN
pub const ARMENIAN: i32 = 8;
// Java field: int HEBREW
pub const HEBREW: i32 = 9;
// Java field: int ARABIC
pub const ARABIC: i32 = 10;
// Java field: int SYRIAC
pub const SYRIAC: i32 = 11;
// Java field: int ARABIC_SUPPLEMENT
pub const ARABIC_SUPPLEMENT: i32 = 12;
// Java field: int THAANA
pub const THAANA: i32 = 13;
// Java field: int NKO
pub const NKO: i32 = 14;
// Java field: int AVESTAN
pub const AVESTAN: i32 = 15;
// Java field: int MANDAIC
pub const MANDAIC: i32 = 16;
// Java field: int SAMARITAN
pub const SAMARITAN: i32 = 17;
// Java field: int DEVANAGARI
pub const DEVANAGARI: i32 = 18;
// Java field: int BENGALI
pub const BENGALI: i32 = 19;
// Java field: int GURMUKHI
pub const GURMUKHI: i32 = 20;
// Java field: int GUJARATI
pub const GUJARATI: i32 = 21;
// Java field: int ORIYA
pub const ORIYA: i32 = 22;
// Java field: int TAMIL
pub const TAMIL: i32 = 23;
// Java field: int TELUGU
pub const TELUGU: i32 = 24;
// Java field: int KANNADA
pub const KANNADA: i32 = 25;
// Java field: int MALAYALAM
pub const MALAYALAM: i32 = 26;
// Java field: int SINHALA
pub const SINHALA: i32 = 27;
// Java field: int THAI
pub const THAI: i32 = 28;
// Java field: int LAO
pub const LAO: i32 = 29;
// Java field: int TIBETAN
pub const TIBETAN: i32 = 30;
// Java field: int MYANMAR
pub const MYANMAR: i32 = 31;
// Java field: int GEORGIAN
pub const GEORGIAN: i32 = 32;
// Java field: int HANGUL_JAMO
pub const HANGUL_JAMO: i32 = 33;
// Java field: int ETHIOPIC
pub const ETHIOPIC: i32 = 34;
// Java field: int CHEROKEE
pub const CHEROKEE: i32 = 35;
// Java field: int UNIFIED_CANADIAN_AS
pub const UNIFIED_CANADIAN_AS: i32 = 36;
// Java field: int OGHAM
pub const OGHAM: i32 = 37;
// Java field: int RUNIC
pub const RUNIC: i32 = 38;
// Java field: int TAGALOG
pub const TAGALOG: i32 = 39;
// Java field: int HANUNOO
pub const HANUNOO: i32 = 40;
// Java field: int BUHID
pub const BUHID: i32 = 41;
// Java field: int TAGBANWA
pub const TAGBANWA: i32 = 42;
// Java field: int KHMER
pub const KHMER: i32 = 43;
// Java field: int MONGOLIAN
pub const MONGOLIAN: i32 = 44;
// Java field: int CHAM
pub const CHAM: i32 = 45;
// Java field: int LIMBU
pub const LIMBU: i32 = 46;
// Java field: int TAI_LE
pub const TAI_LE: i32 = 47;
// Java field: int NEW_TAI_LUE
pub const NEW_TAI_LUE: i32 = 48;
// Java field: int KMERSYMBOLS
pub const KMERSYMBOLS: i32 = 49;
// Java field: int BUGINESE
pub const BUGINESE: i32 = 50;
// Java field: int BATAK
pub const BATAK: i32 = 51;
// Java field: int LANNA
pub const LANNA: i32 = 52;
// Java field: int BALINESE
pub const BALINESE: i32 = 53;
// Java field: int SUNDANESE
pub const SUNDANESE: i32 = 54;
// Java field: int PAHAWH_HMONG
pub const PAHAWH_HMONG: i32 = 55;
// Java field: int LEPCHA
pub const LEPCHA: i32 = 56;
// Java field: int OL_CHIKI
pub const OL_CHIKI: i32 = 57;
// Java field: int MEITHEI
pub const MEITHEI: i32 = 58;
// Java field: int PHONETIC_EXT
pub const PHONETIC_EXT: i32 = 59;
// Java field: int PHONETIC_EXT_SUP
pub const PHONETIC_EXT_SUP: i32 = 60;
// Java field: int COMBINING_DIACRITICS_MARKS_SUP
pub const COMBINING_DIACRITICS_MARKS_SUP: i32 = 61;
// Java field: int LATIN_EXT_ADD
pub const LATIN_EXT_ADD: i32 = 62;
// Java field: int GREEK_EXT
pub const GREEK_EXT: i32 = 63;
// Java field: int GENERAL_MARKS
pub const GENERAL_MARKS: i32 = 64;
// Java field: int GLAGOLITIC
pub const GLAGOLITIC: i32 = 65;
// Java field: int LATIC_EXT_C
pub const LATIC_EXT_C: i32 = 66;
// Java field: int COPTIC
pub const COPTIC: i32 = 67;
// Java field: int GEORGIAN_SUPPLEMENT
pub const GEORGIAN_SUPPLEMENT: i32 = 68;
// Java field: int TIFINAGH
pub const TIFINAGH: i32 = 69;
// Java field: int ETHIOPIC_EXT
pub const ETHIOPIC_EXT: i32 = 70;
// Java field: int SUPPLEMENTAL_PUNCTUATION
pub const SUPPLEMENTAL_PUNCTUATION: i32 = 71;
// Java field: int CJK
pub const CJK: i32 = 72;
// Java field: int HIRAGANA_KATAKANA
pub const HIRAGANA_KATAKANA: i32 = 73;
// Java field: int BOPOMOFO
pub const BOPOMOFO: i32 = 74;
// Java field: int HANGUL_COMPATIBILITY_JAMO
pub const HANGUL_COMPATIBILITY_JAMO: i32 = 75;
// Java field: int KANBUN
pub const KANBUN: i32 = 76;
// Java field: int BOPOMOFO_EXT
pub const BOPOMOFO_EXT: i32 = 77;
// Java field: int CJK_STROKES
pub const CJK_STROKES: i32 = 78;
// Java field: int KATAKANA_PHONETIC_EXT
pub const KATAKANA_PHONETIC_EXT: i32 = 79;
// Java field: int ENCLOSED_CJK_LETTERS_MONTHS
pub const ENCLOSED_CJK_LETTERS_MONTHS: i32 = 80;
// Java field: int CJK_COMPATIBILITY
pub const CJK_COMPATIBILITY: i32 = 81;
// Java field: int CJK_UNIFIED_IDEOGRAPHS_EXT_A
pub const CJK_UNIFIED_IDEOGRAPHS_EXT_A: i32 = 82;
// Java field: int YIJING
pub const YIJING: i32 = 83;
// Java field: int CJK_UNIFIED_SYSBOLS
pub const CJK_UNIFIED_SYSBOLS: i32 = 84;
// Java field: int YI
pub const YI: i32 = 85;
// Java field: int VAI
pub const VAI: i32 = 86;
// Java field: int UNIFIED_CANADIAN_ABO_SYLL_SUP
pub const UNIFIED_CANADIAN_ABO_SYLL_SUP: i32 = 87;
// Java field: int MODIFIER_TONE_LETTERS
pub const MODIFIER_TONE_LETTERS: i32 = 88;
// Java field: int LATIN_EXT_D
pub const LATIN_EXT_D: i32 = 89;
// Java field: int SYLOTI_NAGRI
pub const SYLOTI_NAGRI: i32 = 90;
// Java field: int PHAGS_PA
pub const PHAGS_PA: i32 = 91;
// Java field: int SAURASHTRA
pub const SAURASHTRA: i32 = 92;
// Java field: int JAVANESE
pub const JAVANESE: i32 = 93;
// Java field: int CHAKMA
pub const CHAKMA: i32 = 94;
// Java field: int VARANG_KSHITI
pub const VARANG_KSHITI: i32 = 95;
// Java field: int SORANG_SOMPENG
pub const SORANG_SOMPENG: i32 = 96;
// Java field: int NEWARI
pub const NEWARI: i32 = 97;
// Java field: int VITTHAI
pub const VITTHAI: i32 = 98;
// Java field: int KAYAH_LI
pub const KAYAH_LI: i32 = 99;
// Java field: int HANGUL_SYLLABLES
pub const HANGUL_SYLLABLES: i32 = 100;
// Java field: int ZONEOFUTF16
pub const ZONEOFUTF16: i32 = 101;
// Java field: int PRIVATE
pub const PRIVATE: i32 = 102;
// Java field: int CJK_ECOMPATIBILITY_IDEOGRAPHS
pub const CJK_ECOMPATIBILITY_IDEOGRAPHS: i32 = 103;
// Java field: int ALPHABETIC_PRESENTATION_FORM
pub const ALPHABETIC_PRESENTATION_FORM: i32 = 104;
// Java field: int ARABIC_PRESENTATION_FORMS_A
pub const ARABIC_PRESENTATION_FORMS_A: i32 = 105;
// Java field: int VARIATION_SELECTOR
pub const VARIATION_SELECTOR: i32 = 106;
// Java field: int VERTICAL_FORMS
pub const VERTICAL_FORMS: i32 = 107;
// Java field: int COMBINING_HALF_MARKS
pub const COMBINING_HALF_MARKS: i32 = 108;
// Java field: int CJK_COMPATIBILITY_FORMS
pub const CJK_COMPATIBILITY_FORMS: i32 = 109;
// Java field: int SMALL_FORM_VARIANTS
pub const SMALL_FORM_VARIANTS: i32 = 110;
// Java field: int ARABIC_PRESENTATION_FORMS_B
pub const ARABIC_PRESENTATION_FORMS_B: i32 = 111;
// Java field: int HALF_FULL_WIDTH_FORM
pub const HALF_FULL_WIDTH_FORM: i32 = 112;
// Java field: int SPECIALS
pub const SPECIALS: i32 = 113;
// Java field: int[] unicodes
pub const UNICODES: &[i32] = &[0, 255, 591, 687, 767, 879, 1023, 1327, 1423, 1535, 1791, 1871, 1919, 1983, 2047, 2143, 2175, 2223, 2431, 2559, 2687, 2815, 2943, 3071, 3199, 3327, 3455, 3583, 3711, 3839, 4095, 4255, 4351, 4607, 5023, 5119, 5759, 5791, 5887, 5919, 5951, 5983, 6015, 6143, 6319, 6399, 6479, 6527, 6623, 6655, 6687, 6751, 6895, 7039, 7088, 7167, 7247, 7295, 7391, 7551, 7615, 7679, 7935, 8191, 11263, 11359, 11391, 11519, 11567, 11647, 11743, 11903, 12351, 12543, 12591, 12687, 12703, 12735, 12783, 12799, 13055, 13311, 19903, 19967, 40895, 42191, 42527, 42751, 42783, 43007, 43055, 43135, 43231, 43391, 43487, 43583, 43631, 43743, 43871, 43936, 55215, 57343, 63743, 64255, 64335, 65023, 65039, 65055, 65071, 65103, 65135, 65279, 65519, 65535];
// Java field: int[] sScales
pub const S_SCALES: &[i32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113];
// Java field: Map<Integer, String> mFontKeyName
static M_FONT_KEY_NAME: OnceLock<Mutex<HashMap<i32, &'static str>>> = OnceLock::new();
// Java field: ArrayList<String> sCJKNames
static S_CJK_NAMES: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
// Java field: Map<String, String[]> mapTTC
static MAP_TTC: OnceLock<Mutex<HashMap<String, Vec<String>>>> = OnceLock::new();

fn get_fonts() -> &'static Mutex<Vec<KPdfFont>> {
    M_FONTS.get_or_init(|| Mutex::new(Vec::new()))
}
fn get_used_font_map() -> &'static Mutex<HashMap<String, HashMap<i32, i32>>> {
    S_USED_FONT_MAP.get_or_init(|| Mutex::new(HashMap::new()))
}
fn get_back_font_names() -> &'static Mutex<Vec<String>> {
    M_BACK_FONT_NAMES.get_or_init(|| Mutex::new(Vec::new()))
}
fn get_font_key_name() -> &'static Mutex<HashMap<i32, &'static str>> {
    M_FONT_KEY_NAME.get_or_init(|| Mutex::new(HashMap::new()))
}
fn get_map_ttc() -> &'static Mutex<HashMap<String, Vec<String>>> {
    MAP_TTC.get_or_init(|| Mutex::new(HashMap::new()))
}

// Java: public class KPdfFont extends KPdfDict
pub struct KPdfFont {
    // Java field: KPdfTypeface mTypeface
    pub(crate) mTypeface: KPdfTypeface,
    // Java field: Vector<KPdfObject> mResources
    pub(crate) mResources: Vec<KPdfObject>,
    // Java field: KAdvancedTypefaceMetrics mFontInfo
    // (placeholder)
    // Java field: KPdfDict mDescriptor
    // (placeholder)
    // Java field: int mFontType
    pub(crate) mFontType: i32,
    // Java field: Map<String, KPdfStream> sFontMap
    // (static - see module level)
    // Java field: Vector<String> mBackFontNames
    // (static - see M_BACK_FONT_NAMES)
    // Java field: int LATIN
    // Java field: int LATIN_EXTENED
    // Java field: int IPA_EXT
    // Java field: int SPACING_MOD
    // Java field: int COMBINGING_DIACRITICS_MARKS
    // Java field: int GREEK
    // Java field: int CYRILLIC
    // Java field: int ARMENIAN
    // Java field: int HEBREW
    // Java field: int ARABIC
    // Java field: int SYRIAC
    // Java field: int ARABIC_SUPPLEMENT
    // Java field: int THAANA
    // Java field: int NKO
    // Java field: int AVESTAN
    // Java field: int MANDAIC
    // Java field: int SAMARITAN
    // Java field: int DEVANAGARI
    // Java field: int BENGALI
    // Java field: int GURMUKHI
    // Java field: int GUJARATI
    // Java field: int ORIYA
    // Java field: int TAMIL
    // Java field: int TELUGU
    // Java field: int KANNADA
    // Java field: int MALAYALAM
    // Java field: int SINHALA
    // Java field: int THAI
    // Java field: int LAO
    // Java field: int TIBETAN
    // Java field: int MYANMAR
    // Java field: int GEORGIAN
    // Java field: int HANGUL_JAMO
    // Java field: int ETHIOPIC
    // Java field: int CHEROKEE
    // Java field: int UNIFIED_CANADIAN_AS
    // Java field: int OGHAM
    // Java field: int RUNIC
    // Java field: int TAGALOG
    // Java field: int HANUNOO
    // Java field: int BUHID
    // Java field: int TAGBANWA
    // Java field: int KHMER
    // Java field: int MONGOLIAN
    // Java field: int CHAM
    // Java field: int LIMBU
    // Java field: int TAI_LE
    // Java field: int NEW_TAI_LUE
    // Java field: int KMERSYMBOLS
    // Java field: int BUGINESE
    // Java field: int BATAK
    // Java field: int LANNA
    // Java field: int BALINESE
    // Java field: int SUNDANESE
    // Java field: int PAHAWH_HMONG
    // Java field: int LEPCHA
    // Java field: int OL_CHIKI
    // Java field: int MEITHEI
    // Java field: int PHONETIC_EXT
    // Java field: int PHONETIC_EXT_SUP
    // Java field: int COMBINING_DIACRITICS_MARKS_SUP
    // Java field: int LATIN_EXT_ADD
    // Java field: int GREEK_EXT
    // Java field: int GENERAL_MARKS
    // Java field: int GLAGOLITIC
    // Java field: int LATIC_EXT_C
    // Java field: int COPTIC
    // Java field: int GEORGIAN_SUPPLEMENT
    // Java field: int TIFINAGH
    // Java field: int ETHIOPIC_EXT
    // Java field: int SUPPLEMENTAL_PUNCTUATION
    // Java field: int CJK
    // Java field: int HIRAGANA_KATAKANA
    // Java field: int BOPOMOFO
    // Java field: int HANGUL_COMPATIBILITY_JAMO
    // Java field: int KANBUN
    // Java field: int BOPOMOFO_EXT
    // Java field: int CJK_STROKES
    // Java field: int KATAKANA_PHONETIC_EXT
    // Java field: int ENCLOSED_CJK_LETTERS_MONTHS
    // Java field: int CJK_COMPATIBILITY
    // Java field: int CJK_UNIFIED_IDEOGRAPHS_EXT_A
    // Java field: int YIJING
    // Java field: int CJK_UNIFIED_SYSBOLS
    // Java field: int YI
    // Java field: int VAI
    // Java field: int UNIFIED_CANADIAN_ABO_SYLL_SUP
    // Java field: int MODIFIER_TONE_LETTERS
    // Java field: int LATIN_EXT_D
    // Java field: int SYLOTI_NAGRI
    // Java field: int PHAGS_PA
    // Java field: int SAURASHTRA
    // Java field: int JAVANESE
    // Java field: int CHAKMA
    // Java field: int VARANG_KSHITI
    // Java field: int SORANG_SOMPENG
    // Java field: int NEWARI
    // Java field: int VITTHAI
    // Java field: int KAYAH_LI
    // Java field: int HANGUL_SYLLABLES
    // Java field: int ZONEOFUTF16
    // Java field: int PRIVATE
    // Java field: int CJK_ECOMPATIBILITY_IDEOGRAPHS
    // Java field: int ALPHABETIC_PRESENTATION_FORM
    // Java field: int ARABIC_PRESENTATION_FORMS_A
    // Java field: int VARIATION_SELECTOR
    // Java field: int VERTICAL_FORMS
    // Java field: int COMBINING_HALF_MARKS
    // Java field: int CJK_COMPATIBILITY_FORMS
    // Java field: int SMALL_FORM_VARIANTS
    // Java field: int ARABIC_PRESENTATION_FORMS_B
    // Java field: int HALF_FULL_WIDTH_FORM
    // Java field: int SPECIALS
    // (all constants defined at module level)
    // Java field: int[] unicodes
    // Java field: int[] sScales
    // (static arrays - see UNICODES, S_SCALES)
    // Java field: Map<Integer, String> mFontKeyName
    // Java field: ArrayList<String> sCJKNames
    // Java field: Map<String, String[]> mapTTC
    // (static maps - see module level)
}

// Java: public class KPdfFont extends KPdfDict
#[allow(non_snake_case)]
impl KPdfFont {
    // Java: static void setFont(IFontFamily family)
    pub fn setFont(_family: Box<dyn IFontFamily>) {
      // Java: mFontFamily = family;
    }

    // Java: static IFontFamily getFont()
    pub fn getFont() {
      // Java: return mFontFamily;
    }

    // Java: static Vector<KPdfFont> getFonts()
    pub fn getFonts() -> &'static Mutex<Vec<KPdfFont>> {
      // Java: return mFonts;
      get_fonts()
    }

    // Java: static void clearFonts()
    pub fn clearFonts() {
      // Java: mFonts.clear();
      get_fonts().lock().unwrap().clear();
    }

    // Java: static KPdfFont getFontResource(IFontFamily family, Paint paint, String fontPath)
    pub fn getFontResource(_family: &dyn IFontFamily, _paint: &Paint, _fontPath: &str) {
      // Java: int size = mFonts.size();
      let fonts = get_fonts().lock().unwrap();
      let size = fonts.len();
      // Java: for(int i = 0; i < size; ++i)
      for _i in 0..size {
         // Java: KPdfFont font = (KPdfFont)mFonts.get(i);
         // Java: if (font.mTypeface.isSameTypeface(family, fontPath))
         {
            // Java: return font;
         }
      }
      // Java: KAdvancedTypefaceMetrics info = KAdvancedTypefaceMetrics.getAdvancedTypefaceMetrics(family, paint);
      // Java: KPdfTypeface pdfTypeface = new KPdfTypeface(family, fontPath);
      // Java: KPdfFont pdfFont = new KPdfType0Font(info, pdfTypeface);
      // Java: mFonts.add(pdfFont);
      // Java: return pdfFont;
      todo!("getFontResource requires KPdfType0Font")
    }

    // Java: static Map<Integer, Integer> getCGMap(String path)
    pub fn getCGMap(path: &str) -> HashMap<i32, i32> {
      // Java: Map<Integer, Integer> cgMap = (Map)sUsedFontMap.get(path);
      let mut map = get_used_font_map().lock().unwrap();
      // Java: if (cgMap == null)
      if !map.contains_key(path) {
         // Java: cgMap = new HashMap();
         // Java: sUsedFontMap.put(path, cgMap);
         map.insert(path.to_string(), HashMap::new());
      }
      // Java: return cgMap;
      map.get(path).cloned().unwrap_or_default()
    }

    // Java: static void clearUsedFont()
    pub fn clearUsedFont() {
      // Java: sUsedFontMap.clear();
      get_used_font_map().lock().unwrap().clear();
    }

    // Java: static void addGlyph(String path, IFontFamily fontFamily, int c, int glyphIndex)
    pub fn addGlyph(path: &str, fontFamily: &dyn IFontFamily, c: i32, glyphIndex: i32) {
      // Java: int index = getFontIndex(path, fontFamily);
      let index = Self::getFontIndex(path, Some(fontFamily));
      // Java: Map<Integer, Integer> cgMap = getCGMap(index > 0 ? path + index : path);
      let key = if index > 0 { format!("{}{}", path, index) } else { path.to_string() };
      let mut map = get_used_font_map().lock().unwrap();
      let cgMap = map.entry(key).or_insert_with(HashMap::new);
      // Java: cgMap.put(c, glyphIndex);
      cgMap.insert(c, glyphIndex);
    }

    // Java: static int getGlyph(String path, IFontFamily fontFamily, int c)
    pub fn getGlyph(path: &str, fontFamily: &dyn IFontFamily, c: i32) -> i32 {
      // Java: int index = getFontIndex(path, fontFamily);
      let index = Self::getFontIndex(path, Some(fontFamily));
      // Java: Map<Integer, Integer> cgMap = getCGMap(index > 0 ? path + index : path);
      let key = if index > 0 { format!("{}{}", path, index) } else { path.to_string() };
      let mut map = get_used_font_map().lock().unwrap();
      let cgMap = map.entry(key).or_insert_with(HashMap::new);
      // Java: Integer glyph = (Integer)cgMap.get(c);
      // Java: if (glyph != null)
      if let Some(&glyph) = cgMap.get(&c) {
         // Java: return glyph;
         return glyph;
      }
      // Java: else
      {
         // Java: int g = KPdfUtils.getGlyphIndex(c, path, index);
         let g = -1_i32; // placeholder
         // Java: if (g >= 0)
         if g >= 0 {
            // Java: cgMap.put(c, g);
            cgMap.insert(c, g);
         }
         // Java: return g;
         g
      }
    }

    // Java: static void getGlyphs(String path, IFontFamily fontFamily, int[] cs, int[] gs)
    pub fn getGlyphs(path: &str, fontFamily: &dyn IFontFamily, cs: &[i32], gs: &mut [i32]) {
      // Java: int index = getFontIndex(path, fontFamily);
      let index = Self::getFontIndex(path, Some(fontFamily));
      // Java: Map<Integer, Integer> cgMap = getCGMap(index > 0 ? path + index : path);
      let key = if index > 0 { format!("{}{}", path, index) } else { path.to_string() };
      let mut map = get_used_font_map().lock().unwrap();
      let cgMap = map.entry(key).or_insert_with(HashMap::new);
      // Java: int[] offsets = new int[cs.length];
      let mut offsets = vec![0i32; cs.len()];
      // Java: int[] unicodes = new int[cs.length];
      let mut unicodes_buf = vec![0i32; cs.len()];
      // Java: int count = 0;
      let mut count = 0usize;
      // Java: for(int i = 0; i < cs.length; ++i)
      for i in 0..cs.len() {
         // Java: Integer glyph = (Integer)cgMap.get(cs[i]);
         // Java: if (glyph != null)
         if let Some(&glyph) = cgMap.get(&cs[i]) {
            // Java: gs[i] = glyph;
            gs[i] = glyph;
         }
         // Java: else
         else {
            // Java: offsets[count] = i;
            offsets[count] = i as i32;
            // Java: unicodes[count] = cs[i];
            unicodes_buf[count] = cs[i];
            // Java: ++count;
            count += 1;
         }
      }
      // Java: if (count > 0)
      if count > 0 {
         // Java: long tf = KPdfTypeface.getTypeface(path, index);
         let _tf = KPdfTypeface::getTypeface(path, index);
         // Java: int[] gIndex = new int[count];
         let g_index = vec![0i32; count];
         // Java: FreeTypeJNI.getGlyphIndexes(tf, unicodes, 0, count, gIndex, 0);
         // (FreeTypeJNI not available)
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count {
            // Java: if (gIndex[i] > 0)
            if g_index[i] > 0 {
               // Java: cgMap.put(unicodes[i], gIndex[i]);
               cgMap.insert(unicodes_buf[i], g_index[i]);
            }
            // Java: gs[offsets[i]] = gIndex[i];
            gs[offsets[i] as usize] = g_index[i];
         }
      }
    }

    // Java: static char[] getUsedChars(String path)
    pub fn getUsedChars(path: &str) -> Option<Vec<char>> {
      // Java: Map<Integer, Integer> cgMap = (Map)sUsedFontMap.get(path);
      let map = get_used_font_map().lock().unwrap();
      let cgMap = map.get(path);
      // Java: if (cgMap != null && cgMap.size() >= 1)
      if let Some(cgMap) = cgMap {
          if cgMap.len() >= 1 {
             // Java: char[] chars = new char[cgMap.size()];
             let mut chars = Vec::with_capacity(cgMap.len());
             // Java: int i = 0;
             // Java: for(int c : cgMap.keySet())
             for &c in cgMap.keys() {
                // Java: chars[i++] = (char)c;
                chars.push(char::from_u32(c as u32).unwrap_or('\0'));
             }
             // Java: return chars;
             return Some(chars);
          }
      }
      // Java: else
      {
         // Java: return null;
         None
      }
    }

    // Java: static int[] getUsedGlyphIndexs(String path)
    pub fn getUsedGlyphIndexs(path: &str) -> Option<Vec<i32>> {
      // Java: Map<Integer, Integer> cgMap = (Map)sUsedFontMap.get(path);
      let map = get_used_font_map().lock().unwrap();
      let cgMap = map.get(path);
      // Java: if (cgMap != null && cgMap.size() >= 1)
      if let Some(cgMap) = cgMap {
          if cgMap.len() >= 1 {
             // Java: int[] indexs = new int[cgMap.size()];
             let mut indexs = Vec::with_capacity(cgMap.len());
             // Java: int i = 0;
             // Java: for(int c : cgMap.values())
             for &c in cgMap.values() {
                // Java: indexs[i++] = c;
                indexs.push(c);
             }
             // Java: return indexs;
             return Some(indexs);
          }
      }
      // Java: else
      {
         // Java: return null;
         None
      }
    }

    // Java: static int getUsedGlyphSize(String path)
    pub fn getUsedGlyphSize(path: &str) -> i32 {
      // Java: Map<Integer, Integer> cgMap = (Map)sUsedFontMap.get(path);
      let map = get_used_font_map().lock().unwrap();
      // Java: return cgMap == null ? 0 : cgMap.size();
      map.get(path).map(|m| m.len() as i32).unwrap_or(0)
    }

    // Java: KPdfFont(KAdvancedTypefaceMetrics info, KPdfTypeface typeface)
    pub fn new(_info: KAdvancedTypefaceMetrics, typeface: KPdfTypeface) -> Self {
      // Java: super();
      // Java: this.mFontInfo = info;
      // Java: this.mTypeface = typeface;
      // Java: this.insertName("Type", "Font");
      Self {
          mTypeface: typeface,
          mResources: Vec::new(),
          mFontType: 4,
      }
    }

    // Java: void getResources(Vector<KPdfObject> resources)
    pub fn getResources(&self, _resources: &mut Vec<KPdfObject>) {
      // Java: this.getResourcesHelper(this.mResources, resources);
    }

    // Java: void addResource(KPdfObject object)
    pub fn addResource(&mut self, object: KPdfObject) {
      // Java: this.mResources.add(object);
      self.mResources.push(object);
    }

    // Java: void setFontDescriptor(KPdfDict descriptor)
    pub fn setFontDescriptor(&mut self, _descriptor: KPdfDict) {
      // Java: this.mDescriptor = descriptor;
    }

    // Java: KPdfType0Font getFontSubset()
    pub fn getFontSubset(&self) -> Option<KPdfType0Font> {
      // Java: return null;
      None
    }

    // Java: void setFontInfo(KAdvancedTypefaceMetrics info)
    pub fn setFontInfo(&mut self, _info: KAdvancedTypefaceMetrics) {
      // Java: this.mFontInfo = info;
    }

    // Java: boolean addCommonFontDescriptorEntries(int defaultWidth)
    pub fn addCommonFontDescriptorEntries(&mut self, _defaultWidth: i32) -> bool {
      // Java: if (this.mDescriptor == null)
      {
         // Java: return false;
         return false;
      }
      // Java: else
      {
         // Java: this.mDescriptor.insertName("FontName", KPdfUtils.getGBKBytes(this.mFontInfo.mFontName));
         // Java: this.mDescriptor.insert("Flags", 68);
         // Java: this.mDescriptor.insert("Ascent", this.mFontInfo.mAscent);
         // Java: this.mDescriptor.insert("Descent", this.mFontInfo.mDescent);
         // Java: this.mDescriptor.insert("StemV", this.mFontInfo.mStemV);
         // Java: this.mDescriptor.insert("CapHeight", this.mFontInfo.mCapHeight);
         // Java: this.mDescriptor.insert("ItalicAngle", this.mFontInfo.mItalicAngle);
         // Java: this.mDescriptor.insert((String)"FontBBox", this.makeFontBBox(new RectF(-7.8125F, -144.5313F, 1000.0F, 859.375F)));
         // Java: return true;
         true
      }
    }

    // Java: KPdfArray makeFontBBox(RectF rect)
    pub fn makeFontBBox(&self, rect: RectF) -> KPdfArray {
      // Java: KPdfArray bbox = new KPdfArray();
      let mut bbox = KPdfArray::new();
      // Java: bbox.append((KPdfObject)(new KPdfScalar(rect.left)));
      bbox.appendFloat(rect.left);
      // Java: bbox.append((KPdfObject)(new KPdfScalar(rect.top)));
      bbox.appendFloat(rect.top);
      // Java: bbox.append((KPdfObject)(new KPdfScalar(rect.right)));
      bbox.appendFloat(rect.right);
      // Java: bbox.append((KPdfObject)(new KPdfScalar(rect.bottom)));
      bbox.appendFloat(rect.bottom);
      // Java: return bbox;
      bbox
    }

    // Java: static KPdfStream getFontStream(String fontFile)
    pub fn getFontStream(_fontFile: Option<&str>) -> Option<KPdfStream> {
      // Java: if (fontFile == null)
      let fontFile = match _fontFile {
          None => {
             // Java: return null;
             return None;
          }
          Some(f) => f,
      };
      // Java: else
      {
         // Java: File file = new File(fontFile);
         // Java: if (!file.exists())
         if !std::path::Path::new(fontFile).exists() {
            // Java: return null;
            return None;
         }
         // Java: else
         {
            // Java: KPdfStream stream = (KPdfStream)sFontMap.get(fontFile);
            // Java: if (stream == null)
            {
               // Java: try
               {
                  // Java: KFileReaderStream fileReaderStream = new KFileReaderStream(fontFile);
                  // Java: stream = new KPdfStream(fileReaderStream);
                  // Java: sFontMap.put(fontFile, stream);
               }
               // Java: catch (IOException e)
               {
                  // Java: e.printStackTrace();
               }
            }
            // Java: return stream;
            None
         }
      }
    }

    // Java: static void clearFontsStream()
    pub fn clearFontsStream() {
      // Java: sFontMap.clear();
    }

    // Java: static Vector<String> getBackFonts()
    pub fn getBackFonts() -> Vec<String> {
      // Java: if (mBackFontNames == null)
      let mut names = get_back_font_names().lock().unwrap();
      if names.is_empty() {
         // Java: mBackFontNames = new Vector();
         // Java: mBackFontNames.add("serif");
         names.push("serif".to_string());
         // Java: mBackFontNames.add("宋体");
         names.push("宋体".to_string());
      }
      // Java: return mBackFontNames;
      names.clone()
    }

    // Java: static int getUnicodeScale(char c)
    pub fn getUnicodeScale(c: char) -> i32 {
      // Java: int index = getUnicode(unicodes, 0, unicodes.length, c);
      let index = Self::getUnicode(UNICODES, 0, UNICODES.len() as i32, c);
      // Java: return sScales[index];
      S_SCALES[index as usize]
    }

    // Java: static int getUnicode(int[] unicodes, int index, int count, char c)
    pub fn getUnicode(unicodes: &[i32], index: i32, count: i32, c: char) -> i32 {
      let cv = c as i32;
      // Java: for(int i = index + 1; i < count; ++i)
      for i in (index + 1)..count {
         // Java: if (c > unicodes[i - 1] && c <= unicodes[i])
         if cv > unicodes[(i - 1) as usize] && cv <= unicodes[i as usize] {
            // Java: return i;
            return i;
         }
      }
      // Java: return count - 1;
      count - 1
    }

    // Java: static String getFontKeyName(int index)
    pub fn getFontKeyName(index: i32) -> String {
      // Java: if (mFontKeyName == null)
      let mut map = get_font_key_name().lock().unwrap();
      if map.is_empty() {
         // Java: mFontKeyName = new HashMap();
         // Java: mFontKeyName.put(10, "Arabic");
         map.insert(10, "Arabic");
         // Java: mFontKeyName.put(12, "Arabic");
         map.insert(12, "Arabic");
         // Java: mFontKeyName.put(105, "Arabic");
         map.insert(105, "Arabic");
         // Java: mFontKeyName.put(111, "Arabic");
         map.insert(111, "Arabic");
         // Java: mFontKeyName.put(6, "Greek");
         map.insert(6, "Greek");
         // Java: mFontKeyName.put(7, "Cyrillic");
         map.insert(7, "Cyrillic");
         // Java: mFontKeyName.put(8, "Armenian");
         map.insert(8, "Armenian");
         // Java: mFontKeyName.put(9, "Hebrew");
         map.insert(9, "Hebrew");
         // Java: mFontKeyName.put(11, "Syriac");
         map.insert(11, "Syriac");
         // Java: mFontKeyName.put(13, "Thaana");
         map.insert(13, "Thaana");
         // Java: mFontKeyName.put(14, "NKO");
         map.insert(14, "NKO");
         // Java: mFontKeyName.put(16, "Mandaic");
         map.insert(16, "Mandaic");
         // Java: mFontKeyName.put(15, "Avestan");
         map.insert(15, "Avestan");
         // Java: mFontKeyName.put(17, "Samaritan");
         map.insert(17, "Samaritan");
         // Java: mFontKeyName.put(18, "Devanagari");
         map.insert(18, "Devanagari");
         // Java: mFontKeyName.put(19, "Bengali");
         map.insert(19, "Bengali");
         // Java: mFontKeyName.put(20, "Gurmukhi");
         map.insert(20, "Gurmukhi");
         // Java: mFontKeyName.put(21, "Gujarati");
         map.insert(21, "Gujarati");
         // Java: mFontKeyName.put(22, "Oriya");
         map.insert(22, "Oriya");
         // Java: mFontKeyName.put(23, "Tamil");
         map.insert(23, "Tamil");
         // Java: mFontKeyName.put(24, "Telugu");
         map.insert(24, "Telugu");
         // Java: mFontKeyName.put(25, "Kannada");
         map.insert(25, "Kannada");
         // Java: mFontKeyName.put(26, "Malayalam");
         map.insert(26, "Malayalam");
         // Java: mFontKeyName.put(27, "Sinhala");
         map.insert(27, "Sinhala");
         // Java: mFontKeyName.put(28, "Thai");
         map.insert(28, "Thai");
         // Java: mFontKeyName.put(29, "Lao");
         map.insert(29, "Lao");
         // Java: mFontKeyName.put(30, "Tibetan");
         map.insert(30, "Tibetan");
         // Java: mFontKeyName.put(31, "Myanmar/Padauk/Zawgyi");
         map.insert(31, "Myanmar/Padauk/Zawgyi");
         // Java: mFontKeyName.put(32, "Georgian");
         map.insert(32, "Georgian");
         // Java: mFontKeyName.put(68, "Georgian");
         map.insert(68, "Georgian");
         // Java: mFontKeyName.put(33, "Hangul/KR/SC");
         map.insert(33, "Hangul/KR/SC");
         // Java: mFontKeyName.put(75, "Hangul/KR/SC");
         map.insert(75, "Hangul/KR/SC");
         // Java: mFontKeyName.put(100, "Hangul/KR/SC");
         map.insert(100, "Hangul/KR/SC");
         // Java: mFontKeyName.put(34, "Ethiopic");
         map.insert(34, "Ethiopic");
         // Java: mFontKeyName.put(35, "Cherokee");
         map.insert(35, "Cherokee");
         // Java: mFontKeyName.put(36, "Canadian");
         map.insert(36, "Canadian");
         // Java: mFontKeyName.put(87, "Canadian");
         map.insert(87, "Canadian");
         // Java: mFontKeyName.put(37, "Ogham");
         map.insert(37, "Ogham");
         // Java: mFontKeyName.put(38, "Runic");
         map.insert(38, "Runic");
         // Java: mFontKeyName.put(39, "Tagalog");
         map.insert(39, "Tagalog");
         // Java: mFontKeyName.put(40, "Hanunoo");
         map.insert(40, "Hanunoo");
         // Java: mFontKeyName.put(41, "Buhid");
         map.insert(41, "Buhid");
         // Java: mFontKeyName.put(42, "Tagbanwa");
         map.insert(42, "Tagbanwa");
         // Java: mFontKeyName.put(43, "Khmer");
         map.insert(43, "Khmer");
         // Java: mFontKeyName.put(44, "Mongolian");
         map.insert(44, "Mongolian");
         // Java: mFontKeyName.put(45, "Cham");
         map.insert(45, "Cham");
         // Java: mFontKeyName.put(46, "Limbu");
         map.insert(46, "Limbu");
         // Java: mFontKeyName.put(47, "Tai Le");
         map.insert(47, "Tai Le");
         // Java: mFontKeyName.put(48, "New Tai Lue");
         map.insert(48, "New Tai Lue");
         // Java: mFontKeyName.put(50, "Buginese");
         map.insert(50, "Buginese");
         // Java: mFontKeyName.put(51, "Batak");
         map.insert(51, "Batak");
         // Java: mFontKeyName.put(53, "Balinese");
         map.insert(53, "Balinese");
         // Java: mFontKeyName.put(54, "Sundanese");
         map.insert(54, "Sundanese");
         // Java: mFontKeyName.put(56, "Lepcha");
         map.insert(56, "Lepcha");
         // Java: mFontKeyName.put(57, "Ol Chiki");
         map.insert(57, "Ol Chiki");
         // Java: mFontKeyName.put(65, "Glagolitic");
         map.insert(65, "Glagolitic");
         // Java: mFontKeyName.put(67, "Coptic");
         map.insert(67, "Coptic");
         // Java: mFontKeyName.put(69, "Tifinagh");
         map.insert(69, "Tifinagh");
         // Java: mFontKeyName.put(85, "Yi");
         map.insert(85, "Yi");
         // Java: mFontKeyName.put(86, "Vai");
         map.insert(86, "Vai");
         // Java: mFontKeyName.put(90, "Syloti");
         map.insert(90, "Syloti");
         // Java: mFontKeyName.put(91, "Phags");
         map.insert(91, "Phags");
         // Java: mFontKeyName.put(92, "Saurashtra");
         map.insert(92, "Saurashtra");
         // Java: mFontKeyName.put(93, "Javanese");
         map.insert(93, "Javanese");
         // Java: mFontKeyName.put(99, "Kayah Li");
         map.insert(99, "Kayah Li");
         // Java: mFontKeyName.put(73, "JP/Motoya");
         map.insert(73, "JP/Motoya");
         // Java: mFontKeyName.put(79, "JP/Motoya/MTLmr");
         map.insert(79, "JP/Motoya/MTLmr");
         // Java: mFontKeyName.put(64, "Symbol/Serif/Motoya/MTLmr/SC");
         map.insert(64, "Symbol/Serif/Motoya/MTLmr/SC");
      }
      // Java: String key = (String)mFontKeyName.get(index);
      let key = map.get(&index).copied();
      // Java: if (key == null)
      if let Some(key) = key {
          return key.to_string();
      }
      {
         // Java: UILanguage lang = Platform.getLanguage();
         let lang = Platform::getLanguage();
         // Java: if (lang == UILanguage.UILanguage_chinese)
         if lang == UILanguage::UILanguage_chinese {
            // Java: key = "SC/Serif/Sans Fallback/Sans Mono/Han";
            return "SC/Serif/Sans Fallback/Sans Mono/Han".to_string();
         }
         // Java: else if (lang == UILanguage.UILanguage_japan)
         else if lang == UILanguage::UILanguage_japan {
            // Java: key = "Serif/Sans Fallback/JP/SC/Sans Mono/Han";
            return "Serif/Sans Fallback/JP/SC/Sans Mono/Han".to_string();
         }
         // Java: else if (lang == UILanguage.UILanguage_korean)
         else if lang == UILanguage::UILanguage_korean {
            // Java: key = "Serif/Sans Fallback/KR/SC/Sans Mono/Han";
            return "Serif/Sans Fallback/KR/SC/Sans Mono/Han".to_string();
         }
         // Java: else if (lang != UILanguage.UILanguage_taiwan && lang != UILanguage.UILanguage_hongkong)
         else if lang != UILanguage::UILanguage_taiwan && lang != UILanguage::UILanguage_hongkong {
            // Java: key = "Serif/Sans Fallback/SC/Sans Mono/Han";
            return "Serif/Sans Fallback/SC/Sans Mono/Han".to_string();
         }
         // Java: else
         else {
            // Java: key = "Serif/Sans Fallback/TC/SC/Sans Mono/Han";
            return "Serif/Sans Fallback/TC/SC/Sans Mono/Han".to_string();
         }
      }
      // Java: return key;
    }

    // Java: static boolean isCJK(int scale)
    pub fn isCJK(scale: i32) -> bool {
      // Java: return scale == 72 || scale == 80 || scale == 81 || scale == 109 || scale == 103 || scale == 78 || scale == 82 || scale == 84 || scale == 112;
      scale == 72 || scale == 80 || scale == 81 || scale == 109 || scale == 103 || scale == 78 || scale == 82 || scale == 84 || scale == 112
    }

    // Java: static ArrayList<String> getCJKName()
    pub fn getCJKName() -> Vec<String> {
      // Java: if (sCJKNames == null)
      {
         // Java: sCJKNames = FontHost.getFontTable().getCJKName();
      }
      // Java: return sCJKNames;
      FontHost::getFontTable().getCJKName()
    }

    // Java: static void addFuzzyFont(Set<String> nameSets, ArrayList<String> names, String key)
    pub fn addFuzzyFont(name_sets: &[String], names: &mut Vec<String>, key: &str) {
      // Java: for(String str : nameSets)
      for str in name_sets {
         // Java: if (str.indexOf(key) >= 0 && !names.contains(str))
         if str.contains(key) && !names.contains(str) {
            // Java: names.add(str);
            names.push(str.clone());
         }
      }
    }

    // Java: static ArrayList<IFontFamily> getFontFamily(char c)
    pub fn getFontFamily(_c: char) -> Option<Vec<String>> {
      // Java: IFontTable fontTable = FontHost.getFontTable();
      // Java: ArrayList<String> names = new ArrayList();
      let mut names: Vec<String> = Vec::new();
      // Java: int unicodeScale = getUnicodeScale(c);
      let unicodeScale = Self::getUnicodeScale(_c);
      // Java: String key = getFontKeyName(unicodeScale);
      let key = Self::getFontKeyName(unicodeScale);
      // Java: if (key != null)
      {
         // Java: Map<String, IFontFamily> fontMap = fontTable.getFontMap();
         // Java: Set<String> nameSets = fontMap.keySet();
         let name_sets: Vec<String> = Vec::new();
         // Java: int index = key.indexOf(47);
         let slash_idx = key.find('/');
         // Java: if (index > 0)
         if let Some(_idx) = slash_idx {
            // Java: int last;
            // Java: for(last = 0; index > 0; index = key.indexOf(47, last))
            for part in key.split('/') {
               // Java: String keyName = key.substring(last, index);
               // Java: addFuzzyFont(nameSets, names, keyName);
               Self::addFuzzyFont(&name_sets, &mut names, part);
               // Java: last = index + 1;
            }
            // Java: String keyName = key.substring(last, key.length());
            // Java: addFuzzyFont(nameSets, names, keyName);
         }
         // Java: else
         else {
            // Java: addFuzzyFont(nameSets, names, key);
            Self::addFuzzyFont(&name_sets, &mut names, &key);
         }
      }
      // Java: if (isCJK(unicodeScale))
      if Self::isCJK(unicodeScale) {
         // Java: ArrayList<String> cjks = getCJKName();
         let cjks = Self::getCJKName();
         // Java: if (cjks != null && cjks.size() > 0)
         if !cjks.is_empty() {
            // Java: for(int i = 0; i < cjks.size(); ++i)
            for i in 0..cjks.len() {
               // Java: String str = (String)cjks.get(i);
               let str = &cjks[i];
               // Java: if (!names.contains(str))
               if !names.contains(str) {
                  // Java: names.add(str);
                  names.push(str.clone());
               }
            }
         }
      }
      // Java: int size = names.size();
      let size = names.len();
      // Java: if (names.size() == 0)
      if size == 0 {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: ArrayList<IFontFamily> families = new ArrayList(size);
         // Java: for(int i = 0; i < size; ++i)
         // Java: IFontFamily fontFamily = fontTable.get((String)names.get(i), true);
         // Java: if (fontFamily != null)
         // Java: families.add(fontFamily);
         // Java: return families;
         Some(names)
      }
    }

    // Java: void populateToUnicodeTable()
    pub fn populateToUnicodeTable(&mut self) {
      // Java: String path = this.mTypeface.mFontPath;
      let _path = &self.mTypeface.mFontPath;
      // Java: int index = getFontIndex(path, this.mTypeface.mFamily);
      // Java: if (index > 0)
      {
         // Java: path = path + index;
      }
      // Java: KPdfStream cmap = generateToUnicodeCmap(path);
      // Java: this.addResource(cmap);
      // Java: this.insert("ToUnicode", new KPdfObjectRef(cmap));
    }

    // Java: static KPdfStream generateToUnicodeCmap(String path)
    pub fn generateToUnicodeCmap(_path: &str) {
      // Java: KDynamicMemoryStream cmap = new KDynamicMemoryStream();
      // Java: appendToUnicodeHeader(cmap);
      // Java: appendCmapSections(cmap, path);
      // Java: appendCmapFooter(cmap);
      // Java: return new KPdfStream(cmap);
    }

    // Java: static void appendToUnicodeHeader(KDynamicMemoryStream cmap)
    pub fn appendToUnicodeHeader(cmap: &mut KStream) {
      // Java: String header = "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CIDSystemInfo\n<<  /Registry (Adobe)\n/Ordering (UCS)\n/Supplement 0\n>> def\n/CMapName /Adobe-Identity-UCS def\n/CMapType 2 def\n1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n";
      let header = "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CIDSystemInfo\n<<  /Registry (Adobe)\n/Ordering (UCS)\n/Supplement 0\n>> def\n/CMapName /Adobe-Identity-UCS def\n/CMapType 2 def\n1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n";
      // Java: cmap.write(header);
      cmap.write_str(header);
    }

    // Java: static void appendCmapSections(KDynamicMemoryStream cmap, String path)
    pub fn appendCmapSections(cmap: &mut KStream, path: &str) {
      // Java: if (path != null)
      {
         // Java: Set<Map.Entry<Integer, Integer>> entries = getUsedEntrySet(path);
         let map = get_used_font_map().lock().unwrap();
         let entries = map.get(path);
         // Java: if (entries != null && entries.size() > 0)
         if let Some(entries) = entries {
            if !entries.is_empty() {
               // Java: int size = entries.size();
               let size = entries.len();
               // Java: cmap.write(size);
               cmap.write_int(size as i32);
               // Java: cmap.write(" beginbfchar\n");
               cmap.write_str(" beginbfchar\n");
               // Java: for(Map.Entry<Integer, Integer> entry : entries)
               for (key, value) in entries {
                  // Java: String ci = "<" + String.format(Locale.ENGLISH, "%04x", entry.getValue()) + "> <" + String.format(Locale.ENGLISH, "%04x", entry.getKey()) + ">\n";
                  let ci = format!("<{:04x}> <{:04x}>\n", value, key);
                  // Java: cmap.write(ci);
                  cmap.write_str(&ci);
               }
               // Java: cmap.write("endbfchar\n");
               cmap.write_str("endbfchar\n");
            }
         }
      }
    }

    // Java: static void appendCmapFooter(KDynamicMemoryStream cmap)
    pub fn appendCmapFooter(cmap: &mut KStream) {
      // Java: String footer = "endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend";
      let footer = "endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend";
      // Java: cmap.write(footer);
      cmap.write_str(footer);
    }

    // Java: static String[] getTTFName(String path)
    pub fn getTTFName(path: &str) -> Option<Vec<String>> {
      // Java: if (path.endsWith(".TTC") || path.endsWith(".ttc"))
      if path.ends_with(".TTC") || path.ends_with(".ttc") {
         // Java: if (mapTTC.containsKey(path))
         let mut map = get_map_ttc().lock().unwrap();
         if let Some(names) = map.get(path) {
            // Java: return (String[])mapTTC.get(path);
            return Some(names.clone());
         }
         // Java: try
         {
            // Java: String[][] ttfs = TTCParser.getFontNames(new File(path));
            // Java: int size = ttfs.length;
            // Java: if (size < 1)
            {
               // Java: return null;
               return None;
            }
            // Java: String[] names = new String[size];
            // Java: for(int i = 0; i < size; ++i)
            // Java: names[i] = ttfs[i][0];
            // Java: mapTTC.put(path, names);
            // Java: return names;
         }
         // Java: catch (IOException e)
         {
            // Java: e.printStackTrace();
         }
      }
      // Java: return null;
      None
    }

    // Java: static int getFontIndex(String path, IFontFamily fontFamily)
    pub fn getFontIndex(path: &str, fontFamily: Option<&dyn IFontFamily>) -> i32 {
      // Java: if (fontFamily == null)
      let fontFamily = match fontFamily {
          None => {
             // Java: return 0;
             return 0;
          }
          Some(f) => f,
      };
      // Java: else
      {
         // Java: String[] names = getTTFName(path);
         let names = Self::getTTFName(path);
         // Java: if (names == null)
         let names = match names {
             None => {
                // Java: return 0;
                return 0;
             }
             Some(n) => n,
         };
         // Java: else
         {
            // Java: String name = fontFamily.getName();
            let name = fontFamily.getName();
            // Java: for(int i = 0; i < names.length; ++i)
            for i in 0..names.len() {
               // Java: if (names[i].equals(name))
               if names[i] == name {
                  // Java: return i;
                  return i as i32;
               }
            }
            // Java: return 0;
            0
         }
      }
    }

}

fn getUsedEntrySet(path: &str) -> Option<HashMap<i32, i32>> {
    let map = get_used_font_map().lock().unwrap();
    let cgMap = map.get(path)?;
    if cgMap.is_empty() { None } else { Some(cgMap.clone()) }
}
