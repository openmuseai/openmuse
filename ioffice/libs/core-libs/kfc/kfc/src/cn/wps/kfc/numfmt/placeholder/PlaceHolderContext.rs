// Java: import cn.wps.kfc.numfmt.format.FormatContext
use super::PContext::PContext;
use crate::cn::wps::kfc::numfmt::format::FormatContext::FormatContext;

// Java: public class PlaceHolderContext extends PContext
pub struct PlaceHolderContext {
    pub(crate) base: PContext,
    // Java field: int mPos
    pub(crate) mPos: i32,
    // Java field: int mFillPos
    pub(crate) mFillPos: i32,
    // Java field: char mFillChar
    pub(crate) mFillChar: char,
    // Java field: int mOffset
    pub(crate) mOffset: i32,
    // Java field: boolean mFirstDigitPass
    pub(crate) mFirstDigitPass: bool,
    // Java field: boolean mShowSeparator
    pub(crate) mShowSeparator: bool,
    // Java field: int mLDigitHolderCount
    pub(crate) mLDigitHolderCount: i32,
    // Java field: int mRDigitHolderCount
    pub(crate) mRDigitHolderCount: i32,
    // Java field: int mLDigitOffset
    pub(crate) mLDigitOffset: i32,
    // Java field: int mRDigitOffset
    pub(crate) mRDigitOffset: i32,
    // Java field: boolean mIs24HR
    pub(crate) mIs24HR: bool,
    // Java field: FormatContext mFormatterContext
    pub(crate) mFormatterContext: std::rc::Rc<std::cell::RefCell<FormatContext>>,
}

// Java: public class PlaceHolderContext extends PContext
impl PlaceHolderContext {
    // Java: PlaceHolderContext()
    pub fn new() -> Self {
        // Java: super();
        PlaceHolderContext {
            base: PContext::new(),
            mPos: 0,
            mFillPos: -1,
            mFillChar: '\0',
            mOffset: -1,
            mFirstDigitPass: false,
            mShowSeparator: false,
            mLDigitHolderCount: 0,
            mRDigitHolderCount: 0,
            mLDigitOffset: 0,
            mRDigitOffset: 0,
            mIs24HR: true,
            mFormatterContext: std::rc::Rc::new(std::cell::RefCell::new(FormatContext::new())),
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: super.clear();
        self.base.clear();
        // Java: this.mOffset = -1;
        self.mOffset = -1;
        // Java: this.mPos = -1;
        self.mPos = -1;
        // Java: this.mShowSeparator = false;
        self.mShowSeparator = false;
        // Java: this.mLDigitHolderCount = 0;
        self.mLDigitHolderCount = 0;
        // Java: this.mRDigitHolderCount = 0;
        self.mRDigitHolderCount = 0;
        // Java: this.mLDigitOffset = 0;
        self.mLDigitOffset = 0;
        // Java: this.mRDigitOffset = 0;
        self.mRDigitOffset = 0;
        // Java: this.mFirstDigitPass = false;
        self.mFirstDigitPass = false;
        // Java: this.mFillPos = -1;
        self.mFillPos = -1;
        // Java: this.mIs24HR = true;
        self.mIs24HR = true;
    }

    // Java: boolean isFirstDigitEx()
    pub fn isFirstDigitEx(&mut self) -> bool {
        // Java: boolean isFirst = !this.mFirstDigitPass;
        let is_first = !self.mFirstDigitPass;
        // Java: this.mFirstDigitPass = true;
        self.mFirstDigitPass = true;
        // Java: return isFirst;
        is_first
    }
}

impl std::ops::Deref for PlaceHolderContext {
    type Target = PContext;
    fn deref(&self) -> &PContext {
        &self.base
    }
}
impl std::ops::DerefMut for PlaceHolderContext {
    fn deref_mut(&mut self) -> &mut PContext {
        &mut self.base
    }
}
