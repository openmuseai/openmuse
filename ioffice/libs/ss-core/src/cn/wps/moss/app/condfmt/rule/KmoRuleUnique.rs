use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use super::Rule::{Rule,RuleObject,CfRuleTypes};
pub struct KmoRuleUnique{base:Rule}
impl std::ops::Deref for KmoRuleUnique{type Target=Rule;fn deref(&self)->&Rule{&self.base}}
impl RuleObject for KmoRuleUnique{fn base(&self)->&Rule{&self.base}fn as_any(&self)->&dyn std::any::Any{self}}
impl KmoRuleUnique{
// java public KmoRuleUnique(Rule.CfRuleTypes type, SpreadsheetVersion sv) {
// java this(sv);
// java this.setType(type);
// java }
pub fn new(ctype:CfRuleTypes,sv:SpreadsheetVersion)->Self {
let base=Rule::new(sv);base.setType(Some(ctype));Self{base}
}
}
