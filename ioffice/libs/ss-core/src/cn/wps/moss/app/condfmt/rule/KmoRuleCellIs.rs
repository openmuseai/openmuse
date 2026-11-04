use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use super::Rule::{Rule,RuleObject,CfRuleTypes};
// java public static enum CellIsOperatorType {
// java noComparsion,
// java lessThan,
// java lessThanOrEqual,
// java equal,
// java notEqual,
// java greaterThanOrEqual,
// java greaterThan,
// java between,
// java notBetween;
// java }
// java }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CellIsOperatorType{noComparsion,lessThan,lessThanOrEqual,equal,notEqual,greaterThanOrEqual,greaterThan,between,notBetween}
impl CellIsOperatorType{pub fn valueOf(s:&str)->Option<Self>{match s{"noComparsion"=>Some(Self::noComparsion),"lessThan"=>Some(Self::lessThan),"lessThanOrEqual"=>Some(Self::lessThanOrEqual),"equal"=>Some(Self::equal),"notEqual"=>Some(Self::notEqual),"greaterThanOrEqual"=>Some(Self::greaterThanOrEqual),"greaterThan"=>Some(Self::greaterThan),"between"=>Some(Self::between),"notBetween"=>Some(Self::notBetween),_=>None}}}
// java public class KmoRuleCellIs extends Rule
pub struct KmoRuleCellIs {base:Rule,cellIsOpType:Cell<Option<CellIsOperatorType>>}
impl std::ops::Deref for KmoRuleCellIs {type Target=Rule;fn deref(&self)->&Rule{&self.base}}
impl RuleObject for KmoRuleCellIs {fn base(&self)->&Rule{&self.base}fn as_any(&self)->&dyn std::any::Any{self}}
impl KmoRuleCellIs {
// java public KmoRuleCellIs(SpreadsheetVersion sv) {
// java super(sv);
// java }
pub fn new(sv:SpreadsheetVersion)->Self {
let base=Rule::new(sv);base.setType(Some(CfRuleTypes::cellIs));Self{base,cellIsOpType:Cell::new(None)}
}
// java public CellIsOperatorType getCellIsOpType() {
// java return this.cellIsOpType;
// java }
pub fn getCellIsOpType(&self)->Option<CellIsOperatorType> {
self.cellIsOpType.get()
}
// java public void setCellIsOpType(CellIsOperatorType cellIsOpType) {
// java this.cellIsOpType = cellIsOpType;
// java }
pub fn setCellIsOpType(&self,v:Option<CellIsOperatorType>) {
self.cellIsOpType.set(v);
}
}
