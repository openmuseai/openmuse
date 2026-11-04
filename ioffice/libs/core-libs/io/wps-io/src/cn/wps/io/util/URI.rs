// Java: import java.net.InetAddress
// Java: import java.net.URISyntaxException
// Java: import java.net.UnknownHostException
// Java: import java.util.Locale

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Java: public class URI implements Comparable
#[derive(Clone, Debug)]
pub struct URI {
    // Java field: String string
    pub(crate) string: Option<String>,
    // Java field: String scheme
    pub(crate) scheme: Option<String>,
    // Java field: String schemeSpecificPart
    pub(crate) schemeSpecificPart: Option<String>,
    // Java field: String authority
    pub(crate) authority: Option<String>,
    // Java field: String userInfo
    pub(crate) userInfo: Option<String>,
    // Java field: String host
    pub(crate) host: Option<String>,
    // Java field: int port
    pub(crate) port: i32,
    // Java field: String path
    pub(crate) path: Option<String>,
    // Java field: String query
    pub(crate) query: Option<String>,
    // Java field: String fragment
    pub(crate) fragment: Option<String>,
    // Java field: boolean opaque
    pub(crate) opaque: bool,
    // Java field: boolean absolute
    pub(crate) absolute: bool,
    // Java field: boolean serverAuthority
    serverAuthority: bool,
    // Java field: int hash
    pub(crate) hash: i32,
}

// Java: public class URI implements Comparable
impl URI {
    // Java: URI()
    fn new_empty() -> Self {
      // Java: super();
      URI {
          string: None, scheme: None, schemeSpecificPart: None,
          authority: None, userInfo: None, host: None, port: -1,
          path: None, query: None, fragment: None,
          opaque: false, absolute: false, serverAuthority: false, hash: -1,
      }
    }

    // Java: URI(String spec)
    pub fn new(spec: &str) -> Result<Self, String> {
      // Java: super();
      let mut uri = URI::new_empty();
      // Java: this.parseURI(spec);
      uri.parseURI(spec)?;
      Ok(uri)
    }

    // Java: String validateScheme(char[] chars, int start, int end)
    fn validateScheme(chars: &[char], start: usize, end: usize) -> Result<String, String> {
      // Java: if (end <= start)
      if end <= start
      {
         // Java: throw new URISyntaxException(new String(chars), "Scheme expected", 0);
         return Err("Scheme expected".to_string());
      }
      // Java: else
      {
         // Java: int advance = 0;
         let mut advance: usize;
         // Java: for(int i = start; i < end; i += advance)
         let mut i = start;
         while i < end
         {
            // Java: char c = chars[i];
            let c = chars[i];
            // Java: if (('a' > c || c > 'z') && ('A' > c || c > 'Z'))
            if !(c >= 'a' && c <= 'z') && !(c >= 'A' && c <= 'Z')
            {
               // Java: if ((i <= start || '0' > c || c > '9') && c != '+' && c != '-' && c != '=')
               if (i <= start || !(c >= '0' && c <= '9')) && c != '+' && c != '-' && c != '='
               {
                  // Java: if (c != '%')
                  if c != '%'
                  {
                     // Java: throw new URISyntaxException(new String(chars), "Illegal character in scheme", 0);
                     return Err("Illegal character in scheme".to_string());
                  }
                  // Java: if (i + 2 >= end)
                  if i + 2 >= end
                  {
                     // Java: throw new URISyntaxException(new String(chars), "Incomplete % sequence in scheme", i);
                     return Err(format!("Incomplete % sequence in scheme at {}", i));
                  }
                  // Java: if (!WpsUriUtils.isValidHexChars(chars[i + 1], chars[i + 2]))
                  if !is_valid_hex_chars(chars[i + 1], chars[i + 2])
                  {
                     // Java: throw new URISyntaxException(new String(chars), "Invalid % sequence: " + new String(chars, i, 3) + " in scheme", i);
                     return Err(format!("Invalid % sequence in scheme at {}", i));
                  }
                  // Java: advance = 3;
                  advance = 3;
               }
               // Java: else
               else
               {
                  // Java: advance = 1;
                  advance = 1;
               }
            }
            // Java: else
            else
            {
               // Java: advance = 1;
               advance = 1;
            }
            i += advance;
         }
         // Java: return new String(chars, start, end - start);
         Ok(chars[start..end].iter().collect())
      }
    }

