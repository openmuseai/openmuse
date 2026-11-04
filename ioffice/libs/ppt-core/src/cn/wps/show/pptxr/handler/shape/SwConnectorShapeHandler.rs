// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.data.shape.Shape
// rust: ★③-5 —— `ShapePr` 是 `data::shape` 下的**独立类**（不是 `Shape` 的内部类，
//   `Shape.rs` 的 `use super::ShapePr` 是**私有**再导入 ⇒ 不能经 `Shape::{…}` 取）；
//   `ShapeStyle` 的 Java 真实包是 `cn.wps.drawing.data.style`（**不是** `data.shape`）。
//   原先的 `data::shape::ShapeStyle` 是转写错误（`shape/mod.rs` 未登记该模块 ⇒ E0432）。
//   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::shape::CNvPr::CNvPr;
use crate::cn::wps::drawing::data::shape::Shape::Shape;
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvCxnSpPrHandler
use crate::cn::wps::drawing::dmlr::handler::shape::nvsppr::cnv::CNvCxnSpPrHandler::CNvCxnSpPrHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvPrHandler
use crate::cn::wps::drawing::dmlr::handler::shape::nvsppr::cnv::CNvPrHandler::CNvPrHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.ShapePropertiesHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::ShapePropertiesHandler::ShapePropertiesHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.style.ShapeStyleHandler
use crate::cn::wps::drawing::dmlr::handler::shape::style::ShapeStyleHandler::ShapeStyleHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: ★③-5 —— 原为不存在的 `crate::cn::wps::kfc::xpxml::…`；改接 ② 规范引擎契约。
//   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler as ElementHandlerContract;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import cn.wps.show.data.shape.ShowShape
use crate::cn::wps::show::data::shape::ShowShape::ShowShape;
// Java: (无对应 import) —— Java 侧 `protocol()` / `updateProtocol()` 由
//   `KmoShowShape implements cn.wps.drawing.app.KmoProtocol` 直接提供（`KmoShowShape.java`），
//   方法调用不需要 import。Rust 的 trait 方法必须**在作用域内**才可调用，故此处补入
//   真实实现的 trait 声明（`KmoShowShape.rs:247` 的 `impl KmoProtocol for KmoShowShape`）。
//   本 crate 内共有 4 个同名 trait：`drawing::app::KmoProtocol`（**空 trait**，只有声明）、
//   `app::table::KmoTableCell`、`app::shape::kmo_shape_tree_hosts`、以及本处
//   `app::shape::kmo_show_shape_hosts` —— 只有最后一个是 `KmoShowShape` 的实现面
//   （`protocol(&self) -> Box<dyn KoBin>` / `updateProtocol(&mut self, &dyn KoBin)`）。
//   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::show::app::shape::kmo_show_shape_hosts::KmoProtocol;
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;

use crate::cn::wps::show::pptxr::handler::shape::SwNvPrHandler::SwNvPrHandler;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use std::{cell::RefCell, rc::Rc};
type Shared<T> = Rc<RefCell<T>>;
type Callback<C> = Option<Shared<C>>;

// Java: public class SwConnectorShapeHandler extends DefaultElementHandler
pub struct SwConnectorShapeHandler<
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink>,
> {
    // Java field: DgStore dgStore
    //
    // rust: Java 侧是**可空引用**（`ISlide.parent()` 可返回 null ⇒ Java 的
    //   `NullPointerException` 条件）。本 port 按仓库既定模式（`SwShapeHandler.rs:72`、
    //   `SwGroupShapeHandler.rs:78`）用裸指针承载，`endElement` 以 `is_null()` 守卫。
    //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
    dgStore: *mut DgStore,
    // Java field: ShowShape swConnection
    pub(crate) swConnection: Shared<ShowShape>,
    // Java field: Shape dgConnection
    pub(crate) dgConnection: Shared<Shape>,
    // Java field: KmoShowShape kmoConnector
    kmoConnector: Shared<KmoShowShape>,
    // Java field: HandlerCallback callback
    pub(crate) callback: Callback<C>,
}

// Java: public class NonVisualConnectionShapePropertiesHandler
pub struct NonVisualConnectionShapePropertiesHandler<
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink>,
> {
    dgConnection: Shared<Shape>,
    swConnection: Shared<ShowShape>,
    callback: Callback<C>,
}

