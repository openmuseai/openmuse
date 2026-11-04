use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int BIG5_CLASS_FACTOR
pub(crate) const BIG5_CLASS_FACTOR: i32 = 5;

// Java: public class Big5SMModel extends SMModel
pub struct Big5SMModel;

// Java field: int[] big5ClassTable
pub(crate) const BIG5CLASS_TABLE: [i32; 32] = [
    286331153, 1118481, 286331153, 286327057, 286331153, 286331153, 286331153, 286331153,
    572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 304226850,
    1145324612, 1145324612, 1145324612, 1145324612, 858993460, 858993459, 858993459, 858993459,
    858993459, 858993459, 858993459, 858993459, 858993459, 858993459, 858993459, 53687091,
];
// Java field: int[] big5StateTable
pub(crate) const BIG5STATE_TABLE: [i32; 3] = [
    286339073, 304226833, 1,
];
// Java field: int[] big5CharLenTable
pub(crate) const BIG5CHAR_LEN_TABLE: [i32; 5] = [
    0, 1, 1, 2, 0,
];

// Java: public class Big5SMModel extends SMModel
impl Big5SMModel {
    // Java: Big5SMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, big5ClassTable), 5, new PkgInt(3, 7, 2, 15, big5StateTable), big5CharLenTable, Constants.CHARSET_BIG5);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &BIG5CLASS_TABLE),
            BIG5_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &BIG5STATE_TABLE),
            &BIG5CHAR_LEN_TABLE,
            Constants::CHARSET_BIG5,
        )
    }
}
