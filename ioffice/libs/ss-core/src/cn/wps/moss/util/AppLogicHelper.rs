pub struct AppLogicHelper;
impl AppLogicHelper {
    // java long ret=(long)row;ret<<=32;ret|=(long)col;return ret;
    pub fn rc2long(row:i32,col:i32)->i64{((row as i64)<<32)|(col as i64)}
}

impl AppLogicHelper{
// java public static long cellPos2Long(int sheetUid, int row, int col) {
// java return (long)sheetUid << 48 | ((long)row & 4294967295L) << 16 | (long)col & 65535L;
// java }
pub fn cellPos2Long(uid:i32,row:i32,col:i32)->i64{((uid as i64)<<48)|(((row as i64)&4294967295)<<16)|((col as i64)&65535)}
}
