#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::KSAssert;
// Java: import cn.wps.enml.io.IENMLDocumentImporter
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
// Java: import cn.wps.enml.io.handlers.HandlerPool
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::{ElementHandler, ElementPath};
use crate::cn::wps::enml::io::handlers::inline::InlineElement::InlineElement;
// Java: import cn.wps.enml.io.xmlbeans.XmlEnMedia
// Java: import org.dom4j.Element
// Java: import org.dom4j.ElementPath
// Java: import org.dom4j.io.DocumentEndOfParseException

// Java field: String EN_MEDIA
pub(crate) const EN_MEDIA: &str = "en-media";

// Java: public class EnMediaHandler extends InlineElement
pub struct EnMediaHandler {
    base: InlineElement,
}

// Java: public class EnMediaHandler extends InlineElement
impl EnMediaHandler {
    // Java: EnMediaHandler(IENMLDocumentImporter importer, HandlerPool pool)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool) -> Self {
      // Java: super(importer, pool);
      EnMediaHandler { base: InlineElement::new(importer, pool) }
    }

    // Java: String filterName()
    pub fn filterName(&self) -> String {
      // Java: return "en-media";
      "en-media".to_string()
    }

    // Java: void onStart(ElementPath elementPath)
    pub fn onStart_val(&mut self, _elementPath: &dyn ElementPath) {
      // Java: Element element = elementPath.getCurrent();
      // Java: KSAssert.assertNotNull("element should not be null", element);
      // Java: this.mImporter.onImportMedia(new XmlEnMedia(element));
      todo!("XmlEnMedia + elementPath.getCurrent() integration")
    }
}

impl ElementHandler for EnMediaHandler {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.base.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.base.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
