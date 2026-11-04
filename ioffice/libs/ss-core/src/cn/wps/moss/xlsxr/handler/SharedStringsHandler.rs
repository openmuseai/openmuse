use crate::java_runtime::{Cell, RefCell};
use crate::java_runtime::Rc;

use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use crate::cn::wps::moss::app::KmoBook::KmoBook;
use crate::cn::wps::moss::engine::grid::MemoryMixSQLStringList::OoxmlStr;
use crate::cn::wps::moss::xlsxr::handler::sheet::RichTextHandler::RichTextHandler;


// java package cn.wps.moss.xlsxr.handler;
// java import cn.wps.kfc.xpxml.reader.ElementHandler;
// java import cn.wps.kfc.xpxml.reader.XmlRoAttr;
// java import cn.wps.moss.app.KmoBook;
// java import cn.wps.moss.filefmt.biff8.record.common.UnicodeString;
// java import cn.wps.moss.xlsxr.handler.sheet.RichTextHandler;
// java import java.util.ArrayList;

// java public final class SharedStringsHandler implements ElementHandler {
pub struct SharedStringsHandler {
    // java private RichTextHandler richTextHandler;
    richTextHandler: Rc<RichTextHandler>,
    uniqueCount: Cell<i32>,
    bFinish: Cell<bool>,
}

impl SharedStringsHandler {
    // java public SharedStringsHandler(ArrayList<UnicodeString> strs, KmoBook book) {
    pub fn SharedStringsHandler(strs: Option<Rc<RefCell<Vec<OoxmlStr>>>>, book: Rc<KmoBook>) -> Self {
        // java this.richTextHandler = new RichTextHandler(strs, book);
        let richTextHandler = RichTextHandler::RichTextHandler(strs,book);
        Self { richTextHandler, uniqueCount: Cell::new(0), bFinish: Cell::new(false) }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // java XmlRoAttr tmp;
        // java if ((tmp = attrsNotCopy.getByName(4650)) != null) {
        // java this.uniqueCount = tmp.toInt();
        let _ = element;
        if let Some(tmp) = attrsNotCopy.get_by_name(4650) {
            self.uniqueCount.set(tmp.to_int());
        }
    }

    // java public void endElement(int element) {
    pub fn endElement(&self, element: i32) {
        // java this.bFinish = true;
        let _ = element;
        self.bFinish.set(true);
    }

    // java public void characters(String chars) {
    pub fn characters(&self, chars: &str) {
        let _ = chars;
    }

    // java public ElementHandler enterSubElement(int subElement) {
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // java return subElement == 4649 ? this.richTextHandler : null;
        if subElement == 4649 {
            Some(crate::cn::wps::moss::xlsxr::util::ElementHandlerRef::share(self.richTextHandler.clone()))
        } else {
            None
        }
    }

    // java public void reset(ArrayList<UnicodeString> strs) {
    pub fn reset(&self, strs: Option<Rc<RefCell<Vec<OoxmlStr>>>>) {
        // java this.richTextHandler.reset(strs);
        self.richTextHandler.reset(strs);
    }

    // java public int getUniqueCount() {
    pub fn getUniqueCount(&self) -> i32 {
        // java return this.uniqueCount;
        self.uniqueCount.get()
    }

    // java public boolean hasFinish() {
    pub fn hasFinish(&self) -> bool {
        // java return this.bFinish;
        self.bFinish.get()
    }

}


impl ElementHandler for SharedStringsHandler {
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        SharedStringsHandler::startElement(self, element, attrsNotCopy);
    }
    fn endElement(&self, element: i32) {
        SharedStringsHandler::endElement(self, element);
    }
    fn characters(&self, chars: &str) {
        SharedStringsHandler::characters(self, chars);
    }
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        SharedStringsHandler::enterSubElement(self, subElement)
    }
}
