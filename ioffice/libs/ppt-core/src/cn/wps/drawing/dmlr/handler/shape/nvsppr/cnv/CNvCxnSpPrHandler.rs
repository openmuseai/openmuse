#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.shape.Shape
use crate::cn::wps::drawing::data::shape::Shape::Shape;
// rust: 本行无对应 Java import —— `Shape.Locking` / `Shape.Connection` 是 `Shape` 的**内部类**，
//   Java 侧经 `import cn.wps.drawing.data.shape.Shape` 就能写 `Shape.Locking`；Rust 侧必须显式
//   命名这两个类型（内部类 handler 的构造语义与字段类型都用得到）。
// Java: (Java 类型引用) Shape.Locking
use crate::cn::wps::drawing::data::shape::Shape::Locking;
// Java: (Java 类型引用) Shape.Connection
use crate::cn::wps::drawing::data::shape::Shape::Connection;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本目录已登记的处理器
//   （`CNvPicPrHandler` / `CNvPrHandler`）统一直接实现 `ElementHandler`，本类与其两个内部类同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: 由 `XmlRoAttrList` 承载 —— 它是本包 `ElementHandler` 契约的 attrs 形参类型，
//   `getByName` / `toBoolean` / `toInt` 与 Java `XmlRoAttr` 同名同义。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// rust: Java 的共享可变引用（`private Shape connector`）在 Rust 侧用 `Rc<RefCell<_>>` 承载
//   （「共享 + 可变」语义），同 `CNvPicPrHandler` 对 `Shape` 的处理。
type Shared<T> = Rc<RefCell<T>>;

// Java: public class CNvCxnSpPrHandler extends DefaultElementHandler
//
// M-PPT-SHAPE-DISPATCH 新增：`SwConnectorShapeHandler`（`p:cxnSp`）的 `p:nvCxnSpPr` 子处理器。
//   Java 侧 102 行；本文件是完整移植，无 `todo!`。
//   owner: cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvCxnSpPrHandler
//   wave: M-PPT-SHAPE-DISPATCH
pub struct CNvCxnSpPrHandler {
    // Java field: Shape connector
    connector: Shared<Shape>,
}

// Java: (inner class) CNvCxnSpPrHandler.ConnectorLockingHandler
// Java: private class ConnectorLockingHandler extends DefaultElementHandler
pub struct ConnectorLockingHandler {
    // Java field: Shape.Locking connectorLocking
    //
    // rust: 该字段承载为**宿主 `Shape` 的共享句柄**（而非 `Rc<RefCell<Locking>>`）。
    //   Java 的 `this.connector.locking()` 返回引用，`this.connectorLocking.putXxx(...)` 直接
    //   写回 `Shape` 持有的那个 `Locking`；Rust 侧 `Shape::locking()`（`data/shape/Shape.rs:1568`）
    //   只返回 `&Locking`，槽位为 None 时 `unwrap()` panic，且 `Locking` 没有 `Clone`
    //   ⇒ 在本目录内取不出 `Rc<RefCell<Locking>>` 句柄。持有宿主后按需就地取用
    //   （见 `with_connectorLocking`），写入仍落在**同一个** `Locking` 上，语义等价。
    //   这与同目录 `CNvPicPrHandler::PictureLocksHandler`（`CNvPicPrHandler.rs:36-47,125-132`）
    //   的处理完全一致。
    //   owner: ppt.drawing.data.shape    wave: M-PPT-SHAPE-DISPATCH
    connectorLocking: Shared<Shape>,
}

// Java: (inner class) CNvCxnSpPrHandler.ConnectionHandler
// Java: private class ConnectionHandler extends DefaultElementHandler
pub struct ConnectionHandler {
    // Java field: Shape.Connection connection
    //
    // rust: 同 `ConnectorLockingHandler.connectorLocking` 的理由 —— 承载宿主 `Shape`，
    //   另以 `slot` 区分 Java 侧构造时传入的是 `this.connector.stCxn()` 还是
    //   `this.connector.endCxn()`（Java 用**同一个类**接收两个不同字段的引用，
    //   Rust 侧无法持有 `&mut Connection` 的共享句柄，故把"指向哪一个槽位"显式记下来）。
    //   见 `with_connection`。
    //   owner: ppt.drawing.data.shape    wave: M-PPT-SHAPE-DISPATCH
    connection: Shared<Shape>,
    // Java: (构造实参来源) `this.connector.stCxn()` 或 `this.connector.endCxn()`
    slot: CxnSlot,
}

/// `Shape.Connection` 的两个命名槽位：Java 的 `Shape.stCxn()` / `Shape.endCxn()`。
///
/// rust: 本 enum 无对应 Java 类型 —— Java 的 `ConnectionHandler` 直接持有 `Shape.Connection`
///   引用，因此「哪一个槽位」由引用本身携带；Rust 侧把该信息提升为显式判别值。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CxnSlot {
    /// Java: `Shape.stCxn()`
    St,
    /// Java: `Shape.endCxn()`
    End,
}

