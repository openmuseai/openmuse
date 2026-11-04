#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.IXml07Importer
use crate::io::xwpf::IXml07Importer::IXml07Importer;
use super::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;

// Java field: int TYPE_NORMAL_LOAD
pub const TYPE_NORMAL_LOAD: i32 = 0;
// Java field: int TYPE_FIRST_PRELOAD
pub const TYPE_FIRST_PRELOAD: i32 = 1;
// Java field: int TYPE_SMART_CHART_PRELOAD
pub const TYPE_SMART_CHART_PRELOAD: i32 = 2;

// Java: public class Xml07HandlerBase extends XmlSimpleNodeElementHandler
pub struct Xml07HandlerBase {
    base: XmlSimpleNodeElementHandler,
    // Java field: int mLoadType
    pub(crate) mLoadType: i32,
}

// Java: public class Xml07HandlerBase extends XmlSimpleNodeElementHandler
impl Xml07HandlerBase {
    // Java: Xml07HandlerBase(IXml07Importer importer)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super((IDocumentImporter)importer);
        Xml07HandlerBase {
            base: XmlSimpleNodeElementHandler::new(),
            mLoadType: TYPE_NORMAL_LOAD,
        }
    }

    // Java: void setLoadType(int val)
    pub fn setLoadType(&mut self, val: i32) {
        // Java: this.mLoadType = val;
        self.mLoadType = val;
    }
}
