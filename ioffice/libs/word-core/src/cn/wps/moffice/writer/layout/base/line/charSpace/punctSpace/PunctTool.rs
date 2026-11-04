// Java: import cn.wps.font.ITypeface
use thirdparty_font::cn::wps::font::ITypeface::ITypeface;
// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.writer.cache.TypoRun
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
// Java: import java.util.Arrays

// java private static final char[] BEFORE_COMPRESSABLE_PUNCTS = getSortedArray("‘“〈《「『【〔〖（［｛");
static BEFORE_COMPRESSABLE_PUNCTS: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray("‘“〈《「『【〔〖（［｛"));
// java private static final char[] AFTER_COMPRESSABLE_PUNCTS = getSortedArray("·’”∶、。〃〉》」』】〕〗！｝］＇），．：；");
static AFTER_COMPRESSABLE_PUNCTS: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray("·’”∶、。〃〉》」』】〕〗！｝］＇），．：；"));
// java private static final char[] LINE_HEAD_PUNCTS = getSortedArray("$([{£¥·‘“〈《「『【〔〖〝﹙﹛﹝＄（．［｛￡￥");
static LINE_HEAD_PUNCTS: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray("$([{£¥·‘“〈《「『【〔〖〝﹙﹛﹝＄（．［｛￡￥"));
// java private static final char[] LINE_TAIL_PUNCTS = getSortedArray("!%),.:;>?]}¢¨°·ˇˉ―‖’”…‰′″›℃∶、。〃々〉》」』】〕〗〞︶︺︾﹀﹄﹚﹜﹞！＂％＇），．：；？］｀｜｝～￠");
static LINE_TAIL_PUNCTS: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray("!%),.:;>?]}¢¨°·ˇˉ―‖’”…‰′″›℃∶、。〃々〉》」』】〕〗〞︶︺︾﹀﹄﹚﹜﹞！＂％＇），．：；？］｀｜｝～￠"));
// java private static final char[] HANGABLE_PUNCTS_DEFAULT = getSortedArray("!),.:;?]}￠ˇ’”∶＂");
static HANGABLE_PUNCTS_DEFAULT: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray("!),.:;?]}￠ˇ’”∶＂"));
// java private static final char[] HANGABLE_PUNCTS_DEFAULT_ALT = getSortedArray(",.!),:;?]}");
static HANGABLE_PUNCTS_DEFAULT_ALT: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray(",.!),:;?]}"));
// java private static final char[] HANGABLE_PUNCTS_JP = getSortedArray(",.!),:;?]}、，。");
static HANGABLE_PUNCTS_JP: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray(",.!),:;?]}、，。"));
// java private static final char[] HANGABLE_PUNCTS_KR = getSortedArray(",.!),:;?]}，°");
static HANGABLE_PUNCTS_KR: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray(",.!),:;?]}，°"));
// java private static final char[] HANGABLE_PUNCTS_CN = getSortedArray("!),.:;?]}￠ˇ’”∶、。〉》」』】〕〗！＂＇），．·：；");
static HANGABLE_PUNCTS_CN: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray("!),.:;?]}￠ˇ’”∶、。〉》」』】〕〗！＂＇），．·：；"));
// java private static final char[] HANGABLE_PUNCTS_CN_SIMPLIFIED = getSortedArray("°");
static HANGABLE_PUNCTS_CN_SIMPLIFIED: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray("°"));
// java private static final char[] HANGABLE_PUNCTS_CN_ALT = getSortedArray(",.!),:;?]}");
static HANGABLE_PUNCTS_CN_ALT: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray(",.!),:;?]}"));
// java private static final char[] ROTATED_PUNCTS = getSortedArray("｀、，．：；！＂＇？￠￡￥〃。々");
static ROTATED_PUNCTS: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray("｀、，．：；！＂＇？￠￡￥〃。々"));
// java private static final char[] ROTATED_PUNCTS_CN = getSortedArray("ˇˉ°¨∶");
static ROTATED_PUNCTS_CN: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray("ˇˉ°¨∶"));
// java private static final char[] PUNCT_NEED_FIX_IN_SERIF = getSortedArray("，？！：；。");
static PUNCT_NEED_FIX_IN_SERIF: std::sync::LazyLock<Vec<char>> =
                    std::sync::LazyLock::new(|| PunctTool::getSortedArray("，？！：；。"));
