#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.element_handler.IXml03Importer
// Java: import cn.wps.io.xwpf.element_handler.SectPrHandler

// Java: public class SectHandler extends SubDocumentHandler
pub struct SectHandler {
    // Java field: SectPrHandler mSectPrHandler
    mSectPrHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class SectHandler extends SubDocumentHandler
impl SectHandler {
    // Java: SectHandler(IXml03Importer importer)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(importer, SubDocType.MAIN_DOCUMENT);
        SectHandler {
            mSectPrHandler: None,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int subElement, String namespaceURI)
    pub fn getElementHandler(&mut self, sub_element: i32, _namespace_uri: &str) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: switch (subElement)
        match sub_element {
            // Java: case -906275931:
            -906275931_i32 => {
                // Java: return this.getSectPrHandler();
                self.getSectPrHandler()
            }
            // Java: default:
            _ => {
                // Java: return super.getElementHandler(subElement, namespaceURI);
                todo!()
            }
        }
    }

    // Java: SectPrHandler getSectPrHandler()
    fn getSectPrHandler(&mut self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (this.mSectPrHandler == null)
        if self.mSectPrHandler.is_none() {
            // Java: this.mSectPrHandler = new SectPrHandler((POIXMLDocumentPart)null, this.mDocImporter);
            self.mSectPrHandler = Some(Box::new(()));
        }
        // Java: return this.mSectPrHandler;
        self.mSectPrHandler.as_ref()
    }
}
