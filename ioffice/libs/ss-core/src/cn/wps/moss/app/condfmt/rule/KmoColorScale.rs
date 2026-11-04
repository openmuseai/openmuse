use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use super::Rule::{Rule,RuleObject,CfRuleTypes};
use super::super::{KmoCfvo::KmoCfvo,KmoCfColor::KmoCfColor};
// java public class KmoColorScale extends Rule implements Cloneable
pub struct KmoColorScale{base:Rule,colorList:RefCell<Rc<RefCell<Vec<Rc<RefCell<KmoCfColor>>>>>>,cfvoList:RefCell<Rc<RefCell<Vec<Rc<KmoCfvo>>>>>}
impl std::ops::Deref for KmoColorScale{type Target=Rule;fn deref(&self)->&Rule{&self.base}}
impl RuleObject for KmoColorScale{fn javaEquals(&self,o:&dyn RuleObject)->bool{o.as_any().downcast_ref::<Self>().is_some_and(|o|self.equals(o))}fn javaHashCode(&self)->i32{self.hashCode()}fn base(&self)->&Rule{&self.base}fn as_any(&self)->&dyn std::any::Any{self}}
impl KmoColorScale{
// java public KmoColorScale(SpreadsheetVersion sv) {
// java super(sv);
// java this.setType(Rule.CfRuleTypes.colorScale);
// java this.cfvoList = new ArrayList();
// java this.colorList = new ArrayList();
// java }
pub fn new(sv:SpreadsheetVersion)->Self {
let base=Rule::new(sv);base.setType(Some(CfRuleTypes::colorScale));Self{base,colorList:RefCell::new(Rc::new(RefCell::new(vec![]))),cfvoList:RefCell::new(Rc::new(RefCell::new(vec![])))}
}
// java public List<KmoCfvo> cfvos() {
// java return this.cfvoList;
// java }
pub fn cfvos(&self)->Rc<RefCell<Vec<Rc<KmoCfvo>>>> {
self.cfvoList.borrow().clone()
}
// java public void addCfvo(KmoCfvo cfvo) {
// java this.cfvoList.add(cfvo);
// java }
pub fn addCfvo(&self,v:Rc<KmoCfvo>) {
self.cfvoList.borrow().borrow_mut().push(v);
}
// java public void setCfvos(List<KmoCfvo> cfvos) {
// java this.cfvoList = cfvos;
// java }
pub fn setCfvos(&self,v:Rc<RefCell<Vec<Rc<KmoCfvo>>>>) {
*self.cfvoList.borrow_mut()=v;
}
// java public List<Ptg[]> formulas() {
// java List<Ptg[]> ptgs = new ArrayList();
// java
// java for(KmoCfvo cfvo : this.cfvoList) {
// java Ptg[] formula = cfvo.formula();
// java if (formula != null) {
// java ptgs.add(formula);
// java }
// java }
// java
// java return ptgs;
// java }
pub fn formulas(&self)->Vec<Rc<Vec<Ptg>>> {
self.cfvos().borrow().iter().filter_map(|v|v.formula()).collect()
}
// java public List<KmoCfColor> cfcolors() {
// java return this.colorList;
// java }
pub fn cfcolors(&self)->Rc<RefCell<Vec<Rc<RefCell<KmoCfColor>>>>> {
self.colorList.borrow().clone()
}
// java public void addColor(KmoCfColor color) {
// java this.colorList.add(color);
// java }
pub fn addColor(&self,v:Rc<RefCell<KmoCfColor>>) {
self.colorList.borrow().borrow_mut().push(v);
}
// java public void setCfColors(List<KmoCfColor> colors) {
// java this.colorList = colors;
// java }
pub fn setCfColors(&self,v:Rc<RefCell<Vec<Rc<RefCell<KmoCfColor>>>>>) {
*self.colorList.borrow_mut()=v;
}
}

impl KmoColorScale{// java public boolean equals(Object obj) {
// java if (this == obj) {
// java return true;
// java } else if (!super.equals(obj)) {
// java return false;
// java } else if (this.getClass() != obj.getClass()) {
// java return false;
// java } else {
// java KmoColorScale other = (KmoColorScale)obj;
// java if (this.cfvoList == null) {
// java if (other.cfvoList != null) {
// java return false;
// java }
// java } else if (!this.cfvoList.equals(other.cfvoList)) {
// java return false;
// java }
// java
// java if (this.colorList == null) {
// java if (other.colorList != null) {
// java return false;
// java }
// java } else if (!this.colorList.equals(other.colorList)) {
// java return false;
// java }
// java
// java return true;
// java }
// java }
pub fn equals(&self,o:&Self)->bool{std::ptr::eq(self,o)||(self.base.equals(&o.base)&&{let a=self.cfvos();let b=o.cfvos();let a=a.borrow();let b=b.borrow();a.len()==b.len()&&a.iter().zip(b.iter()).all(|(a,b)|a.equals(b))}&&{let a=self.cfcolors();let b=o.cfcolors();let a=a.borrow();let b=b.borrow();a.len()==b.len()&&a.iter().zip(b.iter()).all(|(a,b)|a.borrow().equals(&b.borrow()))})}// java public int hashCode() {
// java int prime = 31;
// java int result = 1;
// java result = 31 * result + (this.cfvoList == null ? 0 : this.cfvoList.hashCode());
// java result = 31 * result + (this.colorList == null ? 0 : this.colorList.hashCode());
// java return result;
// java }
pub fn hashCode(&self)->i32{use super::super::java_values::{fold,boolean};fold([fold(self.cfvos().borrow().iter().map(|v|v.hashCode())),fold(self.cfcolors().borrow().iter().map(|v|v.borrow().hashCode()))])}}
