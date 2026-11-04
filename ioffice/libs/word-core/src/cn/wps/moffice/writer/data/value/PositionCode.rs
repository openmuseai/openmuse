#![allow(non_snake_case)]

// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
use std::collections::HashMap;
use std::sync::Mutex;

// Java: private static WIntObjectHashMap<PositionCode> sPool
static sPool: std::sync::LazyLock<Mutex<HashMap<i32, PositionCode>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

// Java: public static final int pcVertRelativeToPageMargin = 0
pub const pcVertRelativeToPageMargin: i32 = 0;
// Java: public static final int pcVertRelativeToPage = 1
pub const pcVertRelativeToPage: i32 = 1;
// Java: public static final int pcVertRelativeToParagraph = 2
pub const pcVertRelativeToParagraph: i32 = 2;
// Java: public static final int pcVertNone = 3
pub const pcVertNone: i32 = 3;
// Java: public static final int pcHorzRelativeToColumn = 0
pub const pcHorzRelativeToColumn: i32 = 0;
// Java: public static final int pcHorzRelativeToPageMargin = 1
pub const pcHorzRelativeToPageMargin: i32 = 1;
// Java: public static final int pcHorzRelativeToPage = 2
pub const pcHorzRelativeToPage: i32 = 2;
// Java: public static final int pcHorzNone = 3
pub const pcHorzNone: i32 = 3;

// Java: public class PositionCode
#[derive(Clone)]
pub struct PositionCode {
    // Java field: byte pcVert = 3
    pcVert: i8,
    // Java field: byte pcHorz = 3
    pcHorz: i8,
}

impl Default for PositionCode {
    fn default() -> Self {
        // Java: public PositionCode()
        Self { pcVert: 3, pcHorz: 3 }
    }
}

impl PositionCode {
    // Java: public PositionCode(int vertPos, int horzPos)
    pub fn new(vert_pos: i32, horz_pos: i32) -> Self {
        Self {
            pcVert: vert_pos as i8,
            pcHorz: horz_pos as i8,
        }
    }

    // Java: public byte getVertPos()
    pub fn getVertPos(&self) -> i32 {
        self.pcVert as i32
    }

    // Java: public byte getHorzPos()
    pub fn getHorzPos(&self) -> i32 {
        self.pcHorz as i32
    }

    // Java: public boolean isNotWrap()
    pub fn isNotWrap(&self) -> bool {
        // Java: (pcHorz == 0 || pcHorz == 3) && (pcVert == 0 || pcVert == 3)
        (self.pcHorz == 0 || self.pcHorz == 3) && (self.pcVert == 0 || self.pcVert == 3)
    }

    // Java: public void setVertPos(int vertPos)
    pub fn setVertPos(&mut self, vert_pos: i32) {
        self.pcVert = vert_pos as i8;
    }

    // Java: public void setHorsPos(int horzPos)
    pub fn setHorsPos(&mut self, horz_pos: i32) {
        self.pcHorz = horz_pos as i8;
    }

    // Java: public static synchronized PositionCode obtain(int vertPos, int horzPos)
    pub fn obtain(vert_pos: i32, horz_pos: i32) -> PositionCode {
        let key = (vert_pos << 16) | horz_pos;
        let mut pool = sPool.lock().unwrap();
        if let Some(cached) = pool.get(&key) {
            return cached.clone();
        }
        let cached = PositionCode::new(vert_pos, horz_pos);
        pool.insert(key, cached.clone());
        cached
    }

    // Java: public static synchronized void clear()
    pub fn clear() {
        sPool.lock().unwrap().clear();
    }
}
