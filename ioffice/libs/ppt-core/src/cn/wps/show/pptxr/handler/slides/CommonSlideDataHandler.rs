#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// M-PPT-KMOSLIDE W9 —— 本文件是 W9 的**核心修复点**。
//
// ★W9 前的事实（可复现）：Java `CommonSlideDataHandler.java:47` 是
//   `handler = new CustDataLstHandler(this.commonSlideData.custDataLst(), this.handlerCallback);`
//   而 Rust 侧本文件导入的是 `shape::sw_shape_hosts::CustDataLstHandler` —— 一个**同名单元空壳**
//   （`sw_shape_hosts.rs` 的 `pub struct CustDataLstHandler;`，`new<L, C>(_, _)` 丢弃两个实参），
//   调用点因此写成 `CustDataLstHandler::new((), self.handlerCallback.clone())`。同类还有
//   `sw_shape_hosts::{CommonSlideData, BackgroundHandler, ControlListHandler}` 三个空壳。
//   ⇒ `p:custDataLst`(3211519) 里解析出的 `p:tag` 永不进入模型，`handlerCallback.onTags(...)`
//   从不被调用。W9 已把 3211519 接到**真实** `custdatas::CustDataLstHandler`，并删除空壳
//   `sw_shape_hosts::{CommonSlideData, CustDataLstHandler, ControlListHandler}`（见各文件头）。
//
// ★契约（R14）：规范契约 `kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::ElementHandler`
//   （`startElement(&self, i32, &dyn XmlRoAttr)` / `endElement(&self, i32)` /
//     `characters(&self, &str)` / `enterSubElement(&self, i32) -> Option<Box<dyn ElementHandler>>`），
//   即 `UnifiedXmlReader::read__inputstream_elementhandler` 的形参类型；全部接收者为 `&self`。
//   **不得**加 `kind()`（非该 trait 成员）；**不得**写 `impl ElementHandler for X {}`
//   （该 trait 无默认体）。
//
// ★rust 偏差登记（owner: show.pptxr.handler.slides · wave: M-PPT-KMOSLIDE-W9）：
//   1) 规范契约与影子契约（`sw_shape_hosts::{ElementHandler, XmlRoAttr}`，接收者 `&mut self`，
//      且**没有** `characters`）并存。本文件是二者的**唯一交汇点**：3211606 的目标
//      `SwGroupShapeHandler` 目前只实现影子契约（它自带的 4 个下游 handler
//      `SwPictureHandler` / `SwConnectorShapeHandler` / `SwShapeAlertnateContentHandler` /
//      `SwNvPrHandler` 均未登记，前置 M-PPT-SHAPE-DISPATCH，见 `shape/mod.rs` 实测表），
//      故此处用 `ShadowElementAsCanonical` 适配器把影子子树挂到规范契约下。
//      Java 只有一个 `ElementHandler` 接口；Rust 的 `XmlRoAttr` 是 trait，两个 trait 的
//      `getByName` 返回**各自**的 `&dyn XmlRoAttr`，无法互转 —— 适配器因此自带
//      `CanonicalAttrsAsShadow` 属性视图（按 `count()` / `getAttrAt(i)` 逐项投影），
//      不做任何字段级改写，取值语义与 Java 逐字一致。
//   2) `startElement` 保留一个**固有**重载，形参为 `&dyn sw_shape_hosts::XmlRoAttr`：
//      门禁 target `04_function_common_slide_data` 的本地 `Attrs` 只实现影子 `XmlRoAttr`
//      （tests/ 不可改）；规范契约版实现同名方法同时存在，固有方法在方法调用语法中优先解析。
//   3) `commonSlideData` 是**真实** `show::data::CommonSlideData`（1013 行），不再是空壳。
//      `get_custDataLst_mut() -> &mut CustDataLst` 无法投影出 `Rc<RefCell<CustDataLst>>`，
//      故 3211519 把**属主句柄** `Shared<CommonSlideData>` 交给 `CustDataLstHandler`
//      （其文件头有同项登记；写入的仍是模型里同一个 `CustDataLst`，`endElement` 的
//      `commonSlideData.save()` 随后把它序列化进 film 协议）。
//
// ★p:bg(3211308) 的遗留阻塞（已实测，不在本波文件面内）：真实
//   `show::data::background::Background.rs`（833 行）在默认构建下**不可达** ——
//   `show/mod.rs` 用 `#[cfg(not(feature = "full"))] pub mod data { ... }` 内联桩模块顶替，
//   该内联桩不登记 `background`；`CommonSlideData` 顶部的内联简化桩 `Background` 也**没有**
//   `putBwMode` / `putShadeToTitle` / `bgRef()` / `fill()` / `effect()`，且
//   `show/data/CommonSlideData.rs` 不在本波文件面内。因此 3211308 只能继续返回影子
//   `sw_shape_hosts::BackgroundHandler`（单元空壳，Java 的 `bwMode` 语义仍未接线），
//   该空壳**因此不能删除**。可达性：产品 MVP 入口 `pptx_mvp::write_app_film` 只喂 3211606，
//   不触达 3211308。owner: show.pptxr.handler.slides · wave: M-PPT-KMOSLIDE-W9 ·
//   blocker: show.data.background 收口 wave。

// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::DefaultElementHandler::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler as CanonicalElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
// rust: Java `XmlRoAttr` 是**接口**，Rust 侧是 **trait** ⇒ 形参一律 `&dyn XmlRoAttr`。
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr as CanonicalXmlRoAttr;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import cn.wps.show.app.slide.KmoFilm
use crate::cn::wps::show::app::slide::KmoFilm::KmoFilm;
// Java: import cn.wps.show.data.CommonSlideData
// rust: W9 前此处是空壳 `sw_shape_hosts::CommonSlideData`；现改为**真实**模型类。
use crate::cn::wps::show::data::CommonSlideData::CommonSlideData;
// Java: import cn.wps.show.pptxr.handler.custdatas.CustDataLstHandler
// rust: W9 前此处是空壳 `sw_shape_hosts::CustDataLstHandler`；现改为**真实**类。
use crate::cn::wps::show::pptxr::handler::custdatas::CustDataLstHandler::{
    CustDataLstHandler, TagLstSlot,
};
// Java: import cn.wps.show.pptxr.handler.shape.SwGroupShapeHandler
use crate::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::SwGroupShapeHandler;
// Java: import cn.wps.show.pptxr.handler.slides.BackgroundHandler
// rust: 真实 `slides/BackgroundHandler.rs` 当前不可登记（见文件头 p:bg 遗留阻塞）⇒ 仍用影子。
// rust: ★W6c-3 —— 真身（② 契约），原先因 `show.data.background` 不可达而只能返回影子空壳。
use crate::cn::wps::show::pptxr::handler::slides::BackgroundHandler::BackgroundHandler;
// Java: import cn.wps.show.pptxr.handler.slides.ControlListHandler
// rust: W9 前此处是空壳；现改为**真实**类（`slides/ControlListHandler.rs`，已登记）。
use crate::cn::wps::show::pptxr::handler::slides::ControlListHandler::ControlListHandler;
// rust: ★③-5 —— ② dmlr 契约（真 handler 所在契约）与其 attrs 类型，供 `DmlrElementAsCanonical` 桥使用。
use crate::cn::wps::drawing::dmlr::handler::ElementHandler as ElementHandler;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrLeaf::XmlRoAttrLeaf;
// rust: ★③-5 去假面（形态 F）—— `ShapePr`/`ShapeStyle` 原为 `sw_shape_hosts` 的**同名空壳**
//   （单元结构），使本类无法以真 handler 的 `HandlerCallback` 约束实例化（E0271）。
//   改为**真身** `drawing::data::{shape::ShapePr, style::ShapeStyle}`，与 `SwGroupShapeHandler`
//   / `SwShapeHandler` / `SwPictureHandler` 的约束一致。
//   owner: show.pptxr.handler.slides.CommonSlideDataHandler    wave: M-PPT-SHAPE-DISPATCH
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
// rust: `FileManager` 真身在独立 crate `core_libs_util`（Java 同包同名），
//   与 `SwGroupShapeHandler`/`SwShapeAlertnateContentHandler` 的约束一致。
use util::cn::wps::util::manager::FileManager::FileManager;

