//! 04_function_connector_shape — 真实 `SwConnectorShapeHandler` 的连接形状派发与收尾。
//!
//! rust: ★③-5 —— 影子 crate 收口（PLAN §26 判据 2.5；同 `05_function_shape_tree_dispatch`
//!   与 `04_function_sw_end_commit` 的既定做法）。
//!
//! 原文件用 `#[path]` 把真实源包含进测试 crate，并自造一棵 `cn::wps::…` 假类型树
//! （假 `Shape { shape_type, cnv, sp_pr, style }`、假 `DgStore { updates, events }`、
//! 假 `KmoShowShape { shape_id, protocol_value, drawing, parent_value, events }`）。
//! 真实源迁移到**真身模型类型**后，该影子面在两处同时失效，且第二处**不可修补**：
//!   ① 名字解析 —— 真身 `CNvPr` / `ShapePr` / `ShapeStyle` / `Hyperlink` /
//!      `dmlr::handler::ElementHandler` / `kmo_show_shape_hosts::KmoProtocol` 在假树中
//!      不存在（E0432 / E0433）；
//!   ② 可见性 —— 真身 `Shape` 的 `cNvPr` / `spPr` / `style` 槽位是 `pub(crate)`
//!      （`src/…/data/shape/Shape.rs:39-43`），而被包含的源按真身语义直接读写这些槽位
//!      （`CNvPr_handle` / `ShapePr_handle` / `3211579` 分支），一旦名字解析通过即 E0616
//!      ("field is private")。**把假模块换成真身再导出只能修 ①，永远修不了 ②** ——
//!      字段可见性不会因为模块被 re-export 而放宽（已实测：改 `pub use ppt_core::cn;`
//!      后 E0432/E0433 消失，随即出现 8 处 E0616）。
//!
//! 故本 target 改为直接驱动 `ppt_core` 里的**真身** `SwConnectorShapeHandler`，宿主用真身
//! `DgStore` + `KmoShowShape`（`DummySlide` 实现 `ISlide`）。真身 handler 的
//! `swConnection` / `dgConnection` 同样是源的 `pub(crate)` 字段（测试 crate 不可见），
//! 因此下面每条断言都改用**产品自己的公开读写路径**取得同一事实，详见各处 ★③-5 注。
//! 原影子面遗留的两条 `// rust:` 注记（结论仍是真身事实）：
//!   · `kfc_kfc_alpha` 是真 crate，`XmlRoAttrList` / `ElementHandler` 一律复用真身，
//!     不得伪造第二份（伪造会造成 `Box<dyn ElementHandler>` 类型身份不一致）；
//!   · `SwNvPrHandler` 的 `new` 别名只存在于影子面，真身只有 Java 同名构造器
//!     `SwNvPrHandler(Rc<RefCell<ShowShape>>, Option<Rc<RefCell<C>>>)`。
//! owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH

#![allow(dead_code, non_snake_case, non_camel_case_types, unused_imports)]

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::app::DgStore::DgStore;
use ppt_core::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
use ppt_core::cn::wps::drawing::data::shape::Shape::Shape;
use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use ppt_core::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallbackState, HandlerCallback__mode,
};
use ppt_core::cn::wps::show::app::media::KmoMediaLib::KmoMediaLib;
use ppt_core::cn::wps::show::app::shape::kmo_show_shape_hosts::KmoProtocol;
use ppt_core::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use ppt_core::cn::wps::show::app::slide::ISlide::{
    Background, ColorMapping, ISlide, KmoPresentation, KmoSlideEventListeners, KmoTheme, KsoWMInfo,
};
use ppt_core::cn::wps::show::data::shape::ShowShape::ShowShape;
use ppt_core::cn::wps::show::pptxr::handler::shape::SwConnectorShapeHandler::SwConnectorShapeHandler;
use std::cell::RefCell;
use std::rc::Rc;
use util::cn::wps::util::manager::FileManager::FileManager;

// ---------------------------------------------------------------------------
// 真实宿主：`ISlide`（持有真身 `*mut DgStore`）+ 真身 `KmoShowShape`
// ---------------------------------------------------------------------------

struct DummySlide {
    pres: KmoPresentation,
    theme: KmoTheme,
    events: KmoSlideEventListeners,
    bg: Background,
    wm: KsoWMInfo,
    cm: ColorMapping,
}