    // Java: void parseURI(String uri)
    fn parseURI(&mut self, uri: &str) -> Result<(), String> {
      // Java: int len = uri.length();
      // The parser indexes the `chars` array below. UTF-8 byte length is not
      // an index into that array (for example, a Chinese relationship target).
      let len = uri.chars().count();
      // Java: if (len != 0)
      if len != 0
      {
         // Java: char[] chars = uri.toCharArray();
         let chars: Vec<char> = uri.chars().collect();
         // Java: int fragmentStart = len - 1;
         let mut fragmentStart: i32 = len as i32 - 1;
         // Java: while(fragmentStart >= 0 && chars[fragmentStart--] != '#')
         while fragmentStart >= 0 && chars[fragmentStart as usize] != '#' {
             fragmentStart -= 1;
         }
         // Java: if (fragmentStart >= 0)
         if fragmentStart >= 0
         {
            // Java: ++fragmentStart;
            fragmentStart += 1;
            // Java: this.fragment = WpsUriUtils.commonValidate(chars, fragmentStart + 1, len, "fragment");
            self.fragment = Some(chars[(fragmentStart as usize + 1)..len].iter().collect());
         }
         // Java: else
         else
         {
            // Java: fragmentStart = len;
            fragmentStart = len as i32;
         }
         let frag_start = fragmentStart as usize;
         // Java: int start = 0;
         let mut start: usize = 0;
         // Java: int authorityStart = -1;
         let mut authorityStart: i32 = -1;
         // Java: int schemeSpecificPartStart = 0;
         let mut schemeSpecificPartStart: usize = 0;
         // Java: int fileStart = -1;
         let mut fileStart: i32 = -1;
         // Java: int queryStart = -1;
         let mut queryStart: i32 = -1;
         // Java: int state = 0;
         let mut state = 0;
         // Java: for(int i = 0; i < fragmentStart; ++i)
         let mut i: usize = 0;
         while i < frag_start
         {
            // Java: switch (chars[i])
            match chars[i] {
               // Java: case '/':
               '/' => {
                  // Java: if (state < 2 && i + 1 < fragmentStart && chars[i + 1] == '/')
                  if state < 2 && i + 1 < frag_start && chars[i + 1] == '/'
                  {
                     // Java: if (i + 2 == len)
                     if i + 2 == len
                     {
                        // Java: throw new URISyntaxException(uri, "Authority expected", uri.length());
                        return Err("Authority expected".to_string());
                     }
                     // Java: start = i + 2;
                     start = i + 2;
                     // Java: authorityStart = start;
                     authorityStart = start as i32;
                     // Java: ++i;
                     i += 1;
                     // Java: state = 2;
                     state = 2;
                  }
                  // Java: else if (state <= 2)
                  else if state <= 2
                  {
                     // Java: if (state == 2)
                     if state == 2
                     {
                        // Java: this.authority = WpsUriUtils.authorityValidate(chars, authorityStart, i, "authority");
                        self.authority = Some(chars[authorityStart as usize..i].iter().collect());
                        // Java: fileStart = i;
                        fileStart = i as i32;
                     }
                     // Java: else
                     else
                     {
                        // Java: fileStart = start;
                        fileStart = start as i32;
                     }
                     // Java: state = 3;
                     state = 3;
                  }
               },
               // Java: break;
               // Java: case ':':
               ':' => {
                  // Java: if (state == 0)
                  if state == 0
                  {
                     // Java: start = i + 1;
                     start = i + 1;
                     // Java: if (start == fragmentStart)
                     if start == frag_start
                     {
                        // Java: throw new URISyntaxException(uri, "Scheme-specific part expected", start);
                        return Err("Scheme-specific part expected".to_string());
                     }
                     // Java: this.absolute = true;
                     self.absolute = true;
                     // Java: this.scheme = this.validateScheme(chars, 0, i);
                     self.scheme = Some(Self::validateScheme(&chars, 0, i)?);
                     // Java: schemeSpecificPartStart = start;
                     schemeSpecificPartStart = start;
                     // Java: if (chars[start] != '/')
                     if chars[start] != '/'
                     {
                        // Java: this.opaque = true;
                        self.opaque = true;
                        // Java: this.schemeSpecificPart = WpsUriUtils.commonValidate(chars, start, fragmentStart, "scheme specific part");
                        self.schemeSpecificPart = Some(chars[start..frag_start].iter().collect());
                        // Java: return;
                        return Ok(());
                     }
                     // Java: state = 1;
                     state = 1;
                  }
               },
               // Java: break;
               // Java: case '?':
               '?' => {
                  // Java: if (state < 4)
                  if state < 4
                  {
                     // Java: if (state == 2)
                     if state == 2
                     {
                        // Java: this.authority = WpsUriUtils.authorityValidate(chars, authorityStart, i, "authority");
                        self.authority = Some(chars[authorityStart as usize..i].iter().collect());
                        // Java: fileStart = i;
                        fileStart = i as i32;
                     }
                     // Java: else if (state < 2)
                     else if state < 2
                     {
                        // Java: fileStart = start;
                        fileStart = start as i32;
                     }
                     // Java: this.path = WpsUriUtils.pathValidate(chars, fileStart, i, "path");
                     self.path = Some(chars[fileStart as usize..i].iter().collect());
                     // Java: queryStart = i;
                     queryStart = i as i32;
                     // Java: state = 4;
                     state = 4;
                  }
               },
               _ => {}
            }
            i += 1;
         }
         // Java: this.schemeSpecificPart = new String(chars, schemeSpecificPartStart, fragmentStart - schemeSpecificPartStart);
         self.schemeSpecificPart = Some(chars[schemeSpecificPartStart..frag_start].iter().collect());
         // Java: if (state == 2)
         if state == 2
         {
            // Java: this.authority = WpsUriUtils.authorityValidate(chars, authorityStart, fragmentStart, "authority");
            self.authority = Some(chars[authorityStart as usize..frag_start].iter().collect());
            // Java: this.path = "";
            self.path = Some(String::new());
         }
         // Java: else if (state == 3)
         else if state == 3
         {
            // Java: this.path = WpsUriUtils.pathValidate(chars, fileStart, fragmentStart, "path");
            self.path = Some(chars[fileStart as usize..frag_start].iter().collect());
         }
         // Java: else if (state == 4)
         else if state == 4
         {
            // Java: this.query = WpsUriUtils.commonValidate(chars, queryStart + 1, fragmentStart, "query");
            self.query = Some(chars[(queryStart as usize + 1)..frag_start].iter().collect());
         }
         // Java: else
         else
         {
            // Java: this.path = WpsUriUtils.pathValidate(chars, start, fragmentStart, "path");
            self.path = Some(chars[start..frag_start].iter().collect());
         }
         // Java: this.parseAuthority();
         self.parseAuthority();
      }
      Ok(())
    }

    // Java: void parseAuthority()
    fn parseAuthority(&mut self) {
      // Java: if (this.authority != null)
      if let Some(ref auth) = self.authority.clone()
      {
         // Java: String tempUserInfo = null;
         let mut tempUserInfo: Option<String> = None;
         // Java: String temp = this.authority;
         let mut temp = auth.clone();
         // Java: int index = temp.indexOf(64);
         let mut index = temp.find('@').map(|i| i as i32).unwrap_or(-1);
         // Java: if (index != -1)
         if index != -1
         {
            // Java: tempUserInfo = temp.substring(0, index);
            tempUserInfo = Some(temp[..index as usize].to_string());
            // Java: this.validateUserInfo(this.authority, tempUserInfo, 0);
            let _ = Self::validateUserInfo_static(auth, tempUserInfo.as_deref().unwrap_or(""), 0);
            // Java: temp = temp.substring(index + 1);
            temp = temp[(index as usize + 1)..].to_string();
         }
         // Java: index = temp.lastIndexOf(58);
         index = temp.rfind(':').map(|i| i as i32).unwrap_or(-1);
         // Java: int endIndex = temp.indexOf(93);
         let endIndex = temp.find(']').map(|i| i as i32).unwrap_or(-1);
         // Java: int tempPort = -1;
         let mut tempPort: i32 = -1;
         // Java: String tempHost;
         let tempHost: String;
         // Java: if (index != -1 && endIndex < index)
         if index != -1 && endIndex < index
         {
            // Java: tempHost = temp.substring(0, index);
            tempHost = temp[..index as usize].to_string();
            // Java: if (index < temp.length() - 1)
            if (index as usize) < temp.len() - 1
            {
               // Java: try
               match temp[(index as usize + 1)..].parse::<i32>()
               {
                  Ok(p) => {
                     // Java: tempPort = Integer.parseInt(temp.substring(index + 1));
                     tempPort = p;
                     // Java: if (tempPort < 0)
                     if tempPort < 0
                     {
                        // Java: return;
                        return;
                     }
                  }
                  // Java: catch (NumberFormatException var8)
                  Err(_) =>
                  {
                     // Java: return;
                     return;
                  }
               }
            }
         }
         // Java: else
         else
         {
            // Java: tempHost = temp;
            tempHost = temp;
         }
         // Java: if (!tempHost.isEmpty())
         if !tempHost.is_empty()
         {
            // Java: if (this.isValidHost(tempHost))
            if self.isValidHost(&tempHost)
            {
               // Java: this.userInfo = tempUserInfo;
               self.userInfo = tempUserInfo;
               // Java: this.host = tempHost;
               self.host = Some(tempHost);
               // Java: this.port = tempPort;
               self.port = tempPort;
               // Java: this.serverAuthority = true;
               self.serverAuthority = true;
            }
         }
      }
    }

