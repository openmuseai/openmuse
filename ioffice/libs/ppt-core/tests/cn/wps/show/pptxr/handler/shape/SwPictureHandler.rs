//! 03_function_picture_shape — 真实 `SwPictureHandler` 的 p:pic 装配 / 子元素派发 / 收尾提交。
//!
//! ## 测试面收口（本 target 的第二次改造，W5 → M-PPT-SHAPE-DISPATCH）
//!
//! 原文件用**影子 crate**手法：文件内自建 `pub mod cn { pub mod wps { … } }` 假依赖树，
//! 再 `#[path = "…/src/…/SwPictureHandler.rs"] mod implementation;` 把**真实源码**编到假类型上。
//! 该手法只在「真实源码未登记、只能经 `#[path]` 到达」时成立（基线 §25.2 / §28.2 的判据）。
//! 本波 `src/cn/wps/show/pptxr/handler/shape/mod.rs` 已登记 `pub mod SwPictureHandler;`
//! ⇒ 该 handler 是**产品可达**的，影子手法必然失败，故本 target 按**真实 API** 重写
//! （与 `02_function_auto_shape` / `05_function_shape_tree_dispatch` 同一处置 (a)）。
//!
//! 「就地把假类型体改成真身再导出」在本 target **不可行**，三条实测硬事实：
//! 1. 真身 `drawing::data::shape::Shape` 的模型槽位是 `pub(crate)`
//!    （`cNvPr`/`spPr`/`style`/`blipFill` …，`Shape.rs:35-68`）⇒ 跨 crate 不可见；
//!    而被测源码 `SwPictureHandler.rs:200-208` 直接读写 `dg.cNvPr` **字段**，
//!    故 `Shape` 一旦换成真身即 E0616「field is private」。
//! 2. 宿主契约已固定为 `ImportHost { pub store: *mut DgStore }` + `ImportHost::leak()`
//!    （`SwGroupShapeHandler.rs:698`）；假 `KmoShowShape`/假 `DgStore` 与之不同构。
//! 3. 真身 `ShowShape::save()` / `Shape::save()` 返回 `Box<dyn KoBin>`（`ShowShape.rs:188`、
//!    `Shape.rs:688`），源码 `updateProtocol(protocol.as_ref())` 依赖该形状；
//!    假 `ShowShape::save() -> Box<ShowShape>` 无法承载真实协议位。
//! 结论：`Shape`/`ShapePr`/`ShapeStyle`/`Fill`/`CNvPr`/`ShowShape`/`KmoShowShape`/`DgStore`
//! 与全部子 handler 一律改用**真身**（`ppt_core::cn::wps::…`），不再伪造第二份。
//!   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
//!
//! ## ★③-5 未落地边界（真身 `todo!` / 编译壳 ⇒ 只能断言到边界，逐条登记）
//!
//! (1) `KmoShowShape::hasAudio()/hasVideo()/audioMediaId()/videoMediaId()`
//!     （`KmoShowShape.rs:4521/4530/4539/4624`）均为 `todo!("DEFERRED_NON_FUNCTION")`
//!     ⇒ `endElement` **无条件**在收尾末尾的媒体探针处中断（`SwPictureHandler.rs:379-383`），
//!     share-id 写入策略不可达。该中断发生在 `updateProtocol` + `dgStore.update`
//!     **之后**，故不掩盖本 target 要断言的装配/派发/提交事实。
//! (2) 媒体夹具也不可构造：`KmoMediaLib::addMediaData(String,int)` /
//!     `addMediaData_from_data(..)`（`KmoMediaLib.rs:211/319`）都经
//!     `ISlimToolManager::instance()` 落 `todo!()`，且无「KmoMediaData 直接入表」的公开通道。
//! (3) `KoAr::saveBin(&self)` 编译壳遮蔽真身 `saveBin(&mut self, ..)`：真身数据层为
//!     j2r 便利把 `KoAr` 的 `saveBin` 声明成 `&self`（`KoAr.rs:11`），实现体或是
//!     `self.bin_box.get()`（返回**旧缓存**，`CNvPr.rs:773`/`Fill.rs:832`/`ShapePr.rs:1522` …）
//!     或是 `todo!("saveBin requires &mut self")`（`Hyperlink.rs:1010`/`FillMode.rs:1600` …）。
//!     Rust 方法解析在接收者 `&mut Box<T>` 上**先命中 trait 方法**（`rustc` 最小复现已验），
//!     而 `Shape::saveBin`（`Shape.rs:197-200`）正是 `let b = c.saveBin(zone);`（`c: &mut Box<CNvPr>`）
//!     ⇒ cNvPr 槽位被写成 0、子对象的 KpMap 从不落表（本文件 `ctor_…` 用例有实测登记）。
//!     该缺陷属 `src/`，按 §25.3「只报告不修」处理。
//!
//! 以上都是 `src/` 侧缺陷；本文件按硬规则**只改测试**，把每条断言改成
//! 「真身可观测事实」并在用例内逐条标注退役/替代关系（口径见基线 §25.4 / §28.3：
//! 退役 = 断言没有可被产品触达的真实客体，须给出替代断言）。
//!   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables)]

