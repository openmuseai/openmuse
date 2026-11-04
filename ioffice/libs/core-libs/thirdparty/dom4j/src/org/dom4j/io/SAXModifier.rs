// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import java.io.InputStream
// Java: import java.io.Reader
// Java: import java.net.URL
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.dom4j.Document
// use crate::org::dom4j::Document;
// Java: import org.dom4j.DocumentException
// use crate::org::dom4j::DocumentException;
// Java: import org.dom4j.DocumentFactory
// use crate::org::dom4j::DocumentFactory;
// Java: import org.xml.sax.InputSource
// use crate::org::xml::sax::InputSource;
// Java: import org.xml.sax.SAXException
// use crate::org::xml::sax::SAXException;
// Java: import org.xml.sax.XMLReader
// use crate::org::xml::sax::XMLReader;

// Java: public class SAXModifier
pub struct SAXModifier {
    // Java field: XMLWriter xmlWriter
    pub(crate) xmlWriter: i32,
    // Java field: XMLReader xmlReader
    xmlReader: i32,
    // Java field: boolean pruneElements
    pub(crate) pruneElements: bool,
    // Java field: SAXModifyReader modifyReader
    modifyReader: i32,
    // Java field: HashMap<String, ElementModifier> modifiers
    modifiers: i32,
}

// Java: public class SAXModifier
impl SAXModifier {
    // Java: SAXModifier()
    pub fn SAXModifier(&self) {
      // Java: super();
      todo!()
    }

    // Java: SAXModifier(boolean pruneElements)
    pub fn SAXModifier_prune(&self, _pruneElements: bool) {
      // Java: super();
      // Java: this.pruneElements = pruneElements;
      todo!()
    }

    // Java: SAXModifier(XMLReader xmlReader)
    pub fn SAXModifier_reader(&self, _xmlReader: i32) {
      // Java: super();
      // Java: this.xmlReader = xmlReader;
      todo!()
    }

    // Java: SAXModifier(XMLReader xmlReader, boolean pruneElements)
    pub fn SAXModifier_reader_prune(&self, _xmlReader: i32, _pruneElements: bool) {
      // Java: super();
      // Java: this.xmlReader = xmlReader;
      todo!()
    }

    // Java: Document modify(File source)
    pub fn modify_file(&self, _source: i32) -> i32 {
      // Java: try
      {
         // Java: return this.installModifyReader().read(source);
         todo!()
      }
      // Java: catch (SAXModifyException ex)
      {
         // Java: Throwable cause = ex.getCause();
         // Java: throw new DocumentException(cause.getMessage(), cause);
         todo!()
      }
    }

    // Java: Document modify(InputSource source)
    pub fn modify_inputsource(&self, _source: i32) -> i32 {
      // Java: try
      {
         // Java: return this.installModifyReader().read(source);
         todo!()
      }
      // Java: catch (SAXModifyException ex)
      {
         // Java: Throwable cause = ex.getCause();
         // Java: throw new DocumentException(cause.getMessage(), cause);
         todo!()
      }
    }

    // Java: Document modify(InputStream source)
    pub fn modify_inputstream(&self, _source: i32) -> i32 {
      // Java: try
      {
         // Java: return this.installModifyReader().read(source);
         todo!()
      }
      // Java: catch (SAXModifyException ex)
      {
         // Java: Throwable cause = ex.getCause();
         // Java: throw new DocumentException(cause.getMessage(), cause);
         todo!()
      }
    }

    // Java: Document modify(InputStream source, String systemId)
    pub fn modify_inputstream_systemid(&self, _source: i32, _systemId: &str) -> i32 {
      // Java: try
      {
         // Java: return this.installModifyReader().read(source);
         todo!()
      }
      // Java: catch (SAXModifyException ex)
      {
         // Java: Throwable cause = ex.getCause();
         // Java: throw new DocumentException(cause.getMessage(), cause);
         todo!()
      }
    }

    // Java: Document modify(Reader source)
    pub fn modify_reader(&self, _source: i32) -> i32 {
      // Java: try
      {
         // Java: return this.installModifyReader().read(source);
         todo!()
      }
      // Java: catch (SAXModifyException ex)
      {
         // Java: Throwable cause = ex.getCause();
         // Java: throw new DocumentException(cause.getMessage(), cause);
         todo!()
      }
    }

