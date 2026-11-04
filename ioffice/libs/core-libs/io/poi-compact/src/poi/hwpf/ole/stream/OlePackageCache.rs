// Java: import cn.wps.moffice.util.FileUtil
use util::cn::wps::moffice::util::FileUtil;
// Java: import cn.wps.moffice.util.StringUtil
use util::org::apache::poi::util::StringUtil;
// Java: import java.util.Collection
// TODO_RUST_CODE use ???; // java.util.Collection
// Java: import java.util.HashMap
// TODO_RUST_CODE use ???; // java.util.HashMap
// Java: import java.util.Map
// TODO_RUST_CODE use ???; // java.util.Map

// Java: public class OlePackageCache
pub struct OlePackageCache {
    // Java field: Map<String, String> mMap
    mMap: Option<HashMap<String, String>>,
}

// Java: public class OlePackageCache
impl OlePackageCache {
    // Java: OlePackageCache()
    pub fn OlePackageCache(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: static OlePackageCache create()
    pub fn create(&self) -> OlePackageCache {
      // Java: return new OlePackageCache();
      // TODO_RUST_CODE
    }

    // Java: void add(String binPath, String rawFilePath)
    pub fn add(&self, String binPath, String rawFilePath) {
      // Java: this.mMap.put(binPath, rawFilePath);
      // TODO_RUST_CODE
    }

    // Java: String get(String key)
    pub fn get(&self, String key) -> String {
      // Java: return (String)this.mMap.get(key);
      // TODO_RUST_CODE
    }

    // Java: void dispose()
    pub fn dispose(&self) {
      // Java: this.deleteAllCachedFile();
      // TODO_RUST_CODE
      // Java: if (null != this.mMap)
        // TODO_RUST_CODE
      {
         // Java: this.mMap.clear();
         // TODO_RUST_CODE
         // Java: this.mMap = null;
         // TODO_RUST_CODE
      }
    }

    // Java: void deleteAllCachedFile()
    pub fn deleteAllCachedFile(&self) {
      // Java: if (null != this.mMap)
        // TODO_RUST_CODE
      {
         // Java: Collection<String> values = this.mMap.values();
         // TODO_RUST_CODE
         // Java: if (null == values || values.size() < 1)
            // TODO_RUST_CODE
         {
            // Java: return;
            // TODO_RUST_CODE
         }
         // Java: for(String cacheFile : values)
            // TODO_RUST_CODE
         {
            // Java: if (!StringUtil.isEmpty(cacheFile))
                // TODO_RUST_CODE
            {
               // Java: FileUtil.delFile(cacheFile);
               // TODO_RUST_CODE
            }
         }
      }
    }

}
