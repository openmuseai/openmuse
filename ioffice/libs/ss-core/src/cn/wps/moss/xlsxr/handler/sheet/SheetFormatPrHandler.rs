// Java: cn.wps.moss.xlsxr.handler.sheet.SheetFormatPrHandler.
use crate::java_runtime::{Cell, RefCell};
use crate::java_runtime::Rc;

use crate::cn::wps::moss::filefmt::biff8::record::GutsRecord::GutsRecord;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;

#[derive(Clone)]
pub struct SheetFormatPrHandler {
    sheet: Rc<RefCell<KmoSheet>>,
    // java private boolean isOutlinePrHandle;
    isOutlinePrHandle: Cell<bool>,
}
impl SheetFormatPrHandler {
    // Java: SheetFormatPrHandler(KmoSheet sheet).
    pub fn SheetFormatPrHandler(sheet: Rc<RefCell<KmoSheet>>) -> Self {
        Self {
            sheet,
            isOutlinePrHandle: Cell::new(false),
        }
    }

    // java public void init(boolean isOutlinePrHandle)
    pub fn init(&self, isOutlinePrHandle: bool) {
        // java this.isOutlinePrHandle = isOutlinePrHandle;
        self.isOutlinePrHandle.set(isOutlinePrHandle);
    }
}
impl ElementHandler for SheetFormatPrHandler {
    fn startElement(&self, _element: i32, attrs: &dyn XmlRoAttr) {
        // Java: defaultRowHeight (4806): int height = (int)(tmp.toFloat() * 20.0F).
        if let Some(attr) = attrs.get_by_name(4806) {
            let height = (attr.to_float() * 20.0_f32) as i32;
            if height != self.sheet.borrow().getDefRowHeight() {
                self.sheet.borrow().setDefRowHeight(height);
            }
        }
        // Java: if (attrsNotCopy.getByName(4808) != null) setDefaultRowDyZero(true).
        if attrs.get_by_name(4808).is_some() { self.sheet.borrow().setDefaultRowDyZero(true); }
        // Java: defaultColWidth (4805): (int)tmp.toFloat(), capped at 255.
        if let Some(attr) = attrs.get_by_name(4805) {
            self.sheet.borrow().setDefColWidth((attr.to_float() as i32).min(255));
        }
        // Java: if (attrsNotCopy.getByName(4807) != null) setDefaultRowUnsyned(true).
        if attrs.get_by_name(4807).is_some() { self.sheet.borrow().setDefaultRowUnsyned(true); }
        // java int numRowLevel = 0;
        let mut numRowLevel = 0;
        // java int numColLevel = 0;
        let mut numColLevel = 0;
        // java if ((tmp = attrsNotCopy.getByName(4811)) != null)
        if let Some(tmp) = attrs.get_by_name(4811) {
            // java numRowLevel = tmp.toInt();
            numRowLevel = tmp.to_int();
        }
        // java if ((tmp = attrsNotCopy.getByName(4812)) != null)
        if let Some(tmp) = attrs.get_by_name(4812) {
            // java numColLevel = tmp.toInt();
            numColLevel = tmp.to_int();
        }
        // java if ((numRowLevel != 0 || numColLevel != 0) && !this.isOutlinePrHandle)
        if (numRowLevel != 0 || numColLevel != 0) && !self.isOutlinePrHandle.get() {
            // java WSBoolRecord wSBoolRecord = this.sheet.getWSBool();
            let sheet = self.sheet.borrow();
            let mut wSBoolRecord = sheet.getWSBool().borrow_mut();
            // java wSBoolRecord.setRowSumsRight(true);
            wSBoolRecord.setRowSumsRight(true);
            // java wSBoolRecord.setRowSumsBelow(true);
            wSBoolRecord.setRowSumsBelow(true);
        }
        // java if (numRowLevel != 0 || numColLevel != 0)
        if numRowLevel != 0 || numColLevel != 0 {
            // java GutsRecord gutsRecord = new GutsRecord();
            let mut gutsRecord = GutsRecord::GutsRecord();
            // java if (numRowLevel != 0)
            if numRowLevel != 0 {
                // java gutsRecord.setRowLevelMax((short)(numRowLevel + 1));
                gutsRecord.setRowLevelMax((numRowLevel + 1) as i16);
            }
            // java if (numColLevel != 0)
            if numColLevel != 0 {
                // java gutsRecord.setColLevelMax((short)(numColLevel + 1));
                gutsRecord.setColLevelMax((numColLevel + 1) as i16);
            }
            // java this.sheet.initGutsRecords(gutsRecord);
            self.sheet.borrow().initGutsRecords(gutsRecord);
        }
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _sub_element: i32) -> Option<Box<dyn ElementHandler>> { None }
}
