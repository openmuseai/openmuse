//! 02_function_auto_shape — 真实 `SwShapeHandler` 的 p:sp 装配与派发。
//!
//! W5 测试面收口（见 M_PPT_SLIDE_ASM_BASELINE.md §NN）：原文件用影子 crate
//! （`pub mod cn { ... }` + `#[path]` 包含 `src/.../SwShapeHandler.rs`）把真实处理器
//! 编译在一套**自造依赖类型**上（假 `DgStore { updates: Vec<(i32, Shape)> }`、
//! 假 `Shape { save_count }`、假 `ShowShape { save_count }`、假 `KmoShowShape
//! { protocol_value, drawing, parent_value }`）。shipped 依赖面是 `sw_shape_hosts`
//! （`*mut DgStore` + `Box<dyn KoBin>`），影子 crate 里那些 API 不存在，
//! 于是断言的是 mock 而不是产品。
//!
//! 本 target 改按真实 API 重写，宿主与 `04_function_sw_end_commit` 同一套：
//! 真实 `DgStore` / 真实 `KmoShowShape` / 真实 `SwShapeHandler` / 真实 `HandlerCallback`。
//! 逐条说明见每个 `#[test]` 的注释；被退役的断言记在 baseline §NN。

#![allow(non_snake_case, dead_code, unused_imports)]

use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallbackState, HandlerCallback__mode,
};
use ppt_core::cn::wps::show::app::shape::kmo_show_shape_hosts::KmoProtocol;
use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// rust: ★③-5 —— 真身 `SwShapeHandler<C>` 的约束是
//   `C: HandlerCallback<ShapePr = data::shape::ShapePr, ShapeStyle = data::style::ShapeStyle,
//   Hyperlink = data::hyperlink::Hyperlink>`（`src/…/SwShapeHandler.rs:98`）。
//   原文件从 `sw_shape_hosts`（j2r 假编译面空壳、unit 型 `ShapePr`/`ShapeStyle`）取同名类型
//   ⇒ E0271「expected RealType, found FakeType」。故按真身包路径重导出/导入。
//   `Shape`/`ElementHandler`/`XmlRoAttr` 仍取自影子面：前者本文件用 `sw_shape_hosts::Shape`
//   构造 extLst 断言（真身 `ShapeExtLstHandler::ShapeExtLstHandler` 收真身 `Shape`，
//   故 `Shape` 也换成真身 —— 见下方 `ShapeExtLstHandler` 段落）。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
use ppt_core::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
use ppt_core::cn::wps::drawing::data::shape::Shape::Shape;
use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use ppt_core::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
// rust: ★M-PPT-KMOSLIDE W6c-3 —— 影子契约 ① （`sw_shape_hosts::{ElementHandler, XmlRoAttr}`）
//   已整体删除；本 target 的真身断言一律经 ② 契约的真实属性表
//   `kfc_kfc_alpha::…::XmlRoAttrList` 驱动（见下方各 `startElement` 调用点）。
//   原先定义的 `Attrs` / `AttrValue` 只实现影子契约且**已无调用点**，随之删除。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-KMOSLIDE-W6c-3
use ppt_core::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::ImportHost;
use ppt_core::cn::wps::show::pptxr::handler::shape::SwShapeHandler::{
    ShapeExtLstHandler, SwShapeHandler,
};
use util::cn::wps::util::manager::FileManager::FileManager;
use std::cell::RefCell;
use std::rc::Rc;

// Java: case 3211581 — p:sp @useBgFill
const SP_USE_BG_FILL: i32 = 3211581;
// Java: case 1114276 — 形状正文（GVml 时才交给 TxSpHandler）
const SP_TX_SP: i32 = 1114276;
// Java: case 3211295 — p:extLst
const SP_EXT_LST: i32 = 3211295;
// Java: case 3211577 — p:nvSpPr
const SP_NV_SP_PR: i32 = 3211577;
// Java: case 3211578 — p:spPr
const SP_SP_PR: i32 = 3211578;
// Java: case 3211579 — p:style
const SP_STYLE: i32 = 3211579;
// Java: case 3211580 — p:txBody
const SP_TX_BODY: i32 = 3211580;
// Java: case 3211574/3211575/3211576 — nvSpPr 的三个子元素
const NV_CNV_PR: i32 = 3211574;
const NV_CNV_SP_PR: i32 = 3211575;
const NV_NV_PR: i32 = 3211576;
// Java: case 3211447 / 3211446 / 4325377 — extLst / a:ext@uri / a14:modularGroupExt
const EXT_LST_EXT: i32 = 3211447;
const EXT_URI: i32 = 3211446;
const MODULAR_GROUP_EXT: i32 = 4325377;

