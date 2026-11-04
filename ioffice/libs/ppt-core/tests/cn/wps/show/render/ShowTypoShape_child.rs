// Test 02_function_child_walk — group 子级递归（pl-06）
// 冻结矩阵 FUNCTION.md §3.2 CW1–CW7。
use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use ppt_core::cn::wps::show::convert::convert_kmo_harness::ConvertKmo;
use ppt_core::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use ppt_core::cn::wps::show::util::layout_shape::KmoShowShape as LayoutShape;

fn leaf(id: i32) -> LayoutShape {
    LayoutShape {
        id,
        shape_type: 1,
        ..LayoutShape::default()
    }
}

fn group(id: i32, kids: Vec<LayoutShape>) -> LayoutShape {
    LayoutShape {
        id,
        shape_type: 0,
        children: kids,
        ..LayoutShape::default()
    }
}

fn snap(shape: Option<LayoutShape>) -> ShowTypoShape {
    ShowTypoShape::typo_shape_snapshot(
        shape.map(|s| ConvertKmo::from_layout_tree(&s).into_shape()),
        None,
        None,
    )
}

fn leaf_spid(index: i32) -> i32 {
    ConvertKmo::encode_spid(1, index)
}

#[test]
fn cw1_non_group_empty() {
    let mut p = snap(Some(leaf(1)));
    assert_eq!(p.getChildCount(), 0);
    assert!(p.getChild(0).is_none());
}

#[test]
fn cw2_group_count_and_order() {
    let mut p = snap(Some(group(10, vec![leaf(1), leaf(2), leaf(3)])));
    assert_eq!(p.getChildCount(), 3);
    let ids: Vec<i32> = (0..3)
        .map(|i| p.getChild(i).unwrap().getShowShape().unwrap().id())
        .collect();
    assert_eq!(ids, vec![leaf_spid(1), leaf_spid(2), leaf_spid(3)]);
}

#[test]
fn cw3_out_of_bounds() {
    let mut p = snap(Some(group(10, vec![leaf(1), leaf(2), leaf(3)])));
    assert!(p.getChild(3).is_none());
    assert!(p.getChild(-1).is_none());
}

#[test]
fn cw4_reuse_same_child() {
    let mut p = snap(Some(group(10, vec![leaf(1), leaf(2), leaf(3)])));
    let a = p.getChild(1).unwrap();
    assert_eq!(p.childCacheLen(), 1);
    let b = p.getChild(1).unwrap();
    assert_eq!(p.childCacheLen(), 1);
    assert_eq!(a.getShowShape().unwrap().id(), b.getShowShape().unwrap().id());
    assert_eq!(a.getShowShape().unwrap().id(), leaf_spid(2));
}

#[test]
fn cw5_lazy_construct() {
    let mut p = snap(Some(group(10, vec![leaf(1), leaf(2), leaf(3)])));
    assert_eq!(p.childCacheLen(), 0);
    let _ = p.getChild(0);
    assert_eq!(p.childCacheLen(), 1);
}

#[test]
fn cw6_nested_group() {
    let inner = group(20, vec![leaf(21), leaf(22)]);
    let mut outer = snap(Some(group(10, vec![inner])));
    let mut inner_typo = outer.getChild(0).unwrap();
    assert_eq!(inner_typo.getChildCount(), 2);
    assert_eq!(
        inner_typo.getChild(0).unwrap().getShowShape().unwrap().id(),
        leaf_spid(21)
    );
}

#[test]
fn cw7_null_parent() {
    let mut p = snap(None);
    assert_eq!(p.getChildCount(), 0);
    assert!(p.getChild(0).is_none());
}

#[allow(dead_code)]
fn _touch_app(_: &KmoShowShape) {}

