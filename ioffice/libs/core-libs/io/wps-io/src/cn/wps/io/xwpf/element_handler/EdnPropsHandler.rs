#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.element_handler.prop.PropHandlerHelper
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

// Java: public class EdnPropsHandler extends XmlSimpleNodeElementHandler
pub struct EdnPropsHandler {
    // Java field: POIXMLDocumentPart mPart
    mPart: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: IPropHandler mPropHandler
    mPropHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: EdnPropsHandler mChildHandler
    mChildHandler: Option<Box<EdnPropsHandler>>,
}

// Java: public class EdnPropsHandler extends XmlSimpleNodeElementHandler
impl EdnPropsHandler {
    // Java: EdnPropsHandler(POIXMLDocumentPart part, IDocumentImporter importer, IPropHandler iPropHandler)
    pub fn new(part: Box<dyn std::any::Any + Send + Sync>, _importer: Box<dyn std::any::Any + Send + Sync>, prop_handler: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(importer);
        // Java: this.mPart = part;
        // Java: this.mPropHandler = iPropHandler;
        EdnPropsHandler {
            mPart: Some(part),
            mPropHandler: Some(prop_handler),
            mChildHandler: None,
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, element_id: i32, _namespace_uri: &str, _local_name: &str, _qualified_name: &str, _attributes: &dyn std::any::Any) {
        // Java: int propId = -1;
        // Java: switch (elementId)
        let _prop_id: i32 = match element_id {
            // Java: case -1917338065:
            -1917338065_i32 => {
                // Java: return;
                return;
            }
            // Java: case -1899860868:
            -1899860868_i32 => {
                // Java: propId = 29;
                // Java: break;
                29
            }
            // Java: case -1705465943:
            -1705465943_i32 => {
                // Java: propId = 30;
                // Java: break;
                30
            }
            // Java: case -1034390745:
            -1034390745_i32 => {
                // Java: propId = 28;
                // Java: break;
                28
            }
            // Java: case 111188:
            111188 => {
                // Java: propId = 27;
                // Java: break;
                27
            }
            // Java: default:
            _ => {
                // Java: return;
                return;
            }
        };
        // Java: this.mDocImporter.convertProp(PropHandlerHelper.PropType.SECTPR, this.mPropHandler.getProp(), namespaceURI, propId, attributes);
        todo!()
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, _element_id: i32, _namespace_uri: &str) -> Option<&EdnPropsHandler> {
        // Java: if (null == this.mChildHandler)
        if self.mChildHandler.is_none() {
            // Java: this.mChildHandler = new EdnPropsHandler(this.mPart, this.mDocImporter, this.mPropHandler);
            todo!()
        }
        // Java: return this.mChildHandler;
        self.mChildHandler.as_deref()
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}
