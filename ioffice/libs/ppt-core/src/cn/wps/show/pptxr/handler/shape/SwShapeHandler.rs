#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code)]

// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.data.shape.Shape
// rust: ★③-5 —— 改用**真身**模型类型（原为 `sw_shape_hosts` 同名空壳，形态 F）。
//   `ShapeExt` 同样改指真身 `drawing::data::shape::ShapeExt`（该模块已登记；模板生成的
//   `shapepr_hosts` 影子与本文件无关）。`ShapePr`/`ShapeStyle` 按 Java 真实包路径导入
//   （`data::shape::ShapePr` 是同包独立类；`ShapeStyle` 在 `data::style`）。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::shape::Shape::Shape;
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
// Java: import cn.wps.drawing.data.shape.ShapeExt
use crate::cn::wps::drawing::data::shape::ShapeExt::ShapeExt;
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvPrHandler
// rust: ★③-5 —— 影子导入换成**真身**（`sw_shape_hosts` 的同名类型不实现 ② 契约
//   `dmlr::handler::ElementHandler` ⇒ `Box::new(..)` 处 E0277）。真身路径与
//   `SwPictureHandler.rs` / `SwConnectorShapeHandler.rs` 的既有落法一致。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::dmlr::handler::shape::nvsppr::cnv::CNvPrHandler::CNvPrHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvSpPrHandler
// rust: no-stmt —— 真身 `cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvSpPrHandler`
//   **不存在**（`grep -rn 'pub struct CNvSpPrHandler' src/` 只命中影子
//   `sw_shape_hosts.rs:170`；Java 侧 `CNvSpPrHandler.java` 在
//   `koffice/et/core/cn/wps/moffice/drawing/dmlr/handler/shape/nvsppr/cnv/`，
//   `nvsppr/cnv/mod.rs` 只登记了 CNvCxnSpPr/CNvGraphicFramePr/CNvPicPr/CNvPr）。
//   按「真身不存在时不得造假壳」的纪律，该分支落 `todo!("DEFERRED_NON_FUNCTION")`
//   （见 `enterSubElement` case 3211575）。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.ShapePropertiesHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::ShapePropertiesHandler::ShapePropertiesHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.style.ShapeStyleHandler
use crate::cn::wps::drawing::dmlr::handler::shape::style::ShapeStyleHandler::ShapeStyleHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.TextBodyHandler
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::TextBodyHandler::TextBodyHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: ★③-5 —— 契约改用 **② 规范引擎契约** `drawing::dmlr::handler::ElementHandler`
//   （`&mut self` + `kind()` + `&XmlRoAttrList`），即
//   `UnifiedXmlReader::read__stream_elementhandler`（真 S2 引擎）唯一接受的契约（PLAN §26.1）。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler as ElementHandlerContract;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use crate::cn::wps::show::app::shape::kmo_show_shape_hosts::KmoProtocol;
// Java: import cn.wps.show.clipxr.gvml.handler.TxSpHandler
// rust: no-stmt —— 真身 `cn.wps.show.clipxr.gvml.handler.TxSpHandler` **不存在**
//   （`src/cn/wps/show/clipxr/` 下只有 `ppt14/slides/Ppt14rSlides.rs`；Java 侧该类在
//   `koffice/presentation/core/cn/wps/show/clipxr/gvml/handler/TxSpHandler.java`，
//   全文 21 行，只把 `TextBodyHandler` 透传出去）。属**非管线核心**依赖。
//   按纪律该分支落 `todo!("DEFERRED_NON_FUNCTION")`（见 `enterSubElement` case 1114276）。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
// Java: import cn.wps.show.data.shape.ShowShape
// rust: ★③-5 —— 影子 `sw_shape_hosts::ShowShape::open(Box<dyn KoBin>) -> Shared<Self>`
//   换成真身 `show::data::shape::ShowShape`：`open(&dyn KoBinTrait) -> ShowShape`
//   （值，`ShowShape.rs:96`），`save(&mut self) -> Box<dyn KoBinTrait>`（`ShowShape.rs:484`）。
//   与 `SwPictureHandler.rs:57` / `SwConnectorShapeHandler.rs:31` 的既有落法一致。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::show::data::shape::ShowShape::ShowShape;
// Java: import java.util.ArrayList
// rust: Vec preserves Java ArrayList insertion order.
// Java: import java.util.List
// rust: Vec is the owned Rust list representation.

