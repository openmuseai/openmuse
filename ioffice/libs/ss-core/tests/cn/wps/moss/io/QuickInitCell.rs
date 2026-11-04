use ss_core::cn::wps::moss::app::GridSheetData::GridSheetData;
use ss_core::cn::wps::moss::app::IGridSheet::IGridSheet;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore;
use ss_core::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;
use ss_core::cn::wps::moss::io::QuickInitCell::QuickInitCell;

#[test]
fn wr1_five_cell_kinds_roundtrip() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let sheet_ref = sheet.borrow();
    let mut cell = QuickInitCell::QuickInitCell__sheet(&sheet_ref);

    cell.setBlankCellData(0, 0, 15);
    let blank = sheet_ref.getCellData(0, 0);
    assert_eq!(blank.ixf, 15);
    assert_eq!(blank.id, 0);
    assert_eq!(sheet_ref.getCellValueType__2(0, 0), 0);

    cell.setRKCellData(2, 2, 15, 1.5);
    let number = sheet_ref.getCellData(2, 2);
    assert_eq!(number.ixf, 15);
    assert_eq!(sheet_ref.getCellValueType__2(2, 2), 1);
    assert_eq!(
        book.getCellValueStore().borrow_mut().doubleValue(number.id),
        1.5
    );

    cell.setBoolCellData(3, 1, 15, true);
    assert_eq!(sheet_ref.getCellValueType__2(3, 1), 5);
    cell.setBoolCellData(3, 3, 15, false);
    assert_eq!(sheet_ref.getCellValueType__2(3, 3), 5);

    cell.setErrorCellData(3, 2, 15, 1);
    assert!(sheet_ref.getCellData(3, 2).isNull());
    cell.setErrorCellData(3, 2, 15, 7);
    assert_eq!(sheet_ref.getCellValueType__2(3, 2), 6);

    let raw = cell.setLabelCellData(4, 0, 15, Some("hi"), false);
    assert_eq!(cell.getStringBySSTId(raw), "hi");
    assert_eq!(sheet_ref.getCellValueType__2(4, 0), 2);
    assert!(sheet_ref.getCellData(4, 1).isNull());
    cell.setLabelSSTCellData(4, 1, 15, 99999);
    assert!(sheet_ref.getCellData(4, 1).isNull());
    cell.setLabelSSTCellData(4, 1, 15, raw);
    assert_eq!(sheet_ref.getCellValueType__2(4, 1), 2);
    assert_eq!(
        cell.getStringBySSTId(KmoCellValueStore::id2sstid(sheet_ref.getCellData(4, 1).id)),
        "hi"
    );

    let cache = book.getCellValueStore().borrow_mut().newDoubleValue(2.5);
    let mut formula = KmoFormulaData::KmoFormulaData();
    formula.setCacheValueId(cache);
    cell.setFormulaCellData(5, 0, 15, &formula, 1);
    let stored = sheet_ref.getCellData(5, 0);
    assert_eq!(stored.formulaType, 1);
    assert_eq!(sheet_ref.getCellValueType__2(5, 0), 1);

    let fml_sst = cell.setLabelCellData(5, 1, 15, Some("=A1"), true);
    assert_eq!(
        KmoCellValueStore::identifyTypeByID(sheet_ref.getCellData(5, 1).id),
        2
    );
    assert_eq!(sheet_ref.getCellValueType__2(5, 1), 2);
    assert_eq!(cell.getStringBySSTId(raw), "hi");
    assert_eq!(raw, 0);
    assert_eq!(fml_sst, 0);
    assert_ne!(sheet_ref.getCellData(5, 1).id, sheet_ref.getCellData(4, 0).id);
    drop(sheet_ref);

    let grid = GridSheetData::GridSheetData(sheet);
    assert_eq!(grid.maxRows(), 6);
    assert_eq!(grid.maxColumns(), 4);
    assert_eq!(grid.rowHeight(0), 325);
    assert_eq!(grid.getDefRowHeight(), 325);
    assert!(!grid.isDefaultRowDyZero());
    assert_eq!(grid.maxRowsLimit(), 65536);
    assert_eq!(grid.maxColumnsLimit(), 256);
    assert_eq!(grid.getCellType(2, 2, None), 1);
    let core = grid.coreSheet();
    assert_eq!(core.borrow().getSheetUid(), grid.coreSheet().borrow().getSheetUid());
}
