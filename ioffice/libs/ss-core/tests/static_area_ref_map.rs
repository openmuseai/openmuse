use ss_core::java_runtime::Rc;

use ss_core::cn::wps::et::ss::dependent::refer::StaticAreaRefMap::StaticAreaRefMap;
use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{area3d_ptg, area_ptg, name_ptg};
use ss_core::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::eval::EvaluationCellFormula::EvaluationCellFormula;
use ss_core::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use ss_core::cn::wps::moss::engine::linktb::NameRecord::NameRecord;

#[test]
fn static_area_map_registers_area_name_and_local_area3d_in_java_token_order() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let formula: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 9, 9));
    let sv = book.getSpreadsheetVersion();
    let mut name = NameRecord::NameRecord(sv);
    name.setNameText("SOURCE_AREA");
    name.setNameDefinition(vec![area_ptg(2, 6, 3, 7, false, false, false, false, sv)]);
    let link = book.getLinkTable();
    link.borrow_mut().addName(name);
    let ext_sheet_index = link.borrow_mut().checkNullSupBookExternSheet(0);
    let tokens = [
        area_ptg(2, 6, 3, 7, false, false, false, false, sv),
        name_ptg(0),
        area3d_ptg(ext_sheet_index, 2, 6, 3, 7, false, false, false, false, sv),
    ];
    let mut map = StaticAreaRefMap::StaticAreaRefMap(sv);
    map.registerTokens(&tokens, &formula, &workbook, false);
    let mut hits = Vec::new();
    map.queryAreaReferred(sheet.borrow().getSheetUid(), 4, 5, &mut hits);
    assert_eq!(hits.len(), 3);
    map.unregisterTokens(&tokens, &formula, &workbook);
    hits.clear();
    map.queryAreaReferred(sheet.borrow().getSheetUid(), 4, 5, &mut hits);
    assert!(hits.is_empty());
}
