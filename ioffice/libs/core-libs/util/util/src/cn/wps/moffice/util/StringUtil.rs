// Java: import cn.wps.base.assertion.Assert
// Java: import cn.wps.base.log.Log
// Java: import java.io.File
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.UnsupportedEncodingException
// Java: import java.security.MessageDigest
// Java: import java.security.NoSuchAlgorithmException
// Java: import java.util.Comparator
// Java: import java.util.Random
// Java: import java.util.StringTokenizer
// Java: import java.util.regex.Matcher
// Java: import java.util.regex.Pattern

use std::path::MAIN_SEPARATOR;

// Java: public class StringUtil
pub struct StringUtil;

// Java: public class StringComparator
pub struct StringComparator;

// Java: public class StringUtilException
#[derive(Debug)]
pub struct StringUtilException {
    message: String,
}

impl std::fmt::Display for StringUtilException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for StringUtilException {}

// Java: public class StringUtil
impl StringUtil {
    // Java field: _HEX_DIGITS: &[char]
    const HEX_DIGITS: &[u8] = b"0123456789ABCDEF";
    // Java field: String ROOT_PATH
    const ROOT_PATH: &str = "/";
    // Java field: String DEFAULT_CHARSET
    pub const DEFAULT_CHARSET: &str = "UTF-8";
    // Java field: String HASH_ALGORITHM
    const HASH_ALGORITHM: &str = "MD5";
    // Java field: _PUNCTS: &[char]
    // Java field: _WHITE_SPACES: &[char]
    pub const WHITE_SPACES: &[char] = &[' ', '\t', '\n', '\x0C', '\r'];
    // Java field: String quotation
    const QUOTATION: &str = "\"";

    // Java: StringUtil()
    pub fn new() -> Self {
        // Java: super();
        StringUtil
    }

    // Java: static boolean isEmpty(String s)
    pub fn isEmpty(s: Option<&str>) -> bool {
        // Java: return s == null || s.trim().length() == 0;
        s.map_or(true, |v| v.trim().is_empty())
    }