    // Java: void validateUserInfo(String uri, String userInfo, int index)
    fn validateUserInfo_static(uri: &str, user_info: &str, index: usize) -> Result<(), String> {
      // Java: int i = 0;
      let mut i = 0usize;
      // Java: for(int n = userInfo.length(); i < n; ++i)
      let n = user_info.chars().count();
      while i < n
      {
         let ch = user_info.chars().nth(i).unwrap();
         // Java: char ch = userInfo.charAt(i);
         // Java: if (ch == ']' || ch == '[')
         if ch == ']' || ch == '['
         {
            // Java: throw new URISyntaxException(uri, "Illegal character in userInfo", index + i);
            return Err(format!("Illegal character in userInfo at {}", index + i));
         }
         i += 1;
      }
      Ok(())
    }

    // Java: boolean isValidHost(String host)
    fn isValidHost(&self, host: &str) -> bool {
      // Java: if (host.startsWith("["))
      if host.starts_with('[')
      {
         // Java: if (!host.endsWith("]"))
         if !host.ends_with(']')
         {
            // Java: throw new URISyntaxException(host, "Expected a closing square bracket for IPv6 address", 0);
            return false;
         }
         // Java: else
         else
         {
            // Java: if (host.length() > 3)
            if host.len() > 3
            {
               // Java: char c = host.charAt(1);
               let c = host.chars().nth(1).unwrap_or('\0');
               // Java: if (c == ':' || '0' <= c && c <= '9' || 'A' <= c && c <= 'F' || 'a' <= c && c <= 'f')
               if c == ':' || (c >= '0' && c <= '9') || (c >= 'A' && c <= 'F') || (c >= 'a' && c <= 'f')
               {
                  // Java: try
                  {
                     // Java: InetAddress addr = InetAddress.getByName(host);
                     // Java: if (addr.getAddress().length == 16)
                     {
                        // Java: return true;
                        return true;
                     }
                  }
                  // Java: catch (UnknownHostException var4)
                  {
                  }
               }
            }
            // Java: throw new URISyntaxException(host, "Malformed IPv6 address");
            return false;
         }
      }
      // Java: else if (host.indexOf(91) == -1 && host.indexOf(93) == -1)
      else if !host.contains('[') && !host.contains(']')
      {
         // Java: int index = host.lastIndexOf(46);
         let index = host.rfind('.').map(|i| i as i32).unwrap_or(-1);
         // Java: if (index >= 0 && index != host.length() - 1 && Character.isDigit(host.charAt(index + 1)))
         if index >= 0 && (index as usize) != host.len() - 1 && host.chars().nth(index as usize + 1).map(|c| c.is_ascii_digit()).unwrap_or(false)
         {
            // Java: try
            {
               // Java: InetAddress addr = InetAddress.getByName(host);
               // Java: if (addr.getAddress().length == 4)
               {
                  // Java: return true;
                  return true;
               }
            }
            // Java: catch (UnknownHostException var5)
            {
            }
            // Java: return false;
            return false;
         }
         // Java: else
         else
         {
            // Java: return this.isValidDomainName(host);
            return self.isValidDomainName(host);
         }
      }
      // Java: else
      else
      {
         // Java: throw new URISyntaxException(host, "Illegal character in host name", 0);
         return false;
      }
    }

    // Java: static void validateHost(String s)
    pub fn validateHost(s: &str) -> Result<(), String> {
      // Java: int i = 0;
      let mut i = 0usize;
      // Java: for(int n = s.length(); i < n; ++i)
      let n = s.chars().count();
      while i < n
      {
         let ch = s.chars().nth(i).unwrap();
         // Java: char ch = s.charAt(i);
         // Java: if (ch > 127 || WpsUriUtils.ASCII_FLAG_TABLE[ch] != 5 && ch != '-' && ch != '.')
         if ch as u32 > 127 || (ch != '-' && ch != '.' && !ch.is_ascii_alphanumeric())
         {
            // Java: throw new URISyntaxException(s, "Illegal character", i);
            return Err(format!("Illegal character at {}", i));
         }
         i += 1;
      }
      Ok(())
    }

    // Java: boolean isValidDomainName(String host)
    fn isValidDomainName(&self, host: &str) -> bool {
      // Java: try
      // Java: validateHost(host);
      if Self::validateHost(host).is_err()
      // Java: catch (URISyntaxException var7)
      {
         // Java: return false;
         return false;
      }
      // Java: String lastLabel = null;
      let mut lastLabel: Option<&str> = None;
      // Java: for(String token : host.split("\\."))
      for token in host.split('.')
      {
         // Java: lastLabel = token;
         lastLabel = Some(token);
         // Java: if (token.startsWith("-") || token.endsWith("-"))
         if token.starts_with('-') || token.ends_with('-')
         {
            // Java: return false;
            return false;
         }
      }
      // Java: if (lastLabel == null)
      if lastLabel.is_none()
      {
         // Java: return false;
         return false;
      }
      // Java: else
      {
         // Java: if (!lastLabel.equals(host))
         if lastLabel.unwrap() != host
         {
            // Java: char ch = lastLabel.charAt(0);
            let ch = lastLabel.unwrap().chars().next().unwrap_or('\0');
            // Java: if (ch >= '0' && ch <= '9')
            if ch >= '0' && ch <= '9'
            {
               // Java: return false;
               return false;
            }
         }
         // Java: return true;
         return true;
      }
    }

