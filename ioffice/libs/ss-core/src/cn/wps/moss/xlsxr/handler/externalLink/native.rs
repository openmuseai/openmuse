// Native Java inheritance/reference adapter. Default callbacks call the actual
// DefaultElementHandler base. Returned inner references retain their outer graph.
use crate::java_runtime::Rc;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,DefaultElementHandler::DefaultElementHandler,XmlRoAttr::XmlRoAttr};
pub(super) trait Subclass:std::any::Any+Send+Sync{
    fn startElement(&self,e:i32,a:&dyn XmlRoAttr){DefaultElementHandler.startElement(e,a);}
    fn endElement(&self,e:i32){DefaultElementHandler.endElement(e);}
    fn characters(&self,s:&str){DefaultElementHandler.characters(s);}
    fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{DefaultElementHandler.enterSubElement(e)}
}
struct Shared<T:Subclass>(Rc<T>);
impl<T:Subclass> ElementHandler for Shared<T>{
    fn startElement(&self,e:i32,a:&dyn XmlRoAttr){self.0.startElement(e,a);}
    fn endElement(&self,e:i32){self.0.endElement(e);}
    fn characters(&self,s:&str){self.0.characters(s);}
    fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{self.0.enterSubElement(e)}
}
struct Held{handler:Box<dyn ElementHandler>,owner:Rc<dyn std::any::Any+Send+Sync>}
impl ElementHandler for Held{
    fn startElement(&self,e:i32,a:&dyn XmlRoAttr){self.handler.startElement(e,a);}
    fn endElement(&self,e:i32){self.handler.endElement(e);}
    fn characters(&self,s:&str){self.handler.characters(s);}
    fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{self.handler.enterSubElement(e).map(|h|Box::new(Held{handler:h,owner:self.owner.clone()})as Box<dyn ElementHandler>)}
}
pub(super) fn share<T:Subclass>(handler:Rc<T>)->Box<dyn ElementHandler>{Box::new(Shared(handler))}
pub(super) fn hold<T:Subclass,O:std::any::Any+Send+Sync>(handler:Rc<T>,owner:Rc<O>)->Box<dyn ElementHandler>{Box::new(Held{handler:share(handler),owner})}