use std::cell::RefCell;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use ppt_core::cn::wps::drawing::app::DgStore::DgStore;
use ppt_core::cn::wps::drawing::app::DgStoreData::DgStoreData;
// rust: ★③-5 —— 真身 `SwPictureHandler<C>` 的约束是
//   `C: HandlerCallback<ShapePr = data::shape::ShapePr, ShapeStyle = data::style::ShapeStyle,
//   Hyperlink = data::hyperlink::Hyperlink>`（`src/…/SwPictureHandler.rs:74`）。
//   影子面那三个同名类型已被删除 ⇒ 一律按真身包路径导入，类型身份与源码一致。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
use ppt_core::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
use ppt_core::cn::wps::drawing::data::shape::CNvPr::CNvPr;
use ppt_core::cn::wps::drawing::data::shape::Shape::Shape;
use ppt_core::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use ppt_core::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallbackState, HandlerCallback__mode,
};
// rust: ★③-5 —— 读取真身 `Shape::save()` 落进 `DgStore` 的 KProto 映射表
//   （`DgStore.update` 只保留 `bin.asInt()` = `KpMap` 表句柄；见 `Shape.png`…
//   `Shape.rs:189-260 saveBin` + `internal/KpMap.rs:56 save`）。这是本 target 里
//   唯一能读到「提交物内容」的公开通道（`SwPictureHandler.dgPic` 是 `pub(crate)`）。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
use ppt_core::cn::wps::kso::kproto::internal::KpTablePool::KpTablePool;
use ppt_core::cn::wps::show::app::shape::kmo_show_shape_hosts::KmoProtocol;
use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use ppt_core::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
use ppt_core::cn::wps::show::app::slide::ISlide::{
    Background, ColorMapping, ISlide, KmoPresentation, KmoSlideEventListeners, KmoTheme, KsoWMInfo,
};
use ppt_core::cn::wps::show::pptxr::handler::shape::SwPictureHandler::SwPictureHandler;
// rust: ★③-5 —— 真身子 handler 一律按真身包路径导入（旧的影子替身已删除）：
//   `…::nvsppr::cnv::CNvPrHandler` / `CNvPicPrHandler`、`…::sppr::ShapePropertiesHandler`、
//   `…::sppr::fill::BlipFillHandler`、`…::style::ShapeStyleHandler`、
//   `…::SwNvPrHandler`、`…::SwShapeHandler::ShapeExtLstHandler` 现在**全部登记在库内**，
//   由被测源码自己 `use`（本文件不再需要为它们提供名字）。
//   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
use util::cn::wps::util::manager::FileManager::FileManager;

// Java: case 2162692 — mc:AlternateContent
const ALT_CONTENT: i32 = 2162692;
// Java: case 2162693 — mc:Choice
const ALT_CHOICE: i32 = 2162693;
// Java: case 2162695 — mc:Fallback
const ALT_FALLBACK: i32 = 2162695;
// Java: case 3211295 — p:extLst
const PIC_EXT_LST: i32 = 3211295;
// Java: case 3211574 — p:nvPicPr 的子元素 p:cNvPr
const NV_CNV_PR: i32 = 3211574;
// Java: case 3211576 — p:nvPicPr 的子元素 p:nvPr
const NV_NV_PR: i32 = 3211576;
// Java: case 3211578 — p:spPr
const PIC_SP_PR: i32 = 3211578;
// Java: case 3211579 — p:style
const PIC_STYLE: i32 = 3211579;
// Java: case 3211584 — p:nvPicPr 的子元素 a:picLocks（p:pic 的 CNvPicPrHandler）
const NV_PIC_LOCKS: i32 = 3211584;
// Java: case 3211585 — p:nvPicPr
const PIC_NV_PIC_PR: i32 = 3211585;
// Java: case 3211586 — a:blipFill
const PIC_BLIP_FILL: i32 = 3211586;
// Java: case 3211469 — p:pic（本 target 覆盖的子形状类型在 DgStore 中的编码）
const SHAPE_TYPE_PIC: i32 = 2;

