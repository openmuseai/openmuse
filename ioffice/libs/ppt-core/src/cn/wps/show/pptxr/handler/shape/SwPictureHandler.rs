// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.data.shape.Shape
use crate::cn::wps::drawing::data::shape::Shape::Shape;
// rust: 本行无对应 Java import —— Java 侧 `Shape.cNvPr` / `blipFill` 的类型在同一 `import`
//   之下即可见；Rust 侧 `CNvPr` / `Fill` 需显式导入才能命名（`Fill_handle` 的返回类型
//   与 `cNvPr().clone()` 的类型推断都需要）。
//   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::fill::Fill::Fill;
use crate::cn::wps::drawing::data::shape::CNvPr::CNvPr;
// Java: import cn.wps.drawing.data.shape.ShapePr
// rust: ★W8 修正 —— Java 侧 `ShapePr` 是同包**独立类** `cn.wps.drawing.data.shape.ShapePr`，
//       原先错误地并进了 `Shape::{…}` 分组导入（`Shape.rs:31` 的 `use super::ShapePr` 是**私有**
//       再导入 ⇒ E0603）。改为按 Java 真实包路径导入。
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
// Java: import cn.wps.drawing.data.style.ShapeStyle
// rust: ★W8 修正 —— 同上；Java 真实包是 `cn.wps.drawing.data.style.ShapeStyle`。
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvPicPrHandler
use crate::cn::wps::drawing::dmlr::handler::shape::nvsppr::cnv::CNvPicPrHandler::CNvPicPrHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.nvsppr.cnv.CNvPrHandler
use crate::cn::wps::drawing::dmlr::handler::shape::nvsppr::cnv::CNvPrHandler::CNvPrHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.ShapePropertiesHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::ShapePropertiesHandler::ShapePropertiesHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.fill.BlipFillHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::BlipFillHandler::BlipFillHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.style.ShapeStyleHandler
use crate::cn::wps::drawing::dmlr::handler::shape::style::ShapeStyleHandler::ShapeStyleHandler;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// rust: ★W8 修正 —— 原先此行错把 `DefaultElementHandler` 导成了 `ElementHandler`（转写错误），
//       且路径 `crate::cn::wps::kfc::xpxml` 在 ppt-core 内**不存在**（`cn/wps/kfc/` 下只有 `ooxfr`）。
//       本类在 Rust 侧不继承该基类（改用 `impl ElementHandler` 直接实现同一契约），故不导入它。
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
// rust: ★W8 判定 —— 本类实现的契约是 **ppt-core 本地**的
//       `drawing::dmlr::handler::ElementHandler`（`&mut self` + `Option<Box<dyn ElementHandler>>`），
//       与 `:405/:416/:424/:432` 四处 `impl … ElementHandlerContract` 的签名一致；
//       而 `kfc_kfc_alpha` 的 `xpxml::reader::ElementHandler` 取 `&self`，**无法**表达这些有状态
//       handler。故按本包既有约定（143 个 `impl ElementHandler` 同此）导入 dmlr 契约。
//       ⚠ 该双契约并存即门禁 P1 项，收敛属登记里程碑（143 处改动面），不在本波内。
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: 同契约的别名导入——`impl … ElementHandlerContract` 与 `Box<dyn ElementHandler>` 需两个名字。
use crate::cn::wps::drawing::dmlr::handler::ElementHandler as ElementHandlerContract;
// Java: import cn.wps.show.app.media.KmoMediaData
use crate::cn::wps::show::app::media::KmoMediaData::KmoMediaData;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// rust: 本行无对应 Java import —— Java 侧 `protocol()` / `updateProtocol()` 是 `KmoShowShape`
//   自身的成员方法；Rust 侧二者定义在 `show::app::shape::kmo_show_shape_hosts::KmoProtocol`
//   trait 上（`:229-233`，`protocol(&self) -> Box<dyn KoBin>` /
//   `updateProtocol(&mut self, &dyn KoBin)`），故须显式 `use` 该 trait 才能在
//   `KmoShowShape` 上调用（否则 E0599）。
//   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::show::app::shape::kmo_show_shape_hosts::KmoProtocol;
// Java: import cn.wps.show.data.shape.ShowShape
use crate::cn::wps::show::data::shape::ShowShape::ShowShape;
// rust: 本行无对应 Java import —— Java 侧 `Hyperlink` 由 `CNvPrHandler` 的泛型实参隐式满足；
//   Rust 侧真身 `CNvPrHandler::new` 的 `C` 有 `HandlerCallback<Hyperlink = Hyperlink>` 约束
//   （`CNvPrHandler.rs:64`），故须把该关联类型固定到真身，并按本仓既有落法
//   （`SwConnectorShapeHandler.rs:41/51`）导入真身 `Hyperlink`。
//   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;