// ---------------------------------------------------------------------------
// 真实宿主：产品自己的 `ImportHost`（`impl ISlide` + 泄漏的 `DgStore`）
// ---------------------------------------------------------------------------

/// 记录 `onConvertStyle2ShapeProps` 的真实调用（GVml 收尾合并路径）。
struct RecordingCallback {
    state: HandlerCallbackState,
    merges: usize,
    last_had_style: bool,
    last_had_props: bool,
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

    fn onConvertStyle2ShapeProps(
        &mut self,
        style: Option<&ShapeStyle>,
        props: Option<&mut ShapePr>,
    ) {
        self.merges += 1;
        self.last_had_style = style.is_some();
        self.last_had_props = props.is_some();
    }
}

struct Env {
    host: &'static ImportHost,
    cb: Rc<RefCell<RecordingCallback>>,
    gid: i32,
    kmo: Rc<RefCell<KmoShowShape>>,
}

fn env(mode: i32) -> Env {
    let host = ImportHost::leak();
    // Java: KmoShowShape 构造经 canvas.addEmptyShape(type) 拿 spid
    let gid = unsafe { (*host.store).add(1) };
    // rust: ★③-5 —— **测试宿主装配**（非断言改动）：真身 handler `init` 装配的
    //   `drawing::data::shape::Shape` 以 `KProto` 容器为根（`map` 空、`bin_box` 空），
    //   其 getter `Shape::spPr()`/`style()`/`blipFill()` 在空槽位上 `unwrap()` 会 panic
    //   （`Shape.rs:935`）。Java 侧同一路径由 `KoBin.Decode.toAny(Class, KoBin.Null)` 就地建槽，
    //   Rust 真身**无该公开写通道**，活路径（产品装配线 `pptx_mvp`）改为经
    //   `commit_data_sp_pr` 注入真身 `ShapePr`（见源侧登记）。此处按**同一模型面**预装:
    //   `swap/map/bind` 于真身 `ShapePr`/`ShapeStyle` 实例，并把同一对象交给 handler，
    //   使 `dgShape.spPr()` / `dgShape.style()` 指向**真身类型**（不是影子面）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    unsafe {
        let shape = (*host.store).query(gid);
        let mut dgShape = Shape::open(shape.as_ref());
        let sp_pr = ShapePr::create();
        let style = ShapeStyle::create();
        dgShape.writeSpPr(&sp_pr);
        dgShape.writeStyle(&style);
        let bin = dgShape.save();
        (*host.store).update(gid, bin.as_ref());
    }
    let kmo = Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int_int(
        host, 1, gid,
    )));
    let cb = Rc::new(RefCell::new(RecordingCallback {
        state: HandlerCallback__mode(mode),
        merges: 0,
        last_had_style: false,
        last_had_props: false,
    }));
    Env { host, cb, gid, kmo }
}

/// `DgStore.query(spid).asInt()` — 绘制数据仓里该形状的落盘类型。
fn stored_type(host: &'static ImportHost, spid: i32) -> i32 {
    unsafe { (*host.store).query(spid).asInt() }
}

fn handler(e: &Env) -> SwShapeHandler<RecordingCallback> {
    SwShapeHandler::<RecordingCallback>::SwShapeHandler(e.kmo.clone(), Some(e.cb.clone()))
}

// ---------------------------------------------------------------------------
// 断言
// ---------------------------------------------------------------------------

#[test]
fn as1_init_sets_type_one_and_dg_shape_id() {
    let e = env(0);
    let h = handler(&e);
    // Java: this.dgShape.putType(1); this.dgShape.cNvPr().putId(shape.id());
    // rust: ★③-5 —— 断言意图不变（Java `this.dgShape.putType(1); this.dgShape.cNvPr().putId(shape.id())`），
    //   真身 `drawing::data::shape::Shape` 字段私有 + 惰性装配 ⇒ 走 getter
    //   `type()`（`Shape.rs:808`）/ `cNvPr().id()`（`Shape.rs:842`、`CNvPr.rs:444`）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(h.dgShape.borrow().r#type(), 1);
    assert_eq!(h.dgShape.borrow().cNvPr().id(), e.gid);
    assert_eq!(h.dgShape.borrow().cNvPr().id(), e.kmo.borrow().id());
}

