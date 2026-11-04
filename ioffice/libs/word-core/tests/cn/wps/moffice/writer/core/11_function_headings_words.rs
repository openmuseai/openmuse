// function_id=11_function_headings_words
// cargo test --manifest-path libs/word-core/Cargo.toml --test 11_function_headings_words

use std::path::PathBuf;
use std::process::Command;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::open_text_document;
use word_core::cn::wps::moffice::writer::core::Document::{
    Document, HEADER_DOCUMENT, MAIN_DOCUMENT,
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
fn HW1_main_lazy_creates_headings() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getHeadings().is_some());
}

#[test]
fn HW2_header_does_not_create_headings() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(HEADER_DOCUMENT).expect("header");
    assert!(sd.getHeadings().is_none());
}

#[test]
fn HW3_plc_words_lazy_on_all_stories() {
    let td = TextDocument::new();
    let main = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let header = td.getSubdocument(HEADER_DOCUMENT).expect("header");
    assert!(main.getPLCWords().is_some());
    assert!(header.getPLCWords().is_some());
}

#[test]
fn HW4_headings_stable_same_instance() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let a = sd.getHeadings().expect("a");
    let b = sd.getHeadings().expect("b");
    assert!(std::ptr::eq(a, b));
}

#[test]
fn HW5_trait_same_as_inherent() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let inh = sd.getHeadings().expect("inh");
    let via = Document::getHeadings(sd).expect("trait");
    assert!(std::ptr::eq(via, inh as *const _ as *mut _));
    let inh_w = sd.getPLCWords().expect("inh w");
    let via_w = Document::getPLCWords(sd).expect("trait w");
    assert!(std::ptr::eq(via_w, inh_w as *const _ as *mut _));
}

#[test]
fn HW6_pipeline_hello_world_headings_words() {
    let path = hello_world();
    assert_eq!(
        sha256_file(&path),
        "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let td = open_text_document(&bytes).expect("open_text_document");
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("parsed main");
    assert!(ptr_ok(Document::getHeadings(sd)));
    assert!(ptr_ok(Document::getPLCWords(sd)));
}
