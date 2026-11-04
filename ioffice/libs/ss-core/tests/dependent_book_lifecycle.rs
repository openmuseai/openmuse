use ss_core::java_runtime::Rc;

use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{ref_ptg, serializePtgs};
use ss_core::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::eval::EvaluationCellFormula::EvaluationCellFormula;
use ss_core::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use ss_core::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;
use ss_core::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;

#[test]
fn book_reuses_one_dependent_graph_until_oom() {
    let book = KmoBook::KmoBook__parent(false);
    let first = book.getDependent().expect("ordinary book has a dependency graph");
    let second = book.getDependent().expect("graph remains available");
    assert!(Rc::ptr_eq(&first, &second));
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let through_eval = workbook.getDependent().expect("evaluation workbook uses the book graph");
    assert!(Rc::ptr_eq(&first, &through_eval));
    book.setOOM();
    assert!(book.isOOM());
    assert!(book.getDependent().is_none());
}

#[test]
fn book_dependent_registers_real_formula_tokens_from_cell_store() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let sv = book.getSpreadsheetVersion();
    let tokens = vec![ref_ptg(4, 5, false, false, sv)];
    let encoded = serializePtgs(&tokens);
    let store = book.getCellValueStore();
    let tokens_id = store.borrow_mut().newFormulaTokens(Some(&encoded));
    let mut data = KmoFormulaData::KmoFormulaData();
    data.setTokens(tokens_id, encoded.len() as i32);
    let mut cell = KmoCellData::default();
    cell.formulaType = 1;
    cell.id = store.borrow_mut().newFormulaValue(&data);
    sheet.borrow().setCellData(9, 9, &cell);

    let formula: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 9, 9));
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let graph = book.getDependent().unwrap();
    graph.borrow_mut().registerFml(&formula, &formula.getTokens(), &workbook, false);
    let mut ordinary = Vec::new();
    let mut shared = Vec::new();
    graph.borrow().queryStaticReferred(sheet.borrow().getSheetUid(), 4, 5, &mut ordinary, &mut shared);
    assert_eq!(ordinary.len(), 1);
    assert!(shared.is_empty());
    graph.borrow_mut().unregisterFml(&formula, &workbook);
    ordinary.clear();
    graph.borrow().queryStaticReferred(sheet.borrow().getSheetUid(), 4, 5, &mut ordinary, &mut shared);
    assert!(ordinary.is_empty());
}
