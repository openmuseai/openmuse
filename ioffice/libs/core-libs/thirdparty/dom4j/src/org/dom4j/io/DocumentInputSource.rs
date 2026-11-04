// Java: import java.io.IOException
// Java: import java.io.Reader
// Java: import java.io.StringReader
// Java: import java.io.StringWriter
// Java: import org.dom4j.Document
use crate::org::dom4j::Document;
// Java: import org.xml.sax.InputSource
// use crate::org::xml::sax::InputSource;

// Java: public class DocumentInputSource extends InputSource
pub struct DocumentInputSource {
    // Java field: Document document
    pub(crate) document: i32,
}

// Java: public class DocumentInputSource extends InputSource
impl DocumentInputSource {
    // Java: DocumentInputSource()
    pub fn DocumentInputSource(&self) {
      // Java: super();
      todo!();
    }

    // Java: DocumentInputSource(Document document)
    pub fn DocumentInputSource_2(&self, _document: i32) {
      // Java: super();
      // Java: this.document = document;
      // Java: this.setSystemId(document.getName());
      todo!();
    }

    // Java: Document getDocument()
    pub fn getDocument(&self) -> i32 {
      // Java: return this.document;
      todo!();
    }

    // Java: void setDocument(Document document)
    pub fn setDocument(&self, _document: i32) {
      // Java: this.document = document;
      // Java: this.setSystemId(document.getName());
      todo!();
    }

    // Java: void setCharacterStream(Reader characterStream)
    pub fn setCharacterStream(&self, _characterStream: i32) {
      // Java: throw new UnsupportedOperationException();
      todo!();
    }

    // Java: Reader getCharacterStream()
    pub fn getCharacterStream(&self) -> i32 {
      // Java: try
      // Java: StringWriter out = new StringWriter();
      // Java: XMLWriter writer = new XMLWriter(out);
      // Java: writer.write(this.document);
      // Java: writer.flush();
      // Java: return new StringReader(out.toString());
      // Java: catch (final IOException e)
      // Java: return new Reader()
      // Java: public int read(char[] ch, int offset, int length) throws IOException
      // Java: throw e;
      // Java: public void close() throws IOException
      todo!();
    }

}