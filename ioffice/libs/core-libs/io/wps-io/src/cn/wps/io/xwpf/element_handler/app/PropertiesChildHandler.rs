#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::ElementHandlerSimple;
// Java: import cn.wps.io.xwpf.IXml07Importer
use crate::io::xwpf::IXml07Importer::IXml07Importer;
// Java: import cn.wps.io.xwpf.element_handler.Xml07HandlerBase
use crate::io::xwpf::element_handler::Xml07HandlerBase::Xml07HandlerBase;
// Java: import org.xml.sax.SAXException

// Java: public class PropertiesChildHandler extends Xml07HandlerBase
pub struct PropertiesChildHandler {
    base: Xml07HandlerBase,
    // Java field: String mChars
    pub(crate) mChars: Option<String>,
}

// Java: public class PropertiesChildHandler extends Xml07HandlerBase
impl PropertiesChildHandler {
    // Java: PropertiesChildHandler(IXml07Importer importer)
    pub fn new(importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(importer);
        PropertiesChildHandler {
            base: Xml07HandlerBase::new(importer),
            mChars: None,
        }
    }

    // Java: void onCharacters(char[] buffer, int start, int len)
    pub fn onCharacters(&mut self, buffer: &[char], start: i32, len: i32) {
        // Java: String str = new String(buffer, start, len);
        let str_val: String = buffer[start as usize..(start + len) as usize].iter().collect();
        // Java: this.mChars = str;
        self.mChars = Some(str_val);
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&mut self, elementId: i32, namespaceURI: &str) {
        // Java: IXml07Importer importer = (IXml07Importer)this.mDocImporter;
        // Java: importer.onImportExtendedProperties(elementId, this.mChars);
        // TODO: call importer.onImportExtendedProperties
        // Java: this.mChars = null;
        self.mChars = None;
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, elementId: i32, namespaceURI: &str) -> &mut Self {
        // Java: return this;
        self
    }
}