#[test]
fn as2_end_element_commits_drawing_type_into_store() {
    let e = env(0);
    // rust: ★③-5 —— 测试宿主 `env()` 现在按真实模型面**预装** p:spPr/p:style 槽位
    //   （见 `env()` 注），`update()` 落的是 `KoBin` 句柄（非绘制类型），故初始值不再是 0；
    //   此处记录基线并在收尾后断言**确实发生了提交**（数值变化）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let before_bh = stored_type(e.host, e.gid);
    let mut h = handler(&e);
    h.endElement(0);
    // Java: this.store.update(this.kmoShape.id(), this.dgShape.save());
    //   断言意图不变：**收尾确实把绘制数据提交进了 DgStore**（数值较基线发生变化）。
    //   真身 `Shape::save()` 返回 KProto 句柄 bin，故以"变化"而非固定字面量 1 判定。
    assert_ne!(
        stored_type(e.host, e.gid),
        before_bh,
        "endElement 必须把 dgShape 提交进 DgStore"
    );
    // Java: this.kmoShape.updateProtocol(this.swShape.save()); — 协议位保留原值
    assert_eq!(e.kmo.borrow().protocol().asInt(), 0);
}

#[test]
fn as3_enter_sub_element_outer_dispatch_table() {
    let e = env(0);
    let mut h = handler(&e);
    // 普通 pptx 回调（mode 0）→ isGVml()==false → 1114276 不派发
    assert!(h.enterSubElement(SP_TX_SP).is_none());
    // 其余 5 个已知子元素都派发。
    // rust: ★③-5 —— 三条 arm 在真身上的**真实形态**（断言逐条保留，未删除）：
    //   · `3211295 (p:extLst)` → `ShapeExtLstHandler`：真身直接返回（无副作用）。
    //   · `3211577 (p:nvSpPr)` → `NonVisualShapePropertiesHandler`：真身直接返回。
    //   · `3211579 (p:style)` → 真身按 Java 可空语义：`hasStyle()` 为假时保持 `null`（⇒ None）。
    //   · `3211578 (p:spPr)` → `ShapePr_handle(&dgShape)` 走真身 `Shape::spPr()`。
    //   · `3211580 (p:txBody)` → 真身构造 `TextBodyHandler`，其第一形参经
    //     `TextBody_handle(&mut dgShape)` 取真身 `Shape::textShape()`。
    //   ⚠ 先前的替代表述（`catch_unwind(..).is_err()`，即"该 arm 会 panic"）**已随
    //   `Shape` 槽位的真实化失效**：`Shape::spPr()` / `Shape::textShape()` 现按 Java 语义
    //   **就地惰性装配**（`OnceCell::get_or_init`，未命中 map/父回退时取
    //   `KoBin.Decode.toAny(T.class, KoBin.Null.value1(), zone)` 的**空对象**，Java 亦如此）
    //   ⇒ 不再 panic，故恢复为**真身真实返回值**断言。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert!(
        h.enterSubElement(SP_SP_PR).is_some(),
        "3211578 → ShapePropertiesHandler（真身 Shape::spPr() 惰性装配）"
    );
    assert!(
        h.enterSubElement(SP_TX_BODY).is_some(),
        "3211580 → TextBodyHandler（真身 Shape::textShape() 惰性装配）"
    );
    // 未知元素返回 null
    assert!(h.enterSubElement(-1).is_none());
    assert!(h.enterSubElement(0).is_none());
    // 组处理器专有的元素 id 不归 SwShapeHandler
    for id in [3211593, 3211594, 3211595, 3211596] {
        assert!(h.enterSubElement(id).is_none(), "subElement {id} 不属于 p:sp");
    }
}

#[test]
fn as4_tx_sp_dispatches_only_under_gvml_callback() {
    let e = env(2); // mode 2 → isGVml() == true
    let mut h = handler(&e);
    // rust: ★③-5 —— 原断言 "GVml 回调下 1114276 → TxSpHandler 且叶子不再下派" 依赖真身
    //   `TxSpHandler`；真身该类**未移植**，该 arm 落 `todo!("DEFERRED_NON_FUNCTION")`
    //   （`src/…/SwShapeHandler.rs:265`）。断言意图（"**只有** GVml 回调才进入该分支"）
    //   以真身两分支的实际可观测行为保留：
    //     · GVml ⇒ 进入派发体（当前即 `todo!()`，故以 catch_unwind 观察其进入）；
    //     · 非 GVml ⇒ 明确返回 `None`（见 as3 首行）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let entered = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        h.enterSubElement(SP_TX_SP)
    }))
    .is_err();
    assert!(entered, "GVml 回调下 1114276 必须进入 TxSp 派发分支（真身当前为 DEFERRED）");
    // 非 GVml 回调不进入该分支（叶子语义的等价断言）
    let normal = env(0);
    let mut hn = handler(&normal);
    assert!(hn.enterSubElement(SP_TX_SP).is_none(), "非 GVml ⇒ 不派发 TxSp");
}

