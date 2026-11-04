// Test 02_function_ph_merge_style — ph 样式继承（pl-03）
// 冻结矩阵 FUNCTION.md §3.2 T01–T08。期望由 Java mergeShapePr 87-127 + ShapePr.follow 父解析手算。
use std::rc::Rc;

use ppt_core::cn::wps::show::layout::PlaceHolderMerger::PlaceHolderMerger;
use ppt_core::cn::wps::show::util::layout_shape::{
    KmoShapeTree, KmoShowShape, KmoSlide, KmoSlideLayout, KmoSlideMaster, Placeholder, ShapePr,
};

fn fill_pr(fill: i32) -> ShapePr {
    ShapePr {
        has_fill: true,
        fill,
        ..ShapePr::default()
    }
}

fn ln_pr(ln: i32) -> ShapePr {
    ShapePr {
        has_ln: true,
        ln,
        ..ShapePr::default()
    }
}

fn ph(
    idx: i32,
    parent_type: i32,
    slide: Option<Rc<KmoSlide>>,
    layout: Option<Rc<KmoSlideLayout>>,
    sp_pr: ShapePr,
) -> KmoShowShape {
    KmoShowShape {
        is_placeholder: true,
        ph: Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx,
        },
        has_parent: true,
        parent_type,
        parent_slide: slide,
        parent_layout: layout,
        sp_pr,
        ..KmoShowShape::default()
    }
}

fn layout_child(idx: i32, sp_pr: ShapePr, master: &Rc<KmoSlideMaster>) -> KmoShowShape {
    let parent_layout = Rc::new(KmoSlideLayout {
        master: Some(master.clone()),
        sp_tree: None,
        ..Default::default()
    });
    KmoShowShape {
        is_placeholder: true,
        has_parent: true,
        parent_type: 1,
        parent_layout: Some(parent_layout),
        ph: Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx,
        },
        sp_pr,
        ..KmoShowShape::default()
    }
}

fn slide_with(layout: Rc<KmoSlideLayout>) -> Rc<KmoSlide> {
    Rc::new(KmoSlide {
        layout: Some(layout),
        ..Default::default()
    })
}

#[test]
fn t01_non_placeholder_own_fill() {
    let shape = KmoShowShape {
        is_placeholder: false,
        sp_pr: fill_pr(42),
        ..KmoShowShape::default()
    };
    let pr = PlaceHolderMerger::mergeShapePr(&shape);
    assert!(pr.hasFill());
    assert_eq!(pr.fill(), 42);
    assert!(!pr.hasLn());
}

#[test]
fn t02_three_level_follow_fill_then_ln() {
    let master_ph = KmoShowShape {
        is_placeholder: true,
        ph: Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx: 0,
        },
        sp_pr: ln_pr(20),
        ..KmoShowShape::default()
    };
    let master = Rc::new(KmoSlideMaster {
        sp_tree: Rc::new(KmoShapeTree {
            children: vec![master_ph], ..Default::default() }),
        ..Default::default()
    });
    let lp = layout_child(2, fill_pr(10), &master);
    let layout = Rc::new(KmoSlideLayout {
        master: Some(master),
        sp_tree: Some(Rc::new(KmoShapeTree { children: vec![lp], ..Default::default() })),
        ..Default::default()
    });
    let shape = ph(2, 0, Some(slide_with(layout)), None, ShapePr::default());
    let pr = PlaceHolderMerger::mergeShapePr(&shape);
    assert_eq!(pr.fill(), 10);
    assert_eq!(pr.ln(), 20);
}

#[test]
fn t03_missing_layout_ph_returns_own() {
    let master = Rc::new(KmoSlideMaster {
        sp_tree: Rc::new(KmoShapeTree::default()), ..Default::default() });
    let layout = Rc::new(KmoSlideLayout {
        master: Some(master),
        sp_tree: Some(Rc::new(KmoShapeTree::default())),
        ..Default::default()
    });
    let shape = ph(2, 0, Some(slide_with(layout)), None, fill_pr(99));
    let pr = PlaceHolderMerger::mergeShapePr(&shape);
    assert_eq!(pr.fill(), 99);
    assert!(!pr.hasLn());
}

#[test]
fn t04_layout_hit_master_miss() {
    let master = Rc::new(KmoSlideMaster {
        sp_tree: Rc::new(KmoShapeTree::default()), ..Default::default() });
    let lp = layout_child(2, fill_pr(10), &master);
    let layout = Rc::new(KmoSlideLayout {
        master: Some(master),
        sp_tree: Some(Rc::new(KmoShapeTree { children: vec![lp], ..Default::default() })),
        ..Default::default()
    });
    let shape = ph(2, 0, Some(slide_with(layout)), None, ShapePr::default());
    let pr = PlaceHolderMerger::mergeShapePr(&shape);
    assert_eq!(pr.fill(), 10);
    assert!(!pr.hasLn());
}

#[test]
fn t05_idx_minus_one_follow_master() {
    let master_ph = KmoShowShape {
        is_placeholder: true,
        ph: Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx: 0,
        },
        sp_pr: fill_pr(77),
        ..KmoShowShape::default()
    };
    let master = Rc::new(KmoSlideMaster {
        sp_tree: Rc::new(KmoShapeTree {
            children: vec![master_ph], ..Default::default() }),
        ..Default::default()
    });
    let layout = Rc::new(KmoSlideLayout {
        master: Some(master),
        sp_tree: Some(Rc::new(KmoShapeTree::default())),
        ..Default::default()
    });
    let shape = ph(-1, 0, Some(slide_with(layout)), None, ShapePr::default());
    let pr = PlaceHolderMerger::mergeShapePr(&shape);
    assert_eq!(pr.fill(), 77);
}

#[test]
fn t06_layout_parent_follow_master() {
    let master_ph = KmoShowShape {
        is_placeholder: true,
        ph: Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx: 0,
        },
        sp_pr: fill_pr(55),
        ..KmoShowShape::default()
    };
    let master = Rc::new(KmoSlideMaster {
        sp_tree: Rc::new(KmoShapeTree {
            children: vec![master_ph], ..Default::default() }),
        ..Default::default()
    });
    let parent_layout = Rc::new(KmoSlideLayout {
        master: Some(master),
        sp_tree: None,
        ..Default::default()
    });
    let shape = ph(-1, 1, None, Some(parent_layout), ShapePr::default());
    let pr = PlaceHolderMerger::mergeShapePr(&shape);
    assert_eq!(pr.fill(), 55);
}

#[test]
fn t07_master_parent_type_passthrough() {
    let shape = KmoShowShape {
        is_placeholder: true,
        has_parent: true,
        parent_type: 2,
        sp_pr: fill_pr(3),
        ph: Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx: 0,
        },
        ..KmoShowShape::default()
    };
    let pr = PlaceHolderMerger::mergeShapePr(&shape);
    assert_eq!(pr.fill(), 3);
}

#[test]
fn t08_own_missing_fill_from_layout() {
    let master = Rc::new(KmoSlideMaster {
        sp_tree: Rc::new(KmoShapeTree::default()), ..Default::default() });
    let lp = layout_child(2, fill_pr(10), &master);
    let layout = Rc::new(KmoSlideLayout {
        master: Some(master),
        sp_tree: Some(Rc::new(KmoShapeTree { children: vec![lp], ..Default::default() })),
        ..Default::default()
    });
    let shape = ph(2, 0, Some(slide_with(layout)), None, ShapePr::default());
    let pr = PlaceHolderMerger::mergeShapePr(&shape);
    assert!(!shape.sp_pr.has_fill);
    assert_eq!(pr.fill(), 10);
}
