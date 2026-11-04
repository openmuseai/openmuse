use util::moffice::property::PropertySet::empty;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::data::ITextStream::ITextStream;
use word_core::cn::wps::moffice::writer::data::KTextStream::KTextStream;
use word_core::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;

#[test]
fn java_ktextstream_get_text_uses_document_rope_and_rejects_invalid_range() {
    let mut owned = TextDocument::new();
    owned.createSubDocument(0);
    let main = owned.getSubdocumentMut(0).expect("main document");
    CoreHelper::appendText__string_ps(Some(main as &mut dyn Document), Some("甲乙\r"), Some(&*empty));
    let mut stream = KTextStream::KTextStream(main as &dyn Document);
    let mut output = ['\0'; 4];

    assert_eq!(ITextStream::getText(&mut stream, 0, 3, &mut output, 1), 3);
    assert_eq!(&output[1..4], &['甲', '乙', '\r']);
    stream.checkIndex__int(0);
    stream.checkIndex__int_int(-10, 100); // Java's two-argument method is empty.
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        stream.checkIndex__int(3)
    })).is_err());
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ITextStream::getText(&mut stream, -1, 2, &mut output, 0)
    })).is_err());
}

#[test]
fn java_ktextstream_splits_and_merges_equal_chpx_within_one_paragraph() {
    let mut owned = TextDocument::new();
    owned.createSubDocument(0);
    let main = owned.getSubdocumentMut(0).expect("main document");
    CoreHelper::appendText__string_ps(Some(main as &mut dyn Document), Some("ab\r"), Some(&*empty));
    CoreHelper::insertToPapxTable(Some(main as &mut dyn Document), 0, Some(&*empty));
    let stream = KTextStream::KTextStream(main as &dyn Document);

    assert_eq!(stream.split_chpx(1).position(), 1);
    assert_eq!(stream.seekChpxTable(1).expect("split entry").position(), 1);
    stream.merge_chpx(1);
    assert_eq!(stream.seekChpxTable(1).expect("merged entry").position(), 0);
}

#[test]
fn java_ktextstream_keeps_chpx_boundary_after_paragraph_mark() {
    let mut owned = TextDocument::new();
    owned.createSubDocument(0);
    let main = owned.getSubdocumentMut(0).expect("main document");
    CoreHelper::appendText__string_ps(Some(main as &mut dyn Document), Some("a\rb\r"), Some(&*empty));
    CoreHelper::insertToPapxTable(Some(main as &mut dyn Document), 0, Some(&*empty));
    CoreHelper::insertToPapxTable(Some(main as &mut dyn Document), 2, Some(&*empty));
    let stream = KTextStream::KTextStream(main as &dyn Document);

    assert_eq!(stream.split_chpx(1).position(), 1);
    assert_eq!(stream.split_chpx(2).position(), 2);
    stream.merge_chpx(2);
    assert_eq!(stream.seekChpxTable(2).expect("paragraph boundary").position(), 2);
}
