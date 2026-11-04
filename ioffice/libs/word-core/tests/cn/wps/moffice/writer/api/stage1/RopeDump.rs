// function_id=01_function_rope_dump
// cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_rope_dump

use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::open_text_document;
use word_core::cn::wps::moffice::writer::api::stage1::RopeDump::RopeDump;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn hello_docx() -> Vec<u8> {
    std::fs::read(
        crate_root().join("tests/fixtures/01_function_uniffi_export/plain-one-para.docx"),
    )
    .expect("plain-one-para.docx")
}

fn two_para_docx() -> Vec<u8> {
    std::fs::read(crate_root().join("tests/fixtures/p1_layout_document/two-para.docx"))
        .expect("two-para.docx")
}

fn sample2_docx() -> Vec<u8> {
    std::fs::read(
        crate_root().join("../../samples/docx-mvp-render/flutter/assets/sample2.docx"),
    )
    .expect("sample2.docx")
}

/// RD1 — product open of hello.docx dumps rope containing Hello (not zip-extract empty).
#[test]
fn rd1_hello_contains_hello() {
    let doc = open_text_document(&hello_docx()).expect("open hello");
    let text = RopeDump::dump(&doc);
    assert!(
        text.contains("Hello"),
        "expected Hello in rope, got {text:?}"
    );
}

/// RD2 — two paragraph marks survive as `\\r` (not concatenated w:t).
#[test]
fn rd2_two_para_keeps_cr() {
    let doc = open_text_document(&two_para_docx()).expect("open two-para");
    let text = RopeDump::dump(&doc);
    assert!(
        text.contains('\r'),
        "expected paragraph mark \\\\r in rope, got {text:?}"
    );
    let hello_n = text.matches("Hello").count();
    assert!(
        hello_n >= 2,
        "expected two Hello runs in rope, got {hello_n} text={text:?}"
    );
}

/// RD3 — `_create` with no append: main exists, length 0, dump is empty (no panic).
#[test]
fn rd3_empty_main_is_empty_string() {
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    let text = RopeDump::dump(&doc);
    assert_eq!(text, "", "empty main story must dump empty, got {text:?}");
    let main = doc.getMainDocument().expect("main after _create");
    assert_eq!(main.getLength(), 0);
}

/// RD4 — CJK sample2 rope contains body text (open failure must not become "").
#[test]
fn rd4_sample2_contains_cjk() {
    let doc = open_text_document(&sample2_docx()).expect("open sample2");
    let text = RopeDump::dump(&doc);
    assert!(
        !text.is_empty(),
        "sample2 open must fill rope, not dump empty"
    );
    assert!(
        text.contains("大学生") || text.contains("概述"),
        "expected CJK from sample2 rope, got {text:?}"
    );
}

/// RD5 — bMock ctor without `_create`: no main document → dump "".
#[test]
fn rd5_no_main_is_empty_string() {
    let doc = TextDocument::TextDocument__boolean(true);
    let text = RopeDump::dump(&doc);
    assert_eq!(text, "");
}

/// RD6 — dump length equals main `getLength()` (CpText clamp / full range).
#[test]
fn rd6_dump_len_matches_get_length() {
    let doc = open_text_document(&hello_docx()).expect("open hello");
    let text = RopeDump::dump(&doc);
    let main = doc.getMainDocument().expect("main");
    assert_eq!(
        text.chars().count() as i32,
        main.getLength(),
        "dump char count must match rope length"
    );
}
