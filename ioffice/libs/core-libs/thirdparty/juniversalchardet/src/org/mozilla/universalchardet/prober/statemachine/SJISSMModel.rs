use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int SJIS_CLASS_FACTOR
pub(crate) const SJIS_CLASS_FACTOR: i32 = 6;

// Java: public class SJISSMModel extends SMModel
pub struct SJISSMModel;

// Java field: int[] sjisClassTable
pub(crate) const SJIS_CLASS_TABLE: [i32; 32] = [
    286331153, 1118481, 286331153, 286327057, 286331153, 286331153, 286331153, 286331153,
    572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 304226850,
    858993459, 858993459, 858993459, 858993459, 572662306, 572662306, 572662306, 572662306,
    572662306, 572662306, 572662306, 572662306, 858993459, 1145254707, 1145324612, 279620,
];
// Java field: int[] sjisStateTable
pub(crate) const SJIS_STATE_TABLE: [i32; 3] = [
    286339073, 572657937, 4386,
];
// Java field: int[] sjisCharLenTable
pub(crate) const SJIS_CHAR_LEN_TABLE: [i32; 6] = [
    0, 1, 1, 2, 0, 0,
];

// Java: public class SJISSMModel extends SMModel
impl SJISSMModel {
    // Java: SJISSMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, sjisClassTable), 6, new PkgInt(3, 7, 2, 15, sjisStateTable), sjisCharLenTable, Constants.CHARSET_SHIFT_JIS);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &SJIS_CLASS_TABLE),
            SJIS_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &SJIS_STATE_TABLE),
            &SJIS_CHAR_LEN_TABLE,
            Constants::CHARSET_SHIFT_JIS,
        )
    }
}
