// Java: public class IsfReaderContext
use super::StrokeInfo::StrokeInfo;

// Java: public class IsfReaderContext
pub struct IsfReaderContext {
    // Java field: StrokeInfo mCurrentStrokeInfo
    pub(crate) mCurrentStrokeInfo: Option<StrokeInfo>,
    // Java field: TagStrokeDescBlockFactory mTagStrokeDescBlockFactory
    pub(crate) mTagStrokeDescBlockFactory: super::TagStrokeDescBlockFactory::TagStrokeDescBlockFactory,
    // Java field: DefaultTaggedStructureFactory mDefaultTaggedStructureFactory
    pub(crate) mDefaultTaggedStructureFactory: super::DefaultTaggedStructureFactory::DefaultTaggedStructureFactory,
}

// Java: public class IsfReaderContext
impl IsfReaderContext {
    // Java: IsfReaderContext()
    pub fn IsfReaderContext() -> Self {
        // Java: super();
        Self {
            mCurrentStrokeInfo: None,
            mTagStrokeDescBlockFactory: super::TagStrokeDescBlockFactory::TagStrokeDescBlockFactory::TagStrokeDescBlockFactory(),
            mDefaultTaggedStructureFactory: super::DefaultTaggedStructureFactory::DefaultTaggedStructureFactory::DefaultTaggedStructureFactory(),
        }
    }
}
