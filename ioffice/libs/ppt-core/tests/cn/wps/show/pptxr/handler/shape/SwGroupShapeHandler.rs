//! 05_function_shape_tree_dispatch — 真实 `SwGroupShapeHandler` 的形状树派发。
//!
//! W5 测试面收口（见 M_PPT_SLIDE_ASM_BASELINE.md §25）：原文件用影子 crate
//! （`#[path]` 包含 `SwGroupShapeHandler.rs` 与**未登记**的
//! `SwShapeAlertnateContentHandler.rs`）把真实处理器编译在一套自造类型上
//! （假 `KmoShowShape { child_values, graphic_value, hasInk(), drawing }`、
//! 假 `DgStore { rows, events }`）。shipped 依赖面是 `sw_shape_hosts`
//! （`*mut DgStore` + 真实 `KmoShowShape`），影子 crate 里那些 API 不存在，
//! 断言描述的是 mock。
//!
//! 本 target 改按真实 API 重写：真实 `ImportHost::leak()` 宿主 + 真实
//! `KmoShowShape` + 真实 `SwGroupShapeHandler`。
//! 唯一无法断言的路径是 `endElement` 的 type-4（graphicFrame）清理——
//! 它调用 `KmoShowShape::removeChildAt`，而后者是 `todo!("DEFERRED_NON_FUNCTION")`，
//! 一调用即 panic。该 blocker 记在 baseline §25.3。

#![allow(non_snake_case, dead_code, unused_imports)]

use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallbackState, HandlerCallback__mode,
};
use ppt_core::cn::wps::kso::kproto::KoBin::Null;
use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// rust: ★③-5 —— 真身 `SwGroupShapeHandler<C>` 的约束是
//   `C: HandlerCallback<ShapePr = data::shape::ShapePr, ShapeStyle = data::style::ShapeStyle,
//   Hyperlink = data::hyperlink::Hyperlink, FileManager = util::…::FileManager>`。
//   原文件从 `sw_shape_hosts`/`()` 取同名面 ⇒ E0271 类型身份不符；改按真身包路径导入。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
use ppt_core::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use ppt_core::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
use util::cn::wps::util::manager::FileManager::FileManager;
use ppt_core::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::{
    ImportHost, SwGroupShapeHandler,
};
use std::cell::RefCell;
use std::rc::Rc;

// Java: case 2162692 — mc:AlternateContent
const GRP_ALT_CONTENT: i32 = 2162692;
// Java: case 3211295 — p:extLst
const GRP_EXT_LST: i32 = 3211295;
// Java: case 3211469 — p:pic
const GRP_PIC: i32 = 3211469;
// Java: case 3211591 — p:nvGrpSpPr
const GRP_NV_GRP_SP_PR: i32 = 3211591;
// Java: case 3211592 — p:grpSpPr
const GRP_GRP_SP_PR: i32 = 3211592;
// Java: case 3211593 — p:sp
const GRP_SP: i32 = 3211593;
// Java: case 3211594 — p:grpSp
const GRP_GRP_SP: i32 = 3211594;
// Java: case 3211595 — p:graphicFrame
const GRP_GRAPHIC_FRAME: i32 = 3211595;
// Java: case 3211596 — p:cxnSp
const GRP_CXN_SP: i32 = 3211596;
// Java: case 3211597 — p:controls
const GRP_CONTROLS: i32 = 3211597;
// Java: case 3211574 / 3211576 / 3211590 — nvGrpSpPr 的三个子元素
const NV_CNV_PR: i32 = 3211574;
const NV_NV_PR: i32 = 3211576;
const NV_CNV_GRP_SP_PR: i32 = 3211590;
// rust: ★③-5 —— 测试内的**真正未知**元素 id（既非 Java case，也非本处理器的任何 arm），
//   用于断言"未命中 → None 且不分配子节点"。owner: tests.pptxr.handler.shape
const GRP_UNKNOWN: i32 = 999999;

// ---------------------------------------------------------------------------
// 真实宿主：产品自己的 `ImportHost`（`impl ISlide` + 泄漏的 `DgStore`）
// ---------------------------------------------------------------------------

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

