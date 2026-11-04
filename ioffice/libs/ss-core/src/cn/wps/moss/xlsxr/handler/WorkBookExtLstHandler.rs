#![allow(non_snake_case)]
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use crate::cn::wps::moss::core::theme::WorkbookPr::WorkbookPr;

// Java: public class WorkBookExtLstHandler extends DefaultElementHandler
pub struct WorkBookExtLstHandler { mWorkbookPr: Rc<RefCell<WorkbookPr>> }

impl WorkBookExtLstHandler {
    // Java: public WorkBookExtLstHandler(WorkbookPr workbookPr)
    pub fn WorkBookExtLstHandler(workbookPr: Rc<RefCell<WorkbookPr>>) -> Self {
        Self { mWorkbookPr: workbookPr }
    }
}

impl ElementHandler for WorkBookExtLstHandler {
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: return subElement == 4148 ? new WorkBookExtHandler() : null;
        (subElement == 4148).then(|| Box::new(WorkBookExtHandler { mWorkbookPr: Rc::clone(&self.mWorkbookPr) }) as Box<dyn ElementHandler>)
    }
}

// Java: class WorkBookExtHandler extends DefaultElementHandler
struct WorkBookExtHandler { mWorkbookPr: Rc<RefCell<WorkbookPr>> }
impl ElementHandler for WorkBookExtHandler {
    fn startElement(&self, _: i32, _: &dyn XmlRoAttr) {}
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: case 6422529: return new WorkbookPrHandler();
        (subElement == 6422529).then(|| Box::new(WorkbookPrHandler { mWorkbookPr: Rc::clone(&self.mWorkbookPr) }) as Box<dyn ElementHandler>)
    }
}

// Java: class WorkbookPrHandler extends DefaultElementHandler
struct WorkbookPrHandler { mWorkbookPr: Rc<RefCell<WorkbookPr>> }
impl ElementHandler for WorkbookPrHandler {
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr tmp = attrsNotCopy.getByName(6422530);
        if let Some(tmp) = attrsNotCopy.get_by_name(6422530) {
            // Java: this.mWorkbookPr.setChartTrackingRefBase(tmp.toBoolean());
            self.mWorkbookPr.borrow_mut().setChartTrackingRefBase(tmp.to_boolean());
        }
    }
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    fn enterSubElement(&self, _: i32) -> Option<Box<dyn ElementHandler>> { None }
}
