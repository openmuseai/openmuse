//! 07_function_slide_bg_clrmap —— M-PPT-KMOSLIDE W6c-3 新登记的**真身** handler 的接线验证。
//!
//! 覆盖（全部驱动**产品真身**，不使用任何影子 crate / `#[path]` 内联 / 假模块树）：
//!   · `slides::BackgroundHandler`（Java 76 行）—— `p:bg`(3211308) 的真身；
//!   · `slides::BackgroundPropertiesHandler`（Java 内部类，本 port 提升为模块级）；
//!   · `slides::ColorMapOverrideHandler`（Java 25 行）；
//!   · `dmlr::handler::style::StyleMatrixReferenceHandler`（Java 42 行）；
//!   · `dmlr::handler::shape::sppr::effect::EffectPropertiesHandler`（Java 31 行）。
//!
//! 契约：② `drawing::dmlr::handler::ElementHandler`（`&mut self` + `kind()` + `&XmlRoAttrList`），
//! 与真 S2 引擎同一契约；属性注入一律用真实 `XmlRoAttrList`。
//! owner: tests.pptxr.handler.slides    wave: M-PPT-KMOSLIDE-W6c-3

#![allow(non_snake_case, dead_code, unused_imports)]

use std::cell::RefCell;
use std::rc::Rc;

// rust: 被测真身实现的是 **② dmlr 契约**（`&mut self` + `kind()` + `&XmlRoAttrList`）。
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use ppt_core::cn::wps::drawing::data::effect::EffectProperties::EffectProperties;
use ppt_core::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
use ppt_core::cn::wps::drawing::data::style::StyleMatrixReference::StyleMatrixReference;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::sppr::effect::EffectPropertiesHandler::EffectPropertiesHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::style::StyleMatrixReferenceHandler::StyleMatrixReferenceHandler;
use ppt_core::cn::wps::drawing::dmlr::model::HandlerCallback::{
    HandlerCallback, HandlerCallbackState,
};
use ppt_core::cn::wps::show::data::background::Background::Background;
use ppt_core::cn::wps::show::pptxr::handler::slides::BackgroundHandler::{
    BackgroundHandler, BackgroundPropertiesHandler,
};
use ppt_core::cn::wps::show::pptxr::handler::slides::ColorMapOverrideHandler::ColorMapOverrideHandler;

// Java: 元素 id（与各 Java 类的 switch 逐一对应）
// `BackgroundHandler`
const BG_BW_MODE_ATTR: i32 = 1114443;
const BG_BW_MODE_ATTR_ALT: i32 = 3211605;
const BG_BG_PR: i32 = 3211603; // p:bgPr
const BG_BG_REF: i32 = 3211604; // p:bgRef
// `BackgroundPropertiesHandler`
const BGP_SHADE_TO_TITLE_ATTR: i32 = 3211602;
const BGP_NO_FILL: i32 = 1114370;
const BGP_SOLID_FILL: i32 = 1114371;
const BGP_GRAD_FILL: i32 = 1114372;
const BGP_PATT_FILL: i32 = 1114373;
const BGP_BLIP_FILL: i32 = 3211586;
const BGP_EFFECT_LST: i32 = 1114389;
const BGP_EFFECT_DAG: i32 = 1114390;
// `ColorMapOverrideHandler`
const CLRMAP_OVERRIDE_ATTR: i32 = 1114457;
const CLRMAP_OVERRIDE_MAP: i32 = 1114458;
// `StyleMatrixReferenceHandler`
const SMR_IDX_ATTR: i32 = 1114408;
const SMR_IDX_ATTR_ALT: i32 = 3211459;
// rust: 测试内的**真正未知**元素 id
const UNKNOWN: i32 = 999999;

// ---------------------------------------------------------------------------

struct DummyCb {
    state: HandlerCallbackState,
}

impl HandlerCallback for DummyCb {
    type DiagramPair = ();
    type Blip = ();
    type Color = ();
    type Hyperlink = ();
    type Media = ();
    type AudioFile = ();
    type QuickTimeFile = ();
    type VideoFile = ();
    type EmbeddedWAVAudioFile = ();
    type Model3D = ();
    type Shape = ();
    type ShapePr = ();
    type ShapeStyle = ();
    type TextRun = ();
    type TagLst = ();
    type FileManager = ();

    fn callback_state(&self) -> &HandlerCallbackState {
        &self.state
    }
}

fn cb() -> Option<Rc<RefCell<DummyCb>>> {
    Some(Rc::new(RefCell::new(DummyCb {
        state: HandlerCallbackState::default(),
    })))
}

fn background() -> Rc<RefCell<Background>> {
    Rc::new(RefCell::new(Background::new()))
}

/// 构造真实 `XmlRoAttrList`（② 契约的 attrs 形参类型）。
fn attrs(values: &[(i32, &str)]) -> XmlRoAttrList {
    let mut list = XmlRoAttrList::new();
    for (k, v) in values {
        list.add_attr(*k, v);
    }
    list
}

fn bg_handler(bg: Rc<RefCell<Background>>) -> BackgroundHandler<DummyCb> {
    BackgroundHandler::BackgroundHandler(bg, cb())
}