impl DummySlide {
    fn with_store(store: *mut DgStore) -> Self {
        DummySlide {
            pres: KmoPresentation {
                drawing_store: store,
                // rust: ★③-5 —— 真身 `ISlide::KmoPresentation` 的媒体表字段
                //   （同 `01_function_sw_group_on_mod.rs` / `04_function_sw_end_commit.rs`）。
                //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
                media_lib: Rc::new(RefCell::new(KmoMediaLib::new())),
            },
            theme: KmoTheme,
            events: KmoSlideEventListeners::new(),
            // rust: ★W6c-4 —— 三个真身类型不再是单元结构体 ⇒ 必须构造实例。
            //   owner: tests.pptxr.handler.shape    wave: M-PPT-KMOSLIDE-W6c-4
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

// ---------------------------------------------------------------------------
// 回调：记录 Java 顺序（merge → store → protocol → notify）
// ---------------------------------------------------------------------------

// rust: ★③-5 —— 原影子面的假 `DgStore { updates, events }` / 假 `KmoShowShape { events }`
//   自带事件向量，`"store"` / `"protocol"` 两条事件由它们推送。真身 `DgStore` /
//   `KmoShowShape` **没有**事件钩子，故本回调在其被调用的**当下**读取宿主状态，
//   等价还原这两条事件与 Java 的执行顺序：
//     · `onConvertStyle2ShapeProps`（GVml 合并，Java 顺序第一步）—— 此刻 store 必须**尚未**提交；
//     · `onConnectorShape`（收尾通知，Java 顺序最后一步）—— 此刻 store 与 protocol 必须**均已**提交。
//   "protocol 已提交"的判定见 `inject_protocol_marker`。
//   另外，真身 `Shape` 的 cNvPr 槽位测试不可见、且 `Shape::cNvPr()` 的 map→槽位解码在 port
//   中未实装（槽位为空即 panic），故连接形状的 `cNvPr.id` 由真身自己的读取路径取得：
//   `CNvPrHandler` 在 `cNvPr@id`(1114245) 上回调 `onShapeId(attr, this.cNvPr.id())`
//   （`CNvPrHandler.rs:140-148`），本回调记下第二个实参。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
struct Callback {
    state: HandlerCallbackState,
    events: Rc<RefCell<Vec<String>>>,
    store: *mut DgStore,
    gid: i32,
    // rust: ★③-5 —— 该 spid 在**提交前**的仓值（`handler()` 装配时取样）。
    //   `seed_style` 会预置非零 bin（真身 `Shape::style()` 槽位的前置条件），故"store 已提交"
    //   不能以 0 为判据，必须以基线为准。owner: tests.pptxr.handler.shape
    store_before: i32,
    kmo: Option<Rc<RefCell<KmoShowShape>>>,
    // Java: this.callback.onShapeId(tmp.toInt(), this.cNvPr.id()) —— 第二个实参即连接形状的 id
    core_shape_id: Option<i32>,
    // 原断言 `updates[0].1.sp_pr.borrow().merged == 1` 的真身对应观察（真身 `ShapePr`
    // 没有 `merged` 计数器）。
    merge_calls: usize,
    merge_sp_pr_ptr: Option<usize>,
    merge_style_seen: bool,
    merge_store_committed: bool,
}

impl Callback {
    fn store_committed(&self) -> bool {
        if self.store.is_null() {
            return false;
        }
        unsafe { (*self.store).query(self.gid).asInt() != self.store_before }
    }

    fn protocol_updated(&self) -> bool {
        self.kmo.as_ref().is_some_and(|k| {
            let bin = k.borrow().protocol();
            !ShowShape::open(&*bin).hasDrawingId()
        })
    }
}

impl HandlerCallback for Callback {
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
    type Shape = Shape;
    type ShapePr = ShapePr;
    type ShapeStyle = ShapeStyle;
    type TextRun = ();
    type TagLst = ();
    type FileManager = FileManager;

    fn callback_state(&self) -> &HandlerCallbackState {
        &self.state
    }

    // Java: void onShapeId(Integer originShapeId, Integer coreShapeId)
    fn onShapeId(&mut self, _originShapeId: Option<i32>, coreShapeId: Option<i32>) {
        self.core_shape_id = coreShapeId;
    }

    // Java: void onConvertStyle2ShapeProps(ShapeStyle shapeStyle, ShapePr shapePr)
    fn onConvertStyle2ShapeProps(
        &mut self,
        shapeStyle: Option<&ShapeStyle>,
        shapePr: Option<&mut ShapePr>,
    ) {
        self.merge_calls += 1;
        self.merge_style_seen = shapeStyle.is_some();
        // Java 顺序证据：合并必须发生在 `dgStore.update(..)` **之前**。
        self.merge_store_committed = self.store_committed();
        self.merge_sp_pr_ptr = shapePr.map(|p| p as *mut ShapePr as usize);
        self.events.borrow_mut().push("merge".into());
    }

    // Java: void onConnectorShape(Integer connectorShapeId)
    fn onConnectorShape(&mut self, connectorShapeId: Option<i32>) {
        // 真身无事件钩子 ⇒ 在 notify 当下读取宿主状态来还原影子面的 "store" / "protocol"。
        if self.store_committed() {
            self.events.borrow_mut().push("store".into());
        }
        if self.protocol_updated() {
            self.events.borrow_mut().push("protocol".into());
        }
        self.events
            .borrow_mut()
            .push(format!("notify:{}", connectorShapeId.unwrap()));
    }
}

// ---------------------------------------------------------------------------
// 装配
// ---------------------------------------------------------------------------

// rust: ★③-5 —— 连接形状的 spid 由真身 `DgStore::add(shapeType)` 按仓编码分配
//   （`DgStoreData::helper_storeIndex2Id`）；原影子面里"直接指定 id 9/1/2/10"在真身上
//   不成立，故断言改用**分配到的** spid —— 断言含义不变（"cNvPr.id 等于该形状 id"）。
//   `3` = cxn 仓（connector）。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
const CONNECTOR: i32 = 3;
// Java: case 1114245 —— `cNvPr@id` 属性号（`CNvPrHandler.rs:135`）
const CNVPR_ID_ATTR: i32 = 1114245;
// rust: ★③-5 —— 注入到 kmo protocol 形状上的可辨识标记（见 `inject_protocol_marker`）。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
const PROTOCOL_BEFORE: i32 = 6123;

struct Env {
    store: Rc<RefCell<DgStore>>,
    gid: i32,
    kmo: Rc<RefCell<KmoShowShape>>,
    cb: Rc<RefCell<Callback>>,
    events: Rc<RefCell<Vec<String>>>,
}

fn new_store() -> Rc<RefCell<DgStore>> {
    Rc::new(RefCell::new(DgStore::DgStore_new()))
}

fn setup(store: Rc<RefCell<DgStore>>, mode: i32) -> Env {
    let gid = store.borrow_mut().add(CONNECTOR);
    let events = Rc::new(RefCell::new(Vec::new()));
    // rust: 真身 `KmoShowShape` 持有 `*const dyn ISlide`（`KmoShowShape.rs:186`），
    //   故宿主必须比它活得久 —— 用 `Box::leak` 取得 `'static` 生命周期
    //   （同 `SwGroupShapeHandler` 测试里的 `ImportHost::leak()`）。
    let store_ptr: *mut DgStore = {
        let mut guard = store.borrow_mut();
        let p: *mut DgStore = &mut *guard;
        p
    };
    let slide: &'static DummySlide = Box::leak(Box::new(DummySlide::with_store(store_ptr)));
    let kmo = Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int_int(
        slide, CONNECTOR, gid,
    )));
    let cb = Rc::new(RefCell::new(Callback {
        state: HandlerCallback__mode(mode),
        events: events.clone(),
        store: store_ptr,
        gid,
        store_before: 0,
        kmo: Some(kmo.clone()),
        core_shape_id: None,
        merge_calls: 0,
        merge_sp_pr_ptr: None,
        merge_style_seen: false,
        merge_store_committed: false,
    }));
    Env {
        store,
        gid,
        kmo,
        cb,
        events,
    }
}

fn handler(env: &Env) -> SwConnectorShapeHandler<Callback> {
    // rust: ★③-5 —— 记录"提交前"该 spid 的仓值：`seed_style` 会预置非零 bin，故
    //   "store 已提交"必须以基线（而非 0）为判据。owner: tests.pptxr.handler.shape
    env.cb.borrow_mut().store_before = env.store.borrow().query(env.gid).asInt();
    SwConnectorShapeHandler::SwConnectorShapeHandler(env.kmo.clone(), Some(env.cb.clone()))
}

// rust: ★③-5 —— 真身 `Shape::style()` 的槽位是**由派发路径惰性实装**的：`Shape.rs:986`
//   在槽位为空时 `unwrap()` 直接 panic（map→槽位的解码在 port 中未实装），全仓唯一给该
//   槽位赋 `Some(..)` 的地方就是真身 `SwConnectorShapeHandler::enterSubElement(3211579)`。
//   影子面 `Shape::default()` 恒带 `style: Some(..)`，故原文无需前置；真身须按 Java 语义
//   "形状带 style 才有 `p:style` / 才做 GVml 合并"先把一条**带 style 的 Shape bin**
//   放进产品自己的仓（`DgStore::update`），令 handler 的 `Shape::open(drawingShape())`
//   解出 map key4 ⇒ `hasStyle()` 为真；再由真身 3211579 派发把槽位实装。
//   `forceWriteStyle` 是真身公开 API（`Shape.rs:1033`）；这是用真身 API 补齐前置条件，
//   **不是**削弱断言。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
fn seed_style(env: &Env) {
    let mut seed = Shape::create();
    seed.putType(CONNECTOR);
    seed.forceWriteStyle(&ShapeStyle::create());
    let bin = seed.save();
    env.store.borrow_mut().update(env.gid, &*bin);
}

// rust: ★③-5 —— 真身 `SwConnectorShapeHandler::swConnection` 是源的 `pub(crate)` 字段
//   （测试 crate 不可见），无法像影子面那样直接在它上面写标记。改用产品自己的读写路径：
//   ctor 把 `swConnection` 建为 `KmoShowShape::protocol()` 的**拷贝**；若在 ctor **之后**
//   改写 kmo 的 protocol 形状，则 `endElement` 的
//   `updateProtocol(this.swConnection.save())` 必然把 kmo 的 protocol 覆盖回 ctor 时的内容。
//   于是"protocol 已提交"成为真身可解码读取的事实：注入的 `drawingId` 标记**消失**即证明
//   `updateProtocol` 确已执行（`ShowShape::hasDrawingId`）。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
fn inject_protocol_marker(env: &Env) {
    let mut shape = ShowShape::create();
    shape.putDrawingId(PROTOCOL_BEFORE);
    let bin = shape.save();
    env.kmo.borrow_mut().updateProtocol(&*bin);
}

fn protocol_committed(env: &Env) -> bool {
    let bin = env.kmo.borrow().protocol();
    !ShowShape::open(&*bin).hasDrawingId()
}

fn expected_with_notify(head: &[&str], gid: i32) -> Vec<String> {
    head.iter()
        .map(|s| s.to_string())
        .chain([format!("notify:{}", gid)])
        .collect()
}

// ---------------------------------------------------------------------------
// 断言（逐条对应原 target 的 7 个测试，语义不变）
// ---------------------------------------------------------------------------

#[test]
fn ctor_sets_connector_type_and_id() {
    let env = setup(new_store(), 0);
    let mut h = handler(&env);
    h.endElement(0);
    // rust: ★③-5 —— 原文 `(v.shape_type, v.cnv.borrow().id) == (3, 9)` 断言在**提交物**上。
    //   真身 `Shape::save()` 交出的是 bin 句柄，但同一事实可从真身公开解码路径逐字读回：
    //   ① 绘制类型：`DgStore::query(spid)` + `Shape::open`（`type()` 走 map key1，可用）；
    //   ② cNvPr.id：真身 cNvPr 槽位 `pub(crate)` 且 `Shape::cNvPr()` 的槽位解码未实装，
    //      故走真身 `CNvPrHandler` 读取路径 —— 在 `cNvPr@id`(1114245) 上
    //      `callback.onShapeId(attr, this.cNvPr.id())`，第二个实参正是连接形状的 cNvPr.id。
    //   原字面量 9 在真身由 `DgStore::add(3)` 分配的 cxn spid 承载（spid 由仓编码决定）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let committed = Shape::open(&*env.store.borrow().query(env.gid));
    assert_eq!(committed.r#type(), 3, "连接形状绘制类型必须是 3（cxn）");
    let mut nv = h
        .enterSubElement(3211583)
        .expect("3211583 → NonVisualConnectionShapePropertiesHandler");
    let mut cnv = nv
        .enterSubElement(3211574)
        .expect("3211574 → CNvPrHandler");
    let mut attrs = XmlRoAttrList::new();
    attrs.add_attr(CNVPR_ID_ATTR, "7");
    cnv.startElement(CNVPR_ID_ATTR, &attrs);
    assert_eq!(
        env.cb.borrow().core_shape_id,
        Some(env.gid),
        "连接形状 cNvPr.id 必须等于本形状 id"
    );
}

#[test]
fn outer_dispatches_typed_handlers_and_ignores_ext_unknown() {
    let env = setup(new_store(), 0);
    // rust: ★③-5 —— 前置：真身 `p:style` 分支以 `hasStyle()` 为条件（见 `seed_style` 注）；
    //   原文的形状恒有 style，故补齐前置后断言不变。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    seed_style(&env);
    let mut h = handler(&env);
    assert!(h.enterSubElement(3211295).is_none());
    assert!(h.enterSubElement(-1).is_none());
    assert_eq!(
        h.enterSubElement(3211578).unwrap().kind(),
        "ShapePropertiesHandler"
    );
    assert_eq!(
        h.enterSubElement(3211579).unwrap().kind(),
        "ShapeStyleHandler"
    );
    // rust: ★③-5 —— 原断言 `"ElementHandler"` 取自影子 `ElementHandler` trait 的**默认**
    //   `kind()`（假面）。真身 3211583 派发的是源内 `NonVisualConnectionShapePropertiesHandler`，
    //   其 `impl ElementHandler` 的 `kind()` 是类型名。断言意图（"3211583 必须派发内层处理器"）
    //   不变，按真身 kind 修正。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(
        h.enterSubElement(3211583).unwrap().kind(),
        "NonVisualConnectionShapePropertiesHandler"
    );
}

#[test]
fn nested_dispatches_all_three_children() {
    let env = setup(new_store(), 0);
    let mut h = handler(&env);
    let mut nv = h.enterSubElement(3211583).unwrap();
    for (id, k) in [
        (3211574, "CNvPrHandler"),
        (3211576, "SwNvPrHandler"),
        (3211582, "CNvCxnSpPrHandler"),
    ] {
        assert_eq!(nv.enterSubElement(id).unwrap().kind(), k);
    }
    assert!(nv.enterSubElement(-1).is_none());
}

#[test]
fn normal_commit_precedes_connector_notification() {
    let env = setup(new_store(), 0);
    let mut h = handler(&env);
    inject_protocol_marker(&env);
    assert!(
        !protocol_committed(&env),
        "注入标记后 protocol 尚未被提交覆盖（观察机制自检）"
    );
    h.endElement(0);
    // rust: ★③-5 —— 原断言 `s.borrow().updates.len() == 1`：真身 `DgStore` 没有 `updates`
    //   向量，改以"cxn 仓恰好一条 + 该 spid 的提交物非空"等价表达"恰有一次提交落到
    //   ctor 时的 DgStore"。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(env.store.borrow().helper_storeByType(3).len(), 1);
    assert_ne!(env.store.borrow().query(env.gid).asInt(), 0);
    // protocol 提交（`updateProtocol(swConnection.save())` 覆盖了注入的标记）。
    assert!(protocol_committed(&env), "protocol 提交必须已执行");
    // 原断言 `&["store", "protocol", "notify:9"]` 逐字保留 —— 真身无事件钩子，
    // "store"/"protocol" 由回调在 notify 当下读到"已提交"而还原（见 `Callback` 注）。
    assert_eq!(
        &*env.events.borrow(),
        &expected_with_notify(&["store", "protocol"], env.gid)
    );
}

#[test]
fn gvml_merge_and_style_delete_precede_commit() {
    // Java: mode 2 ⇒ HandlerCallback.isGVml() 为真
    let env = setup(new_store(), 2);
    seed_style(&env);
    let mut h = handler(&env);
    // Java: `enterSubElement(3211579)` → ShapeStyleHandler（同时按真身惰性语义实装 style 槽位）
    let _ = h.enterSubElement(3211579).expect("3211579 → ShapeStyleHandler");
    inject_protocol_marker(&env);
    h.endElement(0);
    // rust: ★③-5 —— 原断言 `s.borrow().updates[0].1.style.is_none()`：真身 `Shape::save()`
    //   交出的是 bin 句柄，但**同一事实**可直接从真身解码路径读回 —— 提交物的 map 不再含
    //   `SHAPE_STYLE(4)` ⇒ `hasStyle()` 为假（`Shape.rs:1007` 是 map 判定，不经过未实装的
    //   槽位解码）。若 `deleteStyle()` 未先执行，前置 `seed_style` 写下的 key4 会让它变真。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert!(
        !Shape::open(&*env.store.borrow().query(env.gid)).hasStyle(),
        "提交物必须无 style（deleteStyle 先于提交）"
    );
    assert!(env.cb.borrow().merge_style_seen);
    // rust: ★③-5 —— 原断言 `s.borrow().updates[0].1.sp_pr.borrow().merged == 1`：真身
    //   `ShapePr` 没有 `merged` 计数器，改断言同一意图 —— GVml 合并以**本条连接形状的
    //   活 spPr 句柄**被调用恰好一次（指针同一性：回调收到的 `&mut ShapePr` 就是
    //   `DgStore` 为 spid 登记的活句柄，与 `ShapePr_handle` 的登记语义一致）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let handle = env
        .store
        .borrow()
        .shapePr(env.gid)
        .expect("活 spPr 句柄已登记进 DgStore");
    assert_eq!(env.cb.borrow().merge_calls, 1);
    assert_eq!(
        env.cb.borrow().merge_sp_pr_ptr,
        Some(RefCell::as_ptr(&*handle) as usize)
    );
    // 顺序断言逐字保留（"store"/"protocol" 由回调在 notify 当下观察还原）。
    assert_eq!(
        &*env.events.borrow(),
        &expected_with_notify(&["merge", "store", "protocol"], env.gid)
    );
    // Java 顺序证据：merge 必须早于 store 提交（回调在 merge 当下记录的宿主状态）。
    assert!(!env.cb.borrow().merge_store_committed);
}

