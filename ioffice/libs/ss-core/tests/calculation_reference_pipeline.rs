static CALCULATION_TEST: std::sync::Mutex<()> = std::sync::Mutex::new(());
use ss_core::java_runtime::RefCell;
use ss_core::{
    cn::wps::{
        et::ss::formula::{
            ptg::Ptg::{ref_ptg, Ptg, PtgKind},
            Formula::Formula,
        },
        moss::{
            app::{KmoBook::KmoBook, KmoSheet::KmoSheet},
            engine::grid::{KmoCellData::KmoCellData, KmoFormulaData::KmoFormulaData},
        },
    },
    java_runtime::Rc,
};
fn token(kind: PtgKind) -> Ptg {
    Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind, class: 0 }
}
fn put(
    book: &Rc<KmoBook>,
    sheet: &Rc<RefCell<KmoSheet>>,
    row: i32,
    col: i32,
    ptgs: &[Ptg],
    kind: i32,
) {
    let f = Formula::create(ptgs, book.getSpreadsheetVersion());
    let values = book.getCellValueStore();
    let tid = values.borrow_mut().newFormulaTokens(Some(&f.getByteEncoding().unwrap().borrow()));
    let mut data = KmoFormulaData::KmoFormulaData();
    data.setTokens(tid, f.getEncodedTokenSize());
    let id = values.borrow_mut().newFormulaValue(&data);
    sheet.borrow().setCellData(
        row,
        col,
        &KmoCellData {
            id,
            formulaType: kind,
            ..KmoCellData::default()
        },
    );
}
fn calculate(book: &Rc<KmoBook>) {
    let cc = book.getCalculationChain().unwrap();
    cc.init();
    cc.forceMarkRecalculation(-1, -1, -1);
    cc.openCalc();
    assert!(cc.calcChain.unportedEvaluationBranches().is_empty());
}
fn num(book: &Rc<KmoBook>, r: i32, c: i32) -> f64 {
    let sheet = book.sheetAt(0);
    let id = sheet.borrow().getCellValueId(r, c);
    let result = book.getCellValueStore().borrow_mut().doubleValue(id);
    result
}
#[test]
fn byte_references_follow_evaluation_callback_and_propagate_div_zero() {
    let _test = CALCULATION_TEST.lock().unwrap();
    let book = KmoBook::KmoBook__parent(true);
    let s = book.sheetAt(0);
    let sv = book.getSpreadsheetVersion();
    put(
        &book,
        &s,
        1,
        0,
        &[
            ref_ptg(1, 1, false, false, sv),
            token(PtgKind::Int(1)),
            token(PtgKind::Add),
        ],
        1,
    );
    put(&book, &s, 1, 1, &[token(PtgKind::Int(5))], 1);
    put(
        &book,
        &s,
        2,
        0,
        &[
            ref_ptg(1, 0, false, false, sv),
            token(PtgKind::Int(0)),
            token(PtgKind::Divide),
        ],
        1,
    );
    put(
        &book,
        &s,
        3,
        0,
        &[
            ref_ptg(2, 0, false, false, sv),
            token(PtgKind::Int(2)),
            token(PtgKind::Add),
        ],
        1,
    );
    calculate(&book);
    assert_eq!(num(&book, 1, 0), 6.0);
    assert_eq!(num(&book, 1, 1), 5.0);
    for r in [2, 3] {
        let id = s.borrow().getCellValueId(r, 0);
        assert_eq!(book.getCellValueStore().borrow().errorValue(id), 7);
    }
}
#[test]
fn real_formula_cycles_use_thread_calc_stack_and_zero_only_blank_caches() {
    let _test = CALCULATION_TEST.lock().unwrap();
    let book = KmoBook::KmoBook__parent(true);
    let s = book.sheetAt(0);
    let sv = book.getSpreadsheetVersion();
    put(&book, &s, 0, 0, &[ref_ptg(0, 1, false, false, sv)], 1);
    put(&book, &s, 0, 1, &[ref_ptg(0, 0, false, false, sv)], 1);
    calculate(&book);
    assert_eq!(num(&book, 0, 0), 0.0);
    assert_eq!(num(&book, 0, 1), 0.0);
    let cc = book.getCalculationChain().unwrap();
    let uid = s.borrow().getSheetUid();
    for c in 0..2 {
        let f = cc.calcChain.getFml(uid, 0, c).unwrap();
        assert!(f.isCycled());
        assert!(!f.isNeedRecalculationFlagSet());
    }
}
#[test]
fn array_master_registration_and_scalar_result_broadcast_use_original_array_class() {
    let _test = CALCULATION_TEST.lock().unwrap();
    let book = KmoBook::KmoBook__parent(true);
    let s = book.sheetAt(0);
    let sv = book.getSpreadsheetVersion();
    s.borrow().getExpFmlManager().borrow().registerArrayFml(
        4,
        5,
        2,
        3,
        Formula::create(&[token(PtgKind::Int(4))], sv),
    );
    let exp = token(PtgKind::Exp {
        row: 4,
        col: 2,
        wide: false,
    });
    for r in 4..=5 {
        for c in 2..=3 {
            put(&book, &s, r, c, &[exp.clone()], 2);
        }
    }
    calculate(&book);
    assert_eq!(book.getFormulaSize(), 1);
    for r in 4..=5 {
        for c in 2..=3 {
            assert_eq!(num(&book, r, c), 4.0);
            assert_eq!(s.borrow().getCellData(r, c).formulaType, 2);
        }
    }
}

#[test]
fn unchecked_java_numeric_exception_does_not_terminate_the_unit() {
    let _test = CALCULATION_TEST.lock().unwrap();
    use ss_core::cn::wps::et::ss::{
        calcchain::{threadpool::UnitThread::UnitThread, CalcChain::ProcessCount},
        formula::eval::NumberEval::NumberEval,
        util::RowColKeyHelper::RowColKeyHelper,
    };
    let book = KmoBook::KmoBook__parent(true);
    let sheet = book.sheetAt(0);
    put(
        &book,
        &sheet,
        0,
        0,
        &[
            token(PtgKind::Number(f64::NAN)),
            token(PtgKind::Int(1)),
            token(PtgKind::Add),
        ],
        1,
    );
    put(&book, &sheet, 0, 1, &[token(PtgKind::Int(2))], 1);
    let cc = book.getCalculationChain().unwrap();
    cc.init();
    cc.forceMarkRecalculation(-1, -1, -1);
    let uid = sheet.borrow().getSheetUid();
    cc.cfe.valuePos.lock().unwrap().entry(uid).or_default();
    ProcessCount.store(1, std::sync::atomic::Ordering::SeqCst);
    let unit = Rc::new(UnitThread::UnitThread(
        cc.calcChain.clone(),
        cc.calcChain.getNode(uid, 0, 0),
        cc.calcChain.getNode(uid, 0, 1),
    ));
    unit.run();
    assert!(cc
        .calcChain
        .getFml(uid, 0, 0)
        .unwrap()
        .isNeedRecalculationFlagSet());
    assert!(!cc
        .calcChain
        .getFml(uid, 0, 1)
        .unwrap()
        .isNeedRecalculationFlagSet());
    let positions = cc.cfe.valuePos.lock().unwrap();
    let value = &positions[&uid][&RowColKeyHelper::makeLongKey(0, 1)];
    assert_eq!(
        value
            .as_any()
            .downcast_ref::<NumberEval>()
            .unwrap()
            .getNumberValue(),
        2.0
    );
}
