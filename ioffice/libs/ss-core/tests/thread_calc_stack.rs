use ss_core::java_runtime::Rc;
use ss_core::cn::wps::et::ss::calcchain::ThreadCalcStack::ThreadCalcStack;
use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{int_ptg, serializePtgs};
use ss_core::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::eval::EvaluationCellFormula::EvaluationCellFormula;
use ss_core::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;
use ss_core::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;

fn formula(book: &Rc<KmoBook>, col: i32) -> Rc<dyn IEvalFormulaBase> {
    let sheet = book.sheetAt(0);
    let bytes = serializePtgs(&[int_ptg(1)]);
    let values = book.getCellValueStore();
    let token = values.borrow_mut().newFormulaTokens(Some(&bytes));
    let mut data = KmoFormulaData::KmoFormulaData();
    data.setTokens(token, bytes.len() as i32);
    let id = values.borrow_mut().newFormulaValue(&data);
    sheet.borrow().setCellData(0, col, &KmoCellData { id, formulaType: 1, ..KmoCellData::default() });
    Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet, 0, col))
}

#[test]
fn java_thread_stack_detects_identity_cycles_and_writes_zero_for_blank_results() {
    let book = KmoBook::KmoBook__parent(true);
    let a = formula(&book, 0);
    let b = formula(&book, 1);
    let stack = ThreadCalcStack::new();
    stack.add(&a);
    stack.addRef(&b);
    stack.add(&b);
    stack.addRef(&a);
    assert!(stack.checkCycl(&a));
    assert!(a.isCycled() && b.isCycled());
    assert!(!a.isNeedRecalculationFlagSet() && !b.isNeedRecalculationFlagSet());
    assert!(a.getRef().is_none());
    let sheet = book.sheetAt(0);
    for col in [0, 1] {
        assert_eq!(sheet.borrow().getCellValueType__2(0, col), 1);
        let id = sheet.borrow().getCellValueId(0, col);
        assert_eq!(book.getCellValueStore().borrow_mut().doubleValue(id), 0.0);
    }
    b.setRef(None);
    stack.reSet();
}

#[test]
fn java_thread_stack_is_per_thread_and_compares_object_identity() {
    let book = KmoBook::KmoBook__parent(true);
    let a = formula(&book, 0);
    let b = formula(&book, 1);
    let same_cell: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(book.sheetAt(0), 0, 0));
    assert!(a.equals(same_cell.as_ref()));
    a.setRef(Some(same_cell.clone()));
    let stack = Rc::new(ThreadCalcStack::new());
    assert!(!stack.checkCycl(&a));
    a.setRef(None);
    stack.add(&a);
    let (worker_stack, worker_a, worker_b) = (stack.clone(), same_cell.clone(), b.clone());
    std::thread::spawn(move || {
        worker_stack.add(&worker_a);
        worker_stack.addRef(&worker_b);
        worker_stack.pop();
    }).join().unwrap();
    assert!(a.getRef().is_none());
    assert!(Rc::ptr_eq(&same_cell.getRef().unwrap(), &b));
    stack.addRef(&b);
    assert!(Rc::ptr_eq(&a.getRef().unwrap(), &b));
    stack.pop();
    a.setRef(None);
    stack.addRef(&same_cell);
    assert!(a.getRef().is_none());
    same_cell.setRef(None);
    stack.reSet();
}
