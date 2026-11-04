#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IVmlImporter
// Java: import cn.wps.io.xwpf.element_handler.IXml03Importer
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler

// Java: public class BgPictHandler extends XmlSimpleNodeElementHandler
pub struct BgPictHandler {
    // Java field: IXml03Importer mXml03Importer
    mXml03Importer: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class BgPictHandler extends XmlSimpleNodeElementHandler
impl BgPictHandler {
    // Java: BgPictHandler(IVmlImporter importer, IXml03Importer xml03Importer)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>, xml03_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(importer);
        // Java: this.mXml03Importer = xml03Importer;
        BgPictHandler {
            mXml03Importer: Some(xml03_importer),
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, element_id: i32, _namespace_uri: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: switch (elementId)
        match element_id {
            // Java: case -1332194002:
            -1332194002_i32 => {
                // Java: return new Xml03BackgroundHandler(this.mVmlImporter);
                todo!()
            }
            // Java: case -109181615:
            -109181615_i32 => {
                // Java: return new BinDataHandler(this.mXml03Importer);
                todo!()
            }
            // Java: default:
            _ => {
                // Java: return super.getElementHandler(elementId, namespaceURI);
                todo!()
            }
        }
    }
}
