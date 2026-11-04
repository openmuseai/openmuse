use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int EUCKR_CLASS_FACTOR
pub(crate) const EUCKR_CLASS_FACTOR: i32 = 4;

// Java: public class EUCKRSMModel extends SMModel
pub struct EUCKRSMModel;

// Java field: int[] euckrClassTable
pub(crate) const EUCKR_CLASS_TABLE: [i32; 32] = [
    286331153, 1118481, 286331153, 286327057, 286331153, 286331153, 286331153, 286331153,
    286331153, 286331153, 286331153, 286331153, 286331153, 286331153, 286331153, 286331153,
    0, 0, 0, 0, 572662304, 858923554, 572662306, 572662306,
    572662306, 572662322, 572662306, 572662306, 572662306, 572662306, 572662306, 35791394,
];
// Java field: int[] euckrStateTable
pub(crate) const EUCKR_STATE_TABLE: [i32; 2] = [
    286331649, 1122850,
];
// Java field: int[] euckrCharLenTable
pub(crate) const EUCKR_CHAR_LEN_TABLE: [i32; 4] = [
    0, 1, 2, 0,
];

// Java: public class EUCKRSMModel extends SMModel
impl EUCKRSMModel {
    // Java: EUCKRSMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, euckrClassTable), 4, new PkgInt(3, 7, 2, 15, euckrStateTable), euckrCharLenTable, Constants.CHARSET_EUC_KR);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &EUCKR_CLASS_TABLE),
            EUCKR_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &EUCKR_STATE_TABLE),
            &EUCKR_CHAR_LEN_TABLE,
            Constants::CHARSET_EUC_KR,
        )
    }
}
