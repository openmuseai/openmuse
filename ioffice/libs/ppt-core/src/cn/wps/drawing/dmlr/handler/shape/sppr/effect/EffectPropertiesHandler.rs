#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// M-PPT-KMOSLIDE W6c-3：本文件由 Java 真身 `EffectPropertiesHandler.java`（31 行）真实移植。
//
// Java: package cn.wps.drawing.dmlr.handler.shape.sppr.effect
//
// ★依赖前置（本波已达成，故可真实移植）：
//   · `cn.wps.drawing.data.effect.EffectProperties` —— `drawing::data::effect` 原先被
//     `#[cfg(feature = "full")]` 关在默认构建之外（假编译面形态 D）；W6c-3 已解除，
//     并给 `effectLst()` / `effectDag()` 补上**共享句柄**访问器（见 `EffectProperties.rs`）。
//   · `EffectListHandler` / `EffectContainerHandler` —— 二者已在默认构建内且实现 ② 契约；
//     其**构造器形参**也随本波还原为 Java 真身（原先因类型不可命名而少一个形参）。
//
// ★契约（R14/§26.1 裁定）：本文件实现 **②** `drawing::dmlr::handler::ElementHandler`
//   （`&mut self` + `kind()` + 具体 `&XmlRoAttrList`），与真 S2 引擎
//   `UnifiedXmlReader::read__stream_elementhandler` 同一契约。
//   owner: ppt.drawing.dmlr.handler.shape.sppr.effect    wave: M-PPT-KMOSLIDE-W6c-3

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.effect.EffectProperties
use crate::cn::wps::drawing::data::effect::EffectProperties::EffectProperties;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器
//   （`OutlineHandler` / `FillPropertiesHandler` / `ShapePropertiesHandler` / `EffectListHandler`）
//   统一直接实现 `ElementHandler`，本类同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: 本行无对应 Java import —— Java 的 `startElement` 由父类继承，Rust 的契约要求显式
//   实现该方法，故需引入形参类型 `XmlRoAttrList`（本包契约的 attrs 类型）。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use super::EffectContainerHandler::EffectContainerHandler;
use super::EffectListHandler::EffectListHandler;

// rust: Java 的可空引用形参（`HandlerCallback handlerCallback`）在 Rust 侧用
//   `Option<Rc<RefCell<_>>>` 承载「可空 + 共享可变」语义。
type Callback<C> = Option<Rc<RefCell<C>>>;
// rust: 别名（Java 无）—— 共享句柄，见 `data::effect::EffectProperties::Shared`。
type Shared<T> = Rc<RefCell<T>>;

// rust: ★M-PPT-KMOSLIDE W6c-3 —— 非泛型调用点（`ShapePropertiesHandler` / `GroupShapePropertiesHandler`）
//   的 callback 承载 stand-in（Java 无此类型）。
//   Java 侧 `HandlerCallback` 是**一个具体类**，任何处理器都能拿到它；本 port 把它做成了泛型参数
//   `C`，而 `ShapePropertiesHandler` / `GroupShapePropertiesHandler` 本身**不是泛型的**
//   （其 callback 形参是 `fill::FillNotify::FillNotifyRef` stand-in，见 `ShapePropertiesHandler.rs`
//   的登记），无法命名 `C`。此处与 `FillNotify` 同一处置，给出一个**空实现**的 sink，使
//   `a:effectLst` / `a:effectDag` 子树可被真实构造并共享同一个 `EffectProperties` 句柄。
//   owner: drawing.dmlr.handler.shape.sppr.effect    wave: M-PPT-DATA-STUBS（callback 面统一收口）
#[derive(Default)]
pub struct EffectNotify {
    // Java: HandlerCallback.mode
    state: crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallbackState,
}

impl HandlerCallback for EffectNotify {
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

    fn callback_state(
        &self,
    ) -> &crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallbackState {
        &self.state
    }
}

// Java: public class EffectPropertiesHandler extends DefaultElementHandler
pub struct EffectPropertiesHandler<C: HandlerCallback> {
    // Java field: EffectProperties effectProperties
    effectProperties: Shared<EffectProperties>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Callback<C>,
}

// rust: ★W6c-3 —— 非泛型调用点的入口（Java 只有一个类，泛型是 Rust port 对 callback 的承载手段）。
//   与 `OutlineHandler::OutlineHandler__shapePr` 同一先例：给出「无 callback」构造，
//   使 `ShapePropertiesHandler`（非泛型）无需命名 `C`。
//   owner: drawing.dmlr.handler.shape.sppr.effect    wave: M-PPT-KMOSLIDE-W6c-3
impl EffectPropertiesHandler<EffectNotify> {
    pub fn EffectPropertiesHandler__no_callback(
        effectProperties: Shared<EffectProperties>,
    ) -> Self {
        EffectPropertiesHandler {
            effectProperties,
            handlerCallback: None,
        }
    }
}

// Java: public class EffectPropertiesHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> EffectPropertiesHandler<C> {
    // Java: public EffectPropertiesHandler(EffectProperties effectProperties, HandlerCallback handlerCallback)
    pub fn EffectPropertiesHandler(
        effectProperties: Shared<EffectProperties>,
        handlerCallback: Callback<C>,
    ) -> Self {
        // Java: this.effectProperties = effectProperties;
        // Java: this.handlerCallback = handlerCallback;
        EffectPropertiesHandler {
            effectProperties,
            handlerCallback,
        }
    }
}

impl<C: HandlerCallback + 'static> ElementHandler for EffectPropertiesHandler<C> {
    fn kind(&self) -> &'static str {
        "EffectPropertiesHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    // —— 继承 `DefaultElementHandler` 的空实现（Java 该成员体为空）。
    fn startElement(&mut self, _element: i32, _attrsNotCopy: &XmlRoAttrList) {}

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let mut handler: Option<Box<dyn ElementHandler>> = None;
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114389:
            1114389 => {
                // Java: this.effectProperties.putType(0);
                self.effectProperties.borrow_mut().putType(0);
                // Java: handler = new EffectListHandler(this.effectProperties.effectLst(), this.handlerCallback);
                let effectLst = self.effectProperties.borrow_mut().effectLst();
                handler = Some(Box::new(EffectListHandler::EffectListHandler(
                    effectLst,
                    self.handlerCallback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 1114390:
            1114390 => {
                // Java: this.effectProperties.putType(1);
                self.effectProperties.borrow_mut().putType(1);
                // Java: handler = new EffectContainerHandler(this.effectProperties.effectDag(), this.handlerCallback);
                let effectDag = self.effectProperties.borrow_mut().effectDag();
                handler = Some(Box::new(EffectContainerHandler::EffectContainerHandler(
                    effectDag,
                    self.handlerCallback.clone(),
                )));
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
