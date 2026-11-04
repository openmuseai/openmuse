#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.moffice.property.MutablePropertySet

// Java: public class StylePrModel
pub struct StylePrModel {
    // Java field: MutablePropertySet mRprBaseSet
    pub mRprBaseSet: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: MutablePropertySet mPPrGeneralSet
    pub mPPrGeneralSet: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: MutablePropertySet mTcPrSet
    pub mTcPrSet: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: MutablePropertySet mTblPrBaseSet
    pub mTblPrBaseSet: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class StylePrModel
impl StylePrModel {
    // Java: StylePrModel()
    pub fn new() -> Self {
        // Java: super();
        StylePrModel {
            mRprBaseSet: None,
            mPPrGeneralSet: None,
            mTcPrSet: None,
            mTblPrBaseSet: None,
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mRprBaseSet = null;
        self.mRprBaseSet = None;
        // Java: this.mPPrGeneralSet = null;
        self.mPPrGeneralSet = None;
        // Java: this.mTcPrSet = null;
        self.mTcPrSet = None;
        // Java: this.mTblPrBaseSet = null;
        self.mTblPrBaseSet = None;
    }
}
