use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int EUCJP_CLASS_FACTOR
pub(crate) const EUCJP_CLASS_FACTOR: i32 = 6;

// Java: public class EUCJPSMModel extends SMModel
pub struct EUCJPSMModel;

// Java field: int[] eucjpClassTable
pub(crate) const EUCJP_CLASS_TABLE: [i32; 32] = [
    1145324612, 1430537284, 1145324612, 1145328708, 1145324612, 1145324612, 1145324612, 1145324612,
    1145324612, 1145324612, 1145324612, 1145324612, 1145324612, 1145324612, 1145324612, 1145324612,
    1431655765, 827675989, 1431655765, 1431655765, 572662309, 572662306, 572662306, 572662306,
    572662306, 572662306, 572662306, 572662306, 0, 0, 0, 1342177280,
];
// Java field: int[] eucjpStateTable
pub(crate) const EUCJP_STATE_TABLE: [i32; 5] = [
    286282563, 572657937, 286265378, 319885329, 4371,
];
// Java field: int[] eucjpCharLenTable
pub(crate) const EUCJP_CHAR_LEN_TABLE: [i32; 6] = [
    2, 2, 2, 3, 1, 0,
];

// Java: public class EUCJPSMModel extends SMModel
impl EUCJPSMModel {
    // Java: EUCJPSMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, eucjpClassTable), 6, new PkgInt(3, 7, 2, 15, eucjpStateTable), eucjpCharLenTable, Constants.CHARSET_EUC_JP);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &EUCJP_CLASS_TABLE),
            EUCJP_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &EUCJP_STATE_TABLE),
            &EUCJP_CHAR_LEN_TABLE,
            Constants::CHARSET_EUC_JP,
        )
    }
}
