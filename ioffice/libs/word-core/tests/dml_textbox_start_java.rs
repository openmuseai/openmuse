use word_core::cn::wps::moffice::writer::api::LayoutDocument::open_text_document;
use word_core::cn::wps::moffice::writer::layout::base::text::RunStream::RunStream;

#[test]
fn sample3_textbox_title_reaches_java_subdocument() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples/docx-mvp-render/flutter/assets/sample3/签到表格.docx");
    let bytes = std::fs::read(path).expect("sample3 DOCX");
    let document = open_text_document(&bytes).expect("DOCX parse");
    let textbox = document.getSubdocument(5).expect("Java main textbox subdocument");
    let text: String = RunStream::collectRuns(textbox, 0, textbox.getLength())
        .into_iter()
        .map(|run| run.text)
        .collect();
    assert!(text.contains("员工绩效考核表"), "textbox text: {text:?}");
}
