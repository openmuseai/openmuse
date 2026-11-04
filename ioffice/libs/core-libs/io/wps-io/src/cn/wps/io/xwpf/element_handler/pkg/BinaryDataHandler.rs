#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.xwpf.IXml07Importer
use crate::io::xwpf::IXml07Importer::IXml07Importer;
// Java: import cn.wps.io.xwpf.element_handler.Xml07HandlerBase
use crate::io::xwpf::element_handler::Xml07HandlerBase::Xml07HandlerBase;

// Java: public class BinaryDataHandler extends Xml07HandlerBase
pub struct BinaryDataHandler {
    base: Xml07HandlerBase,
    // Java field: String mPartName
    pub(crate) mPartName: Option<String>,
    // Java field: String mContentType
    pub(crate) mContentType: Option<String>,
}

// Java: public class BinaryDataHandler extends Xml07HandlerBase
impl BinaryDataHandler {
    // Java: BinaryDataHandler(IXml07Importer importer)
    pub fn new(importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(importer);
        BinaryDataHandler {
            base: Xml07HandlerBase::new(importer),
            mPartName: None,
            mContentType: None,
        }
    }

    // Java: void setPartName(String name)
    pub fn setPartName(&mut self, name: &str) {
        // Java: this.mPartName = name;
        self.mPartName = Some(name.to_string());
    }

    // Java: void setContentType(String contentType)
    pub fn setContentType(&mut self, contentType: &str) {
        // Java: this.mContentType = contentType;
        self.mContentType = Some(contentType.to_string());
    }

    // Java: void onCharacters(char[] buffer, int start, int len)
    pub fn onCharacters_buf(&self, buffer: &[char], start: i32, len: i32) {
        // Java: IXml07Importer importer = (IXml07Importer)this.mDocImporter;
        // Java: importer.onImportBinary(this.mLoadType, this.mPartName, this.mContentType, (String)null, buffer, start, len);
        // TODO: call importer.onImportBinary
    }

    // Java: void onCharacters(String presistPath)
    pub fn onCharacters_str(&self, presistPath: &str) {
        // Java: IXml07Importer importer = (IXml07Importer)this.mDocImporter;
        // Java: importer.onImportBinary(this.mLoadType, this.mPartName, this.mContentType, presistPath, (char[])null, 0, 0);
        // TODO: call importer.onImportBinary
    }
}
