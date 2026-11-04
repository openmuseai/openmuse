// Java field: String LOCALE_FILE_PREFIX
pub(crate) const LOCALE_FILE_PREFIX: &str = "locale_";
// Java field: String BUILT_IN_FMT_PREFIX
pub(crate) const BUILT_IN_FMT_PREFIX: &str = "builtinfmt_";
// Java field: String TOKEN_FILE_PREFIX
pub(crate) const TOKEN_FILE_PREFIX: &str = "token_";
// Java field: String NUMERAL_SHAPE_FILE
pub(crate) const NUMERAL_SHAPE_FILE: &str = "numeral_shape";
// Java field: String ATTR_STR_ARRAY
pub(crate) const ATTR_STR_ARRAY: &str = "string-array";
// Java field: String ATTR_STR_ITEM
pub(crate) const ATTR_STR_ITEM: &str = "string";
// Java field: String ATTR_TYPE
pub(crate) const ATTR_TYPE: &str = "type";
// Java field: String ATTR_SIZE
pub(crate) const ATTR_SIZE: &str = "size";
// Java field: String NODE_ARRAY_ITEM
pub(crate) const NODE_ARRAY_ITEM: &str = "item";
// Java field: String LN_LONG_WEEK
pub(crate) const LN_LONG_WEEK: &str = "long-week";
// Java field: String LN_SHORT_WEEK
pub(crate) const LN_SHORT_WEEK: &str = "short-week";
// Java field: String LN_LONG_MONTH
pub(crate) const LN_LONG_MONTH: &str = "long-month";
// Java field: String LN_SHORT_MONTH
pub(crate) const LN_SHORT_MONTH: &str = "short-month";
// Java field: String LN_SHORTEST_MONTH
pub(crate) const LN_SHORTEST_MONTH: &str = "shortest-month";
// Java field: String LN_AM_PM
pub(crate) const LN_AM_PM: &str = "am-pm";
// Java field: String LN_THOUS_SEP
pub(crate) const LN_THOUS_SEP: &str = "thous-separator";
// Java field: String LN_DECIMAL_POINT
pub(crate) const LN_DECIMAL_POINT: &str = "decimal-point";
// Java field: String LN_COLOR_NAME
pub(crate) const LN_COLOR_NAME: &str = "color-name";
// Java field: String LN_GENERAL
pub(crate) const LN_GENERAL: &str = "general";
// Java field: String LN_YEAR
pub(crate) const LN_YEAR: &str = "year";
// Java field: String LN_MONTH
pub(crate) const LN_MONTH: &str = "month";
// Java field: String LN_DAY
pub(crate) const LN_DAY: &str = "day";
// Java field: String LN_HOUR
pub(crate) const LN_HOUR: &str = "hour";
// Java field: String LN_MINUTE
pub(crate) const LN_MINUTE: &str = "minute";
// Java field: String LN_SECOND
pub(crate) const LN_SECOND: &str = "second";
// Java field: String LN_ESCAPE
pub(crate) const LN_ESCAPE: &str = "escape";
// Java field: String LN_SLASH
pub(crate) const LN_SLASH: &str = "date-slash";
// Java field: String LN_F400_F800
pub(crate) const LN_F400_F800: &str = "f400-f800";
// Java field: String BN_BUILT_IN_FMT
pub(crate) const BN_BUILT_IN_FMT: &str = "builtin-formats";
// Java field: String NN_NUM_SHAPE_PREFIX
pub(crate) const NN_NUM_SHAPE_PREFIX: &str = "shape-";
// Java field: String NN_NUM_SHAPE_UNIT_PREFIX
pub(crate) const NN_NUM_SHAPE_UNIT_PREFIX: &str = "shape-unit-";
// Java field: String NT_TOKEN_DECIMAL
pub(crate) const NT_TOKEN_DECIMAL: &str = "token-decimal";
// Java field: String NT_TOKEN_THOUS
pub(crate) const NT_TOKEN_THOUS: &str = "token-thous";
// Java field: String NT_TOKEN_GENERAL
pub(crate) const NT_TOKEN_GENERAL: &str = "token-general";
// Java field: String NT_TOKEN_BRACKET
pub(crate) const NT_TOKEN_BRACKET: &str = "token-bracket";
// Java field: String NT_TOKEN_YEAR
pub(crate) const NT_TOKEN_YEAR: &str = "token-year";
// Java field: String NT_TOKEN_MONTH
pub(crate) const NT_TOKEN_MONTH: &str = "token-month";
// Java field: String NT_TOKEN_DAY
pub(crate) const NT_TOKEN_DAY: &str = "token-day";
// Java field: String NT_TOKEN_HOUR
pub(crate) const NT_TOKEN_HOUR: &str = "token-hour";
// Java field: String NT_TOKEN_MIN
pub(crate) const NT_TOKEN_MIN: &str = "token-min";
// Java field: String NT_TOKEN_SEC
pub(crate) const NT_TOKEN_SEC: &str = "token-sec";
// Java field: String NT_TOKEN_A
pub(crate) const NT_TOKEN_A: &str = "token-a";
// Java field: String NT_TOKEN_ESCAPE_EX
pub(crate) const NT_TOKEN_ESCAPE_EX: &str = "token-escape-ex";

