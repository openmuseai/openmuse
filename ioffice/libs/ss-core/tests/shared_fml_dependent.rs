use std::any::Any;
use ss_core::java_runtime::Rc;

use ss_core::cn::wps::et::ss::dependent::SharedFmlDependent::SharedFmlDependent;
use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{area_ptg, ref3d_ptg, ref_ptg, Ptg, PtgKind};
use ss_core::cn::wps::et::ss::interfaces::IEvalSharedFormula::IEvalSharedFormula;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

struct SharedFormula {
    uid: i32,
    first_row: i32,
    last_row: i32,
    first_col: i32,
    last_col: i32,
    tokens: Vec<Ptg>,
}

impl IEvalSharedFormula for SharedFormula {
    fn getSheetUid(&self) -> i32 { self.uid }
    fn getSheetIndex(&self) -> i32 { 0 }
    fn getFmlRow(&self) -> i32 { self.first_row }
    fn getFmlCol(&self) -> i32 { self.first_col }
    fn getFmlLastRow(&self) -> i32 { self.last_row }
    fn getFmlLastCol(&self) -> i32 { self.last_col }
    fn getTokens(&self) -> Vec<Ptg> { self.tokens.clone() }
    fn equals(&self, other: &dyn IEvalSharedFormula) -> bool {
        other.as_any().downcast_ref::<SharedFormula>().is_some_and(|x|
            self.uid == x.uid && self.first_row == x.first_row && self.last_row == x.last_row)
    }
    fn hashCode(&self) -> i32 { self.uid ^ self.first_row ^ self.last_row }
    fn as_any(&self) -> &dyn Any { self }
}

#[test]
fn relative_cell_reference_marks_only_the_reached_shared_formula_cells() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let sv = book.getSpreadsheetVersion();
    let formula: Rc<dyn IEvalSharedFormula> = Rc::new(SharedFormula {
        uid: sheet.borrow().getSheetUid(), first_row: 10, last_row: 12, first_col: 20, last_col: 21,
        tokens: vec![ref_ptg(5, 7, true, true, sv)],
    });
    let mut dep = SharedFmlDependent::SharedFmlDependent(sv);
    dep.registerFml(&formula, &workbook, false);
    let mut hits = Vec::new();
    dep.queryStaticReferred(formula.getSheetUid(), 5, 7, &mut hits);
    assert_eq!(hits.len(), 1);
    assert_eq!((hits[0].marked.getFirstRow(), hits[0].marked.getLastRow(), hits[0].marked.getFirstCol(), hits[0].marked.getLastCol()), (10, 10, 20, 20));
    hits.clear();
    dep.queryStaticReferred(formula.getSheetUid(), 7, 8, &mut hits);
    assert_eq!(hits.len(), 1);
    assert_eq!((hits[0].marked.getFirstRow(), hits[0].marked.getLastRow(), hits[0].marked.getFirstCol(), hits[0].marked.getLastCol()), (12, 12, 21, 21));
    dep.unregisterFml(&formula, &workbook);
    hits.clear();
    dep.queryStaticReferred(formula.getSheetUid(), 5, 7, &mut hits);
    assert!(hits.is_empty());
}

#[test]
fn absolute_reference_marks_the_whole_shared_formula_range() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let sv = book.getSpreadsheetVersion();
    let formula: Rc<dyn IEvalSharedFormula> = Rc::new(SharedFormula {
        uid: sheet.borrow().getSheetUid(), first_row: 1, last_row: 3, first_col: 4, last_col: 5,
        tokens: vec![ref_ptg(8, 9, false, false, sv)],
    });
    let mut dep = SharedFmlDependent::SharedFmlDependent(sv);
    dep.registerFml(&formula, &workbook, false);
    let mut hits = Vec::new();
    dep.queryStaticReferred(formula.getSheetUid(), 8, 9, &mut hits);
    assert_eq!(hits.len(), 1);
    assert_eq!((hits[0].marked.getFirstRow(), hits[0].marked.getLastRow(), hits[0].marked.getFirstCol(), hits[0].marked.getLastCol()), (1, 3, 4, 5));
}

