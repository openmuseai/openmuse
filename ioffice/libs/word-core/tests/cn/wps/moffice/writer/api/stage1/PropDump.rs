// function_id=02_function_prop_dump
// cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_prop_dump

use std::path::PathBuf;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::empty;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::open_text_document;
use word_core::cn::wps::moffice::writer::api::stage1::PropDump::{PropDump, Stage1Props};
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn hello_docx() -> Vec<u8> {
    std::fs::read(
        crate_root().join("tests/fixtures/01_function_uniffi_export/plain-one-para.docx"),
    )
    .expect("plain-one-para.docx")
}

fn sample2_docx() -> Vec<u8> {
    std::fs::read(
        crate_root().join("../../samples/docx-mvp-render/flutter/assets/sample2.docx"),
    )
    .expect("sample2.docx")
}

// rust: ★P0 —— 返回 `Box<TextDocument>`：按值返回会把堆对象**移动**出 helper，
//   使 `SubDocument.mParent` / Chpx·Papx 表的 `document` 裸指针悬垂（同 `PapxParas`）。
fn with_main_papx(papx: &util::moffice::property::PropertySet::PropertySet) -> Box<TextDocument> {
    let mut owned = TextDocument::new();
    owned.createSubDocument(0);
    {
        let main = owned.getSubdocumentMut(0).expect("main");
        CoreHelper::appendParagraph__char_ps_ps(Some(main), '\r', Some(&*empty), Some(papx));
    }
    owned
}

/// PD1 — in-memory PAPX 193=1 dumps Some(1), not default-missing.
#[test]
fn pd1_jc_contains_one() {
    let mut m = MutablePropertySet::new();
    m.setInt(193, 1);
    let ps = m.freeze();
    let doc = with_main_papx(&ps);
    let dump = PropDump::dump(&doc);
    assert!(!dump.papx.is_empty(), "expected a PAPX span");
    assert_eq!(dump.papx[0].p193, Some(1));
}

/// PD2 — missing 193 is None, never Some(0).
#[test]
fn pd2_missing_jc_is_null() {
    let doc = with_main_papx(&empty);
    let dump = PropDump::dump(&doc);
    assert!(!dump.papx.is_empty());
    assert_eq!(dump.papx[0].p193, None, "must not invent jc=0");
}

/// PD3 — CHPX sz 18pt dumps 360 twip.
#[test]
fn pd3_sz_twip() {
    let mut owned = TextDocument::new();
    owned.createSubDocument(0);
    {
        let main = owned.getSubdocumentMut(0).expect("main");
        let mut chpx = MutablePropertySet::new();
        chpx.setFloat(10, 18.0);
        let frozen = chpx.freeze();
        CoreHelper::appendParagraph__char_ps_ps(Some(main), '\r', Some(&frozen), Some(&*empty));
    }
    let dump = PropDump::dump(&owned);
    assert!(
        dump.chpx.iter().any(|c| c.sz_twip == Some(360)),
        "expected sz 360 twip, got {:?}",
        dump.chpx
    );
}

/// PD4 — in-memory firstLine 208 dumps Some, hello.docx has no 208.
#[test]
fn pd4_indent_contains_vs_missing() {
    let mut m = MutablePropertySet::new();
    m.setInt(208, 480);
    let ps = m.freeze();
    let with = PropDump::dump(&with_main_papx(&ps));
    assert_eq!(with.papx[0].p208, Some(480));

    let hello = open_text_document(&hello_docx()).expect("open hello");
    let dump = PropDump::dump(&hello);
    for p in &dump.papx {
        assert_eq!(p.p208, None, "hello must not invent firstLine");
    }
}

/// PD5 — product sample2: at least one PAPX has 193=1 (title jc).
#[test]
fn pd5_sample2_title_jc() {
    let doc = open_text_document(&sample2_docx()).expect("open sample2");
    let dump = PropDump::dump(&doc);
    assert!(
        dump.papx.iter().any(|p| p.p193 == Some(1)),
        "sample2 title jc missing; papx={:?}",
        dump.papx
    );
}

/// PD6 — empty document (no main fill) dumps empty lists.
#[test]
fn pd6_empty_doc() {
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    let dump = PropDump::dump(&doc);
    assert_eq!(dump, Stage1Props::default());
}
