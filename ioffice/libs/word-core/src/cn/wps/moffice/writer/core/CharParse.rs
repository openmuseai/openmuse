// Java: import cn.wps.moffice.writer.core.CharMap
use crate::cn::wps::moffice::writer::core::CharMap::{CharMap, Item as CharMapItem};

// java public static final int CC_Ascii = 1;
pub(crate) const CC_Ascii: i32 = 1;
// java public static final int CC_FarEast = 2;
pub(crate) const CC_FarEast: i32 = 2;
// java public static final int CC_NoFarEast = 3;
pub(crate) const CC_NoFarEast: i32 = 3;
// java public static final int CC_Shared = 4;
pub(crate) const CC_Shared: i32 = 4;
// java public static final int CC_Complex = 5;
pub(crate) const CC_Complex: i32 = 5;
// java public static final int CC_Unknown = 6;
pub(crate) const CC_Unknown: i32 = 6;
// java private static final int MASK_LOWER_16 = 65535;
const MASK_LOWER_16: i32 = 65535;
// java private static final int MAX_ASCII = 127;
const MAX_ASCII: i32 = 127;
// java private static final int MAX_CACHE = 5;
const MAX_CACHE: i32 = 5;
// java private static CharParse msCharParse = null;
// rust: ★wr-29 04 —— Java 静态单例；Rust getInstance 直接新建（CharParse 无跨调用状态，
//       缓存仅同实例加速）→ 语义等价（文档化）。
static msCharParse: std::sync::LazyLock<std::sync::Mutex<Option<CharParse>>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// java public final class CharParse
#[derive(Default)]
pub struct CharParse {
    // java CharCache[] mCacheData = new CharCache[5];
    pub(crate) mCacheData: Option<Vec<CharCache>>,
    // java int mCurCache = 0;
    pub(crate) mCurCache: i32,
    // java final CharMap mCharMap = CharMap.getInstance();
    pub(crate) mCharMap: Option<CharMap>,
    // java final CharMap.Item mCharAscii;
    pub(crate) mCharAscii: Option<CharMapItem>,
    // java final int mCharAsciiUnicodeId;
    pub(crate) mCharAsciiUnicodeId: i32,
    // java final int mCharAsciiCharType;
    pub(crate) mCharAsciiCharType: i32,
    // java public int[][] asiaTable = new int[][]{{8216, 8223}, {11904, 12703}, {12800, 40959}, {44032, 57343}, {63744, 64255}, {65056, 65135}, {65280, 65519}};
    pub(crate) asiaTable: Option<Vec<Vec<i32>>>,
    // java public int[][] rtlTable = new int[][]{{1536, 1791}, {1872, 1919}, {64336, 65023}, {65136, 65279}, {1424, 1535}};
    pub(crate) rtlTable: Option<Vec<Vec<i32>>>,
    // java public int[][] arabicTable = new int[][]{{1536, 1791}, {1872, 1919}, {64336, 65023}, {65136, 65279}};
    pub(crate) arabicTable: Option<Vec<Vec<i32>>>,
}

// java public static class CharInfo
#[derive(Default, Clone)]
pub struct CharInfo {
    // java public int unicodeCodeType = 0;
    pub(crate) unicodeCodeType: i32,
    // java public int charType = 6;
    pub(crate) charType: i32,
    // java public int codePoint;
    pub(crate) codePoint: i32,
}

// java static class CharCache
#[derive(Default, Clone)]
pub struct CharCache {
    // java CharMap.Item item = null;
    pub(crate) item: Option<CharMapItem>,
    // java int count = 0;
    pub(crate) count: i32,
}

// java public final class CharParse
impl CharParse {
    // java public  CharParse()
    pub fn CharParse() -> Self {
      // java this.mCharAscii = this.mCharMap.getCharItem((int)0);
      // java this.mCharAsciiUnicodeId = this.mCharAscii.unicodeId;
      // java this.mCharAsciiCharType = this.mCharAscii.getCharTypeUnchecked();
      let mut mCharMap = CharMap::getInstance();
      let mCharAscii = mCharMap.getCharItem__int(0).expect("CharMap ASCII entry");
      Self {
        mCacheData: Some(vec![CharCache::default(); 5]),
        mCurCache: 0,
        mCharMap: Some(mCharMap),
        mCharAscii: Some(mCharAscii),
        mCharAsciiUnicodeId: mCharAscii.unicodeId,
        mCharAsciiCharType: mCharAscii.getCharTypeUnchecked(),
        asiaTable: Some(vec![vec![8216, 8223], vec![11904, 12703], vec![12800, 40959], vec![44032, 57343], vec![63744, 64255], vec![65056, 65135], vec![65280, 65519]]),
        rtlTable: Some(vec![vec![1536, 1791], vec![1872, 1919], vec![64336, 65023], vec![65136, 65279], vec![1424, 1535]]),
        arabicTable: Some(vec![vec![1536, 1791], vec![1872, 1919], vec![64336, 65023], vec![65136, 65279]]),
      }
    }

