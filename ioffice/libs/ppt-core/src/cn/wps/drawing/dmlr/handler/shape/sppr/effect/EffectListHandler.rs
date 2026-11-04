#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.effect.EffectList
// rust: ★M-PPT-KMOSLIDE W6c-3 —— `drawing::data::effect` 已解除 `#[cfg(feature = "full")]`
//   gate（形态 D 消除），`EffectList` 在默认构建下**已可命名**，故本类的字段与构造器
//   形参按 Java 真身还原（原先因类型不可命名而少一个形参，属可消除的门面偏差）。
//   `enterSubElement` 的 8 个取值器仍阻塞于 8 个**无 Rust 文件**的子 handler
//   （BlurHandler/FillOverlayHandler/…），保持 `DEFERRED_NON_FUNCTION` 登记。
//   owner: ppt.drawing.data.effect    wave: M-PPT-KMOSLIDE-W6c-3
use crate::cn::wps::drawing::data::effect::EffectList::EffectList;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器
//   （`OutlineHandler` / `FillPropertiesHandler` / `ShapePropertiesHandler`）统一直接实现
//   `ElementHandler`，本类同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: 本行无对应 Java import —— Java 的 `startElement` 由父类继承（签名里才出现
//   `XmlRoAttr`），Rust 的 `ElementHandler` 契约要求显式实现该方法，故需引入形参类型
//   `XmlRoAttrList`（本包契约的 attrs 类型）。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// rust: Java 的可空引用形参（`HandlerCallback handlerCallback`）在 Rust 侧用
//   `Option<Rc<RefCell<_>>>` 承载「可空 + 共享可变」语义。
type Callback<C> = Option<Rc<RefCell<C>>>;
// rust: 别名（Java 无）—— 共享句柄，见 `data::effect::EffectProperties::Shared`。
type Shared<T> = Rc<RefCell<T>>;

// Java: public class EffectListHandler extends DefaultElementHandler
pub struct EffectListHandler<C: HandlerCallback> {
    // Java field: EffectList effectList
    effectList: Shared<EffectList>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Callback<C>,
}

// Java: public class EffectListHandler extends DefaultElementHandler
impl<C: HandlerCallback> EffectListHandler<C> {
    // Java: EffectListHandler(EffectList efffectList, HandlerCallback handlerCallback)
    //   （Java 形参名原文即 `efffectList`）
    pub fn EffectListHandler(efffectList: Shared<EffectList>, handlerCallback: Callback<C>) -> Self {
        // Java: this.effectList = efffectList;
        // Java: this.handlerCallback = handlerCallback;
        //
        // rust: ★形参已按 Java 还原（Java 形参名原文即 `efffectList`）。
        //   owner: ppt.drawing.data.effect    wave: M-PPT-KMOSLIDE-W6c-3
        EffectListHandler {
            effectList: efffectList,
            handlerCallback,
        }
    }
}

impl<C: HandlerCallback> ElementHandler for EffectListHandler<C> {
    fn kind(&self) -> &'static str {
        "EffectListHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    // —— 继承 `DefaultElementHandler` 的空实现
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114357:
            1114357 => {
                // Java: return new BlurHandler(this.effectList.blur());
                // owner: ppt.drawing.dmlr.handler.shape.sppr.effect    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: BlurHandler
                //   (cn.wps.drawing.dmlr.handler.shape.sppr.effect.BlurHandler)
                //   —— 该处理器在默认构建下无对应 Rust 文件；且取值器 `EffectList.blur()`
                //   所在的 `data::effect` 被 `full` 关掉。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: case 1114361:
            1114361 => {
                // Java: return new FillOverlayHandler(this.effectList.fillOverlay(), this.handlerCallback);
                // owner: ppt.drawing.dmlr.handler.shape.sppr.effect    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: FillOverlayHandler
                //   (cn.wps.drawing.dmlr.handler.shape.sppr.effect.FillOverlayHandler)
                //   —— 同上：无 Rust 对应文件；`EffectList.fillOverlay()` 在 `full` 之外。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: case 1114381:
            1114381 => {
                // Java: return new GlowHandler(this.effectList.glow(), this.handlerCallback);
                // owner: ppt.drawing.dmlr.handler.shape.sppr.effect    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: GlowHandler
                //   (cn.wps.drawing.dmlr.handler.shape.sppr.effect.GlowHandler)
                //   —— 同上：无 Rust 对应文件；`EffectList.glow()` 在 `full` 之外。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: case 1114382:
            1114382 => {
                // Java: return new InnerShadowHandler(this.effectList.innerShadow(), this.handlerCallback);
                // owner: ppt.drawing.dmlr.handler.shape.sppr.effect    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: InnerShadowHandler
                //   (cn.wps.drawing.dmlr.handler.shape.sppr.effect.InnerShadowHandler)
                //   —— 同上：无 Rust 对应文件；`EffectList.innerShadow()` 在 `full` 之外。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: case 1114383:
            1114383 => {
                // Java: return new OuterShadowHandler(this.effectList.outerShadow(), this.handlerCallback);
                // owner: ppt.drawing.dmlr.handler.shape.sppr.effect    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: OuterShadowHandler
                //   (cn.wps.drawing.dmlr.handler.shape.sppr.effect.OuterShadowHandler)
                //   —— 同上：无 Rust 对应文件；`EffectList.outerShadow()` 在 `full` 之外。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: case 1114384:
            1114384 => {
                // Java: return new PresetShadowHandler(this.effectList.presetShadowEffect(), this.handlerCallback);
                // owner: ppt.drawing.dmlr.handler.shape.sppr.effect    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: PresetShadowHandler
                //   (cn.wps.drawing.dmlr.handler.shape.sppr.effect.PresetShadowHandler)
                //   —— 同上：无 Rust 对应文件；`EffectList.presetShadowEffect()` 在 `full` 之外。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: case 1114385:
            1114385 => {
                // Java: return new ReflectionHandler(this.effectList.reflection());
                // owner: ppt.drawing.dmlr.handler.shape.sppr.effect    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: ReflectionHandler
                //   (cn.wps.drawing.dmlr.handler.shape.sppr.effect.ReflectionHandler)
                //   —— 同上：无 Rust 对应文件；`EffectList.reflection()` 在 `full` 之外。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: case 1114387:
            1114387 => {
                // Java: return new SoftEdgeHandler(this.effectList.softEdge());
                // owner: ppt.drawing.dmlr.handler.shape.sppr.effect    wave: M-PPT-KMOSLIDE-W6c-2
                // blocking type: SoftEdgeHandler
                //   (cn.wps.drawing.dmlr.handler.shape.sppr.effect.SoftEdgeHandler)
                //   —— 同上：无 Rust 对应文件；`EffectList.softEdge()` 在 `full` 之外。
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: public void endElement(int element)   —— 继承 `DefaultElementHandler` 的空实现
    fn endElement(&mut self, _element: i32) {}
}
