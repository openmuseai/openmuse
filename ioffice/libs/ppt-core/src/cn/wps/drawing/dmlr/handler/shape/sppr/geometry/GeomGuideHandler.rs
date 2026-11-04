use std::cell::RefCell;
use std::rc::Rc;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
use crate::cn::wps::show::util::layout_shape::GeomGuideObj;

// java public class GeomGuideHandler extends DefaultElementHandler
pub struct GeomGuideHandler {
    // java private GeomGuide guide
    guide: Rc<RefCell<GeomGuideObj>>,
}

impl GeomGuideHandler {
    // java public GeomGuideHandler(GeomGuide guide)
    pub fn GeomGuideHandler(guide: Rc<RefCell<GeomGuideObj>>) -> Self {
        let mut this = Self {
            guide: guide.clone(),
        };
        this.init(guide);
        this
    }

    // java public void init(GeomGuide guide)
    pub fn init(&mut self, guide: Rc<RefCell<GeomGuideObj>>) {
        self.guide = guide;
    }
}

impl ElementHandler for GeomGuideHandler {
    fn kind(&self) -> &'static str {
        "GeomGuideHandler"
    }
    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
        if let Some(temp) = attrsNotCopy.get_by_name(1114391) {
            self.guide.borrow_mut().putFmla(temp.to_string());
        }
        if let Some(temp) = attrsNotCopy.get_by_name(1114136) {
            self.guide.borrow_mut().putName(temp.to_string());
        }
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}
