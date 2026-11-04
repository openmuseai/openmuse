use std::rc::Rc;

use ppt_core::cn::wps::show::layout::page_inheritance::{resolve_page, resolve_pages};
use ppt_core::cn::wps::show::util::layout_shape::{
    KmoShapeTree, KmoShowShape, KmoSlide, KmoSlideLayout, KmoSlideMaster, Placeholder,
};

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

fn layout_ph(idx: i32, xfrm: (i32, i32, i32, i32), ph_type: i32) -> KmoShowShape {
    KmoShowShape {
        is_placeholder: true,
        has_xfrm: true,
        transform2DX: xfrm.0,
        transform2DY: xfrm.1,
        transform2DW: xfrm.2,
        transform2DH: xfrm.3,
        ph: Placeholder {
            has_type: true,
            type_: ph_type,
            has_idx: true,
            idx,
        },
        ..KmoShowShape::default()
    }
}

fn slide_with(layout: Option<Rc<KmoSlideLayout>>, children: Vec<KmoShowShape>) -> KmoSlide {
    KmoSlide {
        layout,
        sp_tree: KmoShapeTree {
            children,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn t01_own_xfrm() {
    let mut slide = slide_with(None, vec![leaf((100, 200), (300, 400))]);
    let resolved = resolve_page(&mut slide);
    assert_eq!(resolved.len(), 1);
    assert_eq!(
        (resolved[0].xfrm.x, resolved[0].xfrm.y, resolved[0].xfrm.cx, resolved[0].xfrm.cy),
        (100, 200, 300, 400)
    );
    assert_eq!(
        (resolved[0].abs.left, resolved[0].abs.top, resolved[0].abs.right, resolved[0].abs.bottom),
        (100.0, 200.0, 400.0, 600.0)
    );
}

#[test]
fn t02_layout_fallback() {
    let lp = layout_ph(3, (5, 6, 7, 8), 1);
    let layout = layout_of(KmoShapeTree { children: vec![lp], ..Default::default() }, None);
    let ph = KmoShowShape {
        is_placeholder: true,
        has_xfrm: false,
        has_parent: true,
        parent_type: 0,
        parent_slide: Some(Rc::new(KmoSlide {
            layout: Some(layout.clone()),
            ..Default::default()
        })),
        ph: Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx: 3,
        },
        ..KmoShowShape::default()
    };
    let mut slide = slide_with(Some(layout), vec![ph]);
    let resolved = resolve_page(&mut slide);
    assert_eq!(
        (resolved[0].xfrm.x, resolved[0].xfrm.y, resolved[0].xfrm.cx, resolved[0].xfrm.cy),
        (5, 6, 7, 8)
    );
}

#[test]
fn t03_master_fallback() {
    let title = layout_ph(0, (1, 2, 3, 4), 0);
    let body = layout_ph(1, (11, 12, 13, 14), 1);
    let master = master_of(vec![title, body]);
    let layout = layout_of(KmoShapeTree::default(), Some(master));
    let ph = KmoShowShape {
        is_placeholder: true,
        has_xfrm: false,
        has_parent: true,
        parent_type: 0,
        parent_slide: Some(Rc::new(KmoSlide {
            layout: Some(layout.clone()),
            ..Default::default()
        })),
        ph: Placeholder {
            has_type: true,
            type_: 1,
            has_idx: true,
            idx: 3,
        },
        ..KmoShowShape::default()
    };
    let mut slide = slide_with(Some(layout), vec![ph]);
    let resolved = resolve_page(&mut slide);
    assert_eq!(
        (resolved[0].xfrm.x, resolved[0].xfrm.y, resolved[0].xfrm.cx, resolved[0].xfrm.cy),
        (11, 12, 13, 14)
    );
}

#[test]
fn t04_group_child_off_ext() {
    let child = leaf((10, 20), (50, 60));
    let group = KmoShowShape {
        shape_type: 0,
        has_xfrm: true,
        transform2DX: 1000,
        transform2DY: 2000,
        transform2DW: 300,
        transform2DH: 300,
        child_off: Some((0, 0)),
        child_ext: Some((300, 300)),
        children: vec![child],
        ..KmoShowShape::default()
    };
    let mut slide = slide_with(None, vec![group]);
    let resolved = resolve_page(&mut slide);
    assert_eq!(resolved.len(), 2);
    assert!(resolved[1].abs.left.is_finite());
    assert!(resolved[1].abs.top.is_finite());
    assert!(resolved[1].abs.right >= resolved[1].abs.left);
    assert!(resolved[1].abs.bottom >= resolved[1].abs.top);
}

#[test]
fn t05_twenty_three_pages_finite_nonneg() {
    let mut slides: Vec<KmoSlide> = (0..23)
        .map(|i| slide_with(None, vec![leaf((i * 100, 0), (50, 50))]))
        .collect();
    let pages = resolve_pages(&mut slides);
    assert_eq!(pages.len(), 23);
    for (i, page) in pages.iter().enumerate() {
        assert_eq!(page.len(), 1, "page {i}");
        let r = &page[0].abs;
        assert!(r.left.is_finite() && r.top.is_finite() && r.right.is_finite() && r.bottom.is_finite());
        assert!(r.left >= 0.0 && r.top >= 0.0);
        assert_eq!(r.left, (i as f32) * 100.0);
    }
}