// Java: (无对应 import) —— 真身 `SwNvPrHandler` 已登记（`show::pptxr::handler::shape`），
//   影子 `sw_shape_hosts::SwNvPrHandler` 不实现 ② 契约 ⇒ 一并改指真身。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::show::pptxr::handler::shape::SwNvPrHandler::SwNvPrHandler;
// Java: (无对应 import) —— Java 的 `XmlRoAttr` 由 `XmlRoAttrList` 承载；Rust 侧的
//   `XmlRoAttrList::get_by_name` 返回**叶子** `&XmlRoAttrLeaf`（`XmlRoAttrList.rs:136`），
//   故局部变量的显式类型需命名该叶子路径（同 `SwShapeAlertnateContentHandler.rs:53`）。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrLeaf::XmlRoAttrLeaf;
// rust: 本行无对应 Java import —— `extLst` 槽位的真身类型（`Shape.rs:47`）；
//   `ShapeExtLstHandler::endElement` 的 take/put-back 需要命名它。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::shape::Shape::ShapeExtList;
// rust: 本行无对应 Java import —— 真身 `CNvPrHandler::new` 的 `C` 带
//   `HandlerCallback<Hyperlink = Hyperlink>` 约束（`CNvPrHandler.rs:64`），
//   Java 侧该关联类型由 `CNvPrHandler` 的泛型实参隐式满足。故须把本类的 `C`
//   也固定到真身 `Hyperlink`（同 `SwPictureHandler.rs:74` /
//   `SwConnectorShapeHandler.rs:41` 的既有落法）。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr as DataShapePr;
use std::cell::RefCell;
use std::rc::Rc;

type Shared<T> = Rc<RefCell<T>>;
type Callback<C> = Option<Shared<C>>;

// Java: public class SwShapeHandler extends DefaultElementHandler
pub struct SwShapeHandler<C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink>> {
    // Java field: DgStore store
    store: *mut DgStore,
    // Java field: ShowShape swShape
    pub(crate) swShape: Shared<ShowShape>,
    // Java field: Shape dgShape
    pub dgShape: Shared<Shape>,
    // Java field: KmoShowShape kmoShape
    kmoShape: Shared<KmoShowShape>,
    // Java field: HandlerCallback handlerCallback
    pub(crate) handlerCallback: Callback<C>,
}

// Java: public class NonVisualShapePropertiesHandler
pub struct NonVisualShapePropertiesHandler<
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink>,
> {
    dgShape: Shared<Shape>,
    swShape: Shared<ShowShape>,
    handlerCallback: Callback<C>,
}

// Java: public class ShapeExtLstHandler
pub struct ShapeExtLstHandler {
    // Java field: ShapeExtHandler extHandler
    pub(crate) extHandler: Option<ShapeExtHandler>,
    // Java field: List<ShapeExt> extLst
    pub(crate) extLst: Shared<Vec<Shared<ShapeExt>>>,
    // Java field: Shape shape
    pub(crate) shape: Shared<Shape>,
}

// Java: public class ShapeExtHandler
pub struct ShapeExtHandler {
    // Java field: DefaultElementHandler handler
    pub(crate) handler: Option<Box<dyn ElementHandler>>,
    // Java field: ShapeExt shapeExt
    pub(crate) shapeExt: Option<Shared<ShapeExt>>,
    extLst: Shared<Vec<Shared<ShapeExt>>>,
}

// Java: public class ModularGroupExtHandler
pub struct ModularGroupExtHandler {
    // Java field: ShapeExt shapeExt
    pub(crate) shapeExt: Option<Shared<ShapeExt>>,
}

