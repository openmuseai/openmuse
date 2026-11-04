// function_id=05_function_di_meta_forward
// cargo test --manifest-path libs/word-core/Cargo.toml --test 05_function_di_meta_forward
#![allow(non_snake_case)]

use std::mem::ManuallyDrop;
use std::panic::{catch_unwind, AssertUnwindSafe};

use io_wps_io::cn::wps::io::xwpf::element_handler::styles::StylePrModel::StylePrModel;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::moffice::writer::core::KStyle::TextDocument as ParentTd;
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::DocumentImporter::DocumentImporter;
// rust: ★M-STAGE1 P2 —— `NumLvlType` 原从 `DocumentImporter` 的**影子**再导出取得
//   （`di03_seam` 内 `pub struct NumLvlType;`）；现影子已删除、改用 wps-io 的
//   **真实枚举** ⇒ 与 `XWPFSettings` 同样改为直接从真实来源导入，
//   与 Java `import cn.wps.io.xwpf.element_handler.numbering.NumLvlType` 一致。
use io_wps_io::cn::wps::io::xwpf::element_handler::numbering::NumLvlType::NumLvlType;
// rust: ★M-STAGE1 P2-1 —— `XWPFSettings` 原从 `DocumentImporter` 的**影子**再导出取得；
//   现该名字已改用 wps-io 的**真实类型**（影子删除、`compile-surface:` 解除）
//   ⇒ 测试改为直接从真实来源导入，与 Java `import cn.wps.io.xwpf.usermodel.XWPFSettings` 一致。
use io_wps_io::cn::wps::io::xwpf::usermodel::XWPFSettings::XWPFSettings;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::IOListener;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

#[path = "helpers/wc03_word_attrs.rs"]
mod wc03_word_attrs;
use wc03_word_attrs::WordAttrs;

impl Attributes for WordAttrs {
    fn getLength(&self) -> i32 {
        self.pairs.len() as i32
    }
    fn getValue(&self, uri: &str, localName: &str) -> Option<String> {
        self.get_value(uri, localName).map(|s| s.to_string())
    }
}

struct RecListener;
impl IOListener for RecListener {
    fn onLoadParas(&mut self, _cp: i32) {}
    // rust: ★M-STAGE2 P1-D1 —— 真实 IOListener（9 方法）补齐此前只对着
    //   3 方法影子写的空实现。
    fn onFirstLock(&mut self) {}

    fn onFirstUnLock(&mut self) {}

    fn onError(&mut self, _resultCode: i32, _msg: &dyn stubs::java::lang::Object::Object) {}

    fn beginLoadOnlineSecurityDoc(&mut self) {}

    fn onFinish(&mut self) {}

    fn onFinishDumpObjects(&mut self) {}

    fn onHtmlOpenError(&mut self) {}

    fn onFinishVerifyWritePassword(&mut self) {}

}

const PPR: i32 = 1;
const SECTPR: i32 = 3;
const TBLPR: i32 = 4;
const TRPR: i32 = 6;
const TCPR_STYLE: i32 = 8;
const RUN_NORMAL: i32 = 2096380931;
const PPR_BOOL: i32 = -1769114414;

fn fixture() -> (ManuallyDrop<Box<TextDocument>>, ManuallyDrop<DocumentImporter>) {
    let mut doc = Box::new(TextDocument::TextDocument());
    let di = DocumentImporter::DocumentImporter__textdocument_iolistener_idlprovider_boolean_themecontext_int_xml07context_xml03context(
        doc.as_mut(),
        Some(Box::new(RecListener)),
        None,
        false,
        None,
        0,
        None,
        None,
    );
    (ManuallyDrop::new(doc), ManuallyDrop::new(di))
}

