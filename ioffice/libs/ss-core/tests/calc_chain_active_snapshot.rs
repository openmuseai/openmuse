use kfc::cn::wps::memor4j::KmoAtomStore::{KmoAtomStore, KmoAtomStoreTrait};
use ss_core::{
    cn::wps::{
        et::ss::{
            formula::{ptg::Ptg::ref_ptg, Formula::Formula},
            interfaces::IEvalFormulaBase::IEvalFormulaBase,
        },
        moss::{
            app::KmoBook::KmoBook,
            engine::{
                eval::EvaluationCellFormula::EvaluationCellFormula,
                grid::{KmoCellData::KmoCellData, KmoFormulaData::KmoFormulaData},
            },
        },
    },
    java_runtime::Rc,
};
fn store<R>(f: impl FnOnce(&mut dyn KmoAtomStoreTrait) -> R) -> R {
    let mut s = KmoAtomStore::sharedStore();
    f(s.as_mut().unwrap().as_mut())
}
#[test]
fn calc_chain_and_live_dependent_graph_share_commit_undo_redo() {
    KmoAtomStore::enable();
    KmoAtomStore::clearUp();
    let book = KmoBook::KmoBook__parent(true);
    let sheet = book.sheetAt(0);
    let sv = book.getSpreadsheetVersion();
    let formula = Formula::create(&[ref_ptg(4, 5, false, false, sv)], sv);
    let values = book.getCellValueStore();
    let mut data = KmoFormulaData::KmoFormulaData();
    let tid = values
        .borrow_mut()
        .newFormulaTokens(Some(&formula.getByteEncoding().unwrap().borrow()));
    data.setTokens(tid, formula.getEncodedTokenSize());
    let id = values.borrow_mut().newFormulaValue(&data);
    sheet.borrow().setCellData(
        0,
        0,
        &KmoCellData {
            id,
            formulaType: 1,
            ..KmoCellData::default()
        },
    );
    let f: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(
        sheet.clone(),
        0,
        0,
    ));
    let uid = f.getSheetUid();
    let cc = book.getCalculationChain().unwrap();
    // The live graph is constructed before the transaction, as in loaded-book init.
    // Java atoms born at the current transaction tip do not snapshot their initial data.
    let _graph = book.getDependent().unwrap();
    let edges = || {
        let mut out = Vec::new();
        book.getDependent()
            .unwrap()
            .borrow()
            .getStaticCellRefMap()
            .queryReferred(uid, 4, 5, &mut out);
        out.len()
    };
    store(|s| s.start());
    cc.calcChain.registerFml(f.clone(), true);
    assert!(store(|s| s.commit()));
    assert_eq!(cc.getFormulaSize(), 1);
    assert_eq!(edges(), 1);
    store(|s| s.undo());
    assert!(cc.calcChain.getFml(uid, 0, 0).is_none());
    assert_eq!(cc.getFormulaSize(), 0);
    assert_eq!(edges(), 0);
    f.setNeedRecalculationFlag(false);
    store(|s| s.redo());
    assert!(Rc::ptr_eq(&cc.calcChain.getFml(uid, 0, 0).unwrap(), &f));
    assert_eq!(edges(), 1);
    assert!(f.isNeedRecalculationFlagSet());
    store(|s| s.start());
    cc.calcChain.unregisterFml(&f);
    assert!(store(|s| s.commit()));
    assert_eq!(edges(), 0);
    store(|s| s.undo());
    assert_eq!(edges(), 1);
    assert!(Rc::ptr_eq(&cc.calcChain.getFml(uid, 0, 0).unwrap(), &f));
    store(|s| s.redo());
    assert_eq!(edges(), 0);
    assert_eq!(cc.getFormulaSize(), 0);
    // The same CalcChain snapshot also owns the real shared master's graph.
    use ss_core::cn::wps::et::ss::{
        calcchain::FormulaHelper::FormulaHelper,
        formula::ptg::Ptg::{serializePtgs, Ptg, PtgKind},
    };
    let absolute = vec![
        ref_ptg(4, 5, false, false, sv),
        ref_ptg(6, 7, true, true, sv),
    ];
    let relative = FormulaHelper::replaceWithNPtgs(&absolute, 30, 30, sv);
    sheet
        .borrow()
        .getExpFmlManager()
        .borrow()
        .registerSharedFml(30, 31, 30, 31, Formula::create(&relative, sv));
    let exp = serializePtgs(&[Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
        kind: PtgKind::Exp {
            row: 30,
            col: 30,
            wide: false,
        },
        class: 0,
    }]);
    let exp_id = values.borrow_mut().newFormulaTokens(Some(&exp));
    let mut data = KmoFormulaData::KmoFormulaData();
    data.setTokens(exp_id, exp.len() as i32);
    let id = values.borrow_mut().newFormulaValue(&data);
    sheet.borrow().setCellData(
        30,
        30,
        &KmoCellData {
            id,
            formulaType: 3,
            ..KmoCellData::default()
        },
    );
    let shared: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(
        sheet.clone(),
        30,
        30,
    ));
    let shared_edges = || {
        let (mut ordinary, mut areas) = (Vec::new(), Vec::new());
        book.getDependent().unwrap().borrow().queryStaticReferred(
            uid,
            4,
            5,
            &mut ordinary,
            &mut areas,
        );
        areas.len()
    };
    store(|s| s.start());
    cc.calcChain.registerFml(shared.clone(), true);
    assert!(store(|s| s.commit()));
    assert_eq!(shared_edges(), 1);
    store(|s| s.undo());
    assert_eq!(shared_edges(), 0);
    assert!(cc.calcChain.getFml(uid, 30, 30).is_none());
    store(|s| s.redo());
    assert_eq!(shared_edges(), 1);
    assert!(Rc::ptr_eq(
        &cc.calcChain.getFml(uid, 30, 30).unwrap(),
        &shared
    ));
    KmoAtomStore::clearUp();
}