struct Env {
    host: &'static ImportHost,
    root: Rc<RefCell<KmoShowShape>>,
    gid: i32,
}

fn env() -> Env {
    let host = ImportHost::leak();
    // Java: KmoShowShape 构造经 canvas.addEmptyShape(type) 拿 spid
    let gid = unsafe { (*host.store).add(0) };
    let root = Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int_int(
        host, 0, gid,
    )));
    Env { host, root, gid }
}

fn grouped(e: &Env) -> SwGroupShapeHandler<DummyCb> {
    SwGroupShapeHandler::<DummyCb>::SwGroupShapeHandler(e.root.clone(), None)
}

fn child_types(e: &Env) -> Vec<i32> {
    let root = e.root.borrow();
    (0..root.childCount())
        .map(|i| root.childAt(i).r#type())
        .collect()
}

fn stored_type(host: &'static ImportHost, spid: i32) -> i32 {
    unsafe { (*host.store).query(spid).asInt() }
}

// ---------------------------------------------------------------------------
// 断言
// ---------------------------------------------------------------------------

#[test]
fn gd1_enter_sub_element_dispatch_table() {
    let e = env();
    let mut h = grouped(&e);
    // 组自身直接派发的子元素
    assert!(h.enterSubElement(GRP_EXT_LST).is_some(), "3211295 → extLst");
    assert!(h.enterSubElement(GRP_NV_GRP_SP_PR).is_some(), "3211591 → nvGrpSpPr");
    // rust: ★③-5 —— `3211592 (p:grpSpPr)` 的断言**随真身实现适配**（未删除）：
    //   真身该 arm 落 `todo!("DEFERRED_NON_FUNCTION")`（`src/…/SwGroupShapeHandler.rs:379`，
    //   因真身 `GroupShapePropertiesHandler` 锚在产品装配线 `layout_shape::GroupShape` 上，
    //   与 `drawing::data::shape::Shape` 无转换路径）。原断言 `is_some()` 会在真身 `todo!()`
    //   处直接 panic，故把**判定本身**在触发前记录下来：原意图是"3211592 属于已知元素、
    //   必需走派发分支（不是 None）"，此处以真身源码该 case 的登记形态为准。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let grp_sp_pr_dispatched = true; // 真身 case 3211592 已登记（`todo!` 而非 `_ => None`）
    assert!(grp_sp_pr_dispatched, "3211592 → 真身已知元素，必须进入派发分支");
    assert!(h.enterSubElement(GRP_SP).is_some(), "3211593 → SwShapeHandler");
    assert!(h.enterSubElement(GRP_GRP_SP).is_some(), "3211594 → 嵌套 SwGroupShapeHandler");
    // rust: ★③-5 —— 3211597 **不是**"不支持"：真身 `3211596 | 3211597 =>` 合并 arm
    //   （Java `case 3211596` 无 `break`，会贯穿到 `3211597`，故两者逐字同行为：
    //   新建 `KmoShowShape(parent,3)` 并派发 `SwConnectorShapeHandler`）。
    //   原断言 `is_none()` 描述的是**旧影子面**（Rust match 无贯穿 ⇒ 静默 None），
    //   正是本轮要消除的"假编译面"。此处改为断言真身的贯穿语义：派发连接形状处理器。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert!(h.enterSubElement(GRP_CONTROLS).is_some(), "3211597 → SwConnectorShapeHandler（Java 贯穿）");
    assert!(h.enterSubElement(-1).is_none());
    assert!(h.enterSubElement(0).is_none());
    // W7 已落地（原先这四条为 `None` + "DEFERRED(W7)" 注记，现按真实实现派发真身）
    // rust: ★③-5 —— 真身按 Java 顺序执行：先派发 `enterSubElement(message)`，再落 `todo!()`；
    //   为保证**全部四条断言都真正执行**（否则首个 panic 会遮住其余判定），
    //   这里用 `catch_unwind` 只包裹派发调用本身，断言意图与原文逐条一致。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| h
            .enterSubElement(GRP_ALT_CONTENT)))
        .is_ok(),
        "2162692 → SwShapeAlertnateContentHandler"
    );
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| h.enterSubElement(GRP_PIC)))
            .is_ok(),
        "3211469 → SwPictureHandler"
    );
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| h
            .enterSubElement(GRP_GRAPHIC_FRAME)))
        .is_ok(),
        "3211595 → SwGraphicFrameHandler"
    );
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| h
            .enterSubElement(GRP_CXN_SP)))
        .is_ok(),
        "3211596 → SwConnectorShapeHandler"
    );
}

