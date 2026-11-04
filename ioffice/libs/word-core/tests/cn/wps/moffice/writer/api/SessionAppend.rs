// function_id=02_function_session_append
// cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_session_append

use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::DocumentSession::DocumentSession;

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

/// SA1 — append "AAA" grows main by 4 codepoints (3 letters + `\r`).
#[test]
fn sa1_append_aaa_grows_by_four() {
    let mut session = open_hello();
    let before = session.main_length();
    session.append_paragraph("AAA").expect("append");
    assert_eq!(session.main_length(), before + 4);
}

/// SA2 — empty input still inserts a paragraph mark (`\r` only).
#[test]
fn sa2_empty_text_inserts_cr_only() {
    let mut session = open_hello();
    let before = session.main_length();
    session.append_paragraph("").expect("append empty");
    assert_eq!(session.main_length(), before + 1);
}

/// SA3 — CJK scalar count, not UTF-8 bytes (one Han + `\r` → +2).
#[test]
fn sa3_cjk_counts_scalars() {
    let mut session = open_hello();
    let before = session.main_length();
    session.append_paragraph("字").expect("append han");
    assert_eq!(session.main_length(), before + 2);
}

/// SA4 — twice: lengths add.
#[test]
fn sa4_append_twice_accumulates() {
    let mut session = open_hello();
    let before = session.main_length();
    session.append_paragraph("A").expect("1");
    session.append_paragraph("B").expect("2");
    assert_eq!(session.main_length(), before + 4);
}

/// SA5 — append does not require layout(); length is storage-side.
#[test]
fn sa5_mutate_without_layout() {
    let mut session = open_hello();
    session.append_paragraph("Z").expect("append");
    assert!(session.main_length() > 0);
}