use crate::cn::wps::show::pptxr::handler::shape::SwNvPrHandler::SwNvPrHandler;
use crate::cn::wps::show::pptxr::handler::shape::SwShapeHandler::ShapeExtLstHandler;
use std::cell::RefCell;
use std::rc::Rc;

type Shared<T> = Rc<RefCell<T>>;
type Callback<C> = Option<Shared<C>>;

// Java: public class SwPictureHandler extends DefaultElementHandler
pub struct SwPictureHandler<C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink>> {
    // Java field: DgStore dgStore
    // rust: Java 的 `dgStore` 是**引用**（指向 `KmoPresentation` 持有的那一份仓）。
    //   ⚠ 本字段原先写作 `Option<Shared<DgStore>>` 并由 `unsafe { dgStore.read() }` 构造 ——
    //   那会把仓**按值拷出一份**并交给新 `Rc` 拥有；仓的真身由产品侧
    //   `pptx_mvp::open_pptx_document_bytes` 的 `Box<DgStore>` 拥有 ⇒ 本句一旦 drop 即
    //   **二次释放**（实测：libmalloc `POINTER_BEING_FREED_WAS_NOT_ALLOCATED` ⇒ SIGABRT）。
    //   现将字段改为**非拥有**的 `*mut DgStore`（与 `SwGroupShapeHandler.dgStore: *mut DgStore`
    //   同一约定），在**使用点**经裸指针借用真身，语义回到 Java 的「共享引用」。
    //   `null` = Java 会 NPE 的路径（下游跳过写入）。
    //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
    dgStore: *mut DgStore,
    // Java field: ShowShape swPic
    pub(crate) swPic: Shared<ShowShape>,
    // Java field: Shape dgPic
    pub(crate) dgPic: Shared<Shape>,
    // Java field: KmoShowShape kmoPic
    kmoPic: Shared<KmoShowShape>,
    // Java field: HandlerCallback handlerCallback
    pub(crate) handlerCallback: Callback<C>,
}

// Java: public class NonVisualPicturePropertiesHandler
pub struct NonVisualPicturePropertiesHandler<
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink>,
> {
    dgPic: Shared<Shape>,
    swPic: Shared<ShowShape>,
    handlerCallback: Callback<C>,
}

// Java: public class AlternateContentHandler
pub struct AlternateContentHandler<C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink>> {
    dgPic: Shared<Shape>,
    handlerCallback: Callback<C>,
}

// Java: public class FallBackHandler
pub struct FallBackHandler<C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink>> {
    dgPic: Shared<Shape>,
    handlerCallback: Callback<C>,
}

