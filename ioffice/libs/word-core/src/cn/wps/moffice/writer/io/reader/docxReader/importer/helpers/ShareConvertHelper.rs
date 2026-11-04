// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.base.io.MetricUtility
use util::base::io::MetricUtility::*;
// Java: import cn.wps.base.io.TypeConversion
use util::base::io::TypeConversion::*;
// Java: import cn.wps.base.io.TypeConversion
use util::base::io::TypeConversion::TypeConversion;
// Java: import cn.wps.io.dom.io.WMap
use io_wps_io::cn::wps::io::dom::io::WMap::*;
// Java: import java.text.DateFormat
// Java: import java.text.ParseException
// Java: import java.text.SimpleDateFormat
// Java: import java.util.Date
// Java: import java.util.TimeZone
// Java: import java.util.regex.Matcher
// Java: import java.util.regex.Pattern
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::{Attributes, ConvertHelper};

// java private static final String VAL = "val";
const VAL: &str = "val";
// java private static final Pattern PATTERN = Pattern.compile("([+-][0-9][0-9]):?([0-9][0-9])$");
static PATTERN: &str = "([+-][0-9][0-9]):?([0-9][0-9])$";

// java public class ShareConvertHelper
#[derive(Default)]
pub struct ShareConvertHelper {
}

// java public class ShareConvertHelper
impl ShareConvertHelper {
    // java public static Integer convertSTLongHexNumber(Attributes attributes)
    pub fn convertSTLongHexNumber(attributes: &dyn Attributes) -> Option<i32> {
      // java return convertSTLongHexNumber(attributes, "val");
      return Self::convertSTLongHexNumber__attributes_string(attributes, "val");
    }
    // java public static Integer convertSTLongHexNumber(Attributes attributes, String name)
    pub fn convertSTLongHexNumber__attributes_string(attributes: &dyn Attributes, name: &str) -> Option<i32> {
      // java String str = ConvertHelper.getString(attributes, name);
      let str: Option<String> = ConvertHelper::getString__String(attributes, name);
      // java return null == str ? null : TypeConversion.hexString2Integer(str);
      return match str {
          None => None,
          Some(v) => TypeConversion::hexString2Integer(&v),
      };
    }
    // java public static Long getLong4STLongHexNumber(Attributes attributes, String name)
    pub fn getLong4STLongHexNumber(attributes: &dyn Attributes, name: &str) -> Option<i64> {
      // java String str = ConvertHelper.getString(attributes, name);
      let str: Option<String> = ConvertHelper::getString__String(attributes, name);
      // java return null == str ? null : TypeConversion.hexString2Long(str);
      return match str {
          None => None,
          Some(v) => TypeConversion::hexString2Long(&v),
      };
    }
    // java public static Float getFloat4STTwipsMeasure(Attributes attributes, String name)
    pub fn getFloat4STTwipsMeasure(attributes: &dyn Attributes, name: &str) -> Option<f32> {
      // java Long longValue = getLong4STTwipsMeasure(attributes, name);
      let longValue: Option<i64> = Self::getLong4STTwipsMeasure__attributes_string(attributes, name);
      // java return null == longValue ? null : longValue.floatValue();
      return longValue.map(|v| v as f32);
    }
    // java public static Long getLong4STTwipsMeasure(Attributes attributes)
    pub fn getLong4STTwipsMeasure(attributes: &dyn Attributes) -> Option<i64> {
      // java return getLong4STTwipsMeasure(attributes, "val");
      return Self::getLong4STTwipsMeasure__attributes_string(attributes, "val");
    }

