#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// M-PPT-KMOSLIDE W9：本文件此前**未登记**（`slides/mod.rs` 只登记 CommonSlideDataHandler /
// SlideHandler），因此 288 行从不进入编译单元；`CommonSlideDataHandler` 反而导入
// `shape::sw_shape_hosts::ControlListHandler` —— 一个**同名单元空壳**（`sw_shape_hosts.rs:262`，
// `ControlListHandler<S, D, C>(_, _, _)` 丢弃全部三个实参，且 `impl ElementHandler` 为空体）。
// ⇒ `p:controls`(3211607) 子树（`p:control` / `mc:AlternateContent` / `mc:Fallback` / `p:pic`）
// 全被静默丢弃。W9 按 Java 真身（`ControlListHandler.java` 98 行）改写并登记，随后删除该空壳。
//
// ★契约（R14）：规范契约 `kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::ElementHandler`
//   （`startElement(&self, i32, &dyn XmlRoAttr)` / `endElement(&self, i32)` /
//     `characters(&self, &str)` / `enterSubElement(&self, i32) -> Option<Box<dyn ElementHandler>>`），
//   即 `UnifiedXmlReader::read__inputstream_elementhandler` 的形参类型；全部接收者为 `&self`，
//   故 Java 的实例可变状态（`imageW` / `imageH` / `picture`）一律经 `Cell` / `RefCell`。
//   **不得**加 `kind()`（非该 trait 成员，会 E0407）；**不得**写
//   `impl ElementHandler for X {}`（该 trait 无默认体，会 E0046）。
//
// ★rust 偏差登记（owner: show.pptxr.handler.slides · wave: M-PPT-KMOSLIDE-W9）：
//   1) Java 三个内部类（`ControllerHandler` / `AlternateContentHandler` /
//      `AlternateContentHandler.FallBackHandler`）各自持有对外层 `ControlListHandler.this`
//      的**隐式外层引用**。Rust 无隐式外层引用，故提升为模块级结构体，并按 Java 的实际消费面
//      传递外层成员句柄：`shapeTree`（`Rc<RefCell<KmoShowShape>>`，与 Java 同一对象）、
//      `dgStore`（`*mut DgStore`，与同类既有约定一致 —— `CommonSlideDataHandler.dgStore`
//      同型）、`callback`（`Option<Rc<RefCell<C>>>`）。语义与 Java 等价：
//      `FallBackHandler` 里 `ControlListHandler.this.new ControllerHandler()` 只读取
//      `shapeTree` / `callback`（`dgStore` 在 Java 的 `ControllerHandler` 内亦未被使用，
//      仅由外层保存）。
//   2) `KmoShowShape` / `DgStore` 的 Java 类型分别是 `cn.wps.show.app.shape.KmoShowShape`
//      （Rust 真实端口 `show::app::shape::KmoShowShape`）与 `cn.wps.drawing.app.DgStore`
//      （Rust 真实端口 `drawing::app::DgStore`），二者均已登记，非替身。
//
// ★遗留阻塞（已实测，不在本波文件面内）：`ControllerHandler.enterSubElement(3211469)`
//   在 Java 里 `new SwPictureHandler(this.picture, ControlListHandler.this.callback)`。
//   真实 `show::pptxr::handler::shape::SwPictureHandler.rs` **未登记且当前不可登记**
//   （单独登记实测新增 30+ 个错误，依赖 `SwNvPrHandler` / `ShapeExtLstHandler::new` /
//   `ShapePropertiesHandler::new` 等尚未移植），前置里程碑 **M-PPT-SHAPE-DISPATCH**
//   （见 `shape/mod.rs` 实测表）。故该分支按 `todo!("DEFERRED_NON_FUNCTION")` 挂起并具名阻塞项。
//   可达性：本波实测默认构建与产品入口均**不触达**该分支 —— 产品 MVP 入口
//   `pptx_mvp::write_app_film` 只喂 `3211606`；`slides/SlideHandler.rs` 的 3211608
//   走 `LayoutCommonSlideData`（其 3211607 返回 None）；`slidelayouts/` / `slidemasters/`
//   的 `SlideLayoutHandler` / `SlideMasterHandler` 未登记。故不构成现存回归。

// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::DefaultElementHandler::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: Java `XmlRoAttr` 是**接口**，Rust 侧是 **trait** ⇒ 形参一律 `&dyn XmlRoAttr`。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import cn.wps.show.pptxr.handler.shape.SwPictureHandler
// rust: 该文件未登记且当前不可登记（见文件头遗留阻塞）⇒ 不导入，对应分支挂起。

use std::cell::{Cell, RefCell};
use std::rc::Rc;

