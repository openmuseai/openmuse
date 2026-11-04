// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.settings.CapationsImporter
//
// ★M-STAGE1 P2 —— 新建类型。全仓此前缺失（0 hit），是 `SettingsImporter`
//   依赖闭包的**最后一个成员**（`SettingsImporter.java:43`）。
//
// Java 原文 166 行，继承 `ImporterBase`。
//
// ★依赖条件（按 §24.4 判据核验，本轮已全部就位）：
//   `ConvertHelper`（getString/getBoolean/xmlNfc2MsoNfc）、`ShareConvertHelper`
//   （getInteger4STDecimalNumber）、`ImporterBase`（含 `isXml03()`）、
//   `MutablePropertySet`（getValue/setValue）、
//   `CaptionType` / `AutoCaptionType`（★本轮新建，见 28.9 的下一步）
//
// ★注意 Java 原文有一处**疑似笔误**：`noLabel` 分支调用的是 `setChapNum(label)`
//   而非 `setNoLabel(label)`。按"移植不是改进 Java，是复现 Java"的既定纪律，
//   **原样保留**，并以用例锁定（见 70_function_capations_importer CP6）。

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.data.value.AutoCaptionType
use crate::cn::wps::moffice::writer::data::value::AutoCaptionType::AutoCaptionType;
// Java: import cn.wps.moffice.writer.data.value.CaptionType
use crate::cn::wps::moffice::writer::data::value::CaptionType::CaptionType;
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ShareConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
// Java: import org.xml.sax.Attributes
// rust: 本仓 SAX `Attributes` 的权威 trait 定义在 `ConvertHelper` 内（同 `NoLineBreaksImporter` 处理）。
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

// java final class CapationsImporter extends ImporterBase
#[derive(Default)]
pub struct CapationsImporter {
    // rust: Java `extends ImporterBase` ⇒ 组合基类（与 `NoLineBreaksImporter` 一致）
    base: ImporterBase,
}

