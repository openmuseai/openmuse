// Java: import cn.wps.kfc.numfmt.IWordFormatter
use kfc::cn::wps::kfc::numfmt::IWordFormatter::*;
// Java: import cn.wps.kfc.numfmt.NumFmtFactory
use kfc::cn::wps::kfc::numfmt::NumFmtFactory::*;
// Java: import cn.wps.moffice.writer.data.field.exception.NumberCannotBeRepresentedInSpecifiedFormat
use crate::cn::wps::moffice::writer::data::field::exception::NumberCannotBeRepresentedInSpecifiedFormat::*;
// Java: import cn.wps.moffice.writer.data.field.exception.UnKnownSwitchAgrument
use crate::cn::wps::moffice::writer::data::field::exception::UnKnownSwitchAgrument::*;
// Java: import cn.wps.moffice.writer.layout.numformat.CodePage
use crate::cn::wps::moffice::writer::layout::numformat::CodePage::*;
// Java: import cn.wps.moffice.writer.layout.numformat.NFIntDefine
use crate::cn::wps::moffice::writer::layout::numformat::NFIntDefine::*;
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Iterator
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use kfc::cn::wps::kfc::numfmt::usermodel::WordFormatter::WordFormatter;
use kfc::cn::wps::kfc::numfmt::IWordFormatter::IWordFormatter;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::LazyLock;

// java private static final int UPPER_TO_LOWER_OFFSET = 32;
const UPPER_TO_LOWER_OFFSET: i32 = 32;
// java private static final int SBC_TO_DBC_OFFSET = 65248;
const SBC_TO_DBC_OFFSET: i32 = 65248;
// java private static final int SBC_START = 33;
const SBC_START: i32 = 33;
// java private static final int SBC_END = 126;
const SBC_END: i32 = 126;
// java private static final int DBC_START = 65281;
const DBC_START: i32 = 65281;
// java private static final int DBC_END = 65374;
const DBC_END: i32 = 65374;
// java private static final char DBC_SPACE = '　';
const DBC_SPACE: char = '　';
// java private static final char SBC_SPACE = ' ';
const SBC_SPACE: char = ' ';
// java private static int length = 0;
static length: AtomicI32 = AtomicI32::new(0);
// java private static IWordFormatter mWordFormatter = NumFmtFactory.createWordFormatter((short)0);
static mWordFormatter: LazyLock<WordFormatter> =
                    LazyLock::new(|| NumFmtFactory::createWordFormatter(0));
// java private static final int MAX_FILED_RESULT_LEN = 30;
const MAX_FILED_RESULT_LEN: i32 = 30;
// java protected static final int[][] fmtPairs;
pub(crate) static fmtPairs: &[[i32; 2]] = &[
    [0, 0], [1, 4], [2, 5], [3, 6], [4, 7], [5, 8], [6, 9], [7, 10], [8, 11],
    [9, NFC_ChineseLwrGen], [10, NFC_ChineseLwrGen], [11, 24], [12, 21], [13, 22], [14, 3], [15, 0],
    [16, 23], [17, 25], [18, 16], [19, 3], [20, 21], [21, 22], [22, NFC_ArabiaAdd0], [23, 1024],
    [24, NFC_ChineseLwrGen], [25, NFC_ChineseLwrGen], [26, 14], [27, 15], [28, 16], [29, 17],
    [30, 12], [31, 13], [32, NFC_ChineseLwrGen], [33, 27], [34, 26], [35, 1], [36, NFC_ChineseLwrGen],
    [37, 1], [38, NFC_ChinesUprGen], [39, 1], [40, 0], [41, NFC_ChineseLwrGen], [42, NFC_ChineseLwrGen],
    [43, NFC_ChineseLwrGen], [44, NFC_ChineseLwrGen], [45, NFC_ChineseLwrGen], [46, 0],
    [47, NFC_ChineseLwrGen], [48, 0], [49, NFC_ChineseLwrGen], [50, NFC_ChineseLwrGen],
    [51, NFC_ChineseLwrGen], [52, NFC_ChineseLwrGen], [53, 28], [54, 29], [55, 30],
    [56, NFC_ChineseLwrGen], [57, 49152], [58, NFC_ChineseLwrGen], [59, NFC_ChineseLwrGen], [60, 61439],
];
// java private static final String[] mapNfc;
static mapNfc: &[&str] = &[
    "ARABIC", "ROMAN", "roman", "ALPHABETIC", "alphabetic", "ORDINAL", "CARDTEXT", "ORDTEXT", "HEX",
    "ARABIC", "DBNUM1", "DBNUM2", "AIUEO", "IROHA", "DBCHAR", "SBCHAR", "DBNUM3", "DBNUM4", "CIRCLENUM",
    "ARABIC", "AIUEO", "IROHA", "ARABIC", "ARABIC", "GANADA", "CHOSUNG", "GB1", "GB2", "GB3", "GB4",
    "ZODIAC1", "ZODIAC2", "ZODIAC3", "CHINESENUM1", "CHINESENUM2", "CHINESENUM3", "CHINESENUM4",
    "CHINESENUM1", "CHINESENUM2", "DBNUM1", "CHINESENUM4", "DBNUM1", "DBNUM2", "DBNUM3", "DBNUM4",
    "HEBREW1", "ARABICALPHA", "HINDIARABIC", "ARABICABJAD", "HINDILETTER1", "HINDILETTER2",
    "HINDIARABIC", "HINDICARDTEXT", "THAILETTER", "THAIARABIC", "THAICARDTEXT", "VIETCARDTEXT",
    "ARABICDASH", "ARABIC", "ARABIC", "",
];

