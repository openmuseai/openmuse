// Java field: int CHAR_FIRST_HIGH_SURROGATE
pub(crate) const CHAR_FIRST_HIGH_SURROGATE: i32 = 55296;
// Java field: int CHAR_LAST_LOW_SURROGATE
pub(crate) const CHAR_LAST_LOW_SURROGATE: i32 = 57343;
// Java field: int INVALID_WIDTH
pub(crate) const INVALID_WIDTH: i32 = -1;
// Java field: char unicode_separator
pub(crate) const unicode_separator: char = '&';
// Java field: char unicode_prefix
pub(crate) const unicode_prefix: char = 'u';
// Java field: char separator
pub(crate) const separator: char = ':';

// Java: public class Emoji
pub struct Emoji {
}

// Java: public class Emoji
impl Emoji {
    // Java: Emoji()
    pub fn new() -> Self {
      // Java: super();
      Emoji {}
    }

    // Java: boolean contains(int emoji)
    pub fn contains(&self, emoji: i32) -> bool {
      // Java: return emoji >= this.getMinimumAndroidPua() && emoji <= this.getMaximumAndroidPua();
      emoji >= self.getMinimumAndroidPua() && emoji <= self.getMaximumAndroidPua()
    }

    // Java: float measureWidth(int emoji)
    pub fn measureWidth(&self, _emoji: i32) -> f32 {
      // Java: return -1.0F;
      -1.0
    }

    // Java: int getMinimumAndroidPua()
    pub fn getMinimumAndroidPua(&self) -> i32 {
      // Java: return -1;
      -1
    }

    // Java: int getMaximumAndroidPua()
    pub fn getMaximumAndroidPua(&self) -> i32 {
      // Java: return -1;
      -1
    }

    // Java: static boolean emojiSurrogate(char c)
    pub fn emojiSurrogate(c: char) -> bool {
      // Java: return c >= '\ud800' && c <= '\udfff';
      let cv = c as u32;
      cv >= 0xd800 && cv <= 0xdfff
    }

    // Java: static boolean isEmojiChar(int codePoint)
    pub fn isEmojiChar(codePoint: i32) -> bool {
      // Java: return codePoint >= 9728 && codePoint <= 10175 || codePoint == 12349 || codePoint == 8265 || codePoint == 8252 || codePoint >= 8192 && codePoint <= 8207 || codePoint >= 8232 && codePoint <= 8239 || codePoint == 8287 || codePoint >= 8293 && codePoint <= 8303 || codePoint >= 8448 && codePoint <= 8527 || codePoint >= 8960 && codePoint <= 9215 || codePoint >= 11008 && codePoint <= 11263 || codePoint >= 10496 && codePoint <= 10623 || codePoint >= 12800 && codePoint <= 13055 || codePoint >= 55296 && codePoint <= 57343 || codePoint >= 57344 && codePoint <= 63743 || codePoint >= 65024 && codePoint <= 65039 || codePoint >= 65536;
      (codePoint >= 9728 && codePoint <= 10175)
        || codePoint == 12349
        || codePoint == 8265
        || codePoint == 8252
        || (codePoint >= 8192 && codePoint <= 8207)
        || (codePoint >= 8232 && codePoint <= 8239)
        || codePoint == 8287
        || (codePoint >= 8293 && codePoint <= 8303)
        || (codePoint >= 8448 && codePoint <= 8527)
        || (codePoint >= 8960 && codePoint <= 9215)
        || (codePoint >= 11008 && codePoint <= 11263)
        || (codePoint >= 10496 && codePoint <= 10623)
        || (codePoint >= 12800 && codePoint <= 13055)
        || (codePoint >= 55296 && codePoint <= 57343)
        || (codePoint >= 57344 && codePoint <= 63743)
        || (codePoint >= 65024 && codePoint <= 65039)
        || codePoint >= 65536
    }

