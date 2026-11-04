use crate::java_runtime::{Rc,RefCell,Cell};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::app::condfmt::{KmoCfRule::KmoCfRule,KmoConditionFormatterDefine::KmoConditionFormatterDefine,rule::{Rule::{RuleObject,CfRuleTypes},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::{KmoIconSet,IconSetTypes}}};
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
use crate::cn::wps::moss::app::condfmt::{KmoCfvo::{KmoCfvo,CfvoTypes},CfRuleUtil::CfRuleUtil};
pub struct CfvoHandler{rule:Rc<dyn RuleObject>,sheet:Rc<RefCell<KmoSheet>>,cfvo:Rc<KmoCfvo>,fHandler:Rc<RefCell<Option<String>>>,hasF:Cell<bool>}
impl CfvoHandler{// java public CfvoHandler(KmoColorScale colorScale, KmoSheet sheet) {
// java this.colorScale = colorScale;
// java this.cfvo = new KmoCfvo();
// java this.sheet = sheet;
// java }
pub fn new(rule:Rc<dyn RuleObject>,sheet:Rc<RefCell<KmoSheet>>)->Self {
Self{rule,sheet,cfvo:Rc::new(KmoCfvo::new()),fHandler:Rc::new(RefCell::new(None)),hasF:Cell::new(false)}
}
// java private void parseValue(String value) {
// java if (this.cfvo.type != KmoCfvo.CfvoTypes.min && this.cfvo.type != KmoCfvo.CfvoTypes.max) {
// java try {
// java if (this.cfvo.type == KmoCfvo.CfvoTypes.percentile) {
// java this.cfvo.setValue(ValueHelper.toDouble(value));
// java return;
// java }
// java } catch (Exception var3) {
// java }
// java
// java Ptg[] ptgs = CfRuleUtil.turnFormulaToPtg(value, this.sheet);
// java this.cfvo.setFormula(ptgs);
// java }
// java }
pub fn parseValue(&self,value:&str) {
if !matches!(self.cfvo.rtype.get(),Some(CfvoTypes::min|CfvoTypes::max)){if self.cfvo.rtype.get()==Some(CfvoTypes::percentile){if let Ok(v)=value.parse::<f64>(){self.cfvo.setValue(v);return;}}let p=CfRuleUtil::turnFormulaToPtg(value,&self.sheet.borrow());self.cfvo.setFormula(Some(Rc::new(p)));}
}
}
impl ElementHandler for CfvoHandler{
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java try {
// java XmlRoAttr tmp;
// java if ((tmp = attrsNotCopy.getByName(4125)) != null) {
// java this.cfvo.type = KmoCfvo.CfvoTypes.valueOf(tmp.toString());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4114)) != null) {
// java String value = tmp.toString();
// java this.parseValue(value);
// java }
// java
// java if (attrsNotCopy.getByName(4972) != null) {
// java this.cfvo.setEqual(false);
// java }
// java } catch (Exception var5) {
// java }
// java
// java }
fn startElement(&self,_e:i32,a:&dyn XmlRoAttr){if let Some(a)=a.get_by_name(4125){let Some(t)=CfvoTypes::valueOf(&a.to_string())else{return;};self.cfvo.rtype.set(Some(t));}if let Some(a)=a.get_by_name(4114){self.parseValue(&a.to_string());}if a.get_by_name(4972).is_some(){self.cfvo.setEqual(false);}}
// java public DefaultElementHandler enterSubElement(int subElement) {
// java if (subElement == 4359) {
// java this.fHandler = new FHandler();
// java return this.fHandler;
// java } else {
// java return null;
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{if e==4359{self.hasF.set(true);Some(Box::new(FHandler(self.fHandler.clone())))}else{None}}
// java public void endElement(int element) {
// java if (null != this.fHandler) {
// java String value = this.fHandler.getValue();
// java if (null != value) {
// java this.parseValue(value);
// java }
// java
// java this.fHandler = null;
// java }
// java
// java if (this.colorScale != null) {
// java this.colorScale.addCfvo(this.cfvo);
// java }
// java
// java if (this.dataBar != null) {
// java this.dataBar.addCfvo(this.cfvo);
// java }
// java
// java if (this.iconSet != null) {
// java this.iconSet.addCfvo(this.cfvo);
// java }
// java
// java }
fn endElement(&self,_e:i32){if self.hasF.get(){if let Some(s)=self.fHandler.borrow().as_ref(){self.parseValue(s);}self.hasF.set(false);}if let Some(r)=self.rule.as_any().downcast_ref::<KmoColorScale>(){r.addCfvo(self.cfvo.clone());}if let Some(r)=self.rule.as_any().downcast_ref::<KmoDataBar>(){r.addCfvo(self.cfvo.clone());}if let Some(r)=self.rule.as_any().downcast_ref::<KmoIconSet>(){r.addCfvo(self.cfvo.clone());}}fn characters(&self,_s:&str){}}
struct FHandler(Rc<RefCell<Option<String>>>);impl ElementHandler for FHandler{fn startElement(&self,_e:i32,_a:&dyn XmlRoAttr){}fn endElement(&self,_e:i32){}fn characters(&self,s:&str){*self.0.borrow_mut()=Some(s.to_string());}fn enterSubElement(&self,_e:i32)->Option<Box<dyn ElementHandler>>{None}}
