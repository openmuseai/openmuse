// function_id=09_function_subdocument_own
// cargo test --manifest-path libs/word-core/Cargo.toml --test 09_function_subdocument_own
// src:  libs/word-core/src/cn/wps/moffice/writer/core/SubDocument.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/core/SubDocument.rs

use word_core::cn::wps::collections::textrope::ITextPool::ITextPool;
use word_core::cn::wps::collections::textrope::ITextRope::ITextRope;
use word_core::cn::wps::moffice::writer::core::Document::{
    FOOTNOTE_DOCUMENT, HEADER_DOCUMENT, MAIN_DOCUMENT,
};
use word_core::cn::wps::moffice::writer::core::KStyles::KStyles;
use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;

fn rope_pool(sd: &SubDocument) -> Option<*mut dyn ITextPool> {
    sd.getTextRope()
        .and_then(|rope| unsafe { (*rope).getTextPool() })
}

fn styles_ptr(td: &TextDocument) -> Option<*mut KStyles> {
    td.getStyles().map(|s| s as *const KStyles as *mut KStyles)
}

#[test]
fn SD1_main_type_is_zero() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert_eq!(sd.getType(), 0);
}

#[test]
fn SD2_empty_length_is_zero() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert_eq!(sd.getLength(), 0);
}

#[test]
fn SD3_owns_rope_chpx_papx_piecetable() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getTextRope().is_some(), "getTextRope");
    assert!(sd.getChpxTable().is_some(), "getChpxTable");
    assert!(sd.getPapxTable().is_some(), "getPapxTable");
    assert!(sd.getPieceTable().is_some(), "getPieceTable");
}

#[test]
fn SD4_main_has_plcsection() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    assert!(sd.getPLCSection().is_some(), "type 0 must own PLCSection");
}

#[test]
fn SD5_footnote_has_no_plcsection() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(FOOTNOTE_DOCUMENT).expect("footnote");
    assert!(
        sd.getPLCSection().is_none(),
        "type 1 must not assign mSections"
    );
}

#[test]
fn SD6_styles_delegate_to_parent() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(MAIN_DOCUMENT).expect("main");
    let from_parent = styles_ptr(&td);
    let from_sd = sd.getStyles();
    assert!(from_parent.is_some(), "parent styles");
    assert_eq!(from_sd, from_parent);
}

#[test]
fn SD7_two_types_share_parent_pool() {
    let td = TextDocument::new();
    let p0 = rope_pool(td.getSubdocument(MAIN_DOCUMENT).expect("main")).expect("main pool");
    let p1 = rope_pool(td.getSubdocument(FOOTNOTE_DOCUMENT).expect("footnote")).expect("footnote pool");
    assert!(std::ptr::eq(p0, p1), "shared parent ITextPool");
    assert!(
        std::ptr::eq(p0, td.getTextPool().expect("parent pool")),
        "rope pool is parent.getTextPool()"
    );
}

#[test]
fn SD8_header_type_is_two_without_plcsection() {
    let td = TextDocument::new();
    let sd = td.getSubdocument(HEADER_DOCUMENT).expect("header");
    assert_eq!(sd.getType(), 2);
    assert!(
        sd.getPLCSection().is_none(),
        "type 2 must not assign mSections"
    );
}
