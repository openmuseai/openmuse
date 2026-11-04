use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int EUCTW_CLASS_FACTOR
pub(crate) const EUCTW_CLASS_FACTOR: i32 = 7;

// Java: public class EUCTWSMModel extends SMModel
pub struct EUCTWSMModel;

// Java field: int[] euctwClassTable
pub(crate) const EUCTW_CLASS_TABLE: [i32; 32] = [
    572662306, 2236962, 572662306, 572654114, 572662306, 572662306, 572662306, 572662306,
    572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306,
    0, 100663296, 0, 0, 1145324592, 286331221, 286331153, 286331153,
    858985233, 858993459, 858993459, 858993459, 858993459, 858993459, 858993459, 53687091,
];
// Java field: int[] euctwStateTable
pub(crate) const EUCTW_STATE_TABLE: [i32; 6] = [
    338898961, 571543825, 269623842, 286330880, 1052949, 16,
];
// Java field: int[] euctwCharLenTable
pub(crate) const EUCTW_CHAR_LEN_TABLE: [i32; 7] = [
    0, 0, 1, 2, 2, 2, 3,
];

// Java: public class EUCTWSMModel extends SMModel
impl EUCTWSMModel {
    // Java: EUCTWSMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, euctwClassTable), 7, new PkgInt(3, 7, 2, 15, euctwStateTable), euctwCharLenTable, Constants.CHARSET_EUC_TW);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &EUCTW_CLASS_TABLE),
            EUCTW_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &EUCTW_STATE_TABLE),
            &EUCTW_CHAR_LEN_TABLE,
            Constants::CHARSET_EUC_TW,
        )
    }
}