// Java: public class SwShapeHandler extends DefaultElementHandler
impl<C> SwShapeHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // Java: SwShapeHandler(KmoShowShape shape, HandlerCallback handlerCallback)
    pub fn SwShapeHandler(shape: Shared<KmoShowShape>, handlerCallback: Callback<C>) -> Self {
        // Java: this.handlerCallback = handlerCallback;
        // rust: assigned in the aggregate below.
        // Java: this.store = shape.parent().getPresentation().drawingStore();
        let store = match shape.borrow().parent() {
            Some(p) => unsafe { (*p).getPresentation().drawingStore() },
            None => std::ptr::null_mut(),
        };
        // Java: this.init(shape);
        let mut this = Self::from_shape(shape, handlerCallback, store);
        let current = this.kmoShape.clone();
        this.init(current);
        this
    }

    fn from_shape(
        shape: Shared<KmoShowShape>,
        handlerCallback: Callback<C>,
        store: *mut DgStore,
    ) -> Self {
        // rust: ★③-5 —— 真身 `ShowShape::open` / `Shape::open` 取 `&dyn KoBinTrait`
        //   且**按值返回**（影子返回 `Shared<..>`）。故先把 `protocol()` /
        //   `drawingShape()` 的 `Box<dyn KoBin>` 绑到局部，再传 `&*local`，
        //   最后用 `Rc::new(RefCell::new(..))` 包成 Java 的引用语义。
        //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        // Java: this.swShape = ShowShape.open(shape.protocol());
        let protocol_bin = shape.borrow().protocol();
        let swShape = Rc::new(RefCell::new(ShowShape::open(&*protocol_bin)));
        // Java: this.dgShape = Shape.open(shape.drawingShape());
        let drawing_bin = shape.borrow().drawingShape();
        let dgShape = Rc::new(RefCell::new(Shape::open(&*drawing_bin)));
        Self {
            store,
            swShape,
            dgShape,
            kmoShape: shape,
            handlerCallback,
        }
    }

    // Java: void init(KmoShowShape shape)
    pub fn init(&mut self, shape: Shared<KmoShowShape>) {
        // Java: this.kmoShape = shape;
        self.kmoShape = shape;
        // Java: this.swShape = ShowShape.open(shape.protocol());
        // rust: 同 `from_shape` —— 真身按值返回 + `&dyn KoBinTrait` 形参。
        //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        let protocol_bin = self.kmoShape.borrow().protocol();
        self.swShape = Rc::new(RefCell::new(ShowShape::open(&*protocol_bin)));
        // Java: this.dgShape = Shape.open(shape.drawingShape());
        let drawing_bin = self.kmoShape.borrow().drawingShape();
        self.dgShape = Rc::new(RefCell::new(Shape::open(&*drawing_bin)));
        // Java: this.dgShape.putType(1);
        self.dgShape.borrow_mut().putType(1);
        // Java: this.dgShape.cNvPr().putId(shape.id());
        //
        // rust: 真身 `Shape::cNvPr()` 取 `&self` 并返回 `&CNvPr`（只读，`Shape.rs:842`），
        //   而 `CNvPr::putId` 需 `&mut self` ⇒ 直接链式调用会 E0596。故按 Java `cNvPr()`
        //   的**惰性创建语义**就地取用/创建槽位再写 `id`（同一落法见
        //   `SwPictureHandler.rs:189-205` / `SwConnectorShapeHandler.rs:149-158`）。
        //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        let id = self.kmoShape.borrow().id();
        {
            use crate::cn::wps::drawing::data::shape::CNvPr::CNvPr;
            let mut dg = self.dgShape.borrow_mut();
            if dg.cNvPr.get().is_none() {
                let _ = dg.cNvPr.set(Box::new(CNvPr::create()));
            }
            if let Some(cnv_pr) = dg.cNvPr.get_mut() {
                cnv_pr.putId(id);
            }
        }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp = null;
        // rust: 真身 `XmlRoAttrList::get_by_name` 返回 `Option<&XmlRoAttrLeaf>`（叶子的
        //   `toString()` 即 Java `XmlRoAttr.toString()`）⇒ 局部类型改叶子。
        //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        let mut temp: Option<&XmlRoAttrLeaf> = None;
        // Java: if ((temp = attrsNotCopy.getByName(3211581)) != null)
        temp = attrsNotCopy.get_by_name(3211581);
        if let Some(temp) = temp {
            // Java: this.swShape.putUseBgFill(temp.toBoolean());
            self.swShape.borrow_mut().putUseBgFill(temp.to_boolean());
        }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114276:
            1114276 => {
                // Java: if (null != this.handlerCallback && this.handlerCallback.isGVml())
                if self
                    .handlerCallback
                    .as_ref()
                    .is_some_and(|c| c.borrow().isGVml())
                {
                    // Java: handler = new TxSpHandler(this.dgShape.textShape().textBody(), this.handlerCallback);
                    //
                    // rust: DEFERRED_NON_FUNCTION —— 真身 `TxSpHandler` 不存在
                    //   （`cn.wps.show.clipxr.gvml.handler` 在 ppt-core 内未登记；Java 类
                    //   见 `koffice/presentation/core/cn/wps/show/clipxr/gvml/handler/TxSpHandler.java`，
                    //   全文 21 行、只做 `TextBodyHandler` 透传）。按「不得制造同名假壳」
                    //   的纪律在此落 defer；该分支仅在 `handlerCallback.isGVml()` 时进入，
                    //   属 **GVML 兼容导入**旁路，非管线核心（主流 OOXML 形状解析走
                    //   case 3211580 的真身 `TextBodyHandler`）。
                    //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                    //   blocking type: cn.wps.show.clipxr.gvml.handler.TxSpHandler
                    todo!("DEFERRED_NON_FUNCTION")
                }
                // Java: else
                else {
                    // Java: handler = null;
                    handler = None;
                }
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211295:
            3211295 => {
                // Java: handler = new ShapeExtLstHandler(this.dgShape);
                handler = Some(Box::new(ShapeExtLstHandler::ShapeExtLstHandler(
                    self.dgShape.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211577:
            3211577 => {
                // Java: handler = new NonVisualShapePropertiesHandler();
                handler = Some(Box::new(
                    NonVisualShapePropertiesHandler::NonVisualShapePropertiesHandler(
                        self.dgShape.clone(),
                        self.swShape.clone(),
                        self.handlerCallback.clone(),
                    ),
                ));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211578:
            3211578 => {
                // Java: handler = new ShapePropertiesHandler(this.dgShape.spPr(), this.handlerCallback);
                //
                // rust: 真身 `ShapePropertiesHandler` 只取一个形参
                //   `ShapePropertiesHandler(Rc<RefCell<ShapePr>>)`（`ShapePropertiesHandler.rs:65`；
                //   第二形参 `handlerCallback` 在真身侧改为内部 `FillNotifyRef` ⇒ 不传）。
                //   Java 的 `Shape::spPr()` 返回**引用**；本 port 真身把它建模为
                //   `Option<Box<ShapePr>>`（所有权值、无 `Clone`），无法别名成 `Rc<RefCell<_>>`，
                //   故经 `ShapePr_handle(..)`（文件末自由函数，与
                //   `SwPictureHandler.rs:667` / `SwConnectorShapeHandler.rs:315` 同构的
                //   KProto 往返惯用法）交出共享句柄，锚定 Java 的同一 `spPr` 引用。
                //   真身构造器名是 Java 同名 `ShapePropertiesHandler(..)`，**无** `new` 别名。
                //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(
                    ShapePropertiesHandler::ShapePropertiesHandler(ShapePr_handle(
                        &self.dgShape.borrow(),
                    )),
                ));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211579:
            3211579 => {
                // Java: handler = new ShapeStyleHandler(this.dgShape.style(), this.handlerCallback);
                //
                // rust: 真身构造器 `ShapeStyleHandler(Rc<RefCell<ShapeStyle>>, Callback<C>)`
                //   （`ShapeStyleHandler.rs:56`），**无** `new` 别名。真身 `Shape.style` 是
                //   `Option<Box<ShapeStyle>>`（`Shape.rs:43`，非共享且无 `Clone`）⇒ 按 Java 的
                //   可空语义：`hasStyle()` 为真才建处理器，否则保持 `null`（⇒ `None`）。
                //   句柄由 `ShapeStyle_handle(..)` 交出（KProto 往返，见文件末登记）。
                //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                let dg = self.dgShape.clone();
                let shape = dg.borrow();
                handler = if shape.hasStyle() {
                    Some(Box::new(ShapeStyleHandler::ShapeStyleHandler(
                        ShapeStyle_handle(&shape),
                        self.handlerCallback.clone(),
                    )))
                } else {
                    // Java: (handler 保持 null)
                    None
                };
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211580:
            3211580 => {
                // Java: handler = new TextBodyHandler(this.dgShape.textShape().textBody(), this.handlerCallback);
                //
                // rust: 真身 `TextBodyHandler` 的构造器是
                //   `TextBodyHandler(Rc<RefCell<TextBody>>, Option<Rc<RefCell<HandlerCallback>>>)`
                //   （`TextBodyHandler.rs:56`），**无** `new` 别名，且第一形参要求
                //   `Rc<RefCell<TextBody>>`（影子要 `TextBodyHost`）。真身 `Shape::textShape()`
                //   返回 `&TextShape`、`TextShape::textBody()` 返回 `&TextBody`（`TextShape.rs:379`）
                //   —— 都是借用，无 `Clone`，故经 `TextBody_handle(..)`（KProto 往返，
                //   见文件末自由函数的登记）交出共享句柄。
                //   ⚠ 真身第二形参是 `Option<Rc<RefCell<HandlerCallback>>>`（**具体**类型，
                //     不是泛型 `C`）；Java 的 `handlerCallback` 是 `HandlerCallback` 引用，
                //     真身的形态即其对应物。本类的 `Callback<C>` 无法与它统一（`C: HandlerCallback`
                //     是泛型参数、非 `HandlerCallback` 本身），故按真身契约交 `None`
                //     （真身 `TextBodyHandler` 的 `handlerCallback` 槽仅作回调透传，
                //      `enterSubElement` 内以 `callback_state(..)` 承接，`None` 即 Java 的 null 回调）。
                //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                let text_body = TextBody_handle(&mut self.dgShape.borrow_mut());
                handler = Some(Box::new(TextBodyHandler::TextBodyHandler(
                    text_body,
                    None,
                )));
            }
            _ => handler = None,
        }
        // Java: return handler;
        handler
    }

    // Java: void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
        // Java: if (null != this.handlerCallback && this.handlerCallback.isGVml())
        if self
            .handlerCallback
            .as_ref()
            .is_some_and(|c| c.borrow().isGVml())
        {
            // Java: this.handlerCallback.onConvertStyle2ShapeProps(this.dgShape.style(), this.dgShape.spPr());
            //
            // rust: 真身 `Shape::style()` 返回 `&ShapeStyle`（`Shape.rs:986`，**不是**
            //   `Option<ShapeStyle>` 的影子形态），而契约形参是
            //   `Option<&Self::ShapeStyle>`（`HandlerCallback.rs:215`）⇒ 直接 `Some(&..)`。
            //   `ShapePr` 侧同理：Java 的第二形参是引用，Rust 契约要 `Option<&mut ShapePr>`，
            //   真身 `Shape::spPr()` 返回 `&ShapePr`（`Shape.rs:916`）—— 为满足 `&mut`，
            //   经 `ShapePr_handle(..)` 取共享句柄再 `borrow_mut()` 投影
            //   （同 `SwPictureHandler.rs:338` 的既有落法）。
            //   ⚠ `onConvertStyle2ShapeProps` 在该投影上的写入不回流模型（真身尚无可变
            //     共享面），属已登记的模型面偏差，非本波修复范围。
            //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
            if let Some(callback) = &self.handlerCallback {
                let sp_pr = {
                    let shape = self.dgShape.borrow();
                    ShapePr_handle(&shape)
                };
                let shape = self.dgShape.borrow();
                callback.borrow_mut().onConvertStyle2ShapeProps(
                    Some(shape.style()),
                    Some(&mut sp_pr.borrow_mut()),
                );
            }
            // Java: this.dgShape.deleteStyle();
            self.dgShape.borrow_mut().deleteStyle();
        }
        // Java: this.store.update(this.kmoShape.id(), this.dgShape.save());
        // rust: 真身 `Shape::save()` 返回 `Box<dyn KoBin>`（`Shape.rs:688`），而
        //   `DgStore::update` 取 `&dyn KoBin`（`DgStore.rs:104`）⇒ 绑局部后传 `&*local`。
        //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        let id = self.kmoShape.borrow().id();
        let saved_shape = self.dgShape.borrow_mut().save();
        if !self.store.is_null() {
            unsafe { (*self.store).update(id, &*saved_shape); }
        }
        // Java: this.kmoShape.updateProtocol(this.swShape.save());
        // rust: 同上 —— 真身 `ShowShape::save()` 返回 `Box<dyn KoBin>`，`updateProtocol`
        //   取 `&dyn KoBin`（`kmo_show_shape_hosts.rs:233`）。
        let saved_protocol = self.swShape.borrow_mut().save();
        self.kmoShape.borrow_mut().updateProtocol(&*saved_protocol);
    }

    /// Persist the data ShapePr written by Java `ShapePropertiesHandler(this.dgShape.spPr())`.
    pub(crate) fn commit_data_sp_pr(&mut self, pr: Rc<RefCell<DataShapePr>>) {
        let id = self.kmoShape.borrow().id();
        if !self.store.is_null() {
            unsafe {
                (*self.store).attachSpPr(id, pr);
            }
        }
        self.endElement(3211578);
    }
}

// Java: public class NonVisualShapePropertiesHandler
impl<C> NonVisualShapePropertiesHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // Java: NonVisualShapePropertiesHandler()
    pub fn NonVisualShapePropertiesHandler(
        dgShape: Shared<Shape>,
        swShape: Shared<ShowShape>,
        handlerCallback: Callback<C>,
    ) -> Self {
        Self {
            dgShape,
            swShape,
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
                // Java: handler = new CNvPrHandler(SwShapeHandler.this.dgShape.cNvPr(), SwShapeHandler.this.handlerCallback);
                //
                // rust: 真身 `CNvPrHandler::new(Shared<CNvPr>, Callback<C>)`
                //   （`CNvPrHandler.rs:102`；其 `Hyperlink` 槽落 `None`，同 Java 现状）。
                //   真身 `Shape::cNvPr()` 返回 `&CNvPr`（`Shape.rs:842`）且真身 `CNvPr` **无
                //   `Clone`** ⇒ 经 `CNvPr_handle(..)`（KProto 往返）交出共享句柄，
                //   与 `SwPictureHandler.rs:456` 同一惯用法。
                //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                let cNvPr = CNvPr_handle(&self.dgShape.borrow());
                handler = Some(Box::new(CNvPrHandler::new(
                    cNvPr,
                    self.handlerCallback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211575:
            3211575 => {
                // Java: handler = new CNvSpPrHandler(SwShapeHandler.this.dgShape);
                //
                // rust: DEFERRED_NON_FUNCTION —— 真身 `CNvSpPrHandler` 不存在
                //   （`nvsppr/cnv/mod.rs` 只登记 CNvCxnSpPr/CNvGraphicFramePr/CNvPicPr/CNvPr；
                //   影子 `sw_shape_hosts::CNvSpPrHandler` 不实现 ② 契约，不得复用）。
                //   Java 该类（`cn/wps/moffice/drawing/dmlr/handler/shape/nvsppr/cnv/CNvSpPrHandler.java`）
                //   只处理一个属性 `putTxBox(3211661)` 与子元素 `ShapeLocksHandler(1114249)`，
                //   属 **non-pipeline-core**（`txBox` 为文本框标记，不影响形状几何/填充/文本解析）。
                //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                //   blocking type: cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvSpPrHandler
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: case 3211576:
            3211576 => {
                // Java: handler = new SwNvPrHandler(SwShapeHandler.this.swShape, SwShapeHandler.this.handlerCallback);
                //
                // rust: 真身 `SwNvPrHandler` 只提供 Java 同名构造器
                //   `SwNvPrHandler(Rc<RefCell<ShowShape>>, Option<Rc<RefCell<C>>>)`
                //   （`SwNvPrHandler.rs:58`），**无** `new` 别名（该别名只在影子模块）。
                //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(SwNvPrHandler::SwNvPrHandler(
                    self.swShape.clone(),
                    self.handlerCallback.clone(),
                )));
            }
            _ => handler = None,
        }
        // Java: return handler;
        handler
    }
}

// Java: public class ShapeExtLstHandler
impl ShapeExtLstHandler {
    // Java: ShapeExtLstHandler(Shape shape)
    pub fn ShapeExtLstHandler(shape: Shared<Shape>) -> Self {
        Self {
            // Java: this.shape = shape;
            shape,
            extHandler: None,
            extLst: Rc::new(RefCell::new(Vec::new())),
        }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211447:
            3211447 => {
                // Java: if (this.extHandler == null)
                if self.extHandler.is_none() {
                    // Java: handler = new ShapeExtHandler();
                    handler = Some(Box::new(ShapeExtHandler::new(self.extLst.clone())));
                } else {
                    handler = None;
                }
            }
            // Java: default:
            _ => {
                // Java: return handler;
                handler = None;
            }
        }
        handler
    }

    // Java: void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
        // Java: if (this.shape != null && this.extLst != null && !this.extLst.isEmpty())
        if !self.extLst.borrow().is_empty() {
            // Java: for(ShapeExt ext : this.extLst)
            for ext in self.extLst.borrow().iter() {
                // Java: ext.save();
                ext.borrow_mut().save();
            }
            // Java: this.shape.extLst().forceWrite(this.extLst);
            // rust: 真身 `ShapeExtList::forceWrite` 取 `&mut self` + `&[&ShapeExt]`
            //   （`Shape.rs:1835`，不是影子 `ExtLstHost::forceWrite(Vec<Shared<ShapeExt>>)`），
            //   但真身 `Shape::extLst()` 取 `&self` 只给 `&ShapeExtList`（`Shape.rs:1102`）
            //   ⇒ 直接链式调用会 E0596。故按本波既有惯用法把 `extLst` 槽位值**取出**、
            //   改完再放回（槽位仍是同一对象，Java 的引用语义不变；同
            //   `SwConnectorShapeHandler.rs:315` 的 take/put-back 落法）。
            //   `extLst` 槽位是 `pub(crate)`（`Shape.rs:47`，同 crate 可见），
            //   空槽位时按 Java `extLst()` 的惰性创建语义就地新建。
            //   借用表：`extLst` 的元素是 `Shared<ShapeExt>`，在 `borrow()` 期间逐元素
            //   `borrow()` 出 `&ShapeExt`（`Ref<ShapeExt>` 解引用即 `&ShapeExt`）。
            //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
            let ext_lst = self.extLst.clone();
            let lst = ext_lst.borrow();
            let borrowed: Vec<_> = lst.iter().map(|e| e.borrow()).collect();
            let refs: Vec<&ShapeExt> = borrowed.iter().map(|r| &**r).collect();
            let mut shape = self.shape.borrow_mut();
            let mut ext_list = match shape.extLst.take() {
                Some(v) => *v,
                None => ShapeExtList::create(
                    crate::cn::wps::kso::kproto::KoZone::KoZone::defaultZone(),
                ),
            };
            ext_list.forceWrite(&refs);
            let _ = shape.extLst.set(Box::new(ext_list));
        }
    }
}

// Java: public class ShapeExtHandler
impl ShapeExtHandler {
    fn new(extLst: Shared<Vec<Shared<ShapeExt>>>) -> Self {
        Self {
            handler: None,
            shapeExt: None,
            extLst,
        }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: XmlRoAttr temp;
        // rust: 真身 `get_by_name` 返回 `Option<&XmlRoAttrLeaf>` ⇒ 局部类型改叶子。
        //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
        let temp: Option<&XmlRoAttrLeaf>;
        // Java: if ((temp = attrsNotCopy.getByName(3211446)) != null)
        temp = attrsNotCopy.get_by_name(3211446);
        if let Some(temp) = temp {
            // Java: ShapeExt ext = ShapeExt.create();
            // rust: 真身 `ShapeExt::create()` **按值**返回（`ShapeExt.rs:57`，影子返回
            //   `Shared<Self>`），且真身 `ShapeExt` 无 `Clone` ⇒ 包 `Rc::new(RefCell::new(..))`
            //   承接 Java 的引用语义（同一落法见 `SwShapeAlertnateContentHandler.rs:560`）。
            //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
            let ext = Rc::new(RefCell::new(ShapeExt::create()));
            // Java: ext.putUri(temp.toString());
            // rust: 真身 `putUri` 取 `&str`（`ShapeExt.rs:399`）⇒ `to_string()` 后 `&`。
            ext.borrow_mut().putUri(&temp.to_string());
            // Java: ShapeExtLstHandler.this.extLst.add(ext);
            self.extLst.borrow_mut().push(ext.clone());
            // Java: this.shapeExt = ext;
            self.shapeExt = Some(ext);
        }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: switch (subElement)
        if subElement == 4325377 {
            // Java: case 4325377:
            // Java: this.handler = new ModularGroupExtHandler(this.shapeExt);
            self.handler = Some(Box::new(ModularGroupExtHandler::ModularGroupExtHandler(
                self.shapeExt.clone(),
            )));
        }
        // Java: default:
        // Java: return this.handler;
        self.handler.take()
    }

    // Java: void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
        // Java: if (this.shapeExt != null)
        if let Some(shapeExt) = &self.shapeExt {
            // Java: this.shapeExt.save();
            shapeExt.borrow_mut().save();
        }
    }
}