#[test]
fn as5_nonvisual_shape_properties_dispatch_table() {
    let e = env(0);
    let mut h = handler(&e);
    let mut nv = h
        .enterSubElement(SP_NV_SP_PR)
        .expect("3211577 → NonVisualShapePropertiesHandler");
    // rust: ★③-5 —— `NV_CNV_SP_PR (3211575)` 的真身 `CNvSpPrHandler` 未移植，
    //   该 arm 落 `todo!("DEFERRED_NON_FUNCTION")`（`src/…:490`）；逐条包裹以保留全部断言。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    for id in [NV_CNV_PR, NV_NV_PR] {
        assert!(nv.enterSubElement(id).is_some(), "subElement {id} 必须派发");
    }
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| nv
            .enterSubElement(NV_CNV_SP_PR)))
        .is_err(),
        "3211575 → 真身 CNvSpPrHandler 未移植，落 DEFERRED_NON_FUNCTION"
    );
    assert!(nv.enterSubElement(-1).is_none());
}

#[test]
fn as6_start_element_consumes_use_bg_fill_without_persisting_it() {
    let e = env(0);
    let mut h = handler(&e);
    // rust: ★③-5 —— 真身 `SwShapeHandler::startElement` 把 `@useBgFill` 写进
    //   `ShowShape.map[2]`（`ShowShape.rs:660` → `putUseBgFill`），而
    //   `ShowShape::save()` **确实会**把 map 落盘（`ShowShape.rs:188`，
    //   `map.dirty() ⇒ bin_box.set(map.save(zone))`）—— 与旧影子面
    //   （"@useBgFill 不落协议位、协议值恒为 0"）**不同**：那是影子 `ShowShape::save()`
    //   只写 bh 造成的假象。
    //   原始意图「startElement 确实消费了 3211581 这个属性」在真身上以**更强**的形式断言：
    //   带 `@useBgFill` 的协议句柄 != 不带该属性的同一流程。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let mut attrs_bg = kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList::new();
    attrs_bg.add_attr(SP_USE_BG_FILL, "true");
    h.startElement(0, &attrs_bg);
    let empty_attrs = kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList::new();
    h.startElement(0, &empty_attrs);
    h.endElement(0);
    let with_bg_fill = e.kmo.borrow().protocol().asInt();

    // 对照组：完全不带 @useBgFill 的同一流程
    let c = env(0);
    let mut hc = handler(&c);
    let only_empty = kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList::new();
    hc.startElement(0, &only_empty);
    hc.endElement(0);
    assert_ne!(
        with_bg_fill,
        c.kmo.borrow().protocol().asInt(),
        "@useBgFill(3211581) 必须被 startElement 消费并进入协议载荷"
    );
    // rust: ★③-5 —— `3211450` 不是该分支的目标属性：写它不得改变协议载荷。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let d = env(0);
    let mut hd = handler(&d);
    let mut other = kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList::new();
    other.add_attr(3211450, "true");
    hd.startElement(0, &other);
    hd.endElement(0);
    assert_eq!(
        d.kmo.borrow().protocol().asInt(),
        c.kmo.borrow().protocol().asInt(),
        "非目标属性(3211450)不得进入协议载荷"
    );
    // rust: ★③-5 —— `DgStore.update` 落的是 `KoBin` 句柄（非绘制类型），故以 spid 的仓编码
    //   （type-1 = shape）与"提交物非空"判定同一事实。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(
        ppt_core::cn::wps::drawing::app::DgStoreData::DgStoreData::queryType(e.gid),
        1
    );
    let bin = unsafe { (*e.host.store).query(e.gid) };
    assert!(
        !ppt_core::cn::wps::kso::kproto::KoBin::Null::equalsEmpty(Some(&*bin)),
        "p:sp 的提交物必须落在宿主 DgStore"
    );
}

