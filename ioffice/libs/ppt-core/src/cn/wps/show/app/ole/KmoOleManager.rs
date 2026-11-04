// Rust stub for KmoOleManager (auto-generated)
// Original: cn/wps/show/app/ole/KmoOleManager.java
use crate::*;

pub struct KmoOleManager {
    // TODO: fields
}

impl KmoOleManager {
    pub fn new() -> Self { todo!() }
    pub fn install(&self, manager: Box<dyn IOleManager>) { todo!() }
    pub fn getOleFilePath(&self, mediaId: i32) -> String { todo!() }
    pub fn addOleInfo(&self, mediaId: i32, oleInfo: OleInfo) { todo!() }
    pub fn copyOleInfo(&self, oleMediaId: i32, newMediaId: i32) -> bool { todo!() }
    pub fn setProviderInfo(&self, providerInfo: ProviderInfo) { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn innerCache(&self) -> OlePackageCache { todo!() }
    pub fn dumpOleBinFileIfNeed(&self, mediaId: i32) -> String { todo!() }
    pub fn getOriginFile(&self, mediaId: i32) -> String { todo!() }
}
