// function_id=05_function_document_fields
// cargo test --manifest-path libs/word-core/Cargo.toml --test 05_function_document_fields

use std::path::PathBuf;
use std::process::Command;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::open_text_document;
use word_core::cn::wps::moffice::writer::core::Document::{Document, MAIN_DOCUMENT};
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
fn DF1_ctor_installs_document_fields() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getDocumentFileds().is_some());
}

#[test]
fn DF2_create_idempotent() {
    let mut td = TextDocument::new();
    let sd = td.getSubdocumentMut(MAIN_DOCUMENT).expect("main");
    let a = sd.getDocumentFileds().map(|p| p as *const _ as usize);
    sd.createPLCOfDocumentFiels();
    let b = sd.getDocumentFileds().map(|p| p as *const _ as usize);
    assert_eq!(a, b);
}

#[test]
fn DF3_new_fields_are_empty() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getDocumentFileds().expect("fields").isEmpty());
}

#[test]
fn DF4_trait_same_as_inherent() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let inh = sd.getDocumentFileds().expect("inh");
    let via = Document::getDocumentFileds(sd).expect("trait");
    assert!(std::ptr::eq(via, inh as *const _ as *mut _));
}

#[test]
fn DF5_has_doc_field_false_on_empty() {
    let td = TextDocument::new();
    assert!(!td.hasDocField());
}

#[test]
fn DF6_pipeline_hello_world_fields() {
    let path = hello_world();
    assert_eq!(
        sha256_file(&path),
        "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let td = open_text_document(&bytes).expect("open_text_document");
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("parsed main");
    assert!(ptr_ok(Document::getDocumentFileds(sd)));
}
