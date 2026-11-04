use crate::java_runtime::{Rc,RefCell,Cell};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::app::condfmt::{KmoCfRule::KmoCfRule,KmoConditionFormatterDefine::KmoConditionFormatterDefine,rule::{Rule::{RuleObject,CfRuleTypes},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::{KmoIconSet,IconSetTypes}}};
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
use crate::cn::wps::moss::{app::condfmt::KmoConditionFormatter::KmoConditionFormatter,filefmt::biff8::util::CellRangeAddress::CellRangeAddress};use super::CfRuleHandler::CfRuleHandler;
pub struct ConditionalFormattingHandler{stylesTable:Rc<RefCell<StylesTable>>,condFmt10:Rc<KmoConditionFormatterDefine>,sheet:Rc<RefCell<KmoSheet>>,cellRangeAddressList:Rc<RefCell<Option<Vec<CellRangeAddress>>>>,isFromExt:bool,cfRuleHandler:RefCell<Option<CfRuleHandler>>}
impl ConditionalFormattingHandler{
// java public ConditionalFormattingHandler(KmoSheet sheet, XlsxrWorkSheet reader, boolean isFromExt) {
// java this.sheet = sheet;
// java this.stylesTable = reader.getStylesTable();
// java this.kmoConditionFormatter = sheet.getCFormatter();
// java this.condFmt10 = new KmoConditionFormatterDefine();
// java this.sheet = sheet;
// java this.isFromExt = isFromExt;
// java }
pub fn new(sheet:Rc<RefCell<KmoSheet>>,stylesTable:Rc<RefCell<StylesTable>>,isFromExt:bool)->Self {
Self{stylesTable,condFmt10:Rc::new(KmoConditionFormatterDefine::new()),sheet,cellRangeAddressList:Rc::new(RefCell::new(None)),isFromExt,cfRuleHandler:RefCell::new(None)}
}
}
impl ElementHandler for ConditionalFormattingHandler{
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java List<KmoRect> address = new ArrayList();
// java XmlRoAttr tmp;
// java if ((tmp = attrsNotCopy.getByName(4945)) != null) {
// java this.condFmt10.pivot = tmp.toBoolean();
// java }
// java
// java if (!this.condFmt10.pivot) {
// java if ((tmp = attrsNotCopy.getByName(4735)) != null) {
// java String addr = tmp.toString();
// java String[] addrs = addr.split(" |;");
// java
// java for(String string : addrs) {
// java if (!string.isEmpty()) {
// java CellRangeAddress cs = CellRangeAddress.valueOf(string, this.sheet.getVersion());
// java if (null != cs) {
// java address.add(ExpandHelper.range2rect(cs));
// java }
// java }
// java }
// java }
// java
// java if (address.size() > 0) {
// java this.condFmt10.setRefRangeList(new ArrayList(address));
// java address.clear();
// java }
// java
// java }
// java }
fn startElement(&self,_e:i32,a:&dyn XmlRoAttr){if let Some(a)=a.get_by_name(4945){self.condFmt10.pivot.set(a.to_boolean());}if !self.condFmt10.pivot.get(){let mut addresses=vec![];if let Some(a)=a.get_by_name(4735){for s in a.to_string().split([' ',';']){if !s.is_empty(){if let Some(cs)=CellRangeAddress::valueOf(s,self.sheet.borrow().getVersion()){addresses.push(cs.rect());}}}}if !addresses.is_empty(){self.condFmt10.setRefRangeList(Rc::new(addresses));}}}
// java public ElementHandler enterSubElement(int subElement) {
// java if (this.condFmt10.pivot) {
// java return null;
// java } else {
// java switch (subElement) {
// java case 4735:
// java if (this.cellRangeAddressList == null) {
// java this.cellRangeAddressList = new CellRangeAddressList();
// java }
// java
// java return new SqrefExtHandler(this.cellRangeAddressList);
// java case 4944:
// java if (null == this.cfRuleHandler) {
// java this.cfRuleHandler = new CfRuleHandler(this.stylesTable, this.condFmt10, this.sheet, this.isFromExt);
// java } else {
// java this.cfRuleHandler.init(this.isFromExt);
// java }
// java
// java return this.cfRuleHandler;
// java default:
// java return null;
// java }
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{if self.condFmt10.pivot.get(){return None;}match e{4735=>{if self.cellRangeAddressList.borrow().is_none(){*self.cellRangeAddressList.borrow_mut()=Some(vec![]);}Some(Box::new(SqrefExtHandler{addresses:self.cellRangeAddressList.clone(),sheet:self.sheet.clone()}))},4944=>{let mut h=self.cfRuleHandler.borrow_mut();if let Some(h)=h.as_ref(){h.init(self.isFromExt);}else{*h=Some(CfRuleHandler::new(self.stylesTable.clone(),self.condFmt10.clone(),self.sheet.clone(),self.isFromExt));}Some(Box::new(h.as_ref().unwrap().clone()))},_=>None}}
// java public void endElement(int element) {
// java if (!this.condFmt10.pivot) {
// java if (this.condFmt10.getRefRangeList().size() <= 0 && null != this.cellRangeAddressList) {
// java int size = this.cellRangeAddressList.getSize();
// java if (size > 0) {
// java List<KmoRect> ads = new ArrayList();
// java
// java for(CellRangeAddress address : this.cellRangeAddressList.getCellRangeAddresses()) {
// java ads.add(ExpandHelper.range2rect(address));
// java }
// java
// java this.condFmt10.setRefRangeList(ads);
// java }
// java }
// java
// java if (!this.condFmt10.getRefRangeList().isEmpty()) {
// java if (this.condFmt10 != null && this.condFmt10.ruleList != null && this.condFmt10.ruleList.size() > 0 && ((KmoCfRule)this.condFmt10.ruleList.get(0)).isExt()) {
// java this.kmoConditionFormatter.addConditionCount();
// java }
// java
// java KmoConditionFormatter.turnDefineToCfRuleFromIO(this.condFmt10, this.kmoConditionFormatter);
// java
// java for(KmoCfRule cfRule : this.condFmt10.ruleList) {
// java ArrayList<String> formulas = this.cfRuleHandler.getFormulas();
// java int size = formulas.size();
// java
// java for(int i = 0; i < size; ++i) {
// java String f = (String)formulas.get(i);
// java cfRule.addFormula(f, this.condFmt10.getEnclosingRange(), this.sheet);
// java }
// java }
// java
// java }
// java }
// java }
fn endElement(&self,_e:i32){if self.condFmt10.pivot.get(){return;}if self.condFmt10.getRefRangeList().is_empty(){if let Some(addresses)=self.cellRangeAddressList.borrow().as_ref(){if !addresses.is_empty(){self.condFmt10.setRefRangeList(Rc::new(addresses.iter().map(CellRangeAddress::rect).collect()));}}}if !self.condFmt10.getRefRangeList().is_empty(){let sheet=self.sheet.borrow();let formatter=sheet.getCFormatter();if self.condFmt10.ruleList.borrow().first().is_some_and(|r|r.isExt()){formatter.addConditionCount();}KmoConditionFormatter::turnDefineToCfRuleFromIO(&self.condFmt10,formatter);for rule in self.condFmt10.ruleList.borrow().iter(){let formulas=self.cfRuleHandler.borrow().as_ref().expect("NullPointerException cfRuleHandler").getFormulas();for f in formulas.borrow().iter(){rule.addFormula(f,&self.condFmt10.getEnclosingRange(),&sheet);}}}}fn characters(&self,_s:&str){}
}
struct SqrefExtHandler{addresses:Rc<RefCell<Option<Vec<CellRangeAddress>>>>,sheet:Rc<RefCell<KmoSheet>>}
impl ElementHandler for SqrefExtHandler{fn startElement(&self,_e:i32,_a:&dyn XmlRoAttr){}fn endElement(&self,_e:i32){}fn enterSubElement(&self,_e:i32)->Option<Box<dyn ElementHandler>>{None}
// java public void characters(String chars) {
// java String add = chars;
// java int k = 0;
// java
// java for(int i = 0; i < add.length(); ++i) {
// java if (add.charAt(i) == ' ' || i == add.length() - 1) {
// java String str;
// java if (add.charAt(i) == ' ') {
// java str = add.substring(k, i);
// java k = i + 1;
// java } else {
// java str = add.substring(k, i + 1);
// java }
// java
// java if (str.equals("#REF!")) {
// java this.address = new CellRangeAddress(-1, -1, -1, -1);
// java } else {
// java this.address = CellRangeAddress.valueOf(str, ConditionalFormattingHandler.this.sheet.getVersion());
// java }
// java
// java this.cellRangeAddressList.addCellRangeAddress(this.address.getFirstRow(), this.address.getFirstColumn(), this.address.getLastRow(), this.address.getLastColumn());
// java }
// java }
// java
// java }
fn characters(&self,s:&str){let mut k=0;let bytes=s.as_bytes();for i in 0..bytes.len(){if bytes[i]==b' '||i==bytes.len()-1{let end=if bytes[i]==b' '{i}else{i+1};let text=&s[k..end];if bytes[i]==b' '{k=i+1;}let address=if text=="#REF!"{CellRangeAddress::new(-1,-1,-1,-1)}else{CellRangeAddress::valueOf(text,self.sheet.borrow().getVersion()).expect("NullPointerException address")};self.addresses.borrow_mut().as_mut().unwrap().push(address);}}}}
