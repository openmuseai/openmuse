// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File; // not available in this crate
// Java: import cn.wps.moffice.kfs.FileInputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileInputStream; // not available in this crate
// Java: import cn.wps.moffice.kfs.FileOutputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileOutputStream; // not available in this crate
// Java: import java.io.BufferedReader
use std::io::BufReader;
// Java: import java.io.FileNotFoundException
// (mapped to std::io::ErrorKind::NotFound)
// Java: import java.io.IOException

// Java: import java.io.InputStreamReader
// (handled by BufReader)
// Java: import java.io.OutputStreamWriter
// (handled by std::io::Write)
// Java: import java.lang.Character.UnicodeBlock
// (no direct Rust equivalent)
// Java: import java.nio.charset.Charset
// (Rust uses &str encoding names)
// Java: import java.nio.charset.StandardCharsets
// (Rust strings are always UTF-8)
// Java: import java.util.ArrayList
// (mapped to Vec<T>)
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map
// (mapped to HashMap<K,V>)
use std::sync::{OnceLock, Mutex};
use super::CSInfo::CSInfo;
use std::io::{BufRead, Write};
use std::fs::OpenOptions;

// Java field: String ISO88591
pub(crate) const ISO88591: &str = "iso88591";
// Java field: Map<Character.UnicodeBlock, Integer> mUnicodeLangMap
#[allow(non_upper_case_globals)]
static mUnicodeLangMap: OnceLock<Mutex<HashMap<i32, i32>>> = OnceLock::new();
// Java field: Map<String, Integer> mLangMap
#[allow(non_upper_case_globals)]
static mLangMap: OnceLock<Mutex<HashMap<String, i32>>> = OnceLock::new();
// Java field: Map<String, CSInfo> endcs
#[allow(non_upper_case_globals)]
static endcs: OnceLock<Mutex<HashMap<String, CSInfo>>> = OnceLock::new();
// Java field: char[][] iso1_tbl
#[allow(non_upper_case_globals)]
static iso1_tbl: OnceLock<Vec<Vec<char>>> = OnceLock::new();
// Java field: char[][] koi8r_tbl
#[allow(non_upper_case_globals)]
static koi8r_tbl: OnceLock<Vec<Vec<char>>> = OnceLock::new();

// Java: public class CSUtil
pub struct CSUtil {
}

// Java: public class CSUtil
#[allow(non_snake_case)]
impl CSUtil {
    // Java: CSUtil()
    pub fn new() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static String mychomp(String str)
    pub fn mychomp(s: &str) -> String {
      // Java: return str;
      s.to_string()
    }

    // Java: static String toLowerCase(String str, CSInfo csconv)
    pub fn toLowerCase(s: &str, csconv: &CSInfo) -> String {
      // Java: if (str == null)
        if false
      {
         // Java: return null;
         return String::new();
      }
      // Java: else
      {
         // Java: int len = str.length();
         let len = s.len();
         // Java: if (len == 0)
            if len == 0
         {
            // Java: return str;
            return s.to_string();
         }
         // Java: else
         {
            // Java: char[] buf = new char[len];
            let chars: Vec<char> = s.chars().collect();
            let mut buf: Vec<char> = vec!['\0'; chars.len()];
            // Java: for(int i = 0; i < buf.length; ++i)
                for i in 0..chars.len()
            {
               // Java: buf[i] = csconv.getLower(str.charAt(i));
               buf[i] = csconv.getLower(chars[i]);
            }
            // Java: return new String(buf);
            return buf.into_iter().collect();
         }
      }
    }

    // Java: static String removeByteOrderMark(String str)
    pub fn removeByteOrderMark(s: &str) -> String {
      // Java: return str.length() >= 3 && str.charAt(0) == 239 && str.charAt(1) == 187 && str.charAt(2) == 191 ? str.substring(3) : str;
      let bytes = s.as_bytes();
      if bytes.len() >= 3 && bytes[0] == 239 && bytes[1] == 187 && bytes[2] == 191 {
          s[3..].to_string()
      } else {
          s.to_string()
      }
    }

