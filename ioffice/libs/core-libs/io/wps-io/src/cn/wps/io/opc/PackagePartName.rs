// Java: import cn.wps.io.util.URI
// Java: import cn.wps.io.util.WpsUriUtils
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JRuntimeException
// Java: import java.net.URISyntaxException

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::io::util::URI::URI;
use crate::io::util::WpsUriUtils::{WpsUriUtils, ASCII_FLAG_TABLE};
// Java: （同包引用）PackagingURIHelper.PACKAGE_ROOT_URI / FORWARD_SLASH_CHAR
// NOTE(port): Java 里它们是 `PackagingURIHelper` 的 **static 成员**（`PackagingURIHelper.PACKAGE_ROOT_URI`）；
// Rust 侧该类的静态量按本仓约定落在**模块级**（`PackagingURIHelper.rs` 的 `package_root_uri()` / `FORWARD_SLASH_CHAR`）⇒ 这里按模块路径引用。
use crate::io::opc::PackagingURIHelper::{package_root_uri, FORWARD_SLASH_CHAR};

// Java: public class PackagePartName implements Comparable
#[derive(Clone)]
pub struct PackagePartName {
    // Java field: URI partNameURI
    pub(crate) partNameURI: URI,
    // Java field: boolean isRelationship
    pub(crate) isRelationship: bool,
    // Java field: int hash
    pub(crate) hash: i32,
    // Java field: char[] partName
    pub(crate) partName: Option<Vec<char>>,
    // Java field: boolean isLowercase
    pub(crate) isLowercase: bool,
}

// Java: public class PackagePartName implements Comparable
impl PackagePartName {
    // Java: PackagePartName(URI uri, boolean checkConformance)
    pub(crate) fn new(uri: &URI, check_conformance: bool) -> Result<PackagePartName, String> {
      // Java: super();
      // Java: if (checkConformance)
      // 字段初值照抄 Java 声明（`:10 hash = -1`、`:13 partName = null`、`:14 isLowercase = true`）
      let mut partName_ = PackagePartName {
         // Java field: URI partNameURI（Java 在本方法末尾赋值；此处先置骨架，随后统一覆盖）
         partNameURI: uri.clone(),
         // Java field: boolean isRelationship
         isRelationship: false,
         // Java: private int hash = -1;
         hash: -1,
         // Java: private char[] partName = null;
         partName: None,
         // Java: private boolean isLowercase = true;
         isLowercase: true,
      };
      if check_conformance {
         // Java: this.throwExceptionIfInvalidPartUri(uri);
         partName_.throwExceptionIfInvalidPartUri(uri)?;
      }
      // Java: else
      else
      {
         // Java: if (!PackagingURIHelper.PACKAGE_ROOT_URI.equals(uri))
         // NOTE(port): Rust `URI` 未移植 `equals`（util 面，本 function 不得改）⇒ 用既有 `compareTo == 0` 等价物；
         // 该分支唯一实际入参就是根 URI `"/"`，两者在此判定上一致。
         if package_root_uri().compareTo(uri) != 0
         {
            // Java: throw new OpenXML4JRuntimeException("OCP conformance must be check for ALL part name except special cases : ['/']");
            return Err("OCP conformance must be check for ALL part name except special cases : ['/']".to_string());
         }
         // Java: this.partName = new char[2];
         partName_.partName = Some(vec!['\0'; 2]);
         // Java: this.partName[0] = PackagingURIHelper.FORWARD_SLASH_CHAR;
         partName_.partName.as_mut().unwrap()[0] = FORWARD_SLASH_CHAR;
      }

      // Java: this.partNameURI = uri;
      partName_.partNameURI = uri.clone();
      // Java: this.isRelationship = this.isRelationshipPartURI(uri);
      partName_.isRelationship = PackagePartName::isRelationshipPartURI_uri(uri);
      // Java: （构造器无 return；Java 抛异常即构造失败）
      Ok(partName_)
    }

