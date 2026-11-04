// Java: public class StrokeInfo
pub struct StrokeInfo {
    // Java field: boolean mHasPressureData
    pub(crate) mHasPressureData: bool,
}

// Java: public class StrokeInfo
impl StrokeInfo {
    // Java: StrokeInfo()
    pub fn StrokeInfo() -> Self {
        // Java: super();
        Self {
            mHasPressureData: false,
        }
    }
}
