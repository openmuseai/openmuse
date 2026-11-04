// Java: public interface ElementStrHandler
pub trait ElementStrHandler {
    // Java: abstract void startElement(String var1)
    fn start_element(&mut self, var1: &str);

    // Java: abstract void endElement(String var1)
    fn end_element(&mut self, var1: &str);

    // Java: abstract void addAttr(String var1, String var2)
    fn add_attr(&mut self, var1: &str, var2: &str);

    // Java: abstract void characters(String var1)
    fn characters(&mut self, var1: &str);

    // Java: abstract ElementStrHandler enterSubElement(String var1)
    fn enter_sub_element(&mut self, var1: &str) -> Option<Box<dyn ElementStrHandler>>;
}
