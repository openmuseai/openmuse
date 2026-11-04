// Test 03_function_blip_image
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::moffice::drawing::render::GemoRender::GemoRender;
use ppt_core::cn::wps::show::render::rendercommand::RenderCommand;
use ppt_core::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use ppt_core::cn::wps::show::render::SlideRender::LayerItem;
use ppt_core::cn::wps::show::util::layout_shape::{FillObj, KmoShowShape};
use util::cn::wps::graphics::RectF::RectF;

fn blip_fill(media_id: Option<i32>) -> FillObj {
    FillObj {
        fill_type: Some(2),
        media_id,
        blip_embed: Some("rId2".into()),
        ..Default::default()
    }
}

fn shape_rect_item() -> LayerItem {
    let typo = ShowTypoShape::typo_shape_snapshot(
        Some(ConvertKmo::from_layout(&KmoShowShape {
            id: 601,
            ..KmoShowShape::default()
        }).into_shape()),
        None,
        Some(RectF::new_with(10.0, 20.0, 110.0, 80.0)),
    );
    LayerItem::Shape(typo)
}

#[test]
fn t01_blip_media_draw_image() {
    let mut item = shape_rect_item();
    let fill = blip_fill(Some(7));
    let geo = ppt_core::cn::wps::show::render::rendercommand::geometry_commands(&mut item);
    let RenderCommand::DrawRect {
        x: gx,
        y: gy,
        w: gw,
        h: gh,
        ..
    } = geo[0]
    else {
        panic!("geometry is DrawRect");
    };
    let mut item = shape_rect_item();
    let cmds = GemoRender::commands_for_layer(&mut item, Some(&fill));
    let images: Vec<_> = cmds
        .iter()
        .filter(|c| matches!(c, RenderCommand::DrawImage { .. }))
        .collect();
    assert_eq!(images.len(), 1);
    match &cmds[0] {
        RenderCommand::DrawImage {
            x,
            y,
            w,
            h,
            media_id,
            r_id,
            ..
        } => {
            assert_eq!((*x, *y, *w, *h), (gx, gy, gw, gh));
            assert_eq!(*media_id, 7);
            assert_eq!(r_id.as_deref(), Some("rId2"));
        }
        other => panic!("expected DrawImage, got {other:?}"),
    }
}

#[test]
fn t02_blip_no_media_placeholder_rect() {
    let mut item = shape_rect_item();
    let fill = blip_fill(None);
    let cmds = GemoRender::commands_for_layer(&mut item, Some(&fill));
    assert!(cmds
        .iter()
        .all(|c| !matches!(c, RenderCommand::DrawImage { .. })));
    match &cmds[0] {
        RenderCommand::DrawRect { fill, .. } => assert!(fill.is_none()),
        other => panic!("expected placeholder DrawRect, got {other:?}"),
    }
}

#[test]
fn t03_solid_fill_no_draw_image() {
    let mut item = shape_rect_item();
    let fill = FillObj {
        fill_type: Some(0),
        solid: Some(0x00FF00),
        ..Default::default()
    };
    let cmds = GemoRender::commands_for_layer(&mut item, Some(&fill));
    assert!(cmds
        .iter()
        .all(|c| !matches!(c, RenderCommand::DrawImage { .. })));
}

#[test]
fn t04_background_blip_full_slide() {
    let mut white = LayerItem::WhiteBackground { w: 96.0, h: 54.0 };
    let white_cmds = GemoRender::commands_for_layer(&mut white, None);
    assert!(matches!(
        &white_cmds[0],
        RenderCommand::DrawRect {
            fill: Some(0xFFFFFFFF),
            ..
        }
    ));

    let mut bg = LayerItem::Background { w: 96.0, h: 54.0, fill: None };
    let fill = blip_fill(Some(11));
    let cmds = GemoRender::commands_for_layer(&mut bg, Some(&fill));
    match &cmds[0] {
        RenderCommand::DrawImage {
            x,
            y,
            w,
            h,
            media_id,
            ..
        } => {
            assert_eq!((*x, *y, *w, *h), (0.0, 0.0, 96.0, 54.0));
            assert_eq!(*media_id, 11);
        }
        other => panic!("expected full-slide DrawImage, got {other:?}"),
    }
}