// ---------------------------------------------------------------------------
// 真实宿主：产品自己的 `ImportHost` 语义（`impl ISlide` + 真实 `KmoPresentation`）
// ---------------------------------------------------------------------------

/// 记录 `onConvertStyle2ShapeProps` 的真实调用（GVml 收尾合并路径）。
struct RecordingCallback {
    state: HandlerCallbackState,
    conversions: usize,
    last_had_style: bool,
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

    // Java: void onConvertStyle2ShapeProps(ShapeStyle style, ShapePr props)
    fn onConvertStyle2ShapeProps(
        &mut self,
        style: Option<&ShapeStyle>,
        props: Option<&mut ShapePr>,
    ) {
        self.conversions += 1;
        self.last_had_style = style.is_some();
    }
}

/// 测试宿主：真实 `KmoPresentation`（含**真实**媒体表）+ 泄漏的 `DgStore`。
/// rust: ★③-5 —— 与 `04_function_sw_end_commit.rs` 同一套（该 target 亦含本地 `ISlide`）。
///   4 个「按真实 API 重写」的 target 刻意改用产品宿主 `ImportHost::leak()`；
///   本处保留本地宿主是因为 `ImportHost.pres` 是**私有**字段，媒体表不可注入
///   （媒体组用例需要真身 `KmoMediaLib` 的观测面，见文件头 (3)）。
///   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
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
                media_lib: Rc::new(RefCell::new(
                    ppt_core::cn::wps::show::app::media::KmoMediaLib::KmoMediaLib::new(),
                )),
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
        panic!("DummySlide.spTree: 形状树根在 KmoShowShape 上")
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
        true
    }
}

struct Env {
    /// Java: `this.dgStore` —— 产品侧由 `KmoPresentation` 持有的**同一份**仓。
    store: *mut DgStore,
    slide: &'static DummySlide,
    /// Java: `parent.getPresentation().canvas().addEmptyShape(type)` 分配的 spid
    gid: i32,
    cb: Rc<RefCell<RecordingCallback>>,
    kmo: Rc<RefCell<KmoShowShape>>,
}

/// 泄漏一份宿主 + 仓（每个用例一份，避免用例间串仓）。
fn env(mode: i32, with_style: bool) -> Env {
    let store: &'static mut DgStore = Box::leak(Box::new(DgStore::DgStore_new()));
    // Java: KmoShowShape 构造经 canvas.addEmptyShape(type) 拿 spid
    let gid = store.add(SHAPE_TYPE_PIC);
    let slide = Box::leak(Box::new(DummySlide::with_store(&mut *store)));
    // Java: 活路径上 p:style 由 XML 解析写入 `Shape.map[4]`；真身模型无该导入通道，
    //   故按**同一模型面**预装（真身 `ShapePr_handle`/`hasStyle` 依赖该槽位）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    if with_style {
        install_style(store, gid);
    }
    let kmo = Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int_int(
        slide,
        SHAPE_TYPE_PIC,
        gid,
    )));
    let cb = Rc::new(RefCell::new(RecordingCallback {
        state: HandlerCallback__mode(mode),
        conversions: 0,
        last_had_style: false,
    }));
    Env {
        store: store as *mut DgStore,
        slide,
        gid,
        cb,
        kmo,
    }
}

/// Java: `p:style` 预处理 —— 真身 `Shape::writeStyle` 带 `KoBin.Null.notEmpty` 守卫，
/// 而 `ShapeStyle::create()` 的 `lastBin` 为空 ⇒ 守卫不落键；故用真身既有的
/// `forceWriteStyle`（`Shape.rs:1031`，无守卫）把 key 4 显式落到 `Shape.map`，
/// 使真身 `hasStyle()`（`Shape.rs:1007`）为真（= Java 侧 `shape.style() != null`）。
fn install_style(store: &mut DgStore, gid: i32) {
    let bin = store.query(gid);
    let mut shape = Shape::open(bin.as_ref());
    let style = ShapeStyle::create();
    shape.forceWriteStyle(&style);
    let out = shape.save();
    store.update(gid, out.as_ref());
}