    // Java: static boolean isLatina(int lang)
    pub fn isLatina(lang: i32) -> bool {
      // Java: return lang == 1 || lang == 34 || lang == 3;
      lang == 1 || lang == 34 || lang == 3
    }

    // Java: static int getCapType(String str, CSInfo csconv)
    pub fn getCapType(s: &str, csconv: &CSInfo) -> i32 {
      // Java: int len = str.length();
      let chars: Vec<char> = s.chars().collect();
      let len = chars.len();
      // Java: if (len != 0 && csconv != null)
        if len != 0
      {
         // Java: int ncap = 0;
         let mut ncap: usize = 0;
         // Java: for(int i = 0; i < len; ++i)
            for i in 0..len
         {
            // Java: char c = str.charAt(i);
            let c = chars[i];
            // Java: if (csconv.getCCase(c) != 0)
                if csconv.getCCase(c) != '\0'
            {
               // Java: ++ncap;
               ncap += 1;
            }
         }
         // Java: boolean bFirstCap = false;
         let mut bFirstCap = false;
         // Java: if (ncap > 0)
            if ncap > 0
         {
            // Java: bFirstCap = csconv.getCCase(str.charAt(0)) != 0;
            bFirstCap = csconv.getCCase(chars[0]) != '\0';
         }
         // Java: if (ncap == 0)
            if ncap == 0
         {
            // Java: return 0;
            return 0;
         }
         // Java: else if (ncap == 1 && bFirstCap)
            else if ncap == 1 && bFirstCap
         {
            // Java: return 1;
            return 1;
         }
         // Java: else if (ncap == len)
            else if ncap == len
         {
            // Java: return 2;
            return 2;
         }
         // Java: else if (ncap > 1 && bFirstCap)
            else if ncap > 1 && bFirstCap
         {
            // Java: return 4;
            return 4;
         }
         // Java: else
            else
         {
            // Java: return 3;
            return 3;
         }
      }
      // Java: else
      {
         // Java: return 0;
         return 0;
      }
    }

    // Java: static String getFileEncoding(String path)
    pub fn getFileEncoding(path: &str) -> String {
      // Java: String encoding = "ISO8859-1";
      let mut encoding = String::from("ISO8859-1");
      // Java: try
      {
         // Java: File file = new File(path);
         let file_result = std::fs::File::open(path);
         // Java: BufferedReader mReader = new BufferedReader(new InputStreamReader(new FileInputStream(file), StandardCharsets.UTF_8));
         if let Ok(file) = file_result {
             let reader = BufReader::new(file);
         // Java: String set = mReader.readLine();
             if let Some(Ok(set)) = reader.lines().next() {
         // Java: encoding = parseString(set);
                 if let Some(parsed) = Self::parseString(&set) {
                     encoding = parsed;
                 }
             }
         // Java: mReader.close();
         }
      }
      // Java: catch (FileNotFoundException e)
      // (handled by file_result above)
      {
         // Java: e.printStackTrace();
      }
      // Java: catch (IOException e)
      // (handled by if let Ok above)
      {
         // Java: e.printStackTrace();
      }
      // Java: return encoding;
      encoding
    }

