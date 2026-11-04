use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::util::cache::MRUCache::MRUCache};use std::sync::{LazyLock,Mutex};use std::collections::HashMap;
// java private static final MRUCache<AreaLocate,Cache> area_cache=new MRUCache();private static int bad_hits=0;
#[derive(Clone,PartialEq,Eq,Hash)]pub struct AreaLocate{pub sheet:i32,pub firstRow:i32,pub lastRow:i32,pub firstCol:i32,pub lastCol:i32}
#[derive(Clone,PartialEq,Eq,Hash)]pub enum MatchObject{Number(u64),Text(Vec<u16>),Bool(bool),Error(i8)}
pub struct Cache{pub countif_cache:RefCell<Option<HashMap<MatchObject,i32>>>}
pub static AREA_CACHE:LazyLock<Mutex<MRUCache<AreaLocate,Rc<Cache>>>>=LazyLock::new(||Mutex::new(MRUCache::new(50)));
pub static BAD_HITS:Mutex<i32>=Mutex::new(0);
pub struct CountifCache;impl CountifCache{
// java public static void clearCache(){synchronized(area_cache){area_cache.clear();bad_hits=0;}}
pub fn clearCache(){AREA_CACHE.lock().unwrap().clear();*BAD_HITS.lock().unwrap()=0;}}

