// Java: import javax.xml.transform.sax.SAXSource
// use crate::javax::xml::transform::sax::SAXSource;
// Java: import org.dom4j.Document
// use crate::org::dom4j::Document;
// Java: import org.dom4j.Node
// use crate::org::dom4j::Node;
// Java: import org.xml.sax.InputSource
// use crate::org::xml::sax::InputSource;
// Java: import org.xml.sax.XMLFilter
// use crate::org::xml::sax::XMLFilter;
// Java: import org.xml.sax.XMLReader
// use crate::org::xml::sax::XMLReader;

// Java field: String DOM4J_FEATURE
pub(crate) const DOM4J_FEATURE: &str = "";

// Java: public class DocumentSource extends SAXSource
pub struct DocumentSource {
    // Java field: XMLReader xmlReader
    pub(crate) xmlReader: i32,
}

// Java: public class DocumentSource extends SAXSource
impl DocumentSource {
    // Java: DocumentSource(Node node)
    pub fn DocumentSource_node(&self, _node: i32) {
      // Java: super();
      // Java: this.setDocument(node.getDocument());
      todo!()
    }

    // Java: DocumentSource(Document document)
    pub fn DocumentSource_document(&self, _document: i32) {
      // Java: super();
      // Java: this.setDocument(document);
      todo!()
    }

    // Java: Document getDocument()
    pub fn getDocument(&self) -> i32 {
      // Java: DocumentInputSource source = (DocumentInputSource)this.getInputSource();
      // Java: return source.getDocument();
      todo!()
    }

    // Java: void setDocument(Document document)
    pub fn setDocument(&self, _document: i32) {
      // Java: super.setInputSource(new DocumentInputSource(document));
      todo!()
    }

    // Java: XMLReader getXMLReader()
    pub fn getXMLReader(&self) -> i32 {
      // Java: return this.xmlReader;
      todo!()
    }

    // Java: void setInputSource(InputSource inputSource)
    pub fn setInputSource(&self, _inputSource: i32) {
      // Java: if (inputSource instanceof DocumentInputSource)
      {
         // Java: super.setInputSource((DocumentInputSource)inputSource);
         todo!()
      }
      // Java: else
      {
         // Java: throw new UnsupportedOperationException();
         todo!()
      }
    }

    // Java: void setXMLReader(XMLReader reader)
    pub fn setXMLReader(&self, _reader: i32) {
      // Java: if (reader instanceof SAXWriter)
      {
         // Java: this.xmlReader = (SAXWriter)reader;
         todo!()
      }
      // Java: else
      {
         // Java: if (!(reader instanceof XMLFilter))
         {
            // Java: throw new UnsupportedOperationException();
            todo!()
         }
         // Java: XMLFilter filter = (XMLFilter)reader;
         // Java: while(true)
         {
            // Java: XMLReader parent = filter.getParent();
            // Java: if (!(parent instanceof XMLFilter))
            {
               // Java: filter.setParent(this.xmlReader);
               // Java: this.xmlReader = filter;
               // Java: break;
               todo!()
            }
            // Java: filter = (XMLFilter)parent;
            todo!()
         }
      }
    }

}
