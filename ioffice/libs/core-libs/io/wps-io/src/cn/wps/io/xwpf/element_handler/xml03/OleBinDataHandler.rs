#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.element_handler.IXml03Importer
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import cn.wps.util.IOUtils
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

// Java: public class OleBinDataHandler extends XmlSimpleNodeElementHandler
pub struct OleBinDataHandler {
    // Java field: String mName
    mName: Option<String>,
}

// Java: public class OleBinDataHandler extends XmlSimpleNodeElementHandler
impl OleBinDataHandler {
    // Java: OleBinDataHandler(IXml03Importer importer)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super((IDocumentImporter)importer);
        OleBinDataHandler {
            mName: None,
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&mut self, _element_id: i32, _namespace_uri: &str, _local_name: &str, _qualified_name: &str, _attributes: &dyn std::any::Any) {
        // Java: this.mName = attributes.getValue("name");
        self.mName = None;
    }

    // Java: void onCharacters(char[] buffer, int start, int len)
    pub fn onCharacters(&mut self, _buffer: &[char], _start: usize, _len: usize) {
        // Java: IXml03Importer importer = (IXml03Importer)this.mDocImporter;
        // Java: byte[] buff = IOUtils.getBytes(buffer, start, len);
        // Java: importer.onImportOleBinData(this.mName, buff, (String)null);
        todo!()
    }

    // Java: void onCharacters(String persistPath)
    pub fn onCharacters_str(&mut self, _persist_path: &str) {
        // Java: IXml03Importer importer = (IXml03Importer)this.mDocImporter;
        // Java: importer.onImportOleBinData(this.mName, (byte[])null, persistPath);
        todo!()
    }

    // Java: void onEnd(String namespaceURI, String localName, String qName)
    pub fn onEnd(&mut self, _namespace_uri: &str, _local_name: &str, _q_name: &str) {
        // Java: this.mName = null;
        self.mName = None;
    }
}
