use std::cell::RefCell;
use std::rc::Rc;

use crate::cn::wps::drawing::dmlr::handler::color::RgbColorModelHexHandler::RgbColorModelHexHandler;
use crate::cn::wps::drawing::dmlr::handler::color::SchemeColorHandler::SchemeColorHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotifyRef;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: ★2B-③-2 —— `FillObj`（影子）改为**真身** `data::fill::Fill`。
use crate::cn::wps::show::util::layout_shape::ColorSlot;
use crate::cn::wps::drawing::data::fill::Fill::Fill;

// java public class ColorHandlerFactory
pub struct ColorHandlerFactory {}

impl ColorHandlerFactory {
    // java public static ElementHandler createColorHandler(int subElement, Color color, HandlerCallback handlerCallback, int belonger)
    pub fn createColorHandler(
        subElement: i32,
        color: Rc<RefCell<ColorSlot>>,
        fill: Option<Rc<RefCell<Fill>>>,
        handlerCallback: FillNotifyRef,
        belonger: i32,
    ) -> Option<Box<dyn ElementHandler>> {
        // java switch (subElement)
        match subElement {
            // java case 1114215:
            1114215 => None,
            // java case 1114216:
            1114216 => Some(Box::new(RgbColorModelHexHandler::RgbColorModelHexHandler(
                color, fill,
            ))),
            // java case 1114217:
            1114217 => None,
            // java case 1114218:
            1114218 => None,
            // java case 1114219:
            1114219 => Some(Box::new(SchemeColorHandler::SchemeColorHandler(
                color,
                fill,
                handlerCallback,
                belonger,
            ))),
            // java case 1114220:
            1114220 => None,
            // java default:
            _ => None,
        }
    }
}
