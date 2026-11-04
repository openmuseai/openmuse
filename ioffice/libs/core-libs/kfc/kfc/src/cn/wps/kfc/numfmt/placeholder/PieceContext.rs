use super::PContext::PContext;

// Java: public class PieceContext extends PContext
pub struct PieceContext {
    pub(crate) base: PContext,
    // Java field: boolean mHasTextPlaceHolder
    pub(crate) mHasTextPlaceHolder: bool,
    // Java field: boolean mHasFill
    pub(crate) mHasFill: bool,
    // Java field: boolean mForceArbicShape
    pub(crate) mForceArbicShape: bool,
    // Java field: short mPieceType
    pub(crate) mPieceType: i16,
    // Java field: String mNumAreaCode
    pub(crate) mNumAreaCode: Option<String>,
    // Java field: String mDBNumCode
    pub(crate) mDBNumCode: Option<String>,
}

// Java: public class PieceContext extends PContext
impl PieceContext {
    // Java: PieceContext()
    pub fn new() -> Self {
        // Java: this((short)2);
        Self::new_with_type(2)
    }

    // Java: PieceContext(short contextType)
    pub fn new_with_type(context_type: i16) -> Self {
        // Java: super();
        let mut base = PContext::new();
        // Java: this.mContextType = contextType;
        base.mContextType = context_type;
        PieceContext {
            base,
            mHasTextPlaceHolder: false,
            mHasFill: false,
            mForceArbicShape: false,
            mPieceType: 0,
            mNumAreaCode: None,
            mDBNumCode: None,
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mAddMinus = false;
        self.base.mAddMinus = false;
        // Java: this.mHasTextPlaceHolder = false;
        self.mHasTextPlaceHolder = false;
        // Java: this.mHasFill = false;
        self.mHasFill = false;
        // Java: this.mForceArbicShape = false;
        self.mForceArbicShape = false;
        // Java: this.mNumAreaCode = null;
        self.mNumAreaCode = None;
        // Java: this.mDBNumCode = null;
        self.mDBNumCode = None;
        // Java: this.mPieceType = 0;
        self.mPieceType = 0;
    }
}

impl std::ops::Deref for PieceContext {
    type Target = PContext;
    fn deref(&self) -> &PContext {
        &self.base
    }
}
impl std::ops::DerefMut for PieceContext {
    fn deref_mut(&mut self) -> &mut PContext {
        &mut self.base
    }
}
