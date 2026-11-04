use super::StylesShared;
use crate::java_runtime::{Cell, Rc};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler, XmlRoAttr::XmlRoAttr};
// java public final class NumberFormatHandler extends DefaultElementHandler
pub struct NumberFormatHandler {
    pub(super) shared: Rc<StylesShared>,
    kmoFmtid: Cell<i16>,
}
impl NumberFormatHandler {
    pub fn new(shared: Rc<StylesShared>) -> Self {
        Self {
            shared,
            kmoFmtid: Cell::new(0),
        }
    }
    // java public short kmoFmtid() { return this.kmoFmtid; }
    pub fn kmoFmtid(&self) -> i16 {
        self.kmoFmtid.get()
    }
}
impl ElementHandler for NumberFormatHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/NumberFormatHandler.java:23
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    // java if (attrsNotCopy.getByName(4319) != null && attrsNotCopy.getByName(5161) != null) {
    // java short numFmtid = attrsNotCopy.getByName(4319).toShort();
    // java String formatCode = attrsNotCopy.getByName(5161).toString();
    // java this.kmoFmtid = (short)this.kmoXFManager.checkFormat(formatCode);
    // java this.stylesTable.addnumFmt(numFmtid, this.kmoFmtid);
    // java }
    // java
    // java }
    fn startElement(&self, _: i32, a: &dyn XmlRoAttr) {
        // rust: if(attrsNotCopy.getByName(4319)!=null&&attrsNotCopy.getByName(5161)!=null){short numFmtid=...;String formatCode=...;this.kmoFmtid=(short)this.kmoXFManager.checkFormat(formatCode);this.stylesTable.addnumFmt(numFmtid,this.kmoFmtid);}
        if let (Some(id), Some(code)) = (a.get_by_name(4319), a.get_by_name(5161)) {
            let kmoFmtid = self
                .shared
                .book
                .getXFManager()
                .borrow()
                .checkFormat(Some(&code.to_string())) as i16;
            self.kmoFmtid.set(kmoFmtid);
            self.shared
                .table
                .borrow()
                .addnumFmt(id.to_short(), kmoFmtid);
        }
    }
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    fn enterSubElement(&self, _: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}
