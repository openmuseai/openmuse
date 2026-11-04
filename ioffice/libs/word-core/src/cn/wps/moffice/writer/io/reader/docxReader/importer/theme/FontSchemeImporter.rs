// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.theme.FontSchemeImporter
//
// ★M-STAGE1 P2-8 —— 全仓原缺失，按 Java 原文（64 行）新建。
//
// ★Java 引用语义（第 8 次出现）：`importMajorFontStart()` / `importMinorFontStart()` 里
//   `this.mFontCollection = new FontCollection(); this.mThemeFontScheme.setMajorFont(this.mFontCollection);`
//   —— Java 的 `setMajorFont` 存**引用**，随后 `importCSFontStart` 等在同一对象上
//   `setComplexScript`，效果经 `themeFontScheme.getMajorFont()` 可见。
//   ⇒ `mFontCollection` 用裸指针，指向 scheme 内的那个 `FontCollection`。

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.writer.data.theme.font.FontCollection
use crate::cn::wps::moffice::writer::data::theme::font::FontCollection::FontCollection;
// Java: import cn.wps.moffice.writer.data.theme.font.SupplementalFont
use crate::cn::wps::moffice::writer::data::theme::font::SupplementalFont::SupplementalFont;
// Java: import cn.wps.moffice.writer.data.theme.font.ThemeFontScheme
use crate::cn::wps::moffice::writer::data::theme::font::ThemeFontScheme::ThemeFontScheme;
// Java: import cn.wps.moffice.writer.data.value.FontInfo
use crate::cn::wps::moffice::writer::data::value::FontInfo::FontInfo;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::ConvertHelper;
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

// java private static final String ATTR_NAME = "name";
pub const ATTR_NAME: &str = "name";

// java public class FontSchemeImporter
#[derive(Default)]
pub struct FontSchemeImporter {
    // java ThemeFontScheme mThemeFontScheme;
    // rust: ★Java 引用语义 —— 见文件头说明
    pub(crate) mThemeFontScheme: Option<*mut ThemeFontScheme>,
    // java FontCollection mFontCollection;
    // rust: ★Java 引用语义 —— 指向 scheme 内那个 FontCollection
    pub(crate) mFontCollection: Option<*mut FontCollection>,
}

// java public class FontSchemeImporter
impl FontSchemeImporter {
    // java public FontSchemeImporter(ThemeFontScheme themeFontScheme)
    pub fn FontSchemeImporter(themeFontScheme: *mut ThemeFontScheme) -> Self {
      // java this.mThemeFontScheme = themeFontScheme;
      Self {
        mThemeFontScheme: Some(themeFontScheme),
        mFontCollection: None,
      }
    }

    // java public void importFontSchemeStart(Attributes attributes)
    pub fn importFontSchemeStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mThemeFontScheme should not be null!", this.mThemeFontScheme);
      IOAssert::assertNotNull_msg("mThemeFontScheme should not be null!", &self.mThemeFontScheme);
      // java String name = ConvertHelper.getString(attributes, "name");
      let name: Option<String> = ConvertHelper::getString__String(attributes, ATTR_NAME);
      // java if (null != name)
      if let Some(name) = name {
        // java this.mThemeFontScheme.setName(name);
        unsafe { (*self.mThemeFontScheme.expect("mThemeFontScheme")).setName(Some(name)); }
      }
    }

    // java public void importMajorFontStart()
    pub fn importMajorFontStart(&mut self) {
      // java IOAssert.assertNotNull("mThemeFontScheme should not be null!", this.mThemeFontScheme);
      IOAssert::assertNotNull_msg("mThemeFontScheme should not be null!", &self.mThemeFontScheme);
      // java this.mFontCollection = new FontCollection();
      let fc: FontCollection = FontCollection::new();
      // java this.mThemeFontScheme.setMajorFont(this.mFontCollection);
      self.mFontCollection = Some(unsafe {
        (*self.mThemeFontScheme.expect("mThemeFontScheme")).setMajorFont_ref(fc)
      });
    }

    // java public void importMinorFontStart()
    pub fn importMinorFontStart(&mut self) {
      // java IOAssert.assertNotNull("mThemeFontScheme should not be null!", this.mThemeFontScheme);
      IOAssert::assertNotNull_msg("mThemeFontScheme should not be null!", &self.mThemeFontScheme);
      // java this.mFontCollection = new FontCollection();
      let fc: FontCollection = FontCollection::new();
      // java this.mThemeFontScheme.setMinorFont(this.mFontCollection);
      self.mFontCollection = Some(unsafe {
        (*self.mThemeFontScheme.expect("mThemeFontScheme")).setMinorFont_ref(fc)
      });
    }

    // java public void importCSFontStart(Attributes attributes)
    pub fn importCSFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mFontCollection should not be null!", this.mFontCollection);
      IOAssert::assertNotNull_msg("mFontCollection should not be null!", &self.mFontCollection);
      // java FontInfo csFont = ConvertHelper.getTextFont(attributes);
      let csFont: FontInfo = ConvertHelper::getTextFont(attributes);
      // java this.mFontCollection.setComplexScript(csFont);
      unsafe { (*self.mFontCollection.expect("mFontCollection")).setComplexScript(Some(csFont)); }
    }

    // java public void importEAFontStart(Attributes attributes)
    pub fn importEAFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mFontCollection should not be null!", this.mFontCollection);
      IOAssert::assertNotNull_msg("mFontCollection should not be null!", &self.mFontCollection);
      // java FontInfo esFont = ConvertHelper.getTextFont(attributes);
      let esFont: FontInfo = ConvertHelper::getTextFont(attributes);
      // java this.mFontCollection.setEastAsian(esFont);
      unsafe { (*self.mFontCollection.expect("mFontCollection")).setEastAsian(Some(esFont)); }
    }

    // java public void importLatinFontStart(Attributes attributes)
    pub fn importLatinFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mFontCollection should not be null!", this.mFontCollection);
      IOAssert::assertNotNull_msg("mFontCollection should not be null!", &self.mFontCollection);
      // java FontInfo latinFont = ConvertHelper.getTextFont(attributes);
      let latinFont: FontInfo = ConvertHelper::getTextFont(attributes);
      // java this.mFontCollection.setLatinFont(latinFont);
      unsafe { (*self.mFontCollection.expect("mFontCollection")).setLatinFont(Some(latinFont)); }
    }

    // java public void importFontStart(Attributes attributes)
    pub fn importFontStart(&mut self, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("mFontCollection should not be null!", this.mFontCollection);
      IOAssert::assertNotNull_msg("mFontCollection should not be null!", &self.mFontCollection);
      // java SupplementalFont font = ConvertHelper.getSupplementalFont(attributes);
      let font: SupplementalFont = ConvertHelper::getSupplementalFont(attributes);
      // java this.mFontCollection.addFont(font);
      unsafe { (*self.mFontCollection.expect("mFontCollection")).addFont(font); }
    }
}