    // Java: int compareTo(URI uri)
    pub fn compareTo(&self, uri: &URI) -> i32 {
      // Java: if (this.scheme == null && uri.scheme != null)
      if self.scheme.is_none() && uri.scheme.is_some()
      {
         // Java: return -1;
         return -1;
      }
      // Java: else if (this.scheme != null && uri.scheme == null)
      else if self.scheme.is_some() && uri.scheme.is_none()
      {
         // Java: return 1;
         return 1;
      }
      // Java: else
      else
      {
         // Java: if (this.scheme != null && uri.scheme != null)
         if let (Some(ref s1), Some(ref s2)) = (&self.scheme, &uri.scheme)
         {
            // Java: int ret = this.scheme.compareToIgnoreCase(uri.scheme);
            let ret = s1.to_lowercase().cmp(&s2.to_lowercase()) as i32;
            // Java: if (ret != 0)
            if ret != 0
            {
               // Java: return ret;
               return ret;
            }
         }
         // Java: if (!this.opaque && uri.opaque)
         if !self.opaque && uri.opaque
         {
            // Java: return -1;
            return -1;
         }
         // Java: else if (this.opaque && !uri.opaque)
         else if self.opaque && !uri.opaque
         {
            // Java: return 1;
            return 1;
         }
         // Java: else
         else
         {
            // Java: if (this.opaque && uri.opaque)
            if self.opaque && uri.opaque
            {
               // Java: int ret = this.schemeSpecificPart.compareTo(uri.schemeSpecificPart);
               let ret = self.schemeSpecificPart.as_deref().unwrap_or("").cmp(uri.schemeSpecificPart.as_deref().unwrap_or("")) as i32;
               // Java: if (ret != 0)
               if ret != 0
               {
                  // Java: return ret;
                  return ret;
               }
            }
            // Java: else
            else
            {
               // Java: if (this.authority != null && uri.authority == null)
               if self.authority.is_some() && uri.authority.is_none()
               {
                  // Java: return 1;
                  return 1;
               }
               // Java: if (this.authority == null && uri.authority != null)
               if self.authority.is_none() && uri.authority.is_some()
               {
                  // Java: return -1;
                  return -1;
               }
               // Java: if (this.authority != null && uri.authority != null)
               if self.authority.is_some() && uri.authority.is_some()
               {
                  // Java: if (this.host != null && uri.host != null)
                  if self.host.is_some() && uri.host.is_some()
                  {
                     // Java: if (this.userInfo != null && uri.userInfo == null)
                     if self.userInfo.is_some() && uri.userInfo.is_none()
                     {
                        // Java: return 1;
                        return 1;
                     }
                     // Java: if (this.userInfo == null && uri.userInfo != null)
                     if self.userInfo.is_none() && uri.userInfo.is_some()
                     {
                        // Java: return -1;
                        return -1;
                     }
                     // Java: if (this.userInfo != null && uri.userInfo != null)
                     if let (Some(ref u1), Some(ref u2)) = (&self.userInfo, &uri.userInfo)
                     {
                        // Java: int ret = this.userInfo.compareTo(uri.userInfo);
                        let ret = u1.cmp(u2) as i32;
                        // Java: if (ret != 0)
                        if ret != 0
                        {
                           // Java: return ret;
                           return ret;
                        }
                     }
                     // Java: int ret = this.host.compareToIgnoreCase(uri.host);
                     let ret = self.host.as_deref().unwrap_or("").to_lowercase().cmp(&uri.host.as_deref().unwrap_or("").to_lowercase()) as i32;
                     // Java: if (ret != 0)
                     if ret != 0
                     {
                        // Java: return ret;
                        return ret;
                     }
                     // Java: if (this.port != uri.port)
                     if self.port != uri.port
                     {
                        // Java: return this.port - uri.port;
                        return self.port - uri.port;
                     }
                  }
                  // Java: else
                  else
                  {
                     // Java: int ret = this.authority.compareTo(uri.authority);
                     let ret = self.authority.as_deref().unwrap_or("").cmp(uri.authority.as_deref().unwrap_or("")) as i32;
                     // Java: if (ret != 0)
                     if ret != 0
                     {
                        // Java: return ret;
                        return ret;
                     }
                  }
               }
               // Java: int ret = this.path.compareTo(uri.path);
               let ret = self.path.as_deref().unwrap_or("").cmp(uri.path.as_deref().unwrap_or("")) as i32;
               // Java: if (ret != 0)
               if ret != 0
               {
                  // Java: return ret;
                  return ret;
               }
               // Java: if (this.query != null && uri.query == null)
               if self.query.is_some() && uri.query.is_none()
               {
                  // Java: return 1;
                  return 1;
               }
               // Java: if (this.query == null && uri.query != null)
               if self.query.is_none() && uri.query.is_some()
               {
                  // Java: return -1;
                  return -1;
               }
               // Java: if (this.query != null && uri.query != null)
               if let (Some(ref q1), Some(ref q2)) = (&self.query, &uri.query)
               {
                  // Java: ret = this.query.compareTo(uri.query);
                  let ret = q1.cmp(q2) as i32;
                  // Java: if (ret != 0)
                  if ret != 0
                  {
                     // Java: return ret;
                     return ret;
                  }
               }
            }
            // Java: if (this.fragment != null && uri.fragment == null)
            if self.fragment.is_some() && uri.fragment.is_none()
            {
               // Java: return 1;
               return 1;
            }
            // Java: else if (this.fragment == null && uri.fragment != null)
            else if self.fragment.is_none() && uri.fragment.is_some()
            {
               // Java: return -1;
               return -1;
            }
            // Java: else
            else
            {
               // Java: if (this.fragment != null && uri.fragment != null)
               if let (Some(ref f1), Some(ref f2)) = (&self.fragment, &uri.fragment)
               {
                  // Java: int var9 = this.fragment.compareTo(uri.fragment);
                  let var9 = f1.cmp(f2) as i32;
                  // Java: if (var9 != 0)
                  if var9 != 0
                  {
                     // Java: return var9;
                     return var9;
                  }
               }
               // Java: return 0;
               return 0;
            }
         }
      }
    }

    // Java: URI duplicate()
    fn duplicate(&self) -> URI {
      // Java: URI clone = new URI();
      let mut clone = URI::new_empty();
      // Java: clone.absolute = this.absolute;
      clone.absolute = self.absolute;
      // Java: clone.authority = this.authority;
      clone.authority = self.authority.clone();
      // Java: clone.fragment = this.fragment;
      clone.fragment = self.fragment.clone();
      // Java: clone.host = this.host;
      clone.host = self.host.clone();
      // Java: clone.opaque = this.opaque;
      clone.opaque = self.opaque;
      // Java: clone.path = this.path;
      clone.path = self.path.clone();
      // Java: clone.port = this.port;
      clone.port = self.port;
      // Java: clone.query = this.query;
      clone.query = self.query.clone();
      // Java: clone.scheme = this.scheme;
      clone.scheme = self.scheme.clone();
      // Java: clone.schemeSpecificPart = this.schemeSpecificPart;
      clone.schemeSpecificPart = self.schemeSpecificPart.clone();
      // Java: clone.userInfo = this.userInfo;
      clone.userInfo = self.userInfo.clone();
      // Java: clone.serverAuthority = this.serverAuthority;
      clone.serverAuthority = self.serverAuthority;
      // Java: return clone;
      clone
    }

    // Java: static void convertHexToLowerCase(char[] chars, int start, int end)
    fn convertHexToLowerCase(chars: &mut Vec<char>, mut start: usize, end: usize) {
      // Java: for(; start < end; ++start)
      while start < end
      {
         // Java: if (chars[start] == '%' && start + 1 < end)
         if chars[start] == '%' && start + 1 < end
         {
            // Java: char c = chars[start + 1];
            let c = chars[start + 1];
            // Java: if ('A' <= c && c <= 'Z')
            if c >= 'A' && c <= 'Z'
            {
               // Java: chars[start + 1] = (char)(c + 97 - 65);
               chars[start + 1] = (c as u8 + 97 - 65) as char;
            }
            // Java: ++start;
            start += 1;
            // Java: if (start + 1 < end)
            if start + 1 < end
            {
               // Java: c = chars[start + 1];
               let c = chars[start + 1];
               // Java: if (c != '%')
               if c != '%'
               {
                  // Java: if ('A' <= c && c <= 'Z')
                  if c >= 'A' && c <= 'Z'
                  {
                     // Java: chars[start + 1] = (char)(c + 97 - 65);
                     chars[start + 1] = (c as u8 + 97 - 65) as char;
                  }
                  // Java: ++start;
                  start += 1;
               }
            }
         }
         start += 1;
      }
    }

