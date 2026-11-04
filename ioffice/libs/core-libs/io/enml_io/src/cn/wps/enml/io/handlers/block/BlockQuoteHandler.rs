#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.enml.io.IENMLDocumentImporter
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
// Java: import cn.wps.enml.io.handlers.HandlerPool
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::{ElementHandler, ElementPath};
use super::BlockLevelElementHandler::BlockLevelElementHandler;

// Java field: String BLOCKQUOTE
pub(crate) const BLOCKQUOTE: &str = "blockquote";

// Java: public class BlockQuoteHandler extends BlockLevelElementHandler
pub struct BlockQuoteHandler {
    base: BlockLevelElementHandler,
}

// Java: public class BlockQuoteHandler extends BlockLevelElementHandler
impl BlockQuoteHandler {
    // Java: BlockQuoteHandler(IENMLDocumentImporter importer, HandlerPool pool)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool) -> Self {
      // Java: super(importer, pool);
      BlockQuoteHandler { base: BlockLevelElementHandler::new(importer, pool) }
    }

    // Java: String filterName()
    pub fn filterName(&self) -> String {
      // Java: return "blockquote";
      "blockquote".to_string()
    }
}

impl ElementHandler for BlockQuoteHandler {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.base.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.base.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.base.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