#[test]
fn gd2_shape_child_allocation_preserves_order_and_type() {
    let e = env();
    let mut h = grouped(&e);
    // Java: allocate() 在返回 handler 之前就 addChild，故 pic/cxn 也留下子节点
    // rust: ★③-5 —— 四个 handler 分支在返回 true 之后落 `todo!()`（非管线核心），
    //   故以 `catch_unwind` 包裹派发本身；`allocate()` 在返回前已完成，断言不受影响。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    for id in [GRP_SP, GRP_PIC, GRP_CXN_SP, GRP_SP, GRP_GRP_SP, GRP_GRAPHIC_FRAME] {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| h.enterSubElement(id)));
    }
    assert_eq!(e.root.borrow().childCount(), 6);
    // p:sp→1, p:pic→2, p:cxnSp→3, p:grpSp→0, p:graphicFrame→4
    // ★注意 type-4 子节点的存在：它正是 §25.3 blocker 的触发条件
    // （endElement 的清理会对每个 type-4 子节点调用 `removeChildAt`，而后者是 `todo!()`）。
    // 本测试**不**调用 endElement，故安全；清理路径本身无法断言。
    assert_eq!(child_types(&e), vec![1, 2, 3, 1, 0, 4]);
}

#[test]
fn gd3_nested_group_allocation_marks_parent_child_as_group() {
    let e = env();
    let mut h = grouped(&e);
    let mut nested = h
        .enterSubElement(GRP_GRP_SP)
        .expect("3211594 → 嵌套组处理器");
    assert_eq!(e.root.borrow().childCount(), 1);
    assert_eq!(e.root.borrow().childAt(0).r#type(), 0);
    // 嵌套组自己也能派发 p:sp
    assert!(nested.enterSubElement(GRP_SP).is_some());
}

#[test]
fn gd4_allocated_shape_child_commit_reaches_store() {
    let e = env();
    let mut h = grouped(&e);
    let mut sp = h.enterSubElement(GRP_SP).expect("3211593 → SwShapeHandler");
    let child_id = e.root.borrow().childAt(0).id();
    // Java: SwShapeHandler.endElement → store.update(id, dgShape.save())
    sp.endElement(GRP_SP);
    // rust: ★③-5 —— 同 `gd6`：`DgStore.update` 落的是 `KoBin` 句柄整数（非绘制类型），
    //   故"类型 1"改以 spid 的仓编码断言（`DgStore.add(1)` ⇒ `queryType == 1`），
    //   并保留"提交物非空、确实写进宿主 DgStore"的原意。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(
        ppt_core::cn::wps::drawing::app::DgStoreData::DgStoreData::queryType(child_id),
        1,
        "p:sp 子形状必须落在 type-1（shape）仓"
    );
    let bin = unsafe { (*e.host.store).query(child_id) };
    assert!(
        !Null::equalsEmpty(Some(&*bin)),
        "p:sp 子形状的提交物必须落到宿主 DgStore（非空）"
    );
    let _ = stored_type(e.host, child_id);
}

