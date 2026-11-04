//! 02_function_showshape_tree — addChild dual-write + KmoShapeTree.shapeCount

use ppt_core::cn::wps::drawing::app::DgStore::DgStore;
use ppt_core::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use ppt_core::cn::wps::show::app::slide::ISlide::{
    Background, ColorMapping, ISlide, KmoPresentation, KmoSlideEventListeners, KmoTheme, KsoWMInfo,
};
use ppt_core::cn::wps::show::util::layout_shape::{
    KmoShapeTree as LayoutTree, KmoShowShape as LayoutShape,
};

struct DummySlide {
    pres: KmoPresentation,
    theme: KmoTheme,
    events: KmoSlideEventListeners,
    bg: Background,
    wm: KsoWMInfo,
    cm: ColorMapping,
}

impl DummySlide {
    fn with_store(store: &mut DgStore) -> Self {
        DummySlide {
            pres: KmoPresentation {
                drawing_store: store as *mut DgStore,
                // rust: ★③-5 —— 真身 `ISlide::KmoPresentation` 新增媒体表字段（`mediaLib()` 供
                //   `SwPictureHandler` 解析 p:pic 媒体）。测试宿主须同步装配，否则 E0063。
                //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
                media_lib: std::rc::Rc::new(std::cell::RefCell::new(
                    ppt_core::cn::wps::show::app::media::KmoMediaLib::KmoMediaLib::new(),
                )),
            },
            theme: KmoTheme,
            events: KmoSlideEventListeners::new(),
            // rust: ★W6c-4 —— 三个真身类型不再是单元结构体 ⇒ 必须构造实例。
            //   owner: tests.app.shape    wave: M-PPT-KMOSLIDE-W6c-4
            bg: Background::new(),
            wm: KsoWMInfo::new(),
            cm: ColorMapping::create(),
        }
    }
}

impl ISlide for DummySlide {
    fn r#type(&self) -> i32 {
        0
    }
    fn id(&self) -> i64 {
        0
    }
    fn getPresentation(&self) -> &KmoPresentation {
        &self.pres
    }
    fn theme(&self) -> &KmoTheme {
        &self.theme
    }
    fn eventListeners(&self) -> &KmoSlideEventListeners {
        &self.events
    }
    fn spTree(&self) -> &KmoShapeTree {
        panic!("DummySlide.spTree unused by 02 tests")
    }
    fn replaceSpid(&self, _var1: i32, _var2: i32) {}
    fn background(&self) -> &Background {
        &self.bg
    }
    fn ksoWMInfo(&self) -> &KsoWMInfo {
        &self.wm
    }
    fn colorMapping(&self) -> &ColorMapping {
        &self.cm
    }
    fn isIOReading(&self) -> bool {
        false
    }
}

#[test]
fn tr0_layout_shape_children_len_is_current_host() {
    let tree = LayoutTree {
        children: vec![LayoutShape {
            id: 1,
            ..LayoutShape::default()
        }],
        ..Default::default()
    };
    assert_eq!(tree.children.len(), 1);
}

#[test]
fn tr1_add_child_len_one() {
    let mut store = DgStore::DgStore_new();
    let gid = store.add(0);
    let cid = store.add(1);
    let dummy = DummySlide::with_store(&mut store);
    let mut root = KmoShowShape::KmoShowShape__islide_int_int(&dummy, 0, gid);
    let child = KmoShowShape::KmoShowShape__islide_int_int(&dummy, 1, cid);
    root.addChild(child);
    assert_eq!(root.childCount(), 1);
    assert_eq!(root.getDgGroup().size(), 1);
    assert_eq!(root.getDgGroup().childAt(0), cid);
}

#[test]
fn tr2_add_child_n() {
    let mut store = DgStore::DgStore_new();
    let gid = store.add(0);
    let c1 = store.add(1);
    let c2 = store.add(1);
    let dummy = DummySlide::with_store(&mut store);
    let mut root = KmoShowShape::KmoShowShape__islide_int_int(&dummy, 0, gid);
    root.addChild(KmoShowShape::KmoShowShape__islide_int_int(&dummy, 1, c1));
    root.addChild(KmoShowShape::KmoShowShape__islide_int_int(&dummy, 1, c2));
    assert_eq!(root.childCount(), 2);
    assert_eq!(root.getDgGroup().size(), 2);
    assert_eq!(root.getDgGroup().childAt(0), c1);
    assert_eq!(root.getDgGroup().childAt(1), c2);
}

#[test]
fn tr3_shape_count_delegates() {
    let mut store = DgStore::DgStore_new();
    let gid = store.add(0);
    let c1 = store.add(1);
    let c2 = store.add(1);
    let dummy = DummySlide::with_store(&mut store);
    let mut tree = KmoShapeTree::KmoShapeTree(&dummy);
    tree.initRoot(KmoShowShape::KmoShowShape__islide_int_int(&dummy, 0, gid));
    tree.addShape(KmoShowShape::KmoShowShape__islide_int_int(&dummy, 1, c1));
    tree.addShape(KmoShowShape::KmoShowShape__islide_int_int(&dummy, 1, c2));
    assert_eq!(tree.shapeCount(), 2);
    assert_eq!(tree.getRoot().childCount(), 2);
}

#[test]
fn tr4_init_root_replaces() {
    let mut store = DgStore::DgStore_new();
    let _occupy = store.add(0);
    let gid = store.add(0);
    let dummy = DummySlide::with_store(&mut store);
    let mut tree = KmoShapeTree::KmoShapeTree(&dummy);
    let ctor_id = tree.getRoot().id();
    let new_root = KmoShowShape::KmoShowShape__islide_int_int(&dummy, 0, gid);
    assert_ne!(gid, ctor_id, "replacement root must not be ctor placeholder id");
    tree.initRoot(new_root);
    assert_eq!(tree.getRoot().id(), gid);
}

#[test]
fn tr5_empty_count() {
    let mut store = DgStore::DgStore_new();
    let dummy = DummySlide::with_store(&mut store);
    let tree = KmoShapeTree::KmoShapeTree(&dummy);
    assert_eq!(tree.shapeCount(), 0);
}