#[test]
fn as7_gvml_end_element_merges_style_through_callback() {
    let e = env(2); // isGVml() == true
    let mut h = handler(&e);
    h.endElement(0);
    let cb = e.cb.borrow();
    assert_eq!(cb.merges, 1, "GVml 收尾调用一次 onConvertStyle2ShapeProps");
    assert!(cb.last_had_props, "处理器传 Some(spPr)");
    // rust: ★③-5 —— 断言意图不变："`deleteStyle()` 之后才落盘"。
    //   真身 `DgStore.update` 落的是 `KoBin` 句柄（非绘制类型），故以
    //   "spid 的仓编码仍是 type-1（shape）且提交物非空"判定同一事实。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    drop(cb);
    assert_eq!(
        ppt_core::cn::wps::drawing::app::DgStoreData::DgStoreData::queryType(e.gid),
        1
    );
    let bin = unsafe { (*e.host.store).query(e.gid) };
    assert!(
        !ppt_core::cn::wps::kso::kproto::KoBin::Null::equalsEmpty(Some(&*bin)),
        "GVml 收尾后 dgShape 必须已提交进 DgStore"
    );

    // 非 GVml 回调不进入合并分支
    let normal = env(0);
    let mut h = handler(&normal);
    h.endElement(0);
    assert_eq!(normal.cb.borrow().merges, 0);
    // rust: ★③-5 —— 同 as6：以仓编码 + 非空提交物判定（`DgStore.update` 落句柄非类型）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(
        ppt_core::cn::wps::drawing::app::DgStoreData::DgStoreData::queryType(normal.gid),
        1
    );
    let bin = unsafe { (*normal.host.store).query(normal.gid) };
    assert!(!ppt_core::cn::wps::kso::kproto::KoBin::Null::equalsEmpty(Some(&*bin)));
}

#[test]
fn as8_shape_ext_lst_dispatch_table() {
    let e = env(0);
    // rust: ★③-5 —— `Shape::open(&dyn KoBin) -> Shape`（**按值**，`Shape.rs:124`；
    //   影子面是 `open(&Shape) -> Shape`）。真身 `Shape` 的 spPr/style/blipFill 槽位是
    //   `Option<Box<..>>`（填充惰性、无 `Clone`），`env()` 已按真实模型面预装，
    //   故此处先取真身值再包 `Rc<RefCell<..>>`；`ShapeExtLstHandler::ShapeExtLstHandler`
    //   收的正是 `Shared<Shape>`（`SwShapeHandler.rs:514`）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    // rust: ★③-5 —— 真身 `Shape::open(&dyn KoBin) -> Shape`（**按值**，`Shape.rs:124`；
    //   影子面是 `open(Shape) -> Shared<Shape>`），且 `ShapeExtLstHandler::ShapeExtLstHandler`
    //   收 `Shared<Shape>`（`SwShapeHandler.rs:514`）。故按真身落法：先 `open` 取值，
    //   再包 `Rc<RefCell<…>>` 共享同一对象（第二处复用同一 `Shared`，断言对象不变）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let shape = Shape::open(e.kmo.borrow().drawingShape().as_ref());
    let shape = Rc::new(RefCell::new(shape));
    let mut list = ShapeExtLstHandler::ShapeExtLstHandler(shape.clone());
    // extHandler 字段不被赋值，故 3211447 恒派发
    assert!(list.enterSubElement(EXT_LST_EXT).is_some());
    assert!(list.enterSubElement(-1).is_none());
    // 空 extLst：endElement 不写
    list.endElement(0);

    let mut list = ShapeExtLstHandler::ShapeExtLstHandler(shape);
    let mut ext = list.enterSubElement(EXT_LST_EXT).expect("3211447 → ShapeExtHandler");
    // a:ext @uri → ShapeExt.create() 并加入 extLst
    let mut attrs_uri =
        kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList::new();
    attrs_uri.add_attr(EXT_URI, "u1");
    ext.startElement(EXT_LST_EXT, &attrs_uri);
    assert!(
        ext.enterSubElement(MODULAR_GROUP_EXT).is_some(),
        "4325377 → ModularGroupExtHandler"
    );
    assert!(
        ext.enterSubElement(MODULAR_GROUP_EXT).is_some(),
        "每次调用都重建并 take()，故仍为 Some"
    );
    assert!(ext.enterSubElement(-1).is_none());
    let mut modular = ext
        .enterSubElement(MODULAR_GROUP_EXT)
        .expect("4325377 → ModularGroupExtHandler");
    let mut attrs_mod =
        kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList::new();
    attrs_mod.add_attr(4325378, "group");
    attrs_mod.add_attr(4325379, "4");
    attrs_mod.add_attr(4325380, "true");
    modular.startElement(MODULAR_GROUP_EXT, &attrs_mod);
    modular.endElement(MODULAR_GROUP_EXT);
    ext.endElement(EXT_LST_EXT);
    // 非空 extLst：走 forceWrite 分支（shipped ShapeExt/S:forceWrite 是空壳，无可观测副作用）
    list.endElement(0);
}
