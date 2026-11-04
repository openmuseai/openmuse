use super::CodePage::CodePage;
use super::NFIntDefine::{NFC_ArabiaAdd0, NFC_ChinesUprGen, NFC_ChineseLwrGen};
use stubs::java::lang::StringBuffer::StringBuffer;

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
    "ArabicDash", "ARABIC", "ARABIC", "",
];

// java public class Numformat
#[derive(Default)]
pub struct Numformat {
}

// java public class Numformat
impl Numformat {
    // java private static int msoNfc2KsoNfc(int msoNfc)
    pub fn msoNfc2KsoNfc(msoNfc: i32) -> i32 {
      // java if (255 == msoNfc) return 61439;
      if 255 == msoNfc {
        return 61439;
      }
      let mut fmt: i32 = 0;
      let mut var3: i32 = 0;
      // java for (int var3 = 0; var3 < fmtPairs.length; ++var3)
      while var3 < (fmtPairs.len() as i32) {
        if fmtPairs[var3 as usize][0] == msoNfc {
          fmt = fmtPairs[var3 as usize][1];
          break;
        }
        var3 += 1;
      }
      fmt
    }

    // java public static boolean txFormatNum(int nVal, int uMsoNfc, StringBuffer strResult)
    pub fn txFormatNum__int_int_stringbuffer(nVal: i32, uMsoNfc: i32, strResult: Option<&mut StringBuffer>) -> bool {
      // java if (strResult == null) return false;
      let Some(strResult) = strResult else { return false };
      strResult.setLength(0);
      // java if (23 != uMsoNfc)
      if 23 != uMsoNfc {
        let ksonfc = Self::msoNfc2KsoNfc(uMsoNfc);
        CodePage::getInstance().serialNumber(nVal as i64, ksonfc, strResult);
        // java if (ksonfc == 49152) wrap with "- " / " -"
        if ksonfc == 49152 {
          strResult.insert_str(0, "- ");
          strResult.append_str(" -");
        }
        return true;
      }
      false
    }

    // java public static String txFormatNum(int nVal, int uMsoNfc)
    pub fn txFormatNum__int_int(nVal: i32, uMsoNfc: i32) -> String {
      let mut strResult = StringBuffer::new();
      Self::txFormatNum__int_int_stringbuffer(nVal, uMsoNfc, Some(&mut strResult));
      strResult.toString()
    }