    // Java: boolean isRelationshipPartURI(URI partUri)
    fn isRelationshipPartURI_uri(partUri: &URI) -> bool {
      // Java: if (partUri == null)
      // (Rust: always valid reference)
      {
         // Java: throw new IllegalArgumentException("partUri");
      }
      // Java: else
      {
         // Java: char[] chars = partUri.getPath().toCharArray();
         let path_str = partUri.getPath().unwrap_or_default();
         let chars: Vec<char> = path_str.chars().collect();
         // Java: int n = chars.length;
         let n = chars.len();
         // Java: int state = 0;
         let mut state = 0;
         // Java: for(int i = 0; i < n; ++i)
         let mut i = 0;
         while i < n
         {
            // Java: char c = chars[i];
            let c = chars[i];
            // Java: switch (c)
            match c
            {
               // Java: case '.':
               '.' => {
                  // Java: if (state == 1)
                  if state == 1
                  {
                     // Java: if (n - i < 5)
                     if n - i < 5
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (n - i == 5 && chars[i + 1] == 'r' && chars[i + 2] == 'e' && chars[i + 3] == 'l' && chars[i + 4] == 's')
                     if n - i == 5 && chars[i + 1] == 'r' && chars[i + 2] == 'e' && chars[i + 3] == 'l' && chars[i + 4] == 's'
                     {
                        // Java: return true;
                        return true;
                     }
                  }
               },
                  // Java: break;
               // Java: case '/':
               '/' => {
                  // Java: if (state == 0)
                  if state == 0
                  {
                     // Java: if (n - i < 12)
                     if n - i < 12
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (chars[i + 1] == '_' && chars[i + 2] == 'r' && chars[i + 3] == 'e' && chars[i + 4] == 'l' && chars[i + 5] == 's' && chars[i + 6] == '/')
                     if chars[i + 1] == '_' && chars[i + 2] == 'r' && chars[i + 3] == 'e' && chars[i + 4] == 'l' && chars[i + 5] == 's' && chars[i + 6] == '/'
                     {
                        // Java: i += 6;
                        i += 6;
                        // Java: state = 1;
                        state = 1;
                     }
                  }
               },
               _ => {}
            }
            i += 1;
         }
         // Java: return false;
         return false;
      }
    }

    // Java: boolean isRelationshipPartURI()
    pub fn isRelationshipPartURI(&self) -> bool {
      // Java: return this.isRelationship;
      self.isRelationship
    }

    // Java: void throwExceptionIfInvalidPartUri(URI partUri)
    fn throwExceptionIfInvalidPartUri(&mut self, partUri: &URI) -> Result<(), String> {
      // Java: if (partUri == null)
      // (Rust: always valid)
      {
         // Java: throw new IllegalArgumentException("partUri");
      }
      // Java: else
      {
         // Java: String uriPath = partUri.getPath();
         let uriPath = partUri.getPath().unwrap_or_default();
         // Java: throwExceptionIfEmptyURI(uriPath);
         Self::throwExceptionIfEmptyURI(&uriPath)?;
         // Java: throwExceptionIfAbsoluteUri(partUri);
         Self::throwExceptionIfAbsoluteUri(partUri)?;
         // Java: throwExceptionIfPartNameNotStartsWithForwardSlashChar(uriPath);
         Self::throwExceptionIfPartNameNotStartsWithForwardSlashChar(&uriPath)?;
         // Java: this.partName = WpsUriUtils.toASCIIString(partUri.toString());
         self.partName = Some(WpsUriUtils::toASCIIString(&partUri.toString()));
         // Java: throwExceptionIfPartNameHaveInvalidSegments(this.partName);
         Self::throwExceptionIfPartNameHaveInvalidSegments(self.partName.as_ref().unwrap())?;
         // Java: char[] tmp = WpsUriUtils.toLowerCase(this.partName);
         let mut tmp = self.partName.clone().unwrap();
         WpsUriUtils::toLowerCase_chars(&mut tmp);
         // Java: this.isLowercase = tmp == this.partName;
         self.isLowercase = tmp == *self.partName.as_ref().unwrap();
         // Java: this.partName = tmp;
         self.partName = Some(tmp);
      }
      Ok(())
    }

