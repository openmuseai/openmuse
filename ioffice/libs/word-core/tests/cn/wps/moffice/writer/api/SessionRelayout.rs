// function_id=04_function_session_relayout
// cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_session_relayout

use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::DocumentSession::DocumentSession;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{FontBlob, ViewParams};

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

/// SR1 — layout after open paints Hello (no mutate).
#[test]
fn sr1_open_layout_paints_hello() {
    let session = DocumentSession::open(&plain_hello_docx()).expect("open");
    let frame = session
        .layout(&[mvp_font()], &ViewParams::default_a4())
        .expect("layout");
    let texts: Vec<&str> = frame
        .text_commands()
        .iter()
        .map(|t| t.text.as_str())
        .collect();
    assert!(
        texts.iter().any(|t| t.contains("Hello")),
        "hello painted: {texts:?}"
    );
}

/// SR2 — append "AAA" then layout paints Hello and AAA (MvpLatin cmap).
#[test]
fn sr2_append_relayouts_aaa() {
    let mut session = DocumentSession::open(&plain_hello_docx()).expect("open");
    assert!(session.main_length() > 0);
    session.append_paragraph("AAA").expect("append");
    let frame = session
        .layout(&[mvp_font()], &ViewParams::default_a4())
        .expect("relayout");
    let texts: Vec<&str> = frame
        .text_commands()
        .iter()
        .map(|t| t.text.as_str())
        .collect();
    assert!(
        texts.iter().any(|t| t.contains("Hello")),
        "hello still painted: {texts:?}"
    );
    assert!(
        texts.iter().any(|t| t.contains("AAA")),
        "appended paragraph painted: {texts:?}"
    );
}

/// SR3 — layout twice without mutate: same visible strings.
#[test]
fn sr3_layout_idempotent_strings() {
    let session = DocumentSession::open(&plain_hello_docx()).expect("open");
    let fonts = [mvp_font()];
    let view = ViewParams::default_a4();
    let a = session.layout(&fonts, &view).expect("1");
    let b = session.layout(&fonts, &view).expect("2");
    let ta: Vec<&str> = a.text_commands().iter().map(|t| t.text.as_str()).collect();
    let tb: Vec<&str> = b.text_commands().iter().map(|t| t.text.as_str()).collect();
    assert_eq!(ta, tb);
}
