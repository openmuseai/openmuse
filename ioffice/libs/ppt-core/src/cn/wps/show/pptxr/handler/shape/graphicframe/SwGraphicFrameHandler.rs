// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.data.shape.Shape
use crate::cn::wps::drawing::data::shape::Shape::Shape;
// Java: import cn.wps.drawing.data.shape.ShapePr
// rust: Java 侧 `ShapePr` 是同包**独立类** `cn.wps.drawing.data.shape.ShapePr`（不是 `Shape` 的内部
//   类）；`Shape.rs:31` 的 `use super::ShapePr` 是**私有**再导入，不能经 `Shape::{…}` 取。
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
// Java: import cn.wps.drawing.data.style.ShapeStyle
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
// Java: import cn.wps.drawing.data.basic.Transform2D
// rust: 本行无对应 Java **import** —— Java 只用 `this.dgGraphicFrame.spPr().xfrm()` 的返回值，
//   从不显式书写 `Transform2D` 类型名；Rust 侧因 `ShapePr.xfrm` 是 `Option<Box<Transform2D>>`
//   而必须显式命名它（见 `xfrmHandle` 字段注）。
// Java: (Java 类型引用) Transform2D
use crate::cn::wps::drawing::data::basic::Transform2D::Transform2D;
// Java: import cn.wps.drawing.data.shape.CNvPr
// rust: 同上 —— Java 只用 `this.dgGraphicFrame.cNvPr()` 的返回值；Rust 侧因 `Shape.cNvPr` 是
//   `Option<Box<CNvPr>>`（且 `Shape::cNvPr()` 的惰性解码分支尚未实现）而必须显式取用槽位。
// Java: (Java 类型引用) CNvPr
use crate::cn::wps::drawing::data::shape::CNvPr::CNvPr;
// Java: (Java 类型引用) Hyperlink
// rust: 本行无对应 Java import —— Java 的 `CNvPrHandler` 只按 `HandlerCallback` 接口取超链接；
//   Rust 侧真身 `CNvPrHandler<C>` 的 `impl` 前置 `C: HandlerCallback<Hyperlink = Hyperlink>`
//   绑定，故本类在构造 `CNvPrHandler` 的那些 `impl` 上必须一并钉住该关联类型。
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvGraphicFramePrHandler
use crate::cn::wps::drawing::dmlr::handler::shape::nvsppr::cnv::CNvGraphicFramePrHandler::CNvGraphicFramePrHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvPrHandler
use crate::cn::wps::drawing::dmlr::handler::shape::nvsppr::cnv::CNvPrHandler::CNvPrHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.transform.Transform2DHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::transform::Transform2DHandler::Transform2DHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.ShapePropertiesHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::ShapePropertiesHandler::ShapePropertiesHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.drawing.dmlr.util.DGMap
use crate::cn::wps::drawing::dmlr::util::DGMap::DGMap;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: no-stmt —— 父类 `DefaultElementHandler` 的 4 个方法体都是空的；本包已登记的处理器统一
//   直接实现 `ElementHandler`，本类与其内部类 `NonVisualGraphicFrameProperties` 同。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
// rust: 本类实现的契约是 **ppt-core 本地**的 `drawing::dmlr::handler::ElementHandler`
//   （`&mut self` + `Option<Box<dyn ElementHandler>>`），与 `SwPictureHandler` 同约定；
//   它也是 `UnifiedXmlReader::read__stream_elementhandler`（真 S2 引擎）唯一接受的契约。
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: 同契约的别名导入 —— `Box<dyn ElementHandler>` 与后续 `impl … for` 需要两个名字。
use crate::cn::wps::drawing::dmlr::handler::ElementHandler as ElementHandlerContract;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: 由 `XmlRoAttrList` 承载 —— 它是本包 `ElementHandler` 契约的 attrs 形参类型，
//   `getByName` / `toString` 与 Java `XmlRoAttr` 同名同义。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// rust: ★③-5 —— `KmoShowShape::{protocol,updateProtocol}` 是 `KmoProtocol` trait 的方法
//   （`KmoShowShape.rs:247-258`），Java 侧它们是本类成员故无需 import；Rust 侧必须显式引入该
//   trait 才能在 `Ref<KmoShowShape>` 上调用（原先缺此导入 ⇒ E0599，正是 `graphicframe/mod.rs`
//   登记的"不可登记"原因之一）。
//   owner: show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::show::app::shape::kmo_show_shape_hosts::KmoProtocol;
// Java: import cn.wps.show.data.shape.ShowShape
use crate::cn::wps::show::data::shape::ShowShape::ShowShape;
// Java: import cn.wps.show.pptxr.handler.shape.SwNvPrHandler
use crate::cn::wps::show::pptxr::handler::shape::SwNvPrHandler::SwNvPrHandler;

