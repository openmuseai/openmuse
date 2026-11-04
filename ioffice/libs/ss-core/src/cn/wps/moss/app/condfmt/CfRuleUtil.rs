use crate::{java_runtime::Rc,cn::wps::{et::ss::{formula::{ptg::Ptg::Ptg,parser::{FormulaParser,ParserOption::ParserOption}},calcchain::FormulaHelper::FormulaHelper},moss::{app::KmoSheet::KmoSheet,util::KmoRect::KmoRect,engine::eval::EvaluationWorkbook::EvaluationWorkbook}}};
use super::{FormulaBuilder::FormulaBuilder,rule::{Rule::{RuleObject,CfRuleTypes},KmoRuleTop::KmoRuleTop,KmoRuleAverage::KmoRuleAverage}};
pub struct CfRuleUtil;impl CfRuleUtil{
// java public static Ptg[] turnFormulaToPtg(String form, KmoSheet sheet) {
// java Ptg[] ptgs;
// java try {
// java ptgs = FormulaParser.parse(form, new EvaluationWorkbook(sheet.getBook()), 0, sheet.getSheetIndex(), sheet.getVersion(), ParserOption.createOOXMLParserOption());
// java } catch (Throwable e) {
// java e.printStackTrace();
// java ptgs = new Ptg[0];
// java }
// java
// java return ptgs;
// java }
pub fn turnFormulaToPtg(form:&str,sheet:&KmoSheet)->Vec<Ptg> {
let wb=EvaluationWorkbook::EvaluationWorkbook(&sheet.getBook());FormulaParser::parse(form,&wb,0,sheet.getSheetIndex(),sheet.getVersion(),&ParserOption::createOOXMLParserOption()).unwrap_or_default()
}
// java public static Ptg[] getSpecialFormula(List<KmoRect> addrs, Rule rule, KmoSheet sheet) {
// java String form1 = "";
// java List<String> ranges = new ArrayList();
// java int i = 0;
// java
// java for(int size = addrs.size(); i < size; ++i) {
// java KmoRect addr = (KmoRect)addrs.get(i);
// java String range1 = AppLogicHelper.getCellStr(true, addr.tl.row, true, addr.tl.col);
// java String range2 = AppLogicHelper.getCellStr(true, addr.br.row, true, addr.br.col);
// java ranges.add(range1 + ":" + range2);
// java }
// java
// java Rule.CfRuleTypes type = rule.type();
// java if (type == Rule.CfRuleTypes.top10) {
// java KmoRuleTop top = (KmoRuleTop)rule;
// java form1 = FormulaBuilder.top10Formula(top.getRank(), top.isPercent(), top.isBottom(), ranges);
// java } else if (type == Rule.CfRuleTypes.aboveAverage) {
// java KmoRuleAverage average = (KmoRuleAverage)rule;
// java form1 = FormulaBuilder.averageFormula(average.isAboveAverage(), average.isEqualAverage(), average.getStdDev(), ranges);
// java } else {
// java if (type == Rule.CfRuleTypes.uniqueValues) {
// java form1 = FormulaBuilder.uniqueFormula(true, ranges);
// java }
// java
// java if (type == Rule.CfRuleTypes.duplicateValues) {
// java form1 = FormulaBuilder.uniqueFormula(false, ranges);
// java }
// java }
// java
// java Ptg[] ptgs = turnFormulaToPtg(form1, sheet);
// java ptgs = FormulaHelper.replaceWithNPtgs(ptgs, 0, 0, sheet.getVersion());
// java return ptgs;
// java }
pub fn getSpecialFormula(addrs:&[KmoRect],rule:&dyn RuleObject,sheet:&KmoSheet)->Vec<Ptg> {
let cell=|row:i32,col:i32|{let mut c=col+1;let mut chars=vec![];while c>0{c-=1;chars.push((b'A'+(c%26) as u8)as char);c/=26;}chars.reverse();format!("${}${}",chars.iter().collect::<String>(),row+1)};let ranges=addrs.iter().map(|r|format!("{}:{}",cell(r.tl.row,r.tl.col),cell(r.br.row,r.br.col))).collect::<Vec<_>>();let formula=match rule.base().ruleType().unwrap(){CfRuleTypes::top10=>{let top=rule.as_any().downcast_ref::<KmoRuleTop>().unwrap();FormulaBuilder::top10Formula(top.getRank(),top.isPercent(),top.isBottom(),&ranges)},CfRuleTypes::aboveAverage=>{let a=rule.as_any().downcast_ref::<KmoRuleAverage>().unwrap();FormulaBuilder::averageFormula(a.isAboveAverage(),a.isEqualAverage(),a.getStdDev(),&ranges)},CfRuleTypes::uniqueValues=>FormulaBuilder::uniqueFormula(true,&ranges),CfRuleTypes::duplicateValues=>FormulaBuilder::uniqueFormula(false,&ranges),_=>String::new()};let p=Self::turnFormulaToPtg(&formula,sheet);FormulaHelper::replaceWithNPtgs(&p,0,0,sheet.getVersion())
}
}
