use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int ISO2022JP_CLASS_FACTOR
pub(crate) const ISO2022JP_CLASS_FACTOR: i32 = 10;

// Java: public class ISO2022JPSMModel extends SMModel
pub struct ISO2022JPSMModel;

// Java field: int[] iso2022jpClassTable
pub(crate) const ISO2022JP_CLASS_TABLE: [i32; 32] = [
    2, 570425344, 0, 4096, 458752, 3, 0, 0,
    525318, 1424, 0, 0, 0, 0, 0, 0,
    572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306,
    572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306,
];
// Java field: int[] iso2022jpStateTable
pub(crate) const ISO2022JP_STATE_TABLE: [i32; 9] = [
    304, 286331136, 572657937, 287449634, 289476945, 303194385, 571543825, 286335249,
    1184017,
];
// Java field: int[] iso2022jpCharLenTable
pub(crate) const ISO2022JP_CHAR_LEN_TABLE: [i32; 10] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
];

// Java: public class ISO2022JPSMModel extends SMModel
impl ISO2022JPSMModel {
    // Java: ISO2022JPSMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, iso2022jpClassTable), 10, new PkgInt(3, 7, 2, 15, iso2022jpStateTable), iso2022jpCharLenTable, Constants.CHARSET_ISO_2022_JP);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &ISO2022JP_CLASS_TABLE),
            ISO2022JP_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &ISO2022JP_STATE_TABLE),
            &ISO2022JP_CHAR_LEN_TABLE,
            Constants::CHARSET_ISO_2022_JP,
        )
    }
}