// rust: 别名（Java 无）—— `Rc<RefCell<..>>` 承载 Java 的共享可变引用语义（repo 既有约定）。
type Shared<T> = Rc<RefCell<T>>;
// Java: HandlerCallback callback（`HandlerCallback` 是 trait ⇒ 泛型 + 可空句柄）。
type Callback<C> = Option<Shared<C>>;

// Java: class ControlListHandler extends DefaultElementHandler
pub struct ControlListHandler<C: HandlerCallback> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java field: KmoShowShape shapeTree
    pub(crate) shapeTree: Shared<KmoShowShape>,
    // Java field: HandlerCallback callback
    pub(crate) callback: Callback<C>,
    // Java field: DgStore dgStore
    pub(crate) dgStore: *mut DgStore,
}

// Java: class ControlListHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> ControlListHandler<C> {
    // Java: ControlListHandler(KmoShowShape shapeTree, DgStore dgStore, HandlerCallback callback)
    pub fn ControlListHandler(
        shapeTree: Shared<KmoShowShape>,
        dgStore: *mut DgStore,
        callback: Callback<C>,
    ) -> Self {
        // Java: this.shapeTree = shapeTree;
        // Java: this.callback = callback;
        // Java: this.dgStore = dgStore;
        // LLM JavaLine Aggregation: merged into struct init below
        ControlListHandler {
            parent: DefaultElementHandler::new(),
            shapeTree,
            callback,
            dgStore,
        }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 2162692:
            2162692 => {
                // Java: handler = new AlternateContentHandler();
                handler = Some(Box::new(AlternateContentHandler::new(
                    self.shapeTree.clone(),
                    self.dgStore,
                    self.callback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211472:
            3211472 => {
                // Java: handler = new ControllerHandler();
                handler = Some(Box::new(ControllerHandler::new(
                    self.shapeTree.clone(),
                    self.callback.clone(),
                )));
            }
            // Java: default:
            _ => {
                // Java: return handler;
                handler = None;
            }
        }
        // Java: return handler;
        handler
    }
}

// Java: class ControlListHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> ElementHandler for ControlListHandler<C> {
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public void endElement(int element)
    fn endElement(&self, _element: i32) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public void characters(String chars)
    fn characters(&self, _chars: &str) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        ControlListHandler::enterSubElement(self, subElement)
    }
}

// Java: (inner class) ControlListHandler.ControllerHandler
// Java: private class ControllerHandler extends DefaultElementHandler
pub struct ControllerHandler<C: HandlerCallback> {
    // Java field: KmoShowShape picture
    // rust: 仅在挂起的 3211469 分支被赋值（见文件头遗留阻塞）；保留字段以对齐 Java 结构。
    pub(crate) picture: RefCell<Option<Shared<KmoShowShape>>>,
    // Java field: int imageW
    // rust: `startElement` 取 `&self` ⇒ `Cell<i32>`（Java 的实例字段可变语义）。
    pub(crate) imageW: Cell<i32>,
    // Java field: int imageH
    pub(crate) imageH: Cell<i32>,
    // Java: ControlListHandler.this.shapeTree（外层引用，见文件头偏差登记 1）
    pub(crate) shapeTree: Shared<KmoShowShape>,
    // Java: ControlListHandler.this.callback（外层引用，见文件头偏差登记 1）
    pub(crate) callback: Callback<C>,
}

// Java: private class ControllerHandler extends DefaultElementHandler
impl<C: HandlerCallback> ControllerHandler<C> {
    // Java: public ControllerHandler()
    pub fn new(shapeTree: Shared<KmoShowShape>, callback: Callback<C>) -> Self {
        ControllerHandler {
            // Java: this.picture = null;（Java 引用字段默认 null，未显式初始化）
            picture: RefCell::new(None),
            // Java: this.imageW = 0;
            imageW: Cell::new(0),
            // Java: this.imageH = 0;
            imageH: Cell::new(0),
            shapeTree,
            callback,
        }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211469:
            3211469 => {
                // Java: this.picture = new KmoShowShape(ControlListHandler.this.shapeTree.parent(), 2);
                // Java: ControlListHandler.this.shapeTree.addChild(this.picture);
                // Java: handler = new SwPictureHandler(this.picture, ControlListHandler.this.callback);
                // Java: 该 case 无 break —— 故意贯穿到 `default: return handler`（返回 picture handler）。
                todo!("DEFERRED_NON_FUNCTION: ControllerHandler 的 p:pic(3211469) 分支需要真实 show::pptxr::handler::shape::SwPictureHandler，该文件当前不可登记（依赖 SwNvPrHandler / ShapeExtLstHandler::new / ShapePropertiesHandler::new 等尚未移植）；owner: show.pptxr.handler.slides · wave: M-PPT-KMOSLIDE-W9 · blocker: M-PPT-SHAPE-DISPATCH")
            }
            // Java: default:
            _ => {
                // Java: return handler;
                handler = None;
            }
        }
        handler
    }
}

