use ss_core::java_runtime::Rc;

use ss_core::cn::wps::et::ss::calcchain::CalcChainNode::CalcChainNode;
use ss_core::cn::wps::et::ss::dependent::refer::base::CellRefMap::CellRefMap;
use ss_core::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::eval::EvaluationCellFormula::EvaluationCellFormula;

#[test]
fn node_identity_follows_java_formula_position_and_sheet_object() {
    let book = KmoBook::KmoBook__parent(false);
    let sheet = book.appendSheet();
    let other_sheet = book.appendSheet();
    let formula = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 12, 7));
    let same_position = Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(), 12, 7));
    let different_sheet = Rc::new(EvaluationCellFormula::EvaluationCellFormula(other_sheet, 12, 7));
    let formula_base: Rc<dyn IEvalFormulaBase> = formula.clone();
    let mut node = CalcChainNode::CalcChainNode(formula_base.clone());
    let same_node = CalcChainNode::CalcChainNode(same_position);
    let other_node = CalcChainNode::CalcChainNode(different_sheet);

    assert_eq!(node.nodeId, CellRefMap::cellPos2Long(sheet.borrow().getSheetUid(), 12, 7));
    assert_eq!(node.hashCode(), formula.hashCode());
    assert!(node.equals(&same_node));
    assert!(!node.equals(&other_node));
    assert_eq!(node.index, 0);
    assert!(!node.interrupted);
    assert!(!node.cycled);
    node.clear();
    assert!(node.refNode.is_none());

    formula_base.lock();
    formula_base.lock();
    assert!(formula_base.islock());
    formula_base.unlock();
    assert!(formula_base.islock());
    formula_base.unlock();
    assert!(!formula_base.islock());
    formula_base.setCycled(true);
    assert!(formula_base.isCycled());
    formula_base.setRef(Some(node.formula.clone()));
    assert!(Rc::ptr_eq(&formula_base.getRef().unwrap(), &node.formula));
    formula_base.setRef(None);
}
