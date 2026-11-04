// function_id=09_function_table_infos
// cargo test --manifest-path libs/word-core/Cargo.toml --test 09_function_table_infos

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
fn TI1_ctor_installs_table_infos() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getTableInfos().is_some());
}

#[test]
fn TI2_new_infos_size_zero() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert_eq!(sd.getTableInfos().expect("infos").size(), 0);
}

#[test]
fn TI3_trait_same_as_inherent() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let inh = sd.getTableInfos().expect("inh");
    let via = Document::getTableInfos(sd).expect("trait");
    assert!(std::ptr::eq(via, inh as *const _ as *mut _));
}

#[test]
fn TI4_header_story_also_gets_infos() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(HEADER_DOCUMENT).expect("header");
    assert!(sd.getTableInfos().is_some());
}

#[test]
fn TI5_getters_stable_same_instance() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let a = sd.getTableInfos().expect("a");
    let b = sd.getTableInfos().expect("b");
    assert!(std::ptr::eq(a, b));
}

#[test]
fn TI6_pipeline_hello_world_table_infos() {
    let path = hello_world();
    assert_eq!(
        sha256_file(&path),
        "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let td = open_text_document(&bytes).expect("open_text_document");
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("parsed main");
    assert!(ptr_ok(Document::getTableInfos(sd)));
}