    // Java: static boolean isHave(_strs: &[String], String str)
    pub fn isHave(strs: Option<&[&str]>, str: Option<&str>) -> bool {
        // Java: if (strs != null && str != null)
        if let (Some(strs), Some(str)) = (strs, str) {
            // Java: for(int i = 0; i < strs.length; ++i)
            for s in strs {
                // Java: if (str.equals(strs[i]))
                if str == *s {
                    // Java: return true;
                    return true;
                }
            }
            // Java: return false;
            false
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }

    // Java: static boolean isStartWith(_strs: &[String], String str)
    pub fn isStartWith(strs: Option<&[&str]>, str: Option<&str>) -> bool {
        // Java: if (str != null && strs != null)
        if let (Some(strs), Some(str)) = (strs, str) {
            // Java: for(int i = 0; i < strs.length; ++i)
            for s in strs {
                // Java: if (str.startsWith(strs[i]))
                if str.starts_with(s) {
                    // Java: return true;
                    return true;
                }
            }
            // Java: return false;
            false
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }

    // Java: static _splitString: &[String](String tokenedStr, String token)
    // Java: static String[] splitString(String tokenedStr, String token)
    pub fn splitString(tokened_str: Option<&str>, token: &str) -> Option<Vec<String>> {
        // Java: String[] ids = null;
        let mut ids: Option<Vec<String>> = None;
        // Java: _ids: &[String] = null;
        // Java: if (tokenedStr != null)
        if let Some(s) = tokened_str {
            // Java: StringTokenizer st = new StringTokenizer(tokenedStr, token);
            let st = s.split(token);
            // Java: int arraySize = st.countTokens();
            let parts: Vec<String> = st.filter(|p| !p.is_empty()).map(|p| p.to_string()).collect();
            // Java: if (arraySize > 0)
            if !parts.is_empty() {
                // Java: ids = new String[arraySize];
                ids = Some(parts);
                // Java: for(int counter = 0; st.hasMoreTokens(); ids[counter++] = st.nextToken())
            }
            // Java: return ids;
            ids
        } else {
            // Java: return ids;
            None
        }
    }

    // Java: static void mergeString(_strs: &[String], String seperator, StringBuilder sb)
    pub fn mergeString_sb(strs: &[&str], seperator: &str, sb: &mut String) {
        // Java: if (strs != null && strs.length != 0)
        if !strs.is_empty() {
            // Java: for(int i = 0; i < strs.length; ++i)
            for i in 0..strs.len() {
                // Java: if (i != 0)
                if i != 0 {
                    // Java: sb.append(seperator);
                    sb.push_str(seperator);
                }
                // Java: sb.append(strs[i]);
                sb.push_str(strs[i]);
            }
        }
    }

    // Java: static String stringWithFormat(String str, Object... args)
    pub fn stringWithFormat(str: &str, args: &[&dyn std::fmt::Display]) -> String {
        // Java: str = String.format(str, args);
        // Java: return str;
        let mut result = str.to_string();
        for arg in args {
            if let Some(pos) = result.find("%d").or_else(|| result.find("%.1f")).or_else(|| result.find("%s")) {
                let end = result[pos..].find(|c: char| c.is_alphabetic() && c != '%').map(|e| pos + e + 1).unwrap_or(result.len());
                result = format!("{}{}{}", &result[..pos], arg, &result[end..]);
            }
        }
        result
    }

    // Java: static String mergeString(_strs: &[String], String seperator)
    pub fn mergeString(strs: &[&str], seperator: &str) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: mergeString(strs, seperator, sb);
        Self::mergeString_sb(strs, seperator, &mut sb);
        // Java: return sb.toString();
        sb
    }

    // Java: static String lowerCaseExtension(String strFileName)
    pub fn lowerCaseExtension(str_file_name: &str) -> String {
        // Java: String strFileNameBody = stringByDeletingPathExtension(strFileName);
        let strFileNameBody = Self::stringByDeletingPathExtension(Some(str_file_name));
        // Java: String strExt = pathExtension(strFileName).toLowerCase();
        let strExt = Self::pathExtension(Some(str_file_name)).to_lowercase();
        // Java: strFileName = strFileNameBody + "." + strExt;
        let strFileName = format!("{}.{}", strFileNameBody, strExt);
        // Java: return strFileName;
        strFileName
    }

    // Java: static String pathExtension(String fileName)
    pub fn pathExtension(file_name: Option<&str>) -> String {
        // Java: if (null == fileName)
        if file_name.is_none() {
            // Java: return "";
            return String::new();
        // Java: else
        }
        let file_name = file_name.unwrap();
        // Java: int point = fileName.lastIndexOf(46);
        let point = file_name.rfind('.').map(|p| p as i32).unwrap_or(-1);
        // Java: int separatorIndex = fileName.lastIndexOf(File.separator);
        let sep_idx = file_name.rfind(MAIN_SEPARATOR).map(|p| p as i32).unwrap_or(-1);
        // Java: if (point < separatorIndex)
        if point < sep_idx {
            // Java: return "";
            return String::new();
        // Java: else
        }
        // Java: int length = fileName.length();
        let length = file_name.len() as i32;
        // Java: return point != -1 && point != length - 1 ? fileName.substring(point + 1, length) : "";
        if point != -1 && point != length - 1 {
            file_name[(point as usize + 1)..].to_string()
        } else {
            String::new()
        }
    }

    // Java: static String stringByDeletingPathExtension(String filename)
    pub fn stringByDeletingPathExtension(filename: Option<&str>) -> String {
        // Java: if (null != filename && filename.length() != 0)
        if let Some(f) = filename {
            if !f.is_empty() {
                // Java: int index = filename.lastIndexOf(".");
                let index = f.rfind('.').map(|p| p as i32).unwrap_or(-1);
                // Java: int separatorIndex = filename.lastIndexOf(File.separator);
                let sep_idx = f.rfind(MAIN_SEPARATOR).map(|p| p as i32).unwrap_or(-1);
                // Java: return index != -1 && index > separatorIndex ? filename.substring(0, index) : filename;
                if index != -1 && index > sep_idx {
                    f[..(index as usize)].to_string()
                } else {
                    f.to_string()
                }
            } else {
                f.to_string()
            }
        // Java: else
        } else {
            // Java: return filename;
            String::new()
        }
    }

    // Java: static String stringByAppendingPathComponent(String strFolder, String filename)
    pub fn stringByAppendingPathComponent(str_folder: &str, filename: &str) -> String {
        // Java: return strFolder + File.separator + filename;
        format!("{}{}{}", str_folder, MAIN_SEPARATOR, filename)
    }

    // Java: static String stringByAppendingPathExtension(String trimExt, String ext)
    pub fn stringByAppendingPathExtension(trim_ext: &str, ext: &str) -> String {
        // Java: return trimExt + "." + ext;
        format!("{}.{}", trim_ext, ext)
    }

    // Java: static String getNamePart(String fileName)
    pub fn getNamePart(file_name: Option<&str>) -> String {
        // Java: if (null == fileName)
        if file_name.is_none() {
            // Java: return "";
            return String::new();
        // Java: else
        }
        let file_name = file_name.unwrap();
        // Java: int point = getPathLastIndex(fileName);
        let point = Self::getPathLastIndex(Some(file_name));
        // Java: int length = fileName.length();
        let length = file_name.len() as i32;
        // Java: if (point == -1)
        if point == -1 {
            // Java: return fileName;
            file_name.to_string()
        // Java: else if (point == length - 1)
        } else if point == length - 1 {
            // Java: int secondPoint = getPathLastIndex(fileName, point - 1);
            let secondPoint = Self::getPathLastIndex_from(Some(file_name), point - 1);
            // Java: if (secondPoint == -1)
            if secondPoint == -1 {
                // Java: return length == 1 ? fileName : fileName.substring(0, point);
                if length == 1 { file_name.to_string() } else { file_name[..(point as usize)].to_string() }
            // Java: else
            } else {
                // Java: return fileName.substring(secondPoint + 1, point);
                file_name[((secondPoint + 1) as usize)..(point as usize)].to_string()
            }
        // Java: else
        } else {
            // Java: return fileName.substring(point + 1);
            file_name[((point + 1) as usize)..].to_string()
        }
    }

    // Java: static String getNamePartWithoutExtension(String fileName)
    pub fn getNamePartWithoutExtension(file_name: Option<&str>) -> String {
        // Java: String namePart = getNamePart(fileName);
        let namePart = Self::getNamePart(file_name);
        // Java: if (namePart != null && namePart.contains("."))
        if namePart.contains('.') {
            // Java: _nameParts: &[String] = namePart.split("\\.");
            // Java: String[] nameParts = namePart.split("\\.");
            let nameParts: Vec<&str> = namePart.split('.').collect();
            // Java: if (nameParts != null && nameParts.length >= 2)
            if nameParts.len() >= 2 {
                // Java: namePart = nameParts[0];
                let namePart = nameParts[0];
                return namePart.to_string();
            }
        }
        // Java: return namePart;
        namePart
    }

    // Java: static String trim(String s)
    pub fn trim(s: Option<&str>) -> Option<String> {
        // Java: if (s == null)
        if s.is_none() {
            // Java: return null;
            return None;
        // Java: else
        }
        let s = s.unwrap();
        // Java: int len = s.length();
        let chars: Vec<char> = s.chars().collect();
        let mut len = chars.len();
        // Java: int st;
        // Java: for(st = 0; st < len && (s.charAt(st) <= ' ' || s.charAt(st) == 12288); ++st)
        let mut st = 0;
        while st < len && (chars[st] <= ' ' || chars[st] as u32 == 12288) {
            st += 1;
        }
        // Java: while(st < len && (s.charAt(st) <= ' ' || s.charAt(st) == 12288))
        while st < len && (chars[len - 1] <= ' ' || chars[len - 1] as u32 == 12288) {
            // Java: --len;
            len -= 1;
        }
        // Java: return st <= 0 && len >= s.length() ? s : s.substring(st, len);
        if st == 0 && len >= chars.len() { Some(s.to_string()) } else { Some(chars[st..len].iter().collect()) }
    }

    // Java: static String getNameWithoutLastSuffix(String fileName)
    pub fn getNameWithoutLastSuffix(file_name: Option<&str>) -> String {
        // Java: if (null != fileName && fileName.length() != 0)
        if let Some(f) = file_name {
            if !f.is_empty() {
                // Java: int endIndex = fileName.lastIndexOf(".");
                if let Some(idx) = f.rfind('.') {
                    // Java: return endIndex == -1 ? fileName : fileName.substring(0, endIndex);
                    return f[..idx].to_string();
                }
                return f.to_string();
            }
        }
        // Java: else
        // Java: return "";
        String::new()
    }

    // Java: static String getNameWithoutSuffix(String filePath)
    pub fn getNameWithoutSuffix(file_path: Option<&str>) -> String {
        // Java: if (null != filePath && filePath.length() != 0)
        if let Some(fp) = file_path {
            if !fp.is_empty() {
                // Java: int separatorIndex = filePath.lastIndexOf(File.separator);
                let sep_idx = fp.rfind(MAIN_SEPARATOR).map(|p| p as i32).unwrap_or(-1);
                // Java: int beginIndex = 0;
                let mut beginIndex = 0usize;
                // Java: int endIndex = filePath.length();
                let mut endIndex = fp.len();
                // Java: if (separatorIndex >= 0)
                if sep_idx >= 0 {
                    // Java: beginIndex = separatorIndex + 1;
                    beginIndex = (sep_idx + 1) as usize;
                }
                // Java: int suffixPos = filePath.lastIndexOf(46);
                let suffix_pos = fp.rfind('.').map(|p| p as i32).unwrap_or(-1);
                // Java: if (suffixPos > 0 && suffixPos < filePath.length() - 1)
                if suffix_pos > 0 && (suffix_pos as usize) < fp.len() - 1 {
                    // Java: endIndex = suffixPos;
                    endIndex = suffix_pos as usize;
                }
                // Java: return endIndex < beginIndex ? filePath.substring(beginIndex) : filePath.substring(beginIndex, endIndex);
                return if endIndex < beginIndex { fp[beginIndex..].to_string() } else { fp[beginIndex..endIndex].to_string() };
            }
        }
        // Java: else
        // Java: return "";
        String::new()
    }

    // Java: static String getNameDelLastPath(String fileName)
    pub fn getNameDelLastPath(file_name: Option<&str>) -> Option<String> {
        // Java: if (fileName == null)
        if file_name.is_none() {
            // Java: return fileName;
            return None;
        // Java: else
        }
        let f = file_name.unwrap();
        // Java: int point = getPathLastIndex(fileName);
        let point = Self::getPathLastIndex(Some(f));
        // Java: return point == -1 ? fileName : fileName.substring(0, point);
        Some(if point == -1 { f.to_string() } else { f[..(point as usize)].to_string() })
    }

    // Java: static int getPathLastIndex(String fileName)
    pub fn getPathLastIndex(file_name: Option<&str>) -> i32 {
        // Java: if (null == fileName)
        if file_name.is_none() {
            // Java: return -1;
            return -1;
        // Java: else
        }
        let f = file_name.unwrap();
        // Java: int point = fileName.lastIndexOf(47);
        let mut point = f.rfind('/').map(|p| p as i32).unwrap_or(-1);
        // Java: if (point == -1)
        if point == -1 {
            // Java: point = fileName.lastIndexOf(92);
            point = f.rfind('\\').map(|p| p as i32).unwrap_or(-1);
        }
        // Java: return point;
        point
    }

    // Java: static int getPathLastIndex(String fileName, int fromIndex)
    pub fn getPathLastIndex_from(file_name: Option<&str>, from_index: i32) -> i32 {
        // Java: if (null == fileName)
        if file_name.is_none() {
            // Java: return -1;
            return -1;
        // Java: else
        }
        let f = file_name.unwrap();
        let end = (from_index + 1) as usize;
        let sub = if end <= f.len() { &f[..end] } else { f };
        // Java: int point = fileName.lastIndexOf(47, fromIndex);
        let mut point = sub.rfind('/').map(|p| p as i32).unwrap_or(-1);
        // Java: if (point == -1)
        if point == -1 {
            // Java: point = fileName.lastIndexOf(92, fromIndex);
            point = sub.rfind('\\').map(|p| p as i32).unwrap_or(-1);
        }
        // Java: return point;
        point
    }

    // Java: static String toString(_ba: &[u8])
    pub fn toString(ba: &[u8]) -> String {
        // Java: return toString(ba, 0, ba.length);
        Self::toString__range(ba, 0, ba.len())
    }

    // Java: static String toString(_ba: &[u8], int offset, int length)
    pub fn toString__range(ba: &[u8], offset: usize, length: usize) -> String {
        // Java: _buf: &[char] = new char[length * 2];
        // Java: char[] buf = new char[length * 2];
        let mut buf = String::with_capacity(length * 2);
        // Java: int i = 0;
        let mut i = 0usize;
        // Java: int k;
        // Java: for(int j = 0; i < length; buf[j++] = HEX_DIGITS[k & 15])
        while i < length {
            // Java: k = ba[offset + i++];
            let k = ba[offset + i] as usize;
            i += 1;
            // Java: buf[j++] = HEX_DIGITS[k >>> 4 & 15];
            buf.push(Self::HEX_DIGITS[(k >> 4) & 0xF] as char);
            buf.push(Self::HEX_DIGITS[k & 0xF] as char);
        }
        // Java: return new String(buf);
        buf
    }

    // Java: static String removeInvalidChar(String str)
    pub fn removeInvalidChar(str: &str) -> String {
        // Java: String regEx = "[`~!@#$%^&*()+=|{}':;＇,\\[\\].<>/?～！＠＃￥％……＆＊（）——＋｜｛｝【】‘；：”“’。，、？]";
        let regEx = "`~!@#$%^&*()+=|{}':;,.<>/?";
        // Java: Pattern p = Pattern.compile(regEx);
        // Java: Matcher m = p.matcher(str);
        // Java: return m.replaceAll("").trim();
        str.chars().filter(|c| !regEx.contains(*c)).collect::<String>().trim().to_string()
    }

    // Java: static String getTextOr(String text, String defaultText)
    pub fn getTextOr(text: Option<&str>, default_text: &str) -> String {
        // Java: return isEmpty(text) ? defaultText : text;
        if Self::isEmpty(text) { default_text.to_string() } else { text.unwrap().to_string() }
    }

    // Java: static boolean isAnyoneEmpty(String... strArray)
    pub fn isAnyoneEmpty(str_array: &[Option<&str>]) -> bool {
        // Java: for(String item : strArray)
        for item in str_array {
            // Java: if (isEmpty(item))
            if Self::isEmpty(*item) {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: static String createRandomKey(int size)
    pub fn createRandomKey(size: usize) -> String {
        // Java: _c: &[char] = new char[]{'1', '2', '3', '4', '5', '6', '7', '8', '9', '0'};
        // Java: char[] c = new char[]{'1', '2', '3', '4', '5', '6', '7', '8', '9', '0'};
        let c = ['1','2','3','4','5','6','7','8','9','0'];
        // Java: Random random = new Random();
        let random = 0usize; // stub: Java Random source is not wired in the Rust port
        // Java: StringBuffer sb = new StringBuffer();
        let mut sb = String::new();
        // Java: for(int i = 0; i < size; ++i)
        for _ in 0..size {
            // Java: int r = random.nextInt();
            // Java: if (r == Integer.MIN_VALUE)
            // Java: r = 0;
            let r = random; // stub: Java random.nextInt() keeps its 0 placeholder
            // Java: sb.append(c[Math.abs(r) % c.length]);
            sb.push(c[r % c.len()]);
        }
        // Java: return sb.toString();
        sb
    }

    // Java: static String stringFromSize(double size)
    pub fn stringFromSize(size: f64) -> String {
        // Java: if (size < (double)0.0F)
        if size < 0.0 {
            // Java: return "-";
            "-".to_string()
        // Java: else if (size == (double)0.0F)
        } else if size == 0.0 {
            // Java: return "0 B";
            "0 B".to_string()
        // Java: else
        } else {
            // Java: double K = size / (double)1024.0F;
            let mut K = size / 1024.0;
            // Java: if (K < (double)1024.0F)
            if K < 1024.0 {
                // Java: if (K < (double)1.0F)
                if K < 1.0 {
                    // Java: if (K < 0.1)
                    if K < 0.1 {
                        // Java: K = 0.1;
                        K = 0.1;
                    }
                    // Java: return stringWithFormat("%.1f KB", K);
                    format!("{:.1} KB", K)
                // Java: else
                } else {
                    // Java: int k = (int)K;
                    let k = K as i32;
                    // Java: return stringWithFormat("%d KB", k);
                    format!("{} KB", k)
                }
            // Java: else
            } else {
                // Java: double M = K / (double)1024.0F;
                let M = K / 1024.0;
                // Java: if (M < (double)1024.0F)
                if M < 1024.0 {
                    // Java: return stringWithFormat("%.1f MB", M);
                    format!("{:.1} MB", M)
                // Java: else
                } else {
                    // Java: double G = M / (double)1024.0F;
                    let G = M / 1024.0;
                    // Java: return stringWithFormat("%.1f GB", G);
                    format!("{:.1} GB", G)
                }
            }
        }
    }

    // Java: static String getFileNameSubStr(String path)
    pub fn getFileNameSubStr(path: &str) -> String {
        // Java: path = path.substring(0, path.lastIndexOf(47) + 1);
        let idx = path.rfind('/').map(|p| p + 1).unwrap_or(0);
        let path = &path[..idx];
        // Java: if ("/".equals(path))
        if path == "/" {
            // Java: return path;
            return path.to_string();
        // Java: else
        }
        // Java: if (path.charAt(path.length() - 1) == '/')
        let path = if path.ends_with('/') {
            // Java: path = path.substring(0, path.length() - 1);
            &path[..path.len() - 1]
        } else { path };
        // Java: return path.substring(path.lastIndexOf(47) + 1);
        let idx = path.rfind('/').map(|p| p + 1).unwrap_or(0);
        path[idx..].to_string()
    }

    // Java: static int search(String text, String tag, int flag)
    pub fn search(text: &str, tag: &str, flag: usize) -> usize {
        // Java: int tagLength = tag.length();
        let tag_length = tag.len();
        // Java: _temp: &[String] = text.split(tag);
        // Java: String[] temp = text.split(tag);
        let temp: Vec<&str> = text.split(tag).collect();
        // Java: int l = temp.length;
        let mut l = temp.len();
        // Java: if (!text.endsWith(tag))
        if !text.ends_with(tag) {
            // Java: --l;
            l -= 1;
        }
        // Java: int position = 0;
        let mut position = 0;
        // Java: _positionarr: &[i32] = new int[l];
        // Java: int[] positionarr = new int[l];
        let mut positionarr = vec![0usize; l];
        // Java: for(int i = 0; i < l; ++i)
        for i in 0..l {
            // Java: String s = temp[i];
            let s = temp[i];
            // Java: position += s.length();
            position += s.len();
            // Java: positionarr[i] = position;
            positionarr[i] = position;
            // Java: position += tagLength;
            position += tag_length;
        }
        // Java: return positionarr[flag - 1];
        positionarr[flag - 1]
    }

    // Java: static String normalizePath(String path)
    pub fn normalizePath(path: Option<&str>) -> Option<String> {
        // Java: if (!isEmpty(path) && path.contains("//"))
        if let Some(p) = path {
            if !p.is_empty() && p.contains("//") {
                // Java: path = path.replaceAll("/+/", "/");
                let mut result = p.to_string();
                while result.contains("//") {
                    result = result.replace("//", "/");
                }
                return Some(result);
            }
        }
        // Java: return path;
        path.map(|p| p.to_string())
    }

    // Java: static String bytesToHex(_bytes: &[u8])
    pub fn bytesToHex(bytes: &[u8]) -> String {
        // Java: return bytesToHex(bytes, false);
        Self::bytesToHex_spaced(bytes, false)
    }

    // Java: static String bytesToHex(_bytes: &[u8], boolean withSpaces)
    pub fn bytesToHex_spaced(bytes: &[u8], with_spaces: bool) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: for(byte hashByte : bytes)
        for &hash_byte in bytes {
            // Java: int intVal = 255 & hashByte;
            let intVal = hash_byte as u32;
            // Java: if (intVal < 16)
            if intVal < 16 {
                // Java: sb.append('0');
                sb.push('0');
            }
            // Java: sb.append(Integer.toHexString(intVal));
            sb.push_str(&format!("{:x}", intVal));
            // Java: if (withSpaces)
            if with_spaces {
                // Java: sb.append(' ');
                sb.push(' ');
            }
        }
        // Java: return sb.toString();
        sb
    }

    // Java: static _hexToBytes: &[u8](String hexString)
    // Java: static byte[] hexToBytes(String hexString)
    pub fn hexToBytes(hex_string: &str) -> Vec<u8> {
        // Java: _result: &[u8] = new byte[hexString.length() / 2];
        // Java: byte[] result = new byte[hexString.length() / 2];
        let mut result = vec![0u8; hex_string.len() / 2];
        // Java: for(int i = 0; i < result.length; ++i)
        for i in 0..result.len() {
            // Java: int offset = i * 2;
            let offset = i * 2;
            // Java: result[i] = (byte)Integer.parseInt(hexString.substring(offset, offset + 2), 16);
            result[i] = u8::from_str_radix(&hex_string[offset..offset + 2], 16).unwrap_or(0);
        }
        // Java: return result;
        result
    }

    // Java: static _stringToBytes: &[u8](String string)
    // Java: static byte[] stringToBytes(String string)
    pub fn stringToBytes(string: &str) -> Vec<u8> {
        // Java: try
        // Java: return stringToBytes(string, "UTF-8");
        // Java: catch (UnsupportedEncodingException e)
        // Java: throw new StringUtilException("UTF-8 not supported", e);
        string.as_bytes().to_vec()
    }

    // Java: static _stringToBytes: &[u8](String string, String charSet)
    // Java: static byte[] stringToBytes(String string, String charSet)
    pub fn stringToBytes_charset(string: Option<&str>, _char_set: &str) -> Option<Vec<u8>> {
        // Java: return string == null ? null : string.getBytes(charSet);
        string.map(|s| s.as_bytes().to_vec())
    }

    // Java: static String bytesToString(_bytes: &[u8])
    pub fn bytesToString(bytes: &[u8]) -> String {
        // Java: try
        // Java: return bytesToString(bytes, "UTF-8");
        // Java: catch (UnsupportedEncodingException e)
        // Java: throw new StringUtilException("UTF-8 not supported", e);
        String::from_utf8_lossy(bytes).to_string()
    }

    // Java: static String bytesToString(_bytes: &[u8], String charSet)
    pub fn bytesToString_charset(bytes: Option<&[u8]>, _char_set: &str) -> Option<String> {
        // Java: return bytes == null ? null : new String(bytes, charSet);
        bytes.map(|b| String::from_utf8_lossy(b).to_string())
    }

    // Java: static _hash: &[u8](_body: &[u8])
    // Java: static byte[] hash(byte[] body)
    pub fn hash_bytes(_body: &[u8]) -> Vec<u8> {
        // Java: try
        // Java: return MessageDigest.getInstance("MD5").digest(body);
        // Java: catch (NoSuchAlgorithmException e)
        // Java: throw new StringUtilException("MD5 not supported", e);
        Vec::new()
    }

    // Java: static _hash: &[u8](InputStream data)
    // Java: static byte[] hash(InputStream data)
    pub fn hash_stream(_data: &mut dyn std::io::Read) -> Vec<u8> {
        /* DEFERRED_NON_FUNCTION（MessageDigest/MD5 未接线：本方法保留空 Vec 占位，Java 模板体停放在此）
        // Java: try
        // Java: MessageDigest digest = MessageDigest.getInstance("MD5");
        // Java: _buf: &[u8] = new byte[4096];
        // Java: byte[] buf = new byte[4096];
        // Java: int len;
        // Java: while((len = data.read(buf)) != -1)
        // Java: digest.update(buf, 0, len);
        // Java: return digest.digest();
        // Java: catch (NoSuchAlgorithmException e)
        // Java: throw new StringUtilException("MD5 not supported", e);
        */
        Vec::new()
    }

    // Java: static _hash: &[u8](String content)
    // Java: static byte[] hash(String content)
    pub fn hash_string(content: &str) -> Vec<u8> {
        // Java: return hash(stringToBytes(content));
        Self::hash_bytes(&Self::stringToBytes(content))
    }

    // Java: static String getFileNameSuffix(String fileName)
    pub fn getFileNameSuffix(file_name: Option<&str>) -> String {
        // Java: if (fileName == null)
        if file_name.is_none() {
            // Java: return "";
            return String::new();
        // Java: else
        }
        let f = file_name.unwrap();
        // Java: int pos = fileName.lastIndexOf(46);
        let pos = f.rfind('.').map(|p| p as i32).unwrap_or(-1);
        // Java: int separatorIndex = fileName.lastIndexOf(File.separator);
        let sep_idx = f.rfind(MAIN_SEPARATOR).map(|p| p as i32).unwrap_or(-1);
        // Java: return pos > 0 && pos > separatorIndex && pos < fileName.length() - 1 ? fileName.substring(pos + 1) : "";
        if pos > 0 && pos > sep_idx && (pos as usize) < f.len() - 1 { f[((pos + 1) as usize)..].to_string() } else { String::new() }
    }

    // Java: static long convertStringToLong(long defaultValue, String value)
    pub fn convertStringToLong(default_value: i64, value: &str) -> i64 {
        // Java: long result = defaultValue;
        let mut result = default_value;
        // Java: try
        // Java: result = Long.parseLong(value);
        // Java: catch (Exception var6)
        result = value.parse::<i64>().unwrap_or(result);
        // Java: return result > 0L ? result : defaultValue;
        if result > 0 { result } else { default_value }
    }

    // Java: static int convertStringToInt(int defaultValue, String value)
    pub fn convertStringToInt(default_value: i32, value: &str) -> i32 {
        // Java: int result = defaultValue;
        let mut result = default_value;
        // Java: try
        // Java: result = Integer.parseInt(value);
        // Java: catch (Exception var4)
        result = value.parse::<i32>().unwrap_or(result);
        // Java: return result > 0 ? result : defaultValue;
        if result > 0 { result } else { default_value }
    }

    // Java: static boolean containsWithoutCase(String string, String value)
    pub fn containsWithoutCase(string: Option<&str>, value: Option<&str>) -> bool {
        // Java: return !isEmpty(string) && !isEmpty(value) ? string.toLowerCase().contains(value.toLowerCase()) : false;
        if let (Some(s), Some(v)) = (string, value) {
            if !s.trim().is_empty() && !v.trim().is_empty() {
                return s.to_lowercase().contains(&v.to_lowercase());
            }
        }
        false
    }

    // Java: static boolean containsWithoutCase(String content, _values: &[String])
    pub fn containsWithoutCase_arr(content: Option<&str>, values: &[&str]) -> bool {
        // Java: if (!isEmpty(content) && values != null && values.length != 0)
        if !Self::isEmpty(content) && !values.is_empty() {
            // Java: for(String value : values)
            for &value in values {
                // Java: if (containsWithoutCase(content, value))
                if Self::containsWithoutCase(content, Some(value)) {
                    // Java: return true;
                    return true;
                }
            }
            // Java: return false;
            false
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }

    // Java: static boolean isContainEmoji(String content)
    pub fn isContainEmoji(_content: &str) -> bool {
        // Java: return EmojiTool.isContainEmoji(content);
        false
    }

    // Java: static boolean isPunctuation(char ch)
    pub fn isPunctuation(ch: char) -> bool {
        // Java: for(int i = 0; i < PUNCTS.length; ++i)
        // Java: if (PUNCTS[i] == ch)
        // Java: return true;
        // Java: return false;
        "!$%(),.;:>?[]{}".contains(ch)
    }

    // Java: static int valueToInt(String value, int defaultValue)
    pub fn valueToInt(value: Option<&str>, default_value: i32) -> i32 {
        // Java: if (isEmpty(value))
        if Self::isEmpty(value) {
            // Java: return defaultValue;
            return default_value;
        // Java: else
        }
        // Java: int res = defaultValue;
        let mut res = default_value;
        // Java: try
        // Java: res = Integer.valueOf(value);
        // Java: catch (Exception e)
        // Java: Log.w("valueToInt", "", e);
        res = value.unwrap().parse::<i32>().unwrap_or(res);
        // Java: return res;
        res
    }

    // Java: static String subString(String originStr, int reqLen)
    pub fn subString(origin_str: Option<&str>, req_len: usize) -> Option<String> {
        // Java: if (originStr != null && originStr.length() != 0)
        if let Some(s) = origin_str {
            if !s.is_empty() {
                // Java: return originStr.length() > reqLen ? originStr.substring(0, reqLen) : originStr;
                let chars: Vec<char> = s.chars().collect();
                return Some(if chars.len() > req_len { chars[..req_len].iter().collect() } else { s.to_string() });
            }
        }
        // Java: else
        // Java: return null;
        None
    }

    // Java: static String centerLimit(String src, int maxLength, String replace)
    pub fn centerLimit(src: Option<&str>, max_length: usize, replace: Option<&str>) -> Option<String> {
        // Java: if (replace == null)
        // Java: replace = "...";
        let replace = replace.unwrap_or("...");
        // Java: if (!isEmpty(src) && src.length() > maxLength)
        if let Some(s) = src {
            if !s.is_empty() && s.len() > max_length {
                // Java: int firstSize = maxLength / 2;
                let firstSize = max_length / 2;
                // Java: int lastSize = maxLength - firstSize;
                let lastSize = max_length - firstSize;
                // Java: String firstString = src.substring(0, firstSize);
                let first = &s[..firstSize];
                // Java: String lastString = src.substring(src.length() - lastSize);
                let last = &s[s.len() - lastSize..];
                // Java: return firstString + replace + lastString;
                return Some(format!("{}{}{}", first, replace, last));
            }
        }
        // Java: else
        // Java: return src;
        src.map(|s| s.to_string())
    }

    // Java: static String leftLimit(String src, int maxLength)
    pub fn leftLimit(src: Option<&str>, max_length: usize) -> Option<String> {
        // Java: if (isEmpty(src))
        if Self::isEmpty(src) {
            // Java: return src;
            return src.map(|s| s.to_string());
        // Java: else
        }
        let s = src.unwrap();
        // Java: return src.length() <= maxLength ? src : src.substring(0, maxLength);
        Some(if s.len() <= max_length { s.to_string() } else { s[..max_length].to_string() })
    }

    // Java: static String filterSpecialChars(String source)
    pub fn filterSpecialChars(source: Option<&str>) -> Option<String> {
        // Java: return isEmpty(source) ? source : source.replaceAll("&quot;", "").replaceAll("&cent;", "").replaceAll("&pound;", "").replaceAll("&copy;", "").replaceAll("&raquo;", "").replaceAll("&laquo;", "").replaceAll("&reg;", "").replaceAll("&deg;", "").replaceAll("&plusmn;", "").replaceAll("&middot;", "").replaceAll("&frac12;", "").replaceAll("&ndash;", "").replaceAll("&mdash;", "").replaceAll("&lsquo;", "").replaceAll("&rsquo;", "").replaceAll("&sbquo;", "").replaceAll("&ldquo;", "").replaceAll("&Dagger;", "").replaceAll("&bull;", "").replaceAll("&hellip;", "").replaceAll("&prime;", "").replaceAll("&Prime;", "").replaceAll("&euro;", "").replaceAll("&trade;", "").replaceAll("&asymp;", "").replaceAll("&ne;", "").replaceAll("&le;", "").replaceAll("&ge;", "").replaceAll("&lt;", "").replaceAll("&gt;", "");
        if Self::isEmpty(source) {
            source.map(|s| s.to_string())
        } else {
            let s = source.unwrap();
            let result = s.replace("&quot;", "").replace("&cent;", "").replace("&pound;", "")
                .replace("&copy;", "").replace("&raquo;", "").replace("&laquo;", "")
                .replace("&reg;", "").replace("&deg;", "").replace("&plusmn;", "")
                .replace("&middot;", "").replace("&frac12;", "").replace("&ndash;", "")
                .replace("&mdash;", "").replace("&lsquo;", "").replace("&rsquo;", "")
                .replace("&sbquo;", "").replace("&ldquo;", "").replace("&Dagger;", "")
                .replace("&bull;", "").replace("&hellip;", "").replace("&prime;", "")
                .replace("&Prime;", "").replace("&euro;", "").replace("&trade;", "")
                .replace("&asymp;", "").replace("&ne;", "").replace("&le;", "")
                .replace("&ge;", "").replace("&lt;", "").replace("&gt;", "");
            Some(result)
        }
    }

    // Java: static String rightLimit(String src, int maxLength, String replace)
    pub fn rightLimit(src: Option<&str>, max_length: usize, replace: Option<&str>) -> Option<String> {
        // Java: if (replace == null)
        // Java: replace = "...";
        let replace = replace.unwrap_or("...");
        // Java: if (!isEmpty(src) && src.length() > maxLength)
        if let Some(s) = src {
            if !s.is_empty() && s.len() > max_length {
                // Java: String end = src.substring(0, maxLength) + replace;
                // Java: return end;
                return Some(format!("{}{}", &s[..max_length], replace));
            }
        }
        // Java: else
        // Java: return src;
        src.map(|s| s.to_string())
    }

    // Java: static String normalize(String strData)
    pub fn normalize(str_data: Option<&str>) -> String {
        // Java: if (strData == null)
        if str_data.is_none() {
            // Java: return "";
            return String::new();
        // Java: else
        }
        let str_data = str_data.unwrap();
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: int length = strData.length();
        // Java: for(int index = 0; index < length; ++index)
        for (index, chr) in str_data.char_indices() {
            // Java: char chr = strData.charAt(index);
            // Java: switch (chr)
            match chr {
                // Java: case '"':
                '"' => {
                    // Java: sb.append("&quot;");
                    sb.push_str("&quot;");
                    // Java: break;
                }
                // Java: case '&':
                '&' => {
                    // Java: if (index + 5 < length)
                    if index + 5 < str_data.len() {
                        // Java: String next = strData.substring(index, index + 5);
                        let next = &str_data[index..index + 5];
                        // Java: if (next.equals("&apos;"))
                        if next == "&apos;" {
                            // Java: sb.append("&apos;");
                            sb.push_str("&apos;");
                            // Java: break;
                            continue;
                        }
                    }
                    // Java: sb.append("&amp;");
                    sb.push_str("&amp;");
                    // Java: break;
                }
                // Java: case '<':
                '<' => {
                    // Java: sb.append("&lt;");
                    sb.push_str("&lt;");
                    // Java: break;
                }
                // Java: case '>':
                '>' => {
                    // Java: sb.append("&gt;");
                    sb.push_str("&gt;");
                    // Java: break;
                }
                // Java: default:
                _ => {
                    // Java: sb.append(chr);
                    sb.push(chr);
                }
            }
        }
        // Java: return sb.toString();
        sb
    }

    // Java: static int findLastDigitFromEnd(String str)
    pub fn findLastDigitFromEnd(str: &str) -> i32 {
        // Java: Assert.assertNotNull("str should not be null", str);
        // Java: for(int i = str.length() - 1; i >= 0; --i)
        let chars: Vec<char> = str.chars().collect();
        for i in (0..chars.len()).rev() {
            // Java: if (Character.isDigit(str.charAt(i)))
            if chars[i].is_ascii_digit() {
                // Java: return i;
                return i as i32;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: static String filterWhiteSpaces(String str)
    pub fn filterWhiteSpaces(str: &str) -> String {
        // Java: Assert.assertNotNull("str should not be null", str);
        // Java: StringBuilder buf = new StringBuilder();
        let mut buf = String::new();
        // Java: int len = str.length();
        // Java: int len2 = WHITE_SPACES.length;
        let len2 = Self::WHITE_SPACES.len();
        // Java: for(int i = 0; i < len; ++i)
        for ch in str.chars() {
            // Java: char ch = str.charAt(i);
            // Java: boolean found = false;
            let mut found = false;
            // Java: for(int j = 0; j < len2; ++j)
            for j in 0..len2 {
                // Java: char whiteSpace = WHITE_SPACES[j];
                let whiteSpace = Self::WHITE_SPACES[j];
                // Java: if (whiteSpace == ch)
                if whiteSpace == ch {
                    // Java: found = true;
                    found = true;
                    // Java: break;
                    break;
                }
            }
            // Java: if (!found)
            if !found {
                // Java: buf.append(ch);
                buf.push(ch);
            }
        }
        // Java: return buf.toString();
        buf
    }

    // Java: static String filterQuotation(String str)
    pub fn filterQuotation(str: &str) -> String {
        // Java: return str.replaceAll("\"", "");
        str.replace('"', "")
    }
}

// Java: public class StringComparator
impl StringComparator {
    // Java: StringComparator()
    pub fn new() -> Self {
        // Java: super();
        StringComparator
    }

    // Java: int compare(String object1, String object2)
    pub fn compare(&self, object1: &str, object2: &str) -> i32 {
        // Java: return object1.compareTo(object2);
        object1.cmp(object2) as i32
    }
}

// Java: public class StringUtilException
impl StringUtilException {
    // Java field: long serialVersionUID

    // Java: StringUtilException(String message, Throwable cause)
    pub fn new(message: &str, _cause: &dyn std::error::Error) -> Self {
        // Java: super(message, cause);
        StringUtilException { message: message.to_string() }
    }
}
