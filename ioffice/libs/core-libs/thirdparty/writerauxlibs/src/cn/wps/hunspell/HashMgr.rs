// Java: import java.io.IOException
use super::Hentry::Hentry;

// Java field: int FLAG_CHAR
pub(crate) const FLAG_CHAR: i32 = 0;
// Java field: int FLAG_LONG
pub(crate) const FLAG_LONG: i32 = 1;
// Java field: int FLAG_NUM
pub(crate) const FLAG_NUM: i32 = 2;
// Java field: int FLAG_UNI
pub(crate) const FLAG_UNI: i32 = 3;
// Java field: int USERNUM
pub(crate) const USERNUM: i32 = 1000;

// Java: public class HashMgr
pub struct HashMgr {
    // Java field: int mTableSize
    pub(crate) mTableSize: i32,
    // Java field: Hentry[] mTables
    pub(crate) mTables: Vec<Option<Hentry>>,
    // Java field: char mForbiddenWord
    pub(crate) mForbiddenWord: char,
    // Java field: int mFlagMode
    pub(crate) mFlagMode: i32,
    // Java field: boolean mComplexPrefixes
    pub(crate) mComplexPrefixes: bool,
    // Java field: String mIgnoreChars
    pub(crate) mIgnoreChars: Option<String>,
}

// Java: public class HashMgr
impl HashMgr {
    // Java: HashMgr(AffixMgr aff, String dic)
    pub fn new_from_aff(aff: i32, dic: &str) -> Self {
      // Java: super();
      todo!("waiting for AffixMgr")
      // Java: this.init(aff, dic);
    }

    // Java: HashMgr(HashMgr hashMgr, String dic)
    pub fn new_from_hash(hash_mgr: &HashMgr, dic: &str) -> Self {
      // Java: super();
      let mut mgr = Self {
          mTableSize: 0,
          mTables: Vec::new(),
          mForbiddenWord: '￦',
          mFlagMode: 0,
          mComplexPrefixes: false,
          mIgnoreChars: None,
      };
      // Java: this.init(hashMgr, dic);
      mgr.init_from_hash(hash_mgr, dic);
      mgr
    }

    // Java: void addWord(String word)
    pub fn addWord(&mut self, word: &str) {
      // Java: this.addWord(word, (char[])null, false);
      self.addWord_with_flags(word, None, false);
    }

    // Java: boolean init(AffixMgr aff, String dic)
    fn init_from_aff(&mut self, _aff: i32, _dic: &str) -> bool {
      // Java: this.mTableSize = 0;
      self.mTableSize = 0;
      // Java: this.loadConfigs(aff);
      todo!("waiting for AffixMgr")
      // Java: return this.loadTables(dic);
    }

    // Java: boolean init(HashMgr hashMgr, String dic)
    fn init_from_hash(&mut self, hash_mgr: &HashMgr, dic: &str) -> bool {
      // Java: this.mTableSize = 0;
      self.mTableSize = 0;
      // Java: this.loadConfigs(hashMgr);
      self.loadConfigs_from_hash(hash_mgr);
      // Java: return this.loadTables(dic);
      self.loadTables(dic)
    }

    // Java: void loadConfigs(HashMgr hashMgr)
    fn loadConfigs_from_hash(&mut self, hash_mgr: &HashMgr) {
      // Java: if (hashMgr != null)
      {
         // Java: this.mFlagMode = hashMgr.mFlagMode;
         self.mFlagMode = hash_mgr.mFlagMode;
         // Java: this.mComplexPrefixes = hashMgr.mComplexPrefixes;
         self.mComplexPrefixes = hash_mgr.mComplexPrefixes;
         // Java: this.mForbiddenWord = hashMgr.mForbiddenWord;
         self.mForbiddenWord = hash_mgr.mForbiddenWord;
         // Java: this.mIgnoreChars = hashMgr.mIgnoreChars;
         self.mIgnoreChars = hash_mgr.mIgnoreChars.clone();
      }
    }

