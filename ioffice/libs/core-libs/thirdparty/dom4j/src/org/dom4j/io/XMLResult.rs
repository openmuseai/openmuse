// Java: import java.io.OutputStream
// Java: import java.io.UnsupportedEncodingException
// Java: import java.io.Writer
// Java: import javax.xml.transform.sax.SAXResult
// use crate::javax::xml::transform::sax::SAXResult;
// Java: import org.xml.sax.ContentHandler
// use io_wps_io::io::xwpf::element_handler::ContentHandler;
// Java: import org.xml.sax.ext.LexicalHandler
// use crate::org::xml::sax::ext::LexicalHandler;

// Java: public class XMLResult extends SAXResult
pub struct XMLResult {
    // Java field: XMLWriter xmlWriter
    pub(crate) xmlWriter: i32,
}

// Java: public class XMLResult extends SAXResult
impl XMLResult {
    // Java: XMLResult()
    pub fn XMLResult(&self) {
      // Java: this(new XMLWriter());
      todo!()
    }

    // Java: XMLResult(Writer writer)
    pub fn XMLResult_writer(&self, _writer: i32) {
      // Java: this(new XMLWriter(writer));
      todo!()
    }

    // Java: XMLResult(Writer writer, OutputFormat format)
    pub fn XMLResult_writer_format(&self, _writer: i32, _format: i32) {
      // Java: this(new XMLWriter(writer, format));
      todo!()
    }

    // Java: XMLResult(OutputStream out)
    pub fn XMLResult_out(&self, _out: i32) {
      // Java: this(new XMLWriter(out));
      todo!()
    }

    // Java: XMLResult(OutputStream out, OutputFormat format)
    pub fn XMLResult_out_format(&self, _out: i32, _format: i32) {
      // Java: this(new XMLWriter(out, format));
      todo!()
    }

    // Java: XMLResult(XMLWriter xmlWriter)
    pub fn XMLResult_xmlwriter(&self, _xmlWriter: i32) {
      // Java: super(xmlWriter);
      // Java: this.xmlWriter = xmlWriter;
      // Java: this.setLexicalHandler(xmlWriter);
      todo!()
    }

    // Java: XMLWriter getXMLWriter()
    pub fn getXMLWriter(&self) -> i32 {
      // Java: return this.xmlWriter;
      todo!()
    }

    // Java: void setXMLWriter(XMLWriter writer)
    pub fn setXMLWriter(&self, _writer: i32) {
      // Java: this.xmlWriter = writer;
      // Java: this.setHandler(this.xmlWriter);
      // Java: this.setLexicalHandler(this.xmlWriter);
      todo!()
    }

    // Java: ContentHandler getHandler()
    pub fn getHandler(&self) -> i32 {
      // Java: return this.xmlWriter;
      todo!()
    }

    // Java: LexicalHandler getLexicalHandler()
    pub fn getLexicalHandler(&self) -> i32 {
      // Java: return this.xmlWriter;
      todo!()
    }

}