use super::super::functions::util::{CountUtils::CountUtils,matcher::{IMatchPredicate::IMatchPredicate,ConditionMatchPredicate::{ConditionMatchPredicate,NumberMatcher,lower}}};
impl CountifCache{// java public static int countif(IAreaEval criteria_range, IMatchPredicate mp) {
// java Object simpleMatcher = ConditionMatchPredicate.toSimpleMatchObject(mp);
// java if (simpleMatcher != null) {
// java AreaLocate al = new AreaLocate(criteria_range.getSheetIndex(), criteria_range.getFirstRow(), criteria_range.getLastRow(), criteria_range.getFirstColumn(), criteria_range.getLastColumn());
// java Cache c;
// java if (criteria_range.getWidth() * criteria_range.getHeight() > 20) {
// java synchronized(area_cache) {
// java c = area_cache.get(al);
// java if (c == null) {
// java c = new Cache();
// java area_cache.put(al, c);
// java }
// java }
// java } else {
// java c = new Cache();
// java }
// java
// java synchronized(c) {
// java c.buildCache(criteria_range);
// java return c.countif(simpleMatcher);
// java }
// java } else {
// java if (mp instanceof ConditionMatchPredicate.NumberMatcher) {
// java ConditionMatchPredicate.NumberMatcher nm = (ConditionMatchPredicate.NumberMatcher)mp;
// java int op = nm.getCode();
// java if (op == 4 || op == 5 || op == 3 || op == 6) {
// java return CountifRankCache.rank(criteria_range, nm.getValue(), op);
// java }
// java }
// java
// java ++bad_hits;
// java return CountUtils.countMatchingCellsInArea(criteria_range, mp);
// java }
// java }
pub fn countif(a:&dyn IAreaEval,p:&dyn IMatchPredicate)->EvalResult<i32> {
if let Some(key)=ConditionMatchPredicate::toSimpleMatchObject(p){let al=AreaLocate{sheet:a.getSheetIndex(),firstRow:a.getFirstRow(),lastRow:a.getLastRow(),firstCol:a.getFirstColumn(),lastCol:a.getLastColumn()};let c=if a.getWidth().wrapping_mul(a.getHeight())>20{let cache=AREA_CACHE.lock().unwrap();if let Some(c)=cache.get(al.clone()){c}else{let c=Rc::new(Cache{countif_cache:RefCell::new(None)});cache.put(al,Some(c.clone()));c}}else{Rc::new(Cache{countif_cache:RefCell::new(None)})};let mut m=c.countif_cache.borrow_mut();if m.is_none(){*m=Some(Cache::buildCache(a)?);}return Ok(*m.as_ref().unwrap().get(&key).unwrap_or(&0));}if let Some(n)=p.as_any().downcast_ref::<NumberMatcher>(){if matches!(n.getCode(),3|4|5|6){return super::CountifRankCache::CountifRankCache::rank(a,n.getValue(),n.getCode());}}*BAD_HITS.lock().unwrap()+=1;CountUtils::countMatchingCellsInArea(a,p)
}
}
impl Cache{// java public void buildCache(IAreaEval ae) {
// java if (this.countif_cache == null) {
// java Map<Object, Integer> countifCache = new HashMap();
// java int count_blank = 0;
// java int count_true = 0;
// java int count_false = 0;
// java int touched = 0;
// java IValueIterator iter = ae.valueIterator();
// java
// java while(iter.hasNext()) {
// java iter.next();
// java IValueEval ve = iter.getValue();
// java ++touched;
// java if (ve != BlankEval.instance && !(ve instanceof MissingArgEval)) {
// java if (ve instanceof StringEval) {
// java String s = ((StringEval)ve).getStringValue().toLowerCase();
// java Integer i = (Integer)countifCache.get(s);
// java countifCache.put(s, i == null ? CountifCache.ONE : i + 1);
// java Double d = OperandResolver.parseDouble(s);
// java if (d != null) {
// java i = (Integer)countifCache.get(d);
// java countifCache.put(d, i == null ? CountifCache.ONE : i + 1);
// java }
// java } else if (ve instanceof NumberEval) {
// java Double d = ((NumberEval)ve).getNumberValue();
// java Integer i = (Integer)countifCache.get(d);
// java countifCache.put(d, i == null ? CountifCache.ONE : i + 1);
// java } else if (ve instanceof BoolEval) {
// java if (((BoolEval)ve).getBooleanValue()) {
// java ++count_true;
// java } else {
// java ++count_false;
// java }
// java } else if (ve instanceof ErrorEval) {
// java Byte e = (byte)((ErrorEval)ve).getErrorCode();
// java Integer i = (Integer)countifCache.get(e);
// java countifCache.put(e, i == null ? CountifCache.ONE : i + 1);
// java }
// java } else {
// java ++count_blank;
// java }
// java }
// java
// java count_blank += ae.getHeight() * ae.getWidth() - touched;
// java if (count_blank > 0) {
// java Integer i = (Integer)countifCache.get("");
// java countifCache.put("", i == null ? count_blank : Integer.valueOf(i + count_blank));
// java }
// java
// java if (count_true > 0) {
// java countifCache.put(Boolean.TRUE, count_true);
// java }
// java
// java if (count_false > 0) {
// java countifCache.put(Boolean.FALSE, count_false);
// java }
// java
// java this.countif_cache = countifCache;
// java }
// java }
pub fn buildCache(a:&dyn IAreaEval)->EvalResult<HashMap<MatchObject,i32>> {
let mut m=HashMap::<MatchObject,i32>::new();let(mut blank,mut yes,mut no,mut touched)=(0i32,0i32,0i32,0i32);let mut i=a.valueIterator();while i.hasNext(){i.next();let v=i.getValue()?;touched+=1;if v.as_any().is::<BlankEval>()||v.as_any().is::<MissingArgEval>(){blank+=1;}else if let Some(s)=v.as_any().downcast_ref::<StringEval>(){let s=lower(s.getStringValue());*m.entry(MatchObject::Text(s.clone())).or_default()+=1;if let Some(n)=OperandResolver::parseDouble(&s)?{*m.entry(MatchObject::Number(if n.is_nan(){0x7ff8000000000000}else{n.to_bits()})).or_default()+=1;}}else if let Some(n)=v.as_any().downcast_ref::<NumberEval>(){let n=n.getNumberValue();*m.entry(MatchObject::Number(if n.is_nan(){0x7ff8000000000000}else{n.to_bits()})).or_default()+=1;}else if let Some(b)=v.as_any().downcast_ref::<BoolEval>(){if b.getBooleanValue(){yes+=1;}else{no+=1;}}else if let Some(e)=v.as_any().downcast_ref::<ErrorEval>(){*m.entry(MatchObject::Error(e.getErrorCode()as i8)).or_default()+=1;}}blank=blank.wrapping_add(a.getHeight().wrapping_mul(a.getWidth()).wrapping_sub(touched));if blank>0{*m.entry(MatchObject::Text(vec![])).or_default()+=blank;}if yes>0{m.insert(MatchObject::Bool(true),yes);}if no>0{m.insert(MatchObject::Bool(false),no);}Ok(m)
}
}
