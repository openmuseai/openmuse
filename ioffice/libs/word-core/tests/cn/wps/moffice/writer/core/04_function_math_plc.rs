// function_id=04_function_math_plc
// cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_math_plc

use std::path::PathBuf;
use std::process::Command;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::open_text_document;
use word_core::cn::wps::moffice::writer::core::Document::{Document, MAIN_DOCUMENT};
use word_core::cn::wps::moffice::writer::core::RangeTool::RangeTool;
use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::data::RANGE::RANGE;

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

fn main_mut(td: &mut TextDocument) -> &mut SubDocument {
    td.getSubdocumentMut(MAIN_DOCUMENT).expect("main")
}

#[test]
fn MP1_create_omath_both_ends() {
    let mut td = TextDocument::new();
    let sd = main_mut(&mut td);
    assert!(sd.getOMathStarts().is_none());
    sd.createPLCOMath();
    assert!(sd.getOMathStarts().is_some());
    assert!(sd.getOMathEnds().is_some());
}

#[test]
fn MP2_create_omath_idempotent() {
    let mut td = TextDocument::new();
    let sd = main_mut(&mut td);
    sd.createPLCOMath();
    let a = sd.getOMathStarts().map(|p| p as *const _ as usize);
    sd.createPLCOMath();
    let b = sd.getOMathStarts().map(|p| p as *const _ as usize);
    assert_eq!(a, b);
}

#[test]
fn MP3_create_plc_math_function() {
    let mut td = TextDocument::new();
    let sd = main_mut(&mut td);
    assert!(sd.getPLCMathFunction().is_none());
    sd.createPLCMath();
    assert!(sd.getPLCMathFunction().is_some());
    let a = sd.getPLCMathFunction().map(|p| p as *const _ as usize);
    sd.createPLCMath();
    let b = sd.getPLCMathFunction().map(|p| p as *const _ as usize);
    assert_eq!(a, b);
}

#[test]
fn MP4_trait_getters_same_as_inherent() {
    let mut td = TextDocument::new();
    {
        let sd = main_mut(&mut td);
        sd.createPLCOMath();
        sd.createPLCMath();
    }
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let inh = td
        .getSubdocument(MAIN_DOCUMENT)
        .expect("main")
        .getOMathStarts()
        .expect("inh start");
    let via = Document::getOMathStarts(sd).expect("trait start");
    assert!(std::ptr::eq(via, inh as *const _ as *mut _));
    let inh_fn = td
        .getSubdocument(MAIN_DOCUMENT)
        .expect("main")
        .getPLCMathFunction()
        .expect("inh fn");
    let via_fn = Document::getPLCMathFunction(sd).expect("trait fn");
    assert!(std::ptr::eq(via_fn, inh_fn as *const _ as *mut _));
}

#[test]
fn MP5_contain_math_false_when_empty_plc() {
    let mut td = TextDocument::new();
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(!Document::isContainMathEquations(sd));
    assert!(Document::getOMathStarts(sd).is_none());
    let sd = main_mut(&mut td);
    sd.createPLCOMath();
    assert!(
        !sd.isContainMathEquations(),
        "Java: empty start PLC ⇒ !isEmpty is false"
    );
}

#[test]
fn MP6_rangetool_no_math_function() {
    let td = TextDocument::new();
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let mut rg = RANGE::default();
    assert!(!RangeTool::expandToSelectMathFunction(sd, 0, false, &mut rg));
}

#[test]
fn MP7_pipeline_create_after_parse() {
    let path = hello_world();
    assert_eq!(
        sha256_file(&path),
        "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e"
    );
    let bytes = std::fs::read(&path).expect("hello-world.docx");
    let mut td = open_text_document(&bytes).expect("open_text_document");
    {
        let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("parsed main");
        assert!(Document::getOMathStarts(sd).is_none());
    }
    td.getSubdocumentMut(MAIN_DOCUMENT)
        .expect("main mut")
        .createPLCOMath();
    let sd: &dyn Document = td.getSubdocument(MAIN_DOCUMENT).expect("parsed main");
    assert!(ptr_ok(Document::getOMathStarts(sd)));
    assert!(ptr_ok(Document::getOMathEnds(sd)));
}