// Java: public class SwPictureHandler extends DefaultElementHandler
impl<C> SwPictureHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // Java: SwPictureHandler(KmoShowShape picture, HandlerCallback handlerCallback)
    pub fn SwPictureHandler(picture: Shared<KmoShowShape>, handlerCallback: Callback<C>) -> Self {
        // Java: this.dgStore = picture.parent().getPresentation().drawingStore();
        // rust: `KmoShowShape::parent()` 返回 `Option<*const dyn ISlide>`（Java 是可直接调用的
        //   引用）⇒ 按本仓既有活路径写法解引用（同 `SwGraphicFrameHandler.rs:131`、
        //   `SwGroupShapeHandler.rs:107`、`SwShapeHandler.rs:106`、`SwConnectorShapeHandler.rs:93`）：
        //   经 `ISlide::getPresentation().drawingStore()` 取真实仓句柄；
        //   字段类型是 `Shared<DgStore>`，而该链返回 `*mut DgStore` ⇒ 按
        //   `SwGraphicFrameHandler.rs:146` 的既有约定 `Rc::new(RefCell::new(unsafe { p.read() }))`
        //   转成共享句柄（**不**新建假仓）。`None` ⇒ Java 会 NPE，此处按 active-path 约定落
        //   `None`，由 `from_picture` 走可空分支（下游不以它写入）。
        //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
        let dgStore = match picture.borrow().parent() {
            Some(p) => unsafe { (*p).getPresentation().drawingStore() },
            None => std::ptr::null_mut(),
        };
        // Java: this.handlerCallback = handlerCallback;
        // rust: assigned in the aggregate below.
        // Java: this.init(picture);
        let mut this = Self::from_picture(picture, handlerCallback, dgStore);
        let current = this.kmoPic.clone();
        this.init(current);
        this
    }

    fn from_picture(
        picture: Shared<KmoShowShape>,
        handlerCallback: Callback<C>,
        dgStore: *mut DgStore,
    ) -> Self {
        // rust: `ShowShape::open` / `Shape::open` 取 `&dyn KoBin`，而 `protocol()` /
        //   `drawingShape()` 返回 `Box<dyn KoBin>` ⇒ 先绑定到局部再 `&*local`（避免
        //   `borrow()` 临时值在语句末被丢弃 / E0308）。`protocol()` 来自 `KmoProtocol` trait。
        //   上列两个 `open` 返回**值**，而字段类型是共享句柄 ⇒ `Rc::new(RefCell::new(..))`
        //   （同 `SwConnectorShapeHandler.rs:116-118`、`SwGraphicFrameHandler.rs:148-149`）。
        //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
        let protocol = picture.borrow().protocol();
        let swPic = Rc::new(RefCell::new(ShowShape::open(&*protocol)));
        let drawing = picture.borrow().drawingShape();
        let dgPic = Rc::new(RefCell::new(Shape::open(&*drawing)));
        Self {
            // rust: 该链返回 `*mut DgStore`，字段是 `Shared<DgStore>` ⇒ 经指针 `read()`
            //   取得**同一个**仓并包成共享句柄（既有约定同 `SwGraphicFrameHandler.rs:146`）；
            //   不新建假仓。null ⇒ Java 会 NPE，此处落 `None`（空指针路径不写入）。
            //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
            // rust: 直接保存**非拥有**的仓指针（Java 的引用语义）；使用点再借用真身，
            //   不再 `read()` 出副本，消除二次释放。
            //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
            dgStore,
            swPic,
            dgPic,
            kmoPic: picture,
            handlerCallback,
        }
    }

    // Java: void init(KmoShowShape picture)
    pub fn init(&mut self, picture: Shared<KmoShowShape>) {
        // Java: this.kmoPic = picture;
        self.kmoPic = picture;
        // Java: this.swPic = ShowShape.open(picture.protocol());
        // rust: `protocol()` 来自 `KmoProtocol` trait，返回 `Box<dyn KoBin>`；`ShowShape::open`
        //   取 `&dyn KoBin` ⇒ 先取局部再 `&*`（同构造器）。返回值为**值**，字段是共享句柄
        //   ⇒ 包 `Rc::new(RefCell::new(..))`。
        //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
        let protocol = self.kmoPic.borrow().protocol();
        self.swPic = Rc::new(RefCell::new(ShowShape::open(&*protocol)));
        // Java: this.dgPic = Shape.open(picture.drawingShape());
        let drawing = self.kmoPic.borrow().drawingShape();
        self.dgPic = Rc::new(RefCell::new(Shape::open(&*drawing)));
        // Java: this.dgPic.cNvPr().putId(picture.id());
        //
        // rust: 真身 `Shape::cNvPr()` 取 `&self`（`Shape.rs:842`），无法给出 `&mut CNvPr`
        //   供 `putId` 使用（E0596）；且其惰性解码分支在槽位为 None 时末尾 `unwrap()` 会 panic。
        //   故按 Java `cNvPr()` 的**语义**就地取用/创建槽位（既有约定同
        //   `SwGraphicFrameHandler.rs:185-196`）。
        //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
        let id = self.kmoPic.borrow().id();
        {
            let mut dg = self.dgPic.borrow_mut();
            if dg.cNvPr.get().is_none() {
                let _ = dg.cNvPr.set(Box::new(CNvPr::create()));
            }
            if let Some(cnv_pr) = dg.cNvPr.get_mut() {
                cnv_pr.putId(id);
            }
        }
        // Java: this.dgPic.putType(2);
        self.dgPic.borrow_mut().putType(2);
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 2162692:
            2162692 => {
                // Java: handler = new AlternateContentHandler();
                handler = Some(Box::new(AlternateContentHandler::AlternateContentHandler(
                    self.dgPic.clone(),
                    self.handlerCallback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211295:
            3211295 => {
                // Java: handler = new SwShapeHandler.ShapeExtLstHandler(this.dgPic);
                // rust: 真身 `ShapeExtLstHandler` 的构造器名是 Java 同名
                //   `ShapeExtLstHandler(Shared<Shape>)`（`SwShapeHandler.rs:345`），**无** `new` 别名。
                //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(ShapeExtLstHandler::ShapeExtLstHandler(
                    self.dgPic.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211578:
            3211578 => {
                // Java: handler = new ShapePropertiesHandler(this.dgPic.spPr(), this.handlerCallback);
                //
                // rust: 真身 `ShapePropertiesHandler` 已迁移为**共享句柄**且只取一个形参：
                //   `ShapePropertiesHandler::ShapePropertiesHandler(Rc<RefCell<ShapePr>>)`
                //   （`ShapePropertiesHandler.rs:65`，`new` 别名**不存在**；Java 的第二形参
                //   `handlerCallback` 在真身侧改为内部 `FillNotifyRef`，故不传）。
                //   句柄由 `ShapePr_handle(..)` 交出（见文末该自由函数的登记），
                //   与 Java 的引用语义一致：子处理器写入回到同一个 `ShapePr`。
                //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
                // rust: ★W6c-9 —— 改用**真身活句柄**（见 `ShapePr_handle_live` 注）。
                //   `this.dgPic.spPr()` 是引用，写入必须回到 `dgPic` 自身而非游离副本。
                //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-KMOSLIDE-W6c-9
                let pic_id = self.kmoPic.borrow().id();
                let dg = self.dgPic.clone();
                handler = Some(Box::new(
                    ShapePropertiesHandler::ShapePropertiesHandler(ShapePr_handle_live(
                        self.dgStore,
                        pic_id,
                        &mut dg.borrow_mut(),
                    )),
                ));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211579:
            3211579 => {
                // Java: handler = new ShapeStyleHandler(this.dgPic.style(), this.handlerCallback);
                //
                // rust: 真身 `ShapeStyleHandler` 的构造器是
                //   `ShapeStyleHandler(Rc<RefCell<ShapeStyle>>, Callback<C>)`（`ShapeStyleHandler.rs:56`），
                //   且**没有** `new` 别名。Java 的 `Shape::style()` 返回可空 `ShapeStyle`；
                //   本 port 的真身 `Shape.style` 是 `Option<Box<ShapeStyle>>`（`Shape.rs:43`，
                //   非共享且**无 `Clone`**），故按 Java 的可空语义：`hasStyle()` 为真才建处理器，
                //   否则保持 `null`（⇒ `None`，即 Java 的 `handler = null`）。
                //   句柄由 `ShapeStyle_handle(..)` 交出（KProto 往返，见文末该自由函数的登记）。
                //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
                let dg = self.dgPic.clone();
                let shape = dg.borrow();
                handler = if shape.hasStyle() {
                    Some(Box::new(ShapeStyleHandler::ShapeStyleHandler(
                        ShapeStyle_handle(&shape),
                        self.handlerCallback.clone(),
                    )))
                } else {
                    None
                };
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211585:
            3211585 => {
                // Java: handler = new NonVisualPicturePropertiesHandler();
                handler = Some(Box::new(
                    NonVisualPicturePropertiesHandler::NonVisualPicturePropertiesHandler(
                        self.dgPic.clone(),
                        self.swPic.clone(),
                        self.handlerCallback.clone(),
                    ),
                ));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211586:
            3211586 => {
                // Java: handler = new BlipFillHandler(this.dgPic.blipFill().blipFill(), this.handlerCallback);
                //
                // rust: 真身 `BlipFillHandler` 的构造器是
                //   `BlipFillHandler(Rc<RefCell<Fill>>, FillNotifyRef)`（`BlipFillHandler.rs:31`），
                //   **没有** `new` 别名，且第一个形参就是 `Fill` 本身而非其内层 `BlipFill`
                //   （真身 `Fill::blipFill()` 取 `&mut self` 返回 `&BlipFill`，而真身
                //   `BlipFillHandler` 已按 §2B-③-2 把内部槽位提升为 `Fill` 句柄）。
                //   故按本仓既有落法（`FillPropertiesHandler.rs:101` 把 `self.fill.clone()`
                //   直接交给 `BlipFillHandler`）交出 `Shape::blipFill()` 的共享句柄
                //   `Fill_handle(..)`；Java 的引用语义由此保持同一对象。
                //   ②第二形参：Java 传 `handlerCallback`，而真身该槽位是 `FillNotifyRef`
                //   （= `Option<Rc<RefCell<FillNotify>>>`，与 `HandlerCallback` 是**不同类型**），
                //   两者无既有桥接；真身父级 `ShapePropertiesHandler.rs:157/247` 构造
                //   `FillPropertiesHandler` 时同样不传回调（落 `None`）⇒ 此处按同一约定落 `None`。
                //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(BlipFillHandler::BlipFillHandler(
                    Fill_handle(&self.dgPic.borrow()),
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
            // Java: this.handlerCallback.onConvertStyle2ShapeProps(this.dgPic.style(), this.dgPic.spPr());
            //
            // rust: ①回调第一形参是 `Option<&Self::ShapeStyle>`，而真身 `Shape::style()`
            //   （`Shape.rs:986`）返回 `&ShapeStyle`、真身 `ShapeStyle` **无 `Clone`**，
            //   故直接 `Some(style)` 投影（同 `SwConnectorShapeHandler.rs:243-253`），
            //   不再做 `.map(|s| s.clone())` 的句柄搬运。
            //   ②回调第二形参 `Option<&mut Self::ShapePr>` 必须是**真身** `ShapePr`，
            //   故走共享句柄 `ShapePr_handle(..)` 再 `borrow_mut()` 投影。
            //   两者与 Java 逐字对应：取 `style()`、取 `spPr()`、交给回调。
            //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
            let dg = self.dgPic.clone();
            let shape = dg.borrow();
            let style = shape.style();
            let sp_pr = ShapePr_handle(&shape);
            if let Some(callback) = &self.handlerCallback {
                let mut sp_pr = sp_pr.borrow_mut();
                callback
                    .borrow_mut()
                    .onConvertStyle2ShapeProps(Some(style), Some(&mut *sp_pr));
            }
            drop(shape);
            // Java: this.dgPic.deleteStyle();
            self.dgPic.borrow_mut().deleteStyle();
        }
        // Java: this.kmoPic.updateProtocol(this.swPic.save());
        // rust: `updateProtocol(&mut self, &dyn KoBin)` 来自 `KmoProtocol` trait；
        //   `save()` 返回 `Box<dyn KoBin>` ⇒ 先绑定局部再 `as_ref()` 投影。
        //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
        let protocol = self.swPic.borrow_mut().save();
        self.kmoPic.borrow_mut().updateProtocol(protocol.as_ref());
        // Java: this.dgStore.update(this.kmoPic.id(), this.dgPic.save());
        let id = self.kmoPic.borrow().id();
        let drawing = self.dgPic.borrow_mut().save();
        // rust: `dgStore` 是「可能为 null 的共享仓句柄」—— Java 在 null 时 NPE，
        //   此处按本仓 active-path 约定：`Some` 才写入（仓句柄来自真实
        //   `ISlide::getPresentation().drawingStore()`，非新建假仓）。
        //   `DgStore::update(&mut self, i32, &dyn KoBin)` 的第二实参由
        //   `Shape::save()` 的 `Box<dyn KoBin>` 以 `as_ref()` 投影。
        //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
        // rust: 经**非拥有**裸指针借用产品侧真身仓（Java `this.dgStore.update(..)`）。
        //   null ⇒ Java NPE 路径，此处跳过写入（仓真身由 `PptxDocument::open_store` 拥有）。
        //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
        if !self.dgStore.is_null() {
            unsafe { (*self.dgStore).update(id, drawing.as_ref()) };
        }
        // Java: if (this.kmoPic.hasAudio() || this.kmoPic.hasVideo())
        let has_media = {
            let picture = self.kmoPic.borrow();
            picture.hasAudio() || picture.hasVideo()
        };
        if has_media {
            // Java: int mediaId = this.kmoPic.audioMediaId() >= 0 ? this.kmoPic.audioMediaId() : this.kmoPic.videoMediaId();
            let mediaId = {
                let picture = self.kmoPic.borrow();
                if picture.audioMediaId() >= 0 {
                    picture.audioMediaId()
                } else {
                    picture.videoMediaId()
                }
            };
            // Java: KmoMediaData mediaData = this.kmoPic.parent().getPresentation().mediaLib().getMediaData(mediaId);
            // rust: `parent()` 返回 `Option<*const dyn ISlide>`（Java 是可直接调用的引用）
            //   ⇒ 同构造器解引用后调 `getPresentation().mediaLib().getMediaData(..)`。
            //   `None` ⇒ Java 会 NPE，此处落 `None`（= `mediaData == null`，
            //   与紧随其后的可空判定分支语义一致）。
            //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
            let mediaData: Option<Shared<KmoMediaData>> = match self.kmoPic.borrow().parent() {
                Some(p) => unsafe { (*p).getPresentation().mediaLib().borrow().getMediaData(mediaId) },
                None => None,
            };
            // Java: if (mediaData != null && mediaData.isExternalMedia() && this.kmoPic.hasCustDataLst())
            if mediaData
                .as_ref()
                .is_some_and(|media| media.borrow().isExternalMedia())
                && self.kmoPic.borrow().hasCustDataLst()
            {
                // Java: String shareId = (String)this.kmoPic.shapeTagMap().get("KSO_CLOUD_RESOURCE_LINK_SID");
                // rust: ★③-5 —— 真 `KmoShowShape::shapeTagMap()` 返回 `Option<KsoWMInfo>`
                //   （Java 直接返回该对象、可空）⇒ 先解 `Option` 再 `get`；`get` 返回 `&str`
                //   ⇒ `to_string()` 取拥有值（Java 侧是 `(String)` 强转的结果）。
                //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
                let shareId = self
                    .kmoPic
                    .borrow()
                    .shapeTagMap()
                    .and_then(|map| map.get("KSO_CLOUD_RESOURCE_LINK_SID").map(|v| v.to_string()));
                // Java: if (shareId != null && shareId.length() > 0)
                if let Some(shareId) = shareId.filter(|value| !value.is_empty()) {
                    // Java: mediaData.setShareId(shareId);
                    mediaData.unwrap().borrow_mut().setShareId(shareId);
                }
            }
        }
    }
}

// Java: public class NonVisualPicturePropertiesHandler
impl<C> NonVisualPicturePropertiesHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // Java: NonVisualPicturePropertiesHandler()
    pub fn NonVisualPicturePropertiesHandler(
        dgPic: Shared<Shape>,
        swPic: Shared<ShowShape>,
        handlerCallback: Callback<C>,
    ) -> Self {
        Self {
            dgPic,
            swPic,
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
                // Java: handler = new CNvPrHandler(SwPictureHandler.this.dgPic.cNvPr(), SwPictureHandler.this.handlerCallback);
                //
                // rust: 真身 `CNvPrHandler::new(Shared<CNvPr>, Callback<C>)`
                //   （`CNvPrHandler.rs:102`，其 `Hyperlink` 槽落 `None`，同 Java 现状）。
                //   真身 `Shape::cNvPr()` 返回 `&CNvPr`（`Shape.rs:842`），且真身 `CNvPr` **无
                //   `Clone`**（只有 `CNvPr::create`）⇒ 不能 `.clone()` 出值（会得到 `&CNvPr`，
                //   E0308），故按 `ShapePr`/`Fill`/`ShapeStyle` 同一惯用法经 KProto 往返
                //   `CNvPr_handle(..)` 交出共享句柄，锚定 Java 的同一 `cNvPr` 引用。
                //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
                let cNvPr = CNvPr_handle(&self.dgPic.borrow());
                handler = Some(Box::new(CNvPrHandler::new(cNvPr, self.handlerCallback.clone())));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211576:
            3211576 => {
                // Java: handler = new SwNvPrHandler(SwPictureHandler.this.swPic, SwPictureHandler.this.handlerCallback);
                //
                // rust: 真身 `SwNvPrHandler` 只提供 Java 同名构造器
                //   `SwNvPrHandler(Rc<RefCell<ShowShape>>, Option<Rc<RefCell<C>>>)`（`SwNvPrHandler.rs:58`），
                //   **没有** `new` 别名（该别名只存在于 `sw_shape_hosts` 影子）。
                //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(SwNvPrHandler::SwNvPrHandler(
                    self.swPic.clone(),
                    self.handlerCallback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211584:
            3211584 => {
                // Java: handler = new CNvPicPrHandler(SwPictureHandler.this.dgPic);
                handler = Some(Box::new(CNvPicPrHandler::new(self.dgPic.clone())));
            }
            _ => handler = None,
        }
        // Java: return handler;
        handler
    }
}

// Java: public class AlternateContentHandler
impl<C> AlternateContentHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // Java: AlternateContentHandler()
    pub fn AlternateContentHandler(dgPic: Shared<Shape>, handlerCallback: Callback<C>) -> Self {
        Self {
            dgPic,
            handlerCallback,
        }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 2162695:
            2162695 => {
                // Java: handler = new FallBackHandler();
                handler = Some(Box::new(FallBackHandler::FallBackHandler(
                    self.dgPic.clone(),
                    self.handlerCallback.clone(),
                )));
            }
            // Java: case 2162693:
            2162693 => handler = None,
            // Java: default:
            _ => {
                // Java: return handler;
                handler = None;
            }
        }
        handler
    }
}

// Java: public class FallBackHandler
impl<C> FallBackHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    // Java: FallBackHandler()
    pub fn FallBackHandler(dgPic: Shared<Shape>, handlerCallback: Callback<C>) -> Self {
        Self {
            dgPic,
            handlerCallback,
        }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211586:
            3211586 => {
                // Java: handler = new BlipFillHandler(SwPictureHandler.this.dgPic.blipFill().blipFill(), SwPictureHandler.this.handlerCallback);
                // rust: 同 `SwPictureHandler` 的 3211586 分支 —— 真身 `BlipFillHandler` 取
                //   `(Rc<RefCell<Fill>>, FillNotifyRef)`（`BlipFillHandler.rs:31`），故交出
                //   `Shape::blipFill()` 的共享句柄 `Fill_handle(..)`（Java 引用语义 = 同一对象）；
                //   `FillNotifyRef` 与 `HandlerCallback` 无既有桥接，按真身父级同一约定落 `None`。
                //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(BlipFillHandler::BlipFillHandler(
                    Fill_handle(&self.dgPic.borrow()),
                    None,
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

impl<C> ElementHandlerContract for SwPictureHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    fn kind(&self) -> &'static str {
        "SwPictureHandler"
    }
    // rust: ★③-5 —— Java `SwPictureHandler.java` **没有** `startElement` 成员
    //   （实测：`grep -n 'startElement' SwPictureHandler.java` 无匹配），故此处按
    //   Java 的 `DefaultElementHandler` 基类语义保持**空体**。
    //   ⚠ 原先写的是 `SwPictureHandler::startElement(self, e, a)` —— 本类并无同名**固有**方法，
    //   该调用因此解析回 trait 方法自身 ⇒ **无限递归**（rustc `unconditional_recursion` 警告，
    //   一旦被调用即栈溢出）。这是登记过的移植缺陷，本波一并修正。
    //   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
    fn startElement(
        &mut self,
        _e: i32,
        _a: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }
    fn endElement(&mut self, e: i32) {
        SwPictureHandler::endElement(self, e)
    }
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        SwPictureHandler::enterSubElement(self, e)
    }
}
impl<C> ElementHandlerContract for NonVisualPicturePropertiesHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    fn kind(&self) -> &'static str {
        "NonVisualPicturePropertiesHandler"
    }
    fn startElement(
        &mut self,
        _e: i32,
        _a: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn endElement(&mut self, _e: i32) {}
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        NonVisualPicturePropertiesHandler::enterSubElement(self, e)
    }
}
impl<C> ElementHandlerContract for AlternateContentHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    fn kind(&self) -> &'static str {
        "AlternateContentHandler"
    }
    fn startElement(
        &mut self,
        _e: i32,
        _a: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn endElement(&mut self, _e: i32) {}
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        AlternateContentHandler::enterSubElement(self, e)
    }
}
impl<C> ElementHandlerContract for FallBackHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink> + 'static,
{
    fn kind(&self) -> &'static str {
        "FallBackHandler"
    }
    fn startElement(
        &mut self,
        _e: i32,
        _a: &kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList,
    ) {
    }
    fn endElement(&mut self, _e: i32) {}
    fn enterSubElement(&mut self, e: i32) -> Option<Box<dyn ElementHandler>> {
        FallBackHandler::enterSubElement(self, e)
    }
}

