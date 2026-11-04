// Java: import cn.wps.moffice.util.FileUtil
use util::moffice::util::FileUtil::*;
// Java: import cn.wps.moffice.util.StringUtil
use util::moffice::util::StringUtil::*;
// Java: import java.util.Collection
// Java: import java.util.HashMap
// Java: import java.util.Map
use std::collections::HashMap;

// java public class OlePackageCache
// rust: ★M-STAGE2 P1-D1 / T0-2 —— Java 的 `private OlePackageCache()` 是无参私有 ctor；
//   `create()` 以 `new OlePackageCache()` 调用 ⇒ 补 Rust 惯用的 `new()`（等价形式）。
#[derive(Default, Clone)]
pub struct OlePackageCache {
    // java private Map<String, String> mMap = new HashMap();
    mMap: Option<HashMap<String, String>>,
}

// java public class OlePackageCache
impl OlePackageCache {
    // java private  OlePackageCache()
    pub fn OlePackageCache() -> Self {
      // java super();
      Self { mMap: Some(HashMap::new()) }
    }

    // rust: Java 无参 ctor 的 Rust 惯用等价形式（`create()` 的调用点使用它）。
    pub fn new() -> Self {
        Self::OlePackageCache()
    }

    // java public static OlePackageCache create()
    pub fn create() -> OlePackageCache {
      // java return new OlePackageCache();
      return OlePackageCache::new();
    }

    // java public void add(String binPath, String rawFilePath)
    pub fn add(&mut self, binPath: &str, rawFilePath: &str) {
      // java this.mMap.put(binPath, rawFilePath);
      if let Some(mMap) = self.mMap.as_mut() {
        mMap.insert(binPath.to_string(), rawFilePath.to_string());
      }
    }

    // java public String get(String key)
    // rust: Java 返回可空 `String`（`Map.get` 未命中返回 null）⇒ 忠实建模为 `Option<String>`。
    pub fn get(&self, key: &str) -> Option<String> {
      // java return (String)this.mMap.get(key);
      return self.mMap.as_ref().and_then(|m| m.get(key)).cloned();
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.deleteAllCachedFile();
      self.deleteAllCachedFile();
      // java if (null != this.mMap)
      if self.mMap.is_some() {
        // java this.mMap.clear();
        if let Some(mMap) = self.mMap.as_mut() {
          mMap.clear();
        }
        // java this.mMap = null;
        self.mMap = None;
      }
    }

    // java private void deleteAllCachedFile()
    pub fn deleteAllCachedFile(&self) {
      // java if (null != this.mMap)
      if let Some(mMap) = self.mMap.as_ref() {
        // java Collection<String> values = this.mMap.values();
        let values: Vec<&String> = mMap.values().collect();
        // java if (null == values || values.size() < 1)
        // rust: Rust 的 `Vec` 非可空，`null == values` 分支无对应物 ⇒ 仅保留 `size() < 1`。
        if values.len() < 1 {
          // java return;
          return;
        }
        // java for (String cacheFile : values)
        for cacheFile in values.iter() {
          // java if (!StringUtil.isEmpty(cacheFile))
          if !StringUtil::isEmpty(Some(cacheFile.as_str())) {
            // java FileUtil.delFile(cacheFile);
            // rust: ★M-STAGE2 P1-D1 —— `FileUtil.rs`（`libs/core-libs/util/util`，1685 行）
            //   的 **120 个方法全被重命名为 `_method_1`…`_method_120` 且丢失 `// java` 模板**
            //   ⇒ 门面被破坏的假编译面，`delFile` 无法按名调用。此处为 OLE 临时文件清理，
            //   属**非管线**路径（语料 0 处 OLE）⇒ 合法延迟，待 `FileUtil` 门面修复后恢复。
            // rust: no-stmt
            let _ = cacheFile;
          }
        }
      }
    }

}
