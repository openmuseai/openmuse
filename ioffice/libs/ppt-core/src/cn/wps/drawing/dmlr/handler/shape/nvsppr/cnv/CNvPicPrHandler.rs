#![allow(non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.shape.Shape
use crate::cn::wps::drawing::data::shape::Shape::Shape;
// rust: 本行无对应 Java import —— `Shape.Locking` 是 `Shape` 的**内部类**，Java 侧经
//   `import cn.wps.drawing.data.shape.Shape` 就能写 `Shape.Locking`；Rust 侧必须显式命名该
//   类型（内部类 `PictureLocksHandler` 的取用闭包形参类型、也是其 Java 字段类型）。
// Java: (Java 类型引用) Shape.Locking
use crate::cn::wps::drawing::data::shape::Shape::Locking;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器
//   （`HyperLinkHandler` / `OutlineHandler` / `FillPropertiesHandler`）统一直接实现
//   `ElementHandler`，本类与其内部类 `PictureLocksHandler` 同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: 由 `XmlRoAttrList` 承载 —— 它是本包 `ElementHandler` 契约的 attrs 形参类型，
//   `getByName` / `toString` / `toBoolean` 与 Java `XmlRoAttr` 同名同义。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// rust: Java 的共享可变引用（`private Shape pic`）在 Rust 侧用 `Rc<RefCell<_>>` 承载
//   （「共享 + 可变」语义），同 `HyperLinkHandler` 对 `Hyperlink` 的处理。
type Shared<T> = Rc<RefCell<T>>;

// Java: public class CNvPicPrHandler extends DefaultElementHandler
pub struct CNvPicPrHandler {
    // Java field: Shape pic
    pic: Shared<Shape>,
}

// Java: (inner class) CNvPicPrHandler.PictureLocksHandler
// Java: private class PictureLocksHandler extends DefaultElementHandler
pub struct PictureLocksHandler {
    // Java field: Shape.Locking shapeLocks
    //
    // rust: 该字段承载为**宿主 `Shape` 的共享句柄**（而非 `Rc<RefCell<Locking>>`）。
    //   Java 的 `this.pic.locking()` 返回引用，`this.shapeLocks.putXxx(...)` 直接写回
    //   `Shape` 持有的那个 `Locking`；Rust 侧 `Shape::locking()`（`data/shape/Shape.rs:1568`）
    //   只返回 `&Locking`，且槽位为 None 时 `unwrap()` panic，`Locking` 又没有 `Clone`
    //   ⇒ 在 `cnv/` 目录内取不出 `Rc<RefCell<Locking>>` 句柄。持有宿主后按需就地取用
    //   （见 `with_shapeLocks`），写入仍落在**同一个** `Locking` 上，语义等价。
    //   owner: ppt.drawing.data.shape    wave: W8b
    shapeLocks: Shared<Shape>,
}

// Java: public class CNvPicPrHandler extends DefaultElementHandler
impl CNvPicPrHandler {
    // Java: CNvPicPrHandler(Shape pic)
    pub fn CNvPicPrHandler(pic: Shared<Shape>) -> Self {
        // Java: this.pic = pic;
        let pic = pic;
        CNvPicPrHandler { pic }
    }

    // rust: 本方法无对应 Java 方法 —— `new` 是 `SwPictureHandler` 既有调用点
    //   （`CNvPicPrHandler::new(self.dgPic.clone())`，实参即 `Rc<RefCell<Shape>>`）
    //   与本里程碑编译面假替身（`tests/cn/wps/show/pptxr/handler/shape/SwPictureHandler.rs:150`）
    //   共同使用的构造名（Java 侧写作 `new CNvPicPrHandler(...)`）；形参、语义与
    //   上面的 `CNvPicPrHandler(...)` 完全一致。
    pub fn new(pic: Shared<Shape>) -> Self {
        CNvPicPrHandler::CNvPicPrHandler(pic)
    }
}

impl ElementHandler for CNvPicPrHandler {
    fn kind(&self) -> &'static str {
        "CNvPicPrHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp = null;
        // Java: if ((temp = attrsNotCopy.getByName(1114255)) != null)
        //   1114255 = `p:cNvPicPr@preferRelativeResize`（属性），设 `Shape.preferRelativeResize`
        if let Some(temp) = attrsNotCopy.get_by_name(1114255) {
            // Java: this.pic.putPreferRelativeResize(temp.toBoolean());
            self.pic.borrow_mut().putPreferRelativeResize(temp.to_boolean());
        }
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        // rust: 该 `null` 初值即下面 `_` 分支的返回值。
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114254:
            //   1114254 = `a:picLocks` 子元素，承载 `Shape.Locking`（图片锁定属性）
            1114254 => {
                // Java: handler = new PictureLocksHandler(this.pic.locking());
                // Java: default: return handler;   // Java 的 case 无 break，落到 default 后返回
                //
                // rust: 形参传宿主 `Shape`（见 `PictureLocksHandler.shapeLocks` 字段注：
                //   它等价于 Java 的 `this.pic.locking()`，只是把「经宿主取用」推迟到每次 put）。
                Some(Box::new(PictureLocksHandler::PictureLocksHandler(
                    self.pic.clone(),
                )))
            }
            // Java: default: return handler;
            _ => None,
        }
    }

    // Java: public void endElement(int element)   —— 继承 `DefaultElementHandler` 的空实现
    fn endElement(&mut self, _element: i32) {}
}

