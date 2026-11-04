// function_id=10_function_autowordstat
// cargo test --manifest-path libs/word-core/Cargo.toml --test 10_function_autowordstat

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
fn AWS1_main_story_installs_autowordstat() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getAutoWordStat().is_some());
}

#[test]
fn AWS2_header_story_has_no_autowordstat() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(HEADER_DOCUMENT).expect("header");
    assert!(sd.getAutoWordStat().is_none());
}

#[test]
fn AWS3_trait_same_as_inherent() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let inh = sd.getAutoWordStat().expect("inh");
    let via = Document::getAutoWordStat(sd).expect("trait");
    assert!(std::ptr::eq(via, inh as *const _ as *mut _));
}

#[test]
fn AWS4_stream_listener_wired_on_main() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let stream = sd.getTextStream().expect("stream");
    assert!(stream.getListener().map(|p| !p.is_null()).unwrap_or(false));
}

#[test]
fn AWS5_core_is_some() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getAutoWordStat().expect("stat").core_is_some());
}

#[test]
fn AWS6_pipeline_hello_world_autowordstat() {
    let path = hello_world();
    assert_eq!(
        sha256_file(&path),
        "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let td = open_text_document(&bytes).expect("open_text_document");
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("parsed main");
    assert!(ptr_ok(Document::getAutoWordStat(sd)));
}
