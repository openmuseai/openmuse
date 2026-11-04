use std::cell::Cell;
use std::rc::Rc;

use ppt_core::cn::wps::show::pptxr::ppt::slides::PptxrSlide::{
    compile_surface::{
        KmoPresentation, KmoSlide, OpcrPackagePart, OpcrRelationship, OpcrRelationships, PptxrThemes,
        OxfPptRelations,
    },
    PptxrSlide,
};

const SLD: &[u8] = br#"<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"/>"#;

fn slide_with_counts() -> KmoSlide {
    KmoSlide {
        theme_override: None,
        update_field_text: Rc::new(Cell::new(0)),
        xml_fill: Rc::new(Cell::new(0)),
    }
}

#[test]
fn t01_null_part_returns_without_xml() {
    let slide = slide_with_counts();
    let xml_fill = slide.xml_fill.clone();
    let field = slide.update_field_text.clone();
    let mut reader = PptxrSlide::PptxrSlide(Some(slide), None, None, None, None, None, 0, None);
    reader.open();
    assert_eq!(xml_fill.get(), 0);
    assert_eq!(field.get(), 0);
}

#[test]
fn t02_xml_then_load_and_update_field_text() {
    let slide = slide_with_counts();
    let xml_fill = slide.xml_fill.clone();
    let field = slide.update_field_text.clone();
    let part = OpcrPackagePart {
        relationships: Some(OpcrRelationships::default()),
        xml: SLD.to_vec(),
        fail: false,
        iolog_count: Rc::new(Cell::new(0)),
    };
    let mut reader = PptxrSlide::PptxrSlide(
        Some(slide),
        Some(KmoPresentation::default()),
        Some(part),
        None,
        None,
        None,
        0,
        None,
    );
    reader.open();
    assert_eq!(field.get(), 1);
}

#[test]
fn t03_stream_throwable_swallowed() {
    let slide = slide_with_counts();
    let field = slide.update_field_text.clone();
    let xml_fill = slide.xml_fill.clone();
    let iolog = Rc::new(Cell::new(0));
    let part = OpcrPackagePart {
        relationships: Some(OpcrRelationships::default()),
        xml: SLD.to_vec(),
        fail: true,
        iolog_count: iolog.clone(),
    };
    let mut reader = PptxrSlide::PptxrSlide(
        Some(slide),
        Some(KmoPresentation::default()),
        Some(part),
        None,
        None,
        None,
        0,
        None,
    );
    reader.open();
    assert_eq!(iolog.get(), 1);
    assert_eq!(field.get(), 0);
    assert_eq!(xml_fill.get(), 0);
}

#[test]
fn t04_theme_override_rel() {
    let slide = slide_with_counts();
    let open_count = Rc::new(Cell::new(0));
    let mut rels = OpcrRelationships::default();
    let typ = OxfPptRelations::themeOverride().getRelsType();
    rels.by_type.insert(
        typ.clone(),
        OpcrRelationship {
            rel_type: typ,
            id: "rIdTheme".into(),
        },
    );
    let part = OpcrPackagePart {
        relationships: Some(rels),
        xml: SLD.to_vec(),
        fail: false,
        iolog_count: Rc::new(Cell::new(0)),
    };
    let themes = PptxrThemes {
        open_count: open_count.clone(),
    };
    let mut reader = PptxrSlide::PptxrSlide(
        Some(slide),
        Some(KmoPresentation::default()),
        Some(part),
        None,
        Some(themes),
        None,
        0,
        None,
    );
    reader.open();
    assert_eq!(open_count.get(), 1);
    assert!(reader.getSlide().unwrap().theme_override.is_some());
}
