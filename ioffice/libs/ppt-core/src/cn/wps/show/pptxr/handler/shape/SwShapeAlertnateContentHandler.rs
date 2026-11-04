// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.data.basic.RelativeRect
use crate::cn::wps::drawing::data::basic::RelativeRect::RelativeRect;
// Java: import cn.wps.drawing.data.basic.Transform2D
use crate::cn::wps::drawing::data::basic::Transform2D::Transform2D;
// Java: import cn.wps.drawing.data.fill.BlipFill
use crate::cn::wps::drawing::data::fill::BlipFill::BlipFill;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.drawing.data.shape.Shape
use crate::cn::wps::drawing::data::shape::Shape::Shape;
// Java: import cn.wps.drawing.data.shape.ShapePr
// rust: 真身关联类型绑定所需的类型名 —— Java 侧 `HandlerCallback` 是**裸**接口（无泛型），
//   `ShapePr` 由 `SwPictureHandler` 等方法签名隐式满足；Rust 侧 `HandlerCallback` 把
//   `ShapePr`/`ShapeStyle`/`Hyperlink` 提为**关联类型**，构造 `SwPictureHandler` /
//   `SwShapeHandler` / `SwGraphicFrameHandler` 等子处理器时必须把这些槽位固定到真身
//   （与 `SwPictureHandler.rs:14`、`SwShapeHandler.rs`、`SwConnectorShapeHandler.rs:41` 同面）。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
// Java: import cn.wps.drawing.data.style.ShapeStyle
// rust: 同上 —— Java 真实包是 `cn.wps.drawing.data.style.ShapeStyle`。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
// Java: import cn.wps.drawing.data.hyperlink.Hyperlink
// rust: 同上 —— `SwGraphicFrameHandler` / `SwPictureHandler` 的 `C` 还固定 `Hyperlink` 槽位
//   （`SwPictureHandler.rs:72`、`SwGraphicFrameHandler.rs:121`），故此处同样绑定。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.transform.Transform2DHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::transform::Transform2DHandler::Transform2DHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: ★③-5 修正 —— `crate::cn::wps::kfc::xpxml` 在本 crate **不存在**（`cn/wps/kfc/` 下只有
//   `ooxfr`；`xpxml` 在独立 crate `kfc_kfc_alpha`）。Java 侧 `DefaultElementHandler` 的四个
//   方法体皆空，本包的处理器统一**直接实现** `ElementHandler`（与 `SwPictureHandler` /
//   `SwGraphicFrameHandler` 同约定），故不导入基类。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
// rust: ★③-5 —— 本类实现的契约是 **② 规范引擎契约**
//   `drawing::dmlr::handler::ElementHandler`（`&mut self` + `Option<Box<dyn ElementHandler>>`），
//   即 `UnifiedXmlReader::read__stream_elementhandler`（真 S2 引擎）唯一接受的契约（PLAN §26.1）。
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: 同契约的别名导入 —— `Box<dyn ElementHandler>` 与后续 `impl … for` 需要两个名字。
use crate::cn::wps::drawing::dmlr::handler::ElementHandler as ElementHandlerContract;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: 由 `XmlRoAttrList` 承载 —— 它是 ② 契约的 attrs 形参类型，
//   `getByName` / `toString` 与 Java `XmlRoAttr` 同名同义。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// rust: `XmlRoAttrList::get_by_name` 返回的叶子类型 —— Java 侧 `getByName` 返回 `XmlRoAttr`
//   本身；本 port 把它拆成「列表 + 叶子」，故 `XmlRoAttr` 的局部需按叶子命名。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrLeaf::XmlRoAttrLeaf;
// rust: `KmoShowShape::parent()` 解引用所需的 trait 名（`*const dyn ISlide` ⇒ `&dyn ISlide`）。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::show::app::slide::ISlide::ISlide;
// rust: 本行无对应 Java import —— 真 `BlipFill` 无 `Clone`，复制按 KProto 往返
//   （`lastBin` + `BlipFill::open`）实现，需要 `KoZone` 作用域句柄
//   （既有落法同 `ShapePr.rs:32/75`）。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::kso::kproto::KoZone::KoZone;
// rust: 本行无对应 Java import —— 真 `Shape` 的 `children()` 是**只读**返回
//   （`Shape.rs:1510`，`&KoIntList`），无 `children_mut`；写入走 `Shape::writeChildren`
//   （`Shape.rs:1543`）。故按 Java `dgGroup.children().add(id)` 的效果，用一个本文件私有的
//   转发函数「读旧表 → 复制到新表 → 追加 → writeChildren 写回」。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::kso::kproto::util::KoIntList::KoIntList;
// rust: 本行无对应 Java import —— 真 `BlipFill::fillMode()` 返回 `&mut dyn KoBinTrait`
//   （KProto 子对象），Java 侧静态类型为 `FillMode`；判型/下发需真身类型名。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
// Java: import cn.wps.show.app.model3d.KmoModel3D
// rust: ★③-5 修正 —— `show/app/model3d/` 目录下只有 `KmoModel3D.rs`，但它既**无 `mod.rs`**
//   （形态 H，整目录未进编译单元）又本身是 form-E 自动生成桩（`use crate::*;` + 10 个 `todo!()`）。
//   本文件对它的唯一用法在 `OleObjectHandler` 的 `onModel3D` 回调分支（Java
//   `handlerCallback.onModel3D(rId, model3D)`），属**非管线核心**（`a:graphic` 内层 3D 派发，
//   PLAN §16 规则）。故按规则以 `DEFERRED_NON_FUNCTION` 处理该分支，**不导入**该桩类型
//   （导入未登记目录等于往编译面里加桩，PLAN §22.1 明确不做）。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// rust: 本行无对应 Java import —— Java 侧 `protocol()` / `updateProtocol()` 是 `KmoShowShape`
//   自身的成员方法；Rust 侧二者定义在 `show::app::shape::kmo_show_shape_hosts::KmoProtocol`
//   trait 上（`KmoShowShape.rs:247-253`），故须显式 `use` 才能在本文件调用（否则 E0599）。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::show::app::shape::kmo_show_shape_hosts::KmoProtocol;
// Java: import cn.wps.show.data.shape.ShowShape
use crate::cn::wps::show::data::shape::ShowShape::ShowShape;
// Java: import cn.wps.show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler
// rust: ★③-5 —— 真身 `SwGraphicFrameHandler` 已完整移植（399 行）并登记于
//   `graphicframe/mod.rs`；此处按 Java 真实包路径导入真身（原先 `graphicframe` 的 `mod.rs`
//   是 0 字节 ⇒ E0432）。
use crate::cn::wps::show::pptxr::handler::shape::graphicframe::SwGraphicFrameHandler::SwGraphicFrameHandler;
// Java: import java.util.ArrayList
// rust: Vec preserves Java ArrayList insertion order.

