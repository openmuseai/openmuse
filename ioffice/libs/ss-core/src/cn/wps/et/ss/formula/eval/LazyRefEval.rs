// java package cn.wps.et.ss.formula.eval;
use super::super::evaluator::{EvalResult, SheetRefEvaluator::SheetRefEvaluator};
use super::{IRefEval::IRefEval, IValueEval, ValueRef};
use std::sync::Arc;
// java public final class LazyRefEval extends RefEvalBase
pub struct LazyRefEval {
    _rowIndex: i32,
    _columnIndex: i32,
    _evaluator: Arc<SheetRefEvaluator>,
}
impl LazyRefEval {
    // java super(rowIndex,columnIndex); this._evaluator=sre;
    pub fn LazyRefEval(row: i32, col: i32, sre: Arc<SheetRefEvaluator>) -> Self {
        Self {
            _rowIndex: row,
            _columnIndex: col,
            _evaluator: sre,
        }
    }
}
impl IRefEval for LazyRefEval {
// java public IAreaEval offset(int relFirstRowIx, int relLastRowIx, int relFirstColIx, int relLastColIx) {
// java IArea area = new IArea.OffsetArea(this.getRow(), this.getColumn(), relFirstRowIx, relLastRowIx, relFirstColIx, relLastColIx);
// java return new LazyAreaEval(area, (byte)0, this._evaluator);
// java }
fn offset(&self,fr:i32,lr:i32,fc:i32,lc:i32)->Arc<dyn super::IAreaEval::IAreaEval>{let r=self._rowIndex;let c=self._columnIndex;Arc::new(super::LazyAreaEval::LazyAreaEval::new(r+fr.min(lr),c+fc.min(lc),r+fr.max(lr),c+fc.max(lc),32,self._evaluator.clone()))}

    fn getInnerValueEval(&self) -> EvalResult<Option<ValueRef>> {
        self._evaluator
            .getEvalForCell(self._rowIndex, self._columnIndex)
    }
    fn getSheetIndex(&self) -> i32 {
        self._evaluator.getSheetIndex()
    }
    fn getRow(&self) -> i32 {
        self._rowIndex
    }
    fn getColumn(&self) -> i32 {
        self._columnIndex
    }
}
impl IValueEval for LazyRefEval {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_ref_eval(&self) -> Option<&dyn IRefEval> {
        Some(self)
    }
}