use std::cell::RefCell;
use std::rc::Rc;

type Shared<T> = Rc<RefCell<T>>;
type Callback<C> = Option<Shared<C>>;

// Java: public class SwGraphicFrameHandler extends DefaultElementHandler
//
// M-PPT-SHAPE-DISPATCH 新增：`p:graphicFrame`（3211595）的真身处理器，由 `SwGroupShapeHandler`
//   与 `SwShapeAlertnateContentHandler` 构造。Java 侧 90 行；本文件是完整移植。
//   唯一 `todo!` 在 `a:graphic`(1114259) 分支（下级 `SwGraphicHandler` 的依赖闭包在仓库内
//   不存在，见该分支注）。
//   owner: cn.wps.show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler
//   wave: M-PPT-SHAPE-DISPATCH
pub struct SwGraphicFrameHandler<C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink>> {
    // Java field: DgStore dgStore
    // rust: Java 的 `dgStore` 是**引用**（指向 `KmoPresentation` 持有的那一份仓）。
    //   ⚠ 本字段原先由 `unsafe { dgStore.read() }` 构造 `Shared<DgStore>` —— 把仓**按值拷出**
    //   并交给新 `Rc` 拥有；仓真身由产品侧 `Box<DgStore>` 拥有 ⇒ drop 时**二次释放**
    //   （实测 libmalloc `POINTER_BEING_FREED_WAS_NOT_ALLOCATED` ⇒ SIGABRT）。
    //   改为**非拥有**裸指针（与 `SwGroupShapeHandler.dgStore: *mut DgStore` 同一约定），
    //   使用点再借用真身，回到 Java 的共享引用语义。
    //   owner: show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler
    //   wave: M-PPT-SHAPE-DISPATCH
    dgStore: *mut DgStore,
    // Java field: KmoShowShape kmoGraphic
    kmoGraphic: Shared<KmoShowShape>,
    // Java field: ShowShape swGraphicFrame
    pub(crate) swGraphicFrame: Shared<ShowShape>,
    // Java field: Shape dgGraphicFrame
    pub(crate) dgGraphicFrame: Shared<Shape>,
    // Java field: HandlerCallback handlerCallback
    pub(crate) handlerCallback: Callback<C>,
    // rust: 本行无对应 Java 字段 —— Java 把 `this.dgGraphicFrame.spPr().xfrm()` 的**引用**直接
    //   交给 `Transform2DHandler`（`Transform2DHandler.java` 也是存引用）。Rust 侧
    //   `Transform2DHandler` 的字段是 `Rc<RefCell<Transform2D>>`，而 `ShapePr.xfrm` 是
    //   `Option<Box<Transform2D>>`：既没有共享句柄访问器，`Transform2D` 也没有 `Clone`
    //   ⇒ 无法把"ShapePr 内那个 Transform2D"的共享句柄交出去。折中：进入 `p:xfrm`(3211589)
    //   时把该槽位**取出**包成共享句柄交给真 `Transform2DHandler`，`endElement` 时再**放回**
    //   同一 `ShapePr.xfrm` 槽位 —— 落点仍是同一个 ShapePr 字段，语义与 Java 等价。
    //   模型层给出 `Rc<RefCell<Transform2D>>` 句柄后可删除该桥（登记：模型层共享句柄改造）。
    //   owner: ppt.drawing.data.shape    wave: M-PPT-SHAPE-DISPATCH
    xfrmHandle: Option<Rc<RefCell<Transform2D>>>,
    // rust: 本行无对应 Java 字段 —— 同 `xfrmHandle` 的桥：`p:spPr`(**3211578**，W6c-9 纠错)
    //   期间把 `Shape.spPr` 槽位取出包成共享句柄交给真 `ShapePropertiesHandler`，
    //   `endElement(3211578)` 再放回。模型层给出 `Rc<RefCell<ShapePr>>` 句柄后可删除该桥。
    //   owner: ppt.drawing.data.shape    wave: M-PPT-KMOSLIDE-W6c-5
    spPrHandle: Option<Shared<ShapePr>>,
}