use crate::cn::wps::show::pptxr::handler::shape::SwConnectorShapeHandler::SwConnectorShapeHandler;
use crate::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::SwGroupShapeHandler;
use crate::cn::wps::show::pptxr::handler::shape::SwPictureHandler::SwPictureHandler;
use crate::cn::wps::show::pptxr::handler::shape::SwShapeHandler::SwShapeHandler;
// Java: import cn.wps.util.manager.FileManager
// rust: ★③-5 修正 —— 真实 Rust 移植在 **独立 crate `core_libs_util`**
//   （`cn.wps.util.manager.FileManager`，Java 同包同名），本 crate 无 `cn::wps::util` 路径。
//   原先的 `crate::cn::wps::util::manager::FileManager` 是转写错误（E0433）；
//   改用真身是**真实接线**，不是编译面替身。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
use util::cn::wps::util::manager::FileManager::FileManager;
use std::{cell::RefCell, rc::Rc};
type Shared<T> = Rc<RefCell<T>>;
type Callback<C> = Option<Shared<C>>;

// rust: 本函数无对应 Java 方法 —— Java 侧 `dgGroup.children()` 返回**可变**的 `KoIntList`
//   引用，`add(id)` 就地追加；本 port 的真 `Shape::children()` 只读（`Shape.rs:1510`），
//   写入通道是 `writeChildren`（`Shape.rs:1543`）。
//   故按"读旧表 → 复制 → 追加 id → 写回"实现 Java 的同一效果（表内容与顺序一致，
//   且 `writeChildren` 会置脏，后续 `save()` 会落盘，语义等价）。
//   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
fn dg_children_add(group: &mut Shape, id: i32) {
    let old = group.children();
    let mut next = KoIntList::new();
    for i in 0..old.size() {
        next.add(old.get(i));
    }
    next.add(id);
    group.writeChildren(&next);
}

pub struct AlternateState<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
>> {
    // Java field: KmoShowShape swParent
    swParent: Shared<KmoShowShape>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Callback<C>,
    // Java field: DgStore dgStore
    dgStore: Shared<DgStore>,
    // Java field: Shape dgGroup
    dgGroup: Shared<Shape>,
    // Java field: KmoShowShape shape
    shape: Option<Shared<KmoShowShape>>,
    // Java field: String rId
    rId: Option<String>,
    // Java field: Transform2D transform2d
    transform2d: Option<Shared<Transform2D>>,
    // Java field: KmoShowShape choiceFrameShape
    choiceFrameShape: Option<Shared<KmoShowShape>>,
    // Java field: SwGraphicFrameHandler graphicFrameHandler
    graphicFrameHandler: Option<Shared<SwGraphicFrameHandler<C>>>,
    // Java field: ArrayList<KmoShowShape> newSps
    newSps: Option<Shared<Vec<Shared<KmoShowShape>>>>,
}

// Java: public class SwShapeAlertnateContentHandler extends DefaultElementHandler
pub struct SwShapeAlertnateContentHandler<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
>> {
    state: Shared<AlternateState<C>>,
}
// Java: public class ChoiceHandler
pub struct ChoiceHandler<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
>> {
    state: Shared<AlternateState<C>>,
}
// Java: public class ContentPartHandler
pub struct ContentPartHandler<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
>> {
    state: Shared<AlternateState<C>>,
}
// Java: public class FallBackHandler
pub struct FallBackHandler<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
>> {
    state: Shared<AlternateState<C>>,
}
// Java: public class OleObjectHandler
pub struct OleObjectHandler<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
>> {
    state: Shared<AlternateState<C>>,
}