    // Java: static void throwExceptionIfEmptyURI(String uriPath)
    fn throwExceptionIfEmptyURI(uriPath: &str) -> Result<(), String> {
      // Java: if (uriPath.length() == 0 || uriPath.length() == 1 && uriPath.charAt(0) == PackagingURIHelper.FORWARD_SLASH_CHAR)
      if uriPath.is_empty() || (uriPath.len() == 1 && uriPath.starts_with('/'))
      {
         // Java: throw new InvalidFormatException("A part name shall not be empty [M1.1]: " + uriPath);
         return Err(format!("A part name shall not be empty [M1.1]: {}", uriPath));
      }
      Ok(())
    }

    // Java: static void throwExceptionIfPartNameHaveInvalidSegments(char[] partName)
    fn throwExceptionIfPartNameHaveInvalidSegments(partName: &[char]) -> Result<(), String> {
      // Java: if (partName[0] != '/')
      if partName[0] != '/'
      {
         // Java: throw new InvalidFormatException("A part name shall not have empty segments [M1.3]: " + new String(partName, 0, partName.length - 1));
         // （Java 的 new String(chars, 0, len-1) ⇒ 去掉 toASCIIString 追加的结尾 NUL）
         return Err(format!("A part name shall not have empty segments [M1.3]: {}", partName[..partName.len() - 1].iter().collect::<String>()));
      }
      // Java: else
      {
         // Java: int count = 0;
         let mut count = 0;
         // Java: int start = 1;
         let mut start: usize = 1;
         // Java: int state = 0;
         let mut state = 0;
         // Java: int nonDotCharCount = 0;
         let mut nonDotCharCount = 0;
         // Java: boolean encounterEmptySeg = false;
         let mut encounterEmptySeg = false;
         // Java: int i = 1;
         let mut i: usize = 1;
         // Java: for(int n = partName.length; i < n; ++i)
         let n = partName.len();
         while i < n
         {
            // Java: switch (partName[i])
            match partName[i]
            {
               // Java: case '\u0000':
               // Java: case '/':
               '\0' | '/' => {
                  // Java: if (i > start)
                  if i > start
                  {
                     // Java: if (encounterEmptySeg)
                     if encounterEmptySeg
                     {
                        // Java: throw new InvalidFormatException("A part name shall not have empty segments [M1.3]: " + new String(partName, 0, partName.length - 1));
                        return Err(format!("A part name shall not have empty segments [M1.3]: {}", partName[..partName.len() - 1].iter().collect::<String>()));
                     }
                     // Java: if (partName[i - 1] == '.')
                     if partName[i - 1] == '.'
                     {
                        // Java: throw new InvalidFormatException("A segment shall not end with a dot ('.') character [M1.9]: " + new String(partName, 0, partName.length - 1));
                        return Err(format!("A segment shall not end with a dot ('.') character [M1.9]: {}", partName[..partName.len() - 1].iter().collect::<String>()));
                     }
                     // Java: if (nonDotCharCount == 0)
                     if nonDotCharCount == 0
                     {
                        // Java: throw new InvalidFormatException("A segment shall include at least one non-dot character. [M1.10]: " + new String(partName, 0, partName.length - 1));
                        return Err(format!("A segment shall include at least one non-dot character. [M1.10]: {}", partName[..partName.len() - 1].iter().collect::<String>()));
                     }
                     // Java: checkPCharCompliance(partName, start, i - start);
                     Self::checkPCharCompliance(partName, start, i - start)?;
                     // Java: ++count;
                     count += 1;
                  }
                  // Java: else
                  else
                  {
                     // Java: encounterEmptySeg = true;
                     encounterEmptySeg = true;
                  }
                  // Java: state = 0;
                  state = 0;
                  // Java: nonDotCharCount = 0;
                  nonDotCharCount = 0;
                  // Java: start = i + 1;
                  start = i + 1;
               },
                  // Java: break;
               // Java: case '\\':
               '\\' => {
                  // Java: if (state == 1)
                  if state == 1
                  {
                     // Java: state = 2;
                     state = 2;
                  }
                  // Java: else
                  else
                  {
                     // Java: state = 1;
                     state = 1;
                  }
               },
                  // Java: break;
               // Java: default:
               _ => {
                  // Java: if (state == 1)
                  if state == 1
                  {
                     // Java: state = 2;
                     state = 2;
                  }
                  // Java: else if (state == 2)
                  else if state == 2
                  {
                     // Java: ++nonDotCharCount;
                     nonDotCharCount += 1;
                     // Java: state = 0;
                     state = 0;
                  }
                  // Java: else
                  else
                  {
                     // Java: ++nonDotCharCount;
                     nonDotCharCount += 1;
                  }
               }
            }
            // Java: ++i;
            i += 1;
         }
         // Java: if (count < 1)
         if count < 1
         {
            // Java: throw new InvalidFormatException("A part name shall not have empty segments [M1.3]: " + new String(partName, 0, partName.length - 1));
            return Err(format!("A part name shall not have empty segments [M1.3]: {}", partName[..partName.len() - 1].iter().collect::<String>()));
         }
      }
      Ok(())
    }

