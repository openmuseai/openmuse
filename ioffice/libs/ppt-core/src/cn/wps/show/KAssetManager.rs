#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import android.content.Context
// Java: import cn.wps.core.runtime.IAssetManager
use util_adapter::cn::wps::core::runtime::IAssetManager::IAssetManager;
// Java: import java.io.IOException
// Java: import java.io.InputStream
use std::io::Read;

struct AssetManager;
impl AssetManager {
    fn open(&self, name: &str) -> Option<Box<dyn Read>> {
        None
    }
    fn list(&self, directoryName: &str) -> Vec<String> {
        Vec::new()
    }
}

pub struct Context;
impl Context {
    fn getAssets(&self) -> AssetManager {
        AssetManager
    }
}

// Java: public class KAssetManager implements IAssetManager
pub struct KAssetManager {
    // Java field: Context mContext
    pub(crate) mContext: Context,
}

// Java: public class KAssetManager implements IAssetManager
impl KAssetManager {
    // Java: KAssetManager(Context context)
    pub fn new(context: Context) -> Self {
      // Java: this.mContext = context;
      KAssetManager { mContext: context }
    }

    // Java: InputStream open(String name)
    pub fn open(&self, name: &str) -> Result<Option<Box<dyn Read>>, std::io::Error> {
      // Java: return this.mContext.getAssets().open(name);
      Ok(self.mContext.getAssets().open(name))
    }

    // Java: String[] list(String directoryName)
    pub fn list(&self, directoryName: &str) -> Result<Vec<String>, std::io::Error> {
      // Java: return this.mContext.getAssets().list(directoryName);
      Ok(self.mContext.getAssets().list(directoryName))
    }

}

impl IAssetManager for KAssetManager {
    // Java: abstract InputStream open(String var1)
    fn open(&self, var1: &str) -> Option<Box<dyn Read>> {
        KAssetManager::open(self, var1).unwrap_or(None)
    }

    // Java: abstract String[] list(String var1)
    fn list(&self, var1: &str) -> Vec<String> {
        KAssetManager::list(self, var1).unwrap_or_default()
    }
}
