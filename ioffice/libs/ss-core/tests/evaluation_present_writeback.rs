use ss_core::java_runtime::RefCell;
use ss_core::java_runtime::Rc;

use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::areaevent::AreaWatcher::{IAreaChangeListener, INameChangeListener};
use ss_core::cn::wps::moss::engine::eval::EvaluationCellFormula::EvaluationCellFormula;
use ss_core::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;
use ss_core::cn::wps::moss::engine::grid::KmoCellValueStore::KmoCellValueStore;
use ss_core::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;
use ss_core::cn::wps::moss::app::KmoSheet::KmoSheet;
use ss_core::cn::wps::moss::util::KmoRect::KmoRect;

#[derive(Default)]
struct Changed(usize);

impl INameChangeListener for Changed {
    fn onSheetNameChanged(&mut self, _sheet: &KmoSheet) {}
}

impl IAreaChangeListener for Changed {
    fn onContentChanged(&mut self) { self.0 += 1; }
    fn onAreaMoved(&mut self, _sheet: &KmoSheet, _rect: &KmoRect) {}
    fn onAreaDeleted(&mut self) {}
}

#[test]
fn formula_result_uses_formula_store_and_notifies_only_on_change() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let store = book.getCellValueStore();
    let mut formula = KmoFormulaData::KmoFormulaData();
    let tokens = [30, 1, 0];
    let tokens_id = store.borrow_mut().newFormulaTokens(Some(&tokens));
    formula.setTokens(tokens_id, tokens.len() as i32);
    let old_id = store.borrow_mut().newFormulaValue(&formula);
    let mut cell = KmoCellData::default();
    cell.ixf = 15;
    cell.formulaType = 1;
    cell.id = old_id;
    sheet.borrow().setCellData(0, 0, &cell);
    let changed = Rc::new(RefCell::new(Changed::default()));
    let listener: Rc<RefCell<dyn IAreaChangeListener>> = changed.clone();
    book.areaWatcher().borrow_mut().registerListener(&sheet.borrow(), &KmoRect::KmoRect__bounds(0, 0, 0, 0), listener);
    book.setDirty(false);

    let fml = EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 0, 0);
    fml.setErrorResult(7);
    let new_cell = sheet.borrow().getCellData(0, 0);
    assert_eq!(new_cell.formulaType, 1);
    assert_eq!(KmoCellValueStore::identifyTypeByID(new_cell.id), 7);
    let mut updated = KmoFormulaData::KmoFormulaData();
    store.borrow().formulaValue(new_cell.id, &mut updated);
    assert_eq!(updated.getTokensId(), tokens_id);
    assert_eq!(updated.getTokensPlainSize(), tokens.len() as i32);
    assert_eq!(KmoCellValueStore::identifyTypeByID(updated.getCacheValueId()), 6);
    assert_eq!(store.borrow().errorValue(updated.getCacheValueId()), 7);
    assert!(book.isDirty());
    assert_eq!(changed.borrow().0, 1);

    book.setDirty(false);
    fml.setErrorResult(7);
    assert_eq!(sheet.borrow().getCellData(0, 0).id, new_cell.id);
    assert!(!book.isDirty());
    assert_eq!(changed.borrow().0, 1);
}

#[test]
fn error_29_preserves_nonblank_cache_and_zero_only_fills_blank() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let store = book.getCellValueStore();
    let mut formula = KmoFormulaData::KmoFormulaData();
    formula.setTokens(0, 0);
    formula.setCacheValueId(store.borrow_mut().newDoubleValue(5.0));
    let mut cell = KmoCellData::default();
    cell.formulaType = 1;
    cell.id = store.borrow_mut().newFormulaValue(&formula);
    sheet.borrow().setCellData(1, 1, &cell);
    let fml = EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 1, 1);
    book.setDirty(false);
    fml.setErrorResult(29);
    fml.setZeroResultIfBlank();
    assert_eq!(sheet.borrow().getCellData(1, 1).id, cell.id);
    assert!(!book.isDirty());
}

#[test]
fn area_watcher_merges_content_events_and_uses_sheet_identity() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let other = book.appendSheet();
    let changed = Rc::new(RefCell::new(Changed::default()));
    let listener: Rc<RefCell<dyn IAreaChangeListener>> = changed.clone();
    let rect = KmoRect::KmoRect__bounds(2, 3, 4, 5);
    let mut watcher = book.areaWatcher().borrow_mut();
    watcher.registerListener(&sheet.borrow(), &rect, listener.clone());
    watcher.startMergeEvents();
    watcher.postContentChangedEvent(&sheet.borrow(), 2, 3);
    watcher.postContentChangedEvent(&sheet.borrow(), 4, 5);
    watcher.postContentChangedEvent(&other.borrow(), 2, 3);
    assert_eq!(changed.borrow().0, 0);
    watcher.finishMergeEvents();
    assert_eq!(changed.borrow().0, 1);
    assert!(watcher.unregisterListener(&sheet.borrow(), &rect, &listener));
    watcher.postContentChangedEvent(&sheet.borrow(), 2, 3);
    assert_eq!(changed.borrow().0, 1);
}

#[test]
fn blank_formula_zero_number_bool_and_string_follow_cache_id_types() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let store = book.getCellValueStore();
    let formula = KmoFormulaData::KmoFormulaData();
    let mut cell = KmoCellData::default();
    cell.formulaType = 1;
    cell.id = store.borrow_mut().newFormulaValue(&formula);
    sheet.borrow().setCellData(0, 0, &cell);
    let fml = EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 0, 0);

    fml.setZeroResultIfBlank();
    let mut value = KmoFormulaData::KmoFormulaData();
    store.borrow().formulaValue(sheet.borrow().getCellData(0, 0).id, &mut value);
    assert_eq!(KmoCellValueStore::identifyTypeByID(value.getCacheValueId()), 1);
    assert_eq!(store.borrow_mut().doubleValue(value.getCacheValueId()), 0.0);

    fml.setNumberResult(2.5);
    store.borrow().formulaValue(sheet.borrow().getCellData(0, 0).id, &mut value);
    assert_eq!(store.borrow_mut().doubleValue(value.getCacheValueId()), 2.5);
    fml.setBoolResult(true);
    store.borrow().formulaValue(sheet.borrow().getCellData(0, 0).id, &mut value);
    assert_eq!(KmoCellValueStore::identifyTypeByID(value.getCacheValueId()), 5);
    assert!(store.borrow().booleanValue(value.getCacheValueId()));
    fml.setStringResult("ab");
    store.borrow().formulaValue(sheet.borrow().getCellData(0, 0).id, &mut value);
    // java identifyTypeByID maps the internal formula-string tag 4 to public type 2.
    assert_eq!(KmoCellValueStore::identifyTypeByID(value.getCacheValueId()), 2);
    assert_eq!(store.borrow_mut().stringValue(value.getCacheValueId()), "ab");
    let id = sheet.borrow().getCellData(0, 0).id;
    fml.setStringResult("ab");
    assert_eq!(sheet.borrow().getCellData(0, 0).id, id);
}
