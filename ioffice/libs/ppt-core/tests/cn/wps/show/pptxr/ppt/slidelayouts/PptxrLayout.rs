use std::cell::Cell;
use std::rc::Rc;

use ppt_core::cn::wps::show::pptxr::ppt::slidelayouts::PptxrLayout::{
    compile_surface::{
        KmoPresentation, KmoSlideLayout, OpcrPackagePart, OpcrRelationship, OpcrRelationships,
        OxfPptRelations, PptxrThemes,
    },
    PptxrLayout,
};

const XML: &[u8] = br#"<p:sldLayout xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"/>"#;

fn part(xml: &[u8], fail_rels: bool) -> (OpcrPackagePart, Rc<Cell<i32>>, Rc<Cell<i32>>) {
    let stream_calls = Rc::new(Cell::new(0));
    let iolog = Rc::new(Cell::new(0));
    (
        OpcrPackagePart {
            relationships: Some(OpcrRelationships::default()),
            xml: xml.to_vec(),
            fail_rels,
            iolog_count: iolog.clone(),
            stream_calls: stream_calls.clone(),
        },
        stream_calls,
        iolog,
    )
}

#[test]
fn t01_bopen_second_open_skips_stream() {
    let (p, stream_calls, _) = part(XML, false);
    let mut reader = PptxrLayout::PptxrLayout(
        Some(KmoPresentation::default()),
        Some(KmoSlideLayout::default()),
        Some(p),
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

#[test]
fn t02_rels_throwable_early_return() {
    let (p, stream_calls, iolog) = part(XML, true);
    let mut reader = PptxrLayout::PptxrLayout(
        Some(KmoPresentation::default()),
        Some(KmoSlideLayout::default()),
        Some(p),
        None,
        None,
        None,
        0,
    );
    reader.open();
    assert!(reader.bOpen);
    assert_eq!(iolog.get(), 1);
    assert_eq!(stream_calls.get(), 0);
}

#[test]
fn t03_theme_override() {
    let open_count = Rc::new(Cell::new(0));
    let mut rels = OpcrRelationships::default();
    let typ = OxfPptRelations::themeOverride().getRelsType();
    rels.by_type.insert(typ.clone(), OpcrRelationship { rel_type: typ });
    let stream_calls = Rc::new(Cell::new(0));
    let part = OpcrPackagePart {
        relationships: Some(rels),
        xml: XML.to_vec(),
        fail_rels: false,
        iolog_count: Rc::new(Cell::new(0)),
        stream_calls,
    };
    let mut reader = PptxrLayout::PptxrLayout(
        Some(KmoPresentation::default()),
        Some(KmoSlideLayout::default()),
        Some(part),
        None,
        Some(PptxrThemes { open_count: open_count.clone() }),
        None,
        0,
    );
    reader.open();
    assert_eq!(open_count.get(), 1);
    assert!(reader.getLayout().unwrap().film.theme.is_some());
}
