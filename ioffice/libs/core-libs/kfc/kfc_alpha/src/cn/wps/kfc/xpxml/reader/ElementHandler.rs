use super::XmlRoAttr::XmlRoAttr;

// Java: public interface ElementHandler
pub trait ElementHandler {
    // Java: abstract void startElement(int var1, XmlRoAttr var2)
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr);

    // Java: abstract void endElement(int var1)
    fn endElement(&self, var1: i32);

    // Java: abstract void characters(String var1)
    fn characters(&self, var1: &str);

    // Java: abstract ElementHandler enterSubElement(int var1)
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>>;
}
