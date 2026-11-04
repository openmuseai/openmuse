// Java: import java.lang.ref.WeakReference
// Java: import java.lang.reflect.Constructor
// Java: import java.util.Map
// Java: import org.dom4j.Namespace
use crate::org::dom4j::Namespace::Namespace;

use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use std::sync::LazyLock;

// Java field: String CONCURRENTREADERHASHMAP_CLASS
pub(crate) const CONCURRENTREADERHASHMAP_CLASS: &str = "EDU.oswego.cs.dl.util.concurrent.ConcurrentReaderHashMap";

// Java field: Map cache
static CACHE: LazyLock<RwLock<HashMap<String, HashMap<String, Namespace>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// Java field: Map noPrefixCache
static NO_PREFIX_CACHE: LazyLock<RwLock<HashMap<String, Namespace>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

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
    pub fn get_with_prefix(&self, prefix: &str, uri: &str) -> Namespace {
        // Java: Map uriCache = this.getURICache(uri);
        let uri_cache = self.getURICache(uri);
        // Java: WeakReference ref = (WeakReference)uriCache.get(prefix);
        // Java: Namespace answer = null;
        let cached = uri_cache.get(prefix).cloned();
        // Java: if (ref != null)
        {
            // Java: answer = (Namespace)ref.get();
        }
        // Java: if (answer == null)
        if cached.is_none()
        {
            // Java: synchronized(uriCache)
            {
                // Java: ref = (WeakReference)uriCache.get(prefix);
                // Java: if (ref != null)
                {
                    // Java: answer = (Namespace)ref.get();
                }
                // Java: if (answer == null)
                {
                    // Java: answer = this.createNamespace(prefix, uri);
                    let answer = self.createNamespace(prefix, uri);
                    // Java: uriCache.put(prefix, new WeakReference(answer));
                    let mut cache = CACHE.write().unwrap();
                    cache.entry(uri.to_string())
                        .or_insert_with(HashMap::new)
                        .insert(prefix.to_string(), answer.clone());
                    return answer;
                }
            }
        }
        // Java: return answer;
        cached.unwrap()
    }

    // Java: Namespace get(String uri)
    pub fn get_by_uri(&self, uri: &str) -> Namespace {
        // Java: WeakReference ref = (WeakReference)noPrefixCache.get(uri);
        // Java: Namespace answer = null;
        let cached = NO_PREFIX_CACHE.read().unwrap().get(uri).cloned();
        // Java: if (ref != null)
        {
            // Java: answer = (Namespace)ref.get();
        }
        // Java: if (answer == null)
        if cached.is_none()
        {
            // Java: synchronized(noPrefixCache)
            {
                // Java: ref = (WeakReference)noPrefixCache.get(uri);
                // Java: if (ref != null)
                {
                    // Java: answer = (Namespace)ref.get();
                }
                // Java: if (answer == null)
                {
                    // Java: answer = this.createNamespace("", uri);
                    let answer = self.createNamespace("", uri);
                    // Java: noPrefixCache.put(uri, new WeakReference(answer));
                    NO_PREFIX_CACHE.write().unwrap().insert(uri.to_string(), answer.clone());
                    return answer;
                }
            }
        }
        // Java: return answer;
        cached.unwrap()
    }

    // Java: Map getURICache(String uri)
    pub fn getURICache(&self, uri: &str) -> HashMap<String, Namespace> {
        // Java: Map answer = (Map)cache.get(uri);
        let answer = CACHE.read().unwrap().get(uri).cloned();
        // Java: if (answer == null)
        if answer.is_none()
        {
            // Java: synchronized(cache)
            {
                // Java: answer = (Map)cache.get(uri);
                // Java: if (answer == null)
                {
                    // Java: answer = new ConcurrentReaderHashMap();
                    let new_map = HashMap::new();
                    // Java: cache.put(uri, answer);
                    CACHE.write().unwrap().insert(uri.to_string(), new_map.clone());
                    return new_map;
                }
            }
        }
        // Java: return answer;
        answer.unwrap()
    }

    // Java: Namespace createNamespace(String prefix, String uri)
    pub fn createNamespace(&self, _prefix: &str, _uri: &str) -> Namespace {
        // Java: return new Namespace(prefix, uri);
        Namespace::new(_prefix, _uri)
    }
}
