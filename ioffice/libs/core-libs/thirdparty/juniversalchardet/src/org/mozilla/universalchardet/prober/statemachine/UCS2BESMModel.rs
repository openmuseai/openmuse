use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int UCS2BE_CLASS_FACTOR
pub(crate) const UCS2BE_CLASS_FACTOR: i32 = 6;

// Java: public class UCS2BESMModel extends SMModel
pub struct UCS2BESMModel;

// Java field: int[] ucs2beClassTable
pub(crate) const UCS2BE_CLASS_TABLE: [i32; 32] = [
    0, 2097408, 0, 12288, 0, 3355440, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1409286144,
];
// Java field: int[] ucs2beStateTable
pub(crate) const UCS2BE_STATE_TABLE: [i32; 7] = [
    288626549, 572657937, 291923490, 1713792614, 393569894, 1717659269, 1140326,
];
// Java field: int[] ucs2beCharLenTable
pub(crate) const UCS2BE_CHAR_LEN_TABLE: [i32; 6] = [
    2, 2, 2, 0, 2, 2,
];

// Java: public class UCS2BESMModel extends SMModel
impl UCS2BESMModel {
    // Java: UCS2BESMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, ucs2beClassTable), 6, new PkgInt(3, 7, 2, 15, ucs2beStateTable), ucs2beCharLenTable, Constants.CHARSET_UTF_16BE);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &UCS2BE_CLASS_TABLE),
            UCS2BE_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &UCS2BE_STATE_TABLE),
            &UCS2BE_CHAR_LEN_TABLE,
            Constants::CHARSET_UTF_16BE,
        )
    }
}