// Java: public class SwConnectorShapeHandler extends DefaultElementHandler
impl<C> SwConnectorShapeHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // Java: SwConnectorShapeHandler(KmoShowShape connectionShape, HandlerCallback callback)
    pub fn SwConnectorShapeHandler(
        connectionShape: Shared<KmoShowShape>,
        callback: Callback<C>,
    ) -> Self {
        // Java: this.dgStore = connectionShape.parent().getPresentation().drawingStore();
        //
        // rust: ★Java 的 `parent()` 返回 `ISlide`（可空引用，调用 `getPresentation()` 直接解引用）；
        //   本 port 的 `KmoShowShape::parent()` 返回 `Option<*const dyn ISlide>`
        //   （`KmoShowShape.rs:846`），故按仓库既定模式（`SwGroupShapeHandler.rs:102-105`、
        //   `SwShapeHandler.rs:104-107`）以 `match` + `unsafe` 解引用；`None` 落空指针，
        //   即 Java 侧的 `NullPointerException` 条件（本文件 `endElement` 对空指针做守卫）。
        //   **不得**改用 `DgStore::DgStore_new()` —— 那是伪造共享存储的反模式。
        //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        let dgStore = match connectionShape.borrow().parent() {
            Some(p) => unsafe { (*p).getPresentation().drawingStore() },
            None => std::ptr::null_mut(),
        };
        // Java: this.callback = callback;
        // rust: assigned in the aggregate below.
        // Java: this.init(connectionShape);
        let mut this = Self::from_connection(connectionShape, callback, dgStore);
        let current = this.kmoConnector.clone();
        this.init(current);
        this
    }
    fn from_connection(
        connection: Shared<KmoShowShape>,
        callback: Callback<C>,
        dgStore: *mut DgStore,
    ) -> Self {
        // rust: ★`Shape::open` / `ShowShape::open` 取 `&dyn KoBin`，而 `protocol()` /
        //   `drawingShape()` 返回 `Box<dyn KoBin>` ⇒ 必须先绑定到局部再 `&*local`
        //   （同 `SwShapeHandler.rs:121-122`）；顺带避免 `borrow()` 临时值在语句末被丢弃。
        //   `ShowShape::open` 返回 `ShowShape` 值，字段类型是**共享句柄** ⇒ `Rc::new(RefCell::new(..))`。
        //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        let protocol = connection.borrow().protocol();
        let swConnection = Rc::new(RefCell::new(ShowShape::open(&*protocol)));
        let drawing = connection.borrow().drawingShape();
        let dgConnection = Rc::new(RefCell::new(Shape::open(&*drawing)));
        Self {
            dgStore,
            swConnection,
            dgConnection,
            kmoConnector: connection,
            callback,
        }
    }

    // Java: void init(KmoShowShape connectionShape)
    pub fn init(&mut self, connectionShape: Shared<KmoShowShape>) {
        // Java: this.kmoConnector = connectionShape;
        self.kmoConnector = connectionShape;
        // Java: this.swConnection = ShowShape.open(this.kmoConnector.protocol());
        // rust: `protocol()` 借用 `kmoConnector`，`ShowShape::open` 取 `&dyn KoBin` ⇒
        //   先取 `Box<dyn KoBin>` 局部、再 `&*` 投影，避免临时 `Ref` 在语句末被丢弃。
        //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        let protocol = self.kmoConnector.borrow().protocol();
        self.swConnection = Rc::new(RefCell::new(ShowShape::open(&*protocol)));
        // Java: this.dgConnection = Shape.open(this.kmoConnector.drawingShape());
        let drawing = self.kmoConnector.borrow().drawingShape();
        self.dgConnection = Rc::new(RefCell::new(Shape::open(&*drawing)));
        // Java: this.dgConnection.putType(3);
        self.dgConnection.borrow_mut().putType(3);
        // Java: this.dgConnection.cNvPr().putId(this.kmoConnector.id());
        let id = self.kmoConnector.borrow().id();
        // rust: 真身 `Shape::cNvPr()` 取 `&self` 并返回 `&CNvPr`（只读），而 `CNvPr::putId`
        //   需 `&mut self`（`CNvPr.rs:472`）⇒ 直接 `borrow().cNvPr().putId(..)` 会 E0596。
        //   故按 `commit_drawing` 的惯用法把 `cNvPr` 槽位值**取出**、改完再放回；
        //   语义与 Java 的 `this.dgConnection.cNvPr().putId(..)` 相同：同一个对象被写入。
        //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        let mut dg = self.dgConnection.borrow_mut();
        let mut cnv_pr = match dg.cNvPr.take() {
            Some(v) => *v,
            None => CNvPr::create(),
        };
        cnv_pr.putId(id);
        let _ = dg.cNvPr.set(Box::new(cnv_pr));
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211295:
            // Java: default:
            3211295 => {
                // Java: break;
                // rust: no-stmt
                handler = None;
            }
            // Java: case 3211578:
            3211578 => {
                // Java: handler = new ShapePropertiesHandler(this.dgConnection.spPr(), this.callback);
                //
                // rust: 真身 `ShapePropertiesHandler` 已迁移为**共享句柄**：
                //   `ShapePropertiesHandler::ShapePropertiesHandler(Rc<RefCell<ShapePr>>)`
                //   （`ShapePropertiesHandler.rs:127`，`new` 别名**尚不存在**，故调 Java 同名构造器）。
                //   句柄由 `ShapePr_handle(..)` 交出（见文末该自由函数的登记）
                //   —— 与 Java 的引用语义一致：子处理器写入回到同一个 `ShapePr`。
                //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                let connector_id = self.kmoConnector.borrow().id();
                let dg = self.dgConnection.clone();
                handler = Some(Box::new(
                    ShapePropertiesHandler::ShapePropertiesHandler(ShapePr_handle(
                        self.dgStore,
                        connector_id,
                        &mut dg.borrow_mut(),
                    )),
                ));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211579:
            3211579 => {
                // Java: handler = new ShapeStyleHandler(this.dgConnection.style(), this.callback);
                //
                // rust: 真身 `ShapeStyleHandler` 的构造器是
                //   `ShapeStyleHandler(Rc<RefCell<ShapeStyle>>, Callback<C>)`（`ShapeStyleHandler.rs:56`），
                //   且**没有** `new` 别名。Java 的 `Shape::style()` 返回可空 `ShapeStyle`；
                //   本 port 的真身 `Shape.style` 是 `Option<Box<ShapeStyle>>`（`Shape.rs:43`，
                //   非共享、无 `#[derive(Clone)]`），无法别名成 `Rc<RefCell<_>>`，
                //   故按仓库既有的「非 Clone 取值」惯用法做 `save()`/`open()` 往返
                //   （同 `KmoShowShape.rs:2149` 等处的 `ShapeStyle::open(..protocol())`）。
                //   按 Java 的可空语义：`hasStyle()` 为真才建处理器，否则保持 `null`（⇒ `None`）。
                //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                let dg = self.dgConnection.clone();
                let mut shape = dg.borrow_mut();
                handler = if shape.hasStyle() {
                    // rust: `ShapeStyle` 无 `Clone` 且 `save()` 需 `&mut self`，故按
                    //   `commit_drawing` 的惯用法取出槽位值往返拷贝后再放回。
                    //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                    let mut filled = match shape.style.take() {
                        Some(v) => *v,
                        None => ShapeStyle::create(),
                    };
                    let saved = filled.save();
                    let style = Rc::new(RefCell::new(ShapeStyle::open(&*saved)));
                    let _ = shape.style.set(Box::new(filled));
                    Some(Box::new(ShapeStyleHandler::ShapeStyleHandler(
                        style,
                        self.callback.clone(),
                    )))
                } else {
                    None
                };
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211583:
            3211583 => {
                // Java: handler = new NonVisualConnectionShapePropertiesHandler();
                handler = Some(Box::new(NonVisualConnectionShapePropertiesHandler::NonVisualConnectionShapePropertiesHandler(self.dgConnection.clone(), self.swConnection.clone(), self.callback.clone())));
            }
            _ => handler = None,
        }
        // Java: return handler;
        handler
    }

    // Java: void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
        // Java: if (null != this.callback && this.callback.isGVml())
        if self.callback.as_ref().is_some_and(|c| c.borrow().isGVml()) {
            // Java: this.callback.onConvertStyle2ShapeProps(this.dgConnection.style(), this.dgConnection.spPr());
            //
            // rust: ①回调第一个形参是 `Option<&Self::ShapeStyle>`，而 `Shape::style()`
            //   （`Shape.rs:986`）返回 `&ShapeStyle`（其实现内部 `unwrap`，同 Java 的引用语义），
            //   故直接 `Some(&style)` 投影；`ShapeStyle` 无 `Clone`，以 `clone_style` 式的
            //   取值不可行，这里在 `shape` 借用期内就地取引用即可。
            //   ②回调的 `ShapePr` 关联类型必须是**真身**，故第二个实参从影子
            //   `Shape::spPr()`（返回值、类型不符）改走真身活共享句柄 `ShapePr_handle(..)`。
            //   两者与 Java 逐字对应：取 `style()`、取 `spPr()`、交给回调。
            //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
            let id = self.kmoConnector.borrow().id();
            let dg = self.dgConnection.clone();
            let mut shape = dg.borrow_mut();
            let sp_pr = ShapePr_handle(self.dgStore, id, &mut shape);
            if let Some(callback) = &self.callback {
                let style = shape.style();
                let mut sp_pr = sp_pr.borrow_mut();
                callback
                    .borrow_mut()
                    .onConvertStyle2ShapeProps(Some(style), Some(&mut *sp_pr));
            }
            drop(shape);
            // Java: this.dgConnection.deleteStyle();
            self.dgConnection.borrow_mut().deleteStyle();
        }
        // Java: this.dgStore.update(this.kmoConnector.id(), this.dgConnection.save());
        let id = self.kmoConnector.borrow().id();
        let drawing = self.dgConnection.borrow_mut().save();
        // rust: `DgStore` 经 `ISlide::getPresentation().drawingStore()` 交出的裸指针持有
        //   （与 `SwShapeHandler.rs:104-107` / `SwGroupShapeHandler.rs:102-105` 同模式）：
        //   先 `is_null()` 守卫再 `unsafe` 解引用；`DgStore::update(&mut self, i32, &dyn KoBin)`
        //   的第二个实参由 `Shape::save()` 的 `Box<dyn KoBin>` 以 `&*drawing` 投影。
        //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        if !self.dgStore.is_null() {
            unsafe {
                (*self.dgStore).update(id, &*drawing);
            }
        }
        // Java: this.kmoConnector.updateProtocol(this.swConnection.save());
        let protocol = self.swConnection.borrow_mut().save();
        self.kmoConnector.borrow_mut().updateProtocol(protocol.as_ref());
        // Java: this.callback.onConnectorShape(this.kmoConnector.id());
        self.callback
            .as_ref()
            .expect("Java contract requires callback at connector end")
            .borrow_mut()
            .onConnectorShape(Some(id));
    }
}

