#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IVmlImporter
// Java: import cn.wps.io.xwpf.element_handler.vml.BackgroundVmlHandler
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

// Java: public class BackgroundHandler extends XmlSimpleNodeElementHandler
pub struct BackgroundHandler {
    // Java field: POIXMLDocumentPart mPart
    mPart: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: BackgroundVmlHandler mBackgroundVmlHandler
    mBackgroundVmlHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class BackgroundHandler extends XmlSimpleNodeElementHandler
impl BackgroundHandler {
    // Java: BackgroundHandler(IVmlImporter importer, POIXMLDocumentPart part)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>, part: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(importer);
        // Java: this.mPart = part;
        BackgroundHandler {
            mPart: Some(part),
            mBackgroundVmlHandler: None,
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, _element_id: i32, _namespace_uri: &str, _local_name: &str, _qualified_name: &str, _attributes: &dyn std::any::Any) {
        // Java: this.mVmlImporter.onImportWBackgroundStart(this.mPart, elementId, attributes);
        todo!()
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&self, _element_id: i32, _namespace_uri: &str) {
        // Java: this.mVmlImporter.onImportBackgroundEnd(elementId, namespaceURI);
        todo!()
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, element_id: i32, namespace_uri: &str) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: switch (elementId)
        match element_id {
            // Java: case -1332194002:
            -1332194002_i32 => {
                // Java: if (namespaceURI.equals("urn:schemas-microsoft-com:vml"))
                if namespace_uri == "urn:schemas-microsoft-com:vml" {
                    // Java: return this.getVmlBackgroundHandler();
                    return self.getVmlBackgroundHandler();
                }
                // Java: default:
                // Java: return null;
                None
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: ElementHandlerSimple getVmlBackgroundHandler()
    fn getVmlBackgroundHandler(&mut self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (null == this.mBackgroundVmlHandler)
        if self.mBackgroundVmlHandler.is_none() {
            // Java: this.mBackgroundVmlHandler = new BackgroundVmlHandler(this.mVmlImporter);
            todo!()
        }
        // Java: return this.mBackgroundVmlHandler;
        self.mBackgroundVmlHandler.as_ref()
    }
}