    // Java: static String escape(String src)
    pub fn escape(src: Option<&str>) -> Option<String> {
      // Java: if (src == null)
      if src.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         let src = src.unwrap();
         // Java: int cpCount = src.codePointCount(0, src.length());
         let chars: Vec<char> = src.chars().collect();
         let cpCount = chars.len();
         // Java: int firCodeIndex = src.offsetByCodePoints(0, 0);
         let firCodeIndex: usize = 0;
         // Java: int lstCodeIndex = src.offsetByCodePoints(0, cpCount - 1);
         let lstCodeIndex: usize = if cpCount > 0 { cpCount - 1 } else { 0 };
         // Java: StringBuilder sb = new StringBuilder(src.length());
         let mut sb = String::with_capacity(src.len());
         // Java: for(int index = firCodeIndex; index <= lstCodeIndex; ++index)
         let mut index = firCodeIndex;
         while index <= lstCodeIndex
         {
            // Java: int codepoint = src.codePointAt(index);
            let codepoint = chars[index] as i32;
            // Java: if (isEmojiChar(codepoint))
            if Self::isEmojiChar(codepoint)
            {
               // Java: String hash = Integer.toHexString(codepoint);
               let hash = format!("{:x}", codepoint);
               // Java: sb.append('&').append(hash.length()).append('u').append(':').append(hash);
               sb.push('&');
               sb.push_str(&hash.len().to_string());
               sb.push('u');
               sb.push(':');
               sb.push_str(&hash);
            }
            // Java: else
            else
            {
               // Java: sb.append((char)codepoint);
               if let Some(c) = char::from_u32(codepoint as u32) {
                  sb.push(c);
               }
            }
            index += 1;
         }
         // Java: return sb.toString();
         Some(sb)
      }
    }

    // Java: static String reverse(String src)
    pub fn reverse(src: Option<&str>) -> Option<String> {
      // Java: if (src == null)
      if src.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         let src = src.unwrap();
         // Java: StringBuilder sb = new StringBuilder(src.length());
         let mut sb = String::with_capacity(src.len());
         // Java: char[] sourceChar = src.toCharArray();
         let sourceChar: Vec<char> = src.chars().collect();
         // Java: int index = 0;
         let mut index: usize = 0;
         // Java: while(index < sourceChar.length)
         while index < sourceChar.len()
         {
            // Java: if (sourceChar[index] == '&')
            if sourceChar[index] == '&'
            {
               // Java: if (index + 6 >= sourceChar.length)
               if index + 6 >= sourceChar.len()
               {
                  // Java: sb.append(sourceChar[index]);
                  sb.push(sourceChar[index]);
                  // Java: ++index;
                  index += 1;
               }
               // Java: else if (sourceChar[index + 1] >= '4' && sourceChar[index + 1] <= '6' && sourceChar[index + 2] == 'u' && sourceChar[index + 3] == ':')
               else if sourceChar[index + 1] >= '4' && sourceChar[index + 1] <= '6' && sourceChar[index + 2] == 'u' && sourceChar[index + 3] == ':'
               {
                  // Java: int length = Integer.parseInt(String.valueOf(sourceChar[index + 1]));
                  let length = sourceChar[index + 1].to_digit(10).unwrap_or(0) as usize;
                  // Java: char[] hexchars = new char[length];
                  let mut hexchars: Vec<char> = vec!['\0'; length];
                  // Java: for(int j = 0; j < length; ++j)
                  let mut valid = true;
                  for j in 0..length
                  {
                     // Java: char ch = sourceChar[index + 4 + j];
                     let ch = sourceChar[index + 4 + j];
                     // Java: if ((ch < '0' || ch > '9') && (ch < 'a' || ch > 'f'))
                     if (ch < '0' || ch > '9') && (ch < 'a' || ch > 'f')
                     {
                        // Java: sb.append(sourceChar[index]);
                        sb.push(sourceChar[index]);
                        // Java: ++index;
                        index += 1;
                        // Java: break;
                        valid = false;
                        break;
                     }
                     // Java: hexchars[j] = ch;
                     hexchars[j] = ch;
                  }
                  if valid {
                     // Java: sb.append(Character.toChars(Integer.parseInt(new String(hexchars), 16)));
                     let hex_str: String = hexchars.iter().collect();
                     if let Ok(cp) = i32::from_str_radix(&hex_str, 16) {
                        if let Some(c) = char::from_u32(cp as u32) {
                           sb.push(c);
                        }
                     }
                     // Java: index += 4 + length;
                     index += 4 + length;
                  }
               }
               // Java: else if (sourceChar[index + 1] != 'u')
               else if sourceChar[index + 1] != 'u'
               {
                  // Java: sb.append(sourceChar[index]);
                  sb.push(sourceChar[index]);
                  // Java: ++index;
                  index += 1;
               }
               // Java: else
               else
               {
                  // Java: char[] hexchars = new char[4];
                  let mut hexchars: Vec<char> = vec!['\0'; 4];
                  // Java: for(int j = 0; j < 4; ++j)
                  let mut valid = true;
                  for j in 0..4usize
                  {
                     // Java: char ch = sourceChar[index + 2 + j];
                     let ch = sourceChar[index + 2 + j];
                     // Java: if ((ch < '0' || ch > '9') && (ch < 'a' || ch > 'f'))
                     if (ch < '0' || ch > '9') && (ch < 'a' || ch > 'f')
                     {
                        // Java: sb.append(sourceChar[index]);
                        sb.push(sourceChar[index]);
                        // Java: ++index;
                        index += 1;
                        // Java: break;
                        valid = false;
                        break;
                     }
                     // Java: hexchars[j] = ch;
                     hexchars[j] = ch;
                     // Java: sb.append(Character.toChars(Integer.parseInt(String.valueOf(hexchars), 16)));
                     let hex_str: String = hexchars[..=j].iter().collect();
                     if let Ok(cp) = i32::from_str_radix(&hex_str, 16) {
                        if let Some(c) = char::from_u32(cp as u32) {
                           sb.push(c);
                        }
                     }
                     // Java: index += 6;
                     index += 6;
                  }
               }
            }
            // Java: else
            else
            {
               // Java: sb.append(sourceChar[index]);
               sb.push(sourceChar[index]);
               // Java: ++index;
               index += 1;
            }
         }
         // Java: return sb.toString();
         Some(sb)
      }
    }

    // Java: static String filter(String src)
    pub fn filter(src: Option<&str>) -> Option<String> {
      // Java: if (src == null)
      if src.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         let src = src.unwrap();
         // Java: int cpCount = src.codePointCount(0, src.length());
         let chars: Vec<char> = src.chars().collect();
         let cpCount = chars.len();
         // Java: int firCodeIndex = src.offsetByCodePoints(0, 0);
         let firCodeIndex: usize = 0;
         // Java: int lstCodeIndex = src.offsetByCodePoints(0, cpCount - 1);
         let lstCodeIndex: usize = if cpCount > 0 { cpCount - 1 } else { 0 };
         // Java: StringBuilder sb = new StringBuilder(src.length());
         let mut sb = String::with_capacity(src.len());
         // Java: int codepoint;
         let mut codepoint: i32;
         // Java: for(int index = firCodeIndex; index <= lstCodeIndex; index += Character.isSupplementaryCodePoint(codepoint) ? 2 : 1)
         let mut index = firCodeIndex;
         while index <= lstCodeIndex
         {
            // Java: codepoint = src.codePointAt(index);
            codepoint = chars[index] as i32;
            // Java: if (!isEmojiChar(codepoint))
            if !Self::isEmojiChar(codepoint)
            {
               // Java: System.err.println("codepoint:" + Integer.toHexString(codepoint));
               eprintln!("codepoint:{:x}", codepoint);
               // Java: sb.append((char)codepoint);
               if let Some(c) = char::from_u32(codepoint as u32) {
                  sb.push(c);
               }
            }
            index += if codepoint > 0xFFFF { 2 } else { 1 };
         }
         // Java: return sb.toString();
         Some(sb)
      }
    }

}
