use crate::java_runtime::{Rc,RefCell,Cell};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::app::condfmt::{KmoCfRule::KmoCfRule,KmoConditionFormatterDefine::KmoConditionFormatterDefine,rule::{Rule::{RuleObject,CfRuleTypes},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::{KmoIconSet,IconSetTypes}}};
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
pub struct ExtListHandler{sheet:Rc<RefCell<KmoSheet>>,styles:Rc<RefCell<StylesTable>>}
impl ExtListHandler{pub fn new(sheet:Rc<RefCell<KmoSheet>>,styles:Rc<RefCell<StylesTable>>)->Self{Self{sheet,styles}}}
impl ElementHandler for ExtListHandler{fn startElement(&self,_e:i32,_a:&dyn XmlRoAttr){}fn endElement(&self,_e:i32){}fn characters(&self,_s:&str){}
// java public ElementHandler enterSubElement(int subElement) {
// java switch (subElement) {
// java case 4148:
// java return new ExtHandler(this.reader, this.isDelay);
// java default:
// java return null;
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{if e==4148{Some(Box::new(super::ExtHandler::ExtHandler::new(self.sheet.clone(),self.styles.clone())))}else{None}}}
