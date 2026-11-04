#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::ElementHandlerSimple;
// Java: import cn.wps.io.xwpf.IDmlCommonImporter
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.IVmlImporter
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use std::rc::Rc;

use crate::io::xwpf::IImporterBase::IImporterBase;
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;

// Java: public class XmlSimpleNodeElementHandler implements ElementHandlerSimple
pub struct XmlSimpleNodeElementHandler {
    // Java field: IDocumentImporter mDocImporter
    pub(crate) mDocImporter: Option<Rc<dyn IDocumentImporter>>,
    // Java field: IDmlCommonImporter mDmlImporter
    pub(crate) mDmlImporter: Option<Box<dyn IImporterBase>>,
    // Java field: IVmlImporter mVmlImporter
    pub(crate) mVmlImporter: Option<Box<dyn IImporterBase>>,
    // Java field: int mode
    mode: i32,
}

// Java: public class XmlSimpleNodeElementHandler implements ElementHandlerSimple
impl XmlSimpleNodeElementHandler {
    // Java: XmlSimpleNodeElementHandler()
    pub fn new() -> Self {
        // Java: super();
        Self {
            mDocImporter: None,
            mDmlImporter: None,
            mVmlImporter: None,
            mode: -1,
        }
    }

    // Java: XmlSimpleNodeElementHandler(IDocumentImporter importer)
    pub fn new_from_doc(importer: Box<dyn IDocumentImporter>) -> Self {
        // Java: super();
        // Java: this.mDocImporter = importer;
        Self::new_from_doc_rc(Rc::from(importer))
    }

    pub fn new_from_doc_rc(importer: Rc<dyn IDocumentImporter>) -> Self {
        // Java: this.mDocImporter = importer;
        Self {
            mDocImporter: Some(importer),
            mDmlImporter: None,
            mVmlImporter: None,
            mode: -1,
        }
    }

    // Java: XmlSimpleNodeElementHandler(IDmlCommonImporter importer)
    pub fn new_from_dml(importer: Box<dyn IImporterBase>) -> Self {
        // Java: super();
        // Java: this.mDmlImporter = importer;
        Self {
            mDocImporter: None,
            mDmlImporter: Some(importer),
            mVmlImporter: None,
            mode: -1,
        }
    }

    // Java: XmlSimpleNodeElementHandler(IVmlImporter importer)
    pub fn new_from_vml(importer: Box<dyn IImporterBase>) -> Self {
        // Java: super();
        // Java: this.mVmlImporter = importer;
        Self {
            mDocImporter: None,
            mDmlImporter: None,
            mVmlImporter: Some(importer),
            mode: -1,
        }
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return -1;
        -1
    }

    // Java: void onCharacters(char[] buffer, int start, int len)
    pub fn onCharacters(&self, buffer: &[char], start: i32, len: i32) {
        // empty in Java
    }

    // Java: void onCharacters(String persistPath)
    pub fn onCharacters_str(&self, persistPath: &str) {
        // empty in Java
    }

    // Java: void onStart(String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
        // empty in Java
    }

    // Java: void onEnd(String namespaceURI, String localName, String qName)
    pub fn onEnd(&self, namespaceURI: &str, localName: &str, qName: &str) {
        // empty in Java
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
        // empty in Java
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: return null;
        None
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd_with_id(&self, elementId: i32, namespaceURI: &str) {
        // empty in Java
    }

    // Java: boolean isDocx()
    pub fn isDocx(&self) -> bool {
        // Java: return 0 == this.mode();
        0 == self.get_mode()
    }

    // Java: boolean isXml03()
    pub fn isXml03(&self) -> bool {
        // Java: return 3 == this.mode();
        3 == self.get_mode()
    }

    // Java: boolean isXml07()
    pub fn isXml07(&self) -> bool {
        // Java: return 1 == this.mode();
        1 == self.get_mode()
    }

    // Java: boolean isGVml()
    pub fn isGVml(&self) -> bool {
        // Java: return 2 == this.mode();
        2 == self.get_mode()
    }

    // Java: int mode()
    fn get_mode(&self) -> i32 {
        // Java: if (this.mode != -1)
        if self.mode != -1 {
            // Java: return this.mode;
            return self.mode;
        }
        // Java: else
        else {
            // Java: if (null != this.mDocImporter)
            if let Some(ref importer) = self.mDocImporter {
                // Java: this.mode = this.mDocImporter.mode();
                return importer.mode();
            }
            // Java: else if (null != this.mDmlImporter)
            else if let Some(ref importer) = self.mDmlImporter {
                // Java: this.mode = this.mDmlImporter.mode();
                return importer.mode();
            }
            // Java: else if (null != this.mVmlImporter)
            else if let Some(ref importer) = self.mVmlImporter {
                // Java: this.mode = this.mVmlImporter.mode();
                return importer.mode();
            }
            // Java: return this.mode;
            self.mode
        }
    }

}

impl ElementHandlerSimple for XmlSimpleNodeElementHandler {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn filterIndex(&self) -> i32 {
        self.filterIndex()
    }

    fn onStart(&mut self, var1: &str, var2: &str, var3: &str, var4: &dyn std::any::Any) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn onEnd(&mut self, var1: &str, var2: &str, var3: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn onStart_with_id(&mut self, var1: i32, var2: &str, var3: &str, var4: &str, var5: &dyn std::any::Any) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn onEnd_with_id(&mut self, var1: i32, var2: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn onCharacters(&mut self, var1: &[char], var2: i32, var3: i32) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn onCharacters_str(&mut self, var1: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn getElementHandler(&self, var1: i32, var2: &str) -> Option<crate::io::simple::ElementHandlerSimple::ElementHandlerRef> {
        None
    }
}
