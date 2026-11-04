// Java: import cn.wps.base.assertion.Assert
// Java: import cn.wps.base.log.Log
// Java: import cn.wps.moffice.util.StringUtil
// Java: import org.apache.poi.openxml.xmlbeans.StringEnumAbstractBase
use std::collections::HashMap;
use std::sync::OnceLock;

// Java field: String TAG

// Java: public class CssColorConverter
pub struct CssColorConverter {
}

// Java: public class ColorType
pub struct ColorType {
    // Java field: String mStr
    pub(crate) mStr: String,
}

// Java: public class Enum
pub struct ColorEnum {
    pub(crate) str_val: String,
    pub(crate) integer: i32,
}

// Java field: int INT_BLACK
pub(crate) const INT_BLACK: i32 = 0;
// Java field: int INT_SILVER
pub(crate) const INT_SILVER: i32 = 12632256;
// Java field: int INT_GRAY
pub(crate) const INT_GRAY: i32 = 8421504;
// Java field: int INT_WHITE
pub(crate) const INT_WHITE: i32 = 16777215;
// Java field: int INT_MAROON
pub(crate) const INT_MAROON: i32 = 8388608;
// Java field: int INT_RED
pub(crate) const INT_RED: i32 = 16711680;
// Java field: int INT_PURPLE
pub(crate) const INT_PURPLE: i32 = 8388736;
// Java field: int INT_FUCHSIA
pub(crate) const INT_FUCHSIA: i32 = 16711935;
// Java field: int INT_GREEN
pub(crate) const INT_GREEN: i32 = 32768;
// Java field: int INT_LIME
pub(crate) const INT_LIME: i32 = 65280;
// Java field: int INT_OLIVE
pub(crate) const INT_OLIVE: i32 = 8421376;
// Java field: int INT_YELLOW
pub(crate) const INT_YELLOW: i32 = 16776960;
// Java field: int INT_NAVY
pub(crate) const INT_NAVY: i32 = 128;
// Java field: int INT_BLUE
pub(crate) const INT_BLUE: i32 = 255;
// Java field: int INT_TEAL
pub(crate) const INT_TEAL: i32 = 32896;
// Java field: int INT_AQUA
pub(crate) const INT_AQUA: i32 = 65535;

// Java field: Enum BLACK
// Java field: Enum SILVER
// Java field: Enum GRAY
// Java field: Enum WHITE
// Java field: Enum MAROON
// Java field: Enum RED
// Java field: Enum PURPLE
// Java field: Enum FUCHSIA
// Java field: Enum GREEN
// Java field: Enum LIME
// Java field: Enum OLIVE
// Java field: Enum YELLOW
// Java field: Enum NAVY
// Java field: Enum BLUE
// Java field: Enum TEAL
// Java field: Enum AQUA
// Java field: StringEnumAbstractBase.Table TABLE

fn color_table() -> &'static HashMap<String, i32> {
    static TABLE: OnceLock<HashMap<String, i32>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("black".to_string(), INT_BLACK);
        m.insert("silver".to_string(), INT_SILVER);
        m.insert("gray".to_string(), INT_GRAY);
        m.insert("white".to_string(), INT_WHITE);
        m.insert("maroon".to_string(), INT_MAROON);
        m.insert("red".to_string(), INT_RED);
        m.insert("purple".to_string(), INT_PURPLE);
        m.insert("fuchsia".to_string(), INT_FUCHSIA);
        m.insert("green".to_string(), INT_GREEN);
        m.insert("lime".to_string(), INT_LIME);
        m.insert("olive".to_string(), INT_OLIVE);
        m.insert("yellow".to_string(), INT_YELLOW);
        m.insert("navy".to_string(), INT_NAVY);
        m.insert("blue".to_string(), INT_BLUE);
        m.insert("teal".to_string(), INT_TEAL);
        m.insert("aqua".to_string(), INT_AQUA);
        m
    })
}

const WHITE_SPACES: &[char] = &[' ', '\t', '\n', '\r', '\x0C'];

// Java: public class CssColorConverter
impl CssColorConverter {
    // Java: CssColorConverter()
    fn new() -> Self {
        // Java: super();
        CssColorConverter {}
    }

