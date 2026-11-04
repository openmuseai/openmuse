//! pp-18-01 p:sp blipFill → media_bytes。矩阵 FUNCTION.md T01–T03。

#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use visual_zip::{visual_pptx, PNG_1X1};

fn sp_blip() -> &'static str {
    r#"<p:sp><p:nvSpPr><p:cNvPr id="1" name="pic"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr><p:spPr><a:xfrm><a:off x="1" y="0"/><a:ext cx="10" cy="10"/></a:xfrm><a:blipFill><a:blip r:embed="rIdImg"/></a:blipFill></p:spPr></p:sp>"#
}

#[test]
fn t01_sp_embed_png_has_bytes() {
    let doc = open_pptx_document_bytes(&visual_pptx(sp_blip(), false, false, true)).unwrap();
    let sp = &doc.slides[0].sp_tree.children[0];
    assert_eq!(sp.shape_type, 1);
    assert_eq!(sp.media_r_id.as_deref(), Some("rIdImg"));
    assert_eq!(sp.media_bytes.as_deref(), Some(PNG_1X1));
    assert!(sp.media_id > 0);
}

#[test]
fn t02_missing_rel_bytes_none() {
    let doc = open_pptx_document_bytes(&visual_pptx(sp_blip(), false, false, false)).unwrap();
    let sp = &doc.slides[0].sp_tree.children[0];
    assert_eq!(sp.media_r_id.as_deref(), Some("rIdImg"));
    assert!(sp.media_bytes.is_none());
}

#[test]
fn t03_group_child_sp_has_bytes() {
    let xml = format!(
        r#"<p:grpSp><p:nvGrpSpPr><p:cNvPr id="9" name="g"/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="100" cy="100"/><a:chOff x="0" y="0"/><a:chExt cx="100" cy="100"/></a:xfrm></p:grpSpPr>{}</p:grpSp>"#,
        sp_blip()
    );
    let doc = open_pptx_document_bytes(&visual_pptx(&xml, false, false, true)).unwrap();
    let group = &doc.slides[0].sp_tree.children[0];
    assert_eq!(group.shape_type, 0);
    let child = &group.children[0];
    assert_eq!(child.media_bytes.as_deref(), Some(PNG_1X1));
    assert!(child.media_id > 0);
}
