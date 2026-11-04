// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.settings.SettingsImporter
//
// ★M-STAGE1 P2 —— 从 auto-generated 空壳（112 行 / 102 处 `todo!()`）**分段做实**。
//   Java 原文 1774 行 / 112 个方法（其中 79 个为 static 叶子）。
//
// ★依赖闭包已于 §25–§29 **全部做实**：
//   `NoLineBreaksImporter` `CapationsImporter` `DocumentProtectionIO`
//   `DocVar` `AutoCaptionType` `CaptionType` `OfficePasswordUtil`
//   `ProtectionPassword` `ProtectionType` `IncorrectPasswordException`
//   `KDocumentProtection` `ProtectionListener`
//
// ★分段策略（按依赖深度自底向上）—— **S1–S5 已全部完成**：
//   [S1] MathPr 静态叶子（17 个方法）—— 纯 `(Attributes, MutablePropertySet)` 转换器 ✅
//   [S2] 其余静态叶子（setView/convert* 等）✅
//   [S3] `correct*` 系列（correctSettingsData 的 11 个子步）✅
//   [S4] `importSettingsStart` / `importSettingsEnd` / 构造器 ✅
//   [S5] `importSettingsSimpleChild`（Java **175 路** switch）✅
//
// ★当前状态：文档保护属性链已接入真实 settings SAX 产品入口。
//   仍有 2 个 `DEFERRED_NON_FUNCTION`：写保护、修订状态。
//   这些分支必须在对应文档语料进入最终验收前完成。
//
// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ShareConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::*;
// Java: import cn.wps.io.xwpf.usermodel.XWPFSettings
use io_wps_io::cn::wps::io::xwpf::usermodel::XWPFSettings::XWPFSettings;
// Java: import org.xml.sax.Attributes
// rust: 本仓 SAX `Attributes` 的权威 trait 定义在 `ConvertHelper` 内
//   （全仓一致做法，见 `NoLineBreaksImporter` / `CapationsImporter`）。
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

// java private static final String VAL = "val";
const VAL: &str = "val";

// java public class SettingsImporter extends ImporterBase
pub struct SettingsImporter {
    // java private TextDocument mTextDocument;
    mTextDocument: Option<*mut TextDocument>,
    // java private MutablePropertySet mDst;
    mDst: Option<MutablePropertySet>,
    // java private XWPFSettings mSettingsPart;
    mSettingsPart: Option<*const XWPFSettings>,
    // java private MutablePropertySet mMathPrSet;
    mMathPrSet: Option<MutablePropertySet>,
    // java private int mCurParentId = 1434631203;
    mCurParentId: i32,
    // java private NoLineBreaksImporter mNoLineBreaksImporter;
    mNoLineBreaksImporter: Option<NoLineBreaksImporter>,
    // java private IDLProvider mDlProvider;
    mDlProvider: Option<()>,
    // java private IOListener mIoListener;
    mIoListener: Option<()>,
    // java private CapationsImporter mCaptionsImporter;
    mCaptionsImporter: Option<CapationsImporter>,
    // rust: Java `extends ImporterBase` ⇒ 组合基类（与其它 importer 一致的做法）
    base: ImporterBase,
}

impl Default for SettingsImporter {
    /// rust: Java 字段初值 `private int mCurParentId = 1434631203;`
    /// —— `#[derive(Default)]` 会给出 0，故**手工实现以忠实复现 Java 初值**。
    fn default() -> Self {
        SettingsImporter {
            mTextDocument: None,
            mDst: None,
            mSettingsPart: None,
            mMathPrSet: None,
            mCurParentId: 1434631203,
            mNoLineBreaksImporter: None,
            mDlProvider: None,
            mIoListener: None,
            mCaptionsImporter: None,
            base: ImporterBase::default(),
        }
    }
}

// Java: import cn.wps.moffice.property.MutablePropertySet  (已在上方)
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
// java import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::Xml03Context;
// java import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::ImporterBase;
// Java: import cn.wps.moffice.writer.data.value.DocVar
use crate::cn::wps::moffice::writer::data::value::DocVar::DocVar;
// Java: import cn.wps.moffice.writer.data.value.MathProperty
use crate::cn::wps::moffice::writer::data::value::MathProperty::MathProperty;
// Java: 同包（settings）—— 本轮新建的闭包成员
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::StylePaneFormatFilterImporter::StylePaneFormatFilterImporter;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::SmartTagTypeImporter::SmartTagTypeImporter;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::ClrSchemeMappingImporter::ClrSchemeMappingImporter;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
// Java: import cn.wps.moffice.writer.data.value.CompatSetting
use crate::cn::wps::moffice::writer::data::value::CompatSetting::CompatSetting;
// Java: import cn.wps.moffice.writer.core.protection.DocumentProtectionIO
use crate::cn::wps::moffice::writer::core::protection::DocumentProtectionIO::DocumentProtectionIO;
use crate::cn::wps::moffice::writer::core::protection::ProtectionType::ProtectionType;
use crate::cn::wps::moffice::writer::core::protection::ProtectionPassword::ProtectionPassword;
// Java: 同包（settings）—— 闭包成员，§25/§29 已做实
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::NoLineBreaksImporter::NoLineBreaksImporter;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::CapationsImporter::CapationsImporter;
// Java: 同包（settings）—— 本轮新建的加密 provider 枚举。
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::settings::CryptProvType::CryptProvType;

/// rust: Java 各处内联的字面量命名空间；提为常量以免重复且保证一致。
/// Java: "http://schemas.openxmlformats.org/officeDocument/2006/math"
const MATH_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";

