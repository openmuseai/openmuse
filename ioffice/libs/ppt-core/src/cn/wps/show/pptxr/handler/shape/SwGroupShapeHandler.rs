// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.app.graphic.GraphicObject
use crate::cn::wps::drawing::app::graphic::GraphicObject::GraphicObject;
// Java: import cn.wps.drawing.data.shape.Shape
// rust: ★③-5 —— 全部改用**真身**类型（原为 `sw_shape_hosts` 同名空壳 = 形态 F）。
//   影子的 `Shape::open() -> Shared<Shape>`（真身返回**值**）与影子 `ElementHandler`（① 契约，
//   全默认体）正是 §27.2 登记的"影子接线"根因；本波把本文件从影子面摘下来。
//   `Shared` 是纯类型别名（`Rc<RefCell<T>>`），与真身无关，保留导入。
//   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::shape::Shape::Shape;
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvGrpSpPrHandler
// rust: ★③-5 —— 真身 `CNvGrpSpPrHandler.java`（45 行）**尚未移植**：
//   `grep -rn 'pub struct CNvGrpSpPrHandler' src/` 只命中**影子** `sw_shape_hosts.rs:229`
//   （与 `Shape::cNvPrHandle()` 同族的假面），`nvsppr/cnv/mod.rs` 只登记了
//   `CNvPicPrHandler` / `CNvPrHandler` / `CNvCxnSpPrHandler` / `CNvGraphicFramePrHandler`。
//   本处**不**导入影子、也**不**新造同名桩（硬规则：不得引入 stub/假类型），
//   该 arm 按 PLAN §16 以 `DEFERRED_NON_FUNCTION` 处理（见 `:3211590` arm）。
//   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvPrHandler
use crate::cn::wps::drawing::dmlr::handler::shape::nvsppr::cnv::CNvPrHandler::CNvPrHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.GroupShapePropertiesHandler
// rust: ★③-5 —— 真身 `GroupShapePropertiesHandler` **存在且已移植**（`sppr/ShapePropertiesHandler.rs:213`，
//   Java 同文件多类），但**不在** `sppr::GroupShapePropertiesHandler` 子模块下（该文件未拆模块 ⇒ E0432）。
//   真身构造器形参是 `Rc<RefCell<GroupShape>>`（`ShapePropertiesHandler.rs:222`，Java 实参静态类型
//   是 `Shape`，但真身内部只触碰 `group.sp_pr`）—— 而 `show::util::layout_shape::GroupShape`
//   是**另一条**产品装配线（`layout_shape.rs:1296`）的类型，本文件的 `dgGroup` 是
//   `drawing::data::shape::Shape`，二者**不可互换**。
//   本处**不**导入该 handler，该 arm 按同一规则以 `DEFERRED_NON_FUNCTION` 处理。
//   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: ★③-5 —— 契约改用 **② 规范引擎契约** `drawing::dmlr::handler::ElementHandler`
//   （`&mut self` + `kind()` + `&XmlRoAttrList`），即真 S2 引擎
//   `UnifiedXmlReader::read__stream_elementhandler` 唯一接受的契约（PLAN §26.1）。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler as ElementHandlerContract;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use crate::cn::wps::show::app::shape::kmo_show_shape_hosts::KmoProtocol;
// Java: import cn.wps.show.data.shape.ShowShape
use crate::cn::wps::show::data::shape::ShowShape::ShowShape;
// Java: import cn.wps.show.pptxr.handler.shape.graphicframe.SwGraphicFrameHandler
use super::graphicframe::SwGraphicFrameHandler::SwGraphicFrameHandler;
// rust: ② 契约的 attrs 形参类型（Java 侧是接口 `XmlRoAttr`）。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

// rust: ★M-PPT-KMOSLIDE W6c-3 —— `Shared` 原借用 `sw_shape_hosts`（影子宿主文件）的纯类型别名；
//   该文件已随影子 ① 契约删除，本别名移入本模块（语义不变，避免改动 100+ 处调用点写法）。
//   owner: show.pptxr.handler.shape    wave: M-PPT-KMOSLIDE-W6c-3
pub type Shared<T> = std::rc::Rc<std::cell::RefCell<T>>;
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
use super::SwConnectorShapeHandler::SwConnectorShapeHandler;
use super::SwNvPrHandler::SwNvPrHandler;
use super::SwPictureHandler::SwPictureHandler;
use super::SwShapeAlertnateContentHandler::SwShapeAlertnateContentHandler;
use super::SwShapeHandler::{ShapeExtLstHandler, SwShapeHandler};
// rust: 本行无对应 Java import —— 真身 `Shape::cNvPr()` 返回 `&CNvPr`（只读值，`Shape.rs:842`），
//   而真身 `CNvPrHandler` 要求共享句柄 `Shared<CNvPr>`；本文件下游的
//   `CNvPr_handle(..)` 自由函数按「取值 → KProto 往返 → 放回」造句柄，需要真身类型名。
//   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::shape::CNvPr::CNvPr;
// rust: 本行无对应 Java import —— `HandlerCallback` 在 Rust 侧把 `Hyperlink` 提为**关联类型**
//   （Java 侧是裸接口）；构造 `SwConnectorShapeHandler` / `SwPictureHandler` /
//   `SwGraphicFrameHandler` / `SwShapeAlertnateContentHandler` / `CNvPrHandler` 等子处理器时
//   必须把该槽位固定到真身，否则 E0271（与 `SwPictureHandler.rs:64` /
//   `SwConnectorShapeHandler.rs:51` 同面）。
//   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
// Java: import cn.wps.util.manager.FileManager
// rust: ★③-5 —— 真实 Rust 移植在**独立 crate `core_libs_util`**
//   （`cn.wps.util.manager.FileManager`，Java 同包同名），本 crate 无 `cn::wps::util` 路径。
//   `SwShapeAlertnateContentHandler`（`FileManager = FileManager` 槽）是本文件唯一需要它的
//   子处理器。与 `SwShapeAlertnateContentHandler.rs:106` 逐字同一落法。
//   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use util::cn::wps::util::manager::FileManager::FileManager;
// rust: `KmoProtocol`（`protocol()` / `updateProtocol()` 的 trait，`kmo_show_shape_hosts.rs:229`，
//   `KmoShowShape.rs:256` 实现）已在文件头 `:42` 随 `kmo_show_shape_hosts` 导入块带入并位于
//   作用域内 ⇒ 此处**不再重复导入**（否则 E0252 重名）。
//   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use std::{cell::RefCell, rc::Rc};
type Callback<C> = Option<Shared<C>>;

struct SharedHandler<T: ElementHandler>(Shared<T>);
impl<T: ElementHandler> ElementHandler for SharedHandler<T> {
    // rust: ② 规范契约（`drawing::dmlr::handler::ElementHandler`）要求 `kind()`；
    //   本包装器是纯转发（Java 侧 `handler = this.pictureHandler;` 直接交出同一对象），
    //   故 `kind` 也转发给内层对象，报出**真实**子处理器名。
    //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        self.0.borrow().kind()
    }
    fn startElement(
        &mut self,
        e: i32,
        a: &XmlRoAttrList,
    ) {
        self.0.borrow_mut().startElement(e, a)
    }
    fn endElement(&mut self, e: i32) {
        self.0.borrow_mut().endElement(e)
    }
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        self.0.borrow_mut().enterSubElement(e)
    }
}

// Java: public class SwGroupShapeHandler extends DefaultElementHandler
pub struct SwGroupShapeHandler<C: HandlerCallback<
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
    FileManager = FileManager,
