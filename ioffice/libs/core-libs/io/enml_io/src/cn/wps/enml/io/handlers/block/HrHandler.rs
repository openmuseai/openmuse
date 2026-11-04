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

// Java field: String HR
pub(crate) const HR: &str = "hr";

// Java: public class HrHandler extends BlockLevelElementHandler
pub struct HrHandler {
    base: BlockLevelElementHandler,
}

// Java: public class HrHandler extends BlockLevelElementHandler
impl HrHandler {
    // Java: HrHandler(IENMLDocumentImporter importer, HandlerPool pool)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool) -> Self {
      // Java: super(importer, pool);
      HrHandler { base: BlockLevelElementHandler::new(importer, pool) }
    }

    // Java: String filterName()
    pub fn filterName(&self) -> String {
      // Java: return "hr";
      "hr".to_string()
    }

    // Java: void onStart(ElementPath elementPath)
    pub fn onStart_val(&mut self, _elementPath: &dyn ElementPath) {
      // Java: Assert.assertNotNull("elementPath should not be null", elementPath);
      // Java: if (!this.mRegistered)
      if !self.base.mRegistered {
         // Java: HandlerHelper.registerInlinelElements(elementPath, this.mPool, this.mImporter);
         todo!("HandlerHelper.registerInlinelElements")
         // Java: this.mRegistered = true;
      }
      // Java: this.mImporter.onInsertParaBreak();
      self.base.mImporter.onInsertParaBreak();
      // Java: this.mImporter.onInsertParaBreak();
      self.base.mImporter.onInsertParaBreak();
    }

    // Java: void onEnd(ElementPath elementPath)
    pub fn onEnd_val(&mut self, _elementPath: &dyn ElementPath) {
    }
}

impl ElementHandler for HrHandler {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.base.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