// java private static RectF[] PUNCT_RECT;
static PUNCT_RECT: std::sync::OnceLock<Option<Vec<RectF>>> =
                std::sync::OnceLock::new();
// java private static final float CALC_FONTSIZE = 100.0F;
const CALC_FONTSIZE: f32 = 100.0f32;
// java public static final float SERIF_PUNCT_OFFSET = 0.2F;
pub(crate) const SERIF_PUNCT_OFFSET: f32 = 0.2f32;

// java public class PunctTool
#[derive(Default)]
pub struct PunctTool {
}

// java public class PunctTool
impl PunctTool {
    // java public static boolean isSerifPunct(char c, ITypeface typeface)
    pub fn isSerifPunct(c: char, typeface: &dyn ITypeface) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return typeface.getFamily().getName() == "serif" && isInSortedArray(PUNCT_NEED_FIX_IN_SERIF, c);
      return typeface.getFamily().getName() == "serif" && Self::isInSortedArray(PUNCT_NEED_FIX_IN_SERIF, c);
      */
    }

    // java public static int getFixSerifPunctOffset(char c, ITypeface typeface, int fontSize, int orgWidth, float charScale)
    pub fn getFixSerifPunctOffset(c: char, typeface: &dyn ITypeface, fontSize: i32, orgWidth: i32, charScale: f32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int index = Arrays.binarySearch(PUNCT_NEED_FIX_IN_SERIF, c);
      let mut index: i32 = Arrays::binarySearch(PUNCT_NEED_FIX_IN_SERIF, c);
      // java if (index < 0)
      if index < 0 {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java RectF rect = PUNCT_RECT[index];
        let mut rect: RectF = PUNCT_RECT[(index) as usize];
        // java if (null == rect)
        if false {
          // java rect = typeface.getTextRect(100.0F, c);
          rect = typeface.getTextRect__float_char(100, c);
          // java PUNCT_RECT[index] = rect;
          PUNCT_RECT[(index) as usize] = rect;
        }
        // java return (int)((-(rect.left * (float)fontSize / 100.0F) + (float)orgWidth * 0.2F) * charScale);
        return ((-rect.left * ((fontSize) as f32) / 100 + ((orgWidth) as f32) * 0.2 * charScale) as i32);
      }
      */
    }

    // java public static boolean needAlignRightWhileRotation(char ch)
    pub fn needAlignRightWhileRotation(ch: char) -> bool {
      // java switch (ch)
      match ch {
        // java case '、': case '。': case '！': case '，': case '．': case '：': case '；': case '？':
        '、' | '。' | '！' | '，' | '．' | '：' | '；' | '？' => {
          // java return true;
          return true;
        }
        // java default:
        _ => {
          // java return false;
          return false;
        }
      }
    }

    // java public static boolean needAlignTopWhileRotation(char ch)
    pub fn needAlignTopWhileRotation(ch: char) -> bool {
      // java switch (ch)
      match ch {
        // java case '、': case '。': case '，': case '．':
        '、' | '。' | '，' | '．' => {
          // java return true;
          return true;
        }
        // java default:
        _ => {
          // java return false;
          return false;
        }
      }
    }

    // java public static boolean needAlignCenterWhileRotation(char ch)
    pub fn needAlignCenterWhileRotation(ch: char) -> bool {
      // java switch (ch)
      match ch {
        // java case '‘': case '’': case '“': case '”':
        '‘' | '’' | '“' | '”' => {
          // java return true;
          return true;
        }
        // java case '‚': case '‛': default:
        '‚' | '‛' | _ => {
          // java return false;
          return false;
        }
      }
    }

    // java static boolean isInSortedArray(char[] array, char ch)
    pub fn isInSortedArray(array: &[char], ch: char) -> bool {
      // java return Arrays.binarySearch(array, ch) >= 0;
      return array.binary_search(&ch).is_ok();
    }

    // java static char[] getSortedArray(String str)
    pub fn getSortedArray(str: &str) -> Vec<char> {
      // java char[] ret = str.toCharArray();
      let mut ret: Vec<char> = str.chars().collect();
      // java Arrays.sort(ret);
      ret.sort_unstable();
      // java return ret;
      return ret;
    }

    // java public static boolean isBeforeCompresssablePunct(char ch)
    pub fn isBeforeCompresssablePunct(ch: char) -> bool {
      // java return isInSortedArray(BEFORE_COMPRESSABLE_PUNCTS, ch);
      return Self::isInSortedArray(BEFORE_COMPRESSABLE_PUNCTS.as_slice(), ch);
    }

    // java public static boolean isAfterCompressablePunct(char ch)
    pub fn isAfterCompressablePunct(ch: char) -> bool {
      // java return isInSortedArray(AFTER_COMPRESSABLE_PUNCTS, ch);
      return Self::isInSortedArray(AFTER_COMPRESSABLE_PUNCTS.as_slice(), ch);
    }

    // java public static boolean isHeadPunct(char ch)
    pub fn isHeadPunct(ch: char) -> bool {
      // java return isInSortedArray(LINE_HEAD_PUNCTS, ch);
      return Self::isInSortedArray(LINE_HEAD_PUNCTS.as_slice(), ch);
    }

    // java public static boolean isTailPunct(char ch)
    pub fn isTailPunct(ch: char) -> bool {
      // java return isInSortedArray(LINE_TAIL_PUNCTS, ch);
      return Self::isInSortedArray(LINE_TAIL_PUNCTS.as_slice(), ch);
    }

    // java public static boolean isPunct(int codepoint)
    pub fn isPunct(codepoint: i32) -> bool {
      // java switch (codepoint)
      match codepoint {
        // java case 33: case 34: case 40: case 41: case 44: case 45: case 46: case 58: case 59: case 63: case 91: case 93: case 123: case 125: case 168: case 176: case 183: case 711: case 713: case 8213: case 8214: case 8216: case 8217: case 8220: case 8221: case 8230: case 8758: case 12289: case 12290: case 12291: case 12293: case 12296: case 12297: case 12298: case 12299: case 12300: case 12301: case 12302: case 12303: case 12304: case 12305: case 12308: case 12309: case 12310: case 12311: case 65281: case 65282: case 65287: case 65288: case 65289: case 65292: case 65294: case 65306: case 65307: case 65311: case 65339: case 65341: case 65344: case 65371: case 65372: case 65373: case 65374: case 65504: case 65505: case 65509:
        33 | 34 | 40 | 41 | 44 | 45 | 46 | 58 | 59 | 63 | 91 | 93 | 123 | 125 | 168 | 176 | 183 | 711 | 713 | 8213 | 8214 | 8216 | 8217 | 8220 | 8221 | 8230 | 8758 | 12289 | 12290 | 12291 | 12293 | 12296 | 12297 | 12298 | 12299 | 12300 | 12301 | 12302 | 12303 | 12304 | 12305 | 12308 | 12309 | 12310 | 12311 | 65281 | 65282 | 65287 | 65288 | 65289 | 65292 | 65294 | 65306 | 65307 | 65311 | 65339 | 65341 | 65344 | 65371 | 65372 | 65373 | 65374 | 65504 | 65505 | 65509 => {
          // java return true;
          return true;
        }
        // java default:
        _ => {
          // java return false;
          return false;
        }
      }
    }

    // java public static int getReplacedRotatePunct(int codepoint)
    pub fn getReplacedRotatePunct(codepoint: i32) -> i32 {
      // java switch (codepoint)
      match codepoint {
        // java case 8216:
        8216 => {
          // java return 12300;
          return 12300;
        }
        // java case 8217:
        8217 => {
          // java return 12301;
          return 12301;
        }
        // java case 8220:
        8220 => {
          // java return 12302;
          return 12302;
        }
        // java case 8221:
        8221 => {
          // java return 12303;
          return 12303;
        }
        // java case 8218: case 8219: default:
        8218 | 8219 | _ => {
          // java return codepoint;
          return codepoint;
        }
      }
    }

    // java public static boolean isRotatedPunct(int codePoint, int langType)
    pub fn isRotatedPunct(codePoint: i32, langType: i32) -> bool {
      // java char ch = (char)codePoint;
      let mut ch: char = char::from_u32(codePoint as u32).unwrap_or('\0');
      // java if (isInSortedArray(ROTATED_PUNCTS, ch))
      if Self::isInSortedArray(ROTATED_PUNCTS.as_slice(), ch) {
        // java return true;
        return true;
      }
      // java else
      else {
        // java return TypoRun.isFarEastLangType(langType) && isInSortedArray(ROTATED_PUNCTS_CN, ch);
        return TypoRun::isFarEastLangType(langType) && Self::isInSortedArray(ROTATED_PUNCTS_CN.as_slice(), ch);
      }
    }

    // java private static boolean isHangablePunctChinese(int lid, char ch, boolean useAltLineBreakRules)
    pub fn isHangablePunctChinese(lid: i32, ch: char, useAltLineBreakRules: bool) -> bool {
      // java if (useAltLineBreakRules)
      if useAltLineBreakRules {
        // java return isInSortedArray(HANGABLE_PUNCTS_CN_ALT, ch);
        return Self::isInSortedArray(HANGABLE_PUNCTS_CN_ALT.as_slice(), ch);
      }
      // java else
      else {
        // java return 2052 == lid && isInSortedArray(HANGABLE_PUNCTS_CN_SIMPLIFIED, ch) ? true : isInSortedArray(HANGABLE_PUNCTS_CN, ch);
        return (if 2052 == lid && Self::isInSortedArray(HANGABLE_PUNCTS_CN_SIMPLIFIED.as_slice(), ch) { true } else { Self::isInSortedArray(HANGABLE_PUNCTS_CN.as_slice(), ch) });
      }
    }

    // java private static boolean isHangablePunctDefault(int lid, char ch, boolean useAltLineBreakRules)
    pub fn isHangablePunctDefault(lid: i32, ch: char, useAltLineBreakRules: bool) -> bool {
      // java return useAltLineBreakRules ? isInSortedArray(HANGABLE_PUNCTS_DEFAULT_ALT, ch) : isInSortedArray(HANGABLE_PUNCTS_DEFAULT, ch);
      return (if useAltLineBreakRules { Self::isInSortedArray(HANGABLE_PUNCTS_DEFAULT_ALT.as_slice(), ch) } else { Self::isInSortedArray(HANGABLE_PUNCTS_DEFAULT.as_slice(), ch) });
    }

    // java public static boolean isHangablePunct(char ch, Integer lid, boolean useAltLineBreakRules)
    pub fn isHangablePunct(ch: char, lid: Option<i32>, useAltLineBreakRules: bool) -> bool {
      // java int languageID = null == lid ? 0 : lid;
      let mut languageID: i32 = lid.unwrap_or(0);
      // java switch (languageID)
      match languageID {
        // java case 1028: case 2052: case 3076: case 4100: case 5124:
        1028 | 2052 | 3076 | 4100 | 5124 => {
          // java return isHangablePunctChinese(languageID, ch, useAltLineBreakRules);
          return Self::isHangablePunctChinese(languageID, ch, useAltLineBreakRules);
        }
        // java case 1041:
        1041 => {
          // java return isInSortedArray(HANGABLE_PUNCTS_JP, ch);
          return Self::isInSortedArray(HANGABLE_PUNCTS_JP.as_slice(), ch);
        }
        // java case 1042:
        1042 => {
          // java return isInSortedArray(HANGABLE_PUNCTS_KR, ch);
          return Self::isInSortedArray(HANGABLE_PUNCTS_KR.as_slice(), ch);
        }
        // java default:
        _ => {
          // java return isHangablePunctDefault(languageID, ch, useAltLineBreakRules);
          return Self::isHangablePunctDefault(languageID, ch, useAltLineBreakRules);
        }
      }
    }

}