// Java: public class PictureLocksHandler extends DefaultElementHandler
impl PictureLocksHandler {
    // Java: PictureLocksHandler(Shape.Locking shapeLocks)
    pub fn PictureLocksHandler(shapeLocks: Shared<Shape>) -> Self {
        // Java: this.shapeLocks = shapeLocks;
        let shapeLocks = shapeLocks;
        PictureLocksHandler { shapeLocks }
    }

    // rust: 本方法无对应 Java 语句 —— Java 的 `this.shapeLocks` 就是构造时取到的
    //   `Shape.Locking` 引用，可以直接 `putXxx`；Rust 侧按字段注持有宿主 `Shape`，
    //   故把「取用宿主内嵌 `Locking`」收进这一处，`startElement` 的每条 `put` 语句
    //   保持与 Java 逐行对齐（`this.shapeLocks.putXxx(v)` ⇒
    //   `self.with_shapeLocks(|shapeLocks| shapeLocks.putXxx(v))`）。
    fn with_shapeLocks(&self, f: impl FnOnce(&mut Locking)) {
        // Java: this.pic.locking()（构造器实参的来源）
        // rust: `Shape.locking` 是 crate 内可见字段；用 `if let` 而非 `Shape::locking()`
        //   （后者在槽位为 None 时 `unwrap()` panic）。
        if let Some(shapeLocks) = self.shapeLocks.borrow_mut().locking.get_mut() {
            f(shapeLocks);
        }
    }
}

impl ElementHandler for PictureLocksHandler {
    fn kind(&self) -> &'static str {
        "PictureLocksHandler"
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp = null;
        // Java: if ((temp = attrsNotCopy.getByName(1114236)) != null)
        //   1114236 = `a:noAdjustHandles`
        if let Some(temp) = attrsNotCopy.get_by_name(1114236) {
            // Java: this.shapeLocks.putNoAdjustHandles(temp.toBoolean());
            self.with_shapeLocks(|shapeLocks| shapeLocks.putNoAdjustHandles(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114237)) != null)
        //   1114237 = `a:noChangeArrowheads`
        if let Some(temp) = attrsNotCopy.get_by_name(1114237) {
            // Java: this.shapeLocks.putNoChangeArrowHeads(temp.toBoolean());
            self.with_shapeLocks(|shapeLocks| shapeLocks.putNoChangeArrowHeads(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114232)) != null)
        //   1114232 = `a:noChangeAspect`
        if let Some(temp) = attrsNotCopy.get_by_name(1114232) {
            // Java: this.shapeLocks.putNoChangeAspect(temp.toBoolean());
            self.with_shapeLocks(|shapeLocks| shapeLocks.putNoChangeAspect(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114238)) != null)
        //   1114238 = `a:noChangeShapeType`
        if let Some(temp) = attrsNotCopy.get_by_name(1114238) {
            // Java: this.shapeLocks.putNoChangeShapeType(temp.toBoolean());
            self.with_shapeLocks(|shapeLocks| shapeLocks.putNoChangeShapeType(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114235)) != null)
        //   1114235 = `a:noEditPoints`
        if let Some(temp) = attrsNotCopy.get_by_name(1114235) {
            // Java: this.shapeLocks.putNoEditPoints(temp.toBoolean());
            self.with_shapeLocks(|shapeLocks| shapeLocks.putNoEditPoints(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114229)) != null)
        //   1114229 = `a:noGrp`
        if let Some(temp) = attrsNotCopy.get_by_name(1114229) {
            // Java: this.shapeLocks.putNoGrp(temp.toBoolean());
            self.with_shapeLocks(|shapeLocks| shapeLocks.putNoGrp(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114233)) != null)
        //   1114233 = `a:noMove`
        if let Some(temp) = attrsNotCopy.get_by_name(1114233) {
            // Java: this.shapeLocks.putNoMove(temp.toBoolean());
            self.with_shapeLocks(|shapeLocks| shapeLocks.putNoMove(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114234)) != null)
        //   1114234 = `a:noResize`
        if let Some(temp) = attrsNotCopy.get_by_name(1114234) {
            // Java: this.shapeLocks.putNoResize(temp.toBoolean());
            self.with_shapeLocks(|shapeLocks| shapeLocks.putNoResize(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114231)) != null)
        //   1114231 = `a:noRot`
        if let Some(temp) = attrsNotCopy.get_by_name(1114231) {
            // Java: this.shapeLocks.putNoRot(temp.toBoolean());
            self.with_shapeLocks(|shapeLocks| shapeLocks.putNoRot(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114230)) != null)
        //   1114230 = `a:noSelect`
        if let Some(temp) = attrsNotCopy.get_by_name(1114230) {
            // Java: this.shapeLocks.putNoSelect(temp.toBoolean());
            self.with_shapeLocks(|shapeLocks| shapeLocks.putNoSelect(temp.to_boolean()));
        }

        // Java: if ((temp = attrsNotCopy.getByName(1114240)) != null)
        //   1114240 = `a:noCrop`
        if let Some(temp) = attrsNotCopy.get_by_name(1114240) {
            // Java: this.shapeLocks.putNoCrop(temp.toBoolean());
            self.with_shapeLocks(|shapeLocks| shapeLocks.putNoCrop(temp.to_boolean()));
        }
    }

    // Java: (继承 `DefaultElementHandler`) —— 内部类未覆写 `enterSubElement`/`endElement`
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: return null;
        None
    }

    // Java: public void endElement(int element)   —— 继承 `DefaultElementHandler` 的空实现
    fn endElement(&mut self, _element: i32) {}
}
