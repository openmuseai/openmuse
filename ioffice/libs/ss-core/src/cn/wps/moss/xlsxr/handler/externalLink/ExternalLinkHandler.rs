use crate::{java_runtime::{Rc,Weak,RefCell},cn::wps::moss::engine::linktb::{LinkTable::ExternalBookBlock,SupBookRecord::SupBookRecord}};
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use super::{LinkSheetNamesHandler::LinkSheetNamesHandler,LinkSheetDataHandler::{LinkSheetDataHandler,CRNBlocks},native::{Subclass,hold,share}};
// java public final class ExternalLinkHandler extends DefaultElementHandler {
pub struct ExternalLinkHandler{
    // java private String targetUri;
    targetUri:Option<String>,
    // java private LinkTable.ExternalBookBlock extBook;
    extBook:RefCell<Option<Rc<ExternalBookBlock>>>,
    // java private LinkSheetNamesHandler sheetNamesHandler;
    sheetNamesHandler:Rc<LinkSheetNamesHandler>,
    // java private List<LinkTable.CRNBlock> crnBlocks=new ArrayList();
    crnBlocks:CRNBlocks,
    // java private LinkSheetDataHandler sheetDataHandler;
    sheetDataHandler:RefCell<Option<Rc<LinkSheetDataHandler>>>,
    selfRef:Weak<Self>,
}
impl ExternalLinkHandler{
    // java public ExternalLinkHandler(String targetUri){this.targetUri=targetUri;this.sheetNamesHandler=new LinkSheetNamesHandler();}
    pub fn ExternalLinkHandler(targetUri:Option<String>)->Rc<Self>{Rc::new_cyclic(|s|Self{targetUri,extBook:RefCell::new(None),sheetNamesHandler:LinkSheetNamesHandler::new(),crnBlocks:Rc::new(RefCell::new(vec![])),sheetDataHandler:RefCell::new(None),selfRef:s.clone()})}
}
impl Subclass for ExternalLinkHandler{
    // java public ElementHandler enterSubElement(int subElement){return subElement==5195?new ExternalBookHandler():null;}
    fn enterSubElement(&self,subElement:i32)->Option<Box<dyn ElementHandler>>{if subElement==5195{Some(hold(Rc::new(ExternalBookHandler{outer:self.selfRef.clone()}),self.selfRef.upgrade().expect("ExternalLinkHandler owner")))}else{None}}
    // java public void endElement(int element){
    fn endElement(&self,_element:i32){
        // java List<String> sheetNames=this.sheetNamesHandler.getSheetNames();String[] sheetNamesCopy=new String[sheetNames.size()];sheetNames.toArray(sheetNamesCopy);sheetNames.clear();
        let sheetNames=self.sheetNamesHandler.getSheetNames();let sheetNamesCopy=sheetNames.borrow().clone();sheetNames.borrow_mut().clear();
        // java SupBookRecord bookRec=SupBookRecord.createExternalReferences(this.targetUri,sheetNamesCopy);this.extBook=new LinkTable.ExternalBookBlock(bookRec,this.crnBlocks);
        let bookRec=SupBookRecord::createExternalReferences(self.targetUri.as_deref(),Some(Rc::new(RefCell::new(sheetNamesCopy))));*self.extBook.borrow_mut()=Some(Rc::new(ExternalBookBlock::ExternalBookBlock(Some(Rc::new(bookRec)),Some(&self.crnBlocks.borrow()))));
    }
}
impl ExternalLinkHandler{
    // java public LinkTable.ExternalBookBlock getBookBlock(){return this.extBook;}
    pub fn getBookBlock(&self)->Option<Rc<ExternalBookBlock>>{self.extBook.borrow().clone()}
    // Native reference transport into the existing boxed SAX facade.
    pub fn asElementHandler(self:&Rc<Self>)->Box<dyn ElementHandler>{share(self.clone())}
}
// java private final class ExternalBookHandler extends DefaultElementHandler {
struct ExternalBookHandler{outer:Weak<ExternalLinkHandler>}
impl Subclass for ExternalBookHandler{
    // java public ElementHandler enterSubElement(int subElement){switch(subElement){case 5198:return ExternalLinkHandler.this.sheetNamesHandler;case 5199:default:return null;case 5200:return ExternalLinkHandler.this.new SheetDataSetHandler();}}
    fn enterSubElement(&self,subElement:i32)->Option<Box<dyn ElementHandler>>{let outer=self.outer.upgrade().expect("ExternalLinkHandler owner");match subElement{5198=>Some(share(outer.sheetNamesHandler.clone())),5200=>Some(hold(Rc::new(SheetDataSetHandler::SheetDataSetHandler(&outer)),outer)),_=>None}}
}
// java private final class SheetDataSetHandler extends DefaultElementHandler {
struct SheetDataSetHandler{outer:Weak<ExternalLinkHandler>}
impl SheetDataSetHandler{
    // java public SheetDataSetHandler(){ExternalLinkHandler.this.sheetDataHandler=new LinkSheetDataHandler(ExternalLinkHandler.this.crnBlocks);}
    fn SheetDataSetHandler(outer:&Rc<ExternalLinkHandler>)->Self{*outer.sheetDataHandler.borrow_mut()=Some(Rc::new(LinkSheetDataHandler::LinkSheetDataHandler(Some(outer.crnBlocks.clone()))));Self{outer:Rc::downgrade(outer)}}
}
impl Subclass for SheetDataSetHandler{
    // java public ElementHandler enterSubElement(int subElement){return subElement==4772?ExternalLinkHandler.this.sheetDataHandler:null;}
    fn enterSubElement(&self,subElement:i32)->Option<Box<dyn ElementHandler>>{if subElement==4772{self.outer.upgrade().expect("ExternalLinkHandler owner").sheetDataHandler.borrow().clone().map(share)}else{None}}
}
