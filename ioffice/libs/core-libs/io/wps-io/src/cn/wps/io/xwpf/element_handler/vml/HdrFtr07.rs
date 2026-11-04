// Java field: int HDR_EVEN
pub(crate) const HDR_EVEN: i32 = 0;
// Java field: int HDR_ODD
pub(crate) const HDR_ODD: i32 = 1;
// Java field: int HDR_FIRST
pub(crate) const HDR_FIRST: i32 = 2;
// Java field: int FTR_EVEN
pub(crate) const FTR_EVEN: i32 = 3;
// Java field: int FTR_ODD
pub(crate) const FTR_ODD: i32 = 4;
// Java field: int FTR_FIRST
pub(crate) const FTR_FIRST: i32 = 5;

// Java: public class HdrFtr07
pub struct HdrFtr07 {
    // Java field: String mRid
    pub mRid: String,
    // Java field: int mType
    pub mType: i32,
}

// Java: public class HdrFtr07
impl HdrFtr07 {
    // Java: HdrFtr07(String rid, int type)
    pub fn new(rid: String, r#type: i32) -> Self {
        // Java: super();
        HdrFtr07 {
            // Java: this.mRid = rid;
            mRid: rid,
            // Java: this.mType = type;
            mType: r#type,
        }
    }
}
