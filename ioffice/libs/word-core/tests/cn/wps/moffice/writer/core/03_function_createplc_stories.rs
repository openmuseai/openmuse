// function_id=03_function_createplc_stories
// cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_createplc_stories

use std::path::PathBuf;
use std::process::Command;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::open_text_document;
use word_core::cn::wps::moffice::writer::core::Document::{
    Document, ENDNOTE_DOCUMENT, FOOTNOTE_DOCUMENT, HEADERTEXTBOX_DOCUMENT, HEADER_DOCUMENT,
    MAIN_DOCUMENT, TEXTBOX_DOCUMENT,
};
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;

fn hello_world() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/p1_layout_document/hello-world.docx")
}

fn sha256_file(path: &PathBuf) -> String {
    let out = Command::new("shasum")
        .args(["-a", "256"])
        .arg(path)
        .output()
        .expect("shasum");
    String::from_utf8(out.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}

fn ptr_ok<T: ?Sized>(p: Option<*mut T>) -> bool {
    p.map(|x| !x.is_null()).unwrap_or(false)
}

#[test]
fn CP1_footnote_story_field_txt_and_main_ref() {
    let td = TextDocument::new();
    let fn_doc: &dyn Document = td.getSubdocument(FOOTNOTE_DOCUMENT).expect("fn");
    assert!(ptr_ok(Document::getPLCField(fn_doc)));
    assert!(ptr_ok(Document::getFootnoteTxt(fn_doc)));
    let main: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(ptr_ok(Document::getFootnoteRef(main)));
}

#[test]
fn CP2_header_plc_header_no_section() {
    let td = TextDocument::new();
    let hdr: &dyn Document = td.getSubdocument(HEADER_DOCUMENT).expect("hdr");
    assert!(ptr_ok(Document::getPLCField(hdr)));
    assert!(
        ptr_ok(Document::getPLCHeader(hdr)),
        "Document::getPLCHeader on type2"
    );
    assert!(Document::getPLCSection(hdr).is_none());
}

#[test]
fn CP3_endnote_story_and_main_ref() {
    let td = TextDocument::new();
    let en: &dyn Document = td.getSubdocument(ENDNOTE_DOCUMENT).expect("en");
    assert!(ptr_ok(Document::getPLCField(en)));
    assert!(ptr_ok(Document::getEndnoteTxt(en)));
    let main: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(ptr_ok(Document::getEndnoteRef(main)));
}

#[test]
fn CP4_textbox_stories_same_field() {
    let td = TextDocument::new();
    for ty in [TEXTBOX_DOCUMENT, HEADERTEXTBOX_DOCUMENT] {
        let sd = td.getSubdocument(ty).unwrap_or_else(|| panic!("slot {ty}"));
        let via_trait = Document::getTextboxText(sd).expect("trait textbox");
        let via_inh = sd.getTextboxText().expect("inherent textbox");
        assert!(!via_trait.is_null());
        assert!(std::ptr::eq(via_trait, via_inh as *const _ as *mut _));
        assert!(ptr_ok(Document::getPLCField(sd)));
    }
}

#[test]
fn CP5_main_still_has_section_and_field() {
    let td = TextDocument::new();
    let main: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(ptr_ok(Document::getPLCSection(main)));
    assert!(ptr_ok(Document::getPLCField(main)));
}

#[test]
fn CP6_pipeline_header_get_plc_header() {
    let path = hello_world();
    assert_eq!(
        sha256_file(&path),
        "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let td = open_text_document(&bytes).expect("open_text_document");
    let hdr: &dyn Document = td.getSubdocument(HEADER_DOCUMENT).expect("parsed hdr");
    assert!(
        ptr_ok(Document::getPLCHeader(hdr)),
        "product header getPLCHeader"
    );
}
