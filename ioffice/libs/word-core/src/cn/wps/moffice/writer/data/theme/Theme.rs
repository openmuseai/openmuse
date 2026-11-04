// Java: cn.wps.moffice.writer.data.theme.Theme
//
// ★M-STAGE1 P2-9 —— 本文件此前**未登记**（`lib.rs` 的 `data::theme` 只登记了
//   `font::SupplementalFont`）⇒ 其类型错误**长期不可见**。
//   登记后暴露 16 个错误：字段是 `Option<T>`，而访问器按**值语义**写
//   （`self.mThemePath = path;` / `return self.mThemePath;` 等）。
//   本轮按 Java 原文（82 行，全部字段可空）统一为 `Option` 语义。

// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
use io_wps_io::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
// Java: import cn.wps.moffice.writer.data.theme.color.ThemeColorScheme
use crate::cn::wps::moffice::writer::data::theme::color::ThemeColorScheme::ThemeColorScheme;
// Java: import cn.wps.moffice.writer.data.theme.font.ThemeFontScheme
use crate::cn::wps::moffice::writer::data::theme::font::ThemeFontScheme::ThemeFontScheme;
// Java: import cn.wps.moffice.writer.data.theme.format.ThemeFormatScheme
use crate::cn::wps::moffice::writer::data::theme::format::ThemeFormatScheme::ThemeFormatScheme;
// Java: import java.util.Map
use std::collections::HashMap;

// java public class Theme
#[derive(Default)]
pub struct Theme {
    // java private ThemeFontScheme mThemeFontScheme;
    mThemeFontScheme: Option<ThemeFontScheme>,
    // java private POIXMLDocumentPart mThemePackagePart;
    // Rust Rc preserves the Java object identity past XWPFDocument::parse.
    mThemePackagePart: Option<std::rc::Rc<POIXMLDocumentPart>>,
    // java private String mThemePath;
    mThemePath: Option<String>,
    // java private String mUofThemePath;
    mUofThemePath: Option<String>,
    // java private Map<String, Integer> mRid2MediaId;
    mRid2MediaId: Option<HashMap<String, i32>>,
    // java private String mThemeName;
    mThemeName: Option<String>,
    // java private ThemeColorScheme mThemeColorScheme;
    mThemeColorScheme: Option<ThemeColorScheme>,
    // java private ThemeFormatScheme mThemeFormatScheme;
    mThemeFormatScheme: Option<ThemeFormatScheme>,
}

// java public class Theme
impl Theme {
    // rust: Java 无显式构造器（走默认构造）⇒ `new()` 为 Rust 侧便利方法。
    pub fn new() -> Self {
        Self::default()
    }

    // java public ThemeFontScheme getThemeFontScheme()
    pub fn getThemeFontScheme(&mut self) -> ThemeFontScheme {
      // java if (null == this.mThemeFontScheme)
      if self.mThemeFontScheme.is_none() {
        // java this.mThemeFontScheme = new ThemeFontScheme();
        self.mThemeFontScheme = Some(ThemeFontScheme::new());
      }
      // java return this.mThemeFontScheme;
      // rust: ★Java 返回**引用** ⇒ Rust 侧此处返回克隆。
      //   需要**共享就地修改**的调用方请用 `getThemeFontScheme_mut()`。
      return self.mThemeFontScheme.clone().expect("mThemeFontScheme");
    }

    /// rust: ★Java 引用语义 —— `ThemeImporter` 构造器
    ///   `new FontSchemeImporter(theme.getThemeFontScheme())` 共享同一 scheme 对象。
    ///   （Rust 侧补充访问器，对应 Java 同一个 `getThemeFontScheme()` 返回引用。）
    pub fn getThemeFontScheme_mut(&mut self) -> *mut ThemeFontScheme {
      // java if (null == this.mThemeFontScheme)
      if self.mThemeFontScheme.is_none() {
        // java this.mThemeFontScheme = new ThemeFontScheme();
        self.mThemeFontScheme = Some(ThemeFontScheme::new());
      }
      self.mThemeFontScheme.as_mut().unwrap() as *mut ThemeFontScheme
    }

    // java public void setThemeFontScheme(ThemeFontScheme themeFontScheme)
    pub fn setThemeFontScheme(&mut self, themeFontScheme: Option<ThemeFontScheme>) {
      // java this.mThemeFontScheme = themeFontScheme;
      self.mThemeFontScheme = themeFontScheme;
    }

