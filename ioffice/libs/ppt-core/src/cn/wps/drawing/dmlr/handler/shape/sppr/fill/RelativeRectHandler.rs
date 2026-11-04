use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::show::util::layout_shape::SrcRectObj;

// java public class RelativeRectHandler extends DefaultElementHandler
pub struct RelativeRectHandler {
    // java private RelativeRect relativeRect
    relativeRect: Rc<RefCell<SrcRectObj>>,
}

impl RelativeRectHandler {
    // java public RelativeRectHandler(RelativeRect relativeRect)
    pub fn RelativeRectHandler(relativeRect: Rc<RefCell<SrcRectObj>>) -> Self {
        Self { relativeRect }
    }
}

impl ElementHandler for RelativeRectHandler {
    fn kind(&self) -> &'static str {
        "RelativeRectHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java if ((temp = attrsNotCopy.getByName(1114213)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114213) {
            // java this.relativeRect.putLeft(temp.toPercentage());
            self.relativeRect.borrow_mut().l = temp.to_percentage();
        }
        // java if ((temp = attrsNotCopy.getByName(1114214)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114214) {
            // java this.relativeRect.putTop(temp.toPercentage());
            self.relativeRect.borrow_mut().t = temp.to_percentage();
        }
        // java if ((temp = attrsNotCopy.getByName(1114193)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114193) {
            // java this.relativeRect.putRight(temp.toPercentage());
            self.relativeRect.borrow_mut().r = temp.to_percentage();
        }
        // java if ((temp = attrsNotCopy.getByName(1114195)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114195) {
            // java this.relativeRect.putBottom(temp.toPercentage());
            self.relativeRect.borrow_mut().b = temp.to_percentage();
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}