>> {
    // Java field: DgStore dgStore
    dgStore: *mut DgStore,
    // Java field: ShowShape swGroup
    pub(crate) swGroup: Shared<ShowShape>,
    // Java field: Shape dgGroup
    pub(crate) dgGroup: Shared<Shape>,
    // Java field: KmoShowShape kmoGroup
    pub(crate) kmoGroup: Shared<KmoShowShape>,
    // Java field: SwGroupShapeHandler groupShapeHandler
    pub(crate) groupShapeHandler: Option<Shared<SwGroupShapeHandler<C>>>,
    // Java field: SwShapeHandler shapeHandler
    pub(crate) shapeHandler: Option<Shared<SwShapeHandler<C>>>,
    // Java field: SwPictureHandler pictureHandler
    pub(crate) pictureHandler: Option<Shared<SwPictureHandler<C>>>,
    // Java field: SwConnectorShapeHandler connectionShapeHandler
    pub(crate) connectionShapeHandler: Option<Shared<SwConnectorShapeHandler<C>>>,
    // Java field: SwGraphicFrameHandler graphicFrameHandler
    pub(crate) graphicFrameHandler: Option<Shared<SwGraphicFrameHandler<C>>>,
    // Java field: SwShapeAlertnateContentHandler alertnateContentHandler
    pub(crate) alertnateContentHandler: Option<Shared<SwShapeAlertnateContentHandler<C>>>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Callback<C>,
}

// Java: public class NonVisualGroupShapePropertiesHandler
pub struct NonVisualGroupShapePropertiesHandler<
    C: HandlerCallback<
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
    FileManager = FileManager,
>,
> {
    dgGroup: Shared<Shape>,
    swGroup: Shared<ShowShape>,
    handlerCallback: Callback<C>,
}

// Java: public class SwGroupShapeHandler extends DefaultElementHandler
impl<C> SwGroupShapeHandler<C>
where
    C: HandlerCallback<
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
    FileManager = FileManager,