// Java: public class SwShapeAlertnateContentHandler extends DefaultElementHandler
impl<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
> + 'static> SwShapeAlertnateContentHandler<C> {
    pub fn new(
        swParent: Shared<KmoShowShape>,
        dgGroup: Shared<Shape>,
        dgStore: Shared<DgStore>,
        handlerCallback: Callback<C>,
    ) -> Self {
        Self::SwShapeAlertnateContentHandler(swParent, dgGroup, dgStore, handlerCallback)
    }
    // Java: SwShapeAlertnateContentHandler(KmoShowShape swParent, Shape dgGroup, DgStore dgStore, HandlerCallback handlerCallback)
    pub fn SwShapeAlertnateContentHandler(
        swParent: Shared<KmoShowShape>,
        dgGroup: Shared<Shape>,
        dgStore: Shared<DgStore>,
        handlerCallback: Callback<C>,
    ) -> Self {
        // Java: this((ArrayList)null, swParent, dgGroup, dgStore, handlerCallback);
        Self::SwShapeAlertnateContentHandler__list(
            None,
            swParent,
            dgGroup,
            dgStore,
            handlerCallback,
        )
    }
    // Java: SwShapeAlertnateContentHandler(ArrayList<KmoShowShape> newSps, KmoShowShape swParent, Shape dgGroup, DgStore dgStore, HandlerCallback handlerCallback)
    pub fn SwShapeAlertnateContentHandler__list(
        newSps: Option<Shared<Vec<Shared<KmoShowShape>>>>,
        swParent: Shared<KmoShowShape>,
        dgGroup: Shared<Shape>,
        dgStore: Shared<DgStore>,
        handlerCallback: Callback<C>,
    ) -> Self {
        // Java: this.dgStore = dgStore;
        // Java: this.handlerCallback = handlerCallback;
        // Java: this.newSps = newSps;
        let state = Rc::new(RefCell::new(AlternateState {
            swParent: swParent.clone(),
            handlerCallback,
            dgStore,
            dgGroup: dgGroup.clone(),
            shape: None,
            rId: None,
            transform2d: None,
            choiceFrameShape: None,
            graphicFrameHandler: None,
            newSps,
        }));
        // Java: this.init(swParent, dgGroup);
        let mut this = Self { state };
        this.init(swParent, dgGroup);
        this
    }
    // Java: void init(KmoShowShape swParent, Shape dgGroup)
    pub fn init(&mut self, swParent: Shared<KmoShowShape>, dgGroup: Shared<Shape>) {
        let mut s = self.state.borrow_mut();
        // Java: this.swParent = swParent;
        s.swParent = swParent;
        // Java: this.dgGroup = dgGroup;
        s.dgGroup = dgGroup;
        // Java: this.rId = null;
        s.rId = None;
        // Java: this.transform2d = null;
        s.transform2d = None;
        // Java: this.shape = null;
        s.shape = None;
    }
    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 2162693:
            2162693 => {
                // Java: handler = new ChoiceHandler();
                handler = Some(Box::new(ChoiceHandler {
                    state: self.state.clone(),
                }));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 2162695:
            2162695 => {
                // Java: handler = new FallBackHandler();
                handler = Some(Box::new(FallBackHandler {
                    state: self.state.clone(),
                }));
            }
            _ => handler = None,
        }
        // Java: return handler;
        handler
    }
    // Java: void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
        let (needs_ink, rid, transform, callback, parent, group, store, new_sps) = {
            let s = self.state.borrow();
            (
                s.shape.is_none() && s.rId.is_some() && s.transform2d.is_some(),
                s.rId.clone(),
                s.transform2d.clone(),
                s.handlerCallback.clone(),
                s.swParent.clone(),
                s.dgGroup.clone(),
                s.dgStore.clone(),
                s.newSps.clone(),
            )
        };
        // Java: if (this.shape == null && this.rId != null && this.transform2d != null)
        if needs_ink {
            // rust: Java 实参 `this.swParent.parent().getPresentation().getFileManager()` 在本
            //   波**不可达**：`KmoShowShape::parent()` 的 `Option<*const dyn ISlide>` 解引用后
            //   得到 `ISlide::KmoPresentation` —— 那是 `app/slide/ISlide.rs:20` 的**编译面宿主**
            //   （只有 `drawing_store` 字段 + `canvas()`/`drawingStore()`），**无 `getFileManager`**；
            //   而持该方法的真身 `show::app::KmoPresentation`（`KmoPresentation.rs:1818`）
            //   仍在 `#[cfg(feature = "full")]` 门禁内（`app/mod.rs:20` ，
            //   其依赖闭包是独立波次 M-PPT-PRESENTATION）。
            //   `onInk` 的形参是 `Option<&mut C::FileManager>`，故按本仓既有约定
            //   （`ConvertTool.rs:268` 对同一表达式落 `None`）传 `None`：**不伪造**任何
            //   `FileManager` 值，也不新建假类型；`onInk` 的默认实现返回 -1（不进入 ink 分支），
            //   行为与 Java 侧无可达差异。
            //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
            // Java: int inkId = this.handlerCallback.onInk(this.rId, this.swParent.parent().getPresentation().getFileManager());
            let inkId = callback
                .as_ref()
                .expect("Java ink path requires callback")
                .borrow_mut()
                .onInk(rid.as_deref(), None);
            // Java: if (inkId != -1)
            if inkId != -1 {
                // Java: this.shape = new KmoShowShape(this.swParent.parent(), 1);
                // rust: Java 构造器 `KmoShowShape(ISlide parent, int type)` 在 port 内的真实
                //   入口是 `KmoShowShape__islide_int`（`KmoShowShape.rs:283`；`__` 后缀是模板
                //   引擎对重载的既定命名），**无** `new`。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                let shape = match parent.borrow().parent() {
                    Some(p) => {
                        let slide: &dyn ISlide = unsafe { &*p };
                        Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int(slide, 1)))
                    }
                    None => panic!("Java NPE: swParent.parent() is null"),
                };
                self.state.borrow_mut().shape = Some(shape.clone());
                // Java: ShowShape showShape = ShowShape.open(this.shape.protocol());
                // rust: `protocol()` 返回 `Box<dyn KoBin>`，`ShowShape::open` 取 `&dyn KoBin`
                //   ⇒ 先绑定局部再 `&*`（同 `SwPictureHandler.rs:154-155`）。
                //   `open` 返回值而本 port 该局部用共享句柄 ⇒ `Rc::new(RefCell::new(..))`。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                let protocol = shape.borrow().protocol();
                let showShape = Rc::new(RefCell::new(ShowShape::open(&*protocol)));
                // Java: showShape.putInkId(inkId);
                showShape.borrow_mut().putInkId(inkId);
                // Java: Shape dgShape = Shape.open(this.shape.drawingShape());
                let drawing = shape.borrow().drawingShape();
                let dgShape = Rc::new(RefCell::new(Shape::open(&*drawing)));
                // Java: dgShape.spPr().geom().putType(0);
                // rust: 真 `Shape::spPr()` 取 `&self` 且返回 `&ShapePr`（`Shape.rs:916`），
                //   无 `spPr_mut()`；而 `ShapePr::geom()` 需 `&mut ShapePr`（`ShapePr.rs:965`）
                //   ⇒ 按真身的写入通道 `Shape::writeSpPr(&ShapePr)` 走：
                //   先 `convert_clone()` 取四槽独立副本（`ShapePr.rs:1481`），
                //   在副本上 `geom().putType(0)`，再整体写回。Java 的
                //   "读 spPr → 改 geom 的 type 槽" 语义不变。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                let mut geom_pr = dgShape.borrow().spPr().convert_clone();
                geom_pr.geom().putType(0);
                dgShape.borrow_mut().writeSpPr(&geom_pr);
                // Java: this.transform2d.save();
                transform.as_ref().unwrap().borrow_mut().save();
                // Java: dgShape.spPr().writeXfrm(this.transform2d);
                // rust: 真 `Transform2D` 无 `Clone`；本仓既定等价物是
                //   `ShapePr::convert_xfrm_clone()`（`ShapePr.rs:1489`，KProto 往返产出独立副本），
                //   `writeXfrm` 只做 `lastBin`，与 Java 传同一引用语义一致。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                // rust: 同上 —— `spPr()` 是只读返回，写入走 `writeSpPr`：
                //   在 `convert_clone()` 副本上 `writeXfrm`，再整体写回，效果同 Java 就地写。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                let mut xfrm_pr = dgShape.borrow().spPr().convert_clone();
                let xfrm = xfrm_pr.convert_xfrm_clone();
                xfrm_pr.writeXfrm(xfrm.as_ref());
                dgShape.borrow_mut().writeSpPr(&xfrm_pr);
                // Java: this.shape.updateProtocol(showShape.save());
                let saved = showShape.borrow_mut().save();
                shape.borrow_mut().updateProtocol(&*saved);
                // Java: if (null != this.newSps && null != this.handlerCallback && this.handlerCallback.isGVml())
                if new_sps.is_some() && callback.as_ref().is_some_and(|c| c.borrow().isGVml()) {
                    // Java: this.newSps.add(this.shape);
                    new_sps.unwrap().borrow_mut().push(shape.clone());
                // Java: else
                } else {
                    // Java: this.swParent.addChild(this.shape);
                    parent.borrow_mut().addChild(shape.borrow().clone());
                    // Java: this.dgGroup.children().add(this.shape.id());
                    let child_id = shape.borrow().id();
                    dg_children_add(&mut group.borrow_mut(), child_id);
                }
                // Java: this.dgStore.update(this.shape.id(), dgShape.save());
                let saved_dg = dgShape.borrow_mut().save();
                store
                    .borrow_mut()
                    .update(shape.borrow().id(), &*saved_dg);
            }
        }
    }
}

