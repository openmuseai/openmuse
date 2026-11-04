use crate::java_runtime::{Rc,RefCell,Cell};
use super::super::tables::DataFormatting::DataFormatting;
// java public static enum TableStyleType
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum TableStyleType{wholeTable,headerRow,totalRow,firstColumn,lastColumn,firstRowStripe,secondRowStripe,firstColumnStripe,secondColumnStripe,firstHeaderCell,lastHeaderCell,firstTotalCell,lastTotalCell,firstSubtotalColumn,secondSubtotalColumn,thirdSubtotalColumn,firstSubtotalRow,secondSubtotalRow,thirdSubtotalRow,blankRow,firstColumnSubheading,secondColumnSubheading,thirdColumnSubheading,firstRowSubheading,secondRowSubheading,thirdRowSubheading,pageFieldLabels,pageFieldValues}
impl TableStyleType{pub fn valueOf(s:&str)->Self{match s{"wholeTable"=>Self::wholeTable,"headerRow"=>Self::headerRow,"totalRow"=>Self::totalRow,"firstColumn"=>Self::firstColumn,"lastColumn"=>Self::lastColumn,"firstRowStripe"=>Self::firstRowStripe,"secondRowStripe"=>Self::secondRowStripe,"firstColumnStripe"=>Self::firstColumnStripe,"secondColumnStripe"=>Self::secondColumnStripe,"firstHeaderCell"=>Self::firstHeaderCell,"lastHeaderCell"=>Self::lastHeaderCell,"firstTotalCell"=>Self::firstTotalCell,"lastTotalCell"=>Self::lastTotalCell,"firstSubtotalColumn"=>Self::firstSubtotalColumn,"secondSubtotalColumn"=>Self::secondSubtotalColumn,"thirdSubtotalColumn"=>Self::thirdSubtotalColumn,"firstSubtotalRow"=>Self::firstSubtotalRow,"secondSubtotalRow"=>Self::secondSubtotalRow,"thirdSubtotalRow"=>Self::thirdSubtotalRow,"blankRow"=>Self::blankRow,"firstColumnSubheading"=>Self::firstColumnSubheading,"secondColumnSubheading"=>Self::secondColumnSubheading,"thirdColumnSubheading"=>Self::thirdColumnSubheading,"firstRowSubheading"=>Self::firstRowSubheading,"secondRowSubheading"=>Self::secondRowSubheading,"thirdRowSubheading"=>Self::thirdRowSubheading,"pageFieldLabels"=>Self::pageFieldLabels,"pageFieldValues"=>Self::pageFieldValues,_=>panic!("IllegalArgumentException TableStyleType")}}}
// java public class KmoTableStyles
pub struct KmoTableStyles{tableStyles:RefCell<Option<Rc<RefCell<Vec<Rc<KmoTableStyle>>>>>>,defaultTableStyle:RefCell<Option<String>>,defaultPivotStyle:RefCell<Option<String>>}
impl KmoTableStyles{pub fn new()->Self{Self{tableStyles:RefCell::new(Some(Rc::new(RefCell::new(vec![])))),defaultTableStyle:RefCell::new(None),defaultPivotStyle:RefCell::new(None)}}
// java public List<KmoTableStyle> getTableStyles() {
// java return this.tableStyles;
// java }
pub fn getTableStyles(&self)->Option<Rc<RefCell<Vec<Rc<KmoTableStyle>>>>> {
self.tableStyles.borrow().clone()
}
// java public void setTableStyles(List<KmoTableStyle> tableStyles) {
// java this.tableStyles = tableStyles;
// java }
pub fn setTableStyles(&self,v:Option<Rc<RefCell<Vec<Rc<KmoTableStyle>>>>>) {
*self.tableStyles.borrow_mut()=v;
}
// java public String getDefaultTableStyle() {
// java return this.defaultTableStyle;
// java }
pub fn getDefaultTableStyle(&self)->Option<String> {
self.defaultTableStyle.borrow().clone()
}
// java public void setDefaultTableStyle(String defaultTableStyle) {
// java this.defaultTableStyle = defaultTableStyle;
// java }
pub fn setDefaultTableStyle(&self,v:Option<String>) {
*self.defaultTableStyle.borrow_mut()=v;
}
// java public String getDefaultPivotStyle() {
// java return this.defaultPivotStyle;
// java }
pub fn getDefaultPivotStyle(&self)->Option<String> {
self.defaultPivotStyle.borrow().clone()
}
// java public void setDefaultPivotStyle(String defaultPivotStyle) {
// java this.defaultPivotStyle = defaultPivotStyle;
// java }
pub fn setDefaultPivotStyle(&self,v:Option<String>) {
*self.defaultPivotStyle.borrow_mut()=v;
}
// java public KmoTableStyle getTableStyleByName(String name) {
// java if (null == this.tableStyles) {
// java return null;
// java } else {
// java int i = 0;
// java
// java for(int size = this.tableStyles.size(); i < size; ++i) {
// java KmoTableStyle tableStyle = (KmoTableStyle)this.tableStyles.get(i);
// java if (null != tableStyle && null != tableStyle.name && tableStyle.name.equals(name)) {
// java return tableStyle;
// java }
// java }
// java
// java return null;
// java }
// java }
pub fn getTableStyleByName(&self,name:&str)->Option<Rc<KmoTableStyle>> {
self.getTableStyles().and_then(|v|v.borrow().iter().find(|x|x.getName().as_deref()==Some(name)).cloned())
}
}
// java public static class KmoTableStyle
pub struct KmoTableStyle{tableStyleElements:RefCell<Option<Rc<RefCell<Vec<Rc<KmoTableStyleElement>>>>>>,name:RefCell<Option<String>>,pivot:RefCell<bool>,table:RefCell<bool>}
impl KmoTableStyle{pub fn new()->Self{Self{tableStyleElements:RefCell::new(Some(Rc::new(RefCell::new(vec![])))),name:RefCell::new(None),pivot:RefCell::new(true),table:RefCell::new(true)}}
// java public List<KmoTableStyleElement> getTableStyleElements() {
// java return this.tableStyleElements;
// java }
pub fn getTableStyleElements(&self)->Option<Rc<RefCell<Vec<Rc<KmoTableStyleElement>>>>> {
self.tableStyleElements.borrow().clone()
}
// java public void setTableStyleElements(List<KmoTableStyleElement> tableStyleElements) {
// java this.tableStyleElements = tableStyleElements;
// java }
pub fn setTableStyleElements(&self,v:Option<Rc<RefCell<Vec<Rc<KmoTableStyleElement>>>>>) {
*self.tableStyleElements.borrow_mut()=v;
}
// java public String getName() {
// java return this.name;
// java }
pub fn getName(&self)->Option<String> {
self.name.borrow().clone()
}
// java public void setName(String name) {
// java this.name = name;
// java }
pub fn setName(&self,v:Option<String>) {
*self.name.borrow_mut()=v;
}
// java public boolean isPivot() {
// java return this.pivot;
// java }
pub fn isPivot(&self)->bool {
self.pivot.borrow().clone()
}
// java public void setPivot(boolean pivot) {
// java this.pivot = pivot;
// java }
pub fn setPivot(&self,v:bool) {
*self.pivot.borrow_mut()=v;
}
// java public boolean isTable() {
// java return this.table;
// java }
pub fn isTable(&self)->bool {
self.table.borrow().clone()
}
// java public void setTable(boolean table) {
// java this.table = table;
// java }
pub fn setTable(&self,v:bool) {
*self.table.borrow_mut()=v;
}
// java public KmoTableStyleElement getTableStyleElement(TableStyleType type) {
// java if (null == this.tableStyleElements) {
// java return null;
// java } else {
// java int i = 0;
// java
// java for(int size = this.tableStyleElements.size(); i < size; ++i) {
// java KmoTableStyleElement tableStyleElement = (KmoTableStyleElement)this.tableStyleElements.get(i);
// java if (null != tableStyleElement && type == tableStyleElement.getType()) {
// java return tableStyleElement;
// java }
// java }
// java
// java return null;
// java }
// java }
pub fn getTableStyleElement(&self,t:TableStyleType)->Option<Rc<KmoTableStyleElement>> {
self.getTableStyleElements().and_then(|v|v.borrow().iter().find(|x|x.getType()==Some(t)).cloned())
}
}
// java public static class KmoTableStyleElement
pub struct KmoTableStyleElement{rtype:RefCell<Option<TableStyleType>>,size:RefCell<i32>,dxfId:RefCell<i32>,dataFormatting:RefCell<Option<Rc<DataFormatting>>>}
impl KmoTableStyleElement{pub fn new()->Self{Self{rtype:RefCell::new(None),size:RefCell::new(1),dxfId:RefCell::new(0),dataFormatting:RefCell::new(None)}}
// java public TableStyleType getType() {
// java return this.type;
// java }
pub fn getType(&self)->Option<TableStyleType> {
self.rtype.borrow().clone()
}
// java public void setType(TableStyleType type) {
// java this.type = type;
// java }
pub fn setType(&self,v:Option<TableStyleType>) {
*self.rtype.borrow_mut()=v;
}
// java public int getSize() {
// java return this.size;
// java }
pub fn getSize(&self)->i32 {
self.size.borrow().clone()
}
// java public void setSize(int size) {
// java this.size = size;
// java }
pub fn setSize(&self,v:i32) {
*self.size.borrow_mut()=v;
}
// java public int getDxfId() {
// java return this.dxfId;
// java }
pub fn getDxfId(&self)->i32 {
self.dxfId.borrow().clone()
}
// java public void setDxfId(int dxfId) {
// java this.dxfId = dxfId;
// java }
pub fn setDxfId(&self,v:i32) {
*self.dxfId.borrow_mut()=v;
}
// java public DataFormatting getDataFormatting() {
// java return null == this.dataFormatting ? new DataFormatting() : this.dataFormatting;
// java }
pub fn getDataFormatting(&self)->Rc<DataFormatting> {
self.dataFormatting.borrow().clone().unwrap_or_else(||Rc::new(DataFormatting::new()))
}
// java public void setDataFormatting(DataFormatting dataFormatting) {
// java this.dataFormatting = dataFormatting;
// java }
pub fn setDataFormatting(&self,v:Option<Rc<DataFormatting>>) {
*self.dataFormatting.borrow_mut()=v;
}
}