    // java public static Long getLong4STTwipsMeasure(Attributes attributes, String name)
    pub fn getLong4STTwipsMeasure__attributes_string(attributes: &dyn Attributes, name: &str) -> Option<i64> {
      // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
      let str: Option<String> = attributes.getValue(WMap::MAIN_NAMESPACING(), name);
      // java if (null == str)
      if str.is_none() {
        // java return null;
        return None;
      // java else
      } else {
        // java Long longValue = getLong4STUnsignedDecimalNumber(str);
        let longValue: Option<i64> = Self::getLong4STUnsignedDecimalNumber(str.as_ref().unwrap());
        // java return null != longValue ? longValue : getLong4STPositiveUniversalMeasure(str);
        return match longValue {
            Some(v) => Some(v),
            None => Self::getLong4STPositiveUniversalMeasure(str.as_ref().unwrap()),
        };
      }
    }
    // java public static Long getLong4STSignedTwipsMeasure(Attributes attributes)
    pub fn getLong4STSignedTwipsMeasure(attributes: &dyn Attributes) -> Option<i64> {
      // java return getLong4STSignedTwipsMeasure(attributes, "val");
      return Self::getLong4STSignedTwipsMeasure__attributes_string(attributes, "val");
    }
    // java public static Long getLong4STSignedTwipsMeasure(Attributes attributes, String name)
    pub fn getLong4STSignedTwipsMeasure__attributes_string(attributes: &dyn Attributes, name: &str) -> Option<i64> {
      // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
      let str: Option<String> = attributes.getValue(WMap::MAIN_NAMESPACING(), name);
      // java if (null == str)
      if str.is_none() {
        // java return null;
        return None;
      // java else
      } else {
        // java Integer intValue = TypeConversion.string2Integer(str);
        let intValue: Option<i32> = TypeConversion::string2Integer(str.as_ref().unwrap());
        // java return null != intValue ? intValue.longValue() : getLong4STUniversalMeasure(str);
        return match intValue {
            Some(v) => Some(v as i64),
            None => Self::getLong4STUniversalMeasure__String(str.as_ref().unwrap()),
        };
      }
    }
    // java public static Integer getInteger4STDecimalNumberOrPercent(Attributes attributes)
    pub fn getInteger4STDecimalNumberOrPercent(attributes: &dyn Attributes) -> Option<i32> {
      // java return getInteger4STDecimalNumberOrPercent(attributes, "val");
      return Self::getInteger4STDecimalNumberOrPercent__attributes_string(attributes, "val");
    }

    // java public static Integer getInteger4STDecimalNumberOrPercent(Attributes attributes, String name)
    pub fn getInteger4STDecimalNumberOrPercent__attributes_string(attributes: &dyn Attributes, name: &str) -> Option<i32> {
      // java String str = ConvertHelper.getString(attributes, name);
      let str: Option<String> = ConvertHelper::getString__String(attributes, name);
      // java if (null == str)
      if str.is_none() {
        // java return null;
        return None;
      // java else
      } else {
        // java Integer intValue = TypeConversion.string2Integer(str);
        let intValue: Option<i32> = TypeConversion::string2Integer(str.as_ref().unwrap());
        // java if (null != intValue)
        if intValue.is_some() {
            // java return intValue;
            return intValue;
        // java else
        } else {
            // java Float perValue = getFloat4STPercentage(str);
            let perValue: Option<f32> = Self::getFloat4STPercentage(str.as_ref().unwrap());
            // java return null == perValue ? null : perValue.intValue();
            return match perValue {
                None => None,
                Some(f) => Some(f as i32),
            };
        }
    }

    }
    // java public static Integer getInteger4STShortHexNumber(Attributes attributes)
    pub fn getInteger4STShortHexNumber(attributes: &dyn Attributes) -> Option<i32> {
      // java String str = ConvertHelper.getString(attributes);
      let str: Option<String> = ConvertHelper::getString(attributes);
      // java return null == str ? null : TypeConversion.hexString2Integer(str);
      return match str {
          None => None,
          Some(v) => TypeConversion::hexString2Integer(&v),
      };
    }
    // java public static Long getLong4STPixelsMeasure(Attributes attributes, String name)
    pub fn getLong4STPixelsMeasure(attributes: &dyn Attributes, name: &str) -> Option<i64> {
      // java String str = ConvertHelper.getString(attributes, name);
      let str: Option<String> = ConvertHelper::getString__String(attributes, name);
      // java return null == str ? null : TypeConversion.string2Long(str);
      return match str {
          None => None,
          Some(v) => TypeConversion::string2Long(&v),
      };
    }
    // java public static Short getShort4STUcharHexNumber(Attributes attributes)
    pub fn getShort4STUcharHexNumber(attributes: &dyn Attributes) -> Option<i16> {
      // java String str = ConvertHelper.getString(attributes);
      let str: Option<String> = ConvertHelper::getString(attributes);
      // java if (null == str)
      if str.is_none() {
        // java return null;
        return None;
      // java else
      } else {
        // java Integer val = TypeConversion.hexString2Integer(str);
        let val: Option<i32> = TypeConversion::hexString2Integer(str.as_ref().unwrap());
        // java return null == val ? null : val.shortValue();
        return val.map(|v| v as i16);
      }
    }
    // java public static Long getLong4STUnsignedDecimalNumber(String str)
    pub fn getLong4STUnsignedDecimalNumber(str: &str) -> Option<i64> {
      // java return TypeConversion.string2Long(str);
      return TypeConversion::string2Long(str);
    }

