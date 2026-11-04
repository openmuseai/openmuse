//! 01_function_sw_group_on_mod — SwGroup 默认编译面 + enterSubElement 分发

use ppt_core::cn::wps::drawing::app::DgStore::DgStore;
use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallbackState,
};
use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use ppt_core::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
use ppt_core::cn::wps::show::app::slide::ISlide::{
    Background, ColorMapping, ISlide, KmoPresentation, KmoSlideEventListeners, KmoTheme, KsoWMInfo,
};
// rust: ★③-5 —— 源侧 handler 已从影子面收敛到**真身**类型（PLAN §26 判据 2.5），
//   本测试的 `DummyCb` 关联类型必须随之对齐，否则 `SwGroupShapeHandler<DummyCb>` 的约束不成立。
use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use ppt_core::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
use ppt_core::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
use util::cn::wps::util::manager::FileManager::FileManager;
use ppt_core::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::SwGroupShapeHandler;
use std::cell::RefCell;
use std::rc::Rc;

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
                // rust: ★③-5 —— `ISlide::KmoPresentation` 新增真实媒体表字段（见源侧登记）。
                media_lib: std::rc::Rc::new(RefCell::new(
                    ppt_core::cn::wps::show::app::media::KmoMediaLib::KmoMediaLib::new(),
                )),
            },
            theme: KmoTheme,
            events: KmoSlideEventListeners::new(),
            // rust: ★W6c-4 —— 三个真身类型不再是单元结构体 ⇒ 必须构造实例。
            //   owner: tests    wave: M-PPT-KMOSLIDE-W6c-4
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
        panic!("unused")
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

struct DummyCb {
    state: HandlerCallbackState,
}
impl HandlerCallback for DummyCb {
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

fn group_handler(
    dummy: &DummySlide,
    store: &mut DgStore,
) -> (Rc<RefCell<KmoShowShape>>, SwGroupShapeHandler<DummyCb>) {
    let gid = store.add(0);
    let kmo = Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int_int(
        dummy, 0, gid,
    )));
    let h = SwGroupShapeHandler::<DummyCb>::SwGroupShapeHandler(kmo.clone(), None);
    (kmo, h)
}

#[test]
fn sg_mod() {
    let mut store = DgStore::DgStore_new();
    let dummy = DummySlide::with_store(&mut store);
    let (_kmo, _h) = group_handler(&dummy, &mut store);
}