// Java: public class CNvCxnSpPrHandler extends DefaultElementHandler
impl CNvCxnSpPrHandler {
    // Java: CNvCxnSpPrHandler(Shape connector)
    pub fn CNvCxnSpPrHandler(connector: Shared<Shape>) -> Self {
        // Java: this.connector = connector;
        let connector = connector;
        CNvCxnSpPrHandler { connector }
    }

    // rust: 本方法无对应 Java 方法 —— `new` 是 Rust 侧 `SwConnectorShapeHandler`
    //   （`p:nvCxnSpPr`，Java 写作 `new CNvCxnSpPrHandler(this.dgCxnSp)`）使用的构造名，
    //   形参、语义与上面的 `CNvCxnSpPrHandler(...)` 完全一致；同 `CNvPicPrHandler::new`。
    pub fn new(connector: Shared<Shape>) -> Self {
        CNvCxnSpPrHandler::CNvCxnSpPrHandler(connector)
    }
}

impl ElementHandler for CNvCxnSpPrHandler {
    fn kind(&self) -> &'static str {
        "CNvCxnSpPrHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy) —— 继承空实现
    fn startElement(&mut self, _element: i32, _attrsNotCopy: &XmlRoAttrList) {}

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        // rust: 该 `null` 初值即下面 `_` 分支的返回值。
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114251:
            //   1114251 = `a:cxnSpLocks` 子元素，承载 `Shape.Locking`（连接形状锁定属性）
            1114251 => {
                // Java: handler = new ConnectorLockingHandler(this.connector.locking());
                //
                // rust: 形参传宿主 `Shape`（见 `ConnectorLockingHandler.connectorLocking`
                //   字段注：它等价于 Java 的 `this.connector.locking()`，只是把「经宿主取用」
                //   推迟到每次 put）。
                Some(Box::new(ConnectorLockingHandler::ConnectorLockingHandler(
                    self.connector.clone(),
                )))
            }
            // Java: case 1114252:
            //   1114252 = `a:stCxn` 子元素，承载 `Shape.Connection`（起点连接）
            1114252 => {
                // Java: handler = new ConnectionHandler(this.connector.stCxn());
                Some(Box::new(ConnectionHandler::ConnectionHandler(
                    self.connector.clone(),
                    CxnSlot::St,
                )))
            }
            // Java: case 1114253:
            //   1114253 = `a:endCxn` 子元素，承载 `Shape.Connection`（终点连接）
            1114253 => {
                // Java: handler = new ConnectionHandler(this.connector.endCxn());
                // Java: (Java 的 case 无 break，落到 switch 末尾后返回 handler)
                Some(Box::new(ConnectionHandler::ConnectionHandler(
                    self.connector.clone(),
                    CxnSlot::End,
                )))
            }
            // Java: default: return handler;
            _ => None,
        }
    }

    // Java: public void endElement(int element)   —— 继承 `DefaultElementHandler` 的空实现
    fn endElement(&mut self, _element: i32) {}
}

// Java: public class ConnectorLockingHandler extends DefaultElementHandler
impl ConnectorLockingHandler {
    // Java: ConnectorLockingHandler(Shape.Locking connectorLocking)
    pub fn ConnectorLockingHandler(connectorLocking: Shared<Shape>) -> Self {
        // Java: this.connectorLocking = connectorLocking;
        let connectorLocking = connectorLocking;
        ConnectorLockingHandler { connectorLocking }
    }

    // rust: 本方法无对应 Java 语句 —— Java 的 `this.connectorLocking` 就是构造时取到的
    //   `Shape.Locking` 引用，可以直接 `putXxx`；Rust 侧按字段注持有宿主 `Shape`，
    //   故把「取用宿主内嵌 `Locking`」收进这一处，`startElement` 的每条 `put` 语句
    //   保持与 Java 逐行对齐（`this.connectorLocking.putXxx(v)` ⇒
    //   `self.with_connectorLocking(|l| l.putXxx(v))`）。
    fn with_connectorLocking(&self, f: impl FnOnce(&mut Locking)) {
        // Java: this.connector.locking()（构造器实参的来源）
        // rust: `Shape.locking` 是 crate 内可见字段；用 `if let` 而非 `Shape::locking()`
        //   （后者在槽位为 None 时 `unwrap()` panic）。
        if let Some(connectorLocking) = self.connectorLocking.borrow_mut().locking.get_mut() {
            f(connectorLocking);
        }
    }
}