// rust: 下面的两个自由函数无对应 Java 成员 —— Java 的 `this.dgConnection.spPr()` /
//   `.cNvPr()` 返回的是**引用**，本 port 的真身 `cn.wps.drawing.data.shape.Shape` 却把
//   槽位建模成所有权值 `Option<Box<ShapePr>>` / `Option<Box<CNvPr>>`（`Shape.rs:41` / `:39`），
//   既非共享也无 `#[derive(Clone)]`，无法别名成 `Rc<RefCell<_>>`；而真身
//   `ShapePropertiesHandler` / `CNvPrHandler` 要求共享句柄。
//   本仓既定的「活句柄」机制是 `DgStore::shapePr(spid)`（`DgStore.rs:70`，
//   `spPrById: HashMap<i32, Rc<RefCell<ShapePr>>>`）—— 由 `SwShapeHandler::commit_data_sp_pr`
//   （`SwShapeHandler.rs:275`）与 `SwGroupShapeHandler::commit_drawing`（`:775`）在
//   `store.update(id, save())` 之后 `attachSpPr(id, handle)` 登记，正是 Java
//   `Shape.open(query(spid)).spPr()` 的 Rust 对应（见 `DgStore.rs:69` 的文档行）。
//   故此处按该机制取句柄：优先复用已登记的**同一个** `ShapePr`；未登记时新建并登记。
//   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH

