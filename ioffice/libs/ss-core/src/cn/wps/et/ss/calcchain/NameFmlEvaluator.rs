use std::sync::Arc;use crate::cn::wps::et::ss::{interfaces::IEvalWorkbook::IEvalWorkbook,formula::{ptg::Ptg::Ptg,eval::{ValueRef,BlankEval::BlankEval,MissingArgEval::MissingArgEval,NumberEval::NumberEval,ErrorEval::ErrorEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval,AreaArrayEval::AreaArrayEval}},evalcache::CountifCache::CountifCache,evaluator::{EvalResult,EvaluationContext::EvaluationContext,WorkbookEvaluator::WorkbookEvaluator}}};use super::FormulaHelper::FormulaHelper;use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
pub struct NameFmlEvaluator{workbookEvaluator:Arc<WorkbookEvaluator>,workbook:Arc<dyn IEvalWorkbook>,sv:SpreadsheetVersion}impl NameFmlEvaluator{
// java public NameFmlEvaluator(IEvalWorkbook eb) {
// java this.workbook = eb;
// java this.workbookEvaluator = new WorkbookEvaluator(eb);
// java this.sv = this.workbook.getSpreadsheetVersion();
// java }
pub fn new(workbook:Arc<dyn IEvalWorkbook>)->Self {
let sv=workbook.getSpreadsheetVersion();Self{sv,workbookEvaluator:Arc::new(WorkbookEvaluator::WorkbookEvaluator(workbook.clone())),workbook}
}
// java public IValueEval evaluateConditionFormula(int sheetIndex, int rowIndex, int colIndex, Ptg[] ptgs) {
// java IValueEval var7;
// java try {
// java ptgs = FormulaHelper.replaceNPtgs(ptgs, rowIndex, colIndex, this.sv);
// java EvaluationContext ec = new EvaluationContext(this.workbookEvaluator, this.workbook, sheetIndex, rowIndex, colIndex, false);
// java ec.setArrayFormula(true);
// java ec.setCondFormula(true);
// java IValueEval ret = this.workbookEvaluator.evaluateFormula(ec, ptgs);
// java var7 = this.pickOneResult(ret);
// java } finally {
// java CountifCache.clearCache();
// java }
// java
// java return var7;
// java }
pub fn evaluateConditionFormula(&self,sheet:i32,row:i32,col:i32,ptgs:&[Ptg])->EvalResult<Option<ValueRef>> {
let _finally=ClearCountifFinally;let ptgs=FormulaHelper::replaceNPtgs(ptgs,row,col,self.sv);let mut ec=EvaluationContext::EvaluationContext(self.workbookEvaluator.clone(),self.workbook.clone(),sheet,row,col,false);ec.setArrayFormula(true);ec.setCondFormula(true);let ret=self.workbookEvaluator.evaluateFormula(&mut ec,&ptgs).and_then(|v|v.map(|v|self.pickOneResult(v)).transpose());ret
}
// java private IValueEval pickOneResult(IValueEval ve) {
// java if (ve instanceof IRefEval) {
// java ve = ((IRefEval)ve).getInnerValueEval();
// java } else if (ve instanceof IAreaEval) {
// java ve = ((IAreaEval)ve).getRelativeValue(0, 0);
// java } else if (ve instanceof ArrayEval) {
// java ve = ((ArrayEval)ve).getFirstValue();
// java if (ve instanceof IRefEval) {
// java ve = ((IRefEval)ve).getInnerValueEval();
// java }
// java } else if (ve instanceof AreaArrayEval) {
// java ve = ErrorEval.VALUE_INVALID;
// java }
// java
// java if (ve != BlankEval.instance && !(ve instanceof MissingArgEval)) {
// java if (ve instanceof UnionEval) {
// java ve = ErrorEval.VALUE_INVALID;
// java }
// java } else {
// java ve = NumberEval.ZERO;
// java }
// java
// java return ve;
// java }
pub fn pickOneResult(&self,mut v:ValueRef)->EvalResult<ValueRef> {
if let Some(r)=v.as_ref_eval(){v=r.getInnerValueEval()?.ok_or(crate::cn::wps::et::ss::formula::evaluator::EvalFailure::Runtime("null reference"))?;}else if let Some(a)=v.as_area_eval(){v=a.getRelativeValue(0,0)?;}else if let Some(a)=v.as_any().downcast_ref::<ArrayEval>(){v=a.getFirstValue();if let Some(r)=v.as_ref_eval(){v=r.getInnerValueEval()?.ok_or(crate::cn::wps::et::ss::formula::evaluator::EvalFailure::Runtime("null reference"))?;}}else if v.as_any().is::<AreaArrayEval>(){v=Arc::new(ErrorEval::VALUE_INVALID);}if v.as_any().is::<BlankEval>()||v.as_any().is::<MissingArgEval>(){v=Arc::new(NumberEval::ZERO);}else if v.as_any().is::<UnionEval>(){v=Arc::new(ErrorEval::VALUE_INVALID);}Ok(v)
}
}

struct ClearCountifFinally;impl Drop for ClearCountifFinally{fn drop(&mut self){CountifCache::clearCache();}}