// java public class StrFormatUtil
#[derive(Default)]
pub struct StrFormatUtil {
}

// java public class StrFormatUtil
impl StrFormatUtil {
    // java public static String renderByFormat(FieldCodeLex mCodeLex, Object value)
    pub fn renderByFormat(mCodeLex: Option<&FieldCodeLex>, value: Option<&dyn Object>) -> Option<String> {
      // java if (mCodeLex != null && value != null)
      if let (Some(mCodeLex), Some(value)) = (mCodeLex, value) {
        // java int formatCount = mCodeLex.getFormatCount();
        let mut formatCount: i32 = mCodeLex.getFormatCount();
        // java if (formatCount > 0)
        if formatCount > 0 {
          // java HashMap<String, Boolean> formatMap = mCodeLex.getFormats();
          let mut formatMap: &HashMap<String, bool> = mCodeLex.getFormats();
          // java Iterator<String> iter = formatMap.keySet().iterator();
          let mut iter = formatMap.keys();
          // java String format = (String)iter.next();
          let mut format: String = iter.next().cloned().unwrap_or_default();
          // java boolean strFormat = (Boolean)formatMap.get(format);
          let mut strFormat: bool = *formatMap.get(&format).unwrap_or(&false);
          // java if (strFormat)
          if strFormat {
            // java if (value instanceof char[])
            if value.as_any().downcast_ref::<Vec<char>>().is_some() {
              // java char[] newValue = (char[])value;
              let mut newValue: Vec<char> = value.as_any().downcast_ref::<Vec<char>>().cloned().unwrap_or_default();
              // java return strFormatTrans(newValue, format);
              return Self::strFormatTrans(Some(&newValue), Some(&format));
            }
          }
          // java else if (value instanceof Long)
          else if value.as_any().downcast_ref::<i64>().is_some() {
            // java int nfc = txNfcString2MsoNfc(format);
            let mut nfc: i32 = Self::txNfcString2MsoNfc(&format);
            // java Long newValue = (Long)value;
            let mut newValue: i64 = *value.as_any().downcast_ref::<i64>().unwrap();
            // java String rs = mWordFormatter.formatGeneral(newValue, nfc);
            let mut rs: String = mWordFormatter.formatGeneral(&newValue, nfc);
            // java if (null != rs && rs.length() > 30)
            if (rs.len() as i32) > 30 {
              // java throw new NumberCannotBeRepresentedInSpecifiedFormat();
              panic!("NumberCannotBeRepresentedInSpecifiedFormat");
            }
            // java return rs;
            return Some(rs);
          }
        }
        // java else
        else {
          // java if (value instanceof char[])
          if value.as_any().downcast_ref::<Vec<char>>().is_some() {
            // java char[] temValue = (char[])value;
            let mut temValue: Vec<char> = value.as_any().downcast_ref::<Vec<char>>().cloned().unwrap_or_default();
            // java return new String(temValue);
            return Some(temValue.iter().collect::<String>());
          }
          // java if (value instanceof Long)
          if value.as_any().downcast_ref::<i64>().is_some() {
            // java Long temValue = (Long)value;
            let mut temValue: i64 = *value.as_any().downcast_ref::<i64>().unwrap();
            // java return String.valueOf(temValue);
            return Some(temValue.to_string());
          }
        }
        // java return value.toString();
        return Some(value.toString());
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public static String strFormatTrans(char[] str, String format)
    pub fn strFormatTrans(str: Option<&Vec<char>>, format: Option<&str>) -> Option<String> {
      // java if (str == null)
      if str.is_none() {
        // java return null;
        return None;
      }
      // java else if (str.length != 0 && format != null)
      else if (str.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) != 0 && format.is_some() {
          // java format = format.toLowerCase();
          let format = format.unwrap().to_lowercase();
          // java length = str.length;
          length.store(str.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32, Ordering::SeqCst);
          // java if ("upper".equals(format))
          if "upper" == format {
            // java return (new String(str)).toUpperCase();
            return Some(str.unwrap().iter().collect::<String>().to_uppercase());
          }
          // java else if ("lower".equals(format))
          else if "lower" == format {
            // java return (new String(str)).toLowerCase();
            return Some(str.unwrap().iter().collect::<String>().to_lowercase());
          }
          // java else if ("firstcap".equals(format))
          else if "firstcap" == format {
            // java return transToFirstCap(str);
            let mut buf = str.unwrap().clone();
            return Some(Self::transToFirstCap(&mut buf));
          }
          // java else if ("caps".equals(format))
          else if "caps" == format {
            // java return transToCaps(str);
            let mut buf = str.unwrap().clone();
            return Some(Self::transToCaps(&mut buf));
          }
          // java else if ("sbchar".equals(format))
          else if "sbchar" == format {
            // java return transToSBChar(str);
            let mut buf = str.unwrap().clone();
            return Some(Self::transToSBChar(&mut buf));
          }
          // java else if ("dbchar".equals(format))
          else if "dbchar" == format {
            // java return transToDBChar(str);
            let mut buf = str.unwrap().clone();
            return Some(Self::transToDBChar(&mut buf));
          }
          // java else
          else {
            // java throw new UnKnownSwitchAgrument();
            panic!("UnKnownSwitchAgrument");
          }
      }
      // java else
      else {
        // java return new String(str);
        return Some(str.unwrap().iter().collect::<String>());
      }
    }

    // java private static String transToFirstCap(char[] str)
    pub fn transToFirstCap(str: &mut Vec<char>) -> String {
      let mut i: i32 = 0;
      // java for(int i = 0; i < length; ++i)
      while i < length.load(Ordering::SeqCst) {
        // java if (str[i] >= 'a' && str[i] <= 'z')
        if str[(i) as usize] >= 'a' && str[(i) as usize] <= 'z' {
          // java str[i] = (char)(str[i] - 32);
          str[(i) as usize] = char::from_u32(str[(i) as usize] as u32 - 32).unwrap_or(str[(i) as usize]);
          // java break;
          break;
        }
        i += 1;
      }
      // java return new String(str);
      return str.iter().collect::<String>();
    }

    // java private static String transToCaps(char[] str)
    pub fn transToCaps(str: &mut Vec<char>) -> String {
      // java int i = 0;
      let mut i: i32 = 0;
      // java while (i < length)
      while i < length.load(Ordering::SeqCst) {
        // java if (str[i] >= 'a' && str[i] <= 'z')
        if str[(i) as usize] >= 'a' && str[(i) as usize] <= 'z' {
          // java for(str[i] = (char)(str[i] - 32) i < length && str[i] != ' ' ++i)
          str[(i) as usize] = char::from_u32(str[(i) as usize] as u32 - 32).unwrap_or(str[(i) as usize]);
          while i < length.load(Ordering::SeqCst) && str[(i) as usize] != ' ' {
            i += 1;
          }
        }
        // java else
        else {
          // java ++i;
          { let _tmp = i; i += 1; _tmp };
        }
      }
      // java return new String(str);
      return str.iter().collect::<String>();
    }

    // java private static String transToSBChar(char[] str)
    pub fn transToSBChar(str: &mut Vec<char>) -> String {
      let mut i: i32 = 0;
      // java for(int i = 0; i < length; ++i)
      while i < length.load(Ordering::SeqCst) {
        // java if (str[i] == 12288)
        if str[(i) as usize] as u32 == 12288 {
          // java str[i] = ' ';
          str[(i) as usize] = ' ';
        }
        // java else if (str[i] >= '！' && str[i] <= '～')
        else if str[(i) as usize] >= '！' && str[(i) as usize] <= '～' {
          // java str[i] -= 'ﻠ';
          str[(i) as usize] = char::from_u32((str[(i) as usize] as u32).wrapping_sub(SBC_TO_DBC_OFFSET as u32)).unwrap_or(str[(i) as usize]);
        }
        i += 1;
      }
      // java return new String(str);
      return str.iter().collect::<String>();
    }

    // java private static String transToDBChar(char[] str)
    pub fn transToDBChar(str: &mut Vec<char>) -> String {
      let mut i: i32 = 0;
      // java for(int i = 0; i < length; ++i)
      while i < length.load(Ordering::SeqCst) {
        // java if (str[i] == ' ')
        if str[(i) as usize] == ' ' {
          // java str[i] = 12288;
          str[(i) as usize] = char::from_u32(12288).unwrap_or('　');
        }
        // java else if (str[i] >= '!' && str[i] <= '~')
        else if str[(i) as usize] >= '!' && str[(i) as usize] <= '~' {
          // java str[i] += 'ﻠ';
          str[(i) as usize] = char::from_u32((str[(i) as usize] as u32).wrapping_add(SBC_TO_DBC_OFFSET as u32)).unwrap_or(str[(i) as usize]);
        }
        i += 1;
      }
      // java return new String(str);
      return str.iter().collect::<String>();
    }

    // java private static int msoNfc2KsoNfc(int msoNfc)
    pub fn msoNfc2KsoNfc(msoNfc: i32) -> i32 {
      // java if (255 == msoNfc)
      if 255 == msoNfc {
        // java return 61439;
        return 61439;
      }
      // java else
      else {
        // java int i = 0;
        let mut i: i32 = 0;
        // java int fmt = 0;
        let mut fmt: i32 = 0;
        let mut var3: i32 = 0;
        // java for(int var3 = 0; var3 < fmtPairs.length ++var3)
        while var3 < (fmtPairs.len() as i32) {
          // java if (fmtPairs[var3][0] == msoNfc)
          if fmtPairs[(var3) as usize][0usize] == msoNfc {
            // java fmt = fmtPairs[var3][1];
            fmt = fmtPairs[(var3) as usize][1usize];
            // java break;
            break;
          }
          var3 += 1;
        }
        // java return fmt;
        return fmt;
      }
    }

    // java public static boolean txFormatNum(int nVal, int uMsoNfc, StringBuffer strResult)
    pub fn txFormatNum__int_int_stringbuffer(nVal: i32, uMsoNfc: i32, strResult: Option<&mut StringBuffer>) -> bool {
      // java if (strResult == null)
      if strResult.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        let strResult = strResult.unwrap();
        // java strResult.setLength(0);
        strResult.setLength(0);
        // java if (23 != uMsoNfc)
        if 23 != uMsoNfc {
          // java int ksonfc = msoNfc2KsoNfc(uMsoNfc);
          let mut ksonfc: i32 = Self::msoNfc2KsoNfc(uMsoNfc);
          // java CodePage.getInstance().serialNumber((long)nVal, ksonfc, strResult);
          CodePage::getInstance().serialNumber(((nVal) as i64), ksonfc, strResult);
          // java if (ksonfc == 49152)
          if ksonfc == 49152 {
            // java strResult.insert(0, "- ");
            strResult.insert_str(0, "- ");
            // java strResult.append(" -");
            strResult.append_str(" -");
          }
          // java return true;
          return true;
        }
        // java else
        else {
          // java return false;
          return false;
        }
      }
    }

    // java public static String txFormatNum(int nVal, int uMsoNfc)
    pub fn txFormatNum__int_int(nVal: i32, uMsoNfc: i32) -> String {
      // java StringBuffer strResult = new StringBuffer();
      let mut strResult: StringBuffer = StringBuffer::new();
      // java txFormatNum(nVal, uMsoNfc, strResult);
      Self::txFormatNum__int_int_stringbuffer(nVal, uMsoNfc, Some(&mut strResult));
      // java return strResult.toString();
      return strResult.toString();
    }

    // java public static String numformat_2(String numStr, String format)
    pub fn numformat_2(numStr: &str, format: &str) -> String {
      // java return mWordFormatter.formatNumeric(Double.parseDouble(numStr), format);
      return mWordFormatter.formatNumeric(numStr.parse::<f64>().unwrap_or(0.0), format);
    }

    // java public static int txNfcString2MsoNfc(String string)
    pub fn txNfcString2MsoNfc(string: &str) -> i32 {
      // java if (string.compareTo(mapNfc[2]) == 0)
      if string == mapNfc[2usize] {
        // java return 2;
        return 2;
      }
      // java else if (string.compareTo(mapNfc[4]) == 0)
      else if string == mapNfc[4usize] {
        // java return 4;
        return 4;
      }
      // java else
      else {
        // java String upperStr = string.toUpperCase();
        let mut upperStr: String = string.to_uppercase();
        // java int i = 0;
        let mut i: i32 = 0;
        let mut len: i32 = (mapNfc.len() as i32);
        // java for(int len = mapNfc.length; i < len; ++i)
        while i < len {
          // java if (upperStr.compareTo(mapNfc[i]) == 0)
          if upperStr == mapNfc[(i) as usize] {
            // java return i;
            return i;
          }
          i += 1;
        }
        // java return -1;
        return -1;
      }
    }
}
