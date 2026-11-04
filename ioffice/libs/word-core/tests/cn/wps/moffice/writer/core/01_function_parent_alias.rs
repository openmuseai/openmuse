// function_id=01_function_parent_alias
// cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_parent_alias

use std::path::PathBuf;
use std::process::Command;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::open_text_document;
use word_core::cn::wps::moffice::writer::core::Document::{
    Document, HEADER_DOCUMENT, MAIN_DOCUMENT, SoftTextDocument,
};
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;

fn hello_world() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/p1_layout_document/hello-world.docx")
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

fn styles_ptr(td: &TextDocument) -> Option<*mut KStyles> {
    td.getStyles().map(|s| s as *const KStyles as *mut KStyles)
}

#[test]
fn PA1_ctor_parent_is_core_textdocument() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let parent = sd.getTextDocument();
    assert!(!parent.is_null(), "getTextDocument must not be null");
    assert!(
        std::ptr::eq(
            parent,
            td.as_ref() as *const TextDocument as *mut TextDocument
        ),
        "mParent must be the owning TextDocument"
    );
}

#[test]
fn PA2_document_trait_get_text_document_is_some() {
    let td = TextDocument::new();
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(
        Document::getTextDocument(sd).is_some(),
        "Document::getTextDocument on SubDocument"
    );
}

#[test]
fn PA3_textdocument_get_text_document_is_self() {
    let td = TextDocument::new();
    let via = Document::getTextDocument(td.as_ref()).expect("TextDocument.getTextDocument");
    assert!(
        std::ptr::eq(
            via as *const dyn SoftTextDocument as *const (),
            td.as_ref() as *const TextDocument as *const ()
        ),
        "Java return this"
    );
}

#[test]
fn PA4_main_get_main_document_is_slot_zero() {
    let td = TextDocument::new();
    let main = td.getMainDocument().expect("getMainDocument");
    let via = main.getMainDocument().expect("SubDocument.getMainDocument");
    let via_sd = via
        .asAny()
        .downcast_ref::<SubDocument>()
        .expect("downcast main");
    assert!(std::ptr::eq(via_sd, main));
}

#[test]
fn PA5_header_get_main_document_is_slot_zero_not_self() {
    let td = TextDocument::new();
    let main = td.getSubdocument(MAIN_DOCUMENT).expect("main") as *const SubDocument;
    let hdr = td.getSubdocument(HEADER_DOCUMENT).expect("header");
    let via = hdr.getMainDocument().expect("header getMainDocument");
    let via_sd = via
        .asAny()
        .downcast_ref::<SubDocument>()
        .expect("downcast");
    assert!(std::ptr::eq(via_sd as *const SubDocument, main));
    assert!(!std::ptr::eq(via_sd, hdr));
}

#[test]
fn PA6_get_styles_same_as_parent() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert_eq!(sd.getStyles(), styles_ptr(&td));
}

#[test]
fn PA7_seven_slots_share_parent() {
    let td = TextDocument::new();
    let expected = td.as_ref() as *const TextDocument as *mut TextDocument;
    for i in 0..7 {
        let sd = td.getSubdocument(i).unwrap_or_else(|| panic!("slot {i}"));
        assert!(
            std::ptr::eq(sd.getTextDocument(), expected),
            "slot {i} parent"
        );
    }
}

#[test]
fn PA8_create_subdocument_keeps_parent() {
    let mut td = TextDocument::new();
    let before = td
        .getSubdocument(MAIN_DOCUMENT)
        .expect("main")
        .getTextDocument();
    let after = td
        .createSubDocument(MAIN_DOCUMENT)
        .expect("createSubDocument(0)")
        .getTextDocument();
    assert!(std::ptr::eq(before, after));
}

#[test]
fn PA9_mock_has_no_main() {
    let td = TextDocument::TextDocument__boolean(true);
    assert!(td.getMainDocument().is_none());
    assert!(td.getSubdocument(0).is_none());
}

#[test]
fn PA10_pipeline_hello_world_parent_alias() {
    let path = hello_world();
    let digest = sha256_file(&path);
    assert_eq!(
        digest.len(),
        64,
        "sha256 of hello-world.docx: {digest}"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let td = open_text_document(&bytes).expect("open_text_document");
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("parsed main");
    assert!(
        Document::getTextDocument(sd).is_some(),
        "product parse getTextDocument"
    );
    let main = td.getMainDocument().expect("main after parse");
    assert!(std::ptr::eq(
        main.getTextDocument(),
        td.as_ref() as *const TextDocument as *mut TextDocument
    ));
}
