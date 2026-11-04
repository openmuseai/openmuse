use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::color::ColorHandlerFactory::ColorHandlerFactory;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotifyRef;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: ★2B-③-2 —— 由影子 `layout_shape::FillObj` 改为**真身** `data::fill::Fill`。
use crate::cn::wps::drawing::data::fill::Fill::Fill;

// java public class SolidFillHandler extends DefaultElementHandler
pub struct SolidFillHandler {
    // java private Color color = null
    color: Rc<RefCell<Fill>>,
    // java private HandlerCallback handlerCallback
    handlerCallback: FillNotifyRef,
}

impl SolidFillHandler {
    // java public SolidFillHandler(Color color, HandlerCallback handlerCallback)
    pub fn SolidFillHandler(fill: Rc<RefCell<Fill>>, handlerCallback: FillNotifyRef) -> Self {
        Self {
            color: fill,
            handlerCallback,
        }
    }
}

impl ElementHandler for SolidFillHandler {
    fn kind(&self) -> &'static str {
        "SolidFillHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java return ColorHandlerFactory.createColorHandler(subElement, this.color, this.handlerCallback, 1114371);
        // rust: ★2B-③-2 —— 真身 `Fill` 的 `Color` 经 `solidFill_mut()` 取得；
        //   `slot` 仅作子处理器读取用的临时镜像，写回目标已是真 `Fill`。
        let slot = {
            let mut f = self.color.borrow_mut();
            let c = f.solidFill_mut();
            crate::cn::wps::show::util::layout_shape::ColorSlot {
                mode: Some(c.mode()),
                srgb: None,
                scheme: None,
            }
        };
        ColorHandlerFactory::createColorHandler(
            subElement,
            Rc::new(RefCell::new(slot)),
            Some(self.color.clone()),
            self.handlerCallback.clone(),
            1114371,
        )
    }
    fn endElement(&mut self, _element: i32) {}
}
