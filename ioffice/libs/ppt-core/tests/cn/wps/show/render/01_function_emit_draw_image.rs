#[path = "../pptxr/visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use ppt_core::cn::wps::show::render::mvp_pipeline::render_slide;
use ppt_core::cn::wps::show::render::rendercommand::RenderCommand;
use visual_zip::visual_pptx;

#[test]
fn t01_pic_emits_draw_image() {
    let pic = r#"<p:pic><p:blipFill><a:blip r:embed="rIdImg"/></p:blipFill><p:spPr><a:xfrm><a:off x="1" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:pic>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(pic, false, false, true)).unwrap();
    let cmds = render_slide(&doc.slides[0], 0);
    assert!(cmds.iter().any(|c| matches!(
        c,
        RenderCommand::DrawImage {
            bytes_b64: Some(_),
            ..
        }
    )));
}

#[test]
fn t02_pic_without_bytes_still_draw_image() {
    let pic = r#"<p:pic><p:blipFill><a:blip r:embed="rIdImg"/></p:blipFill><p:spPr><a:xfrm><a:off x="1" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:pic>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(pic, false, false, false)).unwrap();
    let cmds = render_slide(&doc.slides[0], 0);
    assert!(cmds
        .iter()
        .any(|c| matches!(c, RenderCommand::DrawImage { bytes_b64: None, .. })));
}

#[test]
fn t03_plain_shape_is_not_draw_image() {
    let doc = open_pptx_document_bytes(&visual_pptx(
        r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:sp>"#,
        false,
        false,
        false,
    ))
    .unwrap();
    let cmds = render_slide(&doc.slides[0], 0);
    assert!(cmds
        .iter()
        .all(|c| !matches!(c, RenderCommand::DrawImage { .. })));
}
