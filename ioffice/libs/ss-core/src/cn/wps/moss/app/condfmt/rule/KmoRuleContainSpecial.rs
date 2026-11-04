use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use super::Rule::{Rule,RuleObject,CfRuleTypes};
pub struct KmoRuleContainSpecial{base:Rule}
impl std::ops::Deref for KmoRuleContainSpecial{type Target=Rule;fn deref(&self)->&Rule{&self.base}}
impl RuleObject for KmoRuleContainSpecial{fn base(&self)->&Rule{&self.base}fn as_any(&self)->&dyn std::any::Any{self}}
impl KmoRuleContainSpecial{
// java public KmoRuleContainSpecial(Rule.CfRuleTypes type, SpreadsheetVersion sv) {
// java this(sv);
// java this.setType(type);
// java }
pub fn new(ctype:CfRuleTypes,sv:SpreadsheetVersion)->Self {
let base=Rule::new(sv);base.setType(Some(ctype));Self{base}
}
}