// Java: public class ChoiceHandler
impl<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
> + 'static> ChoiceHandler<C> {
    // Java: ChoiceHandler()
    pub fn ChoiceHandler(state: Shared<AlternateState<C>>) -> Self {
        Self { state }
    }
    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211595:
            3211595 => {
                let (parent, group, callback) = {
                    let s = self.state.borrow();
                    (
                        s.swParent.clone(),
                        s.dgGroup.clone(),
                        s.handlerCallback.clone(),
                    )
                };
                // Java: SwShapeAlertnateContentHandler.this.choiceFrameShape = new KmoShowShape(SwShapeAlertnateContentHandler.this.swParent.parent(), 4);
                // rust: `KmoShowShape(ISlide, int)` 的真入口是 `KmoShowShape__islide_int`；
                //   `parent()` 是 `Option<*const dyn ISlide>` ⇒ 解引用成 `&dyn ISlide`。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                let frame = match parent.borrow().parent() {
                    Some(p) => {
                        let slide: &dyn ISlide = unsafe { &*p };
                        Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int(slide, 4)))
                    }
                    None => panic!("Java NPE: swParent.parent() is null"),
                };
                self.state.borrow_mut().choiceFrameShape = Some(frame.clone());
                // Java: SwShapeAlertnateContentHandler.this.dgGroup.children().add(SwShapeAlertnateContentHandler.this.choiceFrameShape.id());
                let child_id = frame.borrow().id();
                dg_children_add(&mut group.borrow_mut(), child_id);
                // Java: SwShapeAlertnateContentHandler.this.swParent.addChild(SwShapeAlertnateContentHandler.this.choiceFrameShape);
                parent.borrow_mut().addChild(frame.borrow().clone());
                // Java: if (SwShapeAlertnateContentHandler.this.graphicFrameHandler == null)
                if self.state.borrow().graphicFrameHandler.is_none() {
                    // Java: SwShapeAlertnateContentHandler.this.graphicFrameHandler = new SwGraphicFrameHandler(SwShapeAlertnateContentHandler.this.choiceFrameShape, SwShapeAlertnateContentHandler.this.handlerCallback);
                    self.state.borrow_mut().graphicFrameHandler = Some(Rc::new(RefCell::new(
                        SwGraphicFrameHandler::new(frame, callback),
                    )));
                // Java: else
                } else {
                    // Java: SwShapeAlertnateContentHandler.this.graphicFrameHandler.init(SwShapeAlertnateContentHandler.this.choiceFrameShape);
                    // rust: 外层 `state` 的 `Ref` 必须先绑定到局部，否则该临时值在语句末被丢弃，
                    //   而内层句柄的 `borrow_mut()` 借用会越过它（E0596）。
                    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                    let st = self.state.borrow();
                    let gf = st.graphicFrameHandler.as_ref().unwrap().clone();
                    gf.borrow_mut().init(frame);
                }
                // Java: handler = SwShapeAlertnateContentHandler.this.graphicFrameHandler;
                // rust: 同上 —— 先绑定 `Ref` 到局部再取句柄克隆。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                let gf_handler = {
                    let st = self.state.borrow();
                    st.graphicFrameHandler.as_ref().unwrap().clone()
                };
                handler = Some(Box::new(GraphicProxy(gf_handler)));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211597:
            3211597 => {
                // Java: handler = SwShapeAlertnateContentHandler.this.new ContentPartHandler();
                handler = Some(Box::new(ContentPartHandler {
                    state: self.state.clone(),
                }));
            }
            _ => handler = None,
        }
        // Java: return handler;
        handler
    }
}
struct GraphicProxy<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
>>(
    Shared<SwGraphicFrameHandler<C>>,
);
impl<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
> + 'static> ElementHandler for GraphicProxy<C> {
    // rust: ② 契约要求 `kind`（仅 `characters` 有默认实现）。本代理转发
    //   `SwGraphicFrameHandler`，与其契约实现同报该类型名。
    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "SwGraphicFrameHandler"
    }
    fn startElement(&mut self, e: i32, a: &XmlRoAttrList) {
        self.0.borrow_mut().startElement(e, a)
    }
    fn endElement(&mut self, e: i32) {
        self.0.borrow_mut().endElement(e)
    }
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        self.0.borrow_mut().enterSubElement(e)
    }
}