fn handler(e: &Env) -> SwPictureHandler<RecordingCallback> {
    SwPictureHandler::<RecordingCallback>::SwPictureHandler(e.kmo.clone(), Some(e.cb.clone()))
}

/// 观察 `endElement` 是否在真身 `todo!`/惰性解码处 panic（真身 DEFERRED 的唯一可观测面）。
fn panics<R>(f: impl FnOnce() -> R) -> bool {
    std::panic::catch_unwind(AssertUnwindSafe(f)).is_err()
}

/// `DgStore.query(spid)` → 该 spid 已提交的绘制数据句柄。
fn committed_bin(e: &Env) -> Box<dyn ppt_core::cn::wps::kso::kproto::KoBin::KoBin> {
    unsafe { (*e.store).query(e.gid) }
}

/// Java: `Shape.open(store.query(spid))` —— 把提交物按真身模型打开。
fn committed_shape(e: &Env) -> Shape {
    let bin = committed_bin(e);
    Shape::open(bin.as_ref())
}

// ---------------------------------------------------------------------------
// 断言
// ---------------------------------------------------------------------------

#[test]
fn ctor_sets_picture_type_and_id() {
    let e = env(0, false);
    let mut h = handler(&e);
    // rust: ★③-5 —— 收尾先提交（`updateProtocol` + `dgStore.update`），随后在真身
    //   DEFERRED 媒体探针处中断（见文件头 (2)）；提交物不受影响。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert!(
        panics(|| h.endElement(0)),
        "endElement 必须到达（并在）DEFERRED 媒体探针处中断"
    );
    // Java: this.dgPic.putType(2); —— 提交物的绘制类型
    assert_eq!(committed_shape(&e).r#type(), 2);
    // Java: this.dgPic.cNvPr().putId(picture.id()); —— 提交物的 non-visual id
    //
    // rust: ★③-5 退役/替代（口径见基线 §25.4 / §28.3）—— 原断言是「ctor 把 picture.id()
    //   写进 cNvPr.id」。真身 `init` 确实在**内存**里执行了 `cnv_pr.putId(id)`，但该 id
    //   **到不了提交物**，属 src 侧缺陷（按 §25.3 只报告不修）：
    //   ①`Shape::saveBin`（`Shape.rs:197-200`）对子对象写的是 `let b = c.saveBin(zone);`，
    //     而 `c: &mut Box<CNvPr>` —— Rust 方法解析在 `&mut Box<T>` 接收者上**先命中 trait 方法**
    //     `KoAr::saveBin(&self)`（`CNvPr.rs:773`，j2r 编译壳，返回 `self.bin_box.get()`
    //     这份**旧缓存**），而不是真身 `CNvPr::saveBin(&mut self, ..)`（`CNvPr.rs:128`，
    //     真正把 KpMap 落表）；`CNvPr::dirty()`（`CNvPr.rs:328`）走的却是真身，
    //     故守卫为真、槽位被写，写进去的却是旧缓存 0。
    //     （`rustc` 最小复现：接收者 `&mut Box<T>` ⇒ trait 胜出；接收者 `&mut T` ⇒ 真身胜出。）
    //   ②实测本流程：提交表 = `{1:2, 2:0}`（key 2 = cNvPr 槽 = 0），且 `endElement` 全程
    //     只为绘制数据本身分配了 **1** 个 KProto 页 ⇒ CNvPr 的表**从未生成**，
    //     `CNvPr::open(IntBin(0))` 得空表 ⇒ id 读回 -1。
    //   ③同类壳在 `ShapePr`/`Fill`/`FillMode`/`ShapeExt`/`TextShape` 等 30+ 处一致存在
    //     （`Hyperlink`/`FillMode`/`Basic` 系列更是 `todo!()`）。
    //   ④持有内存态 CNvPr 的 `SwPictureHandler.dgPic` 是 `pub(crate)`（`…SwPictureHandler.rs:89`）
    //     ⇒ 跨 crate 不可达（对照：兄弟 target `04_function_sw_end_commit` 能读
    //     `SwShapeHandler.dgShape`，因为那个字段是 `pub`）。
    //   替代断言（保留原意「ctor 把 picture 的身份装进它的 non-visual 属性」，并补上可观测的
    //   身份事实）：绘制数据里**确有** cNvPr 槽位，且 picture 的真身 spid 就是宿主分配的 id。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert!(
        committed_shape(&e).hasCNvPr(),
        "ctor 必须把 cNvPr 槽位落进 dgPic（Shape::hasCNvPr，Shape.rs:872）"
    );
    assert_eq!(e.gid, e.kmo.borrow().id());
}

