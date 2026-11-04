// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.settings.NoLineBreaksImporter
//
// ★M-STAGE1 P2 —— 新建类型。全仓此前缺失（0 hit），是 `SettingsImporter`
//   依赖闭包中的一环（`SettingsImporter.java:40 private NoLineBreaksImporter mNoLineBreaksImporter;`）。
//   按 §24.4 判据：闭包内类型须逐个做实后才能登记依赖者。
//
// Java 原文 81 行；继承 `ImporterBase`（`final class ... extends ImporterBase`）。
// 依赖闭包已全部就位：`ConvertHelper` / `Xml07Context` / `Xml03Context` /
// `ImporterBase` / `MutablePropertySet` —— 均已登记且零 `todo!()`。

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

// java final class NoLineBreaksImporter extends ImporterBase
#[derive(Default)]
pub struct NoLineBreaksImporter {
    // java private String mAfterVal = null;
    mAfterVal: Option<String>,
    // java private String mAfterLang = null;
    mAfterLang: Option<String>,
    // java private String mBeforeVal = null;
    mBeforeVal: Option<String>,
    // java private String mBeforeLang = null;
    mBeforeLang: Option<String>,
    // rust: Java `extends ImporterBase` ⇒ 组合基类（与其它 importer 一致的做法）
    base: ImporterBase,
}

impl NoLineBreaksImporter {
    // java private static final String LANG = "lang";
    const LANG: &'static str = "lang";

    // java NoLineBreaksImporter(int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn NoLineBreaksImporter(
        mode: i32,
        xml07Context: Option<Xml07Context>,
        xml03Context: Option<Xml03Context>,
    ) -> Self {
        // java super(mode, xml07Context, xml03Context);
        NoLineBreaksImporter {
            mAfterVal: None,
            mAfterLang: None,
            mBeforeVal: None,
            mBeforeLang: None,
            base: ImporterBase::ImporterBase(mode, xml07Context, xml03Context),
        }
    }

    // java void setNoLineBreaksAfter(Attributes attributes)
    pub fn setNoLineBreaksAfter(&mut self, attributes: &dyn Attributes) {
        // java IOAssert.assertNotNull("attributes should not be null", attributes);
        // rust: `&dyn Attributes` 天然非空 ⇒ 由类型系统保证。
        // java this.mAfterVal = ConvertHelper.getString(attributes);
        self.mAfterVal = ConvertHelper::getString(attributes);
        // java this.mAfterLang = ConvertHelper.getString(attributes, "lang");
        self.mAfterLang = ConvertHelper::getString__String(attributes, Self::LANG);
    }

    // java void setNoLineBreaksBefore(Attributes attributes)
    pub fn setNoLineBreaksBefore(&mut self, attributes: &dyn Attributes) {
        // java IOAssert.assertNotNull("attributes should not be null", attributes);
        // rust: `&dyn Attributes` 天然非空 ⇒ 由类型系统保证。
        // java this.mBeforeVal = ConvertHelper.getString(attributes);
        self.mBeforeVal = ConvertHelper::getString(attributes);
        // java this.mBeforeLang = ConvertHelper.getString(attributes, "lang");
        self.mBeforeLang = ConvertHelper::getString__String(attributes, Self::LANG);
    }

    // java void correctNoLineBreaks(MutablePropertySet dst)
    pub fn correctNoLineBreaks(&mut self, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // rust: `&mut` 天然非空 ⇒ 由类型系统保证。
        // java Integer lang = null;
        let mut lang: Option<i32> = None;
        // java if (null != this.mBeforeLang && this.mBeforeLang.equals(this.mAfterLang))
        if self.mBeforeLang.is_some() && self.mBeforeLang == self.mAfterLang {
            // java lang = this.convertLanguage(this.mBeforeLang);
            lang = Some(self.convertLanguage(self.mBeforeLang.clone().unwrap()));
        }

        // java Boolean isStrict = (Boolean)dst.getValue(468);
        // rust: 真实 `PropertyValue = Arc<dyn Object>` ⇒ 按仓内既有模式 downcast
        let isStrict: Option<bool> = dst.getValue(468).and_then(|v| v.as_ref().as_any().downcast_ref::<bool>().copied());
        // java int dstVal = 0;
        let mut dstVal: i32 = 0;
        // java if (null != isStrict && isStrict)
        if isStrict == Some(true) {
            // java dstVal = 1;
            dstVal = 1;
        } else {
            // java if (null != this.mBeforeVal && null != lang)
            if self.mBeforeVal.is_some() && lang.is_some() {
                // java dstVal = 2;
                dstVal = 2;
                // java dst.setString(378, this.mBeforeVal);
                dst.setString(378, self.mBeforeVal.clone().unwrap());
                // java dst.setInt(381, lang);
                dst.setInt(381, lang.unwrap());
                // java dst.setInt(379, lang);
                dst.setInt(379, lang.unwrap());
            }

            // java if (null != this.mAfterVal && null != lang)
            if self.mAfterVal.is_some() && lang.is_some() {
                // java dstVal = 2;
                dstVal = 2;
                // java dst.setString(380, this.mAfterVal);
                dst.setString(380, self.mAfterVal.clone().unwrap());
                // java dst.setInt(381, lang);
                dst.setInt(381, lang.unwrap());
                // java dst.setInt(379, lang);
                dst.setInt(379, lang.unwrap());
            }
        }

        // java dst.setInt(384, dstVal);
        dst.setInt(384, dstVal);
    }

    // java private int convertLanguage(String lang)
    fn convertLanguage(&self, lang: String) -> i32 {
        // java IOAssert.assertNotNull("lang should not be null", lang);
        // rust: 参数为 `String` 按值传入 ⇒ 非空由类型系统保证。
        // java int dstVal = 0;
        let mut dstVal: i32 = 0;
        // java if ("zh-CN".equals(lang))
        if "zh-CN" == lang {
            // java dstVal = 2;
            dstVal = 2;
        // java } else if ("zh-TW".equals(lang))
        } else if "zh-TW" == lang {
            // java dstVal = 4;
            dstVal = 4;
        // java } else if ("ja-JP".equals(lang))
        } else if "ja-JP" == lang {
            // java dstVal = 1;
            dstVal = 1;
        // java } else if ("ko-KR".equals(lang))
        } else if "ko-KR" == lang {
            // java dstVal = 3;
            dstVal = 3;
        } else {
            // java IOAssert.fail("It should not reach here.");
            IOAssert::fail_msg("It should not reach here.");
        }

        // java return dstVal;
        return dstVal;
    }
}
