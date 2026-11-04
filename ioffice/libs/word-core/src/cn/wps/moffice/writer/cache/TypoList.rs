// Java: cn.wps.moffice.writer.cache.TypoList
// rust: ★wr-29 05 —— TypoList 静态访问器真实化（size/get/quickGet/last/first/alloc/insert）：
//       行指针数组布局 [pList+0..2]=header、[pList+2]=size、[pList+3..]=元素。
#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;

#[derive(Default, Clone)]
pub struct TypoList {}

impl TypoList {
    // java static final int OFFSET_START = 2;
    // java static final int OFFSET_SIZE = 2;
    // java static final int OFFSET_MAX = 3;

    // java public static int first(int pList, TypoSnapshot snapshot)
    pub fn first__int_typosnapshot(pList: i32, snapshot: &TypoSnapshot) -> i32 {
        // java return size(pList, snapshot) <= 0 ? 0 : snapshot.get(pList + 3);
        if Self::size__int_typosnapshot(pList, snapshot) <= 0 { 0 } else { snapshot.get__int(pList + 3) }
    }

    // java public static int size(int pList, TypoSnapshot snapshot)
    pub fn size__int_typosnapshot(pList: i32, snapshot: &TypoSnapshot) -> i32 {
        // java return pList == 0 ? 0 : snapshot.get(pList + 2);
        if pList == 0 { 0 } else { snapshot.get__int(pList + 2) }
    }

    // java public static int get(int index, int pList, TypoSnapshot snapshot)
    pub fn get__int_int_typosnapshot(index: i32, pList: i32, snapshot: &TypoSnapshot) -> i32 {
        // java return index >= 0 && index < size(pList, snapshot) ? snapshot.get(pList + 3 + index) : 0;
        if index >= 0 && index < Self::size__int_typosnapshot(pList, snapshot) {
            snapshot.get__int(pList + 3 + index)
        } else {
            0
        }
    }

    // java public static int quickGet(int index, int pList, TypoSnapshot snapshot)
    pub fn quickGet__int_int_typosnapshot(index: i32, pList: i32, snapshot: &mut TypoSnapshot) -> i32 {
        // java return snapshot.get(pList + 3 + index);
        snapshot.get__int(pList + 3 + index)
    }

    // java public static int last(int pList, TypoSnapshot snapshot)
    pub fn last__int_typosnapshot(pList: i32, snapshot: &TypoSnapshot) -> i32 {
        // java int size = size(pList, snapshot); return size <= 0 ? 0 : snapshot.get(pList + 3 + size - 1);
        let size: i32 = Self::size__int_typosnapshot(pList, snapshot);
        if size <= 0 { 0 } else { snapshot.get__int(pList + 3 + size - 1) }
    }

    // java public static int alloc(TypoSnapshot snapshot, int capacitySize)
    pub fn alloc__typosnapshot_int(snapshot: &mut TypoSnapshot, capacitySize: i32) -> i32 {
        // java int allocSize = 3 + capacitySize; int pointer = snapshot.alloc(allocSize); snapshot.set(pointer, 15, allocSize);
        let allocSize: i32 = 3 + capacitySize;
        let pointer: i32 = snapshot.alloc(allocSize);
        snapshot.set__int_int_int(pointer, 15, allocSize);
        pointer
    }

