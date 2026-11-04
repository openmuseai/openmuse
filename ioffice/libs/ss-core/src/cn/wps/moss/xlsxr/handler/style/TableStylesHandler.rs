use crate::java_runtime::{Rc,RefCell,Cell};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};use crate::cn::wps::moss::{app::{KmoBook::KmoBook,xf::KmoTableStyles::{KmoTableStyles,KmoTableStyle,KmoTableStyleElement,TableStyleType},tables::DataFormatting::DataFormatting},xlsxr::model::StylesTable::StylesTable,filefmt::biff8::record::cf::{FontFormatting::FontFormatting,NumberFormatFormatting::NumberFormatFormatting}};use crate::cn::wps::moss::xlsxr::handler::sheet::CfRuleHandler::CfRuleHandler;
#[derive(Clone)]pub struct TableStylesHandler{book:Rc<KmoBook>,styles:Rc<RefCell<StylesTable>>,table:Rc<KmoTableStyles>}impl TableStylesHandler{
// java public TableStylesHandler(KmoBook kmoBook, StylesTable stylesTable) {
// java this.stylesTable = stylesTable;
// java this.kmoXFManager = kmoBook.getXFManager();
// java kmoBook.getXFManager().setKmoTableStyles(this.kmoTableStyles);
// java }
pub fn new(book:Rc<KmoBook>,styles:Rc<RefCell<StylesTable>>)->Self {
let table=Rc::new(KmoTableStyles::new());book.getXFManager().borrow().setKmoTableStyles(Some(table.clone()));Self{book,styles,table}
}
}
impl ElementHandler for TableStylesHandler{// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java this.kmoTableStyles.setDefaultPivotStyle(XmlRoAttrUtils.getString(5183, attrsNotCopy));
// java this.kmoTableStyles.setDefaultTableStyle(XmlRoAttrUtils.getString(5182, attrsNotCopy));
// java }
fn startElement(&self,_:i32,a:&dyn XmlRoAttr){self.table.setDefaultPivotStyle(a.get_by_name(5183).map(|v|v.to_string()));self.table.setDefaultTableStyle(a.get_by_name(5182).map(|v|v.to_string()));}// java public ElementHandler enterSubElement(int subElement) {
// java if (subElement == 5181) {
// java KmoTableStyles.KmoTableStyle tableStyle = new KmoTableStyles.KmoTableStyle();
// java this.kmoTableStyles.getTableStyles().add(tableStyle);
// java return new TableStyleHandler(tableStyle);
// java } else {
// java return null;
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{if e==5181{let style=Rc::new(KmoTableStyle::new());self.table.getTableStyles().unwrap().borrow_mut().push(style.clone());Some(Box::new(TableStyleHandler{owner:self.clone(),style}))}else{None}}fn endElement(&self,_:i32){}fn characters(&self,_:&str){}}
struct TableStyleHandler{owner:TableStylesHandler,style:Rc<KmoTableStyle>}impl ElementHandler for TableStyleHandler{// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java this.tableStyle.setName(XmlRoAttrUtils.getString(4217, attrsNotCopy));
// java this.tableStyle.setPivot(XmlRoAttrUtils.getBoolean(4945, attrsNotCopy, true));
// java this.tableStyle.setTable(XmlRoAttrUtils.getBoolean(5185, attrsNotCopy, true));
// java }
fn startElement(&self,_:i32,a:&dyn XmlRoAttr){self.style.setName(a.get_by_name(4217).map(|v|v.to_string()));self.style.setPivot(a.get_by_name(4945).map(|v|v.to_boolean()).unwrap_or(true));self.style.setTable(a.get_by_name(5185).map(|v|v.to_boolean()).unwrap_or(true));}// java public ElementHandler enterSubElement(int subElement) {
// java if (subElement == 5184) {
// java KmoTableStyles.KmoTableStyleElement styleElement = new KmoTableStyles.KmoTableStyleElement();
// java this.tableStyle.getTableStyleElements().add(styleElement);
// java return TableStylesHandler.this.new TableStyleElementHandler(styleElement);
// java } else {
// java return null;
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{if e==5184{let style=Rc::new(KmoTableStyleElement::new());self.style.getTableStyleElements().unwrap().borrow_mut().push(style.clone());Some(Box::new(TableStyleElementHandler{owner:self.owner.clone(),style}))}else{None}}fn endElement(&self,_:i32){}fn characters(&self,_:&str){}}
struct TableStyleElementHandler{owner:TableStylesHandler,style:Rc<KmoTableStyleElement>}impl ElementHandler for TableStyleElementHandler{// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java this.styleElement.setType(KmoTableStyles.TableStyleType.valueOf(XmlRoAttrUtils.getString(4125, attrsNotCopy)));
// java this.styleElement.setSize(XmlRoAttrUtils.getInt(5186, attrsNotCopy, 1));
// java this.styleElement.setDxfId(XmlRoAttrUtils.getInt(4121, attrsNotCopy, -1));
// java }
fn startElement(&self,_:i32,a:&dyn XmlRoAttr){self.style.setType(Some(TableStyleType::valueOf(&a.get_by_name(4125).map(|v|v.to_string()).expect("null TableStyleType"))));self.style.setSize(a.get_by_name(5186).map(|v|v.to_int()).unwrap_or(1));self.style.setDxfId(a.get_by_name(4121).map(|v|v.to_int()).unwrap_or(-1));}fn enterSubElement(&self,_:i32)->Option<Box<dyn ElementHandler>>{None}fn characters(&self,_:&str){}// java public void endElement(int element) {
// java int dxfId = this.styleElement.getDxfId();
// java if (dxfId != -1) {
// java DataFormatting dataFormatting = new DataFormatting();
// java this.styleElement.setDataFormatting(dataFormatting);
// java int bordid = -1;
// java int patid = -1;
// java int fontid = -1;
// java int numFmtId = -1;
// java DxfPropertyId dxf = TableStylesHandler.this.stylesTable.getdxf(dxfId);
// java if (null != dxf) {
// java bordid = dxf.getBordid();
// java patid = dxf.getPatid();
// java fontid = dxf.getFontid();
// java numFmtId = dxf.getNumFmtId();
// java }
// java
// java if (patid != -1) {
// java PatternFormatting pa = TableStylesHandler.this.stylesTable.getPatternFillAt(patid);
// java if (pa != null) {
// java dataFormatting.setPatternFormatting(pa);
// java }
// java }
// java
// java if (bordid != -1) {
// java BorderFormatting bo = TableStylesHandler.this.stylesTable.getBorderAt(bordid);
// java if (bo != null) {
// java CfRuleHandler.dealBorderFormatting(bo);
// java dataFormatting.setBorderFormatting(bo);
// java }
// java }
// java
// java if (fontid != -1) {
// java FontFormatting font = new FontFormatting();
// java CfRuleHandler.iniFontFormat(TableStylesHandler.this.stylesTable, font, fontid);
// java dataFormatting.setFontFormatting(font);
// java }
// java
// java if (numFmtId != -1) {
// java KmoFormat format = TableStylesHandler.this.kmoXFManager.getFormat(numFmtId);
// java String fmtCode = null;
// java if (format != null) {
// java fmtCode = format.getFormatString();
// java }
// java
// java NumberFormatFormatting numFmt = new NumberFormatFormatting(numFmtId, fmtCode);
// java dataFormatting.setNumberFormatFormatting(numFmt);
// java XmlStringUtils.formatNumFmtCode2Java(numFmt);
// java }
// java
// java }
// java }
fn endElement(&self,_:i32){let id=self.style.getDxfId();if id== -1{return;}let df=Rc::new(DataFormatting::new());self.style.setDataFormatting(Some(df.clone()));let table=self.owner.styles.borrow();let Some(dxf)=table.getdxf(id)else{return;};if dxf.getPatid()!= -1{df.setPatternFormatting(table.getPatternFillAt(dxf.getPatid()));}if dxf.getBordid()!= -1{if let Some(b)=table.getBorderAt(dxf.getBordid()){CfRuleHandler::dealBorderFormatting(&mut b.borrow_mut());df.setBorderFormatting(Some(b));}}if dxf.getFontid()!= -1{let font=Rc::new(FontFormatting::new());CfRuleHandler::iniFontFormat(&table,&font,dxf.getFontid());df.setFontFormatting(Some(font));}if dxf.getNumFmtId()!= -1{let code=self.owner.book.getXFManager().borrow().getFormat(dxf.getNumFmtId()).and_then(|f|f.getFormatString());let mut numfmt=NumberFormatFormatting::new(dxf.getNumFmtId(),code.clone());numfmt.setFmtCode(code.map(|s|crate::cn::wps::moss::app::pivot::util::XmlStringUtils::XmlStringUtils::xml2JavaString(&s)));df.setNumberFormatFormatting(Some(Rc::new(numfmt)));}}}