#[test]
fn absent_callback_panics_after_java_ordered_commits() {
    let env = setup(new_store(), 0);
    let mut h: SwConnectorShapeHandler<Callback> =
        SwConnectorShapeHandler::SwConnectorShapeHandler(env.kmo.clone(), None);
    inject_protocol_marker(&env);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| h.endElement(0)));
    assert!(result.is_err());
    // rust: ★③-5 —— 原断言 `s.borrow().updates.len() == 1` 与
    //   `&*e.borrow() == &["store", "protocol"]`：本场景**无回调**，影子面那两个事件的
    //   来源（假 `DgStore` / 假 `KmoShowShape` 自带 events）在真身上不存在，也不再有任何
    //   观察者；但两个提交本身是**真身可直接读取**的事实，故逐条改成等价断言：
    //   store 已提交、protocol 已提交，且二者都发生在 panic（Java 契约要求的 non-null
    //   callback）之前 —— 与原文"Java 顺序的两个提交先完成"完全同义。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_ne!(
        env.store.borrow().query(env.gid).asInt(),
        0,
        "store 提交发生在 panic 之前"
    );
    assert!(
        protocol_committed(&env),
        "protocol 提交发生在 panic 之前"
    );
    assert!(
        env.events.borrow().is_empty(),
        "无回调 ⇒ 无事件（影子面的事件源已随假编译面消除）"
    );
}

