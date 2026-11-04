use crate::{java_runtime::{Rc,RefCell,Cell},cn::wps::moss::util::KmoRect::KmoRect};use super::KmoCfRule::KmoCfRule;
// java public class KmoConditionFormatterDefine
pub struct KmoConditionFormatterDefine{pub ruleList:RefCell<Vec<Rc<KmoCfRule>>>,refRangeList:RefCell<Rc<Vec<KmoRect>>>,enclosingRange:RefCell<Option<KmoRect>>,pub pivot:Cell<bool>}
impl KmoConditionFormatterDefine{
// java public KmoConditionFormatterDefine() {
// java }
pub fn new()->Self {
Self{ruleList:RefCell::new(vec![]),refRangeList:RefCell::new(Rc::new(vec![])),enclosingRange:RefCell::new(None),pivot:Cell::new(false)}
}
// java public void addCfRule(KmoCfRule cfRule) {
// java this.ruleList.add(cfRule);
// java }
pub fn addCfRule(&self,cfRule:Rc<KmoCfRule>) {
self.ruleList.borrow_mut().push(cfRule);
}
// java public List<KmoRect> getRefRangeList() {
// java return this.refRangeList;
// java }
pub fn getRefRangeList(&self)->Rc<Vec<KmoRect>> {
self.refRangeList.borrow().clone()
}
// java public void setRefRangeList(List<KmoRect> refRangeList) {
// java this.refRangeList = refRangeList;
// java this.createEnclosingRange();
// java }
pub fn setRefRangeList(&self,ranges:Rc<Vec<KmoRect>>) {
*self.refRangeList.borrow_mut()=ranges;self.createEnclosingRange();
}
// java public KmoRect getEnclosingRange() {
// java if (this.enclosingRange == null) {
// java this.createEnclosingRange();
// java }
// java
// java return this.enclosingRange;
// java }
pub fn getEnclosingRange(&self)->KmoRect {
if self.enclosingRange.borrow().is_none(){self.createEnclosingRange();}self.enclosingRange.borrow().clone().unwrap()
}
// java private void createEnclosingRange() {
// java KmoRect rect = new KmoRect((KmoRect)this.getRefRangeList().get(0));
// java
// java for(KmoRect cellAddr : this.getRefRangeList()) {
// java rect.combine(cellAddr);
// java }
// java
// java this.enclosingRange = rect;
// java }
pub fn createEnclosingRange(&self) {
let ranges=self.getRefRangeList();let mut r=ranges[0].clone();for a in ranges.iter(){r.combine(a);}*self.enclosingRange.borrow_mut()=Some(r);
}
}
