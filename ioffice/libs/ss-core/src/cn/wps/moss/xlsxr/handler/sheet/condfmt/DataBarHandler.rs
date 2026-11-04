use crate::java_runtime::{Rc,RefCell,Cell};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::app::condfmt::{KmoCfRule::KmoCfRule,KmoConditionFormatterDefine::KmoConditionFormatterDefine,rule::{Rule::{RuleObject,CfRuleTypes},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::{KmoIconSet,IconSetTypes}}};
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
pub struct DataBarHandler{cfRule:Rc<KmoCfRule>,rule:Rc<dyn RuleObject>,sheet:Rc<RefCell<KmoSheet>>}
impl DataBarHandler{
// java public DataBarHandler(KmoCfRule cfRule, KmoSheet sheet) {
// java this.cfRule = cfRule;
// java this.dataBar = (KmoDataBar)cfRule.getRule();
// java this.sheet = sheet;
// java }
pub fn new(cfRule:Rc<KmoCfRule>,sheet:Rc<RefCell<KmoSheet>>)->Self {
let rule=cfRule.getRule();Self{cfRule,rule,sheet}
}
}
impl ElementHandler for DataBarHandler{
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java XmlRoAttr tmp;
// java if ((tmp = attrsNotCopy.getByName(4969)) != null) {
// java this.dataBar.maxLength = tmp.toInt();
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4968)) != null) {
// java this.dataBar.minLength = tmp.toInt();
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4970)) != null) {
// java this.dataBar.setShowValue(tmp.toBoolean());
// java }
// java
// java }
fn startElement(&self,_e:i32,a:&dyn XmlRoAttr){let bar=self.rule.as_any().downcast_ref::<KmoDataBar>().unwrap();if let Some(a)=a.get_by_name(4969){bar.maxLength.set(a.to_int());}if let Some(a)=a.get_by_name(4968){bar.minLength.set(a.to_int());}if let Some(a)=a.get_by_name(4970){bar.setShowValue(a.to_boolean());}}
// java public ElementHandler enterSubElement(int subElement) {
// java switch (subElement) {
// java case 4661:
// java case 5499:
// java return new ColorHandler(this.dataBar);
// java case 4967:
// java return new CfvoHandler(this.dataBar, this.sheet);
// java default:
// java return null;
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{match e{4967=>Some(Box::new(super::CfvoHandler::CfvoHandler::new(self.rule.clone(),self.sheet.clone()))),4661|5499=>Some(Box::new(super::ColorHandler::ColorHandler::new(self.rule.clone()))),_=>None}}
// java public void endElement(int element) {
// java if (this.cfRule.isExt() && this.dataBar.cfColors().isEmpty()) {
// java KmoCfColor cfColor = new KmoCfColor();
// java cfColor.auto = true;
// java this.dataBar.addColor(cfColor);
// java }
// java
// java if (this.cfRule.isExt() && this.cfRule.getRule() != null && this.cfRule.getRule().getPriority() == -1) {
// java this.cfRule.getRule().setPriority(32767);
// java }
// java
// java this.cfRule.setRule(this.dataBar);
// java }
fn endElement(&self,_e:i32){let bar=self.rule.as_any().downcast_ref::<KmoDataBar>().unwrap();if self.cfRule.isExt()&&bar.cfColors().borrow().is_empty(){let mut c=crate::cn::wps::moss::app::condfmt::KmoCfColor::KmoCfColor::new();c.auto=true;bar.addColor(Rc::new(RefCell::new(c)));}if self.cfRule.isExt()&&self.rule.getPriority()== -1{self.rule.setPriority(32767);}self.cfRule.setRule(self.rule.clone());}
fn characters(&self,_s:&str){}
}
