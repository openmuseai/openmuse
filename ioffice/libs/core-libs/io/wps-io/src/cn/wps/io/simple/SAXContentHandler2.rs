#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException
// Java: import org.xml.sax.helpers.DefaultHandler

use super::ElementHandlerSimple::ElementHandlerRef;

// Java: public class SAXContentHandler2 extends DefaultHandler
pub struct SAXContentHandler2 {
    // Java field: ElementHandlerSimple elementHandler
    elementHandler: Option<ElementHandlerRef>,
}

// Java: public class SAXContentHandler2 extends DefaultHandler
impl SAXContentHandler2 {
    // Java: SAXContentHandler2(ElementHandlerSimple elementHandler2)
    pub fn new(element_handler2: Option<ElementHandlerRef>) -> Self {
        // Java: super();
        // Java: this.elementHandler = elementHandler2;
        SAXContentHandler2 {
            elementHandler: element_handler2,
        }
    }

    // Java: void startElement(String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn startElement(&mut self, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn std::any::Any) {
        // Java: if (this.elementHandler != null)
        if self.elementHandler.is_some() {
            // Java: this.elementHandler.onStart(namespaceURI, localName, qualifiedName, attributes);
            let _ = self.elementHandler.as_ref().unwrap().borrow_mut().onStart(namespace_uri, local_name, qualified_name, attributes);
        }
    }

    // Java: void endElement(String namespaceURI, String localName, String qName)
    pub fn endElement(&mut self, namespace_uri: &str, local_name: &str, q_name: &str) {
        // Java: if (this.elementHandler != null)
        if self.elementHandler.is_some() {
            // Java: this.elementHandler.onEnd(namespaceURI, localName, qName);
            let _ = self.elementHandler.as_ref().unwrap().borrow_mut().onEnd(namespace_uri, local_name, q_name);
        }
    }

    // Java: void characters(char[] ch, int start, int end)
    pub fn characters(&mut self, ch: &[char], start: i32, end: i32) {
        // Java: this.elementHandler.onCharacters(ch, start, end);
        let _ = self.elementHandler.as_ref().unwrap().borrow_mut().onCharacters(ch, start, end);
    }
}