    // Java: String getScheme()
    pub fn getScheme(&self) -> Option<&str> {
      // Java: return this.scheme;
      self.scheme.as_deref()
    }

    // Java: String getSchemeSpecificPart()
    pub fn getSchemeSpecificPart(&self) -> Option<String> {
      // Java: return this.decode(this.schemeSpecificPart);
      self.decode(&self.schemeSpecificPart)
    }

    // Java: String getRawSchemeSpecificPart()
    pub fn getRawSchemeSpecificPart(&self) -> Option<&str> {
      // Java: return this.schemeSpecificPart;
      self.schemeSpecificPart.as_deref()
    }

    // Java: String getAuthority()
    pub fn getAuthority(&self) -> Option<String> {
      // Java: return this.decode(this.authority);
      self.decode(&self.authority)
    }

    // Java: String getRawAuthority()
    pub fn getRawAuthority(&self) -> Option<&str> {
      // Java: return this.authority;
      self.authority.as_deref()
    }

    // Java: String getUserInfo()
    pub fn getUserInfo(&self) -> Option<String> {
      // Java: return this.decode(this.userInfo);
      self.decode(&self.userInfo)
    }

    // Java: String getRawUserInfo()
    pub fn getRawUserInfo(&self) -> Option<&str> {
      // Java: return this.userInfo;
      self.userInfo.as_deref()
    }

    // Java: String getHost()
    pub fn getHost(&self) -> Option<&str> {
      // Java: return this.host;
      self.host.as_deref()
    }

    // Java: int getPort()
    pub fn getPort(&self) -> i32 {
      // Java: return this.port;
      self.port
    }

    // Java: int getEffectivePort()
    pub fn getEffectivePort_instance(&self) -> i32 {
      // Java: return getEffectivePort(this.scheme, this.port);
      Self::getEffectivePort(self.scheme.as_deref(), self.port)
    }

    // Java: static int getEffectivePort(String scheme, int specifiedPort)
    pub fn getEffectivePort(scheme: Option<&str>, specifiedPort: i32) -> i32 {
      // Java: if (specifiedPort != -1)
      if specifiedPort != -1
      {
         // Java: return specifiedPort;
         return specifiedPort;
      }
      // Java: else if ("http".equalsIgnoreCase(scheme))
      if scheme.map(|s| s.eq_ignore_ascii_case("http")).unwrap_or(false)
      {
         // Java: return 80;
         return 80;
      }
      // Java: else
      {
         // Java: return "https".equalsIgnoreCase(scheme) ? 443 : -1;
         if scheme.map(|s| s.eq_ignore_ascii_case("https")).unwrap_or(false) { 443 } else { -1 }
      }
    }

    // Java: String getPath()
    pub fn getPath(&self) -> Option<String> {
      // Java: return this.decode(this.path);
      self.decode(&self.path)
    }

    // Java: String getRawPath()
    pub fn getRawPath(&self) -> Option<&str> {
      // Java: return this.path;
      self.path.as_deref()
    }

    // Java: String getQuery()
    pub fn getQuery(&self) -> Option<String> {
      // Java: return this.decode(this.query);
      self.decode(&self.query)
    }

    // Java: String getRawQuery()
    pub fn getRawQuery(&self) -> Option<&str> {
      // Java: return this.query;
      self.query.as_deref()
    }

    // Java: String getFragment()
    pub fn getFragment(&self) -> Option<String> {
      // Java: return this.decode(this.fragment);
      self.decode(&self.fragment)
    }

    // Java: String getRawFragment()
    pub fn getRawFragment(&self) -> Option<&str> {
      // Java: return this.fragment;
      self.fragment.as_deref()
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: if (this.hash == -1)
      if self.hash == -1
      {
         // Java: this.hash = this.getHashString().hashCode();
         let hs = self.getHashString();
         let mut hasher = DefaultHasher::new();
         hs.hash(&mut hasher);
         self.hash = hasher.finish() as i32;
      }
      // Java: return this.hash;
      self.hash
    }

    // Java: boolean isAbsolute()
    pub fn isAbsolute(&self) -> bool {
      // Java: return this.absolute;
      self.absolute
    }

    // Java: boolean isOpaque()
    pub fn isOpaque(&self) -> bool {
      // Java: return this.opaque;
      self.opaque
    }

    // Java: static String canonicalizePath(String path, boolean discardRelativePrefix)
    pub fn canonicalizePath(path_in: &str, discardRelativePrefix: bool) -> String {
      // Java: int segmentStart = 0;
      let mut segmentStart: usize = 0;
      // Java: int deletableSegments = 0;
      let mut deletableSegments: i32 = 0;
      // Java: int i = 0;
      let mut i: usize = 0;
      let mut path = path_in.to_string();
      // Java: while(i <= path.length())
      while i <= path.len()
      {
         // Java: int nextSegmentStart;
         let nextSegmentStart: usize;
         // Java: if (i == path.length())
         if i == path.len()
         {
            // Java: nextSegmentStart = i;
            nextSegmentStart = i;
         }
         // Java: else
         else
         {
            // Java: if (path.charAt(i) != '/')
            if path.chars().nth(i).unwrap_or('\0') != '/'
            {
               // Java: ++i;
               i += 1;
               // Java: continue;
               continue;
            }
            // Java: nextSegmentStart = i + 1;
            nextSegmentStart = i + 1;
         }
         // Java: if (i == segmentStart + 1 && path.regionMatches(segmentStart, ".", 0, 1))
         if i == segmentStart + 1 && path.get(segmentStart..segmentStart + 1) == Some(".")
         {
            // Java: path = path.substring(0, segmentStart) + path.substring(nextSegmentStart);
            path = format!("{}{}", &path[..segmentStart], &path[nextSegmentStart..]);
            // Java: i = segmentStart;
            i = segmentStart;
         }
         // Java: else if (i == segmentStart + 2 && path.regionMatches(segmentStart, "..", 0, 2))
         else if i == segmentStart + 2 && path.get(segmentStart..segmentStart + 2) == Some("..")
         {
            // Java: if (deletableSegments <= 0 && !discardRelativePrefix)
            if deletableSegments <= 0 && !discardRelativePrefix
            {
               // Java: ++i;
               i += 1;
               // Java: segmentStart = i;
               segmentStart = i;
            }
            // Java: else
            else
            {
               // Java: --deletableSegments;
               deletableSegments -= 1;
               // Java: int prevSegmentStart = path.lastIndexOf(47, segmentStart - 2) + 1;
               let prevSegmentStart = if segmentStart >= 2 {
                   path[..segmentStart - 1].rfind('/').map(|p| p + 1).unwrap_or(0)
               } else { 0 };
               // Java: path = path.substring(0, prevSegmentStart) + path.substring(nextSegmentStart);
               path = format!("{}{}", &path[..prevSegmentStart], &path[nextSegmentStart..]);
               // Java: segmentStart = prevSegmentStart;
               segmentStart = prevSegmentStart;
               // Java: i = prevSegmentStart;
               i = prevSegmentStart;
            }
         }
         // Java: else
         else
         {
            // Java: if (i > 0)
            if i > 0
            {
               // Java: ++deletableSegments;
               deletableSegments += 1;
            }
            // Java: ++i;
            i += 1;
            // Java: segmentStart = i;
            segmentStart = i;
         }
      }
      // Java: return path;
      path
    }

