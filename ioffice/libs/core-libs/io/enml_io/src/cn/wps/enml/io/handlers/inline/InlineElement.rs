#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.enml.io.ENMLEndOfImportException
// Java: import cn.wps.enml.io.IENMLDocumentImporter
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
// Java: import cn.wps.enml.io.handlers.HandlerHelper
// Java: import cn.wps.enml.io.handlers.HandlerPool
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;
// Java: import cn.wps.enml.io.handlers.XmlDefaultHandler
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::{XmlDefaultHandler, ElementHandler, ElementPath};
// Java: import org.dom4j.ElementPath
// Java: import org.dom4j.io.DocumentEndOfParseException

// Java: public class InlineElement extends XmlDefaultHandler
pub struct InlineElement {
    // Java field: IENMLDocumentImporter mImporter
    pub(crate) mImporter: Box<dyn IENMLDocumentImporter>,
    // Java field: HandlerPool mPool
    pub(crate) mPool: HandlerPool,
    // Java field: boolean mRegistered
    mRegistered: bool,
}

// Java: public class InlineElement extends XmlDefaultHandler
impl InlineElement {
    // Java: InlineElement(IENMLDocumentImporter importer, HandlerPool pool)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool) -> Self {
      // Java: super();
      // Java: Assert.assertNotNull("importer should not be null", importer);
      // Java: Assert.assertNotNull("pool should not be null", pool);
      // Java: this.mImporter = importer;
      // Java: this.mPool = pool;
      InlineElement {
          mImporter: importer,
          mPool: pool,
          mRegistered: false,
      }
    }

    // Java: void onStart(ElementPath elementPath)
    pub fn onStart_val(&mut self, _elementPath: &dyn ElementPath) {
      // Java: if (!this.mRegistered)
      if !self.mRegistered {
         // Java: HandlerHelper.registerInlinelElements(elementPath, this.mPool, this.mImporter);
         todo!("HandlerHelper.registerInlinelElements")
         // Java: this.mRegistered = true;
      }
    }

    // Java: void onCharacters(String chars)
    pub fn onCharacters_val(&mut self, chars: &str) {
      // Java: Assert.assertNotNull("chars should not be null", chars);
      // Java: try
      // Java: this.mImporter.onCharacters(chars);
      // Java: catch (ENMLEndOfImportException var3)
      // Java: throw new DocumentEndOfParseException();
      self.mImporter.onCharacters(chars);
    }

    // Java: void onEnd(ElementPath elementPath)
    pub fn onEnd_val(&mut self, _elementPath: &dyn ElementPath) {
    }
}

impl ElementHandler for InlineElement {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
