#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.kfc.annotation.PackageVisible
// LLM: PackageVisible is a Java annotation, no Rust equivalent
// Java: import cn.wps.kfc.util.KcBitsOp
// LLM: KcBitsOp is a bit field utility, inlined below
// Java: import java.util.ArrayList
// LLM: using Vec
// Java: import java.util.List
use std::vec::Vec;


pub struct KcBitsOp {
    mask: i32,
}

impl KcBitsOp {
    pub fn new(mask: i32) -> Self { KcBitsOp { mask } }

    pub fn getValue(&self, value: i32) -> i32 {
        // Java: return this.getRawValue(holder) >>> this._shift_count;
        let shift = self.mask.trailing_zeros();
        ((value as u32 & self.mask as u32) >> shift) as i32
    }

    pub fn setValue(&self, value: i32, field_val: i32) -> i32 {
        let shift = self.mask.trailing_zeros();
        (value & !self.mask) | ((field_val << shift) & self.mask)
    }
}

pub struct ShapeBin {
    pub(crate) bh: i32,
}

static SPT: KcBitsOp = KcBitsOp { mask: -1048576 };
static SPID: KcBitsOp = KcBitsOp { mask: 1048575 };

// Java: public class DgStoreData
// Java: public class DgStoreData
pub struct DgStoreData {
    // Java field: List<ShapeBin> groupStore
    pub(crate) groupStore: Vec<ShapeBin>,
    // Java field: List<ShapeBin> shapeStore
    pub(crate) shapeStore: Vec<ShapeBin>,
    // Java field: List<ShapeBin> picStore
    pub(crate) picStore: Vec<ShapeBin>,
    // Java field: List<ShapeBin> cxnStore
    pub(crate) cxnStore: Vec<ShapeBin>,
    // Java field: List<ShapeBin> graphicStore
    pub(crate) graphicStore: Vec<ShapeBin>,
    // Java field: List<ShapeBin> textStore
    pub(crate) textStore: Vec<ShapeBin>,
}

// Java: public class DgStoreData
impl DgStoreData {
    pub fn DgStoreData_new() -> Self {
        DgStoreData {
            groupStore: Vec::new(), shapeStore: Vec::new(),
            picStore: Vec::new(), cxnStore: Vec::new(),
            graphicStore: Vec::new(), textStore: Vec::new(),
        }
    }

    // Java: static int queryType(int spid)
    pub fn queryType(spid: i32) -> i32 {
        // Java: return $spt.getValue(spid);
        SPT.getValue(spid)
    }

    // Java: static boolean isGroup(int spid)
    pub fn isGroup(spid: i32) -> bool {
        // Java: return $spid.getValue(spid) == 0;
        SPID.getValue(spid) == 0
    }

    // Java: static boolean isShape(int spid)
    pub fn isShape(spid: i32) -> bool {
        // Java: return $spid.getValue(spid) == 1;
        SPID.getValue(spid) == 1
    }

    // Java: static boolean isPicture(int spid)
    pub fn isPicture(spid: i32) -> bool {
        // Java: return $spid.getValue(spid) == 2;
        SPID.getValue(spid) == 2
    }

    // Java: static boolean isConnector(int spid)
    pub fn isConnector(spid: i32) -> bool {
        // Java: return $spid.getValue(spid) == 3;
        SPID.getValue(spid) == 3
    }

    // Java: static boolean isGraphicFrame(int spid)
    pub fn isGraphicFrame(spid: i32) -> bool {
        // Java: return $spid.getValue(spid) == 4;
        SPID.getValue(spid) == 4
    }

    // Java: static boolean isTextShape(int spid)
    pub fn isTextShape(spid: i32) -> bool {
        // Java: return $spid.getValue(spid) == 5;
        SPID.getValue(spid) == 5
    }

    // Java: int helper_storeIndex2Id(int shapeType, int idInList)
    pub fn helper_storeIndex2Id(&self, shapeType: i32, idInList: i32) -> i32 {
        // Java: return $spt.setValue(0, shapeType) | $spid.setValue(0, idInList);
        SPT.setValue(0, shapeType) | SPID.setValue(0, idInList)
    }

    // Java: int helper_getIndexById(int id)
    pub fn helper_getIndexById(&self, id: i32) -> i32 {
        // Java: return $spid.getValue(id);
        SPID.getValue(id)
    }

    // Java: List<ShapeBin> helper_storeByType(int spt)
    pub fn helper_storeByType(&self, spt: i32) -> &Vec<ShapeBin> {
        // Java: switch (spt)
        match spt {
            // Java: case 0:
            0 => {
                // Java: return this.groupStore;
                &self.groupStore
            }
            // Java: case 1:
            1 => {
                // Java: return this.shapeStore;
                &self.shapeStore
            }
            // Java: case 2:
            2 => {
                // Java: return this.picStore;
                &self.picStore
            }
            // Java: case 3:
            3 => {
                // Java: return this.cxnStore;
                &self.cxnStore
            }
            // Java: case 4:
            4 => {
                // Java: return this.graphicStore;
                &self.graphicStore
            }
            // Java: case 5:
            5 => {
                // Java: return this.textStore;
                &self.textStore
            }
            // Java: default:
            _ => {
                // Java: throw new IllegalArgumentException();
                panic!("IllegalArgumentException")
            }
        }
    }

    pub fn helper_storeByType_mut(&mut self, spt: i32) -> &mut Vec<ShapeBin> {
        match spt {
            0 => &mut self.groupStore,
            1 => &mut self.shapeStore,
            2 => &mut self.picStore,
            3 => &mut self.cxnStore,
            4 => &mut self.graphicStore,
            5 => &mut self.textStore,
            _ => panic!("IllegalArgumentException"),
        }
    }

    // Java: List<ShapeBin> helper_storeById(int spid)
    pub fn helper_storeById(&self, spid: i32) -> &Vec<ShapeBin> {
        // Java: return this.helper_storeByType(DgStore.queryType(spid));
        self.helper_storeByType(DgStoreData::queryType(spid))
    }

    pub fn helper_storeById_mut(&mut self, spid: i32) -> &mut Vec<ShapeBin> {
        self.helper_storeByType_mut(DgStoreData::queryType(spid))
    }

    // Java: boolean helper_isGood(int spid)
    pub fn helper_isGood(&self, spid: i32) -> bool {
        // Java: List<ShapeBin> store = this.helper_storeById(spid);
        let store = self.helper_storeById(spid);
        // Java: int index = this.helper_getIndexById(spid);
        let index = self.helper_getIndexById(spid);
        // Java: return index < store.size() && index >= 0;
        (index as usize) < store.len() && index >= 0
    }
}
