// function_id=10_function_textdocument_create
// cargo test --manifest-path libs/word-core/Cargo.toml --test 10_function_textdocument_create
// src:  libs/word-core/src/cn/wps/moffice/writer/core/TextDocument.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/core/TextDocument.rs

use word_core::cn::wps::collections::textrope::ITextPool::ITextPool;
use word_core::cn::wps::collections::textrope::ITextRope::ITextRope;
use word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;

fn rope_pool(sd: &SubDocument) -> Option<*mut dyn ITextPool> {
    sd.getTextRope()
        .and_then(|rope| unsafe { (*rope).getTextPool() })
}

#[test]
fn TD1_seven_subdocument_slots() {
    let td = TextDocument::new();
    for i in 0..7 {
        assert!(
            td.getSubdocument(i).is_some(),
            "getSubdocument({i}) must be Some after _create"
        );
    }
}

#[test]
fn TD2_main_document_is_slot_zero() {
    let td = TextDocument::new();
    let main = td.getMainDocument().expect("getMainDocument");
    assert_eq!(main.getType(), 0);
    let slot0 = td.getSubdocument(0).expect("slot 0");
    assert_eq!(main as *const SubDocument, slot0 as *const SubDocument);
}

#[test]
fn TD3_each_slot_type_matches_index() {
    let td = TextDocument::new();
    for i in 0..7 {
        let sd = td.getSubdocument(i).unwrap_or_else(|| panic!("slot {i}"));
        assert_eq!(sd.getType(), i, "slot {i} getType");
    }
}

#[test]
fn TD4_styles_non_null() {
    let td = TextDocument::new();
    assert!(td.getStyles().is_some(), "getStyles after _create");
}

#[test]
fn TD5_slot_zero_and_six_share_pool() {
    let td = TextDocument::new();
    let p0 = rope_pool(td.getSubdocument(0).expect("slot 0")).expect("slot 0 pool");
    let p6 = rope_pool(td.getSubdocument(6).expect("slot 6")).expect("slot 6 pool");
    let doc = td.getTextPool().expect("document pool");
    assert!(std::ptr::eq(p0, p6), "slot 0 and 6 must share the text pool");
    assert!(std::ptr::eq(p0, doc), "subdocument pool must be document getTextPool()");
}

#[test]
fn TD6_first_uusti_is_4097() {
    let mut td = TextDocument::new();
    let sti = td.getUUSti().expect("getUUSti").getUUSti();
    assert_eq!(sti, 4097);
}

#[test]
fn TD7_first_numbering_id_is_one() {
    let mut td = TextDocument::new();
    let id = td.getUUNumberingId().expect("getUUNumberingId").getNextId();
    assert_eq!(id, 1);
}

#[test]
fn TD8_create_subdocument_does_not_rebuild_main() {
    let mut td = TextDocument::new();
    let before = td.getSubdocument(0).expect("slot 0") as *const SubDocument;
    let after = td.createSubDocument(0).expect("createSubDocument(0)") as *const SubDocument;
    assert_eq!(before, after, "createSubDocument(0) must return the existing Main");
}

#[test]
fn TD9_mock_skips_create() {
    let td = TextDocument::TextDocument__boolean(true);
    assert!(td.getMainDocument().is_none(), "mock must not _create Main");
    assert!(td.getSubdocument(0).is_none(), "mock slot 0 must stay empty");
}