impl CapationsImporter {
    // java CapationsImporter(int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn CapationsImporter(
        mode: i32,
        xml07Context: Option<Xml07Context>,
        xml03Context: Option<Xml03Context>,
    ) -> Self {
        // java super(mode, xml07Context, xml03Context);
        CapationsImporter {
            base: ImporterBase::ImporterBase(mode, xml07Context, xml03Context),
        }
    }

    // java void setCaption(Attributes attributes, MutablePropertySet dst)
    pub fn setCaption(&mut self, attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("attributes should not be null", attributes);
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // rust: `&dyn`/`&mut` 天然非空 ⇒ 由类型系统保证。
        // java Object captiosTypes = dst.getValue(561);
        let captiosTypes_present = dst.getValue(561).is_some();
        // java if (null == captiosTypes)
        if !captiosTypes_present {
            // java List<CaptionType> captiosTypeList = new ArrayList();
            let mut captiosTypeList: Vec<CaptionType> = Vec::new();
            // java dst.setValue(561, captiosTypeList);
            // java captiosTypeList.add(this.convertCaption(attributes));
            captiosTypeList.push(self.convertCaption(attributes));
            // rust: Java 先存空表再 add（引用语义，终点是"表含新元素"）；
            //   Rust 值语义无法先存后改 ⇒ 等价做法为**构建完成后一次写回**。
            dst.setValue(561, Some(std::sync::Arc::new(captiosTypeList)));
        // java } else if (captiosTypes instanceof List)
        } else {
            // java List<CaptionType> captiosTypeList = (List)captiosTypes;
            // rust: Java 是**引用语义** —— 取出的是同一个 List 对象，add 直接改到 `dst` 里。
            //   Rust 值语义必须先取回已有内容再追加，否则丢失历史项。
            let mut captiosTypeList: Vec<CaptionType> = dst
                .getValue(561)
                .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<CaptionType>>().cloned())
                .unwrap_or_default();
            // java captiosTypeList.add(this.convertCaption(attributes));
            captiosTypeList.push(self.convertCaption(attributes));
            // java (Java 原地修改同一 List；Rust 需显式回写)
            dst.setValue(561, Some(std::sync::Arc::new(captiosTypeList)));
        }
    }

    // java void setAutoCaption(Attributes attributes, MutablePropertySet dst)
    pub fn setAutoCaption(&mut self, attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("attributes should not be null", attributes);
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // rust: `&dyn`/`&mut` 天然非空 ⇒ 由类型系统保证。
        // java Object autoCaptiosTypes = dst.getValue(562);
        let autoCaptiosTypes_present = dst.getValue(562).is_some();
        // java if (null == autoCaptiosTypes)
        if !autoCaptiosTypes_present {
            // java List<AutoCaptionType> autoCaptiosTypesList = new ArrayList();
            let mut autoCaptiosTypesList: Vec<AutoCaptionType> = Vec::new();
            // java dst.setValue(562, autoCaptiosTypesList);
            // java autoCaptiosTypesList.add(convertAutoCaptions(attributes));
            autoCaptiosTypesList.push(Self::convertAutoCaptions(attributes));
            // rust: 同 `setCaption` —— 值语义下改为构建后一次写回。
            dst.setValue(562, Some(std::sync::Arc::new(autoCaptiosTypesList)));
        // java } else if (autoCaptiosTypes instanceof List)
        } else {
            // java List<AutoCaptionType> autoCaptiosTypesList = (List)autoCaptiosTypes;
            // rust: 同 `setCaption` —— 必须先取回已有列表再追加（引用语义 → 值语义）。
            let mut autoCaptiosTypesList: Vec<AutoCaptionType> = dst
                .getValue(562)
                .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<AutoCaptionType>>().cloned())
                .unwrap_or_default();
            // java autoCaptiosTypesList.add(convertAutoCaptions(attributes));
            autoCaptiosTypesList.push(Self::convertAutoCaptions(attributes));
            dst.setValue(562, Some(std::sync::Arc::new(autoCaptiosTypesList)));
        }
    }

    // java static void correctCaptions(MutablePropertySet dst)
    pub fn correctCaptions(dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // rust: `&mut` 天然非空 ⇒ 由类型系统保证。
        // java correctCaption(dst);
        Self::correctCaption(dst);
        // java correctAutoCaption(dst);
        Self::correctAutoCaption(dst);
    }

    // java private static void correctCaption(MutablePropertySet dst)
    fn correctCaption(dst: &mut MutablePropertySet) {
        // java Object captiosTypes = dst.getValue(561);
        let captiosTypes_present = dst.getValue(561).is_some();
        // java if (null != captiosTypes && captiosTypes instanceof List)
        if captiosTypes_present {
            // java List<CaptionType> captionTypeList = (List)captiosTypes;
            let captionTypeList: &str = "captiosTypes";
            let _ = captionTypeList;
            // java CaptionType[] captionTypeArr = new CaptionType[captionTypeList.size()];
            // java captionTypeList.toArray(captionTypeArr);
            // java dst.setValue(561, captionTypeArr);
            // rust: Java 把 `List` 转成**数组**存回同一属性位；
            //   Rust 侧 `Vec<T>` 即数组语义 ⇒ 该转换是恒等的，原样保留语义。
        }
    }

    // java private static void correctAutoCaption(MutablePropertySet dst)
    fn correctAutoCaption(dst: &mut MutablePropertySet) {
        // java Object autoCaptiosTypes = dst.getValue(562);
        let autoCaptiosTypes_present = dst.getValue(562).is_some();
        // java if (null != autoCaptiosTypes && autoCaptiosTypes instanceof List)
        if autoCaptiosTypes_present {
            // java List<AutoCaptionType> autoCaptiosTypeList = (List)autoCaptiosTypes;
            let autoCaptiosTypeList: &str = "autoCaptiosTypes";
            let _ = autoCaptiosTypeList;
            // java AutoCaptionType[] autoCaptiosTypesArr = new AutoCaptionType[autoCaptiosTypeList.size()];
            // java autoCaptiosTypeList.toArray(autoCaptiosTypesArr);
            // java dst.setValue(562, autoCaptiosTypesArr);
            // rust: 同 `correctCaption`：`List`→数组 在 Rust 侧为恒等转换。
        }
    }

    // java private CaptionType convertCaption(Attributes attributes)
    fn convertCaption(&self, attributes: &dyn Attributes) -> CaptionType {
        // java String attrName = "name";
        let attrName = "name";
        // java String attrPos = "pos";
        let attrPos = "pos";
        // java String attrChapNum = "chapNum";
        let attrChapNum = "chapNum";
        // java String attrHeading = "heading";
        let attrHeading = "heading";
        // java String attrNoLabel = "noLabel";
        let attrNoLabel = "noLabel";
        // java String attrNumFmt = "numFmt";
        let attrNumFmt = "numFmt";
        // java String attrSep = "sep";
        let attrSep = "sep";
        // java String name = ConvertHelper.getString(attributes, "name");
        let name: Option<String> = ConvertHelper::getString__String(attributes, attrName);
        // java IOAssert.assertNotNull("name should not be null", name);
        IOAssert::assertNotNull_msg("name should not be null", &name);
        // java CaptionType captionType = new CaptionType(name);
        let mut captionType = CaptionType::CaptionType(name.unwrap_or_default());
        // java String pos = ConvertHelper.getString(attributes, "pos");
        let pos: Option<String> = ConvertHelper::getString__String(attributes, attrPos);
        // java if (null != pos)
        if pos.is_some() {
            // java captionType.setPos(convertCaptionPos(pos));
            captionType.setPos(Self::convertCaptionPos(&pos.unwrap()));
        }

        // java Boolean chapNum = ConvertHelper.getBoolean(attributes, "chapNum");
        let chapNum: Option<bool> = ConvertHelper::getBoolean(attributes, attrChapNum);
        // java if (null != chapNum)
        if chapNum.is_some() {
            // java captionType.setChapNum(chapNum);
            captionType.setChapNum(chapNum.unwrap());
        }

        // java Integer heading = ShareConvertHelper.getInteger4STDecimalNumber(attributes, "heading");
        let heading: Option<i32> =
            ShareConvertHelper::getInteger4STDecimalNumber__attributes_string(attributes, attrHeading);
        // java if (null != heading)
        if heading.is_some() {
            // java captionType.setHeading(heading);
            captionType.setHeading(heading.unwrap());
        }

        // java Boolean label = ConvertHelper.getBoolean(attributes, "noLabel");
        let label: Option<bool> = ConvertHelper::getBoolean(attributes, attrNoLabel);
        // java if (null != label)
        if label.is_some() {
            // java captionType.setChapNum(label);
            // rust: ★Java 原文此处调用 `setChapNum`（而非 `setNoLabel`），
            //   与变量名 `label`/属性名 `noLabel` 不符，属**疑似笔误**。
            //   按"复现 Java、不改进 Java"的纪律**原样保留**，由 CP6 锁定。
            captionType.setChapNum(label.unwrap());
        }

        // java String numFmt = ConvertHelper.getString(attributes, "numFmt");
        let numFmt: Option<String> = ConvertHelper::getString__String(attributes, attrNumFmt);
        // java if (null != numFmt)
        if numFmt.is_some() {
            // java captionType.setNumFmt(ConvertHelper.xmlNfc2MsoNfc(numFmt, this.isXml03()));
            let converted = ConvertHelper::xmlNfc2MsoNfc(&numFmt.unwrap(), self.base.isXml03());
            captionType.setNumFmt(converted.unwrap_or(0));
        }

        // java String sep = ConvertHelper.getString(attributes, "sep");
        let sep: Option<String> = ConvertHelper::getString__String(attributes, attrSep);
        // java if (null != sep)
        if sep.is_some() {
            // java captionType.setSep(convertChapterSep(sep));
            captionType.setSep(Self::convertChapterSep(&sep.unwrap()));
        }

        // java return captionType;
        return captionType;
    }

    // java private static int convertCaptionPos(String captionPos)
    fn convertCaptionPos(captionPos: &str) -> i32 {
        // java int corePos = 0;
        let mut corePos: i32 = 0;
        // java if ("above".equals(captionPos))
        if "above" == captionPos {
            // java corePos = 0;
            corePos = 0;
        // java } else if ("below".equals(captionPos))
        } else if "below" == captionPos {
            // java corePos = 1;
            corePos = 1;
        // java } else if ("left".equals(captionPos))
        } else if "left" == captionPos {
            // java corePos = 2;
            corePos = 2;
        // java } else if ("right".equals(captionPos))
        } else if "right" == captionPos {
            // java corePos = 3;
            corePos = 3;
        } else {
            // java IOAssert.fail("It should not reach here!");
            IOAssert::fail_msg("It should not reach here!");
        }

        // java return corePos;
        return corePos;
    }

    // java private static int convertChapterSep(String chapterSep)
    fn convertChapterSep(chapterSep: &str) -> i32 {
        // java int coreSep = 0;
        let mut coreSep: i32 = 0;
        // java if ("colon".equals(chapterSep))
        if "colon" == chapterSep {
            // java coreSep = 2;
            coreSep = 2;
        // java } else if ("emDash".equals(chapterSep))
        } else if "emDash" == chapterSep {
            // java coreSep = 3;
            coreSep = 3;
        // java } else if ("enDash".equals(chapterSep))
        } else if "enDash" == chapterSep {
            // java coreSep = 4;
            coreSep = 4;
        // java } else if ("hyphen".equals(chapterSep))
        } else if "hyphen" == chapterSep {
            // java coreSep = 0;
            coreSep = 0;
        // java } else if ("period".equals(chapterSep))
        } else if "period" == chapterSep {
            // java coreSep = 1;
            coreSep = 1;
        } else {
            // java IOAssert.fail("It should not reach here!");
            IOAssert::fail_msg("It should not reach here!");
        }

        // java return coreSep;
        return coreSep;
    }

    // java private static AutoCaptionType convertAutoCaptions(Attributes attributes)
    fn convertAutoCaptions(attributes: &dyn Attributes) -> AutoCaptionType {
        // java String attrName = "name";
        let attrName = "name";
        // java String attrCaption = "caption";
        let attrCaption = "caption";
        // java String name = ConvertHelper.getString(attributes, "name");
        let name: Option<String> = ConvertHelper::getString__String(attributes, attrName);
        // java String caption = ConvertHelper.getString(attributes, "caption");
        let caption: Option<String> = ConvertHelper::getString__String(attributes, attrCaption);
        // java return new AutoCaptionType(name, caption);
        return AutoCaptionType::AutoCaptionType(name.unwrap_or_default(), caption.unwrap_or_default());
    }
}
