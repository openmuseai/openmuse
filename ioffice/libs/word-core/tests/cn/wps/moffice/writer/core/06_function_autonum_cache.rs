// function_id=06_function_autonum_cache
// cargo test --manifest-path libs/word-core/Cargo.toml --test 06_function_autonum_cache

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
fn AN1_ctor_installs_autonum_count_and_field() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getAutoNumCount().is_some());
    assert!(sd.getAutoNumField().is_some());
}

#[test]
fn AN2_getters_stable_same_instance() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let c1 = sd.getAutoNumCount().map(|p| p as *const _ as usize);
    let c2 = sd.getAutoNumCount().map(|p| p as *const _ as usize);
    let f1 = sd.getAutoNumField().map(|p| p as *const _ as usize);
    let f2 = sd.getAutoNumField().map(|p| p as *const _ as usize);
    assert_eq!(c1, c2);
    assert_eq!(f1, f2);
}

#[test]
fn AN3_trait_same_as_inherent_count() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let inh = sd.getAutoNumCount().expect("inh");
    let via = Document::getAutoNumCount(sd).expect("trait");
    assert!(std::ptr::eq(via, inh as *const _ as *mut _));
}

#[test]
fn AN4_trait_same_as_inherent_field() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let inh = sd.getAutoNumField().expect("inh");
    let via = Document::getAutoNumField(sd).expect("trait");
    assert!(std::ptr::eq(via, inh as *const _ as *mut _));
}

#[test]
fn AN5_header_story_also_gets_caches() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(HEADER_DOCUMENT).expect("header");
    assert!(sd.getAutoNumCount().is_some());
    assert!(sd.getAutoNumField().is_some());
}

#[test]
fn AN6_pipeline_hello_world_autonum() {
    let path = hello_world();
    assert_eq!(
        sha256_file(&path),
        "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let td = open_text_document(&bytes).expect("open_text_document");
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("parsed main");
    assert!(ptr_ok(Document::getAutoNumCount(sd)));
    assert!(ptr_ok(Document::getAutoNumField(sd)));
}
