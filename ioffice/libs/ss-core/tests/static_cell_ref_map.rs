use ss_core::java_runtime::Rc;

use ss_core::cn::wps::et::ss::dependent::refer::StaticCellRefMap::StaticCellRefMap;
use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{name_ptg, ref_ptg, ref3d_ptg, PtgKind};
use ss_core::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::eval::EvaluationCellFormula::EvaluationCellFormula;
use ss_core::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use ss_core::cn::wps::moss::engine::linktb::NameRecord::NameRecord;

#[test]
fn static_cell_map_registers_ref_name_and_local_ref3d_in_java_token_order() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let formula: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 9, 9));
    let sv = book.getSpreadsheetVersion();
    let mut name = NameRecord::NameRecord(sv);
    name.setNameText("SOURCE");
    name.setNameDefinition(vec![ref_ptg(4, 5, false, false, sv)]);
    let link = book.getLinkTable();
    link.borrow_mut().addName(name);
    let ext_sheet_index = link.borrow_mut().checkNullSupBookExternSheet(0);
    let tokens = [
        ref_ptg(4, 5, false, false, sv),
        name_ptg(0),
        ref3d_ptg(ext_sheet_index, 4, 5, false, false, sv),
    ];
    let mut map = StaticCellRefMap::StaticCellRefMap(sv);
    map.registerTokens(&tokens, &formula, &workbook, false);
    let mut hits = Vec::new();
    map.queryReferred(sheet.borrow().getSheetUid(), 4, 5, &mut hits);
    assert_eq!(hits.len(), 3);
    map.unregisterTokens(&tokens, &formula, &workbook);
    hits.clear();
    map.queryReferred(sheet.borrow().getSheetUid(), 4, 5, &mut hits);
    assert!(hits.is_empty());
}

#[test]
fn static_cell_map_skips_addin_name_x_but_handles_anonymous_ref3d() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let formula: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 2, 2));
    let sv = book.getSpreadsheetVersion();
    let name_x = book.getLinkTable().borrow_mut().checkUDFName("MY_UNKNOWN_UDF");
    let PtgKind::NameX { sheet_ref, .. } = name_x.kind else { panic!("NameXPtg") };
    let anonymous_ref = ref3d_ptg(sheet_ref, 7, 8, false, false, sv);
    let mut map = StaticCellRefMap::StaticCellRefMap(sv);
    map.registerTokens(&[name_x, anonymous_ref], &formula, &workbook, false);
    let mut hits = Vec::new();
    map.queryReferred(sheet.borrow().getSheetUid(), 7, 8, &mut hits);
    assert_eq!(hits.len(), 1);
}
