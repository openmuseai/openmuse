use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use super::Rule::{Rule,RuleObject,CfRuleTypes};
// java public class KmoRuleFormula extends Rule
pub struct KmoRuleFormula {base:Rule,}
impl std::ops::Deref for KmoRuleFormula {type Target=Rule;fn deref(&self)->&Rule{&self.base}}
impl RuleObject for KmoRuleFormula {fn base(&self)->&Rule{&self.base}fn as_any(&self)->&dyn std::any::Any{self}}
impl KmoRuleFormula {
// java public KmoRuleFormula(SpreadsheetVersion sv) {
// java super(sv);
// java this.setType(Rule.CfRuleTypes.expression);
// java }
pub fn new(sv:SpreadsheetVersion)->Self {
let base=Rule::new(sv);base.setType(Some(CfRuleTypes::expression));Self{base,}
}
}
