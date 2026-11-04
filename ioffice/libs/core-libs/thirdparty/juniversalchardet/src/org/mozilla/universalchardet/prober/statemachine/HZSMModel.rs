use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int HZS_CLASS_FACTOR
pub(crate) const HZS_CLASS_FACTOR: i32 = 6;

// Java: public class HZSMModel extends SMModel
pub struct HZSMModel;

// Java field: int[] hzsClassTable
pub(crate) const HZS_CLASS_TABLE: [i32; 32] = [
    1, 0, 0, 4096, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 38813696,
    286331153, 286331153, 286331153, 286331153, 286331153, 286331153, 286331153, 286331153,
    286331153, 286331153, 286331153, 286331153, 286331153, 286331153, 286331153, 286331153,
];
// Java field: int[] hzsStateTable
pub(crate) const HZS_STATE_TABLE: [i32; 6] = [
    285213456, 572657937, 335548706, 341120533, 336872468, 36,
];
// Java field: int[] hzsCharLenTable
pub(crate) const HZS_CHAR_LEN_TABLE: [i32; 6] = [
    0, 0, 0, 0, 0, 0,
];

// Java: public class HZSMModel extends SMModel
impl HZSMModel {
    // Java: HZSMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, hzsClassTable), 6, new PkgInt(3, 7, 2, 15, hzsStateTable), hzsCharLenTable, Constants.CHARSET_HZ_GB_2312);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &HZS_CLASS_TABLE),
            HZS_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &HZS_STATE_TABLE),
            &HZS_CHAR_LEN_TABLE,
            Constants::CHARSET_HZ_GB_2312,
        )
    }
}
