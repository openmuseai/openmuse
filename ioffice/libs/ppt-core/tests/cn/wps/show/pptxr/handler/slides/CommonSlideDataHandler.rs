//! 04_function_common_slide_data — 真实 `CommonSlideDataHandler` 的 p:cSld 装配。
//!
//! W5 测试面收口（见 M_PPT_SLIDE_ASM_BASELINE.md §NN）：原文件用影子 crate
//! （`pub mod cn { ... }` + 三个 `#[path]` 包含）把真实 `CommonSlideDataHandler`
//! 编译在一套自造依赖类型上，并额外把**未登记进 crate 的** `BackgroundHandler.rs` /
//! `ControlListHandler.rs` 也 `#[path]` 进来断言。shipped 依赖面是
//! `...::handler::shape::sw_shape_hosts`（`*mut DgStore` + `KmoFilm`），影子 crate
//! 里没有该模块，且 `slides/mod.rs` 只登记 `CommonSlideDataHandler` 与 `SlideHandler`。
//!
//! 本 target 改按真实 API 重写：真实 `KmoFilm` + 真实 `KmoShapeTree` + 真实
//! `CommonSlideDataHandler`。被退役的断言（BackgroundHandler / ControlListHandler
//! 影子断言、假 `commonSlideData.name`）记在 baseline §NN。

#![allow(non_snake_case, dead_code, unused_imports)]

use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallbackState, HandlerCallback__mode,
};
use ppt_core::cn::wps::show::app::slide::KmoFilm::KmoFilm;
// rust: ★③-5 —— 真身 handler 的关联类型已收敛到**真身模型/工具类型**：
//   `CommonSlideDataHandler<C>` 要求 `C: HandlerCallback<ShapePr = data::shape::ShapePr,
//   ShapeStyle = data::style::ShapeStyle, Hyperlink = data::hyperlink::Hyperlink,
//   FileManager = util::…::FileManager>`。原文件从 `sw_shape_hosts`（假编译面空壳）取同名类型
//   ⇒ E0271 类型身份不符（`expected RealType, found FakeType`）。故按真身包路径导入。
//   `XmlRoAttr` 仍取自影子面（本文件自身的属性注入面，被测源已不吃该 trait）。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
use ppt_core::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use ppt_core::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
use util::cn::wps::util::manager::FileManager::FileManager;
// rust: ★M-PPT-KMOSLIDE W6c-3 —— 影子契约 ① （`sw_shape_hosts::{ElementHandler, XmlRoAttr}`）
//   已整体删除（`p:bg` 已接真身 handler）。本 target 原先用影子契约做**属性注入面**，
//   现改用**规范契约 ③** 的真实属性表 `XmlRoAttrList`（`CommonSlideDataHandler` 所在契约）。
//   owner: tests.pptxr.handler.slides    wave: M-PPT-KMOSLIDE-W6c-3
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler as CanonicalElementHandler;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::ImportHost;
use ppt_core::cn::wps::show::pptxr::handler::slides::CommonSlideDataHandler::CommonSlideDataHandler;
use std::cell::RefCell;
use std::rc::Rc;

// Java: case 3211295 — p:extLst（不派发）
const CSLD_EXT_LST: i32 = 3211295;
// Java: case 3211308 — p:bg
const CSLD_BG: i32 = 3211308;
// Java: case 3211519 — p:custDataLst
const CSLD_CUST_DATA_LST: i32 = 3211519;
// Java: case 3211606 — p:spTree
const CSLD_SP_TREE: i32 = 3211606;
// Java: case 3211607 — p:controls
const CSLD_CONTROLS: i32 = 3211607;
// Java: case 3211449 — p:cSld @name
const CSLD_NAME: i32 = 3211449;
// Java: case 3211593 — p:sp
const GRP_SP: i32 = 3211593;

// ---------------------------------------------------------------------------
// 真实宿主：产品自己的 `ImportHost`（`impl ISlide` + 泄漏的 `DgStore`）
// ---------------------------------------------------------------------------

struct RecordingCallback {
    state: HandlerCallbackState,
}

impl HandlerCallback for RecordingCallback {
    type DiagramPair = ();
    type Blip = ();
    type Color = ();
    type Hyperlink = Hyperlink;
    type Media = ();
    type AudioFile = ();
    type QuickTimeFile = ();
    type VideoFile = ();
    type EmbeddedWAVAudioFile = ();
    type Model3D = ();
    type Shape = ();
    type ShapePr = ShapePr;
    type ShapeStyle = ShapeStyle;
    type TextRun = ();
    type TagLst = ();
    type FileManager = FileManager;

    fn callback_state(&self) -> &HandlerCallbackState {
        &self.state
    }
}