    // Java: static int hexChar2Int(char c)
    fn hexChar2Int(c: char) -> i32 {
      // Java: if (c >= '0' && c <= '9')
      if c >= '0' && c <= '9'
      {
         // Java: return c - 48;
         return c as i32 - 48;
      }
      // Java: else if (c >= 'A' && c <= 'F')
      else if c >= 'A' && c <= 'F'
      {
         // Java: return c - 65 + 10;
         return c as i32 - 65 + 10;
      }
      // Java: else if (c >= 'a' && c <= 'f')
      else if c >= 'a' && c <= 'f'
      {
         // Java: return c - 97 + 10;
         return c as i32 - 97 + 10;
      }
      // Java: else
      else
      {
         // Java: throw new NumberFormatException(String.valueOf(c));
         // （NumberFormatException 是 RuntimeException（未受检）⇒ 会穿透构造器的 throws 声明；Rust 的对应物是 panic）
         panic!("{}", c);
      }
    }

    // Java: static void checkPCharCompliance(char[] segment, int begin, int len)
    fn checkPCharCompliance(segment: &[char], begin: usize, len: usize) -> Result<(), String> {
      // Java: for(int i = begin; i < len; ++i)
      let mut i = begin;
      while i < len
      {
         // Java: char c = segment[i];
         let c = segment[i];
         // Java: boolean errorFlag = true;
         let mut errorFlag;
         // Java: if (c <= 127 && WpsUriUtils.ASCII_FLAG_TABLE[c] >= 2)
         if (c as u32) <= 127 && ASCII_FLAG_TABLE[c as usize] >= 2
         {
            // Java: errorFlag = false;
            errorFlag = false;
         }
         // Java: else
         else
         {
            // Java: errorFlag = true;
            errorFlag = true;
         }
         // Java: if (errorFlag)
         if errorFlag
         {
            // Java: if (c != '%')
            if c != '%'
            {
               // Java: throw new InvalidFormatException("A segment shall not hold any characters other than pchar characters. [M1.6]");
               return Err("A segment shall not hold any characters other than pchar characters. [M1.6]".to_string());
            }
            // Java: if (len - i < 2)
            if len - i < 2
            {
               // Java: throw new InvalidFormatException("The segment " + new String(segment, begin, len) + " contain invalid encoded character !");
               // （Java 的 new String(chars, offset, count) ⇒ 取 [begin, begin + len)，与上面的 `len - i < 2` 同一套「长度当结束下标」用法）
               return Err(format!(
                  "The segment {} contain invalid encoded character !",
                  segment[begin..begin + len].iter().collect::<String>()
               ));
            }
            // Java: errorFlag = false;
            errorFlag = false;
            // Java: char decodedChar = (char)(hexChar2Int(segment[i + 1]) << 4 | hexChar2Int(segment[i + 2]));
            let decodedChar = ((Self::hexChar2Int(segment[i + 1]) << 4) | Self::hexChar2Int(segment[i + 2])) as u8 as char;
            // Java: i += 2;
            i += 2;
            // Java: if (decodedChar == '/' || decodedChar == '\\')
            if decodedChar == '/' || decodedChar == '\\'
            {
               // Java: throw new InvalidFormatException("A segment shall not contain percent-encoded forward slash ('/'), or backward slash ('') characters. [M1.7]");
               return Err("A segment shall not contain percent-encoded forward slash ('/'), or backward slash ('') characters. [M1.7]".to_string());
            }
            // Java: if (decodedChar <= 127 && WpsUriUtils.ASCII_FLAG_TABLE[decodedChar] >= 3)
            if (decodedChar as u32) <= 127 && ASCII_FLAG_TABLE[decodedChar as usize] >= 3
            {
               // Java: throw new InvalidFormatException("A segment shall not contain percent-encoded unreserved characters. [M1.8]");
               return Err("A segment shall not contain percent-encoded unreserved characters. [M1.8]".to_string());
            }
         }
         i += 1;
      }
      Ok(())
    }

