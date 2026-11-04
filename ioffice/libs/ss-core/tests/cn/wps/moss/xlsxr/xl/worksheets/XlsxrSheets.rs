#[derive(Default)]
struct OpenObserver;
impl ss_core::cn::wps::moss::app::event::KmoBookOpenEventListener::KmoBookOpenEventListener for OpenObserver{}
use ss_core::java_runtime::Rc;

use kfc_kfc_alpha::kfc::ooxfr::OxfrPackage::OxfrPackage;
use kfc_kfc_alpha::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::xlsxr::util::XlsxrPackage::XlsxrPackage;
use ss_core::cn::wps::moss::xlsxr::xl::XlsxrWorkbook::XlsxrWorkbook;

fn fixture(name: &str) -> String {
    format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn open_book(path: &str) -> (Rc<KmoBook>, Rc<XlsxrWorkbook>, Result<(), std::io::Error>) {
    let book = KmoBook::KmoBook();
    book.init(false);
    let mut pack = OxfrPackage::OxfrPackage(path).unwrap();
    let part = XlsxrPackage::getBookPart(&mut pack);
    let zip = part.getZipFile();
    let entry = part.getZipEntry().unwrap();
    let owned = OpcrPackagePart::OpcrPackagePart(&zip, &entry);
    let reader = XlsxrWorkbook::XlsxrWorkbook(Rc::clone(&book),None,Some(Rc::new(OpenObserver)),None);
    reader.setBookPart(owned);
    let result = XlsxrWorkbook::open(&reader);
    let _ = pack;
    (book, reader, result)
}

fn style_xf_size(reader: &XlsxrWorkbook) -> i32 {
    reader
        .stylesTable()
        .expect("StylesTable")
        .borrow()
        .getStyleXfSize()
}

#[test]
fn route1_basic_sheet_data_keeps_default_row_and_empty_cells() {
    let (book, reader, result) = open_book(&fixture("basic-types.xlsx"));
    result.unwrap();
    let sheet = book.sheetAt(0);
    let sheet = sheet.borrow();
    assert_eq!(sheet.name(), "Sheet1");
    assert_eq!(style_xf_size(&reader), 1);
    assert_eq!(sheet.getRowHeight(0), 325);
    assert_eq!(sheet.getRowHeight(1), 325);
    assert_eq!(sheet.getCellValueType__2(0, 0), 1);
    let a1 = sheet.getCellData(0, 0);
    assert_eq!(sheet.getCellValueStore().borrow_mut().doubleValue(a1.id), 1.5);
    assert_eq!(sheet.getCellValueType__2(1, 0), 2);
    assert_eq!(
        sheet
            .getCellValueStore()
            .borrow_mut()
            .stringValue(sheet.getCellData(1, 0).id),
        "中文"
    );
}

#[test]
fn route2_row_height_does_not_leak_across_sheets() {
    let (book, _reader, result) = open_book(&fixture("sparse-multisheet.xlsx"));
    result.unwrap();
    assert_eq!(book.sheetCount(), 3);
    let visible = book.sheetAt(0);
    let visible = visible.borrow();
    assert_eq!(visible.name(), "Visible");
    assert_eq!(visible.getRowHeight(0), 600);
    assert_eq!(visible.getRowHeight(4), 325);
    assert_eq!(
        visible
            .getCellValueStore()
            .borrow_mut()
            .doubleValue(visible.getCellData(0, 0).id),
        1.0
    );
    assert_eq!(
        visible
            .getCellValueStore()
            .borrow_mut()
            .doubleValue(visible.getCellData(4, 2).id),
        9.0
    );
    drop(visible);
    let hidden = book.sheetAt(1);
    let hidden = hidden.borrow();
    assert!(hidden.isSheetHidden());
    assert_eq!(hidden.getRowHeight(0), 325);
    assert_eq!(
        hidden
            .getCellValueStore()
            .borrow_mut()
            .doubleValue(hidden.getCellData(0, 0).id),
        2.0
    );
    drop(hidden);
    let other = book.sheetAt(2);
    let other = other.borrow();
    assert_eq!(other.name(), "Other");
    assert_eq!(other.getRowHeight(1), 325);
    assert!(other.getCellData(0, 0).isNull());
    assert_eq!(
        other
            .getCellValueStore()
            .borrow_mut()
            .doubleValue(other.getCellData(1, 1).id),
        3.0
    );
}

#[test]
fn route3_missing_sheet_part_does_not_fail_the_book() {
    let (book, reader, result) = open_book(&fixture("broken-rel.xlsx"));
    result.unwrap();
    assert_eq!(book.sheetCount(), 1);
    assert_eq!(book.sheetAt(0).borrow().name(), "Sheet1");
    assert_eq!(style_xf_size(&reader), 1);
    assert_eq!(book.sheetAt(0).borrow().getRowHeight(0), 325);
    assert!(book.sheetAt(0).borrow().getCellData(0, 0).isNull());
}
