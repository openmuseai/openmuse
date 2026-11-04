use super::XPathHandler::XPathHandler;

// Java: public interface SAXPathEventSource
pub trait SAXPathEventSource {
    // Java: abstract void setXPathHandler(XPathHandler var1)
    fn setXPathHandler(&mut self, var1: Box<dyn XPathHandler>);

    // Java: abstract XPathHandler getXPathHandler()
    fn getXPathHandler(&self) -> &dyn XPathHandler;
}
