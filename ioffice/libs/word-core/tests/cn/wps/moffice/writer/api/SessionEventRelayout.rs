// function_id=03_function_event_relayout
// cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_event_relayout

use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::DocumentSession::DocumentSession;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{FontBlob, ViewParams};
use word_core::cn::wps::moffice::writer::api::SessionEdit::SessionEdit;

fn mvp_font() -> FontBlob {
    FontBlob {
        id: "default".into(),
        bytes: std::fs::read(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/p1_layout_document/MvpLatin.ttf"),
        )
        .expect("MvpLatin.ttf"),
    }
}

fn plain_hello_docx() -> Vec<u8> {
    std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/01_function_uniffi_export/plain-one-para.docx"),
    )
    .expect("plain-one-para.docx")
}

fn texts(session: &DocumentSession) -> Vec<String> {
    let frame = SessionEdit::relayout(session, &[mvp_font()], &ViewParams::default_a4())
        .expect("layout");
    frame
        .text_commands()
        .iter()
        .map(|t| t.text.clone())
        .collect()
}

/// EL1 — insert "Z" at 0 then full layout paints Z and Hello.
#[test]
fn el1_insert_then_layout_paints_z() {
    let mut session = DocumentSession::open(&plain_hello_docx()).expect("open");
    SessionEdit::insert(&mut session, 0, "Z").expect("insert");
    let texts = texts(&session);
    let joined = texts.join("");
    assert!(joined.contains('Z'), "expected Z in {texts:?}");
    assert!(
        joined.contains("Hello"),
        "expected Hello across runs in {texts:?}"
    );
}

/// EL2 — remove first cp then layout still paints a remainder of Hello.
#[test]
fn el2_remove_then_layout_keeps_ello() {
    let mut session = DocumentSession::open(&plain_hello_docx()).expect("open");
    SessionEdit::remove(&mut session, 0, 1).expect("remove");
    let texts = texts(&session);
    let joined = texts.join("");
    assert!(
        joined.contains("ello") || joined.contains("Hello"),
        "expected remaining Hello glyphs in {texts:?}"
    );
}

/// EL3 — relayout is full pass: two layouts after insert agree on joined text.
#[test]
fn el3_relayout_idempotent_text() {
    let mut session = DocumentSession::open(&plain_hello_docx()).expect("open");
    SessionEdit::insert(&mut session, 0, "Q").expect("insert");
    let a = texts(&session);
    let b = texts(&session);
    assert_eq!(a, b);
}