    // java public void setPackagePart(POIXMLDocumentPart packagePart)
    pub fn setPackagePart(&mut self, packagePart: Option<std::rc::Rc<POIXMLDocumentPart>>) {
      // java this.mThemePackagePart = packagePart;
      self.mThemePackagePart = packagePart;
    }

    // java public POIXMLDocumentPart getPackagePart()
    // rust: ★Java 返回**引用** ⇒ Rust 返回借用（`POIXMLDocumentPart` 非 `Clone`）。
    pub fn getPackagePart(&self) -> Option<&POIXMLDocumentPart> {
      // java return this.mThemePackagePart;
      return self.mThemePackagePart.as_deref();
    }

    // java public void setThemePath(String path)
    pub fn setThemePath(&mut self, path: Option<String>) {
      // java this.mThemePath = path;
      self.mThemePath = path;
    }

    // java public String getThemePath()
    pub fn getThemePath(&self) -> Option<String> {
      // java return this.mThemePath;
      return self.mThemePath.clone();
    }

    // java public void setUofThemePath(String path)
    pub fn setUofThemePath(&mut self, path: Option<String>) {
      // java this.mUofThemePath = path;
      self.mUofThemePath = path;
    }

    // java public String getUofThemePath()
    pub fn getUofThemePath(&self) -> Option<String> {
      // java return this.mUofThemePath;
      return self.mUofThemePath.clone();
    }

    // java public Map<String, Integer> getRid2MediaId()
    pub fn getRid2MediaId(&self) -> Option<HashMap<String, i32>> {
      // java return this.mRid2MediaId;
      return self.mRid2MediaId.clone();
    }

    // java public void setRid2MediaId(Map<String, Integer> mRid2MediaId)
    pub fn setRid2MediaId(&mut self, mRid2MediaId: Option<HashMap<String, i32>>) {
      // java this.mRid2MediaId = mRid2MediaId;
      self.mRid2MediaId = mRid2MediaId;
    }

    // java public void setThemeColorScheme(ThemeColorScheme colorScheme)
    pub fn setThemeColorScheme(&mut self, colorScheme: Option<ThemeColorScheme>) {
      // java this.mThemeColorScheme = colorScheme;
      self.mThemeColorScheme = colorScheme;
    }

    // java public Integer getSchemeColor(String name)
    pub fn getSchemeColor__string(&self, name: String) -> Option<i32> {
      // java return null != this.mThemeColorScheme ? this.mThemeColorScheme.getSchemeColor(name) : null;
      return match self.mThemeColorScheme.as_ref() {
        Some(scheme) => scheme.getSchemeColor(name),
        None => None,
      };
    }

    // java public Integer getSchemeColor(int colorSchemeIndex)
    pub fn getSchemeColor__int(&self, colorSchemeIndex: i32) -> Option<i32> {
      // java return null != this.mThemeColorScheme ? this.mThemeColorScheme.getSchemeColor(colorSchemeIndex) : null;
      return match self.mThemeColorScheme.as_ref() {
        Some(scheme) => scheme.getSchemeColor_1(colorSchemeIndex),
        None => None,
      };
    }

    // java public ThemeColorScheme getThemeColorScheme()
    pub fn getThemeColorScheme(&self) -> Option<ThemeColorScheme> {
      // java return this.mThemeColorScheme;
      return self.mThemeColorScheme.clone();
    }

    // java public ThemeFormatScheme getThemeFormatScheme()
    // rust: ★Java 返回**引用** ⇒ Rust 返回借用（`ThemeFormatScheme` 非 `Clone`）。
    pub fn getThemeFormatScheme(&self) -> Option<&ThemeFormatScheme> {
      // java return this.mThemeFormatScheme;
      return self.mThemeFormatScheme.as_ref();
    }

    // java public void setThemeFormatScheme(ThemeFormatScheme themeFormatScheme)
    pub fn setThemeFormatScheme(&mut self, themeFormatScheme: Option<ThemeFormatScheme>) {
      // java this.mThemeFormatScheme = themeFormatScheme;
      self.mThemeFormatScheme = themeFormatScheme;
    }

    // java public void setName(String name)
    pub fn setName(&mut self, name: Option<String>) {
      // java this.mThemeName = name;
      self.mThemeName = name;
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
      // java return this.mThemeName;
      return self.mThemeName.clone();
    }
}
