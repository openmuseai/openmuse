// Java: cn.wps.moss.xlsxr.handler.sheet.MergeCellsHandler.
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::util::KmoRect::KmoRect;

#[derive(Clone)]
pub struct MergeCellsHandler { sheet: Rc<RefCell<KmoSheet>> }
impl MergeCellsHandler {
    pub fn MergeCellsHandler(sheet: Rc<RefCell<KmoSheet>>) -> Self { Self { sheet } }
}
impl ElementHandler for MergeCellsHandler {
    fn startElement(&self, _element: i32, _attrs: &dyn XmlRoAttr) {}
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, sub_element: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: return subElement == 4894 ? this.mergeHandler : null;
        (sub_element == 4894).then(|| Box::new(MergeCellHandler { sheet: Rc::clone(&self.sheet) }) as Box<dyn ElementHandler>)
    }
}
struct MergeCellHandler { sheet: Rc<RefCell<KmoSheet>> }
impl ElementHandler for MergeCellHandler {
    fn startElement(&self, _element: i32, attrs: &dyn XmlRoAttr) {
        // Java: CellRangeAddress.valueOf(tmp.toString(), sheet.getVersion()); ExcelConst.adaptRegion(...).
        if let Some(reference) = attrs.get_by_name(4100) {
            if let Some(rect) = parse_range(&reference.to_string()) {
                // Java: sheet.merge(new KmoRect(firstRow, firstColumn, lastRow, lastColumn)).
                self.sheet.borrow().merge(&rect);
            }
        }
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _sub_element: i32) -> Option<Box<dyn ElementHandler>> { None }
}

fn parse_range(reference: &str) -> Option<KmoRect> {
    let (first, last) = reference.split_once(':')?;
    fn point(text: &str) -> Option<(i32, i32)> {
        let mut col = 0i32;
        let mut row = 0i32;
        let mut in_row = false;
        for ch in text.chars() {
            if ch == '$' { continue; }
            if ch.is_ascii_alphabetic() && !in_row {
                col = col.checked_mul(26)?.checked_add(i32::from(ch.to_ascii_uppercase() as u8 - b'A' + 1))?;
            } else if ch.is_ascii_digit() {
                in_row = true;
                row = row.checked_mul(10)?.checked_add(i32::from(ch as u8 - b'0'))?;
            } else { return None; }
        }
        (row > 0 && col > 0).then_some((row - 1, col - 1))
    }
    let (r1, c1) = point(first)?;
    let (r2, c2) = point(last)?;
    (r1 <= r2 && c1 <= c2).then(|| KmoRect::KmoRect__bounds(r1, c1, r2, c2))
}
