// Java: import org.dom4j.DocumentFactory
// use crate::org::dom4j::DocumentFactory;
// Java: import org.xml.sax.SAXException
// use crate::org::xml::sax::SAXException;
// Java: import org.xml.sax.XMLReader
// use crate::org::xml::sax::XMLReader;

// Java: public class SAXModifyReader extends SAXReader
pub struct SAXModifyReader {
    // Java field: XMLWriter xmlWriter
    xmlWriter: i32,
    // Java field: boolean pruneElements
    pub(crate) pruneElements: bool,
}

// Java: public class SAXModifyReader extends SAXReader
impl SAXModifyReader {
    // Java: SAXModifyReader()
    pub fn SAXModifyReader(&self) {
      // Java: super();
      todo!()
    }

    // Java: SAXModifyReader(boolean validating)
    pub fn SAXModifyReader_validating(&self, _validating: bool) {
      // Java: super(validating);
      todo!()
    }

    // Java: SAXModifyReader(DocumentFactory factory)
    pub fn SAXModifyReader_factory(&self, _factory: i32) {
      // Java: super(factory);
      todo!()
    }

    // Java: SAXModifyReader(DocumentFactory factory, boolean validating)
    pub fn SAXModifyReader_factory_validating(&self, _factory: i32, _validating: bool) {
      // Java: super(factory, validating);
      todo!()
    }

    // Java: SAXModifyReader(XMLReader xmlReader)
    pub fn SAXModifyReader_reader(&self, _xmlReader: i32) {
      // Java: super(xmlReader);
      todo!()
    }

    // Java: SAXModifyReader(XMLReader xmlReader, boolean validating)
    pub fn SAXModifyReader_reader_validating(&self, _xmlReader: i32, _validating: bool) {
      // Java: super(xmlReader, validating);
      todo!()
    }

    // Java: SAXModifyReader(String xmlReaderClassName)
    pub fn SAXModifyReader_classname(&self, _xmlReaderClassName: &str) {
      // Java: super(xmlReaderClassName);
      todo!()
    }

    // Java: SAXModifyReader(String xmlReaderClassName, boolean validating)
    pub fn SAXModifyReader_classname_validating(&self, _xmlReaderClassName: &str, _validating: bool) {
      // Java: super(xmlReaderClassName, validating);
      todo!()
    }

    // Java: void setXMLWriter(XMLWriter writer)
    pub fn setXMLWriter(&self, _writer: i32) {
      // Java: this.xmlWriter = writer;
      todo!()
    }

    // Java: boolean isPruneElements()
    pub fn isPruneElements(&self) -> bool {
      // Java: return this.pruneElements;
      todo!()
    }

    // Java: void setPruneElements(boolean pruneElements)
    pub fn setPruneElements(&self, _pruneElements: bool) {
      // Java: this.pruneElements = pruneElements;
      todo!()
    }

    // Java: SAXContentHandler createContentHandler(XMLReader reader)
    pub fn createContentHandler(&self, _reader: i32) -> i32 {
      // Java: SAXModifyContentHandler handler = new SAXModifyContentHandler(this.getDocumentFactory(), this.getDispatchHandler());
      // Java: handler.setXMLWriter(this.xmlWriter);
      // Java: return handler;
      todo!()
    }

    // Java: XMLWriter getXMLWriter()
    pub fn getXMLWriter(&self) -> i32 {
      // Java: return this.xmlWriter;
      todo!()
    }

}
