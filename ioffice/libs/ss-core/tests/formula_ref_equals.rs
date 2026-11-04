use ss_core::java_runtime::Rc;

use ss_core::cn::wps::et::ss::dependent::refer::FormulaRef::FormulaRef;
use ss_core::cn::wps::et::ss::dependent::refer::base::NameRefMap::NameRefMap;
use ss_core::cn::wps::et::ss::dependent::refer::base::CellRefMap::CellRefMap;
use ss_core::cn::wps::et::ss::dependent::refer::base::AreaRefMap::AreaRefMap;
use ss_core::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use ss_core::cn::wps::et::ss::util::Area::Area;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::eval::EvaluationCellFormula::EvaluationCellFormula;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

#[test]
fn formula_list_removal_uses_java_formula_equals() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let original: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 12, 7));
    let same_cell_new_object: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 12, 7));
    assert!(!Rc::ptr_eq(&original, &same_cell_new_object));
    let mut map = NameRefMap::NameRefMap(SpreadsheetVersion::EXCEL2007);
    map.addToMap(3, FormulaRef(original.clone()), false);
    assert!(map.removeFromMap(3, &FormulaRef(same_cell_new_object)).is_some());
    let mut referred = Vec::new();
    map.queryReferred(3, &mut referred);
    assert!(referred.is_empty());
}

#[test]
fn cell_and_area_maps_remove_equivalent_formula_instances() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let original: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 12, 7));
    let equivalent: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet, 12, 7));
    let mut cells = CellRefMap::CellRefMap(SpreadsheetVersion::EXCEL2007);
    cells.addToMap(1, 5, 6, FormulaRef(original.clone()), false);
    assert!(cells.removeFromMap(1, 5, 6, &FormulaRef(equivalent.clone())).is_some());
    let mut areas = AreaRefMap::AreaRefMap(SpreadsheetVersion::EXCEL2007);
    let area = Area::Area(1, 3, 2, 4);
    areas.addToMap(1, &area, FormulaRef(original), false);
    assert!(areas.removeFromMap(1, &area, &FormulaRef(equivalent)).is_some());
}
