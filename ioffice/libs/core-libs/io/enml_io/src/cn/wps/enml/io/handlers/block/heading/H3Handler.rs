#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::{ElementHandler, ElementPath};
use crate::cn::wps::enml::io::handlers::block::BlockLevelElementHandler::BlockLevelElementHandler;

// Java field: String H3
pub(crate) const H3: &str = "h3";

// Java: public class H3Handler extends BlockLevelElementHandler
pub struct H3Handler { base: BlockLevelElementHandler }

// Java: public class H3Handler extends BlockLevelElementHandler
impl H3Handler {
    // Java: H3Handler(IENMLDocumentImporter importer, HandlerPool pool)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool) -> Self {
      // Java: super(importer, pool);
      H3Handler { base: BlockLevelElementHandler::new(importer, pool) }
    }
    // Java: String filterName()
    pub fn filterName(&self) -> String {
      // Java: return "h3";
      "h3".to_string()
    }
}

impl ElementHandler for H3Handler {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.base.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.base.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.base.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
