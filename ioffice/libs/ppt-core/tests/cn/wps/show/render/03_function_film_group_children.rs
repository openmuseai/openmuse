//! pl-08-03 film 组内子 shape。矩阵 FUNCTION.md §3.4 T01–T04。

use std::sync::{Arc, Once};

use ppt_core::cn::wps::show::render::SlideRender::{LayerItem, SlideRender};
use ppt_core::cn::wps::show::util::layout_shape::{KmoShapeTree, KmoShowShape, KmoSlide};
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
use util_adapter::cn::wps::core::runtime::Platform::Platform;

static INIT: Once = Once::new();
fn setup_metrics() {
    INIT.call_once(|| {
        Platform::setDisplayMetrics(Arc::new(KDisplayMetrics::new_with_params(
            0, 0, 1.0, 160, 1.0, 96.0, 96.0,
        )));
    });
}

fn shape_layers(slide: &KmoSlide) -> usize {
    setup_metrics();
    SlideRender::layer_order(Some(slide), 0, 0)
        .into_iter()
        .filter(|l| matches!(l, LayerItem::Shape(_)))
        .count()
}

#[test]
fn t01_group_two_children_two_shape_layers() {
    let a = KmoShowShape {
        id: 1,
        has_xfrm: true,
        transform2DX: 10,
        transform2DY: 10,
        transform2DW: 20,
        transform2DH: 20,
        ..KmoShowShape::default()
    };
    let b = KmoShowShape {
        id: 2,
        has_xfrm: true,
        transform2DX: 40,
        transform2DY: 10,
        transform2DW: 20,
        transform2DH: 20,
        ..KmoShowShape::default()
    };
    let group = KmoShowShape {
        id: 9,
        shape_type: 0,
        children: vec![a, b],
        ..KmoShowShape::default()
    };
    let slide = KmoSlide {
        sp_tree: KmoShapeTree {
            children: vec![group],
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(shape_layers(&slide), 2);
}

#[test]
fn t02_empty_group_zero_shapes() {
    let group = KmoShowShape {
        shape_type: 0,
        children: vec![],
        ..KmoShowShape::default()
    };
    let slide = KmoSlide {
        sp_tree: KmoShapeTree {
            children: vec![group],
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(shape_layers(&slide), 0);
}

#[test]
fn t03_nested_group_films_leaf_only() {
    let leaf = KmoShowShape {
        id: 3,
        shape_type: 1,
        ..KmoShowShape::default()
    };
    let inner = KmoShowShape {
        shape_type: 0,
        children: vec![leaf],
        ..KmoShowShape::default()
    };
    let outer = KmoShowShape {
        shape_type: 0,
        children: vec![inner],
        ..KmoShowShape::default()
    };
    let slide = KmoSlide {
        sp_tree: KmoShapeTree {
            children: vec![outer],
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(shape_layers(&slide), 1);
}

#[test]
fn t04_top_level_sp_still_filmed() {
    let sp = KmoShowShape {
        id: 1,
        shape_type: 1,
        ..KmoShowShape::default()
    };
    let slide = KmoSlide {
        sp_tree: KmoShapeTree {
            children: vec![sp],
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(shape_layers(&slide) >= 1);
}
