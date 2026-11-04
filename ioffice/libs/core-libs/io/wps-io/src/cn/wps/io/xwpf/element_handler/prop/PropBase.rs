// Java: import cn.wps.moffice.property.MutablePropertySet
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;

use super::IProp::IProp;

// Java: public class PropBase implements IProp
pub struct PropBase {
    // Java field: MutablePropertySet mProp
    pub(crate) mProp: Option<MutablePropertySet>,
}

// Java: public class PropBase implements IProp
impl PropBase {
    // Java: PropBase()
    pub fn new() -> Self {
        // Java: super();
        Self { mProp: None }
    }

    // Java: MutablePropertySet getProp()
    pub fn getProp(&mut self) -> &mut MutablePropertySet {
        // Java: if (null == this.mProp)
        if self.mProp.is_none() {
            // Java: this.mProp = new MutablePropertySet();
            self.mProp = Some(MutablePropertySet::new());
        }
        // Java: return this.mProp;
        self.mProp.as_mut().unwrap()
    }

    // Java: void clearProp()
    pub fn clearProp(&mut self) {
        // Java: this.mProp = null;
        self.mProp = None;
    }

    // Java: boolean isContainProp()
    pub fn isContainProp(&self) -> bool {
        // Java: return null != this.mProp;
        self.mProp.is_some()
    }

}

impl IProp for PropBase {
    fn getProp(&mut self) -> &mut MutablePropertySet {
        PropBase::getProp(self)
    }
    fn isContainProp(&self) -> bool {
        PropBase::isContainProp(self)
    }
    fn clearProp(&mut self) {
        PropBase::clearProp(self)
    }
}
