// Test 01_function_layer_order — 三层胶片层序（pr-01）
use std::sync::{Arc, Once};

use ppt_core::cn::wps::base::converter::UnitsConverter::UnitsConverter;
use ppt_core::cn::wps::show::render::SlideRender::{LayerItem, SlideRender};
use ppt_core::cn::wps::show::util::DrawFlags::DrawFlags;
use ppt_core::cn::wps::show::util::layout_shape::{
    KmoShapeTree, KmoShowShape, KmoSlide, KmoSlideLayout, KmoSlideMaster,
};
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
use util_adapter::cn::wps::core::runtime::Platform::Platform;
use std::rc::Rc;

static INIT: Once = Once::new();
fn setup_metrics() {
    INIT.call_once(|| {
        Platform::setDisplayMetrics(Arc::new(KDisplayMetrics::new_with_params(
            0, 0, 1.0, 160, 1.0, 96.0, 96.0,
        )));
    });
}

fn leaf(id: i32) -> KmoShowShape {
    KmoShowShape {
        id,
        ..KmoShowShape::default()
    }
}

fn slide_min() -> KmoSlide {
    KmoSlide {
        slide_width: 12_192_000,
        slide_height: 6_858_000,
        show_master_shape: false,
        has_background: false,
        sp_tree: KmoShapeTree {
            children: vec![leaf(1)], ..Default::default() },
        ..Default::default()
    }
}

fn kind(item: &LayerItem) -> &'static str {
    match item {
        LayerItem::WhiteBackground { .. } => "white",
        LayerItem::Background { .. } => "bg",
        LayerItem::Shape(_) => "shape",
    }
}

#[test]
fn t01_white_then_slide_film() {
    setup_metrics();
    let s = slide_min();
    let layers = SlideRender::layer_order(Some(&s), 0, 0);
    let names: Vec<_> = layers.iter().map(kind).collect();
    assert_eq!(names, vec!["white", "shape"]);
}

#[test]
fn t02_no_master_when_layout_hides() {
    setup_metrics();
    let master = Rc::new(KmoSlideMaster {
        sp_tree: Rc::new(KmoShapeTree {
            children: vec![leaf(9)], ..Default::default() }),
        ..Default::default()
    });
    let layout = Rc::new(KmoSlideLayout {
        master: Some(master),
        show_master_shape: false,
        sp_tree: Some(Rc::new(KmoShapeTree {
            children: vec![leaf(8)], ..Default::default() })),
        ..Default::default()
    });
    let s = KmoSlide {
        layout: Some(layout),
        show_master_shape: true,
        slide_width: 12_192_000,
        slide_height: 6_858_000,
        sp_tree: KmoShapeTree {
            children: vec![leaf(1)], ..Default::default() },
        ..Default::default()
    };
    let layers = SlideRender::layer_order(Some(&s), 0, 0);
    let names: Vec<_> = layers.iter().map(kind).collect();
    assert_eq!(names, vec!["white", "shape", "shape"]);
}

#[test]
fn t03_no_layout_film_when_slide_hides_master() {
    setup_metrics();
    let layout = Rc::new(KmoSlideLayout {
        show_master_shape: true,
        sp_tree: Some(Rc::new(KmoShapeTree {
            children: vec![leaf(8)], ..Default::default() })),
        ..Default::default()
    });
    let s = KmoSlide {
        layout: Some(layout),
        show_master_shape: false,
        slide_width: 12_192_000,
        slide_height: 6_858_000,
        sp_tree: KmoShapeTree {
            children: vec![leaf(1)], ..Default::default() },
        ..Default::default()
    };
    let layers = SlideRender::layer_order(Some(&s), 0, 0);
    let names: Vec<_> = layers.iter().map(kind).collect();
    assert_eq!(names, vec!["white", "shape"]);
}

#[test]
fn t04_white_size_from_emu() {
    setup_metrics();
    let s = slide_min();
    let layers = SlideRender::layer_order(Some(&s), 0, 0);
    let LayerItem::WhiteBackground { w, h } = &layers[0] else {
        panic!("first must be white");
    };
    let u = UnitsConverter::render();
    assert_eq!(*w, u.fromEmuX(12_192_000.0));
    assert_eq!(*h, u.fromEmuY(6_858_000.0));
}

#[test]
fn t05_background_skip() {
    setup_metrics();
    let mut s = slide_min();
    s.has_background = false;
    let layers = SlideRender::layer_order(Some(&s), 0, 0);
    assert!(!layers.iter().any(|i| matches!(i, LayerItem::Background { .. })));
    s.has_background = true;
    let layers = SlideRender::layer_order(Some(&s), DrawFlags::NOT_DRAW_BACKGROUND_SHAPE, 0);
    assert!(!layers.iter().any(|i| matches!(i, LayerItem::Background { .. })));
    let layers = SlideRender::layer_order(Some(&s), 0, 0);
    assert!(layers.iter().any(|i| matches!(i, LayerItem::Background { .. })));
}

#[test]
fn t06_read_thumb_flags() {
    let r = SlideRender::getOrAllocReadSlideRender();
    assert_eq!(r.slideDrawFlags, 21505154);
    assert_eq!(r.followerDrawFlags, 532609);
    let t = SlideRender::getOrAllocThumbSlideRender();
    assert_eq!(t.slideDrawFlags, 29909122);
    assert_eq!(t.followerDrawFlags, 548993);
}

#[test]
#[ignore = "spec: post-MVP"]
fn t07_graphic_frame() {}
