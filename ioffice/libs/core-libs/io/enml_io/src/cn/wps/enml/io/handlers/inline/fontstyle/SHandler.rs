#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::{ElementHandler, ElementPath};
use crate::cn::wps::enml::io::handlers::inline::fontstyle::StrikeHandler::StrikeHandler;

// Java field: String S
pub(crate) const S: &str = "s";

// Java: public class SHandler extends StrikeHandler
pub struct SHandler { base: StrikeHandler }

// Java: public class SHandler extends StrikeHandler
impl SHandler {
    // Java: SHandler(IENMLDocumentImporter importer, HandlerPool pool)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool) -> Self {
      // Java: super(importer, pool);
      SHandler { base: StrikeHandler::new(importer, pool) }
    }
    // Java: String filterName()
    pub fn filterName(&self) -> String {
      // Java: return "s";
      "s".to_string()
    }
}

impl ElementHandler for SHandler {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.base.base.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.base.base.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.base.base.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
