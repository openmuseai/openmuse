use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use crate::cn::wps::moss::filefmt::biff8::record::cf::{FontFormatting::FontFormatting,BorderFormatting::BorderFormatting,PatternFormatting::PatternFormatting,NumberFormatFormatting::NumberFormatFormatting};
use std::any::Any;
// java public static enum CfRuleTypes {error,expression,cellIs,colorScale,dataBar,iconSet,top10,uniqueValues,duplicateValues,containsText,notContainsText,beginsWith,endsWith,containsBlanks,notContainsBlanks,containsErrors,notContainsErrors,timePeriod,aboveAverage;}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CfRuleTypes{error,expression,cellIs,colorScale,dataBar,iconSet,top10,uniqueValues,duplicateValues,containsText,notContainsText,beginsWith,endsWith,containsBlanks,notContainsBlanks,containsErrors,notContainsErrors,timePeriod,aboveAverage}
impl CfRuleTypes {pub fn valueOf(s:&str)->Option<Self>{match s {"error"=>Some(Self::error),"expression"=>Some(Self::expression),"cellIs"=>Some(Self::cellIs),"colorScale"=>Some(Self::colorScale),"dataBar"=>Some(Self::dataBar),"iconSet"=>Some(Self::iconSet),"top10"=>Some(Self::top10),"uniqueValues"=>Some(Self::uniqueValues),"duplicateValues"=>Some(Self::duplicateValues),"containsText"=>Some(Self::containsText),"notContainsText"=>Some(Self::notContainsText),"beginsWith"=>Some(Self::beginsWith),"endsWith"=>Some(Self::endsWith),"containsBlanks"=>Some(Self::containsBlanks),"notContainsBlanks"=>Some(Self::notContainsBlanks),"containsErrors"=>Some(Self::containsErrors),"notContainsErrors"=>Some(Self::notContainsErrors),"timePeriod"=>Some(Self::timePeriod),"aboveAverage"=>Some(Self::aboveAverage),_=>None}}}
// rust: Dynamic dispatch retains the Java Rule inheritance hierarchy and shared base identity.
pub trait RuleObject:Any+Send+Sync {fn javaEquals(&self,o:&dyn RuleObject)->bool{self.as_any().type_id()==o.as_any().type_id()&&self.base().equals(o.base())}fn javaHashCode(&self)->i32{self.base().hashCode()}fn base(&self)->&Rule;fn as_any(&self)->&dyn Any;}
impl std::ops::Deref for dyn RuleObject {type Target=Rule;fn deref(&self)->&Rule{self.base()}}
impl RuleObject for Rule{fn base(&self)->&Rule{self}fn as_any(&self)->&dyn Any{self}}
// java public abstract class Rule implements Cloneable
pub struct Rule {sv:SpreadsheetVersion,rtype:Cell<Option<CfRuleTypes>>,dxfId:Cell<i32>,priority:Cell<i32>,stopIfTrue:Cell<bool>,formula1:RefCell<Option<Vec<Ptg>>>,formula2:RefCell<Option<Vec<Ptg>>>,font:RefCell<Option<Rc<FontFormatting>>>,border:RefCell<Option<Rc<RefCell<BorderFormatting>>>>,pattern:RefCell<Option<Rc<RefCell<PatternFormatting>>>>,numFmt:RefCell<Option<Rc<NumberFormatFormatting>>>}
impl Rule {
// java public Rule(SpreadsheetVersion sv) {
// java this.sv = sv;
// java }
pub fn new(sv:SpreadsheetVersion)->Self {
Self{sv,rtype:Cell::new(None),dxfId:Cell::new(-1),priority:Cell::new(-1),stopIfTrue:Cell::new(false),formula1:RefCell::new(Some(vec![])),formula2:RefCell::new(Some(vec![])),font:RefCell::new(None),border:RefCell::new(None),pattern:RefCell::new(None),numFmt:RefCell::new(None)}
}
// java public CfRuleTypes type() {
// java return this.type;
// java }
pub fn ruleType(&self)->Option<CfRuleTypes> {
self.rtype.get()
}
// java public void setType(CfRuleTypes type) {
// java this.type = type;
// java }
pub fn setType(&self,v:Option<CfRuleTypes>) {
self.rtype.set(v);
}
// java public int getPriority() {
// java return this.priority;
// java }
pub fn getPriority(&self)->i32 {
self.priority.get()
}
// java public void setPriority(int priority) {
// java this.priority = priority;
// java }
pub fn setPriority(&self,v:i32) {
self.priority.set(v);
}
// java public boolean isStopIfTrue() {
// java return this.stopIfTrue;
// java }
pub fn isStopIfTrue(&self)->bool {
self.stopIfTrue.get()
}
// java public void setStopIfTrue(boolean stopIfTrue) {
// java this.stopIfTrue = stopIfTrue;
// java }
pub fn setStopIfTrue(&self,v:bool) {
self.stopIfTrue.set(v);
}
// java public int getDxfId() {
// java return this.dxfId;
// java }
pub fn getDxfId(&self)->i32 {
self.dxfId.get()
}
// java public void setDxfId(int dxfId) {
// java this.dxfId = dxfId;
// java }
pub fn setDxfId(&self,v:i32) {
self.dxfId.set(v);
}
// java public SpreadsheetVersion getVersion() {
// java return this.sv;
// java }
pub fn getVersion(&self)->SpreadsheetVersion {
self.sv
}
// java public Ptg[] formula1() {
// java return this.formula1;
// java }
pub fn formula1(&self)->Option<Vec<Ptg>> {
self.formula1.borrow().clone()
}
// java public void setFormula1(Ptg[] formula1) {
// java this.formula1 = formula1;
// java }
pub fn setFormula1(&self,v:Option<Vec<Ptg>>) {
*self.formula1.borrow_mut()=v;
}
// java public Ptg[] formula2() {
// java return this.formula2;
// java }
pub fn formula2(&self)->Option<Vec<Ptg>> {
self.formula2.borrow().clone()
}
// java public void setFormula2(Ptg[] formula2) {
// java this.formula2 = formula2;
// java }
pub fn setFormula2(&self,v:Option<Vec<Ptg>>) {
*self.formula2.borrow_mut()=v;
}
// java public FontFormatting getFont() {
// java return this.font;
// java }
pub fn getFont(&self)->Option<Rc<FontFormatting>> {
self.font.borrow().clone()
}
// java public void setFont(FontFormatting font) {
// java this.font = font;
// java }
pub fn setFont(&self,v:Option<Rc<FontFormatting>>) {
*self.font.borrow_mut()=v;
}
// java public BorderFormatting getBorder() {
// java return this.border;
// java }
pub fn getBorder(&self)->Option<Rc<RefCell<BorderFormatting>>> {
self.border.borrow().clone()
}
// java public void setBorder(BorderFormatting border) {
// java this.border = border;
// java }
pub fn setBorder(&self,v:Option<Rc<RefCell<BorderFormatting>>>) {
*self.border.borrow_mut()=v;
}
// java public PatternFormatting getPattern() {
// java return this.pattern;
// java }
pub fn getPattern(&self)->Option<Rc<RefCell<PatternFormatting>>> {
self.pattern.borrow().clone()
}
// java public void setPattern(PatternFormatting pattern) {
// java this.pattern = pattern;
// java }
pub fn setPattern(&self,v:Option<Rc<RefCell<PatternFormatting>>>) {
*self.pattern.borrow_mut()=v;
}
// java public NumberFormatFormatting getNumFmt() {
// java return this.numFmt;
// java }
pub fn getNumFmt(&self)->Option<Rc<NumberFormatFormatting>> {
self.numFmt.borrow().clone()
}
// java public void setNumFmt(NumberFormatFormatting numFmt) {
// java this.numFmt = numFmt;
// java }
pub fn setNumFmt(&self,v:Option<Rc<NumberFormatFormatting>>) {
*self.numFmt.borrow_mut()=v;
}
// java public List<Ptg[]> formulas() {
// java List<Ptg[]> ptgs = new ArrayList(2);
// java if (this.formula1 != null) {
// java ptgs.add(this.formula1);
// java }
// java
// java if (this.formula2 != null) {
// java ptgs.add(this.formula2);
// java }
// java
// java return ptgs;
// java }
pub fn formulas(&self)->Vec<Vec<Ptg>> {
[self.formula1(),self.formula2()].into_iter().flatten().collect()
}
// java public int getPatternType() {
// java return this.pattern != null ? this.pattern.getFillPattern() : 0;
// java }
pub fn getPatternType(&self)->i32 {
self.getPattern().map_or(0,|p|p.borrow().getFillPattern() as i32)
}
// java public int getPatternColorIndex() {
// java return this.pattern != null ? this.pattern.getFillForegroundColor() : 0;
// java }
pub fn getPatternColorIndex(&self)->i32 {
self.getPattern().map_or(0,|p|p.borrow().getFillForegroundColor() as i32)
}
// java public int getColor() {
// java return this.pattern != null ? this.pattern.getFillBackgroundColor() : 0;
// java }
pub fn getColor(&self)->i32 {
self.getPattern().map_or(0,|p|p.borrow().getFillBackgroundColor() as i32)
}
}

