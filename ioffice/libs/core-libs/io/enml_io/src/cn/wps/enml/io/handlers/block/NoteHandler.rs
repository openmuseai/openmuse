#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.enml.io.IENMLDocumentImporter
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
// Java: import cn.wps.enml.io.handlers.HandlerHelper
// Java: import cn.wps.enml.io.handlers.HandlerPool
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::{ElementHandler, ElementPath};
use super::BlockLevelElementHandler::BlockLevelElementHandler;
// Java: import org.dom4j.ElementPath
// Java: import org.dom4j.io.DocumentEndOfParseException

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;
// Java field: String FILTER_NAME
pub(crate) const FILTER_NAME: &str = "en-note";

// Java: public class NoteHandler extends BlockLevelElementHandler
pub struct NoteHandler {
    base: BlockLevelElementHandler,
}

// Java: public class NoteHandler extends BlockLevelElementHandler
impl NoteHandler {
    // Java: NoteHandler(IENMLDocumentImporter importer)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>) -> Self {
      // Java: super(importer, new HandlerPool());
      NoteHandler { base: BlockLevelElementHandler::new(importer, HandlerPool::new()) }
    }

    // Java: String filterName()
    pub fn filterName(&self) -> String {
      // Java: return "en-note";
      "en-note".to_string()
    }

    // Java: void onStart(ElementPath elementPath)
    pub fn onStart_val(&mut self, _elementPath: &dyn ElementPath) {
      // Java: Assert.assertNotNull("elementPath should not be null", elementPath);
      // Java: HandlerHelper.registerBlockLevelElements(elementPath, this.mPool, this.mImporter, this.mTableLayer);
      // Java: HandlerHelper.registerInlinelElements(elementPath, this.mPool, this.mImporter);
      todo!("HandlerHelper registration")
      // Java: this.mImporter.onStartDocument();
    }

    // Java: void onCharacters(String chars)
    pub fn onCharacters_val(&mut self, chars: &str) {
      // Java: super.onCharacters(chars);
      self.base.onCharacters_val(chars);
    }

    // Java: void onEnd(ElementPath elementPath)
    pub fn onEnd_val(&mut self, _elementPath: &dyn ElementPath) {
      // Java: Assert.assertNotNull("elementPath should not be null", elementPath);
      // Java: this.mImporter.onEndDocument();
      self.base.mImporter.onEndDocument();
    }
}

impl ElementHandler for NoteHandler {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