    // java public static Long getLong4STPositiveUniversalMeasure(String str)
    pub fn getLong4STPositiveUniversalMeasure(str: &str) -> Option<i64> {
      // java return getLong4STUniversalMeasure(str);
      return Self::getLong4STUniversalMeasure__String(str);
    }

    // java public static Float getFloat4STTextScale(Attributes attributes)
    pub fn getFloat4STTextScale(attributes: &dyn Attributes) -> Option<f32> {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      // java String str = ConvertHelper.getString(attributes);
      let str: Option<String> = ConvertHelper::getString(attributes);
      // java if (null == str)
      if str.is_none() {
        // java return null;
        return None;
      // java else
      } else {
        // java Float val = TypeConversion.string2Float(str);
        let val: Option<f32> = TypeConversion::string2Float(str.as_ref().unwrap());
        // java return null != val ? val : getFloat4STPercentage(str);
        return match val {
            Some(v) => Some(v),
            None => Self::getFloat4STPercentage(str.as_ref().unwrap()),
        };
      }
    }
    // java public static Float getFloat4STPercentage(String str)
    pub fn getFloat4STPercentage(str: &str) -> Option<f32> {
      // java String longStr = getDigitString4Percentage(str);
      let longStr: Option<String> = Self::getDigitString4Percentage(Some(str));
      // java return null == longStr ? null : TypeConversion.string2Float(longStr);
      return match longStr {
          None => None,
          Some(s) => TypeConversion::string2Float(&s),
      };
    }

    // java public static Long getLong4STUniversalMeasure(Attributes attributes, String name)
    pub fn getLong4STUniversalMeasure__attributes_string(attributes: &dyn Attributes, name: &str) -> Option<i64> {
      // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
      let str: Option<String> = attributes.getValue(WMap::MAIN_NAMESPACING(), name);
      // java return null == str ? null : getLong4STUniversalMeasure(str);
      return match str {
          None => None,
          Some(v) => Self::getLong4STUniversalMeasure__String(&v),
      };
    }
    // java public static Long getLong4STDecimalNumber(Attributes attributes)
    pub fn getLong4STDecimalNumber(attributes: &dyn Attributes) -> Option<i64> {
      // java return getLong4STDecimalNumber(attributes, "val");
      return Self::getLong4STDecimalNumber__attributes_string(attributes, "val");
    }

    // java public static Long getLong4STDecimalNumber(Attributes attributes, String name)
    pub fn getLong4STDecimalNumber__attributes_string(attributes: &dyn Attributes, name: &str) -> Option<i64> {
      // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
      let str = attributes.getValue(WMap::MAIN_NAMESPACING(), name);
      // java return null == str ? null : TypeConversion.string2Long(str);
      return match str {
        None => None,
        Some(s) => TypeConversion::string2Long(&s),
      };
    }

    // java public static Integer getInteger4STDecimalNumber(Attributes attributes)
    pub fn getInteger4STDecimalNumber(attributes: &dyn Attributes) -> Option<i32> {
      // java return getInteger4STDecimalNumber(attributes, "val");
      return Self::getInteger4STDecimalNumber__attributes_string(attributes, "val");
    }

    // java public static Integer getInteger4STDecimalNumber(Attributes attributes, String name)
    pub fn getInteger4STDecimalNumber__attributes_string(attributes: &dyn Attributes, name: &str) -> Option<i32> {
      // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
      let str: Option<String> = attributes.getValue(WMap::MAIN_NAMESPACING(), name);
      // java return null == str ? null : TypeConversion.string2Integer(str);
      return match str {
          None => None,
          Some(s) => TypeConversion::string2Integer(&s),
      };
      let null = ();
    }

