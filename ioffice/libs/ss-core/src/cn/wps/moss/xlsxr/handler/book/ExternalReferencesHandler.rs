use crate::{java_runtime::{Rc,RefCell,Weak},cn::wps::moss::xlsxr::{xl::XlsxrWorkbook::{XlsxrWorkbook,RidList},util::ElementHandlerRef::{hold,share}}};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
// java public final class ExternalReferencesHandler extends DefaultElementHandler {
pub struct ExternalReferencesHandler{
    // java private XlsxrWorkbook reader;
    reader:Rc<XlsxrWorkbook>,
    // java private ExternalReferenceHandler extRefHandler;
    extRefHandler:Rc<ExternalReferenceHandler>,
    // java private List<String> rids;
    rids:RidList,
    owner:Weak<Self>,
}
impl ExternalReferencesHandler{
    // java public ExternalReferencesHandler(XlsxrWorkbook reader){this.reader=reader;this.extRefHandler=new ExternalReferenceHandler();this.rids=new ArrayList();}
    pub fn ExternalReferencesHandler(reader:Rc<XlsxrWorkbook>)->Rc<Self>{Rc::new_cyclic(|owner|Self{reader,extRefHandler:Rc::new(ExternalReferenceHandler{outer:owner.clone()}),rids:Rc::new(RefCell::new(vec![])),owner:owner.clone()})}
    // java public ElementHandler enterSubElement(int subElement){return this.extRefHandler;}
    pub fn enterSubElement(&self,_subElement:i32)->Option<Box<dyn ElementHandler>>{Some(hold(self.extRefHandler.clone(),self.owner.upgrade().unwrap()))}
    // java public void endElement(int element){this.reader.setExternalReferences(this.rids);}
    pub fn endElement(&self,_element:i32){self.reader.setExternalReferences(Some(self.rids.clone()));}
    pub fn asElementHandler(self:&Rc<Self>)->Box<dyn ElementHandler>{share(self.clone())}
}
impl ElementHandler for ExternalReferencesHandler{
    fn startElement(&self,_e:i32,_a:&dyn XmlRoAttr){}
    fn endElement(&self,e:i32){self.endElement(e);}
    fn characters(&self,_s:&str){}
    fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{self.enterSubElement(e)}
}
// java private final class ExternalReferenceHandler extends DefaultElementHandler {
struct ExternalReferenceHandler{outer:Weak<ExternalReferencesHandler>}
impl ElementHandler for ExternalReferenceHandler{
    // java public void startElement(int element,XmlRoAttr attrsNotCopy){XmlRoAttr tmp=attrsNotCopy.getByName(4259841);if(tmp!=null){String rid=tmp.toString();ExternalReferencesHandler.this.rids.add(rid);}}
    fn startElement(&self,_element:i32,attrsNotCopy:&dyn XmlRoAttr){if let Some(tmp)=attrsNotCopy.get_by_name(4259841){let rid=tmp.to_string();self.outer.upgrade().unwrap().rids.borrow_mut().push(Some(rid));}}
    fn endElement(&self,_e:i32){}
    fn characters(&self,_s:&str){}
    fn enterSubElement(&self,_e:i32)->Option<Box<dyn ElementHandler>>{None}
}
