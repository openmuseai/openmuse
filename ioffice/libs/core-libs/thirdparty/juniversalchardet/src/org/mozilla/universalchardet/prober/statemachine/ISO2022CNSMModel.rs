use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int ISO2022CN_CLASS_FACTOR
pub(crate) const ISO2022CN_CLASS_FACTOR: i32 = 9;

// Java: public class ISO2022CNSMModel extends SMModel
pub struct ISO2022CNSMModel;

// Java field: int[] iso2022cnClassTable
pub(crate) const ISO2022CN_CLASS_TABLE: [i32; 32] = [
    2, 0, 0, 4096, 0, 48, 0, 0,
    16384, 0, 0, 0, 0, 0, 0, 0,
    572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306,
    572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306,
];
// Java field: int[] iso2022cnStateTable
pub(crate) const ISO2022CN_STATE_TABLE: [i32; 8] = [
    304, 286331152, 572662289, 336663074, 286335249, 286331237, 286335249, 18944273,
];
// Java field: int[] iso2022cnCharLenTable
pub(crate) const ISO2022CN_CHAR_LEN_TABLE: [i32; 9] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0,
];

// Java: public class ISO2022CNSMModel extends SMModel
impl ISO2022CNSMModel {
    // Java: ISO2022CNSMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, iso2022cnClassTable), 9, new PkgInt(3, 7, 2, 15, iso2022cnStateTable), iso2022cnCharLenTable, Constants.CHARSET_ISO_2022_CN);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &ISO2022CN_CLASS_TABLE),
            ISO2022CN_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &ISO2022CN_STATE_TABLE),
            &ISO2022CN_CHAR_LEN_TABLE,
            Constants::CHARSET_ISO_2022_CN,
        )
    }
}
