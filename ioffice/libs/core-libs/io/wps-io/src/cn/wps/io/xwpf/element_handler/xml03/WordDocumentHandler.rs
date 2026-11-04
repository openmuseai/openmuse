#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDocumentImporter
// Java: import cn.wps.io.xwpf.element_handler.IXml03Importer
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
// Java: import cn.wps.io.xwpf.element_handler.font.FontsHandler
// Java: import cn.wps.io.xwpf.element_handler.settings.SettingsHandler
// Java: import cn.wps.io.xwpf.element_handler.styles.StylesHandler
// Java: import cn.wps.io.xwpf.element_handler.xml03.lists.ListsHandler
// Java: import cn.wps.io.xwpf.usermodel.XWPFSettings

// Java: public class WordDocumentHandler extends XmlSimpleNodeElementHandler
pub struct WordDocumentHandler {
    _private: (),
}

// Java: public class WordDocumentHandler extends XmlSimpleNodeElementHandler
impl WordDocumentHandler {
    // Java: WordDocumentHandler(IXml03Importer iXml03Importer)
    pub fn new(_i_xml03_importer: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super((IDocumentImporter)iXml03Importer);
        WordDocumentHandler { _private: () }
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, element_id: i32, _namespace_uri: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: switch (elementId)
        match element_id {
            // Java: case -1391700465:
            -1391700465_i32 => {
                // Java: return new BgPictHandler(this.mDocImporter.getVmlImporter(SubDocType.MAIN_DOCUMENT), (IXml03Importer)this.mDocImporter);
                todo!()
            }
            // Java: case -942829362:
            -942829362_i32 => {
                // Java: return new DocumentPropertiesHandler((IXml03Importer)this.mDocImporter);
                todo!()
            }
            // Java: case -923097569:
            -923097569_i32 => {
                // Java: return new CustomDocumentPropertiesHandler((IXml03Importer)this.mDocImporter);
                todo!()
            }
            // Java: case -891774750:
            -891774750_i32 => {
                // Java: return new StylesHandler(this.mDocImporter);
                todo!()
            }
            // Java: case -210239078:
            -210239078_i32 => {
                // Java: return new DocOleDataHandler((IXml03Importer)this.mDocImporter);
                todo!()
            }
            // Java: case 3029410:
            3029410 => {
                // Java: return new BodyHandler((IXml03Importer)this.mDocImporter);
                todo!()
            }
            // Java: case 95756634:
            95756634 => {
                // Java: return new SettingsHandler(this.mDocImporter, (XWPFSettings)null);
                todo!()
            }
            // Java: case 97615364:
            97615364 => {
                // Java: return new FontsHandler(this.mDocImporter);
                todo!()
            }
            // Java: case 102982549:
            102982549 => {
                // Java: return new ListsHandler((IXml03Importer)this.mDocImporter);
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
