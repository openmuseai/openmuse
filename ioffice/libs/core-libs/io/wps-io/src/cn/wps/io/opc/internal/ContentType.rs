// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Java: public class ContentType
pub struct ContentType {
    // Java field: String type
    r#type: String,
    // Java field: String subType
    subType: String,
    // Java field: int hash
    hash: i32,
}

// Java field: byte[] VALID_CHAR_TABLE
static VALID_CHAR_TABLE: &[u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0,
];

// Java: public class ContentType
impl ContentType {
    // Java: ContentType(String contentType)
    pub fn new(contentType: &str) -> Result<Self, String> {
      // Java: super();
      // Java: char[] chars = contentType.toCharArray();
      let chars: Vec<char> = contentType.chars().collect();
      // Java: int posSlash = -1;
      let mut posSlash: i32 = -1;
      // Java: int n = chars.length;
      let n = chars.len();
      // Java: int i;
      let mut i: usize = 0;
      // Java: for(i = 0; i < n; ++i)
      while i < n
      {
         // Java: char c = chars[i];
         let c = chars[i];
         // Java: if (c > 127)
         if c as u32 > 127
         {
            // Java: throw new InvalidFormatException("The specified content type is not an ASCII value.");
            return Err("The specified content type is not an ASCII value.".to_string());
         }
         // Java: if (VALID_CHAR_TABLE[c] == 0)
         if VALID_CHAR_TABLE[c as usize] == 0
         {
            // Java: if (c != '/')
            if c != '/'
            {
               // Java: throw new InvalidFormatException("The specified content type '" + contentType + "' is not compliant with RFC 2616: malformed content type. pos: " + i);
               return Err(format!("The specified content type '{}' is not compliant with RFC 2616: malformed content type. pos: {}", contentType, i));
            }
            // Java: if (posSlash >= 0)
            if posSlash >= 0
            {
               // Java: throw new InvalidFormatException("The specified content type '" + contentType + "' is not compliant with RFC 2616: malformed content type. pos: " + i);
               return Err(format!("The specified content type '{}' is not compliant with RFC 2616: malformed content type. pos: {}", contentType, i));
            }
            // Java: if (i < 1)
            if i < 1
            {
               // Java: throw new InvalidFormatException("The specified content type '" + contentType + "' is not compliant with RFC 2616: malformed content type. pos: " + i);
               return Err(format!("The specified content type '{}' is not compliant with RFC 2616: malformed content type. pos: {}", contentType, i));
            }
            // Java: posSlash = i;
            posSlash = i as i32;
         }
         i += 1;
      }
      // Java: if (posSlash != -1 && i > posSlash)
      if posSlash != -1 && (i as i32) > posSlash
      {
         // Java: this.type = new String(chars, 0, posSlash);
         let type_str: String = chars[..posSlash as usize].iter().collect();
         // Java: this.subType = new String(chars, posSlash + 1, n - posSlash - 1);
         let sub_type_str: String = chars[(posSlash as usize + 1)..n].iter().collect();
         Ok(ContentType { r#type: type_str, subType: sub_type_str, hash: -1 })
      }
      // Java: else
      else
      {
         // Java: throw new InvalidFormatException("The specified content type '" + contentType + "' is not compliant with RFC 2616: malformed content type. pos: " + i);
         Err(format!("The specified content type '{}' is not compliant with RFC 2616: malformed content type. pos: {}", contentType, i))
      }
    }

    // Rust-native: Java 构造器 `public ContentType(String contentType)` 的**同名入口**
    // （仓内 ctor 约定：与 Java 同名；`new` 与它同一实现，供既有调用方使用）。
    pub fn ContentType(contentType: &str) -> Result<Self, String> {
      // Java: super();
      ContentType::new(contentType)
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuffer retVal = new StringBuffer();
      let mut retVal = String::new();
      // Java: retVal.append(this.getType());
      retVal.push_str(&self.getType());
      // Java: retVal.append("/");
      retVal.push('/');
      // Java: retVal.append(this.getSubType());
      retVal.push_str(&self.getSubType());
      // Java: return retVal.toString();
      retVal
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, obj: &ContentType) -> bool {
      // Java: return !(obj instanceof ContentType) || this.toString().equalsIgnoreCase(obj.toString());
      self.toString().to_lowercase() == obj.toString().to_lowercase()
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: if (this.hash == -1)
      if self.hash == -1
      {
         // Java: this.hash = this.toString().hashCode();
         let mut hasher = DefaultHasher::new();
         self.toString().hash(&mut hasher);
         self.hash = hasher.finish() as i32;
      }
      // Java: return this.hash;
      self.hash
    }

    // Java: String getSubType()
    pub fn getSubType(&self) -> &str {
      // Java: return this.subType;
      &self.subType
    }

    // Java: String getType()
    pub fn getType(&self) -> &str {
      // Java: return this.type;
      &self.r#type
    }

}

// 集合 key 语义（Java Hashtable<ContentType,...>）：equals 用 ignoreCase，hashCode 用 toString().hashCode()（大小写敏感）
impl PartialEq for ContentType {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
impl Eq for ContentType {}
impl Hash for ContentType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.toString().hash(state);
    }
}
