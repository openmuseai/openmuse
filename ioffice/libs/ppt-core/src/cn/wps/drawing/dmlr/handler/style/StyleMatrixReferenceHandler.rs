#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// M-PPT-KMOSLIDE W6c-3：本文件由 Java 真身 `StyleMatrixReferenceHandler.java`（42 行）真实移植。
//
// Java: package cn.wps.drawing.dmlr.handler.style
//
// ★本波达成的前置：
//   · `DGMap.fontCollectionIndexMap` 原先在 Rust 侧**缺失**（Java `DGMap.java:692-698` 有）
//     ⇒ 本波按 Java 逐项补出（见 `dmlr/util/DGMap.rs`），`startElement` 的映射分支因此可真实落地。
//   · `StyleMatrixReference` 已在默认构建内（`drawing/data/style/mod.rs`）。
//
// ★唯一的**登记偏差**（`enterSubElement`）：Java 是
//   `ColorHandlerFactory.createColorHandler(subElement, this.reference.color(), …)`，
//   其中 `reference.color()` 是**真模型** `cn.wps.drawing.data.color.Color`；而本 port 的
//   `ColorHandlerFactory`（`dmlr/handler/color/`）形参是 `Rc<RefCell<layout_shape::ColorSlot>>`
//   —— app-KMO 的**扁平投影**类型（渲染对象模型，裁定 R13 / 里程碑 M-PPT-RENDER-DEFACADE）。
//   两个 `Color` 表示之间**没有已登记的投影函数**，且该分支不参与本里程碑的装配管线
//   （`a:bgRef` 的配色在产物 A 中只影响背景主题引用，不进入形状树结构/oracle 判据）。
//   故按 PLAN §16「非管线核心」规则以 `todo!("DEFERRED_NON_FUNCTION")` 停车并具名登记。
//   owner: ppt.drawing.dmlr.handler.style    wave: M-PPT-RENDER-DEFACADE
//
// ★契约（R14/§26.1 裁定）：实现 **②** `drawing::dmlr::handler::ElementHandler`。

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.style.StyleMatrixReference
use crate::cn::wps::drawing::data::style::StyleMatrixReference::StyleMatrixReference;
// Java: import cn.wps.drawing.dmlr.handler.color.ColorHandlerFactory
use crate::cn::wps::drawing::dmlr::handler::color::ColorHandlerFactory::ColorHandlerFactory;
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
// rust: 由 `XmlRoAttrList` 承载 —— 本包 `ElementHandler` 契约的 attrs 形参类型，
//   `getByName` / `toString` / `toInt` 与 Java `XmlRoAttr` 同名同义。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// rust: Java 的可空引用形参（`HandlerCallback handlerCallback`）在 Rust 侧用
//   `Option<Rc<RefCell<_>>>` 承载「可空 + 共享可变」语义。
type Callback<C> = Option<Rc<RefCell<C>>>;
// rust: 别名（Java 无）—— 共享句柄，见 `data::effect::EffectProperties::Shared`。
type Shared<T> = Rc<RefCell<T>>;

// Java: public class StyleMatrixReferenceHandler extends DefaultElementHandler
pub struct StyleMatrixReferenceHandler<C: HandlerCallback> {
    // Java field: StyleMatrixReference reference
    reference: Shared<StyleMatrixReference>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Callback<C>,
    // Java field: int mBelonger
    mBelonger: i32,
}

// Java: public class StyleMatrixReferenceHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> StyleMatrixReferenceHandler<C> {
    // Java: public StyleMatrixReferenceHandler(StyleMatrixReference reference, HandlerCallback handlerCallback, int belonger)
    pub fn StyleMatrixReferenceHandler(
        reference: Shared<StyleMatrixReference>,
        handlerCallback: Callback<C>,
        belonger: i32,
    ) -> Self {
        // Java: this.init(reference);
        // Java: this.handlerCallback = handlerCallback;
        // Java: this.mBelonger = belonger;
        let mut this = StyleMatrixReferenceHandler {
            reference: reference.clone(),
            handlerCallback,
            mBelonger: belonger,
        };
        this.init(reference);
        this
    }

    // Java: public void init(StyleMatrixReference reference)
    pub fn init(&mut self, reference: Shared<StyleMatrixReference>) {
        // Java: this.reference = reference;
        self.reference = reference;
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement_inherent(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr tmp = null;
        let mut tmp = attrsNotCopy.get_by_name(1114408);
        // Java: if ((tmp = attrsNotCopy.getByName(1114408)) != null || (tmp = attrsNotCopy.getByName(3211459)) != null)
        if tmp.is_none() {
            tmp = attrsNotCopy.get_by_name(3211459);
        }
        if let Some(tmp) = tmp {
            // Java: Integer idx = (Integer)DGMap.fontCollectionIndexMap.get(tmp.toString());
            let idx = DGMap::fontCollectionIndexMap()
                .get(tmp.to_string().as_str())
                .copied();
            // Java: if (idx != null)
            if let Some(idx) = idx {
                // Java: this.reference.putIdx(idx);
                self.reference.borrow_mut().putIdx(idx);
            }
            // Java: else
            else {
                // Java: this.reference.putIdx(tmp.toInt());
                let v = tmp.to_int();
                self.reference.borrow_mut().putIdx(v);
            }
        }
    }
}

impl<C: HandlerCallback + 'static> ElementHandler for StyleMatrixReferenceHandler<C> {
    fn kind(&self) -> &'static str {
        "StyleMatrixReferenceHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
        StyleMatrixReferenceHandler::startElement_inherent(self, element, attrsNotCopy);
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: return ColorHandlerFactory.createColorHandler(subElement, this.reference.color(), this.handlerCallback, this.mBelonger);
        //
        // owner: ppt.drawing.dmlr.handler.style    wave: M-PPT-RENDER-DEFACADE
        // blocking type: cn.wps.drawing.data.color.Color（真模型）↔
        //   `show::util::layout_shape::ColorSlot`（app-KMO 扁平投影，`ColorHandlerFactory` 的形参类型）
        //   —— 二者无已登记投影函数；且 `ColorHandlerFactory` 实现 ② 契约但绑定的是投影模型。
        //   本分支不参与本里程碑装配管线（见文件头），故按 PLAN §16 停车。
        let _ = (subElement, self.mBelonger);
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public void endElement(int element)
    // —— 继承 `DefaultElementHandler` 的空实现。
    fn endElement(&mut self, _element: i32) {}
}
