#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]
// Java: import cn.wps.moffice.util.PlatformAdapter
use crate::util::PlatformAdapter::*;
use std::collections::HashMap;

// Java field:    private static final String FILE_OBSERVER_CLASS = "cn.wps.moffice.KFileObserver";
const FILE_OBSERVER_CLASS: &str = "cn.wps.moffice.KFileObserver";
// Java field:    public static final int INVALID = -1;
pub const INVALID: i32 = -1;

// Java: public class KEntrySourceLib 
#[derive(Default)]
pub struct KEntrySourceLib {
    // Java field:    private int mId;
    mId: i32,
    // Java field:    private boolean mHaveFound;
    mHaveFound: bool,
    // Java field:    private HashMap<String, Integer> mPath2Id;
    mPath2Id: HashMap<String, i32>,
    // Java field:    private HashMap<Integer, String> mId2Path;
    mId2Path: HashMap<i32, String>,
    // Java field:    private HashMap<String, String> mInfos;
    mInfos: HashMap<String, String>,
}

// Java: public class KEntrySourceLib 
impl KEntrySourceLib {
    // Java: public KEntrySourceLib() 
    pub fn new() -> KEntrySourceLib {
        Self {
            mId: 0,
            mHaveFound: false,
            mPath2Id: HashMap::new(),
            mId2Path: HashMap::new(),
            mInfos: HashMap::new(),
        }
    }

    // Java: public int getId(String path) 
    pub fn getId(&self, path: &str) -> i32 {
        // Java: Integer ret = (Integer)this.mPath2Id.get(path);
        // Java: return null == ret ? -1 : ret;
        self.mPath2Id.get(path).copied().unwrap_or(-1)
    }

    // Java: public String getPath(int id) 
    pub fn getPath(&self, id: i32) -> Option<&String> {
        // Java: return (String)this.mId2Path.get(id);
        self.mId2Path.get(&id)
    }

    // Java: public int addPath(String path, String hostInfo) 
    pub fn addPath(&mut self, path: &str, hostInfo: &str) -> i32 {
        // Java: if (!this.findFileObserverClass())
        // File observer class loading uses Java reflection - stubbed in Rust
        // Java: int ret = this.mId++;
        let ret = self.mId;
        self.mId += 1;
        // Java: this.mId2Path.put(ret, path);
        self.mId2Path.insert(ret, path.to_string());
        // Java: this.mPath2Id.put(path, ret);
        self.mPath2Id.insert(path.to_string(), ret);
        // Java: this.mInfos.put(path, hostInfo);
        self.mInfos.insert(path.to_string(), hostInfo.to_string());
        // Java: return ret;
        ret
    }

    // Java: public void remove(String path) 
    pub fn remove(&mut self, path: &str) {
        // Java: if (null != path)
        if let Some(id) = self.mPath2Id.remove(path) {
            // Java: this.mId2Path.remove(id);
            self.mId2Path.remove(&id);
        }
        self.mInfos.remove(path);
    }

    // Java: public void remove(int id) 
    pub fn remove__int(&mut self, id: i32) {
        // Java: if (-1 != id)
        if id != -1 {
            // Java: String path = (String)this.mId2Path.get(id);
            if let Some(path) = self.mId2Path.remove(&id) {
                // Java: this.remove(path);
                self.mPath2Id.remove(&path);
                self.mInfos.remove(&path);
            }
        }
    }

    // Java: public void resetMaps() 
    pub fn resetMaps(&mut self) {
        // Java: this.mPath2Id = new HashMap();
        self.mPath2Id = HashMap::new();
        // Java: this.mId2Path = new HashMap();
        self.mId2Path = HashMap::new();
        // Java: this.mInfos = new HashMap();
        self.mInfos = HashMap::new();
    }

    // Java: public String toString() 
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        format!("[KEntrySourceLib] {{mId2Path size: {}}}{{mPath2Id size: {}}}",
                self.mId2Path.len(), self.mPath2Id.len())
    }

    // Java: private boolean findFileObserverClass() 
    fn findFileObserverClass(&mut self) -> bool {
        // Java reflection for class loading - not available in Rust, always return true
        true
    }
}
