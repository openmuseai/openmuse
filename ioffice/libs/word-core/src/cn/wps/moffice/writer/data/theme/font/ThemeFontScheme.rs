// Java: cn.wps.moffice.writer.data.theme.font.ThemeFontScheme
//
// ★M-STAGE1 P2-8 —— 由 auto-generated 桩（17 行 / 7 处 `todo!()`）**做实**。
//   Java 原文 31 行：3 个字段 + 3 对访问器。

// Java: import cn.wps.moffice.writer.data.theme.font.FontCollection
use crate::cn::wps::moffice::writer::data::theme::font::FontCollection::FontCollection;

// java public class ThemeFontScheme
#[derive(Default, Clone)]
pub struct ThemeFontScheme {
    // java private String mName;
    mName: Option<String>,
    // java private FontCollection mMajorFont;
    mMajorFont: Option<FontCollection>,
    // java private FontCollection mMinorFont;
    mMinorFont: Option<FontCollection>,
}

// java public class ThemeFontScheme
impl ThemeFontScheme {
    // rust: Java 无显式构造器（走默认构造）⇒ `new()` 为 Rust 侧便利方法，
    //   不对应 Java 成员，故不写 `// java` 方法注释。
    pub fn new() -> Self {
        Self::default()
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
        // java return this.mName;
        return self.mName.clone();
    }

    // java public void setName(String name)
    pub fn setName(&mut self, name: Option<String>) {
        // java this.mName = name;
        self.mName = name;
    }

    // java public FontCollection getMajorFont()
    pub fn getMajorFont(&self) -> Option<FontCollection> {
        // java return this.mMajorFont;
        return self.mMajorFont.clone();
    }

    // java public void setMajorFont(FontCollection majorFont)
    pub fn setMajorFont(&mut self, majorFont: Option<FontCollection>) {
        // java this.mMajorFont = majorFont;
        self.mMajorFont = majorFont;
    }

    // java public FontCollection getMinorFont()
    pub fn getMinorFont(&self) -> Option<FontCollection> {
        // java return this.mMinorFont;
        return self.mMinorFont.clone();
    }

    // java public void setMinorFont(FontCollection minorFont)
    pub fn setMinorFont(&mut self, minorFont: Option<FontCollection>) {
        // java this.mMinorFont = minorFont;
        self.mMinorFont = minorFont;
    }

    /// rust: ★Java 引用语义 —— `FontSchemeImporter.importMajorFontStart()` 里
    ///   `this.mFontCollection = new FontCollection(); this.mThemeFontScheme.setMajorFont(this.mFontCollection);`
    ///   之后在 **同一对象** 上 `setComplexScript` / `setEastAsian` / `setLatinFont` / `addFont`，
    ///   效果经 `themeFontScheme.getMajorFont()` 可见。Rust 需裸指针才能保持该共享语义。
    ///   （Rust 侧补充访问器，对应 Java 的同一个 `setMajorFont` + 引用保留。）
    pub fn setMajorFont_ref(&mut self, fc: FontCollection) -> *mut FontCollection {
        self.mMajorFont = Some(fc);
        self.mMajorFont.as_mut().unwrap() as *mut FontCollection
    }

    /// rust: ★Java 引用语义 —— 见 `setMajorFont_ref`（`importMinorFontStart` 同理）。
    pub fn setMinorFont_ref(&mut self, fc: FontCollection) -> *mut FontCollection {
        self.mMinorFont = Some(fc);
        self.mMinorFont.as_mut().unwrap() as *mut FontCollection
    }
}