    // Java: Document modify(Reader source, String systemId)
    pub fn modify_reader_systemid(&self, _source: i32, _systemId: &str) -> i32 {
      // Java: try
      {
         // Java: return this.installModifyReader().read(source);
         todo!()
      }
      // Java: catch (SAXModifyException ex)
      {
         // Java: Throwable cause = ex.getCause();
         // Java: throw new DocumentException(cause.getMessage(), cause);
         todo!()
      }
    }

    // Java: Document modify(URL source)
    pub fn modify_url(&self, _source: i32) -> i32 {
      // Java: try
      {
         // Java: return this.installModifyReader().read(source);
         todo!()
      }
      // Java: catch (SAXModifyException ex)
      {
         // Java: Throwable cause = ex.getCause();
         // Java: throw new DocumentException(cause.getMessage(), cause);
         todo!()
      }
    }

    // Java: Document modify(String source)
    pub fn modify_string(&self, _source: &str) -> i32 {
      // Java: try
      {
         // Java: return this.installModifyReader().read(source);
         todo!()
      }
      // Java: catch (SAXModifyException ex)
      {
         // Java: Throwable cause = ex.getCause();
         // Java: throw new DocumentException(cause.getMessage(), cause);
         todo!()
      }
    }

    // Java: void addModifier(String path, ElementModifier modifier)
    pub fn addModifier(&self, _path: &str, _modifier: i32) {
      // Java: this.modifiers.put(path, modifier);
      todo!()
    }

    // Java: void resetModifiers()
    pub fn resetModifiers(&self) {
      // Java: this.modifiers.clear();
      // Java: this.getSAXModifyReader().resetHandlers();
      todo!()
    }

    // Java: void removeModifier(String path)
    pub fn removeModifier(&self, _path: &str) {
      // Java: this.modifiers.remove(path);
      // Java: this.getSAXModifyReader().removeHandler(path);
      todo!()
    }

    // Java: DocumentFactory getDocumentFactory()
    pub fn getDocumentFactory(&self) -> i32 {
      // Java: return this.getSAXModifyReader().getDocumentFactory();
      todo!()
    }

    // Java: void setDocumentFactory(DocumentFactory factory)
    pub fn setDocumentFactory(&self, _factory: i32) {
      // Java: this.getSAXModifyReader().setDocumentFactory(factory);
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
      todo!()
    }

    // Java: boolean isPruneElements()
    pub fn isPruneElements(&self) -> bool {
      // Java: return this.pruneElements;
      todo!()
    }

    // Java: SAXReader installModifyReader()
    pub fn installModifyReader(&self) -> i32 {
      // Java: try
      {
         // Java: SAXModifyReader reader = this.getSAXModifyReader();
         // Java: if (this.isPruneElements())
         {
            // Java: this.modifyReader.setDispatchHandler(new PruningDispatchHandler());
            todo!()
         }
         // Java: reader.resetHandlers();
         // Java: for(Map.Entry<String, ElementModifier> entry : this.modifiers.entrySet())
         {
            // Java: SAXModifyElementHandler handler = new SAXModifyElementHandler((ElementModifier)entry.getValue());
            // Java: reader.addHandler((String)entry.getKey(), handler);
            todo!()
         }
         // Java: reader.setXMLWriter(this.getXMLWriter());
         // Java: reader.setXMLReader(this.getXMLReader());
         // Java: return reader;
         todo!()
      }
      // Java: catch (SAXException ex)
      {
         // Java: throw new DocumentException(ex.getMessage(), ex);
         todo!()
      }
    }

    // Java: XMLReader getXMLReader()
    pub fn getXMLReader(&self) -> i32 {
      // Java: if (this.xmlReader == null)
      {
         // Java: this.xmlReader = SAXHelper.createXMLReader(false);
         todo!()
      }
      // Java: return this.xmlReader;
      todo!()
    }

    // Java: SAXModifyReader getSAXModifyReader()
    pub fn getSAXModifyReader(&self) -> i32 {
      // Java: if (this.modifyReader == null)
      {
         // Java: this.modifyReader = new SAXModifyReader();
         todo!()
      }
      // Java: return this.modifyReader;
      todo!()
    }

}