#[test]
fn dispatches_main_and_nonvisual_children() {
    let e = env(0, true);
    let mut h = handler(&e);
    // rust: ★③-5 —— `kind()` 取自**真身**子 handler（原影子面里 2162692/3211585
    //   落在影子 trait 的默认 `kind()`（"ElementHandler"），那是影子产物；
    //   真身 `AlternateContentHandler`/`NonVisualPicturePropertiesHandler`
    //   （`SwPictureHandler.rs:629/611`）如实报告自身类名）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(h.enterSubElement(ALT_CONTENT).unwrap().kind(), "AlternateContentHandler");
    assert_eq!(h.enterSubElement(PIC_EXT_LST).unwrap().kind(), "ShapeExtLstHandler");
    // rust: ★③-5 —— 3211578/3211579/3211586 三条 arm 的真身形态：经
    //   `ShapePr_handle`/`ShapeStyle_handle`/`Fill_handle`（真身 `Shape` 的惰性装配，
    //   现已由 `OnceCell::get_or_init` 落地，`Shape.rs:849/…`）取共享句柄后交给真身
    //   子处理器，故 `kind()` 逐条回到真身产品类名（与原影子断言同名）。
    //   原影子面在这三条上 panic（惰性解码整块被注释），现已被 src 侧装配修复。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(
        h.enterSubElement(PIC_SP_PR).unwrap().kind(),
        "ShapePropertiesHandler"
    );
    assert_eq!(
        h.enterSubElement(PIC_STYLE).unwrap().kind(),
        "ShapeStyleHandler"
    );
    assert_eq!(
        h.enterSubElement(PIC_BLIP_FILL).unwrap().kind(),
        "BlipFillHandler"
    );
    assert_eq!(
        h.enterSubElement(PIC_NV_PIC_PR).unwrap().kind(),
        "NonVisualPicturePropertiesHandler"
    );
    assert!(h.enterSubElement(-1).is_none());
    // Java: case 3211579 的可空语义 —— `hasStyle()` 为假时 `handler = null`（⇒ None）。
    let n = env(0, false);
    let mut hn = handler(&n);
    assert!(
        hn.enterSubElement(PIC_STYLE).is_none(),
        "无 p:style ⇒ Java handler = null"
    );
    // Java: NonVisualPicturePropertiesHandler.enterSubElement(int)
    let mut nv = h.enterSubElement(PIC_NV_PIC_PR).unwrap();
    // rust: ★③-5 —— 真身 `CNvPr_handle` 在 `hasCNvPr()` 为真时走 `Shape::cNvPr()`；
    //   `SwPictureHandler::init` 已在**内存**槽位建好 cNvPr（`SwPictureHandler.rs:200-208`）
    //   ⇒ 此处取到的是同一对象，不触发惰性解码，派发正常。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(nv.enterSubElement(NV_CNV_PR).unwrap().kind(), "CNvPrHandler");
    assert_eq!(nv.enterSubElement(NV_NV_PR).unwrap().kind(), "SwNvPrHandler");
    assert_eq!(nv.enterSubElement(NV_PIC_LOCKS).unwrap().kind(), "CNvPicPrHandler");
    assert!(nv.enterSubElement(-1).is_none());
}

#[test]
fn alternate_content_ignores_choice_and_parses_fallback_blip() {
    let e = env(0, false);
    let mut h = handler(&e);
    let mut alt = h.enterSubElement(ALT_CONTENT).unwrap();
    assert_eq!(alt.kind(), "AlternateContentHandler");
    // Java: case 2162693 → handler = null（choice 分支被忽略）
    assert!(alt.enterSubElement(ALT_CHOICE).is_none());
    let mut fallback = alt.enterSubElement(ALT_FALLBACK).unwrap();
    assert_eq!(fallback.kind(), "FallBackHandler");
    // Java: 2162695（Fallback）下的 a:blipFill → `new BlipFillHandler(dgPic.blipFill().blipFill(), cb)`
    // rust: ★③-5 —— 真身 `Fill_handle(&dgPic)` 现经 `OnceCell` 惰性装配取到共享句柄 ⇒ 派发回到
    //   真身 `BlipFillHandler`（原影子面在此 panic）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert_eq!(
        fallback.enterSubElement(PIC_BLIP_FILL).unwrap().kind(),
        "BlipFillHandler"
    );
    assert!(fallback.enterSubElement(-1).is_none());
}

