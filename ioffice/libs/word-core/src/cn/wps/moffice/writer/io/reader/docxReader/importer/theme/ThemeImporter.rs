// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.theme.ThemeImporter
//
// ★M-STAGE1 P2-8 —— 由 auto-generated 桩（19 行 / 9 处 `todo!()`）**做实**。
//   Java 原文 58 行：8 个方法，全部是「断言非空 + 转发给 mFontSchemeImporter」。
//
// ★Java 引用语义：`this.mFontSchemeImporter = new FontSchemeImporter(themeFontScheme)`
//   其中 `themeFontScheme` 来自 `theme.getThemeFontScheme()`（Java 返回**引用**）。
//   ⇒ `mFontSchemeImporter` 直接持有 `FontSchemeImporter`（它内部已用裸指针指向 scheme），
//      故此处无需再裸指针化。

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
// Java: import cn.wps.moffice.writer.data.theme.Theme
use crate::cn::wps::moffice::writer::data::theme::Theme::Theme;
// Java: import cn.wps.moffice.writer.data.theme.font.ThemeFontScheme
use crate::cn::wps::moffice::writer::data::theme::font::ThemeFontScheme::ThemeFontScheme;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.theme.FontSchemeImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::theme::FontSchemeImporter::FontSchemeImporter;
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

// java public class ThemeImporter
#[derive(Default)]
pub struct ThemeImporter {
    // java FontSchemeImporter mFontSchemeImporter;
    mFontSchemeImporter: Option<FontSchemeImporter>,
    // java Theme mTheme;
    // rust: ★Java 引用语义 —— `this.mTheme.setPackagePart(...)` 需**就地修改** theme
    mTheme: Option<*mut Theme>,
}

// java public class ThemeImporter
impl ThemeImporter {
    // java public ThemeImporter(Theme theme)
    pub fn ThemeImporter(theme: *mut Theme) -> Self {
      // java IOAssert.assertNotNull("theme should not be null", theme);
      let theme_opt = Some(theme);
      IOAssert::assertNotNull_msg("theme should not be null", &theme_opt);
      // java ThemeFontScheme themeFontScheme = theme.getThemeFontScheme();
      // rust: Java 返回引用；Rust 侧 `Theme::getThemeFontScheme()` 返回 `ThemeFontScheme`
      //   ⇒ 取 scheme 字段的可变裸指针，保持 Java 的共享语义。
      let scheme: *mut ThemeFontScheme = unsafe {
        (*theme).getThemeFontScheme_mut()
      };
      Self {
        // java this.mFontSchemeImporter = new FontSchemeImporter(themeFontScheme);
        mFontSchemeImporter: Some(FontSchemeImporter::FontSchemeImporter(scheme)),
        // java this.mTheme = theme;
        mTheme: Some(theme),
      }
    }

    // java public void importFontSchemeStart(Attributes attributes)
    pub fn importFontSchemeStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mFontSchemeImporter should not be null!", this.mFontSchemeImporter);
      IOAssert::assertNotNull_msg("mFontSchemeImporter should not be null!", &self.mFontSchemeImporter);
      // java this.mFontSchemeImporter.importFontSchemeStart(attributes);
      self.mFontSchemeImporter.as_mut().unwrap().importFontSchemeStart(attributes);
    }

    // java public void importMajorFontStart()
    pub fn importMajorFontStart(&mut self) {
      // java IOAssert.assertNotNull("mFontSchemeImporter should not be null!", this.mFontSchemeImporter);
      IOAssert::assertNotNull_msg("mFontSchemeImporter should not be null!", &self.mFontSchemeImporter);
      // java this.mFontSchemeImporter.importMajorFontStart();
      self.mFontSchemeImporter.as_mut().unwrap().importMajorFontStart();
    }

    // java public void importMinorFontStart()
    pub fn importMinorFontStart(&mut self) {
      // java IOAssert.assertNotNull("mFontSchemeImporter should not be null!", this.mFontSchemeImporter);
      IOAssert::assertNotNull_msg("mFontSchemeImporter should not be null!", &self.mFontSchemeImporter);
      // java this.mFontSchemeImporter.importMinorFontStart();
      self.mFontSchemeImporter.as_mut().unwrap().importMinorFontStart();
    }

    // java public void importCSFontStart(Attributes attributes)
    pub fn importCSFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mFontSchemeImporter should not be null!", this.mFontSchemeImporter);
      IOAssert::assertNotNull_msg("mFontSchemeImporter should not be null!", &self.mFontSchemeImporter);
      // java this.mFontSchemeImporter.importCSFontStart(attributes);
      self.mFontSchemeImporter.as_mut().unwrap().importCSFontStart(attributes);
    }

    // java public void importEAFontStart(Attributes attributes)
    pub fn importEAFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mFontSchemeImporter should not be null!", this.mFontSchemeImporter);
      IOAssert::assertNotNull_msg("mFontSchemeImporter should not be null!", &self.mFontSchemeImporter);
      // java this.mFontSchemeImporter.importEAFontStart(attributes);
      self.mFontSchemeImporter.as_mut().unwrap().importEAFontStart(attributes);
    }

    // java public void importLatinFontStart(Attributes attributes)
    pub fn importLatinFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mFontSchemeImporter should not be null!", this.mFontSchemeImporter);
      IOAssert::assertNotNull_msg("mFontSchemeImporter should not be null!", &self.mFontSchemeImporter);
      // java this.mFontSchemeImporter.importLatinFontStart(attributes);
      self.mFontSchemeImporter.as_mut().unwrap().importLatinFontStart(attributes);
    }

    // java public void importFontStart(Attributes attributes)
    pub fn importFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mFontSchemeImporter should not be null!", this.mFontSchemeImporter);
      IOAssert::assertNotNull_msg("mFontSchemeImporter should not be null!", &self.mFontSchemeImporter);
      // java this.mFontSchemeImporter.importFontStart(attributes);
      self.mFontSchemeImporter.as_mut().unwrap().importFontStart(attributes);
    }

    // java public void setThemePackagePart(POIXMLDocumentPart packagePart)
    pub fn setThemePackagePart(&mut self, packagePart: std::rc::Rc<POIXMLDocumentPart>) {
      // java this.mTheme.setPackagePart(packagePart);
      unsafe { (*self.mTheme.expect("mTheme")).setPackagePart(Some(packagePart)); }
    }
}
