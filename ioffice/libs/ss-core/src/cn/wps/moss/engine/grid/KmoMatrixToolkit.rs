// java package cn.wps.moss.engine.grid;

// java abstract class KmoMatrixToolkit
pub struct KmoMatrixToolkit;

// java abstract class KmoMatrixToolkit
impl KmoMatrixToolkit {
    // java public static byte[] newBuff()
    pub fn newBuff() -> [u8; 7] {
        // java return new byte[7];
        [0u8; 7]
    }

    // java public static final boolean isRowValid(int row, int maxRow)
    pub fn isRowValid(row: i32, maxRow: i32) -> bool {
        // java return row >= 0 && row < maxRow;
        row >= 0 && row < maxRow
    }

    // java public static final boolean isColValid(int col, int maxCol)
    pub fn isColValid(col: i32, maxCol: i32) -> bool {
        // java return col >= 0 && col < maxCol;
        col >= 0 && col < maxCol
    }
}