// Java: public class ModularGroupExtHandler
impl ModularGroupExtHandler {
    // Java: ModularGroupExtHandler(ShapeExt shapeExt)
    pub fn ModularGroupExtHandler(shapeExt: Option<Shared<ShapeExt>>) -> Self {
        Self {
            // Java: this.shapeExt = shapeExt;
            shapeExt,
        }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // Java: if (this.shapeExt != null)
        if let Some(shapeExt) = &self.shapeExt {
            // Java: XmlRoAttr temp;
            // rust: 真身 `get_by_name` 返回 `Option<&XmlRoAttrLeaf>` ⇒ 局部类型改叶子；
            //   真身 `putType/putIndex` 取 `&str`（`ShapeExt.rs:435` / `:471`）。
            //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
            let mut temp: Option<&XmlRoAttrLeaf>;
            // Java: if ((temp = attrsNotCopy.getByName(4325378)) != null)
            temp = attrsNotCopy.get_by_name(4325378);
            if let Some(temp) = temp {
                // Java: this.shapeExt.putType(temp.toString());
                shapeExt.borrow_mut().putType(&temp.to_string());
            }
            // Java: if ((temp = attrsNotCopy.getByName(4325379)) != null)
            temp = attrsNotCopy.get_by_name(4325379);
            if let Some(temp) = temp {
                // Java: this.shapeExt.putIndex(temp.toString());
                shapeExt.borrow_mut().putIndex(&temp.to_string());
            }
            // Java: if ((temp = attrsNotCopy.getByName(4325380)) != null)
            temp = attrsNotCopy.get_by_name(4325380);
            if let Some(temp) = temp {
                // Java: this.shapeExt.putIsDefault(temp.toBoolean());
                shapeExt.borrow_mut().putIsDefault(temp.to_boolean());
            }
        }
    }

