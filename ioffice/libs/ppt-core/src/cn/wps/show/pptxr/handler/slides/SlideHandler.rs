// Java: public class SlideHandler extends DefaultElementHandler
//
// `p:sld` 事件装配成真实 `app::shape::KmoShowShape` 树。
// spid 来自 `DgStore.add`，组子节点在元素结束时 `addChild`。
// `p:timing` / `p:transition` 返回 null：不崩、也不把幻灯片子树吃掉。
// `app::slide::KmoSlide` 仍未参与编译，树根先挂在调用方传入的形状上。

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use crate::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::{
    assert_store_holds, ImportHost,
};
use crate::cn::wps::show::pptxr::handler::slides::ColorMapOverrideHandler::ColorMapOverrideHandler;
use crate::cn::wps::show::pptxr::handler::slides::CommonSlideDataHandler::LayoutCommonSlideData;
// Java: import cn.wps.drawing.data.style.ColorMapping
use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;

// Java: public class SlideHandler extends DefaultElementHandler
pub struct SlideHandler {
    host: &'static ImportHost,
    root: Rc<RefCell<KmoShowShape>>,
    cnvpr_ids: Rc<Cell<i32>>,
    // Java: slideData.putShowMasterSp
    show_master_sp: Option<bool>,
    // Java: `this.slideData.clrMap()` 的**共享句柄**槽（无 Java 对应字段）。
    //   Java 的 `clrMap()` 返回 `KmoSlide` 自己的 `ColorMapping` 引用，`ColorMapOverrideHandler`
    //   长期持有它并按 `p:clrMapOvr` 写回；Rust 的 `&mut T` 无法跨处理器持有 ⇒ 按
    //   `ShapePr::{fill,ln,..}_handle`（PLAN §27.4 2B-②）的既有约定给出 `Rc<RefCell<_>>` 句柄。
    //   owner: show.pptxr.handler.slides.SlideHandler    wave: M-PPT-KMOSLIDE-W6c-3
    clr_map: Rc<RefCell<ColorMapping>>,
}

impl SlideHandler {
    // Java: SlideHandler(KmoSlide slide, DgStore dgStore, HandlerCallback handlerCallback)
    pub fn SlideHandler(host: &'static ImportHost) -> Self {
        let root = KmoShowShape::KmoShowShape__islide_int(host, 0);
        assert_store_holds(host, root.id());
        Self {
            host,
            root: Rc::new(RefCell::new(root)),
            cnvpr_ids: Rc::new(Cell::new(0)),
            show_master_sp: None,
            // Java: `KmoSlide` 的 `ColorMapping` 槽在本 port 由本处理器持有（同一对象）。
            clr_map: Rc::new(RefCell::new(ColorMapping::create())),
        }
    }

    pub fn take_root(&self) -> KmoShowShape {
        self.root.borrow().clone()
    }

    pub fn cnvpr_count(&self) -> i32 {
        self.cnvpr_ids.get()
    }

    pub fn show_master_sp(&self) -> Option<bool> {
        self.show_master_sp
    }

    fn root(&self) -> Rc<RefCell<KmoShowShape>> {
        self.root.clone()
    }

    fn cnvpr_ids(&self) -> Rc<Cell<i32>> {
        self.cnvpr_ids.clone()
    }

    // rust: `this.slideData.clrMap()` 的共享句柄访问器（Java 中由 `KmoSlide` 提供）。
    //   owner: show.pptxr.handler.slides.SlideHandler    wave: M-PPT-KMOSLIDE-W6c-3
    pub fn clr_map(&self) -> Rc<RefCell<ColorMapping>> {
        self.clr_map.clone()
    }
}

impl ElementHandler for SlideHandler {
    fn kind(&self) -> &'static str {
        "SlideHandler"
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrs: &XmlRoAttrList) {
        // Java: case 3211600: this.slideData.putShowMasterSp(tmp.toBoolean())
        if let Some(tmp) = attrs.get_by_name(3211600) {
            self.show_master_sp = Some(tmp.to_boolean());
        }
        // Java: case 3211601: this.slideData.putShowMasterPhAnim(tmp.toBoolean())
        // Java: case 3211610: this.slideData.putShow(tmp.toBoolean())
        let _ = attrs.get_by_name(3211601);
        let _ = attrs.get_by_name(3211610);
    }

    // Java: ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, sub_element: i32) -> Option<Box<dyn ElementHandler>> {
        match sub_element {
            // Java: case 2162692: TransitionAlternateContentHandler — 不在本里程碑
            2162692 => None,
            // Java: case 3211295: return null
            3211295 => None,
            // Java: case 3211402: TransitionHandler — 不在本里程碑
            3211402 => None,
            // Java: case 3211599: return new ColorMapOverrideHandler(this.slideData.clrMap())
            //
            // rust: ★W6c-3 —— 真身接线。原先此处返回 `None`（「不改变形状树」的旧说辞），
            //   但 Java 确实构造 `ColorMapOverrideHandler` 并按 `p:clrMapOvr` 写回幻灯片自己的
            //   `ColorMapping`（**影响渲染配色解析**，属管线语义）。`ColorMapOverrideHandler`
            //   实现 **② 契约**（与本文件同一契约）⇒ 直接装箱，无需适配器。
            //   owner: show.pptxr.handler.slides.SlideHandler    wave: M-PPT-KMOSLIDE-W6c-3
            3211599 => Some(Box::new(ColorMapOverrideHandler::ColorMapOverrideHandler(
                self.clr_map(),
            ))),
            // Java: case 3211608: return new CommonSlideDataHandler(this.mSlide.film(), ...)
            3211608 => Some(Box::new(LayoutCommonSlideData::new(
                self.host,
                self.root(),
                self.cnvpr_ids(),
            ))),
            // Java: case 3211609: TimingHandler — 不在本里程碑
            3211609 => None,
            _ => None,
        }
    }

    // Java: void endElement(int element) — updateProtocol(slideData.save())
    fn endElement(&mut self, _element: i32) {}
}