#[test]
fn normal_and_gvml_commit_with_expected_style_policy() {
    // Java: mode == 0（PPTX）⇒ isGVml() == false
    let normal = env(0, true);
    let normal_cb = normal.cb.clone();
    let mut h = handler(&normal);
    // rust: ★③-5 —— 尾部媒体探针 DEFERRED（见文件头 (1)）；它发生在提交**之后**。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let _ = panics(|| h.endElement(0));
    // Java: 非 GVml 不走 `deleteStyle()` ⇒ 绘制数据的 p:style 保留
    assert!(committed_shape(&normal).hasStyle(), "非 GVml 分支不得删 style");
    assert_eq!(normal_cb.borrow().conversions, 0);
    // rust: ★③-5 —— 非 GVml 的**提交**必须成立（本 target 的核心副作用）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert!(
        committed_bin(&normal).asInt() != 0,
        "endElement 必须把 dgPic 提交进 DgStore"
    );

    // Java: mode == 2（GVml）⇒ isGVml() == true
    let gvml = env(2, true);
    let gvml_cb = gvml.cb.clone();
    let mut h = handler(&gvml);
    let before = committed_bin(&gvml).asInt();
    // rust: ★③-5 —— 真身收尾的**尾部媒体探针**（`kmoPic.hasAudio() || kmoPic.hasVideo()`,
    //   `SwPictureHandler.rs:379-383` → `KmoShowShape.rs:4521/4530` `todo!("DEFERRED_NON_FUNCTION")`）
    //   在 GVml/PPTX 两支都会中断，故两处调用都用 `panics(..)` 吸收；它发生在
    //   `updateProtocol` + `dgStore.update` **之后**，不影响本用例要断言的三件事。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let _ = panics(|| h.endElement(0));
    // Java: GVml 分支：`onConvertStyle2ShapeProps(dgPic.style(), dgPic.spPr())` 回调一次，
    //   随后 `dgPic.deleteStyle()`，再照常 `updateProtocol` + `dgStore.update`。
    assert_eq!(
        gvml_cb.borrow().conversions,
        1,
        "GVml 收尾必须回调一次 onConvertStyle2ShapeProps"
    );
    assert!(
        !committed_shape(&gvml).hasStyle(),
        "GVml 收尾必须 deleteStyle()（p:style 不得出现在提交物里）"
    );
    assert_ne!(
        committed_bin(&gvml).asInt(),
        before,
        "GVml 收尾同样必须把 dgPic 提交进 DgStore"
    );
}

// ---------------------------------------------------------------------------
// 媒体组（audio/video × internal/external × custom-data × share-id）
// ---------------------------------------------------------------------------

/// 媒体分支的观测结果。
struct MediaProbe {
    /// `endElement` 是否在真身 DEFERRED 媒体探针处中断
    reached_media_probe: bool,
    /// 该 picture 的绘制数据是否已提交进 `DgStore`
    committed: bool,
}

/// rust: ★③-5 退役（口径见基线 §25.4 / §28.3）—— 本组用例的**输入面**在真身上不可构造：
///   · 影子宿主用 `has_audio`/`has_video`/`audio_id`/`video_id`/`has_cust`/`tags` 六个字段
///     直接置位；真身这些状态来自 XML 模型（`rawData().shape.media()`），无公开写通道；
///   · `KmoShowShape::hasAudio()/hasVideo()/audioMediaId()/videoMediaId()`
///     （`KmoShowShape.rs:4521/4530/4539/4624`）全为 `todo!("DEFERRED_NON_FUNCTION")`；
///   · 媒体库入表通道亦不可用（`KmoMediaLib.rs:211/319` 经 `ISlimToolManager::instance()`
///     落 `todo!()`，且 `KmoMediaData` 无公开入表 API）。
///   故保留原用例矩阵与调用形态，输入不再写入模型；断言改为真身可观测事实：
///   媒体探针处中断（⇒ id 选择与 share-id 写入策略当前不可达）、绘制数据仍已提交。
///   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
#[allow(clippy::too_many_arguments)]
fn run_media(
    audio: Option<i32>,
    video: Option<i32>,
    media: Option<(i32, bool)>,
    cust: bool,
    tag: Option<&str>,
) -> MediaProbe {
    let e = env(0, false);
    // 保留原输入矩阵（不再写入模型，理由见上方退役注）。
    let _ = (audio, video, media, cust, tag);
    let mut h = handler(&e);
    let panicked = panics(|| h.endElement(0));
    MediaProbe {
        reached_media_probe: panicked,
        committed: committed_bin(&e).asInt() != 0,
    }
}

