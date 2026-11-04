// Test 01_function_shape_absolute_rect — 单 shape 绝对 EMU 框（pl-02）
// 冻结矩阵 FUNCTION.md §3.2 T01–T07。期望由 Java ShapeUtil.calcRelativeRect /
// calcChildRelativeRect / toDegree 公式手算（koffice presentation ShapeUtil.java）。
use std::rc::Rc;

use ppt_core::cn::wps::show::util::layout_shape::KmoShowShape;
use ppt_core::cn::wps::show::util::ShapeUtil::ShapeUtil;
use util::cn::wps::graphics::RectF::RectF;

fn rect() -> RectF {
    RectF::new()
}

fn assert_rect(rc: &RectF, left: f32, top: f32, right: f32, bottom: f32) {
    assert_eq!(
        (rc.left, rc.top, rc.right, rc.bottom),
        (left, top, right, bottom),
        "rect mismatch"
    );
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

fn group(off: (i32, i32), ext: (i32, i32), ch_off: (i32, i32), ch_ext: (i32, i32)) -> KmoShowShape {
    KmoShowShape {
        shape_type: 0,
        transform2DX: off.0,
        transform2DY: off.1,
        transform2DW: ext.0,
        transform2DH: ext.1,
        child_off: Some(ch_off),
        child_ext: Some(ch_ext),
        ..KmoShowShape::default()
    }
}

#[test]
fn t01_no_parent_absolute_rect() {
    let shape = leaf((100, 200), (300, 400));
    let mut rc = rect();
    ShapeUtil::calcRelativeRect__kmoshowshape_rectf(&shape, &mut rc);
    assert_rect(&rc, 100.0, 200.0, 400.0, 600.0);
}

#[test]
fn t02_group_direct_child_one_to_one() {
    let parent = Rc::new(group((1000, 2000), (300, 300), (0, 0), (300, 300)));
    let mut child = leaf((0, 0), (300, 300));
    child.parent = Some(parent);
    let mut rc = rect();
    ShapeUtil::calcRelativeRect__kmoshowshape_rectf(&child, &mut rc);
    assert_rect(&rc, 0.0, 0.0, 300.0, 300.0);
}

#[test]
fn t03_two_level_nested_groups() {
    let outer = Rc::new(group((0, 0), (1000, 1000), (0, 0), (1000, 1000)));
    let mut inner = group((100, 100), (400, 400), (0, 0), (400, 400));
    inner.parent = Some(outer);
    let inner = Rc::new(inner);
    let mut shape = leaf((10, 20), (50, 60));
    shape.parent = Some(inner);
    let mut rc = rect();
    ShapeUtil::calcRelativeRect__kmoshowshape_rectf(&shape, &mut rc);
    assert_rect(&rc, 10.0, 20.0, 60.0, 80.0);
}

#[test]
fn t04_calc_self_false_keeps_nonempty_rect() {
    let shape = leaf((100, 200), (300, 400));
    let mut rc = RectF::new_with(1.0, 2.0, 3.0, 4.0);
    ShapeUtil::calcRelativeRect__kmoshowshape_rectf_boolean(&shape, &mut rc, false);
    assert_rect(&rc, 1.0, 2.0, 3.0, 4.0);
}

#[test]
fn t05_can_rotate_false_does_not_use_rotation() {
    // Non-uniform scale: leaked 45° would swap axes; canRotate=false must keep no-swap.
    let parent = Rc::new(group((0, 0), (200, 400), (0, 0), (400, 200)));
    let mut child = leaf((0, 0), (400, 200));
    child.rotation = 2_700_000; // 45°
    child.can_rotate = false;
    child.parent = Some(parent);
    let mut rc = rect();
    ShapeUtil::calcRelativeRect__kmoshowshape_rectf(&child, &mut rc);
    assert_rect(&rc, 0.0, 0.0, 200.0, 400.0);
}

#[test]
fn t06_empty_xfrm_is_zero_rect() {
    let shape = KmoShowShape::default();
    let mut rc = rect();
    ShapeUtil::calcRelativeRect__kmoshowshape_rectf(&shape, &mut rc);
    assert_rect(&rc, 0.0, 0.0, 0.0, 0.0);
}

#[test]
fn t07_to_degree_and_to_rotation() {
    assert_eq!(ShapeUtil::toDegree(900_000), 15.0);
    assert_eq!(ShapeUtil::toRotation(15.0), 900_000);
}