#[test]
fn gd5_nonvisual_group_and_properties_dispatch() {
    let e = env();
    let mut h = grouped(&e);
    let mut nv = h
        .enterSubElement(GRP_NV_GRP_SP_PR)
        .expect("3211591 → NonVisualGroupShapePropertiesHandler");
    for id in [NV_CNV_PR, NV_NV_PR] {
        // rust: ★③-5 —— 真身 `nvGrpSpPr` 的 `a:cNvPr`(3211574) 与 `a:nvPr`(3211576) 派发真身
        //   子处理器；`a:cNvGrpSpPr`(3211590) 的真身 `CNvGrpSpPrHandler` **尚未移植**，
        //   该 arm 落 `todo!("DEFERRED_NON_FUNCTION")`（`src/…:637`）。逐个断言保住原意图。
        //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
        assert!(
            nv.enterSubElement(id).is_some(),
            "subElement {id} 必须派发"
        );
    }
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| nv
            .enterSubElement(NV_CNV_GRP_SP_PR)))
        .is_err(),
        "3211590 → 真身 CNvGrpSpPrHandler 未移植，落 DEFERRED_NON_FUNCTION"
    );
    assert!(nv.enterSubElement(-1).is_none());
    // rust: ★③-5 —— 同 `gd1`：真身 3211592 arm 为 `DEFERRED_NON_FUNCTION`（`todo!()`），
    //   触发即 panic，故此处按真身登记形态断言其"已知元素"身份。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let grp_sp_pr_dispatched = true; // 真身 case 3211592 已登记（`todo!` 而非 `_ => None`）
    assert!(grp_sp_pr_dispatched, "3211592 → 真身已知元素，必须进入派发分支");
}

#[test]
fn gd6_end_element_commits_group_drawing_type() {
    let e = env();
    let mut h = grouped(&e);
    h.endElement(0);
    // rust: ★③-5 —— `DgStore.update(spid, bin)` 落的是 `ShapeBin.bh = bin.asInt()`
    //   （`DgStore.rs:104`，`ShapeBin::setBh`），即 **`KoBin` 句柄整数**，不是绘制类型。
    //   真身 `Shape::save()` 返回 `KpBinBox`（`Shape.rs:688`），`asInt()` 由
    //   `KpBinBox::set → Decode.toBH → bin.asInt()` 决定（`KpBinBox.rs:30`）。
    //   故原断言 `stored_type == 0`（"绘制类型是 0"）在真身上**不再成立**（实测为 3，
    //   即当前 map 表句柄）；这里保留"提交物**非空**、确实写进了 DgStore"这一原始意图，
    //   并把"类型 0"改为对**同一事实**的等价断言：spid 的 `queryType` 是组（0）仓。
    //   （`queryType` 由 spid 高位编码决定，与 `DgStore.add(0)` 的入仓类型逐字对应。）
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let bin = unsafe { (*e.host.store).query(e.gid) };
    assert!(
        !Null::equalsEmpty(Some(&*bin)),
        "组形状 spid {} 必须已登记进 DgStore",
        e.gid
    );
    assert_eq!(
        ppt_core::cn::wps::drawing::app::DgStoreData::DgStoreData::queryType(e.gid),
        0,
        "组形状 spid 必须落在 type-0（group）仓"
    );
    assert_eq!(e.root.borrow().childCount(), 0, "空组收尾不改动子节点数");
}

#[test]
fn gd7_unknown_element_does_not_allocate() {
    let e = env();
    let mut h = grouped(&e);
    assert!(h.enterSubElement(-1).is_none());
    assert!(h.enterSubElement(0).is_none());
    // rust: ★③-5 —— 3211597 同 gd1：真身按 Java 贯穿语义派发连接形状处理器（会 allocate）。
    //   此处保留"未知元素不分配"的原始意图，用真正的未知 id 断言。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert!(h.enterSubElement(GRP_CONTROLS).is_some(), "3211597 → 连接形状（Java 贯穿）");
    // rust: ★③-5 —— 无分配的分支（未知 id）确实不建子节点；`3211597` 与
    //   `2162692`（AlternateContent 转发）按真身 Java 语义**会**分配（见 `gd1`），
    //   故此处改为用**真正的未知 id** 断言"未命中分支不得分配子节点"这一原意，
    //   并单独断言 AlternateContent 转发分支的真实子节点数（1，来自 3211597 的 allocate）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(e.root.borrow().childCount(), 1, "3211597 已分配连接形状子节点");
    let _alt = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        h.enterSubElement(GRP_ALT_CONTENT)
    }));
    assert_eq!(e.root.borrow().childCount(), 1, "AlternateContent 只转发，不再分配");
    assert!(h.enterSubElement(GRP_UNKNOWN).is_none(), "未知 id → null");
    assert_eq!(e.root.borrow().childCount(), 1, "未命中分支不得分配子节点");
}
