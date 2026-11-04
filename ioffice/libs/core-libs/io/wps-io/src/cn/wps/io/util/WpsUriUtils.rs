// Java: import java.io.ByteArrayOutputStream
// Java: import java.net.URISyntaxException
// Java: import java.nio.charset.Charset
// Java: import java.util.Locale

// Java field: char[] hexCharTbl
const HEX_CHAR_TBL: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

// Java field: Charset CHARSET_UTF8
// (Rust uses UTF-8 natively)

// Java field: byte[] ASCII_FLAG_TABLE
pub(crate) static ASCII_FLAG_TABLE: [u8; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 4, 0, 0, 4, 0, 4, 4, 4, 4, 4, 4, 4, 3, 3, 1,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 4, 4, 0, 4, 0, 1,
    2, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 0, 1, 0, 3,
    0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 0, 0, 3, 0,
];

// Java: public class WpsUriUtils
pub struct WpsUriUtils {
}

// Java: public class WpsUriUtils
impl WpsUriUtils {
    // Java: WpsUriUtils()
    pub fn new() -> Self {
      // Java: super();
      WpsUriUtils {}
    }

    // Java: static int appendHexStrs(StringBuilder sb, String str)
    fn appendHexStrs(sb: &mut String, s: &str) -> i32 {
      // Java: int ret = sb.length();
      let ret = sb.len() as i32;
      // Java: byte[] bytes = str.getBytes(CHARSET_UTF8);
      let bytes = s.as_bytes();
      // Java: int i = 0;
      // Rust: Java 的下标 `i` 在 Rust 侧保留为同名绑定（下方 `while i < n` 与 Java 的
      //       `for(int n = bytes.length; i < n; ++i)` 逐句对齐；迭代顺序与 `for b in bytes` 等价）。
      let mut i = 0usize;
      // Java: for(int n = bytes.length; i < n; ++i)
      let n = bytes.len();
      while i < n
      {
         // Java: byte b = bytes[i];
         let b = bytes[i];
         // Java: sb.append('%').append(hexCharTbl[b >>> 4 & 15]).append(hexCharTbl[b & 15]);
         sb.push('%');
         sb.push(HEX_CHAR_TBL[(b as usize >> 4) & 0xF]);
         sb.push(HEX_CHAR_TBL[(b as usize) & 0xF]);
         i += 1;
      }
      // Java: return ret;
      ret
    }

