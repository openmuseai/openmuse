use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::BlipFillHandler::BlipFillHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotifyRef;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::GradientFillHandler::GradientFillHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::PatternFillHandler::PatternFillHandler;
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::SolidFillHandler::SolidFillHandler;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// rust: ★2B-③-2 —— 由影子 `layout_shape::FillObj` 改为**真身** `data::fill::Fill`。
//   本处理器只经 `fill.putType(..)` 写类型，真身 `Fill::putType` 语义一致。
use crate::cn::wps::drawing::data::fill::Fill::Fill;

pub use crate::cn::wps::drawing::dmlr::handler::shape::sppr::fill::FillNotify::FillNotify;

// java public class FillPropertiesHandler extends DefaultElementHandler
pub struct FillPropertiesHandler {
    // java private Fill fill
    fill: Rc<RefCell<Fill>>,
    // java private HandlerCallback handlerCallback
    handlerCallback: FillNotifyRef,
}

impl FillPropertiesHandler {
    // java public FillPropertiesHandler(Fill fill, HandlerCallback handlerCallback)
    pub fn FillPropertiesHandler(fill: Rc<RefCell<Fill>>) -> Self {
        Self {
            fill,
            handlerCallback: None,
        }
    }

    pub fn FillPropertiesHandler__callback(
        fill: Rc<RefCell<Fill>>,
        handlerCallback: FillNotifyRef,
    ) -> Self {
        Self {
            fill,
            handlerCallback,
        }
    }
}

impl ElementHandler for FillPropertiesHandler {
    fn kind(&self) -> &'static str {
        "FillPropertiesHandler"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114370:
            1114370 => {
                // java this.fill.putType(5);
                self.fill.borrow_mut().putType(5);
                None
            }
            // java case 1114371:
            1114371 => {
                // java this.fill.putType(0);
                self.fill.borrow_mut().putType(0);
                // java handler = new SolidFillHandler(this.fill.solidFill(), this.handlerCallback);
                Some(Box::new(SolidFillHandler::SolidFillHandler(
                    self.fill.clone(),
                    self.handlerCallback.clone(),
                )))
            }
            // java case 1114372:
            1114372 => {
                // java this.fill.putType(4);
                self.fill.borrow_mut().putType(4);
                // java handler = new GradientFillHandler(this.fill.gradFill(), this.handlerCallback);
                Some(Box::new(GradientFillHandler::GradientFillHandler(
                    self.fill.clone(),
                    self.handlerCallback.clone(),
                )))
            }
            // java case 1114373:
            1114373 => {
                // java this.fill.putType(1);
                self.fill.borrow_mut().putType(1);
                // java handler = new PatternFillHandler(this.fill.pattFill(), this.handlerCallback);
                Some(Box::new(PatternFillHandler::PatternFillHandler(
                    self.fill.clone(),
                    self.handlerCallback.clone(),
                )))
            }
            // java case 1114374:
            1114374 => {
                // java this.fill.putType(3);
                self.fill.borrow_mut().putType(3);
                None
            }
            // java case 3211586:
            3211586 => {
                // java this.fill.putType(2);
                self.fill.borrow_mut().putType(2);
                // java handler = new BlipFillHandler(this.fill.blipFill(), this.handlerCallback);
                Some(Box::new(BlipFillHandler::BlipFillHandler(
                    self.fill.clone(),
                    self.handlerCallback.clone(),
                )))
            }
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}
