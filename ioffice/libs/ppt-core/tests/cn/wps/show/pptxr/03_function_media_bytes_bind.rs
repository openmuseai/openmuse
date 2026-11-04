#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use visual_zip::{visual_pptx, PNG_1X1};

#[test]
fn t01_media_bytes_bound_to_pic() {
    let pic = r#"<p:pic><p:blipFill><a:blip r:embed="rIdImg"/></p:blipFill><p:spPr><a:xfrm><a:off x="1" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:pic>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(pic, false, false, true)).unwrap();
    let pic = doc.slides[0]
        .sp_tree
        .children
        .iter()
        .find(|s| s.shape_type == 2)
        .unwrap();
    assert_eq!(pic.media_bytes.as_deref(), Some(PNG_1X1));
    assert!(pic.media_id > 0);
}

#[test]
fn t02_missing_media_part_leaves_bytes_none() {
    let pic = r#"<p:pic><p:blipFill><a:blip r:embed="rIdImg"/></p:blipFill><p:spPr><a:xfrm><a:off x="1" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:pic>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(pic, false, false, false)).unwrap();
    let pic = &doc.slides[0].sp_tree.children[0];
    assert_eq!(pic.media_r_id.as_deref(), Some("rIdImg"));
    assert!(pic.media_bytes.is_none());
}

#[test]
fn t03_non_pic_has_no_media() {
    let doc = open_pptx_document_bytes(&visual_pptx(
        r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm></p:spPr></p:sp>"#,
        false,
        false,
        true,
    ))
    .unwrap();
    assert!(doc.slides[0].sp_tree.children[0].media_bytes.is_none());
}