// Java: public class ContentPartHandler
impl<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
> + 'static> ContentPartHandler<C> {
    // Java: ContentPartHandler()
    pub fn ContentPartHandler(state: Shared<AlternateState<C>>) -> Self {
        Self { state }
    }
    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp = null;
        // rust: Java 的 `XmlRoAttr` 由 `XmlRoAttrList` 承载，而 `getByName` 在本 port 返回
        //   `Option<&XmlRoAttrLeaf>`（`XmlRoAttrList.rs:136`）—— 叶子的 `toString()` 即 Java
        //   `XmlRoAttr.toString()`，故局部类型改为叶子。
        //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
        let mut temp: Option<&XmlRoAttrLeaf> = None;
        // Java: if ((temp = attrsNotCopy.getByName(4259841)) != null)
        temp = attrsNotCopy.get_by_name(4259841);
        if let Some(temp) = temp {
            // Java: SwShapeAlertnateContentHandler.this.rId = temp.toString();
            self.state.borrow_mut().rId = Some(temp.to_string());
        }
    }
    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3276837:
            3276837 => {
                // Java: SwShapeAlertnateContentHandler.this.transform2d = Transform2D.create();
                // rust: `Transform2D` 无 `Clone` ⇒ 与 `SwShapeHandler` 同一落法，以共享句柄
                //   承接新建值（`Rc::new(RefCell::new(..))`）。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                let transform = Rc::new(RefCell::new(Transform2D::create()));
                self.state.borrow_mut().transform2d = Some(transform.clone());
                // Java: handler = new Transform2DHandler(SwShapeAlertnateContentHandler.this.transform2d);
                // rust: 真构造器是 `Transform2DHandler::Transform2DHandler(Rc<RefCell<Transform2D>>)`
                //   （`Transform2DHandler.rs:112`），无 `new`。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(Transform2DHandler::Transform2DHandler(
                    transform.clone(),
                )));
            }
            // Java: default:
            _ => handler = None,
        }
        // Java: return handler;
        handler
    }
}