    // java private static Long getLong4STUniversalMeasure(String str)
    pub fn getLong4STUniversalMeasure__String(str: &str) -> Option<i64> {
      // java String longStr = getDigitString(str);
      let longStr: Option<String> = Self::getDigitString(Some(str));
      // java Float floatValue = null == longStr ? null : TypeConversion.string2Float(longStr);
      let floatValue: Option<f32> = match longStr {
          None => None,
          Some(v) => TypeConversion::string2Float(&v),
      };
      // java if (null == floatValue)
      if floatValue.is_none() {
        // java return null;
        return None;
      // java else
      } else {
        // java String measStr = getMeasureString(str);
        let measStr: Option<String> = Self::getMeasureString(Some(str));
        // java if (null == measStr)
        if measStr.is_none() {
            // java return floatValue.longValue();
            return Some(floatValue.unwrap() as i64);
        // java else
        } else {
            let fv = floatValue.unwrap();
            let m = measStr.as_ref().unwrap().to_lowercase();
            // java String MM = "mm";
            let MM = "mm"; let _ = MM;
            // java String CM = "cm";
            let CM = "cm"; let _ = CM;
            // java String IN = "in";
            let IN = "in"; let _ = IN;
            // java String PT = "pt";
            let PT = "pt"; let _ = PT;
            // java String PC = "pc";
            let PC = "pc"; let _ = PC;
            // java String PI = "pi";
            let PI = "pi"; let _ = PI;
            // java if (measStr.equalsIgnoreCase("cm"))
            if m == "cm" {
                // java return (long)MetricUtility.CM2Twips(floatValue);
                return Some(MetricUtility::CM2Twips(fv) as i64);
            // java } else if (measStr.equalsIgnoreCase("in"))
            } else if m == "in" {
                // java return (long)MetricUtility.Inches2Twips(floatValue);
                return Some(MetricUtility::Inches2Twips(fv) as i64);
            // java } else if (measStr.equalsIgnoreCase("mm"))
            } else if m == "mm" {
                // java return (long)MetricUtility.MM2Twips(floatValue);
                return Some(MetricUtility::MM2Twips(fv) as i64);
            // java } else if (measStr.equalsIgnoreCase("pc"))
            } else if m == "pc" {
                // java return (long)MetricUtility.PC2Twips(floatValue);
                return Some(MetricUtility::PC2Twips(fv) as i64);
            // java } else if (measStr.equalsIgnoreCase("pi"))
            } else if m == "pi" {
                // java return (long)MetricUtility.PC2Twips(floatValue);
                return Some(MetricUtility::PC2Twips(fv) as i64);
            // java } else
            } else {
                // java return measStr.equalsIgnoreCase("pt") ? (long)MetricUtility.Points2Twips(floatValue) : floatValue.longValue();
                return Some(if m == "pt" {
                    MetricUtility::Points2Twips_f32(fv) as i64
                } else {
                    fv as i64
                });
            }
      }
    }
    }
    // java public static String getDigitString(String str)
    pub fn getDigitString(str: Option<&str>) -> Option<String> {
      // java if (str != null && str.length() > 0)
      if str.is_some() && str.unwrap().len() > 0 {
        // java int i = findLastDigitChar(str);
        let i: i32 = Self::findLastDigitChar(str.unwrap());
        // java return i < 0 ? null : str.substring(0, i + 1);
        return if i < 0 {
            None
        } else {
            Some(str.unwrap()[..(i + 1) as usize].to_string())
        };
      // java else
      } else {
        // java return null;
        return None;
      }
    }
    // java public static String getDigitString4Percentage(String str)
    pub fn getDigitString4Percentage(str: Option<&str>) -> Option<String> {
      // java if (null == str)
      if str.is_none() {
        // java return null;
        return None;
      // java else
      } else {
        // java int len = str.length();
        let len = str.unwrap().len();
        // java if (0 == len)
        if 0 == len {
            // java return null;
            return None;
        // java else
        } else {
            // java char percentageChar = '%';
            let percentageChar = '%';
            let _ = percentageChar;
            // java char endChar = str.charAt(len - 1);
            let endChar = str.unwrap().chars().last();
            // java return endChar != '%' ? null : str.substring(0, len - 1);
            return if endChar != Some('%') {
                None
            } else {
                Some(str.unwrap()[..len - 1].to_string())
            };
        }
    }

    }
    // java private static int findLastDigitChar(String str)
    pub fn findLastDigitChar(str: &str) -> i32 {
      // java str = str.trim();
      let str: &str = str.trim();
      // java for(int i = str.length() - 1; i >= 0; --i)
      let mut i: i32 = str.len() as i32 - 1;
      while i >= 0 {
        // java if (Character.isDigit(str.charAt(i)))
        if str.as_bytes()[i as usize].is_ascii_digit() {
            // java return i;
            return i;
        }
        // java --i
        i -= 1;
      }
      // java return -1;
      return -1;
    }
    // java private static String getMeasureString(String str)
    pub fn getMeasureString(str: Option<&str>) -> Option<String> {
      // java if (str != null && str.length() > 0)
      if str.is_some() && str.unwrap().len() > 0 {
        // java str = str.trim();
        let str: &str = str.unwrap().trim();
        // java int index = str.length() - 2;
        let index: i32 = str.len() as i32 - 2;
        // java return index < 0 ? null : str.substring(index);
        return if index < 0 {
            None
        } else {
            Some(str[index as usize..].to_string())
        };
      // java else
      } else {
        // java return null;
        return None;
      }
    }
    // java public static Date convertDate(String dateTimeStr)
    pub fn convertDate(dateTimeStr: Option<&str>) -> Option<i64> {
      todo!("DEFERRED_NON_FUNCTION");
      // java if (null != dateTimeStr && dateTimeStr.length() > 0)
      let _ = "if";
          // java Matcher mat = PATTERN.matcher(dateTimeStr);
      let mat = ();
          // java TimeZone zoneID = null;
      let zoneID = ();
          // java if (mat.find())
      let _ = "if";
            // java IOAssert.assertTrue("2 == mat.end() should not be false!", 2 == mat.end());
      let _ = "IOAssert";
            // java String timeZoneCode = "GMT" + mat.group(1) + mat.group(2);
      let timeZoneCode = ();
            // java zoneID = TimeZone.getTimeZone(timeZoneCode);
      let zoneID = ();
          // java DateFormat formatter = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss");
      let formatter = ();
          // java if (zoneID != null)
      let _ = "if";
            // java formatter.setTimeZone(zoneID);
      let _ = "formatter";
          // java else
      let _ = "else";
            // java formatter.setTimeZone(TimeZone.getDefault());
      let _ = "formatter";
              // java return formatter.parse(dateTimeStr);
      let _ = "return";
              // java ParseException var5
      let _ = "ParseException";
                // java return null;
      let _ = "return";
      // java else
      let _ = "else";
        // java return null;
      let _ = "return";
    }