    // Java: static char[] toASCIIString(String s)
    pub fn toASCIIString(s: &str) -> Vec<char> {
      // Java: if (s == null)
      // (Rust: s is always a valid reference)
      {
         // Java: throw new NullPointerException("s == null");
         // unreachable in Rust
      }
      // Java: else
      {
         // Java: StringBuilder sb = null;
         let mut sb: Option<String> = None;
         // Java: int len = s.length();
         let len = s.len();
         // Java: char[] chars = new char[len + 1];
         let mut chars: Vec<char> = s.chars().collect();
         chars.push('\0');
         // Java: s.getChars(0, len, chars, 0);
         // Java: int escapeStart = -1;
         let mut escapeStart: i32 = -1;
         // Java: int replacedChars = 0;
         let mut replacedChars: usize = 0;
         // Java: for(int i = 0; i < len; ++i)
         for i in 0..len
         {
            // Java: char c = chars[i];
            let c = chars[i];
            // Java: if (c > 127)
            if c as u32 > 127
            {
               // Java: if (escapeStart == -1)
               if escapeStart == -1
               {
                  // Java: escapeStart = i;
                  escapeStart = i as i32;
               }
            }
            // Java: else
            else
            {
               // Java: if (escapeStart != -1)
               if escapeStart != -1
               {
                  // Java: if (sb == null)
                  if sb.is_none()
                  {
                     // Java: sb = new StringBuilder("\u0000");
                     sb = Some(String::from("\0"));
                  }
                  // Java: chars[escapeStart] = (char)(-appendHexStrs(sb, s.substring(escapeStart, i)));
                  let hex_pos = Self::appendHexStrs(sb.as_mut().unwrap(), &s[escapeStart as usize..i]);
                  chars[escapeStart as usize] = char::from_u32((-hex_pos) as u32).unwrap_or('\0');
                  // Java: for(int j = escapeStart + 1; j < i; ++j)
                  for j in (escapeStart as usize + 1)..i
                  {
                     // Java: chars[j] = 0;
                     chars[j] = '\0';
                  }
                  // Java: replacedChars += i - escapeStart;
                  replacedChars += i - escapeStart as usize;
                  // Java: escapeStart = -1;
                  escapeStart = -1;
               }
               // Java: if (c == ' ')
               if c == ' '
               {
                  // Java: chars[i] = '+';
                  chars[i] = '+';
               }
            }
         }
         // Java: if (escapeStart != -1)
         if escapeStart != -1
         {
            // Java: if (sb == null)
            if sb.is_none()
            {
               // Java: sb = new StringBuilder("\u0000");
               sb = Some(String::from("\0"));
            }
            // Java: chars[escapeStart] = (char)(-appendHexStrs(sb, s.substring(escapeStart, len)));
            let hex_pos = Self::appendHexStrs(sb.as_mut().unwrap(), &s[escapeStart as usize..len]);
            chars[escapeStart as usize] = char::from_u32((-hex_pos) as u32).unwrap_or('\0');
            // Java: for(int j = escapeStart + 1; j < len; ++j)
            for j in (escapeStart as usize + 1)..len
            {
               // Java: chars[j] = 0;
               chars[j] = '\0';
            }
            // Java: replacedChars += len - escapeStart;
            replacedChars += len - escapeStart as usize;
         }
         // Java: if (sb == null)
         if sb.is_none()
         {
            // Java: return chars;
            return chars;
         }
         // Java: else
         else
         {
            // Java: int lastPos = sb.length();
            let sb_ref = sb.as_ref().unwrap();
            let mut lastPos = sb_ref.len();
            // Java: int j = len - replacedChars + lastPos - 1;
            let mut j = len - replacedChars + lastPos - 1;
            // Java: char[] ret = new char[j + 1];
            let mut ret = vec!['\0'; j + 1];
            // Java: for(int var12 = len - 1; var12 >= 0; --var12)
            let mut var12 = len as i32 - 1;
            while var12 >= 0
            {
               // Java: int ch = (short)chars[var12];
               let ch = chars[var12 as usize] as i16;
               // Java: if (ch > 0)
               if ch > 0
               {
                  // Java: --j;
                  j -= 1;
                  // Java: ret[j] = (char)ch;
                  ret[j] = ch as u8 as char;
               }
               // Java: else if (ch < 0)
               else if ch < 0
               {
                  // Java: j -= lastPos + ch;
                  let offset = lastPos as i32 + ch as i32;
                  j -= offset as usize;
                  // Java: sb.getChars(-ch, lastPos, ret, j);
                  let sb_chars: Vec<char> = sb_ref.chars().collect();
                  let start = (-ch) as usize;
                  for k in start..lastPos {
                      ret[j + k - start] = sb_chars[k];
                  }
                  // Java: lastPos = -ch;
                  lastPos = (-ch) as usize;
               }
               var12 -= 1;
            }
            // Java: return ret;
            return ret;
         }
      }
    }

    // Java: static String toASCIIString2(String s)
    pub fn toASCIIString2(s: &str) -> String {
      // Java: char[] chars = toASCIIString(s);
      let chars = Self::toASCIIString(s);
      // Java: return new String(chars, 0, chars.length - 1);
      chars[..chars.len() - 1].iter().collect()
    }

    // Java: static char[] toLowerCase(char[] chars)
    pub fn toLowerCase_chars(chars: &mut Vec<char>) {
      // Java: String defultLanguage = Locale.getDefault().getLanguage();
      // Java: if (!defultLanguage.equals(Locale.CHINA.getLanguage()) && !defultLanguage.equals(Locale.ENGLISH.getLanguage()))
      {
         /* DEFERRED_NON_FUNCTION：Java 的**非 CJK 语言**分支（`Locale` 判断）本移植未实现 —— 恒走 else 的
            ASCII 小写路径 ⇒ 该分支语句停车、保留原 Java 模板（末尾 `(skipped: ...)` 为原注记）。
         // Java: String strLowerCase = (new String(chars, 0, chars.length - 1)).toLowerCase();
         // Java: int len = strLowerCase.length();
         // Java: char[] ret = new char[len + 1];
         // Java: strLowerCase.getChars(0, len, ret, 0);
         // Java: return ret;
         // (skipped: in Rust we always use ASCII lowercase below)
         */
      }
      // Java: else
      {
         // Java: int i = 0;
         // Rust: 同 `appendHexStrs` —— 保留 Java 下标名 `i`，与 `for(int n = chars.length; i < n; ++i)` 对齐。
         let mut i = 0usize;
         // Java: for(int n = chars.length; i < n; ++i)
         let n = chars.len();
         while i < n
         {
            // Java: int c = chars[i];
            let c = chars[i];
            // Java: if (c >= 65 && c <= 90)
            if c >= 'A' && c <= 'Z'
            {
               // Java: chars[i] = (char)(c + 97 - 65);
               chars[i] = (c as u8 + 97 - 65) as char;
            }
            i += 1;
         }
         // Java: return chars;
      }
    }

