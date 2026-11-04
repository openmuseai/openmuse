use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use super::Rule::{Rule,RuleObject,CfRuleTypes};
// java public static enum SpecialTextOperatorType {
// java containsText,
// java notContains,
// java beginsWith,
// java endsWith;
// java }
// java }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SpecialTextOperatorType{containsText,notContains,beginsWith,endsWith}
impl SpecialTextOperatorType{pub fn valueOf(s:&str)->Option<Self>{match s{"containsText"=>Some(Self::containsText),"notContains"=>Some(Self::notContains),"beginsWith"=>Some(Self::beginsWith),"endsWith"=>Some(Self::endsWith),_=>None}}}
// java public class KmoRuleSpecialText extends Rule
pub struct KmoRuleSpecialText {base:Rule,text:RefCell<String>,opType:Cell<Option<SpecialTextOperatorType>>}
impl std::ops::Deref for KmoRuleSpecialText {type Target=Rule;fn deref(&self)->&Rule{&self.base}}
impl RuleObject for KmoRuleSpecialText {fn base(&self)->&Rule{&self.base}fn as_any(&self)->&dyn std::any::Any{self}}
impl KmoRuleSpecialText {
// java public KmoRuleSpecialText(SpreadsheetVersion sv) {
// java super(sv);
// java this.text = "";
// java }
pub fn new(sv:SpreadsheetVersion)->Self {
let base=Rule::new(sv);base.setType(Some(CfRuleTypes::containsText));Self{base,text:RefCell::new(String::new()),opType:Cell::new(None)}
}
// java public String getText() {
// java return this.text;
// java }
pub fn getText(&self)->String {
self.text.borrow().clone()
}
// java public void setText(String text) {
// java this.text = text;
// java }
pub fn setText(&self,v:String) {
*self.text.borrow_mut()=v;
}
// java public SpecialTextOperatorType getOpType() {
// java return this.opType;
// java }
pub fn getOpType(&self)->Option<SpecialTextOperatorType> {
self.opType.get()
}
// java public void setOpType(SpecialTextOperatorType opType) {
// java this.opType = opType;
// java }
pub fn setOpType(&self,v:Option<SpecialTextOperatorType>) {
self.opType.set(v);
}
}