    // Java: static void throwExceptionIfPartNameNotStartsWithForwardSlashChar(String uriPath)
    fn throwExceptionIfPartNameNotStartsWithForwardSlashChar(uriPath: &str) -> Result<(), String> {
      // Java: if (uriPath.length() > 0 && uriPath.charAt(0) != PackagingURIHelper.FORWARD_SLASH_CHAR)
      if !uriPath.is_empty() && !uriPath.starts_with('/')
      {
         // Java: throw new InvalidFormatException("A part name shall start with a forward slash ('/') character [M1.4]: " + uriPath);
         return Err(format!("A part name shall start with a forward slash ('/') character [M1.4]: {}", uriPath));
      }
      Ok(())
    }

    // Java: static void throwExceptionIfPartNameEndsWithForwardSlashChar(String uriPath)
    fn throwExceptionIfPartNameEndsWithForwardSlashChar(uriPath: &str) -> Result<(), String> {
      // Java: if (uriPath.length() > 0 && uriPath.charAt(uriPath.length() - 1) == PackagingURIHelper.FORWARD_SLASH_CHAR)
      if !uriPath.is_empty() && uriPath.ends_with('/')
      {
         // Java: throw new InvalidFormatException("A part name shall not have a forward slash as the last character [M1.5]: " + uriPath);
         return Err(format!("A part name shall not have a forward slash as the last character [M1.5]: {}", uriPath));
      }
      Ok(())
    }

    // Java: static void throwExceptionIfAbsoluteUri(URI partUri)
    fn throwExceptionIfAbsoluteUri(partUri: &URI) -> Result<(), String> {
      // Java: if (partUri.isAbsolute())
      if partUri.isAbsolute()
      {
         // Java: throw new InvalidFormatException("Absolute URI forbidden: " + partUri);
         return Err(format!("Absolute URI forbidden: {}", partUri.toString()));
      }
      Ok(())
    }