// Java: (inner class) SwGraphicFrameHandler.NonVisualGraphicFrameProperties
// Java: private class NonVisualGraphicFrameProperties extends DefaultElementHandler
pub struct NonVisualGraphicFrameProperties<
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink>,
> {
    // rust: 无对应 Java 字段 —— Java 的内部类靠 `SwGraphicFrameHandler.this.*` 取外层字段；
    //   Rust 侧把外层句柄显式传入（同 `SwPictureHandler::NonVisualPicturePropertiesHandler`）。
    dgGraphicFrame: Shared<Shape>,
    swGraphicFrame: Shared<ShowShape>,
    handlerCallback: Callback<C>,
    // rust: 同 `SwGraphicFrameHandler::xfrmHandle` 的理由 —— `Shape.cNvPr` 是
    //   `Option<Box<CNvPr>>`，而真 `CNvPrHandler` 取 `Rc<RefCell<CNvPr>>` ⇒ 进入 `p:cNvPr`
    //   时取出、`endElement` 时放回同一槽位。
    //   owner: ppt.drawing.data.shape    wave: M-PPT-SHAPE-DISPATCH
    cnvPrHandle: Option<Rc<RefCell<CNvPr>>>,
}

// Java: public class SwGraphicFrameHandler extends DefaultElementHandler
impl<C> SwGraphicFrameHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // Java: SwGraphicFrameHandler(KmoShowShape graphicFrame, HandlerCallback handlerCallback)
    pub fn SwGraphicFrameHandler(
        graphicFrame: Shared<KmoShowShape>,
        handlerCallback: Callback<C>,
    ) -> Self {
        // Java: this.dgStore = graphicFrame.parent().getPresentation().drawingStore();
        // rust: ★③-5 —— `KmoShowShape::parent()` 返回 `Option<*const dyn ISlide>`（Java 是可直接
        //   调用的引用）。按本仓既有活路径写法解引用（同 `SwGroupShapeHandler.rs:102-105`、
        //   `SwShapeHandler.rs:104`、`SwConnectorShapeHandler.rs:79`、`SwPictureHandler.rs:78`）：
        //   经 `ISlide::getPresentation().drawingStore()` 取**真实泄漏的单例仓**。
        //   `None` ⇒ Java 会 NPE；此处按本仓 active-path 约定给 null 指针（下游不经它写入）。
        //   owner: show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler
        //   wave: M-PPT-SHAPE-DISPATCH
        let dgStore = match graphicFrame.borrow().parent() {
            Some(p) => unsafe { (*p).getPresentation().drawingStore() },
            None => std::ptr::null_mut(),
        };
        // Java: this.handlerCallback = handlerCallback;
        // rust: assigned in the aggregate below.
        // Java: this.init(graphicFrame);
        // rust: Java `ShowShape.open(graphicFrame.protocol())` —— `protocol()` 是 `KmoProtocol`
        //   trait 的方法（`KmoShowShape.rs:247-252`），须先 `use` 该 trait；
        //   `Shape.open(graphicFrame.drawingShape())` 的 `drawingShape()` 返回 `Box<dyn KoBin>`
        //   而 `Shape::open` 取 `&dyn KoBin` ⇒ 先绑定到局部再 `&*`（避免临时值生命周期错误）。
        let sw_protocol = graphicFrame.borrow().protocol();
        let sw_graphic_frame = ShowShape::open(&*sw_protocol);
        let dg_shape_bin = graphicFrame.borrow().drawingShape();
        let dg_graphic_frame = Shape::open(&*dg_shape_bin);
        let mut this = Self {
            // rust: `DgStore` 句柄 —— 与 `init()` 使用同一份仓（Java `this.dgStore = …drawingStore()`）。
            // rust: 保存**非拥有**仓指针（Java 引用语义），不再 `read()` 副本。
            //   owner: show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler
            //   wave: M-PPT-SHAPE-DISPATCH
            dgStore,
            kmoGraphic: graphicFrame.clone(),
            swGraphicFrame: Rc::new(RefCell::new(sw_graphic_frame)),
            dgGraphicFrame: Rc::new(RefCell::new(dg_graphic_frame)),
            handlerCallback,
            xfrmHandle: None,
            spPrHandle: None,
        };
        let current = this.kmoGraphic.clone();
        this.init(current);
        this
    }

    // rust: 本方法无对应 Java 方法之外的构造名 —— 调用点按 Java `new SwGraphicFrameHandler(...)`
    //   的写法使用；形参、语义与上面完全一致。
    pub fn new(graphicFrame: Shared<KmoShowShape>, handlerCallback: Callback<C>) -> Self {
        Self::SwGraphicFrameHandler(graphicFrame, handlerCallback)
    }

    // Java: public void init(KmoShowShape graphicFrame)
    pub fn init(&mut self, graphicFrame: Shared<KmoShowShape>) {
        // Java: this.kmoGraphic = graphicFrame;
        self.kmoGraphic = graphicFrame.clone();
        // Java: this.swGraphicFrame = ShowShape.open(graphicFrame.protocol());
        // rust: `protocol()` 来自 `KmoProtocol` trait；返回 `Box<dyn KoBin>`，`ShowShape::open`
        //   取 `&dyn KoBin` ⇒ 绑定局部后 `&*`（同构造器）。
        let sw_protocol = graphicFrame.borrow().protocol();
        self.swGraphicFrame = Rc::new(RefCell::new(ShowShape::open(&*sw_protocol)));
        // Java: this.dgGraphicFrame = Shape.open(graphicFrame.drawingShape());
        let dg_shape_bin = graphicFrame.borrow().drawingShape();
        self.dgGraphicFrame = Rc::new(RefCell::new(Shape::open(&*dg_shape_bin)));
        // Java: this.dgGraphicFrame.putType(4);
        self.dgGraphicFrame.borrow_mut().putType(4);
        // Java: this.dgGraphicFrame.cNvPr().putId(graphicFrame.id());
        //
        // rust: `Shape::cNvPr()` 的惰性解码分支（`Shape.rs:842-866` 的三条 `KoBin.Decode.toAny`）
        //   在 Rust 侧仍是空壳，槽位为 None 时其末尾 `unwrap()` 会 panic ⇒ 这里按 Java
        //   `cNvPr()` 的**语义**就地取用/创建槽位，而不是调用会 panic 的访问器。
        let dg_id = graphicFrame.borrow().id();
        let mut dg = self.dgGraphicFrame.borrow_mut();
        if dg.cNvPr.get().is_none() {
            let _ = dg.cNvPr.set(Box::new(CNvPr::create()));
        }
        if let Some(cnv_pr) = dg.cNvPr.get_mut() {
            cnv_pr.putId(dg_id);
        }
    }

    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp = null;
        // Java: if ((temp = attrsNotCopy.getByName(3211605)) != null)
        //   3211605 = `p:graphicFrame@bwMode`（黑白模式）
        if let Some(temp) = attrsNotCopy.get_by_name(3211605) {
            // Java: Integer blackWhiteMode = (Integer)DGMap.blackWhiteModeMap.get(temp.toString());
            let blackWhiteMode = DGMap::blackWhiteModeMap().get(temp.to_string().as_str()).copied();
            // Java: if (blackWhiteMode != null)
            if let Some(blackWhiteMode) = blackWhiteMode {
                // Java: this.swGraphicFrame.putBwMode(blackWhiteMode);
                self.swGraphicFrame.borrow_mut().putBwMode(blackWhiteMode);
            }
        }
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114259:
            //   1114259 = `a:graphic`（图文框内容：表格 / 图表 / 图示 / 3D / OLE）
            1114259 => {
                // Java: handler = new SwGraphicHandler(this.dgGraphicFrame, this.kmoGraphic.parent(),
                //                                    this.kmoGraphic, this.handlerCallback);
                //
                // rust: ★DEFERRED_NON_FUNCTION —— `SwGraphicHandler`（Java 166 行）的下级依赖
                //   闭包在本仓库内**整体不存在**：`KmoOleChart` / `KmoTable` / `TableStyle` /
                //   `ChartHandler` / `DgmRelIdsHandler` / `TableHanlder` / `KmoDiagram` /
                //   `KmoOleObject` / `KmoTableStyleList` / `LockedCanvasHandler` 所对应的目录
                //   `drawing/dmlr/handler/{table,chart,diagram}`、`show/app/diagram`、
                //   `show/clipxr/gvml/handler` 均不存在。按 PLAN §16「非管线核心不阻塞管线推进」
                //   规则延后，不影响图形框自身的 spPr / xfrm / cNvPr / nvPr / 文本解析。
                // owner: cn.wps.show.pptxr.handler.shape.graphicframe.SwGraphicHandler
                // wave: M-PPT-SHAPE-DISPATCH
                todo!("DEFERRED_NON_FUNCTION: SwGraphicHandler (a:graphic table/chart/diagram/model3d/ole dispatch)");
            }
            // Java: case 3211578:
            //   ★W6c-9 —— **元素号纠错**：`p:spPr` 的 Java 元素号是 **3211578**
            //   （实测 Java 原文：`SwShapeHandler.java:66` / `SwConnectorShapeHandler.java:42` /
            //   `SwPictureHandler.java:47` 三处均为 `case 3211578:`）。
            //   本分支此前写作 `3211587`，而 `3211587` 在 Java 中是
            //   `p:cNvGraphicFramePr`（`SwGraphicFrameHandler.java:83`，属**内部类**
            //   `NonVisualGraphicFrameProperties` 的 case，本文件 `:458` 已正确实现）。
            //   ⇒ 旧编号使本分支**永不可达**（外层永远收不到 3211587 的 `p:spPr`），
            //   图文框的 `p:spPr` 子树被静默丢弃 —— 这正是门禁 ⑰.3 长期只报 1/4 的原因之一。
            //   ★另注：Java 的 `SwGraphicFrameHandler.enterSubElement` **本身没有** `p:spPr`
            //   分支（只有 1114259/3211588/3211589）；图文框的 `spPr` 在 Java 里由
            //   `p:xfrm`(3211589) 走 `this.dgGraphicFrame.spPr().xfrm()` 命中同一个 `ShapePr`。
            //   本分支是本 port 为让填充/线/几何也能落到 `spPr` 而补的**产品等价分支**，
            //   故必须用与另三类一致的 3211578，保持"同一个 `spPr` 元素号"。
            //   owner: show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler
            //   wave: M-PPT-KMOSLIDE-W6c-9
            3211578 => {
                // Java: handler = new ShapePropertiesHandler(this.dgGraphicFrame.spPr(), this.handlerCallback);
                //
                // rust: ★W6c-5 —— 本分支此前**缺失**（`p:spPr` 整棵子树被静默丢弃，
                //   `SwGraphicFrameHandler` 是 §17.3 统计里"4 类子形状中唯一没有 spPr 落点"的一类）。
                //   真身 `ShapePropertiesHandler` 已迁移为**共享句柄**且只取**一个**形参
                //   （`ShapePropertiesHandler.rs`，`new` 别名不存在 ⇒ 调 Java 同名构造器），
                //   而 `Shape.spPr` 是 `OnceCell<Box<ShapePr>>`：`spPr()` 取 `&ShapePr` 无法别名。
                //   ⇒ 与 `xfrmHandle` 同法：把该槽位**取出**包成共享句柄交给真 handler，
                //   `endElement(3211578)` 时再**放回**同一 `ShapePr` 槽位 —— 落点仍是同一个字段。
                //   owner: show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler
                //   wave: M-PPT-KMOSLIDE-W6c-5
                let handle = {
                    let mut shape = self.dgGraphicFrame.borrow_mut();
                    if shape.spPr.get().is_none() {
                        let _ = shape.spPr.set(Box::new(ShapePr::create()));
                    }
                    // Java: this.dgGraphicFrame.spPr()（取得该字段的引用）
                    // rust: `OnceCell<Box<ShapePr>>` 无 `take` ⇒ 先 `mem::replace` 取出真身，
                    //   再以同值放回一个占位（`endElement` 会用 handler 内的真身覆盖）。
                    let taken = std::mem::replace(
                        shape.spPr.get_mut().expect("spPr just created"),
                        Box::new(ShapePr::create()),
                    );
                    Rc::new(RefCell::new(*taken))
                };
                self.spPrHandle = Some(handle.clone());
                Some(Box::new(ShapePropertiesHandler::ShapePropertiesHandler(
                    handle,
                )))
            }
            // Java: case 3211588:
            //   3211588 = `p:nvGraphicFramePr`
            3211588 => {
                // Java: handler = new NonVisualGraphicFrameProperties();
                Some(Box::new(
                    NonVisualGraphicFrameProperties::NonVisualGraphicFrameProperties(
                        self.dgGraphicFrame.clone(),
                        self.swGraphicFrame.clone(),
                        self.handlerCallback.clone(),
                    ),
                ))
            }
            // Java: case 3211589:
            //   3211589 = `p:xfrm`（图文框位置与尺寸）
            3211589 => {
                // Java: handler = new Transform2DHandler(this.dgGraphicFrame.spPr().xfrm());
                //
                // rust: 见 `xfrmHandle` 字段注 —— 把 `ShapePr.xfrm` 槽位取出成共享句柄。
                let handle = {
                    let mut shape = self.dgGraphicFrame.borrow_mut();
                    if shape.spPr.get().is_none() {
                        let _ = shape.spPr.set(Box::new(ShapePr::create()));
                    }
                    let shape_pr = shape.spPr.get_mut().expect("spPr just created");
                    if shape_pr.xfrm.is_none() {
                        shape_pr.xfrm = Some(Box::new(Transform2D::new()));
                    }
                    // Java: this.dgGraphicFrame.spPr().xfrm()（取得该字段的引用）
                    let taken = *shape_pr.xfrm.take().expect("xfrm just created");
                    Rc::new(RefCell::new(taken))
                };
                self.xfrmHandle = Some(handle.clone());
                Some(Box::new(Transform2DHandler::Transform2DHandler(handle)))
            }
            // Java: default: return handler;
            _ => None,
        }
    }

    // Java: public void endElement(int element)
    pub fn endElement(&mut self, element: i32) {
        // rust: 见 `spPrHandle` 字段注 —— `p:spPr`(3211578) 闭合时把 handler 内的真身放回
        //   `Shape.spPr` 槽位（Java 侧从头到尾就是那一个对象，故 Java 无此语句）。
        //   owner: show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler
        //   wave: M-PPT-KMOSLIDE-W6c-5
        if element == 3211578 {
            if let Some(handle) = self.spPrHandle.take() {
                let value = std::mem::replace(&mut *handle.borrow_mut(), ShapePr::create());
                let mut shape = self.dgGraphicFrame.borrow_mut();
                if shape.spPr.get().is_none() {
                    let _ = shape.spPr.set(Box::new(ShapePr::create()));
                }
                *shape.spPr.get_mut().expect("spPr just created") = Box::new(value);
            }
        }
        // rust: 见 `xfrmHandle` 字段注 —— 把 `p:xfrm` 期间取出解析的 Transform2D 放回
        //   `ShapePr.xfrm`（Java 侧从头到尾就是那一个对象，故 Java 无此语句）。
        if let Some(handle) = self.xfrmHandle.take() {
            // 用 `mem::replace` 而非 `Rc::try_unwrap`：`Transform2DHandler` 的 Box 此时已被引擎
            // 释放，但 `PositiveSize2DHandler`/`Point2DHandler` 可能仍持有 clone，`try_unwrap`
            // 会失败并丢值。
            let value = {
                let mut cell = handle.borrow_mut();
                std::mem::replace(&mut *cell, Transform2D::new())
            };
            let mut shape = self.dgGraphicFrame.borrow_mut();
            if let Some(shape_pr) = shape.spPr.get_mut() {
                shape_pr.xfrm = Some(Box::new(value));
            }
        }
        // Java: this.dgStore.update(this.kmoGraphic.id(), this.dgGraphicFrame.save());
        let id = self.kmoGraphic.borrow().id();
        let drawing = self.dgGraphicFrame.borrow_mut().save();
        // rust: 经**非拥有**裸指针借用产品侧真身仓（Java `this.dgStore.update(..)`）。
        //   owner: show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler
        //   wave: M-PPT-SHAPE-DISPATCH
        if !self.dgStore.is_null() {
            unsafe { (*self.dgStore).update(id, drawing.as_ref()) };
        }
        // Java: this.kmoGraphic.updateProtocol(this.swGraphicFrame.save());
        let protocol = self.swGraphicFrame.borrow_mut().save();
        self.kmoGraphic.borrow_mut().updateProtocol(protocol.as_ref());
    }
}

