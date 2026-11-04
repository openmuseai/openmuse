use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use super::Rule::{Rule,RuleObject,CfRuleTypes};
// java public class KmoRuleTop extends Rule
pub struct KmoRuleTop {base:Rule,percent:Cell<bool>,bottom:Cell<bool>,rank:Cell<i32>}
impl std::ops::Deref for KmoRuleTop {type Target=Rule;fn deref(&self)->&Rule{&self.base}}
impl RuleObject for KmoRuleTop {fn base(&self)->&Rule{&self.base}fn as_any(&self)->&dyn std::any::Any{self}}
impl KmoRuleTop {
// java public KmoRuleTop(SpreadsheetVersion sv) {
// java super(sv);
// java this.setType(Rule.CfRuleTypes.top10);
// java }
pub fn new(sv:SpreadsheetVersion)->Self {
let base=Rule::new(sv);base.setType(Some(CfRuleTypes::top10));Self{base,percent:Cell::new(false),bottom:Cell::new(false),rank:Cell::new(-1)}
}
// java public boolean isPercent() {
// java return this.percent;
// java }
pub fn isPercent(&self)->bool {
self.percent.get()
}
// java public void setPercent(boolean percent) {
// java this.percent = percent;
// java }
pub fn setPercent(&self,v:bool) {
self.percent.set(v);
}
// java public boolean isBottom() {
// java return this.bottom;
// java }
pub fn isBottom(&self)->bool {
self.bottom.get()
}
// java public void setBottom(boolean bottom) {
// java this.bottom = bottom;
// java }
pub fn setBottom(&self,v:bool) {
self.bottom.set(v);
}
// java public int getRank() {
// java return this.rank;
// java }
pub fn getRank(&self)->i32 {
self.rank.get()
}
// java public void setRank(int rank) {
// java this.rank = rank;
// java }
pub fn setRank(&self,v:i32) {
self.rank.set(v);
}
}
