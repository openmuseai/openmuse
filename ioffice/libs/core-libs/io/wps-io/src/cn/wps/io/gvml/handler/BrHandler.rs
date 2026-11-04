#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.IGVmlDocImporter
// Java: import cn.wps.io.xwpf.element_handler.IPropHandler
// Java: import cn.wps.io.xwpf.element_handler.RprHandler
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.SAXException

// Java: public class BrHandler extends XmlSimpleNodeElementHandler implements IPropHandler
pub struct BrHandler {
    // Java field: RprHandler mRprHandler
    mRprHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: SubDocType mSubDocType
    mSubDocType: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: POIXMLDocumentPart mPart
    mPart: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: int mTableLayer
    mTableLayer: i32,
}

// Java: public class BrHandler extends XmlSimpleNodeElementHandler implements IPropHandler
impl BrHandler {
    // Java: BrHandler(POIXMLDocumentPart part, IGVmlDocImporter importer, SubDocType subDocType, int tableLayer)
    pub fn new(part: Box<dyn std::any::Any + Send + Sync>, _importer: Box<dyn std::any::Any + Send + Sync>, sub_doc_type: Box<dyn std::any::Any + Send + Sync>, table_layer: i32) -> Self {
        // Java: super((IDocumentImporter)importer);
        // Java: this.mPart = part;
        // Java: this.mSubDocType = subDocType;
        // Java: this.mTableLayer = tableLayer;
        BrHandler {
            mRprHandler: None,
            mSubDocType: Some(sub_doc_type),
            mPart: Some(part),
            mTableLayer: table_layer,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, element_id: i32, _namespace_uri: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: switch (elementId)
        match element_id {
            // Java: case 112148:
            112148 => {
                // Java: return this.getRprHandler();
                todo!()
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&mut self, _element_id: i32, _namespace_uri: &str) {
        // Java: this.mDocImporter.onImportRun(this.mSubDocType, this.getProp(), "\u000b");
        // Java: this.clearProp();
        todo!()
    }

    // Java: ElementHandlerSimple getRprHandler()
    fn getRprHandler(&mut self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (null == this.mRprHandler)
        if self.mRprHandler.is_none() {
            // Java: this.mRprHandler = new RprHandler(this.mDocImporter);
            todo!()
        }
        // Java: return this.mRprHandler;
        self.mRprHandler.as_ref()
    }

    // Java: MutablePropertySet getProp()
    pub fn getProp(&self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: return null != this.mRprHandler ? this.mRprHandler.getProp() : null;
        todo!()
    }

    // Java: void clearProp()
    pub fn clearProp(&mut self) {
        // Java: if (null != this.mRprHandler)
        if self.mRprHandler.is_some() {
            // Java: this.mRprHandler.clearProp();
            todo!()
        }
    }
}