    // Java: void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
        // Java: super.endElement(element);
        // rust: default handler end has no side effect.
    }
}

impl<C> ElementHandlerContract for SwShapeHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // rust: ② 契约要求 `kind()`（`dmlr/handler/mod.rs:4`；影子契约没有该方法）⇒ 补上，
    //   返回本类型名，同 `SwPictureHandler.rs:417` / `SwNvPrHandler.rs:242`。
    //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "SwShapeHandler"
    }
    fn startElement(&mut self, e: i32, a: &XmlRoAttrList) {
        SwShapeHandler::startElement(self, e, a);
    }
    fn endElement(&mut self, e: i32) {
        SwShapeHandler::endElement(self, e);
    }
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        SwShapeHandler::enterSubElement(self, e)
    }
}

impl<C> ElementHandlerContract for NonVisualShapePropertiesHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // rust: 补 `kind()`；`startElement` / `endElement` 在 Java 侧继承
    //   `DefaultElementHandler` 的空实现，故按空实现补齐（② 契约无默认体）。
    //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "NonVisualShapePropertiesHandler"
    }
    // Java: (继承 DefaultElementHandler 的空 startElement)
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    // Java: (继承 DefaultElementHandler 的空 endElement)
    fn endElement(&mut self, _e: i32) {}
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        NonVisualShapePropertiesHandler::enterSubElement(self, e)
    }
}