    // Java: static Integer toCoreColor(String value)
    pub fn toCoreColor(value: &str) -> Option<i32> {
      // Java: Assert.assertNotNull("value should not be null", value);
      // Java: String trimStr = value.trim();
      let trim_str = value.trim();
      // Java: int i = 0;
      let mut i: usize;
      // Java: int len = trimStr.length();
      let len = trim_str.len();

      // Java: label78:
      let chars: Vec<char> = trim_str.chars().collect();
      i = len;
      // Java: for(i = 0; i < len; ++i)
      'label78: for idx in 0..len
      {
         // Java: for(char whiteSpace : StringUtil.WHITE_SPACES)
         for &ws in WHITE_SPACES
         {
            // Java: if (whiteSpace == trimStr.charAt(i))
            if ws == chars[idx]
            {
               // Java: break label78;
               i = idx;
               break 'label78;
            }
         }
      }

      // Java: String sharp = "#";
      // Java: boolean isHex = trimStr.startsWith("#");
      let is_hex = trim_str.starts_with('#');
      // Java: if (isHex)
      if is_hex
      {
         // Java: int start = "#".length();
         let start = 1;
         // Java: trimStr = trimStr.substring(start, i);
         let hex_part = &trim_str[start..i];
         // Java: trimStr = toLongCssHexColor(trimStr);
         let long_hex = Self::toLongCssHexColor(hex_part);
         // Java: Integer rgb = null;
         let mut rgb: Option<i32> = None;

         // Java: try
         {
            // Java: rgb = Integer.valueOf(trimStr, 16);
            rgb = i32::from_str_radix(&long_hex, 16).ok();
         }
         // Java: catch (NumberFormatException var11)
         {
         }
         // Java: return rgb;
         return rgb;
      }
      // Java: else
      {
         // Java: ColorType.Enum preDefined = (new ColorType(trimStr.substring(0, i))).toEnum();
         let ct = ColorType::new(&trim_str[..i]);
         let pre_defined = ct.toEnum();
         // Java: if (null != preDefined)
         if let Some(color_val) = pre_defined
         {
            // Java: return preDefined.intValue();
            return Some(color_val);
         }
         // Java: else
         {
            // Java: int special = 0;
            let mut special: i32 = 0;
            // Java: if (trimStr.startsWith("fill darken"))
            if trim_str.starts_with("fill darken")
            {
               // Java: special = 268435952;
               special = 268435952;
            }
            // Java: else if (trimStr.startsWith("fill lighten"))
            else if trim_str.starts_with("fill lighten")
            {
               // Java: special = 268436208;
               special = 268436208;
            }
            // Java: else
            else
            {
               // Java: if (trimStr.startsWith("fill"))
               if trim_str.starts_with("fill")
               {
                  // Java: return 285147888;
                  return Some(285147888);
               }

               // Java: if (trimStr.startsWith("shadow add"))
               if trim_str.starts_with("shadow add")
               {
                  // Java: special = 268436467;
                  special = 268436467;
               }
               // Java: else
               else
               {
                  // Java: if (!trimStr.startsWith("lineOrFill darken"))
                  if !trim_str.starts_with("lineOrFill darken")
                  {
                     // Java: if ("auto".equals(trimStr))
                     if trim_str == "auto"
                     {
                        // Java: return -16777216;
                        return Some(-16777216);
                     }

                     // Java: if ("windowtext".equals(trimStr))
                     if trim_str == "windowtext"
                     {
                        // Java: return -16777216;
                        return Some(-16777216);
                     }

                     // Java: Integer rgb = null;
                     let mut rgb2: Option<i32> = None;

                     // Java: try
                     {
                        // Java: rgb = Integer.valueOf(trimStr, 10);
                        rgb2 = trim_str.parse::<i32>().ok();
                     }
                     // Java: catch (NumberFormatException var12)
                     {
                     }
                     // Java: return rgb;
                     return rgb2;
                  }

                  // Java: special = 268435953;
                  special = 268435953;
               }
            }

            // Java: Integer percent = extractClosedNum(trimStr, '(', ')');
            let percent = Self::extractClosedNum(trim_str, '(', ')');
            // Java: if (null != percent)
            if let Some(p) = percent
            {
               // Java: special |= percent << 16;
               special |= p << 16;
            }

            // Java: return special;
            return Some(special);
         }
      }
    }

    // Java: static String toLongCssHexColor(String str)
    fn toLongCssHexColor(str_val: &str) -> String {
      // Java: Assert.assertNotNull("str should not be null!", str);
      // Java: int sizeLongCssHexColor = 6;
      // Java: int sizeStr = str.length();
      let size_str = str_val.len();
      // Java: if (6 == sizeStr)
      if 6 == size_str
      {
         // Java: return str;
         return str_val.to_string();
      }
      // Java: else
      {
         // Java: StringBuilder longCssHexColor = new StringBuilder(6);
         let mut long_css = String::with_capacity(6);
         // Java: for(int i = 0; i < sizeStr; ++i)
         for ch in str_val.chars()
         {
            // Java: char ch = str.charAt(i);
            // Java: longCssHexColor.append(ch);
            long_css.push(ch);
            // Java: longCssHexColor.append(ch);
            long_css.push(ch);
         }
         // Java: return longCssHexColor.toString();
         long_css
      }
    }

    // Java: static Integer extractClosedNum(String str, char left, char right)
    fn extractClosedNum(str_val: &str, left: char, right: char) -> Option<i32> {
      // Java: int iLeft = str.indexOf(left);
      let i_left = str_val.find(left);
      // Java: int iRight = str.indexOf(right);
      let i_right = str_val.find(right);
      // Java: if (-1 != iLeft && -1 != iRight)
      if let (Some(l), Some(r)) = (i_left, i_right)
      {
         // Java: try
         {
            // Java: return Integer.valueOf(str.substring(iLeft + 1, iRight));
            return str_val[l + 1..r].parse::<i32>().ok();
         }
         // Java: catch (NumberFormatException e)
         {
            // Java: Log.e(TAG, "NumberFormatException: ", e);
            // Java: return null;
            return None;
         }
      }
      // Java: else
      {
         // Java: return null;
         None
      }
    }
}

// Java: public class ColorType
impl ColorType {
    // Java: ColorType(String str)
    pub fn new(str_val: &str) -> Self {
         // Java: super();
         // Java: Assert.assertNotNull("str should not be null", str);
         // Java: this.mStr = str;
         ColorType { mStr: str_val.to_lowercase() }
    }

    // Java: Enum toEnum()
    pub fn toEnum(&self) -> Option<i32> {
         // Java: return (Enum)CssColorConverter.ColorType.Enum.TABLE.forString(this.mStr);
         color_table().get(&self.mStr).copied()
    }
}

// Java: public class Enum
impl ColorEnum {
    // Java: Enum(String s, int i)
    pub fn new(s: &str, i: i32) -> Self {
            // Java: super(s, i);
            ColorEnum { str_val: s.to_lowercase(), integer: i }
    }
}
