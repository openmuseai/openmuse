// Test 02_function_group_child_rect — group 子级坐标映射（pl-02）
// 冻结矩阵 FUNCTION.md §3.2 T01–T11。期望由 Java calcChildRelativeRect /
// calcShape2TopMatrix 公式手算。
use std::rc::Rc;

use ppt_core::cn::wps::show::util::layout_shape::KmoShowShape;
use ppt_core::cn::wps::show::util::ShapeUtil::ShapeUtil;
use util::cn::wps::graphics::RectF::RectF;

fn assert_rect(rc: &RectF, left: f32, top: f32, right: f32, bottom: f32) {
    assert_eq!(
        (rc.left, rc.top, rc.right, rc.bottom),
        (left, top, right, bottom),
        "rect mismatch"
    );
}

fn group(off: (i32, i32), ext: (i32, i32), ch_off: Option<(i32, i32)>, ch_ext: Option<(i32, i32)>) -> KmoShowShape {
    KmoShowShape {
        shape_type: 0,
        transform2DX: off.0,
        transform2DY: off.1,
        transform2DW: ext.0,
        transform2DH: ext.1,
        child_off: ch_off,
        child_ext: ch_ext,
        ..KmoShowShape::default()
    }
}

fn leaf(off: (i32, i32), ext: (i32, i32)) -> KmoShowShape {
    KmoShowShape {
        transform2DX: off.0,
        transform2DY: off.1,
        transform2DW: ext.0,
        transform2DH: ext.1,
        ..KmoShowShape::default()
    }
}

/// rcGroup 200×400, chExt 400×200, child (0,0,400,200), rot 决定是否换挡。
fn scaled_child(rot: f32) -> RectF {
    let g = group((0, 0), (200, 400), Some((0, 0)), Some((400, 200)));
    let rc_group = RectF::new_with(0.0, 0.0, 200.0, 400.0);
    let mut rc_child = RectF::new_with(0.0, 0.0, 400.0, 200.0);
    ShapeUtil::calcChildRelativeRect(&g, &rc_group, &mut rc_child, rot);
    rc_child
}

#[test]
fn t01_missing_child_extents_leaves_rect() {
    let g = group((0, 0), (300, 300), None, None);
    let rc_group = RectF::new_with(0.0, 0.0, 300.0, 300.0);
    let mut rc_child = RectF::new_with(1.0, 2.0, 3.0, 4.0);
    ShapeUtil::calcChildRelativeRect(&g, &rc_group, &mut rc_child, 0.0);
    assert_rect(&rc_child, 1.0, 2.0, 3.0, 4.0);
}

#[test]
fn t02_one_to_one_scale() {
    let g = group((0, 0), (300, 300), Some((0, 0)), Some((300, 300)));
    let rc_group = RectF::new_with(0.0, 0.0, 300.0, 300.0);
    let mut rc_child = RectF::new_with(0.0, 0.0, 300.0, 300.0);
    ShapeUtil::calcChildRelativeRect(&g, &rc_group, &mut rc_child, 0.0);
    assert_rect(&rc_child, 0.0, 0.0, 300.0, 300.0);
}

#[test]
fn t03_offset_and_half_scale() {
    let g = group((0, 0), (300, 300), Some((10, 20)), Some((600, 600)));
    let rc_group = RectF::new_with(0.0, 0.0, 300.0, 300.0);
    let mut rc_child = RectF::new_with(10.0, 20.0, 310.0, 320.0);
    ShapeUtil::calcChildRelativeRect(&g, &rc_group, &mut rc_child, 0.0);
    assert_rect(&rc_child, 0.0, 0.0, 150.0, 150.0);
}

#[test]
fn t04_rot_15_no_axis_swap() {
    assert_rect(&scaled_child(15.0), 0.0, 0.0, 200.0, 400.0);
}

#[test]
fn t05_rot_45_swaps_axes() {
    assert_rect(&scaled_child(45.0), -300.0, 150.0, 500.0, 250.0);
}

#[test]
fn t06_rot_135_no_swap() {
    assert_rect(&scaled_child(135.0), 0.0, 0.0, 200.0, 400.0);
}

#[test]
fn t07_rot_315_no_swap() {
    assert_rect(&scaled_child(315.0), 0.0, 0.0, 200.0, 400.0);
}

#[test]
fn t08_rotation_normalized() {
    // -45 → 315, 400 → 40; both no-swap
    assert_rect(&scaled_child(-45.0), 0.0, 0.0, 200.0, 400.0);
    assert_rect(&scaled_child(400.0), 0.0, 0.0, 200.0, 400.0);
}

#[test]
fn t09_two_level_calc_shape2_top_matches_relative() {
    let outer = Rc::new(group((0, 0), (1000, 1000), Some((0, 0)), Some((1000, 1000))));
    let mut inner = group((100, 100), (400, 400), Some((0, 0)), Some((400, 400)));
    inner.parent = Some(outer);
    let inner = Rc::new(inner);
    let mut shape = leaf((10, 20), (50, 60));
    shape.parent = Some(inner.clone());
    let mut rc = RectF::new();
    ShapeUtil::calcRelativeRect__kmoshowshape_rectf(&shape, &mut rc);
    assert_rect(&rc, 10.0, 20.0, 60.0, 80.0);
    let top = ShapeUtil::calcShape2TopMatrix__kmoshowshape_matrix(&inner, None);
    assert_rect(&top, 100.0, 100.0, 500.0, 500.0);
}

#[test]
fn t10_matrix_none_returns_rect_only() {
    let shape = leaf((100, 200), (300, 400));
    let rc = ShapeUtil::calcShape2TopMatrix__kmoshowshape_matrix(&shape, None);
    assert_rect(&rc, 100.0, 200.0, 400.0, 600.0);
}

#[test]
fn t11_cx_zero_scale_is_one() {
    let g = group((0, 0), (300, 300), Some((0, 0)), Some((0, 300)));
    let rc_group = RectF::new_with(0.0, 0.0, 300.0, 300.0);
    let mut rc_child = RectF::new_with(0.0, 0.0, 100.0, 150.0);
    ShapeUtil::calcChildRelativeRect(&g, &rc_group, &mut rc_child, 0.0);
    // scaleX=1 (cx==0), scaleY=300/300=1 → unchanged
    assert_rect(&rc_child, 0.0, 0.0, 100.0, 150.0);
}
