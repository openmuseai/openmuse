use crate::{java_runtime::{Rc,RefCell,Cell},cn::wps::moss::app::KmoSheet::KmoSheet,org::apache::poi::ss::util::CellReference::CellReference};use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};use super::ActivePaneMap::ActivePaneMap;
#[derive(Clone)]pub struct PaneHandler{sheet:Rc<RefCell<KmoSheet>>,frozen:Rc<Cell<bool>>,ap:Rc<RefCell<Option<String>>>,xSplit:Rc<Cell<i32>>,ySplit:Rc<Cell<i32>>}
impl PaneHandler{pub fn new(sheet:Rc<RefCell<KmoSheet>>)->Self{Self{sheet,frozen:Rc::new(Cell::new(false)),ap:Rc::new(RefCell::new(None)),xSplit:Rc::new(Cell::new(0)),ySplit:Rc::new(Cell::new(0))}}}
impl ElementHandler for PaneHandler{
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java this.frozen = false;
// java this.xSplit = 0;
// java this.ySplit = 0;
// java XmlRoAttr tmp;
// java if ((tmp = attrsNotCopy.getByName(4864)) != null) {
// java this.xSplit = (int)tmp.toDouble();
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4865)) != null) {
// java this.ySplit = (int)tmp.toDouble();
// java }
// java
// java int column = 1;
// java int row = 1;
// java if ((tmp = attrsNotCopy.getByName(4857)) != null) {
// java CellReference cellRef = new CellReference(tmp.toString());
// java column = cellRef.getCol();
// java row = cellRef.getRow();
// java }
// java
// java if (column < this.sheet.getMaxColumns() && row < this.sheet.getMaxRows()) {
// java byte activePane = 3;
// java if ((tmp = attrsNotCopy.getByName(4866)) != null) {
// java this.ap = tmp.toString();
// java activePane = ActivePaneMap.getPane(this.ap);
// java }
// java
// java String state = "split";
// java if ((tmp = attrsNotCopy.getByName(4867)) != null) {
// java state = tmp.toString();
// java }
// java
// java if (state.equals("split")) {
// java this.sheet.initPaneInfo(false, false, this.xSplit, this.ySplit, row, column, activePane);
// java } else if (state.equals("frozen")) {
// java this.frozen = true;
// java this.sheet.initPaneInfo(true, true, this.xSplit, this.ySplit, row, column, activePane);
// java } else if (state.equals("frozenSplit")) {
// java this.frozen = true;
// java this.sheet.initPaneInfo(true, false, this.xSplit, this.ySplit, row, column, activePane);
// java }
// java
// java }
// java }
fn startElement(&self,_e:i32,a:&dyn XmlRoAttr){self.frozen.set(false);self.xSplit.set(0);self.ySplit.set(0);if let Some(v)=a.get_by_name(4864){self.xSplit.set(v.to_double()as i32);}if let Some(v)=a.get_by_name(4865){self.ySplit.set(v.to_double()as i32);}let(mut col,mut row)=(1,1);if let Some(v)=a.get_by_name(4857){let cell=CellReference::fromString(&v.to_string()).expect("Pane cell reference");col=cell.getCol()as i32;row=cell.getRow();}let sheet=self.sheet.borrow();if col<sheet.getMaxColumns()&&row<sheet.getMaxRows(){let mut active=3;if let Some(v)=a.get_by_name(4866){let text=v.to_string();active=ActivePaneMap::getPane(Some(&text));*self.ap.borrow_mut()=Some(text);}let state=a.get_by_name(4867).map(|v|v.to_string()).unwrap_or_else(||"split".into());match state.as_str(){"split"=>sheet.initPaneInfo(false,false,self.xSplit.get(),self.ySplit.get(),row,col,active as i32),"frozen"=>{self.frozen.set(true);sheet.initPaneInfo(true,true,self.xSplit.get(),self.ySplit.get(),row,col,active as i32);},"frozenSplit"=>{self.frozen.set(true);sheet.initPaneInfo(true,false,self.xSplit.get(),self.ySplit.get(),row,col,active as i32);},_=>{}}}}
fn endElement(&self,_e:i32){}fn characters(&self,_s:&str){}fn enterSubElement(&self,_e:i32)->Option<Box<dyn ElementHandler>>{Some(Box::new(self.clone()))}
}