// Java: private class ControllerHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> ElementHandler for ControllerHandler<C> {
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr temp = null;
        // Java: if ((temp = attrsNotCopy.getByName(3211464)) != null) { this.imageH = temp.toInt(); }
        if let Some(temp) = attrsNotCopy.get_by_name(3211464) {
            // Java: this.imageH = temp.toInt();
            self.imageH.set(temp.to_int());
        }
        // Java: if ((temp = attrsNotCopy.getByName(3211463)) != null) { this.imageW = temp.toInt(); }
        if let Some(temp) = attrsNotCopy.get_by_name(3211463) {
            // Java: this.imageW = temp.toInt();
            self.imageW.set(temp.to_int());
        }
    }

    // Java: public void endElement(int element)
    fn endElement(&self, _element: i32) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public void characters(String chars)
    fn characters(&self, _chars: &str) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        ControllerHandler::enterSubElement(self, subElement)
    }
}

// Java: (inner class) ControlListHandler.AlternateContentHandler
// Java: private class AlternateContentHandler extends DefaultElementHandler
pub struct AlternateContentHandler<C: HandlerCallback> {
    // Java: ControlListHandler.this.shapeTree（外层引用，见文件头偏差登记 1）
    pub(crate) shapeTree: Shared<KmoShowShape>,
    // Java: ControlListHandler.this.dgStore（外层引用，见文件头偏差登记 1）
    pub(crate) dgStore: *mut DgStore,
    // Java: ControlListHandler.this.callback（外层引用，见文件头偏差登记 1）
    pub(crate) callback: Callback<C>,
}

// Java: private class AlternateContentHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> AlternateContentHandler<C> {
    // Java: private AlternateContentHandler()
    pub fn new(
        shapeTree: Shared<KmoShowShape>,
        dgStore: *mut DgStore,
        callback: Callback<C>,
    ) -> Self {
        AlternateContentHandler {
            shapeTree,
            dgStore,
            callback,
        }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 2162695:
            2162695 => {
                // Java: handler = new FallBackHandler();
                handler = Some(Box::new(FallBackHandler::new(
                    self.shapeTree.clone(),
                    self.dgStore,
                    self.callback.clone(),
                )));
                // Java: 该 case 无 break —— 故意贯穿到 `case 2162693: default: return handler`
                //   （返回 FallBackHandler，而不是继续赋值）。
            }
            // Java: case 2162693:
            2162693 => {
                // Java: return handler;
                // rust: 2162693 不赋值 ⇒ 返回 null（也是 2162695 的贯穿落点）。
                handler = None;
            }
            // Java: default:
            _ => {
                // Java: return handler;
                handler = None;
            }
        }
        handler
    }
}

// Java: private class AlternateContentHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> ElementHandler for AlternateContentHandler<C> {
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public void endElement(int element)
    fn endElement(&self, _element: i32) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public void characters(String chars)
    fn characters(&self, _chars: &str) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        AlternateContentHandler::enterSubElement(self, subElement)
    }
}

// Java: (inner class) ControlListHandler.AlternateContentHandler.FallBackHandler
// Java: private class FallBackHandler extends DefaultElementHandler
pub struct FallBackHandler<C: HandlerCallback> {
    // Java: ControlListHandler.this.shapeTree（外层引用，见文件头偏差登记 1）
    pub(crate) shapeTree: Shared<KmoShowShape>,
    // Java: ControlListHandler.this.dgStore（外层引用，见文件头偏差登记 1）
    pub(crate) dgStore: *mut DgStore,
    // Java: ControlListHandler.this.callback（外层引用，见文件头偏差登记 1）
    pub(crate) callback: Callback<C>,
}

// Java: private class FallBackHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> FallBackHandler<C> {
    // Java: private FallBackHandler()
    pub fn new(
        shapeTree: Shared<KmoShowShape>,
        dgStore: *mut DgStore,
        callback: Callback<C>,
    ) -> Self {
        FallBackHandler {
            shapeTree,
            dgStore,
            callback,
        }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211472:
            3211472 => {
                // Java: handler = ControlListHandler.this.new ControllerHandler();
                handler = Some(Box::new(ControllerHandler::new(
                    self.shapeTree.clone(),
                    self.callback.clone(),
                )));
            }
            // Java: default:
            _ => {
                // Java: return handler;
                handler = None;
            }
        }
        handler
    }
}

// Java: private class FallBackHandler extends DefaultElementHandler
impl<C: HandlerCallback + 'static> ElementHandler for FallBackHandler<C> {
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public void endElement(int element)
    fn endElement(&self, _element: i32) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public void characters(String chars)
    fn characters(&self, _chars: &str) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        FallBackHandler::enterSubElement(self, subElement)
    }
}
