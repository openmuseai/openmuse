#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.shape.Shape
use crate::cn::wps::drawing::data::shape::Shape::Shape;
// rust: 本行无对应 Java import —— `Shape.Locking` 是 `Shape` 的**内部类**，Java 侧经
//   `import cn.wps.drawing.data.shape.Shape` 就能写 `Shape.Locking`；Rust 侧必须显式命名。
// Java: (Java 类型引用) Shape.Locking
use crate::cn::wps::drawing::data::shape::Shape::Locking;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本目录已登记的处理器
//   统一直接实现 `ElementHandler`，本类与其内部类 `GraphicFrameLocksHandler` 同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: 由 `XmlRoAttrList` 承载 —— 它是本包 `ElementHandler` 契约的 attrs 形参类型。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// rust: Java 的共享可变引用（`private Shape graphicFrame`）在 Rust 侧用 `Rc<RefCell<_>>` 承载。
type Shared<T> = Rc<RefCell<T>>;

// Java: public class CNvGraphicFramePrHandler extends DefaultElementHandler
//
// M-PPT-SHAPE-DISPATCH 新增：`SwGraphicFrameHandler`（`p:graphicFrame`）的 `p:nvGraphicFramePr`
//   子处理器。Java 侧 58 行；本文件是完整移植，无 `todo!`。
//   owner: cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvGraphicFramePrHandler
//   wave: M-PPT-SHAPE-DISPATCH
pub struct CNvGraphicFramePrHandler {
    // Java field: Shape graphicFrame
    graphicFrame: Shared<Shape>,
}

// Java: (inner class) CNvGraphicFramePrHandler.GraphicFrameLocksHandler
// Java: private class GraphicFrameLocksHandler extends DefaultElementHandler
pub struct GraphicFrameLocksHandler {
    // rust: 本行无对应 Java **字段** —— Java 的 `GraphicFrameLocksHandler` 是**无字段**的内部类，
    //   它在 `startElement` 里**每次**重新取 `CNvGraphicFramePrHandler.this.graphicFrame.locking()`
    //   （见 Java `startElement` 第一行），即"锁定对象"是**外层处理器**的字段。
    //   Rust 侧无法在内部类里做 `Outer.this` 的外层借用（外部类字段 `graphicFrame` 是私有的、
    //   且内部类与外层类无所有权关系），故把该宿主 `Shape` 句柄作为内部类字段显式传入 ——
    //   这与 Java 的**语义**等价：每次 `put` 都落到同一个 `Shape.locking()` 上，
    //   且"经外层字段取用"这一步仍被推迟到每次 put（见 `with_graphicalObjectFrameLocking`）。
    //   同目录 `CNvPicPrHandler::PictureLocksHandler`、`CNvCxnSpPrHandler::ConnectorLockingHandler`
    //   是同一处理。
    //   owner: ppt.drawing.data.shape    wave: M-PPT-SHAPE-DISPATCH
    graphicFrame: Shared<Shape>,
}

// Java: public class CNvGraphicFramePrHandler extends DefaultElementHandler
impl CNvGraphicFramePrHandler {
    // Java: CNvGraphicFramePrHandler(Shape nvGraphicFramePr)
    pub fn CNvGraphicFramePrHandler(nvGraphicFramePr: Shared<Shape>) -> Self {
        // Java: this.graphicFrame = nvGraphicFramePr;
        CNvGraphicFramePrHandler {
            graphicFrame: nvGraphicFramePr,
        }
    }

    // rust: 本方法无对应 Java 方法 —— `new` 是 Rust 侧 `SwGraphicFrameHandler`
    //   （Java 写作 `new CNvGraphicFramePrHandler(this.dgGraphicFrame)`）使用的构造名，
    //   形参、语义与上面的 `CNvGraphicFramePrHandler(...)` 完全一致；同 `CNvPicPrHandler::new`。
    pub fn new(nvGraphicFramePr: Shared<Shape>) -> Self {
        CNvGraphicFramePrHandler::CNvGraphicFramePrHandler(nvGraphicFramePr)
    }
}