impl Rule{// java public boolean equals(Object obj) {
// java if (this == obj) {
// java return true;
// java } else if (obj == null) {
// java return false;
// java } else if (this.getClass() != obj.getClass()) {
// java return false;
// java } else {
// java Rule other = (Rule)obj;
// java if (this.border == null) {
// java if (other.border != null) {
// java return false;
// java }
// java } else if (!this.border.equals(other.border)) {
// java return false;
// java }
// java
// java if (this.dxfId != other.dxfId) {
// java return false;
// java } else {
// java if (this.font == null) {
// java if (other.font != null) {
// java return false;
// java }
// java } else if (!this.font.equals(other.font)) {
// java return false;
// java }
// java
// java if (!Arrays.equals(this.formula1, other.formula1)) {
// java return false;
// java } else if (!Arrays.equals(this.formula2, other.formula2)) {
// java return false;
// java } else {
// java if (this.numFmt == null) {
// java if (other.numFmt != null) {
// java return false;
// java }
// java } else if (!this.numFmt.equals(other.numFmt)) {
// java return false;
// java }
// java
// java if (this.pattern == null) {
// java if (other.pattern != null) {
// java return false;
// java }
// java } else if (!this.pattern.equals(other.pattern)) {
// java return false;
// java }
// java
// java if (this.priority != other.priority) {
// java return false;
// java } else if (this.stopIfTrue != other.stopIfTrue) {
// java return false;
// java } else {
// java return this.type == other.type;
// java }
// java }
// java }
// java }
// java }
pub fn equals(&self,o:&Self)->bool {
use super::super::java_values::formulas_eq;if std::ptr::eq(self,o){return true;}if self.dxfId.get()!=o.dxfId.get()||self.priority.get()!=o.priority.get()||self.stopIfTrue.get()!=o.stopIfTrue.get()||self.rtype.get()!=o.rtype.get()||!formulas_eq(&self.formula1.borrow(),&o.formula1.borrow())||!formulas_eq(&self.formula2.borrow(),&o.formula2.borrow()){return false;}let font=match(&*self.font.borrow(),&*o.font.borrow()){(None,None)=>true,(Some(a),Some(b))=>a.equals(b),_=>false};let border=match(&*self.border.borrow(),&*o.border.borrow()){(None,None)=>true,(Some(a),Some(b))=>a.borrow().equals(&b.borrow()),_=>false};let pattern=match(&*self.pattern.borrow(),&*o.pattern.borrow()){(None,None)=>true,(Some(a),Some(b))=>a.borrow().equals(&b.borrow()),_=>false};let num=match(&*self.numFmt.borrow(),&*o.numFmt.borrow()){(None,None)=>true,(Some(a),Some(b))=>a.equals(b),_=>false};font&&border&&pattern&&num
}
// java public int hashCode() {
// java int prime = 31;
// java int result = 1;
// java result = 31 * result + (this.border == null ? 0 : this.border.hashCode());
// java result = 31 * result + this.dxfId;
// java result = 31 * result + (this.font == null ? 0 : this.font.hashCode());
// java result = 31 * result + Arrays.hashCode(this.formula1);
// java result = 31 * result + Arrays.hashCode(this.formula2);
// java result = 31 * result + (this.numFmt == null ? 0 : this.numFmt.hashCode());
// java result = 31 * result + (this.pattern == null ? 0 : this.pattern.hashCode());
// java result = 31 * result + this.priority;
// java result = 31 * result + (this.stopIfTrue ? 1231 : 1237);
// java result = 31 * result + (this.type == null ? 0 : this.type.hashCode());
// java return result;
// java }
pub fn hashCode(&self)->i32 {
use super::super::java_values::{fold,boolean,formulas_hash};fold([self.border.borrow().as_ref().map_or(0,|b|b.borrow().hashCode()),self.dxfId.get(),self.font.borrow().as_ref().map_or(0,|b|b.hashCode()),formulas_hash(&self.formula1.borrow()),formulas_hash(&self.formula2.borrow()),self.numFmt.borrow().as_ref().map_or(0,|b|b.hashCode()),self.pattern.borrow().as_ref().map_or(0,|b|b.borrow().hashCode()),self.priority.get(),boolean(self.stopIfTrue.get()),self.rtype.get().map_or(0,|v|v as i32+512)])
}
}
