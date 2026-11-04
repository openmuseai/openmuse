//! 01_function_parse_show_master_sp — p:sld / p:sldLayout showMasterSp → isShowMasterShape
//! 矩阵 FUNCTION.md §3.2 SM-DEF / SM-T / SM-F。Java Slide.java:345-347 缺省 true。

#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use visual_zip::{NS, REL_NS, presentation_xml, presentation_rels, visual_pptx_show_master, zip_pptx};

fn pptx_sld(show_master_sp: Option<&str>) -> Vec<u8> {
    let attr = match show_master_sp {
        Some(v) => format!(r#" showMasterSp="{v}""#),
        None => String::new(),
    };
    let slide = format!(
        r#"<p:sld {NS}{attr}><p:cSld><p:spTree><p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm></p:spPr></p:sp></p:spTree></p:cSld></p:sld>"#
    );
    let pres = presentation_xml();
    let rels = presentation_rels("");
    let slide_rels = format!(r#"<Relationships {REL_NS}></Relationships>"#);
    zip_pptx(&[
        ("[Content_Types].xml", b"<Types/>"),
        ("ppt/presentation.xml", pres.as_bytes()),
        ("ppt/_rels/presentation.xml.rels", rels.as_bytes()),
        ("ppt/slides/slide1.xml", slide.as_bytes()),
        ("ppt/slides/_rels/slide1.xml.rels", slide_rels.as_bytes()),
    ])
}

const SLIDE_SP: &str = r#"<p:sp><p:spPr><a:xfrm><a:off x="10" y="20"/><a:ext cx="30" cy="40"/></a:xfrm></p:spPr></p:sp>"#;

#[test]
fn sm_def() {
    let doc = open_pptx_document_bytes(&pptx_sld(None)).unwrap();
    assert!(
        doc.slides[0].isShowMasterShape(),
        "Java Slide.isShowMasterSp: map 无 key 3 且 parent==null → true"
    );
}

#[test]
fn sm_t() {
    let doc = open_pptx_document_bytes(&pptx_sld(Some("1"))).unwrap();
    assert!(doc.slides[0].isShowMasterShape(), "showMasterSp=1 → true");
}

#[test]
fn sm_f() {
    let doc = open_pptx_document_bytes(&pptx_sld(Some("0"))).unwrap();
    assert!(
        !doc.slides[0].isShowMasterShape(),
        "showMasterSp=0 → false"
    );
}

#[test]
fn sm_layout_def() {
    let doc = open_pptx_document_bytes(&visual_pptx_show_master(SLIDE_SP, "", "")).unwrap();
    let layout = doc.slides[0]
        .layout()
        .expect("layout bound from slide rels");
    assert!(
        layout.isShowMasterShape(),
        "Java SlideLayout.isShowMasterSp: 无属性且 parent==null → true"
    );
}

#[test]
fn sm_layout_f() {
    let doc = open_pptx_document_bytes(&visual_pptx_show_master(
        SLIDE_SP,
        "",
        r#" showMasterSp="0""#,
    ))
    .unwrap();
    let layout = doc.slides[0]
        .layout()
        .expect("layout bound from slide rels");
    assert!(
        !layout.isShowMasterShape(),
        "p:sldLayout showMasterSp=0 → layout.isShowMasterShape false"
    );
}