    // java public static int getCapacitySize(int pointer, TypoSnapshot snapshot)
    pub fn getCapacitySize__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        snapshot.get__int(pointer + 1) - 3
    }

    // java public static int insert(int item, int pos, int pList, TypoSnapshot snapshot)
    pub fn insert__int_int_int_typosnapshot(item: i32, pos: i32, mut pList: i32, snapshot: &mut TypoSnapshot) -> i32 {
        if pos < 0 {
            return pList;
        }
        if pList == 0 {
            pList = Self::alloc__typosnapshot_int(snapshot, 4);
        }
        let old_size = Self::size__int_typosnapshot(pList, snapshot);
        let new_size = old_size.saturating_add(1).max(pos + 1);
        let old_capacity = Self::getCapacitySize__int_typosnapshot(pList, snapshot);
        if new_size >= old_capacity {
            let grown = if old_capacity <= 0 { 4 } else { old_capacity.saturating_mul(2).max(4) };
            let new_list = Self::alloc__typosnapshot_int(snapshot, grown);
            if pos > 0 {
                snapshot.copy__int_int_int(pList + 3, new_list + 3, pos);
            }
            if pos < old_size {
                snapshot.copy__int_int_int(pList + 3 + pos, new_list + 3 + pos + 1, old_size - pos);
            }
            pList = new_list;
        } else if pos != old_size {
            let copy_point = pList + 3 + pos;
            snapshot.copy__int_int_int(copy_point, copy_point + 1, old_size - pos);
        }
        snapshot.set__int_int(pList + 3 + pos, item);
        snapshot.set__int_int(pList + 2, new_size);
        pList
    }

    // java public static int add(int item, int pList, TypoSnapshot snapshot)
    pub fn add__int_int_typosnapshot(item: i32, mut p_list: i32, snapshot: &mut TypoSnapshot) -> i32 {
        if p_list == 0 {
            p_list = Self::alloc__typosnapshot_int(snapshot, 4);
        }
        let old_size = Self::size__int_typosnapshot(p_list, snapshot);
        let new_size = old_size + 1;
        let old_capacity = Self::getCapacitySize__int_typosnapshot(p_list, snapshot);
        if new_size >= old_capacity {
            let new_list = Self::alloc__typosnapshot_int(snapshot, old_capacity << 1);
            snapshot.copy__int_int_int(p_list + 3, new_list + 3, old_size);
            p_list = new_list;
        }
        snapshot.set__int_int(p_list + 3 + old_size, item);
        snapshot.set__int_int(p_list + 2, new_size);
        p_list
    }

    // java public static int remove(int pos, int pList, TypoSnapshot snapshot)
    pub fn remove__int_int_typosnapshot(pos: i32, p_list: i32, snapshot: &mut TypoSnapshot) -> i32 {
        if p_list == 0 || pos < 0 { return 0; }
        let old_size = Self::size__int_typosnapshot(p_list, snapshot);
        if pos >= old_size { return 0; }
        let removed = Self::quickGet__int_int_typosnapshot(pos, p_list, snapshot);
        if pos < old_size - 1 {
            let copy_point = p_list + 3 + pos + 1;
            snapshot.copy__int_int_int(copy_point, copy_point - 1, old_size - pos - 1);
        }
        snapshot.set__int_int(p_list + 2, old_size - 1);
        removed
    }

    // java public static int indexOf(int pItem, int pList, TypoSnapshot snapshot)
    pub fn indexOf__int_int_typosnapshot(p_item: i32, p_list: i32, snapshot: &TypoSnapshot) -> i32 {
        if p_list == 0 { return -1; }
        let size = Self::size__int_typosnapshot(p_list, snapshot);
        for i in 0..size {
            if snapshot.get__int(p_list + 3 + i) == p_item { return i; }
        }
        -1
    }

    // java public static int setSize(int newSize, int pList, TypoSnapshot snapshot)
    pub fn setSize__int_int_typosnapshot(new_size: i32, mut p_list: i32, snapshot: &mut TypoSnapshot) -> i32 {
        if p_list == 0 || new_size < 0 { return p_list; }
        let old_capacity = Self::getCapacitySize__int_typosnapshot(p_list, snapshot);
        if new_size <= old_capacity {
            snapshot.set__int_int(p_list + 2, new_size);
            return p_list;
        }
        let mut new_capacity = old_capacity;
        while new_size > new_capacity { new_capacity <<= 1; }
        let new_list = Self::alloc__typosnapshot_int(snapshot, new_capacity);
        snapshot.copy__int_int_int(p_list + 3, new_list + 3, old_capacity);
        snapshot.set__int_int(new_list + 2, new_size);
        p_list = new_list;
        p_list
    }
}
