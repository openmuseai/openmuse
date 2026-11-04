use super::KerningPairCache::KerningPairCache;
use super::DefaultKerningCache::DefaultKerningCache;

// Java: public class KerningCacheFactory
pub struct KerningCacheFactory {
}

// Java: public class KerningCacheFactory
impl KerningCacheFactory {
    // Java: KerningCacheFactory()
    pub fn new() -> Self {
        // Java: super();
        KerningCacheFactory {}
    }

    // Java: static KerningPairCache createCache(String fontFamilyName)
    pub fn createCache(_font_family_name: &str) -> DefaultKerningCache {
        // Java: KerningPairCache cache = new DefaultKerningCache();
        let cache = DefaultKerningCache::new();
        // Java: return cache;
        cache
    }
}