// Java: Shape.spPr() —— 活共享句柄投影（见上注）。
fn ShapePr_handle(dg_store: *mut DgStore, spid: i32, dg: &mut Shape) -> Shared<ShapePr> {
    if dg_store.is_null() {
        // rust: Java 侧 `dgStore` 为 null 会 NPE；Rust 侧退化为不登记的临时句柄
        //   （`endElement` 对空指针有守卫），不掩盖调用方的空指针条件。
        return Rc::new(RefCell::new(ShapePr::create()));
    }
    let store = unsafe { &mut *dg_store };
    if let Some(pr) = store.shapePr(spid) {
        return pr;
    }
    // rust: 仓内尚无该 spid 的活句柄 —— 按 Java `Shape.spPr()` 的惰性创建语义
    //   （`Shape.rs:916`，槽位为空即以 `KoBin.Null` 解码出新对象）新建。
    //   `ShapePr` 无 `Clone` 且 `save()`/`open()` 需 `&mut self`，故按 `commit_drawing`
    //   的惯用法把槽位值**取出**、往返拷贝、再放回（槽位仍是原对象，Java 语义不变）。
    let mut filled = match dg.spPr.take() {
        Some(v) => *v,
        None => ShapePr::create(),
    };
    let saved = filled.save();
    let pr = Rc::new(RefCell::new(ShapePr::open(&*saved)));
    let _ = dg.spPr.set(Box::new(filled));
    store.attachSpPr(spid, pr.clone());
    pr
}