    // java public static CharParse getInstance()
    pub fn getInstance() -> CharParse {
      // java synchronized (CharParse.class) { if (msCharParse == null) { msCharParse = new CharParse(); } return msCharParse; }
      // rust: 直接新建等价实例（注释见文件头）
      CharParse::CharParse()
    }

    // java private void _dispose()
    pub fn _dispose(&mut self) {
      // java this.asiaTable = null;
      self.asiaTable = None;
    }

    // java public static void dispose()
    pub fn dispose() {
      // java if (msCharParse != null) { msCharParse._dispose(); msCharParse = null; }
      let mut guard = msCharParse.lock().unwrap();
      if let Some(mut inner) = guard.take() {
        inner._dispose();
      }
    }

    // java public boolean isWhitespace(char ch)
    pub fn isWhitespace(&self, ch: char) -> bool {
      // java return Character.isWhitespace(ch) || '\f' == ch || 1 == ch || 2 == ch || 3 == ch || 4 == ch || 5 == ch || 7 == ch || '\b' == ch || 19 == ch || 20 == ch || 21 == ch;
      // java return Character.isWhitespace(ch) || '\f' == ch || 1 == ch || ... || 21 == ch;
      let u: u32 = ch as u32;
      return ch.is_whitespace() || '\u{000c}' == ch || matches!(u, 1 | 2 | 3 | 4 | 5 | 7 | 19 | 20 | 21) || '\u{0008}' == ch;
    }

    // java public boolean isNewPage(char ch)
    pub fn isNewPage(&self, ch: char) -> bool {
      // java return '\f' == ch;
      return '\u{000c}' == ch;
    }

    // java public boolean isSpace(char ch)
    pub fn isSpace(&self, ch: char) -> bool {
      // java return ' ' == ch || 12288 == ch || 1 == ch || 2 == ch || 3 == ch || 4 == ch || 5 == ch || '\b' == ch || 19 == ch || 20 == ch || 21 == ch;
      let u: u32 = ch as u32;
      return ' ' == ch || matches!(u, 12288 | 1 | 2 | 3 | 4 | 5 | 19 | 20 | 21) || '\u{0008}' == ch;
    }

    // java public boolean isTab(char ch)
    pub fn isTab(&self, ch: char) -> bool {
      // java return '\t' == ch;
      return '\t' == ch;
    }

    // java public boolean isNewLine(char ch)
    pub fn isNewLine(&self, ch: char) -> bool {
      // java return 11 == ch || 7 == ch;
      return matches!(ch as u32, 11 | 7);
    }

    // java public boolean isParagraphBreak(char ch)
    pub fn isParagraphBreak(&self, ch: char) -> bool {
      // java return '\r' == ch;
      return '\u{000d}' == ch;
    }

    // java public boolean charIsAsia(char ch, int hint)
    pub fn charIsAsia(&mut self, ch: char, hint: i32) -> bool {
      // java if (this.charIsAscii(ch))
      if self.charIsAscii(ch) {
        // java return false;
        return false;
      }
      // java else if (hint == 1 && this.charIsAsia2(ch))
      else if hint == 1 && self.charIsAsia2(ch) {
        // java return true;
        return true;
      }
      // java else
      else {
        // java int charType = this.getCharType(ch);
        // Java CharMap table is wired; absent entries still map to Unknown.
        let charType: i32 = self.getCharType(ch).unwrap_or(6);
        // java return charType == 2;
        return charType == 2;
      }
    }

    // java public boolean isLetter(char ch)
    pub fn isLetter(&self, ch: char) -> bool {
      // java return 'A' <= ch && ch <= 'Z' || 'a' <= ch && ch <= 'z';
      return 'A' <= ch && ch <= 'Z' || 'a' <= ch && ch <= 'z';
    }

    // java public boolean isNumber(char c)
    pub fn isNumber(&self, c: char) -> bool {
      // java return c >= '0' && c <= '9';
      return c >= '0' && c <= '9';
    }