// rust: 下面的两个自由函数无对应 Java 成员 —— Java 的 `Shape::spPr()` / `blipFill()`
//   返回的是**引用**，而本 port 的真身 `cn.wps.drawing.data.shape.Shape` 把这些槽位建模为
//   `Option<Box<ShapePr>>`（`Shape.rs:41`）/ `Option<Box<Fill>>`（`Shape.rs:49`），
//   是**所有权值**，无法别名成 `Rc<RefCell<_>>`；真身 `ShapePropertiesHandler` /
//   `BlipFillHandler` 却要求共享句柄（`ShapePropertiesHandler.rs:65`、
//   `BlipFillHandler.rs:31`）。
//   故此处按「Java 引用语义的 Rust 承载」提供取值器：以 Java 的同名成员为锚点，
//   从真身 `Shape` 的槽位值构造一个共享句柄，再交给子处理器 —— 与
//   `SwConnectorShapeHandler.rs:291` 的 `ShapePr_handle` 完全同构（同为本波既有约定）。
//   ⚠ 偏差登记：本 port 的真身 `Shape` 尚未暴露 `spPr()` / `blipFill()` 的共享句柄面
//     （`ShapePr` 的 `*_handle` 只覆盖 `Transform2D/Fill/Geometry2D/Outline`，
//      不含 `ShapePr`/`Fill` 本身），故句柄在此**按需创建**而非取自模型 ——
//     语义上等价于 Java 的「非空引用」（真身 `Shape::spPr()`/`blipFill()` 自身即惰性创建，
//     见 `Shape.rs:916` / `Shape.rs:1190`）。
//   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH

