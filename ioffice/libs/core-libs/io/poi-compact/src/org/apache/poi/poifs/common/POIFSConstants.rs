use super::POIFSBigBlockSize::POIFSBigBlockSize;
use std::sync::OnceLock;

// Java: public interface POIFSConstants
pub trait POIFSConstants {
    // Java field: int SMALLER_BIG_BLOCK_SIZE
    // Java field: POIFSBigBlockSize SMALLER_BIG_BLOCK_SIZE_DETAILS
    // Java field: int LARGER_BIG_BLOCK_SIZE
    // Java field: POIFSBigBlockSize LARGER_BIG_BLOCK_SIZE_DETAILS
    // Java field: int PROPERTY_SIZE
    // Java field: int LARGEST_REGULAR_SECTOR_NUMBER
    // Java field: int DIFAT_SECTOR_BLOCK
    // Java field: int FAT_SECTOR_BLOCK
    // Java field: int END_OF_CHAIN
    // Java field: int UNUSED_BLOCK
    // Java field: byte[] OOXML_FILE_HEADER
}

// Java field: int SMALLER_BIG_BLOCK_SIZE
pub const SMALLER_BIG_BLOCK_SIZE: i32 = 512;
// Java field: int LARGER_BIG_BLOCK_SIZE
pub const LARGER_BIG_BLOCK_SIZE: i32 = 4096;
// Java field: int PROPERTY_SIZE
pub const PROPERTY_SIZE: i32 = 128;
// Java field: int LARGEST_REGULAR_SECTOR_NUMBER
pub const LARGEST_REGULAR_SECTOR_NUMBER: i32 = -5;
// Java field: int DIFAT_SECTOR_BLOCK
pub const DIFAT_SECTOR_BLOCK: i32 = -4;
// Java field: int FAT_SECTOR_BLOCK
pub const FAT_SECTOR_BLOCK: i32 = -3;
// Java field: int END_OF_CHAIN
pub const END_OF_CHAIN: i32 = -2;
// Java field: int UNUSED_BLOCK
pub const UNUSED_BLOCK: i32 = -1;
// Java field: byte[] OOXML_FILE_HEADER
pub static OOXML_FILE_HEADER: &[u8] = &[80, 75, 3, 4];

static SMALLER_BIG_BLOCK_SIZE_DETAILS_LOCK: OnceLock<POIFSBigBlockSize> = OnceLock::new();
// Java field: POIFSBigBlockSize SMALLER_BIG_BLOCK_SIZE_DETAILS
pub fn get_smaller_big_block_size_details() -> &'static POIFSBigBlockSize {
    SMALLER_BIG_BLOCK_SIZE_DETAILS_LOCK.get_or_init(|| POIFSBigBlockSize::new(512, 9))
}

static LARGER_BIG_BLOCK_SIZE_DETAILS_LOCK: OnceLock<POIFSBigBlockSize> = OnceLock::new();
// Java field: POIFSBigBlockSize LARGER_BIG_BLOCK_SIZE_DETAILS
pub fn get_larger_big_block_size_details() -> &'static POIFSBigBlockSize {
    LARGER_BIG_BLOCK_SIZE_DETAILS_LOCK.get_or_init(|| POIFSBigBlockSize::new(4096, 12))
}
