use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use super::Rule::{Rule,RuleObject,CfRuleTypes};
// java public class KmoRuleAverage extends Rule
pub struct KmoRuleAverage {base:Rule,aboveAverage:Cell<bool>,equalAverage:Cell<bool>,stdDev:Cell<i32>}
impl std::ops::Deref for KmoRuleAverage {type Target=Rule;fn deref(&self)->&Rule{&self.base}}
impl RuleObject for KmoRuleAverage {fn base(&self)->&Rule{&self.base}fn as_any(&self)->&dyn std::any::Any{self}}
impl KmoRuleAverage {
// java public KmoRuleAverage(SpreadsheetVersion sv) {
// java super(sv);
// java this.aboveAverage = true;
// java this.equalAverage = false;
// java this.stdDev = Integer.MAX_VALUE;
// java }
pub fn new(sv:SpreadsheetVersion)->Self {
let base=Rule::new(sv);base.setType(Some(CfRuleTypes::aboveAverage));Self{base,aboveAverage:Cell::new(true),equalAverage:Cell::new(false),stdDev:Cell::new(i32::MAX)}
}
// java public boolean isAboveAverage() {
// java return this.aboveAverage;
// java }
pub fn isAboveAverage(&self)->bool {
self.aboveAverage.get()
}
// java public void setAboveAverage(boolean aboveAverage) {
// java this.aboveAverage = aboveAverage;
// java }
pub fn setAboveAverage(&self,v:bool) {
self.aboveAverage.set(v);
}
// java public boolean isEqualAverage() {
// java return this.equalAverage;
// java }
pub fn isEqualAverage(&self)->bool {
self.equalAverage.get()
}
// java public void setEqualAverage(boolean equalAverage) {
// java this.equalAverage = equalAverage;
// java }
pub fn setEqualAverage(&self,v:bool) {
self.equalAverage.set(v);
}
// java public int getStdDev() {
// java return this.stdDev;
// java }
pub fn getStdDev(&self)->i32 {
self.stdDev.get()
}
// java public void setStdDev(int stdDev) {
// java this.stdDev = stdDev;
// java }
pub fn setStdDev(&self,v:i32) {
self.stdDev.set(v);
}
}