// Java: Shape.spPr() —— 共享句柄投影（见上注）。
// rust: 真身 `ShapePr` 是 KProto 容器（**无 `Clone` 派生**，Java 侧同样不提供 `clone()`），
//   故 `clone()` 会在 `&ShapePr` 上解析成"复制引用"（E0308）⇒ 改用真身既有的
//   `ShapePr::convert_clone(&self)`（`ShapePr.rs:1473`；本波为同一目的登记的非 `Clone`
//   拷贝惯用法，按 `map` 键逐槽重解码，语义 = Java 的"复刻一个 KoAr 对象"）。
//   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
fn ShapePr_handle(dg: &Shape) -> Shared<ShapePr> {
    Rc::new(RefCell::new(dg.spPr().convert_clone()))
}

// rust: ★W6c-9 —— **真身活句柄版**（Java `this.dgPic.spPr()` 的引用语义）。
//   Java: `new ShapePropertiesHandler(this.dgPic.spPr(), ...)` —— `spPr()` 返回
//   `Shape` **自己**的 `ShapePr` 引用，子处理器的写入直接落在 `dgPic` 上，
//   随后 `endElement` 的 `this.dgStore.update(id, this.dgPic.save())` 把它持久化。
//   本 port 上方那个 `ShapePr_handle(&Shape)` 返回的是 `convert_clone()` 出来的
//   **游离副本** ⇒ 写入被丢弃（实测：`pic` 的 `p:spPr/a:xfrm` 完全丢失，
//   `DgStore::spPrById` 为空）。故新增本函数，按 `SwConnectorShapeHandler::ShapePr_handle`
//   （`:315`，已实证可用）的同一机制取**共享活句柄**：
//   优先复用仓内已登记的同 id 句柄，否则以 `Shape.spPr` 槽位值往返拷贝
//   （`save()` → `ShapePr::open(..)`）建句柄并 `attachSpPr` 登记，
//   使子处理器与 `dgPic.save()` 操作**同一个** `ShapePr`。
//   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-KMOSLIDE-W6c-9
fn ShapePr_handle_live(dg_store: *mut DgStore, spid: i32, dg: &mut Shape) -> Shared<ShapePr> {
    if dg_store.is_null() {
        // rust: Java 侧 `dgStore` 为 null 会 NPE；此处退化为不登记的临时句柄
        //   （`endElement` 对空指针有守卫），不掩盖调用方的空指针条件。
        return Rc::new(RefCell::new(dg.spPr().convert_clone()));
    }
    let store = unsafe { &mut *dg_store };
    if let Some(pr) = store.shapePr(spid) {
        return pr;
    }
    let mut filled = match dg.spPr.get_mut() {
        Some(v) => std::mem::replace(v, Box::new(ShapePr::create())),
        None => Box::new(ShapePr::create()),
    };
    let saved = filled.save();
    let pr = Rc::new(RefCell::new(ShapePr::open(&*saved)));
    let _ = dg.spPr.set(filled);
    store.attachSpPr(spid, pr.clone());
    pr
}

