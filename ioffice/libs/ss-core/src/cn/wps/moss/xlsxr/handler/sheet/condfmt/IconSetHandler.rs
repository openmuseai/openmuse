use crate::java_runtime::{Rc,RefCell,Cell};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::app::condfmt::{KmoCfRule::KmoCfRule,KmoConditionFormatterDefine::KmoConditionFormatterDefine,rule::{Rule::{RuleObject,CfRuleTypes},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::{KmoIconSet,IconSetTypes}}};
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
pub struct IconSetHandler{cfRule:Rc<KmoCfRule>,rule:Rc<dyn RuleObject>,sheet:Rc<RefCell<KmoSheet>>}
impl IconSetHandler{
// java public IconSetHandler(KmoCfRule cfRule, KmoSheet sheet) {
// java this.cfRule = cfRule;
// java this.iconSet = (KmoIconSet)cfRule.getRule();
// java this.sheet = sheet;
// java }
pub fn new(cfRule:Rc<KmoCfRule>,sheet:Rc<RefCell<KmoSheet>>)->Self {
let rule=cfRule.getRule();Self{cfRule,rule,sheet}
}
}
impl ElementHandler for IconSetHandler{
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java XmlRoAttr tmp;
// java if ((tmp = attrsNotCopy.getByName(4123)) != null) {
// java try {
// java this.iconSet.setIconSetType(KmoIconSet.IconSetTypes.valueOf("$" + tmp.toString()));
// java } catch (IllegalArgumentException var5) {
// java this.iconSet.setIconSetType(KmoIconSet.IconSetTypes.$3TrafficLights1);
// java }
// java } else {
// java this.iconSet.setIconSetType(KmoIconSet.IconSetTypes.$3TrafficLights1);
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4119)) != null) {
// java this.iconSet.setPercent(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4971)) != null) {
// java this.iconSet.setReverse(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4970)) != null) {
// java this.iconSet.setShowValue(tmp.toBoolean());
// java }
// java
// java }
fn startElement(&self,_e:i32,a:&dyn XmlRoAttr){let icon=self.rule.as_any().downcast_ref::<KmoIconSet>().unwrap();icon.setIconSetType(Some(a.get_by_name(4123).and_then(|a|IconSetTypes::valueOf(&format!("${}",a.to_string()))).unwrap_or(IconSetTypes::I3TrafficLights1)));if let Some(a)=a.get_by_name(4119){icon.setPercent(a.to_boolean());}if let Some(a)=a.get_by_name(4971){icon.setReverse(a.to_boolean());}if let Some(a)=a.get_by_name(4970){icon.setShowValue(a.to_boolean());}}
// java public ElementHandler enterSubElement(int subElement) {
// java switch (subElement) {
// java case 4967:
// java return new CfvoHandler(this.iconSet, this.sheet);
// java default:
// java return null;
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{match e{4967=>Some(Box::new(super::CfvoHandler::CfvoHandler::new(self.rule.clone(),self.sheet.clone()))),_=>None}}
// java public void endElement(int element) {
// java this.cfRule.setRule(this.iconSet);
// java }
fn endElement(&self,_e:i32){self.cfRule.setRule(self.rule.clone());}
fn characters(&self,_s:&str){}
}
