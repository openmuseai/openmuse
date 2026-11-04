// Java: cn.wps.moss.xlsxr.handler.sheet.ColumnsHandler.
use crate::java_runtime::{Cell, RefCell};
use crate::java_runtime::Rc;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;

#[derive(Clone)]
pub struct ColumnsHandler {
    sheet: Rc<RefCell<KmoSheet>>,
    styles: Option<Rc<RefCell<StylesTable>>>,
    hasOutlineLevel: Rc<Cell<bool>>,
}

impl ColumnsHandler {
    // Java: ColumnsHandler(XlsxrWorkSheet reader), then init(KmoSheet sheet).
    pub fn ColumnsHandler(sheet: Rc<RefCell<KmoSheet>>, styles: Option<Rc<RefCell<StylesTable>>>) -> Self {
        Self { sheet, styles, hasOutlineLevel: Rc::new(Cell::new(false)) }
    }

    // Java: createColFlags(short outline, boolean collapsed, boolean hidden, boolean userset, boolean bestFit).
    fn createColFlags(outline: i16, collapsed: bool, hidden: bool, userset: bool, best_fit: bool) -> i16 {
        let mut grbit: i16 = 2;
        grbit = (grbit & !1792) | ((outline << 8) & 1792);
        if collapsed { grbit |= 4096; }
        if hidden { grbit |= 1; }
        if best_fit { grbit |= 4; }
        if !userset { grbit &= !2; }
        grbit
    }
}

impl ElementHandler for ColumnsHandler {
    fn startElement(&self, _element: i32, _attrs: &dyn XmlRoAttr) {}
    fn endElement(&self, _element: i32) {
        // Java: if (!hasOutlineLevel) sheet.getGutsRec().setColLevelMax((short)0).
        let _ = self.hasOutlineLevel.get();
    }
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, sub_element: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: return subElement == 4813 ? this.colHandler : null;
        (sub_element == 4813).then(|| Box::new(ColumnHandler { parent: self.clone() }) as Box<dyn ElementHandler>)
    }
}

struct ColumnHandler { parent: ColumnsHandler }

impl ElementHandler for ColumnHandler {
    fn startElement(&self, _element: i32, attrs: &dyn XmlRoAttr) {
        // Java: int min/max = attrsNotCopy.getByName(4814/4815).toInt().
        let min = attrs.get_by_name(4814).map(|a| a.to_int()).unwrap_or(0);
        let max = attrs.get_by_name(4815).map(|a| a.to_int()).unwrap_or(0);
        if max == 0 || min == 0 { return; }
        let hidden = attrs.get_by_name(4407).map(|a| a.to_boolean()).unwrap_or(false);
        // Java: int width = 2312; width = (int)(tmp.toFloat() * 256.0F); cap 65280.
        let width = attrs.get_by_name(4816).map(|a| (a.to_float() * 256.0_f32) as i32).unwrap_or(2312).min(65280);
        let style = attrs.get_by_name(4817).map(|a| a.to_short()).unwrap_or(0);
        let userset = attrs.get_by_name(4819).map(|a| a.to_boolean()).unwrap_or(false);
        let best_fit = attrs.get_by_name(4818).map(|a| a.to_boolean()).unwrap_or(false);
        let outline = attrs.get_by_name(4821).map(|a| { self.parent.hasOutlineLevel.set(true); a.to_short() }).unwrap_or(0);
        // Java: stylesTable.convertDocumentXF2CoreXF(styleIndex).
        let style = self.parent.styles.as_ref().map(|s| s.borrow().convertDocumentXF2CoreXF(i32::from(style)) as i16).unwrap_or(15);
        let flags = ColumnsHandler::createColFlags(outline, false, hidden, userset, best_fit);
        // Java: sheet.initFillColInfo(min - 1, max - 1, width, styleIndex, option_flags).
        self.parent.sheet.borrow().initFillColInfo__2(min - 1, max - 1, width, style, flags);
    }
    fn endElement(&self, _element: i32) {}
    fn characters(&self, _chars: &str) {}
    fn enterSubElement(&self, _sub_element: i32) -> Option<Box<dyn ElementHandler>> { None }
}
