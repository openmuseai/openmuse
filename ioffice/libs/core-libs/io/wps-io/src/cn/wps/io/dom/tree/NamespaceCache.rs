// Java: import cn.wps.io.dom.Namespace
// use crate::io::dom::Namespace;
// Java: import java.lang.ref.WeakReference
// Java: import java.lang.reflect.Constructor
// Java: import java.util.Map
use std::collections::HashMap;
use std::sync::Mutex;

// Java field: String CONCURRENTREADERHASHMAP_CLASS
pub(crate) const CONCURRENTREADERHASHMAP_CLASS: &str = "EDU.oswego.cs.dl.util.concurrent.ConcurrentReaderHashMap";
// Java field: Map cache
static CACHE: std::sync::OnceLock<Mutex<HashMap<String, HashMap<String, String>>>> = std::sync::OnceLock::new();
// Java field: Map noPrefixCache
static NO_PREFIX_CACHE: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();

// Java: public class NamespaceCache
pub struct NamespaceCache {
}

// Java: public class NamespaceCache
impl NamespaceCache {
    // Java: NamespaceCache()
    pub fn new() -> Self {
      // Java: super();
      NamespaceCache {}
    }

    // Java: Namespace get(String prefix, String uri)
    pub fn get(&self, prefix: &str, uri: &str) -> NamespaceCacheEntry {
      // Java: Map uriCache = this.getURICache(uri);
      let mut uri_cache = self.getURICache(uri);
      // Java: WeakReference ref = (WeakReference)uriCache.get(prefix);
      let cached = uri_cache.get(prefix).cloned();
      // Java: Namespace answer = null;
      let mut answer: Option<NamespaceCacheEntry> = None;
      // Java: if (ref != null)
      if let Some(_val) = &cached
      {
         // Java: answer = (Namespace)ref.get();
         answer = Some(NamespaceCacheEntry { prefix: prefix.to_string(), uri: uri.to_string() });
      }
      // Java: if (answer == null)
      if answer.is_none()
      {
         // Java: synchronized(uriCache)
         // Rust: no synchronized needed, using Mutex
         {
            // Java: ref = (WeakReference)uriCache.get(prefix);
            let cached2 = uri_cache.get(prefix).cloned();
            // Java: if (ref != null)
            if let Some(_val) = &cached2
            {
               // Java: answer = (Namespace)ref.get();
               answer = Some(NamespaceCacheEntry { prefix: prefix.to_string(), uri: uri.to_string() });
            }
            // Java: if (answer == null)
            if answer.is_none()
            {
               // Java: answer = this.createNamespace(prefix, uri);
               answer = Some(self.createNamespace(prefix, uri));
               // Java: uriCache.put(prefix, new WeakReference(answer));
               uri_cache.insert(prefix.to_string(), uri.to_string());
            }
         }
      }
      // Java: return answer;
      answer.unwrap()
    }

    // Java: Namespace get(String uri)
    pub fn get_by_uri(&self, uri: &str) -> NamespaceCacheEntry {
      // Java: WeakReference ref = (WeakReference)noPrefixCache.get(uri);
      let no_prefix_cache = NO_PREFIX_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
      let cache_guard = no_prefix_cache.lock().unwrap();
      let cached = cache_guard.get(uri).cloned();
      drop(cache_guard);
      // Java: Namespace answer = null;
      let mut answer: Option<NamespaceCacheEntry> = None;
      // Java: if (ref != null)
      if let Some(_val) = &cached
      {
         // Java: answer = (Namespace)ref.get();
         answer = Some(NamespaceCacheEntry { prefix: String::new(), uri: uri.to_string() });
      }
      // Java: if (answer == null)
      if answer.is_none()
      {
         // Java: synchronized(noPrefixCache)
         let mut cache_guard = no_prefix_cache.lock().unwrap();
         {
            // Java: ref = (WeakReference)noPrefixCache.get(uri);
            let cached2 = cache_guard.get(uri).cloned();
            // Java: if (ref != null)
            if let Some(_val) = &cached2
            {
               // Java: answer = (Namespace)ref.get();
               answer = Some(NamespaceCacheEntry { prefix: String::new(), uri: uri.to_string() });
            }
            // Java: if (answer == null)
            if answer.is_none()
            {
               // Java: answer = this.createNamespace("", uri);
               answer = Some(self.createNamespace("", uri));
               // Java: noPrefixCache.put(uri, new WeakReference(answer));
               cache_guard.insert(uri.to_string(), uri.to_string());
            }
         }
      }
      // Java: return answer;
      answer.unwrap()
    }

    // Java: Map getURICache(String uri)
    fn getURICache(&self, uri: &str) -> HashMap<String, String> {
      // Java: Map answer = (Map)cache.get(uri);
      let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
      let cache_guard = cache.lock().unwrap();
      let mut answer = cache_guard.get(uri).cloned();
      drop(cache_guard);
      // Java: if (answer == null)
      if answer.is_none()
      {
         // Java: synchronized(cache)
         let mut cache_guard = cache.lock().unwrap();
         {
            // Java: answer = (Map)cache.get(uri);
            answer = cache_guard.get(uri).cloned();
            // Java: if (answer == null)
            if answer.is_none()
            {
               // Java: answer = new ConcurrentReaderHashMap();
               let new_map = HashMap::new();
               // Java: cache.put(uri, answer);
               cache_guard.insert(uri.to_string(), new_map.clone());
               answer = Some(new_map);
            }
         }
      }
      // Java: return answer;
      answer.unwrap()
    }

    // Java: Namespace createNamespace(String prefix, String uri)
    pub fn createNamespace(&self, prefix: &str, uri: &str) -> NamespaceCacheEntry {
      // Java: return new Namespace(prefix, uri);
      NamespaceCacheEntry { prefix: prefix.to_string(), uri: uri.to_string() }
    }

}

pub struct NamespaceCacheEntry {
    pub prefix: String,
    pub uri: String,
}