    // Java: void loadConfigs(AffixMgr aff)
    fn loadConfigs_from_aff(&mut self, _aff: i32) {
      // Java: if (aff != null)
      {
         // Java: this.mComplexPrefixes = aff.mComplexPrefixes;
         todo!("waiting for AffixMgr")
         // Java: this.mForbiddenWord = aff.mForbiddenWord;
         // Java: this.mIgnoreChars = aff.mIgnoreChars;
      }
    }

    // Java: boolean loadTables(String dic)
    fn loadTables(&mut self, dic: &str) -> bool {
      // Java: FileMgr dict = null;
      let mut dict_lines: Vec<String> = Vec::new();
      let mut dict_idx: usize = 0;
      // Java: try
      {
         // Java: dict = new FileMgr(dic);
         if let Ok(content) = std::fs::read_to_string(dic) {
             dict_lines = content.lines().map(|s| s.to_string()).collect();
         }
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
      }
      // Java: if (dict == null)
      if dict_lines.is_empty()
      {
         // Java: return false;
         return false;
      }
      // Java: else
      {
         // Java: String ts = dict.getLine();
         let mut ts: String = if dict_idx < dict_lines.len() {
             let s = dict_lines[dict_idx].clone();
             dict_idx += 1;
             s
         } else {
             String::new()
         };
         // Java: if (ts == null)
         if ts.is_empty() && dict_idx > dict_lines.len()
         {
            // Java: dict.close();
            // Java: return false;
            return false;
         }
         // Java: else
         {
            // Java: ts = CSUtil.removeByteOrderMark(ts);
            if ts.len() >= 3 && ts.as_bytes().get(0) == Some(&0xEF) && ts.as_bytes().get(1) == Some(&0xBB) && ts.as_bytes().get(2) == Some(&0xBF) {
                ts = ts[3..].to_string();
            }
            // Java: if (CSUtil.isAllNum(ts))
            if !ts.is_empty() && ts.chars().all(|c| c == ',' || c == '.' || c == '-' || (c >= '0' && c <= '9'))
            {
               // Java: this.mTableSize = Integer.parseInt(ts);
               self.mTableSize = ts.parse::<i32>().unwrap_or(0);
               // Java: if (this.mTableSize <= 0)
               if self.mTableSize <= 0
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
            else
            {
               // Java: this.mTableSize = 1000;
               self.mTableSize = 1000;
            }
            // Java: if (this.mTableSize < 1000)
            if self.mTableSize < 1000
            {
               // Java: this.mTableSize = 1000;
               self.mTableSize = 1000;
            }
            // Java: this.mTables = new Hentry[this.mTableSize];
            self.mTables = (0..self.mTableSize as usize).map(|_| None).collect();
            // Java: char sep = '/';
            let sep = '/';
            // Java: char[] as;
            let mut aff_flags: Option<Vec<char>>;
            // Java: for(as = null; (ts = dict.getLine()) != null; this.addWord(ts, as, false))
            while dict_idx < dict_lines.len()
            {
               ts = dict_lines[dict_idx].clone();
               dict_idx += 1;
               // Java: as = null;
               aff_flags = None;
               // Java: int index = ts.indexOf(sep);
               let index = ts.find(sep).map(|i| i as i32).unwrap_or(-1);
               // Java: if (index > 0)
               if index > 0
               {
                  // Java: int start = index + 1;
                  let start = (index + 1) as usize;
                  // Java: int end = ts.length();
                  let end = ts.len();
                  // Java: as = new char[end - start];
                  // Java: ts.getChars(start, end, as, 0);
                  aff_flags = Some(ts[start..end].chars().collect());
                  // Java: ts = ts.substring(0, index);
                  ts = ts[..index as usize].to_string();
               }
               self.addWord_with_flags(&ts, aff_flags.as_deref(), false);
            }
            // Java: dict.close();
            // Java: return true;
            return true;
         }
      }
    }

    // Java: boolean addWord(String ts, char[] as, boolean onlyupcase)
    fn addWord_with_flags(&mut self, ts: &str, aff_flags: Option<&[char]>, _onlyupcase: bool) -> bool {
      let mut ts = ts.to_string();
      // Java: if (this.mIgnoreChars != null)
      if let Some(ref ignore_chars) = self.mIgnoreChars.clone()
      {
         // Java: ts = CSUtil.removeIgnoredChars(ts, this.mIgnoreChars);
         let len = ts.len();
         let buf: String = ts.chars().filter(|c| !ignore_chars.contains(*c)).collect();
         ts = buf;
      }
      // Java: if (this.mComplexPrefixes)
      if self.mComplexPrefixes
      {
         // Java: ts = CSUtil.reverseWord(ts);
         ts = ts.chars().rev().collect();
      }
      // Java: int hash = this.hash(ts);
      let hash = self.hash(&ts) as usize;
      // Java: Hentry hentry = new Hentry(ts, as);
      let aff_vec = aff_flags.map(|f| f.to_vec()).unwrap_or_default();
      let hentry = Hentry::new(ts, aff_vec);
      // Java: Hentry dp = this.mTables[hash];
      // Java: if (dp == null)
      if self.mTables[hash].is_none()
      {
         // Java: this.mTables[hash] = hentry;
         self.mTables[hash] = Some(hentry);
         // Java: return true;
         return true;
      }
      // Java: else
      {
         // Java: while(dp.next != null)
         let mut dp = self.mTables[hash].as_mut().unwrap();
         while dp.next.is_some()
         {
            // Java: dp = dp.next;
            dp = dp.next.as_mut().unwrap();
         }
         // Java: dp.next = hentry;
         dp.next = Some(Box::new(hentry));
         // Java: return true;
         return true;
      }
    }

    // Java: boolean testAff(char[] conts, char flag)
    pub fn testAff(&self, conts: &[char], flag: char) -> bool {
      // Java: return CSUtil.flagBsearch(conts, flag, conts.length) != 0;
      Self::flag_bsearch(conts, flag, conts.len() as i32) != 0
    }

    // Java: boolean testAff(String aff, char flag)
    pub fn testAff_str(&self, aff: &str, flag: char) -> bool {
      // Java: return CSUtil.flagBsearch(aff.toCharArray(), flag, aff.length()) != 0;
      let chars: Vec<char> = aff.chars().collect();
      Self::flag_bsearch(&chars, flag, chars.len() as i32) != 0
    }

    // Java: int hash(String str)
    fn hash(&self, s: &str) -> i32 {
      // Java: int hash = str.hashCode();
      let mut hash: i32 = 0;
      for c in s.chars() {
          hash = hash.wrapping_mul(31).wrapping_add(c as i32);
      }
      // Java: return hash >= 0 ? hash % this.mTableSize : -hash % this.mTableSize;
      if hash >= 0 { hash % self.mTableSize } else { (-hash) % self.mTableSize }
    }

    // Java: Hentry lookup(String word)
    pub fn lookup(&self, word: &str) -> Option<&Hentry> {
      // Java: int hash = this.hash(word);
      let hash = self.hash(word) as usize;
      // Java: for(Hentry hentry = this.mTables[hash]; hentry != null; hentry = hentry.next)
      let mut current = self.mTables[hash].as_ref();
      while let Some(hentry) = current
      {
         // Java: if (word.equals(hentry.word))
         if word == hentry.word
         {
            // Java: return hentry;
            return Some(hentry);
         }
         current = hentry.next.as_ref().map(|b| b.as_ref());
      }
      // Java: return null;
      None
    }

    // Java: boolean isAliasf()
    pub fn isAliasf(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: boolean isAliasm()
    pub fn isAliasm(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: String getAliasm(int index)
    pub fn getAliasm(&self, _index: i32) -> Option<String> {
      // Java: return null;
      None
    }

    fn flag_bsearch(flags: &[char], flag: char, length: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = length - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if flags[mid as usize] == flag {
                return 1;
            }
            if flag < flags[mid as usize] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        0
    }
}
