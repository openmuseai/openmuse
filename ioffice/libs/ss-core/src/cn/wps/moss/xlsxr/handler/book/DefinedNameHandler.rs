use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::engine::linktb::NameRecord::{NameRecord,NameRecordRef},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
use super::BuiltInNameMap::BuiltInNameMap;
// java public final class DefinedNameHandler extends DefaultElementHandler {
pub struct DefinedNameHandler{
    // java private NameRecordCache mRecordCache;
    mRecordCache:Rc<RefCell<NameRecordCache>>,
}
impl DefinedNameHandler{
    // java DefinedNameHandler(NameRecordCache recordCache) {this.mRecordCache=recordCache;}
    pub(super) fn DefinedNameHandler(recordCache:Rc<RefCell<NameRecordCache>>)->Self{Self{mRecordCache:recordCache}}
    // java public void startElement(int element, XmlRoAttr attrsNotCopy) {
    pub fn startElement(&self,_element:i32,attrsNotCopy:&dyn XmlRoAttr){
        // java int sheetId=0;
        let mut sheetId=0;
        // java if ((tmp=attrsNotCopy.getByName(4745))!=null) {sheetId=tmp.toInt()+1;}
        if let Some(tmp)=attrsNotCopy.get_by_name(4745){sheetId=tmp.to_int().wrapping_add(1);}
        // java String str=attrsNotCopy.getByName(4217).toString();
        let str=attrsNotCopy.get_by_name(4217).expect("NullPointerException").to_string();
        // java if (BuiltInNameMap.hasBuiltIn(str)) {
        let mut nameRec=if BuiltInNameMap::hasBuiltIn(Some(&str)){
            // java byte builtin=BuiltInNameMap.getBuiltIn(str);
            let builtin=BuiltInNameMap::getBuiltIn(Some(&str));
            // java nameRec=new NameRecord(builtin,sheetId,SpreadsheetVersion.EXCEL2007);
            NameRecord::NameRecord__builtin(builtin,sheetId,SpreadsheetVersion::EXCEL2007)
        }else{
            // java nameRec=new NameRecord(SpreadsheetVersion.EXCEL2007);
            let mut nameRec=NameRecord::NameRecord(SpreadsheetVersion::EXCEL2007);
            // java nameRec.setNameText(str);
            nameRec.setNameText(&str);
            // java if(sheetId>0){nameRec.setSheetNumber(sheetId);}
            if sheetId>0{nameRec.setSheetNumber(sheetId);}
            nameRec
        };
        // java if ((tmp=attrsNotCopy.getByName(4407))!=null && tmp.toBoolean()) {nameRec.setHidden(true);}
        if let Some(tmp)=attrsNotCopy.get_by_name(4407){if tmp.to_boolean(){nameRec.setHidden(true);}}
        // java this.mRecordCache.nameRec=nameRec;
        self.mRecordCache.borrow_mut().nameRec=Some(Rc::new(RefCell::new(nameRec)));
    }
    // java public void characters(String chars) {this.mRecordCache.formula=chars;}
    pub fn characters(&self,chars:&str){self.mRecordCache.borrow_mut().formula=Some(chars.to_string());}
}
impl ElementHandler for DefinedNameHandler{
    fn startElement(&self,e:i32,a:&dyn XmlRoAttr){self.startElement(e,a);}
    fn characters(&self,s:&str){self.characters(s);}
    // java inherited DefaultElementHandler.endElement / enterSubElement.
    fn endElement(&self,_e:i32){}
    fn enterSubElement(&self,_e:i32)->Option<Box<dyn ElementHandler>>{None}
}
// java static class NameRecordCache {
pub(super) struct NameRecordCache{
    // java NameRecord nameRec=null;
    pub nameRec:Option<NameRecordRef>,
    // java String formula=null;
    pub formula:Option<String>,
    // java boolean bImported=false;
    pub bImported:bool,
}
impl NameRecordCache{pub fn new()->Self{Self{nameRec:None,formula:None,bImported:false}}}
