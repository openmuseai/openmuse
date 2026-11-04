#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.element_handler.IXml03Importer
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import cn.wps.io.xwpf.element_handler.numbering.LvlHandler
// Java: import cn.wps.io.xwpf.element_handler.numbering.NumLvlType
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

// Java: public class ListDefHandler extends XmlSimpleNodeElementHandler
pub struct ListDefHandler {
    // Java field: LvlHandler mLvlHandler
    mLvlHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: ListDefChildHandler mListDefChildHandler
    mListDefChildHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class ListDefHandler extends XmlSimpleNodeElementHandler
impl ListDefHandler {
    // Java: ListDefHandler(IXml03Importer importer)
    pub fn new(_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super((IDocumentImporter)importer);
        ListDefHandler {
            mLvlHandler: None,
            mListDefChildHandler: None,
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(&self, _element_id: i32, _namespace_uri: &str, _local_name: &str, _qualified_name: &str, _attributes: &dyn std::any::Any) {
        // Java: this.mDocImporter.onImportNumberingAbstractNumStart(attributes);
        todo!()
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd(&self, _element_id: i32, _namespace_uri: &str) {
        // Java: this.mDocImporter.onImportNumberingAbstractNumEnd();
        todo!()
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&mut self, element_id: i32, _namespace_uri: &str) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: return 107554 == elementId ? this.getLvlHandler() : this.getListDefChildHandler();
        if 107554 == element_id {
            self.getLvlHandler()
        } else {
            self.getListDefChildHandler()
        }
    }

    // Java: ElementHandlerSimple getLvlHandler()
    fn getLvlHandler(&mut self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (null == this.mLvlHandler)
        if self.mLvlHandler.is_none() {
            // Java: this.mLvlHandler = new LvlHandler(this.mDocImporter, NumLvlType.AbstractNum);
            self.mLvlHandler = Some(Box::new(()));
        }
        // Java: return this.mLvlHandler;
        self.mLvlHandler.as_ref()
    }

    // Java: ElementHandlerSimple getListDefChildHandler()
    fn getListDefChildHandler(&mut self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (this.mListDefChildHandler == null)
        if self.mListDefChildHandler.is_none() {
            // Java: this.mListDefChildHandler = new ListDefChildHandler((IXml03Importer)this.mDocImporter);
            self.mListDefChildHandler = Some(Box::new(()));
        }
        // Java: return this.mListDefChildHandler;
        self.mListDefChildHandler.as_ref()
    }
}
