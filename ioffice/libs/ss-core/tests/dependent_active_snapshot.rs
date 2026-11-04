use ss_core::java_runtime::Rc;
use kfc::cn::wps::memor4j::KmoAtomStore::{KmoAtomStore, KmoAtomStoreTrait};
use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{area_ptg, ref_ptg, serializePtgs, Ptg, PtgKind};
use ss_core::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::eval::EvaluationCellFormula::EvaluationCellFormula;
use ss_core::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use ss_core::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;
use ss_core::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;

fn store<R>(operation: impl FnOnce(&mut dyn KmoAtomStoreTrait) -> R) -> R {
    let mut shared = KmoAtomStore::sharedStore();
    operation(shared.as_mut().unwrap().as_mut())
}

#[test]
fn java_active_snapshot_restores_the_original_book_reference_graph_and_formula_flags() {
    KmoAtomStore::enable();
    KmoAtomStore::clearUp();
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let sv = book.getSpreadsheetVersion();
    let tokens = vec![
        ref_ptg(4, 5, false, false, sv),
        area_ptg(10, 11, 12, 13, false, false, false, false, sv),
        Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Name { index: 0 }, class: 0 },
    ];
    let encoded = serializePtgs(&tokens);
    let values = book.getCellValueStore();
    let token_id = values.borrow_mut().newFormulaTokens(Some(&encoded));
    let mut data = KmoFormulaData::KmoFormulaData();
    data.setTokens(token_id, encoded.len() as i32);
    let cell = KmoCellData { formulaType: 1, id: values.borrow_mut().newFormulaValue(&data), ..KmoCellData::default() };
    sheet.borrow().setCellData(20, 20, &cell);
    let formula: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 20, 20));
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let graph = book.getDependent().unwrap();
    let uid = sheet.borrow().getSheetUid();
    let assert_edges = |count| {
        for (row, col) in [(4, 5), (10, 12)] {
            let (mut ordinary, mut shared) = (Vec::new(), Vec::new());
            graph.borrow().queryStaticReferred(uid, row, col, &mut ordinary, &mut shared);
            assert_eq!(ordinary.len(), count);
            assert!(shared.is_empty());
            for item in ordinary { assert!(Rc::ptr_eq(&item, &formula)); }
        }
        let mut named = Vec::new();
        graph.borrow().queryNameReferred(0, &mut named);
        assert_eq!(named.len(), count);
    };

    // java start -> registerFml(inTrans=true) -> commit retains the actual map instances.
    store(|s| s.start());
    graph.borrow_mut().registerFml(&formula, &tokens, &workbook, true);
    assert_edges(1);
    assert!(store(|s| s.commit()));
    formula.setNeedRecalculationFlag(false);
    // Java's singleton and object references can also be used by another thread.
    std::thread::spawn(|| store(|s| s.undo())).join().unwrap();
    assert_edges(0);
    assert!(formula.isNeedRecalculationFlagSet());
    formula.setNeedRecalculationFlag(false);
    store(|s| s.redo());
    assert_edges(1);
    assert!(formula.isNeedRecalculationFlagSet());

    // java unregisterFml records DEL; undo restores all three reference tables.
    store(|s| s.start());
    graph.borrow_mut().unregisterFml(&formula, &workbook);
    assert!(store(|s| s.commit()));
    assert_edges(0);
    store(|s| s.undo());
    assert_edges(1);
    store(|s| s.redo());
    assert_edges(0);

    // java Backup.copy forces tip 0: repeated ADD operations remain separate images.
    store(|s| s.start());
    graph.borrow_mut().registerFml(&formula, &tokens, &workbook, true);
    graph.borrow_mut().registerFml(&formula, &tokens, &workbook, true);
    assert_edges(2);
    store(|s| s.discard());
    assert_edges(0);

    // java Dependent routes a real shared master to SharedFmlDependent's generic
    // point/area maps. The snapshot must keep those original indexes as well.
    use ss_core::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper;
    use ss_core::cn::wps::et::ss::formula::Formula::Formula;
    let shared_tokens = vec![ref_ptg(4, 5, false, false, sv), ref_ptg(6, 7, true, true, sv)];
    let relative_tokens = FormulaHelper::replaceWithNPtgs(&shared_tokens, 30, 30, sv);
    sheet.borrow().getExpFmlManager().borrow().registerSharedFml(30, 31, 30, 31, Formula::create(&relative_tokens, sv));
    let exp = serializePtgs(&[Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Exp { row: 30, col: 30, wide: false }, class: 0 }]);
    let exp_id = values.borrow_mut().newFormulaTokens(Some(&exp));
    let mut shared_data = KmoFormulaData::KmoFormulaData();
    shared_data.setTokens(exp_id, exp.len() as i32);
    let shared_id = values.borrow_mut().newFormulaValue(&shared_data);
    sheet.borrow().setCellData(30, 30, &KmoCellData { formulaType: 3, id: shared_id, ..KmoCellData::default() });
    let master: Rc<dyn IEvalFormulaBase> = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 30, 30));
    assert!(master.as_cell_formula().unwrap().isMasterSharedFormula());
    store(|s| s.start());
    graph.borrow_mut().registerFml(&master, &master.getTokens(), &workbook, true);
    assert!(store(|s| s.commit()));
    let shared_hits = |row, col| {
        let (mut ordinary, mut shared) = (Vec::new(), Vec::new());
        graph.borrow().queryStaticReferred(uid, row, col, &mut ordinary, &mut shared);
        assert!(ordinary.is_empty());
        shared
    };
    let absolute = shared_hits(4, 5);
    assert_eq!(absolute.len(), 1);
    assert_eq!((absolute[0].marked.getFirstRow(), absolute[0].marked.getLastRow()), (30, 31));
    let relative = shared_hits(6, 7);
    assert_eq!(relative.len(), 1);
    assert_eq!((relative[0].marked.getFirstRow(), relative[0].marked.getLastRow()), (30, 30));
    store(|s| s.undo());
    assert!(shared_hits(4, 5).is_empty() && shared_hits(6, 7).is_empty());
    store(|s| s.redo());
    assert!(Rc::ptr_eq(&absolute[0].fml, &shared_hits(4, 5)[0].fml));
    assert!(Rc::ptr_eq(&relative[0].fml, &shared_hits(6, 7)[0].fml));
    store(|s| s.start());
    graph.borrow_mut().unregisterFml(&master, &workbook);
    assert!(store(|s| s.commit()));
    assert!(shared_hits(4, 5).is_empty() && shared_hits(6, 7).is_empty());
    store(|s| s.undo());
    assert_eq!(shared_hits(4, 5).len(), 1);
    assert_eq!(shared_hits(6, 7).len(), 1);
    KmoAtomStore::clearUp();
}
