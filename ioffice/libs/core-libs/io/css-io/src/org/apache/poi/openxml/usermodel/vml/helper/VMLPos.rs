// Java: import cn.wps.base.io.TypeConversion
use util::base::io::TypeConversion::TypeConversion;

// Java field: char F_REF
pub(crate) const F_REF: char = '@';
// Java field: char ADJ_REF
pub(crate) const ADJ_REF: char = '#';
// Java field: char PLAIN
pub(crate) const PLAIN: char = '\0';

// Java: public class VMLPos
pub struct VMLPos {
    // Java field: char type
    pub(crate) r#type: char,
    // Java field: int val
    pub(crate) val: i32,
}

// Java: public class VMLPos
impl VMLPos {
    // Java: VMLPos(char type, int val)
    pub fn new(r#type: char, val: i32) -> Self {
      // Java: super();
      // Java: this.type = type;
      // Java: this.val = val;
      VMLPos { r#type, val }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return 0 != this.type ? this.type + String.valueOf(this.val) : String.valueOf(this.val);
      if self.r#type != '\0' {
          format!("{}{}", self.r#type, self.val)
      } else {
          self.val.to_string()
      }
    }

    // Java: static VMLPos valueOf(String str)
    pub fn valueOf(str: Option<&str>) -> VMLPos {
      // Java: if (null == str)
      if str.is_none()
      {
         // Java: throw invalidVmlPos(str);
         Self::invalidVmlPos(str.unwrap_or(""));
      }
      // Java: else
      {
         // Java: String orgStr = str;
         let org_str = str.unwrap();
         // Java: str = str.trim();
         let trimmed = org_str.trim();
         // Java: int len = str.length();
         let len = trimmed.len();
         // Java: if (len <= 0)
         if len <= 0
         {
            // Java: throw invalidVmlPos(orgStr);
            Self::invalidVmlPos(org_str);
         }
         // Java: else
         {
            // Java: int PREFIX_SYMBOL_INDEX = 0;
            // Java: char chFirst = str.charAt(0);
            let ch_first = trimmed.as_bytes()[0] as char;
            // Java: char posType = getType(chFirst);
            let pos_type = Self::getType(ch_first);
            // Java: String intStr = str;
            let mut int_str = trimmed;
            // Java: if (0 != posType)
            if pos_type != '\0'
            {
               // Java: int VAL_INDEX = 1;
               // Java: String var10 = str.substring(1, str.length());
               let var10 = &trimmed[1..];
               // Java: intStr = var10.trim();
               int_str = var10.trim();
               // Java: int intStrLen = intStr.length();
               let int_str_len = int_str.len();
               // Java: if (intStrLen <= 0)
               if int_str_len <= 0
               {
                  // Java: throw invalidVmlPos(orgStr);
                  Self::invalidVmlPos(org_str);
               }
            }
            // Java: Integer posVal = TypeConversion.string2Integer(intStr);
            let pos_val = TypeConversion::string2Integer(int_str);
            // Java: if (null == posVal)
            if pos_val.is_none()
            {
               // Java: throw invalidVmlPos(orgStr);
               Self::invalidVmlPos(org_str);
            }
            // Java: else
            {
               // Java: return new VMLPos(posType, posVal);
               return VMLPos::new(pos_type, pos_val.unwrap());
            }
         }
      }
    }

    // Java: static char getType(char ch)
    fn getType(ch: char) -> char {
      // Java: char type = '\u0000';
      // Java: switch (ch)
      match ch
      {
         // Java: case '#':
         '#' =>
         {
            // Java: type = '#';
            // Java: break;
            '#'
         }
         // Java: case '@':
         '@' =>
         {
            // Java: type = '@';
            // Java: break;
            '@'
         }
         // Java: default:
         _ =>
         {
            // Java: type = '\u0000';
            '\0'
         }
      }
      // Java: return type;
    }

    // Java: static NumberFormatException invalidVmlPos(String s)
    fn invalidVmlPos(s: &str) -> ! {
      // Java: throw new NumberFormatException("Invalid VMLPos: \"" + s + "\"");
      panic!("Invalid VMLPos: \"{}\"", s);
    }

}
