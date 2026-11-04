use ss_core::java_runtime::Rc;

use ss_core::cn::wps::et::ss::dependent::refer::StaticNameRefMap::StaticNameRefMap;
use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{name_ptg, PtgKind};
use ss_core::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::eval::EvaluationCellFormula::EvaluationCellFormula;
use ss_core::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use ss_core::cn::wps::moss::engine::linktb::NameRecord::NameRecord;

#[test]
fn static_name_map_recurses_only_when_java_class_sequence_differs() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let formula: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet, 1, 1));
    let mut first = NameRecord::NameRecord(book.getSpreadsheetVersion());
    first.setNameText("FIRST");
    first.setNameDefinition(vec![name_ptg(1), name_ptg(2)]);
    let mut second = NameRecord::NameRecord(book.getSpreadsheetVersion());
    second.setNameText("SECOND");
    second.setNameDefinition(vec![name_ptg(2)]);
    let mut third = NameRecord::NameRecord(book.getSpreadsheetVersion());
    third.setNameText("THIRD");
    let table = book.getLinkTable();
    table.borrow_mut().addName(first);
    table.borrow_mut().addName(second);
    table.borrow_mut().addName(third);

    let mut map = StaticNameRefMap::StaticNameRefMap(book.getSpreadsheetVersion());
    let token = name_ptg(0);
    map.registerName__tokens(&[token.clone()], &formula, &workbook, false);
    for (index, expected) in [(0, 1), (1, 1), (2, 2)] {
        let mut hits = Vec::new();
        map.queryReferred(index, &mut hits);
        assert_eq!(hits.len(), expected);
    }
    // Java unregisterName scans the supplied token array directly; it does not
    // recursively unregister the referenced name definition.
    map.unregisterName__tokens(&[token], &formula, &workbook);
    let mut direct = Vec::new();
    map.queryReferred(0, &mut direct);
    assert!(direct.is_empty());
    let mut nested = Vec::new();
    map.queryReferred(1, &mut nested);
    assert_eq!(nested.len(), 1);
}

#[test]
fn static_name_map_skips_addin_name_x_and_registers_local_name_x() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let formula: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet, 1, 1));
    let mut record = NameRecord::NameRecord(book.getSpreadsheetVersion());
    record.setNameText("LOCAL");
    let table = book.getLinkTable();
    table.borrow_mut().addName(record);
    let local_sheet_ref = table.borrow_mut().checkNullSupBookExternSheet(0);
    let local = ss_core::cn::wps::et::ss::formula::ptg::Ptg::name_x_ptg(local_sheet_ref, 0);
    let addin = table.borrow_mut().checkUDFName("MY_UNKNOWN_UDF");
    assert!(matches!(addin.kind, PtgKind::NameX { .. }));
    let mut map = StaticNameRefMap::StaticNameRefMap(book.getSpreadsheetVersion());
    map.registerName__tokens(&[local, addin], &formula, &workbook, false);
    let mut hits = Vec::new();
    map.queryReferred(0, &mut hits);
    assert_eq!(hits.len(), 1);
}
