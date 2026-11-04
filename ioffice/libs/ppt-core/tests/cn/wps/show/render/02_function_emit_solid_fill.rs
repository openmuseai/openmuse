#[path = "../pptxr/visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use ppt_core::cn::wps::show::render::mvp_pipeline::render_slide;
use ppt_core::cn::wps::show::render::rendercommand::RenderCommand;
use visual_zip::visual_pptx;

#[test]
fn t01_solid_fill_on_draw_rect() {
    let slide = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm><a:solidFill><a:srgbClr val="4472C4"/></a:solidFill></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, false, false, false)).unwrap();
    let cmds = render_slide(&doc.slides[0], 0);
    assert!(cmds.iter().any(|c| matches!(
        c,
        RenderCommand::DrawRect {
            fill: Some(0xFF4472C4),
            ..
        }
    )));
}

#[test]
fn t02_no_fill_has_none_or_stroke_only() {
    let slide = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, false, false, false)).unwrap();
    let cmds = render_slide(&doc.slides[0], 0);
    let shapes: Vec<_> = cmds
        .iter()
        .filter(|c| matches!(c, RenderCommand::DrawRect { x, .. } if *x > 0.0 || matches!(c, RenderCommand::DrawRect { w, h, .. } if *w < 900.0 )))
        .collect();
    let _ = shapes;
    assert!(cmds.iter().any(|c| matches!(
        c,
        RenderCommand::DrawRect { fill: None, .. }
            | RenderCommand::DrawRect {
                fill: Some(0xFFFFFFFF),
                ..
            }
    )));
}

#[test]
fn t03_scheme_fill_emits_argb() {
    let slide = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="10" cy="10"/></a:xfrm><a:solidFill><a:schemeClr val="accent1"/></a:solidFill></p:spPr></p:sp>"#;
    let doc = open_pptx_document_bytes(&visual_pptx(slide, false, true, false)).unwrap();
    let cmds = render_slide(&doc.slides[0], 0);
    assert!(cmds.iter().any(|c| matches!(
        c,
        RenderCommand::DrawRect {
            fill: Some(0xFF4472C4),
            ..
        }
    )));
}
