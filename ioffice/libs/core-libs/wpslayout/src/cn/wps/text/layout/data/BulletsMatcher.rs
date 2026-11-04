// Java: import cn.wps.font.FontHost
// Java: import cn.wps.font.IFontFamily
// Java: import cn.wps.font.IFontTable

use std::sync::{Mutex, OnceLock};

pub struct FontHost;
impl FontHost {
    pub fn getFontTable() -> FontTableStub { FontTableStub }
}

pub struct FontTableStub;
impl FontTableStub {
    pub fn get(&self, _name: &str, _default: bool) -> FontFamilyStub { FontFamilyStub { name: String::new() } }
}

pub struct FontFamilyStub { name: String }
impl FontFamilyStub {
    pub fn getName(&self) -> &str { &self.name }
}

// Java field: char SOLID_CIRCLE_WPS_SPECIAL_1
pub(crate) const SOLID_CIRCLE_WPS_SPECIAL_1: char = '\u{f06c}';
// Java field: char SOLID_BIG_RECT_WPS_SPECIAL_1
pub(crate) const SOLID_BIG_RECT_WPS_SPECIAL_1: char = '\u{f06e}';
// Java field: char SOLID_DIAMOND_WPS_SPECIAL_1
pub(crate) const SOLID_DIAMOND_WPS_SPECIAL_1: char = '\u{f075}';
// Java field: char FOUR_DIAMOND_WPS_SPECIAL_1
pub(crate) const FOUR_DIAMOND_WPS_SPECIAL_1: char = '\u{f076}';
// Java field: char SOLID_SMALL_RECT_WPS_SPECIAL_1
pub(crate) const SOLID_SMALL_RECT_WPS_SPECIAL_1: char = '\u{f0a7}';
// Java field: char GUILLEMOTLEFT_WPS_SPECIAL_1
pub(crate) const GUILLEMOTLEFT_WPS_SPECIAL_1: char = '\u{f0ab}';
// Java field: char TWOSUPERIOR_WPS_SPECIAL_1
pub(crate) const TWOSUPERIOR_WPS_SPECIAL_1: char = '\u{f0b2}';
// Java field: char OSLASH_WPS_SPECIAL_1
pub(crate) const OSLASH_WPS_SPECIAL_1: char = '\u{f0d8}';
// Java field: char UDIERESIS_WPS_SPECIAL_1
pub(crate) const UDIERESIS_WPS_SPECIAL_1: char = '\u{f0fc}';
// Java field: char EXCLAMDOWN_WPS_SPECIAL_1
pub(crate) const EXCLAMDOWN_WPS_SPECIAL_1: char = '\u{f0a1}';
// Java field: char HOLLOW_BOX_WPS_SPECIAL_1
pub(crate) const HOLLOW_BOX_WPS_SPECIAL_1: char = '\u{f0a8}';
// Java field: char HOLLOW_BOX4_WPS_SPECIAL_1
pub(crate) const HOLLOW_BOX4_WPS_SPECIAL_1: char = '\u{f070}';
// Java field: char UNDEF_WPS_SPECIAL_1
pub(crate) const UNDEF_WPS_SPECIAL_1: char = '\u{f0ff}';
// Java field: char HOLLOW_LESS_TRIANGLE_WPS_SPECIAL_2
pub(crate) const HOLLOW_LESS_TRIANGLE_WPS_SPECIAL_2: char = '\u{f03c}';
// Java field: char LATIN_CAPITAL_THORN_WPS_SPECIAL_3
pub(crate) const LATIN_CAPITAL_THORN_WPS_SPECIAL_3: char = '\u{f0de}';
// Java field: char LATIN_CAPITAL_D_WPS_SPECIAL_3
pub(crate) const LATIN_CAPITAL_D_WPS_SPECIAL_3: char = '\u{f044}';
// Java field: char HOLLOW_BOX_SHADOW_WINGDINGS
pub(crate) const HOLLOW_BOX_SHADOW_WINGDINGS: char = '\u{f071}';
// Java field: char OSLASH_DOWN_WINGDINGS
pub(crate) const OSLASH_DOWN_WINGDINGS: char = '\u{f0da}';
// Java field: char SOLID_CIRCLE_WINGDINGS_2
pub(crate) const SOLID_CIRCLE_WINGDINGS_2: char = '\u{f097}';
// Java field: char HEX_STAR_WINGDINGS_2
pub(crate) const HEX_STAR_WINGDINGS_2: char = '\u{f0df}';
// Java field: char FIVE_STAR_WINGDINGS_2
pub(crate) const FIVE_STAR_WINGDINGS_2: char = '\u{f0d4}';
// Java field: char RIGHT_LABEL_WINGDINGS_3
pub(crate) const RIGHT_LABEL_WINGDINGS_3: char = '\u{f0b4}';
// Java field: char LEFT_BLACK_SOLID_TRIANGLE_WINGDINGS_3
pub(crate) const LEFT_BLACK_SOLID_TRIANGLE_WINGDINGS_3: char = '\u{f074}';
// Java field: char UP_TRIANGLE_WEBDINGS
pub(crate) const UP_TRIANGLE_WEBDINGS: char = '\u{f0ea}';
// Java field: String WPS_SPECIAL_1
pub(crate) const WPS_SPECIAL_1: &str = "Wingdings";
// Java field: String WEBDINGS
pub(crate) const WEBDINGS: &str = "Webdings";
// Java field: String WINGDINGS_2
pub(crate) const WINGDINGS_2: &str = "Wingdings 2";
// Java field: String WINGDINGS_3
pub(crate) const WINGDINGS_3: &str = "Wingdings 3";
// Java field: String COMMERCIALPI_BT
pub(crate) const COMMERCIALPI_BT: &str = "CommercialPi BT";
// Java field: String AIGDT
pub(crate) const AIGDT: &str = "AIGDT";
// Java field: String ARIAL
pub(crate) const ARIAL: &str = "Arial";
// Java field: String MONOTYPE_SORTS
pub(crate) const MONOTYPE_SORTS: &str = "Monotype Sorts";
// Java field: int MAX_POOL_SIZE
pub(crate) const MAX_POOL_SIZE: i32 = 2;
// Java field: Object mPoolSync
// Java field: int mPoolIndex
// Java field: BulletsMatcher mPool
static POOL: OnceLock<Mutex<(Vec<BulletsMatcher>, i32)>> = OnceLock::new();

