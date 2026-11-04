// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler as ElementHandlerContract;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use crate::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// Java: import cn.wps.show.data.HeaderFooter
use crate::cn::wps::show::data::HeaderFooter::HeaderFooter;

use std::cell::RefCell;
use std::rc::Rc;

// Java: public class HeaderFooterHandler extends DefaultElementHandler
pub struct HeaderFooterHandler {
    // Java field: HeaderFooter headerFooter
    pub(crate) headerFooter: Option<Rc<RefCell<HeaderFooter>>>,
}

// Java: public class HeaderFooterHandler extends DefaultElementHandler
impl HeaderFooterHandler {
    // Java: HeaderFooterHandler(HeaderFooter headerFooter)
    pub fn HeaderFooterHandler(headerFooter: Option<Rc<RefCell<HeaderFooter>>>) -> Self {
        // Java: this.headerFooter = headerFooter;
        // Java: if (headerFooter != null)
        if let Some(headerFooter) = &headerFooter {
            // Java: headerFooter.putDateTime(true);
            headerFooter.borrow_mut().putDateTime(true);
            // Java: headerFooter.putHeader(true);
            headerFooter.borrow_mut().putHeader(true);
            // Java: headerFooter.putFooter(true);
            headerFooter.borrow_mut().putFooter(true);
            // Java: headerFooter.putSlideNum(true);
            headerFooter.borrow_mut().putSlideNum(true);
        }
        HeaderFooterHandler { headerFooter }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: int count = attrsNotCopy.count();
        let count = attrsNotCopy.count();
        // Java: XmlRoAttr tmp = null;
        let mut tmp: Option<&dyn XmlRoAttr> = None;
        // Java: int attrName = 0;
        let mut attrName: i32 = 0;
        // Java: for(int i = 0; i < count; ++i)
        let mut i = 0;
        while i < count {
            // Java: if ((tmp = attrsNotCopy.getAttrAt(i)) != null)
            tmp = attrsNotCopy.get_attr_at(i);
            if let Some(tmp) = tmp {
                // Java: attrName = tmp.getName();
                attrName = tmp.get_name();
                // Java: switch (attrName)
                match attrName {
                    // Java: case 3211458:
                    3211458 => {
                        // Java: this.headerFooter.putDateTime(tmp.toBoolean());
                        if let Some(hf) = &self.headerFooter {
                            hf.borrow_mut().putDateTime(tmp.to_boolean());
                        }
                        // Java: break;
                        // rust: no-stmt
                    }
                    // Java: case 3211566:
                    3211566 => {
                        if let Some(hf) = &self.headerFooter {
                            hf.borrow_mut().putSlideNum(tmp.to_boolean());
                        }
                        // Java: break;
                        // rust: no-stmt
                    }
                    // Java: case 3211567:
                    3211567 => {
                        if let Some(hf) = &self.headerFooter {
                            hf.borrow_mut().putHeader(tmp.to_boolean());
                        }
                        // Java: break;
                        // rust: no-stmt
                    }
                    // Java: case 3211568:
                    3211568 => {
                        if let Some(hf) = &self.headerFooter {
                            hf.borrow_mut().putFooter(tmp.to_boolean());
                        }
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    }
}

impl ElementHandlerContract for HeaderFooterHandler {
    fn startElement(&mut self, element: i32, attrs: &dyn XmlRoAttr) {
        HeaderFooterHandler::startElement(self, element, attrs);
    }
    fn kind(&self) -> &'static str {
        "HeaderFooterHandler"
    }
}