    // Java: static boolean isAllNum(String word)
    pub fn isAllNum(word: &str) -> bool {
      // Java: if (word != null && word.length() != 0)
        if !word.is_empty()
      {
         // Java: boolean bNum = true;
         let mut bNum = true;
         // Java: int wl = word.length();
         let wl = word.len();
         // Java: for(int i = 0; i < wl; ++i)
         {
            let chars: Vec<char> = word.chars().collect();
            for i in 0..wl {
            // Java: char c = word.charAt(i);
            let c = chars[i];
            // Java: if (c != ',' && c != '.' && c != '-' && (c > '9' || c < '0'))
                if c != ',' && c != '.' && c != '-' && (c > '9' || c < '0')
            {
               // Java: bNum = false;
               bNum = false;
               // Java: break;
               break;
            }
            }
         }
         // Java: return bNum;
         return bNum;
      }
      // Java: else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: static boolean appendWord(String path, String word)
    pub fn appendWord(path: &str, word: &str) -> bool {
      // Java: appendWord(path, word, StandardCharsets.UTF_8.displayName());
      Self::appendWord_with_encoding(path, word, "UTF-8");
      // Java: return true;
      true
    }

    // Java: static boolean appendWord(String path, String word, String encoding)
    pub fn appendWord_with_encoding(path: &str, word: &str, _encoding: &str) -> bool {
      // Java: File file = new File(path);
      let file_path = std::path::Path::new(path);
      // Java: if (!file.exists())
        if !file_path.exists()
      {
         // Java: try
         {
            // Java: file.createNewFile();
            let _ = std::fs::File::create(path);
         }
         // Java: catch (IOException e)
         {
            // Java: e.printStackTrace();
         }
      }
      // Java: try
      {
         // Java: Charset charset = Charset.forName(encoding);
         // (Rust writes UTF-8 by default)
         // Java: OutputStreamWriter osWriter = new OutputStreamWriter(new FileOutputStream(file, true), charset);
         if let Ok(mut f) = OpenOptions::new().append(true).open(path) {
         // Java: osWriter.write("\n");
             let _ = f.write_all(b"\n");
         // Java: osWriter.write(word);
             let _ = f.write_all(word.as_bytes());
         // Java: osWriter.close();
         }
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
      }
      // Java: return true;
      true
    }

    // Java: static boolean isSupportLang(int lang)
    pub fn isSupportLang(lang: i32) -> bool {
      // Java: int[] supportLangs = new int[]{1, 34, 7, 3};
      let support_langs: [i32; 4] = [1, 34, 7, 3];
      // Java: for(int i = 0; i < supportLangs.length; ++i)
        for i in 0..support_langs.len()
      {
         // Java: if (lang == supportLangs[i])
            if lang == support_langs[i]
         {
            // Java: return true;
            return true;
         }
      }
      // Java: return false;
      false
    }

    // Java: static String getLangPath(int lang)
    pub fn getLangPath(lang: i32) -> Option<&'static str> {
      // Java: switch (lang)
        match lang
      {
         // Java: case 1:
         1 =>
            // Java: return "en/";
            { return Some("en/"); }
         // Java: case 3:
         3 =>
            // Java: return "pt-br/";
            { return Some("pt-br/"); }
         // Java: case 7:
         7 =>
            // Java: return "ru-ru/";
            { return Some("ru-ru/"); }
         // Java: case 34:
         34 =>
            // Java: return "es/";
            { return Some("es/"); }
         // Java: default:
         _ =>
            // Java: return null;
            { return None; }
      }
    }

