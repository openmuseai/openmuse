use crate::java_runtime::{Rc,RefCell,Cell};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::app::condfmt::{KmoCfRule::KmoCfRule,KmoConditionFormatterDefine::KmoConditionFormatterDefine,rule::{Rule::{RuleObject,CfRuleTypes},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::{KmoIconSet,IconSetTypes}}};
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
pub struct ExtHandler{sheet:Rc<RefCell<KmoSheet>>,styles:Rc<RefCell<StylesTable>>}
impl ExtHandler{pub fn new(sheet:Rc<RefCell<KmoSheet>>,styles:Rc<RefCell<StylesTable>>)->Self{Self{sheet,styles}}}
impl ElementHandler for ExtHandler{fn startElement(&self,_e:i32,_a:&dyn XmlRoAttr){}fn endElement(&self,_e:i32){}fn characters(&self,_s:&str){}
// java public ElementHandler enterSubElement(int subElement) {
// java switch (subElement) {
// java case 4794:
// java return new DataValidationsExtHandler(this.sheet);
// java case 5371:
// java if (this.isDelay) {
// java return new SparklineGroupsHandler(this.sheet);
// java }
// java
// java this.reader.setHasSparklineGroups(true);
// java return null;
// java case 5498:
// java return new ConditionalFormattingsHandler(this.sheet, this.reader);
// java default:
// java return null;
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{if e==5498{Some(Box::new(super::ConditionalFormattingsHandler::ConditionalFormattingsHandler::new(self.sheet.clone(),self.styles.clone())))}else{None}}}
