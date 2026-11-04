#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]
// Java: import cn.wps.moffice.kfs.raw.KEntrySourceLib
use crate::kfs::raw::KEntrySourceLib::*;
// Java: import cn.wps.moffice.kfs.raw.KRawHostBase
use crate::kfs::raw::KRawHostBase::*;

// Java: public class KRawDataManager 
#[derive(Default)]
pub struct KRawDataManager {
    // Java field:    private KEntrySourceLib mEntrySourceLib = new KEntrySourceLib();
    mEntrySourceLib: KEntrySourceLib,
    // Java field:    private HashSet<KRawHostBase> mHosts;
    mHosts: Option<Vec<KRawHostBase>>,
}

// Java: public class KRawDataManager 
impl KRawDataManager {
    // Java: public KRawDataManager() 
    pub fn new() -> KRawDataManager {
        Self { mEntrySourceLib: KEntrySourceLib::new(), mHosts: None }
    }

    // Java: public void resetRawData(String path, String info) 
    pub fn resetRawData(&mut self, path: String, info: String) {
        // Java: if (null != this.mHosts)
        if let Some(hosts) = self.mHosts.take() {
            // Java: synchronized(this.mHosts)
            // Java: int entrySourceId = this.mEntrySourceLib.addPath(path, info);
            let entrySourceId: i32 = self.mEntrySourceLib.addPath(&path, &info);
            // Java: if (-1 != entrySourceId)
            if -1 != entrySourceId {
                // Java: for(KRawHostBase host : this.mHosts)
                for mut host in hosts {
                    // Java: host.resetRawData(entrySourceId);
                    host.resetRawData(entrySourceId);
                }
                // Java: this.mHosts = null;
            } else {
                self.mHosts = Some(hosts);
            }
        }
    }

    // Java: public void reset() 
    pub fn reset(&mut self) {
        // Java: this.mEntrySourceLib.resetMaps();
        self.mEntrySourceLib.resetMaps();
        // Java: if (null != this.mHosts)
        // Java: this.mHosts = null;
        self.mHosts = None;
    }

    // Java: public KEntrySourceLib getEntrySourceLib() 
    pub fn getEntrySourceLib(&mut self) -> &mut KEntrySourceLib {
        // Java: return this.mEntrySourceLib;
        &mut self.mEntrySourceLib
    }

    // Java: public void setRawDataHosts(HashSet<KRawHostBase> hosts) 
    pub fn setRawDataHosts(&mut self, hosts: Vec<KRawHostBase>) {
        // Java: this.mHosts = hosts;
        self.mHosts = Some(hosts);
    }
}
