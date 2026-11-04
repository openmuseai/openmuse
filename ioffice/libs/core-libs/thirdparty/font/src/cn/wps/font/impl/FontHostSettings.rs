use std::sync::OnceLock;

// Java field: SchemaItem[] FONT_MAPPING_SCHEME_JP
static FONT_MAPPING_SCHEME_JP: OnceLock<Vec<SchemaItem>> = OnceLock::new();
// Java field: SchemaItem[] FONT_MAPPING_SCHEME_JINGHUA
static FONT_MAPPING_SCHEME_JINGHUA: OnceLock<Vec<SchemaItem>> = OnceLock::new();
// Java field: String[] FONT_FILES_JP
static FONT_FILES_JP: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] FONT_FILES_SYMBOLS
static FONT_FILES_SYMBOLS: OnceLock<Vec<String>> = OnceLock::new();
// Java field: String[] FONT_FILES_JINGHUA
static FONT_FILES_JINGHUA: OnceLock<Vec<String>> = OnceLock::new();

pub fn get_FONT_MAPPING_SCHEME_JP() -> &'static Vec<SchemaItem> {
    FONT_MAPPING_SCHEME_JP.get_or_init(|| vec![
        SchemaItem::new("ＭＳ Ｐゴシック".to_string(), "IPAPGothic".to_string(), true),
    ])
}

pub fn get_FONT_MAPPING_SCHEME_JINGHUA() -> &'static Vec<SchemaItem> {
    FONT_MAPPING_SCHEME_JINGHUA.get_or_init(|| vec![
        SchemaItem::new("仿宋".to_string(), "方正仿宋_GB18030".to_string(), true),
        SchemaItem::new("仿宋_GB2312".to_string(), "仿宋_GB2312".to_string(), true),
        SchemaItem::new("微软雅黑".to_string(), "方正兰亭黑_GB18030".to_string(), true),
        SchemaItem::new("宋体".to_string(), "方正小标宋_GBK".to_string(), true),
        SchemaItem::new("楷体".to_string(), "方正楷体".to_string(), true),
        SchemaItem::new("楷体_GB2312".to_string(), "方正楷体".to_string(), true),
        SchemaItem::new("隶书".to_string(), "方正隶书_GBK".to_string(), true),
        SchemaItem::new("黑体".to_string(), "方正黑体".to_string(), true),
        SchemaItem::new("华文中宋".to_string(), "方正小标宋_GBK".to_string(), false),
        SchemaItem::new("华文行楷".to_string(), "方正行楷_GBK".to_string(), false),
    ])
}

pub fn get_FONT_FILES_JP() -> &'static Vec<String> {
    FONT_FILES_JP.get_or_init(|| vec![
        "ipaexg.ttf".to_string(), "ipaexm.ttf".to_string(), "ipag.ttf".to_string(),
        "ipagp.ttf".to_string(), "ipam.ttf".to_string(), "ipamp.ttf".to_string(),
    ])
}

pub fn get_FONT_FILES_SYMBOLS() -> &'static Vec<String> {
    FONT_FILES_SYMBOLS.get_or_init(|| vec![
        "mtextra.ttf".to_string(), "symbol.ttf".to_string(), "wingding.ttf".to_string(),
    ])
}

pub fn get_FONT_FILES_JINGHUA() -> &'static Vec<String> {
    FONT_FILES_JINGHUA.get_or_init(|| vec![
        "FZFS_GB18030.TTF".to_string(), "FZHT_GB18030.TTF".to_string(),
        "FZKT_GB18030.TTF".to_string(), "FZLS_GBK.TTF".to_string(),
        "FZLTH_GB18030.TTF".to_string(), "FZSS_GB18030_DZ5.TTF".to_string(),
        "FZSY_GB18030_DZ5.TTF".to_string(), "FZXBS_GBK.TTF".to_string(),
        "FZXH_GB18030.TTF".to_string(), "FZXK_GBK.TTF".to_string(),
    ])
}

// Java: public class FontHostSettings
pub struct FontHostSettings {
}

// Java: public class SchemaItem
pub struct SchemaItem {
    // Java field: String displayFontName
    pub(crate) displayFontName: String,
    // Java field: String actualFontName
    pub(crate) actualFontName: String,
    // Java field: boolean isSelectableInFontList
    pub(crate) isSelectableInFontList: bool,
}

// Java: public class FontHostSettings
impl FontHostSettings {
    // Java: FontHostSettings()
    pub fn new() -> Self {
        // Java: super();
        FontHostSettings {}
    }
}

// Java: public class SchemaItem
impl SchemaItem {
    // Java: SchemaItem(String displayName, String actualName, boolean isSelectable)
    pub fn new(displayName: String, actualName: String, isSelectable: bool) -> Self {
        // Java: super();
        // Java: this.displayFontName = displayName;
        // Java: this.actualFontName = actualName;
        // Java: this.isSelectableInFontList = isSelectable;
        SchemaItem {
            displayFontName: displayName,
            actualFontName: actualName,
            isSelectableInFontList: isSelectable,
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int prime = 31;
        let prime: i32 = 31;
        // Java: int result = 1;
        let mut result: i32 = 1;
        // Java: result = 31 * result + (this.actualFontName == null ? 0 : this.actualFontName.hashCode());
        result = 31 * result + Self::str_hash(&self.actualFontName);
        // Java: result = 31 * result + (this.displayFontName == null ? 0 : this.displayFontName.hashCode());
        result = 31 * result + Self::str_hash(&self.displayFontName);
        // Java: result = 31 * result + (this.isSelectableInFontList ? 1231 : 1237);
        result = 31 * result + if self.isSelectableInFontList { 1231 } else { 1237 };
        // Java: return result;
        result
    }

    fn str_hash(s: &str) -> i32 {
        let mut h: i32 = 0;
        for ch in s.chars() {
            h = h.wrapping_mul(31).wrapping_add(ch as i32);
        }
        h
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, obj: &SchemaItem) -> bool {
        // Java: if (this == obj)
        // Java: return true;

        // Java: else if (obj == null)
        // Java: return false;

        // Java: else if (this.getClass() != obj.getClass())
        // Java: return false;

        // Java: else
        // Java: SchemaItem other = (SchemaItem)obj;
        let other = obj;
        // Java: if (this.actualFontName == null)
        // Java: if (other.actualFontName != null)
        // Java: return false;
        // Java: else if (!this.actualFontName.equals(other.actualFontName))
        // Java: return false;
        if self.actualFontName != other.actualFontName {
            return false;
        }

        // Java: if (this.displayFontName == null)
        // Java: if (other.displayFontName != null)
        // Java: return false;
        // Java: else if (!this.displayFontName.equals(other.displayFontName))
        // Java: return false;
        if self.displayFontName != other.displayFontName {
            return false;
        }

        // Java: return this.isSelectableInFontList == other.isSelectableInFontList;
        self.isSelectableInFontList == other.isSelectableInFontList
    }
}
