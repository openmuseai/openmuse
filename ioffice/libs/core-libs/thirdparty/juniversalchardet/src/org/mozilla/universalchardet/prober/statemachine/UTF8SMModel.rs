use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int UTF8_CLASS_FACTOR
pub(crate) const UTF8_CLASS_FACTOR: i32 = 16;

// Java: public class UTF8SMModel extends SMModel
pub struct UTF8SMModel;

// Java field: int[] utf8ClassTable
pub(crate) const UTF8CLASS_TABLE: [i32; 32] = [
    286331153, 1118481, 286331153, 286327057, 286331153, 286331153, 286331153, 286331153,
    286331153, 286331153, 286331153, 286331153, 286331153, 286331153, 286331153, 286331153,
    858989090, 1145324612, 1145324612, 1145324612, 1431655765, 1431655765, 1431655765, 1431655765,
    1717986816, 1717986918, 1717986918, 1717986918, -2004318073, -2003269496, -1145324614, 16702940,
];
// Java field: int[] utf8StateTable
pub(crate) const UTF8STATE_TABLE: [i32; 26] = [
    -1408167679, 878082233, 286331153, 286331153, 572662306, 572662306, 290805009, 286331153,
    290803985, 286331153, 293041937, 286331153, 293015825, 286331153, 295278865, 286331153,
    294719761, 286331153, 298634257, 286331153, 297865489, 286331153, 287099921, 286331153,
    285212689, 286331153,
];
// Java field: int[] utf8CharLenTable
pub(crate) const UTF8CHAR_LEN_TABLE: [i32; 16] = [
    0, 1, 0, 0, 0, 0, 2, 3,
    3, 3, 4, 4, 5, 5, 6, 6,
];

// Java: public class UTF8SMModel extends SMModel
impl UTF8SMModel {
    // Java: UTF8SMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, utf8ClassTable), 16, new PkgInt(3, 7, 2, 15, utf8StateTable), utf8CharLenTable, Constants.CHARSET_UTF_8);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &UTF8CLASS_TABLE),
            UTF8_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &UTF8STATE_TABLE),
            &UTF8CHAR_LEN_TABLE,
            Constants::CHARSET_UTF_8,
        )
    }
}
