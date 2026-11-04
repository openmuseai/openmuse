#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// M-PPT-KMOSLIDE W6c-3：本文件由 Java 真身 `BackgroundHandler.java`（76 行）真实移植并**登记**。
//
// Java: package cn.wps.show.pptxr.handler.slides
//
// ★本波消除的阻塞（原文件头登记的「p:bg 遗留阻塞」三项，逐条已闭合）：
//   1) 字段类型 `cn.wps.show.data.background.Background` 在默认构建下**已可达**：
//      `drawing::data::effect` 的 `#[cfg(feature = "full")]` gate 已解除（形态 D 消除），
//      `show/mod.rs` 的内联桩 data 模块已登记真实 `background`，
//      `CommonSlideData.rs` 顶部的内联简化桩 `Background` 也已删除（形态 C 消除）。
//   2) `EffectPropertiesHandler`（Java 31 行）已在本波真实移植并登记。
//   3) `StyleMatrixReferenceHandler`（Java 42 行）已在本波真实移植并登记；
//      `FillPropertiesHandler` 经 `Background::{fill,effect,bgRef}_handle()` 的共享句柄接入。
//   ⇒ 原先的「影子 ① 契约 + `sw_shape_hosts::BackgroundHandler` 空壳」不再需要，
//     本文件按 ② 契约真实落地，`CommonSlideDataHandler` 的 3211308 因此派发到**真身**。
//
// ★契约（R14/§26.1 裁定）：实现 **②** `drawing::dmlr::handler::ElementHandler`
//   （`&mut self` + `kind()` + 具体 `&XmlRoAttrList`），与真 S2 引擎同一契约。
//
// ★登记偏差（逐条）：
//   · Java 的内部类 `BackgroundPropertiesHandler`（私有，捕获外部实例）在本 port 提升为
//     **模块级结构体**（与 `ControlListHandler` 的内部类同一处置），字段与 Java 捕获
//     一致（同一个 `backgroudShape` 句柄 + 同一个 callback）。
//   · `FillPropertiesHandler` 的 Rust 端口持有 `FillNotifyRef`（`FillNotify` 独立结构体）
//     而非 Java 的 `HandlerCallback` ⇒ 本文件按该端口的现有形参调用（不传 callback），
//     与 `ShapePropertiesHandler.rs` 的既有约定一致。该 stand-in 属 `M-PPT-DATA-STUBS`
//     边界，不影响本文件的门面（`enterSubElement` 的 case 集合与 Java 一致）。
//   owner: show.pptxr.handler.slides    wave: M-PPT-KMOSLIDE-W6c-3

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.effect.EffectPropertiesHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::effect::EffectPropertiesHandler::EffectPropertiesHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.fill.FillPropertiesHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillPropertiesHandler::FillPropertiesHandler;
// Java: import cn.wps.drawing.dmlr.handler.style.StyleMatrixReferenceHandler
use crate::cn::wps::drawing::dmlr::handler::style::StyleMatrixReferenceHandler::StyleMatrixReferenceHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.drawing.dmlr.util.DGMap
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器统一
//   直接实现 `ElementHandler`，本类同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: 由 `XmlRoAttrList` 承载（② 契约的 attrs 形参类型，`getByName`/`toString`/`toBoolean` 同名同义）。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// Java: import cn.wps.show.data.background.Background
use crate::cn::wps::show::data::background::Background::Background;

// rust: Java 的可空引用形参（`HandlerCallback handlerCallback`）在 Rust 侧用
//   `Option<Rc<RefCell<_>>>` 承载「可空 + 共享可变」语义。
type Callback<C> = Option<Rc<RefCell<C>>>;
// rust: 别名（Java 无）—— 共享句柄，见 `show::data::background::Background::Shared`。
type Shared<T> = Rc<RefCell<T>>;

// Java: class BackgroundHandler extends DefaultElementHandler
pub struct BackgroundHandler<C: HandlerCallback> {
    // Java field: Background backgroudShape
    //   （Java 字段名原文即 `backgroudShape`，保持不动以维持门面一致）
    backgroudShape: Shared<Background>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Callback<C>,
}

// Java: class BackgroundHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> BackgroundHandler<C> {
    // Java: public BackgroundHandler(Background background, HandlerCallback handlerCallback)
    pub fn BackgroundHandler(
        background: Shared<Background>,
        handlerCallback: Callback<C>,
    ) -> Self {
        let this = BackgroundHandler {
            handlerCallback: handlerCallback.clone(),
            backgroudShape: background,
        };
        // Java: this.handlerCallback = handlerCallback;
        // Java: this.backgroudShape = background;
        // rust: no-stmt
        this
    }
}