// Java: public class ResourceDef
pub struct ResourceDef {
}

// Java: public class ResourceDef
impl ResourceDef {
    // Java: ResourceDef()
    pub fn new() -> Self {
      // Java: super();
      ResourceDef {}
    }

    // Java: static String getNumShapeName(byte numeralShape)
    pub fn getNumShapeName(numeral_shape: i8) -> String {
      // Java: return name2Str(numeralShape, "shape-");
      Self::name2Str_byte(numeral_shape, "shape-")
    }

    // Java: static String getNumShapeUnitName(byte numeralShape)
    pub fn getNumShapeUnitName(numeral_shape: i8) -> String {
      // Java: return name2Str(numeralShape, "shape-unit-");
      Self::name2Str_byte(numeral_shape, "shape-unit-")
    }

    // Java: static String getLocaleResName(short lcid)
    pub fn getLocaleResName(lcid: i16) -> String {
      // Java: return name2Str(lcid, "locale_");
      Self::name2Str_short(lcid, "locale_")
    }

    // Java: static String getBuiltinFormatsName(short lcid)
    pub fn getBuiltinFormatsName(lcid: i16) -> String {
      // Java: return name2Str(lcid, "builtinfmt_");
      Self::name2Str_short(lcid, "builtinfmt_")
    }

    // Java: static String getTokenResName(short lcid)
    pub fn getTokenResName(lcid: i16) -> String {
      // Java: return name2Str(lcid, "token_");
      Self::name2Str_short(lcid, "token_")
    }

    // Java: static String name2Str(short name, String prefix)
    pub fn name2Str_short(name: i16, prefix: &str) -> String {
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = String::new();
      // Java: buffer.setLength(0);
      buffer.clear();
      // Java: buffer.append(prefix);
      buffer.push_str(prefix);
      // Java: shortToHexString(name, buffer);
      Self::shortToHexString(name, &mut buffer);
      // Java: return buffer.toString();
      buffer
    }

    // Java: static String name2Str(byte name, String prefix)
    pub fn name2Str_byte(name: i8, prefix: &str) -> String {
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = String::new();
      // Java: buffer.setLength(0);
      buffer.clear();
      // Java: buffer.append(prefix);
      buffer.push_str(prefix);
      // Java: byteToHexString(name, buffer);
      Self::byteToHexString(name as u8, &mut buffer);
      // Java: return buffer.toString();
      buffer
    }

    // Java: static void shortToHexString(short value, StringBuffer buffer)
    fn shortToHexString(value: i16, buffer: &mut String) {
      // Java: byteToHexString((byte)(value >> 8 & 255), buffer);
      Self::byteToHexString(((value >> 8) & 255) as u8, buffer);
      // Java: byteToHexString((byte)(value & 255), buffer);
      Self::byteToHexString((value & 255) as u8, buffer);
    }

    // Java: static void byteToHexString(byte b, StringBuffer buffer)
    fn byteToHexString(b: u8, buffer: &mut String) {
      // Java: buffer.append(toHexChar(b >>> 4 & 15));
      buffer.push(Self::toHexChar(((b >> 4) & 15) as i32));
      // Java: buffer.append(toHexChar(b & 15));
      buffer.push(Self::toHexChar((b & 15) as i32));
    }

    // Java: static char toHexChar(int b)
    fn toHexChar(b: i32) -> char {
      // Java: return 0 <= b && b < 16 ? (char)(b < 10 ? b + 48 : b + 65 - 10) : '\u0000';
      if 0 <= b && b < 16 {
          if b < 10 { (b + 48) as u8 as char } else { (b + 65 - 10) as u8 as char }
      } else {
          '\0'
      }
    }

}