impl ElementHandlerContract for ShapeExtLstHandler {
    // rust: 补 `kind()`；`startElement` 在 Java 侧继承空实现 ⇒ 空体。
    //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "ShapeExtLstHandler"
    }
    // Java: (继承 DefaultElementHandler 的空 startElement)
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn endElement(&mut self, e: i32) {
        ShapeExtLstHandler::endElement(self, e);
    }
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        ShapeExtLstHandler::enterSubElement(self, e)
    }
}

impl ElementHandlerContract for ShapeExtHandler {
    // rust: 补 `kind()`。
    //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "ShapeExtHandler"
    }
    fn startElement(&mut self, e: i32, a: &XmlRoAttrList) {
        ShapeExtHandler::startElement(self, e, a);
    }
    fn endElement(&mut self, e: i32) {
        ShapeExtHandler::endElement(self, e);
    }
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        ShapeExtHandler::enterSubElement(self, e)
    }
}

impl ElementHandlerContract for ModularGroupExtHandler {
    // rust: 补 `kind()` 与 `enterSubElement` —— 后者在 Java 侧由
    //   `DefaultElementHandler` 提供（返回 null），② 契约无默认体 ⇒ 显式补空实现。
    //   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
    fn kind(&self) -> &'static str {
        "ModularGroupExtHandler"
    }
    fn startElement(&mut self, e: i32, a: &XmlRoAttrList) {
        ModularGroupExtHandler::startElement(self, e, a);
    }
    fn endElement(&mut self, e: i32) {
        ModularGroupExtHandler::endElement(self, e);
    }
    // Java: (继承 DefaultElementHandler 的 enterSubElement ⇒ 返回 null)
    fn enterSubElement(&mut self, _e: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}

