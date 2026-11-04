// Java: import java.io.IOException
// Java: import org.dom4j.Document
// use crate::org::dom4j::Document;
// Java: import org.xml.sax.ContentHandler
// use io_wps_io::io::xwpf::element_handler::ContentHandler;
// Java: import org.xml.sax.ErrorHandler
// use crate::org::xml::sax::ErrorHandler;
// Java: import org.xml.sax.SAXException
// use crate::org::xml::sax::SAXException;
// Java: import org.xml.sax.XMLReader
// use crate::org::xml::sax::XMLReader;
// Java: import org.xml.sax.helpers.DefaultHandler
// use crate::org::xml::sax::helpers::DefaultHandler;

// Java: public class SAXValidator
pub struct SAXValidator {
    // Java field: XMLReader xmlReader
    pub(crate) xmlReader: i32,
    // Java field: ErrorHandler errorHandler
    pub(crate) errorHandler: i32,
}

// Java: public class SAXValidator
impl SAXValidator {
    // Java: SAXValidator()
    pub fn new() -> Self {
      // Java: super();
      Self { xmlReader: 0, errorHandler: 0 }
    }

    // Java: SAXValidator(XMLReader xmlReader)
    pub fn new_with_reader(_xmlReader: i32) -> Self {
      // Java: super();
      // Java: this.xmlReader = xmlReader;
      Self { xmlReader: _xmlReader, errorHandler: 0 }
    }

    // Java: void validate(Document document)
    pub fn validate(&self, _document: i32) {
      // Java: if (document != null)
      {
         // Java: XMLReader reader = this.getXMLReader();
         // Java: if (this.errorHandler != null)
         {
            // Java: reader.setErrorHandler(this.errorHandler);
         }
         // Java: try
         {
            // Java: reader.parse(new DocumentInputSource(document));
         }
         // Java: catch (IOException e)
         {
            // Java: throw new RuntimeException("Caught and exception that should never happen: " + e);
         }
      }
    }

    // Java: XMLReader getXMLReader()
    pub fn getXMLReader(&self) -> i32 {
      // Java: if (this.xmlReader == null)
      {
         // Java: this.xmlReader = this.createXMLReader();
         // Java: this.configureReader();
      }
      // Java: return this.xmlReader;
      0
    }

    // Java: void setXMLReader(XMLReader reader)
    pub fn setXMLReader(&self, _reader: i32) {
      // Java: this.xmlReader = reader;
      // Java: this.configureReader();
    }

    // Java: ErrorHandler getErrorHandler()
    pub fn getErrorHandler(&self) -> i32 {
      // Java: return this.errorHandler;
      0
    }

    // Java: void setErrorHandler(ErrorHandler errorHandler)
    pub fn setErrorHandler(&self, _errorHandler: i32) {
      // Java: this.errorHandler = errorHandler;
    }

    // Java: XMLReader createXMLReader()
    pub fn createXMLReader(&self) -> i32 {
      // Java: return SAXHelper.createXMLReader(true);
      0
    }

    // Java: void configureReader()
    pub fn configureReader(&self) {
      // Java: ContentHandler handler = this.xmlReader.getContentHandler();
      // Java: if (handler == null)
      {
         // Java: this.xmlReader.setContentHandler(new DefaultHandler());
      }
      // Java: this.xmlReader.setFeature("http://xml.org/sax/features/validation", true);
      // Java: this.xmlReader.setFeature("http://xml.org/sax/features/namespaces", true);
      // Java: this.xmlReader.setFeature("http://xml.org/sax/features/namespace-prefixes", false);
    }

}