impl ElementHandler for ConnectorLockingHandler {
    fn kind(&self) -> &'static str {
        "ConnectorLockingHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp = null;
        // Java: if ((temp = attrsNotCopy.getByName(1114236)) != null)
        //   1114236 = `a:noAdjustHandles`
        if let Some(temp) = attrsNotCopy.get_by_name(1114236) {
            // Java: this.connectorLocking.putNoAdjustHandles(temp.toBoolean());
            self.with_connectorLocking(|l| l.putNoAdjustHandles(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114237)) != null)
        //   1114237 = `a:noChangeArrowheads`
        if let Some(temp) = attrsNotCopy.get_by_name(1114237) {
            // Java: this.connectorLocking.putNoChangeArrowHeads(temp.toBoolean());
            self.with_connectorLocking(|l| l.putNoChangeArrowHeads(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114232)) != null)
        //   1114232 = `a:noChangeAspect`
        if let Some(temp) = attrsNotCopy.get_by_name(1114232) {
            // Java: this.connectorLocking.putNoChangeAspect(temp.toBoolean());
            self.with_connectorLocking(|l| l.putNoChangeAspect(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114238)) != null)
        //   1114238 = `a:noChangeShapeType`
        if let Some(temp) = attrsNotCopy.get_by_name(1114238) {
            // Java: this.connectorLocking.putNoChangeShapeType(temp.toBoolean());
            self.with_connectorLocking(|l| l.putNoChangeShapeType(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114235)) != null)
        //   1114235 = `a:noEditPoints`
        if let Some(temp) = attrsNotCopy.get_by_name(1114235) {
            // Java: this.connectorLocking.putNoEditPoints(temp.toBoolean());
            self.with_connectorLocking(|l| l.putNoEditPoints(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114229)) != null)
        //   1114229 = `a:noGrp`
        if let Some(temp) = attrsNotCopy.get_by_name(1114229) {
            // Java: this.connectorLocking.putNoGrp(temp.toBoolean());
            self.with_connectorLocking(|l| l.putNoGrp(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114233)) != null)
        //   1114233 = `a:noMove`
        if let Some(temp) = attrsNotCopy.get_by_name(1114233) {
            // Java: this.connectorLocking.putNoMove(temp.toBoolean());
            self.with_connectorLocking(|l| l.putNoMove(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114234)) != null)
        //   1114234 = `a:noResize`
        if let Some(temp) = attrsNotCopy.get_by_name(1114234) {
            // Java: this.connectorLocking.putNoResize(temp.toBoolean());
            self.with_connectorLocking(|l| l.putNoResize(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114231)) != null)
        //   1114231 = `a:noRot`
        if let Some(temp) = attrsNotCopy.get_by_name(1114231) {
            // Java: this.connectorLocking.putNoRot(temp.toBoolean());
            self.with_connectorLocking(|l| l.putNoRot(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114230)) != null)
        //   1114230 = `a:noSelect`
        if let Some(temp) = attrsNotCopy.get_by_name(1114230) {
            // Java: this.connectorLocking.putNoSelect(temp.toBoolean());
            self.with_connectorLocking(|l| l.putNoSelect(temp.to_boolean()));
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

// Java: public class ConnectionHandler extends DefaultElementHandler
impl ConnectionHandler {
    // Java: ConnectionHandler(Shape.Connection connection)
    pub fn ConnectionHandler(connection: Shared<Shape>, slot: CxnSlot) -> Self {
        // Java: this.connection = connection;
        // Java: (Rust) this.slot = （构造实参来自 stCxn 还是 endCxn）
        ConnectionHandler { connection, slot }
    }

    // rust: 本方法无对应 Java 语句 —— 理由同 `ConnectorLockingHandler::with_connectorLocking`。
    fn with_connection(&self, f: impl FnOnce(&mut Connection)) {
        // Java: this.connector.stCxn() / this.connector.endCxn()（构造器实参的来源）
        let mut shape = self.connection.borrow_mut();
        let slot = match self.slot {
            CxnSlot::St => shape.stCxn.get_mut(),
            CxnSlot::End => shape.endCxn.get_mut(),
        };
        if let Some(connection) = slot {
            f(connection);
        }
    }
}

impl ElementHandler for ConnectionHandler {
    fn kind(&self) -> &'static str {
        "ConnectionHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp = null;
        // Java: if ((temp = attrsNotCopy.getByName(1114245)) != null)
        //   1114245 = `a:stCxn@id` / `a:endCxn@id`
        if let Some(temp) = attrsNotCopy.get_by_name(1114245) {
            // Java: this.connection.putId(temp.toInt());
            self.with_connection(|c| c.putId(temp.to_int()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114408)) != null)
        //   1114408 = `a:stCxn@idx` / `a:endCxn@idx`
        if let Some(temp) = attrsNotCopy.get_by_name(1114408) {
            // Java: this.connection.putIdx(temp.toInt());
            self.with_connection(|c| c.putIdx(temp.to_int()));
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

