use crate::{java_runtime::{Rc,RefCell},cn::wps::{moss::{app::KmoSheet::KmoSheet,util::{KmoRect::KmoRect,AppLogicHelper::AppLogicHelper},engine::eval::EvaluationWorkbook::EvaluationWorkbook},et::ss::{calcchain::NameFmlEvaluator::NameFmlEvaluator,formula::{ptg::Ptg::Ptg,eval::{ValueRef,NumberEval::NumberEval,BoolEval::BoolEval,StringEval::StringEval,ErrorEval::ErrorEval},evaluator::{EvalResult,EvalFailure},util::DoubleList::DoubleList,functions::Percentile::Percentile}}},org::apache::poi::ss::util::NumberComparer::NumberComparer};
use super::{KmoCfRule::KmoCfRule,KmoCfvo::{KmoCfvo,CfvoTypes},KmoCfColor::KmoCfColor,KmoConditionFormatter::{CfRuleResult,CfCacheItem},rule::{Rule::CfRuleTypes,KmoRuleCellIs::{KmoRuleCellIs,CellIsOperatorType},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::KmoIconSet},valuebased::{CfDataBarValue::CfDataBarValue,CfColorScaleValue::CfColorScaleValue,CfIconSetValue::CfIconSetValue}};
use std::sync::{Mutex,atomic::{AtomicBool,Ordering}};
use kfc::cn::wps::memor4j::KmoAtomStore::KmoAtomStore;
// java public class KmoCondFmtEval implements Runnable
pub struct KmoCondFmtEval{mSheet:RefCell<Option<Rc<RefCell<KmoSheet>>>>,mFmlEval:RefCell<Option<Rc<NameFmlEvaluator>>>,screenRange:Mutex<Option<Rc<Vec<KmoRect>>>>,stopping:AtomicBool,running:AtomicBool,listener:RefCell<Option<Rc<dyn ICondFmtEvalListener>>>}
pub trait ICondFmtEvalListener:Send+Sync{fn evalFinished(&self,sheetUid:i32,row:i32,col:i32);}
struct TipStamp(i32);impl kfc::cn::wps::memor4j::KmoTipStamp::KmoTipStamp for TipStamp{fn tip(&self)->i32{self.0}}impl kfc::cn::wps::memor4j::KmoAtomStore::KmoTipStamp for TipStamp{fn tip(&self)->i32{self.0}}
impl KmoCondFmtEval{
// java field initializers: screenRange=new AtomicReference();stopping=false;running=new AtomicBoolean(false).
pub fn new()->Self{Self{mSheet:RefCell::new(None),mFmlEval:RefCell::new(None),screenRange:Mutex::new(None),stopping:AtomicBool::new(false),running:AtomicBool::new(false),listener:RefCell::new(None)}}
// java public void setEvalListener(ICondFmtEvalListener l) {
// java this.listener = l;
// java }
pub fn setEvalListener(&self,l:Option<Rc<dyn ICondFmtEvalListener>>) {
*self.listener.borrow_mut()=l;
}
// java public void setSheet(KmoSheet sheet) {
// java this.mSheet = sheet;
// java }
pub fn setSheet(&self,sheet:Rc<RefCell<KmoSheet>>) {
*self.mSheet.borrow_mut()=Some(sheet);
}
// java public Thread requestEval(KmoSheet sheet, List<KmoRect> area) {
// java this.mSheet = sheet;
// java this.screenRange.set(area);
// java if (this.running.compareAndSet(false, true)) {
// java Thread t = new Thread(this, "CondFmtEval");
// java t.setPriority(1);
// java t.setDaemon(true);
// java t.start();
// java return t;
// java } else {
// java return null;
// java }
// java }
pub fn requestEval(self:&Rc<Self>,sheet:Rc<RefCell<KmoSheet>>,area:Rc<Vec<KmoRect>>)->Option<std::thread::JoinHandle<()>> {
self.setSheet(sheet);*self.screenRange.lock().unwrap()=Some(area);if self.running.compare_exchange(false,true,Ordering::SeqCst,Ordering::SeqCst).is_ok(){let this=self.clone();Some(std::thread::Builder::new().name("CondFmtEval".into()).spawn(move||this.run()).expect("CondFmtEval thread"))}else{None}
}
// java public void requestStop() {
// java this.stopping = true;
// java }
pub fn requestStop(&self) {
self.stopping.store(true,Ordering::SeqCst);
}
// java public void run() {
// java if (this.mSheet != null) {
// java ThreadAffinityCallback threadAffinityCallback = this.mSheet.getBook().getThreadAffinityCallback();
// java if (threadAffinityCallback != null) {
// java threadAffinityCallback.enableBigCores();
// java }
// java }
// java
// java while(!this.stopping && this.screenRange.get() != null) {
// java try {
// java this.evalImpl();
// java } catch (Exception e) {
// java e.printStackTrace();
// java }
// java }
// java
// java this.running.set(false);
// java }
pub fn run(&self) {
// rust: ThreadAffinityCallback priority/daemon are host scheduling adapters; the original request/drain state remains here.
while !self.stopping.load(Ordering::SeqCst)&&self.screenRange.lock().unwrap().is_some(){let _=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||self.evalImpl()));}self.running.store(false,Ordering::SeqCst);
}
// java private void evalImpl() {
// java KmoSheet sheet = null;
// java List<KmoRect> screenAreas = null;
// java
// java do {
// java sheet = this.mSheet;
// java screenAreas = (List)this.screenRange.get();
// java boolean result = this.screenRange.compareAndSet(screenAreas, null);
// java
// java try {
// java if (!result) {
// java Thread.sleep(500L);
// java }
// java } catch (InterruptedException var19) {
// java }
// java } while(this.screenRange.get() != null);
// java
// java if (screenAreas != null) {
// java int count = 0;
// java int i = 0;
// java
// java for(int size = screenAreas.size(); i < size; ++i) {
// java count += ((KmoRect)screenAreas.get(i)).area();
// java }
// java
// java MRULongKeyCache<KmoConditionFormatter.CfCacheItem> cache = sheet.getCFormatter().getResultCache();
// java if (count <= cache.getCapacity()) {
// java int sheetUid = sheet.getSheetUid();
// java KmoTipStamp tip = KmoAtomStore.sharedStore().getCurrentTipStamp();
// java List<KmoCfRule> rules = new ArrayList();
// java
// java for(int areaIdx = 0; areaIdx < screenAreas.size(); ++areaIdx) {
// java KmoRect area = (KmoRect)screenAreas.get(areaIdx);
// java
// java for(int row = area.tl.row; row <= area.br.row; ++row) {
// java for(int col = area.tl.col; col <= area.br.col; ++col) {
// java if (this.stopping) {
// java return;
// java }
// java
// java long k = AppLogicHelper.rc2long(row, col);
// java KmoConditionFormatter.CfCacheItem item = cache.get(k);
// java if (item == null || !KmoAtomStore.sharedStore().isTheSameTipStamp(item.tip)) {
// java int actualRow = row;
// java int actualCol = col;
// java KmoRect mergedRange = this.mSheet.getMergedRange(row, col);
// java if (mergedRange != null) {
// java actualRow = mergedRange.tl.row;
// java actualCol = mergedRange.tl.col;
// java }
// java
// java rules.clear();
// java this.mSheet.getCFormatter().getCellRules(actualRow, actualCol, rules);
// java if (rules.size() == 0) {
// java if (item != null) {
// java cache.remove(k);
// java if (this.listener != null) {
// java this.listener.evalFinished(sheetUid, row, col);
// java }
// java }
// java } else {
// java KmoConditionFormatter.CfRuleResult result = this.getSatisfyRuleEx(actualRow, actualCol, rules);
// java if (result != null && !result.isEmpty()) {
// java if (item != null) {
// java if (result.equals(item.rules)) {
// java continue;
// java }
// java
// java item.rules = result;
// java item.tip = tip;
// java } else {
// java item = new KmoConditionFormatter.CfCacheItem(result, tip);
// java cache.put(k, item);
// java }
// java
// java if (this.listener != null) {
// java this.listener.evalFinished(sheetUid, row, col);
// java }
// java } else if (item != null) {
// java cache.remove(k);
// java if (this.listener != null) {
// java this.listener.evalFinished(sheetUid, row, col);
// java }
// java }
// java }
// java }
// java }
// java }
// java }
// java
// java }
// java }
// java }
pub fn evalImpl(&self) {
let (sheet,areas)=loop{let sheet=self.mSheet.borrow().clone().expect("null sheet");let areas=self.screenRange.lock().unwrap().clone();let success={let mut current=self.screenRange.lock().unwrap();let same=match(&*current,&areas){(Some(a),Some(b))=>Rc::ptr_eq(a,b),(None,None)=>true,_=>false};if same{*current=None;}same};if !success{std::thread::sleep(std::time::Duration::from_millis(500));}if self.screenRange.lock().unwrap().is_none(){break(sheet,areas);}};let Some(areas)=areas else{return;};let count=areas.iter().fold(0i32,|n,a|n.wrapping_add(a.area()));let sh=sheet.borrow();let cache=sh.getCFormatter().getResultCache();if count>cache.getCapacity(){return;}let uid=sh.getSheetUid();let tip=KmoAtomStore::sharedStore().as_ref().unwrap().getCurrentTipStamp().tip();let mut rules=vec![];
for area in areas.iter(){for row in area.tl.row..=area.br.row{for col in area.tl.col..=area.br.col{if self.stopping.load(Ordering::SeqCst){return;}let key=AppLogicHelper::rc2long(row,col);let item=cache.get(key);let stale=item.as_ref().is_none_or(|i|i.tip.borrow().as_ref().is_none_or(|s|!KmoAtomStore::sharedStore().as_ref().unwrap().isTheSameTipStamp(&TipStamp(s.tip()))));if !stale{continue;}let(actualRow,actualCol)=sh.getMergedRange(row,col).map_or((row,col),|r|(r.tl.row,r.tl.col));rules.clear();self.mSheet.borrow().as_ref().unwrap().borrow().getCFormatter().getCellRules(actualRow,actualCol,&mut rules);if rules.is_empty(){if item.is_some(){cache.remove(key);self.finished(uid,row,col);}}else{let result=self.getSatisfyRuleEx(actualRow,actualCol,&mut rules);if !result.isEmpty(){if let Some(item)=item{if result.equals(&item.rules.borrow()){continue;}*item.rules.borrow_mut()=result;*item.tip.borrow_mut()=Some(Rc::new(TipStamp(tip)));}else{cache.put(key,Some(Rc::new(CfCacheItem{rules:RefCell::new(result),tip:RefCell::new(Some(Rc::new(TipStamp(tip))))})));}self.finished(uid,row,col);}else if item.is_some(){cache.remove(key);self.finished(uid,row,col);}}}}}
}
fn finished(&self,uid:i32,row:i32,col:i32){if let Some(l)=self.listener.borrow().as_ref(){l.evalFinished(uid,row,col);}}
// java public KmoConditionFormatter.CfRuleResult getSatisfyRuleEx(int actualRow, int actualCol, List<KmoCfRule> rules) {
// java int count = rules.size();
// java if (count > 1) {
// java Collections.sort(rules, cfRuleComparator);
// java }
// java
// java KmoConditionFormatter.CfRuleResult result = new KmoConditionFormatter.CfRuleResult();
// java int i = 0;
// java
// java for(int size = rules.size(); i < size; ++i) {
// java KmoCfRule rule = (KmoCfRule)rules.get(i);
// java if (this.isSatisfyRule(this.mSheet, rule, actualRow, actualCol)) {
// java if (result.generals == null) {
// java result.generals = new ArrayList();
// java }
// java
// java result.generals.add(rule);
// java if (rule.getRule().isStopIfTrue()) {
// java return result;
// java }
// java }
// java }
// java
// java i = 0;
// java
// java for(int size = rules.size(); i < size; ++i) {
// java KmoCfRule rule = (KmoCfRule)rules.get(i);
// java this.satisfy07Rule(this.mSheet, rule, actualRow, actualCol, result);
// java if (result.colorScaleValue != null && result.dataBarValue != null && result.iconSetValue != null) {
// java break;
// java }
// java }
// java
// java return result;
// java }
pub fn getSatisfyRuleEx(&self,row:i32,col:i32,rules:&mut [Rc<KmoCfRule>])->CfRuleResult {
if rules.len()>1{rules.sort_by(|a,b|a.getRule().getPriority().wrapping_sub(b.getRule().getPriority()).cmp(&0));}let mut result=CfRuleResult::default();for rule in rules.iter(){if self.isSatisfyRule(rule,row,col){result.generals.get_or_insert_with(Vec::new).push(rule.clone());if rule.getRule().isStopIfTrue(){return result;}}}for rule in rules.iter(){self.satisfy07Rule(rule,row,col,&mut result);if result.colorScaleValue.is_some()&&result.dataBarValue.is_some()&&result.iconSetValue.is_some(){break;}}result
}
// java private IValueEval evalulateFml(KmoSheet sheet, int row, int col, Ptg[] ptgs) {
// java if (ptgs.length == 0) {
// java return null;
// java } else {
// java if (this.mFmlEval == null) {
// java this.mFmlEval = new NameFmlEvaluator(new EvaluationWorkbook(sheet.getBook()));
// java }
// java
// java return this.mFmlEval.evaluateConditionFormula(sheet.getSheetIndex(), row, col, ptgs);
// java }
// java }
pub fn evalulateFml(&self,sheet:&KmoSheet,row:i32,col:i32,ptgs:&[Ptg])->EvalResult<Option<ValueRef>> {
if ptgs.is_empty(){return Ok(None);}let evaluator={let mut f=self.mFmlEval.borrow_mut();f.get_or_insert_with(||Rc::new(NameFmlEvaluator::new(Rc::new(EvaluationWorkbook::EvaluationWorkbook(&sheet.getBook()))))).clone()};evaluator.evaluateConditionFormula(sheet.getSheetIndex(),row,col,ptgs)
}
// java private boolean isSatisfyRule(KmoSheet sheet, KmoCfRule rule, int row, int col) {
// java boolean fSatisfy = false;
// java
// java try {
// java Rule.CfRuleTypes type = rule.ruleType();
// java if (this.isCellIsType(type)) {
// java fSatisfy = this.judgeCellIs(rule, sheet, row, col);
// java } else if (this.isSupportedFmlType(type)) {
// java if (this.notJoinInRule(type, sheet, row, col)) {
// java fSatisfy = false;
// java } else {
// java fSatisfy = this.judgeFml(rule, sheet, row, col);
// java }
// java } else {
// java fSatisfy = false;
// java }
// java } catch (Exception var7) {
// java fSatisfy = false;
// java } catch (OutOfMemoryError var8) {
// java fSatisfy = false;
// java }
// java
// java return fSatisfy;
// java }
pub fn isSatisfyRule(&self,rule:&KmoCfRule,row:i32,col:i32)->bool {
std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
let sheet=self.mSheet.borrow().as_ref().unwrap().clone();let sheet=sheet.borrow();let Some(t)=rule.ruleType()else{return false;};if t==CfRuleTypes::cellIs{self.judgeCellIs(rule,&sheet,row,col).unwrap_or(false)}else if Self::isSupportedFmlType(t){if Self::notJoinInRule(t,&sheet,row,col){false}else{self.judgeFml(rule,&sheet,row,col).unwrap_or(false)}}else{false}
})).unwrap_or(false)
}
// java private boolean isSupportedFmlType(Rule.CfRuleTypes ruleType) {
// java switch (ruleType) {
// java case colorScale:
// java case dataBar:
// java case iconSet:
// java default:
// java return false;
// java case cellIs:
// java case expression:
// java case containsText:
// java case notContainsText:
// java case beginsWith:
// java case endsWith:
// java case notContainsBlanks:
// java case containsBlanks:
// java case timePeriod:
// java case uniqueValues:
// java case duplicateValues:
// java case containsErrors:
// java case notContainsErrors:
// java case aboveAverage:
// java case top10:
// java return true;
// java }
// java }
pub fn isSupportedFmlType(t:CfRuleTypes)->bool {
!matches!(t,CfRuleTypes::colorScale|CfRuleTypes::dataBar|CfRuleTypes::iconSet|CfRuleTypes::error)
}
// java private boolean notJoinInRule(Rule.CfRuleTypes ruleType, KmoSheet sheet, int row, int col) {
// java if (ruleType == Rule.CfRuleTypes.top10 || ruleType == Rule.CfRuleTypes.aboveAverage) {
// java int cellValueType = sheet.getCellValueType(row, col);
// java if (cellValueType != 1) {
// java return true;
// java }
// java }
// java
// java return false;
// java }
pub fn notJoinInRule(t:CfRuleTypes,sheet:&KmoSheet,row:i32,col:i32)->bool {
matches!(t,CfRuleTypes::top10|CfRuleTypes::aboveAverage)&&sheet.getCellValueType(row,col,None)!=1
}
// java private boolean judgeFml(KmoCfRule rule, KmoSheet sheet, int row, int col) {
// java boolean fSatisfy = false;
// java Ptg[] ptgs1 = rule.getRule().formula1();
// java if (ptgs1.length == 0) {
// java return false;
// java } else {
// java IValueEval ve1 = this.evalulateFml(sheet, row, col, ptgs1);
// java fSatisfy = this.fmlResult(ve1);
// java return fSatisfy;
// java }
// java }
pub fn judgeFml(&self,rule:&KmoCfRule,sheet:&KmoSheet,row:i32,col:i32)->EvalResult<bool> {
let ptgs=rule.getRule().formula1().ok_or(EvalFailure::Runtime("null formula"))?;if ptgs.is_empty(){return Ok(false);}let v=self.evalulateFml(sheet,row,col,&ptgs)?;Ok(v.is_some_and(|v|Self::fmlResult(&v)))
}
// java private boolean fmlResult(IValueEval ve) {
// java if (ve instanceof NumberEval) {
// java return ((NumberEval)ve).getNumberValue() != (double)0.0F;
// java } else if (ve instanceof BoolEval) {
// java return ((BoolEval)ve).getBooleanValue();
// java } else if (ve instanceof StringEval) {
// java return false;
// java } else {
// java return ve instanceof ErrorEval ? false : false;
// java }
// java }
pub fn fmlResult(v:&ValueRef)->bool {
if let Some(n)=v.as_any().downcast_ref::<NumberEval>(){n.getNumberValue()!=0.0}else if let Some(b)=v.as_any().downcast_ref::<BoolEval>(){b.getBooleanValue()}else{false}
}
// java private boolean judgeCellIs(KmoCfRule rule, KmoSheet sheet, int row, int col) {
// java boolean fSatisfy = false;
// java Rule cellIs = rule.getRule();
// java KmoRuleCellIs.CellIsOperatorType cp = ((KmoRuleCellIs)cellIs).getCellIsOpType();
// java switch (cp) {
// java case lessThan:
// java case lessThanOrEqual:
// java case equal:
// java case notEqual:
// java case greaterThanOrEqual:
// java case greaterThan: {
// java IValueEval ve1 = this.evalulateFml(sheet, row, col, cellIs.formula1());
// java fSatisfy = this.compareOneFml(sheet, row, col, ve1, cp);
// java break;
// java }
// java case between:
// java case notBetween: {
// java IValueEval ve1 = this.evalulateFml(sheet, row, col, cellIs.formula1());
// java IValueEval ve2 = this.evalulateFml(sheet, row, col, cellIs.formula2());
// java fSatisfy = this.compareTwoFml(sheet, row, col, ve1, ve2, cp);
// java break;
// java }
// java default:
// java fSatisfy = false;
// java }
// java
// java return fSatisfy;
// java }
pub fn judgeCellIs(&self,rule:&KmoCfRule,sheet:&KmoSheet,row:i32,col:i32)->EvalResult<bool> {
let base=rule.getRule();let cell=base.as_any().downcast_ref::<KmoRuleCellIs>().ok_or(EvalFailure::Runtime("ClassCastException cellIs"))?;let cp=cell.getCellIsOpType().ok_or(EvalFailure::Runtime("null operator"))?;use CellIsOperatorType::*;match cp{lessThan|lessThanOrEqual|equal|notEqual|greaterThanOrEqual|greaterThan=>{let v=self.evalulateFml(sheet,row,col,&base.formula1().unwrap())?.ok_or(EvalFailure::Runtime("null eval"))?;Self::compareOneFml(sheet,row,col,&v,cp)},between|notBetween=>{let a=self.evalulateFml(sheet,row,col,&base.formula1().unwrap())?.ok_or(EvalFailure::Runtime("null eval"))?;let b=self.evalulateFml(sheet,row,col,&base.formula2().unwrap())?.ok_or(EvalFailure::Runtime("null eval"))?;Self::compareTwoFml(sheet,row,col,&a,&b,cp)},_=>Ok(false)}
}
// java private int getFmlResultType(IValueEval ve) {
// java if (ve instanceof NumberEval) {
// java return 1;
// java } else if (ve instanceof StringEval) {
// java return 2;
// java } else if (ve instanceof BoolEval) {
// java return 5;
// java } else if (ve instanceof ErrorEval) {
// java return 6;
// java } else {
// java throw new RuntimeException("Unexpected eval class (" + ve.getClass().getName() + ")");
// java }
// java }
pub fn getFmlResultType(v:&ValueRef)->EvalResult<i32> {
if v.as_any().is::<NumberEval>(){Ok(1)}else if v.as_any().is::<StringEval>(){Ok(2)}else if v.as_any().is::<BoolEval>(){Ok(5)}else if v.as_any().is::<ErrorEval>(){Ok(6)}else{Err(EvalFailure::Runtime("Unexpected eval class"))}
}
fn strCompare(a:&[u16],b:&[u16])->i32{for(a,b)in a.iter().zip(b){if a!=b{return *a as i32-*b as i32;}}a.len()as i32-b.len()as i32}
// java private boolean compareOneFml(KmoSheet sheet, int row, int col, IValueEval ve, KmoRuleCellIs.CellIsOperatorType cp) {
// java int cellType = sheet.getCellValueType(row, col);
// java int v1Type = this.getFmlResultType(ve);
// java if (v1Type != 6 && cellType != 6) {
// java if ((cellType != 0 || v1Type != 1) && cellType != v1Type) {
// java if (cellType == 0 && v1Type == 2) {
// java String strVe1 = ((StringEval)ve).getStringValue();
// java if (strVe1.length() <= 0) {
// java return this.numericalCompare((double)0.0F, cp);
// java }
// java }
// java
// java if (cellType == 0 && v1Type == 5) {
// java int b2 = ((BoolEval)ve).getBooleanValue() ? 1 : 0;
// java int cmp = 0 - b2;
// java return this.numericalCompare((double)cmp, cp);
// java } else {
// java return cellType == 0 ? this.numericalCompare((double)-1.0F, cp) : this.numericalCompare((double)(cellType - v1Type), cp);
// java }
// java } else {
// java double cmp = (double)0.0F;
// java if ((cellType == 1 || cellType == 0) && ve instanceof NumberEval) {
// java double v = cellType == 1 ? sheet.getCellNumericValue(row, col) : (double)0.0F;
// java double v1 = ((NumberEval)ve).getNumberValue();
// java cmp = (double)NumberComparer.compare(v, v1);
// java } else if (cellType == 2) {
// java String strCellValue = sheet.getFormattedCellValue(row, col);
// java String strVe1 = ((StringEval)ve).getStringValue();
// java cmp = (double)strCellValue.compareTo(strVe1);
// java } else if (cellType == 5) {
// java int b1 = sheet.getCellBooleanValue(row, col) ? 1 : 0;
// java int b2 = ((BoolEval)ve).getBooleanValue() ? 1 : 0;
// java cmp = (double)(b1 - b2);
// java }
// java
// java return this.numericalCompare(cmp, cp);
// java }
// java } else {
// java return false;
// java }
// java }
pub fn compareOneFml(sheet:&KmoSheet,row:i32,col:i32,v:&ValueRef,cp:CellIsOperatorType)->EvalResult<bool> {
let cell=sheet.getCellValueType(row,col,None);let typ=Self::getFmlResultType(v)?;if typ==6||cell==6{return Ok(false);}let cmp=if (cell!=0||typ!=1)&&cell!=typ{if cell==0&&typ==2&&v.as_any().downcast_ref::<StringEval>().unwrap().getStringValue().is_empty(){return Ok(Self::numericalCompare(0.0,cp));}if cell==0&&typ==5{-(v.as_any().downcast_ref::<BoolEval>().unwrap().getBooleanValue()as i32)as f64}else if cell==0{-1.0}else{(cell-typ)as f64}}else if(cell==1||cell==0)&&typ==1{NumberComparer::compare(if cell==1{sheet.getCellNumericValue(row,col)}else{0.0},v.as_any().downcast_ref::<NumberEval>().unwrap().getNumberValue())as f64}else if cell==2{Self::strCompare(&sheet.getFormattedCellValue(row,col).encode_utf16().collect::<Vec<_>>(),v.as_any().downcast_ref::<StringEval>().unwrap().getStringValue())as f64}else if cell==5{((sheet.getCellBooleanValue(row,col)as i32)-(v.as_any().downcast_ref::<BoolEval>().unwrap().getBooleanValue()as i32))as f64}else{0.0};Ok(Self::numericalCompare(cmp,cp))
}
// java private boolean compareTwoFml(KmoSheet sheet, int row, int col, IValueEval ve1, IValueEval ve2, KmoRuleCellIs.CellIsOperatorType cp) {
// java int cellType = sheet.getCellValueType(row, col);
// java if (cellType != 6 && !(ve1 instanceof ErrorEval) && !(ve2 instanceof ErrorEval)) {
// java if (ve1 instanceof NumberEval && ve2 instanceof NumberEval) {
// java if (cellType != 1 && cellType != 0) {
// java return cp == KmoRuleCellIs.CellIsOperatorType.notBetween;
// java } else {
// java double v1 = ((NumberEval)ve1).getNumberValue();
// java double v2 = ((NumberEval)ve2).getNumberValue();
// java double min = v1 < v2 ? v1 : v2;
// java double max = v1 < v2 ? v2 : v1;
// java double v = cellType == 1 ? sheet.getCellNumericValue(row, col) : (double)0.0F;
// java return this.numericalCompare2(v, min, max, cp);
// java }
// java } else {
// java double cmp = this.compareBetweenTwoFml(ve1, ve2);
// java IValueEval min = cmp < (double)0.0F ? ve1 : ve2;
// java IValueEval max = cmp < (double)0.0F ? ve2 : ve1;
// java return this.compareTwoFml2(sheet, row, col, min, max, cp);
// java }
// java } else {
// java return false;
// java }
// java }
pub fn compareTwoFml(sheet:&KmoSheet,row:i32,col:i32,a:&ValueRef,b:&ValueRef,cp:CellIsOperatorType)->EvalResult<bool> {
let typ=sheet.getCellValueType(row,col,None);if typ==6||a.as_any().is::<ErrorEval>()||b.as_any().is::<ErrorEval>(){return Ok(false);}if let(Some(a),Some(b))=(a.as_any().downcast_ref::<NumberEval>(),b.as_any().downcast_ref::<NumberEval>()){if typ!=1&&typ!=0{return Ok(cp==CellIsOperatorType::notBetween);}let(a,b)=(a.getNumberValue(),b.getNumberValue());let(min,max)=if a<b{(a,b)}else{(b,a)};let v=if typ==1{sheet.getCellNumericValue(row,col)}else{0.0};Ok(Self::numericalCompare2(v,min,max,cp))}else{let cmp=Self::compareBetweenTwoFml(a,b)?;let(min,max)=if cmp<0.0{(a,b)}else{(b,a)};Self::compareTwoFml2(sheet,row,col,min,max,cp)}
}
// java private double compareBetweenTwoFml(IValueEval ve1, IValueEval ve2) {
// java int type1 = this.getFmlResultType(ve1);
// java int type2 = this.getFmlResultType(ve2);
// java if (type1 != type2) {
// java return (double)(type1 - type2);
// java } else if (type1 == 1) {
// java return ((NumberEval)ve1).getNumberValue() - ((NumberEval)ve2).getNumberValue();
// java } else if (type1 == 2) {
// java String strVe1 = ((StringEval)ve1).getStringValue();
// java String strVe2 = ((StringEval)ve2).getStringValue();
// java return (double)strVe1.compareTo(strVe2);
// java } else if (type1 == 5) {
// java int b1 = ((BoolEval)ve1).getBooleanValue() ? 1 : 0;
// java int b2 = ((BoolEval)ve2).getBooleanValue() ? 1 : 0;
// java return (double)(b1 - b2);
// java } else {
// java return (double)(type1 - type2);
// java }
// java }
pub fn compareBetweenTwoFml(a:&ValueRef,b:&ValueRef)->EvalResult<f64> {
let x=Self::getFmlResultType(a)?;let y=Self::getFmlResultType(b)?;Ok(if x!=y{(x-y)as f64}else if x==1{a.as_any().downcast_ref::<NumberEval>().unwrap().getNumberValue()-b.as_any().downcast_ref::<NumberEval>().unwrap().getNumberValue()}else if x==2{Self::strCompare(a.as_any().downcast_ref::<StringEval>().unwrap().getStringValue(),b.as_any().downcast_ref::<StringEval>().unwrap().getStringValue())as f64}else if x==5{((a.as_any().downcast_ref::<BoolEval>().unwrap().getBooleanValue()as i32)-(b.as_any().downcast_ref::<BoolEval>().unwrap().getBooleanValue()as i32))as f64}else{(x-y)as f64})
}
// java private boolean compareTwoFml2(KmoSheet sheet, int row, int col, IValueEval minVE, IValueEval maxVE, KmoRuleCellIs.CellIsOperatorType cp) {
// java if (cp == KmoRuleCellIs.CellIsOperatorType.between) {
// java return this.compareOneFml(sheet, row, col, minVE, KmoRuleCellIs.CellIsOperatorType.greaterThanOrEqual) && this.compareOneFml(sheet, row, col, maxVE, KmoRuleCellIs.CellIsOperatorType.lessThanOrEqual);
// java } else if (cp != KmoRuleCellIs.CellIsOperatorType.notBetween) {
// java return false;
// java } else {
// java return this.compareOneFml(sheet, row, col, minVE, KmoRuleCellIs.CellIsOperatorType.lessThan) || this.compareOneFml(sheet, row, col, maxVE, KmoRuleCellIs.CellIsOperatorType.greaterThan);
// java }
// java }
pub fn compareTwoFml2(sheet:&KmoSheet,row:i32,col:i32,min:&ValueRef,max:&ValueRef,cp:CellIsOperatorType)->EvalResult<bool> {
use CellIsOperatorType::*;if cp==between{Ok(Self::compareOneFml(sheet,row,col,min,greaterThanOrEqual)?&&Self::compareOneFml(sheet,row,col,max,lessThanOrEqual)?)}else if cp==notBetween{Ok(Self::compareOneFml(sheet,row,col,min,lessThan)?||Self::compareOneFml(sheet,row,col,max,greaterThan)?)}else{Ok(false)}
}
// java private boolean numericalCompare(double cmp, KmoRuleCellIs.CellIsOperatorType cp) {
// java switch (cp) {
// java case lessThan:
// java return cmp < (double)0.0F;
// java case lessThanOrEqual:
// java return cmp <= (double)0.0F;
// java case equal:
// java return cmp == (double)0.0F;
// java case notEqual:
// java return cmp != (double)0.0F;
// java case greaterThanOrEqual:
// java return cmp >= (double)0.0F;
// java case greaterThan:
// java return cmp > (double)0.0F;
// java default:
// java return false;
// java }
// java }
pub fn numericalCompare(cmp:f64,cp:CellIsOperatorType)->bool {
use CellIsOperatorType::*;match cp{lessThan=>cmp<0.0,lessThanOrEqual=>cmp<=0.0,equal=>cmp==0.0,notEqual=>cmp!=0.0,greaterThanOrEqual=>cmp>=0.0,greaterThan=>cmp>0.0,_=>false}
}
// java private boolean numericalCompare2(double v, double min, double max, KmoRuleCellIs.CellIsOperatorType cp) {
// java if (cp == KmoRuleCellIs.CellIsOperatorType.between) {
// java return v >= min && v <= max;
// java } else if (cp != KmoRuleCellIs.CellIsOperatorType.notBetween) {
// java return false;
// java } else {
// java return v < min || v > max;
// java }
// java }
pub fn numericalCompare2(v:f64,min:f64,max:f64,cp:CellIsOperatorType)->bool {
if cp==CellIsOperatorType::between{v>=min&&v<=max}else if cp==CellIsOperatorType::notBetween{v<min||v>max}else{false}
}
// java private boolean satisfy07Rule(KmoSheet sheet, KmoCfRule rule, int row, int col, KmoConditionFormatter.CfRuleResult result) {
// java boolean fSatisfy = false;
// java
// java try {
// java Rule.CfRuleTypes type = rule.ruleType();
// java if (this.is07SupportedFmlType(type)) {
// java int cellValueType = sheet.getCellValueType(row, col);
// java if (cellValueType == 1) {
// java switch (type) {
// java case colorScale:
// java if (result.colorScaleValue == null) {
// java fSatisfy = this.judgeColorScale(rule, result, sheet, row, col);
// java }
// java break;
// java case dataBar:
// java if (result.dataBarValue == null) {
// java fSatisfy = this.judgeDataBar(rule, result, sheet, row, col);
// java }
// java break;
// java case iconSet:
// java if (result.iconSetValue == null) {
// java fSatisfy = this.judgeIconSet(rule, result, sheet, row, col);
// java }
// java break;
// java default:
// java fSatisfy = false;
// java }
// java }
// java }
// java } catch (Exception var9) {
// java fSatisfy = false;
// java }
// java
// java return fSatisfy;
// java }
pub fn satisfy07Rule(&self,rule:&KmoCfRule,row:i32,col:i32,result:&mut CfRuleResult)->bool {
std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
let sheet=self.mSheet.borrow().as_ref().unwrap().clone();let sheet=sheet.borrow();if sheet.getCellValueType(row,col,None)!=1{return false;}match rule.ruleType(){Some(CfRuleTypes::colorScale)if result.colorScaleValue.is_none()=>self.judgeColorScale(rule,result,&sheet,row,col).unwrap_or(false),Some(CfRuleTypes::dataBar)if result.dataBarValue.is_none()=>self.judgeDataBar(rule,result,&sheet,row,col).unwrap_or(false),Some(CfRuleTypes::iconSet)if result.iconSetValue.is_none()=>self.judgeIconSet(rule,result,&sheet,row,col).unwrap_or(false),_=>false}
})).unwrap_or(false)
}
fn cfvo_value(&self,c:&KmoCfvo,sheet:&KmoSheet,row:i32,col:i32)->EvalResult<f64>{let p=c.formula();if let Some(p)=p.filter(|p|!p.is_empty()){let v=self.evalulateFml(sheet,row,col,&p)?.ok_or(EvalFailure::Runtime("null cfvo eval"))?;v.as_any().downcast_ref::<NumberEval>().map(|v|v.getNumberValue()).ok_or(EvalFailure::Runtime("non-number cfvo formula"))}else{Ok(c.value())}}
// java private boolean judgeDataBar(KmoCfRule rule, KmoConditionFormatter.CfRuleResult result, KmoSheet sheet, int row, int col) {
// java double cellValue = sheet.getCellNumericValue(row, col);
// java ConFmtRangeData rangesData = new ConFmtRangeData(rule.ranges(), sheet, cellValue);
// java double min = rangesData.min;
// java double max = rangesData.max;
// java DoubleList dl = rangesData.dl;
// java KmoDataBar dataBar = (KmoDataBar)rule.getRule();
// java List<Double> cfValue = new ArrayList();
// java if (dataBar != null) {
// java result.dataBarValue = new CfDataBarValue();
// java result.dataBarValue.setPriority(rule.getRule().getPriority());
// java List<KmoCfvo> cfvoList = dataBar.cfvos();
// java KmoCfvo cfvo = null;
// java double value = (double)0.0F;
// java int i = 0;
// java
// java for(int size = cfvoList.size(); i < size; ++i) {
// java cfvo = (KmoCfvo)cfvoList.get(i);
// java Ptg[] ptgs = cfvo.formula();
// java if (ptgs != null && ptgs.length != 0) {
// java IValueEval valueEval = this.evalulateFml(sheet, row, col, ptgs);
// java if (!(valueEval instanceof NumberEval)) {
// java return false;
// java }
// java
// java value = ((NumberEval)valueEval).getNumberValue();
// java }
// java
// java if (ptgs == null || ptgs.length == 0) {
// java value = cfvo.value();
// java }
// java
// java switch (cfvo.type) {
// java case max:
// java case autoMax:
// java value = max;
// java break;
// java case min:
// java case autoMin:
// java value = min;
// java break;
// java case percent:
// java value = value / (double)100.0F * (max - min) + min;
// java break;
// java case percentile:
// java value = Percentile.percentile(dl, value / (double)100.0F);
// java }
// java
// java cfValue.add(value);
// java }
// java
// java min = (Double)cfValue.get(0);
// java max = (Double)cfValue.get(cfValue.size() - 1);
// java i = 0;
// java if (max == min) {
// java i = cellValue > max ? 100 : (cellValue == max ? 50 : 0);
// java } else {
// java i = (int)((cellValue - min) / (max - min) * (double)100.0F);
// java }
// java
// java result.dataBarValue.setLength(this.getDataBarLen(i, dataBar));
// java result.dataBarValue.setShowValue(dataBar.isShowValue());
// java int color = ((KmoCfColor)dataBar.cfColors().get(0)).getColor();
// java result.dataBarValue.setColor(color);
// java }
// java
// java return result.dataBarValue != null;
// java }
pub fn judgeDataBar(&self,rule:&KmoCfRule,result:&mut CfRuleResult,sheet:&KmoSheet,row:i32,col:i32)->EvalResult<bool> {
let cell=sheet.getCellNumericValue(row,col);let mut data=ConFmtRangeData::new(&rule.ranges().unwrap(),sheet,cell);let base=rule.getRule();let bar=base.as_any().downcast_ref::<KmoDataBar>().ok_or(EvalFailure::Runtime("ClassCastException dataBar"))?;let out=Rc::new(CfDataBarValue::new());out.setPriority(base.getPriority());result.dataBarValue=Some(out.clone());let mut values=vec![];for cfvo in bar.cfvos().borrow().iter(){let mut value=self.cfvo_value(cfvo,sheet,row,col)?;value=match cfvo.rtype.get().ok_or(EvalFailure::Runtime("null cfvo type"))?{CfvoTypes::max|CfvoTypes::autoMax=>data.max,CfvoTypes::min|CfvoTypes::autoMin=>data.min,CfvoTypes::percent=>value/100.0*(data.max-data.min)+data.min,CfvoTypes::percentile=>Percentile::percentile(&mut data.dl,value/100.0),_=>value};values.push(value);}let min=*values.first().ok_or(EvalFailure::Runtime("IndexOutOfBoundsException"))?;let max=*values.last().unwrap();let i=if max==min{if cell>max{100}else if cell==max{50}else{0}}else{((cell-min)/(max-min)*100.0)as i32};out.setLength(Self::getDataBarLen(i,bar));out.setShowValue(bar.isShowValue());let colors=bar.cfColors();let colors=colors.borrow();out.setColor(colors.first().ok_or(EvalFailure::Runtime("IndexOutOfBoundsException color"))?.borrow().getColor());Ok(true)
}
// java private int getDataBarLen(int length, KmoDataBar dataBar) {
// java length = length < 0 ? 0 : (length > 100 ? 100 : length);
// java int result = dataBar.minLength + length * (dataBar.maxLength - dataBar.minLength) / 100;
// java return result;
// java }
pub fn getDataBarLen(length:i32,bar:&KmoDataBar)->i32 {
let len=length.clamp(0,100);bar.minLength.get().wrapping_add(len.wrapping_mul(bar.maxLength.get().wrapping_sub(bar.minLength.get()))/100)
}
// java private boolean judgeColorScale(KmoCfRule rule, KmoConditionFormatter.CfRuleResult result, KmoSheet sheet, int row, int col) {
// java double cellValue = sheet.getCellNumericValue(row, col);
// java ConFmtRangeData rangesData = new ConFmtRangeData(rule.ranges(), sheet, cellValue);
// java double min = rangesData.min;
// java double max = rangesData.max;
// java DoubleList dl = rangesData.dl;
// java KmoColorScale colorScale = (KmoColorScale)rule.getRule();
// java if (colorScale != null) {
// java result.colorScaleValue = new CfColorScaleValue();
// java result.colorScaleValue.setPriority(rule.getRule().getPriority());
// java List<KmoCfvo> cfvoList = colorScale.cfvos();
// java List<Double> cfValue = new ArrayList();
// java KmoCfvo cfvo = null;
// java double value = (double)0.0F;
// java int i = 0;
// java
// java for(int size = cfvoList.size(); i < size; ++i) {
// java cfvo = (KmoCfvo)cfvoList.get(i);
// java Ptg[] ptgs = cfvo.formula();
// java if (ptgs != null && ptgs.length != 0) {
// java IValueEval valueEval = this.evalulateFml(sheet, row, col, ptgs);
// java if (!(valueEval instanceof NumberEval)) {
// java return false;
// java }
// java
// java value = ((NumberEval)valueEval).getNumberValue();
// java }
// java
// java if (ptgs == null || ptgs.length == 0) {
// java value = cfvo.value();
// java }
// java
// java switch (cfvo.type) {
// java case max:
// java case autoMax:
// java value = max;
// java break;
// java case min:
// java case autoMin:
// java value = min;
// java break;
// java case percent:
// java value = value / (double)100.0F * (max - min) + min;
// java break;
// java case percentile:
// java value = Percentile.percentile(dl, value / (double)100.0F);
// java }
// java
// java cfValue.add(value);
// java }
// java
// java Collections.sort(cfValue);
// java i = cfValue.size();
// java
// java int colorIndex;
// java for(colorIndex = i - 1; colorIndex >= 0; --colorIndex) {
// java double ruleVal = (Double)cfValue.get(colorIndex);
// java if (cellValue >= ruleVal) {
// java if (i == 3 && colorIndex == 1 && ruleVal == (Double)cfValue.get(colorIndex - 1) && ruleVal == cellValue) {
// java --colorIndex;
// java }
// java break;
// java }
// java }
// java
// java result.colorScaleValue.setColor(this.getColorScaleRGB(colorScale.cfcolors(), colorIndex, cellValue, cfValue));
// java }
// java
// java return result.colorScaleValue != null;
// java }
pub fn judgeColorScale(&self,rule:&KmoCfRule,result:&mut CfRuleResult,sheet:&KmoSheet,row:i32,col:i32)->EvalResult<bool> {
let cell=sheet.getCellNumericValue(row,col);let mut data=ConFmtRangeData::new(&rule.ranges().unwrap(),sheet,cell);let base=rule.getRule();let scale=base.as_any().downcast_ref::<KmoColorScale>().ok_or(EvalFailure::Runtime("ClassCastException colorScale"))?;let out=Rc::new(CfColorScaleValue::new());out.setPriority(base.getPriority());result.colorScaleValue=Some(out.clone());let mut values=vec![];for cfvo in scale.cfvos().borrow().iter(){let mut value=self.cfvo_value(cfvo,sheet,row,col)?;value=match cfvo.rtype.get().ok_or(EvalFailure::Runtime("null cfvo type"))?{CfvoTypes::max|CfvoTypes::autoMax=>data.max,CfvoTypes::min|CfvoTypes::autoMin=>data.min,CfvoTypes::percent=>value/100.0*(data.max-data.min)+data.min,CfvoTypes::percentile=>Percentile::percentile(&mut data.dl,value/100.0),_=>value};values.push(value);}values.sort_by(|a,b|crate::cn::wps::et::ss::formula::functions::util::matcher::ConditionMatchPredicate::doubleCompare(*a,*b).cmp(&0));let size=values.len();let mut index=size as i32-1;while index>=0{let v=values[index as usize];if cell>=v{if size==3&&index==1&&v==values[0]&&v==cell{index-=1;}break;}index-=1;}out.setColor(Self::getColorScaleRGB(&scale.cfcolors().borrow(),index,cell,&values)?);Ok(true)
}
// java private int getColorScaleRGB(List<KmoCfColor> colorList, int colorIndex, double cellValue, List<Double> cfValueList) {
// java KmoCfColor curColor = null;
// java KmoCfColor preColor = null;
// java double preValue = (double)0.0F;
// java double nextValue = (double)0.0F;
// java int len = colorList.size();
// java if (colorIndex >= 0 && colorIndex < len - 1) {
// java curColor = (KmoCfColor)colorList.get(colorIndex + 1);
// java preColor = (KmoCfColor)colorList.get(colorIndex);
// java preValue = (Double)cfValueList.get(colorIndex);
// java nextValue = (Double)cfValueList.get(colorIndex + 1);
// java } else {
// java if (colorIndex < 0) {
// java colorIndex = 0;
// java }
// java
// java curColor = (KmoCfColor)colorList.get(colorIndex);
// java preColor = curColor.clone();
// java preValue = nextValue = (Double)cfValueList.get(colorIndex);
// java }
// java
// java int curRGB = curColor.getColor();
// java int preRGB = preColor.getColor();
// java int preRed = preRGB >> 16 & 255;
// java int preGreen = preRGB >> 8 & 255;
// java int preBlue = preRGB & 255;
// java int curRed = curRGB >> 16 & 255;
// java int curGreen = curRGB >> 8 & 255;
// java int curBlue = curRGB & 255;
// java double ratio = preValue == nextValue ? (double)0.0F : (cellValue - preValue) / (nextValue - preValue);
// java return AppLogicHelper.rgb(preRed + (int)((double)(curRed - preRed) * ratio), preGreen + (int)((double)(curGreen - preGreen) * ratio), preBlue + (int)((double)(curBlue - preBlue) * ratio));
// java }
pub fn getColorScaleRGB(colors:&[Rc<RefCell<KmoCfColor>>],mut index:i32,cell:f64,values:&[f64])->EvalResult<i32> {
let size=colors.len()as i32;let(cur,pre,pv,nv)=if index>=0&&index<size-1{(colors[index as usize+1].borrow().getColor(),colors[index as usize].borrow().getColor(),values[index as usize],values[index as usize+1])}else{if index<0{index=0;}let c=colors.get(index as usize).ok_or(EvalFailure::Runtime("IndexOutOfBoundsException"))?.borrow().getColor();let v=*values.get(index as usize).ok_or(EvalFailure::Runtime("IndexOutOfBoundsException"))?;(c,c,v,v)};let ratio=if pv==nv{0.0}else{(cell-pv)/(nv-pv)};let channel=|shift:u32|{let a=(pre>>shift)&255;let b=(cur>>shift)&255;a.wrapping_add(((b-a)as f64*ratio)as i32)};Ok(-16777216|channel(16).wrapping_shl(16)|channel(8).wrapping_shl(8)|channel(0))
}
// java private boolean judgeIconSet(KmoCfRule rule, KmoConditionFormatter.CfRuleResult result, KmoSheet sheet, int row, int col) {
// java double cellValue = sheet.getCellNumericValue(row, col);
// java ConFmtRangeData rangesData = new ConFmtRangeData(rule.ranges(), sheet, cellValue);
// java double min = rangesData.min;
// java double max = rangesData.max;
// java DoubleList dl = rangesData.dl;
// java KmoIconSet iconSet = (KmoIconSet)rule.getRule();
// java if (iconSet != null) {
// java result.iconSetValue = new CfIconSetValue();
// java result.iconSetValue.setPriority(rule.getRule().getPriority());
// java List<KmoCfvo> cfvoList = iconSet.cfvos();
// java KmoCfvo cfvo = null;
// java double value = (double)0.0F;
// java int iconIndex = -1;
// java int size = cfvoList.size() - 1;
// java
// java for(int i = size; i >= 0; --i) {
// java cfvo = (KmoCfvo)cfvoList.get(i);
// java Ptg[] ptgs = cfvo.formula();
// java if (ptgs != null && ptgs.length != 0) {
// java IValueEval valueEval = this.evalulateFml(sheet, row, col, ptgs);
// java if (!(valueEval instanceof NumberEval)) {
// java return false;
// java }
// java
// java value = ((NumberEval)valueEval).getNumberValue();
// java }
// java
// java if (ptgs == null || ptgs.length == 0) {
// java value = cfvo.value();
// java }
// java
// java switch (cfvo.type) {
// java case percent:
// java value = value / (double)100.0F * (max - min) + min;
// java break;
// java case percentile:
// java value = Percentile.percentile(dl, value / (double)100.0F);
// java }
// java
// java if (cfvo.isEqual()) {
// java if (cellValue >= value) {
// java iconIndex = i;
// java break;
// java }
// java } else if (cellValue > value) {
// java iconIndex = i;
// java break;
// java }
// java }
// java
// java if (iconIndex == -1) {
// java iconIndex = 0;
// java }
// java
// java iconIndex = iconSet.isReverse() ? iconIndex + 1 : size - iconIndex + 1;
// java result.iconSetValue.setIndex(iconIndex);
// java result.iconSetValue.setType(iconSet.getIconSetType());
// java result.iconSetValue.setShowValue(iconSet.isShowValue());
// java }
// java
// java return result.iconSetValue != null;
// java }
pub fn judgeIconSet(&self,rule:&KmoCfRule,result:&mut CfRuleResult,sheet:&KmoSheet,row:i32,col:i32)->EvalResult<bool> {
let cell=sheet.getCellNumericValue(row,col);let mut data=ConFmtRangeData::new(&rule.ranges().unwrap(),sheet,cell);let base=rule.getRule();let icon=base.as_any().downcast_ref::<KmoIconSet>().ok_or(EvalFailure::Runtime("ClassCastException iconSet"))?;let out=Rc::new(CfIconSetValue::new());out.setPriority(base.getPriority());result.iconSetValue=Some(out.clone());let list=icon.cfvos();let list=list.borrow();let size=list.len()as i32-1;let mut index= -1;for i in(0..list.len()).rev(){let c=&list[i];let mut value=self.cfvo_value(c,sheet,row,col)?;value=match c.rtype.get().ok_or(EvalFailure::Runtime("null cfvo type"))?{CfvoTypes::percent=>value/100.0*(data.max-data.min)+data.min,CfvoTypes::percentile=>Percentile::percentile(&mut data.dl,value/100.0),_=>value};if if c.isEqual(){cell>=value}else{cell>value}{index=i as i32;break;}}if index== -1{index=0;}let index=if icon.isReverse(){index+1}else{size-index+1};out.setIndex(index);out.setType(icon.getIconSetType());out.setShowValue(icon.isShowValue());Ok(true)
}
}
struct ConFmtRangeData{max:f64,min:f64,dl:DoubleList}impl ConFmtRangeData{
// java public ConFmtRangeData(KmoRect[] ranges, KmoSheet sheet, double tempValue) {
// java this.calculateConFmtRange(ranges, sheet, tempValue);
// java }
pub fn new(ranges:&[KmoRect],sheet:&KmoSheet,temp:f64)->Self {
let mut d=Self{max:temp,min:temp,dl:DoubleList::new()};d.calculateConFmtRange(ranges,sheet,temp);d
}
// java private void calculateConFmtRange(KmoRect[] rectRangeList, KmoSheet sheet, double tempValue) {
// java this.max = this.min = tempValue;
// java
// java for(KmoRect kmoRect : rectRangeList) {
// java KmoSheet.CellIterator cellIterator = sheet.getCellIterator(kmoRect);
// java
// java while(cellIterator.hasNext()) {
// java cellIterator.next();
// java int row = cellIterator.row();
// java int col = cellIterator.col();
// java if (sheet.getCellValueType(row, col) == 1) {
// java double cellValue = sheet.getCellNumericValue(row, col);
// java if (cellValue > this.max) {
// java this.max = cellValue;
// java } else if (cellValue < this.min) {
// java this.min = cellValue;
// java }
// java
// java this.dl.add(cellValue);
// java }
// java }
// java }
// java
// java }
pub fn calculateConFmtRange(&mut self,ranges:&[KmoRect],sheet:&KmoSheet,temp:f64) {
self.max=temp;self.min=temp;for rect in ranges{let mut it=sheet.getCellIterator(rect.tl.row,rect.br.row,rect.tl.col,rect.br.col);while it.hasNext(){it.next();let row=it.row();let col=it.col();if sheet.getCellValueType(row,col,None)==1{let v=sheet.getCellNumericValue(row,col);if v>self.max{self.max=v;}else if v<self.min{self.min=v;}self.dl.add(v);}}}
}
}
