use super::super::{IValueEval,ValueRef,ErrorEval::ErrorEval};use std::sync::Arc;
pub struct AreaArrayEval{rows:i32,cols:i32,areas:Arc<Vec<ValueRef>>}impl AreaArrayEval{
// java public AreaArrayEval(int rowCount, int colCount, IValueEval[] areas) {
// java fixValueType(areas);
// java this.rowCount = rowCount;
// java this.colCount = colCount;
// java this.areas = areas;
// java }
pub fn new(rows:i32,cols:i32,areas:Arc<Vec<ValueRef>>)->Self {
let areas=Arc::new(areas.iter().map(|a|if a.as_area_eval().is_some()||a.as_any().is::<ErrorEval>(){a.clone()}else{Arc::new(ErrorEval::VALUE_INVALID) as ValueRef}).collect());Self{rows,cols,areas}
}
// java public int getRowCount() {
// java return this.rowCount;
// java }
pub fn getRowCount(&self)->i32 {
self.rows
}
// java public int getColCount() {
// java return this.colCount;
// java }
pub fn getColCount(&self)->i32 {
self.cols
}
// java public IAreaEval getArea(int row, int col) throws EvaluationException {
// java IValueEval ret = this.areas[row * this.colCount + col];
// java if (ret instanceof ErrorEval) {
// java throw EvaluationException.valueOf((ErrorEval)ret);
// java } else {
// java return (IAreaEval)ret;
// java }
// java }
pub fn getArea(&self,r:i32,c:i32)->Result<ValueRef,&'static ErrorEval> {
let v=self.areas[(r*self.cols+c)as usize].clone();if let Some(e)=v.as_any().downcast_ref::<ErrorEval>(){Err(ErrorEval::valueOf(e.getErrorCode()))}else{Ok(v)}
}
}
impl IValueEval for AreaArrayEval{fn as_any(&self)->&dyn std::any::Any{self}}