/// Java: `SwGraphicFrameHandler implements ElementHandler`（Java 侧经 `DefaultElementHandler` 继承）。
///
/// rust: 本类**真实存在**且由真 S2 引擎驱动（`UnifiedXmlReader::read__stream_elementhandler` 取
///   `&mut dyn dmlr::handler::ElementHandler`），故必须提供契约的全部方法；`kind()` 返回 Java 类名。
impl<C> ElementHandlerContract for SwGraphicFrameHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    fn kind(&self) -> &'static str {
        "SwGraphicFrameHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        SwGraphicFrameHandler::startElement(self, element, attrs);
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        SwGraphicFrameHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, element: i32) {
        SwGraphicFrameHandler::endElement(self, element);
    }
}

// Java: public class NonVisualGraphicFrameProperties extends DefaultElementHandler
impl<C> NonVisualGraphicFrameProperties<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // Java: private NonVisualGraphicFrameProperties()   —— 无参私有构造器
    //
    // rust: 形参是"外层 host"的显式承载，理由见字段注；Java 的无参构造器本身无语句。
    pub fn NonVisualGraphicFrameProperties(
        dgGraphicFrame: Shared<Shape>,
        swGraphicFrame: Shared<ShowShape>,
        handlerCallback: Callback<C>,
    ) -> Self {
        NonVisualGraphicFrameProperties {
            dgGraphicFrame,
            swGraphicFrame,
            handlerCallback,
            cnvPrHandle: None,
        }
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211574:
            //   3211574 = `p:cNvPr`
            3211574 => {
                // Java: handler = new CNvPrHandler(SwGraphicFrameHandler.this.dgGraphicFrame.cNvPr(),
                //                                  SwGraphicFrameHandler.this.handlerCallback);
                //
                // rust: `Shape::cNvPr()` 会 panic（见 `init` 注），故就地取槽位；`CNvPrHandler`
                //   需要 `Rc<RefCell<CNvPr>>` 共享句柄，而 `Shape.cNvPr` 是 `Option<Box<CNvPr>>`
                //   ⇒ 与 `xfrmHandle` 同法：取出 → 交给真 handler → `endElement` 放回。
                let handle = {
                    let mut shape = self.dgGraphicFrame.borrow_mut();
                    if shape.cNvPr.get().is_none() {
                        let _ = shape.cNvPr.set(Box::new(CNvPr::create()));
                    }
                    let taken = *shape.cNvPr.take().expect("cNvPr just created");
                    Rc::new(RefCell::new(taken))
                };
                self.cnvPrHandle = Some(handle.clone());
                Some(Box::new(CNvPrHandler::<C>::new(
                    handle,
                    self.handlerCallback.clone(),
                )))
            }
            // Java: case 3211576:
            //   3211576 = `p:nvPr`
            3211576 => {
                // Java: handler = new SwNvPrHandler(SwGraphicFrameHandler.this.swGraphicFrame,
                //                                   SwGraphicFrameHandler.this.handlerCallback);
                Some(Box::new(SwNvPrHandler::<C>::SwNvPrHandler(
                    self.swGraphicFrame.clone(),
                    self.handlerCallback.clone(),
                )))
            }
            // Java: case 3211587:
            //   3211587 = `p:cNvGraphicFramePr`
            3211587 => {
                // Java: handler = new CNvGraphicFramePrHandler(SwGraphicFrameHandler.this.dgGraphicFrame);
                Some(Box::new(
                    CNvGraphicFramePrHandler::CNvGraphicFramePrHandler(
                        self.dgGraphicFrame.clone(),
                    ),
                ))
            }
            // Java: default: return handler;
            _ => None,
        }
    }

    // Java: public void endElement(int element)   —— 内部类未覆写 ⇒ 继承空实现
    pub fn endElement(&mut self, _element: i32) {
        // rust: 把 `p:cNvPr` 期间取出解析的 CNvPr 放回 `Shape.cNvPr`（Java 侧是同一个对象）。
        if let Some(handle) = self.cnvPrHandle.take() {
            let value = {
                let mut cell = handle.borrow_mut();
                std::mem::replace(&mut *cell, CNvPr::create())
            };
            let mut shape = self.dgGraphicFrame.borrow_mut();
            let _ = shape.cNvPr.set(Box::new(value));
        }
    }
}

impl<C> ElementHandlerContract for NonVisualGraphicFrameProperties<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    fn kind(&self) -> &'static str {
        "NonVisualGraphicFrameProperties"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        NonVisualGraphicFrameProperties::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, element: i32) {
        NonVisualGraphicFrameProperties::endElement(self, element);
    }
}