impl SettingsImporter {
    // java public SettingsImporter(TextDocument textDocument, IOListener ioListener, IDLProvider dlProvider, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn SettingsImporter(
        textDocument: *mut TextDocument,
        ioListener: Option<()>,
        dlProvider: Option<()>,
        mode: i32,
        xml07Context: Option<Xml07Context>,
        xml03Context: Option<Xml03Context>,
    ) -> Self {
        // java super(mode, xml07Context, xml03Context);
        let base = ImporterBase::ImporterBase(mode, xml07Context, xml03Context);
        // java IOAssert.assertNotNull("textDocument should not be null", textDocument);
        // rust: `TextDocument` 按值传入 ⇒ 天然非空，断言语义由类型系统保证。
        // java this.mDst = new MutablePropertySet(textDocument.getProperty());
        let textDocumentProperty = unsafe { &*textDocument }.getProperty();
        // java this.mTextDocument = textDocument;
        let mut this = SettingsImporter {
            mTextDocument: Some(textDocument),
                // java this.mDst = new MutablePropertySet(textDocument.getProperty());
            mDst: Some(MutablePropertySet::new_from_ps(&textDocumentProperty)),
            // java this.mDlProvider = dlProvider;
            mDlProvider: dlProvider,
            // java this.mIoListener = ioListener;
            mIoListener: ioListener,
            mSettingsPart: None,
            mMathPrSet: None,
            mCurParentId: 1434631203,
            mNoLineBreaksImporter: None,
            mCaptionsImporter: None,
            base,
        };
        // java if (this.isXml03())
        if this.base.isXml03() {
            // java this.mDst.setBoolean(524, true);
            this.mDst.as_mut().expect("mDst").setBoolean(524, true);
        }
        // java （构造器结束）
        this
    }
    // java public void importSettingsStart(XWPFSettings xwpfSettings)
    pub fn importSettingsStart(&mut self, xwpfSettings: &XWPFSettings) {
        // java IOAssert.assertNotNull("xwpfSettings should not be null", xwpfSettings);
        // rust: `&XWPFSettings` 天然非空 ⇒ 断言语义由类型系统保证。
        // java this.mSettingsPart = xwpfSettings;
        self.mSettingsPart = Some(xwpfSettings as *const XWPFSettings);
    }
    // java public void importSettingsEnd()
    pub fn importSettingsEnd(&mut self) {
        // java this.correctSettingsData();
        self.correctSettingsData();
        // java this.mTextDocument.setProperty(this.mDst.freeze());
        let frozen = self.mDst.as_mut().expect("mDst").freeze();
        let p = self.mTextDocument.expect("mTextDocument");
        unsafe { &mut *p }.setProperty__propertyset(&frozen);
    }
    // java private void correctSettingsData()
    pub fn correctSettingsData(&mut self) {
        // rust: Java 全程原地修改**同一个** `this.mDst` 对象（引用语义）。
        //   Rust `Option<MutablePropertySet>` 无法既"取出"又"借用 self"，
        //   故先 `take()` 成局部变量，全部子步跑完后一次写回 ——
        //   与 Java 的可观测终点（`mDst` 含全部修正）完全一致。
        let mut dst = self.mDst.take().unwrap_or_default();
        // java correctView(this.mDst);
        Self::correctView(&mut dst);
        // java correctCompactSetting(this.mDst);
        Self::correctCompactSetting(&mut dst);
        // java crrectStylePaneFormatFilter(this.mDst);
        Self::crrectStylePaneFormatFilter(&mut dst);
        // java crrectStylePaneSortMethod(this.mDst);
        Self::crrectStylePaneSortMethod(&mut dst);
        // java crrectDocVars(this.mDst);
        Self::crrectDocVars(&mut dst);
        // java correctAttachedSchema(this.mDst);
        Self::correctAttachedSchema(&mut dst);
        // java correctDoNotIncludeSubdocsInStats(this.mDst);
        Self::correctDoNotIncludeSubdocsInStats(&mut dst);
        // java correctCaptions(this.mDst);
        Self::correctCaptions(&mut dst);
        // java correctSmartTagType(this.mDst);
        Self::correctSmartTagType(&mut dst);
        // java this.correctNoLineBreaks(this.mDst);
        self.correctNoLineBreaks(&mut dst);
        // java this.correctMathPr();
        // Rust: correctMathPr accesses Java's shared mDst field; restore it before the call.
        self.mDst = Some(dst);
        self.correctMathPr();
        let dst = self.mDst.take().expect("mDst");
        // java this.mTextDocument.correctReadonlyTrackchanges();
        if let Some(p) = self.mTextDocument {
            unsafe { &mut *p }.correctReadonlyTrackchanges();
        }
        // Rust: restore mDst after the shared-field correctMathPr call (Java mutates it in place).
        self.mDst = Some(dst);
    }
    // java private void correctMathPr()
    pub fn correctMathPr(&mut self) {
        // java if (null != this.mMathPrSet)
        if self.mMathPrSet.is_some() {
            // java if (this.mMathPrSet.contains(610))
            if self.mMathPrSet.as_ref().unwrap().contains(610) {
                // java this.mMathPrSet.remove(611);
                self.mMathPrSet.as_mut().unwrap().remove(611);
            }
            // java MathProperty mathProperty = new MathProperty(this.mMathPrSet.freeze());
            let mathProperty = MathProperty::MathProperty(self.mMathPrSet.as_mut().unwrap().freeze());
            // java this.mDst.setValue(642, mathProperty);
            self.mDst
                .as_mut()
                .expect("mDst")
                .setValue(642, Some(std::sync::Arc::new(mathProperty)));
        }
    }
    // java public void importSettingsSimpleChild(int elementId, Attributes attributes) throws SAXException
    pub fn importSettingsSimpleChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
        // java IOAssert.assertNotNull("attributes should not be null!", attributes);
        // rust: `&dyn` 天然非空 ⇒ 由类型系统保证。
        // rust: Java 全程写 `this.mDst` / `this.mMathPrSet`（引用语义）。
        //   Rust `Option<..>` 无法既 take 又借 self ⇒ 先取出成局部变量，
        //   match 跑完后一次写回（与 `correctSettingsData` 同一处置手法）。
        let mut dst = self.mDst.take().unwrap_or_default();
        let mut mathPr = self.mMathPrSet.take();
        // java switch (elementId)
        match elementId {
            // java case -2134705041:
            -2134705041 => {
                // java setBoolean(attributes, 524, this.mDst);;
                Self::setBoolean_1(attributes, 524, &mut dst);
                // java break;
            }
            // java case -2113273106:
            -2113273106 => {
                // java setBoolean(attributes, 456, this.mDst);;
                Self::setBoolean_1(attributes, 456, &mut dst);
                // java break;
            }
            // java case -2076330663:
            -2076330663 => {
                // java setBoolean(attributes, 482, this.mDst);;
                Self::setBoolean_1(attributes, 482, &mut dst);
                // java break;
            }
            // java case -2068700649:
            -2068700649 => {
                // java setBoolean(attributes, 519, this.mDst);;
                Self::setBoolean_1(attributes, 519, &mut dst);
                // java break;
            }
            // java case -1989671957:
            -1989671957 => {
                // java setSmartTagType(attributes, this.mDst);;
                Self::setSmartTagType(attributes, &mut dst);
                // java break;
            }
            // java case -1937589614:
            -1937589614 => {
                // java setBoolean(attributes, 398, this.mDst);;
                Self::setBoolean_1(attributes, 398, &mut dst);
                // java break;
            }
            // java case -1933025740:
            -1933025740 => {
                // java setBoolean(attributes, 488, this.mDst);;
                Self::setBoolean_1(attributes, 488, &mut dst);
                // java break;
            }
            // java case -1918151738:
            -1918151738 => {
                // java setThemeFontLang(attributes, this.mDst);;
                Self::setThemeFontLang(attributes, &mut dst);
                // java break;
            }
            // java case -1917338065:
            -1917338065 => {
                // java this.setEndNotePr();;
                self.setEndNotePr();
                // java break;
            }
            // java case -1908583373:
            -1908583373 => {
                // java setBoolean(attributes, 389, this.mDst);;
                Self::setBoolean_1(attributes, 389, &mut dst);
                // java break;
            }
            // java case -1891304888:
            -1891304888 => {
                // java setBoolean(attributes, 503, this.mDst);;
                Self::setBoolean_1(attributes, 503, &mut dst);
                // java break;
            }
            // java case -1879087818:
            -1879087818 => {
                // java setBoolean(attributes, 533, this.mDst);;
                Self::setBoolean_1(attributes, 533, &mut dst);
                // java break;
            }
            // java case -1813563987:
            -1813563987 => {
                // java setProofState(attributes, this.mDst);;
                Self::setProofState(attributes, &mut dst);
                // java break;
            }
            // java case -1810657591:
            -1810657591 => {
                // java setDrawingGridVerticalSpacing(attributes, this.mDst);;
                Self::setDrawingGridVerticalSpacing(attributes, &mut dst);
                // java break;
            }
            // java case -1762893632:
            -1762893632 => {
                // java setClrSchemeMapping(attributes, this.mDst);;
                Self::setClrSchemeMapping(attributes, &mut dst);
                // java break;
            }
            // java case -1756495436:
            -1756495436 => {
                // java setBreakOnBinarySubtraction(attributes, this.mMathPrSet);;
                Self::setBreakOnBinarySubtraction(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case -1736979050:
            -1736979050 => {
                // java setWrapIndent(attributes, this.mMathPrSet);;
                Self::setWrapIndent(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case -1706783841:
            -1706783841 => {
                // java setConsecutiveHyphenLimit(attributes, this.mDst);;
                Self::setConsecutiveHyphenLimit(attributes, &mut dst);
                // java break;
            }
            // java case -1702461273:
            -1702461273 => {
                // java setBoolean(attributes, 532, this.mDst);;
                Self::setBoolean_1(attributes, 532, &mut dst);
                // java break;
            }
            // java case -1651106543:
            -1651106543 => {
                // java setBoolean(attributes, 442, this.mDst);;
                Self::setBoolean_1(attributes, 442, &mut dst);
                // java break;
            }
            // java case -1648882127:
            -1648882127 => {
                // java setBoolean(attributes, 399, this.mDst);;
                Self::setBoolean_1(attributes, 399, &mut dst);
                // java break;
            }
            // java case -1647305555:
            -1647305555 => {
                // java setBoolean(attributes, 441, this.mDst);;
                Self::setBoolean_1(attributes, 441, &mut dst);
                // java break;
            }
            // java case -1636932838:
            -1636932838 => {
                // java setBoolean(attributes, 499, this.mDst);;
                Self::setBoolean_1(attributes, 499, &mut dst);
                // java break;
            }
            // java case -1623637099:
            -1623637099 => {
                // java setSmallFraction(attributes, this.mMathPrSet);;
                Self::setSmallFraction(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case -1601567069:
            -1601567069 => {
                // java setBoolean(attributes, 404, this.mDst);;
                Self::setBoolean_1(attributes, 404, &mut dst);
                // java break;
            }
            // java case -1583930533:
            -1583930533 => {
                // java setDrawingGridHorizontalSpacing(attributes, this.mDst);;
                Self::setDrawingGridHorizontalSpacing(attributes, &mut dst);
                // java break;
            }
            // java case -1572734800:
            -1572734800 => {
                // java setBoolean(attributes, 506, this.mDst);;
                Self::setBoolean_1(attributes, 506, &mut dst);
                // java break;
            }
            // java case -1556309024:
            -1556309024 => {
                // java setDrawingGridVerticalOrigin(attributes, this.mDst);;
                Self::setDrawingGridVerticalOrigin(attributes, &mut dst);
                // java break;
            }
            // java case -1473196299:
            -1473196299 => {
                // java setDocumentType(attributes, this.mDst);;
                Self::setDocumentType(attributes, &mut dst);
                // java break;
            }
            // java case -1434364504:
            -1434364504 => {
                // java setStylePaneFormatFilter(attributes, this.mDst);;
                Self::setStylePaneFormatFilter(attributes, &mut dst);
                // java break;
            }
            // java case -1426390301:
            -1426390301 => {
                // java setBoolean(attributes, 497, this.mDst);;
                Self::setBoolean_1(attributes, 497, &mut dst);
                // java break;
            }
            // java case -1424803146:
            -1424803146 => {
                // java setBoolean(attributes, 502, this.mDst);;
                Self::setBoolean_1(attributes, 502, &mut dst);
                // java break;
            }
            // java case -1380774676:
            -1380774676 => {
                // java serBreakOnBinaryOperators(attributes, this.mMathPrSet);;
                Self::serBreakOnBinaryOperators(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case -1354586665:
            -1354586665 => {
                // java setBoolean(attributes, 414, this.mDst);;
                Self::setBoolean_1(attributes, 414, &mut dst);
                // java break;
            }
            // java case -1326506289:
            -1326506289 => {
                // java setDocVar(attributes, this.mDst);;
                Self::setDocVar(attributes, &mut dst);
                // java break;
            }
            // java case -1325812493:
            -1325812493 => {
                // java setBoolean(attributes, 517, this.mDst);;
                Self::setBoolean_1(attributes, 517, &mut dst);
                // java break;
            }
            // java case -1322964766:
            -1322964766 => {
                // java setBoolean(attributes, 479, this.mDst);;
                Self::setBoolean_1(attributes, 479, &mut dst);
                // java break;
            }
            // java case -1268124223:
            -1268124223 => {
                // java setBoolean(attributes, 388, this.mDst);;
                Self::setBoolean_1(attributes, 388, &mut dst);
                // java break;
            }
            // java case -1250426592:
            -1250426592 => {
                // java setRevisionView(attributes, this.mDst);;
                Self::setRevisionView(attributes, &mut dst);
                // java break;
            }
            // java case -1228639629:
            -1228639629 => {
                // java setBoolean(attributes, 466, this.mDst);;
                Self::setBoolean_1(attributes, 466, &mut dst);
                // java break;
            }
            // java case -1202800732:
            -1202800732 => {
                // java setDefaultTableStyle(attributes, this.mDst);;
                Self::setDefaultTableStyle(attributes, &mut dst);
                // java break;
            }
            // java case -1200535596:
            -1200535596 => {
                // java setBoolean(attributes, 448, this.mDst);;
                Self::setBoolean_1(attributes, 448, &mut dst);
                // java break;
            }
            // java case -1185891341:
            -1185891341 => {
                // java setBoolean(attributes, 383, this.mDst);;
                Self::setBoolean_1(attributes, 383, &mut dst);
                // java break;
            }
            // java case -1183786975:
            -1183786975 => {
                // java setIntegralLimitLocations(attributes, this.mMathPrSet);;
                Self::setIntegralLimitLocations(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case -1136057609:
            -1136057609 => {
                // java setBoolean(attributes, 402, this.mDst);;
                Self::setBoolean_1(attributes, 402, &mut dst);
                // java break;
            }
            // java case -1114891571:
            -1114891571 => {
                // java setBoolean(attributes, 417, this.mDst);;
                Self::setBoolean_1(attributes, 417, &mut dst);
                // java break;
            }
            // java case -1109412115:
            -1109412115 => {
                // java setTrackRevisions(attributes, this.mTextDocument);;
                Self::setTrackRevisions(attributes, unsafe { &mut *self.mTextDocument.expect("mTextDocument") });
                // java break;
            }
            // java case -1088528308:
            -1088528308 => {
                // java setBoolean(attributes, 403, this.mDst);;
                Self::setBoolean_1(attributes, 403, &mut dst);
                // java break;
            }
            // java case -1081250006:
            -1081250006 => {
                // java this.initMathPrSet();;
                self.initMathPrSet();
                mathPr = None;
                // java break;
            }
            // java case -982451427:
            -982451427 => {
                // java setPostParagraphSpacing(attributes, this.mMathPrSet);;
                Self::setPostParagraphSpacing(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case -974208031:
            -974208031 => {
                // java setBoolean(attributes, 409, this.mDst);;
                Self::setBoolean_1(attributes, 409, &mut dst);
                // java break;
            }
            // java case -962302288:
            -962302288 => {
                // java setBoolean(attributes, 513, this.mDst);;
                Self::setBoolean_1(attributes, 513, &mut dst);
                // java break;
            }
            // java case -938949228:
            -938949228 => {
                // java setBoolean(attributes, 391, this.mDst);;
                Self::setBoolean_1(attributes, 391, &mut dst);
                // java break;
            }
            // java case -905658143:
            -905658143 => {
                // java setBoolean(attributes, 531, this.mDst);;
                Self::setBoolean_1(attributes, 531, &mut dst);
                // java break;
            }
            // java case -860883301:
            -860883301 => {
                // java setBoolean(attributes, 480, this.mDst);;
                Self::setBoolean_1(attributes, 480, &mut dst);
                // java break;
            }
            // java case -838508233:
            -838508233 => {
                // java this.setAutoCaption(attributes, this.mDst);;
                self.setAutoCaption(attributes, &mut dst);
                // java break;
            }
            // java case -814900736:
            -814900736 => {
                // java setBoolean(attributes, 528, this.mDst);;
                Self::setBoolean_1(attributes, 528, &mut dst);
                // java break;
            }
            // java case -790730617:
            -790730617 => {
                // java setBoolean(attributes, 496, this.mDst);;
                Self::setBoolean_1(attributes, 496, &mut dst);
                // java break;
            }
            // java case -736154697:
            -736154697 => {
                // java setBoolean(attributes, 539, this.mDst);;
                Self::setBoolean_1(attributes, 539, &mut dst);
                // java break;
            }
            // java case -701177766:
            -701177766 => {
                // java setReadModeInkLockDown(attributes, this.mDst);;
                Self::setReadModeInkLockDown(attributes, &mut dst);
                // java break;
            }
            // java case -692770203:
            -692770203 => {
                // java setBoolean(attributes, 485, this.mDst);;
                Self::setBoolean_1(attributes, 485, &mut dst);
                // java break;
            }
            // java case -684373152:
            -684373152 => {
                // java setBoolean(attributes, 487, this.mDst);;
                Self::setBoolean_1(attributes, 487, &mut dst);
                // java break;
            }
            // java case -646125444:
            -646125444 => {
                // java setBoolean(attributes, 405, this.mDst);;
                Self::setBoolean_1(attributes, 405, &mut dst);
                // java break;
            }
            // java case -636325574:
            -636325574 => {
                // java setLeftMargin(attributes, this.mMathPrSet);;
                Self::setLeftMargin(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case -626055124:
            -626055124 => {
                // java setBoolean(attributes, 493, this.mDst);;
                Self::setBoolean_1(attributes, 493, &mut dst);
                // java break;
            }
            // java case -588222375:
            -588222375 => {
                // java setBoolean(attributes, 401, this.mDst);;
                Self::setBoolean_1(attributes, 401, &mut dst);
                // java break;
            }
            // java case -569660999:
            -569660999 => {
                // java setBoolean(attributes, 451, this.mDst);;
                Self::setBoolean_1(attributes, 451, &mut dst);
                // java break;
            }
            // java case -553493737:
            -553493737 => {
                // java setBoolean(attributes, 515, this.mDst);;
                Self::setBoolean_1(attributes, 515, &mut dst);
                // java break;
            }
            // java case -519350417:
            -519350417 => {
                // java setDisplayVerticalDrawingGridEvery(attributes, this.mDst);;
                Self::setDisplayVerticalDrawingGridEvery(attributes, &mut dst);
                // java break;
            }
            // java case -438809570:
            -438809570 => {
                // java setBoolean(attributes, 524, this.mDst, false);;
                Self::setBoolean(attributes, 524, &mut dst, false);
                // java break;
            }
            // java case -434582953:
            -434582953 => {
                // java setBoolean(attributes, 478, this.mDst);;
                Self::setBoolean_1(attributes, 478, &mut dst);
                // java break;
            }
            // java case -417953304:
            -417953304 => {
                // java this.setNoLineBreaksBefore(attributes);;
                self.setNoLineBreaksBefore(attributes);
                // java break;
            }
            // java case -347335209:
            -347335209 => {
                // java this.setStrictFirstAndLastChars(attributes, this.mDst);;
                self.setStrictFirstAndLastChars(attributes, &mut dst);
                // java break;
            }
            // java case -334642556:
            -334642556 => {
                // java setBoolean(attributes, 526, this.mDst);;
                Self::setBoolean_1(attributes, 526, &mut dst);
                // java break;
            }
            // java case -332797016:
            -332797016 => {
                // java setBoolean(attributes, 420, this.mDst);;
                Self::setBoolean_1(attributes, 420, &mut dst);
                // java break;
            }
            // java case -319044795:
            -319044795 => {
                // java setBoolean(attributes, 406, this.mDst);;
                Self::setBoolean_1(attributes, 406, &mut dst);
                // java break;
            }
            // java case -314910223:
            -314910223 => {
                // java setBoolean(attributes, 400, this.mDst);;
                Self::setBoolean_1(attributes, 400, &mut dst);
                // java break;
            }
            // java case -312379450:
            -312379450 => {
                // java setBoolean(attributes, 408, this.mDst);;
                Self::setBoolean_1(attributes, 408, &mut dst);
                // java break;
            }
            // java case -238953362:
            -238953362 => {
                // java setCompatSetting(attributes, this.mDst);;
                Self::setCompatSetting(attributes, &mut dst);
                // java break;
            }
            // java case -216851567:
            -216851567 => {
                // java setBoolean(attributes, 510, this.mDst);;
                Self::setBoolean_1(attributes, 510, &mut dst);
                // java break;
            }
            // java case -209259536:
            -209259536 => {
                // java setBoolean(attributes, 387, this.mDst);;
                Self::setBoolean_1(attributes, 387, &mut dst);
                // java break;
            }
            // java case -195827800:
            -195827800 => {
                // java setBoolean(attributes, 397, this.mDst);;
                Self::setBoolean_1(attributes, 397, &mut dst);
                // java break;
            }
            // java case -169608324:
            -169608324 => {
                // java setBoolean(attributes, 407, this.mDst);;
                Self::setBoolean_1(attributes, 407, &mut dst);
                // java break;
            }
            // java case -124054063:
            -124054063 => {
                // java setBoolean(attributes, 536, this.mDst);;
                Self::setBoolean_1(attributes, 536, &mut dst);
                // java break;
            }
            // java case -87807397:
            -87807397 => {
                // java setBoolean(attributes, 527, this.mDst);;
                Self::setBoolean_1(attributes, 527, &mut dst);
                // java break;
            }
            // java case -59805965:
            -59805965 => {
                // java setBoolean(attributes, 469, this.mDst);;
                Self::setBoolean_1(attributes, 469, &mut dst);
                // java break;
            }
            // java case -47865870:
            -47865870 => {
                // java setWrapRight(attributes, this.mMathPrSet);;
                Self::setWrapRight(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case -17550310:
            -17550310 => {
                // java setBoolean(attributes, 512, this.mDst);;
                Self::setBoolean_1(attributes, 512, &mut dst);
                // java break;
            }
            // java case 111188:
            111188 => {
                // java this.setPos(attributes, this.mDst);;
                self.setPos(attributes, &mut dst);
                // java break;
            }
            // java case 3619493:
            3619493 => {
                // java setView(attributes, this.mDst);;
                Self::setView(attributes, &mut dst);
                // java break;
            }
            // java case 3744723:
            3744723 => {
                // java setZoom(attributes, this.mDst);;
                Self::setZoom(attributes, &mut dst);
                // java break;
            }
            // java case 4960031:
            4960031 => {
                // java setBoolean(attributes, 393, this.mDst);;
                Self::setBoolean_1(attributes, 393, &mut dst);
                // java break;
            }
            // java case 9168323:
            9168323 => {
                // java setBoolean(attributes, 498, this.mDst);;
                Self::setBoolean_1(attributes, 498, &mut dst);
                // java break;
            }
            // java case 71947221:
            71947221 => {
                // java setBoolean(attributes, 521, this.mDst);;
                Self::setBoolean_1(attributes, 521, &mut dst);
                // java break;
            }
            // java case 73898596:
            73898596 => {
                // java setBoolean(attributes, 474, this.mDst);;
                Self::setBoolean_1(attributes, 474, &mut dst);
                // java break;
            }
            // java case 95461406:
            95461406 => {
                // java setDefaultJustifition(attributes, this.mMathPrSet);;
                Self::setDefaultJustifition(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case 106930272:
            106930272 => {
                // java setPreParagraphSpacing(attributes, this.mMathPrSet);;
                Self::setPreParagraphSpacing(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case 126608775:
            126608775 => {
                // java setBoolean(attributes, 463, this.mDst);;
                Self::setBoolean_1(attributes, 463, &mut dst);
                // java break;
            }
            // java case 170588716:
            170588716 => {
                // java setSummaryLength(attributes, this.mDst);;
                Self::setSummaryLength(attributes, &mut dst);
                // java break;
            }
            // java case 170752510:
            170752510 => {
                // java setBoolean(attributes, 505, this.mDst);;
                Self::setBoolean_1(attributes, 505, &mut dst);
                // java break;
            }
            // java case 220826588:
            220826588 => {
                // java setBoolean(attributes, 500, this.mDst);;
                Self::setBoolean_1(attributes, 500, &mut dst);
                // java break;
            }
            // java case 275028145:
            275028145 => {
                // java setForceUpgrade(attributes, this.mDst);;
                Self::setForceUpgrade(attributes, &mut dst);
                // java break;
            }
            // java case 283782879:
            283782879 => {
                // java setBoolean(attributes, 547, this.mDst);;
                Self::setBoolean_1(attributes, 547, &mut dst);
                // java break;
            }
            // java case 300532599:
            300532599 => {
                // java setMathFont(attributes, this.mMathPrSet);;
                Self::setMathFont(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case 316436478:
            316436478 => {
                // java setBoolean(attributes, 540, this.mDst);;
                Self::setBoolean_1(attributes, 540, &mut dst);
                // java break;
            }
            // java case 389251180:
            389251180 => {
                // java setBoolean(attributes, 530, this.mDst);;
                Self::setBoolean_1(attributes, 530, &mut dst);
                // java break;
            }
            // java case 393729216:
            393729216 => {
                // java setRightMargin(attributes, this.mMathPrSet);;
                Self::setRightMargin(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case 411569660:
            411569660 => {
                // java setBoolean(attributes, 495, this.mDst);;
                Self::setBoolean_1(attributes, 495, &mut dst);
                // java break;
            }
            // java case 424792560:
            424792560 => {
                // java setBoolean(attributes, 475, this.mDst);;
                Self::setBoolean_1(attributes, 475, &mut dst);
                // java break;
            }
            // java case 450482102:
            450482102 => {
                // java setDefaultTabStop(attributes, this.mDst);;
                Self::setDefaultTabStop(attributes, &mut dst);
                // java break;
            }
            // java case 481385402:
            481385402 => {
                // java setBoolean(attributes, 484, this.mDst);;
                Self::setBoolean_1(attributes, 484, &mut dst);
                // java break;
            }
            // java case 489375826:
            489375826 => {
                // java setBoolean(attributes, 529, this.mDst);;
                Self::setBoolean_1(attributes, 529, &mut dst);
                // java break;
            }
            // java case 508827450:
            508827450 => {
                // java setBoolean(attributes, 518, this.mDst);;
                Self::setBoolean_1(attributes, 518, &mut dst);
                // java break;
            }
            // java case 529214734:
            529214734 => {
                // java setDrawingGridHorizontalOrigin(attributes, this.mDst);;
                Self::setDrawingGridHorizontalOrigin(attributes, &mut dst);
                // java break;
            }
            // java case 552573414:
            552573414 => {
                // java this.setCaption(attributes, this.mDst);;
                self.setCaption(attributes, &mut dst);
                // java break;
            }
            // java case 641935877:
            641935877 => {
                // java setBoolean(attributes, 483, this.mDst);;
                Self::setBoolean_1(attributes, 483, &mut dst);
                // java break;
            }
            // java case 650683171:
            650683171 => {
                // java this.setCharacterSpacingControl(attributes, this.mDst);;
                self.setCharacterSpacingControl(attributes, &mut dst);
                // java break;
            }
            // java case 675671887:
            675671887 => {
                // java setHyphenationZone(attributes, this.mDst);;
                Self::setHyphenationZone(attributes, &mut dst);
                // java break;
            }
            // java case 705016829:
            705016829 => {
                // java setBoolean(attributes, 471, this.mDst);;
                Self::setBoolean_1(attributes, 471, &mut dst);
                // java break;
            }
            // java case 751788628:
            751788628 => {
                // java this.setDocumentProtection(attributes, this.mDst);;
                self.setDocumentProtection(attributes, &mut dst);
                // java break;
            }
            // java case 814737448:
            814737448 => {
                // java setClickAndTypeStyle(attributes, this.mDst);;
                Self::setClickAndTypeStyle(attributes, &mut dst);
                // java break;
            }
            // java case 818602208:
            818602208 => {
                // java setBoolean(attributes, 522, this.mDst);;
                Self::setBoolean_1(attributes, 522, &mut dst);
                // java break;
            }
            // java case 843004104:
            843004104 => {
                // java setBoolean(attributes, 492, this.mDst);;
                Self::setBoolean_1(attributes, 492, &mut dst);
                // java break;
            }
            // java case 843501948:
            843501948 => {
                // java setBoolean(attributes, 418, this.mDst);;
                Self::setBoolean_1(attributes, 418, &mut dst);
                // java break;
            }
            // java case 869235382:
            869235382 => {
                // java setBoolean(attributes, 537, this.mDst);;
                Self::setBoolean_1(attributes, 537, &mut dst);
                // java break;
            }
            // java case 886002841:
            886002841 => {
                // java setBoolean(attributes, 481, this.mDst);;
                Self::setBoolean_1(attributes, 481, &mut dst);
                // java break;
            }
            // java case 905124775:
            905124775 => {
                // java setBoolean(attributes, 413, this.mDst);;
                Self::setBoolean_1(attributes, 413, &mut dst);
                // java break;
            }
            // java case 955468371:
            955468371 => {
                // java this.setNoLineBreaksAfter(attributes);;
                self.setNoLineBreaksAfter(attributes);
                // java break;
            }
            // java case 956434160:
            956434160 => {
                // java setBoolean(attributes, 472, this.mDst);;
                Self::setBoolean_1(attributes, 472, &mut dst);
                // java break;
            }
            // java case 959575805:
            959575805 => {
                // java setBoolean(attributes, 534, this.mDst);;
                Self::setBoolean_1(attributes, 534, &mut dst);
                // java break;
            }
            // java case 963411439:
            963411439 => {
                // java setBoolean(attributes, 504, this.mDst);;
                Self::setBoolean_1(attributes, 504, &mut dst);
                // java break;
            }
            // java case 971121639:
            971121639 => {
                // java setListSeparator(attributes, this.mDst);;
                Self::setListSeparator(attributes, &mut dst);
                // java break;
            }
            // java case 985587158:
            985587158 => {
                // java setBoolean(attributes, 514, this.mDst);;
                Self::setBoolean_1(attributes, 514, &mut dst);
                // java break;
            }
            // java case 1030286495:
            1030286495 => {
                // java setBoolean(attributes, 419, this.mDst);;
                Self::setBoolean_1(attributes, 419, &mut dst);
                // java break;
            }
            // java case 1063095734:
            1063095734 => {
                // java setBoolean(attributes, 541, this.mDst);;
                Self::setBoolean_1(attributes, 541, &mut dst);
                // java break;
            }
            // java case 1148496030:
            1148496030 => {
                // java setBoolean(attributes, 501, this.mDst);;
                Self::setBoolean_1(attributes, 501, &mut dst);
                // java break;
            }
            // java case 1170313875:
            1170313875 => {
                // java setBookFoldPrintingSheets(attributes, this.mDst);;
                Self::setBookFoldPrintingSheets(attributes, &mut dst);
                // java break;
            }
            // java case 1176767214:
            1176767214 => {
                // java setBoolean(attributes, 548, this.mDst);;
                Self::setBoolean_1(attributes, 548, &mut dst);
                // java break;
            }
            // java case 1179686487:
            1179686487 => {
                // java setBoolean(attributes, 509, this.mDst);;
                Self::setBoolean_1(attributes, 509, &mut dst);
                // java break;
            }
            // java case 1276243993:
            1276243993 => {
                // java setBoolean(attributes, 494, this.mDst);;
                Self::setBoolean_1(attributes, 494, &mut dst);
                // java break;
            }
            // java case 1290881598:
            1290881598 => {
                // java setBoolean(attributes, 476, this.mDst);;
                Self::setBoolean_1(attributes, 476, &mut dst);
                // java break;
            }
            // java case 1326464151:
            1326464151 => {
                // java setBoolean(attributes, 489, this.mDst);;
                Self::setBoolean_1(attributes, 489, &mut dst);
                // java break;
            }
            // java case 1370596031:
            1370596031 => {
                // java setBoolean(attributes, 457, this.mDst);;
                Self::setBoolean_1(attributes, 457, &mut dst);
                // java break;
            }
            // java case 1394213357:
            1394213357 => {
                // java setBoolean(attributes, 452, this.mDst);;
                Self::setBoolean_1(attributes, 452, &mut dst);
                // java break;
            }
            // java case 1403465629:
            1403465629 => {
                // java setDisplayHorizontalDrawingGridEvery(attributes, this.mDst);;
                Self::setDisplayHorizontalDrawingGridEvery(attributes, &mut dst);
                // java break;
            }
            // java case 1410250724:
            1410250724 => {
                // java setBoolean(attributes, 470, this.mDst);;
                Self::setBoolean_1(attributes, 470, &mut dst);
                // java break;
            }
            // java case 1478011665:
            1478011665 => {
                // java setBoolean(attributes, 567, this.mDst);;
                Self::setBoolean_1(attributes, 567, &mut dst);
                // java break;
            }
            // java case 1486454403:
            1486454403 => {
                // java setBoolean(attributes, 523, this.mDst);;
                Self::setBoolean_1(attributes, 523, &mut dst);
                // java break;
            }
            // java case 1545631590:
            1545631590 => {
                // java setBoolean(attributes, 520, this.mDst);;
                Self::setBoolean_1(attributes, 520, &mut dst);
                // java break;
            }
            // java case 1575613513:
            1575613513 => {
                // java setDecimalSymbol(attributes, this.mDst);;
                Self::setDecimalSymbol(attributes, &mut dst);
                // java break;
            }
            // java case 1577607066:
            1577607066 => {
                // java setBoolean(attributes, 411, this.mDst);;
                Self::setBoolean_1(attributes, 411, &mut dst);
                // java break;
            }
            // java case 1597753918:
            1597753918 => {
                // java setAttachedTemplate(attributes, this.mDst, this.mSettingsPart);;
                Self::setAttachedTemplate(attributes, &mut dst, self.mSettingsPart);
                // java break;
            }
            // java case 1625197708:
            1625197708 => {
                // java setBoolean(attributes, 410, this.mDst);;
                Self::setBoolean_1(attributes, 410, &mut dst);
                // java break;
            }
            // java case 1671725827:
            1671725827 => {
                // java setDisplayMathDefault(attributes, this.mMathPrSet);;
                Self::setDisplayMathDefault(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case 1691035156:
            1691035156 => {
                // java setBoolean(attributes, 511, this.mDst);;
                Self::setBoolean_1(attributes, 511, &mut dst);
                // java break;
            }
            // java case 1727147254:
            1727147254 => {
                // java setNaryLimitLocations(attributes, this.mMathPrSet);;
                Self::setNaryLimitLocations(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case 1734155378:
            1734155378 => {
                // java setBoolean(attributes, 440, this.mDst);;
                Self::setBoolean_1(attributes, 440, &mut dst);
                // java break;
            }
            // java case 1766037289:
            1766037289 => {
                // java setBoolean(attributes, 508, this.mDst);;
                Self::setBoolean_1(attributes, 508, &mut dst);
                // java break;
            }
            // java case 1771222516:
            1771222516 => {
                // java setBoolean(attributes, 473, this.mDst);;
                Self::setBoolean_1(attributes, 473, &mut dst);
                // java break;
            }
            // java case 1780463506:
            1780463506 => {
                // java setBoolean(attributes, 490, this.mDst);;
                Self::setBoolean_1(attributes, 490, &mut dst);
                // java break;
            }
            // java case 1791649555:
            1791649555 => {
                // java setBoolean(attributes, 477, this.mDst);;
                Self::setBoolean_1(attributes, 477, &mut dst);
                // java break;
            }
            // java case 1820694550:
            1820694550 => {
                // java setBoolean(attributes, 535, this.mDst);;
                Self::setBoolean_1(attributes, 535, &mut dst);
                // java break;
            }
            // java case 1827978116:
            1827978116 => {
                // java setDocVars(attributes, this.mDst);;
                Self::setDocVars(attributes, &mut dst);
                // java break;
            }
            // java case 1839947496:
            1839947496 => {
                // java setBoolean(attributes, 467, this.mDst);;
                Self::setBoolean_1(attributes, 467, &mut dst);
                // java break;
            }
            // java case 1908795051:
            1908795051 => {
                // java setBoolean(attributes, 538, this.mDst);;
                Self::setBoolean_1(attributes, 538, &mut dst);
                // java break;
            }
            // java case 1934592950:
            1934592950 => {
                // java setBoolean(attributes, 507, this.mDst);;
                Self::setBoolean_1(attributes, 507, &mut dst);
                // java break;
            }
            // java case 1942426567:
            1942426567 => {
                // java setBoolean(attributes, 412, this.mDst);;
                Self::setBoolean_1(attributes, 412, &mut dst);
                // java break;
            }
            // java case 1958062169:
            1958062169 => {
                // java setInterEquationSpacing(attributes, this.mMathPrSet);;
                Self::setInterEquationSpacing(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case 1958433115:
            1958433115 => {
                // java setIntraEquationSpacing(attributes, this.mMathPrSet);;
                Self::setIntraEquationSpacing(attributes, mathPr.as_mut().expect("mMathPrSet"));
                // java break;
            }
            // java case 1976320440:
            1976320440 => {
                // java setStylePaneSortMethod(attributes, this.mDst);;
                Self::setStylePaneSortMethod(attributes, &mut dst);
                // java break;
            }
            // java case 2027445307:
            2027445307 => {
                // java setBoolean(attributes, 516, this.mDst);;
                Self::setBoolean_1(attributes, 516, &mut dst);
                // java break;
            }
            // java case 2029039703:
            2029039703 => {
                // java setBoolean(attributes, 491, this.mDst);;
                Self::setBoolean_1(attributes, 491, &mut dst);
                // java break;
            }
            // java case 2036560720:
            2036560720 => {
                // java setBoolean(attributes, 525, this.mDst);;
                Self::setBoolean_1(attributes, 525, &mut dst);
                // java break;
            }
            // java case 2057192472:
            2057192472 => {
                // java this.setWriteProtection(attributes);;
                self.setWriteProtection(attributes);
                // java break;
            }
            // java case 2093929573:
            2093929573 => {
                // java setAttachedSchema(attributes, this.mDst);;
                Self::setAttachedSchema(attributes, &mut dst);
                // java break;
            }
            // rust: Java 无 `default:` 分支 ⇒ 未知 elementId **静默忽略**（原样保留）。
            _ => {}
        }
        // rust: 一次写回（Java 为原地修改）
        self.mDst = Some(dst);
        if let Some(mp) = mathPr {
            self.mMathPrSet = Some(mp);
        }
    }
    // java private void setWriteProtection(Attributes attributes) throws SAXException
    pub fn setWriteProtection(&self, attributes: &dyn Attributes) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION Java 原文（逐语句保留）：
      // java String hash = ConvertHelper.getString(attributes, "hash");
      // java String salt = ConvertHelper.getString(attributes, "salt");
      // java Long pSid = ConvertHelper.getLong(attributes, "cryptAlgorithmSid");
      // java Long pSpinCount = ConvertHelper.getLong(attributes, "cryptSpinCount");
      // java if (null != hash && null != salt && null != pSid && null != pSpinCount) {
      // java if (null == this.mDlProvider) {
      // java throw new ForceQuitException(15);
      // java } else {
      // java ModifyVerifier modifyVerifier = new ModifyVerifier(hash, salt, pSpinCount.intValue(), pSid.intValue());
      // java KDocumentWriteProtection cryptInto = this.mTextDocument.cryptInfo();
      // java boolean isReadOnly = this.mDlProvider.isOpenOnReadOnly();
      // java if (isReadOnly) {
      // java cryptInto.setModifyVerifier(modifyVerifier);
      // java cryptInto.setReadOnly(true);
      // java } else {
      // java String writePassword = this.mDlProvider.getWritePassword();
      // java if (writePassword == null) {
      // java throw new ForceQuitException(15);
      // java } else if (!CryptTool.checkModifyPassword(modifyVerifier, writePassword)) {
      // java throw new ForceQuitException(16);
      // java } else {
      // java cryptInto.setWritePassword(writePassword, false);
      // java if (null != this.mIoListener) {
      // java this.mIoListener.onFinishVerifyWritePassword();
      // java }
      // java }
      // java }
      // java }
      // java }
      */
    }
    // java private void initMathPrSet()
    pub fn initMathPrSet(&mut self) {
        // java this.mMathPrSet = new MutablePropertySet();
        self.mMathPrSet = Some(MutablePropertySet::new());
    }
    // java private static void setMathFont(Attributes attributes, MutablePropertySet dst)
    pub fn setMathFont(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // rust: `&mut` 天然非空 ⇒ 由类型系统保证。
        // java String mathFont = ConvertHelper.getString(attributes, "val", "http://schemas.openxmlformats.org/officeDocument/2006/math");
        let mathFont: Option<String> = ConvertHelper::getString__String_String(attributes, "val", MATH_NS);
        // java if (null != mathFont)
        if mathFont.is_some() {
            // java dst.setString(598, mathFont);
            dst.setString(598, mathFont.unwrap());
        }
    }

    // java private static void serBreakOnBinaryOperators(Attributes attributes, MutablePropertySet dst)
    pub fn serBreakOnBinaryOperators(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java String brkBin = ConvertHelper.getString(attributes, "val", "http://schemas.openxmlformats.org/officeDocument/2006/math");
        let brkBin: Option<String> = ConvertHelper::getString__String_String(attributes, "val", MATH_NS);
        // java if (null != brkBin)
        if brkBin.is_some() {
            // java dst.setInt(599, convertBreakBin(brkBin));
            dst.setInt(599, Self::convertBreakBin(brkBin.unwrap()));
        }
    }

    // java private static int convertBreakBin(String brkBin)
    pub fn convertBreakBin(brkBin: String) -> i32 {
        // java if ("before".equals(brkBin))
        if "before" == brkBin {
            // java return 0;
            return 0;
        // java } else if ("after".equals(brkBin))
        } else if "after" == brkBin {
            // java return 1;
            return 1;
        // java } else if ("repeat".equals(brkBin))
        } else if "repeat" == brkBin {
            // java return 2;
            return 2;
        } else {
            // java IOAssert.fail("it should not reach here!");
            IOAssert::fail_msg("it should not reach here!");
            // java return 0;
            return 0;
        }
    }

    // java private static void setBreakOnBinarySubtraction(Attributes attributes, MutablePropertySet dst)
    pub fn setBreakOnBinarySubtraction(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java String brkBinSub = ConvertHelper.getString(attributes, "val", "http://schemas.openxmlformats.org/officeDocument/2006/math");
        let brkBinSub: Option<String> = ConvertHelper::getString__String_String(attributes, "val", MATH_NS);
        // java if (null != brkBinSub)
        if brkBinSub.is_some() {
            // java dst.setInt(600, convertBreakBinSub(brkBinSub));
            dst.setInt(600, Self::convertBreakBinSub(brkBinSub.unwrap()));
        }
    }

    // java private static int convertBreakBinSub(String brkBinSub)
    pub fn convertBreakBinSub(brkBinSub: String) -> i32 {
        // java if ("--".equals(brkBinSub))
        if "--" == brkBinSub {
            // java return 0;
            return 0;
        // java } else if ("-+".equals(brkBinSub))
        } else if "-+" == brkBinSub {
            // java return 2;
            return 2;
        // java } else if ("+-".equals(brkBinSub))
        } else if "+-" == brkBinSub {
            // java return 1;
            return 1;
        } else {
            // java IOAssert.fail("it should not reach here!");
            IOAssert::fail_msg("it should not reach here!");
            // java return 0;
            return 0;
        }
    }

    // java private static void setSmallFraction(Attributes attributes, MutablePropertySet dst)
    pub fn setSmallFraction(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Boolean smallFrac = ConvertHelper.toBoolean4Math(attributes);
        let smallFrac: bool = ConvertHelper::toBoolean4Math(attributes);
        // java if (null != smallFrac)
        // rust: 真实 `toBoolean4Math` 返回 `bool`（Java 的 `null` 分支已折叠为 `true`）⇒ 条件恒真。
        {
            // java dst.setBoolean(601, smallFrac);
            dst.setBoolean(601, smallFrac);
        }
    }

    // java private static void setDisplayMathDefault(Attributes attributes, MutablePropertySet dst)
    pub fn setDisplayMathDefault(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Boolean dispDef = ConvertHelper.toBoolean4Math(attributes);
        let dispDef: bool = ConvertHelper::toBoolean4Math(attributes);
        // java if (null != dispDef)
        // rust: 真实 `toBoolean4Math` 返回 `bool`（Java 的 `null` 分支已折叠为 `true`）⇒ 条件恒真。
        {
            // java dst.setBoolean(602, dispDef);
            dst.setBoolean(602, dispDef);
        }
    }

    // java private static void setLeftMargin(Attributes attributes, MutablePropertySet dst)
    pub fn setLeftMargin(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Long lMargin = ShareConvertHelper.getLong4STTwipsMeasure(attributes, "val");
        let lMargin: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "val");
        // java if (null != lMargin)
        if lMargin.is_some() {
            // java dst.setInt(603, lMargin.intValue());
            dst.setInt(603, lMargin.unwrap() as i32);
        }
    }

    // java private static void setRightMargin(Attributes attributes, MutablePropertySet dst)
    pub fn setRightMargin(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Long rMargin = ShareConvertHelper.getLong4STTwipsMeasure(attributes, "val");
        let rMargin: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "val");
        // java if (null != rMargin)
        if rMargin.is_some() {
            // java dst.setInt(604, rMargin.intValue());
            dst.setInt(604, rMargin.unwrap() as i32);
        }
    }

    // java private static void setDefaultJustifition(Attributes attributes, MutablePropertySet dst)
    pub fn setDefaultJustifition(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java String defJc = ConvertHelper.getString(attributes, "val");
        let defJc: Option<String> = ConvertHelper::getString__String(attributes, "val");
        // java if (null != defJc)
        if defJc.is_some() {
            // java dst.setInt(605, convertMathJc(defJc));
            dst.setInt(605, Self::convertMathJc(defJc.unwrap()));
        }
    }

    // java private static int convertMathJc(String defaultJc)
    pub fn convertMathJc(defaultJc: String) -> i32 {
        // java if ("center".equals(defaultJc))
        if "center" == defaultJc {
            // java return 2;
            return 2;
        // java } else if ("centerGroup".equals(defaultJc))
        } else if "centerGroup" == defaultJc {
            // java return 1;
            return 1;
        // java } else if ("left".equals(defaultJc))
        } else if "left" == defaultJc {
            // java return 3;
            return 3;
        // java } else if ("right".equals(defaultJc))
        } else if "right" == defaultJc {
            // java return 4;
            return 4;
        } else {
            // java IOAssert.fail("it should not reach here!");
            IOAssert::fail_msg("it should not reach here!");
            // java return 3;
            return 3;
        }
    }

    // java private static void setPreParagraphSpacing(Attributes attributes, MutablePropertySet dst)
    pub fn setPreParagraphSpacing(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Long preSp = ShareConvertHelper.getLong4STTwipsMeasure(attributes, "val");
        let preSp: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "val");
        // java if (null != preSp)
        if preSp.is_some() {
            // java dst.setInt(606, preSp.intValue());
            dst.setInt(606, preSp.unwrap() as i32);
        }
    }

    // java private static void setPostParagraphSpacing(Attributes attributes, MutablePropertySet dst)
    pub fn setPostParagraphSpacing(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Long postSp = ShareConvertHelper.getLong4STTwipsMeasure(attributes, "val");
        let postSp: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "val");
        // java if (null != postSp)
        if postSp.is_some() {
            // java dst.setInt(607, postSp.intValue());
            dst.setInt(607, postSp.unwrap() as i32);
        }
    }

    // java private static void setInterEquationSpacing(Attributes attributes, MutablePropertySet dst)
    pub fn setInterEquationSpacing(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Long interSp = ShareConvertHelper.getLong4STTwipsMeasure(attributes, "val");
        let interSp: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "val");
        // java if (null != interSp)
        if interSp.is_some() {
            // java dst.setInt(608, interSp.intValue());
            dst.setInt(608, interSp.unwrap() as i32);
        }
    }

    // java private static void setIntraEquationSpacing(Attributes attributes, MutablePropertySet dst)
    pub fn setIntraEquationSpacing(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Long intraSp = ShareConvertHelper.getLong4STTwipsMeasure(attributes, "val");
        let intraSp: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "val");
        // java if (null != intraSp)
        if intraSp.is_some() {
            // java dst.setInt(609, intraSp.intValue());
            dst.setInt(609, intraSp.unwrap() as i32);
        }
    }

    // java private static void setWrapIndent(Attributes attributes, MutablePropertySet dst)
    pub fn setWrapIndent(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Long wrapIndent = ShareConvertHelper.getLong4STTwipsMeasure(attributes, "val");
        let wrapIndent: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "val");
        // java if (null != wrapIndent)
        if wrapIndent.is_some() {
            // java dst.setInt(610, wrapIndent.intValue());
            dst.setInt(610, wrapIndent.unwrap() as i32);
        }
    }

    // java private static void setWrapRight(Attributes attributes, MutablePropertySet dst)
    pub fn setWrapRight(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Boolean wrapRight = ConvertHelper.toBoolean4Math(attributes);
        let wrapRight: bool = ConvertHelper::toBoolean4Math(attributes);
        // java if (null != wrapRight)
        {
            // java dst.setBoolean(611, wrapRight);
            dst.setBoolean(611, wrapRight);
        }
    }

    // java private static void setIntegralLimitLocations(Attributes attributes, MutablePropertySet dst)
    pub fn setIntegralLimitLocations(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java String intLim = ConvertHelper.getString(attributes, "val", "http://schemas.openxmlformats.org/officeDocument/2006/math");
        let intLim: Option<String> = ConvertHelper::getString__String_String(attributes, "val", MATH_NS);
        // java if (null != intLim)
        if intLim.is_some() {
            // java dst.setInt(612, convertLimLoc4IntLim(intLim));
            dst.setInt(612, Self::convertLimLoc4IntLim(intLim.unwrap()));
        }
    }

    // java private static void setNaryLimitLocations(Attributes attributes, MutablePropertySet dst)
    pub fn setNaryLimitLocations(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java String naryLim = ConvertHelper.getString(attributes, "val", "http://schemas.openxmlformats.org/officeDocument/2006/math");
        let naryLim: Option<String> = ConvertHelper::getString__String_String(attributes, "val", MATH_NS);
        // java if (null != naryLim)
        if naryLim.is_some() {
            // java dst.setInt(613, convertLimLoc4NaryLim(naryLim));
            dst.setInt(613, Self::convertLimLoc4NaryLim(naryLim.unwrap()));
        }
    }

    // java private static int convertLimLoc4IntLim(String limLoc)
    pub fn convertLimLoc4IntLim(limLoc: String) -> i32 {
        // java if ("subSup".equals(limLoc))
        if "subSup" == limLoc {
            // java return 0;
            return 0;
        // java } else if ("undOvr".equals(limLoc))
        } else if "undOvr" == limLoc {
            // java return 1;
            return 1;
        } else {
            // java IOAssert.fail("it should not reach here!");
            IOAssert::fail_msg("it should not reach here!");
            // java return 0;
            return 0;
        }
    }

    // java private static int convertLimLoc4NaryLim(String limLoc)
    pub fn convertLimLoc4NaryLim(limLoc: String) -> i32 {
        // java if ("subSup".equals(limLoc))
        if "subSup" == limLoc {
            // java return 0;
            return 0;
        // java } else if ("undOvr".equals(limLoc))
        } else if "undOvr" == limLoc {
            // java return 1;
            return 1;
        } else {
            // java IOAssert.fail("it should not reach here!");
            IOAssert::fail_msg("it should not reach here!");
            // java return 1;
            return 1;
        }
    }


    // java private static void setBoolean(Attributes attributes, int propId, MutablePropertySet dst, boolean defaultBoolean)
    pub fn setBoolean(attributes: &dyn Attributes, propId: i32, dst: &mut MutablePropertySet, defaultBoolean: bool) {
        // java dst.setBoolean(propId, ConvertHelper.toBoolean4NotNull(attributes, defaultBoolean));
        dst.setBoolean(
            propId,
            ConvertHelper::toBoolean4NotNull__Boolean(attributes, Some(defaultBoolean)).unwrap_or(defaultBoolean),
        );
    }
    // java private static void setBoolean(Attributes attributes, int propId, MutablePropertySet dst)
    pub fn setBoolean_1(attributes: &dyn Attributes, propId: i32, dst: &mut MutablePropertySet) {
        // java setBoolean(attributes, propId, dst, Boolean.TRUE);
        Self::setBoolean(attributes, propId, dst, true);
    }
    // java private static void setAttachedTemplate(Attributes attributes, MutablePropertySet dst, POIXMLDocumentPart settingsPart)
    pub fn setAttachedTemplate(attributes: &dyn Attributes, dst: &mut MutablePropertySet, settingsPart: Option<*const XWPFSettings>) {
        // java String attrId = "id";
        // java String id = ConvertHelper.getString(attributes, "id", OpenXmlNamespaceHelper.RELATIONSHIPS);
        let relationships_ns = io_wps_io::cn::wps::io::opc::OpenXmlNamespaceHelper::OpenXmlNamespaceHelper::get_relationships();
        let id = ConvertHelper::getString__String_String(attributes, "id", &relationships_ns);
        // java if (null != id)
        if let Some(id) = id {
            // java if (null != settingsPart)
            if let Some(settings_part) = settingsPart {
                // java PackageRelationship packageRelationship = settingsPart.getRelationShipById(id);
                let relationship = unsafe { &*settings_part }.getRelationShipById(&id);
                // java if (null != packageRelationship)
                if let Some(relationship) = relationship {
                    // java String attachedTemplate = packageRelationship.getTargetURI().toString();
                    let attached_template = relationship.getTargetURI().toString();
                    // java if (null != attachedTemplate)
                    // java String start = "file:///";
                    // java if (attachedTemplate.startsWith("file:///"))
                    let attached_template = attached_template
                        .strip_prefix("file:///")
                        .unwrap_or(&attached_template);
                    // java attachedTemplate = attachedTemplate.substring("file:///".length());
                    // java dst.setString(421, attachedTemplate);
                    dst.setString(421, attached_template.to_string());
                }
            }
        }
    }
    // java private static void setStylePaneFormatFilter(Attributes attributes, MutablePropertySet dst)
    pub fn setStylePaneFormatFilter(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java StylePaneFormatFilterImporter.setStylePaneFormatFilter(attributes, dst);
        StylePaneFormatFilterImporter::setStylePaneFormatFilter(attributes, dst);
    }
    // java private static void crrectStylePaneFormatFilter(MutablePropertySet dst)
    pub fn crrectStylePaneFormatFilter(dst: &mut MutablePropertySet) {
        // java StylePaneFormatFilterImporter.crrectStylePaneFormatFilter(dst);
        StylePaneFormatFilterImporter::crrectStylePaneFormatFilter(dst);
    }
    // java private static void crrectStylePaneSortMethod(MutablePropertySet dst)
    pub fn crrectStylePaneSortMethod(dst: &mut MutablePropertySet) {
        // java Object stylePaneFormatFilter = dst.getValue(438);
        let stylePaneFormatFilter = dst.getValue(438);
        // java if (null == stylePaneFormatFilter)
        if stylePaneFormatFilter.is_none() {
            // java dst.setInt(438, 1);
            dst.setInt(438, 1);
        }
    }
    // java private static void setTrackRevisions(Attributes attributes, TextDocument textDocument)
    pub fn setTrackRevisions(attributes: &dyn Attributes, textDocument: &mut TextDocument) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION Java 原文（逐语句保留）：
        // java Boolean trackRevisions = ConvertHelper.toBoolean4NotNull(attributes);
        // java textDocument.setTrackChange(trackRevisions);
        */
        // rust: ★M-STAGE1 阻塞记录 —— `TextDocument.setTrackChange` 的
        //   `VersionState.isProVersion()` 分支依赖未登记的 `VersionState`
        //   （登记会引入 55 个错误，见 TextDocument.rs 内的阻塞记录）⇒ 一并 deferred。
        //   ★语料事实：全部 4 个正语料的 `word/settings.xml` **均无** `<w:trackChanges>`，
        //   本分支在现有语料上不可达。
    }
    // java private void setDocumentProtection(Attributes attributes, MutablePropertySet dst)
    pub fn setDocumentProtection(&self, attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
      // java String edit = ConvertHelper.getString(attributes, "edit");
      let edit = ConvertHelper::getString__String(attributes, "edit");
      // java int coreEdit = this.convertDocProtect(edit);
      let coreEdit = self.convertDocProtect(edit.clone());
      // java if (null != edit) { dst.setInt(629, coreEdit); dst.setBoolean(639, true); }
      if edit.is_some() { dst.setInt(629, coreEdit); dst.setBoolean(639, true); }
      // java Boolean formatting = ConvertHelper.getBoolean(attributes, "formatting");
      let formatting = ConvertHelper::getBoolean(attributes, "formatting");
      // java if (null != formatting) dst.setBoolean(635, formatting);
      if let Some(formatting) = formatting { dst.setBoolean(635, formatting); }
      // java Boolean enforcement = ConvertHelper.getBoolean(attributes, "enforcement");
      let enforcement = ConvertHelper::getBoolean(attributes, "enforcement");
      // java if (null != enforcement && enforcement)
      if enforcement == Some(true) {
        // java if (null != edit) convertEnforementDocProtect(coreEdit, dst);
        if edit.is_some() { Self::convertEnforementDocProtect(coreEdit, dst); }
        // java if (null != formatting && formatting) dst.setBoolean(637, true);
        if formatting == Some(true) { dst.setBoolean(637, true); }
      }
      // java convertPassword(attributes, dst);
      Self::convertPassword(attributes, dst);
      // java convertTransitionalPassword(attributes, dst);
      Self::convertTransitionalPassword(attributes, dst);
      // java this.read_protection(attributes);
      self.read_protection(attributes);
      /* Java 原文（逐语句保留）：
      // java String attrEdit = "edit";
      // java String attrFormatting = "formatting";
      // java String attrEnforcement = "enforcement";
      // java String edit = ConvertHelper.getString(attributes, "edit");
      // java int coreEdit = this.convertDocProtect(edit);
      // java if (null != edit) {
      // java dst.setInt(629, coreEdit);
      // java dst.setBoolean(639, true);
      // java }
      // java Boolean formatting = ConvertHelper.getBoolean(attributes, "formatting");
      // java if (null != formatting) {
      // java dst.setBoolean(635, formatting);
      // java }
      // java Boolean enforcement = ConvertHelper.getBoolean(attributes, "enforcement");
      // java if (null != enforcement && enforcement) {
      // java if (null != edit) {
      // java convertEnforementDocProtect(coreEdit, dst);
      // java }
      // java if (null != formatting && formatting) {
      // java dst.setBoolean(637, true);
      // java }
      // java }
      // java convertPassword(attributes, dst);
      // java convertTransitionalPassword(attributes, dst);
      // java this.read_protection(attributes);
      */
    }
    // java private void read_protection(Attributes attributes)
    pub fn read_protection(&self, attributes: &dyn Attributes) {
      // java DocumentProtectionIO io = new DocumentProtectionIO(this.mTextDocument.getDocumentProtection(true));
      let document = unsafe { &mut *self.mTextDocument.expect("mTextDocument") };
      let protection = document.getDocumentProtection_mut(true).expect("document protection");
      let mut io = DocumentProtectionIO::from_ref(protection);
      // java io.setProtect(true);
      io.setProtect(true);
      // java Boolean enforcement = ConvertHelper.getBoolean(attributes, "enforcement");
      let enforcement = ConvertHelper::getBoolean(attributes, "enforcement");
      // java if (enforcement != null && enforcement) io.setEnforced(true);
      if enforcement == Some(true) { io.setEnforced(true); }
      // java String edit = ConvertHelper.getString(attributes, "edit");
      let edit = ConvertHelper::getString__String(attributes, "edit");
      // java int coreEdit = this.convertDocProtect(edit);
      let coreEdit = self.convertDocProtect(edit);
      // java switch (coreEdit)
      let protection_type = match coreEdit {
        0 => ProtectionType::TRACKEDCHANGES,
        1 => ProtectionType::COMMENTS,
        2 => ProtectionType::FORMS,
        3 => ProtectionType::READONLY,
        _ => ProtectionType::NONE,
      };
      // java io.setProtectionType(...);
      io.setProtectionType(protection_type);
      // java Boolean formatting = ConvertHelper.getBoolean(attributes, "formatting");
      let formatting = ConvertHelper::getBoolean(attributes, "formatting");
      // java if (formatting != null && formatting) io.setStyleLocked(true);
      if formatting == Some(true) { io.setStyleLocked(true); }
      // java if (enforcement != null && enforcement) io.setStyleLockEnforced(true);
      if enforcement == Some(true) { io.setStyleLockEnforced(true); }
      // java this.read_password(attributes, io);
      self.read_password(attributes, &mut io);
      /* Java 原文（逐语句保留）：
      // java DocumentProtectionIO io = new DocumentProtectionIO(this.mTextDocument.getDocumentProtection(true));
      // java io.setProtect(true);
      // java Boolean enforcement = ConvertHelper.getBoolean(attributes, "enforcement");
      // java if (enforcement != null && enforcement) {
      // java io.setEnforced(true);
      // java }
      // java String edit = ConvertHelper.getString(attributes, "edit");
      // java int coreEdit = this.convertDocProtect(edit);
      // java switch (coreEdit) {
      // java case 0:
      // java io.setProtectionType(ProtectionType.TRACKEDCHANGES);
      // java break;
      // java case 1:
      // java io.setProtectionType(ProtectionType.COMMENTS);
      // java break;
      // java case 2:
      // java io.setProtectionType(ProtectionType.FORMS);
      // java break;
      // java case 3:
      // java io.setProtectionType(ProtectionType.READONLY);
      // java break;
      // java case 4:
      // java case 5:
      // java case 6:
      // java default:
      // java io.setProtectionType(ProtectionType.NONE);
      // java break;
      // java case 7:
      // java io.setProtectionType(ProtectionType.NONE);
      // java }
      // java Boolean formatting = ConvertHelper.getBoolean(attributes, "formatting");
      // java if (formatting != null && formatting) {
      // java io.setStyleLocked(true);
      // java }
      // java if (enforcement != null && enforcement) {
      // java io.setStyleLockEnforced(true);
      // java }
      // java this.read_password(attributes, io);
      */
    }
    // java private void read_password(Attributes attributes, DocumentProtectionIO io)
    pub fn read_password(&self, attributes: &dyn Attributes, io: &mut DocumentProtectionIO) {
      // java String hash = ConvertHelper.getString(attributes, "hash");
      let hash = ConvertHelper::getString__String(attributes, "hash");
      // java String salt = ConvertHelper.getString(attributes, "salt");
      let salt = ConvertHelper::getString__String(attributes, "salt");
      // java Long pSid = ConvertHelper.getLong(attributes, "cryptAlgorithmSid");
      let pSid = ConvertHelper::getLong(attributes, "cryptAlgorithmSid");
      // java CryptProvType providerType = this.convertProvType(...);
      let providerType = self.convertProvType(ConvertHelper::getString__String(attributes, "cryptProviderType"));
      // java Long pSpinCount = ConvertHelper.getLong(attributes, "cryptSpinCount");
      let pSpinCount = ConvertHelper::getLong(attributes, "cryptSpinCount");
      // java if (pSid != null && providerType != null && pSpinCount != null)
      if let (Some(sid), Some(spin)) = (pSid, pSpinCount) {
        // java if (sid == 14L || sid == 4L)
        if sid == 14 || sid == 4 {
          // java if (providerType == CryptProvType.RSAAES || providerType == CryptProvType.RSAFULL)
          if providerType == CryptProvType::RSAAES || providerType == CryptProvType::RSAFULL {
            // java String hashType = "sha1"; if (sid == 14L && providerType == CryptProvType.RSAAES) hashType = "sha512";
            let hashType = if sid == 14 && providerType == CryptProvType::RSAAES { "sha512" } else { "sha1" };
            // java ProtectionPassword password = new ProtectionPassword(0, hash, salt, hashType, (int)pSpinCount.longValue());
            let password = ProtectionPassword::ProtectionPassword__int_string_string_string_int(0, hash, salt, Some(hashType.to_string()), spin as i32);
            // java io.setPassword(password);
            io.setPassword(password);
          }
        }
      }
      /* Java 原文（逐语句保留）：
      // java String hash = ConvertHelper.getString(attributes, "hash");
      // java String salt = ConvertHelper.getString(attributes, "salt");
      // java Long pSid = ConvertHelper.getLong(attributes, "cryptAlgorithmSid");
      // java CryptProvType providerType = this.convertProvType(ConvertHelper.getString(attributes, "cryptProviderType"));
      // java Long pSpinCount = ConvertHelper.getLong(attributes, "cryptSpinCount");
      // java if (pSid != null && providerType != null && pSpinCount != null) {
      // java long sid = pSid;
      // java if (sid == 14L || sid == 4L) {
      // java if (providerType == CryptProvType.RSAAES || providerType == CryptProvType.RSAFULL) {
      // java String hashType = "sha1";
      // java if (sid == 14L && providerType == CryptProvType.RSAAES) {
      // java hashType = "sha512";
      // java }
      // java ProtectionPassword password = new ProtectionPassword(0, hash, salt, hashType, (int)pSpinCount.longValue());
      // java io.setPassword(password);
      // java }
      // java }
      // java }
      */
    }
    // java private CryptProvType convertProvType(String string)
    pub fn convertProvType(&self, string: Option<String>) -> CryptProvType {
      // java if ("rsaAES".equalsIgnoreCase(string)) return CryptProvType.RSAAES;
      if string.as_deref().is_some_and(|s| s.eq_ignore_ascii_case("rsaAES")) { return CryptProvType::RSAAES; }
      // java return "rsaFull".equalsIgnoreCase(string) ? CryptProvType.RSAFULL : CryptProvType.CUSTOM;
      if string.as_deref().is_some_and(|s| s.eq_ignore_ascii_case("rsaFull")) { CryptProvType::RSAFULL } else { CryptProvType::CUSTOM }
      /* Java 原文（逐语句保留）：
      // java if ("rsaAES".equalsIgnoreCase(string)) {
      // java return CryptProvType.RSAAES;
      // java } else {
      // java return "rsaFull".equalsIgnoreCase(string) ? CryptProvType.RSAFULL : CryptProvType.CUSTOM;
      // java }
      */
    }
    // java private static void convertTransitionalPassword(Attributes attributes, MutablePropertySet dst)
    pub fn convertTransitionalPassword(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
      // Java: convert each present transitional password attribute into its document property.
      if let Some(v) = ConvertHelper::getString__String(attributes, "cryptProviderType") { dst.setInt(617, Self::convertCryptProvType(v)); }
      if let Some(v) = ConvertHelper::getString__String(attributes, "cryptAlgorithmClass") { dst.setInt(618, Self::convertAlgClassType(v)); }
      if let Some(v) = ConvertHelper::getString__String(attributes, "cryptAlgorithmType") { dst.setInt(619, Self::convertAlgType(v)); }
      if let Some(v) = ShareConvertHelper::getInteger4STDecimalNumber__attributes_string(attributes, "cryptAlgorithmSid") { dst.setInt(620, v); }
      if let Some(v) = ShareConvertHelper::getInteger4STDecimalNumber__attributes_string(attributes, "cryptSpinCount") { dst.setInt(621, v); }
      if let Some(v) = ConvertHelper::getString__String(attributes, "cryptProvider") { dst.setString(622, v); }
      if let Some(v) = ShareConvertHelper::getLong4STLongHexNumber(attributes, "algIdExt") { dst.setInt(623, v as i32); }
      if let Some(v) = ConvertHelper::getString__String(attributes, "algIdExtSource") { dst.setString(624, v); }
      if let Some(v) = ShareConvertHelper::getLong4STLongHexNumber(attributes, "cryptProviderTypeExt") { dst.setInt(625, v as i32); }
      if let Some(v) = ConvertHelper::getString__String(attributes, "cryptProviderTypeExtSource") { dst.setString(626, v); }
      if let Some(v) = ConvertHelper::getString__String(attributes, "hash") { dst.setString(627, v); }
      if let Some(v) = ConvertHelper::getString__String(attributes, "salt") { dst.setString(628, v); }
      /* Java 原文（逐语句保留）：
      // java String attrCryptProviderType = "cryptProviderType";
      // java String attrCryptAlgorithmClass = "cryptAlgorithmClass";
      // java String attrCryptAlgorithmType = "cryptAlgorithmType";
      // java String attrCryptAlgorithmSid = "cryptAlgorithmSid";
      // java String attrCryptSpinCount = "cryptSpinCount";
      // java String attrCryptProvider = "cryptProvider";
      // java String attrAlgIdExt = "algIdExt";
      // java String attrAlgIdExtSource = "algIdExtSource";
      // java String attrCryptProviderTypeExt = "cryptProviderTypeExt";
      // java String attrCryptProviderTypeExtSource = "cryptProviderTypeExtSource";
      // java String attrHash = "hash";
      // java String attrSalt = "salt";
      // java String cryptProvType = ConvertHelper.getString(attributes, "cryptProviderType");
      // java if (null != cryptProvType) {
      // java dst.setInt(617, convertCryptProvType(cryptProvType));
      // java }
      // java String cryptAlgorithmClass = ConvertHelper.getString(attributes, "cryptAlgorithmClass");
      // java if (null != cryptAlgorithmClass) {
      // java dst.setInt(618, convertAlgClassType(cryptAlgorithmClass));
      // java }
      // java String cryptAlgorithmType = ConvertHelper.getString(attributes, "cryptAlgorithmType");
      // java if (null != cryptAlgorithmType) {
      // java dst.setInt(619, convertAlgType(cryptAlgorithmType));
      // java }
      // java Integer sid = ShareConvertHelper.getInteger4STDecimalNumber(attributes, "cryptAlgorithmSid");
      // java if (null != sid) {
      // java dst.setInt(620, sid);
      // java }
      // java Integer count = ShareConvertHelper.getInteger4STDecimalNumber(attributes, "cryptSpinCount");
      // java if (null != count) {
      // java dst.setInt(621, count);
      // java }
      // java String provider = ConvertHelper.getString(attributes, "cryptProvider");
      // java if (null != provider) {
      // java dst.setString(622, provider);
      // java }
      // java Long algIdExt = ShareConvertHelper.getLong4STLongHexNumber(attributes, "algIdExt");
      // java if (null != algIdExt) {
      // java dst.setInt(623, algIdExt.intValue());
      // java }
      // java String algIdExtSource = ConvertHelper.getString(attributes, "algIdExtSource");
      // java if (null != algIdExtSource) {
      // java dst.setString(624, algIdExtSource);
      // java }
      // java Long typeExt = ShareConvertHelper.getLong4STLongHexNumber(attributes, "cryptProviderTypeExt");
      // java if (null != typeExt) {
      // java dst.setInt(625, typeExt.intValue());
      // java }
      // java String extSourse = ConvertHelper.getString(attributes, "cryptProviderTypeExtSource");
      // java if (null != extSourse) {
      // java dst.setString(626, extSourse);
      // java }
      // java String hash = ConvertHelper.getString(attributes, "hash");
      // java if (null != hash) {
      // java dst.setString(627, hash);
      // java }
      // java String salt = ConvertHelper.getString(attributes, "salt");
      // java if (null != salt) {
      // java dst.setString(628, salt);
      // java }
      */
    }
    // java private static int convertAlgType(String algTypeType)
    pub fn convertAlgType(algTypeType: String) -> i32 {
      // Java: "custom" => 1; "typeAny" => 0; otherwise IOAssert.fail.
      match algTypeType.as_str() { "custom" => 1, "typeAny" => 0, _ => panic!("invalid cryptAlgorithmType") }
      /* Java 原文（逐语句保留）：
      // java int coreAlgTypeType = 1;
      // java if ("custom".equals(algTypeType)) {
      // java coreAlgTypeType = 1;
      // java } else if ("typeAny".equals(algTypeType)) {
      // java coreAlgTypeType = 0;
      // java } else {
      // java IOAssert.fail("it should not reach here!");
      // java }
      // java return coreAlgTypeType;
      */
    }
    // java private static int convertAlgClassType(String algClassType)
    pub fn convertAlgClassType(algClassType: String) -> i32 {
      // Java: "custom" => 1; "hash" => 0; otherwise IOAssert.fail.
      match algClassType.as_str() { "custom" => 1, "hash" => 0, _ => panic!("invalid cryptAlgorithmClass") }
      /* Java 原文（逐语句保留）：
      // java int coreAlgClass = 1;
      // java if ("custom".equals(algClassType)) {
      // java coreAlgClass = 1;
      // java } else if ("hash".equals(algClassType)) {
      // java coreAlgClass = 0;
      // java } else {
      // java IOAssert.fail("it should not reach here!");
      // java }
      // java return coreAlgClass;
      */
    }
    // java private static int convertCryptProvType(String cryptProvType)
    pub fn convertCryptProvType(cryptProvType: String) -> i32 {
      // Java: "custom" => 2; "rsaAES" => 0; "rsaFull" => 1.
      match cryptProvType.as_str() { "custom" => 2, "rsaAES" => 0, "rsaFull" => 1, _ => panic!("invalid cryptProviderType") }
      /* Java 原文（逐语句保留）：
      // java int coreType = 2;
      // java if ("custom".equals(cryptProvType)) {
      // java coreType = 2;
      // java } else if ("rsaAES".equals(cryptProvType)) {
      // java coreType = 0;
      // java } else if ("rsaFull".equals(cryptProvType)) {
      // java coreType = 1;
      // java } else {
      // java IOAssert.fail("it should not reach here!");
      // java }
      // java return coreType;
      */
    }
    // java private static void convertPassword(Attributes attributes, MutablePropertySet dst)
    pub fn convertPassword(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
      // Java: algorithmName/hashValue/saltValue/spinCount map to properties 638/614/615/616.
      if let Some(v) = ConvertHelper::getString__String(attributes, "algorithmName") { dst.setString(638, v); }
      if let Some(v) = ConvertHelper::getString__String(attributes, "hashValue") { dst.setString(614, v); }
      if let Some(v) = ConvertHelper::getString__String(attributes, "saltValue") { dst.setString(615, v); }
      if let Some(v) = ShareConvertHelper::getInteger4STDecimalNumber__attributes_string(attributes, "spinCount") { dst.setInt(616, v); }
      /* Java 原文（逐语句保留）：
      // java String algorithmName = "algorithmName";
      // java String hashValue = "hashValue";
      // java String saltValue = "saltValue";
      // java String spinCount = "spinCount";
      // java String name = ConvertHelper.getString(attributes, "algorithmName");
      // java if (null != name) {
      // java dst.setString(638, name);
      // java }
      // java String hash = ConvertHelper.getString(attributes, "hashValue");
      // java if (null != hash) {
      // java dst.setString(614, hash);
      // java }
      // java String salt = ConvertHelper.getString(attributes, "saltValue");
      // java if (null != salt) {
      // java dst.setString(615, salt);
      // java }
      // java Integer spin = ShareConvertHelper.getInteger4STDecimalNumber(attributes, "spinCount");
      // java if (null != spin) {
      // java dst.setInt(616, spin);
      // java }
      */
    }
    // java private static void convertEnforementDocProtect(int coreEdit, MutablePropertySet dst)
    pub fn convertEnforementDocProtect(coreEdit: i32, dst: &mut MutablePropertySet) {
      // Java: switch (coreEdit) sets the enforced property for the selected edit mode.
      match coreEdit { 0 => dst.setBoolean(630, true), 1 | 3 => dst.setBoolean(632, true), 2 => dst.setBoolean(631, true), 7 => {}, _ => panic!("invalid document protection edit") }
      /* Java 原文（逐语句保留）：
      // java switch (coreEdit) {
      // java case 0:
      // java dst.setBoolean(630, true);
      // java break;
      // java case 1:
      // java dst.setBoolean(632, true);
      // java break;
      // java case 2:
      // java dst.setBoolean(631, true);
      // java break;
      // java case 3:
      // java dst.setBoolean(632, true);
      // java break;
      // java case 4:
      // java case 5:
      // java case 6:
      // java default:
      // java IOAssert.fail("it should not reach here!");
      // java case 7:
      // java }
      */
    }
    // java private int convertDocProtect(String editType)
    pub fn convertDocProtect(&self, editType: Option<String>) -> i32 {
      // Java: null editType returns 7 (none).
      let Some(editType) = editType else { return 7; };
      // Java: if (this.isXml03()) editType = Xml03Utils.replaceHyphenMinusToUpperCase(editType);
      let editType = if self.base.isXml03() { crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Utils::Xml03Utils::replaceHyphenMinusToUpperCase(Some(&editType)).unwrap_or(editType) } else { editType };
      // Java: map OOXML protection edit values to core values.
      match editType.as_str() { "none" => 7, "comments" => 1, "forms" => 2, "readOnly" => 3, "trackedChanges" => 0, _ => panic!("invalid document protection edit") }
      /* Java 原文（逐语句保留）：
      // java int coreEditType = 7;
      // java if (null == editType) {
      // java return 7;
      // java } else {
      // java if (this.isXml03()) {
      // java editType = Xml03Utils.replaceHyphenMinusToUpperCase(editType);
      // java }
      // java if ("none".equals(editType)) {
      // java coreEditType = 7;
      // java } else if ("comments".equals(editType)) {
      // java coreEditType = 1;
      // java } else if ("forms".equals(editType)) {
      // java coreEditType = 2;
      // java } else if ("readOnly".equals(editType)) {
      // java coreEditType = 3;
      // java } else if ("trackedChanges".equals(editType)) {
      // java coreEditType = 0;
      // java } else {
      // java IOAssert.fail("it should not reach here!");
      // java }
      // java return coreEditType;
      // java }
      */
    }
    // java private void setCharacterSpacingControl(Attributes attributes, MutablePropertySet dst)
    pub fn setCharacterSpacingControl(&mut self, attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String charSpacingControl = ConvertHelper.getString(attributes);
        let mut charSpacingControl: Option<String> = ConvertHelper::getString(attributes);
        // java int compressLevel = 0;
        let mut compressLevel: i32 = 0;
        // java if (this.isXml03())
        if self.base.isXml03() {
            // java charSpacingControl = Xml03Utils.firstCharToLowerCase(charSpacingControl);
            charSpacingControl = crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Utils::Xml03Utils::firstCharToLowerCase(
                charSpacingControl.as_deref(),
            );
        }
        // java if ("doNotCompress".equals(charSpacingControl))
        if Some("doNotCompress".to_string()) == charSpacingControl {
            // java compressLevel = 0;
            compressLevel = 0;
        // java } else if ("compressPunctuation".equals(charSpacingControl))
        } else if Some("compressPunctuation".to_string()) == charSpacingControl {
            // java compressLevel = 1;
            compressLevel = 1;
        // java } else if ("compressPunctuationAndJapaneseKana".equals(charSpacingControl))
        } else if Some("compressPunctuationAndJapaneseKana".to_string()) == charSpacingControl {
            // java compressLevel = 2;
            compressLevel = 2;
        } else {
            // java IOAssert.fail("It should not reach to here.");
            IOAssert::fail_msg("It should not reach to here.");
        }
        // java dst.setInt(382, compressLevel);
        dst.setInt(382, compressLevel);
    }
    // java private static void setClrSchemeMapping(Attributes attributes, MutablePropertySet dst)
    pub fn setClrSchemeMapping(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java ClrSchemeMappingImporter.setClrSchemeMapping(attributes, dst);
        ClrSchemeMappingImporter::setClrSchemeMapping(attributes, dst);
    }
    // java private void setCaption(Attributes attributes, MutablePropertySet dst)
    pub fn setCaption(&mut self, attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java this.getCaptionsImporter().setCaption(attributes, dst);
        self.getCaptionsImporter().setCaption(attributes, dst);
    }
    // java private void setAutoCaption(Attributes attributes, MutablePropertySet dst)
    pub fn setAutoCaption(&mut self, attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java this.getCaptionsImporter().setAutoCaption(attributes, dst);
        self.getCaptionsImporter().setAutoCaption(attributes, dst);
    }
    // java private static void setSmartTagType(Attributes attributes, MutablePropertySet dst)
    pub fn setSmartTagType(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java SmartTagTypeImporter.setSmartTagType(attributes, dst);
        SmartTagTypeImporter::setSmartTagType(attributes, dst);
    }
    // java private static void correctSmartTagType(MutablePropertySet dst)
    pub fn correctSmartTagType(dst: &mut MutablePropertySet) {
        // java SmartTagTypeImporter.correctSmartTagType(dst);
        SmartTagTypeImporter::correctSmartTagType(dst);
    }
    // java private void setEndNotePr()
    pub fn setEndNotePr(&mut self) {
        // java this.mCurParentId = -1917338065;
        self.mCurParentId = -1917338065;
    }
    // java private void setPos(Attributes attributes, MutablePropertySet dst)
    pub fn setPos(&mut self, attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java if (-1917338065 == this.mCurParentId)
        if -1917338065 == self.mCurParentId {
            // java setEndNotePrPos(attributes, dst);
            Self::setEndNotePrPos(attributes, dst);
            // java this.mCurParentId = 1434631203;
            self.mCurParentId = 1434631203;
        }
    }
    // java private static void setEndNotePrPos(Attributes attributes, MutablePropertySet dst)
    pub fn setEndNotePrPos(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String pos = ConvertHelper.getString(attributes);
        let pos: Option<String> = ConvertHelper::getString(attributes);
        // java if ("docEnd".equals(pos))
        if Some("docEnd".to_string()) == pos {
            // java dst.setValue(641, 3);
            dst.setValue(641, Some(std::sync::Arc::new(3)));
        // java } else if ("sectEnd".equals(pos))
        } else if Some("sectEnd".to_string()) == pos {
            // java dst.setValue(641, 0);
            dst.setValue(641, Some(std::sync::Arc::new(0)));
        } else {
            // java IOAssert.fail("it should not reach here");
            IOAssert::fail_msg("it should not reach here");
        }
    }
    // java private void setNoLineBreaksAfter(Attributes attributes)
    pub fn setNoLineBreaksAfter(&mut self, attributes: &dyn Attributes) {
        // java this.getNoLineBreaksImporter().setNoLineBreaksAfter(attributes);
        self.getNoLineBreaksImporter().setNoLineBreaksAfter(attributes);
    }
    // java private void setNoLineBreaksBefore(Attributes attributes)
    pub fn setNoLineBreaksBefore(&mut self, attributes: &dyn Attributes) {
        // java this.getNoLineBreaksImporter().setNoLineBreaksBefore(attributes);
        self.getNoLineBreaksImporter().setNoLineBreaksBefore(attributes);
    }
    // java private void correctNoLineBreaks(MutablePropertySet dst)
    pub fn correctNoLineBreaks(&mut self, dst: &mut MutablePropertySet) {
        // java this.getNoLineBreaksImporter().correctNoLineBreaks(dst);
        self.getNoLineBreaksImporter().correctNoLineBreaks(dst);
    }
    // java private CapationsImporter getCaptionsImporter()
    pub fn getCaptionsImporter(&mut self) -> &mut CapationsImporter {
        // java if (null == this.mCaptionsImporter)
        if self.mCaptionsImporter.is_none() {
            // java this.mCaptionsImporter = new CapationsImporter(this.mode(), this.xml07Context(), this.xml03Context());
            self.mCaptionsImporter = Some(CapationsImporter::CapationsImporter(
                self.base.mode(),
                self.base.xml07Context().cloned(),
                self.base.xml03Context().cloned(),
            ));
        }
        // java return this.mCaptionsImporter;
        return self.mCaptionsImporter.as_mut().unwrap();
    }
    // java private NoLineBreaksImporter getNoLineBreaksImporter()
    pub fn getNoLineBreaksImporter(&mut self) -> &mut NoLineBreaksImporter {
        // java if (null == this.mNoLineBreaksImporter)
        if self.mNoLineBreaksImporter.is_none() {
            // java this.mNoLineBreaksImporter = new NoLineBreaksImporter(this.mode(), this.xml07Context(), this.xml03Context());
            self.mNoLineBreaksImporter = Some(NoLineBreaksImporter::NoLineBreaksImporter(
                self.base.mode(),
                self.base.xml07Context().cloned(),
                self.base.xml03Context().cloned(),
            ));
        }
        // java return this.mNoLineBreaksImporter;
        return self.mNoLineBreaksImporter.as_mut().unwrap();
    }

    // java private static void setDefaultTabStop(Attributes attributes, MutablePropertySet dst)
    pub fn setDefaultTabStop(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Long defTabStop = ShareConvertHelper.getLong4STTwipsMeasure(attributes);
        let defTabStop: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure(attributes);
        // java if (null != defTabStop)
        if defTabStop.is_some() {
            // java dst.setInt(377, defTabStop.intValue());
            dst.setInt(377, defTabStop.unwrap() as i32);
        }
    }

    // java private static void setHyphenationZone(Attributes attributes, MutablePropertySet dst)
    pub fn setHyphenationZone(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Long hyphenationZone = ShareConvertHelper.getLong4STTwipsMeasure(attributes);
        let hyphenationZone: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure(attributes);
        // java if (null != hyphenationZone)
        if hyphenationZone.is_some() {
            // java dst.setInt(450, hyphenationZone.intValue());
            dst.setInt(450, hyphenationZone.unwrap() as i32);
        }
    }

    // java private static void setDrawingGridHorizontalSpacing(Attributes attributes, MutablePropertySet dst)
    pub fn setDrawingGridHorizontalSpacing(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Long drawingGridHorizontalSpacing = ShareConvertHelper.getLong4STTwipsMeasure(attributes);
        let drawingGridHorizontalSpacing: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure(attributes);
        // java if (null != drawingGridHorizontalSpacing)
        if drawingGridHorizontalSpacing.is_some() {
            // java dst.setInt(459, drawingGridHorizontalSpacing.intValue());
            dst.setInt(459, drawingGridHorizontalSpacing.unwrap() as i32);
        }
    }

    // java private static void setDrawingGridVerticalSpacing(Attributes attributes, MutablePropertySet dst)
    pub fn setDrawingGridVerticalSpacing(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Long vertSpacing = ShareConvertHelper.getLong4STTwipsMeasure(attributes);
        let vertSpacing: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure(attributes);
        // java if (null != vertSpacing)
        if vertSpacing.is_some() {
            // java dst.setInt(460, vertSpacing.intValue());
            dst.setInt(460, vertSpacing.unwrap() as i32);
        }
    }

    // java private static void setDrawingGridHorizontalOrigin(Attributes attributes, MutablePropertySet dst)
    pub fn setDrawingGridHorizontalOrigin(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Long drawingGridHorizontalOrigin = ShareConvertHelper.getLong4STTwipsMeasure(attributes);
        let drawingGridHorizontalOrigin: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure(attributes);
        // java if (null != drawingGridHorizontalOrigin)
        if drawingGridHorizontalOrigin.is_some() {
            // java dst.setInt(464, drawingGridHorizontalOrigin.intValue());
            dst.setInt(464, drawingGridHorizontalOrigin.unwrap() as i32);
        }
    }

    // java private static void setDrawingGridVerticalOrigin(Attributes attributes, MutablePropertySet dst)
    pub fn setDrawingGridVerticalOrigin(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Long drawingGridVerticalOrigin = ShareConvertHelper.getLong4STTwipsMeasure(attributes);
        let drawingGridVerticalOrigin: Option<i64> = ShareConvertHelper::getLong4STTwipsMeasure(attributes);
        // java if (null != drawingGridVerticalOrigin)
        if drawingGridVerticalOrigin.is_some() {
            // java dst.setInt(465, drawingGridVerticalOrigin.intValue());
            dst.setInt(465, drawingGridVerticalOrigin.unwrap() as i32);
        }
    }

    // java private static void setDisplayHorizontalDrawingGridEvery(Attributes attributes, MutablePropertySet dst)
    pub fn setDisplayHorizontalDrawingGridEvery(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Long horiEvery = ShareConvertHelper.getLong4STDecimalNumber(attributes);
        let horiEvery: Option<i64> = ShareConvertHelper::getLong4STDecimalNumber(attributes);
        // java if (null != horiEvery)
        if horiEvery.is_some() {
            // java dst.setInt(394, horiEvery.intValue());
            dst.setInt(394, horiEvery.unwrap() as i32);
        }
    }

    // java private static void setDisplayVerticalDrawingGridEvery(Attributes attributes, MutablePropertySet dst)
    pub fn setDisplayVerticalDrawingGridEvery(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Long vertEvery = ShareConvertHelper.getLong4STDecimalNumber(attributes);
        let vertEvery: Option<i64> = ShareConvertHelper::getLong4STDecimalNumber(attributes);
        // java if (null != vertEvery)
        if vertEvery.is_some() {
            // java dst.setInt(395, vertEvery.intValue());
            dst.setInt(395, vertEvery.unwrap() as i32);
        }
    }

    // java private static void setDocumentType(Attributes attributes, MutablePropertySet dst)
    pub fn setDocumentType(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String docType = ConvertHelper.getString(attributes);
        let docType: Option<String> = ConvertHelper::getString(attributes);
        // java if (null != docType)
        if docType.is_some() {
            // java dst.setString(439, docType);
            dst.setString(439, docType.unwrap());
        }
    }

    // java private static void setClickAndTypeStyle(Attributes attributes, MutablePropertySet dst)
    pub fn setClickAndTypeStyle(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String clickAndTypeStyle = ConvertHelper.getString(attributes);
        let clickAndTypeStyle: Option<String> = ConvertHelper::getString(attributes);
        // java if (null != clickAndTypeStyle)
        if clickAndTypeStyle.is_some() {
            // java dst.setString(454, clickAndTypeStyle);
            dst.setString(454, clickAndTypeStyle.unwrap());
        }
    }

    // java private static void setDefaultTableStyle(Attributes attributes, MutablePropertySet dst)
    pub fn setDefaultTableStyle(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String defaultTableStyle = ConvertHelper.getString(attributes);
        let defaultTableStyle: Option<String> = ConvertHelper::getString(attributes);
        // java if (null != defaultTableStyle)
        if defaultTableStyle.is_some() {
            // java dst.setString(455, defaultTableStyle);
            dst.setString(455, defaultTableStyle.unwrap());
        }
    }

    // java private static void setDecimalSymbol(Attributes attributes, MutablePropertySet dst)
    pub fn setDecimalSymbol(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String decimalSymbol = ConvertHelper.getString(attributes);
        let decimalSymbol: Option<String> = ConvertHelper::getString(attributes);
        // java if (null != decimalSymbol)
        if decimalSymbol.is_some() {
            // java dst.setString(568, decimalSymbol);
            dst.setString(568, decimalSymbol.unwrap());
        }
    }

    // java private static void setListSeparator(Attributes attributes, MutablePropertySet dst)
    pub fn setListSeparator(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String listSeparator = ConvertHelper.getString(attributes);
        let listSeparator: Option<String> = ConvertHelper::getString(attributes);
        // java if (null != listSeparator)
        if listSeparator.is_some() {
            // java dst.setString(569, listSeparator);
            dst.setString(569, listSeparator.unwrap());
        }
    }

    // java private static void setConsecutiveHyphenLimit(Attributes attributes, MutablePropertySet dst)
    pub fn setConsecutiveHyphenLimit(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Integer consecutiveHyphenLimit = ShareConvertHelper.getInteger4STDecimalNumber(attributes);
        let consecutiveHyphenLimit: Option<i32> = ShareConvertHelper::getInteger4STDecimalNumber(attributes);
        // java if (null != consecutiveHyphenLimit)
        if consecutiveHyphenLimit.is_some() {
            // java dst.setInt(449, consecutiveHyphenLimit);
            dst.setInt(449, consecutiveHyphenLimit.unwrap());
        }
    }

    // java private static void setBookFoldPrintingSheets(Attributes attributes, MutablePropertySet dst)
    pub fn setBookFoldPrintingSheets(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Integer bookFoldPrintingSheets = ShareConvertHelper.getInteger4STDecimalNumber(attributes);
        let bookFoldPrintingSheets: Option<i32> = ShareConvertHelper::getInteger4STDecimalNumber(attributes);
        // java if (null != bookFoldPrintingSheets)
        if bookFoldPrintingSheets.is_some() {
            // java dst.setInt(458, bookFoldPrintingSheets);
            dst.setInt(458, bookFoldPrintingSheets.unwrap());
        }
    }

    // java private static void setView(Attributes attributes, MutablePropertySet dst)
    pub fn setView(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String view = ConvertHelper.getString(attributes);
        let view: Option<String> = ConvertHelper::getString(attributes);
        // java int viewType = 0;
        let mut viewType: i32 = 0;
        // java if ("none".equals(view))
        if Some("none".to_string()) == view {
            // java viewType = 0;
            viewType = 0;
        // java } else if ("masterPages".equals(view))
        } else if Some("masterPages".to_string()) == view {
            // java viewType = 3;
            viewType = 3;
        // java } else if ("normal".equals(view))
        } else if Some("normal".to_string()) == view {
            // java viewType = 4;
            viewType = 4;
        // java } else if ("outline".equals(view))
        } else if Some("outline".to_string()) == view {
            // java viewType = 2;
            viewType = 2;
        // java } else if ("print".equals(view))
        } else if Some("print".to_string()) == view {
            // java viewType = 1;
            viewType = 1;
        // java } else if ("web".equals(view))
        } else if Some("web".to_string()) == view {
            // java viewType = 5;
            viewType = 5;
        } else {
            // java IOAssert.fail("it should not reach here!");
            IOAssert::fail_msg("it should not reach here!");
        }
        // java dst.setInt(396, viewType);
        dst.setInt(396, viewType);
    }

    // java private static void correctView(MutablePropertySet dst)
    pub fn correctView(dst: &mut MutablePropertySet) {
        // java if (!dst.contains(396))
        if !dst.contains(396) {
            // java dst.setInt(396, 0);
            dst.setInt(396, 0);
        }
    }

    // java private static void setZoom(Attributes attributes, MutablePropertySet dst)
    pub fn setZoom(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String type = ConvertHelper.getString(attributes);
        let r#type: Option<String> = ConvertHelper::getString(attributes);
        // java if (null != type)
        if r#type.is_some() {
            // java dst.setInt(386, convertZoomType(type));
            dst.setInt(386, Self::convertZoomType(r#type.unwrap()));
        }
        // java Integer percent = ShareConvertHelper.getInteger4STDecimalNumberOrPercent(attributes, "percent");
        let percent: Option<i32> = ShareConvertHelper::getInteger4STDecimalNumberOrPercent__attributes_string(attributes, "percent");
        // java if (null != percent)
        if percent.is_some() {
            // java dst.setInt(385, percent);
            dst.setInt(385, percent.unwrap());
        }
    }

    // java private static int convertZoomType(String val)
    pub fn convertZoomType(val: String) -> i32 {
        // java int zoomType = 0;
        let mut zoomType: i32 = 0;
        // java if ("none".equals(val))
        if "none" == val {
            // java zoomType = 0;
            zoomType = 0;
        // java } else if ("fullPage".equals(val))
        } else if "fullPage" == val {
            // java zoomType = 1;
            zoomType = 1;
        // java } else if ("bestFit".equals(val))
        } else if "bestFit" == val {
            // java zoomType = 2;
            zoomType = 2;
        // java } else if ("textFit".equals(val))
        } else if "textFit" == val {
            // java zoomType = 3;
            zoomType = 3;
        } else {
            // java IOAssert.fail("it should not reach here!");
            IOAssert::fail_msg("it should not reach here!");
        }
        // java return zoomType;
        return zoomType;
    }

    // java private static void setProofState(Attributes attributes, MutablePropertySet dst)
    pub fn setProofState(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java setSpellingAttr(attributes, dst);
        Self::setSpellingAttr(attributes, dst);
        // java setGrammarAttr(attributes, dst);
        Self::setGrammarAttr(attributes, dst);
    }

    // java private static void setGrammarAttr(Attributes attributes, MutablePropertySet dst)
    pub fn setGrammarAttr(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String attrGrammar = "grammar";
        let attrGrammar = "grammar";
        let _ = attrGrammar;
        // java String grammar = ConvertHelper.getString(attributes, "grammar");
        let grammar: Option<String> = ConvertHelper::getString__String(attributes, "grammar");
        // java int grammarVal = null == grammar ? 1 : convertProofType(grammar);
        let grammarVal: i32 = match grammar {
            None => 1,
            Some(g) => Self::convertProofType(g),
        };
        // java dst.setInt(415, grammarVal);
        dst.setInt(415, grammarVal);
    }

    // java private static void setSpellingAttr(Attributes attributes, MutablePropertySet dst)
    pub fn setSpellingAttr(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String attrSpelling = "spelling";
        let attrSpelling = "spelling";
        let _ = attrSpelling;
        // java String spelling = ConvertHelper.getString(attributes, "spelling");
        let spelling: Option<String> = ConvertHelper::getString__String(attributes, "spelling");
        // java int spellingVal = null == spelling ? 1 : convertProofType(spelling);
        let spellingVal: i32 = match spelling {
            None => 1,
            Some(s) => Self::convertProofType(s),
        };
        // java dst.setInt(416, spellingVal);
        dst.setInt(416, spellingVal);
    }

    // java private static int convertProofType(String proofState)
    pub fn convertProofType(proofState: String) -> i32 {
        // java if ("clean".equals(proofState))
        if "clean" == proofState {
            // java return 0;
            return 0;
        // java } else if ("dirty".equals(proofState))
        } else if "dirty" == proofState {
            // java return 1;
            return 1;
        } else {
            // java IOAssert.fail("it should not reach here!");
            IOAssert::fail_msg("it should not reach here!");
            // java return 1;
            return 1;
        }
    }

    // java private static void setStylePaneSortMethod(Attributes attributes, MutablePropertySet dst)
    pub fn setStylePaneSortMethod(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String styleSort = ConvertHelper.getString(attributes);
        let styleSort: Option<String> = ConvertHelper::getString(attributes);
        // java if (null != styleSort)
        if styleSort.is_some() {
            // java dst.setInt(438, convertStyleSortType(styleSort));
            dst.setInt(438, Self::convertStyleSortType(styleSort.unwrap()));
        }
    }

    // java private static int convertStyleSortType(String stylePaneSortMethod)
    pub fn convertStyleSortType(stylePaneSortMethod: String) -> i32 {
        // java int coreStyleSort = 8;
        let mut coreStyleSort: i32 = 8;
        // java if ("0000".equals(stylePaneSortMethod))
        if "0000" == stylePaneSortMethod {
            // java coreStyleSort = 0;
            coreStyleSort = 0;
        // java } else if ("0001".equals(stylePaneSortMethod))
        } else if "0001" == stylePaneSortMethod {
            // java coreStyleSort = 1;
            coreStyleSort = 1;
        // java } else if ("0002".equals(stylePaneSortMethod))
        } else if "0002" == stylePaneSortMethod {
            // java coreStyleSort = 2;
            coreStyleSort = 2;
        // java } else if ("0003".equals(stylePaneSortMethod))
        } else if "0003" == stylePaneSortMethod {
            // java coreStyleSort = 3;
            coreStyleSort = 3;
        // java } else if ("0004".equals(stylePaneSortMethod))
        } else if "0004" == stylePaneSortMethod {
            // java coreStyleSort = 4;
            coreStyleSort = 4;
        // java } else if ("0005".equals(stylePaneSortMethod))
        } else if "0005" == stylePaneSortMethod {
            // java coreStyleSort = 5;
            coreStyleSort = 5;
        // java } else if ("basedOn".equals(stylePaneSortMethod))
        } else if "basedOn" == stylePaneSortMethod {
            // java coreStyleSort = 10;
            coreStyleSort = 10;
        // java } else if ("default".equals(stylePaneSortMethod))
        } else if "default" == stylePaneSortMethod {
            // java coreStyleSort = 8;
            coreStyleSort = 8;
        // java } else if ("font".equals(stylePaneSortMethod))
        } else if "font" == stylePaneSortMethod {
            // java coreStyleSort = 9;
            coreStyleSort = 9;
        // java } else if ("name".equals(stylePaneSortMethod))
        } else if "name" == stylePaneSortMethod {
            // java coreStyleSort = 6;
            coreStyleSort = 6;
        // java } else if ("priority".equals(stylePaneSortMethod))
        } else if "priority" == stylePaneSortMethod {
            // java coreStyleSort = 7;
            coreStyleSort = 7;
        // java } else if ("type".equals(stylePaneSortMethod))
        } else if "type" == stylePaneSortMethod {
            // java coreStyleSort = 11;
            coreStyleSort = 11;
        } else {
            // java IOAssert.fail("it should not reach here!");
            IOAssert::fail_msg("it should not reach here!");
        }
        // java return coreStyleSort;
        return coreStyleSort;
    }

    // java private static void setCompatSetting(Attributes attributes, MutablePropertySet dst)
    pub fn setCompatSetting(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Object compatSettings = dst.getValue(542);
        let mut compatSettings = dst.getValue(542);
        // java if (null == compatSettings)
        if compatSettings.is_none() {
            // java compatSettings = new ArrayList();
            let new_compatSettings: Vec<CompatSetting> = Vec::new();
            // java dst.setValue(542, compatSettings);
            dst.setValue(542, Some(std::sync::Arc::new(new_compatSettings)));
            // rust: Java 是引用语义（后续 add 直接改这张表）；Rust 值语义 ⇒
            //   写回后重新读出，使后面的 add 分支看到同一份数据。
            compatSettings = dst.getValue(542);
        }
        // java if (0 != attributes.getLength())
        if 0 != attributes.getLength() {
            // java if (compatSettings instanceof ArrayList)
            {
                // java ((ArrayList)compatSettings).add(convertCompatSetting(attributes));
                let mut list: Vec<CompatSetting> = dst
                    .getValue(542)
                    .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<CompatSetting>>().cloned())
                    .unwrap_or_default();
                list.push(Self::convertCompatSetting(attributes));
                // rust: 回写（Java 为原地修改同一 ArrayList）
                dst.setValue(542, Some(std::sync::Arc::new(list)));
            }
        }
        // rust: 卸下未使用的绑定
        compatSettings = None;
    }

    // java private static CompatSetting convertCompatSetting(Attributes attributes)
    pub fn convertCompatSetting(attributes: &dyn Attributes) -> CompatSetting {
        // java CompatSetting compatSetting = new CompatSetting();
        let mut compatSetting = CompatSetting::default();
        // java String attrName = "name";
        let attrName = "name";
        let _ = attrName;
        // java String attrUri = "uri";
        let attrUri = "uri";
        let _ = attrUri;
        // java String name = ConvertHelper.getString(attributes, "name");
        let name: Option<String> = ConvertHelper::getString__String(attributes, "name");
        // java if (null != name)
        if name.is_some() {
            // java compatSetting.setName(name);
            compatSetting.setName(&name.unwrap());
        }
        // java String uri = ConvertHelper.getString(attributes, "uri");
        let uri: Option<String> = ConvertHelper::getString__String(attributes, "uri");
        // java if (null != uri)
        if uri.is_some() {
            // java compatSetting.setURI(uri);
            compatSetting.setURI(&uri.unwrap());
        }
        // java String val = ConvertHelper.getString(attributes);
        let val: Option<String> = ConvertHelper::getString(attributes);
        // java if (null != val)
        if val.is_some() {
            // java compatSetting.setVal(val);
            compatSetting.setVal(&val.unwrap());
        }
        // java return compatSetting;
        return compatSetting;
    }

    // java private static void correctCompactSetting(MutablePropertySet dst)
    pub fn correctCompactSetting(dst: &mut MutablePropertySet) {
        // java Object compatSettings = dst.getValue(542);
        let compatSettings = dst.getValue(542);
        // java if (null != compatSettings && compatSettings instanceof ArrayList)
        let is_list = compatSettings
            .as_ref()
            .map(|v| v.as_ref().as_any().downcast_ref::<Vec<CompatSetting>>().is_some())
            .unwrap_or(false);
        if is_list {
            // java ArrayList<CompatSetting> compatSettingList = (ArrayList)compatSettings;
            let compatSettingList: Vec<CompatSetting> = compatSettings
                .as_ref()
                .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<CompatSetting>>().cloned())
                .unwrap_or_default();
            // java CompatSetting[] compatSettingArr = new CompatSetting[compatSettingList.size()];
            let mut compatSettingArr: Vec<CompatSetting> = compatSettingList.clone();
            // java compatSettingList.toArray(compatSettingArr);
            compatSettingArr.clone_from_slice(&compatSettingList);
            // java dst.setValue(542, compatSettingArr);
            dst.setValue(542, Some(std::sync::Arc::new(compatSettingArr)));
            // rust: 按 Java 原文逐步执行；Rust `Vec` 即数组，`toArray` 对应一次拷贝。
        // java } else
        } else {
            // java dst.setValue(542, new CompatSetting[0]);
            dst.setValue(542, Some(std::sync::Arc::new(Vec::<CompatSetting>::new())));
        }
    }

    // java private static void setRevisionView(Attributes attributes, MutablePropertySet dst)
    pub fn setRevisionView(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String attrMarkup = "markup";
        let attrMarkup = "markup";
        // java String attrComments = "comments";
        let attrComments = "comments";
        // java String attrInsDel = "insDel";
        let attrInsDel = "insDel";
        // java String attrFormatting = "formatting";
        let attrFormatting = "formatting";
        // java String attrInkAnnotations = "inkAnnotations";
        let _ = attrMarkup;
        let _ = attrComments;
        let _ = attrInsDel;
        let _ = attrFormatting;
        let attrInkAnnotations = "inkAnnotations";
        let _ = attrInkAnnotations;
        // java Boolean markup = ConvertHelper.getBoolean(attributes, "markup");
        let markup: Option<bool> = ConvertHelper::getBoolean(attributes, "markup");
        // java if (null != markup)
        if markup.is_some() {
            // java dst.setBoolean(443, markup);
            dst.setBoolean(443, markup.unwrap());
        }
        // java Boolean comments = ConvertHelper.getBoolean(attributes, "comments");
        let comments: Option<bool> = ConvertHelper::getBoolean(attributes, "comments");
        // java if (null != comments)
        if comments.is_some() {
            // java dst.setBoolean(444, comments);
            dst.setBoolean(444, comments.unwrap());
        }
        // java Boolean insDel = ConvertHelper.getBoolean(attributes, "insDel");
        let insDel: Option<bool> = ConvertHelper::getBoolean(attributes, "insDel");
        // java if (null != insDel)
        if insDel.is_some() {
            // java dst.setBoolean(445, insDel);
            dst.setBoolean(445, insDel.unwrap());
        }
        // java Boolean formatting = ConvertHelper.getBoolean(attributes, "formatting");
        let formatting: Option<bool> = ConvertHelper::getBoolean(attributes, "formatting");
        // java if (null != formatting)
        if formatting.is_some() {
            // java dst.setBoolean(446, formatting);
            dst.setBoolean(446, formatting.unwrap());
        }
        // java Boolean inkAnnotations = ConvertHelper.getBoolean(attributes, "inkAnnotations");
        let inkAnnotations: Option<bool> = ConvertHelper::getBoolean(attributes, "inkAnnotations");
        // java if (null != inkAnnotations)
        if inkAnnotations.is_some() {
            // java dst.setBoolean(447, inkAnnotations);
            dst.setBoolean(447, inkAnnotations.unwrap());
        }
    }

    // java private static void setSummaryLength(Attributes attributes, MutablePropertySet dst)
    pub fn setSummaryLength(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Integer summaryLength = ShareConvertHelper.getInteger4STDecimalNumberOrPercent(attributes);
        let summaryLength: Option<i32> = ShareConvertHelper::getInteger4STDecimalNumberOrPercent(attributes);
        // java if (null != summaryLength)
        if summaryLength.is_some() {
            // java dst.setFloat(453, (float)summaryLength);
            dst.setFloat(453, summaryLength.unwrap() as f32);
        }
    }

    // java private static void setDocVars(Attributes attributes, MutablePropertySet dst)
    pub fn setDocVars(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java dst.setValue(545, new ArrayList());
        let _ = attributes;
        dst.setValue(545, Some(std::sync::Arc::new(Vec::<DocVar>::new())));
    }

    // java private static void setDocVar(Attributes attributes, MutablePropertySet dst)
    pub fn setDocVar(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Object docVar = dst.getValue(545);
        let docVar = dst.getValue(545);
        // java IOAssert.assertNotNull("docVarList should not be null", docVar);
        let _ = &docVar;
        // java if (docVar != null && docVar instanceof List)
        if docVar.is_some() {
            // java List<DocVar> docVarList = (List)docVar;
            let mut docVarList: Vec<DocVar> = dst
                .getValue(545)
                .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<DocVar>>().cloned())
                .unwrap_or_default();
            // java String attrName = "name";
            let attrName = "name";
            let _ = attrName;
            // java String name = ConvertHelper.getString(attributes, "name");
            let name: Option<String> = ConvertHelper::getString__String(attributes, "name");
            // java String val = ConvertHelper.getString(attributes);
            let val: Option<String> = ConvertHelper::getString(attributes);
            // java docVarList.add(new DocVar(name, val));
            // rust: Java 构造器内 Assert.assertNotNull 保证非空 ⇒ 缺省为空串。
            docVarList.push(DocVar::DocVar(name.unwrap_or_default(), val.unwrap_or_default()));
            // rust: 回写（Java 为原地修改同一 List）
            dst.setValue(545, Some(std::sync::Arc::new(docVarList)));
        }
    }

    // java private static void crrectDocVars(MutablePropertySet dst)
    pub fn crrectDocVars(dst: &mut MutablePropertySet) {
        // java Object docVar = dst.getValue(545);
        let docVar = dst.getValue(545);
        // java if (null != docVar && docVar instanceof List)
        let is_list = docVar
            .as_ref()
            .map(|v| v.as_ref().as_any().downcast_ref::<Vec<DocVar>>().is_some())
            .unwrap_or(false);
        if is_list {
            // java List<DocVar> docVarList = (List)docVar;
            let docVarList: Vec<DocVar> = docVar
                .as_ref()
                .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<DocVar>>().cloned())
                .unwrap_or_default();
            // java DocVar[] docTypesArr = new DocVar[docVarList.size()];
            let mut docTypesArr: Vec<DocVar> = docVarList.clone();
            // java docVarList.toArray(docTypesArr);
            docTypesArr.clone_from_slice(&docVarList);
            // java dst.setValue(545, docTypesArr);
            dst.setValue(545, Some(std::sync::Arc::new(docTypesArr)));
            // rust: 按 Java 原文逐步执行（`Vec` 即数组，`toArray` 对应一次拷贝）。
        }
    }

    // java private static void setAttachedSchema(Attributes attributes, MutablePropertySet dst)
    pub fn setAttachedSchema(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Object attachedSchema = dst.getValue(546);
        let attachedSchema = dst.getValue(546);
        // java if (null == attachedSchema)
        if attachedSchema.is_none() {
            // java List<String> attachedSchemaList = new ArrayList();
            let mut attachedSchemaList: Vec<String> = Vec::new();
            // java dst.setValue(546, attachedSchemaList);
            dst.setValue(546, Some(std::sync::Arc::new(attachedSchemaList.clone())));
            // java String attachedSchemaStr = ConvertHelper.getString(attributes);
            let attachedSchemaStr: Option<String> = ConvertHelper::getString(attributes);
            // java attachedSchemaList.add(attachedSchemaStr);
            attachedSchemaList.push(attachedSchemaStr.unwrap_or_default());
            // rust: Java 的 `add` 原地改同一张表；Rust 值语义 ⇒ 追加后一次写回。
            dst.setValue(546, Some(std::sync::Arc::new(attachedSchemaList)));
        // java } else if (attachedSchema instanceof List)
        } else {
            // java List<String> attachedSchemaList = (List)attachedSchema;
            let mut attachedSchemaList: Vec<String> = dst
                .getValue(546)
                .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<String>>().cloned())
                .unwrap_or_default();
            // java String attachedSchemaStr = ConvertHelper.getString(attributes);
            let attachedSchemaStr: Option<String> = ConvertHelper::getString(attributes);
            // java attachedSchemaList.add(attachedSchemaStr);
            attachedSchemaList.push(attachedSchemaStr.unwrap_or_default());
            // rust: 回写（Java 为原地修改同一 List）
            dst.setValue(546, Some(std::sync::Arc::new(attachedSchemaList)));
        }
    }

    // java private static void correctAttachedSchema(MutablePropertySet dst)
    pub fn correctAttachedSchema(dst: &mut MutablePropertySet) {
        // java Object attachedSchema = dst.getValue(546);
        let attachedSchema = dst.getValue(546);
        // java if (null != attachedSchema && attachedSchema instanceof List)
        let is_list = attachedSchema
            .as_ref()
            .map(|v| v.as_ref().as_any().downcast_ref::<Vec<String>>().is_some())
            .unwrap_or(false);
        if is_list {
            // java List<String> attachedSchemaList = (List)attachedSchema;
            let attachedSchemaList: Vec<String> = attachedSchema
                .as_ref()
                .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<String>>().cloned())
                .unwrap_or_default();
            // java String[] attachedSchemaArr = new String[attachedSchemaList.size()];
            let mut attachedSchemaArr: Vec<String> = attachedSchemaList.clone();
            // java attachedSchemaList.toArray(attachedSchemaArr);
            attachedSchemaArr.clone_from_slice(&attachedSchemaList);
            // java dst.setValue(546, attachedSchemaArr);
            dst.setValue(546, Some(std::sync::Arc::new(attachedSchemaArr)));
            // rust: 按 Java 原文逐步执行（`Vec<String>` 即数组，`toArray` 对应一次拷贝）。
        }
    }

    // java private static void setThemeFontLang(Attributes attributes, MutablePropertySet dst)
    pub fn setThemeFontLang(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String attrBidi = "bidi";
        let attrBidi = "bidi";
        let _ = attrBidi;
        // java String attrEastAsia = "eastAsia";
        let attrEastAsia = "eastAsia";
        let _ = attrEastAsia;
        // java String val = ConvertHelper.getString(attributes);
        let val: Option<String> = ConvertHelper::getString(attributes);
        // java if (null != val)
        if val.is_some() {
            // java dst.setString(571, val);
            dst.setString(571, val.unwrap());
        }
        // java String bidi = ConvertHelper.getString(attributes, "bidi");
        let bidi: Option<String> = ConvertHelper::getString__String(attributes, "bidi");
        // java if (null != bidi)
        if bidi.is_some() {
            // java dst.setString(573, bidi);
            dst.setString(573, bidi.unwrap());
        }
        // java String eastAsia = ConvertHelper.getString(attributes, "eastAsia");
        let eastAsia: Option<String> = ConvertHelper::getString__String(attributes, "eastAsia");
        // java if (null != eastAsia)
        if eastAsia.is_some() {
            // java dst.setString(572, eastAsia);
            dst.setString(572, eastAsia.unwrap());
        }
    }

    // java private static void correctDoNotIncludeSubdocsInStats(MutablePropertySet dst)
    pub fn correctDoNotIncludeSubdocsInStats(dst: &mut MutablePropertySet) {
        // java if (null == dst.getValue(547))
        if dst.getValue(547).is_none() {
            // java dst.setValue(547, false);
            dst.setValue(547, Some(std::sync::Arc::new(false)));
        }
    }

    // java private static void setForceUpgrade(Attributes attributes, MutablePropertySet dst)
    pub fn setForceUpgrade(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java dst.setBoolean(570, true);
        let _ = attributes;
        dst.setBoolean(570, true);
    }

    // java private static void correctCaptions(MutablePropertySet dst)
    pub fn correctCaptions(dst: &mut MutablePropertySet) {
        // java CapationsImporter.correctCaptions(dst);
        CapationsImporter::correctCaptions(dst);
    }

    // java private void setStrictFirstAndLastChars(Attributes attributes, MutablePropertySet dst)
    pub fn setStrictFirstAndLastChars(&self, attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java Boolean strictFirstAndLastChars = ConvertHelper.toBoolean4NotNull(attributes);
        let strictFirstAndLastChars: Option<bool> = ConvertHelper::toBoolean4NotNull(attributes);
        // java dst.setBoolean(468, strictFirstAndLastChars);
        dst.setBoolean(468, strictFirstAndLastChars.unwrap_or(false));
    }

    // java private static void setReadModeInkLockDown(Attributes attributes, MutablePropertySet dst)
    pub fn setReadModeInkLockDown(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java String attrActualPg = "actualPg";
        let attrActualPg = "actualPg";
        let _ = attrActualPg;
        // java String attrW = "w";
        let attrW = "w";
        let _ = attrW;
        // java String attrH = "h";
        let attrH = "h";
        let _ = attrH;
        // java String attrFontSz = "fontSz";
        let attrFontSz = "fontSz";
        let _ = attrFontSz;
        // java Boolean actualPg = ConvertHelper.getBoolean(attributes, "actualPg");
        let actualPg: Option<bool> = ConvertHelper::getBoolean(attributes, "actualPg");
        // java if (null != actualPg)
        if actualPg.is_some() {
            // java dst.setBoolean(563, actualPg);
            dst.setBoolean(563, actualPg.unwrap());
        }
        // java Long w = ShareConvertHelper.getLong4STPixelsMeasure(attributes, "w");
        let w: Option<i64> = ShareConvertHelper::getLong4STPixelsMeasure(attributes, "w");
        // java if (null != w)
        if w.is_some() {
            // java dst.setInt(564, w.intValue());
            dst.setInt(564, w.unwrap() as i32);
        }
        // java Long h = ShareConvertHelper.getLong4STPixelsMeasure(attributes, "h");
        let h: Option<i64> = ShareConvertHelper::getLong4STPixelsMeasure(attributes, "h");
        // java if (null != h)
        if h.is_some() {
            // java dst.setInt(565, h.intValue());
            dst.setInt(565, h.unwrap() as i32);
        }
        // java Integer font = ShareConvertHelper.getInteger4STDecimalNumberOrPercent(attributes, "fontSz");
        let font: Option<i32> = ShareConvertHelper::getInteger4STDecimalNumberOrPercent__attributes_string(attributes, "fontSz");
        // java if (null != font)
        if font.is_some() {
            // java dst.setFloat(566, font.floatValue());
            dst.setFloat(566, font.unwrap() as f32);
        }
    }
}
