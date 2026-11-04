use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::{CountifCache::AreaLocate,AreaNumberCollector::AreaNumberCollector};use super::super::functions::util::{MathX::RAND,numbercollector::{IBlankCollectingPolicy::IBlankCollectingPolicy as BP,IBooleanCollectingPolicy::IBooleanCollectingPolicy as BLP,IStringCollectingPolicy::IStringCollectingPolicy as SP,IErrorCollectingPolicy::IErrorCollectingPolicy as EP}};use crate::cn::wps::moss::util::cache::MRUCache::MRUCache;use std::sync::{Mutex,LazyLock};
static RANK_CACHE:LazyLock<Mutex<MRUCache<AreaLocate,Arc<Mutex<Cache>>>>>=LazyLock::new(||Mutex::new(MRUCache::new(50)));static HITS:Mutex<i32>=Mutex::new(0);
// rust: Stable node indices are native references to the original RankCache.Node skip-list objects.
struct Node{index:i32,count:i32,next:Vec<Option<usize>>}
struct Cache{values:Option<DoubleList>,head:Vec<Option<usize>>,nodes:Vec<Node>}
pub struct CountifRankCache;impl CountifRankCache{
// java public static void clearCache() {
// java rank_cache.clear();
// java hits = 0;
// java }
pub fn clearCache() {
RANK_CACHE.lock().unwrap().clear();*HITS.lock().unwrap()=0;
}
// java public static int rank(IAreaEval ae, double num, int op) {
// java ++hits;
// java AreaLocate k = new AreaLocate(ae.getSheetIndex(), ae.getFirstRow(), ae.getLastRow(), ae.getFirstColumn(), ae.getLastColumn());
// java Cache c;
// java if (ae.getWidth() * ae.getHeight() > 20) {
// java synchronized(rank_cache) {
// java c = rank_cache.get(k);
// java if (c == null) {
// java c = new Cache();
// java rank_cache.put(k, c);
// java }
// java }
// java } else {
// java c = new Cache();
// java }
// java
// java synchronized(c) {
// java c.collectValues(ae);
// java switch (op) {
// java case 3:
// java return c.values.size() - c.rank(num, false);
// java case 4:
// java return c.rank(num, true);
// java case 5:
// java return c.rank(num, false);
// java case 6:
// java return c.values.size() - c.rank(num, true);
// java default:
// java throw new IllegalStateException();
// java }
// java }
// java }
pub fn rank(a:&dyn IAreaEval,num:f64,op:i32)->EvalResult<i32> {
*HITS.lock().unwrap()+=1;let k=AreaLocate{sheet:a.getSheetIndex(),firstRow:a.getFirstRow(),lastRow:a.getLastRow(),firstCol:a.getFirstColumn(),lastCol:a.getLastColumn()};let c=if a.getWidth().wrapping_mul(a.getHeight())>20{let cache=RANK_CACHE.lock().unwrap();if let Some(c)=cache.get(k.clone()){c}else{let c=Arc::new(Mutex::new(Cache::new()));cache.put(k,Some(c.clone()));c}}else{Arc::new(Mutex::new(Cache::new()))};let mut c=c.lock().unwrap();c.collectValues(a)?;Ok(match op{3=>c.values.as_ref().unwrap().size()-c.rank(num,false),4=>c.rank(num,true),5=>c.rank(num,false),6=>c.values.as_ref().unwrap().size()-c.rank(num,true),_=>return Err(EvalFailure::Runtime("IllegalStateException"))})
}
}
impl Cache{fn new()->Self{Self{values:None,head:vec![None],nodes:vec![]}}// java public void collectValues(IAreaEval ae) {
// java if (this.values == null) {
// java DoubleList dl = new DoubleList();
// java
// java try {
// java AreaNumberCollector.collectNumbers(ae, IBlankCollectingPolicy.IGNORE, IBooleanCollectingPolicy.IGNORE, IStringCollectingPolicy.IGNORE, IErrorCollectingPolicy.IGNORE, dl);
// java } catch (EvaluationException e) {
// java throw new RuntimeException(e);
// java }
// java
// java this.values = dl;
// java }
// java }
pub fn collectValues(&mut self,a:&dyn IAreaEval)->EvalResult<()> {
if self.values.is_none(){let mut dl=DoubleList::new();match AreaNumberCollector::collectNumbers(a,BP::IGNORE,BLP::IGNORE,SP::IGNORE,EP::IGNORE,&mut dl){Ok(())=>{},Err(FunctionFailure::Runtime(e))=>return Err(e),Err(FunctionFailure::Checked(_))=>return Err(EvalFailure::Runtime("EvaluationException"))}self.values=Some(dl);}Ok(())
}
// java private static int randomLevel() {
// java int k;
// java for(k = 0; k < 16 && MathX.RAND.nextBoolean(); ++k) {
// java }
// java
// java return k;
// java }
pub fn randomLevel()->usize {
let mut k=0;while k<16&&RAND.lock().unwrap().nextBoolean(){k+=1;}k
}
// java private void insertNode(RankCache.Node n, RankCache.Node[] pre_lv) {
// java int level = randomLevel();
// java if (level + 1 > this.head.length) {
// java RankCache.Node[] nh = new RankCache.Node[level + 1];
// java System.arraycopy(this.head, 0, nh, 0, this.head.length);
// java Arrays.fill(nh, this.head.length, level + 1, n);
// java this.head = nh;
// java }
// java
// java n.next = new RankCache.Node[level + 1];
// java int i = 0;
// java
// java for(int len = pre_lv.length; i < len && i <= level; ++i) {
// java RankCache.Node t;
// java if (pre_lv[i] == null) {
// java t = this.head[i];
// java this.head[i] = n;
// java } else {
// java t = pre_lv[i].next[i];
// java pre_lv[i].next[i] = n;
// java }
// java
// java n.next[i] = t;
// java }
// java
// java }
pub fn insertNode(&mut self,n:usize,pre:&[Option<usize>]) {
let level=Self::randomLevel();if level+1>self.head.len(){self.head.resize(level+1,Some(n));}self.nodes[n].next=vec![None;level+1];for i in 0..pre.len().min(level+1){let t=if let Some(p)=pre[i]{let t=self.nodes[p].next[i];self.nodes[p].next[i]=Some(n);t}else{let t=self.head[i];self.head[i]=Some(n);t};self.nodes[n].next[i]=t;}
}
// java private int rankOfNode(RankCache.Node n, boolean ascending) {
// java return ascending ? n.index : this.values.size() - n.index - n.count;
// java }
pub fn rankOfNode(&self,n:usize,ascending:bool)->i32 {
let n=&self.nodes[n];if ascending{n.index}else{self.values.as_ref().unwrap().size()-n.index-n.count}
}
// java private RankCache.Node rank_range(double x, int begin, int end) {
// java int less_count = 0;
// java int eq_count = 0;
// java
// java for(int i = begin; i < end; ++i) {
// java double v = this.values.get(i);
// java int first_eq = begin + less_count;
// java int first_greater = first_eq + eq_count;
// java if (v < x) {
// java this.values.set(i, this.values.get(first_greater));
// java this.values.set(first_greater, this.values.get(first_eq));
// java this.values.set(first_eq, v);
// java ++less_count;
// java } else if (v == x) {
// java this.values.set(i, this.values.get(first_greater));
// java this.values.set(first_greater, v);
// java ++eq_count;
// java }
// java }
// java
// java if (eq_count == 0) {
// java return null;
// java } else {
// java RankCache.Node n = new RankCache.Node();
// java n.index = begin + less_count;
// java n.count = eq_count;
// java return n;
// java }
// java }
pub fn rank_range(&mut self,x:f64,begin:i32,end:i32)->Option<usize> {
let values=self.values.as_mut().unwrap();let(mut less,mut eq)=(0,0);for i in begin..end{let v=values.get(i);let first_eq=begin+less;let first_greater=first_eq+eq;if v<x{values.set(i,values.get(first_greater));values.set(first_greater,values.get(first_eq));values.set(first_eq,v);less+=1;}else if v==x{values.set(i,values.get(first_greater));values.set(first_greater,v);eq+=1;}}if eq==0{None}else{let id=self.nodes.len();self.nodes.push(Node{index:begin+less,count:eq,next:vec![]});Some(id)}
}
fn boundary(&self,x:f64,begin:i32,end:i32,ascending:bool)->Option<i32>{let v=self.values.as_ref().unwrap();if begin==end{return Some(if ascending{begin}else{v.size()-begin});}if begin+1==end{if v.get(begin)==x{return Some(if ascending{begin}else{v.size()-end});}if v.get(begin)>x{return Some(if ascending{begin}else{v.size()-begin});}return Some(if ascending{end}else{v.size()-end});}None}
// java public int rank(double x, boolean ascending) {
// java RankCache.Node[] pre_lv = new RankCache.Node[this.head.length];
// java RankCache.Node pre = null;
// java int lv = this.head.length - 1;
// java
// java while(true) {
// java RankCache.Node n = pre == null ? this.head[lv] : pre.next[lv];
// java if (n == null) {
// java pre_lv[lv] = pre;
// java } else {
// java double nv = this.values.get(n.index);
// java if (!(nv > x)) {
// java if (nv == x) {
// java return this.rankOfNode(n, ascending);
// java }
// java
// java pre = n;
// java continue;
// java }
// java
// java pre_lv[lv] = pre;
// java }
// java
// java --lv;
// java if (lv < 0) {
// java int begin;
// java int end;
// java if (pre_lv[0] == null) {
// java begin = 0;
// java end = this.head[0] == null ? this.values.size() : this.head[0].index;
// java } else {
// java begin = pre_lv[0].index + pre_lv[0].count;
// java end = pre_lv[0].next[0] == null ? this.values.size() : pre_lv[0].next[0].index;
// java }
// java
// java if (begin == end) {
// java return ascending ? begin : this.values.size() - begin;
// java }
// java
// java if (begin + 1 == end) {
// java if (this.values.get(begin) == x) {
// java return ascending ? begin : this.values.size() - end;
// java }
// java
// java if (this.values.get(begin) > x) {
// java return ascending ? begin : this.values.size() - begin;
// java }
// java
// java return ascending ? end : this.values.size() - end;
// java }
// java
// java RankCache.Node picked = this.rank_range(this.values.get(begin + MathX.RAND.nextInt(end - begin)), begin, end);
// java this.insertNode(picked, pre_lv);
// java double nv = this.values.get(picked.index);
// java if (x < nv) {
// java end = picked.index;
// java if (this.head.length > pre_lv.length) {
// java RankCache.Node[] np = new RankCache.Node[this.head.length];
// java System.arraycopy(pre_lv, 0, np, 0, pre_lv.length);
// java pre_lv = np;
// java }
// java } else {
// java if (x == this.values.get(picked.index)) {
// java return this.rankOfNode(picked, ascending);
// java }
// java
// java begin = picked.index + picked.count;
// java if (this.head.length > pre_lv.length) {
// java RankCache.Node[] np = new RankCache.Node[this.head.length];
// java Arrays.fill(np, picked);
// java pre_lv = np;
// java } else {
// java Arrays.fill(pre_lv, 0, picked.next.length, picked);
// java }
// java }
// java
// java if (begin == end) {
// java return ascending ? begin : this.values.size() - begin;
// java }
// java
// java if (begin + 1 == end) {
// java if (this.values.get(begin) == x) {
// java return ascending ? begin : this.values.size() - end;
// java }
// java
// java if (this.values.get(begin) > x) {
// java return ascending ? begin : this.values.size() - begin;
// java }
// java
// java return ascending ? end : this.values.size() - end;
// java }
// java
// java n = this.rank_range(x, begin, end);
// java if (n == null) {
// java int less_count = 0;
// java
// java for(int i = begin; i < end; ++i) {
// java if (this.values.get(i) < x) {
// java ++less_count;
// java }
// java }
// java
// java return ascending ? begin + less_count : this.values.size() - begin - less_count;
// java }
// java
// java this.insertNode(n, pre_lv);
// java return this.rankOfNode(n, ascending);
// java }
// java }
// java }
pub fn rank(&mut self,x:f64,ascending:bool)->i32 {
let mut pre_lv=vec![None;self.head.len()];let mut pre:Option<usize>=None;let mut lv=self.head.len()as i32-1;loop{let n=if let Some(p)=pre{self.nodes[p].next[lv as usize]}else{self.head[lv as usize]};if let Some(n)=n{let nv=self.values.as_ref().unwrap().get(self.nodes[n].index);if !(nv>x){if nv==x{return self.rankOfNode(n,ascending);}pre=Some(n);continue;}pre_lv[lv as usize]=pre;}else{pre_lv[lv as usize]=pre;}lv-=1;if lv<0{let(mut begin,mut end)=if let Some(p)=pre_lv[0]{(self.nodes[p].index+self.nodes[p].count,self.nodes[p].next[0].map_or(self.values.as_ref().unwrap().size(),|n|self.nodes[n].index))}else{(0,self.head[0].map_or(self.values.as_ref().unwrap().size(),|n|self.nodes[n].index))};if let Some(n)=self.boundary(x,begin,end,ascending){return n;}let pivot=begin+RAND.lock().unwrap().nextInt(end-begin);let picked=self.rank_range(self.values.as_ref().unwrap().get(pivot),begin,end).expect("null random rank node");self.insertNode(picked,&pre_lv);let nv=self.values.as_ref().unwrap().get(self.nodes[picked].index);if x<nv{end=self.nodes[picked].index;if self.head.len()>pre_lv.len(){pre_lv.resize(self.head.len(),None);}}else{if x==nv{return self.rankOfNode(picked,ascending);}begin=self.nodes[picked].index+self.nodes[picked].count;if self.head.len()>pre_lv.len(){pre_lv=vec![Some(picked);self.head.len()];}else{pre_lv[..self.nodes[picked].next.len()].fill(Some(picked));}}if let Some(n)=self.boundary(x,begin,end,ascending){return n;}let n=self.rank_range(x,begin,end);if let Some(n)=n{self.insertNode(n,&pre_lv);return self.rankOfNode(n,ascending);}let mut less=0;for i in begin..end{if self.values.as_ref().unwrap().get(i)<x{less+=1;}}return if ascending{begin+less}else{self.values.as_ref().unwrap().size()-begin-less};}}
}
}