use std::cell::RefCell;
use std::rc::Rc;

// rust: 别名（Java 无）—— `Rc<RefCell<..>>` 承载 Java 的共享可变引用语义（repo 既有约定）。
type Shared<T> = Rc<RefCell<T>>;
type Callback<C> = Option<Shared<C>>;

pub(crate) fn snapshot_kmo(src: &KmoShowShape) -> KmoShowShape {
    let parent = src.parent().expect("NullPointerException");
    let mut out =
        KmoShowShape::KmoShowShape__islide_int_int(unsafe { &*parent }, src.r#type(), src.id());
    for i in 0..src.childCount() {
        out.addChild(snapshot_kmo(src.childAt(i)));
    }
    out
}

// ---------------------------------------------------------------------------
// rust: ★M-PPT-KMOSLIDE W6c-3 —— 原先此处是**影子契约（①）↔ 规范契约（③）**的两个适配器
//   `CanonicalAttrsAsShadow` / `ShadowElementAsCanonical`（假编译面形态 F 的最后两个消费者）。
//   本波已把 `p:bg`(3211308) 接到**真身** `slides::BackgroundHandler`（② 契约，见该文件），
//   影子 ① 契约已零消费者，故随 `shape/sw_shape_hosts.rs` 一并删除。
//   保留的是 **② dmlr 契约 → ③ kfc_alpha 契约** 的反向桥 `DmlrElementAsCanonical`
//   （本文件是 ③，而真 handler 族收敛在 ②）。
//   owner: show.pptxr.handler.slides    wave: M-PPT-KMOSLIDE-W6c-3
// ---------------------------------------------------------------------------

// rust: ★③-5 —— **② dmlr 契约 → ③ kfc_alpha 契约** 适配器。
//   背景（PLAN §26.1 的契约三分法）：
//     · ③ `kfc_kfc_alpha::…::ElementHandler`（`&self`，attrs `&dyn XmlRoAttr`）—— 本文件
//       `CommonSlideDataHandler` 所在的契约，也是 `UnifiedXmlReader::read__inputstream_elementhandler`
//       的形参类型；
//     · ② `drawing::dmlr::handler::ElementHandler`（`&mut self`，`kind()`，attrs `&XmlRoAttrList`）
//       —— **真 S2 引擎** `UnifiedXmlReader::read__stream_elementhandler` 实际使用的活契约，
//       也是全部真 handler（`SwGroupShapeHandler`/`SwPictureHandler`/…）收敛到的契约。
//   桥的语义：`&self` 接收者由 `RefCell` 承载可变状态（Java 侧 handler 本就是可变对象），
//   attrs 侧把 ③ 的叶子逐项投影为 ② 的 `XmlRoAttrList`（`getByName` / `toString` / `toBoolean`
//   取值语义逐字一致，不做字段级改写）。
//   owner: show.pptxr.handler.slides.CommonSlideDataHandler    wave: M-PPT-SHAPE-DISPATCH
struct DmlrElementAsCanonical(RefCell<Box<dyn ElementHandler>>);

/// ③ 的属性视图 → ② 的 `XmlRoAttrList`。
/// Java 里 `XmlRoAttr` 只有**一个**接口，`getByName` 返回同一族对象；
/// 两个 Rust trait 的返回类型各自绑定自己的 trait object，故按 `count()`/`get_attr_at(i)`
/// 逐项投影。
struct CanonicalAttrsAsDmlr;

impl CanonicalAttrsAsDmlr {
    // Java: XmlRoAttr 只有一族实现；Rust 两契约的返回类型各自绑定自己的 trait object。
    fn wrap(attrs: &dyn CanonicalXmlRoAttr) -> XmlRoAttrList {
        let mut list = XmlRoAttrList::new();
        // ② 的 `XmlRoAttrList` 以 `add_attr(name, value)` 逐项累积，
        // 与 ③ 的 `count()`/`get_attr_at(i)` 一一对应，**顺序保持不变**。
        for i in 0..attrs.count() {
            // rust: ③ 契约的 `get_attr_at` 返回 `Option`（Java 的同名方法返回可空引用）
            //   ⇒ 空项按 Java 的 `null` 跳过（不产生条目）。
            if let Some(attr) = attrs.get_attr_at(i) {
                list.add_attr(attr.get_name(), &attr.to_string());
            }
        }
        list
    }
}

impl CanonicalElementHandler for DmlrElementAsCanonical {
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, element: i32, attrsNotCopy: &dyn CanonicalXmlRoAttr) {
        let attrs = CanonicalAttrsAsDmlr::wrap(attrsNotCopy);
        self.0.borrow_mut().startElement(element, &attrs);
    }

    // Java: public void endElement(int element)
    fn endElement(&self, element: i32) {
        self.0.borrow_mut().endElement(element);
    }

    // Java: public void characters(String chars)
    fn characters(&self, chars: &str) {
        self.0.borrow_mut().characters(chars);
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn CanonicalElementHandler>> {
        self.0
            .borrow_mut()
            .enterSubElement(subElement)
            .map(|inner| Box::new(DmlrElementAsCanonical(RefCell::new(inner)))
                as Box<dyn CanonicalElementHandler>)
    }
}

// Java: public class CommonSlideDataHandler extends DefaultElementHandler
pub struct CommonSlideDataHandler<C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink, FileManager = FileManager>> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java field: KmoFilm film
    film: Shared<KmoFilm>,
    // Java field: CommonSlideData commonSlideData（真实模型类，见文件头偏差登记 3）
    pub(crate) commonSlideData: Shared<CommonSlideData>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Callback<C>,
    // Java field: DgStore dgStore
    dgStore: *mut DgStore,
    // Java field: KmoShowShape shapeTree
    pub(crate) shapeTree: Shared<KmoShowShape>,
}

