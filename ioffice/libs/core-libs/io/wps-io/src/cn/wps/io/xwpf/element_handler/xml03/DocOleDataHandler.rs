#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.element_handler.IXml03Importer
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler

// Java: public class DocOleDataHandler extends XmlSimpleNodeElementHandler
pub struct DocOleDataHandler {
    // Java field: OleBinDataHandler mBinDataHandler
    mBinDataHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class DocOleDataHandler extends XmlSimpleNodeElementHandler
impl DocOleDataHandler {
    // Java: DocOleDataHandler(IXml03Importer importer)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super((IDocumentImporter)importer);
        DocOleDataHandler {
            mBinDataHandler: None,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, element_id: i32, _namespace_uri: &str) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: switch (elementId)
        match element_id {
            // Java: case -109181615:
            -109181615_i32 => {
                // Java: return this.getBinDataHandler();
                self.getBinDataHandler()
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: OleBinDataHandler getBinDataHandler()
    fn getBinDataHandler(&mut self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (null == this.mBinDataHandler)
        if self.mBinDataHandler.is_none() {
            // Java: this.mBinDataHandler = new OleBinDataHandler((IXml03Importer)this.mDocImporter);
            self.mBinDataHandler = Some(Box::new(()));
        }
        // Java: return this.mBinDataHandler;
        self.mBinDataHandler.as_ref()
    }
}
