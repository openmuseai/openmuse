#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
use word_core::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory;

#[test]
fn sample2_default_paragraph_style_reaches_final_papx() {
    let bytes = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../samples/docx-mvp-render/flutter/assets/sample1/sample2.docx"
    )).unwrap();
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    DocumentOpen::fill_text_document_from_docx(&mut doc, &bytes, "").unwrap();
    let main = doc.getMainDocument().unwrap();
    let table = main.getPapxTable().unwrap();
    let entry = unsafe { IPapxTable::seek(&*table, 23) }.unwrap();
    let direct = word_core::cn::wps::moffice::writer::data::IPapxTable::Entry::getProperty(entry.as_ref()).unwrap();
    assert!(!direct.contains(193), "body has no direct w:jc");
    let mut factory = PropertyToolFactory::default();
    let mut tool = factory.create(main);
    let final_prop = tool.getFinalParaProp__entry_boolean(entry.as_ref(), true);
    assert_eq!(final_prop.getInt__int_int(193, 0), 3, "Normal style w:jc=both must be inherited");
}
