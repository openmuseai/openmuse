// function_id=07_function_text_stream
// cargo test --manifest-path libs/word-core/Cargo.toml --test 07_function_text_stream

use std::path::PathBuf;
use std::process::Command;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::open_text_document;
use word_core::cn::wps::moffice::writer::core::Document::{
    Document, HEADER_DOCUMENT, MAIN_DOCUMENT,
};
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::data::ITextStream::ITextStream;

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
fn TS1_ctor_installs_text_stream() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getTextStream().is_some());
}

#[test]
fn TS2_getter_stable_same_instance() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let a = sd.getTextStream().map(|p| p as *const dyn ITextStream as *const () as usize);
    let b = sd.getTextStream().map(|p| p as *const dyn ITextStream as *const () as usize);
    assert_eq!(a, b);
}

#[test]
fn TS3_trait_same_as_inherent() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let inh = sd.getTextStream().expect("inh") as *const dyn ITextStream;
    let via = Document::getTextStream(sd).expect("trait");
    assert!(std::ptr::eq(via, inh as *mut dyn ITextStream));
}

#[test]
fn TS4_get_length_matches_document_rope() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let stream = sd.getTextStream().expect("stream");
    assert_eq!(stream.getLength(), sd.getLength());
}

#[test]
fn TS5_header_story_also_gets_stream() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(HEADER_DOCUMENT).expect("header");
    assert!(sd.getTextStream().is_some());
}

#[test]
fn TS6_pipeline_hello_world_text_stream() {
    let path = hello_world();
    assert_eq!(
        sha256_file(&path),
        "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let td = open_text_document(&bytes).expect("open_text_document");
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("parsed main");
    assert!(ptr_ok(Document::getTextStream(sd)));
}
