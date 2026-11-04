use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int ISO2022KR_CLASS_FACTOR
pub(crate) const ISO2022KR_CLASS_FACTOR: i32 = 6;

// Java: public class ISO2022KRSMModel extends SMModel
pub struct ISO2022KRSMModel;

// Java field: int[] iso2022krClassTable
pub(crate) const ISO2022KR_CLASS_TABLE: [i32; 32] = [
    2, 0, 0, 4096, 196608, 64, 0, 0,
    20480, 0, 0, 0, 0, 0, 0, 0,
    572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306,
    572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306,
];
// Java field: int[] iso2022krStateTable
pub(crate) const ISO2022KR_STATE_TABLE: [i32; 5] = [
    285212976, 572657937, 289476898, 286593297, 8465,
];
// Java field: int[] iso2022krCharLenTable
pub(crate) const ISO2022KR_CHAR_LEN_TABLE: [i32; 6] = [
    0, 0, 0, 0, 0, 0,
];

// Java: public class ISO2022KRSMModel extends SMModel
impl ISO2022KRSMModel {
    // Java: ISO2022KRSMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, iso2022krClassTable), 6, new PkgInt(3, 7, 2, 15, iso2022krStateTable), iso2022krCharLenTable, Constants.CHARSET_ISO_2022_KR);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &ISO2022KR_CLASS_TABLE),
            ISO2022KR_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &ISO2022KR_STATE_TABLE),
            &ISO2022KR_CHAR_LEN_TABLE,
            Constants::CHARSET_ISO_2022_KR,
        )
    }
}