    // java public static String numformat_2(String numStr, String format)
    pub fn numformat_2(numStr: Option<String>, format: Option<String>) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (numStr != null && numStr.length() != 0 && format != null && format.length() != 0)
      if let (Some(numStr), Some(format)) = (numStr, format) {
        if numStr.length() != 0 && format.length() != 0 {
          // java char[] strArray = numStr.toCharArray();
          let mut strArray: Vec<char> = numStr.toCharArray();
          // java char[] formatArray = format.toCharArray();
          let mut formatArray: Vec<char> = format.toCharArray();
          // java int temLength = formatArray.length + 32;
          let mut temLength: i32 = (formatArray.len() as i32) + 32;
          // java char[] temArray = new char[temLength];
          let mut temArray: Vec<char> = vec![Default::default(); (temLength) as usize];
          // java boolean hasComma = false;
          let mut hasComma: bool = false;
          // java int index_semi = -1;
          let mut index_semi: i32 = -1;
          // java int firstZeroIndex_format = -1;
          let mut firstZeroIndex_format: i32 = -1;
          // java int pointIndexFormat = -1;
          let mut pointIndexFormat: i32 = -1;
          // java int pointIndexNumStr = -1;
          let mut pointIndexNumStr: i32 = -1;
          // java int pointLengthInNumStr = 0;
          let mut pointLengthInNumStr: i32 = 0;
          // java int count = 0;
          let mut count: i32 = 0;
          // java int currentIndex_str = strArray.length - 1;
          let mut currentIndex_str: i32 = (strArray.len() as i32) - 1;
          // java int currentIndex_tem = temArray.length - 1;
          let mut currentIndex_tem: i32 = (temArray.len() as i32) - 1;
          // java int formatLength = formatArray.length;
          let mut formatLength: i32 = (formatArray.len() as i32);
          let mut i: i32 = 0;
          // java for(int i = 0; i < formatLength; ++i)
          while i < formatLength {
            // java if (';' == formatArray[i])
            if ';' == formatArray[(i) as usize] {
              // java index_semi = i;
              index_semi = i;
              // java break;
              break;
            }
          }
          // java if (index_semi != -1)
          if index_semi != -1 {
            // java formatLength = index_semi;
            formatLength = index_semi;
          }
          // java if (formatLength == 0)
          if formatLength == 0 {
            // java return "";
            return Some("");
          }
          // java else
          else {
            let mut i: i32 = 1;
            // java for(int i = 1; i < formatLength - 1; ++i)
            while i < formatLength - 1 {
              // java if (formatArray[i] == '.' && (formatArray[i - 1] == '0' || formatArray[i + 1] == '0'))
              if formatArray[(i) as usize] == '.' && formatArray[(i - 1) as usize] == '0' || formatArray[(i + 1) as usize] == '0' {
                // java pointIndexFormat = i;
                pointIndexFormat = i;
                // java break;
                break;
              }
            }
            let mut i: i32 = 1;
            // java for(int i = 1; i < strArray.length; ++i)
            while i < (strArray.len() as i32) {
              // java if ('.' == strArray[i])
              if '.' == strArray[(i) as usize] {
                // java pointIndexNumStr = i;
                pointIndexNumStr = i;
                // java break;
                break;
              }
            }
            // java if (pointIndexNumStr > 0)
            if pointIndexNumStr > 0 {
              // java pointLengthInNumStr = strArray.length - pointIndexNumStr - 1;
              pointLengthInNumStr = (strArray.len() as i32) - pointIndexNumStr - 1;
            }
            let mut i: i32 = 1;
            // java for(int i = 1; i < formatLength; ++i)
            while i < formatLength {
              // java if (formatArray[i] == ',')
              if formatArray[(i) as usize] == ',' {
                // java hasComma = true;
                hasComma = true;
                // java break;
                break;
              }
            }
            let mut i: i32 = 0;
            // java for(int i = 0; i < formatLength; ++i)
            while i < formatLength {
              // java if (formatArray[i] == '0')
              if formatArray[(i) as usize] == '0' {
                // java firstZeroIndex_format = i;
                firstZeroIndex_format = i;
                // java break;
                break;
              }
            }
            // java if (firstZeroIndex_format < 0)
            if firstZeroIndex_format < 0 {
              // java return numStr;
              return numStr;
            }
            // java else
            else {
              // java if (pointIndexFormat == -1 && pointIndexNumStr != -1)
              if pointIndexFormat == -1 && pointIndexNumStr != -1 {
                // java currentIndex_str = pointIndexNumStr - 1;
                currentIndex_str = pointIndexNumStr - 1;
              }
              let mut i: i32 = formatLength - 1;
              // java for(int i = formatLength - 1; i >= 0 --i)
              while i >= 0 {
                // java if (formatArray[i] != '0' && formatArray[i] != '#' && formatArray[i] != ',' && formatArray[i] != '.')
                if formatArray[(i) as usize] != '0' && formatArray[(i) as usize] != '#' && formatArray[(i) as usize] != ',' && formatArray[(i) as usize] != '.' {
                  // java temArray[currentIndex_tem--] = formatArray[i];
                  temArray[({ let _tmp = currentIndex_tem; currentIndex_tem -= 1; _tmp }) as usize] = formatArray[(i) as usize];
                }
                // java else if (formatArray[i] == '#' && i > firstZeroIndex_format)
                else if formatArray[(i) as usize] == '#' && i > firstZeroIndex_format {
                  // java temArray[currentIndex_tem--] = ' ';
                  temArray[({ let _tmp = currentIndex_tem; currentIndex_tem -= 1; _tmp }) as usize] = ' ';
                }
                // java else if (formatArray[i] == '.')
                else if formatArray[(i) as usize] == '.' {
                  // java if (pointIndexNumStr == -1)
                  if pointIndexNumStr == -1 {
                    // java temArray[currentIndex_tem--] = formatArray[i];
                    temArray[({ let _tmp = currentIndex_tem; currentIndex_tem -= 1; _tmp }) as usize] = formatArray[(i) as usize];
                  }
                }
                // java else if (pointIndexFormat != -1 && i >= pointIndexFormat)
                else if pointIndexFormat != -1 && i >= pointIndexFormat {
                  // java int length_format = i - pointIndexFormat;
                  let mut length_format: i32 = i - pointIndexFormat;
                  // java if (length_format > pointLengthInNumStr)
                  if length_format > pointLengthInNumStr {
                    // java temArray[currentIndex_tem--] = formatArray[i];
                    temArray[({ let _tmp = currentIndex_tem; currentIndex_tem -= 1; _tmp }) as usize] = formatArray[(i) as usize];
                  }
                  // java else
                  else {
                    // java currentIndex_str = pointIndexNumStr + length_format;
                    currentIndex_str = pointIndexNumStr + length_format;
                    // java temArray[currentIndex_tem--] = strArray[currentIndex_str--];
                    temArray[({ let _tmp = currentIndex_tem; currentIndex_tem -= 1; _tmp }) as usize] = strArray[({ let _tmp = currentIndex_str; currentIndex_str -= 1; _tmp }) as usize];
                  }
                }
                // java else if (i > firstZeroIndex_format)
                else if i > firstZeroIndex_format {
                  // java temArray[currentIndex_tem--] = strArray[currentIndex_str--];
                  temArray[({ let _tmp = currentIndex_tem; currentIndex_tem -= 1; _tmp }) as usize] = strArray[({ let _tmp = currentIndex_str; currentIndex_str -= 1; _tmp }) as usize];
                  // java if (hasComma)
                  if hasComma {
                    // java ++count;
                    { let _tmp = count; count += 1; _tmp };
                    // java if (count % 3 == 0)
                    if count % 3 == 0 {
                      // java temArray[currentIndex_tem--] = ',';
                      temArray[({ let _tmp = currentIndex_tem; currentIndex_tem -= 1; _tmp }) as usize] = ',';
                      // java count = 0;
                      count = 0;
                    }
                  }
                }
                // java else
                else {
                  // java while (currentIndex_str >= 0)
                  while currentIndex_str >= 0 {
                    // java temArray[currentIndex_tem--] = strArray[currentIndex_str--];
                    temArray[({ let _tmp = currentIndex_tem; currentIndex_tem -= 1; _tmp }) as usize] = strArray[({ let _tmp = currentIndex_str; currentIndex_str -= 1; _tmp }) as usize];
                    // java if (hasComma)
                    if hasComma {
                      // java ++count;
                      { let _tmp = count; count += 1; _tmp };
                      // java if (count % 3 == 0)
                      if count % 3 == 0 {
                        // java temArray[currentIndex_tem--] = ',';
                        temArray[({ let _tmp = currentIndex_tem; currentIndex_tem -= 1; _tmp }) as usize] = ',';
                        // java count = 0;
                        count = 0;
                      }
                    }
                  }
                }
              }
              // java return new String(temArray, currentIndex_tem + 1, temArray.length - currentIndex_tem - 1);
              return { let __x: Option<String> = todo!("no align strategy matched"); __x };
            }
          }
        }
      }
      // java else
      else {
        // java return numStr;
        return numStr;
      }
      */
    }

    // java public static int txNfcString2MsoNfc(String string)
    pub fn txNfcString2MsoNfc(string: Option<String>) -> i32 {
      // java if (string.compareTo(mapNfc[2]) == 0) return 2; else if mapNfc[4]; else upper compare
      let Some(string) = string else { return -1 };
      if string == mapNfc[2] {
        return 2;
      }
      if string == mapNfc[4] {
        return 4;
      }
      let upper_str = string.to_uppercase();
      let mut i: i32 = 0;
      let len = mapNfc.len() as i32;
      while i < len {
        if upper_str == mapNfc[i as usize] {
          return i;
        }
        i += 1;
      }
      -1
      /* DEFERRED_NON_FUNCTION
      // java if (string.compareTo(mapNfc[2]) == 0)
      if string.compareTo__object(mapNfc[2usize]) == 0 {
        // java return 2;
        return 2;
      }
      // java else if (string.compareTo(mapNfc[4]) == 0)
      else if string.compareTo(mapNfc[4usize]) == 0 {
        // java return 4;
        return 4;
      }
      // java else
      else {
        // java String upperStr = string.toUpperCase();
        let mut upperStr: String = string.toUpperCase();
        // java int i = 0;
        let mut i: i32 = 0;
        let mut len: i32 = (mapNfc.len() as i32);
        // java for(int len = mapNfc.length; i < len; ++i)
        while i < len {
          // java if (upperStr.compareTo(mapNfc[i]) == 0)
          if upperStr.compareTo(mapNfc[(i) as usize]) == 0 {
            // java return i;
            return i;
          }
        }
        // java return -1;
        return -1;
      }
      */
    }

}