// Java: public class CommonSlideDataHandler extends DefaultElementHandler
impl<C> CommonSlideDataHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink, FileManager = FileManager> + 'static,
    C::TagLst: TagLstSlot<C>,
{
    // Java: CommonSlideDataHandler(KmoFilm film, DgStore dgStore, HandlerCallback handlerCallback)
    pub fn CommonSlideDataHandler(
        film: Shared<KmoFilm>,
        dgStore: *mut DgStore,
        handlerCallback: Callback<C>,
    ) -> Self {
        // Java: this.film = film;
        let film = film;
        // Java: this.handlerCallback = handlerCallback;
        let handlerCallback = handlerCallback;
        // Java: this.dgStore = dgStore;
        let dgStore = dgStore;
        // Java: this.shapeTree = new KmoShowShape(film.parent(), 0);
        let parent = film.borrow().parent().expect("NullPointerException");
        let shapeTree = Rc::new(RefCell::new(KmoShowShape::KmoShowShape__islide_int(
            unsafe { &*parent },
            0,
        )));
        // Java: this.commonSlideData = CommonSlideData.create();
        let commonSlideData = Rc::new(RefCell::new(CommonSlideData::create()));
        CommonSlideDataHandler {
            parent: DefaultElementHandler::new(),
            film,
            commonSlideData,
            handlerCallback,
            dgStore,
            shapeTree,
        }
    }

    // Java: this.commonSlideData.putName(tmp.toString())
    // rust: 两个契约的 `startElement` 共用的落点（属性提取各自 1 行，逻辑同源）。
    pub fn putName(&self, name: &str) {
        self.commonSlideData.borrow_mut().putName(name);
    }

    // Java: void endElement(int element)
    pub fn endElement(&mut self, element: i32) {
        self.endElement_body(element);
    }

    // Java: void endElement(int element)
    fn endElement_body(&self, _element: i32) {
        // Java: this.film.spTree().initRoot(this.shapeTree);
        let snapshot = snapshot_kmo(&self.shapeTree.borrow());
        self.film.borrow_mut().spTree().initRoot(snapshot);
        // Java: this.film.updateProtocol(this.commonSlideData.save());
        self.film
            .borrow_mut()
            .updateProtocol(self.commonSlideData.borrow_mut().save());
    }

    // Java: ElementHandler enterSubElement(int subElement)
    // rust: 固有重载 —— 规范契约的 `enterSubElement` 与本方法同体（见文件头偏差登记 2）。
    //   产品入口 `pptx_mvp::write_app_film` 与门禁 target 都用方法调用语法直接调用本方法，
    //   二者所在文件不在本波文件面内，故保留该固有入口。
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn CanonicalElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn CanonicalElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3211295:
            // Java: default:
            // Java: break;
            3211295 => {
                // Java: return handler;  ⇒ handler 仍为 null
                handler = None;
            }
            // Java: case 3211308:
            3211308 => {
                // Java: handler = new BackgroundHandler(this.commonSlideData.background(), this.handlerCallback);
                //
                // rust: ★W6c-3 —— **真实接线**：原先此处构造影子空壳
                //   `sw_shape_hosts::BackgroundHandler`（单元结构体 ⇒ 不解析任何属性，`bwMode`
                //   语义整体丢失），现改为真身 `slides/BackgroundHandler`，并按既有约定经
                //   `DmlrElementAsCanonical`（②→③）挂到本文件的规范契约下。
                //   `background()` 是 Java 的就地惰性装配访问器；Rust 侧 `&mut T` 无法跨处理器
                //   持有 ⇒ 用 `Background::background_handle()` 的**共享句柄**承载同一对象
                //   （与 `ShapePr::{fill,ln,..}_handle` 同一约定，PLAN §27.4 2B-②）。
                //   owner: show.pptxr.handler.slides.CommonSlideDataHandler    wave: M-PPT-KMOSLIDE-W6c-3
                let background = self.commonSlideData.borrow_mut().background_handle();
                handler = Some(Box::new(DmlrElementAsCanonical(RefCell::new(Box::new(
                    BackgroundHandler::BackgroundHandler(background, self.handlerCallback.clone()),
                )))));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211519:
            3211519 => {
                // ★W9 核心修复：Java
                //   handler = new CustDataLstHandler(this.commonSlideData.custDataLst(), this.handlerCallback);
                //   此前此处是 `sw_shape_hosts::CustDataLstHandler` 空壳（`new((), cb)` 丢弃
                //   custDataLst）⇒ `p:custDataLst` 的 tags 从未进入模型。现接**真实**类。
                // rust: `get_custDataLst_mut()` 只能给出 `&mut CustDataLst`，无法得到
                //   `Rc<RefCell<CustDataLst>>` ⇒ 传**属主句柄**，由 `TagsHandler::startElement`
                //   惰性取 `get_custDataLst_mut().tags()`（同一对象，见 `CustDataLstHandler.rs` 文件头）。
                handler = Some(Box::new(CustDataLstHandler::<C>::new(
                    self.commonSlideData.clone(),
                    self.handlerCallback.clone(),
                )));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211606:
            3211606 => {
                // Java: handler = new SwGroupShapeHandler(this.shapeTree, this.handlerCallback);
                // rust: ★③-5 —— 适配器退场。`SwGroupShapeHandler` 现已落在 **② 规范契约**
                //   （`drawing::dmlr::handler::ElementHandler`）上（其影子导入已全部摘除，见
                //   `shape/SwGroupShapeHandler.rs` 文件头），故与 Java 同形**直接**构造，
                //   不再经 `ShadowElementAsCanonical` 投影。Java 侧本来就只有一条
                //   `new SwGroupShapeHandler(...)`，此处恢复该形状。
                //   owner: show.pptxr.handler.slides.CommonSlideDataHandler
                //   wave: M-PPT-SHAPE-DISPATCH
                // rust: ★③-5 —— 本文件在 **③ kfc_alpha** 契约上（`&self`），而
                //   `SwGroupShapeHandler` 本波已收敛到 **② dmlr** 契约（`&mut self`）⇒
                //   经 `DmlrElementAsCanonical` 桥接（②→③，见该类型注释）。
                //   Java 只有一个接口，本桥只做契约投影，不改任何取值语义。
                //   owner: show.pptxr.handler.slides.CommonSlideDataHandler
                //   wave: M-PPT-SHAPE-DISPATCH
                handler = Some(Box::new(DmlrElementAsCanonical(RefCell::new(Box::new(
                    SwGroupShapeHandler::<C>::SwGroupShapeHandler(
                        self.shapeTree.clone(),
                        self.handlerCallback.clone(),
                    ),
                )))));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3211607:
            3211607 => {
                // Java: handler = new ControlListHandler(this.shapeTree, this.dgStore, this.handlerCallback);
                // rust: W9 前是空壳 `sw_shape_hosts::ControlListHandler`；现为**真实**类。
                handler = Some(Box::new(ControlListHandler::<C>::ControlListHandler(
                    self.shapeTree.clone(),
                    self.dgStore,
                    self.handlerCallback.clone(),
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

// Java: public class CommonSlideDataHandler extends DefaultElementHandler
impl<C> CanonicalElementHandler for CommonSlideDataHandler<C>
where
    C: HandlerCallback<ShapePr = ShapePr, ShapeStyle = ShapeStyle, Hyperlink = Hyperlink, FileManager = FileManager> + 'static,
    C::TagLst: TagLstSlot<C>,
{
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn CanonicalXmlRoAttr) {
        // Java: XmlRoAttr tmp = null;
        // Java: if ((tmp = attrsNotCopy.getByName(3211449)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(3211449) {
            // Java: this.commonSlideData.putName(tmp.toString());
            self.putName(&tmp.to_string());
        }
    }

    // Java: public void endElement(int element)
    fn endElement(&self, element: i32) {
        // Java: super.endElement(element);
        self.endElement_body(element);
    }

    // Java: public void characters(String chars)
    fn characters(&self, _chars: &str) {
        // Java: （继承 DefaultElementHandler 的空实现）
    }

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn CanonicalElementHandler>> {
        // Java: return handler;
        CommonSlideDataHandler::enterSubElement(self, subElement)
    }
}

/// Java `CommonSlideDataHandler` 在产品模型上的装配。
/// `p:spTree`(3211606) 交给真实 `KmoShowShape` 组。
pub struct LayoutCommonSlideData {
    host: &'static crate::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::ImportHost,
    root: std::rc::Rc<std::cell::RefCell<crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape>>,
    cnvpr_ids: std::rc::Rc<std::cell::Cell<i32>>,
}

impl LayoutCommonSlideData {
    pub fn new(
        host: &'static crate::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::ImportHost,
        root: std::rc::Rc<std::cell::RefCell<crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape>>,
        cnvpr_ids: std::rc::Rc<std::cell::Cell<i32>>,
    ) -> Self {
        Self {
            host,
            root,
            cnvpr_ids,
        }
    }
}

impl crate::cn::wps::drawing::dmlr::handler::ElementHandler for LayoutCommonSlideData {
    fn kind(&self) -> &'static str {
        "CommonSlideDataHandler"
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
        match sub_element {
            // Java: case 3211295: break
            3211295 => None,
            // Java: case 3211308: BackgroundHandler — 背景不在形状树计数里
            3211308 => None,
            // Java: case 3211519: CustDataLstHandler
            3211519 => None,
            // Java: case 3211606: return new SwGroupShapeHandler(this.shapeTree, ...)
            3211606 => Some(Box::new(
                crate::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::LayoutSwGroup::container(
                    self.root.clone(),
                    self.host,
                    self.cnvpr_ids.clone(),
                ),
            )),
            // Java: case 3211607: ControlListHandler
            3211607 => None,
            _ => None,
        }
    }
    // Java: this.film.spTree().initRoot(this.shapeTree)
    // 子形状在 `SwGroupShapeHandler.endElement` 里已经写入 `shapes`。
    fn endElement(&mut self, _element: i32) {}
}
