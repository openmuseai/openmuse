use super::super::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,ErrorEval::ErrorEval,NumberEval::NumberEval,StringEval::StringEval};use crate::java_runtime::Cell;use std::sync::Arc;use crate::cn::wps::et::ss::formula::{ptg::Ptg::{Ptg,PtgKind},constant::ConstantValueParser::ConstantValue};
// java public final class ArrayEval implements IValueEval,Iterable<IValueEval>
pub struct ArrayEval{rowCount:i32,colCount:i32,values:Arc<Vec<ValueRef>>,orgRowCount:Cell<i32>,orgColCount:Cell<i32>}
impl ArrayEval{
// java public ArrayEval(ArrayPtg ptg) {
// java this.rowCount = ptg.getRowCount();
// java this.colCount = ptg.getColumnCount();
// java Object[] consts = ptg.getPlainTokenArrayValues();
// java int len = consts.length;
// java this.values = new IValueEval[len];
// java
// java for(int i = 0; i < len; ++i) {
// java this.values[i] = wrapconst(consts[i]);
// java }
// java
// java }
pub fn fromPtg(ptg:&Ptg)->Self {
let PtgKind::Array{columns,rows,values,..}=&ptg.kind else{panic!("ClassCastException ArrayPtg");};Self{rowCount:*rows as i32,colCount:*columns as i32,values:Arc::new(values.as_ref().expect("NullPointerException").iter().map(Self::wrapconst).collect()),orgRowCount:Cell::new(0),orgColCount:Cell::new(0)}
}
// java public ArrayEval(int rowCount, int colCount, IValueEval[] values) {
// java this.rowCount = this.orgRowCount = rowCount;
// java this.colCount = this.orgColCount = colCount;
// java this.values = values;
// java }
pub fn new(rows:i32,cols:i32,values:Arc<Vec<ValueRef>>)->Self {
Self{rowCount:rows,colCount:cols,values,orgRowCount:Cell::new(rows),orgColCount:Cell::new(cols)}
}
// java public ArrayEval(int rowCount, int colCount, int orgRowCount, int orgColCount, IValueEval[] values) {
// java this.rowCount = rowCount;
// java this.colCount = colCount;
// java this.orgRowCount = orgRowCount;
// java this.orgColCount = orgColCount;
// java this.values = values;
// java }
pub fn withOriginal(rows:i32,cols:i32,orgRows:i32,orgCols:i32,values:Arc<Vec<ValueRef>>)->Self {
Self{rowCount:rows,colCount:cols,values,orgRowCount:Cell::new(orgRows),orgColCount:Cell::new(orgCols)}
}
// java private static IValueEval wrapconst(Object v) {
// java if (v == ConstantValueParser.EMPTY_REPRESENTATION) {
// java return BlankEval.instance;
// java } else if (v instanceof Integer) {
// java return new NumberEval((double)(Integer)v);
// java } else if (v instanceof Double) {
// java return new NumberEval((Double)v);
// java } else if (v instanceof String) {
// java return new StringEval((String)v);
// java } else if (v instanceof Boolean) {
// java return BoolEval.valueOf((Boolean)v);
// java } else {
// java return v instanceof ErrorConstant ? ErrorEval.valueOf(((ErrorConstant)v).getErrorCode()) : ErrorEval.VALUE_INVALID;
// java }
// java }
pub fn wrapconst(v:&ConstantValue)->ValueRef {
match v{ConstantValue::Empty=>Arc::new(BlankEval::instance),ConstantValue::Number(v)=>Arc::new(NumberEval::NumberEval(*v)),ConstantValue::String(s)=>Arc::new(StringEval::StringEval(s)),ConstantValue::StringUtf16(v)=>Arc::new(StringEval::from_utf16(v.clone())),ConstantValue::Boolean(v)=>Arc::new(BoolEval::valueOf(*v)),ConstantValue::Error(v)=>Arc::new(ErrorEval::valueOf(*v as i32))}
}
// java public int getRowCount() {
// java return this.rowCount;
// java }
pub fn getRowCount(&self)->i32 {
self.rowCount
}
// java public int getColCount() {
// java return this.colCount;
// java }
pub fn getColCount(&self)->i32 {
self.colCount
}
// java public int getOrgRowCount() {
// java return this.orgRowCount;
// java }
pub fn getOrgRowCount(&self)->i32 {
self.orgRowCount.get()
}
// java public int getOrgColCount() {
// java return this.orgColCount;
// java }
pub fn getOrgColCount(&self)->i32 {
self.orgColCount.get()
}
// java public void setOrgRowCount(int orgRowCount) {
// java this.orgRowCount = orgRowCount;
// java }
pub fn setOrgRowCount(&self,v:i32) {
self.orgRowCount.set(v);
}
// java public void setOrgColCount(int orgColCount) {
// java this.orgColCount = orgColCount;
// java }
pub fn setOrgColCount(&self,v:i32) {
self.orgColCount.set(v);
}
// java public IValueEval getFirstValue() {
// java return (IValueEval)(this.values.length == 0 ? ErrorEval.VALUE_INVALID : this.values[0]);
// java }
pub fn getFirstValue(&self)->ValueRef {
self.values.first().cloned().unwrap_or_else(||Arc::new(ErrorEval::VALUE_INVALID))
}
// java public IValueEval getValue(int row, int col) {
// java if (row >= 0 && row < this.rowCount && col >= 0 && col < this.colCount) {
// java return this.values[row * this.colCount + col];
// java } else {
// java throw new IndexOutOfBoundsException();
// java }
// java }
pub fn getValue(&self,r:i32,c:i32)->ValueRef {
assert!(r>=0&&r<self.rowCount&&c>=0&&c<self.colCount,"IndexOutOfBoundsException");self.values[(r*self.colCount+c)as usize].clone()
}
// java public ArrayEval getRow(int row) {
// java IValueEval[] rs = new IValueEval[this.colCount];
// java
// java for(int i = 0; i < this.colCount; ++i) {
// java rs[i] = this.values[row * this.colCount + i];
// java }
// java
// java return new ArrayEval(1, this.colCount, 1, this.orgColCount, rs);
// java }
pub fn getRow(&self,r:i32)->Self {
Self::withOriginal(1,self.colCount,1,self.orgColCount.get(),Arc::new((0..self.colCount).map(|c|self.getValue(r,c)).collect()))
}
// java public ArrayEval getCol(int col) {
// java IValueEval[] rs = new IValueEval[this.rowCount];
// java
// java for(int i = 0; i < this.rowCount; ++i) {
// java rs[i] = this.values[i * this.colCount + col];
// java }
// java
// java return new ArrayEval(this.rowCount, 1, this.orgRowCount, 1, rs);
// java }
pub fn getCol(&self,c:i32)->Self {
Self::withOriginal(self.rowCount,1,self.orgRowCount.get(),1,Arc::new((0..self.rowCount).map(|r|self.getValue(r,c)).collect()))
}
// java public int totalValues() {
// java return this.values.length;
// java }
pub fn totalValues(&self)->i32 {
self.values.len() as i32
}
}
impl IValueEval for ArrayEval{fn as_any(&self)->&dyn std::any::Any{self}}