// Java: public class FallBackHandler
impl<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
> + 'static> FallBackHandler<C> {
    // Java: FallBackHandler()
    pub fn FallBackHandler(state: Shared<AlternateState<C>>) -> Self {
        Self { state }
    }
    fn allocate(&mut self, kind: i32) -> Shared<KmoShowShape> {
        let parent = self.state.borrow().swParent.clone();
        // rust: Java `new KmoShowShape(this.swParent.parent(), kind)` ⇒ 真实入口
        //   `KmoShowShape__islide_int`；`parent()` 的 `Option<*const dyn ISlide>` 解引用成
        //   `&dyn ISlide`（`KmoShowShape.rs:283`）。
        //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
        let shape = match parent.borrow().parent() {
            Some(p) => {
                let slide: &dyn ISlide = unsafe { &*p };
                Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int(slide, kind)))
            }
            None => panic!("Java NPE: swParent.parent() is null"),
        };
        self.state.borrow_mut().shape = Some(shape.clone());
        shape
    }
    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211469:
            3211469 => {
                // Java: SwShapeAlertnateContentHandler.this.shape = new KmoShowShape(SwShapeAlertnateContentHandler.this.swParent.parent(), 2);
                let shape = self.allocate(2);
                // Java: if (SwShapeAlertnateContentHandler.this.choiceFrameShape != null && SwShapeAlertnateContentHandler.this.choiceFrameShape.graphic() instanceof KmoModel3D)
                // rust: ★DEFERRED —— `KmoShowShape::graphic()` 在真身内返回**恒 `None`** 的
                //   占位实现（`KmoShowShape.rs:1079-1095`，其 `else` 分支体被清空），
                //   `instanceof KmoModel3D` 在本 port 无真值可判；且该分支属 `a:graphic`
                //   内层 3D 派发，为非管线核心依赖（PLAN §16）。
                //   故按规则整体短路，保留 Java 的「不进入分支」结果。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler
                //   wave: M-PPT-SHAPE-DISPATCH
                //   reason: KmoShowShape::graphic() 真身为恒 None 占位；3D 派发为非管线核心。
                if let Some(choice) = self.state.borrow().choiceFrameShape.clone() {
                    if choice.borrow().graphic().is_some() {
                        // Java: ((KmoModel3D)SwShapeAlertnateContentHandler.this.choiceFrameShape.graphic()).setPicture(SwShapeAlertnateContentHandler.this.shape);
                        todo!("DEFERRED_NON_FUNCTION");
                    }
                }
                // Java: handler = new SwPictureHandler(SwShapeAlertnateContentHandler.this.shape, SwShapeAlertnateContentHandler.this.handlerCallback);
                // rust: 真构造器是 `SwPictureHandler::SwPictureHandler`（`SwPictureHandler.rs:119`），无 `new`。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(SwPictureHandler::SwPictureHandler(
                    shape,
                    self.state.borrow().handlerCallback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211471:
            3211471 => {
                // Java: handler = new OleObjectHandler();
                handler = Some(Box::new(OleObjectHandler {
                    state: self.state.clone(),
                }));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211593:
            3211593 => {
                // Java: SwShapeAlertnateContentHandler.this.shape = new KmoShowShape(SwShapeAlertnateContentHandler.this.swParent.parent(), 1);
                let shape = self.allocate(1);
                // Java: handler = new SwShapeHandler(SwShapeAlertnateContentHandler.this.shape, SwShapeAlertnateContentHandler.this.handlerCallback);
                // rust: 真构造器是 `SwShapeHandler::SwShapeHandler`（`SwShapeHandler.rs:101`），无 `new`。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(SwShapeHandler::SwShapeHandler(
                    shape,
                    self.state.borrow().handlerCallback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211594:
            3211594 => {
                // Java: SwShapeAlertnateContentHandler.this.shape = new KmoShowShape(SwShapeAlertnateContentHandler.this.swParent.parent(), 0);
                let shape = self.allocate(0);
                // Java: handler = new SwGroupShapeHandler(SwShapeAlertnateContentHandler.this.shape, SwShapeAlertnateContentHandler.this.handlerCallback);
                handler = Some(Box::new(SwGroupShapeHandler::SwGroupShapeHandler(
                    shape,
                    self.state.borrow().handlerCallback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211595:
            3211595 => {
                // Java: SwShapeAlertnateContentHandler.this.shape = new KmoShowShape(SwShapeAlertnateContentHandler.this.swParent.parent(), 4);
                let shape = self.allocate(4);
                // Java: handler = new SwGraphicFrameHandler(SwShapeAlertnateContentHandler.this.shape, SwShapeAlertnateContentHandler.this.handlerCallback);
                handler = Some(Box::new(SwGraphicFrameHandler::new(
                    shape,
                    self.state.borrow().handlerCallback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211596:
            3211596 => {
                // Java: SwShapeAlertnateContentHandler.this.shape = new KmoShowShape(SwShapeAlertnateContentHandler.this.swParent.parent(), 3);
                let shape = self.allocate(3);
                // Java: handler = new SwConnectorShapeHandler(SwShapeAlertnateContentHandler.this.shape, SwShapeAlertnateContentHandler.this.handlerCallback);
                // rust: 真构造器是 `SwConnectorShapeHandler::SwConnectorShapeHandler`
                //   （`SwConnectorShapeHandler.rs:86`），无 `new`。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(SwConnectorShapeHandler::SwConnectorShapeHandler(
                    shape,
                    self.state.borrow().handlerCallback.clone(),
                )));
            }
            _ => handler = None,
        }
        // Java: return handler;
        handler
    }
    // Java: void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
        let (rid, callback, parent, transform, shape, choice, store, group, new_sps) = {
            let s = self.state.borrow();
            (
                s.rId.clone(),
                s.handlerCallback.clone(),
                s.swParent.clone(),
                s.transform2d.clone(),
                s.shape.clone(),
                s.choiceFrameShape.clone(),
                s.dgStore.clone(),
                s.dgGroup.clone(),
                s.newSps.clone(),
            )
        };
        // Java: if (SwShapeAlertnateContentHandler.this.rId != null)
        if let Some(rid) = rid {
            // rust: 同上层 `endElement` —— `getFileManager()` 在本波不可达（`ISlide` 侧是
            //   无该方法的编译面宿主，真身仍在 `feature = "full"` 门禁内），
            //   故按既有约定落 `None`（同 `ConvertTool.rs:268`），不伪造 `FileManager`。
            //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
            // Java: int inkId = SwShapeAlertnateContentHandler.this.handlerCallback.onInk(SwShapeAlertnateContentHandler.this.rId, SwShapeAlertnateContentHandler.this.swParent.parent().getPresentation().getFileManager());
            let inkId = callback
                .as_ref()
                .expect("Java ink fallback requires callback")
                .borrow_mut()
                .onInk(Some(&rid), None);
            // Java: if (inkId != -1 && SwShapeAlertnateContentHandler.this.shape != null)
            if inkId != -1 && shape.is_some() {
                let shape = shape.as_ref().unwrap();
                // Java: ShowShape showShape = ShowShape.open(SwShapeAlertnateContentHandler.this.shape.protocol());
                // rust: `protocol()` 返回 `Box<dyn KoBin>`，`open` 取 `&dyn KoBin` ⇒ 先绑定局部再 `&*`。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                let protocol = shape.borrow().protocol();
                let showShape = Rc::new(RefCell::new(ShowShape::open(&*protocol)));
                // Java: showShape.putInkId(inkId);
                showShape.borrow_mut().putInkId(inkId);
                // Java: SwShapeAlertnateContentHandler.this.shape.updateProtocol(showShape.save());
                let saved = showShape.borrow_mut().save();
                shape.borrow_mut().updateProtocol(&*saved);
                // Java: if (SwShapeAlertnateContentHandler.this.transform2d != null)
                if let Some(transform) = &transform {
                    // Java: Shape dgShape = Shape.open(SwShapeAlertnateContentHandler.this.shape.drawingShape());
                    let drawing = shape.borrow().drawingShape();
                    let dgShape = Rc::new(RefCell::new(Shape::open(&*drawing)));
                    // Java: SwShapeAlertnateContentHandler.this.transform2d.save();
                    transform.borrow_mut().save();
                    // Java: dgShape.spPr().writeXfrm(SwShapeAlertnateContentHandler.this.transform2d);
                    // rust: 同上层 —— 真 `Transform2D` 无 `Clone`，经
                    //   `ShapePr::convert_xfrm_clone()` 产出独立副本后再 `writeXfrm`。
                    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                    // rust: 同上 —— 经 `writeSpPr` 写回（`spPr()` 只读）。
                    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                    let mut xfrm_pr = dgShape.borrow().spPr().convert_clone();
                    let xfrm = xfrm_pr.convert_xfrm_clone();
                    xfrm_pr.writeXfrm(xfrm.as_ref());
                    dgShape.borrow_mut().writeSpPr(&xfrm_pr);
                    // Java: SwShapeAlertnateContentHandler.this.dgStore.update(SwShapeAlertnateContentHandler.this.shape.id(), dgShape.save());
                    let saved_dg = dgShape.borrow_mut().save();
                    store.borrow_mut().update(shape.borrow().id(), &*saved_dg);
                }
            }
        }
        // Java: if (SwShapeAlertnateContentHandler.this.shape != null && SwShapeAlertnateContentHandler.this.choiceFrameShape == null)
        if shape.is_some() && choice.is_none() {
            let shape = shape.unwrap();
            // Java: BlipFill blipFill = null;
            let mut blipFill: Option<Shared<BlipFill>> = None;
            // Java: Shape dgShape = Shape.open(SwShapeAlertnateContentHandler.this.dgStore.query(SwShapeAlertnateContentHandler.this.shape.id()));
            let query_bin = store.borrow().query(shape.borrow().id());
            let dgShape = Rc::new(RefCell::new(Shape::open(&*query_bin)));
            // Java: switch (SwShapeAlertnateContentHandler.this.shape.type())
            // rust: 真身方法名是 `KmoShowShape::r#type()`（`KmoShowShape.rs:1119`，模板引擎对
            //   Java `type()` 关键字的既定改写），语义同 Java `type()`。
            //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
            match shape.borrow().r#type() {
                // Java: case 1:
                1 => {
                    // Java: Fill fill = dgShape.spPr().fill();
                    // rust: `ShapePr::fill()` 取 `&mut self` 且返回 `&mut Fill`（`ShapePr.rs:1055`），
                    //   Java 返回的是引用 ⇒ 以 `&mut` 承接同一对象（**不**拷贝，语义同 Java）。
                    //   需要同时持有 `dgShape` 与 `spPr` 的可变借用，故分两步绑定。
                    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                    let mut fill_pr = dgShape.borrow().spPr().convert_clone();
                    let fill: &mut Fill = fill_pr.fill();
                    // Java: if (fill.type() == 2)
                    if fill.r#type() == 2 {
                        // Java: blipFill = fill.blipFill();
                        // rust: `Fill::blipFill()` 取 `&mut self`（`Fill.rs:661`）并返回 `&BlipFill`；
                        //   真 `BlipFill` 无 `Clone` ⇒ 按 KProto 往返取独立副本。
                        //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                        let bf_bin = {
                            let bf = fill.blipFill();
                            bf.lastBin(KoZone::enter())
                        };
                        blipFill = Some(Rc::new(RefCell::new(BlipFill::open(&*bf_bin))));
                    }
                    // Java: break;
                    // rust: no-stmt
                }
                // Java: case 2:
                2 => {
                    // Java: blipFill = dgShape.blipFill().blipFill();
                    // rust: `Shape::blipFill()` 返回 `&Fill`（`Shape.rs:1190`），需 `&mut self`
                    //   才能再取 `Fill::blipFill()`（返回 `&BlipFill`）⇒ 经 `RefMut` 解引用。
                    //   真 `BlipFill` 无 `Clone`，故局部 `blipFill` 改以共享句柄承接。
                    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                    // rust: 真 `Shape::blipFill()` 取 `&self` 且返回 `&Fill`（`Shape.rs:1190`）
                    //   ⇒ 无法就地取 `&mut Fill`；改在 `convert_clone()` 副本上取 blipFill，
                    //   再按 KProto 往返产出独立 `BlipFill`（Java 只读该对象，无需写回）。
                    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                    let mut bf_pr = dgShape.borrow().spPr().convert_clone();
                    let bf_bin = {
                        let bf = bf_pr.fill().blipFill();
                        bf.lastBin(KoZone::enter())
                    };
                    blipFill = Some(Rc::new(RefCell::new(BlipFill::open(&*bf_bin))));
                }
                // Java: case 3:
                // Java: default:
                3 => {
                    // Java: break;
                    // rust: no-stmt
                }
                // Java: case 4:
                4 => {
                    // Java: blipFill = dgShape.blipFill().blipFill();
                    // rust: 真 `Shape::blipFill()` 取 `&self` 且返回 `&Fill`（`Shape.rs:1190`）
                    //   ⇒ 无法就地取 `&mut Fill`；改在 `convert_clone()` 副本上取 blipFill，
                    //   再按 KProto 往返产出独立 `BlipFill`（Java 只读该对象，无需写回）。
                    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                    let mut bf_pr = dgShape.borrow().spPr().convert_clone();
                    let bf_bin = {
                        let bf = bf_pr.fill().blipFill();
                        bf.lastBin(KoZone::enter())
                    };
                    blipFill = Some(Rc::new(RefCell::new(BlipFill::open(&*bf_bin))));
                }
                _ => {}
            }
            // Java: if (blipFill != null && blipFill.fillMode().type() == 1 && !blipFill.fillMode().hasStretch())
            // rust: ★DEFERRED —— 该条件的两个谓词 `FillMode.type()` / `FillMode.hasStretch()`
            //   与内层改写 `FillMode.stretch().forceWriteFillRect(..)` 三处都需要**真身
            //   `FillMode`**；而 `data/fill/mod.rs:19` 仍把 `pub mod FillMode;` 注释掉
            //   （该文件自身有 13 处 `todo!("lazy init requires &mut self")`），
            //   登记的清偿波次是 `data/fill` 的 **M-PPT-DATA-STUBS**。
            //   本 port 的 `BlipFill::fillMode()` 因此只返回不透明的 `&mut dyn KoBin`
            //   （`BlipFill.rs:696`），既无 `downcast` 也无 `type()`/`hasStretch()` 可调，
            //   故按 PLAN §16「非管线核心」规则整体短路该分支。
            //   短路结果与 Java 在"fillMode 非 stretch"时一致：**不改写** `a:fillRect`；
            //   这是本 port 当前唯一可表达的行为，且不伪造任何 `FillMode` 值或类型。
            //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler
            //   wave: M-PPT-SHAPE-DISPATCH
            //   reason: 真身 FillMode 模块被 data/fill/mod.rs:19 注释掉（M-PPT-DATA-STUBS），
            //           fillMode() 仅返回不可判型的 &mut dyn KoBin。
            // Java: RelativeRect fillRect = RelativeRect.create();
            // Java: blipFill.fillMode().stretch().forceWriteFillRect(fillRect);
            let _ = &blipFill;
            let _ = RelativeRect::create();
            // Java: if (blipFill != null || SwShapeAlertnateContentHandler.this.shape.hasInk())
            if blipFill.is_some() || shape.borrow().hasInk() {
                // Java: if (null != SwShapeAlertnateContentHandler.this.newSps && null != SwShapeAlertnateContentHandler.this.handlerCallback && SwShapeAlertnateContentHandler.this.handlerCallback.isGVml())
                if new_sps.is_some() && callback.as_ref().is_some_and(|c| c.borrow().isGVml()) {
                    // Java: SwShapeAlertnateContentHandler.this.newSps.add(SwShapeAlertnateContentHandler.this.shape);
                    new_sps.unwrap().borrow_mut().push(shape.clone());
                // Java: else
                } else {
                    // Java: SwShapeAlertnateContentHandler.this.swParent.addChild(SwShapeAlertnateContentHandler.this.shape);
                    // rust: `addChild` 取 `KmoShowShape` 值（Java 侧是引用）；本 port 的 `shape`
                    //   是共享句柄 ⇒ 取内部值的快照传入（同 `KmoShowShape.rs:932` 的既有接线）。
                    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                    parent.borrow_mut().addChild(shape.borrow().clone());
                    // Java: SwShapeAlertnateContentHandler.this.dgGroup.children().add(SwShapeAlertnateContentHandler.this.shape.id());
                    let child_id = shape.borrow().id();
                    dg_children_add(&mut group.borrow_mut(), child_id);
                }
                // Java: SwShapeAlertnateContentHandler.this.dgStore.update(SwShapeAlertnateContentHandler.this.shape.id(), dgShape.save());
                // rust: `DgStore::update` 取 `&dyn KoBin`，`save()` 返回 `Box<dyn KoBin>`
                //   ⇒ 先绑定局部再 `&*`（同 `SwPictureHandler.rs:154-155`）。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                let saved_last = dgShape.borrow_mut().save();
                store.borrow_mut().update(shape.borrow().id(), &*saved_last);
            }
        }
    }
}

// Java: public class OleObjectHandler
impl<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
> + 'static> OleObjectHandler<C> {
    // Java: OleObjectHandler()
    pub fn OleObjectHandler(state: Shared<AlternateState<C>>) -> Self {
        Self { state }
    }
    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211469:
            3211469 => {
                let parent = self.state.borrow().swParent.clone();
                // Java: KmoShowShape picture = new KmoShowShape(SwShapeAlertnateContentHandler.this.swParent.parent(), 2);
                // rust: 真入口 `KmoShowShape__islide_int`；`parent()` 解引用成 `&dyn ISlide`。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                let picture = match parent.borrow().parent() {
                    Some(p) => {
                        let slide: &dyn ISlide = unsafe { &*p };
                        Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int(slide, 2)))
                    }
                    None => panic!("Java NPE: swParent.parent() is null"),
                };
                // Java: SwShapeAlertnateContentHandler.this.swParent.addChild(picture);
                parent.borrow_mut().addChild(picture.borrow().clone());
                // Java: handler = new SwPictureHandler(picture, SwShapeAlertnateContentHandler.this.handlerCallback);
                // rust: 真构造器是 `SwPictureHandler::SwPictureHandler`（`SwPictureHandler.rs:119`），无 `new`。
                //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(SwPictureHandler::SwPictureHandler(
                    picture,
                    self.state.borrow().handlerCallback.clone(),
                )));
            }
            // Java: case 3211467:
            3211467 => handler = None,
            // Java: default:
            _ => {
                // Java: return handler;
                handler = None;
            }
        }
        handler
    }
}