    // Java: String normalize(String path, boolean discardRelativePrefix)
    fn normalize_path(&self, path_in: &str, discardRelativePrefix: bool) -> String {
      // Java: path = canonicalizePath(path, discardRelativePrefix);
      let mut path = Self::canonicalizePath(path_in, discardRelativePrefix);
      // Java: int colon = path.indexOf(58);
      let colon = path.find(':');
      // Java: if (colon != -1)
      if let Some(colon_pos) = colon
      {
         // Java: int slash = path.indexOf(47);
         let slash = path.find('/');
         // Java: if (slash == -1 || colon < slash)
         if slash.is_none() || colon_pos < slash.unwrap()
         {
            // Java: path = "./" + path;
            path = format!("./{}", path);
         }
      }
      // Java: return path;
      path
    }

    // Java: URI normalize()
    pub fn normalize(&self) -> URI {
      // Java: if (this.opaque)
      if self.opaque
      {
         // Java: return this;
         return self.clone();
      }
      // Java: else
      else
      {
         // Java: String normalizedPath = this.normalize(this.path, false);
         let normalizedPath = self.normalize_path(self.path.as_deref().unwrap_or(""), false);
         // Java: if (this.path.equals(normalizedPath))
         if self.path.as_deref().unwrap_or("") == normalizedPath
         {
            // Java: return this;
            return self.clone();
         }
         // Java: else
         else
         {
            // Java: URI result = this.duplicate();
            let mut result = self.duplicate();
            // Java: result.path = normalizedPath;
            result.path = Some(normalizedPath);
            // Java: result.setSchemeSpecificPart();
            result.setSchemeSpecificPart();
            // Java: return result;
            result
         }
      }
    }

    // Java: URI resolve(URI relative)
    pub fn resolve(&self, relative: &URI) -> URI {
      // Java: if (!relative.absolute && !this.opaque)
      if !relative.absolute && !self.opaque
      {
         // Java: if (relative.authority != null)
         if relative.authority.is_some()
         {
            // Java: URI result = relative.duplicate();
            let mut result = relative.duplicate();
            // Java: result.scheme = this.scheme;
            result.scheme = self.scheme.clone();
            // Java: result.absolute = this.absolute;
            result.absolute = self.absolute;
            // Java: return result;
            return result;
         }
         // Java: else if (relative.path.isEmpty() && relative.scheme == null && relative.query == null)
         else if relative.path.as_deref().unwrap_or("").is_empty() && relative.scheme.is_none() && relative.query.is_none()
         {
            // Java: URI result = this.duplicate();
            let mut result = self.duplicate();
            // Java: result.fragment = relative.fragment;
            result.fragment = relative.fragment.clone();
            // Java: return result;
            return result;
         }
         // Java: else
         else
         {
            // Java: URI result = this.duplicate();
            let mut result = self.duplicate();
            // Java: result.fragment = relative.fragment;
            result.fragment = relative.fragment.clone();
            // Java: result.query = relative.query;
            result.query = relative.query.clone();
            // Java: String resolvedPath;
            let resolvedPath: String;
            // Java: if (relative.path.startsWith("/"))
            if relative.path.as_deref().unwrap_or("").starts_with('/')
            {
               // Java: resolvedPath = relative.path;
               resolvedPath = relative.path.clone().unwrap_or_default();
            }
            // Java: else if (relative.path.isEmpty())
            else if relative.path.as_deref().unwrap_or("").is_empty()
            {
               // Java: resolvedPath = this.path;
               resolvedPath = self.path.clone().unwrap_or_default();
            }
            // Java: else
            else
            {
               // Java: int endIndex = this.path.lastIndexOf(47) + 1;
               let self_path = self.path.as_deref().unwrap_or("");
               let endIndex = self_path.rfind('/').map(|i| i + 1).unwrap_or(0);
               // Java: resolvedPath = this.path.substring(0, endIndex) + relative.path;
               resolvedPath = format!("{}{}", &self_path[..endIndex], relative.path.as_deref().unwrap_or(""));
            }
            // Java: result.path = WpsUriUtils.authoritySafePath(result.authority, this.normalize(resolvedPath, true));
            result.path = Some(self.normalize_path(&resolvedPath, true));
            // Java: result.setSchemeSpecificPart();
            result.setSchemeSpecificPart();
            // Java: return result;
            return result;
         }
      }
      // Java: else
      {
         // Java: return relative;
         return relative.clone();
      }
    }

    // Java: void setSchemeSpecificPart()
    fn setSchemeSpecificPart(&mut self) {
      // Java: int lenAuthority = this.authority != null ? this.authority.length() : 0;
      let lenAuthority = self.authority.as_ref().map(|s| s.len()).unwrap_or(0);
      // Java: int lenPath = this.path != null ? this.path.length() : 0;
      let lenPath = self.path.as_ref().map(|s| s.len()).unwrap_or(0);
      // Java: int lenQuery = this.query != null ? this.query.length() : 0;
      let lenQuery = self.query.as_ref().map(|s| s.len()).unwrap_or(0);
      // Java: int len = lenAuthority + lenPath + lenQuery;
      let mut len = lenAuthority + lenPath + lenQuery;
      // Java: if (len == 0)
      if len == 0
      {
         // Java: this.schemeSpecificPart = "";
         self.schemeSpecificPart = Some(String::new());
         // Java: this.string = null;
         self.string = None;
      }
      // Java: else
      else
      {
         // Java: char[] chars = new char[len + 3];
         let mut chars = String::with_capacity(len + 3);
         // Java: len = 0;
         len = 0;
         // Java: if (lenAuthority != 0)
         if lenAuthority != 0
         {
            // Java: chars[0] = '/';
            // Java: chars[1] = '/';
            chars.push_str("//");
            // Java: this.authority.getChars(0, lenAuthority, chars, 2);
            chars.push_str(self.authority.as_deref().unwrap_or(""));
            // Java: len = 2 + lenAuthority;
            len = 2 + lenAuthority;
         }
         // Java: if (lenPath != 0)
         if lenPath != 0
         {
            // Java: this.path.getChars(0, lenPath, chars, len);
            chars.push_str(self.path.as_deref().unwrap_or(""));
            // Java: len += lenPath;
            len += lenPath;
         }
         // Java: if (lenQuery != 0)
         if lenQuery != 0
         {
            // Java: chars[len++] = '?';
            chars.push('?');
            // Java: this.query.getChars(0, lenQuery, chars, len);
            chars.push_str(self.query.as_deref().unwrap_or(""));
            // Java: len += lenQuery;
            len += lenQuery;
         }
         // Java: this.schemeSpecificPart = new String(chars, 0, len);
         self.schemeSpecificPart = Some(chars);
         // Java: this.string = null;
         self.string = None;
      }
    }

