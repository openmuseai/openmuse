// function_id=01_function_session_open
// cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_session_open

use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::DocumentSession::DocumentSession;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::LayoutErrorKind;

fn plain_hello_docx() -> Vec<u8> {
    std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/01_function_uniffi_export/plain-one-para.docx"),
    )
    .expect("plain-one-para.docx")
}

/// SO1 — legal deflate docx → heap session, main story length > 0.
#[test]
fn so1_open_hello_has_main_length() {
    let session = DocumentSession::open(&plain_hello_docx()).expect("open");
    assert!(session.main_length() > 0, "expected imported main story");
}

/// SO2 — two opens are independent (no shared Box).
#[test]
fn so2_two_opens_are_independent() {
    let a = DocumentSession::open(&plain_hello_docx()).expect("a");
    let b = DocumentSession::open(&plain_hello_docx()).expect("b");
    assert_eq!(a.main_length(), b.main_length());
    assert!(!std::ptr::eq(a.document(), b.document()));
}

/// SO3 — not PK zip → Zip error; no session.
#[test]
fn so3_not_zip_is_zip_error() {
    let err = match DocumentSession::open(b"not-a-docx") {
        Err(e) => e,
        Ok(_) => panic!("expected zip error"),
    };
    assert_eq!(err.kind, LayoutErrorKind::Zip);
}

/// SO4 — PK magic but truncated archive → Zip error.
#[test]
fn so4_truncated_zip_is_zip_error() {
    let err = match DocumentSession::open(b"PK\x03\x04") {
        Err(e) => e,
        Ok(_) => panic!("expected zip error"),
    };
    assert_eq!(err.kind, LayoutErrorKind::Zip);
}

/// SO5 — empty bytes → Zip error.
#[test]
fn so5_empty_bytes_is_zip_error() {
    let err = match DocumentSession::open(b"") {
        Err(e) => e,
        Ok(_) => panic!("expected zip error"),
    };
    assert_eq!(err.kind, LayoutErrorKind::Zip);
}