// rust: 下面的四个自由函数无对应 Java 成员 —— Java 的 `Shape::spPr()` / `style()` /
//   `cNvPr()` / `textShape().textBody()` 返回的是**引用**，而本 port 的真身
//   `cn.wps.drawing.data.shape.Shape` 把这些槽位建模为 `Option<Box<_>>` / `Box<_>`
//   （`Shape.rs:41/43/49`，`TextShape.rs`），是**所有权值**且**无 `Clone`**，无法别名成
//   `Rc<RefCell<_>>`；真身 `ShapePropertiesHandler` / `ShapeStyleHandler` / `CNvPrHandler` /
//   `TextBodyHandler` 却要求共享句柄。
//   故此处按「Java 引用语义的 Rust 承载」提供取值器：以 Java 的同名成员为锚点，经
//   KProto 往返（`lastBin()` + `open(..)` / `convert_clone()`）交出共享句柄，再交给子处理器
//   —— 与 `SwPictureHandler.rs:667+` / `SwConnectorShapeHandler.rs:315+` 完全同构
//   （同为本波既有约定）。
//   ⚠ 偏差登记：本 port 的真身 `Shape` 尚未暴露这些槽位的共享句柄面
//     （`ShapePr` 的 `*_handle` 只覆盖 `Transform2D/Fill/Geometry2D/Outline`，
//      不含 `ShapePr`/`ShapeStyle`/`CNvPr`/`TextBody` 本身），故句柄在此**按需创建**
//     而非取自模型 —— 语义上等价于 Java 的「非空引用」（真身同名取值器自身即惰性创建）。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH

