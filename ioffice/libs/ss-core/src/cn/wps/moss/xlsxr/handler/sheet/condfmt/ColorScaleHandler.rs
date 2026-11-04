use crate::java_runtime::{Rc,RefCell,Cell};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::app::condfmt::{KmoCfRule::KmoCfRule,KmoConditionFormatterDefine::KmoConditionFormatterDefine,rule::{Rule::{RuleObject,CfRuleTypes},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::{KmoIconSet,IconSetTypes}}};
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
pub struct ColorScaleHandler{cfRule:Rc<KmoCfRule>,rule:Rc<dyn RuleObject>,sheet:Rc<RefCell<KmoSheet>>}
impl ColorScaleHandler{
// java public ColorScaleHandler(KmoCfRule cfRule, KmoSheet sheet) {
// java this.cfRule = cfRule;
// java this.colorScale = (KmoColorScale)cfRule.getRule();
// java this.sheet = sheet;
// java }
pub fn new(cfRule:Rc<KmoCfRule>,sheet:Rc<RefCell<KmoSheet>>)->Self {
let rule=cfRule.getRule();Self{cfRule,rule,sheet}
}
}
impl ElementHandler for ColorScaleHandler{
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java }
fn startElement(&self,_e:i32,a:&dyn XmlRoAttr){}
// java public ElementHandler enterSubElement(int subElement) {
// java switch (subElement) {
// java case 4661:
// java return new ColorHandler(this.colorScale);
// java case 4967:
// java return new CfvoHandler(this.colorScale, this.sheet);
// java default:
// java return null;
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{match e{4967=>Some(Box::new(super::CfvoHandler::CfvoHandler::new(self.rule.clone(),self.sheet.clone()))),4661=>Some(Box::new(super::ColorHandler::ColorHandler::new(self.rule.clone()))),_=>None}}
// java public void endElement(int element) {
// java this.cfRule.setRule(this.colorScale);
// java }
fn endElement(&self,_e:i32){self.cfRule.setRule(self.rule.clone());}
fn characters(&self,_s:&str){}
}