#[test]
fn D5_all_meta_forward_matrix() {
    {
        let parent = ParentTd::default();
        let styles = KStyles::KStyles(&parent);
        let mut light = DocumentImporter::DocumentImporter__kstyles_int(Some(styles), 0);
        light.onMainDocumentEnd();
    }

    let (_doc, mut di) = fixture();
    let empty = WordAttrs::empty();

    di.onMainDocumentStart(true);
    assert!(di.mHasHeaderFooter, "D5-1");
    {
        let si = di.mStylesImporter.as_ref().expect("styles");
        assert!(si.mStyles.borrow().getStyleById__int(0).is_some(), "D5-2");
    }
    di.onMainDocumentStart(false);
    {
        let si = di.mStylesImporter.as_ref().expect("styles");
        let n = si.mStyles.borrow().getStyles().len();
        assert!(n >= 1, "D5-3");
        assert!(si.mStyles.borrow().getStyleById__int(0).is_some(), "D5-3");
    }

    di.onMainDocumentEnd();

    {
        let mut r = MutablePropertySet::new();
        let mut p = MutablePropertySet::new();
        r.setInt(10, 24);
        p.setInt(191, 0);
        di.onImportStyleDocDefaults(&mut r, &mut p);
        {
            let si = di.mStylesImporter.as_ref().expect("styles");
            assert_eq!(si.mStyles.borrow().getRPrDefault().getInt(10), 24, "D5-6 defaults");
        }
        let start = WordAttrs::empty()
            .with(wc03_word_attrs::MAIN_NS, "type", "paragraph")
            .with(wc03_word_attrs::MAIN_NS, "styleId", "H1");
        di.onImportStyleStart(&start);
        let sp = StylePrModel::new();
        di.onImportStyleEnd(&sp);
        di.onImportStylesEnd();
        di.onImportStyleTblStylePrEnd(&sp);
    }

    di.onImportNumberingNumStart(0, &empty);
    di.onImportNumberingNumChild(0, &empty);
    di.onImportNumberingNumEnd();
    // rust: ★影子时代此处是 `NumLvlType::default()`（空壳无字段）；
    //   改用**真实枚举**后取 `Num` 变体（本段测的是 numbering 的 num 侧流程）。
    let lvl = NumLvlType::Num;
    di.onImportNumberingLevelStart(&lvl, &empty);
    di.onImportNumberingLevelChild(&lvl, 0, &empty);
    let rpr = MutablePropertySet::new();
    let ppr = MutablePropertySet::new();
    di.onImportNumberingLevelEnd(&lvl, &rpr, &ppr);
    di.onImportNumberingAbstractNumStart(&empty);
    di.onImportNumberingAbstractNumChild(0, &empty);
    di.onImportNumberingAbstractNumEnd();

    // rust: 真实 `XWPFSettings` 无 `Default`（Java 侧也只有 `public XWPFSettings()`
    //   这一无参构造）⇒ 改用其真实构造器 `XWPFSettings::new()`。
    let settings = XWPFSettings::new();
    di.onImportSettingsStart(&settings);
    di.onImportSettingsChild(0, &empty);
    di.onImportSettingsEnd();

    di.onImportFontStart(&empty);
    di.onImportFontChild(0, &empty);
    di.onImportFontEnd();
    di.onImportFontSchemeStart(&empty);
    di.onImportMajorFontStart();
    di.onImportMinorFontStart();
    di.onImportThemeCSFontStart(&empty);
    di.onImportThemeEAFontStart(&empty);
    di.onImportThemeLatinFontStart(&empty);
    di.onImportThemeFontStart(&empty);

    {
        let mut mps = MutablePropertySet::new();
        mps.setInt(305, 5);
        di.convertProp(Some(TRPR), &mut mps, "", -990885909, &empty);
        assert!(mps.getBoolean(320), "D5-11");
        unsafe {
            let p = di.mDocument.expect("doc");
            assert!((*p).getProperty().getBoolean__int_boolean(643, false), "D5-25");
        }
        di.convertProp(Some(TRPR), &mut mps, "", 0, &empty);
        unsafe {
            let p = di.mDocument.expect("doc");
            assert!((*p).getProperty().getBoolean__int_boolean(643, false), "D5-26");
        }
    }
    {
        let mut mps = MutablePropertySet::new();
        di.convertProp(Some(TBLPR), &mut mps, "", 3385, &empty);
        assert!(mps.contains(303), "D5-12");
        assert_eq!(mps.getInt(303), 0, "D5-12");
    }
    {
        let mut mps = MutablePropertySet::new();
        di.convertProp(Some(PPR), &mut mps, "", PPR_BOOL, &empty);
        assert!(mps.contains(195), "D5-13");
    }
    {
        let mut mps = MutablePropertySet::new();
        di.convertProp(Some(SECTPR), &mut mps, "", 0, &empty);
        assert_eq!(mps.getInt(90014), 1, "D5-14");
    }
    {
        let mut mps = MutablePropertySet::new();
        di.convertProp(Some(TCPR_STYLE), &mut mps, "", -1040545365, &empty);
        assert!(mps.getBoolean(356), "D5-15");
    }
    {
        let mut mps = MutablePropertySet::new();
        let r = catch_unwind(AssertUnwindSafe(|| {
            di.convertProp(None, &mut mps, "", 0, &empty);
        }));
        assert!(r.is_err(), "D5-16");
    }
    {
        let mut mps = MutablePropertySet::new();
        di.convertRunProp(&mut mps, "", RUN_NORMAL, &empty);
        assert!(mps.contains(69), "D5-17");
    }
    {
        let mut mps = MutablePropertySet::new();
        di.convertRunProp(&mut mps, "", 1, &empty);
        assert_eq!(mps.getInt(90018), 1, "D5-18");
    }
    {
        let mut mps = MutablePropertySet::new();
        let out = di.correctProps(-906275931, &mut mps, None);
        assert!(out.is_some(), "D5-19");
        assert_eq!(mps.getInt(90019), 1, "D5-19");
    }
    {
        let mut mps = MutablePropertySet::new();
        let mut orig = MutablePropertySet::new();
        let out = di.correctProps(112148, &mut mps, Some(&mut orig));
        assert!(out.is_some(), "D5-20");
        assert_eq!(orig.getInt(90020), 1, "D5-20");
    }
    {
        let mut mps = MutablePropertySet::new();
        let out = di.correctProps(110226, &mut mps, None);
        assert!(out.is_some(), "D5-21");
    }
    {
        let mut mps = MutablePropertySet::new();
        let out = di.correctProps(110154336, &mut mps, None);
        assert!(out.is_none(), "D5-22");
    }
    {
        let mut mps = MutablePropertySet::new();
        let out = di.correctProps(3553489, &mut mps, None);
        assert!(out.is_none(), "D5-23");
    }
    {
        let mut mps = MutablePropertySet::new();
        let out = di.correctProps(1, &mut mps, None);
        assert!(out.is_none(), "D5-24");
    }

    di.importCustomXmls();

    di.mStylesImporter = None;
    let r = catch_unwind(AssertUnwindSafe(|| {
        di.onImportStyleStart(&empty);
    }));
    assert!(r.is_err(), "D5-7");
}
