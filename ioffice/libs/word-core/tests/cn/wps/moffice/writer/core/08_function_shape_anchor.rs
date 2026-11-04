// function_id=08_function_shape_anchor
// cargo test --manifest-path libs/word-core/Cargo.toml --test 08_function_shape_anchor

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
fn SA1_ctor_installs_shape_anchors() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getShapeAnchors().is_some());
}

#[test]
fn SA2_create_still_some() {
    let mut td = TextDocument::new();
    let sd = td.getSubdocumentMut(MAIN_DOCUMENT).expect("main");
    assert!(sd.getShapeAnchors().is_some());
    sd.createPLC();
    assert!(sd.getShapeAnchors().is_some());
}

#[test]
fn SA3_new_anchors_are_empty() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getShapeAnchors().expect("anchors").isEmpty());
}

#[test]
fn SA4_trait_same_as_inherent() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let inh = sd.getShapeAnchors().expect("inh");
    let via = Document::getShapeAnchors(sd).expect("trait");
    assert!(std::ptr::eq(via, inh as *const _ as *mut _));
}

#[test]
fn SA5_header_story_also_gets_anchors() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(HEADER_DOCUMENT).expect("header");
    assert!(sd.getShapeAnchors().is_some());
}

#[test]
fn SA6_pipeline_hello_world_anchors() {
    let path = hello_world();
    assert_eq!(
        sha256_file(&path),
        "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let td = open_text_document(&bytes).expect("open_text_document");
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("parsed main");
    assert!(ptr_ok(Document::getShapeAnchors(sd)));
}