    // Java: static char[] toLowerCase(char[] chars, int start, int end, Locale locale)
    pub fn toLowerCase_range(chars: &mut Vec<char>, start: usize, end: usize) {
      // Java: if (!locale.getLanguage().equals(Locale.CHINA.getLanguage()) && !locale.getLanguage().equals(Locale.ENGLISH.getLanguage()))
      {
         // Java: return (new String(chars, start, end - start)).toLowerCase().toCharArray();
         // (skipped: in Rust we always use ASCII lowercase below)
      }
      // Java: else
      {
         // Java: for(int i = start; i < end; ++i)
         for i in start..end.min(chars.len())
         {
            // Java: int c = chars[i];
            let c = chars[i];
            // Java: if (c >= 65 && c <= 90)
            if c >= 'A' && c <= 'Z'
            {
               // Java: chars[i] = (char)(c + 97 - 65);
               chars[i] = (c as u8 + 97 - 65) as char;
            }
         }
         // Java: return chars;
      }
    }

    // Java: static String toLowerCase(String str)
    pub fn toLowerCase_str(s: &str) -> String {
      /* DEFERRED_NON_FUNCTION：Java 以 UTF-16 下标遍历（`int i = 0` + `str.charAt(i)`）；Rust 用
         `s.chars()` 迭代器，无该下标变量 ⇒ 仅此条声明停车（本 fn 其余语句均为活代码）。
      // Java: int i = 0;
      */
      // Java: for(int n = str.length(); i < n; ++i)
      for ch in s.chars()
      {
         // Java: char c = str.charAt(i);
         // Java: if (c < 'a' || c > 'z')
         if ch < 'a' || ch > 'z'
         {
            // Java: return str.toLowerCase();
            return s.to_lowercase();
         }
      }
      // Java: return str;
      s.to_string()
    }

    // Java: static boolean isValidHexChars(char c1, char c2)
    pub fn isValidHexChars(c1: char, c2: char) -> bool {
      // Java: return ('0' <= c1 && c1 <= '9' || 'A' <= c1 && c1 <= 'F' || 'a' <= c1 && c1 <= 'f') && ('0' <= c2 && c2 <= '9' || 'A' <= c2 && c2 <= 'F' || 'a' <= c2 && c2 <= 'f');
      let is_hex = |c: char| (c >= '0' && c <= '9') || (c >= 'A' && c <= 'F') || (c >= 'a' && c <= 'f');
      is_hex(c1) && is_hex(c2)
    }

