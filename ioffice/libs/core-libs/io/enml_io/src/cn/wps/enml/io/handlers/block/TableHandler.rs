#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::KSAssert;
// Java: import cn.wps.enml.io.IENMLDocumentImporter
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
// Java: import cn.wps.enml.io.handlers.HandlerPool
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;
// Java: import cn.wps.enml.io.handlers.XmlDefaultHandler
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::{XmlDefaultHandler, ElementHandler, ElementPath};
// Java: import org.dom4j.ElementPath

// Java field: String TABLE
pub(crate) const TABLE: &str = "table";

// Java: public class TableHandler extends XmlDefaultHandler
pub struct TableHandler {
    // Java field: IENMLDocumentImporter mImporter
    mImporter: Box<dyn IENMLDocumentImporter>,
    // Java field: HandlerPool mPool
    mPool: HandlerPool,
    // Java field: boolean mRegistered
    mRegistered: bool,
    // Java field: int mTableLayer
    pub(crate) mTableLayer: i32,
}

// Java: public class TableHandler extends XmlDefaultHandler
impl TableHandler {
    // Java: TableHandler(IENMLDocumentImporter importer, HandlerPool pool)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool) -> Self {
      // Java: super();
      // Java: Assert.assertNotNull("importer should not be null", importer);
      // Java: Assert.assertNotNull("pool should not be null", pool);
      // Java: this.mImporter = importer;
      // Java: this.mPool = pool;
      TableHandler {
          mImporter: importer,
          mPool: pool,
          mRegistered: false,
          mTableLayer: 0,
      }
    }

    // Java: String filterName()
    pub fn filterName(&self) -> String {
      // Java: return "table";
      "table".to_string()
    }

    // Java: int getTableLayer()
    pub fn getTableLayer(&self) -> i32 {
      // Java: return this.mTableLayer;
      self.mTableLayer
    }

    // Java: void setTableLayer(int layer)
    pub fn setTableLayer(&mut self, layer: i32) {
      // Java: Assert.assertNotNull("layer >= 0 should be true", layer >= 0);
      assert!(layer >= 0, "layer >= 0 should be true");
      // Java: this.mTableLayer = layer;
      self.mTableLayer = layer;
    }

    // Java: void onStart(ElementPath elementPath)
    pub fn onStart_val(&mut self, _elementPath: &dyn ElementPath) {
      // Java: KSAssert.assertNotNull("elementPath should not be null", elementPath);
      // Java: if (!this.mRegistered)
      if !self.mRegistered {
         // Java: TBodyHandler tbodyHandler = new TBodyHandler(this.mImporter, this.mPool, this.mTableLayer);
         // Java: elementPath.addHandler(tbodyHandler.filterName(), tbodyHandler);
         // Java: TrHandler trHandler = new TrHandler(this.mImporter, this.mPool, this.mTableLayer);
         // Java: elementPath.addHandler(trHandler.filterName(), trHandler);
         todo!("TBodyHandler and TrHandler not yet converted")
         // Java: this.mRegistered = true;
      }
    }

    // Java: void onCharacters(String chars)
    pub fn onCharacters_val(&mut self, _chars: &str) {
    // Java: (empty body)
    }

    // Java: void onEnd(ElementPath elementPath)
    pub fn onEnd_val(&mut self, _elementPath: &dyn ElementPath) {
    // Java: (empty body)
    }
}

impl ElementHandler for TableHandler {
    fn onStart(&mut self, var1: &dyn ElementPath) {
        self.onStart_val(var1);
    }

    fn onCharacters(&mut self, var1: &str) {
        self.onCharacters_val(var1);
    }

    fn onEnd(&mut self, var1: &dyn ElementPath) {
        self.onEnd_val(var1);
    }

    fn shouldNotifyChild(&self) -> bool {
        false
    }

    fn onStartChild(&mut self, _var1: &dyn ElementPath) {
    }

    fn onEndChild(&mut self, _var1: &dyn ElementPath) {
    }
}
