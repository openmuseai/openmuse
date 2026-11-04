#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.enml.io.ENMLEndOfImportException
// Java: import cn.wps.enml.io.IENMLDocumentImporter
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
// Java: import cn.wps.enml.io.handlers.HandlerPool
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::{ElementHandler, ElementPath};
use crate::cn::wps::enml::io::handlers::inline::InlineElement::InlineElement;
// Java: import org.dom4j.ElementPath
// Java: import org.dom4j.io.DocumentEndOfParseException

// Java field: String BR
pub(crate) const BR: &str = "br";

// Java: public class BrHandler extends InlineElement
pub struct BrHandler {
    base: InlineElement,
}

// Java: public class BrHandler extends InlineElement
impl BrHandler {
    // Java: BrHandler(IENMLDocumentImporter importer, HandlerPool pool)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool) -> Self {
      // Java: super(importer, pool);
      BrHandler { base: InlineElement::new(importer, pool) }
    }

    // Java: String filterName()
    pub fn filterName(&self) -> String {
      // Java: return "br";
      "br".to_string()
    }

    // Java: void onStart(ElementPath elementPath)
    pub fn onStart_val(&mut self, elementPath: &dyn ElementPath) {
      // Java: super.onStart(elementPath);
      self.base.onStart_val(elementPath);
      // Java: try
      // Java: this.mImporter.onInsertLineBreak();
      // Java: catch (ENMLEndOfImportException var3)
      // Java: throw new DocumentEndOfParseException();
      self.base.mImporter.onInsertLineBreak();
    }
}

impl ElementHandler for BrHandler {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.base.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.base.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
