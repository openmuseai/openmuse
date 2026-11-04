#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::collections::textrope::INotifyTextRope::Listener as TextRopeListener;
use crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode;
use crate::cn::wps::moffice::writer::data::table::tree::RowInfoNode::RowInfoNode;
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::TableInfoNode;
use crate::cn::wps::moffice::writer::data::table::tree::TreeTableCache::TreeTableCache;

/// Java `TableInfos` interface.
pub trait TableInfos {
    // java int size();
    fn size(&self) -> i32 {
        0
    }
    /// 单元格里的嵌套表个数。Java 把它们放在 `CellInfoNode` 下，不计入 `size()`。
    fn nestedTableCount(&self) -> i32 {
        0
    }
    // java TableInfo getTableByIndex(int index);
    fn getTableByIndex(&self, _index: i32) -> Option<TableInfoNode> {
        None
    }
    // java TableInfo getTableInfoByLevel(int cp, int level);
    fn getTableInfoByLevel(&self, _cp: i32, _level: i32) -> Option<TableInfoNode> {
        None
    }
    // java RowInfo getRowInfoByLevel(int cp, int level);
    fn getRowInfoByLevel(&self, _cp: i32, _level: i32) -> Option<RowInfoNode> {
        None
    }
    // java CellInfo getCellInfoByLevel(int cp, int level);
    fn getCellInfoByLevel(&self, _cp: i32, _level: i32) -> Option<CellInfoNode> {
        None
    }
    // java TableInfo getTableInfoByRange(int startCP, int endCP);
    fn getTableInfoByRange(&self, _start_cp: i32, _end_cp: i32) -> Option<TableInfoNode> {
        None
    }
    // java boolean hasTable(int startCP, int endCP, int level);
    fn hasTable(&self, _start_cp: i32, _end_cp: i32, _level: i32) -> bool {
        false
    }
    // java void invalidAfter(int cp);
    fn invalidAfter(&self, _cp: i32) {}
    // java void doGridAutoResize();
    fn doGridAutoResize(&self) {}
    // java INotifyTextRope.Listener getTextListener();
    fn getTextListener(&self) -> Option<&dyn TextRopeListener> {
        None
    }
    /// Stable address of the cache after `TableInfosImpl` is boxed.
    fn cache_ptr(&self) -> *mut TreeTableCache {
        std::ptr::null_mut()
    }
}
