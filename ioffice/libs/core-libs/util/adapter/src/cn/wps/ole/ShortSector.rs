// Java: public class ShortSector
pub struct ShortSector {
    // Java field: int mShortSectorIndex
    mShortSectorIndex: i32,
    // Java field: int mNormalSector
    mNormalSector: i32,
    // Java field: int mOffset
    mOffset: i32,
}

// Java: public class ShortSector
impl ShortSector {
    pub fn new() -> Self {
        ShortSector {
            mShortSectorIndex: 0,
            mNormalSector: 0,
            mOffset: 0,
        }
    }
}
