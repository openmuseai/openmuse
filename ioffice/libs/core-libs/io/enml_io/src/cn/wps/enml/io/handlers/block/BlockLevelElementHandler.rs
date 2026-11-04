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

// Java: public class BlockLevelElementHandler extends XmlDefaultHandler
pub struct BlockLevelElementHandler {
    // Java field: IENMLDocumentImporter mImporter
    pub(crate) mImporter: Box<dyn IENMLDocumentImporter>,
    // Java field: HandlerPool mPool
    pub(crate) mPool: HandlerPool,
    // Java field: boolean mRegistered
    pub(crate) mRegistered: bool,
    // Java field: int mTableLayer
    pub(crate) mTableLayer: i32,
}

// Java: public class BlockLevelElementHandler extends XmlDefaultHandler
impl BlockLevelElementHandler {
    // Java: BlockLevelElementHandler(IENMLDocumentImporter importer, HandlerPool pool)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool) -> Self {
      // Java: super();
      // Java: this.mTableLayer = 0;
      // Java: Assert.assertNotNull("importer should not be null", importer);
      // Java: Assert.assertNotNull("pool should not be null", pool);
      // Java: this.mImporter = importer;
      // Java: this.mPool = pool;
      BlockLevelElementHandler {
          mImporter: importer,
          mPool: pool,
          mRegistered: false,
          mTableLayer: 0,
      }
    }

    // Java: BlockLevelElementHandler(IENMLDocumentImporter importer, HandlerPool pool, int layer)
    pub fn new_with_layer(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool, layer: i32) -> Self {
      // Java: this(importer, pool);
      let mut handler = Self::new(importer, pool);
      // Java: this.setTableLayer(layer);
      handler.setTableLayer(layer);
      handler
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
      // Java: Assert.assertNotNull("elementPath should not be null", elementPath);
      // Java: if (!this.mRegistered)
      if !self.mRegistered {
         // Java: HandlerHelper.registerBlockLevelElements(elementPath, this.mPool, this.mImporter, this.mTableLayer);
         // Java: HandlerHelper.registerInlinelElements(elementPath, this.mPool, this.mImporter);
         todo!("HandlerHelper.registerBlockLevelElements / registerInlinelElements")
         // Java: this.mRegistered = true;
      }
      // Java: try
      // Java: this.mImporter.onStartBlock();
      // Java: catch (ENMLEndOfImportException var3)
      // Java: throw new DocumentEndOfParseException();
      self.mImporter.onStartBlock();
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
      // Java: Assert.assertNotNull("elementPath should not be null", elementPath);
      // Java: try
      // Java: this.mImporter.onEndBlock();
      // Java: catch (ENMLEndOfImportException var3)
      // Java: throw new DocumentEndOfParseException();
      self.mImporter.onEndBlock();
    }
}

impl ElementHandler for BlockLevelElementHandler {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