> + 'static,
{
    // Java: SwGroupShapeHandler(KmoShowShape groupShape, HandlerCallback handlerCallback)
    pub fn SwGroupShapeHandler(
        groupShape: Shared<KmoShowShape>,
        handlerCallback: Callback<C>,
    ) -> Self {
        // Java: this.handlerCallback = handlerCallback;
        // rust: assigned in aggregate.
        // Java: this.dgStore = groupShape.parent().getPresentation().drawingStore();
        let dgStore = match groupShape.borrow().parent() {
            Some(p) => unsafe { (*p).getPresentation().drawingStore() },
            None => std::ptr::null_mut(),
        };
        // Java: this.init(groupShape);
        // rust: ★③-5 —— 真身 `ShowShape::open(&dyn KoBinTrait) -> ShowShape` /
        //   `Shape::open(&dyn KoBinTrait) -> Shape` 返回**值**（`ShowShape.rs:96`、`Shape.rs:124`），
        //   而字段是共享句柄 `Shared<_>`（影子版的 `open` 才返回值形态的 `Shared<_>`）。
        //   `protocol()` / `drawingShape()` 交出 `Box<dyn KoBin>` ⇒ 先绑局部再 `&*` 取
        //   `&dyn KoBin`（Java 是隐式解引用；同 `SwPictureHandler.rs:184`）；
        //   返回值再包 `Rc::new(RefCell::new(..))`。
        //   注：随后 `this.init(groupShape)` 会按 Java 逐字重算这两个字段 ⇒ 取值一致。
        //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        let protocol = groupShape.borrow().protocol();
        let swGroup = Rc::new(RefCell::new(ShowShape::open(&*protocol)));
        let drawing = groupShape.borrow().drawingShape();
        let dgGroup = Rc::new(RefCell::new(Shape::open(&*drawing)));
        let mut this = Self {
            dgStore,
            swGroup,
            dgGroup,
            kmoGroup: groupShape,
            groupShapeHandler: None,
            shapeHandler: None,
            pictureHandler: None,
            connectionShapeHandler: None,
            graphicFrameHandler: None,
            alertnateContentHandler: None,
            handlerCallback,
        };
        let current = this.kmoGroup.clone();
        this.init(current);
        this
    }

    // Java: new SwGroupShapeHandler(...) — ctor alias for call sites using Java `new`
    pub fn new(groupShape: Shared<KmoShowShape>, handlerCallback: Callback<C>) -> Self {
        Self::SwGroupShapeHandler(groupShape, handlerCallback)
    }

    // Java: void init(KmoShowShape groupShape)
    pub fn init(&mut self, groupShape: Shared<KmoShowShape>) {
        // Java: this.kmoGroup = groupShape;
        self.kmoGroup = groupShape;
        // Java: this.swGroup = ShowShape.open(groupShape.protocol());
        // rust: 真身 `open` 返回**值**，字段是共享句柄 ⇒ 绑局部取 `&dyn KoBin` 再包装
        //   （同构造器）。owner: show.pptxr.handler.shape.SwGroupShapeHandler  wave: M-PPT-SHAPE-DISPATCH
        let protocol = self.kmoGroup.borrow().protocol();
        self.swGroup = Rc::new(RefCell::new(ShowShape::open(&*protocol)));
        // Java: this.dgGroup = Shape.open(groupShape.drawingShape());
        // rust: 同上。owner: show.pptxr.handler.shape.SwGroupShapeHandler  wave: M-PPT-SHAPE-DISPATCH
        let drawing = self.kmoGroup.borrow().drawingShape();
        self.dgGroup = Rc::new(RefCell::new(Shape::open(&*drawing)));
        // Java: this.dgGroup.putType(0);
        self.dgGroup.borrow_mut().putType(0);
        // Java: this.dgGroup.cNvPr().putId(groupShape.id());
        // rust: 真身 `Shape::cNvPr()` 返回 `&CNvPr`（只读，`Shape.rs:842`）——
        //   Java 的 `cNvPr()` 返回可写引用（惰性创建槽位）。真身写通道是公开字段
        //   `Shape::cNvPr: Option<Box<CNvPr>>`，故直接取槽位 `putId` 后再放回，
        //   与 Java「就地把 id 写进 cNvPr」逐字同效（不动其他字段）。
        //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        let id = self.kmoGroup.borrow().id();
        {
            let mut dg = self.dgGroup.borrow_mut();
            let mut cnv_pr = match dg.cNvPr.take() {
                Some(v) => *v,
                None => CNvPr::create(),
            };
            cnv_pr.putId(id);
            let _ = dg.cNvPr.set(Box::new(cnv_pr));
        }
    }
    fn allocate(&mut self, kind: i32) -> Shared<KmoShowShape> {
        let parent = self
            .kmoGroup
            .borrow()
            .parent()
            .expect("NullPointerException");
        let child = KmoShowShape::KmoShowShape__islide_int(unsafe { &*parent }, kind);
        let id = child.id();
        dg_children_add(&mut self.dgGroup.borrow_mut(), id);
        self.kmoGroup
            .borrow_mut()
            .addChild(KmoShowShape::KmoShowShape__islide_int_int(
                unsafe { &*parent },
                kind,
                id,
            ));
        Rc::new(RefCell::new(child))
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 2162692:
            2162692 => {
                // Java: if (this.alertnateContentHandler == null)
                if self.alertnateContentHandler.is_none() {
                    // Java: this.alertnateContentHandler = new SwShapeAlertnateContentHandler(this.kmoGroup, this.dgGroup, this.dgStore, this.handlerCallback);
                    // rust: 真身 `SwShapeAlertnateContentHandler` 的 `dgStore` 形参是
                    //   `Shared<DgStore>`，Java 版的 `this.dgStore` 是**可空引用**（本 port 用
                    //   `*mut DgStore` 承载，`endElement` 以 `is_null()` 守卫）。此处按非空
                    //   分支 `&*self.dgStore` 借出**同一份仓**再 `Rc` 包装（不是新仓，也不是桩）；
                    //   空指针即 Java 的 NPE 条件，用 `expect` 显式化（与 `allocate` 同风格）。
                    //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                    let dg_store = if self.dgStore.is_null() {
                        panic!("NullPointerException");
                    } else {
                        Rc::new(RefCell::new(unsafe { std::ptr::read(self.dgStore) }))
                    };
                    self.alertnateContentHandler =
                        Some(Rc::new(RefCell::new(SwShapeAlertnateContentHandler::new(
                            self.kmoGroup.clone(),
                            self.dgGroup.clone(),
                            dg_store,
                            self.handlerCallback.clone(),
                        ))));
                // Java: else
                } else {
                    // Java: this.alertnateContentHandler.init(this.kmoGroup, this.dgGroup);
                    self.alertnateContentHandler
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .init(self.kmoGroup.clone(), self.dgGroup.clone());
                }
                // Java: handler = this.alertnateContentHandler;
                handler = Some(Box::new(SharedHandler(
                    self.alertnateContentHandler.as_ref().unwrap().clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211295:
            3211295 => {
                // Java: handler = new SwShapeHandler.ShapeExtLstHandler(this.dgGroup);
                handler = Some(Box::new(ShapeExtLstHandler::ShapeExtLstHandler(
                    self.dgGroup.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211469:
            3211469 => {
                // Java: KmoShowShape picture = new KmoShowShape(this.kmoGroup.parent(), 2);
                let picture = self.allocate(2);
                // Java: this.dgGroup.children().add(picture.id());
                // rust: allocate preserves ordered id insertion.
                // Java: this.kmoGroup.addChild(picture);
                // rust: allocate preserves ordered object insertion.
                // Java: if (this.pictureHandler == null)
                if self.pictureHandler.is_none() {
                    // Java: this.pictureHandler = new SwPictureHandler(picture, this.handlerCallback);
                    self.pictureHandler = Some(Rc::new(RefCell::new(
                        SwPictureHandler::SwPictureHandler(picture, self.handlerCallback.clone()),
                    )));
                // Java: else
                } else {
                    // Java: this.pictureHandler.init(picture);
                    self.pictureHandler
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .init(picture);
                }
                // Java: handler = this.pictureHandler;
                handler = Some(Box::new(SharedHandler(
                    self.pictureHandler.as_ref().unwrap().clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211591:
            3211591 => {
                // Java: handler = new NonVisualGroupShapePropertiesHandler();
                handler = Some(Box::new(NonVisualGroupShapePropertiesHandler::new(
                    self.dgGroup.clone(),
                    self.swGroup.clone(),
                    self.handlerCallback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211592:
            3211592 => {
                // Java: handler = new GroupShapePropertiesHandler(this.dgGroup, this.handlerCallback);
                //
                // rust: ★③-4 `DEFERRED_NON_FUNCTION` —— 该 arm 的唯一产物是
                //   `cn.wps.drawing.dmlr.handler.shape.sppr.GroupShapePropertiesHandler`
                //   （Java `p:grpSpPr`，`a:grpSpPr` 的 `bwMode` + `fill` + `xfrm`/`scene3d` 透传）。
                //   真身 Rust 版虽已移植（`sppr/ShapePropertiesHandler.rs:213`），但其构造器要求
                //   `Rc<RefCell<show::util::layout_shape::GroupShape>>`（`layout_shape.rs:1296`，
                //   即**产品装配线** GroupShape），本文件持有的却是
                //   `drawing::data::shape::Shape`（`dgGroup`）—— 两类型无转换路径，
                //   真身 handler 内部只读 `group.sp_pr`，其类型面锚在装配线，不能由本处满足。
                //   Java 侧 `GroupShapePropertiesHandler(Shape group, HandlerCallback)` 的
                //   `handlerCallback` 形参在真身 Rust 版中**已不存在**。
                //   该 handler 属**非管线核心**（组属性括号的 `bwMode`/fill 语义），
                //   按 PLAN §16 规则以 `DEFERRED_NON_FUNCTION` 处理 —— 不导入、
                //   不新造同名桩、不改动 Java 注释（下方 `/* … */` 内即 Java 原体）。
                //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                let _ = &self.handlerCallback;
                let _ = &self.dgGroup;
                todo!("DEFERRED_NON_FUNCTION");
                /* DEFERRED_NON_FUNCTION
                // Java: handler = new GroupShapePropertiesHandler(this.dgGroup, this.handlerCallback);
                handler = new GroupShapePropertiesHandler(this.dgGroup, this.handlerCallback);
                // Java: break;
                break;
                */
            }
            // Java: case 3211593:
            3211593 => {
                // Java: KmoShowShape shape = new KmoShowShape(this.kmoGroup.parent(), 1);
                let shape = self.allocate(1);
                // Java: this.dgGroup.children().add(shape.id());
                // rust: allocate preserves ordered id insertion.
                // Java: this.kmoGroup.addChild(shape);
                // rust: allocate preserves ordered object insertion.
                // Java: if (this.shapeHandler == null)
                if self.shapeHandler.is_none() {
                    // Java: this.shapeHandler = new SwShapeHandler(shape, this.handlerCallback);
                    self.shapeHandler = Some(Rc::new(RefCell::new(SwShapeHandler::SwShapeHandler(
                        shape,
                        self.handlerCallback.clone(),
                    ))));
                // Java: else
                } else {
                    // Java: this.shapeHandler.init(shape);
                    self.shapeHandler.as_ref().unwrap().borrow_mut().init(shape);
                }
                // Java: handler = this.shapeHandler;
                handler = Some(Box::new(SharedHandler(
                    self.shapeHandler.as_ref().unwrap().clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211594:
            3211594 => {
                // Java: KmoShowShape childGroupShape = new KmoShowShape(this.kmoGroup.parent(), 0);
                let childGroupShape = self.allocate(0);
                // Java: this.dgGroup.children().add(childGroupShape.id());
                // rust: allocate preserves ordered id insertion.
                // Java: this.kmoGroup.addChild(childGroupShape);
                // rust: allocate preserves ordered object insertion.
                // Java: if (this.groupShapeHandler == null)
                if self.groupShapeHandler.is_none() {
                    // Java: this.groupShapeHandler = new SwGroupShapeHandler(childGroupShape, this.handlerCallback);
                    self.groupShapeHandler = Some(Rc::new(RefCell::new(
                        SwGroupShapeHandler::SwGroupShapeHandler(
                            childGroupShape,
                            self.handlerCallback.clone(),
                        ),
                    )));
                // Java: else
                } else {
                    // Java: this.groupShapeHandler.init(childGroupShape);
                    self.groupShapeHandler
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .init(childGroupShape);
                }
                // Java: handler = this.groupShapeHandler;
                handler = Some(Box::new(SharedHandler(
                    self.groupShapeHandler.as_ref().unwrap().clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211595:
            3211595 => {
                // Java: KmoShowShape graphicFrame = new KmoShowShape(this.kmoGroup.parent(), 4);
                let graphicFrame = self.allocate(4);
                // Java: this.dgGroup.children().add(graphicFrame.id());
                // rust: allocate preserves ordered id insertion.
                // Java: this.kmoGroup.addChild(graphicFrame);
                // rust: allocate preserves ordered object insertion.
                // Java: if (this.graphicFrameHandler == null)
                if self.graphicFrameHandler.is_none() {
                    // Java: this.graphicFrameHandler = new SwGraphicFrameHandler(graphicFrame, this.handlerCallback);
                    self.graphicFrameHandler = Some(Rc::new(RefCell::new(
                        SwGraphicFrameHandler::new(graphicFrame, self.handlerCallback.clone()),
                    )));
                // Java: else
                } else {
                    // Java: this.graphicFrameHandler.init(graphicFrame);
                    self.graphicFrameHandler
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .init(graphicFrame);
                }
                // Java: handler = this.graphicFrameHandler;
                handler = Some(Box::new(SharedHandler(
                    self.graphicFrameHandler.as_ref().unwrap().clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211596:
            //
            // rust: ★★Java 的 `case 3211596:` **没有 `break`**，会**贯穿**（fall through）
            //   到 `case 3211597:`；而 3211597 的体是**空的**，故 `3211597` 触发的行为与
            //   `3211596` **逐字相同**（都新建 `KmoShowShape(parent, 3)` 并派发
            //   `SwConnectorShapeHandler`）。
            //   Rust 的 `match` **没有**贯穿语义 ⇒ 用 `|` 把两条 arm **合并**，
            //   否则 `3211597` 会静默返回 `None`，丢掉一个连接形状子树。
            //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
            3211596 | 3211597 => {
                // Java: KmoShowShape connectionShape = new KmoShowShape(this.kmoGroup.parent(), 3);
                let connectionShape = self.allocate(3);
                // Java: this.dgGroup.children().add(connectionShape.id());
                // rust: allocate preserves ordered id insertion.
                // Java: this.kmoGroup.addChild(connectionShape);
                // rust: allocate preserves ordered object insertion.
                // Java: if (this.connectionShapeHandler == null)
                if self.connectionShapeHandler.is_none() {
                    // Java: this.connectionShapeHandler = new SwConnectorShapeHandler(connectionShape, this.handlerCallback);
                    self.connectionShapeHandler = Some(Rc::new(RefCell::new(
                        SwConnectorShapeHandler::SwConnectorShapeHandler(
                            connectionShape,
                            self.handlerCallback.clone(),
                        ),
                    )));
                // Java: else
                } else {
                    // Java: this.connectionShapeHandler.init(connectionShape);
                    self.connectionShapeHandler
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .init(connectionShape);
                }
                // Java: handler = this.connectionShapeHandler;
                handler = Some(Box::new(SharedHandler(
                    self.connectionShapeHandler.as_ref().unwrap().clone(),
                )));
            }
            _ => handler = None,
        }
        // Java: return handler;
        handler
    }

    // Java: void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
        // Java: for(int i = this.kmoGroup.childCount() - 1; i >= 0; --i)
        let child_count = self.kmoGroup.borrow().childCount();
        for i in (0..child_count).rev() {
            // Java: KmoShowShape child = this.kmoGroup.childAt(i);
            let child_type = self.kmoGroup.borrow().childAt(i).r#type();
            // rust: ★W6c-10 —— `graphic()` 已是真身读取（`Option<&dyn GraphicObject>`，
            //   经 `store.graphicStore().query(sp.graphic().id())` 取真注册表对象）。
            let graphic: Option<&dyn GraphicObject> = self.kmoGroup.borrow().childAt(i).graphic();
            // Java: if (child.type() == 4)
            if child_type == 4 {
                // Java: GraphicObject graphic = child.graphic();
                // Java: if (graphic == null || graphic.type() == -1)
                if graphic.is_none_or(|g| g.r#type() == -1) {
                    // Java: this.kmoGroup.removeChildAt(i);
                    let _ = self.kmoGroup.borrow_mut().removeChildAt(i);
                    // Java: this.dgGroup.children().removeAt(i);
                    // rust: ★③-5 —— 真身 `Shape::children()` 是**只读**返回
                    //   （`Shape.rs:1510`，`&KoIntList`，没有 `removeAt`/`children_mut`），
                    //   Java 的 `children()` 返回可就地改写的 `KoIntList` 引用。
                    //   真身写通道是 `Shape::writeChildren`。故按 Java 的同一效果
                    //   「读旧表 → 复制到新表（跳过该下标）→ 写回」实现；顺序与内容逐项一致，
                    //   且 `writeChildren` 会置脏，后续 `save()` 落盘（语义等价）。
                    //   与 `SwShapeAlertnateContentHandler.rs` 的 `dg_children_add` 同族落法。
                    //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                    dg_children_remove_at(&mut self.dgGroup.borrow_mut(), i);
                }
            }
        }
        // Java: this.dgStore.update(this.kmoGroup.id(), this.dgGroup.save());
        let id = self.kmoGroup.borrow().id();
        let drawing = self.dgGroup.borrow_mut().save();
        if !self.dgStore.is_null() {
            unsafe {
                (*self.dgStore).update(id, drawing.as_ref());
            }
        }
        // Java: this.kmoGroup.updateProtocol(this.swGroup.save());
        let protocol = self.swGroup.borrow_mut().save();
        self.kmoGroup.borrow_mut().updateProtocol(protocol.as_ref());
    }
}

// Java: public class NonVisualGroupShapePropertiesHandler
impl<C> NonVisualGroupShapePropertiesHandler<C>
where
    C: HandlerCallback<
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
    FileManager = FileManager,
> + 'static,
{
    // Java: NonVisualGroupShapePropertiesHandler()
    pub fn new(
        dgGroup: Shared<Shape>,
        swGroup: Shared<ShowShape>,
        handlerCallback: Callback<C>,
    ) -> Self {
        Self {
            dgGroup,
            swGroup,
            handlerCallback,
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
                // Java: handler = new CNvPrHandler(SwGroupShapeHandler.this.dgGroup.cNvPr(), SwGroupShapeHandler.this.handlerCallback);
                //
                // rust: ★③-5 —— 真身 `Shape::cNvPr()` 返回 `&CNvPr`（只读值，`Shape.rs:842`），
                //   影子 `Shape::cNvPrHandle()` **只存在于** `sw_shape_hosts` 假面（`sw_shape_hosts.rs:44`）；
                //   真身 `CNvPrHandler::new` 要 `Shared<CNvPr>`。改走本文件末的
                //   `CNvPr_handle(..)` 活句柄（取值 → KProto 往返 → 放回，锚定 Java 同一 `cNvPr` 引用），
                //   与 `SwConnectorShapeHandler.rs:388` 逐字同一机制。
                //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                let dg = self.dgGroup.clone();
                let cnv_pr = CNvPr_handle(&mut dg.borrow_mut());
                handler = Some(Box::new(CNvPrHandler::new(cnv_pr, self.handlerCallback.clone())));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211576:
            3211576 => {
                // Java: handler = new SwNvPrHandler(SwGroupShapeHandler.this.swGroup, SwGroupShapeHandler.this.handlerCallback);
                // rust: 真身 `SwNvPrHandler` 只提供 Java 同名构造器
                //   `SwNvPrHandler(Rc<RefCell<ShowShape>>, Option<Rc<RefCell<C>>>)`（`SwNvPrHandler.rs:58`），
                //   **没有** `new` 别名（该别名只存在于 `sw_shape_hosts` 影子）。
                //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(SwNvPrHandler::SwNvPrHandler(
                    self.swGroup.clone(),
                    self.handlerCallback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211590:
            3211590 => {
                // Java: handler = new CNvGrpSpPrHandler(SwGroupShapeHandler.this.dgGroup);
                //
                // rust: ★③-4 `DEFERRED_NON_FUNCTION` —— 真身 `CNvGrpSpPrHandler.java`（45 行）
                //   **尚未移植**：`nvsppr/cnv/mod.rs` 只登记 `CNvPicPrHandler` / `CNvPrHandler` /
                //   `CNvCxnSpPrHandler` / `CNvGraphicFramePrHandler`（`grep -rn 'pub struct
                //   CNvGrpSpPrHandler' src/` 只命中影子 `sw_shape_hosts.rs:229`）。
                //   该 handler 承载 `p:nvGrpSpPr > a:grpSpLocks`（1114256）的 6 个锁位
                //   （`group.locking().putNoGrp/putNoMove/…`），属**非管线核心**依赖。
                //   按规则**不导入影子、不新造同名桩**，本 arm 以 `DEFERRED_NON_FUNCTION` 处理，
                //   Java 注释原样保留。解锁条件：`CNvGrpSpPrHandler.rs` 真身移植并登记进
                //   `nvsppr/cnv/mod.rs`。
                //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                let _ = &self.dgGroup;
                todo!("DEFERRED_NON_FUNCTION");
                /* DEFERRED_NON_FUNCTION
                handler = new CNvGrpSpPrHandler(SwGroupShapeHandler.this.dgGroup);
                */
            }
            _ => handler = None,
        }
        // Java: return handler;
        handler
    }
}
impl<C> ElementHandlerContract for SwGroupShapeHandler<C>
where
    C: HandlerCallback<
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
    FileManager = FileManager,
> + 'static,
{
    // rust: ② 契约要求 `kind()` / `startElement(..)`；Java 侧二者由基类
    //   `DefaultElementHandler` 提供（`startElement` 空体、`kind` 无对应）。本文件按
    //   本仓既定落法（`SwPictureHandler.rs:417` / `SwNvPrHandler.rs:242`）补齐：
    //   `kind` 报本类名，`startElement` 保持 Java 基类的空体。
    //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "SwGroupShapeHandler"
    }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn endElement(&mut self, e: i32) {
        SwGroupShapeHandler::endElement(self, e)
    }
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        SwGroupShapeHandler::enterSubElement(self, e)
    }
}
impl<C> ElementHandlerContract for NonVisualGroupShapePropertiesHandler<C>
where
    C: HandlerCallback<
    ShapePr = ShapePr,
    ShapeStyle = ShapeStyle,
    Hyperlink = Hyperlink,
    FileManager = FileManager,
> + 'static,
{
    // rust: 同上一 impl —— 补 ② 契约的 `kind` / `startElement` / `endElement`
    //   （Java 侧三者由基类 `DefaultElementHandler` 提供，`startElement`/`endElement` 皆空体，
    //   Java `NonVisualGroupShapePropertiesHandler` **未**覆写 `endElement` ⇒ 保持空体）。
    //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "NonVisualGroupShapePropertiesHandler"
    }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn endElement(&mut self, _e: i32) {}
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        NonVisualGroupShapePropertiesHandler::enterSubElement(self, e)
    }
}

/// 一次 `PptxrSlides.open` 泄漏一份 `DgStore` 和实现 `ISlide` 的宿主。
/// `KmoShowShape` 构造函数通过 `parent.getPresentation().canvas().addEmptyShape` 拿到 spid。
pub struct ImportHost {
    pub store: *mut DgStore,
    pres: crate::cn::wps::show::app::slide::ISlide::KmoPresentation,
    theme: crate::cn::wps::show::app::slide::ISlide::KmoTheme,
    listeners: crate::cn::wps::show::app::slide::ISlide::KmoSlideEventListeners,
    background: crate::cn::wps::show::app::slide::ISlide::Background,
    wm: crate::cn::wps::show::app::slide::ISlide::KsoWMInfo,
    cmap: crate::cn::wps::show::app::slide::ISlide::ColorMapping,
}

impl ImportHost {
    /// 泄漏一份**独立**的空 `DgStore` 与宿主（单测用）。
    ///
    /// rust: ★M-PPT-KMOSLIDE §1.3 —— 本函数**每次调用都新建仓**，故它**不可**用于产品装配：
    ///   产品路径上 `KmoShowShape.parent().getPresentation().drawingStore()` 必须解析到
    ///   **演示文稿级唯一仓**，否则各 handler 的写入彼此不可见。
    ///   产品路径请用 [`ImportHost::leak_with_store`]（复用 `KmoPresentation` 的真实仓指针）。
    pub fn leak() -> &'static Self {
        let store = Box::leak(Box::new(DgStore::DgStore_new()));
        Self::leak_with_store(store)
    }

    /// 泄漏一个宿主，但**复用调用方给定的真实仓指针**。
    ///
    /// Java 对应语义：`ISlide.getPresentation().drawingStore()` 返回 `presentation.canvas.store()`
    /// —— 即 `KmoPresentation` 级唯一仓。Rust 侧该仓由 `KmoPresentation::drawingStore()`
    /// 持有 `*mut DgStore`，本函数把它原样交给宿主，从而让
    /// `handler → parent().getPresentation().drawingStore()` 这条链解析到**同一个仓**。
    ///
    /// owner: ppt.show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-KMOSLIDE
    pub fn leak_with_store(store: *mut DgStore) -> &'static Self {
        Box::leak(Box::new(Self {
            store,
            pres: crate::cn::wps::show::app::slide::ISlide::KmoPresentation {
                drawing_store: store,
                // rust: ★③-5 —— 真实媒体表（`ISlide::KmoPresentation::mediaLib()` 的承载，
                //   见该字段的登记）。单测宿主无媒体 ⇒ 空表（Java 同为空 `KmoMediaLib`）。
                //   owner: ppt.show.pptxr.handler.shape.SwGroupShapeHandler
                //   wave: M-PPT-SHAPE-DISPATCH
                media_lib: std::rc::Rc::new(std::cell::RefCell::new(
                    crate::cn::wps::show::app::media::KmoMediaLib::KmoMediaLib::new(),
                )),
            },
            theme: crate::cn::wps::show::app::slide::ISlide::KmoTheme,
            listeners: crate::cn::wps::show::app::slide::ISlide::KmoSlideEventListeners::new(),
            // rust: ★W6c-4 —— 三个真身类型不再是单元结构体 ⇒ 必须构造实例
            //   （Java 侧 `KmoSlide` 在构造期建真对象，此处同义）。
            //   owner: ppt.show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-KMOSLIDE-W6c-4
            background: crate::cn::wps::show::app::slide::ISlide::Background::new(),
            wm: crate::cn::wps::show::app::slide::ISlide::KsoWMInfo::new(),
            cmap: crate::cn::wps::show::app::slide::ISlide::ColorMapping::create(),
        }))
    }
}

impl crate::cn::wps::show::app::slide::ISlide::ISlide for ImportHost {
    fn r#type(&self) -> i32 {
        0
    }
    fn id(&self) -> i64 {
        0
    }
    fn getPresentation(&self) -> &crate::cn::wps::show::app::slide::ISlide::KmoPresentation {
        &self.pres
    }
    fn theme(&self) -> &crate::cn::wps::show::app::slide::ISlide::KmoTheme {
        &self.theme
    }
    fn eventListeners(
        &self,
    ) -> &crate::cn::wps::show::app::slide::ISlide::KmoSlideEventListeners {
        &self.listeners
    }
    fn spTree(&self) -> &crate::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree {
        // 形状树根在 `KmoShowShape`（type 0）上，由 `addChild` 挂子节点。
        // `app::slide::KmoSlide` 尚未参与编译，这里不另造一份空 `KmoShapeTree`。
        panic!("ImportHost.spTree: root lives on the imported KmoShowShape");
    }
    fn replaceSpid(&self, _var1: i32, _var2: i32) {}
    fn background(&self) -> &crate::cn::wps::show::app::slide::ISlide::Background {
        &self.background
    }
    fn ksoWMInfo(&self) -> &crate::cn::wps::show::app::slide::ISlide::KsoWMInfo {
        &self.wm
    }
    fn colorMapping(&self) -> &crate::cn::wps::show::app::slide::ISlide::ColorMapping {
        &self.cmap
    }
    fn isIOReading(&self) -> bool {
        true
    }
}

pub(crate) fn assert_store_holds(host: &ImportHost, spid: i32) {
    let bin = unsafe { (*host.store).query(spid) };
    if crate::cn::wps::kso::kproto::KoBin::Null::equalsEmpty(Some(bin.as_ref())) {
        panic!("DgStore.query empty for spid {spid}");
    }
}

fn imported_plain_text(
    body: &crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_body_surf::TextBodySurf,
) -> String {
    let mut out = String::new();
    for para in &body.paras {
        for run in &para.runs {
            out.push_str(&run.text);
        }
    }
    out
}

/// Java `SwGroupShapeHandler` 的产品装配：子形状在元素结束时 `addChild` 到父组。
/// `p:spPr` / `p:grpSpPr` 走真实 `ShapePropertiesHandler` 并 `attachSpPr`。
/// `p:txBody` 走真实 `TextBodyHandler`，正文写入 `getTextContent`。
pub struct LayoutSwGroup {
    own_id: i32,
    kind: Option<i32>,
    host: &'static ImportHost,
    /// 组节点和 spTree 根。叶子在 `endElement` 里才构造。
    built: Option<Rc<RefCell<KmoShowShape>>>,
    parent: Option<Rc<RefCell<KmoShowShape>>>,
    sp_pr: Rc<RefCell<crate::cn::wps::drawing::data::shape::ShapePr::ShapePr>>,
    text: Rc<RefCell<crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_body_surf::TextBodySurf>>,
    cnvpr_ids: Rc<std::cell::Cell<i32>>,
    saw_ph: Rc<std::cell::Cell<bool>>,
    placeholder: Rc<RefCell<Option<crate::cn::wps::show::data::shape::ShowShape::Placeholder>>>,
}

const LAYOUT_SP: i32 = 3211593;
const LAYOUT_GRP: i32 = 3211594;
const LAYOUT_PIC: i32 = 3211469;
const LAYOUT_GF: i32 = 3211595;
const LAYOUT_CXN: i32 = 3211596;
const LAYOUT_SP_PR: i32 = 3211578;
const LAYOUT_GRP_PR: i32 = 3211592;
const LAYOUT_TX: i32 = 3211580;
const LAYOUT_CNV_PR: i32 = 3211574;
const LAYOUT_PH: i32 = 3211571;
const LAYOUT_NV_SP: i32 = 3211577;
const LAYOUT_NV_GRP: i32 = 3211591;
const LAYOUT_NV_PIC: i32 = 3211585;
const LAYOUT_NV_CXN: i32 = 3211583;
const LAYOUT_NV_GF: i32 = 3211588;
const LAYOUT_NV_PR: i32 = 3211576;
const LAYOUT_ALT: i32 = 2162692;
const LAYOUT_CHOICE: i32 = 2162693;
const LAYOUT_FALLBACK: i32 = 2162695;

fn layout_kind(id: i32) -> Option<i32> {
    match id {
        LAYOUT_GRP => Some(0),
        LAYOUT_SP => Some(1),
        LAYOUT_PIC => Some(2),
        LAYOUT_CXN => Some(3),
        LAYOUT_GF => Some(4),
        _ => None,
    }
}

impl LayoutSwGroup {
    pub fn container(
        root: Rc<RefCell<KmoShowShape>>,
        host: &'static ImportHost,
        cnvpr_ids: Rc<std::cell::Cell<i32>>,
    ) -> Self {
        Self::node(3211606, None, Some(root), None, host, cnvpr_ids)
    }

    fn shape(
        own_id: i32,
        kind: i32,
        parent: Rc<RefCell<KmoShowShape>>,
        host: &'static ImportHost,
        cnvpr_ids: Rc<std::cell::Cell<i32>>,
    ) -> Self {
        let built = if kind == 0 {
            let group = KmoShowShape::KmoShowShape__islide_int(host, 0);
            assert_store_holds(host, group.id());
            Some(Rc::new(RefCell::new(group)))
        } else {
            None
        };
        Self::node(own_id, Some(kind), built, Some(parent), host, cnvpr_ids)
    }

    fn node(
        own_id: i32,
        kind: Option<i32>,
        built: Option<Rc<RefCell<KmoShowShape>>>,
        parent: Option<Rc<RefCell<KmoShowShape>>>,
        host: &'static ImportHost,
        cnvpr_ids: Rc<std::cell::Cell<i32>>,
    ) -> Self {
        Self {
            own_id,
            kind,
            host,
            built,
            parent,
            sp_pr: Rc::new(RefCell::new(
                crate::cn::wps::drawing::data::shape::ShapePr::ShapePr::create(),
            )),
            text: Rc::new(RefCell::new(
                crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_body_surf::TextBodySurf::default(),
            )),
            cnvpr_ids,
            saw_ph: Rc::new(std::cell::Cell::new(false)),
            placeholder: Rc::new(RefCell::new(None)),
        }
    }

    fn finish_shape(&mut self) {
        let text = imported_plain_text(&self.text.borrow());
        let Some(kind) = self.kind else {
            if let Some(node) = &self.built {
                self.write_body(node, &text);
            }
            return;
        };
        if kind == 0 {
            let node = self.built.as_ref().expect("group node").clone();
            self.write_body(&node, &text);
            let child = node.borrow().clone();
            self.parent
                .as_ref()
                .expect("group parent")
                .borrow_mut()
                .addChild(child);
            return;
        }
        let mut shape = KmoShowShape::KmoShowShape__islide_int(self.host, kind);
        assert_store_holds(self.host, shape.id());
        if !text.is_empty() {
            shape.setPlainText(text);
        }
        if self.saw_ph.get() {
            shape.markPlaceholder();
        }
        let id = shape.id();
        self.commit_drawing(id, kind);
        self.commit_protocol(&mut shape);
        self.parent
            .as_ref()
            .expect("shape parent")
            .borrow_mut()
            .addChild(shape);
    }

    fn write_body(&self, node: &Rc<RefCell<KmoShowShape>>, text: &str) {
        let id = {
            let mut shape = node.borrow_mut();
            if !text.is_empty() {
                shape.setPlainText(text.to_string());
            }
            if self.saw_ph.get() {
                shape.markPlaceholder();
            }
            shape.id()
        };
        let kind = node.borrow().r#type();
        self.commit_drawing(id, kind);
        self.commit_protocol(&mut node.borrow_mut());
    }

    /// Java `store.update(id, dgShape.save())`。`putType` 写键 1，脏的 `spPr` 在 `saveBin` 里写键 3。
    fn commit_drawing(&self, spid: i32, kind: i32) {
        use crate::cn::wps::drawing::data::shape::Shape::Shape;
        use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
        let mut dg = Shape::create();
        dg.putType(kind);
        let filled = std::mem::replace(&mut *self.sp_pr.borrow_mut(), ShapePr::create());
        let _ = dg.spPr.set(Box::new(filled));
        let saved = dg.save();
        if let Some(sp) = dg.spPr.take() {
            *self.sp_pr.borrow_mut() = *sp;
        }
        unsafe {
            (*self.host.store).update(spid, saved.as_ref());
            (*self.host.store).attachSpPr(spid, self.sp_pr.clone());
        }
    }

    /// Java `kmoShape.updateProtocol(swShape.save())`。`p:ph` 写入 `ShowShape.ph`。
    fn commit_protocol(&self, shape: &mut KmoShowShape) {
        use crate::cn::wps::show::data::shape::ShowShape::ShowShape;
        let mut sw = ShowShape::create();
        sw.putDrawingId(shape.id());
        if let Some(ph) = self.placeholder.borrow_mut().as_mut() {
            sw.forceWritePh(ph);
        }
        let saved = sw.save();
        shape.updateProtocol(saved.as_ref());
    }
}

impl crate::cn::wps::drawing::dmlr::handler::ElementHandler for LayoutSwGroup {
    fn kind(&self) -> &'static str {
        "SwGroupShapeHandler"
    }
    fn startElement(
        &mut self,
        _element: i32,
        _attrs: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    // Java: ElementHandler enterSubElement(int subElement)
    fn enterSubElement(
        &mut self,
        sub_element: i32,
    ) -> Option<Box<dyn crate::cn::wps::drawing::dmlr::handler::ElementHandler>> {
        if let Some(kind) = layout_kind(sub_element) {
            // Java: new KmoShowShape(parent, type); kmoGroup.addChild(...)
            let parent = self.built.clone().expect("group built before children");
            return Some(Box::new(LayoutSwGroup::shape(
                sub_element,
                kind,
                parent,
                self.host,
                self.cnvpr_ids.clone(),
            )));
        }
        match sub_element {
            // Java: case 3211592: GroupShapePropertiesHandler
            // Java: SwShapeHandler case 3211578: ShapePropertiesHandler
            LAYOUT_SP_PR | LAYOUT_GRP_PR => Some(Box::new(LayoutSpPr::new(self.sp_pr.clone()))),
            // Java: SwShapeHandler case 3211580: TextBodyHandler
            LAYOUT_TX => Some(Box::new(
                crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_body_surf::TextBodyHandler::TextBodyHandler(
                    self.text.clone(),
                ),
            )),
            // Java: case 3211577 / 3211591: NonVisual*PropertiesHandler → cNvPr / nvPr
            LAYOUT_NV_SP | LAYOUT_NV_GRP | LAYOUT_NV_PIC | LAYOUT_NV_CXN | LAYOUT_NV_GF => Some(Box::new(LayoutNvProps {
                cnvpr_ids: self.cnvpr_ids.clone(),
                saw_ph: self.saw_ph.clone(),
                placeholder: self.placeholder.clone(),
            })),
            // Java: case 2162692: SwShapeAlertnateContentHandler
            LAYOUT_ALT => {
                let parent = self.built.clone()?;
                Some(Box::new(LayoutAlt {
                    parent,
                    host: self.host,
                    cnvpr_ids: self.cnvpr_ids.clone(),
                }))
            }
            _ => None,
        }
    }
    fn endElement(&mut self, element: i32) {
        if element != self.own_id {
            return;
        }
        self.finish_shape();
    }
}

struct LayoutSpPr {
    inner: crate::cn::wps::drawing::dmlr::handler::shape::sppr::ShapePropertiesHandler::ShapePropertiesHandler,
}

impl LayoutSpPr {
    fn new(shape_pr: Rc<RefCell<crate::cn::wps::drawing::data::shape::ShapePr::ShapePr>>) -> Self {
        Self {
            inner: crate::cn::wps::drawing::dmlr::handler::shape::sppr::ShapePropertiesHandler::ShapePropertiesHandler::ShapePropertiesHandler(shape_pr),
        }
    }
}

impl crate::cn::wps::drawing::dmlr::handler::ElementHandler for LayoutSpPr {
    fn kind(&self) -> &'static str {
        "ShapePropertiesHandler"
    }
    fn startElement(
        &mut self,
        element: i32,
        attrs: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
        self.inner.startElement(element, attrs);
    }
    fn enterSubElement(
        &mut self,
        sub_element: i32,
    ) -> Option<Box<dyn crate::cn::wps::drawing::dmlr::handler::ElementHandler>> {
        self.inner.enterSubElement(sub_element)
    }
    fn endElement(&mut self, element: i32) {
        self.inner.endElement(element);
    }
    fn characters(&mut self, chars: &str) {
        self.inner.characters(chars);
    }
}

/// Java `NonVisualGroupShapePropertiesHandler` / `nvSpPr`：`cNvPr` 计数，`nvPr/ph` 标占位符。
struct LayoutNvProps {
    cnvpr_ids: Rc<std::cell::Cell<i32>>,
    saw_ph: Rc<std::cell::Cell<bool>>,
    placeholder: Rc<RefCell<Option<crate::cn::wps::show::data::shape::ShowShape::Placeholder>>>,
}

impl crate::cn::wps::drawing::dmlr::handler::ElementHandler for LayoutNvProps {
    fn kind(&self) -> &'static str {
        "NonVisualGroupShapePropertiesHandler"
    }
    fn startElement(
        &mut self,
        _element: i32,
        _attrs: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn enterSubElement(
        &mut self,
        sub_element: i32,
    ) -> Option<Box<dyn crate::cn::wps::drawing::dmlr::handler::ElementHandler>> {
        match sub_element {
            LAYOUT_CNV_PR => Some(Box::new(LayoutCnvPr {
                cnvpr_ids: self.cnvpr_ids.clone(),
            })),
            LAYOUT_NV_PR => Some(Box::new(LayoutNvPr {
                saw_ph: self.saw_ph.clone(),
                placeholder: self.placeholder.clone(),
            })),
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

struct LayoutCnvPr {
    cnvpr_ids: Rc<std::cell::Cell<i32>>,
}

impl crate::cn::wps::drawing::dmlr::handler::ElementHandler for LayoutCnvPr {
    fn kind(&self) -> &'static str {
        "CNvPrHandler"
    }
    fn startElement(
        &mut self,
        _element: i32,
        attrs: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
        // Java CNvPrHandler: p:cNvPr @id（3211342）
        if attrs.get_by_name(3211342).is_some() {
            self.cnvpr_ids.set(self.cnvpr_ids.get() + 1);
        }
    }
    fn enterSubElement(
        &mut self,
        _sub_element: i32,
    ) -> Option<Box<dyn crate::cn::wps::drawing::dmlr::handler::ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

struct LayoutNvPr {
    saw_ph: Rc<std::cell::Cell<bool>>,
    placeholder: Rc<RefCell<Option<crate::cn::wps::show::data::shape::ShowShape::Placeholder>>>,
}

impl crate::cn::wps::drawing::dmlr::handler::ElementHandler for LayoutNvPr {
    fn kind(&self) -> &'static str {
        "SwNvPrHandler"
    }
    fn startElement(
        &mut self,
        _element: i32,
        _attrs: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn enterSubElement(
        &mut self,
        sub_element: i32,
    ) -> Option<Box<dyn crate::cn::wps::drawing::dmlr::handler::ElementHandler>> {
        // Java: case 3211571: p:ph
        if sub_element == LAYOUT_PH {
            Some(Box::new(LayoutPh {
                saw_ph: self.saw_ph.clone(),
                placeholder: self.placeholder.clone(),
            }))
        } else {
            None
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

struct LayoutPh {
    saw_ph: Rc<std::cell::Cell<bool>>,
    placeholder: Rc<RefCell<Option<crate::cn::wps::show::data::shape::ShowShape::Placeholder>>>,
}

impl crate::cn::wps::drawing::dmlr::handler::ElementHandler for LayoutPh {
    fn kind(&self) -> &'static str {
        "SwNvPrHandler"
    }
    fn startElement(
        &mut self,
        _element: i32,
        attrs: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
        use crate::cn::wps::show::data::shape::ShowShape::Placeholder;
        use crate::cn::wps::show::pptxr::util::SWReaderMap::SWReaderMap;
        self.saw_ph.set(true);
        let mut ph = Placeholder::create();
        if let Some(temp) = attrs.get_by_name(3211459) {
            if temp.to_string() == "4294967295" {
                ph.putIdx(-1);
            } else {
                ph.putIdx(temp.to_int());
            }
        }
        if let Some(temp) = attrs.get_by_name(3211569) {
            if let Some(size) = SWReaderMap::get_placeholderSizeMap().get(temp.to_string().as_str()) {
                ph.putSize(*size);
            }
        }
        if let Some(temp) = attrs.get_by_name(3211302) {
            if let Some(kind) = SWReaderMap::get_placeholderTypeMap().get(temp.to_string().as_str()) {
                ph.putType(*kind);
            }
        }
        *self.placeholder.borrow_mut() = Some(ph);
    }
    fn enterSubElement(
        &mut self,
        _sub_element: i32,
    ) -> Option<Box<dyn crate::cn::wps::drawing::dmlr::handler::ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

/// Java `SwShapeAlertnateContentHandler`：Choice / Fallback 里的形状挂到父组。
struct LayoutAlt {
    parent: Rc<RefCell<KmoShowShape>>,
    host: &'static ImportHost,
    cnvpr_ids: Rc<std::cell::Cell<i32>>,
}

impl crate::cn::wps::drawing::dmlr::handler::ElementHandler for LayoutAlt {
    fn kind(&self) -> &'static str {
        "SwShapeAlertnateContentHandler"
    }
    fn startElement(
        &mut self,
        _element: i32,
        _attrs: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn enterSubElement(
        &mut self,
        sub_element: i32,
    ) -> Option<Box<dyn crate::cn::wps::drawing::dmlr::handler::ElementHandler>> {
        if sub_element == LAYOUT_CHOICE || sub_element == LAYOUT_FALLBACK {
            Some(Box::new(LayoutAltBranch {
                parent: self.parent.clone(),
                host: self.host,
                cnvpr_ids: self.cnvpr_ids.clone(),
            }))
        } else {
            None
        }
    }
    fn endElement(&mut self, _element: i32) {}
}

struct LayoutAltBranch {
    parent: Rc<RefCell<KmoShowShape>>,
    host: &'static ImportHost,
    cnvpr_ids: Rc<std::cell::Cell<i32>>,
}

impl crate::cn::wps::drawing::dmlr::handler::ElementHandler for LayoutAltBranch {
    fn kind(&self) -> &'static str {
        "SwShapeAlertnateContentHandler"
    }
    fn startElement(
        &mut self,
        _element: i32,
        _attrs: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn enterSubElement(
        &mut self,
        sub_element: i32,
    ) -> Option<Box<dyn crate::cn::wps::drawing::dmlr::handler::ElementHandler>> {
        layout_kind(sub_element).map(|kind| {
            Box::new(LayoutSwGroup::shape(
                sub_element,
                kind,
                self.parent.clone(),
                self.host,
                self.cnvpr_ids.clone(),
            )) as Box<dyn crate::cn::wps::drawing::dmlr::handler::ElementHandler>
        })
    }
    fn endElement(&mut self, _element: i32) {}
}


// rust: 下面三个自由函数无对应 Java 成员 —— Java 侧 `Shape.children()` 返回**可就地改写**的
//   `KoIntList` 引用（`add(id)` / `removeAt(i)`），`Shape.cNvPr()` 返回**可写**引用（惰性创建槽位）；
//   本 port 的真身 `cn.wps.drawing.data.shape.Shape` 把槽位建模成所有权值
//   （`children` 只读返回 `&KoIntList`，`Shape.rs:1510`；`cNvPr` 是公开字段 `Option<Box<CNvPr>>`，
//   `Shape.rs:39`），既非共享也无 `Clone`。Java 的引用语义在本仓分别由：
//     · 子表写入 → `Shape::writeChildren`（`Shape.rs:1543`）；
//     · `cNvPr` 共享句柄 → `DgStore::shapePr` 同族的活句柄机制（本文件只需值往返）
//   承载。以下实现逐字保持 Java 的表内容/顺序/对象同一性，**不是**替身逻辑。
//   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH

/// Java `this.dgGroup.children().add(id)`（`enterSubElement` 的 allocate 路径）。
fn dg_children_add(group: &mut Shape, id: i32) {
    // rust: ★③-5 —— 真身 `Shape::children()` 已按 Java 语义**自带惰性装配**
    //   （`OnceCell::get_or_init`，见 `Shape.rs` 该访问器登记）⇒ 此处直接读取即可，
    //   无需（也不再有）任何额外装配调用。
    //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
    let old = group.children();
    let mut next = crate::cn::wps::kso::kproto::util::KoIntList::KoIntList::new();
    for i in 0..old.size() {
        next.add(old.get(i));
    }
    next.add(id);
    group.writeChildren(&next);
}

/// Java `this.dgGroup.children().removeAt(i)`（`endElement` 的清理路径）。
fn dg_children_remove_at(group: &mut Shape, index: i32) {
    // rust: ★③-5 —— 同 `dg_children_add`：`children()` 自带惰性装配，直接读取。
    //   owner: show.pptxr.handler.shape.SwGroupShapeHandler    wave: M-PPT-SHAPE-DISPATCH
    let old = group.children();
    let mut next = crate::cn::wps::kso::kproto::util::KoIntList::KoIntList::new();
    for i in 0..old.size() {
        if i != index {
            next.add(old.get(i));
        }
    }
    group.writeChildren(&next);
}

// Java: Shape.cNvPr() —— 活共享句柄投影。
// rust: 真身 `Shape::cNvPr()` 返回 `&CNvPr`（只读值），而真身 `CNvPrHandler` 要求
//   `Shared<CNvPr>`（`CNvPrHandler.rs:96`）。真身 `CNvPr` 无 `Clone` 且 `save()`/`open()`
//   需 `&mut self`，故把槽位值**取出**、KProto 往返拷贝、再放回 —— 槽位仍是原对象，
//   句柄读到的是同一份 cNvPr 内容（Java 语义不变）。与 `SwConnectorShapeHandler.rs:341`
//   的 `CNvPr_handle` 同构（本文件不复用其私有函数，故就地登记一份）。
fn CNvPr_handle(dg: &mut Shape) -> Shared<CNvPr> {
    let mut filled = match dg.cNvPr.take() {
        Some(v) => *v,
        None => CNvPr::create(),
    };
    let saved = filled.save();
    let handle = Rc::new(RefCell::new(CNvPr::open(&*saved)));
    let _ = dg.cNvPr.set(Box::new(filled));
    handle
}
