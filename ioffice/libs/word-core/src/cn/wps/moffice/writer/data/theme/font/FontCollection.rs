// Java: cn.wps.moffice.writer.data.theme.font.FontCollection
//
// ★M-STAGE1 P2-8 —— 全仓原缺失，由 Java 原文（51 行）新建。

// Java: import cn.wps.moffice.writer.data.value.FontInfo
use crate::cn::wps::moffice::writer::data::value::FontInfo::FontInfo;
// Java: import java.util.ArrayList
use crate::cn::wps::moffice::writer::data::theme::font::SupplementalFont::SupplementalFont;

// java public class FontCollection
#[derive(Default, Clone)]
pub struct FontCollection {
    // java private FontInfo mLatinFont;
    mLatinFont: Option<FontInfo>,
    // java private FontInfo mEastAsian;
    mEastAsian: Option<FontInfo>,
    // java private FontInfo mComplexScript;
    mComplexScript: Option<FontInfo>,
    // java private ArrayList<SupplementalFont> mSupplementalFonts;
    mSupplementalFonts: Option<Vec<SupplementalFont>>,
}

// java public class FontCollection
impl FontCollection {
    // rust: Java 无显式构造器（走默认构造）⇒ `new()` 为 Rust 侧便利方法，
    //   不对应 Java 成员，故不写 `// java` 方法注释。
    pub fn new() -> Self {
        Self::default()
    }

    // java public FontInfo getLatinFont()
    pub fn getLatinFont(&self) -> Option<FontInfo> {
        // java return this.mLatinFont;
        return self.mLatinFont.clone();
    }

    // java public void setLatinFont(FontInfo latinFont)
    pub fn setLatinFont(&mut self, latinFont: Option<FontInfo>) {
        // java this.mLatinFont = latinFont;
        self.mLatinFont = latinFont;
    }

    // java public FontInfo getEastAsian()
    pub fn getEastAsian(&self) -> Option<FontInfo> {
        // java return this.mEastAsian;
        return self.mEastAsian.clone();
    }

    // java public void setEastAsian(FontInfo eastAsian)
    pub fn setEastAsian(&mut self, eastAsian: Option<FontInfo>) {
        // java this.mEastAsian = eastAsian;
        self.mEastAsian = eastAsian;
    }

    // java public FontInfo getComplexScript()
    pub fn getComplexScript(&self) -> Option<FontInfo> {
        // java return this.mComplexScript;
        return self.mComplexScript.clone();
    }

    // java public void setComplexScript(FontInfo complexScript)
    pub fn setComplexScript(&mut self, complexScript: Option<FontInfo>) {
        // java this.mComplexScript = complexScript;
        self.mComplexScript = complexScript;
    }

    // java public ArrayList<SupplementalFont> getSupplementalFont()
    pub fn getSupplementalFont(&self) -> Option<Vec<SupplementalFont>> {
        // java return this.mSupplementalFonts;
        return self.mSupplementalFonts.clone();
    }

    // java public void setSupplementalFont(ArrayList<SupplementalFont> supplementalFont)
    pub fn setSupplementalFont(&mut self, supplementalFont: Option<Vec<SupplementalFont>>) {
        // java this.mSupplementalFonts = supplementalFont;
        self.mSupplementalFonts = supplementalFont;
    }

    // java public void addFont(SupplementalFont font)
    pub fn addFont(&mut self, font: SupplementalFont) {
        // java if (null == this.mSupplementalFonts)
        if self.mSupplementalFonts.is_none() {
            // java this.mSupplementalFonts = new ArrayList();
            self.mSupplementalFonts = Some(Vec::new());
        }
        // java this.mSupplementalFonts.add(font);
        self.mSupplementalFonts.as_mut().unwrap().push(font);
    }
}
