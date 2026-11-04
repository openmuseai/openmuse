use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::moss::{util::KmoRect::KmoRect,app::KmoSheet::KmoSheet}};
use super::rule::{Rule::{Rule,RuleObject,CfRuleTypes},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::KmoIconSet,KmoRuleCellIs::KmoRuleCellIs,KmoRuleContainSpecial::KmoRuleContainSpecial,KmoRuleTimePeriod::KmoRuleTimePeriod,KmoRuleSpecialText::KmoRuleSpecialText,KmoRuleFormula::KmoRuleFormula,KmoRuleTop::KmoRuleTop,KmoRuleUnique::KmoRuleUnique,KmoRuleAverage::KmoRuleAverage};
// java public class KmoCfRule implements Cloneable
pub struct KmoCfRule {rule:RefCell<Option<Rc<dyn RuleObject>>>,refRanges:RefCell<Option<Rc<Vec<KmoRect>>>>,enclosingRange:RefCell<Option<KmoRect>>,isExt:Cell<bool>,formatterGroupIndex:Cell<i32>}
impl KmoCfRule {
// java public KmoCfRule() {
// java }
pub fn new()->Self {
Self{rule:RefCell::new(None),refRanges:RefCell::new(None),enclosingRange:RefCell::new(None),isExt:Cell::new(false),formatterGroupIndex:Cell::new(-1)}
}
// java public KmoCfRule(Rule rule) {
// java this.rule = rule;
// java }
pub fn fromRule(rule:Rc<dyn RuleObject>)->Self {
let s=Self::new();s.setRule(rule);s
}
// java public Rule getRule() {
// java return this.rule;
// java }
pub fn getRule(&self)->Rc<dyn RuleObject> {
self.rule.borrow().clone().expect("NullPointerException: Rule")
}
// java public void setRule(Rule rule) {
// java this.rule = rule;
// java }
pub fn setRule(&self,rule:Rc<dyn RuleObject>) {
*self.rule.borrow_mut()=Some(rule);
}
// java public Rule.CfRuleTypes ruleType() {
// java return this.rule.type();
// java }
pub fn ruleType(&self)->Option<CfRuleTypes> {
self.getRule().ruleType()
}
// java public boolean isExt() {
// java return this.isExt;
// java }
pub fn isExt(&self)->bool {
self.isExt.get()
}
// java public void setExt(boolean ext) {
// java this.isExt = ext;
// java }
pub fn setExt(&self,v:bool) {
self.isExt.set(v);
}
// java public int getFormatterGroupIndex() {
// java return this.formatterGroupIndex;
// java }
pub fn getFormatterGroupIndex(&self)->i32 {
self.formatterGroupIndex.get()
}
// java public void setFormatterGroupIndex(int formatterGroupIndex) {
// java this.formatterGroupIndex = formatterGroupIndex;
// java }
pub fn setFormatterGroupIndex(&self,v:i32) {
self.formatterGroupIndex.set(v);
}
// java public KmoRect[] ranges() {
// java return this.refRanges;
// java }
pub fn ranges(&self)->Option<Rc<Vec<KmoRect>>> {
self.refRanges.borrow().clone()
}
// java public void setRanges(List<KmoRect> ranges) {
// java KmoRect[] ref = new KmoRect[ranges.size()];
// java this.refRanges = (KmoRect[])ranges.toArray(ref);
// java this.createEnclosingRange();
// java }
pub fn setRanges(&self,ranges:Vec<KmoRect>) {
*self.refRanges.borrow_mut()=Some(Rc::new(ranges));self.createEnclosingRange();
}
// java private void createEnclosingRange() {
// java KmoRect rect = new KmoRect(this.ranges()[0]);
// java
// java for(KmoRect cellAddr : this.ranges()) {
// java rect.combine(cellAddr);
// java }
// java
// java this.enclosingRange = rect;
// java }
pub fn createEnclosingRange(&self) {
let ranges=self.ranges().expect("null ranges");let mut rect=ranges[0].clone();for addr in ranges.iter(){rect.combine(addr);}*self.enclosingRange.borrow_mut()=Some(rect);
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
// java public static Rule buildRule(Rule.CfRuleTypes type, SpreadsheetVersion sv) {
// java switch (type) {
// java case colorScale:
// java return new KmoColorScale(sv);
// java case dataBar:
// java return new KmoDataBar(sv);
// java case iconSet:
// java return new KmoIconSet(sv);
// java case cellIs:
// java return new KmoRuleCellIs(type, sv);
// java case containsBlanks:
// java case notContainsBlanks:
// java case containsErrors:
// java case notContainsErrors:
// java return new KmoRuleContainSpecial(type, sv);
// java case timePeriod:
// java return new KmoRuleTimePeriod(sv);
// java case containsText:
// java case notContainsText:
// java case beginsWith:
// java case endsWith:
// java return new KmoRuleSpecialText(type, sv);
// java case expression:
// java return new KmoRuleFormula(sv);
// java case top10:
// java return new KmoRuleTop(sv);
// java case uniqueValues:
// java case duplicateValues:
// java return new KmoRuleUnique(type, sv);
// java case aboveAverage:
// java return new KmoRuleAverage(type, sv);
// java default:
// java return new KmoRuleCellIs(type, sv);
// java }
// java }
pub fn buildRule(ctype:CfRuleTypes,sv:SpreadsheetVersion)->Rc<dyn RuleObject> {
use CfRuleTypes::*;let r:Rc<dyn RuleObject>=match ctype{colorScale=>Rc::new(KmoColorScale::new(sv)),dataBar=>Rc::new(KmoDataBar::new(sv)),iconSet=>Rc::new(KmoIconSet::new(sv)),timePeriod=>Rc::new(KmoRuleTimePeriod::new(sv)),expression=>Rc::new(KmoRuleFormula::new(sv)),top10=>Rc::new(KmoRuleTop::new(sv)),aboveAverage=>Rc::new(KmoRuleAverage::new(sv)),uniqueValues|duplicateValues=>Rc::new(KmoRuleUnique::new(ctype,sv)),containsBlanks|notContainsBlanks|containsErrors|notContainsErrors=>Rc::new(KmoRuleContainSpecial::new(ctype,sv)),containsText|notContainsText|beginsWith|endsWith=>Rc::new(KmoRuleSpecialText::new(sv)),_=>Rc::new(KmoRuleCellIs::new(sv))};r.setType(Some(ctype));r
}
// java public void addFormula(String formula, KmoRect enclosingRect, KmoSheet kmoSheet) {
// java int row = enclosingRect.tl.row;
// java int col = enclosingRect.tl.col;
// java Ptg[] ptgs = CfRuleUtil.turnFormulaToPtg(formula, kmoSheet);
// java ptgs = FormulaHelper.replaceWithNPtgs(ptgs, row, col, kmoSheet.getVersion());
// java if (this.rule.formula1().length == 0) {
// java this.rule.setFormula1(ptgs);
// java } else {
// java this.rule.setFormula2(ptgs);
// java }
// java
// java }
pub fn addFormula(&self,formula:&str,enclosingRect:&KmoRect,kmoSheet:&KmoSheet) {
use crate::cn::wps::et::ss::{formula::parser::{FormulaParser,ParserOption::ParserOption},calcchain::FormulaHelper::FormulaHelper};use crate::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;let wb=EvaluationWorkbook::EvaluationWorkbook(&kmoSheet.getBook());let ptgs=FormulaParser::parse(formula,&wb,0,kmoSheet.getSheetIndex(),kmoSheet.getVersion(),&ParserOption::createOOXMLParserOption()).unwrap_or_default();let ptgs=FormulaHelper::replaceWithNPtgs(&ptgs,enclosingRect.tl.row,enclosingRect.tl.col,kmoSheet.getVersion());let rule=self.getRule();if rule.formula1().unwrap().is_empty(){rule.setFormula1(Some(ptgs));}else{rule.setFormula2(Some(ptgs));}
}
// java public boolean checkNeedCreateFormula(List<KmoRect> addrs, KmoSheet kmoSheet) {
// java Rule.CfRuleTypes type = this.rule.type();
// java if (isNeedCreateFormulaRuleType(type)) {
// java Ptg[] ptgs = CfRuleUtil.getSpecialFormula(addrs, this.rule, kmoSheet);
// java this.rule.setFormula1(ptgs);
// java return true;
// java } else {
// java return false;
// java }
// java }
pub fn checkNeedCreateFormula(&self,addrs:&[KmoRect],sheet:&KmoSheet)->bool {
let r=self.getRule();if matches!(r.ruleType(),Some(CfRuleTypes::top10|CfRuleTypes::aboveAverage|CfRuleTypes::uniqueValues|CfRuleTypes::duplicateValues)){let p=super::CfRuleUtil::CfRuleUtil::getSpecialFormula(addrs,r.as_ref(),sheet);r.setFormula1(Some(p));true}else{false}
}
// java public static void exchangeBorderHairAndDotted(BorderFormatting border) {
// java if (border != null) {
// java int bottom = border.getBorderBottom();
// java if (bottom == 4) {
// java border.setBorderBottom(7);
// java }
// java
// java if (bottom == 7) {
// java border.setBorderBottom(4);
// java }
// java
// java int left = border.getBorderLeft();
// java if (left == 4) {
// java border.setBorderLeft(7);
// java }
// java
// java if (left == 7) {
// java border.setBorderLeft(4);
// java }
// java
// java int right = border.getBorderRight();
// java if (right == 4) {
// java border.setBorderRight(7);
// java }
// java
// java if (right == 7) {
// java border.setBorderRight(4);
// java }
// java
// java int top = border.getBorderTop();
// java if (top == 4) {
// java border.setBorderTop(7);
// java }
// java
// java if (top == 7) {
// java border.setBorderTop(4);
// java }
// java
// java }
// java }
pub fn exchangeBorderHairAndDotted(border:&mut crate::cn::wps::moss::filefmt::biff8::record::cf::BorderFormatting::BorderFormatting) {
let style=border.getBorderBottom();if style==4{border.setBorderBottom(7);}if style==7{border.setBorderBottom(4);}let style=border.getBorderLeft();if style==4{border.setBorderLeft(7);}if style==7{border.setBorderLeft(4);}let style=border.getBorderRight();if style==4{border.setBorderRight(7);}if style==7{border.setBorderRight(4);}let style=border.getBorderTop();if style==4{border.setBorderTop(7);}if style==7{border.setBorderTop(4);}
}
}

impl KmoCfRule{// java public boolean equals(Object obj) {
// java if (this == obj) {
// java return true;
// java } else if (obj == null) {
// java return false;
// java } else if (this.getClass() != obj.getClass()) {
// java return false;
// java } else {
// java KmoCfRule other = (KmoCfRule)obj;
// java if (this.enclosingRange == null) {
// java if (other.enclosingRange != null) {
// java return false;
// java }
// java } else if (!this.enclosingRange.equals(other.enclosingRange)) {
// java return false;
// java }
// java
// java if (!Arrays.equals(this.refRanges, other.refRanges)) {
// java return false;
// java } else {
// java if (this.rule == null) {
// java if (other.rule != null) {
// java return false;
// java }
// java } else if (!this.rule.equals(other.rule)) {
// java return false;
// java }
// java
// java return true;
// java }
// java }
// java }
pub fn equals(&self,o:&Self)->bool {
std::ptr::eq(self,o)||(*self.enclosingRange.borrow()==*o.enclosingRange.borrow()&&*self.refRanges.borrow()==*o.refRanges.borrow()&&match(&*self.rule.borrow(),&*o.rule.borrow()){(None,None)=>true,(Some(a),Some(b))=>a.javaEquals(b.as_ref()),_=>false})
}
// java public int hashCode() {
// java int prime = 31;
// java int result = 1;
// java result = 31 * result + (this.colorScaleValue == null ? 0 : this.colorScaleValue.hashCode());
// java result = 31 * result + (this.dataBarValue == null ? 0 : this.dataBarValue.hashCode());
// java result = 31 * result + (this.enclosingRange == null ? 0 : this.enclosingRange.hashCode());
// java result = 31 * result + (this.iconSetValue == null ? 0 : this.iconSetValue.hashCode());
// java result = 31 * result + Arrays.hashCode(this.refRanges);
// java result = 31 * result + (this.rule == null ? 0 : this.rule.hashCode());
// java return result;
// java }
pub fn hashCode(&self)->i32 {
use super::java_values::fold;fold([0,0,self.enclosingRange.borrow().as_ref().map_or(0,|r|r.hashCode()),0,self.refRanges.borrow().as_ref().map_or(0,|rs|fold(rs.iter().map(|r|r.hashCode()))),self.rule.borrow().as_ref().map_or(0,|r|r.javaHashCode())])
}
}
