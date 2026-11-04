use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::shape::sppr::geometry::GeomGuideHandler::GeomGuideHandler;
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::show::util::layout_shape::GeomGuideObj;

// java public class AdjustValueListHanlder extends DefaultElementHandler
pub struct AdjustValueListHanlder {
    // java private List<GeomGuide> adjustableValues
    adjustableValues: Rc<RefCell<Vec<Rc<RefCell<GeomGuideObj>>>>>,
}

impl AdjustValueListHanlder {
    // java public AdjustValueListHanlder(List<GeomGuide> adjustableValues)
    pub fn AdjustValueListHanlder(
        adjustableValues: Rc<RefCell<Vec<Rc<RefCell<GeomGuideObj>>>>>,
    ) -> Self {
        Self { adjustableValues }
    }
}

impl ElementHandler for AdjustValueListHanlder {
    fn kind(&self) -> &'static str {
        "AdjustValueListHanlder"
    }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    // java public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        match subElement {
            // java case 1114392:
            1114392 => {
                let guide = Rc::new(RefCell::new(GeomGuideObj::default()));
                self.adjustableValues.borrow_mut().push(guide.clone());
                Some(Box::new(GeomGuideHandler::GeomGuideHandler(guide)))
            }
            _ => None,
        }
    }
    fn endElement(&mut self, _element: i32) {}
}