// Java: Shape.cNvPr() —— 共享句柄投影（见上注）。
fn CNvPr_handle(dg: &mut Shape) -> Shared<CNvPr> {
    // rust: 同 `ShapePr_handle` —— 真身 `CNvPr` 无 `Clone`，以取值往返拷贝。
    let mut filled = match dg.cNvPr.take() {
        Some(v) => *v,
        None => CNvPr::create(),
    };
    let saved = filled.save();
    let handle = Rc::new(RefCell::new(CNvPr::open(&*saved)));
    // rust: Java `cNvPr()` 的惰性槽位在调用后仍指向同一对象（本处放回原值）；
    //   `id` 已在 `init()` 写入，故该句柄读到的是同一份 cNvPr 内容。
    let _ = dg.cNvPr.set(Box::new(filled));
    handle
}
// Java: public class NonVisualConnectionShapePropertiesHandler
impl<C> NonVisualConnectionShapePropertiesHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // Java: NonVisualConnectionShapePropertiesHandler()
    pub fn NonVisualConnectionShapePropertiesHandler(
        dgConnection: Shared<Shape>,
        swConnection: Shared<ShowShape>,
        callback: Callback<C>,
    ) -> Self {
        Self {
            dgConnection,
            swConnection,
            callback,
        }
    }
    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211574:
            3211574 => {
                // Java: handler = new CNvPrHandler(SwConnectorShapeHandler.this.dgConnection.cNvPr(), SwConnectorShapeHandler.this.callback);
                //
                // rust: 真身 `CNvPrHandler::new(Shared<CNvPr>, Callback<C>)`
                //   （`CNvPrHandler.rs:102`，其 `Hyperlink` 槽落 `None`，同 Java 现状）。
                //   影子 `Shape::cNvPrHandle()` 返回**值** `CNvPr`（`sw_shape_hosts.rs:44`），
                //   类型不符且是快照；改包成共享句柄，锚定 Java 的同一 `cNvPr` 引用，
                //   使子处理器的 `putId` 等写入与 Java 一样落在连接形状的 cNvPr 上。
                //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                let dg = self.dgConnection.clone();
                let cnv_pr = CNvPr_handle(&mut dg.borrow_mut());
                handler = Some(Box::new(CNvPrHandler::new(cnv_pr, self.callback.clone())));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211576:
            3211576 => {
                // Java: handler = new SwNvPrHandler(SwConnectorShapeHandler.this.swConnection, SwConnectorShapeHandler.this.callback);
                //
                // rust: 真身 `SwNvPrHandler` 只提供 Java 同名构造器
                //   `SwNvPrHandler(Rc<RefCell<ShowShape>>, Option<Rc<RefCell<C>>>)`（`SwNvPrHandler.rs:58`），
                //   **没有** `new` 别名（该别名只存在于 `sw_shape_hosts` 影子）。
                //   owner: show.pptxr.handler.shape.SwConnectorShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(SwNvPrHandler::SwNvPrHandler(
                    self.swConnection.clone(),
                    self.callback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211582:
            3211582 => {
                // Java: handler = new CNvCxnSpPrHandler(SwConnectorShapeHandler.this.dgConnection);
                handler = Some(Box::new(CNvCxnSpPrHandler::new(self.dgConnection.clone())));
            }
            _ => handler = None,
        }
        // Java: return handler;
        handler
    }
}

impl<C> ElementHandlerContract for SwConnectorShapeHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    fn kind(&self) -> &'static str {
        "SwConnectorShapeHandler"
    }
    fn startElement(
        &mut self,
        _e: i32,
        _a: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn endElement(&mut self, e: i32) {
        SwConnectorShapeHandler::endElement(self, e)
    }
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        SwConnectorShapeHandler::enterSubElement(self, e)
    }
}
impl<C> ElementHandlerContract for NonVisualConnectionShapePropertiesHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    fn kind(&self) -> &'static str {
        "NonVisualConnectionShapePropertiesHandler"
    }
    fn startElement(
        &mut self,
        _e: i32,
        _a: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn endElement(&mut self, _e: i32) {}
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        NonVisualConnectionShapePropertiesHandler::enterSubElement(self, e)
    }
}
