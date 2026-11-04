// function_id=02_function_document_read_surface
// cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_document_read_surface

use std::path::PathBuf;
use std::process::Command;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::open_text_document;
use word_core::cn::wps::moffice::writer::core::Document::{
    Document, HEADER_DOCUMENT, MAIN_DOCUMENT,
};
use word_core::cn::wps::moffice::writer::core::RangeTool::RangeTool;
use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
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

fn main_dyn(td: &TextDocument) -> &dyn Document {
    td.getSubdocument(MAIN_DOCUMENT).expect("main")
}

fn ptr_ok<T: ?Sized>(p: Option<*mut T>) -> bool {
    p.map(|x| !x.is_null()).unwrap_or(false)
}

fn assert_hot_nonnull(doc: &dyn Document) {
    assert!(ptr_ok(Document::getTextRope(doc)), "getTextRope");
    assert!(ptr_ok(Document::getChpxTable(doc)), "getChpxTable");
    assert!(ptr_ok(Document::getPapxTable(doc)), "getPapxTable");
    assert!(ptr_ok(Document::getPLCField(doc)), "getPLCField");
    assert!(ptr_ok(Document::getStyles(doc)), "getStyles");
}

#[test]
fn DR1_empty_main_length_zero_matches_inherent() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let doc: &dyn Document = sd;
    assert_eq!(Document::getLength(doc), 0);
    assert_eq!(Document::getLength(doc), SubDocument::getLength(sd));
}

#[test]
fn DR2_trait_get_text_rope_some() {
    let td = TextDocument::new();
    assert!(ptr_ok(Document::getTextRope(main_dyn(&td))));
}

#[test]
fn DR3_trait_get_chpx_table_some() {
    let td = TextDocument::new();
    assert!(ptr_ok(Document::getChpxTable(main_dyn(&td))));
}

#[test]
fn DR4_trait_get_papx_table_some() {
    let td = TextDocument::new();
    assert!(ptr_ok(Document::getPapxTable(main_dyn(&td))));
}

#[test]
fn DR5_type0_get_plc_section_some() {
    let td = TextDocument::new();
    assert!(ptr_ok(Document::getPLCSection(main_dyn(&td))));
}

#[test]
fn DR6_get_plc_field_same_as_inherent() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let via_trait = Document::getPLCField(sd).expect("trait getPLCField");
    let via_inh = sd.getPLCField().expect("inherent getPLCField");
    assert!(!via_trait.is_null());
    assert!(std::ptr::eq(
        via_trait,
        via_inh as *const _ as *mut _
    ));
}

#[test]
fn DR7_get_styles_some() {
    let td = TextDocument::new();
    assert!(ptr_ok(Document::getStyles(main_dyn(&td))));
}

#[test]
fn DR8_header_section_none_field_and_tables_some() {
    let td = TextDocument::new();
    let hdr: &dyn Document = td.getSubdocument(HEADER_DOCUMENT).expect("header");
    assert!(
        Document::getPLCSection(hdr).is_none(),
        "Java type2 does not assign mSections"
    );
    assert_hot_nonnull(hdr);
}

#[test]
fn DR9_mock_has_no_main() {
    let td = TextDocument::TextDocument__boolean(true);
    assert!(td.getSubdocument(0).is_none());
}

#[test]
fn DR10_pipeline_hello_world_document_read_surface() {
    let path = hello_world();
    let digest = sha256_file(&path);
    assert_eq!(
        digest,
        "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let td = open_text_document(&bytes).expect("open_text_document");
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("parsed main");
    assert_hot_nonnull(sd);
    assert!(ptr_ok(Document::getPLCSection(sd)));
    assert!(
        Document::getLength(sd) > 0,
        "parsed main getLength > 0"
    );
    assert!(
        RangeTool::isBeginOfSubDocumentsPLCNode(sd, 0),
        "product RangeTool reads Document"
    );
}