fn get_pool() -> &'static Mutex<(Vec<BulletsMatcher>, i32)> {
    POOL.get_or_init(|| Mutex::new((Vec::new(), 0)))
}

// Java: public class BulletsMatcher
pub struct BulletsMatcher {
    // Java field: BulletsMatcher mNext
    // Java field: boolean mRecycled
    mRecycled: bool,
    // Java field: String mMatchedFontName
    pub(crate) mMatchedFontName: Option<String>,
    // Java field: String mMatchedBullet
    pub(crate) mMatchedBullet: Option<String>,
}

// Java: public class BulletsMatcher
impl BulletsMatcher {
    // Java: String getMatchedFontName()
    pub fn getMatchedFontName(&self) -> Option<&str> {
      // Java: return this.mMatchedFontName;
      self.mMatchedFontName.as_deref()
    }

    // Java: String getMatchedBullet()
    pub fn getMatchedBullet(&self) -> Option<&str> {
      // Java: return this.mMatchedBullet;
      self.mMatchedBullet.as_deref()
    }

    // Java: BulletsMatcher()
    pub fn new() -> Self {
      // Java: super();
      BulletsMatcher { mRecycled: false, mMatchedFontName: None, mMatchedBullet: None }
    }

    // Java: static BulletsMatcher obtain()
    pub fn obtain() -> BulletsMatcher {
      // Java: synchronized(mPoolSync)
      let mut pool = get_pool().lock().unwrap();
      {
         // Java: if (mPool != null)
         if let Some(mut m) = pool.0.pop()
         {
            // Java: BulletsMatcher m = mPool;
            // Java: mPool = m.mNext;
            // Java: m.mNext = null;
            // Java: m.mRecycled = false;
            m.mRecycled = false;
            // Java: --mPoolIndex;
            pool.1 -= 1;
            // Java: return m;
            return m;
         }
      }
      // Java: return new BulletsMatcher();
      BulletsMatcher::new()
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
      // Java: synchronized(mPoolSync)
      let mut pool = get_pool().lock().unwrap();
      {
         // Java: if (!this.mRecycled && mPoolIndex < 2)
         if !self.mRecycled && pool.1 < 2
         {
            // Java: this.clear();
            self.clear();
            // Java: this.mNext = mPool;
            // Java: mPool = this;
            // Java: ++mPoolIndex;
            pool.1 += 1;
            // Java: this.mRecycled = true;
            self.mRecycled = true;
         }
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mMatchedFontName = null;
      self.mMatchedFontName = None;
      // Java: this.mMatchedBullet = null;
      self.mMatchedBullet = None;
    }

    // Java: boolean match(String fontName, String bulletStr)
    pub fn r#match(&mut self, fontName: &str, bulletStr: &str) -> bool {
      // Java: IFontTable fontTable = FontHost.getFontTable();
      let fontTable = FontHost::getFontTable();
      // Java: IFontFamily fontFamily = fontTable.get(fontName, true);
      let fontFamily = fontTable.get(fontName, true);
      // Java: if (fontFamily.getName().equals(fontName))
      if fontFamily.getName() == fontName
      {
         // Java: return false;
         return false;
      }
      // Java: else
      {
         // Java: assert bulletStr != null && bulletStr.length() > 0;
         debug_assert!(!bulletStr.is_empty());
         // Java: boolean matched = true;
         let mut matched = true;
         // Java: char c = bulletStr.charAt(0);
         let c = bulletStr.chars().next().unwrap();
         // Java: int key = Integer.valueOf(c);
         let key = c as i32;
         // Java: this.mMatchedBullet = bulletStr;
         self.mMatchedBullet = Some(bulletStr.to_string());
         // Java: this.mMatchedFontName = fontName;
         self.mMatchedFontName = Some(fontName.to_string());
         // Java: if (fontName.equals("AIGDT"))
         if fontName == "AIGDT"
         {
            // Java: switch (key)
            match key
            {
               // Java: case 108:
               108 => {
                  // Java: this.mMatchedBullet = String.valueOf('⊙');
                  self.mMatchedBullet = Some(String::from("\u{2299}"));
                  // Java: break;
               }
               // Java: default:
               _ => {
                  // Java: matched = false;
                  matched = false;
               }
            }
         }
         // Java: else if (fontName.equals("CommercialPi BT"))
         else if fontName == "CommercialPi BT"
         {
            // Java: switch (key)
            match key
            {
               // Java: case 38:
               38 => {
                  // Java: this.mMatchedBullet = String.valueOf('●');
                  self.mMatchedBullet = Some(String::from("\u{25cf}"));
                  // Java: break;
               }
               // Java: default:
               _ => {
                  // Java: matched = false;
                  matched = false;
               }
            }
         }
         // Java: else if (fontName.equals("Arial"))
         else if fontName == "Arial"
         {
            // Java: switch (key)
            match key
            {
               // Java: case 187:
               187 => {
                  // Java: this.mMatchedBullet = String.valueOf('»');
                  self.mMatchedBullet = Some(String::from("\u{00bb}"));
                  // Java: break;
               }
               // Java: case 8211:
               8211 => {
                  // Java: this.mMatchedBullet = String.valueOf('–');
                  self.mMatchedBullet = Some(String::from("\u{2013}"));
                  // Java: break;
               }
               // Java: case 8226:
               8226 => {
                  // Java: this.mMatchedBullet = String.valueOf('•');
                  self.mMatchedBullet = Some(String::from("\u{2022}"));
                  // Java: break;
               }
               // Java: case 9642:
               9642 => {
                  // Java: this.mMatchedBullet = String.valueOf('▪');
                  self.mMatchedBullet = Some(String::from("\u{25aa}"));
                  // Java: break;
               }
               // Java: default:
               _ => {
                  // Java: matched = false;
                  matched = false;
               }
            }
         }
         // Java: else if (fontName.equals("Monotype Sorts"))
         else if fontName == "Monotype Sorts"
         {
            // Java: switch (key)
            match key
            {
               // Java: case 108:
               108 => {
                  // Java: this.mMatchedBullet = String.valueOf('●');
                  self.mMatchedBullet = Some(String::from("\u{25cf}"));
                  // Java: break;
               }
               // Java: case 110:
               // Java: case 167:
               // Java: case 9679:
               110 | 167 | 9679 => {
                  // Java: this.mMatchedBullet = String.valueOf('■');
                  self.mMatchedBullet = Some(String::from("\u{25a0}"));
                  // Java: break;
               }
               // Java: default:
               _ => {
                  // Java: matched = false;
                  matched = false;
               }
            }
         }
         // Java: else if (fontName.equals("Wingdings 2"))
         else if fontName == "Wingdings 2"
         {
            // Java: switch (key)
            match key
            {
               // Java: case 61591:
               61591 => {
                  // Java: this.mMatchedFontName = "Wingdings";
                  self.mMatchedFontName = Some(String::from("Wingdings"));
                  // Java: this.mMatchedBullet = String.valueOf('\uf06c');
                  self.mMatchedBullet = Some(String::from("\u{f06c}"));
                  // Java: break;
               }
               // Java: case 61663:
               61663 => {
                  // Java: this.mMatchedBullet = String.valueOf('*');
                  self.mMatchedBullet = Some(String::from("*"));
                  // Java: break;
               }
               // Java: default:
               _ => {
                  // Java: matched = false;
                  matched = false;
               }
            }
         }
         // Java: else
         else
         {
            // Java: matched = false;
            matched = false;
         }
         // Java: return matched;
         matched
      }
    }

}
