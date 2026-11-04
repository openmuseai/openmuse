#[derive(Default)]
struct OpenObserver;
impl ss_core::cn::wps::moss::app::event::KmoBookOpenEventListener::KmoBookOpenEventListener for OpenObserver{}
// The original XlsxrSharedStrings/XlsxrStyles caches are static across books.
// Isolate independent fixture tests, while preserving each book's original three reader workers.
static ORIGINAL_READER_GLOBALS:std::sync::Mutex<()>=std::sync::Mutex::new(());
use ss_core::java_runtime::Rc;

use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage::OxfrPackage;
use kfc_kfc_alpha::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
use ss_core::cn::wps::moss::app::KmoSheet::KmoSheet;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore;
use ss_core::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;
use ss_core::cn::wps::moss::xlsxr::xl::XlsxrSharedStrings::XlsxrSharedStrings;
use ss_core::cn::wps::moss::xlsxr::xl::XlsxrWorkbook::XlsxrWorkbook;

fn fixture(name: &str) -> String {
    format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn open_book(path: &str) -> Rc<KmoBook> {
    let book = KmoBook::KmoBook();
    book.init(false);
    let mut pack = OxfrPackage::OxfrPackage(path).unwrap();
    let part = XlsxrPackage::getBookPart(&mut pack);
    let zip = part.getZipFile();
    let entry = part.getZipEntry().unwrap();
    let owned = OpcrPackagePart::OpcrPackagePart(&zip, &entry);
    let reader = XlsxrWorkbook::XlsxrWorkbook(Rc::clone(&book),None,Some(Rc::new(OpenObserver)),None);
    reader.setBookPart(owned);
    XlsxrWorkbook::open(&reader).unwrap();
    let _ = pack;
    book
}

fn cache_id(sheet: &KmoSheet, id: i32) -> i32 {
    if KmoCellValueStore::isFormulaId(id) {
        sheet.getCellValueStore().borrow().formulaCachaId(id)
    } else {
        id
    }
}

fn number(sheet: &KmoSheet, row: i32, col: i32) -> f64 {
    let data = sheet.getCellData(row, col);
    assert_eq!(data.ixf, 15);
    assert_eq!(sheet.getCellValueType__2(row, col), 1);
    let id = cache_id(sheet, data.id);
    sheet.getCellValueStore().borrow_mut().doubleValue(id)
}

fn text(sheet: &KmoSheet, row: i32, col: i32) -> String {
    let data = sheet.getCellData(row, col);
    assert_eq!(data.ixf, 15);
    assert_eq!(sheet.getCellValueType__2(row, col), 2);
    sheet.getCellValueStore().borrow_mut().stringValue(data.id)
}

#[test]
fn cell1_basic_types_keep_raw_value_and_column() {
    let _fixture=ORIGINAL_READER_GLOBALS.lock().unwrap();
    let book = open_book(&fixture("basic-types.xlsx"));
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    assert_eq!(number(&sheet, 0, 0), 1.5);
    let b1 = sheet.getCellData(0, 1);
    assert_eq!(b1.ixf, 15);
    assert_eq!(sheet.getCellValueType__2(0, 1), 5);
    assert!(sheet.getCellValueStore().borrow().booleanValue(b1.id));
    let c1 = sheet.getCellData(0, 2);
    assert_eq!(c1.ixf, 15);
    assert_eq!(sheet.getCellValueType__2(0, 2), 6);
    assert_eq!(sheet.getCellValueStore().borrow().errorValue(c1.id), 7);
    assert!(sheet.getCellData(0, 3).isNull());
    assert_eq!(text(&sheet, 0, 4), "Hello");
    assert_eq!(text(&sheet, 0, 5), "世界");
    assert_eq!(text(&sheet, 0, 6), "Latin");
    assert_eq!(text(&sheet, 1, 0), "中文");
    assert!(sheet.getCellData(1, 1).isNull());
    assert_eq!(XlsxrSharedStrings::getUniqueCount(), 3);
}

#[test]
fn cell2_sparse_values_do_not_leak_across_sheets() {
    let _fixture=ORIGINAL_READER_GLOBALS.lock().unwrap();
    let book = open_book(&fixture("sparse-multisheet.xlsx"));
    let visible = book.sheetAt(0);
    let visible = visible.borrow();
    assert_eq!(number(&visible, 0, 0), 1.0);
    assert_eq!(number(&visible, 4, 2), 9.0);
    assert!(visible.getCellData(0, 2).isNull());
    drop(visible);
    let hidden = book.sheetAt(1);
    let hidden = hidden.borrow();
    assert_eq!(number(&hidden, 0, 0), 2.0);
    assert!(hidden.getCellData(4, 2).isNull());
    drop(hidden);
    let other = book.sheetAt(2);
    let other = other.borrow();
    assert!(other.getCellData(0, 0).isNull());
    assert_eq!(number(&other, 1, 1), 3.0);
}

#[test]
fn cell3_formula_keeps_cache_and_does_not_invent_zero() {
    let _fixture=ORIGINAL_READER_GLOBALS.lock().unwrap();
    let book = open_book(&fixture("formula-cache.xlsx"));
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    let a1 = sheet.getCellData(0, 0);
    assert_eq!(a1.formulaType, 1);
    assert_eq!(number(&sheet, 0, 0), 2.0);
    let b1 = sheet.getCellData(0, 1);
    assert_eq!(b1.formulaType, 1);
    assert_eq!(sheet.getCellValueType__2(0, 1), 2);
    let b1_cache_id = cache_id(&sheet, b1.id);
    assert_eq!(
        sheet
            .getCellValueStore()
            .borrow_mut()
            .stringValue(b1_cache_id),
        "ab"
    );
    assert!(KmoCellValueStore::isFormulaId(b1.id));
    assert!(KmoCellValueStore::isStringId(b1_cache_id));
    let c1 = sheet.getCellData(0, 2);
    assert_eq!(c1.formulaType, 1);
    assert!(!c1.isBlank());
    assert_eq!(sheet.getCellValueId(0, 2), 0);
    assert_eq!(sheet.getCellValueType__2(0, 2), 0);
}

#[test]
fn cell4_empty_sheet_data_writes_no_cell() {
    let _fixture=ORIGINAL_READER_GLOBALS.lock().unwrap();
    let book = open_book(&fixture("empty.xlsx"));
    let sheet = book.sheetAt(0);
    assert!(sheet.borrow().getCellData(0, 0).isNull());
}