    // Java: static String commonValidate(char[] chars, int start, int end, String name)
    pub fn commonValidate(chars: &[char], start: usize, end: usize, name: &str) -> Result<String, String> {
      // Java: for(int i = start; i < end; ++i)
      let mut i = start;
      while i < end
      {
         // Java: int c = chars[i];
         let c = chars[i] as u32;
         // Java: if (c <= 127)
         if c <= 127
         {
            // Java: if (ASCII_FLAG_TABLE[c] <= 0)
            if ASCII_FLAG_TABLE[c as usize] <= 0
            {
               // Java: if (c != 37)
               if c != 37
               {
                  // Java: throw new URISyntaxException(new String(chars), "Illegal character in " + name, i);
                  return Err(format!("Illegal character in {} at {}", name, i));
               }
               // Java: if (i + 2 >= end)
               if i + 2 >= end
               {
                  // Java: throw new URISyntaxException(new String(chars), "Incomplete % sequence in " + name, i);
                  return Err(format!("Incomplete % sequence in {} at {}", name, i));
               }
               // Java: if (!isValidHexChars(chars[i + 1], chars[i + 2]))
               if !Self::isValidHexChars(chars[i + 1], chars[i + 2])
               {
                  // Java: throw new URISyntaxException(new String(chars), "Invalid % sequence: " + new String(chars, i, 3) + " in " + name, i);
                  return Err(format!("Invalid % sequence in {} at {}", name, i));
               }
               // Java: i += 2;
               i += 2;
            }
         }
         // Java: else if (Character.isSpaceChar(c) || Character.isISOControl(c))
         else if (chars[i]).is_whitespace() || c < 32
         {
            // Java: throw new URISyntaxException(new String(chars), "Illegal character in " + name, i);
            return Err(format!("Illegal character in {} at {}", name, i));
         }
         i += 1;
      }
      // Java: return new String(chars, start, end - start);
      Ok(chars[start..end].iter().collect())
    }

    // Java: static String pathValidate(char[] chars, int start, int end, String name)
    pub fn pathValidate(chars: &[char], start: usize, end: usize, name: &str) -> Result<String, String> {
      // Java: for(int i = start; i < end; ++i)
      let mut i = start;
      while i < end
      {
         // Java: int c = chars[i];
         let c = chars[i] as u32;
         // Java: if (c <= 127)
         if c <= 127
         {
            // Java: if (ASCII_FLAG_TABLE[c] < 2 && c != 47)
            if ASCII_FLAG_TABLE[c as usize] < 2 && c != 47
            {
               // Java: if (c != 37)
               if c != 37
               {
                  // Java: throw new URISyntaxException(new String(chars), "Illegal character in " + name, i);
                  return Err(format!("Illegal character in {} at {}", name, i));
               }
               // Java: if (i + 2 >= end)
               if i + 2 >= end
               {
                  // Java: throw new URISyntaxException(new String(chars), "Incomplete % sequence in " + name, i);
                  return Err(format!("Incomplete % sequence in {} at {}", name, i));
               }
               // Java: if (!isValidHexChars(chars[i + 1], chars[i + 2]))
               if !Self::isValidHexChars(chars[i + 1], chars[i + 2])
               {
                  // Java: throw new URISyntaxException(new String(chars), "Invalid % sequence: " + new String(chars, i, 3) + " in " + name, i);
                  return Err(format!("Invalid % sequence in {} at {}", name, i));
               }
               // Java: i += 2;
               i += 2;
            }
         }
         // Java: else if (Character.isSpaceChar(c) || Character.isISOControl(c))
         else if (chars[i]).is_whitespace() || c < 32
         {
            // Java: throw new URISyntaxException(new String(chars), "Illegal character in " + name, i);
            return Err(format!("Illegal character in {} at {}", name, i));
         }
         i += 1;
      }
      // Java: return new String(chars, start, end - start);
      Ok(chars[start..end].iter().collect())
    }

    // Java: static String authorityValidate(char[] chars, int start, int end, String name)
    pub fn authorityValidate(chars: &[char], start: usize, end: usize, name: &str) -> Result<String, String> {
      // Java: for(int i = start; i < end; ++i)
      let mut i = start;
      while i < end
      {
         // Java: int c = chars[i];
         let c = chars[i] as u32;
         // Java: if (c <= 127)
         if c <= 127
         {
            // Java: if (ASCII_FLAG_TABLE[c] < 2 && c != 91 && c != 93)
            if ASCII_FLAG_TABLE[c as usize] < 2 && c != 91 && c != 93
            {
               // Java: if (c != 37)
               if c != 37
               {
                  // Java: throw new URISyntaxException(new String(chars), "Illegal character in " + name, i);
                  return Err(format!("Illegal character in {} at {}", name, i));
               }
               // Java: if (i + 2 >= end)
               if i + 2 >= end
               {
                  // Java: throw new URISyntaxException(new String(chars), "Incomplete % sequence in " + name, i);
                  return Err(format!("Incomplete % sequence in {} at {}", name, i));
               }
               // Java: if (!isValidHexChars(chars[i + 1], chars[i + 2]))
               if !Self::isValidHexChars(chars[i + 1], chars[i + 2])
               {
                  // Java: throw new URISyntaxException(new String(chars), "Invalid % sequence: " + new String(chars, i, 3) + " in " + name, i);
                  return Err(format!("Invalid % sequence in {} at {}", name, i));
               }
               // Java: i += 2;
               i += 2;
            }
         }
         // Java: else if (Character.isSpaceChar(c) || Character.isISOControl(c))
         else if (chars[i]).is_whitespace() || c < 32
         {
            // Java: throw new URISyntaxException(new String(chars), "Illegal character in " + name, i);
            return Err(format!("Illegal character in {} at {}", name, i));
         }
         i += 1;
      }
      // Java: return new String(chars, start, end - start);
      Ok(chars[start..end].iter().collect())
    }