    // java public int getCharType(char ch)
    pub fn getCharType(&mut self, ch: char) -> Option<i32> {
      // java if (ch <= 127)
      if (ch as u32) <= 127 {
        // java return this.mCharAsciiCharType;
        return Some(self.mCharAsciiCharType);
      }
      // java else
      else {
        // java CharMap.Item item = this.mCharMap.getCharItem(ch);
        let item: Option<CharMapItem> = self.mCharMap.as_mut().and_then(|m| m.getCharItem__char(ch));
        // java return null == item ? 6 : item.getCharType(ch);
        return item.as_ref().map(|item| item.getCharType((ch as u32) as i32)).or(Some(6));
      }
    }

    // java public boolean charIsAscii(char ch)
    pub fn charIsAscii(&mut self, ch: char) -> bool {
      // java return 0 <= ch && ch <= 127;
      return (ch as u32) <= 127;
    }

    // java public boolean isSymbolChar(int codePoint)
    pub fn isSymbolChar(&self, codePoint: i32) -> bool {
      // java return 61440 <= codePoint && codePoint <= 61567;
      return 61440 <= codePoint && codePoint <= 61567;
    }

    // java public boolean charIsShared(char ch)
    pub fn charIsShared(&mut self, ch: char) -> bool {
      // java return this.getCharType(ch) == 4;
      return self.getCharType(ch) == Some(4);
    }

