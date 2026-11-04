#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::{
    open_pptx_document_bytes, slide_layout_part_from_rels,
};
use visual_zip::{visual_pptx, LAYOUT_TYPE, REL_NS};

#[test]
fn t01_relative_layout_target_from_slide_part() {
    let rels = format!(
        r#"<Relationships {REL_NS}><Relationship Id="rIdL" Type="{LAYOUT_TYPE}" Target="../slideLayouts/slideLayout12.xml"/></Relationships>"#
    );
    let part = slide_layout_part_from_rels("ppt/slides/slide1.xml", &rels).unwrap();
    assert_eq!(part, "ppt/slideLayouts/slideLayout12.xml");
    let doc = open_pptx_document_bytes(&visual_pptx(
        r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:sp>"#,
        true,
        false,
        false,
    ))
    .unwrap();
    assert!(doc.slides[0].layout.is_some());
}

#[test]
fn t02_missing_layout_rel_is_none() {
    let rels = format!(r#"<Relationships {REL_NS}></Relationships>"#);
    assert!(slide_layout_part_from_rels("ppt/slides/slide1.xml", &rels).is_none());
    let doc = open_pptx_document_bytes(&visual_pptx(
        r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:sp>"#,
        false,
        false,
        false,
    ))
    .unwrap();
    assert!(doc.slides[0].layout.is_none());
    assert_eq!(doc.slides.len(), 1);
}

#[test]
fn t03_first_layout_rel_wins_and_dotdot_collapses() {
    let rels = format!(
        r#"<Relationships {REL_NS}><Relationship Id="rIdA" Type="{LAYOUT_TYPE}" Target="../slideLayouts/slideLayout1.xml"/><Relationship Id="rIdB" Type="{LAYOUT_TYPE}" Target="../slideLayouts/slideLayout2.xml"/></Relationships>"#
    );
    let part = slide_layout_part_from_rels("ppt/slides/slide1.xml", &rels).unwrap();
    assert_eq!(part, "ppt/slideLayouts/slideLayout1.xml");
    let escaped = slide_layout_part_from_rels(
        "ppt/slides/slide1.xml",
        &format!(
            r#"<Relationships {REL_NS}><Relationship Id="rIdL" Type="{LAYOUT_TYPE}" Target="../../slideLayouts/slideLayout1.xml"/></Relationships>"#
        ),
    )
    .unwrap();
    assert!(!escaped.contains(".."));
}
