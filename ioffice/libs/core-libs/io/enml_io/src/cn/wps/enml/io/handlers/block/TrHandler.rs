#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.enml.io.IENMLDocumentImporter
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
// Java: import cn.wps.enml.io.handlers.HandlerPool
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;
// Java: import cn.wps.enml.io.handlers.XmlDefaultHandler
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::{XmlDefaultHandler, ElementHandler, ElementPath};
// Java: import org.dom4j.ElementPath

// Java field: String FILTER_NAME
pub(crate) const FILTER_NAME: &str = "tr";

// Java: public class TrHandler extends XmlDefaultHandler
pub struct TrHandler {
    // Java field: IENMLDocumentImporter mImporter
    mImporter: Box<dyn IENMLDocumentImporter>,
    // Java field: HandlerPool mPool
    mPool: HandlerPool,
    // Java field: boolean mRegistered
    mRegistered: bool,
    // Java field: int mTableLayer
    mTableLayer: i32,
}

// Java: public class TrHandler extends XmlDefaultHandler
impl TrHandler {
    // Java: TrHandler(IENMLDocumentImporter importer, HandlerPool pool, int layer)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool, layer: i32) -> Self {
      // Java: super();
      // Java: Assert.assertNotNull("importer should not be null", importer);
      // Java: Assert.assertNotNull("pool should not be null", pool);
      // Java: Assert.assertNotNull("layer >= 1 should be true", layer >= 1);
      assert!(layer >= 1, "layer >= 1 should be true");
      // Java: this.mImporter = importer;
      // Java: this.mPool = pool;
      // Java: this.mTableLayer = layer;
      TrHandler {
          mImporter: importer,
          mPool: pool,
          mRegistered: false,
          mTableLayer: layer,
      }
    }

    // Java: String filterName()
    pub fn filterName(&self) -> String {
      // Java: return "tr";
      "tr".to_string()
    }

    // Java: void onStart(ElementPath elementPath)
    pub fn onStart_val(&mut self, _elementPath: &dyn ElementPath) {
      // Java: if (!this.mRegistered)
      if !self.mRegistered {
         // Java: TdHandler tdHandler = new TdHandler(this.mImporter, this.mPool, this.mTableLayer);
         // Java: elementPath.addHandler(tdHandler.filterName(), tdHandler);
         todo!("TdHandler not yet available as owned instance")
         // Java: this.mRegistered = true;
      }
    }

    // Java: void onCharacters(String chars)
    pub fn onCharacters_val(&mut self, _chars: &str) {
    }

    // Java: void onEnd(ElementPath elementPath)
    pub fn onEnd_val(&mut self, _elementPath: &dyn ElementPath) {
    }
}

impl ElementHandler for TrHandler {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
