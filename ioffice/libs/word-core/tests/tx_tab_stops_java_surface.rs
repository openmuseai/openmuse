use word_core::cn::wps::moffice::writer::data::value::TBD::TBD;
use word_core::cn::wps::moffice::writer::data::value::TxTabStops::TxTabStops;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ParaPropsConverterSimple::ParaPropsConverterSimple;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use std::sync::Arc;
use word_core::cn::wps::moffice::writer::api::DocumentOpen::open_boxed_text_document_from_docx_outcome;
use word_core::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};
use word_core::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
use word_core::cn::wps::moffice::writer::data::ITextStream::ITextStream;

#[test]
fn java_merge_sorts_deletions_and_inserts_additions_in_search_order() {
    let left_tbd = TBD::TBD__int_int(1, 2);
    let right_tbd = TBD::TBD__int_int(3, 4);
    let mut left = TxTabStops::new__int_int(1, 2);
    left.rgdxaDel = Some(vec![400]);
    left.rgdxaClose = Some(vec![350]);
    left.insertAdd(500, Some(&left_tbd), 0);
    left.insertAdd(900, Some(&left_tbd), 1);
    let mut right = TxTabStops::new__int_int(1, 1);
    right.rgdxaDel = Some(vec![200]);
    right.rgdxaClose = Some(vec![100]);
    right.insertAdd(700, Some(&right_tbd), 0);

    let merged = TxTabStops::merge(Some(&left), Some(&right)).unwrap();
    assert_eq!(merged.itbdDelMax, 2);
    assert_eq!(merged.itbdAddMax, 3);
    assert_eq!(merged.rgdxaDel.as_deref(), Some(&[200, 400][..]));
    assert_eq!(merged.rgdxaClose.as_deref(), Some(&[100, 350][..]));
    assert_eq!(merged.rgdxaAdd.as_deref(), Some(&[500, 700, 900][..]));
    assert!(merged.contain(700, &right_tbd));
    assert!(!merged.contain(700, &left_tbd));
    assert_eq!(merged.rgtbdAdd.as_ref().unwrap()[1].as_ref().unwrap().frontCharType, 4);
    assert!(TxTabStops::merge(None, None).is_none());
    assert!(TxTabStops::merge(None, Some(&left)).unwrap().equals(&left));
}

#[test]
fn java_null_tbd_array_element_affects_equality_but_not_hash() {
    let mut a = TxTabStops::new__int_int(0, 1);
    a.insertAdd(120, None, 0);
    let b = a.clone();
    assert!(!a.equals(&b));
    assert_eq!(a.hashCode(), b.hashCode());
    assert!(!a.contain(120, &TBD::TBD__int_int(0, 0)));
    assert_eq!(a.array_hash(None, i32::MAX), 0);
    assert!(a.array_equals__intarr_intarr_int(None, Some(&vec![]), i32::MAX));
}

#[test]
fn java_docx_tab_correction_emits_property_236_and_removes_temporary_lists() {
    let mut properties = MutablePropertySet::new();
    properties.setValue(280, Some(Arc::new(vec![700_i32, 200, 500])));
    properties.setValue(281, Some(Arc::new(vec![Some(TBD::TBD__int_int(2, 1)), None, Some(TBD::TBD__int_int(1, 3))])));
    ParaPropsConverterSimple::correctTabs(&mut properties);
    let tabs = properties.getValue(236).unwrap();
    let tabs = tabs.as_ref().as_any().downcast_ref::<TxTabStops>().unwrap();
    assert_eq!(tabs.rgdxaDel.as_deref(), Some(&[200][..]));
    assert_eq!(tabs.rgdxaClose.as_deref(), Some(&[200][..]));
    assert_eq!(tabs.rgdxaAdd.as_deref(), Some(&[500, 700][..]));
    assert_eq!(tabs.rgtbdAdd.as_ref().unwrap()[0].as_ref().unwrap().frontCharType, 3);
    assert!(properties.getValue(280).is_none());
    assert!(properties.getValue(281).is_none());
    assert_eq!(ParaPropsConverterSimple::convertTBDJC(108511772), 2);
    assert_eq!(ParaPropsConverterSimple::convertTabFont(389238644), 5);
}

#[test]
fn complex_product_docx_reaches_tab_property_in_main_papx() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples/docx-mvp-render/flutter/assets/sample2/机械制造与自动化毕业论文.docx");
    let bytes = std::fs::read(path).unwrap();
    let (document, _) = open_boxed_text_document_from_docx_outcome(&bytes, "").unwrap();
    let main = document.getMainDocument().unwrap();
    let papx = main.getPapxTable().unwrap();
    let mut entry = IPapxTable::begin(papx);
    let mut count = 0;
    while let Some(current) = entry {
        if current.isEnd() { break; }
        if PapxEntry::getProperty(&*current).and_then(|p| p.getValue(236)).is_some() { count += 1; }
        entry = PapxEntry::getNext(&*current);
    }
    assert!(count > 0, "DOCX tab nodes must reach PAPX property 236");
}

#[test]
fn complex_product_text_stream_seeks_the_same_real_property_tables() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples/docx-mvp-render/flutter/assets/sample2/机械制造与自动化毕业论文.docx");
    let bytes = std::fs::read(path).unwrap();
    let (document, _) = open_boxed_text_document_from_docx_outcome(&bytes, "").unwrap();
    let main = document.getMainDocument().unwrap();
    let stream = main.getTextStream().expect("main story text stream");
    let papx = main.getPapxTable().expect("main story PAPX");
    let chpx = main.getChpxTable().expect("main story CHPX");
    for cp in [0, 1850] {
        let via_stream = ITextStream::seekPapxTable(stream, cp).expect("stream PAPX entry");
        let direct = IPapxTable::seek(papx, cp).expect("direct PAPX entry");
        assert_eq!(via_stream.position(), direct.position());
        assert_eq!(via_stream.endCp(), direct.endCp());
        assert_eq!(PapxEntry::getProperty(&*via_stream).map(|p| p.size()),
                   PapxEntry::getProperty(&*direct).map(|p| p.size()));

        let via_stream = ITextStream::seekChpxTable(stream, cp).expect("stream CHPX entry");
        let direct = IChpxTable::seek(chpx, cp).expect("direct CHPX entry");
        assert_eq!(via_stream.position(), direct.position());
        assert_eq!(via_stream.endCp(), direct.endCp());
    }
}
