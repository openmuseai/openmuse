use crate::java_runtime::{Rc,RefCell,Cell};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::app::condfmt::{KmoCfRule::KmoCfRule,KmoConditionFormatterDefine::KmoConditionFormatterDefine,rule::{Rule::{RuleObject,CfRuleTypes},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::{KmoIconSet,IconSetTypes}}};
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
use crate::cn::wps::moss::app::condfmt::KmoCfColor::KmoCfColor;
pub struct ColorHandler {rule:Rc<dyn RuleObject>,cfColor:Rc<RefCell<KmoCfColor>>}
impl ColorHandler{// java public ColorHandler(KmoColorScale colorScale) {
// java this.colorScale = colorScale;
// java this.cfColor = new KmoCfColor();
// java }
pub fn new(rule:Rc<dyn RuleObject>)->Self {
Self{rule,cfColor:Rc::new(RefCell::new(KmoCfColor::new()))}
}
}
impl ElementHandler for ColorHandler{
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java XmlRoAttr tmp;
// java if ((tmp = attrsNotCopy.getByName(5153)) != null) {
// java this.cfColor.auto = tmp.toBoolean();
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(5154)) != null) {
// java this.cfColor.indexed = tmp.toInt();
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(5155)) != null) {
// java this.cfColor.rgb = tmp.toString();
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(5156)) != null) {
// java this.cfColor.theme = tmp.toInt();
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(5157)) != null) {
// java this.cfColor.tint = tmp.toDouble();
// java }
// java
// java }
fn startElement(&self,_e:i32,a:&dyn XmlRoAttr){let mut c=self.cfColor.borrow_mut();if let Some(a)=a.get_by_name(5153){c.auto=a.to_boolean();}if let Some(a)=a.get_by_name(5154){c.indexed=a.to_int();}if let Some(a)=a.get_by_name(5155){c.rgb=a.to_string();}if let Some(a)=a.get_by_name(5156){c.theme=a.to_int();}if let Some(a)=a.get_by_name(5157){c.tint=a.to_double();}}
// java public void endElement(int element) {
// java if (this.colorScale != null) {
// java this.colorScale.addColor(this.cfColor);
// java }
// java
// java if (this.dataBar != null) {
// java this.dataBar.addColor(this.cfColor);
// java }
// java
// java }
fn endElement(&self,_e:i32){if let Some(r)=self.rule.as_any().downcast_ref::<KmoColorScale>(){r.addColor(self.cfColor.clone());}if let Some(r)=self.rule.as_any().downcast_ref::<KmoDataBar>(){r.addColor(self.cfColor.clone());}}fn characters(&self,_s:&str){}fn enterSubElement(&self,_e:i32)->Option<Box<dyn ElementHandler>>{None}}
