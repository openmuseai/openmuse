#![allow(non_snake_case, non_upper_case_globals, dead_code)]

// Java field: int MEMORYMETAFILE
pub const MEMORYMETAFILE: i32 = 1;
// Java field: int DISKMETAFILE
pub const DISKMETAFILE: i32 = 2;
// Java field: int EMF_HEADER_LEN
pub const EMF_HEADER_LEN: i32 = 152;
// Java field: int WMF_HEADER_LEN
pub const WMF_HEADER_LEN: i32 = 40;
// Java field: int EMF_SIGNATURE
pub const EMF_SIGNATURE: i32 = 1179469088;
// Java field: int WMF_SIGNATURE
pub const WMF_SIGNATURE: i32 = -1698247209_i32;
// Java field: int EMFPLUS_SIGNATURE
pub const EMFPLUS_SIGNATURE: i32 = 726027589;

// Java: public interface MetaFileHeader
pub trait MetaFileHeader {
    // Java: abstract boolean isValid()
    fn isValid(&self) -> bool;

    // Java: abstract int getHandles()
    fn getHandles(&self) -> i32;
}
