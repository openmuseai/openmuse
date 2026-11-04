#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.element_handler.IXml03Importer
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

// Java: public class CustomDocumentPropertiesHandler extends XmlSimpleNodeElementHandler
pub struct CustomDocumentPropertiesHandler {
    // Java field: IXml03Importer iXml03Importer
    iXml03Importer: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: CustomDocumentPropertiesChildHandler mCustomDocumentPropertiesChildHandler
    mCustomDocumentPropertiesChildHandler: Option<CustomDocumentPropertiesChildHandler>,
}

// Java: public class CustomDocumentPropertiesHandler extends XmlSimpleNodeElementHandler
impl CustomDocumentPropertiesHandler {
    // Java: CustomDocumentPropertiesHandler(IXml03Importer importer)
    pub fn new(importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super((IDocumentImporter)importer);
        // Java: this.iXml03Importer = importer;
        CustomDocumentPropertiesHandler {
            iXml03Importer: Some(importer),
            mCustomDocumentPropertiesChildHandler: None,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, _element_id: i32, _namespace_uri: &str) -> Option<&CustomDocumentPropertiesChildHandler> {
        // Java: if (this.mCustomDocumentPropertiesChildHandler == null)
        if self.mCustomDocumentPropertiesChildHandler.is_none() {
            // Java: this.mCustomDocumentPropertiesChildHandler = new CustomDocumentPropertiesChildHandler(this.iXml03Importer);
            self.mCustomDocumentPropertiesChildHandler = Some(CustomDocumentPropertiesChildHandler::new(Box::new(())));
        }
        // Java: this.mCustomDocumentPropertiesChildHandler.clear();
        if let Some(ref mut handler) = self.mCustomDocumentPropertiesChildHandler {
            handler.clear();
        }
        // Java: return this.mCustomDocumentPropertiesChildHandler;
        self.mCustomDocumentPropertiesChildHandler.as_ref()
    }
}

// Java: public class CustomDocumentPropertiesChildHandler
pub struct CustomDocumentPropertiesChildHandler {
    // Java field: IXml03Importer mIXml03Importer
    mIXml03Importer: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: String mDt
    mDt: Option<String>,
    // Java field: String mLocalName
    mLocalName: Option<String>,
    // Java field: String mContent
    mContent: Option<String>,
}

// Java: public class CustomDocumentPropertiesChildHandler
impl CustomDocumentPropertiesChildHandler {
    // Java: CustomDocumentPropertiesChildHandler(IXml03Importer importer)
    pub fn new(importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super((IDocumentImporter)importer);
        // Java: this.mIXml03Importer = importer;
        CustomDocumentPropertiesChildHandler {
            mIXml03Importer: Some(importer),
            mDt: None,
            mLocalName: None,
            mContent: None,
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&mut self, _element_id: i32, _namespace_uri: &str, local_name: &str, _qualified_name: &str, _attributes: &dyn std::any::Any) {
        // Java: this.mLocalName = localName;
        self.mLocalName = Some(local_name.to_string());
        // Java: this.mDt = attributes.getValue("dt");
        self.mDt = None;
    }

    // Java: void onCharacters(char[] buffer, int start, int len)
    pub fn onCharacters(&mut self, buffer: &[char], start: usize, len: usize) {
        // Java: super.onCharacters(buffer, start, len);
        // Java: this.mContent = new String(buffer, start, len);
        let content: String = buffer[start..start + len].iter().collect();
        self.mContent = Some(content);
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&mut self, _element_id: i32, _namespace_uri: &str) {
        // Java: super.onEnd(elementId, namespaceURI);
        // Java: this.mIXml03Importer.onImportCustomPropertyEnd(elementId, this.mContent, this.mLocalName, this.mDt);
        todo!()
    }

    // Java: void clear()
    fn clear(&mut self) {
        // Java: this.mDt = null;
        self.mDt = None;
        // Java: this.mLocalName = null;
        self.mLocalName = None;
        // Java: this.mContent = null;
        self.mContent = None;
    }
}
