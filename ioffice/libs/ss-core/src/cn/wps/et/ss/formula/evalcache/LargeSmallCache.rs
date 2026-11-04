use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::{AreaNumberCollector::AreaNumberCollector,CountifCache::AreaLocate};use super::super::functions::util::{MathX::RAND,numbercollector::{IBlankCollectingPolicy::IBlankCollectingPolicy as BP,IBooleanCollectingPolicy::IBooleanCollectingPolicy as BLP,IStringCollectingPolicy::IStringCollectingPolicy as SP,IErrorCollectingPolicy::IErrorCollectingPolicy as EP}};use crate::cn::wps::moss::util::cache::MRUCache::MRUCache;use std::sync::{Mutex,LazyLock};
static LARGE_CACHE:LazyLock<Mutex<MRUCache<AreaLocate,Arc<Mutex<Cache>>>>>=LazyLock::new(||Mutex::new(MRUCache::new(50)));
pub struct LargeSmallCache;impl LargeSmallCache{
// java public static void clearCache() {
// java large_cache.clear();
// java }
pub fn clearCache() {
LARGE_CACHE.lock().unwrap().clear();
}
// java public static IValueEval largesmall(IAreaEval ae, int k, double dn, boolean isLarge) throws EvaluationException {
// java AreaLocate e = new AreaLocate(ae.getSheetIndex(), ae.getFirstRow(), ae.getLastRow(), ae.getFirstColumn(), ae.getLastColumn());
// java Cache lc;
// java if (ae.getWidth() * ae.getHeight() > 20) {
// java synchronized(large_cache) {
// java lc = large_cache.get(e);
// java if (lc == null) {
// java lc = new Cache();
// java large_cache.put(e, lc);
// java }
// java }
// java } else {
// java lc = new Cache();
// java }
// java
// java synchronized(lc) {
// java if (lc.values == null) {
// java DoubleList dl = new DoubleList();
// java AreaNumberCollector.collectNumbers(ae, IBlankCollectingPolicy.IGNORE, IBooleanCollectingPolicy.IGNORE, IStringCollectingPolicy.IGNORE, IErrorCollectingPolicy.THROW, dl);
// java lc.setValues(dl);
// java }
// java }
// java
// java if (dn > (double)lc.values.size()) {
// java return ErrorEval.NUM_ERROR;
// java } else {
// java double rs;
// java if (isLarge) {
// java lc.nth_value(lc.values.size() - k);
// java rs = lc.values.get(lc.values.size() - k);
// java } else {
// java lc.nth_value(k - 1);
// java rs = lc.values.get(k - 1);
// java }
// java
// java return new NumberEval(rs);
// java }
// java }
pub fn largesmall(a:&dyn IAreaEval,k:i32,dn:f64,large:bool)->FunctionResult<ValueRef> {
let key=AreaLocate{sheet:a.getSheetIndex(),firstRow:a.getFirstRow(),lastRow:a.getLastRow(),firstCol:a.getFirstColumn(),lastCol:a.getLastColumn()};let c=if a.getWidth().wrapping_mul(a.getHeight())>20{let cache=LARGE_CACHE.lock().unwrap();if let Some(c)=cache.get(key.clone()){c}else{let c=Arc::new(Mutex::new(Cache::new()));cache.put(key,Some(c.clone()));c}}else{Arc::new(Mutex::new(Cache::new()))};let mut c=c.lock().unwrap();if c.values.is_none(){let mut dl=DoubleList::new();AreaNumberCollector::collectNumbers(a,BP::IGNORE,BLP::IGNORE,SP::IGNORE,EP::THROW,&mut dl)?;c.setValues(dl);}c.result(k,dn,large)
}
// java public static IValueEval largeSmallCond(IAreaEval ae, int k, double dn, boolean isLarge) throws EvaluationException {
// java Cache lc = new Cache();
// java DoubleList dl = new DoubleList();
// java AreaNumberCollector.collectNumbersCond(ae, IBlankCollectingPolicy.IGNORE, IBooleanCollectingPolicy.IGNORE, IStringCollectingPolicy.IGNORE, IErrorCollectingPolicy.THROW, dl);
// java lc.setValues(dl);
// java if (dn > (double)lc.values.size()) {
// java return ErrorEval.NUM_ERROR;
// java } else {
// java double rs;
// java if (isLarge) {
// java lc.nth_value(lc.values.size() - k);
// java rs = lc.values.get(lc.values.size() - k);
// java } else {
// java lc.nth_value(k - 1);
// java rs = lc.values.get(k - 1);
// java }
// java
// java return new NumberEval(rs);
// java }
// java }
pub fn largeSmallCond(a:&dyn IAreaEval,k:i32,dn:f64,large:bool)->FunctionResult<ValueRef> {
let mut c=Cache::new();let mut dl=DoubleList::new();AreaNumberCollector::collectNumbersCond(a,BP::IGNORE,BLP::IGNORE,SP::IGNORE,EP::THROW,&mut dl)?;c.setValues(dl);c.result(k,dn,large)
}
}
struct Cache{values:Option<DoubleList>,ranked:Vec<bool>}impl Cache{fn new()->Self{Self{values:None,ranked:vec![]}}
// java public void setValues(DoubleList values) {
// java this.values = values;
// java this.ranked = new BitSet(values.size());
// java }
pub fn setValues(&mut self,v:DoubleList) {
self.ranked=vec![false;v.size()as usize];self.values=Some(v);
}
// java public synchronized void nth_value(int n) {
// java if (!this.ranked.get(n)) {
// java int begin;
// java for(begin = n; begin > 0 && !this.ranked.get(begin - 1); --begin) {
// java }
// java
// java int end;
// java for(end = n + 1; end < this.values.size() && !this.ranked.get(end); ++end) {
// java }
// java
// java this.nth_value(begin, end, n - begin);
// java }
// java }
pub fn nth_value(&mut self,n:i32) {
assert!(n>=0);let n=n as usize;if !self.ranked[n]{let mut begin=n;while begin>0&&!self.ranked[begin-1]{begin-=1;}let mut end=n+1;while end<self.ranked.len()&&!self.ranked[end]{end+=1;}self.nth_value_range(begin,end,n-begin);}
}
// java private void nth_value(int begin, int end, int n) {
// java if (begin >= 0 && begin < end && end <= this.values.size() && n >= 0 && n < end - begin) {
// java while(true) {
// java swap(this.values, begin, begin + MathX.RAND.nextInt(end - begin));
// java double guard = this.values.get(begin);
// java int bound = begin + 1;
// java
// java for(int i = begin + 1; i < end; ++i) {
// java if (this.values.get(i) < guard) {
// java swap(this.values, bound++, i);
// java }
// java }
// java
// java swap(this.values, begin, bound - 1);
// java this.ranked.set(bound - 1);
// java if (bound - 1 == begin + n) {
// java return;
// java }
// java
// java if (bound - 1 < begin + n) {
// java n -= bound - begin;
// java begin = bound;
// java } else {
// java end = bound;
// java }
// java }
// java } else {
// java throw new IllegalArgumentException(String.format("begin:%d end:%d n:%d", begin, end, n));
// java }
// java }
pub fn nth_value_range(&mut self,mut begin:usize,mut end:usize,mut n:usize) {
assert!(begin<end&&end<=self.ranked.len()&&n<end-begin);let v=self.values.as_mut().unwrap();loop{let pivot=begin+RAND.lock().unwrap().nextInt((end-begin)as i32)as usize;v.innerArray().swap(begin,pivot);let guard=v.get(begin as i32);let mut bound=begin+1;for i in begin+1..end{if v.get(i as i32)<guard{v.innerArray().swap(bound,i);bound+=1;}}v.innerArray().swap(begin,bound-1);self.ranked[bound-1]=true;if bound-1==begin+n{return;}if bound-1<begin+n{n-=bound-begin;begin=bound;}else{end=bound;}}
}
fn result(&mut self,k:i32,dn:f64,large:bool)->FunctionResult<ValueRef>{let size=self.values.as_ref().unwrap().size();if dn>size as f64{return Ok(Arc::new(ErrorEval::NUM_ERROR));}let n=if large{size-k}else{k-1};self.nth_value(n);Ok(Arc::new(NumberEval::NumberEval(self.values.as_ref().unwrap().get(n))))}}