impl ElementHandler for CNvGraphicFramePrHandler {
    fn kind(&self) -> &'static str {
        "CNvGraphicFramePrHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy) —— 继承空实现
    fn startElement(&mut self, _element: i32, _attrsNotCopy: &XmlRoAttrList) {}

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114257:
            //   1114257 = `a:graphicFrameLocks` 子元素，承载 `Shape.Locking`（图文框锁定属性）
            1114257 => {
                // Java: handler = new GraphicFrameLocksHandler();
                // Java: (Java 的 case 无 break，紧接着 `default: return handler;` ⇒ 直接返回)
                Some(Box::new(GraphicFrameLocksHandler::GraphicFrameLocksHandler(
                    self.graphicFrame.clone(),
                )))
            }
            // Java: default: return handler;
            _ => None,
        }
    }

    // Java: public void endElement(int element)   —— 继承 `DefaultElementHandler` 的空实现
    fn endElement(&mut self, _element: i32) {}
}

// Java: public class GraphicFrameLocksHandler extends DefaultElementHandler
impl GraphicFrameLocksHandler {
    // Java: private GraphicFrameLocksHandler()   —— 无参私有构造器（无字段赋值）
    //
    // rust: 形参是"外层 host"的显式承载，理由见字段注；Java 的无参构造器本身无语句。
    pub fn GraphicFrameLocksHandler(graphicFrame: Shared<Shape>) -> Self {
        // Java: (无参构造器，无赋值语句)
        GraphicFrameLocksHandler { graphicFrame }
    }

    // rust: 本方法无对应 Java 语句 —— Java 在 `startElement` 首行取
    //   `CNvGraphicFramePrHandler.this.graphicFrame.locking()` 并复用同一个局部变量；
    //   Rust 侧把"取用宿主内嵌 `Locking`"收进这一处，`startElement` 的每条 `put` 语句
    //   保持与 Java 逐行对齐。
    fn with_graphicalObjectFrameLocking(&self, f: impl FnOnce(&mut Locking)) {
        // Java: Shape.Locking graphicalObjectFrameLocking = CNvGraphicFramePrHandler.this.graphicFrame.locking();
        // rust: `Shape.locking` 是 crate 内可见字段；用 `if let` 而非 `Shape::locking()`
        //   （后者在槽位为 None 时 `unwrap()` panic）。
        if let Some(locking) = self.graphicFrame.borrow_mut().locking.get_mut() {
            f(locking);
        }
    }
}

impl ElementHandler for GraphicFrameLocksHandler {
    fn kind(&self) -> &'static str {
        "GraphicFrameLocksHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp = null;
        // Java: if ((temp = attrsNotCopy.getByName(1114232)) != null)
        //   1114232 = `a:noChangeAspect`
        if let Some(temp) = attrsNotCopy.get_by_name(1114232) {
            // Java: graphicalObjectFrameLocking.putNoChangeAspect(temp.toBoolean());
            self.with_graphicalObjectFrameLocking(|l| l.putNoChangeAspect(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114229)) != null)
        //   1114229 = `a:noGrp`
        if let Some(temp) = attrsNotCopy.get_by_name(1114229) {
            // Java: graphicalObjectFrameLocking.putNoGrp(temp.toBoolean());
            self.with_graphicalObjectFrameLocking(|l| l.putNoGrp(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114233)) != null)
        //   1114233 = `a:noMove`
        if let Some(temp) = attrsNotCopy.get_by_name(1114233) {
            // Java: graphicalObjectFrameLocking.putNoMove(temp.toBoolean());
            self.with_graphicalObjectFrameLocking(|l| l.putNoMove(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114234)) != null)
        //   1114234 = `a:noResize`
        if let Some(temp) = attrsNotCopy.get_by_name(1114234) {
            // Java: graphicalObjectFrameLocking.putNoResize(temp.toBoolean());
            self.with_graphicalObjectFrameLocking(|l| l.putNoResize(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114230)) != null)
        //   1114230 = `a:noSelect`
        if let Some(temp) = attrsNotCopy.get_by_name(1114230) {
            // Java: graphicalObjectFrameLocking.putNoSelect(temp.toBoolean());
            self.with_graphicalObjectFrameLocking(|l| l.putNoSelect(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114242)) != null)
        //   1114242 = `a:noDrilldown`
        if let Some(temp) = attrsNotCopy.get_by_name(1114242) {
            // Java: graphicalObjectFrameLocking.putNoDrilldown(temp.toBoolean());
            self.with_graphicalObjectFrameLocking(|l| l.putNoDrilldown(temp.to_boolean()));
        }
    }

    // Java: (继承 `DefaultElementHandler`) —— 内部类未覆写 `enterSubElement`
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: return null;
        None
    }

    // Java: public void endElement(int element)   —— 继承 `DefaultElementHandler` 的空实现
    fn endElement(&mut self, _element: i32) {}
}
