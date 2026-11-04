#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.element_handler.IXml03Importer
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import org.xml.sax.SAXException

// Java: public class DocumentPropertiesHandler extends XmlSimpleNodeElementHandler
pub struct DocumentPropertiesHandler {
    // Java field: IXml03Importer mIXml03Importer
    mIXml03Importer: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: DocumentPropertiesChildHandler mDocumentPropertiesChildHandler
    mDocumentPropertiesChildHandler: Option<DocumentPropertiesChildHandler>,
}

// Java: public class DocumentPropertiesHandler extends XmlSimpleNodeElementHandler
impl DocumentPropertiesHandler {
    // Java: DocumentPropertiesHandler(IXml03Importer xml03Importer)
    pub fn new(xml03_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super((IDocumentImporter)xml03Importer);
        // Java: this.mIXml03Importer = xml03Importer;
        DocumentPropertiesHandler {
            mIXml03Importer: Some(xml03_importer),
            mDocumentPropertiesChildHandler: None,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, element_id: i32, _namespace_uri: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: switch (elementId)
        match element_id {
            // Java: case -2043158851:
            // Java: case -2003091227:
            // Java: case -1971855305:
            // Java: case -1795053683:
            // Java: case -1679829923:
            // Java: case -1601759544:
            // Java: case -1056078198:
            // Java: case -876104735:
            // Java: case -289326735:
            // Java: case -203231988:
            // Java: case -196142085:
            // Java: case -56677412:
            // Java: case 73424607:
            // Java: case 76873636:
            // Java: case 80818744:
            // Java: case 83765897:
            // Java: case 115155230:
            // Java: case 587793354:
            // Java: case 1426685969:
            // Java: case 1972506027:
            // Java: case 2016261304:
            -2043158851_i32 | -2003091227_i32 | -1971855305_i32 | -1795053683_i32 | -1679829923_i32 |
            -1601759544_i32 | -1056078198_i32 | -876104735_i32 | -289326735_i32 | -203231988_i32 |
            -196142085_i32 | -56677412_i32 | 73424607 | 76873636 | 80818744 |
            83765897 | 115155230 | 587793354 | 1426685969 | 1972506027 | 2016261304 => {
                // Java: if (this.mDocumentPropertiesChildHandler != null)
                if self.mDocumentPropertiesChildHandler.is_some() {
                    // Java: this.mDocumentPropertiesChildHandler.content = null;
                    if let Some(ref mut handler) = self.mDocumentPropertiesChildHandler {
                        handler.content = None;
                    }
                    // Java: return this.mDocumentPropertiesChildHandler;
                    return Some(Box::new(()));
                }
                // Java: return new DocumentPropertiesChildHandler(this.mIXml03Importer);
                self.mDocumentPropertiesChildHandler = Some(DocumentPropertiesChildHandler::new(Box::new(())));
                Some(Box::new(()))
            }
            // Java: default:
            _ => {
                // Java: return null;
                None
            }
        }
    }
}

// Java: public class DocumentPropertiesChildHandler
pub struct DocumentPropertiesChildHandler {
    // Java field: IXml03Importer mIXml03Importer
    mIXml03Importer: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: String content
    pub content: Option<String>,
}

// Java: public class DocumentPropertiesChildHandler
impl DocumentPropertiesChildHandler {
    // Java: DocumentPropertiesChildHandler(IXml03Importer xml03Importer)
    pub fn new(xml03_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super((IDocumentImporter)xml03Importer);
        // Java: this.mIXml03Importer = xml03Importer;
        DocumentPropertiesChildHandler {
            mIXml03Importer: Some(xml03_importer),
            content: None,
        }
    }

    // Java: void onCharacters(char[] buffer, int start, int len)
    pub fn onCharacters(&mut self, buffer: &[char], start: usize, len: usize) {
        // Java: super.onCharacters(buffer, start, len);
        // Java: this.content = new String(buffer, start, len);
        let s: String = buffer[start..start + len].iter().collect();
        self.content = Some(s);
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&mut self, _element_id: i32, _namespace_uri: &str) {
        // Java: super.onEnd(elementId, namespaceURI);
        // Java: this.mIXml03Importer.onImportExtendedProperties(elementId, this.content);
        todo!()
    }
}
