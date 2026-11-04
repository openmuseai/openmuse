// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import java.util.WeakHashMap
use stubs::java::util::WeakHashMap::*;
// Java: import cn.wps.moffice.writer.core.property.IPropertyCalculate
use crate::cn::wps::moffice::writer::core::property::IPropertyCalculate::*;
// Java: import cn.wps.moffice.writer.core.property.StyleCache
use crate::cn::wps::moffice::writer::core::property::StyleCache::*;
// Java: import cn.wps.moffice.writer.core.property.PropertyCalculateImpl
use crate::cn::wps::moffice::writer::core::property::PropertyCalculateImpl::*;

// java private static WeakHashMap<Document, IPropertyCalculate> mMap = new WeakHashMap();
static mMap: std::sync::LazyLock<std::sync::Arc<Option<WeakHashMap>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(WeakHashMap::new()));
// java private static WeakHashMap<TextDocument, StyleCache> mStyleCaches = new WeakHashMap();
static mStyleCaches: std::sync::LazyLock<std::sync::Arc<Option<WeakHashMap>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(WeakHashMap::new()));

// java public class PropertyCalculateFactory
#[derive(Default)]
pub struct PropertyCalculateFactory {
}

// java public class PropertyCalculateFactory
impl PropertyCalculateFactory {
    // java public static IPropertyCalculate create(Document document)
    pub fn create(document: Option<&dyn Document>) -> Box<dyn IPropertyCalculate> {
      // java if (document == null)
      if document.is_none() {
        // java throw new NullPointerException("document must not be null");
        panic!("{}", NullPointerException::new("document must not be null"));
      }
      // java else
      else {
        // java IPropertyCalculate p = (IPropertyCalculate)mMap.get(document);
        let mut p: Box<dyn IPropertyCalculate> = todo!("cast → Box<dyn IPropertyCalculate>");
        // java if (p == null)
        if p.is_none() {
          // java p = new PropertyCalculateImpl(document, getStyleCache(document.getTextDocument()));
          p = PropertyCalculateImpl::new(document, Self::getStyleCache(&document.getTextDocument()));
          // java mMap.put(document, p);
          mMap.put(document, p);
        }
        // java return p;
        return p;
      }
    }

    // java public static void updateStyleCache(TextDocument document)
    pub fn updateStyleCache(document: &TextDocument) {
      // java StyleCache p = (StyleCache)mStyleCaches.get(document);
      let mut p: StyleCache = (mStyleCaches.get(document)).downcast_ref::<StyleCache>().unwrap();
      // java if (p == null)
      if p.is_none() {
        // java p = new StyleCache(document);
        p = StyleCache::new(document);
        // java mStyleCaches.put(document, p);
        mStyleCaches.put(document, p);
      }
      // java else
      else {
        // java p.reset(document);
        p.reset(document);
      }
    }

    // java private static StyleCache getStyleCache(TextDocument document)
    pub fn getStyleCache(document: &TextDocument) -> StyleCache {
      // java StyleCache p = (StyleCache)mStyleCaches.get(document);
      let mut p: StyleCache = (mStyleCaches.get(document)).downcast_ref::<StyleCache>().unwrap();
      // java if (p == null)
      if p.is_none() {
        // java p = new StyleCache(document);
        p = StyleCache::new(document);
        // java mStyleCaches.put(document, p);
        mStyleCaches.put(document, p);
      }
      // java return p;
      return p;
    }

}
