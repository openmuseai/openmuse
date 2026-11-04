use ss_core::java_runtime::Rc;

use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::app::KmoSheet::CellTypeInfo;
use ss_core::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;

fn book() -> Rc<KmoBook> {
    KmoBook::KmoBook__parent(false)
}

#[test]
fn bs1_sheet_order_active_and_ids() {
    let book = book();
    assert_eq!(book.sheetCount(), 0);
    let first = book.appendSheet__2(Some("A\nB".to_string()));
    let second = book.appendSheet();
    let third = book.appendSheet__5(7, Some("Ｓｈｅｅｔ３".to_string()), 0);
    assert_eq!(book.sheetCount(), 3);
    assert_eq!(first.borrow().name(), "AB");
    assert_eq!(second.borrow().name(), "Sheet1");
    assert_eq!(third.borrow().name(), "Ｓｈｅｅｔ３");
    assert_eq!(first.borrow().getSheetId(), 0);
    assert_eq!(second.borrow().getSheetId(), 1);
    assert_eq!(third.borrow().getSheetId(), 7);
    assert_eq!(book.sheetIndex__2("ab"), 0);
    assert_eq!(book.sheetIndex__2("sheet1"), 1);
    assert_eq!(book.sheetIndex__2("Sheet3"), 2);
    assert!(book.isDirty());
    assert_eq!(book.whichSheetIsActivated(), 0);
    assert!(book.getActiveSheet().borrow().same_sheet(&first.borrow()));
    book.activateSheet(2);
    assert_eq!(book.whichSheetIsActivated(), 2);
    assert!(third.borrow().isSelected());
    assert!(!first.borrow().isSelected());
    assert!(book.getActiveSheet().borrow().same_sheet(&third.borrow()));
    let again = book.appendSheet();
    assert_eq!(again.borrow().name(), "Sheet2");
    assert_eq!(again.borrow().getSheetId(), 8);
    let empty = book.sheetAt(99);
    let empty_again = book.sheetAt(-1);
    assert!(empty.borrow().same_sheet(&empty_again.borrow()));
    assert_eq!(empty.borrow().name(), "");
    assert_eq!(empty.borrow().getSheetId(), 9);
    assert_eq!(book.sheetCount(), 4);
}

#[test]
fn bs2_empty_cell_readback_and_value_type() {
    let book = book();
    book.setIsIOState(true);
    let sheet = book.appendSheet__2(Some("Grid".to_string()));
    assert!(!book.isDirty());
    let sheet = sheet.borrow();
    let blank = sheet.getCellData(0, 32);
    assert!(blank.isNull());
    assert_eq!(sheet.getCellValueType__2(0, 32), 0);
    let mut written = KmoCellData::default();
    let id = book.getCellValueStore().borrow_mut().newDoubleValue(1.5);
    written.ixf = 15;
    written.id = id;
    written.formulaType = 1;
    sheet.getGrid().cellDele().write(5, 3, &written);
    let read = sheet.getCellData(5, 3);
    assert_eq!(read.ixf, 15);
    assert_eq!(read.id, id);
    assert_eq!(read.formulaType, 1);
    assert_eq!(sheet.getCellValueType__2(5, 3), 1);
    let same_block = sheet.getCellData(5, 4);
    assert!(same_block.isNull());
    assert_eq!(sheet.getCellValueType__2(5, 4), 0);
    let used = sheet.getUsedRange();
    assert_eq!(used.tl.row, 5);
    assert_eq!(used.tl.col, 3);
    assert_eq!(used.br.row, 6);
    assert_eq!(used.br.col, 4);
    let mut info = CellTypeInfo::CellTypeInfo();
    assert_eq!(sheet.getCellValueType(5, 3, Some(&mut info)), 1);
    assert_eq!(info.cellType, 1);
    assert!(!info.hasRun);
    let string_id = book.getCellValueStore().borrow_mut().newStringValue(Some("hi"));
    let mut text = KmoCellData::default();
    text.id = string_id;
    sheet.getGrid().cellDele().write(1, 1, &text);
    let mut text_info = CellTypeInfo::CellTypeInfo();
    assert_eq!(sheet.getCellValueType(1, 1, Some(&mut text_info)), 2);
    assert!(text_info.hasRun == book.getCellValueStore().borrow().isRunsString2(string_id));
}

#[test]
fn bs3_row_height_and_version() {
    let book = book();
    let sheet = book.appendSheet();
    let sheet = sheet.borrow();
    assert_eq!(sheet.getMaxRows(), 65536);
    assert_eq!(sheet.getMaxColumns(), 256);
    sheet.initFillRowInfo(5, 325, 0, 256);
    assert_eq!(sheet.getRowHeight(5), 325);
    sheet.initFillRowInfo(6, 400, 15, 256);
    assert_eq!(sheet.getRowHeight(6), 400);
    sheet.initFillRowInfo(7, 1, 0, 256);
    assert_eq!(sheet.getRowHeight(7), 325);
    assert_eq!(sheet.getSheetIndex(), 0);
}

#[test]
fn bs4_default_sheet_and_should_dirty() {
    let created = KmoBook::KmoBook__parent(true);
    assert_eq!(created.sheetCount(), 1);
    assert_eq!(created.sheetAt(0).borrow().name(), "Sheet1");
    assert!(created.isDirty());
    let book = book();
    assert!(!book.isDirty());
    book.setShouldDirty(false);
    book.appendSheet__2(Some("Later".to_string()));
    assert!(!book.isDirty());
    assert_eq!(book.sheetAt(0).borrow().name(), "Later");
}
