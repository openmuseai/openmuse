use crate::kfc::xpxml::reader::ElementStrHandler::ElementStrHandler;

// Java: public class DefaultElementStrHandler implements ElementStrHandler
#[derive(Default)]
pub struct DefaultElementStrHandler {
}

// Java: public class DefaultElementStrHandler implements ElementStrHandler
impl DefaultElementStrHandler {
    // Java: DefaultElementStrHandler()
    pub fn new() -> Self {
      // Java: super();
      DefaultElementStrHandler {}
    }

    // Java: void startElement(String label)
    pub fn startElement(&self, _label: &str) {
    }

    // Java: void endElement(String label)
    pub fn endElement(&self, _label: &str) {
    }

    // Java: void addAttr(String name, String value)
    pub fn addAttr(&self, _name: &str, _value: &str) {
    }

    // Java: void characters(String chars)
    pub fn characters(&self, _chars: &str) {
    }

    // Java: ElementStrHandler enterSubElement(String subLabel)
    pub fn enterSubElement(&self, _subLabel: &str) -> Option<Box<dyn ElementStrHandler>> {
      // Java: return null;
      None
    }

}

impl ElementStrHandler for DefaultElementStrHandler {
    fn start_element(&mut self, _var1: &str) {}
    fn end_element(&mut self, _var1: &str) {}
    fn add_attr(&mut self, _var1: &str, _var2: &str) {}
    fn characters(&mut self, _var1: &str) {}
    fn enter_sub_element(&mut self, _var1: &str) -> Option<Box<dyn ElementStrHandler>> { None }
}

// Native Java inheritance adapter. A subclass only overrides the original
// methods it declares; all other calls retain DefaultElementStrHandler bodies.
pub trait Subclass {
    fn start_element(&mut self,label:&str){DefaultElementStrHandler::new().startElement(label);}
    fn end_element(&mut self,label:&str){DefaultElementStrHandler::new().endElement(label);}
    fn add_attr(&mut self,name:&str,value:&str){DefaultElementStrHandler::new().addAttr(name,value);}
    fn characters(&mut self,chars:&str){DefaultElementStrHandler::new().characters(chars);}
    fn enter_sub_element(&mut self,label:&str)->Option<Box<dyn ElementStrHandler>>{DefaultElementStrHandler::new().enterSubElement(label)}
}
impl<T:Subclass> ElementStrHandler for T{
    fn start_element(&mut self,label:&str){Subclass::start_element(self,label);}
    fn end_element(&mut self,label:&str){Subclass::end_element(self,label);}
    fn add_attr(&mut self,name:&str,value:&str){Subclass::add_attr(self,name,value);}
    fn characters(&mut self,chars:&str){Subclass::characters(self,chars);}
    fn enter_sub_element(&mut self,label:&str)->Option<Box<dyn ElementStrHandler>>{Subclass::enter_sub_element(self,label)}
}
