use crate::{java_runtime::Rc,cn::wps::moss::{app::KmoBook::KmoBook,xlsxr::handler::book::DefinedNamesHandler::DefinedNamesHandler}};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
// java public final class WorkbookDelayHandler extends DefaultElementHandler {
pub struct WorkbookDelayHandler{
    // java private KmoBook mBook;
    mBook:Rc<KmoBook>,
}
impl WorkbookDelayHandler{
    // java public WorkbookDelayHandler(KmoBook book){this.mBook=book;}
    pub fn WorkbookDelayHandler(book:Rc<KmoBook>)->Self{Self{mBook:book}}
    // java public ElementHandler enterSubElement(int subElement){return subElement==5199?new DefinedNamesHandler(this.mBook):null;}
    pub fn enterSubElement(&self,subElement:i32)->Option<Box<dyn ElementHandler>>{if subElement==5199{Some(Box::new(DefinedNamesHandler::DefinedNamesHandler(self.mBook.clone())))}else{None}}
}
impl ElementHandler for WorkbookDelayHandler{
    fn startElement(&self,_e:i32,_a:&dyn XmlRoAttr){}
    fn endElement(&self,_e:i32){}
    fn characters(&self,_s:&str){}
    fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{self.enterSubElement(e)}
}
