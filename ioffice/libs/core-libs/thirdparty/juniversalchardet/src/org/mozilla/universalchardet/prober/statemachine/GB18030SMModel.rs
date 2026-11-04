use super::PkgInt::PkgInt;
use super::SMModel::SMModel;
use crate::org::mozilla::universalchardet::Constants;

// Java field: int GB18030_CLASS_FACTOR
pub(crate) const GB18030_CLASS_FACTOR: i32 = 7;

// Java: public class GB18030SMModel extends SMModel
pub struct GB18030SMModel;

// Java field: int[] gb18030ClassTable
pub(crate) const GB18030CLASS_TABLE: [i32; 32] = [
    286331153, 1118481, 286331153, 286327057, 286331153, 286331153, 858993459, 286331187,
    572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 572662306, 1109533218,
    1717986917, 1717986918, 1717986918, 1717986918, 1717986918, 1717986918, 1717986918, 1717986918,
    1717986918, 1717986918, 1717986918, 1717986918, 1717986918, 1717986918, 1717986918, 107374182,
];
// Java field: int[] gb18030StateTable
pub(crate) const GB18030STATE_TABLE: [i32; 6] = [
    318767105, 571543825, 17965602, 286326804, 303109393, 17,
];
// Java field: int[] gb18030CharLenTable
pub(crate) const GB18030CHAR_LEN_TABLE: [i32; 7] = [
    0, 1, 1, 1, 1, 1, 2,
];

// Java: public class GB18030SMModel extends SMModel
impl GB18030SMModel {
    // Java: GB18030SMModel()
    pub fn new() -> SMModel {
        // Java: super(new PkgInt(3, 7, 2, 15, gb18030ClassTable), 7, new PkgInt(3, 7, 2, 15, gb18030StateTable), gb18030CharLenTable, Constants.CHARSET_GB18030);
        SMModel::new(
            PkgInt::new(3, 7, 2, 15, &GB18030CLASS_TABLE),
            GB18030_CLASS_FACTOR,
            PkgInt::new(3, 7, 2, 15, &GB18030STATE_TABLE),
            &GB18030CHAR_LEN_TABLE,
            Constants::CHARSET_GB18030,
        )
    }
}
