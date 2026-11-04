use word_core::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use word_core::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use word_core::cn::wps::moffice::writer::cache::TypoTextBox::TypoTextBox;

#[test]
fn drawing_textbox_link_preserves_page_parent_and_sets_textbox_parent() {
    let mut snapshot = TypoSnapshot::new();
    let page = snapshot.alloc(19);
    let mut drawing = TypoDrawing::new();
    drawing.alloc(&mut snapshot);
    drawing.setParent(page);
    let mut textbox = TypoTextBox::new();
    textbox.alloc(&mut snapshot);

    drawing.setTextbox(textbox.getPointer());

    assert_eq!(snapshot.get__int(drawing.getPointer() + 22), textbox.getPointer());
    assert_eq!(snapshot.get__int(textbox.getPointer() + 2), drawing.getPointer());
    assert_eq!(snapshot.get__int(drawing.getPointer() + 2), page);
}
