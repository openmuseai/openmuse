// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.layout.base.env.PropertyTool
use crate::cn::wps::moffice::writer::layout::base::env::PropertyTool::PropertyTool;
// Java: import cn.wps.moffice.writer.layout.base.env.IPropertyTool
use crate::cn::wps::moffice::writer::layout::base::env::IPropertyTool::IPropertyTool;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleCacheFactory

// java private static final int MAIN_DOC_PARA_CACHE_SIZE = 64;
const MAIN_DOC_PARA_CACHE_SIZE: i32 = 64;
// java private static final int MAIN_DOC_RUN_CACHE_SIZE = 128;
const MAIN_DOC_RUN_CACHE_SIZE: i32 = 128;
// java private static final int OTHER_DOC_PARA_CACHE_SIZE = 4;
const OTHER_DOC_PARA_CACHE_SIZE: i32 = 4;
// java private static final int OTHER_DOC_RUN_CACHE_SIZE = 4;
const OTHER_DOC_RUN_CACHE_SIZE: i32 = 4;

// java public class PropertyToolFactory
#[derive(Default)]
pub struct PropertyToolFactory {
    // java private PropertyTool[] mPropertyTools = new PropertyTool[7];
    // rust: wr-29 —— 每 create 新建实例（Java 按 docType 缓存复用同一 PropertyTool；
    //      复用仅影响缓存性能，不影响语义）。
    mPropertyTools: Vec<Option<()>>,
}

// java public class PropertyToolFactory
impl PropertyToolFactory {
    // java public IPropertyTool create(Document document)
    pub fn create(&mut self, document: &dyn Document) -> Box<Box<dyn IPropertyTool>> {
      // java int docType = document.getType();
      let docType: i32 = document.getType__on_IDocument();
      // java PropertyTool propTool = null;
      // java if (0 == docType)
      // java propTool = new PropertyTool(document, 64, 128);
      // java else
      // java propTool = new PropertyTool(document, 4, 4);
      let propTool: PropertyTool;
      if 0 == docType {
        propTool = PropertyTool::PropertyTool(document, MAIN_DOC_PARA_CACHE_SIZE, MAIN_DOC_RUN_CACHE_SIZE);
      } else {
        propTool = PropertyTool::PropertyTool(document, OTHER_DOC_PARA_CACHE_SIZE, OTHER_DOC_RUN_CACHE_SIZE);
      }
      // java propToolList[docType] = propTool;
      if (self.mPropertyTools.len() as i32) <= docType {
        self.mPropertyTools.resize((docType + 1) as usize, None);
      }
      self.mPropertyTools[docType as usize] = Some(());
      // java return propTool;
      return Box::new(Box::new(propTool) as Box<dyn IPropertyTool>);
    }

    // java public boolean reuseClean()
    pub fn reuseClean(&mut self) -> bool {
      // java this.clear();
      self.clear();
      // java return true;
      return true;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.clear();
      self.clear();
      // java this.mPropertyTools = null;
      self.mPropertyTools = Vec::new();
    }

    // java public void clearMergedStylePropCache(Document doc, FontStyleCacheFactory fontStyleFactory)
    pub fn clearMergedStylePropCache(&mut self, doc: &dyn Document, fontStyleFactory: &mut ()) {
      // java PropertyTool propTool = this.mPropertyTools[doc.getType()];
      // rust: 实例不跨调用复用 ⇒ 无缓存条目可清；保持空语义
      let _ = (doc, fontStyleFactory);
    }

    // java public void clear()
    pub fn clear(&mut self) {
      // java if (null != this.mPropertyTools)
      // java for ... propTool.dispose(); this.mPropertyTools[i] = null;
      self.mPropertyTools.clear();
    }

}