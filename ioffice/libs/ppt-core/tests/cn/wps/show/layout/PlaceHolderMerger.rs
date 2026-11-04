// Test 01_function_ph_merge_transform — ph 位置继承（pl-03）
// 冻结矩阵 FUNCTION.md §3.2 T01–T11。期望由 Java PlaceHolderMerger.mergeTransform 53-83 手算。
use std::rc::Rc;

use ppt_core::cn::wps::show::layout::PlaceHolderMerger::PlaceHolderMerger;
use ppt_core::cn::wps::show::util::layout_shape::{
    KmoShapeTree, KmoShowShape, KmoSlide, KmoSlideLayout, KmoSlideMaster, Placeholder, Transform2D,
};

fn assert_xfrm(got: &Transform2D, x: i32, y: i32, cx: i32, cy: i32) {
    assert_eq!((got.x, got.y, got.cx, got.cy), (x, y, cx, cy));
}

fn ph_shape(
    has_xfrm: bool,
    off: (i32, i32),
    ext: (i32, i32),
    ph: Placeholder,
    layout: Option<Rc<KmoSlideLayout>>,
) -> KmoShowShape {
    let mut s = KmoShowShape {
        is_placeholder: true,
        has_xfrm,
        transform2DX: off.0,
        transform2DY: off.1,
        transform2DW: ext.0,
        transform2DH: ext.1,
        ph,
        has_parent: true,
        parent_type: 0,
        parent_slide: Some(Rc::new(KmoSlide {
            layout,
            ..Default::default()
        })),
        ..KmoShowShape::default()
    };
    s.is_placeholder = true;
    s
}

fn layout_of(tree: KmoShapeTree, master: Option<Rc<KmoSlideMaster>>) -> Rc<KmoSlideLayout> {
    Rc::new(KmoSlideLayout {
        master,
        sp_tree: Some(Rc::new(tree)),
        ..Default::default()
    })
}

fn master_of(children: Vec<KmoShowShape>) -> Rc<KmoSlideMaster> {
    Rc::new(KmoSlideMaster {
        sp_tree: Rc::new(KmoShapeTree { children, ..Default::default() }),
        ..Default::default()
    })
}

fn layout_ph(idx: i32, xfrm: (i32, i32, i32, i32), ph_type: i32, has_type: bool) -> KmoShowShape {
    KmoShowShape {
        is_placeholder: true,
        has_xfrm: true,
        transform2DX: xfrm.0,
        transform2DY: xfrm.1,
        transform2DW: xfrm.2,
        transform2DH: xfrm.3,
        ph: Placeholder {
            has_type,
            type_: ph_type,
            has_idx: true,
            idx,
        },
        ..KmoShowShape::default()
    }
}

#[test]
fn t01_non_placeholder_identity() {
    let shape = KmoShowShape {
        is_placeholder: false,
        has_xfrm: true,
        transform2DX: 10,
        transform2DY: 20,
        transform2DW: 30,
        transform2DH: 40,
        ..KmoShowShape::default()
    };
    let t = PlaceHolderMerger::mergeTransform(&shape);
    assert_xfrm(&t, 10, 20, 30, 40);
}

#[test]
fn t02_own_xfrm_wins() {
    let layout = layout_of(KmoShapeTree::default(), None);
    let shape = ph_shape(
        true,
        (100, 200),
        (300, 400),
        Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx: 3,
        },
        Some(layout),
    );
    let t = PlaceHolderMerger::mergeTransform(&shape);
    assert_xfrm(&t, 100, 200, 300, 400);
}

#[test]
fn t03_layout_ph_by_idx() {
    let lp = layout_ph(3, (5, 6, 7, 8), 1, true);
    let layout = layout_of(KmoShapeTree { children: vec![lp], ..Default::default() }, None);
    let shape = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx: 3,
        },
        Some(layout),
    );
    let t = PlaceHolderMerger::mergeTransform(&shape);
    assert_xfrm(&t, 5, 6, 7, 8);
}

#[test]
fn t04_no_idx_uses_index_zero() {
    let lp = layout_ph(0, (9, 10, 11, 12), 1, true);
    let layout = layout_of(KmoShapeTree { children: vec![lp], ..Default::default() }, None);
    let shape = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: true,
            type_: 1,
            has_idx: false,
            idx: 99,
        },
        Some(layout),
    );
    let t = PlaceHolderMerger::mergeTransform(&shape);
    assert_xfrm(&t, 9, 10, 11, 12);
}

