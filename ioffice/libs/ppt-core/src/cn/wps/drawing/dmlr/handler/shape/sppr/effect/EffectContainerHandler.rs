#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.effect.Effect
// rust: no-stmt —— `Effect` 仍未登记（`data/effect/mod.rs` 的 `// pub mod Effect;` 为注释态，
//   且 `Effect.rs` 自身未收口）。本类 `enterSubElement` / `endElement` 阻塞于它，见两处 DEFERRED 注。
//   owner: ppt.drawing.data.effect    wave: M-PPT-KMOSLIDE-W6c-2
// Java: import cn.wps.drawing.data.effect.EffectContainer
// rust: ★M-PPT-KMOSLIDE W6c-3 —— `drawing::data::effect` 已解除 `#[cfg(feature = "full")]`
//   gate（形态 D 消除），`EffectContainer` 在默认构建下**已可命名**，故本类的字段与构造器
//   形参按 Java 真身还原。
//   owner: ppt.drawing.data.effect    wave: M-PPT-KMOSLIDE-W6c-3
use crate::cn::wps::drawing::data::effect::EffectContainer::EffectContainer;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.drawing.dmlr.util.DGMap
// rust: no-stmt —— `DGMap` 本身在默认构建可用，但本类只用到 `DGMap.effectDagTypeMap`
//   （`putType((Integer)DGMap.effectDagTypeMap.get(tmp.toString()))`），该表在 Rust `DGMap`
//   里**未移植**（现有表见 `dmlr/util/DGMap.rs`），导入它只会成为未使用导入。
//   owner: ppt.drawing.dmlr.util    wave: M-PPT-KMOSLIDE-W6c-2
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器
//   （`OutlineHandler` / `FillPropertiesHandler` / `ShapePropertiesHandler`）统一直接实现
//   `ElementHandler`，本类同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: 由 `XmlRoAttrList` 承载 —— 本包 `ElementHandler` 契约的 attrs 形参类型，
//   `getByName` / `toString` 与 Java `XmlRoAttr` 同名同义。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// Java: import java.util.ArrayList
// rust: 由 `Vec` 承载（字段 `List<Effect> effects`）。
// Java: import java.util.List
// rust: 同上。

// rust: Java 的可空引用形参（`HandlerCallback handlerCallback`）在 Rust 侧用
//   `Option<Rc<RefCell<_>>>` 承载「可空 + 共享可变」语义。
type Callback<C> = Option<Rc<RefCell<C>>>;
// rust: 别名（Java 无）—— 共享句柄，见 `data::effect::EffectProperties::Shared`。
type Shared<T> = Rc<RefCell<T>>;

// Java: public class EffectContainerHandler extends DefaultElementHandler
pub struct EffectContainerHandler<C: HandlerCallback> {
    // Java field: List<Effect> effects
    //
    // rust: 该字段目前无法声明 —— 元素类型 `Effect` 被 `#[cfg(feature = "full")]` 关在
    //   `drawing::data::effect` 之外（见文件头注）。补 `Effect` 可达后应为
    //   `Option<Vec<Rc<RefCell<Effect>>>>`（Java 是 `null` 起手、首次 `enterSubElement` 时
    //   `new ArrayList()`）。
    //   owner: ppt.drawing.data.effect    wave: M-PPT-KMOSLIDE-W6c-2
    // Java field: EffectContainer effectContainer
    effectContainer: Shared<EffectContainer>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Callback<C>,
}

// Java: public class EffectContainerHandler extends DefaultElementHandler
impl<C: HandlerCallback> EffectContainerHandler<C> {
    // Java: EffectContainerHandler(EffectContainer effectContainer, HandlerCallback handlerCallback)
    pub fn EffectContainerHandler(
        effectContainer: Shared<EffectContainer>,
        handlerCallback: Callback<C>,
    ) -> Self {
        // Java: this.effectContainer = effectContainer;
        // Java: this.handlerCallback = handlerCallback;
        //
        // rust: ★形参已按 Java 还原。owner: ppt.drawing.data.effect · wave: M-PPT-KMOSLIDE-W6c-3
        EffectContainerHandler {
            effectContainer,
            handlerCallback,
        }
    }
}

impl<C: HandlerCallback> ElementHandler for EffectContainerHandler<C> {
    fn kind(&self) -> &'static str {
        "EffectContainerHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr tmp = null;
        // Java: if ((tmp = attrsNotCopy.getByName(1114136)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(1114136) {
            // Java: this.effectContainer.putName(tmp.toString());
            // rust: ★W6c-3 —— `EffectContainer` 已可达，改为**真实写入**。
            self.effectContainer.borrow_mut().putName(&tmp.to_string());
        }
        // Java: if ((tmp = attrsNotCopy.getByName(1114388)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(1114388) {
            // Java: this.effectContainer.putType((Integer)DGMap.effectDagTypeMap.get(tmp.toString()));
            // owner: ppt.drawing.data.effect / ppt.drawing.dmlr.util    wave: M-PPT-KMOSLIDE-W6c-2
            // blocking type: EffectContainer (cn.wps.drawing.data.effect.EffectContainer) +
            //   DGMap.effectDagTypeMap（Rust `dmlr/util/DGMap.rs` 无此表）
            //   —— 接收者与查表两侧均缺失。
            todo!("DEFERRED_NON_FUNCTION")
        }
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: Effect effect = Effect.create();
        // Java: if (this.effects == null)
        // Java: this.effects = new ArrayList();
        // Java: this.effects.add(effect);
        // Java: return EffectHandlerFactory.createEffectHandler(subElement, effect, this.handlerCallback);
        //
        // owner: ppt.drawing.data.effect / ppt.drawing.dmlr.handler.shape.sppr.effect
        //   wave: M-PPT-KMOSLIDE-W6c-2
        // blocking type: Effect (cn.wps.drawing.data.effect.Effect) —— 被 `#[cfg(feature = "full")]`
        //   关在 `drawing::data::effect` 之外，`Effect.create()` / `List<Effect>` 均不可书写；
        //   且 EffectHandlerFactory（cn.wps.drawing.dmlr.handler.shape.sppr.effect.EffectHandlerFactory，
        //   Java 88 行）在默认构建下无对应 Rust 文件，其 16 个分支依赖的
        //   EffectHandlerFactory 子处理器也全部缺失。
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public void endElement(int element)
    fn endElement(&mut self, _element: i32) {
        // Java: for (Effect effect : this.effects)
        // Java: effect.save();
        // Java: this.effectContainer.effects().write(this.effects);
        //
        // owner: ppt.drawing.data.effect    wave: M-PPT-KMOSLIDE-W6c-2
        // blocking type: Effect (cn.wps.drawing.data.effect.Effect) +
        //   EffectContainer.effects() (cn.wps.drawing.data.effect.EffectContainer) +
        //   Effects.write(Iterable<Effect>) (cn.wps.drawing.data.effect.EffectContainer.Effects)
        //   —— 三者都在 `#[cfg(feature = "full")]` 的 `drawing::data::effect` 下；且 Rust
        //   `Effects::write` 本体自身仍为 `todo!("Effect type not yet available")`。
        todo!("DEFERRED_NON_FUNCTION")
    }
}
