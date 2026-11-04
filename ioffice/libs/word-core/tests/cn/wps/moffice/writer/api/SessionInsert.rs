// function_id=01_function_event_insert
// cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_event_insert

use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::DocumentSession::DocumentSession;
use word_core::cn::wps::moffice::writer::api::SessionEdit::SessionEdit;
use word_core::cn::wps::moffice::writer::core::DocumentEvent::EVENT_INSERT;

fn plain_hello_docx() -> Vec<u8> {
    std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/01_function_uniffi_export/plain-one-para.docx"),
    )
    .expect("plain-one-para.docx")
}

fn open_hello() -> DocumentSession {
    DocumentSession::open(&plain_hello_docx()).expect("open")
}

/// EI1 — insert "Z" at cp 0 grows main by 1; event type INSERT.
#[test]
fn ei1_insert_at_start() {
    let mut session = open_hello();
    let before = session.main_length();
    let ev = SessionEdit::insert(&mut session, 0, "Z").expect("insert");
    assert_eq!(ev.getType(), EVENT_INSERT);
    assert_eq!(ev.getOffset(), 0);
    assert_eq!(ev.getLength(), 1);
    assert_eq!(session.main_length(), before + 1);
}

/// EI2 — insert at end (cp == length) grows by scalar count.
#[test]
fn ei2_insert_at_end() {
    let mut session = open_hello();
    let before = session.main_length();
    let ev = SessionEdit::insert(&mut session, before, "AA").expect("insert");
    assert_eq!(ev.getLength(), 2);
    assert_eq!(session.main_length(), before + 2);
}

/// EI3 — CJK one scalar +1, not UTF-8 bytes.
#[test]
fn ei3_cjk_scalar() {
    let mut session = open_hello();
    let before = session.main_length();
    SessionEdit::insert(&mut session, 0, "字").expect("han");
    assert_eq!(session.main_length(), before + 1);
}

/// EI4 — empty text is INSERT length 0; storage unchanged.
#[test]
fn ei4_empty_noop() {
    let mut session = open_hello();
    let before = session.main_length();
    let ev = SessionEdit::insert(&mut session, 0, "").expect("empty");
    assert_eq!(ev.getLength(), 0);
    assert_eq!(session.main_length(), before);
}

/// EI5 — cp past length is error.
#[test]
fn ei5_cp_past_len() {
    let mut session = open_hello();
    let before = session.main_length();
    match SessionEdit::insert(&mut session, before + 1, "X") {
        Err(_) => {}
        Ok(_) => panic!("expected out of range"),
    }
    assert_eq!(session.main_length(), before);
}

/// EI6 — negative cp is error.
#[test]
fn ei6_negative_cp() {
    let mut session = open_hello();
    match SessionEdit::insert(&mut session, -1, "X") {
        Err(_) => {}
        Ok(_) => panic!("expected out of range"),
    }
}