// ---------------------------------------------------------------------------
// BackgroundHandler
// ---------------------------------------------------------------------------

#[test]
fn background_handler_dispatch_table() {
    let mut h = bg_handler(background());
    assert_eq!(h.kind(), "BackgroundHandler");
    // Java: case 3211603 → BackgroundPropertiesHandler
    assert_eq!(
        h.enterSubElement(BG_BG_PR).unwrap().kind(),
        "BackgroundPropertiesHandler"
    );
    // Java: case 3211604 → StyleMatrixReferenceHandler
    assert_eq!(
        h.enterSubElement(BG_BG_REF).unwrap().kind(),
        "StyleMatrixReferenceHandler"
    );
    // Java: 无 default 分支 ⇒ 未命中返回 null
    for id in [UNKNOWN, -1, 0] {
        assert!(h.enterSubElement(id).is_none(), "subElement {id} 不应派发");
    }
}

#[test]
fn background_handler_consumes_bw_mode() {
    // Java: DGMap.blackWhiteModeMap.get("gray") = 2 ⇒ putBwMode(2)
    let bg = background();
    let mut h = bg_handler(bg.clone());
    h.startElement(0, &attrs(&[(BG_BW_MODE_ATTR, "gray")]));
    assert!(bg.borrow().hasBwMode(), "bwMode 必须被写入真身 Background");
    assert_eq!(bg.borrow().bwMode(), 2);

    // Java: 两个属性 id 取「先 1114443，再 3211605」的短路顺序
    let bg2 = background();
    let mut h2 = bg_handler(bg2.clone());
    h2.startElement(0, &attrs(&[(BG_BW_MODE_ATTR_ALT, "black")]));
    assert_eq!(bg2.borrow().bwMode(), 8);

    // Java: 映射表未命中 ⇒ 不写
    let bg3 = background();
    let mut h3 = bg_handler(bg3.clone());
    h3.startElement(0, &attrs(&[(BG_BW_MODE_ATTR, "not-a-mode")]));
    assert!(!bg3.borrow().hasBwMode(), "未命中映射表不得写入");
}

// ---------------------------------------------------------------------------
// BackgroundPropertiesHandler
// ---------------------------------------------------------------------------

#[test]
fn background_props_consumes_shade_to_title() {
    let bg = background();
    let mut h = BackgroundPropertiesHandler::BackgroundPropertiesHandler(bg.clone(), cb());
    assert_eq!(h.kind(), "BackgroundPropertiesHandler");
    // Java: 3211602 → BackgroundHandler.this.backgroudShape.putShadeToTitle(temp.toBoolean())
    h.startElement(0, &attrs(&[(BGP_SHADE_TO_TITLE_ATTR, "true")]));
    assert!(bg.borrow().hasShadeToTitle());
    assert!(bg.borrow().isShadeToTitle());
    // 未命中属性不写
    let bg2 = background();
    let mut h2 = BackgroundPropertiesHandler::BackgroundPropertiesHandler(bg2.clone(), cb());
    h2.startElement(0, &attrs(&[(UNKNOWN, "true")]));
    assert!(!bg2.borrow().hasShadeToTitle());
}

#[test]
fn background_props_dispatches_fill_and_effect() {
    let mut h =
        BackgroundPropertiesHandler::BackgroundPropertiesHandler(background(), cb());
    // Java: 1114371/1114372/1114373/3211586 → FillPropertiesHandler 的对应分支
    assert_eq!(h.enterSubElement(BGP_SOLID_FILL).unwrap().kind(), "SolidFillHandler");
    assert_eq!(
        h.enterSubElement(BGP_GRAD_FILL).unwrap().kind(),
        "GradientFillHandler"
    );
    assert_eq!(
        h.enterSubElement(BGP_PATT_FILL).unwrap().kind(),
        "PatternFillHandler"
    );
    // Java: 1114370 → putType(5) 后返回 null
    assert!(h.enterSubElement(BGP_NO_FILL).is_none());
    // Java: 1114389/1114390 → EffectPropertiesHandler 的对应分支
    assert_eq!(
        h.enterSubElement(BGP_EFFECT_LST).unwrap().kind(),
        "EffectListHandler"
    );
    assert_eq!(
        h.enterSubElement(BGP_EFFECT_DAG).unwrap().kind(),
        "EffectContainerHandler"
    );
    // Java: 无 default 分支
    assert!(h.enterSubElement(UNKNOWN).is_none());
}

#[test]
fn background_fill_and_effect_handles_are_the_backgrounds_own_slots() {
    // rust: ★共享句柄语义断言 —— `Background::fill_handle()` / `effect_handle()` 返回的是
    //   `Background` 自己的**同一个**对象；handler 写入后必须能从 Background 侧读到。
    let bg = background();
    let fill = bg.borrow_mut().fill_handle();
    let mut h =
        BackgroundPropertiesHandler::BackgroundPropertiesHandler(bg.clone(), cb());
    // Java: case 1114373: this.fill.putType(1)
    let _ = h.enterSubElement(BGP_PATT_FILL);
    assert_eq!(fill.borrow().fill_type(), 1, "fill() 返回同一对象（句柄）");

    let effect = bg.borrow_mut().effect_handle();
    let _ = h.enterSubElement(BGP_EFFECT_LST);
    // Java: case 1114389: this.effectProperties.putType(0)
    assert_eq!(effect.borrow().get_type(), 0, "effect() 返回同一对象（句柄）");
}

