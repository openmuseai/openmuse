// Java: cn.wps.moffice.writer.layout.base.env.FontStyleCacheFactory
// rust: ★wr-29 04 —— 真实移植（Java 大文件的最小热路径）：create(doc) → 按 docType 缓存
//       FontStyleCache（Java mFontStyleCaches[docType] 语义）；balloon/reuseClean 其余 DEFERRED。
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::{Document, SoftTextDocument};
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleCache
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleCache::FontStyleCache;

// java private static final int MAIN_DOC_CACHE_SIZE = 32;
const MAIN_DOC_CACHE_SIZE: i32 = 32;
// java private static final int OHTER_DOC_CACHE_SIZE = 8;
const OHTER_DOC_CACHE_SIZE: i32 = 8;

// java public final class FontStyleCacheFactory
#[derive(Default)]
pub struct FontStyleCacheFactory {
    // java private FontStyleCache[] mFontStyleCaches = new FontStyleCache[7];
    // rust: Vec<Option<FontStyleCache>>（Java 数组语义）
    pub(crate) mFontStyleCaches: Vec<Option<FontStyleCache>>,
}

impl FontStyleCacheFactory {
    // java public FontStyleCache create(Document doc)
    pub fn create(&mut self, doc: &dyn Document) -> &mut FontStyleCache {
      // java int docType = doc.getType();
      let docType: i32 = doc.getType__on_IDocument();
      // java FontStyleCache[] cacheList = this.mFontStyleCaches;
      // java if (null != cacheList[docType]) { return cacheList[docType]; }
      while self.mFontStyleCaches.len() <= docType as usize {
        self.mFontStyleCaches.push(None);
      }
      if self.mFontStyleCaches[docType as usize].is_none() {
        // java TextDocument textDoc = (TextDocument)doc.getTextDocument();
        let textDoc: Option<&dyn SoftTextDocument> = doc.getTextDocument();
        // java int capacity = 0 == docType ? MAIN_DOC_CACHE_SIZE : OHTER_DOC_CACHE_SIZE;
        let capacity: i32 = if 0 == docType { MAIN_DOC_CACHE_SIZE } else { OHTER_DOC_CACHE_SIZE };
        // java cacheList[docType] = new FontStyleCache(textDoc, capacity);
        let cache = if let Some(td) = textDoc {
          FontStyleCache::new(td, capacity)
        } else {
          FontStyleCache::new(&SoftTextDocumentPlaceholder, capacity)
        };
        self.mFontStyleCaches[docType as usize] = Some(cache);
      }
      // java return cacheList[docType];
      self.mFontStyleCaches[docType as usize].as_mut().unwrap()
    }

    // rust: LangLayout/GraphFactory 经 documentEnv 取缓存（Java 同实例复用）的别名入口
    pub fn create_borrowed(&mut self, doc: Option<*mut dyn Document>) -> &mut FontStyleCache {
        // java return this.mFontStyleCaches[doc.getType()];
        // rust: 管线恒 Some；None = 测试空 document → 主文档槽
        match doc {
            Some(d) => self.create(unsafe { &*d }),
            None => {
                if self.mFontStyleCaches.is_empty() {
                    self.mFontStyleCaches.push(Some(FontStyleCache::new(&SoftTextDocumentPlaceholder, MAIN_DOC_CACHE_SIZE)));
                }
                self.mFontStyleCaches[0].as_mut().unwrap()
            }
        }
    }
}

/// rust: 空 document 占位（SoftTextDocument 面；仅 None 分支使用）
pub struct SoftTextDocumentPlaceholder;
impl crate::cn::wps::moffice::writer::core::Document::SoftTextDocument for SoftTextDocumentPlaceholder {
    fn getTransaction(&self) -> Option<&util::moffice::transaction::Transaction::Transaction> {
        // rust: 空 document 占位 → 无事务
        None
    }
}
