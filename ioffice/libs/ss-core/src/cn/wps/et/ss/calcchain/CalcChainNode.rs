// java package cn.wps.et.ss.calcchain;

use crate::java_runtime::RefCell;
use crate::java_runtime::{Rc, Weak};

// java import cn.wps.et.ss.dependent.refer.base.CellRefMap;
use crate::cn::wps::et::ss::dependent::refer::base::CellRefMap::CellRefMap;
// java import cn.wps.et.ss.interfaces.IEvalFormulaBase;
use crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;

// java public final class CalcChainNode {
pub struct CalcChainNode {
    // java public final IEvalFormulaBase formula;
    pub formula: Rc<dyn IEvalFormulaBase>,
    // java public CalcChainNode preCalcChainNode;
    pub preCalcChainNode: Option<Weak<RefCell<CalcChainNode>>>,
    // java public CalcChainNode nextCalcChainNode;
    pub nextCalcChainNode: Option<Rc<RefCell<CalcChainNode>>>,
    // java public int index;
    pub index: i32,
    // java public boolean interrupted = false;
    pub interrupted: bool,
    // java public boolean cycled = false;
    pub cycled: bool,
    // java public long nodeId;
    pub nodeId: i64,
    // java public CalcChainNode refNode;
    pub refNode: Option<Weak<RefCell<CalcChainNode>>>,
}

impl CalcChainNode {
    // java public CalcChainNode(IEvalFormulaBase fml)
    pub fn CalcChainNode(fml: Rc<dyn IEvalFormulaBase>) -> Self {
        // java this.formula = fml;
        let mut node = Self { formula: fml, preCalcChainNode: None, nextCalcChainNode: None, index: 0, interrupted: false, cycled: false, nodeId: 0, refNode: None };
        // java this.setId();
        node.setId();
        node
    }

    // java private void setId()
    fn setId(&mut self) {
        // java this.nodeId = CellRefMap.cellPos2Long(this.formula.getSheetUid(), this.formula.getFmlRow(), this.formula.getFmlCol());
        self.nodeId = CellRefMap::cellPos2Long(self.formula.getSheetUid(), self.formula.getFmlRow(), self.formula.getFmlCol());
    }

    // java public void clear()
    pub fn clear(&mut self) {
        // java this.refNode = null;
        self.refNode = None;
    }

    // java public boolean equals(Object o)
    pub fn equals(&self, other: &CalcChainNode) -> bool {
        // java return this.formula.equals(ccn.formula);
        self.formula.equals(other.formula.as_ref())
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
        // java return this.formula.hashCode();
        self.formula.hashCode()
    }
}
