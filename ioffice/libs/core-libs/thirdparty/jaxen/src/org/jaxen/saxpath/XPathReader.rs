use super::SAXPathException::SAXPathException;
use super::SAXPathEventSource::SAXPathEventSource;

// Java: public interface XPathReader extends SAXPathEventSource
pub trait XPathReader: SAXPathEventSource {
    // Java: abstract void parse(String var1)
    fn parse(&mut self, var1: &str) -> Result<(), SAXPathException>;
}
