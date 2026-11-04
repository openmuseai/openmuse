use crate::{java_runtime::{Rc,RefCell,Cell},cn::wps::moss::engine::linktb::LinkTable::CRNBlock};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
use super::{LinkRowHandler::{LinkRowHandler,CRNRecords},native::{Subclass,share}};
pub type CRNBlocks=Rc<RefCell<Vec<Option<Rc<CRNBlock>>>>>;
// java public final class LinkSheetDataHandler extends DefaultElementHandler {
pub struct LinkSheetDataHandler{
    // java private int sheetId;
    sheetId:Cell<i32>,
    // java private List<CRNRecord> crnRecs;
    crnRecs:RefCell<Option<CRNRecords>>,
    // java private LinkRowHandler rowHandler;
    rowHandler:RefCell<Option<Rc<LinkRowHandler>>>,
    // java private List<LinkTable.CRNBlock> crnBlocks;
    crnBlocks:Option<CRNBlocks>,
}
impl LinkSheetDataHandler{
    // java public LinkSheetDataHandler(List<LinkTable.CRNBlock> crnBlocks){this.crnBlocks=crnBlocks;}
    pub fn LinkSheetDataHandler(crnBlocks:Option<CRNBlocks>)->Self{Self{sheetId:Cell::new(0),crnRecs:RefCell::new(None),rowHandler:RefCell::new(None),crnBlocks}}
}
impl Subclass for LinkSheetDataHandler{
    // java public void startElement(int element,XmlRoAttr attrsNotCopy){this.sheetId=attrsNotCopy.getByName(4704).toInt();this.crnRecs=new ArrayList();this.rowHandler=new LinkRowHandler(this.crnRecs);}
    fn startElement(&self,_element:i32,attrsNotCopy:&dyn XmlRoAttr){self.sheetId.set(attrsNotCopy.get_by_name(4704).expect("NullPointerException").to_int());*self.crnRecs.borrow_mut()=Some(Rc::new(RefCell::new(vec![])));*self.rowHandler.borrow_mut()=Some(LinkRowHandler::LinkRowHandler(self.crnRecs.borrow().clone()));}
    // java public ElementHandler enterSubElement(int subElement){return subElement==4802?this.rowHandler:null;}
    fn enterSubElement(&self,subElement:i32)->Option<Box<dyn ElementHandler>>{if subElement==4802{self.rowHandler.borrow().clone().map(share)}else{None}}
    // java public void endElement(int element){LinkTable.CRNBlock crnBlock=LinkTable.CRNBlock.createCRNBlock(this.sheetId,this.crnRecs);this.crnBlocks.add(crnBlock);this.crnRecs.clear();}
    fn endElement(&self,_element:i32){let crnBlock=CRNBlock::createCRNBlock(self.sheetId.get(),self.crnRecs.borrow().as_ref().map(|v|v.borrow()).as_deref().map(|v|v.as_slice()));self.crnBlocks.as_ref().expect("NullPointerException").borrow_mut().push(Some(Rc::new(crnBlock)));self.crnRecs.borrow().as_ref().expect("NullPointerException").borrow_mut().clear();}
}
