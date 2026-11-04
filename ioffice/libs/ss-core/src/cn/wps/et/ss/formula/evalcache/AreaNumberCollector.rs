use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::super::functions::util::numbercollector::{IBlankCollectingPolicy::IBlankCollectingPolicy as BP,IBooleanCollectingPolicy::IBooleanCollectingPolicy as BLP,IStringCollectingPolicy::IStringCollectingPolicy as SP,IErrorCollectingPolicy::IErrorCollectingPolicy as EP};
use crate::cn::wps::moss::util::cache::MRUCache::MRUCache;use std::sync::{Mutex,LazyLock};
#[derive(Clone,PartialEq,Eq,Hash)]struct AreaKey{sheet:i32,fr:i32,lr:i32,fc:i32,lc:i32,bp:BP,blp:BLP,sp:SP,ep:EP}
struct Cache{area:Arc<dyn IAreaEval>,bp:BP,blp:BLP,sp:SP,ep:EP,values:DoubleList,lastReadedRow:i32,lastReadedCol:i32}
static AREA_NUMBER_CACHE:LazyLock<Mutex<MRUCache<AreaKey,Arc<Mutex<Cache>>>>>=LazyLock::new(||Mutex::new(MRUCache::new(100)));
pub struct AreaNumberCollector;impl AreaNumberCollector{
// java public static void clearCache() {
// java area_number_cache.clear();
// java }
pub fn clearCache() {
AREA_NUMBER_CACHE.lock().unwrap().clear();
}
// java public static void collectNumbers(IAreaEval ae, IBlankCollectingPolicy bp, IBooleanCollectingPolicy blp, IStringCollectingPolicy sp, IErrorCollectingPolicy ep, DoubleList out) throws EvaluationException {
// java AreaKey k = new AreaKey(ae.getSheetIndex(), ae.getFirstRow(), ae.getLastRow(), ae.getFirstColumn(), ae.getLastColumn(), bp, blp, sp, ep);
// java Cache c;
// java if (ae.getWidth() * ae.getHeight() > 20) {
// java synchronized(area_number_cache) {
// java c = area_number_cache.get(k);
// java if (c == null) {
// java c = new Cache(ae, bp, blp, sp, ep);
// java area_number_cache.put(k, c);
// java }
// java }
// java } else {
// java c = new Cache(ae, bp, blp, sp, ep);
// java }
// java
// java c.collectNumbers();
// java out.addAll(c.values);
// java }
pub fn collectNumbers(ae:&dyn IAreaEval,bp:BP,blp:BLP,sp:SP,ep:EP,out:&mut DoubleList)->FunctionResult<()> {
let k=AreaKey{sheet:ae.getSheetIndex(),fr:ae.getFirstRow(),lr:ae.getLastRow(),fc:ae.getFirstColumn(),lc:ae.getLastColumn(),bp,blp,sp,ep};let c=if ae.getWidth().wrapping_mul(ae.getHeight())>20{let cache=AREA_NUMBER_CACHE.lock().unwrap();if let Some(c)=cache.get(k.clone()){c}else{let c=Arc::new(Mutex::new(Cache::new(ae,bp,blp,sp,ep)));cache.put(k,Some(c.clone()));c}}else{Arc::new(Mutex::new(Cache::new(ae,bp,blp,sp,ep)))};let mut c=c.lock().unwrap();c.collectNumbers()?;out.addAll(&c.values);Ok(())
}
// java public static void collectNumbersCond(IAreaEval ae, IBlankCollectingPolicy bp, IBooleanCollectingPolicy blp, IStringCollectingPolicy sp, IErrorCollectingPolicy ep, DoubleList out) throws EvaluationException {
// java Cache c = new Cache(ae, bp, blp, sp, ep);
// java c.collectNumbers();
// java out.addAll(c.values);
// java }
pub fn collectNumbersCond(ae:&dyn IAreaEval,bp:BP,blp:BLP,sp:SP,ep:EP,out:&mut DoubleList)->FunctionResult<()> {
let mut c=Cache::new(ae,bp,blp,sp,ep);c.collectNumbers()?;out.addAll(&c.values);Ok(())
}
}
impl Cache{// java public Cache(IAreaEval ae, IBlankCollectingPolicy bp, IBooleanCollectingPolicy blp, IStringCollectingPolicy sp, IErrorCollectingPolicy ep) {
// java this.area = ae;
// java this.blankPolicy = bp;
// java this.boolPolicy = blp;
// java this.stringPolicy = sp;
// java this.errorPolicy = ep;
// java }
pub fn new(ae:&dyn IAreaEval,bp:BP,blp:BLP,sp:SP,ep:EP)->Self {
Self{area:ae.sharedArea(),bp,blp,sp,ep,values:DoubleList::new(),lastReadedRow:-1,lastReadedCol:-1}
}
fn collect(&mut self,v:&ValueRef,blank:bool)->FunctionResult<()>{if let Some(n)=v.as_any().downcast_ref::<NumberEval>(){self.values.add(n.getNumberValue());}else if let Some(b)=v.as_any().downcast_ref::<BoolEval>(){self.blp.collect(b,&mut self.values)?;}else if let Some(e)=v.as_any().downcast_ref::<ErrorEval>(){self.ep.collect(e,&mut self.values)?;}else if let Some(s)=v.as_any().downcast_ref::<StringEval>(){self.sp.collect(s,&mut self.values)?;}else if blank&&v.as_any().is::<BlankEval>(){self.bp.collect(BlankEval::instance,&mut self.values)?;}Ok(())}
// java private void collectNumbersIgnoreBlank(IValueIterator iter) throws EvaluationException {
// java if (iter != null) {
// java int c;
// java for(; iter.hasNext(); this.lastReadedCol = c) {
// java iter.next();
// java int r = iter.getRowIndex();
// java c = iter.getColIndex();
// java this.lastReadedRow = r;
// java this.lastReadedCol = c - 1;
// java IValueEval ve = iter.getValue();
// java if (ve instanceof NumberEval) {
// java double v = ((NumberEval)ve).getNumberValue();
// java this.values.add(v);
// java } else if (ve instanceof BoolEval) {
// java this.boolPolicy.collect((BoolEval)ve, this.values);
// java } else if (ve instanceof ErrorEval) {
// java this.errorPolicy.collect((ErrorEval)ve, this.values);
// java } else if (ve instanceof StringEval) {
// java this.stringPolicy.collect((StringEval)ve, this.values);
// java }
// java }
// java
// java }
// java }
pub fn collectNumbersIgnoreBlank(&mut self,mut iter:Box<dyn super::super::evaluator::IValueIterator::IValueIterator>)->FunctionResult<()> {
while iter.hasNext(){iter.next();let r=iter.getRowIndex();let c=iter.getColIndex();self.lastReadedRow=r;self.lastReadedCol=c-1;let v=iter.getValue()?;self.collect(&v,false)?;self.lastReadedCol=c;}Ok(())
}
// java public synchronized void collectNumbers() throws EvaluationException {
// java if (this.lastReadedRow <= this.area.getLastRow() && (this.lastReadedRow != this.area.getLastRow() || this.lastReadedCol < this.area.getLastColumn())) {
// java if (this.blankPolicy == IBlankCollectingPolicy.IGNORE) {
// java IValueIterator iter1 = null;
// java IValueIterator iter2 = null;
// java if (this.lastReadedRow < this.area.getFirstRow()) {
// java iter1 = this.area.valueIterator();
// java } else if (this.lastReadedCol < this.area.getFirstColumn()) {
// java iter1 = this.area.offset(this.lastReadedRow - this.area.getFirstRow(), this.area.getLastRow() - this.area.getFirstRow(), 0, this.area.getLastColumn() - this.area.getFirstColumn()).valueIterator();
// java } else if (this.lastReadedCol >= this.area.getLastColumn()) {
// java iter1 = this.area.offset(this.lastReadedRow + 1 - this.area.getFirstRow(), this.area.getLastRow() - this.area.getFirstRow(), 0, this.area.getLastColumn() - this.area.getFirstColumn()).valueIterator();
// java } else if (this.lastReadedRow == this.area.getLastRow()) {
// java iter1 = this.area.offset(this.lastReadedRow - this.area.getFirstRow(), this.lastReadedRow - this.area.getFirstRow(), this.lastReadedCol + 1 - this.area.getFirstColumn(), this.area.getLastColumn() - this.area.getFirstColumn()).valueIterator();
// java } else {
// java iter1 = this.area.offset(this.lastReadedRow - this.area.getFirstRow(), this.lastReadedRow - this.area.getFirstRow(), this.lastReadedCol + 1 - this.area.getFirstColumn(), this.area.getLastColumn() - this.area.getFirstColumn()).valueIterator();
// java iter2 = this.area.offset(this.lastReadedRow + 1 - this.area.getFirstRow(), this.area.getLastRow() - this.area.getFirstRow(), 0, this.area.getLastColumn() - this.area.getFirstColumn()).valueIterator();
// java }
// java
// java this.collectNumbersIgnoreBlank(iter1);
// java this.collectNumbersIgnoreBlank(iter2);
// java } else {
// java int firstRow = this.area.getFirstRow();
// java int lastRow = this.area.getLastRow();
// java int firstCol = this.area.getFirstColumn();
// java int lastCol = this.area.getLastColumn();
// java int r = this.lastReadedRow;
// java int c = this.lastReadedCol + 1;
// java
// java while(r <= lastRow) {
// java if (c < firstCol) {
// java c = firstCol;
// java } else if (c > lastCol) {
// java ++r;
// java c = firstCol;
// java continue;
// java }
// java
// java if (r < firstRow) {
// java r = firstRow;
// java } else {
// java this.lastReadedRow = r;
// java this.lastReadedCol = c - 1;
// java IValueEval ve = this.area.getAbsoluteValue(r, c);
// java if (ve instanceof NumberEval) {
// java double v = ((NumberEval)ve).getNumberValue();
// java this.values.add(v);
// java } else if (ve instanceof BlankEval) {
// java this.blankPolicy.collect((BlankEval)ve, this.values);
// java } else if (ve instanceof BoolEval) {
// java this.boolPolicy.collect((BoolEval)ve, this.values);
// java } else if (ve instanceof StringEval) {
// java this.stringPolicy.collect((StringEval)ve, this.values);
// java } else if (ve instanceof ErrorEval) {
// java this.errorPolicy.collect((ErrorEval)ve, this.values);
// java }
// java
// java this.lastReadedCol = c++;
// java }
// java }
// java
// java }
// java }
// java }
pub fn collectNumbers(&mut self)->FunctionResult<()> {
let a=self.area.clone();if self.lastReadedRow<=a.getLastRow()&&(self.lastReadedRow!=a.getLastRow()||self.lastReadedCol<a.getLastColumn()){if self.bp==BP::IGNORE{let(mut i1,mut i2)=(None,None);let(fr,lr,fc,lc)=(a.getFirstRow(),a.getLastRow(),a.getFirstColumn(),a.getLastColumn());if self.lastReadedRow<fr{i1=Some(a.valueIterator());}else if self.lastReadedCol<fc{i1=Some(a.offset(self.lastReadedRow-fr,lr-fr,0,lc-fc).valueIterator());}else if self.lastReadedCol>=lc{i1=Some(a.offset(self.lastReadedRow+1-fr,lr-fr,0,lc-fc).valueIterator());}else if self.lastReadedRow==lr{i1=Some(a.offset(self.lastReadedRow-fr,self.lastReadedRow-fr,self.lastReadedCol+1-fc,lc-fc).valueIterator());}else{i1=Some(a.offset(self.lastReadedRow-fr,self.lastReadedRow-fr,self.lastReadedCol+1-fc,lc-fc).valueIterator());i2=Some(a.offset(self.lastReadedRow+1-fr,lr-fr,0,lc-fc).valueIterator());}if let Some(i)=i1{self.collectNumbersIgnoreBlank(i)?;}if let Some(i)=i2{self.collectNumbersIgnoreBlank(i)?;}}else{let(fr,lr,fc,lc)=(a.getFirstRow(),a.getLastRow(),a.getFirstColumn(),a.getLastColumn());let(mut r,mut c)=(self.lastReadedRow,self.lastReadedCol+1);while r<=lr{if c<fc{c=fc;}else if c>lc{r+=1;c=fc;continue;}if r<fr{r=fr;}else{self.lastReadedRow=r;self.lastReadedCol=c-1;let v=a.getAbsoluteValue(r,c)?;self.collect(&v,true)?;self.lastReadedCol=c;c+=1;}}}}Ok(())
}
}
