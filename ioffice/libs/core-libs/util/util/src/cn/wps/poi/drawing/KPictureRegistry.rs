#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.util.FileDataStorage
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use crate::moffice::util::FileDataStorage::FileDataStorage;

// Java field: String KEY_CORE_PICTURE
pub const KEY_CORE_PICTURE: &str = "cn.wps.poi.drawing.KPictureRegistry.KEY_CORE_PICTURE";
// Java field: String KEY_RAW_DATA
pub const KEY_RAW_DATA: &str = "cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_DATA";
// Java field: String KEY_RAW_COMPRESSED
pub const KEY_RAW_COMPRESSED: &str = "cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_COMPRESSED";
// Java field: String KEY_RAW_DATA_MD4
pub const KEY_RAW_DATA_MD4: &str = "cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_DATA_MD4";
// Java field: String KEY_COMPRESSION_METHOD
pub const KEY_COMPRESSION_METHOD: &str = "cn.wps.poi.drawing.KPictureRegistry.KEY_COMPRESSION_METHOD";
// Java field: String KEY_CALCULATION_DATE
pub const KEY_CALCULATION_DATE: &str = "cn.wps.poi.drawing.KPictureRegistry.KEY_CALCULATION_DATE";
// Java field: String KEY_CORE_PICTURE_COMPRESSED
pub const KEY_CORE_PICTURE_COMPRESSED: &str = "cn.wps.poi.drawing.KPictureRegistry.KEY_CORE_PICTURE_COMPRESSED";
// Java field: String KEY_HTTP_URL
pub const KEY_HTTP_URL: &str = "KEY_HTTP_URL";
// Java field: String KEY_IS_EXTERNAL
pub const KEY_IS_EXTERNAL: &str = "KEY_IS_EXTERNAL";
// Java field: String KEY_IS_DOWNLOADED
pub const KEY_IS_DOWNLOADED: &str = "KEY_IS_DOWNLOADED";
// Java field: String KEY_WEB_URL
pub const KEY_WEB_URL: &str = "KEY_WEB_URL";

// Java: public class KPictureRegistry
pub struct KPictureRegistry {
    // Java field: HashMap<String, FileDataStorage> mPictures
    mPictures: HashMap<String, FileDataStorage>,
}

// Java field: KPictureRegistry _instance
static INSTANCE: OnceLock<Mutex<KPictureRegistry>> = OnceLock::new();

// Java: public class KPictureRegistry
impl KPictureRegistry {
    // Java: KPictureRegistry()
    pub fn new() -> Self {
        // Java: super();
        Self {
            mPictures: HashMap::new(),
        }
    }

    // Java: FileDataStorage register(String path)
    pub fn register(&mut self, path: &str) -> &FileDataStorage {
        // Java: FileDataStorage pData = (FileDataStorage)this.mPictures.get(path);
        // Java: if (pData == null)
        if !self.mPictures.contains_key(path)
        {
            // Java: pData = new FileDataStorage(new File(path));
            let p_data = FileDataStorage::new(PathBuf::from(path));
            self.mPictures.insert(path.to_string(), p_data);
        }
        // Java: return pData;
        self.mPictures.get(path).unwrap()
    }

    // Java: void register(FileDataStorage pData)
    pub fn register_storage(&mut self, p_data: FileDataStorage) {
        // Java: this.mPictures.put(pData.getFile().getAbsolutePath(), pData);
        let path = p_data.mFile.as_ref().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
        self.mPictures.insert(path, p_data);
    }

    // Java: FileDataStorage get(String path)
    pub fn get(&self, path: &str) -> Option<&FileDataStorage> {
        // Java: return (FileDataStorage)this.mPictures.get(path);
        self.mPictures.get(path)
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: for(FileDataStorage storage : this.mPictures.values())
        for (_key, storage) in self.mPictures.iter_mut()
        {
            // Java: storage.disposeUserData();
            storage.disposeUserData();
        }
        // Java: this.mPictures.clear();
        self.mPictures.clear();
    }

    // Java: static KPictureRegistry instance()
    pub fn instance() -> &'static Mutex<KPictureRegistry> {
        // Java: return _instance;
        INSTANCE.get_or_init(|| Mutex::new(KPictureRegistry::new()))
    }
}