    // Java: static String authoritySafePath(String authority, String path)
    pub fn authoritySafePath(authority: Option<&str>, path: &str) -> String {
      // Java: return authority != null && !authority.isEmpty() && !path.isEmpty() && !path.startsWith("/") ? "/" + path : path;
      if let Some(auth) = authority {
          if !auth.is_empty() && !path.is_empty() && !path.starts_with('/') {
              return format!("/{}", path);
          }
      }
      path.to_string()
    }

    // Java: static int hex2Int(char c)
    fn hex2Int(c: char) -> i32 {
      // Java: if ('0' <= c && c <= '9')
      if c >= '0' && c <= '9'
      {
         // Java: return c - 48;
         return c as i32 - 48;
      }
      // Java: else if ('A' <= c && c <= 'F')
      if c >= 'A' && c <= 'F'
      {
         // Java: return 10 + (c - 65);
         return 10 + (c as i32 - 65);
      }
      // Java: else
      {
         // Java: return 'a' <= c && c <= 'f' ? 10 + (c - 97) : -1;
         if c >= 'a' && c <= 'f' { 10 + (c as i32 - 97) } else { -1 }
      }
    }

    // Java: static String decode(String s)
    pub fn decode(s: &str) -> String {
      // Java: if (s.indexOf(37) == -1)
      if !s.contains('%')
      {
         // Java: return s;
         return s.to_string();
      }
      // Java: else
      {
         // Java: StringBuilder sb = new StringBuilder(s.length());
         let mut sb = String::with_capacity(s.len());
         // Java: ByteArrayOutputStream bout = new ByteArrayOutputStream();
         let mut bout: Vec<u8> = Vec::new();
         // Java: int i = 0;
         let mut i: usize = 0;
         // Java: int n = s.length();
         let n = s.len();
         let chars: Vec<char> = s.chars().collect();
         // Java: label42:
         // Java: while(i < n)
         'label42: while i < n
         {
            // Java: char c = s.charAt(i);
            let c = chars[i];
            // Java: if (c == '%')
            if c == '%'
            {
               // Java: int d1;
               // Java: int d2;
               // Java: while(i + 2 < n && (d1 = hex2Int(s.charAt(i + 1))) != -1 && (d2 = hex2Int(s.charAt(i + 2))) != -1)
               loop {
                  if i + 2 >= n { break; }
                  let d1 = Self::hex2Int(chars[i + 1]);
                  let d2 = Self::hex2Int(chars[i + 2]);
                  if d1 == -1 || d2 == -1 { break; }
                  {
                     // Java: bout.write((byte)(d1 << 4 | d2));
                     bout.push((d1 << 4 | d2) as u8);
                     // Java: i += 3;
                     i += 3;
                     // Java: if (i >= n || s.charAt(i) != '%')
                     if i >= n || chars.get(i).copied() != Some('%')
                     {
                        // Java: sb.append(new String(bout.toByteArray(), CHARSET_UTF8));
                        sb.push_str(&String::from_utf8_lossy(&bout));
                        // Java: bout.reset();
                        bout.clear();
                        // Java: continue label42;
                        continue 'label42;
                     }
                  }
               }
               // Java: throw new IllegalArgumentException("Invalid % sequence at " + i + ": " + s);
               panic!("Invalid % sequence at {}: {}", i, s);
            }
            // Java: else
            else
            {
               // Java: sb.append(c);
               sb.push(c);
               // Java: ++i;
               i += 1;
            }
         }
         // Java: return sb.toString();
         sb
      }
    }

}
