#![allow(non_snake_case, non_upper_case_globals)]

// Java: import java.util.HashMap
// Java: import java.util.Map
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use super::DumpInfo::DumpInfo;
use super::PictureLoader::BlipHandlerRef;

// Java: protected static Map<Integer, DumpInfo> mDumpMap = new HashMap();
pub(crate) static mDumpMap: LazyLock<Mutex<HashMap<i32, DumpInfo>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// Java: public class PictureDumpMap
pub struct PictureDumpMap;

impl PictureDumpMap {
    // Java: public static void reset()
    pub fn reset() {
        // Java: mDumpMap.clear();
        mDumpMap.lock().expect("picture dump map lock").clear();
    }

    // Java: public static void add(int shapeID, int type, PictureLoader.IDumpBlipHandler handler)
    pub fn add(shapeID: i32, r#type: i32, handler: Option<BlipHandlerRef>) {
        let mut map = mDumpMap.lock().expect("picture dump map lock");
        // Java: DumpInfo info = (DumpInfo)mDumpMap.get(shapeID);
        // Java: if (info == null) { info = new DumpInfo(); mDumpMap.put(shapeID, info); }
        let info = map.entry(shapeID).or_insert_with(DumpInfo::new);
        // Java: switch (type)
        match r#type {
            // Java: case 1: info.picHandler = handler;
            DumpInfo::PIC => info.picHandler = handler,
            // Java: case 2: info.fillHandler = handler;
            DumpInfo::FILL => info.fillHandler = handler,
            // Java: case 3: info.lineHandler = handler;
            DumpInfo::LINE => info.lineHandler = handler,
            _ => {}
        }
    }

    // Java: public static PictureLoader.IDumpBlipHandler get(int shapeID, int type)
    pub fn get(shapeID: i32, r#type: i32) -> Option<BlipHandlerRef> {
        // Java: DumpInfo info = (DumpInfo)mDumpMap.get(shapeID);
        let map = mDumpMap.lock().expect("picture dump map lock");
        // Java: return DumpInfo.getHandler(info, type);
        DumpInfo::getHandler(map.get(&shapeID), r#type)
    }

    // Java: public static boolean isFinshed()
    pub fn isFinshed() -> bool {
        // Java: return mDumpMap.size() == 0;
        mDumpMap.lock().expect("picture dump map lock").is_empty()
    }
}
