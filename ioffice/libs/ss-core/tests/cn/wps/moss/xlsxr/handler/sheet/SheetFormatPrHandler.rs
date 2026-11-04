use ss_core::java_runtime::RefCell;
use ss_core::java_runtime::Rc;

use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttrValue::XmlRoAttrValue;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::xlsxr::handler::sheet::WorksheetHandler::WorksheetHandler;

struct AttrValue {
    name: i32,
    value: String,
}

impl XmlRoAttrValue for AttrValue {
    fn to_string(&self) -> String { self.value.clone() }
    fn to_int(&self) -> i32 { self.value.parse().unwrap_or(0) }
    fn to_byte(&self) -> i8 { self.to_int() as i8 }
    fn to_short(&self) -> i16 { self.to_int() as i16 }
    fn to_boolean(&self) -> bool { matches!(self.value.as_str(), "1" | "true") }
    fn to_float(&self) -> f32 { self.value.parse().unwrap_or(0.0) }
    fn to_double(&self) -> f64 { self.value.parse().unwrap_or(0.0) }
    fn to_long(&self) -> i64 { self.value.parse().unwrap_or(0) }
    fn to_percentage(&self) -> f64 { self.to_double() }
}

impl XmlRoAttr for AttrValue {
    fn get_by_name(&self, _: i32) -> Option<&dyn XmlRoAttr> { None }
    fn get_attr_at(&self, _: i32) -> Option<&dyn XmlRoAttr> { None }
    fn get_name_at(&self, _: i32) -> i32 { 0 }
    fn count(&self) -> i32 { 0 }
    fn get_characters(&self) -> String { String::new() }
    fn get_name(&self) -> i32 { self.name }
    fn get_by_name_with_flag(&self, _: i32, _: bool) -> Option<&dyn XmlRoAttr> { None }
    fn is_leaf(&self) -> bool { true }
}

struct NamedAttrs {
    items: Vec<AttrValue>,
}

impl NamedAttrs {
    fn new(pairs: &[(i32, &str)]) -> Self {
        Self {
            items: pairs
                .iter()
                .map(|(name, value)| AttrValue {
                    name: *name,
                    value: (*value).to_string(),
                })
                .collect(),
        }
    }
}

impl XmlRoAttrValue for NamedAttrs {
    fn to_string(&self) -> String { String::new() }
    fn to_int(&self) -> i32 { 0 }
    fn to_byte(&self) -> i8 { 0 }
    fn to_short(&self) -> i16 { 0 }
    fn to_boolean(&self) -> bool { false }
    fn to_float(&self) -> f32 { 0.0 }
    fn to_double(&self) -> f64 { 0.0 }
    fn to_long(&self) -> i64 { 0 }
    fn to_percentage(&self) -> f64 { 0.0 }
}

impl XmlRoAttr for NamedAttrs {
    fn get_by_name(&self, name: i32) -> Option<&dyn XmlRoAttr> {
        self.items.iter().find(|item| item.name == name).map(|item| item as &dyn XmlRoAttr)
    }
    fn get_attr_at(&self, index: i32) -> Option<&dyn XmlRoAttr> {
        self.items.get(index as usize).map(|item| item as &dyn XmlRoAttr)
    }
    fn get_name_at(&self, index: i32) -> i32 {
        self.items.get(index as usize).map(|item| item.name).unwrap_or(0)
    }
    fn count(&self) -> i32 { self.items.len() as i32 }
    fn get_characters(&self) -> String { String::new() }
    fn get_name(&self) -> i32 { 0 }
    fn get_by_name_with_flag(&self, name: i32, _: bool) -> Option<&dyn XmlRoAttr> { self.get_by_name(name) }
    fn is_leaf(&self) -> bool { false }
}

fn open_sheet() -> (
    Rc<KmoBook>,
    Rc<RefCell<ss_core::cn::wps::moss::app::KmoSheet::KmoSheet>>,
) {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    (book, sheet)
}

#[test]
fn fmt1_outline_pr_keeps_explicit_summary_and_stores_guts() {
    let (_book, sheet) = open_sheet();
    let handler = WorksheetHandler::WorksheetHandler(Rc::clone(&sheet), None);
    let sheet_pr = handler.enterSubElement(4768).unwrap();
    let outline = sheet_pr.enterSubElement(4831).unwrap();
    outline.startElement(4831, &NamedAttrs::new(&[(4884, "0"), (4885, "1")]));
    let format = handler.enterSubElement(4770).unwrap();
    format.startElement(4770, &NamedAttrs::new(&[(4811, "2"), (4806, "15")]));
    let sheet = sheet.borrow();
    assert_eq!(sheet.getDefRowHeight(), 300);
    assert!(!sheet.getWSBool().borrow().getRowSumsBelow());
    assert!(sheet.getWSBool().borrow().getRowSumsRight());
    assert_eq!(sheet.getGutsRec().getRowLevelMax(), 3);
    assert_eq!(sheet.getGutsRec().getColLevelMax(), 0);
}

#[test]
fn fmt2_levels_without_outline_pr_force_summary_flags() {
    let (_book, sheet) = open_sheet();
    let handler = WorksheetHandler::WorksheetHandler(Rc::clone(&sheet), None);
    let format = handler.enterSubElement(4770).unwrap();
    format.startElement(4770, &NamedAttrs::new(&[(4812, "1")]));
    let sheet = sheet.borrow();
    assert!(sheet.getWSBool().borrow().getRowSumsBelow());
    assert!(sheet.getWSBool().borrow().getRowSumsRight());
    assert_eq!(sheet.getGutsRec().getColLevelMax(), 2);
    assert_eq!(sheet.getGutsRec().getRowLevelMax(), 0);
}

#[test]
fn fmt3_unknown_sheet_pr_child_is_absent() {
    let (_book, sheet) = open_sheet();
    let handler = WorksheetHandler::WorksheetHandler(Rc::clone(&sheet), None);
    let sheet_pr = handler.enterSubElement(4768).unwrap();
    assert!(sheet_pr.enterSubElement(4830).is_none());
    assert!(sheet_pr.enterSubElement(4999).is_none());
}