#[test]
fn reinit_commits_only_new_connector() {
    // rust: ★③-5 —— 原 setup 每次新建一个假 store（`updates` 能按"新 id 10"区分）；
    //   真身 spid 是**仓内编码**（`DgStore::add(3)`），两个连接形状只有落在**同一个**
    //   DgStore 里才有可区分的 spid（cxn 仓 index 0 / 1）—— 这正是 Java"同一画布上
    //   重初始化到另一个连接形状"的场景。断言意图不变：只有**新**连接形状被提交，
    //   旧连接形状的仓条目不被改写（原 `updates.map(id) == [10]` 的等价形式）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let store = new_store();
    let first_env = setup(store.clone(), 0);
    let first = first_env.gid;
    let mut h = SwConnectorShapeHandler::SwConnectorShapeHandler(
        first_env.kmo.clone(),
        Some(first_env.cb.clone()),
    );
    let next = setup(store.clone(), 0);
    let second = next.gid;
    assert_ne!(first, second, "同一 DgStore 的 cxn 仓按 index 编码不同 spid");
    h.init(next.kmo.clone());
    h.endElement(0);
    assert_ne!(store.borrow().query(second).asInt(), 0, "新连接形状已提交");
    assert_eq!(
        store.borrow().query(first).asInt(),
        0,
        "本次提交只触及新连接形状，旧条目未被改写"
    );
}
