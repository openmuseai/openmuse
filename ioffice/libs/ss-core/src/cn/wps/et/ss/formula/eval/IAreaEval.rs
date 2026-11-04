use super::{IValueEval,ValueRef,AreaEvalBase::AreaEvalBase};use super::super::evaluator::EvalResult;
// java public interface IAreaEval extends IValueEval,IArea
pub trait IAreaEval:IValueEval{fn sharedArea(&self)->std::sync::Arc<dyn IAreaEval>;fn offset(&self,fr:i32,lr:i32,fc:i32,lc:i32)->std::sync::Arc<dyn IAreaEval>;
fn getOrgHeight(&self)->i32{self.base().orgLastRow.get()-self.base().firstRow.get()+1}fn getOrgWidth(&self)->i32{self.base().orgLastColumn.get()-self.base().firstColumn.get()+1}
fn valueIterator(&self)->Box<dyn super::super::evaluator::IValueIterator::IValueIterator>;fn base(&self)->&AreaEvalBase;fn getSheetIndex(&self)->i32;fn getRelativeValue(&self,r:i32,c:i32)->EvalResult<ValueRef>;
fn getAbsoluteValue(&self,r:i32,c:i32)->EvalResult<ValueRef>{let b=self.base();let row=r-b.getFirstRow();let col=c-b.getFirstColumn();assert!(row>=0&&row<b.nRows.get()&&col>=0&&col<b.nColumns.get(),"Specified index outside allowed range");self.getRelativeValue(row,col)}
}
impl std::ops::Deref for dyn IAreaEval+'_ {type Target=AreaEvalBase;fn deref(&self)->&AreaEvalBase{self.base()}}