    // Java: static String getLangName(int lang)
    pub fn getLangName(lang: i32) -> Option<&'static str> {
      // Java: switch (lang)
        match lang
      {
         // Java: case 1:
         1 =>
            // Java: return "en_US";
            { return Some("en_US"); }
         // Java: case 3:
         3 =>
            // Java: return "pt_BR";
            { return Some("pt_BR"); }
         // Java: case 7:
         7 =>
            // Java: return "ru_RU";
            { return Some("ru_RU"); }
         // Java: case 34:
         34 =>
            // Java: return "es_ANY";
            { return Some("es_ANY"); }
         // Java: default:
         _ =>
            // Java: return null;
            { return None; }
      }
    }

    // Java: static int getLang(Character.UnicodeBlock block)
    pub fn getLang(_block: i32) -> i32 {
      // Java: Map<Character.UnicodeBlock, Integer> langMap = getUnicodeLangMap();
      // (UnicodeBlock not available in Rust, returning default)
      // Java: Integer lang = (Integer)langMap.get(block);
      // Java: return lang != null ? lang : 999;
      999
    }

    // Java: static private static final synchronized Map<String, Integer> getLangMap()
    fn getLangMap() -> &'static Mutex<HashMap<String, i32>> {
      // Java: if (mLangMap == null)
        mLangMap.get_or_init(||
      {
         // Java: mLangMap = new HashMap();
         let mut map = HashMap::new();
         // Java: mLangMap.put("ar", 96);
         map.insert("ar".to_string(), 96);
         // Java: mLangMap.put("az", 100);
         map.insert("az".to_string(), 100);
         // Java: mLangMap.put("az_AZ", 100);
         map.insert("az_AZ".to_string(), 100);
         // Java: mLangMap.put("bg", 41);
         map.insert("bg".to_string(), 41);
         // Java: mLangMap.put("ca", 37);
         map.insert("ca".to_string(), 37);
         // Java: mLangMap.put("cs", 42);
         map.insert("cs".to_string(), 42);
         // Java: mLangMap.put("da", 45);
         map.insert("da".to_string(), 45);
         // Java: mLangMap.put("de", 49);
         map.insert("de".to_string(), 49);
         // Java: mLangMap.put("el", 30);
         map.insert("el".to_string(), 30);
         // Java: mLangMap.put("en", 1);
         map.insert("en".to_string(), 1);
         // Java: mLangMap.put("es", 34);
         map.insert("es".to_string(), 34);
         // Java: mLangMap.put("eu", 10);
         map.insert("eu".to_string(), 10);
         // Java: mLangMap.put("gl", 38);
         map.insert("gl".to_string(), 38);
         // Java: mLangMap.put("fr", 2);
         map.insert("fr".to_string(), 2);
         // Java: mLangMap.put("hr", 78);
         map.insert("hr".to_string(), 78);
         // Java: mLangMap.put("hu", 36);
         map.insert("hu".to_string(), 36);
         // Java: mLangMap.put("hu_HU", 36);
         map.insert("hu_HU".to_string(), 36);
         // Java: mLangMap.put("it", 39);
         map.insert("it".to_string(), 39);
         // Java: mLangMap.put("la", 99);
         map.insert("la".to_string(), 99);
         // Java: mLangMap.put("lv", 101);
         map.insert("lv".to_string(), 101);
         // Java: mLangMap.put("nl", 31);
         map.insert("nl".to_string(), 31);
         // Java: mLangMap.put("pl", 48);
         map.insert("pl".to_string(), 48);
         // Java: mLangMap.put("pt", 3);
         map.insert("pt".to_string(), 3);
         // Java: mLangMap.put("sv", 50);
         map.insert("sv".to_string(), 50);
         // Java: mLangMap.put("tr", 90);
         map.insert("tr".to_string(), 90);
         // Java: mLangMap.put("tr_TR", 90);
         map.insert("tr_TR".to_string(), 90);
         // Java: mLangMap.put("ru", 7);
         map.insert("ru".to_string(), 7);
         // Java: mLangMap.put("uk", 80);
         map.insert("uk".to_string(), 80);
         Mutex::new(map)
      })
      // Java: return mLangMap;
    }

    // Java: static int getLangNum(String lang)
    pub fn getLangNum(lang: &str) -> i32 {
      // Java: Map<String, Integer> langMap = getLangMap();
      let lang_map = Self::getLangMap().lock().unwrap();
      // Java: Integer result = (Integer)langMap.get(lang);
      let result = lang_map.get(lang);
      // Java: return result != null ? result : 999;
      result.copied().unwrap_or(999)
    }

    // Java: static void flagqsort(char[] flags, int begin, int end)
    pub fn flagqsort(flags: &mut [char], begin: i32, end: i32) {
      // Java: if (end > begin)
        if end > begin
      {
         // Java: char pivot = flags[begin];
         let pivot = flags[begin as usize];
         // Java: int l = begin + 1;
         let mut l = begin + 1;
         // Java: int r = end;
         let mut r = end;
         // Java: while(l < r)
            while l < r
         {
            // Java: if (flags[l] <= pivot)
                if flags[l as usize] <= pivot
            {
               // Java: ++l;
               l += 1;
            }
            // Java: else
                else
            {
               // Java: --r;
               r -= 1;
               // Java: char reg = flags[l];
               let reg = flags[l as usize];
               // Java: flags[l] = flags[r];
               flags[l as usize] = flags[r as usize];
               // Java: flags[r] = reg;
               flags[r as usize] = reg;
            }
         }
         // Java: --l;
         l -= 1;
         // Java: char reg = flags[begin];
         let reg = flags[begin as usize];
         // Java: flags[begin] = flags[l];
         flags[begin as usize] = flags[l as usize];
         // Java: flags[l] = reg;
         flags[l as usize] = reg;
         // Java: flagqsort(flags, begin, l);
         Self::flagqsort(flags, begin, l);
         // Java: flagqsort(flags, r, end);
         Self::flagqsort(flags, r, end);
      }
    }

    // Java: static int flagBsearch(char[] flags, char flag, int length)
    pub fn flagBsearch(flags: &[char], flag: char, length: i32) -> i32 {
      // Java: int left = 0;
      let mut left: i32 = 0;
      // Java: int right = length - 1;
      let mut right: i32 = length - 1;
      // Java: while(left <= right)
        while left <= right
      {
         // Java: int mid = (left + right) / 2;
         let mid = (left + right) / 2;
         // Java: if (flags[mid] == flag)
            if flags[mid as usize] == flag
         {
            // Java: return 1;
            return 1;
         }
         // Java: if (flag < flags[mid])
            if flag < flags[mid as usize]
         {
            // Java: right = mid - 1;
            right = mid - 1;
         }
         // Java: else
            else
         {
            // Java: left = mid + 1;
            left = mid + 1;
         }
      }
      // Java: return 0;
      0
    }

    // Java: static String removeIgnoredChars(String str, String ignore)
    pub fn removeIgnoredChars(s: &str, ignore: &str) -> String {
      // Java: if (str != null && str.length() >= 1 && ignore != null && ignore.length() >= 1)
        if s.len() >= 1 && ignore.len() >= 1
      {
         // Java: int len = str.length();
         let chars: Vec<char> = s.chars().collect();
         let len = chars.len();
         // Java: char[] buf = new char[len];
         let mut buf: Vec<char> = vec!['\0'; len];
         // Java: int index = 0;
         let mut index: usize = 0;
         // Java: for(int i = 0; i < len; ++i)
            for i in 0..len
         {
            // Java: char c = str.charAt(i);
            let c = chars[i];
            // Java: if (ignore.indexOf(c) < 0)
                if !ignore.contains(c)
            {
               // Java: buf[index++] = c;
               buf[index] = c;
               index += 1;
            }
         }
         // Java: return index == len ? str : String.valueOf(buf);
         return if index == len { s.to_string() } else { buf[..index].iter().collect() };
      }
      // Java: else
      {
         // Java: return str;
         return s.to_string();
      }
    }

    // Java: static String reverseWord(String str)
    pub fn reverseWord(s: &str) -> String {
      // Java: if (str == null)
        if false
      {
         // Java: return null;
         return String::new();
      }
      // Java: else
      {
         // Java: int len = str.length();
         let chars: Vec<char> = s.chars().collect();
         let len = chars.len();
         // Java: char[] buf = new char[len];
         let mut buf: Vec<char> = vec!['\0'; len];
         // Java: for(int i = 0; i < len; ++i)
            for i in 0..len
         {
            // Java: buf[i] = str.charAt(len - i - 1);
            buf[i] = chars[len - i - 1];
         }
         // Java: return String.valueOf(buf);
         buf.into_iter().collect()
      }
    }

    // Java: static String parseString(String str)
    pub fn parseString(s: &str) -> Option<String> {
      // Java: int delim = 0;
      let delim: i32 = 0;
      // Java: int piece1 = mystrsep(str, delim);
      let piece1 = Self::mystrsep(s, delim);
      // Java: if (piece1 < 0)
        if piece1 < 0
      {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: str = str.substring(piece1 + 1);
         let s2: String = s.chars().skip((piece1 + 1) as usize).collect();
         // Java: int piece2 = mystrsep(str, delim);
         let mut piece2 = Self::mystrsep(&s2, delim);
         // Java: if (piece2 < 0)
            if piece2 < 0
         {
            // Java: piece2 = str.length();
            piece2 = s2.len() as i32;
         }
         // Java: return str.substring(0, piece2);
         Some(s2.chars().take(piece2 as usize).collect())
      }
    }

    // Java: static int mystrsep(String str, int delim)
    pub fn mystrsep(s: &str, _delim: i32) -> i32 {
      // Java: return mystrsep(str, 0, 0);
      Self::mystrsep_with_start(s, 0, 0)
    }

    // Java: static int mystrsep(String str, int start, int delim)
    pub fn mystrsep_with_start(s: &str, start: i32, delim: i32) -> i32 {
      // Java: if (str != null && start >= 0)
        if start >= 0
      {
         // Java: int i = -1;
         let mut i: i32 = -1;
         // Java: if (delim != 0)
            if delim != 0
         {
            // Java: i = str.indexOf(delim);
            let delim_char = char::from_u32(delim as u32).unwrap_or('\0');
            i = s.find(delim_char).map(|p| p as i32).unwrap_or(-1);
         }
         // Java: else
            else
         {
            // Java: int len = str.length();
            let chars: Vec<char> = s.chars().collect();
            let len = chars.len();
            // Java: for(int j = start; j < len; ++j)
                for j in (start as usize)..len
            {
               // Java: char c = str.charAt(j);
               let c = chars[j];
               // Java: if (c == ' ' || c == '\t')
                    if c == ' ' || c == '\t'
               {
                  // Java: i = j;
                  i = j as i32;
                  // Java: break;
                  break;
               }
            }
         }
         // Java: return i;
         return i;
      }
      // Java: else
      {
         // Java: return -1;
         return -1;
      }
    }

    // Java: static ArrayList<String> mystrsepall(String str, int start, int delim, ArrayList<String> seps)
    pub fn mystrsepall_into(s: &str, start: i32, _delim: i32, seps: &mut Vec<String>) -> Vec<String> {
      // Java: seps.clear();
      seps.clear();
      // Java: int last = start;
      let mut last = start;
      // Java: int pos;
      let mut pos: i32;
      // Java: for(pos = mystrsep(str, start, 0); pos >= 0; pos = mystrsep(str, last, 0))
        pos = Self::mystrsep_with_start(s, start, 0);
        while pos >= 0
      {
         // Java: String sub = str.substring(last, pos);
         let chars: Vec<char> = s.chars().collect();
         let sub: String = chars[(last as usize)..(pos as usize)].iter().collect();
         // Java: if (sub != null && sub.length() > 0)
            if !sub.is_empty()
         {
            // Java: seps.add(sub);
            seps.push(sub);
         }
         // Java: last = pos + 1;
         last = pos + 1;
         pos = Self::mystrsep_with_start(s, last, 0);
      }
      // Java: if (pos < 0 && last > 0)
        if pos < 0 && last > 0
      {
         // Java: String sub = str.substring(last);
         let chars: Vec<char> = s.chars().collect();
         let sub: String = chars[(last as usize)..].iter().collect();
         // Java: if (sub != null && sub.length() > 0)
            if !sub.is_empty()
         {
            // Java: seps.add(sub);
            seps.push(sub);
         }
      }
      // Java: return seps;
      seps.clone()
    }

    // Java: static ArrayList<String> mystrsepall(String str, int start, int delim)
    pub fn mystrsepall(s: &str, start: i32, delim: i32) -> Vec<String> {
      // Java: ArrayList<String> seps = new ArrayList();
      let mut seps: Vec<String> = Vec::new();
      // Java: mystrsepall(str, start, delim, seps);
      Self::mystrsepall_into(s, start, delim, &mut seps);
      // Java: return seps;
      seps
    }

    // Java: static String makeAllCap(String str, CSInfo csconv)
    pub fn makeAllCap(s: &str, csconv: &CSInfo) -> Option<String> {
      // Java: if (str != null && csconv != null)
        if true
      {
         // Java: int len = str.length();
         let chars: Vec<char> = s.chars().collect();
         let len = chars.len();
         // Java: char[] buf = str.toCharArray();
         let mut buf = chars;
         // Java: for(int i = 0; i < len; ++i)
            for i in 0..len
         {
            // Java: buf[i] = csconv.getUpper(buf[i]);
            buf[i] = csconv.getUpper(buf[i]);
         }
         // Java: return String.valueOf(buf);
         return Some(buf.into_iter().collect());
      }
      // Java: else
      {
         // Java: return null;
         #[allow(unreachable_code)]
         return None;
      }
    }

    // Java: static String makeAllSmall(String str, CSInfo csconv)
    pub fn makeAllSmall(s: &str, csconv: &CSInfo) -> Option<String> {
      // Java: if (str != null && csconv != null)
        if true
      {
         // Java: int len = str.length();
         let chars: Vec<char> = s.chars().collect();
         let len = chars.len();
         // Java: char[] buf = str.toCharArray();
         let mut buf = chars;
         // Java: for(int i = 0; i < len; ++i)
            for i in 0..len
         {
            // Java: buf[i] = csconv.getLower(buf[i]);
            buf[i] = csconv.getLower(buf[i]);
         }
         // Java: return String.valueOf(buf);
         return Some(buf.into_iter().collect());
      }
      // Java: else
      {
         // Java: return null;
         #[allow(unreachable_code)]
         return None;
      }
    }

    // Java: static CSInfo getCurrentCS(String es)
    pub fn getCurrentCS(es: &str) -> CSInfo {
      // Java: CSInfo info = null;
      let mut info: Option<CSInfo> = None;
      // Java: Map<String, CSInfo> cs = getEndCS();
      let cs = Self::getEndCS().lock().unwrap();
      // Java: String normalizedEncoding = toAsciiLowerAndRemoveNonAlphanumeric(es);
      let normalized_encoding = Self::toAsciiLowerAndRemoveNonAlphanumeric(es);
      // Java: if (normalizedEncoding != null)
        if let Some(ref ne) = normalized_encoding
      {
         // Java: info = (CSInfo)cs.get(normalizedEncoding);
         info = cs.get(ne).cloned();
      }
      // Java: return info != null ? info : (CSInfo)cs.get("iso88591");
      info.unwrap_or_else(|| cs.get("iso88591").cloned().unwrap_or_else(|| CSInfo::new(vec![])))
    }

    // Java: static String toAsciiLowerAndRemoveNonAlphanumeric(String str)
    pub fn toAsciiLowerAndRemoveNonAlphanumeric(s: &str) -> Option<String> {
      // Java: if (str != null && str.length() != 0)
        if !s.is_empty()
      {
         // Java: int len = str.length();
         let chars: Vec<char> = s.chars().collect();
         let len = chars.len();
         // Java: char[] buf = new char[len];
         let mut buf: Vec<char> = vec!['\0'; len];
         // Java: int n = 0;
         let mut n: usize = 0;
         // Java: for(int i = 0; i < len; ++i)
            for i in 0..len
         {
            // Java: char c = str.charAt(i);
            let c = chars[i];
            // Java: if (c >= 'A' && c <= 'Z')
                if c >= 'A' && c <= 'Z'
            {
               // Java: buf[n++] = (char)(c + 32);
               buf[n] = (c as u8 + 32) as char;
               n += 1;
            }
            // Java: else if (c >= 'a' && c <= 'z' || c >= '0' && c <= '9')
                else if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9')
            {
               // Java: buf[n++] = c;
               buf[n] = c;
               n += 1;
            }
         }
         // Java: return String.valueOf(buf, 0, n);
         return Some(buf[..n].iter().collect());
      }
      // Java: else
      {
         // Java: return null;
         return None;
      }
    }

    // Java: static protected static final synchronized Map<String, CSInfo> getEndCS()
    fn getEndCS() -> &'static Mutex<HashMap<String, CSInfo>> {
      // Java: if (endcs == null)
        endcs.get_or_init(||
      {
         // Java: endcs = new HashMap();
         let mut map = HashMap::new();
         // Java: endcs.put("iso88591", new CSInfo(iso1_tbl));
         let iso1 = iso1_tbl.get_or_init(|| vec![]);
         map.insert("iso88591".to_string(), CSInfo::new(iso1.clone()));
         // Java: endcs.put("koi8r", new CSInfo(koi8r_tbl));
         let koi8r = koi8r_tbl.get_or_init(|| vec![]);
         map.insert("koi8r".to_string(), CSInfo::new(koi8r.clone()));
         Mutex::new(map)
      })
      // Java: return endcs;
    }

}
