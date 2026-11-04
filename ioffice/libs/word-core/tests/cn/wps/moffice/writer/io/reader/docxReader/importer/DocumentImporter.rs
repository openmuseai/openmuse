// function_id=03_function_di_construct_route
// cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_di_construct_route
#![allow(non_snake_case)]

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::empty;
use word_core::cn::wps::moffice::writer::core::KStyle::TextDocument as SoftTextDocument;
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::DocumentImporter::DocumentImporter;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::IOListener;

struct DummyListener;
impl IOListener for DummyListener {
    // rust: ★M-STAGE2 P1-D1 —— 真实 IOListener（9 方法）补齐此前只对着
    //   3 方法影子写的空实现。
    fn onLoadParas(&mut self, _cp: i32) {}
    fn onFirstLock(&mut self) {}

    fn onFirstUnLock(&mut self) {}

    fn onError(&mut self, _resultCode: i32, _msg: &dyn stubs::java::lang::Object::Object) {}

    fn beginLoadOnlineSecurityDoc(&mut self) {}

    fn onFinish(&mut self) {}

    fn onFinishDumpObjects(&mut self) {}

    fn onHtmlOpenError(&mut self) {}

    fn onFinishVerifyWritePassword(&mut self) {}
}


fn light(mode: i32) -> DocumentImporter {
    let ks = KStyles::KStyles(&SoftTextDocument::default());
    DocumentImporter::DocumentImporter__kstyles_int(Some(ks), mode)
}

fn main_di(doc: &mut TextDocument) -> DocumentImporter {
    DocumentImporter::DocumentImporter__textdocument_iolistener_idlprovider_boolean_themecontext_int_xml07context_xml03context(
        doc,
        Some(Box::new(DummyListener)),
        None,
        false,
        None,
        0,
        None,
        None,
    )
}

#[test]
fn D3_1_light_assembles_maps_and_converters() {
    let di = light(0);
    assert!(di.mStyleId2Sti.is_some());
    assert!(di.mNumIdRedirection.is_some());
    assert!(di.mStylesImporter.is_some());
    assert!(di.mParaPropsConverter.is_some());
}

#[test]
#[should_panic(expected = "kStyles should not be null")]
fn D3_2_light_null_kstyles_panics() {
    let _ = DocumentImporter::DocumentImporter__kstyles_int(None, 0);
}

#[test]
fn D3_3_main_empty_sets_property_empty() {
    let mut doc = TextDocument::TextDocument();
    let _di = main_di(&mut doc);
    assert!(doc.getProperty().isEmpty());
}

#[test]
fn D3_4_main_nonempty_keeps_property() {
    let mut owned = TextDocument::new();
    owned.createSubDocument(0);
    {
        let main = owned.getSubdocumentMut(0).expect("main");
        CoreHelper::appendText__string_ps(
            Some(main as &mut dyn word_core::cn::wps::moffice::writer::core::Document::Document),
            Some("x"),
            Some(&*empty),
        );
    }
    let mut mps = MutablePropertySet::new();
    mps.setInt(1, 7);
    let kept = mps.freeze();
    owned.setProperty__propertyset(&kept);
    let before = owned.getProperty();
    let _di = main_di(&mut owned);
    assert!(!before.isEmpty());
    assert_eq!(owned.getProperty().getInt(1), before.getInt(1));
}

#[test]
fn D3_5_main_assembles_all_ordinals() {
    let mut doc = TextDocument::TextDocument();
    let di = main_di(&mut doc);
    for ord in 0..=6 {
        let imp = di.getSubDocumentImporter(ord).expect("importer");
        assert_eq!(imp.getRoutedType(), ord);
    }
}

#[test]
fn D3_6_route_main() {
    let mut doc = TextDocument::TextDocument();
    let di = main_di(&mut doc);
    assert_eq!(di.getSubDocumentImporter(0).unwrap().getRoutedType(), 0);
}

#[test]
fn D3_7_route_textbox() {
    let mut doc = TextDocument::TextDocument();
    let di = main_di(&mut doc);
    assert_eq!(di.getSubDocumentImporter(5).unwrap().getRoutedType(), 5);
}

#[test]
fn D3_8_route_header() {
    let mut doc = TextDocument::TextDocument();
    let di = main_di(&mut doc);
    assert_eq!(di.getSubDocumentImporter(2).unwrap().getRoutedType(), 2);
}

#[test]
fn D3_9_route_comment() {
    let mut doc = TextDocument::TextDocument();
    let di = main_di(&mut doc);
    assert_eq!(di.getSubDocumentImporter(3).unwrap().getRoutedType(), 3);
}

#[test]
fn D3_10_route_endnote() {
    let mut doc = TextDocument::TextDocument();
    let di = main_di(&mut doc);
    assert_eq!(di.getSubDocumentImporter(4).unwrap().getRoutedType(), 4);
}

#[test]
fn D3_11_route_footnote() {
    let mut doc = TextDocument::TextDocument();
    let di = main_di(&mut doc);
    assert_eq!(di.getSubDocumentImporter(1).unwrap().getRoutedType(), 1);
}

#[test]
fn D3_12_route_header_textbox() {
    let mut doc = TextDocument::TextDocument();
    let di = main_di(&mut doc);
    assert_eq!(di.getSubDocumentImporter(6).unwrap().getRoutedType(), 6);
}

#[test]
#[should_panic(expected = "Add more case for sub-document type")]
fn D3_13_route_bad_ordinal_panics() {
    let mut doc = TextDocument::TextDocument();
    let di = main_di(&mut doc);
    let _ = di.getSubDocumentImporter(7);
}

#[test]
fn D3_14_islock_null_document_false() {
    let di = light(0);
    assert!(!di.isLock());
}

#[test]
fn D3_15_islock_follows_main() {
    let mut doc = TextDocument::TextDocument();
    let di = main_di(&mut doc);
    assert!(!di.isLock());
    let p = di.getSubDocumentImporter(0).unwrap().getSubDocumentPtr().unwrap();
    unsafe {
        (*p).setLocked(true);
    }
    assert!(di.isLock());
}

#[test]
fn D3_16_hascontent_empty_and_nonempty() {
    let mut empty_doc = TextDocument::TextDocument();
    let di_empty = main_di(&mut empty_doc);
    assert!(!di_empty.hasContent());

    let mut doc = TextDocument::TextDocument();
    doc.createSubDocument(0);
    {
        let main = doc.getSubdocumentMut(0).expect("main");
        CoreHelper::appendText__string_ps(
            Some(main as &mut dyn word_core::cn::wps::moffice::writer::core::Document::Document),
            Some("x"),
            Some(&*empty),
        );
    }
    let di = main_di(&mut doc);
    assert!(di.hasContent());
}

#[test]
fn D3_17_dispose_no_panic() {
    let mut doc = TextDocument::TextDocument();
    let mut di = main_di(&mut doc);
    di.dispose();
    di.dispose();
}
