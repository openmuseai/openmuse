use crate::java_runtime::{Rc,RefCell,Cell};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::app::condfmt::{KmoCfRule::KmoCfRule,KmoConditionFormatterDefine::KmoConditionFormatterDefine,rule::{Rule::{RuleObject,CfRuleTypes},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::{KmoIconSet,IconSetTypes}}};
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
pub struct WorksheetDelayHandler{sheet:Rc<RefCell<KmoSheet>>,styles:Rc<RefCell<StylesTable>>}
impl WorksheetDelayHandler{pub fn new(sheet:Rc<RefCell<KmoSheet>>,styles:Rc<RefCell<StylesTable>>)->Self{Self{sheet,styles}}}
impl ElementHandler for WorksheetDelayHandler{fn startElement(&self,_e:i32,_a:&dyn XmlRoAttr){}fn endElement(&self,_e:i32){}fn characters(&self,_s:&str){}
// java public ElementHandler enterSubElement(int subElement) {
// java if (subElement == 4794) {
// java return new DataValidationsHandler(this.mReader.getSheet());
// java } else {
// java return subElement == 4776 ? new ConditionalFormattingHandler(this.mReader.getSheet(), this.mReader) : null;
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{if e==4776{Some(Box::new(super::ConditionalFormattingHandler::ConditionalFormattingHandler::new(self.sheet.clone(),self.styles.clone(),false)))}else{None}}}