impl<C: HandlerCallback + 'static> ElementHandler for BackgroundHandler<C> {
    fn kind(&self) -> &'static str {
        "BackgroundHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp = null;
        let mut temp = attrsNotCopy.get_by_name(1114443);
        // Java: if ((temp = attrsNotCopy.getByName(1114443)) != null || (temp = attrsNotCopy.getByName(3211605)) != null)
        if temp.is_none() {
            temp = attrsNotCopy.get_by_name(3211605);
        }
        if let Some(temp) = temp {
            // Java: Integer blackWhiteMode = (Integer)DGMap.blackWhiteModeMap.get(temp.toString());
            let blackWhiteMode = DGMap::blackWhiteModeMap()
                .get(temp.to_string().as_str())
                .copied();
            // Java: if (blackWhiteMode != null)
            if let Some(blackWhiteMode) = blackWhiteMode {
                // Java: this.backgroudShape.putBwMode(blackWhiteMode);
                self.backgroudShape.borrow_mut().putBwMode(blackWhiteMode);
            }
        }
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let mut handler: Option<Box<dyn ElementHandler>> = None;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211603:
            3211603 => {
                // Java: handler = new BackgroundPropertiesHandler();
                handler = Some(Box::new(
                    BackgroundPropertiesHandler::BackgroundPropertiesHandler(
                        self.backgroudShape.clone(),
                        self.handlerCallback.clone(),
                    ),
                ));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211604:
            3211604 => {
                // Java: handler = new StyleMatrixReferenceHandler(this.backgroudShape.bgRef(), this.handlerCallback, 3211604);
                let bgRef = self.backgroudShape.borrow_mut().bgRef_handle();
                handler = Some(Box::new(
                    StyleMatrixReferenceHandler::StyleMatrixReferenceHandler(
                        bgRef,
                        self.handlerCallback.clone(),
                        3211604,
                    ),
                ));
            }
            // Java: default: （无 default 分支，`handler` 保持 null）
            _ => {}
        }
        // Java: return handler;
        handler
    }

    // Java: public void endElement(int element)
    // —— 继承 `DefaultElementHandler` 的空实现（Java 该成员体为空）。
    fn endElement(&mut self, _element: i32) {}
}

// Java: private class BackgroundPropertiesHandler extends DefaultElementHandler
//
// rust: ★Java 的**内部类**（捕获 `BackgroundHandler.this`）在本 port 提升为模块级结构体，
//   字段与 Java 的捕获一致（同一个 background 句柄 + 同一个 callback）。
pub struct BackgroundPropertiesHandler<C: HandlerCallback> {
    // Java: （内部类捕获 BackgroundHandler.this.backgroudShape）
    backgroudShape: Shared<Background>,
    // Java: （内部类捕获 BackgroundHandler.this.handlerCallback）
    handlerCallback: Callback<C>,
}

impl<C: HandlerCallback + 'static> BackgroundPropertiesHandler<C> {
    // Java: private BackgroundPropertiesHandler()
    pub fn BackgroundPropertiesHandler(
        backgroudShape: Shared<Background>,
        handlerCallback: Callback<C>,
    ) -> Self {
        BackgroundPropertiesHandler {
            backgroudShape,
            handlerCallback,
        }
    }
}

impl<C: HandlerCallback + 'static> ElementHandler for BackgroundPropertiesHandler<C> {
    fn kind(&self) -> &'static str {
        "BackgroundPropertiesHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp = null;
        let temp = attrsNotCopy.get_by_name(3211602);
        // Java: if ((temp = attrsNotCopy.getByName(3211602)) != null)
        if let Some(temp) = temp {
            // Java: BackgroundHandler.this.backgroudShape.putShadeToTitle(temp.toBoolean());
            self.backgroudShape
                .borrow_mut()
                .putShadeToTitle(temp.to_boolean());
        }
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let mut handler: Option<Box<dyn ElementHandler>> = None;
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114370: case 1114371: case 1114372: case 1114373: case 3211586:
            1114370 | 1114371 | 1114372 | 1114373 | 3211586 => {
                // Java: handler = (new FillPropertiesHandler(BackgroundHandler.this.backgroudShape.fill(), BackgroundHandler.this.handlerCallback)).enterSubElement(subElement);
                let fill = self.backgroudShape.borrow_mut().fill_handle();
                handler =
                    FillPropertiesHandler::FillPropertiesHandler(fill).enterSubElement(subElement);
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 1114389: case 1114390:
            1114389 | 1114390 => {
                // Java: handler = (new EffectPropertiesHandler(BackgroundHandler.this.backgroudShape.effect(), BackgroundHandler.this.handlerCallback)).enterSubElement(subElement);
                let effect = self.backgroudShape.borrow_mut().effect_handle();
                handler = EffectPropertiesHandler::EffectPropertiesHandler(
                    effect,
                    self.handlerCallback.clone(),
                )
                .enterSubElement(subElement);
            }
            // Java: default: （无 default 分支，`handler` 保持 null）
            _ => {}
        }
        // Java: return handler;
        handler
    }

    // Java: public void endElement(int element)
    // —— 继承 `DefaultElementHandler` 的空实现。
    fn endElement(&mut self, _element: i32) {}
}