fn master_title_body() -> Rc<KmoSlideMaster> {
    let title = layout_ph(0, (1, 2, 3, 4), 0, true);
    let body = layout_ph(1, (11, 12, 13, 14), 1, true);
    master_of(vec![title, body])
}

#[test]
fn t05_layout_miss_scans_master_title() {
    let master = master_title_body();
    let layout = layout_of(KmoShapeTree::default(), Some(master));
    let shape = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: true,
            type_: 0,
            has_idx: true,
            idx: 3,
        },
        Some(layout),
    );
    let t = PlaceHolderMerger::mergeTransform(&shape);
    assert_xfrm(&t, 1, 2, 3, 4);
}

#[test]
fn t06_layout_miss_scans_master_body() {
    let master = master_title_body();
    let layout = layout_of(KmoShapeTree::default(), Some(master));
    let shape = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx: 3,
        },
        Some(layout),
    );
    let t = PlaceHolderMerger::mergeTransform(&shape);
    assert_xfrm(&t, 11, 12, 13, 14);
}

#[test]
fn t07_as_title_ctr_and_subtitle() {
    let master = master_title_body();
    let layout = layout_of(KmoShapeTree::default(), Some(master.clone()));
    let ctr = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: true,
            type_: 2,
            has_idx: true,
            idx: 3,
        },
        Some(layout.clone()),
    );
    assert_xfrm(&PlaceHolderMerger::mergeTransform(&ctr), 1, 2, 3, 4);
    let sub = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: true,
            type_: 3,
            has_idx: true,
            idx: 3,
        },
        Some(layout),
    );
    assert_xfrm(&PlaceHolderMerger::mergeTransform(&sub), 11, 12, 13, 14);
}

#[test]
fn t08_idx_minus_one_skips_layout_uses_master_ph() {
    let master = master_title_body();
    let layout = layout_of(KmoShapeTree::default(), Some(master));
    let shape = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx: -1,
        },
        Some(layout),
    );
    let t = PlaceHolderMerger::mergeTransform(&shape);
    assert_xfrm(&t, 11, 12, 13, 14);
}

#[test]
fn t09_find_master_ph_type_convert() {
    let t8 = layout_ph(0, (80, 81, 82, 83), 8, true);
    let t0 = layout_ph(0, (0, 1, 2, 3), 0, true);
    let t1 = layout_ph(0, (10, 11, 12, 14), 1, true);
    let master = master_of(vec![t8, t0, t1]);
    let layout = layout_of(KmoShapeTree::default(), Some(master));
    // no type → find 8 first
    let no_type = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: false,
            type_: 0,
            has_idx: true,
            idx: -1,
        },
        Some(layout.clone()),
    );
    assert_xfrm(&PlaceHolderMerger::mergeTransform(&no_type), 80, 81, 82, 83);
    // type 2 → realType 2 miss, convert 0
    let ctr = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: true,
            type_: 2,
            has_idx: true,
            idx: -1,
        },
        Some(layout.clone()),
    );
    assert_xfrm(&PlaceHolderMerger::mergeTransform(&ctr), 0, 1, 2, 3);
    // type 3 → convert 1
    let sub = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: true,
            type_: 3,
            has_idx: true,
            idx: -1,
        },
        Some(layout),
    );
    assert_xfrm(&PlaceHolderMerger::mergeTransform(&sub), 10, 11, 12, 14);
}

#[test]
fn t10_fallback_empty_xfrm() {
    let master = master_of(vec![]);
    let layout = layout_of(KmoShapeTree::default(), Some(master));
    let shape = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: true,
            type_: 7,
            has_idx: true,
            idx: -1,
        },
        Some(layout),
    );
    let t = PlaceHolderMerger::mergeTransform(&shape);
    assert_xfrm(&t, 0, 0, 0, 0);
}

#[test]
fn t11_null_layout_no_panic() {
    let shape = ph_shape(
        false,
        (0, 0),
        (0, 0),
        Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx: 3,
        },
        None,
    );
    let t = PlaceHolderMerger::mergeTransform(&shape);
    assert_xfrm(&t, 0, 0, 0, 0);
}
