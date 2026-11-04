//! pl-08-02 子级 slide 绝对框写回。矩阵 FUNCTION.md §3.4 T01–T05。

use ppt_core::cn::wps::show::layout::page_inheritance::resolve_page;
use ppt_core::cn::wps::show::util::layout_shape::{KmoShapeTree, KmoShowShape, KmoSlide};

fn leaf(off: (i32, i32), ext: (i32, i32)) -> KmoShowShape {
    KmoShowShape {
        has_xfrm: true,
        transform2DX: off.0,
        transform2DY: off.1,
        transform2DW: ext.0,
        transform2DH: ext.1,
        ..KmoShowShape::default()
    }
}

fn group(
    off: (i32, i32),
    ext: (i32, i32),
    ch_off: Option<(i32, i32)>,
    ch_ext: Option<(i32, i32)>,
    children: Vec<KmoShowShape>,
) -> KmoShowShape {
    KmoShowShape {
        shape_type: 0,
        has_xfrm: true,
        transform2DX: off.0,
        transform2DY: off.1,
        transform2DW: ext.0,
        transform2DH: ext.1,
        child_off: ch_off,
        child_ext: ch_ext,
        children,
        ..KmoShowShape::default()
    }
}

fn slide_of(root: KmoShowShape) -> KmoSlide {
    KmoSlide {
        sp_tree: KmoShapeTree {
            children: vec![root],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn t01_writeback_adds_group_origin() {
    let mut slide = slide_of(group(
        (1000, 2000),
        (300, 300),
        Some((0, 0)),
        Some((300, 300)),
        vec![leaf((0, 0), (300, 300))],
    ));
    let _ = resolve_page(&mut slide);
    let got = &slide.sp_tree.children[0].children[0];
    assert_eq!(
        (
            got.transform2DX,
            got.transform2DY,
            got.transform2DW,
            got.transform2DH
        ),
        (1000, 2000, 300, 300)
    );
    let g = &slide.sp_tree.children[0];
    assert_eq!((g.transform2DX, g.transform2DY), (1000, 2000));
}

#[test]
fn t02_missing_choff_does_not_map() {
    let mut slide = slide_of(group(
        (1000, 2000),
        (300, 300),
        None,
        None,
        vec![leaf((5000, 6000), (10, 10))],
    ));
    let _ = resolve_page(&mut slide);
    let got = &slide.sp_tree.children[0].children[0];
    assert_eq!(got.transform2DX, 5000);
    assert_eq!(got.transform2DY, 6000);
}

#[test]
fn t03_nested_groups() {
    let mut slide = slide_of(group(
        (1000, 2000),
        (1000, 1000),
        Some((0, 0)),
        Some((1000, 1000)),
        vec![group(
            (100, 100),
            (400, 400),
            Some((0, 0)),
            Some((400, 400)),
            vec![leaf((10, 20), (50, 60))],
        )],
    ));
    let _ = resolve_page(&mut slide);
    let inner = &slide.sp_tree.children[0].children[0];
    assert_eq!(
        (inner.transform2DX, inner.transform2DY, inner.shape_type),
        (100, 100, 0),
        "inner group keeps local off/ext"
    );
    let leaf_sp = &inner.children[0];
    assert_eq!(
        (
            leaf_sp.transform2DX,
            leaf_sp.transform2DY,
            leaf_sp.transform2DW,
            leaf_sp.transform2DH
        ),
        (1110, 2120, 50, 60)
    );
}

#[test]
fn t04_chext_cx_zero_scale_one() {
    let mut slide = slide_of(group(
        (100, 200),
        (300, 300),
        Some((0, 0)),
        Some((0, 100)),
        vec![leaf((0, 0), (50, 50))],
    ));
    let resolved = resolve_page(&mut slide);
    let got = &slide.sp_tree.children[0].children[0];
    assert!(got.transform2DW > 0);
    assert!(resolved[1].abs.left.is_finite());
    assert!(resolved[1].abs.right.is_finite());
}

#[test]
fn t05_transform_fields_match_abs_not_only_resolved_vec() {
    let mut slide = slide_of(group(
        (1000, 2000),
        (300, 300),
        Some((0, 0)),
        Some((300, 300)),
        vec![leaf((0, 0), (300, 300))],
    ));
    let resolved = resolve_page(&mut slide);
    let got = &slide.sp_tree.children[0].children[0];
    assert_eq!(resolved.len(), 2);
    assert_eq!(got.transform2DX as f32, resolved[1].abs.left);
    assert_eq!(got.transform2DY as f32, resolved[1].abs.top);
    assert_eq!(
        (got.transform2DX + got.transform2DW) as f32,
        resolved[1].abs.right
    );
}
