// Native Java reference transport. Inner classes retain the same outer graph;
// no parser state or ET decisions live in these forwarding objects.
use crate::java_runtime::Rc;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
struct Shared<T:ElementHandler>(Rc<T>);
impl<T:ElementHandler> ElementHandler for Shared<T>{
    fn startElement(&self,e:i32,a:&dyn XmlRoAttr){self.0.startElement(e,a);}
    fn endElement(&self,e:i32){self.0.endElement(e);}
    fn characters(&self,s:&str){self.0.characters(s);}
    fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{self.0.enterSubElement(e)}
}
struct Held{handler:Box<dyn ElementHandler>,owner:Rc<dyn std::any::Any>}
impl ElementHandler for Held{
    fn startElement(&self,e:i32,a:&dyn XmlRoAttr){self.handler.startElement(e,a);}
    fn endElement(&self,e:i32){self.handler.endElement(e);}
    fn characters(&self,s:&str){self.handler.characters(s);}
    fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{self.handler.enterSubElement(e).map(|handler|Box::new(Held{handler,owner:self.owner.clone()})as Box<dyn ElementHandler>)}
}
pub(crate) fn share<T:ElementHandler+'static>(handler:Rc<T>)->Box<dyn ElementHandler>{Box::new(Shared(handler))}
pub(crate) fn hold<T:ElementHandler+'static,O:std::any::Any>(handler:Rc<T>,owner:Rc<O>)->Box<dyn ElementHandler>{Box::new(Held{handler:share(handler),owner})}
