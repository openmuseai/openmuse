// Java: cn.wps.moss.core.sheet.KmoColInfoData extends KmoAtomData.
// The atom store is shared infrastructure; this data class preserves the Java
// field defaults and the constructor consumed by KmoColInfo.
#[derive(Clone, Debug)]
pub struct KmoColInfoData {
    pub minColIndex: i16,
    pub maxColIndex: i16,
    pub xfIndex: i16,
    pub width: i32,
    pub option_flags: i16,
    pub maxWidthCell: i32,
    pub minWidthCell: i32,
}

impl KmoColInfoData {
    pub const DEFAULT_COLINFO_WIDTH: i32 = 2560;
    pub const DEFAULT_COLINFO_OP_FLAG: i16 = 6;

    // Java: KmoColInfoData(short min, short max, int colWidth, short ixf, short option_flags).
    pub fn KmoColInfoData(min: i16, max: i16, col_width: i32, ixf: i16, option_flags: i16) -> Self {
        Self { minColIndex: min, maxColIndex: max, xfIndex: ixf,
            width: col_width, option_flags, maxWidthCell: -1, minWidthCell: -1 }
    }
}
