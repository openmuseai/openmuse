#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.enml.io.IENMLDocumentImporter
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
// Java: import cn.wps.enml.io.handlers.HandlerPool
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::{ElementHandler, ElementPath};
use super::BlockLevelElementHandler::BlockLevelElementHandler;
// Java: import org.dom4j.ElementPath
// Java: import org.dom4j.io.DocumentEndOfParseException

// Java field: String FILTER_NAME
pub(crate) const FILTER_NAME: &str = "td";

// Java: public class TdHandler extends BlockLevelElementHandler
pub struct TdHandler {
    base: BlockLevelElementHandler,
}

// Java: public class TdHandler extends BlockLevelElementHandler
impl TdHandler {
    // Java: TdHandler(IENMLDocumentImporter importer, HandlerPool pool, int layer)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool, layer: i32) -> Self {
      // Java: super(importer, pool, layer);
      TdHandler { base: BlockLevelElementHandler::new_with_layer(importer, pool, layer) }
    }

    // Java: String filterName()
    pub fn filterName(&self) -> String {
      // Java: return "td";
      "td".to_string()
    }

    // Java: void onStart(ElementPath elementPath)
    pub fn onStart_val(&mut self, elementPath: &dyn ElementPath) {
      // Java: super.onStart(elementPath);
      self.base.onStart_val(elementPath);
    }

    // Java: void onEnd(ElementPath elementPath)
    pub fn onEnd_val(&mut self, _elementPath: &dyn ElementPath) {
    }
}

impl ElementHandler for TdHandler {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.base.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
