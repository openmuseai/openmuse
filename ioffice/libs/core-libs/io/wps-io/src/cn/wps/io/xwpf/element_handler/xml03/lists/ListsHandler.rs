#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.element_handler.IXml03Importer
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import cn.wps.io.xwpf.element_handler.numbering.NumHandler

// Java: public class ListsHandler extends XmlSimpleNodeElementHandler
pub struct ListsHandler {
    // Java field: ListDefHandler mListDefHandler
    mListDefHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: NumHandler mNumHandler
    mNumHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class ListsHandler extends XmlSimpleNodeElementHandler
impl ListsHandler {
    // Java: ListsHandler(IXml03Importer importer)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super((IDocumentImporter)importer);
        ListsHandler {
            mListDefHandler: None,
            mNumHandler: None,
        }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, element_id: i32, _namespace_uri: &str) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: switch (elementId)
        match element_id {
            // Java: case 3322014:
            3322014 => {
                // Java: return this.getNumHandler();
                self.getNumHandler()
            }
            // Java: case 181939847:
            181939847 => {
                // Java: return this.getListDefHandler();
                self.getListDefHandler()
            }
            // Java: default:
            _ => {
                // Java: return super.getElementHandler(elementId, namespaceURI);
                todo!()
            }
        }
    }

    // Java: NumHandler getNumHandler()
    fn getNumHandler(&mut self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (this.mNumHandler == null)
        if self.mNumHandler.is_none() {
            // Java: this.mNumHandler = new NumHandler(this.mDocImporter);
            self.mNumHandler = Some(Box::new(()));
        }
        // Java: return this.mNumHandler;
        self.mNumHandler.as_ref()
    }

    // Java: ListDefHandler getListDefHandler()
    fn getListDefHandler(&mut self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (this.mListDefHandler == null)
        if self.mListDefHandler.is_none() {
            // Java: this.mListDefHandler = new ListDefHandler((IXml03Importer)this.mDocImporter);
            self.mListDefHandler = Some(Box::new(()));
        }
        // Java: return this.mListDefHandler;
        self.mListDefHandler.as_ref()
    }
}