// Java: Shape.spPr() —— 共享句柄投影（见上注）。
// rust: 真身 `ShapePr` 是 KProto 容器（无 `Clone` 派生）⇒ 用真身既有的
//   `ShapePr::convert_clone(&self)`（`ShapePr.rs:1473`；本波为同一目的登记的非 `Clone`
//   拷贝惯用法，按 `map` 键逐槽重解码，语义 = Java 的「复刻一个 KoAr 对象」）。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
fn ShapePr_handle(dg: &Shape) -> Shared<ShapePr> {
    Rc::new(RefCell::new(dg.spPr().convert_clone()))
}

// Java: Shape.style() —— 共享句柄投影（见上注）。
// rust: 真身 `ShapeStyle` 同为无 `Clone` 的 KProto 容器（只有 `ShapeStyle::create`），
//   故同 `ShapePr` 的惯用法：`lastBin()` + `ShapeStyle::open(..)`。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
fn ShapeStyle_handle(dg: &Shape) -> Shared<ShapeStyle> {
    let bin = dg
        .style()
        .lastBin(crate::cn::wps::kso::kproto::KoZone::KoZone::defaultZone());
    Rc::new(RefCell::new(ShapeStyle::open(bin.as_ref())))
}

// Java: Shape.cNvPr() —— 共享句柄投影（见上注）。
// rust: 真身 `CNvPr` 同为无 `Clone` 的 KProto 容器（只有 `CNvPr::create`）⇒ 同上惯用法。
//   `Shape.cNvPr` 槽位是 `pub(crate)` ⇒ 只能走公开的 `hasCNvPr()` / `cNvPr()`；
//   空槽位时按 Java `cNvPr()` 的语义就地创建（同 `SwPictureHandler.rs:697`），
//   避免真身 `Shape::cNvPr()` 末尾 `unwrap()` 在空槽位时 panic。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
fn CNvPr_handle(dg: &Shape) -> Shared<crate::cn::wps::drawing::data::shape::CNvPr::CNvPr> {
    use crate::cn::wps::drawing::data::shape::CNvPr::CNvPr;
    if dg.hasCNvPr() {
        let bin = dg
            .cNvPr()
            .lastBin(crate::cn::wps::kso::kproto::KoZone::KoZone::defaultZone());
        Rc::new(RefCell::new(CNvPr::open(bin.as_ref())))
    } else {
        Rc::new(RefCell::new(CNvPr::create()))
    }
}

// Java: Shape.textShape().textBody() —— 共享句柄投影（见上注）。
// rust: 真身 `TextBody` 是 KProto 容器（无 `Clone`）⇒ `save()` + `TextBody::open(..)`
//   的取值往返拷贝。真身 `Shape::textShape()` 取 `&self`（`Shape.rs:1044`）只给
//   `&TextShape`，而 `TextShape::textBody()` 需 `&mut self`（`TextShape.rs:379`）
//   ⇒ 直接链式调用会 E0596。故按本波 take/put-back 惯用法：把 `textShape` 槽位值取出、
//   在其上取 `textBody`（同样取出/放回），再从 `textBody` 的 `lastBin()` 往返出共享句柄；
//   最后把两个槽位按原值放回（Java 的引用语义不变，同
//   `SwConnectorShapeHandler.rs:315` / `SwPictureHandler.rs:667` 的落法）。
//   owner: show.pptxr.handler.shape.SwShapeHandler    wave: M-PPT-SHAPE-DISPATCH
fn TextBody_handle(
    dg: &mut Shape,
) -> Shared<crate::cn::wps::drawing::data::text::TextBody::TextBody> {
    use crate::cn::wps::drawing::data::text::TextBody::TextBody;
    use crate::cn::wps::drawing::data::text::TextShape::TextShape;
    // rust: Java `textShape()` 的惰性槽位：空时按同一语义新建（`TextShape::create`）。
    let mut text_shape = match dg.textShape.take() {
        Some(v) => *v,
        None => TextShape::create(),
    };
    // rust: 同 `textShape` —— `textBody()` 亦需 `&mut self`，故取出/放回。
    let mut text_body = match text_shape.textBody.take() {
        Some(v) => *v,
        None => TextBody::create(),
    };
    let saved = text_body.save();
    let handle = Rc::new(RefCell::new(TextBody::open(&*saved)));
    text_shape.textBody = Some(Box::new(text_body));
    let _ = dg.textShape.set(Box::new(text_shape));
    handle
}