    // Java: int compareTo(PackagePartName otherPartName)
    pub fn compareTo(&self, otherPartName: Option<&PackagePartName>) -> i32 {
      // Java: if (otherPartName == null)
      if otherPartName.is_none()
      {
         // Java: return -1;
         return -1;
      }
      // Java: else
      let other = otherPartName.unwrap();
      {
         // Java: char[] chars = this.partName;
         let chars = self.partName.as_ref().unwrap();
         // Java: char[] charsOther = otherPartName.partName;
         let charsOther = other.partName.as_ref().unwrap();
         // Java: int n = chars.length;
         let mut n = chars.len();
         // Java: if (charsOther.length < n)
         if charsOther.len() < n
         {
            // Java: n = charsOther.length;
            n = charsOther.len();
         }
         // Java: for(int i = 0; i < n; ++i)
         for i in 0..n
         {
            // Java: if (chars[i] < charsOther[i])
            if chars[i] < charsOther[i]
            {
               // Java: return -1;
               return -1;
            }
            // Java: if (chars[i] > charsOther[i])
            if chars[i] > charsOther[i]
            {
               // Java: return 1;
               return 1;
            }
         }
         // Java: return 0;
         return 0;
      }
    }

    // Java: String getExtension()
    pub fn getExtension(&self) -> String {
      // Java: String fragment = this.partNameURI.getPath();
      let fragment = self.partNameURI.getPath().unwrap_or_default();
      // Java: for(int i = fragment.length() - 1; i >= 0; --i)
      let chars: Vec<char> = fragment.chars().collect();
      let mut i = chars.len() as i32 - 1;
      while i >= 0
      {
         // Java: if (fragment.charAt(i) == '.')
         if chars[i as usize] == '.'
         {
            // Java: return fragment.substring(i + 1);
            return chars[(i as usize + 1)..].iter().collect();
         }
         i -= 1;
      }
      // Java: return "";
      String::new()
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
      // Java: char[] chars = WpsUriUtils.toASCIIString(this.partNameURI.toString());
      let chars = WpsUriUtils::toASCIIString(&self.partNameURI.toString());
      // Java: return new String(chars, 0, chars.length - 1);
      chars[..chars.len() - 1].iter().collect()
    }

    // Java: boolean equals(Object otherPartName)
    pub fn equals(&self, otherPartName: Option<&PackagePartName>) -> bool {
      // Java: if (otherPartName != null && otherPartName instanceof PackagePartName)
      if let Some(other) = otherPartName
      {
         // Java: char[] chars = this.partName;
         let chars = self.partName.as_ref().unwrap();
         // Java: char[] charsOther = ((PackagePartName)otherPartName).partName;
         let charsOther = other.partName.as_ref().unwrap();
         // Java: int len = chars.length - 1;
         let len = chars.len() - 1;
         // Java: if (len != charsOther.length - 1)
         if len != charsOther.len() - 1
         {
            // Java: return false;
            return false;
         }
         // Java: else
         else
         {
            // Java: for(int i = 0; i < len; ++i)
            for i in 0..len
            {
               // Java: if (chars[i] != charsOther[i])
               if chars[i] != charsOther[i]
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: return true;
            return true;
         }
      }
      // Java: else
      else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: if (this.hash != -1)
      if self.hash != -1
      {
         // Java: return this.hash;
         return self.hash;
      }
      // Java: else
      else
      {
         // Java: this.hash = (new String(this.partName, 0, this.partName.length - 1)).hashCode();
         let s: String = self.partName.as_ref().unwrap()[..self.partName.as_ref().unwrap().len() - 1].iter().collect();
         let mut hasher = DefaultHasher::new();
         s.hash(&mut hasher);
         self.hash = hasher.finish() as i32;
         // Java: return this.hash;
         return self.hash;
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return this.getName();
      self.getName()
    }

    // Java: URI getURI()
    pub fn getURI(&self) -> &URI {
      // Java: return this.partNameURI;
      &self.partNameURI
    }

}

impl PartialEq for PackagePartName {
    fn eq(&self, other: &Self) -> bool {
        self.getName() == other.getName()
    }
}

impl Eq for PackagePartName {}

impl Hash for PackagePartName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.getName().hash(state);
    }
}