    // Java: String decode(String s)
    fn decode(&self, s: &Option<String>) -> Option<String> {
      // Java: return s != null ? WpsUriUtils.decode(s) : null;
      s.as_ref().map(|v| v.clone())
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: if (this.string != null)
      if let Some(ref s) = self.string
      {
         // Java: return this.string;
         return s.clone();
      }
      // Java: else
      {
         // Java: int lenScheme = this.scheme != null ? this.scheme.length() : 0;
         let lenScheme = self.scheme.as_ref().map(|s| s.len()).unwrap_or(0);
         // Java: int lenFragment = this.fragment != null ? this.fragment.length() : 0;
         let lenFragment = self.fragment.as_ref().map(|s| s.len()).unwrap_or(0);
         // Java: if (this.opaque)
         if self.opaque
         {
            // Java: int len = lenScheme + lenFragment;
            let mut len = lenScheme + lenFragment;
            // Java: if (len == 0)
            if len == 0
            {
               // Java: this.string = this.schemeSpecificPart != null ? this.schemeSpecificPart : "";
               let s = self.schemeSpecificPart.clone().unwrap_or_default();
               // Java: return this.string;
               return s;
            }
            // Java: else
            else
            {
               // Java: int lenSchemeSpecificPart = this.schemeSpecificPart != null ? this.schemeSpecificPart.length() : 0;
               let lenSchemeSpecificPart = self.schemeSpecificPart.as_ref().map(|s| s.len()).unwrap_or(0);
               // Java: len += lenSchemeSpecificPart;
               len += lenSchemeSpecificPart;
               // Java: char[] chars = new char[len + 2];
               let mut chars = String::with_capacity(len + lenSchemeSpecificPart + 2);
               // Java: len = 0;
               len = 0;
               // Java: if (lenScheme != 0)
               if lenScheme != 0
               {
                  // Java: this.scheme.getChars(0, lenScheme, chars, 0);
                  chars.push_str(self.scheme.as_deref().unwrap_or(""));
                  // Java: chars[lenScheme] = ':';
                  chars.push(':');
                  // Java: len = lenScheme + 1;
                  len = lenScheme + 1;
               }
               // Java: if (lenSchemeSpecificPart != 0)
               if lenSchemeSpecificPart != 0
               {
                  // Java: this.schemeSpecificPart.getChars(0, lenSchemeSpecificPart, chars, len);
                  chars.push_str(self.schemeSpecificPart.as_deref().unwrap_or(""));
                  // Java: len += lenSchemeSpecificPart;
                  len += lenSchemeSpecificPart;
               }
               // Java: if (lenFragment != 0)
               if lenFragment != 0
               {
                  // Java: chars[len++] = '#';
                  chars.push('#');
                  // Java: this.fragment.getChars(0, lenFragment, chars, len);
                  chars.push_str(self.fragment.as_deref().unwrap_or(""));
                  // Java: len += lenFragment;
                  len += lenFragment;
               }
               // Java: this.string = new String(chars, 0, len);
               // Java: return this.string;
               return chars;
            }
         }
         // Java: else
         else
         {
            // Java: int lenAuthority = this.authority != null ? this.authority.length() : 0;
            let lenAuthority = self.authority.as_ref().map(|s| s.len()).unwrap_or(0);
            // Java: int lenPath = this.path != null ? this.path.length() : 0;
            let lenPath = self.path.as_ref().map(|s| s.len()).unwrap_or(0);
            // Java: int lenQuery = this.query != null ? this.query.length() : 0;
            let lenQuery = self.query.as_ref().map(|s| s.len()).unwrap_or(0);
            // Java: int len = lenScheme + lenAuthority + lenPath + lenQuery + lenFragment;
            let mut len = lenScheme + lenAuthority + lenPath + lenQuery + lenFragment;
            // Java: if (len == 0)
            if len == 0
            {
               // Java: this.string = "";
               // Java: return this.string;
               return String::new();
            }
            // Java: else
            else
            {
               // Java: char[] chars = new char[len + 5];
               let mut chars = String::with_capacity(len + 5);
               // Java: len = 0;
               len = 0;
               // Java: if (lenScheme != 0)
               if lenScheme != 0
               {
                  // Java: this.scheme.getChars(0, lenScheme, chars, 0);
                  chars.push_str(self.scheme.as_deref().unwrap_or(""));
                  // Java: chars[lenScheme] = ':';
                  chars.push(':');
                  // Java: len = lenScheme + 1;
                  len = lenScheme + 1;
               }
               // Java: if (lenAuthority != 0)
               if lenAuthority != 0
               {
                  // Java: chars[len++] = '/';
                  chars.push('/');
                  // Java: chars[len++] = '/';
                  chars.push('/');
                  // Java: this.authority.getChars(0, lenAuthority, chars, len);
                  chars.push_str(self.authority.as_deref().unwrap_or(""));
                  // Java: len += lenAuthority;
                  len += lenAuthority;
               }
               // Java: if (lenPath != 0)
               if lenPath != 0
               {
                  // Java: this.path.getChars(0, lenPath, chars, len);
                  chars.push_str(self.path.as_deref().unwrap_or(""));
                  // Java: len += lenPath;
                  len += lenPath;
               }
               // Java: if (lenQuery != 0)
               if lenQuery != 0
               {
                  // Java: chars[len++] = '?';
                  chars.push('?');
                  // Java: this.query.getChars(0, lenQuery, chars, len);
                  chars.push_str(self.query.as_deref().unwrap_or(""));
                  // Java: len += lenQuery;
                  len += lenQuery;
               }
               // Java: if (lenFragment != 0)
               if lenFragment != 0
               {
                  // Java: chars[len++] = '#';
                  chars.push('#');
                  // Java: this.fragment.getChars(0, lenFragment, chars, len);
                  chars.push_str(self.fragment.as_deref().unwrap_or(""));
                  // Java: len += lenFragment;
                  len += lenFragment;
               }
               // Java: this.string = new String(chars, 0, len);
               // Java: return this.string;
               return chars;
            }
         }
      }
    }