// ---------------------------------------------------------------------------
// ColorMapOverrideHandler
// ---------------------------------------------------------------------------

#[test]
fn clr_map_override_dispatch_table() {
    let map = Rc::new(RefCell::new(ColorMapping::create()));
    let mut h = ColorMapOverrideHandler::ColorMapOverrideHandler(map);
    assert_eq!(h.kind(), "ColorMapOverrideHandler");
    // Java: case 1114457 → null
    assert!(h.enterSubElement(CLRMAP_OVERRIDE_ATTR).is_none());
    // Java: case 1114458 → new ColorMapHandler(this.colorMapping)
    assert_eq!(
        h.enterSubElement(CLRMAP_OVERRIDE_MAP).unwrap().kind(),
        "ColorMapHandler"
    );
    // Java: default → null
    assert!(h.enterSubElement(UNKNOWN).is_none());
}

#[test]
fn clr_map_handler_writes_through_to_the_override_mapping() {
    // Java: ColorMapHandler.startElement 把 `a:accent1`(1114128) 之类按 schemeColorMap 写入
    //   同一个 `ColorMapping` ⇒ 必须能从本测试持有的句柄读到。
    let map = Rc::new(RefCell::new(ColorMapping::create()));
    let mut ovr = ColorMapOverrideHandler::ColorMapOverrideHandler(map.clone());
    let mut inner = ovr.enterSubElement(CLRMAP_OVERRIDE_MAP).unwrap();
    // DGMap.schemeColorMap.get("accent1") = 4 ⇒ putAccent1(4)
    inner.startElement(1114128, &attrs(&[(1114128, "accent1")]));
    assert_eq!(map.borrow().accent1(), 4, "写回的是同一个 ColorMapping");
}

// ---------------------------------------------------------------------------
// StyleMatrixReferenceHandler
// ---------------------------------------------------------------------------

#[test]
fn style_matrix_reference_consumes_index() {
    let reference = Rc::new(RefCell::new(StyleMatrixReference::create()));
    let mut h = StyleMatrixReferenceHandler::StyleMatrixReferenceHandler(
        reference.clone(),
        cb(),
        3211604,
    );
    assert_eq!(h.kind(), "StyleMatrixReferenceHandler");
    // Java: DGMap.fontCollectionIndexMap.get("major") = 0 ⇒ putIdx(0)
    h.startElement(0, &attrs(&[(SMR_IDX_ATTR, "major")]));
    assert!(reference.borrow().hasIdx(), "idx 必须被写入");
    assert_eq!(reference.borrow().idx(), 0);
    // Java: 备选属性 id 3211459
    h.startElement(0, &attrs(&[(SMR_IDX_ATTR_ALT, "minor")]));
    assert_eq!(reference.borrow().idx(), 1);
    // Java: 映射表未命中 ⇒ putIdx(tmp.toInt())
    h.startElement(0, &attrs(&[(SMR_IDX_ATTR, "7")]));
    assert_eq!(reference.borrow().idx(), 7);
    // Java: 两个属性都不在 ⇒ 不写
    let r2 = Rc::new(RefCell::new(StyleMatrixReference::create()));
    let mut h2 =
        StyleMatrixReferenceHandler::StyleMatrixReferenceHandler(r2.clone(), cb(), 3211604);
    h2.startElement(0, &attrs(&[(UNKNOWN, "major")]));
    // Java: `StyleMatrixReference.idx()` 未设置时返回默认值 0（`map.has(1)` 为假）
    assert!(!r2.borrow().hasIdx(), "未命中属性不得写入 idx");
}

// ---------------------------------------------------------------------------
// EffectPropertiesHandler（真身共享句柄）
// ---------------------------------------------------------------------------

#[test]
fn effect_properties_handler_uses_the_shared_effect_properties() {
    let effect = Rc::new(RefCell::new(EffectProperties::create()));
    let mut h = EffectPropertiesHandler::<DummyCb>::EffectPropertiesHandler(effect.clone(), cb());
    assert_eq!(h.kind(), "EffectPropertiesHandler");
    // Java: case 1114389 → putType(0) + EffectListHandler(effectLst())
    assert_eq!(h.enterSubElement(BGP_EFFECT_LST).unwrap().kind(), "EffectListHandler");
    assert_eq!(effect.borrow().get_type(), 0);
    // Java: case 1114390 → putType(1) + EffectContainerHandler(effectDag())
    assert_eq!(
        h.enterSubElement(BGP_EFFECT_DAG).unwrap().kind(),
        "EffectContainerHandler"
    );
    assert_eq!(effect.borrow().get_type(), 1);
    // Java: 未命中 ⇒ null
    assert!(h.enterSubElement(UNKNOWN).is_none());
}
