use super::{AreaEvalBase::AreaEvalBase,IAreaEval::IAreaEval,IValueEval,ValueRef};use super::super::evaluator::{EvalResult,EvalFailure,SheetRefEvaluator::SheetRefEvaluator};use std::sync::Arc;
// java public final class LazyAreaEval extends AreaEvalBase
#[derive(Clone)]pub struct LazyAreaEval{base:Arc<AreaEvalBase>,evaluator:Arc<SheetRefEvaluator>,maxRow:i32,maxCol:i32}
impl LazyAreaEval{
// java public LazyAreaEval(int firstRowIndex, int firstColumnIndex, int lastRowIndex, int lastColumnIndex, byte rav, SheetRefEvaluator evaluator) {
// java super(firstRowIndex, firstColumnIndex, lastRowIndex, lastColumnIndex, rav);
// java this.initLazyAreaEval(evaluator);
// java }
pub fn new(fr:i32,fc:i32,lr:i32,lc:i32,rav:i8,evaluator:Arc<SheetRefEvaluator>)->Self {
let version=evaluator.getBook().getSpreadsheetVersion();Self{base:Arc::new(AreaEvalBase::new(fr,fc,lr,lc,rav)),evaluator,maxRow:version.getMaxRows(),maxCol:version.getMaxColumns()}
}
// java public int getOrgHeight() {
// java return this._orgLastRow - this._firstRow + 1;
// java }
pub fn getOrgHeight(&self)->i32 {
self.base.orgLastRow.get()-self.base.firstRow.get()+1
}
// java public int getOrgWidth() {
// java return this._orgLastColumn - this._firstColumn + 1;
// java }
pub fn getOrgWidth(&self)->i32 {
self.base.orgLastColumn.get()-self.base.firstColumn.get()+1
}
// java public void setOrgLastRow(int orgLastRow) {
// java this._orgLastRow = orgLastRow < 0 ? this._lastRow : orgLastRow;
// java }
pub fn setOrgLastRow(&self,v:i32) {
self.base.orgLastRow.set(if v<0{self.base.lastRow.get()}else{v});
}
// java public void setOrgLastCol(int orgLastCol) {
// java this._orgLastColumn = orgLastCol < 0 ? this._lastColumn : orgLastCol;
// java }
pub fn setOrgLastCol(&self,v:i32) {
self.base.orgLastColumn.set(if v<0{self.base.lastColumn.get()}else{v});
}
// java public int getOrgLastRow() {
// java return this._orgLastRow;
// java }
pub fn getOrgLastRow(&self)->i32 {
self.base.orgLastRow.get()
}
// java public int getOrgLastCol() {
// java return this._orgLastColumn;
// java }
pub fn getOrgLastCol(&self)->i32 {
self.base.orgLastColumn.get()
}
// java public SheetRefEvaluator sheetRefEvaluator() {
// java return this._evaluator;
// java }
pub fn sheetRefEvaluator(&self)->Arc<SheetRefEvaluator> {
self.evaluator.clone()
}
}
impl std::ops::Deref for LazyAreaEval{type Target=AreaEvalBase;fn deref(&self)->&AreaEvalBase{&self.base}}
impl IAreaEval for LazyAreaEval{fn sharedArea(&self)->Arc<dyn IAreaEval>{Arc::new(self.clone())}
// java public IAreaEval offset(int relFirstRowIx, int relLastRowIx, int relFirstColIx, int relLastColIx) {
// java IArea area = new IArea.OffsetArea(this.getFirstRow(), this.getFirstColumn(), relFirstRowIx, relLastRowIx, relFirstColIx, relLastColIx);
// java byte rav = this.getRAV();
// java if (area.getFirstRow() < area.getLastRow() || area.getFirstColumn() < area.getLastColumn()) {
// java rav = 64;
// java }
// java
// java return new LazyAreaEval(area, rav, this._evaluator);
// java }
fn offset(&self,fr:i32,lr:i32,fc:i32,lc:i32)->Arc<dyn IAreaEval>{let r=self.getFirstRow();let c=self.getFirstColumn();let(fr,lr,fc,lc)=(r+fr.min(lr),r+fr.max(lr),c+fc.min(lc),c+fc.max(lc));let rav=if fr<lr||fc<lc{64}else{self.getRAV()};Arc::new(Self::new(fr,fc,lr,lc,rav,self.evaluator.clone()))}
fn valueIterator(&self)->Box<dyn super::super::evaluator::IValueIterator::IValueIterator>{self.evaluator.getValueIterator(self.base.getFirstRow(),self.base.getLastRow(),self.base.getFirstColumn(),self.base.getLastColumn())}fn base(&self)->&AreaEvalBase{&self.base}fn getSheetIndex(&self)->i32{self.evaluator.getSheetIndex()}
// java public IValueEval getRelativeValue(int relativeRowIndex, int relativeColumnIndex) {
// java int rowIx = relativeRowIndex + this.getFirstRow() & this.maxRow - 1;
// java int colIx = relativeColumnIndex + this.getFirstColumn() & this.maxCol - 1;
// java return this._evaluator.getEvalForCell(rowIx, colIx);
// java }
fn getRelativeValue(&self,r:i32,c:i32)->EvalResult<ValueRef>{let r=(r+self.base.getFirstRow())&(self.maxRow-1);let c=(c+self.base.getFirstColumn())&(self.maxCol-1);self.evaluator.getEvalForCell(r,c)?.ok_or(EvalFailure::Runtime("NullPointerException"))}}
impl IValueEval for LazyAreaEval{fn as_any(&self)->&dyn std::any::Any{self}fn as_area_eval(&self)->Option<&dyn IAreaEval>{Some(self)}}