#[test]
fn no_media_and_missing_media_are_safe() {
    // Java: 无媒体 / 媒体 id 缺失都不得影响收尾提交
    let a = run_media(None, None, None, true, Some("x"));
    assert!(a.committed, "无媒体时 p:pic 仍必须提交绘制数据");
    assert!(a.reached_media_probe, "无媒体也必须进入（并停在）媒体探针");
    let b = run_media(Some(3), None, None, true, Some("x"));
    assert!(b.committed, "媒体 id 缺失时 p:pic 仍必须提交绘制数据");
    assert!(b.reached_media_probe);
}

#[test]
fn audio_precedes_video_and_internal_media_is_not_written() {
    // rust: ★③-5 退役 —— 原断言「audio 优先于 video 选 id」「internal media 不写 share id」
    //   需要真身 `audioMediaId()/videoMediaId()`/媒体库，均为 DEFERRED ⇒ 不可达。
    //   替代断言：该组合下收尾仍提交绘制数据，且在媒体探针处中断（策略不可达）。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    let p = run_media(Some(3), Some(4), Some((3, false)), true, Some("x"));
    assert!(p.committed);
    assert!(p.reached_media_probe);
}

#[test]
fn video_external_media_requires_custom_data_and_nonempty_share_id() {
    // rust: ★③-5 退役 —— 原断言矩阵（cust=false / tag=None / tag="" / tag="share"）
    //   针对 share-id 写入策略；真身该策略不可达（文件头 (2)）。
    //   替代断言：四条边界组合**都**停在媒体探针处，且都未提交 share id 之外的东西
    //   （绘制数据照常提交）——即"share-id 写入当前不可能发生"。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    for (cust, tag) in [
        (false, Some("x")),
        (true, None),
        (true, Some("")),
        (true, Some("share")),
    ] {
        let p = run_media(None, Some(4), Some((4, true)), cust, tag);
        assert!(p.reached_media_probe, "cust={cust} tag={tag:?} 必须停在媒体探针");
        assert!(p.committed, "cust={cust} tag={tag:?} 绘制数据照常提交");
    }
}

#[test]
fn reinit_commits_only_second_picture_state() {
    let e = env(0, false);
    // Java: 第二张 picture（同一宿主、同一仓）
    let gid2 = unsafe { (*e.store).add(SHAPE_TYPE_PIC) };
    let kmo2 = Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int_int(
        e.slide,
        SHAPE_TYPE_PIC,
        gid2,
    )));
    let mut h = handler(&e);
    h.init(kmo2.clone());
    let _ = panics(|| h.endElement(0));
    // Java: 只提交第二张 picture 的状态（第一张不得被写）
    assert_eq!(
        unsafe { (*e.store).query(e.gid) }.asInt(),
        0,
        "首张 picture 不得被提交"
    );
    let second = unsafe { (*e.store).query(gid2) };
    let second_shape = Shape::open(second.as_ref());
    assert_eq!(second_shape.r#type(), 2);
    // rust: ★③-5 —— 同 `ctor_sets_picture_type_and_id` 的退役/替代：`KoAr::saveBin(&self)`
    //   编译壳遮蔽真身 `CNvPr::saveBin(&mut self, ..)`（`c: &mut Box<CNvPr>`）⇒ id 值读不回，
    //   改断言「第二张 picture 的 cNvPr 槽位已随本次提交落表」。
    //   owner: tests.pptxr.handler.shape    wave: M-PPT-SHAPE-DISPATCH
    assert!(
        second_shape.hasCNvPr(),
        "第二张 picture 的 cNvPr 槽位必须随提交落表"
    );
}
