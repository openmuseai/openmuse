// Java: import javax.xml.transform.sax.SAXResult
// use crate::javax::xml::transform::sax::SAXResult;
// Java: import org.w3c.dom.Document
// use io_wps_io::cn::wps::io::dom::Document;
// Java: import org.xml.sax.ContentHandler
// use io_wps_io::io::xwpf::element_handler::ContentHandler;
// Java: import org.xml.sax.ext.LexicalHandler
// use crate::org::xml::sax::ext::LexicalHandler;

// Java: public class DOMDocumentResult extends SAXResult
pub struct DOMDocumentResult {
    // Java field: DOMSAXContentHandler contentHandler
    pub(crate) contentHandler: i32,
}

// Java: public class DOMDocumentResult extends SAXResult
impl DOMDocumentResult {
    // Java: DOMDocumentResult()
    pub fn DOMDocumentResult(&self) {
      // Java: this(new DOMSAXContentHandler());
      todo!()
    }

    // Java: DOMDocumentResult(DOMSAXContentHandler contentHandler)
    pub fn DOMDocumentResult_with_handler(&self, _contentHandler: i32) {
      // Java: super();
      // Java: this.contentHandler = contentHandler;
      // Java: super.setHandler(this.contentHandler);
      // Java: super.setLexicalHandler(this.contentHandler);
      todo!()
    }

    // Java: Document getDocument()
    pub fn getDocument(&self) -> i32 {
      // Java: return this.contentHandler.getDocument();
      todo!()
    }

    // Java: void setHandler(ContentHandler handler)
    pub fn setHandler(&self, _handler: i32) {
      // Java: if (handler instanceof DOMSAXContentHandler)
      {
         // Java: this.contentHandler = (DOMSAXContentHandler)handler;
         // Java: super.setHandler(this.contentHandler);
         todo!()
      }
    }

    // Java: void setLexicalHandler(LexicalHandler handler)
    pub fn setLexicalHandler(&self, _handler: i32) {
      // Java: if (handler instanceof DOMSAXContentHandler)
      {
         // Java: this.contentHandler = (DOMSAXContentHandler)handler;
         // Java: super.setLexicalHandler(this.contentHandler);
         todo!()
      }
    }

}
