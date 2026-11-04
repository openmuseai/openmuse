use crate::{java_runtime::{Rc,RefCell,Cell},cn::wps::moss::app::KmoSheet::KmoSheet,org::apache::poi::ss::util::CellReference::CellReference};use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};use super::ActivePaneMap::ActivePaneMap;
use crate::cn::wps::moss::{xlsxr::util::ValueHelper::ValueHelper,filefmt::biff8::{record::SelectionRecord::SelectionRecord,util::CellRangeAddress::CellRangeAddress},core::sheet::KmoSelection::KmoSelection,io::ExcelConst::ExcelConst};
// java public class SheetViewHandler extends DefaultElementHandler
pub struct SheetViewHandler{sheet:Rc<RefCell<KmoSheet>>,havaPane:Rc<Cell<bool>>}
impl SheetViewHandler{pub fn new(sheet:Rc<RefCell<KmoSheet>>)->Self{Self{sheet,havaPane:Rc::new(Cell::new(false))}}}
impl ElementHandler for SheetViewHandler{
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java KmoSheet sheet = this.reader.getSheet();
// java XmlRoAttr tmp;
// java if ((tmp = attrsNotCopy.getByName(4857)) != null) {
// java int row = ValueHelper.getRowIndex(tmp.toString());
// java if (row < sheet.getMaxRows()) {
// java sheet.setLeftTopRow(row);
// java }
// java
// java int col = ValueHelper.getColumnIndex(tmp.toString());
// java if (col < sheet.getMaxColumns()) {
// java sheet.setLeftTopCol(col);
// java }
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4850)) != null) {
// java sheet.setSheetRightToLeft(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4847)) != null) {
// java sheet.setIsDrawGridLines(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4848)) != null) {
// java sheet.setIsDrawHeader(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4854)) != null) {
// java sheet.setDefaultHdr(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4858)) != null) {
// java sheet.setHeaderColor(tmp.toInt());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4859)) != null) {
// java sheet.setScale((short)tmp.toInt(), (short)100);
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4851)) != null) {
// java sheet.setSelected(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4853)) != null) {
// java sheet.setIsDisplayGuts(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4849)) != null) {
// java sheet.setIsShowZeros(tmp.toBoolean());
// java }
// java
// java }
fn startElement(&self,_e:i32,a:&dyn XmlRoAttr){let sheet=self.sheet.borrow();if let Some(v)=a.get_by_name(4857){let text=v.to_string();let row=ValueHelper::getRowIndex(&text);if row<sheet.getMaxRows(){sheet.setLeftTopRow(row);}let col=ValueHelper::getColumnIndex(&text);if col<sheet.getMaxColumns(){sheet.setLeftTopCol(col);}}
if let Some(v)=a.get_by_name(4850){sheet.setSheetRightToLeft(v.to_boolean());}
if let Some(v)=a.get_by_name(4847){sheet.setIsDrawGridLines(v.to_boolean());}
if let Some(v)=a.get_by_name(4848){sheet.setIsDrawHeader(v.to_boolean());}
if let Some(v)=a.get_by_name(4854){sheet.setDefaultHdr(v.to_boolean());}
if let Some(v)=a.get_by_name(4851){sheet.setSelected(v.to_boolean());}
if let Some(v)=a.get_by_name(4853){sheet.setIsDisplayGuts(v.to_boolean());}
if let Some(v)=a.get_by_name(4849){sheet.setIsShowZeros(v.to_boolean());}
if let Some(v)=a.get_by_name(4858){sheet.setHeaderColor(v.to_int());}if let Some(v)=a.get_by_name(4859){sheet.setScale(v.to_int()as i16,100);}}
// java public ElementHandler enterSubElement(int subElement) {
// java if (subElement == 4842) {
// java this.havaPane = true;
// java return new PaneHandler(this.reader);
// java } else {
// java return subElement == 4843 ? this.selectionHandler : null;
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{match e{4842=>{self.havaPane.set(true);Some(Box::new(super::PaneHandler::PaneHandler::new(self.sheet.clone())))},4843=>Some(Box::new(SelectionHandler{sheet:self.sheet.clone(),havaPane:self.havaPane.clone()})),_=>None}}fn endElement(&self,_e:i32){}fn characters(&self,_s:&str){}
}
struct SelectionHandler{sheet:Rc<RefCell<KmoSheet>>,havaPane:Rc<Cell<bool>>}
impl ElementHandler for SelectionHandler{
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java KmoSheet sheet = SheetViewHandler.this.reader.getSheet();
// java byte pane = 3;
// java XmlRoAttr tmp;
// java if ((tmp = attrsNotCopy.getByName(4842)) != null) {
// java pane = ActivePaneMap.getPane(tmp.toString());
// java }
// java
// java if (!SheetViewHandler.this.havaPane || pane == sheet.getActivePane()) {
// java ArrayList<CellRangeAddress> address = new ArrayList();
// java int col = 0;
// java int row = 0;
// java SelectionRecord selectionRecord = null;
// java if ((tmp = attrsNotCopy.getByName(4877)) != null) {
// java String str = tmp.toString();
// java row = this.getCellRow(str);
// java col = this.getCellColumn(str);
// java if (row >= sheet.getMaxRows() || col > sheet.getMaxColumns() - 1 || row < 0 || col < 0) {
// java return;
// java }
// java
// java selectionRecord = new SelectionRecord(row, col);
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4735)) != null) {
// java if (selectionRecord == null) {
// java selectionRecord = new SelectionRecord(0, 0);
// java }
// java
// java String add = tmp.toString();
// java int j = 0;
// java
// java for(int i = 0; i < add.length(); ++i) {
// java if (add.charAt(i) == ' ' || add.charAt(i) == ';') {
// java String str = add.substring(j, i);
// java j = i + 1;
// java CellRangeAddress cs = CellRangeAddress.valueOf(str, sheet.getVersion());
// java address.add(ExcelConst.adaptRegion(cs, sheet.getVersion()));
// java }
// java
// java if (i == add.length() - 1) {
// java String str = add.substring(j, i + 1);
// java CellRangeAddress cs = CellRangeAddress.valueOf(str, sheet.getVersion());
// java address.add(ExcelConst.adaptRegion(cs, sheet.getVersion()));
// java }
// java }
// java
// java selectionRecord.setPane(pane);
// java int ref = 0;
// java int i = 0;
// java
// java for(int len = address.size(); i < len; ++i) {
// java if (((CellRangeAddress)address.get(i)).isInRange(row, col)) {
// java ref = i;
// java break;
// java }
// java }
// java
// java selectionRecord.setRefs(address);
// java selectionRecord.setActiveCellRef(ref);
// java selectionRecord.setActiveCellRow(row);
// java selectionRecord.setActiveCellCol((short)col);
// java KmoSelection kmoSelection = KmoSelection.fromRecord(selectionRecord);
// java sheet.setSelection(kmoSelection);
// java SelectionRecord var18 = null;
// java }
// java
// java }
// java }
fn startElement(&self,_e:i32,a:&dyn XmlRoAttr){let sheet=self.sheet.borrow();let mut pane=3;if let Some(v)=a.get_by_name(4842){pane=ActivePaneMap::getPane(Some(&v.to_string()));}if !self.havaPane.get()||pane as i32==sheet.getActivePane(){let(mut row,mut col)=(0,0);let mut rec=None;if let Some(v)=a.get_by_name(4877){let cell=CellReference::fromString(&v.to_string()).expect("selection cell");row=cell.getRow();col=cell.getCol()as i32;if row>=sheet.getMaxRows()||col>sheet.getMaxColumns()-1||row<0||col<0{return;}rec=Some(SelectionRecord::new(row,col));}if let Some(v)=a.get_by_name(4735){let mut rec=rec.unwrap_or_else(||SelectionRecord::new(0,0));let text=v.to_string();let units=text.encode_utf16().collect::<Vec<_>>();let mut ranges=vec![];let mut j=0;for i in 0..units.len(){if units[i]==32||units[i]==59{let part=String::from_utf16(&units[j..i]).expect("UTF16 selection");j=i+1;let range=CellRangeAddress::valueOf(&part,sheet.getVersion()).expect("selection range");ranges.push(ExcelConst::adaptRegion(range,sheet.getVersion()));}if i==units.len()-1{let part=String::from_utf16(&units[j..i+1]).expect("UTF16 selection");let range=CellRangeAddress::valueOf(&part,sheet.getVersion()).expect("selection range");ranges.push(ExcelConst::adaptRegion(range,sheet.getVersion()));}}rec.setPane(pane);let index=ranges.iter().position(|r|r.isInRange(row,col)).unwrap_or(0);rec.setRefs(ranges);rec.setActiveCellRef(index as i32);rec.setActiveCellRow(row);rec.setActiveCellCol(col as i16);sheet.setSelection(Rc::new(KmoSelection::fromRecord(&rec)));}}}
fn endElement(&self,_e:i32){}fn characters(&self,_s:&str){}fn enterSubElement(&self,_e:i32)->Option<Box<dyn ElementHandler>>{None}
}