    // Java: String getHashString()
    fn getHashString(&self) -> String {
      // Java: int lenScheme = this.scheme != null ? this.scheme.length() : 0;
      let lenScheme = self.scheme.as_ref().map(|s| s.len()).unwrap_or(0);
      // Java: int lenFragment = this.fragment != null ? this.fragment.length() : 0;
      let lenFragment = self.fragment.as_ref().map(|s| s.len()).unwrap_or(0);
      // Java: if (this.opaque)
      if self.opaque
      {
         // Java: int lenSchemeSpecificPart = this.schemeSpecificPart != null ? this.schemeSpecificPart.length() : 0;
         let lenSchemeSpecificPart = self.schemeSpecificPart.as_ref().map(|s| s.len()).unwrap_or(0);
         // Java: int len = lenScheme + lenFragment + lenSchemeSpecificPart;
         let mut len = lenScheme + lenFragment + lenSchemeSpecificPart;
         // Java: if (len == 0)
         if len == 0
         {
            // Java: return "";
            return String::new();
         }
         // Java: else
         else
         {
            // Java: char[] chars = new char[len + 2];
            let mut chars: Vec<char> = Vec::with_capacity(len + 2);
            // Java: len = 0;
            len = 0;
            // Java: if (lenScheme != 0)
            if lenScheme != 0
            {
               // Java: this.scheme.getChars(0, lenScheme, chars, 0);
               let scheme_chars: Vec<char> = self.scheme.as_deref().unwrap_or("").chars().collect();
               chars.extend_from_slice(&scheme_chars);
               // Java: WpsUriUtils.toLowerCase(chars, 0, lenScheme, Locale.US);
               for i in 0..chars.len() {
                   chars[i] = chars[i].to_ascii_lowercase();
               }
               // Java: chars[lenScheme] = ':';
               chars.push(':');
               // Java: len = lenScheme + 1;
               len = lenScheme + 1;
            }
            // Java: if (lenSchemeSpecificPart != 0)
            if lenSchemeSpecificPart != 0
            {
               // Java: this.schemeSpecificPart.getChars(0, lenSchemeSpecificPart, chars, len);
               chars.extend(self.schemeSpecificPart.as_deref().unwrap_or("").chars());
               // Java: len += lenSchemeSpecificPart;
               len += lenSchemeSpecificPart;
            }
            // Java: if (lenFragment != 0)
            if lenFragment != 0
            {
               // Java: chars[len++] = '#';
               chars.push('#');
               // Java: this.fragment.getChars(0, lenFragment, chars, len);
               chars.extend(self.fragment.as_deref().unwrap_or("").chars());
               // Java: len += lenFragment;
               len += lenFragment;
            }
            // Java: convertHexToLowerCase(chars, 0, len);
            let clen = chars.len();
            Self::convertHexToLowerCase(&mut chars, 0, clen);
            // Java: return new String(chars, 0, len);
            return chars.into_iter().collect();
         }
      }
      // Java: else
      else
      {
         // Java: int lenAuthority = this.authority != null ? this.authority.length() : 0;
         let lenAuthority = self.authority.as_ref().map(|s| s.len()).unwrap_or(0);
         // Java: int lenPath = this.path != null ? this.path.length() : 0;
         let lenPath = self.path.as_ref().map(|s| s.len()).unwrap_or(0);
         // Java: int lenQuery = this.query != null ? this.query.length() : 0;
         let lenQuery = self.query.as_ref().map(|s| s.len()).unwrap_or(0);
         // Java: int len = lenScheme + lenAuthority + lenPath + lenQuery + lenFragment;
         let mut len = lenScheme + lenAuthority + lenPath + lenQuery + lenFragment;
         // Java: if (len == 0)
         if len == 0
         {
            // Java: return "";
            return String::new();
         }
         // Java: else
         else
         {
            // Java: char[] chars = new char[len + 5];
            let mut chars: Vec<char> = Vec::with_capacity(len + 5);
            // Java: len = 0;
            let mut current_len: usize = 0;
            // Java: if (lenScheme != 0)
            if lenScheme != 0
            {
               // Java: this.scheme.getChars(0, lenScheme, chars, 0);
               let scheme_chars: Vec<char> = self.scheme.as_deref().unwrap_or("").chars().collect();
               chars.extend_from_slice(&scheme_chars);
               // Java: WpsUriUtils.toLowerCase(chars, 0, lenScheme, Locale.US);
               for i in 0..chars.len() {
                   chars[i] = chars[i].to_ascii_lowercase();
               }
               // Java: chars[lenScheme] = ':';
               chars.push(':');
               // Java: len = lenScheme + 1;
               current_len = lenScheme + 1;
            }
            // Java: if (lenAuthority != 0)
            if lenAuthority != 0
            {
               // Java: chars[len++] = '/';
               chars.push('/');
               // Java: chars[len++] = '/';
               chars.push('/');
               // Java: this.authority.getChars(0, lenAuthority, chars, len);
               let auth_start = chars.len();
               chars.extend(self.authority.as_deref().unwrap_or("").chars());
               // Java: if (this.host != null)
               if let Some(ref host) = self.host
               {
                  // Java: int lenHost = this.host.length();
                  let lenHost = host.len();
                  // Java: if (lenHost != 0)
                  if lenHost != 0
                  {
                     // Java: int pos = (this.userInfo != null && !this.userInfo.isEmpty() ? this.userInfo.length() + 1 : 0) + len;
                     let userInfoOffset = self.userInfo.as_ref().filter(|u| !u.is_empty()).map(|u| u.len() + 1).unwrap_or(0);
                     let pos = auth_start + userInfoOffset;
                     // Java: WpsUriUtils.toLowerCase(chars, pos, pos + lenHost, Locale.US);
                     for j in pos..(pos + lenHost).min(chars.len()) {
                         chars[j] = chars[j].to_ascii_lowercase();
                     }
                  }
               }
               // Java: len += lenAuthority;
               current_len = chars.len();
            }
            // Java: if (lenPath != 0)
            if lenPath != 0
            {
               // Java: this.path.getChars(0, lenPath, chars, len);
               chars.extend(self.path.as_deref().unwrap_or("").chars());
               // Java: len += lenPath;
               len += lenPath;
            }
            // Java: if (lenQuery != 0)
            if lenQuery != 0
            {
               // Java: chars[len++] = '?';
               chars.push('?');
               // Java: this.query.getChars(0, lenQuery, chars, len);
               chars.extend(self.query.as_deref().unwrap_or("").chars());
               // Java: len += lenQuery;
               len += lenQuery;
            }
            // Java: if (lenFragment != 0)
            if lenFragment != 0
            {
               // Java: chars[len++] = '#';
               chars.push('#');
               // Java: this.fragment.getChars(0, lenFragment, chars, len);
               chars.extend(self.fragment.as_deref().unwrap_or("").chars());
               // Java: len += lenFragment;
               len += lenFragment;
            }
            // Java: convertHexToLowerCase(chars, 0, len);
            let total = chars.len();
            Self::convertHexToLowerCase(&mut chars, 0, total);
            // Java: return new String(chars, 0, len);
            return chars.into_iter().collect();
         }
      }
    }

}

fn is_valid_hex_chars(c1: char, c2: char) -> bool {
    let is_hex = |c: char| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F');
    is_hex(c1) && is_hex(c2)
}