#[test]
fn sg_sp() {
    let mut store = DgStore::DgStore_new();
    let dummy = DummySlide::with_store(&mut store);
    let (kmo, mut h) = group_handler(&dummy, &mut store);
    let handler = h.enterSubElement(3211593);
    assert!(handler.is_some(), "3211593 must return SwShapeHandler");
    assert_eq!(kmo.borrow().childCount(), 1);
    assert_eq!(kmo.borrow().childAt(0).r#type(), 1);
}

#[test]
fn sg_grp() {
    let mut store = DgStore::DgStore_new();
    let dummy = DummySlide::with_store(&mut store);
    let (kmo, mut h) = group_handler(&dummy, &mut store);
    let handler = h.enterSubElement(3211594);
    assert!(handler.is_some(), "3211594 must return nested SwGroupShapeHandler");
    assert_eq!(kmo.borrow().childCount(), 1);
    assert_eq!(kmo.borrow().childAt(0).r#type(), 0);
}

#[test]
fn sg_pic() {
    let mut store = DgStore::DgStore_new();
    let dummy = DummySlide::with_store(&mut store);
    let (kmo, mut h) = group_handler(&dummy, &mut store);
    let handler = h.enterSubElement(3211469);
    assert!(handler.is_some(), "3211469 must return SwPictureHandler");
    assert_eq!(kmo.borrow().childCount(), 1);
    assert_eq!(kmo.borrow().childAt(0).r#type(), 2);
}

#[test]
fn sg_gf() {
    let mut store = DgStore::DgStore_new();
    let dummy = DummySlide::with_store(&mut store);
    let (kmo, mut h) = group_handler(&dummy, &mut store);
    let handler = h.enterSubElement(3211595);
    assert!(handler.is_some(), "3211595 must return SwGraphicFrameHandler");
    assert_eq!(kmo.borrow().childCount(), 1);
    assert_eq!(kmo.borrow().childAt(0).r#type(), 4);
}

#[test]
fn sg_unk() {
    let mut store = DgStore::DgStore_new();
    let dummy = DummySlide::with_store(&mut store);
    let (kmo, mut h) = group_handler(&dummy, &mut store);
    let handler = h.enterSubElement(0);
    assert!(handler.is_none(), "unknown subElement returns null");
    assert_eq!(kmo.borrow().childCount(), 0);
}

// ─────────────────────────────────────────────────────────────────────────────
// ★W6c-5：`p:spPr` 在**四类子形状**上的落点（Java 覆盖 sp / pic / cxnSp / graphicFrame）
// ─────────────────────────────────────────────────────────────────────────────
//
// 背景：门禁 ⑰.3 原先报「spPr 写入调用点 1（目标 4）」——产品的 `feed_open_sp_pr`
//   只给 `sp` 喂了 `p:spPr`，`pic` / `cxnSp` / `graphicFrame` 三类的 `spPr` 子树
//   在派发层就断了（`SwGraphicFrameHandler::enterSubElement` 缺 `3211587` 分支）。
//   真身 handler 侧 `SwPictureHandler` / `SwConnectorShapeHandler` **本来就有**该分支
//   （`3211600` / `3211599`），缺的是产品喂入端与 `graphicFrame` 的派发端。
//   本组断言直接驱动**真 handler 派发**，证明四类都返回真身 `ShapePropertiesHandler`。
//
// owner: tests.pptxr.handler.shape    wave: M-PPT-KMOSLIDE-W6c-5

/// 从 `SwGroupShapeHandler` 派发 `kind` 对应的子形状，返回其 `spPr` 分支的 handler。
fn dispatch_sp_pr(h: &mut SwGroupShapeHandler<DummyCb>, child_id: i32, sp_pr_id: i32) -> String {
    let mut child = h.enterSubElement(child_id).expect("child handler");
    let mut sp_pr = child
        .enterSubElement(sp_pr_id)
        .expect("child must expose p:spPr as ShapePropertiesHandler");
    let kind = sp_pr.kind().to_string();
    sp_pr.endElement(sp_pr_id);
    kind
}

#[test]
fn sg_sp_pr_dispatches_on_all_four_child_kinds() {
    use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler as DmlrElementHandler;

    // Java `SwShapeHandler` / `SwPictureHandler` / `SwConnectorShapeHandler` /
    // `SwGraphicFrameHandler` 各自都有 `case <p:spPr>` 分支。
    // rust: `p:spPr` 在 `sp`/`pic`/`cxnSp` 三类 handler 中的 element id **一致 = 3211578**
    //   （`SwShapeHandler.rs:298` / `SwPictureHandler.rs:241` / `SwConnectorShapeHandler.rs:178`）；
    //   `SwGraphicFrameHandler` 原先**整条分支缺失**（⑰.3 报「1/4」的根因），本波补上。
    //
    //   ★W6c-9 —— **编号纠正 + 断言纠正**：本测试原先给 graphicFrame 传 `3211587`，
    //   依据是"取其实际分支 id"（当时该 handler 确写作 3211587）。但对照 Java 原文，
    //   `3211587` 是 `p:cNvGraphicFramePr`（`SwGraphicFrameHandler.java:83`，**内部类**
    //   `NonVisualGraphicFrameProperties` 的 case），**不是** `p:spPr`；
    //   `p:spPr` 在 Java 三处均为 `3211578`
    //   （`SwShapeHandler.java:66` / `SwConnectorShapeHandler.java:42` / `SwPictureHandler.java:47`）。
    //   ⇒ 旧的 `3211587 => p:spPr` 分支是**永不可达的转写错误**（外层永远收不到该号）。
    //   已把 `SwGraphicFrameHandler` 外层分支纠为 `3211578`，故本测试四类**统一用 3211578**。
    //   ★教训：测试若按"实现当前怎么写"取值，会把**缺陷**固化成**契约**
    //   （本例中它一度让错误编号看起来是"已登记的合理偏差"）。
    //   取值权威应是 **Java 原文**，不是当前 Rust 实现。
    //   owner: show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler
    //   wave: M-PPT-KMOSLIDE-W6c-9
    let cases: [(i32, i32, &str); 4] = [
        (3211593, 3211578, "sp"),
        (3211469, 3211578, "pic"),
        (3211596, 3211578, "cxnSp"),
        (3211595, 3211578, "graphicFrame"),
    ];
    for (child_id, sp_pr_id, label) in cases {
        let mut store = DgStore::DgStore_new();
        let dummy = DummySlide::with_store(&mut store);
        let (_kmo, mut h) = group_handler(&dummy, &mut store);
        let kind = dispatch_sp_pr(&mut h, child_id, sp_pr_id);
        assert_eq!(
            kind, "ShapePropertiesHandler",
            "{label}: p:spPr({sp_pr_id}) 必须派发真身 ShapePropertiesHandler"
        );
        // 具名类型绑定，避免 `unused` 告警噪音（同时证明该 trait 确在作用域内可用）。
        let _ = std::any::type_name::<dyn DmlrElementHandler>();
    }
}
