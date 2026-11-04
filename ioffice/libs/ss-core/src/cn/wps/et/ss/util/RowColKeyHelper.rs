// java package cn.wps.et.ss.util;
pub struct RowColKeyHelper;
impl RowColKeyHelper {
    // java return (long)row << 32 | (long)col;
    pub fn makeLongKey(row: i32, col: i32) -> i64 { ((row as i64) << 32) | col as i64 }
    // java return (int)(key >> 32);
    pub fn getRowFromKey(key: i64) -> i32 { (key >> 32) as i32 }
    // java return (int)key;
    pub fn getColFromKey(key: i64) -> i32 { key as i32 }
}
