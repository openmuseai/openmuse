// function_id=02_function_event_remove
// cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_event_remove

use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::DocumentSession::DocumentSession;
use word_core::cn::wps::moffice::writer::api::SessionEdit::SessionEdit;
use word_core::cn::wps::moffice::writer::core::DocumentEvent::EVENT_REMOVE;

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

/// ER1 — erase [0,1) shortens by 1; type REMOVE.
#[test]
fn er1_erase_first_cp() {
    let mut session = open_hello();
    let before = session.main_length();
    assert!(before >= 2);
    let ev = SessionEdit::remove(&mut session, 0, 1).expect("remove");
    assert_eq!(ev.getType(), EVENT_REMOVE);
    assert_eq!(ev.getOffset(), 0);
    assert_eq!(ev.getLength(), 1);
    assert_eq!(session.main_length(), before - 1);
}

/// ER2 — erase two cps.
#[test]
fn er2_erase_two() {
    let mut session = open_hello();
    let before = session.main_length();
    SessionEdit::remove(&mut session, 0, 2).expect("remove");
    assert_eq!(session.main_length(), before - 2);
}

/// ER3 — empty interval is REMOVE length 0.
#[test]
fn er3_empty_interval() {
    let mut session = open_hello();
    let before = session.main_length();
    let ev = SessionEdit::remove(&mut session, 1, 1).expect("empty");
    assert_eq!(ev.getLength(), 0);
    assert_eq!(session.main_length(), before);
}

/// ER4 — inverted start>end treated as empty (start >= end).
#[test]
fn er4_inverted_is_empty() {
    let mut session = open_hello();
    let before = session.main_length();
    let ev = SessionEdit::remove(&mut session, 3, 1).expect("inverted");
    assert_eq!(ev.getLength(), 0);
    assert_eq!(session.main_length(), before);
}

/// ER5 — end past length is error.
#[test]
fn er5_end_past_len() {
    let mut session = open_hello();
    let before = session.main_length();
    match SessionEdit::remove(&mut session, 0, before + 1) {
        Err(_) => {}
        Ok(_) => panic!("expected out of bounds"),
    }
}

/// ER6 — negative start is error.
#[test]
fn er6_negative_start() {
    let mut session = open_hello();
    match SessionEdit::remove(&mut session, -1, 1) {
        Err(_) => {}
        Ok(_) => panic!("expected out of bounds"),
    }
}
