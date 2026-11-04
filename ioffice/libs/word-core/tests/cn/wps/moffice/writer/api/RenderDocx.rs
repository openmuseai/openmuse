use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::RenderDocx::render_docx;
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::RenderCommand;

fn fixture() -> Vec<u8> {
    std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/01_function_uniffi_export/plain-one-para.docx"),
    )
    .expect("fixture docx")
}

#[test]
fn ex1_plain_has_draw_text() {
    let doc = render_docx(&fixture()).expect("render");
    let n = doc
        .commands
        .iter()
        .filter(|c| matches!(c, RenderCommand::Text(_)))
        .count();
    assert!(n >= 1, "expected DrawText, got {}", doc.commands.len());
}

#[test]
fn ex2_bad_bytes_err() {
    match render_docx(b"not-a-zip") {
        Err(_) => {}
        Ok(_) => panic!("must fail"),
    }
}