    // java private final boolean isInTable(int[][] table, char ch)
    pub fn isInTable(&self, table: &Vec<Vec<i32>>, ch: char) -> bool {
      // java for(int[] row : table)
      for row in table.iter() {
        // java if (ch >= row[0] && ch <= row[1])
        if (ch as u32) >= (row[0usize] as u32) && (ch as u32) <= (row[1usize] as u32) {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
    }

    // java public boolean charIsAsia2(char ch)
    pub fn charIsAsia2(&self, ch: char) -> bool {
      // java return this.isInTable(this.asiaTable, ch);
      return self.asiaTable.as_ref().map(|t| self.isInTable(t, ch)).unwrap_or(false);
    }

    // java public boolean charIsRTL(char ch)
    pub fn charIsRTL(&self, ch: char) -> bool {
      // java return this.isInTable(this.rtlTable, ch);
      return self.rtlTable.as_ref().map(|t| self.isInTable(t, ch)).unwrap_or(false);
    }

    // java public boolean charIsArabic(char ch)
    pub fn charIsArabic(&self, ch: char) -> bool {
      // java return this.isInTable(this.arabicTable, ch);
      return self.arabicTable.as_ref().map(|t| self.isInTable(t, ch)).unwrap_or(false);
    }

    // java public boolean charIsMirrored(char ch)
    pub fn charIsMirrored(&mut self, ch: char) -> bool {
      // java return Character.isMirrored(ch);
      // rust: CHARMAP_DEFERRED —— 镜像表未接线；BMP 常用镜像符号子集
      return matches!(ch, '(' | ')' | '<' | '>' | '[' | ']' | '{' | '}' | '\u{00AB}' | '\u{00BB}');
    }

    // java public static boolean charIsThai(char ch)
    pub fn charIsThai(ch: char) -> bool {
      // java return ch >= 3584 && ch < 3712;
      let u: u32 = ch as u32;
      return u >= 3584 && u < 3712;
    }

    // java public void getCharInfo(CharInfo charInfo, int codePoint)
    pub fn getCharInfo(&mut self, charInfo: &mut CharInfo, codePoint: i32) {
      // java this.getCharInfoUseCache(charInfo, codePoint, false);
      self.getCharInfoUseCache(charInfo, codePoint, false);
    }

    // java public void getCharInfoUseCache(CharInfo charInfo, int codePoint, boolean useCache)
    pub fn getCharInfoUseCache(&mut self, charInfo: &mut CharInfo, codePoint: i32, useCache: bool) {
      // java charInfo.codePoint = codePoint;
      charInfo.codePoint = codePoint;
      // java if (codePoint <= 127)
      if codePoint <= 127 {
        // java charInfo.unicodeCodeType = this.mCharAsciiUnicodeId;
        charInfo.unicodeCodeType = self.mCharAsciiUnicodeId;
        // java charInfo.charType = this.mCharAsciiCharType;
        charInfo.charType = self.mCharAsciiCharType;
      }
      // java else
      else {
        // java if (useCache)
        if useCache {
          // java CharCache cache = this.findInCache(codePoint);
          let cache: Option<CharCache> = self.findInCache__int(codePoint);
          // java if (null != cache)
          if let Some(cache) = cache {
            // java charInfo.unicodeCodeType = cache.item.unicodeId;
            // java charInfo.charType = cache.item.getCharType(codePoint);
            // java return;
            if let Some(item) = cache.item {
              charInfo.unicodeCodeType = item.unicodeId;
              charInfo.charType = item.getCharType(codePoint);
              return;
            }
          }
        }
        // java CharMap.Item item = this.mCharMap.getCharItem(codePoint);
        let item: Option<CharMapItem> = self.mCharMap.as_mut().and_then(|m| m.getCharItem__int(codePoint));
        // java if (null == item)
        if item.is_none() {
          // java charInfo.unicodeCodeType = 255;
          charInfo.unicodeCodeType = 255;
          // java charInfo.charType = 6;
          charInfo.charType = 6;
        }
        // java else
        else {
          // java charInfo.unicodeCodeType = item.unicodeId;
          // java charInfo.charType = item.getCharType(codePoint);
          if let Some(item) = &item {
            charInfo.unicodeCodeType = item.unicodeId;
            charInfo.charType = item.getCharType(codePoint);
            // java if (useCache) { this.addToCache(item); }
            if useCache {
              self.addToCache(item);
            }
          }
        }
      }
    }

    // java  CharCache findInCache(char ch)
    pub fn findInCache__char(&mut self, ch: char) -> Option<CharCache> {
      // java int codePoint = ch & '\uffff';
      let codePoint: i32 = (ch as u32 & 0xFFFF) as i32;
      // java return this.findInCache(codePoint);
      return self.findInCache__int(codePoint);
    }

    // java  CharCache findInCache(int codePoint)
    pub fn findInCache__int(&mut self, codePoint: i32) -> Option<CharCache> {
      // java for(int i = 0; i < this.mCurCache; ++i)
      for i in 0..self.mCurCache as usize {
        let cache = self.mCacheData.as_mut()?.get_mut(i)?;
        // java if (codePoint >= cache.item.start && codePoint <= cache.item.end)
        if let Some(item) = cache.item {
          if codePoint >= item.start && codePoint <= item.end {
            // java ++cache.count;
            cache.count += 1;
            let found = cache.clone();
            // java if (i > 0) { this.bubbleUp(i); }
            if i > 0 { self.bubbleUp(i as i32); }
            // java return cache;
            return Some(found);
          }
        }
      }
      // java return null;
      None
    }


    // java final void bubbleUp(int i)
    pub fn bubbleUp(&mut self, mut i: i32) {
      // java while(true)
      while i > 0 {
        let data = self.mCacheData.as_mut().expect("CharParse cache");
        // java if (cachePrev.count < cache.count) { swap; --i; continue; }
        if data[(i - 1) as usize].count < data[i as usize].count {
          data.swap((i - 1) as usize, i as usize);
          i -= 1;
          continue;
        }
        // java return;
        return;
      }
    }


    // java final void addToCache(CharMap.Item item)
    pub fn addToCache(&mut self, item: &CharMapItem) {
      let data = self.mCacheData.as_mut().expect("CharParse cache");
      // java if (this.mCurCache >= 5)
      if self.mCurCache >= 5 {
        // java CharCache cache = this.mCacheData[this.mCurCache - 1]; cache.count = 0; cache.item = item;
        let cache = &mut data[(self.mCurCache - 1) as usize];
        cache.count = 0;
        cache.item = Some(*item);
      } else {
        // java this.mCacheData[this.mCurCache++] = new CharCache(item);
        data[self.mCurCache as usize] = CharCache::CharCache(item);
        self.mCurCache += 1;
      }
    }


    // java public boolean charIsMirroredPunct(char ch)
    pub fn charIsMirroredPunct(&mut self, ch: char) -> bool {
      // java if (ch >= 12296 && ch <= 12305)
      let u: u32 = ch as u32;
      if u >= 12296 && u <= 12305 {
        // java return true;
        return true;
      }
      // java else if (ch >= 12308 && ch <= 12315)
      else if u >= 12308 && u <= 12315 {
        // java return true;
        return true;
      }
      // java else
      else {
        // java switch (ch)
        match ch {
          // java case '（': case '）': case '［': case '］': case '｛': case '｝': case '｟': case '｠': case '｢': case '｣':
          '（' | '）' | '［' | '］' | '｛' | '｝' | '｟' | '｠' | '｢' | '｣' => {
            // java return true;
            return true;
          }
          // java default:
          _ => {
            // java return false;
            return false;
          }
        }
      }
    }

}

// java public static class CharInfo
impl CharInfo {
}

// java static class CharCache
impl CharCache {
    // java   CharCache(CharMap.Item item)
    pub fn CharCache(item: &CharMapItem) -> Self {
      // java this.item = item;
      Self { item: Some(*item), count: 0 }
    }

}