    // java public static Float getFloatSTMeasurementOrPercent(Attributes attributes, String name)
    pub fn getFloatSTMeasurementOrPercent(attributes: &dyn Attributes, name: &str) -> Option<f32> {
      // java Float floatValue = getFloatSTDecimalNumberOrPercent(attributes, name);
      let floatValue: Option<f32> = Self::getFloatSTDecimalNumberOrPercent(attributes, name);
      // java if (null != floatValue)
      if floatValue.is_some() {
        // java return floatValue;
        return floatValue;
      // java else
      } else {
        // java Long longValue = getLong4STUniversalMeasure(attributes, name);
        let longValue: Option<i64> = Self::getLong4STUniversalMeasure__attributes_string(attributes, name);
        // java return null == longValue ? null : longValue.floatValue();
        return longValue.map(|v| v as f32);
      }
    }
    // java private static Float getFloatSTDecimalNumberOrPercent(Attributes attributes, String name)
    pub fn getFloatSTDecimalNumberOrPercent(attributes: &dyn Attributes, name: &str) -> Option<f32> {
      // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
      let str: Option<String> = attributes.getValue(WMap::MAIN_NAMESPACING(), name);
      // java if (null == str)
      if str.is_none() {
        // java return null;
        return None;
      // java else
      } else {
        // java Float floatValue = ConvertHelper.getFloat(str);
        let floatValue: Option<f32> = ConvertHelper::getFloat__String(str.as_ref().unwrap());
        // java if (null != floatValue)
        if floatValue.is_some() {
            // java return floatValue;
            return floatValue;
        // java else
        } else {
            // java Float perValue = getFloat4STPercentage(str);
            let perValue: Option<f32> = Self::getFloat4STPercentage(str.as_ref().unwrap());
            // java return null == perValue ? null : perValue * 50.0F;
            return perValue.map(|v| v * 50.0);
        }
    }
    }
    // java public static Long getLong4STHpsMeasure(Attributes attributes)
    pub fn getLong4STHpsMeasure(attributes: &dyn Attributes) -> Option<i64> {
      // java String str = attributes.getValue(WMap.MAIN_NAMESPACING, "val");
      let str = attributes.getValue(WMap::MAIN_NAMESPACING(), "val");
      // java if (null == str)
      if str.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        let str = str.unwrap();
        // java Long longValue = getLong4STUnsignedDecimalNumber(str);
        let longValue = Self::getLong4STUnsignedDecimalNumber(&str);
        // java return null != longValue ? longValue : getLong4STPositiveUniversalMeasure(str);
        return if longValue.is_some() { longValue } else { Self::getLong4STPositiveUniversalMeasure(&str) };
      }
}
}