/// 属性注入面：规范契约 ③ 的**真实**属性表 `XmlRoAttrList`
/// （`add_attr(id, value)` 逐项累积；`get_by_name(id)` 与 Java `XmlRoAttr.getByName` 同名同义）。
fn attrs(values: &[(i32, &str)]) -> XmlRoAttrList {
    let mut list = XmlRoAttrList::new();
    for (k, v) in values {
        list.add_attr(*k, v);
    }
    list
}

struct Env {
    host: &'static ImportHost,
    film: Rc<RefCell<KmoFilm>>,
    cb: Rc<RefCell<RecordingCallback>>,
}

fn env() -> Env {
    let host = ImportHost::leak();
    let film = Rc::new(RefCell::new(KmoFilm::KmoFilm(host)));
    let cb = Rc::new(RefCell::new(RecordingCallback {
        state: HandlerCallback__mode(0),
    }));
    Env { host, film, cb }
}

fn handler(e: &Env) -> CommonSlideDataHandler<RecordingCallback> {
    CommonSlideDataHandler::<RecordingCallback>::CommonSlideDataHandler(
        e.film.clone(),
        e.host.store,
        Some(e.cb.clone()),
    )
}

fn shape_count(film: &Rc<RefCell<KmoFilm>>) -> i32 {
    film.borrow_mut().spTree().shapeCount()
}

// ---------------------------------------------------------------------------
// 断言
// ---------------------------------------------------------------------------

#[test]
fn csld1_enter_sub_element_dispatch_table() {
    let e = env();
    let mut h = handler(&e);
    // p:extLst 与未知元素不派发
    for id in [CSLD_EXT_LST, -1, 0] {
        assert!(h.enterSubElement(id).is_none(), "subElement {id} 不应派发");
    }
    // p:bg / p:custDataLst / p:spTree / p:controls 都派发
    for id in [CSLD_BG, CSLD_CUST_DATA_LST, CSLD_SP_TREE, CSLD_CONTROLS] {
        assert!(h.enterSubElement(id).is_some(), "subElement {id} 必须派发");
    }
    // p:spTree 交给真实 SwGroupShapeHandler
    // （`sw_shape_hosts::ElementHandler` 没有 `kind()`，用"能继续派发 p:sp"证明是组处理器）
    let mut group = h.enterSubElement(CSLD_SP_TREE).unwrap();
    assert!(group.enterSubElement(GRP_SP).is_some(), "组可继续派发 p:sp");
}

#[test]
fn csld2_stub_children_have_no_subtree() {
    let e = env();
    let mut h = handler(&e);
    // p:bg / p:custDataLst / p:controls 在 shipped 依赖面是空壳 ElementHandler：
    // 派发得到对象，但不再向下展开。
    for id in [CSLD_BG, CSLD_CUST_DATA_LST, CSLD_CONTROLS] {
        let mut child = h.enterSubElement(id).unwrap();
        assert!(
            child.enterSubElement(CSLD_SP_TREE).is_none(),
            "subElement {id} 的空壳不再派发"
        );
    }
}

#[test]
fn csld3_end_element_installs_shape_tree_into_film() {
    let e = env();
    let mut h = handler(&e);
    // 构造后 film.spTree() 还是 KmoFilm 自己的空根
    assert_eq!(shape_count(&e.film), 0);

    let mut group = h.enterSubElement(CSLD_SP_TREE).expect("3211606 → 组装配");
    let mut leaf = group.enterSubElement(GRP_SP).expect("3211593 → 形状");
    leaf.endElement(GRP_SP);
    group.endElement(CSLD_SP_TREE);
    // Java: this.film.spTree().initRoot(this.shapeTree);
    h.endElement(CSLD_SP_TREE);

    assert_eq!(
        shape_count(&e.film),
        1,
        "p:cSld/p:spTree 的 p:sp 应成为 film.spTree() 根的子节点"
    );
    let mut film = e.film.borrow_mut();
    let root = film.spTree().getRoot();
    assert_eq!(root.childCount(), 1);
    assert_eq!(root.childAt(0).r#type(), 1);
}

#[test]
fn csld4_start_element_consumes_name_attribute_without_changing_tree() {
    let e = env();
    let mut h = handler(&e);
    // Java: this.commonSlideData.putName(tmp.toString())
    CanonicalElementHandler::startElement(&h, CSLD_SP_TREE, &attrs(&[(CSLD_NAME, "Slide 1")]));
    // 无属性 / 非目标属性都不改变装配结果
    CanonicalElementHandler::startElement(&h, CSLD_SP_TREE, &attrs(&[]));
    CanonicalElementHandler::startElement(&h, CSLD_SP_TREE, &attrs(&[(3211450, "x")]));

    let mut group = h.enterSubElement(CSLD_SP_TREE).unwrap();
    let mut leaf = group.enterSubElement(GRP_SP).unwrap();
    leaf.endElement(GRP_SP);
    group.endElement(CSLD_SP_TREE);
    h.endElement(CSLD_SP_TREE);
    assert_eq!(shape_count(&e.film), 1);
}