#[test]
fn relative_reference_wraps_at_both_excel97_axes() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let sv = SpreadsheetVersion::EXCEL97;
    let formula: Rc<dyn IEvalSharedFormula> = Rc::new(SharedFormula {
        uid: sheet.borrow().getSheetUid(), first_row: 10, last_row: 12, first_col: 20, last_col: 22,
        tokens: vec![ref_ptg(65534, 254, true, true, sv)],
    });
    let mut dep = SharedFmlDependent::SharedFmlDependent(sv);
    dep.registerFml(&formula, &workbook, false);
    let mut hits = Vec::new();
    dep.queryStaticReferred(formula.getSheetUid(), 0, 0, &mut hits);
    assert_eq!(hits.len(), 1);
    assert_eq!((hits[0].marked.getFirstRow(), hits[0].marked.getFirstCol()), (12, 22));
}

#[test]
fn relative_area_reference_marks_the_intersecting_shared_cells() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let sv = book.getSpreadsheetVersion();
    let formula: Rc<dyn IEvalSharedFormula> = Rc::new(SharedFormula {
        uid: sheet.borrow().getSheetUid(), first_row: 10, last_row: 11, first_col: 20, last_col: 21,
        tokens: vec![area_ptg(5, 6, 7, 8, true, true, true, true, sv)],
    });
    let mut dep = SharedFmlDependent::SharedFmlDependent(sv);
    dep.registerFml(&formula, &workbook, false);
    let mut hits = Vec::new();
    dep.queryStaticReferred(formula.getSheetUid(), 5, 7, &mut hits);
    assert_eq!(hits.len(), 1);
    assert_eq!((hits[0].marked.getFirstRow(), hits[0].marked.getLastRow(), hits[0].marked.getFirstCol(), hits[0].marked.getLastCol()), (10, 10, 20, 20));
    hits.clear();
    dep.queryStaticReferred(formula.getSheetUid(), 6, 8, &mut hits);
    assert_eq!(hits.len(), 1);
    assert_eq!((hits[0].marked.getFirstRow(), hits[0].marked.getLastRow(), hits[0].marked.getFirstCol(), hits[0].marked.getLastCol()), (10, 11, 20, 21));
    dep.unregisterFml(&formula, &workbook);
    hits.clear();
    dep.queryStaticReferred(formula.getSheetUid(), 5, 7, &mut hits);
    assert!(hits.is_empty());
}

#[test]
fn local_ref3d_uses_workbook_sheet_uid_mapping() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let sv = book.getSpreadsheetVersion();
    let ext_sheet_index = book.getLinkTable().borrow_mut().checkNullSupBookExternSheet(0);
    let formula: Rc<dyn IEvalSharedFormula> = Rc::new(SharedFormula {
        uid: sheet.borrow().getSheetUid(), first_row: 10, last_row: 11, first_col: 20, last_col: 20,
        tokens: vec![ref3d_ptg(ext_sheet_index, 5, 7, true, false, sv)],
    });
    let mut dep = SharedFmlDependent::SharedFmlDependent(sv);
    dep.registerFml(&formula, &workbook, false);
    let mut hits = Vec::new();
    dep.queryStaticReferred(sheet.borrow().getSheetUid(), 6, 7, &mut hits);
    assert_eq!(hits.len(), 1);
    assert_eq!((hits[0].marked.getFirstRow(), hits[0].marked.getLastRow()), (11, 11));
}

#[test]
fn external_ref3d_follows_java_register_and_unregister_branches() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let token = Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Ref3DExt { sheet_index1: 0, sheet_index2: 1, row: 5, col_raw: 7, wide: true }, class: 0 };
    let formula: Rc<dyn IEvalSharedFormula> = Rc::new(SharedFormula {
        uid: sheet.borrow().getSheetUid(), first_row: 10, last_row: 11, first_col: 20, last_col: 20,
        tokens: vec![token],
    });
    let mut dep = SharedFmlDependent::SharedFmlDependent(book.getSpreadsheetVersion());
    // java registerTokens case 58 skips Ref3DExtPtg.
    dep.registerFml(&formula, &workbook, false);
    let mut hits = Vec::new();
    dep.queryStaticReferred(formula.getSheetUid(), 5, 7, &mut hits);
    assert!(hits.is_empty());
    // java unregisterTokens case 58 unconditionally casts to Ref3DPtg and throws ClassCastException.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| dep.unregisterFml(&formula, &workbook)));
    assert!(result.is_err());
}
