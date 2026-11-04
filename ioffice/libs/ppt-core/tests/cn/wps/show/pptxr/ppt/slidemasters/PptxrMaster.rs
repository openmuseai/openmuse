use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use ppt_core::cn::wps::show::pptxr::ppt::slidemasters::PptxrMaster::{
    compile_surface::{
        KmoPresentation, KmoSlideMaster, OpcrPackagePart, OpcrRelationship, OpcrRelationships,
        OxfPptRelations, PptxrThemes,
    },
    PptxrMaster,
};

const XML: &[u8] = br#"<p:sldMaster xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"/>"#;

#[test]
fn t01_null_part_replaces_master() {
    let add_theme = Rc::new(Cell::new(0));
    let pres = KmoPresentation {
        add_theme: add_theme.clone(),
        ..Default::default()
    };
    let orig = KmoSlideMaster::default();
    let mut reader = PptxrMaster::PptxrMaster(
        Some(pres),
        Some(orig),
        None,
        None,
        None,
        None,
        0,
    );
    reader.open();
    let master = reader.getMaster().unwrap();
    assert!(master.from_default);
    assert!(!master.io_reading);
    assert!(add_theme.get() >= 1);
}

#[test]
fn t02_theme_rel_vs_default() {
    let open_count = Rc::new(Cell::new(0));
    let add_theme = Rc::new(Cell::new(0));
    let mut rels = OpcrRelationships::default();
    let typ = OxfPptRelations::theme().getRelsType();
    rels.by_type.insert(typ, OpcrRelationship::default());
    let stream_calls = Rc::new(Cell::new(0));
    let part = OpcrPackagePart {
        relationships: Some(rels),
        xml: XML.to_vec(),
        stream_calls,
    };
    let mut with_rel = PptxrMaster::PptxrMaster(
        Some(KmoPresentation::default()),
        Some(KmoSlideMaster::default()),
        Some(part),
        None,
        Some(PptxrThemes { open_count: open_count.clone() }),
        None,
        0,
    );
    with_rel.open();
    assert_eq!(open_count.get(), 1);

    let add2 = Rc::new(Cell::new(0));
    let part2 = OpcrPackagePart {
        relationships: Some(OpcrRelationships::default()),
        xml: XML.to_vec(),
        stream_calls: Rc::new(Cell::new(0)),
    };
    let mut no_rel = PptxrMaster::PptxrMaster(
        Some(KmoPresentation { add_theme: add2.clone(), ..Default::default() }),
        Some(KmoSlideMaster::default()),
        Some(part2),
        None,
        None,
        None,
        0,
    );
    no_rel.open();
    assert_eq!(add2.get(), 1);
    assert!(no_rel.getMaster().unwrap().film.theme.is_some());
}

#[test]
fn t03_on_slide_layout_ids() {
    let mut rels = OpcrRelationships::default();
    rels.by_id.insert(
        "rId1".into(),
        OpcrRelationship {
            id: "rId1".into(),
            part_name: "ppt/slideLayouts/slideLayout1.xml".into(),
            part: Some(OpcrPackagePart::default()),
        },
    );
    rels.by_id.insert("rIdSkip".into(), OpcrRelationship { id: "rIdSkip".into(), part_name: "x".into(), part: None });
    let layouts = Rc::new(Cell::new(0));
    let master = KmoSlideMaster { layouts: layouts.clone(), ..Default::default() };
    let part = OpcrPackagePart {
        relationships: Some(rels),
        xml: XML.to_vec(),
        stream_calls: Rc::new(Cell::new(0)),
    };
    let mut reader = PptxrMaster::PptxrMaster(
        Some(KmoPresentation::default()),
        Some(master),
        Some(part),
        None,
        None,
        None,
        0,
    );
    let mut ids = HashMap::new();
    ids.insert("rId1".into(), 1i64);
    ids.insert("rIdSkip".into(), 2i64);
    ids.insert("rIdMissing".into(), 3i64);
    reader.onSlideLayoutIds(ids);
    assert_eq!(reader.mLayoutsReader.len(), 1);
    assert!(reader.mLayoutsReader.contains_key("ppt/slideLayouts/slideLayout1.xml"));
    assert_eq!(layouts.get(), 1);
}

#[test]
fn t04_bopen_idempotent() {
    let stream_calls = Rc::new(Cell::new(0));
    let part = OpcrPackagePart {
        relationships: Some(OpcrRelationships::default()),
        xml: XML.to_vec(),
        stream_calls: stream_calls.clone(),
    };
    let mut reader = PptxrMaster::PptxrMaster(
        Some(KmoPresentation::default()),
        Some(KmoSlideMaster::default()),
        Some(part),
        None,
        None,
        None,
        0,
    );
    reader.open();
    reader.open();
    assert!(reader.bOpen);
    assert_eq!(stream_calls.get(), 1);
}