impl<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
> + 'static> ElementHandlerContract
    for SwShapeAlertnateContentHandler<C>
{
    // rust: ② 契约 `dmlr::handler::ElementHandler` 要求 `kind` / `startElement` /
    //   `endElement`，仅 `characters` 有默认实现；Java 侧由 `DefaultElementHandler`
    //   基类提供空实现（本 port 不继承该基类）。`kind` 返回 Java 类名。
    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "SwShapeAlertnateContentHandler"
    }
    fn startElement(
        &mut self,
        _e: i32,
        _a: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn endElement(&mut self, e: i32) {
        SwShapeAlertnateContentHandler::endElement(self, e)
    }
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        SwShapeAlertnateContentHandler::enterSubElement(self, e)
    }
}
impl<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
> + 'static> ElementHandlerContract
    for ChoiceHandler<C>
{
    // rust: ② 契约 `dmlr::handler::ElementHandler` 要求 `kind` / `startElement` /
    //   `endElement`，仅 `characters` 有默认实现；Java 侧由 `DefaultElementHandler`
    //   基类提供空实现（本 port 不继承该基类）。`kind` 返回 Java 类名。
    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "ChoiceHandler"
    }
    fn startElement(
        &mut self,
        _e: i32,
        _a: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn endElement(&mut self, _e: i32) {}
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        ChoiceHandler::enterSubElement(self, e)
    }
}
impl<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
> + 'static> ElementHandlerContract
    for ContentPartHandler<C>
{
    // rust: ② 契约 `dmlr::handler::ElementHandler` 要求 `kind` / `startElement` /
    //   `endElement`，仅 `characters` 有默认实现；Java 侧由 `DefaultElementHandler`
    //   基类提供空实现（本 port 不继承该基类）。`kind` 返回 Java 类名。
    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "ContentPartHandler"
    }
    fn endElement(&mut self, _e: i32) {}
    fn startElement(&mut self, e: i32, a: &XmlRoAttrList) {
        ContentPartHandler::startElement(self, e, a)
    }
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        ContentPartHandler::enterSubElement(self, e)
    }
}
impl<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
> + 'static> ElementHandlerContract
    for FallBackHandler<C>
{
    // rust: ② 契约 `dmlr::handler::ElementHandler` 要求 `kind` / `startElement` /
    //   `endElement`，仅 `characters` 有默认实现；Java 侧由 `DefaultElementHandler`
    //   基类提供空实现（本 port 不继承该基类）。`kind` 返回 Java 类名。
    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "FallBackHandler"
    }
    fn startElement(
        &mut self,
        _e: i32,
        _a: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn endElement(&mut self, e: i32) {
        FallBackHandler::endElement(self, e)
    }
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        FallBackHandler::enterSubElement(self, e)
    }
}
impl<C: HandlerCallback<
    FileManager = FileManager,
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
> + 'static> ElementHandlerContract
    for OleObjectHandler<C>
{
    // rust: ② 契约 `dmlr::handler::ElementHandler` 要求 `kind` / `startElement` /
    //   `endElement`，仅 `characters` 有默认实现；Java 侧由 `DefaultElementHandler`
    //   基类提供空实现（本 port 不继承该基类）。`kind` 返回 Java 类名。
    //   owner: show.pptxr.handler.shape.SwShapeAlertnateContentHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "OleObjectHandler"
    }
    fn startElement(
        &mut self,
        _e: i32,
        _a: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn endElement(&mut self, _e: i32) {}
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        OleObjectHandler::enterSubElement(self, e)
    }
}