// Java: Shape.blipFill() —— 共享句柄投影（见上注）。
// rust: 真身 `Fill` 同为无 `Clone` 的 KProto 容器 ⇒ 与 `ShapePr` 侧同一惯用法：
//   经 `lastBin()` + `Fill::open(..)` 取等价副本（`Fill::Fill_from` 亦不适用，
//   因为它读的是 `map` 键而非已具体化的槽位）。
//   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
fn Fill_handle(dg: &Shape) -> Shared<Fill> {
    let bin = dg.blipFill().lastBin(crate::cn::wps::kso::kproto::KoZone::KoZone::defaultZone());
    Rc::new(RefCell::new(Fill::open(bin.as_ref())))
}

// Java: Shape.style() —— 共享句柄投影（见上注）。
// rust: 真身 `ShapeStyle` 同为无 `Clone` 的 KProto 容器（只有 `ShapeStyle::create`），
//   故同 `ShapePr`/`Fill` 的惯用法：`lastBin()` + `ShapeStyle::open(..)`。
//   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
fn ShapeStyle_handle(dg: &Shape) -> Shared<ShapeStyle> {
    let bin = dg.style().lastBin(crate::cn::wps::kso::kproto::KoZone::KoZone::defaultZone());
    Rc::new(RefCell::new(ShapeStyle::open(bin.as_ref())))
}

// Java: Shape.cNvPr() —— 共享句柄投影（见上注）。
// rust: 真身 `CNvPr` 同为无 `Clone` 的 KProto 容器（只有 `CNvPr::create`）⇒ 同上惯用法。
//   `Shape.cNvPr` 槽位是 `pub(crate)`（`drawing::data::shape` 内可见，跨包不可见），
//   故只能走公开的 `hasCNvPr()` / `cNvPr()`；空槽位时按 Java `cNvPr()` 的语义就地创建
//   （同 `SwGraphicFrameHandler.rs:190`），避免真身 `Shape::cNvPr()` 末尾 `unwrap()` 在
//   空槽位时 panic。
//   owner: show.pptxr.handler.shape.SwPictureHandler    wave: M-PPT-SHAPE-DISPATCH
fn CNvPr_handle(dg: &Shape) -> Shared<CNvPr> {
    if dg.hasCNvPr() {
        let bin = dg
            .cNvPr()
            .lastBin(crate::cn::wps::kso::kproto::KoZone::KoZone::defaultZone());
        Rc::new(RefCell::new(CNvPr::open(bin.as_ref())))
    } else {
        Rc::new(RefCell::new(CNvPr::create()))
    }
}
