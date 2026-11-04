// Java: import cn.wps.moffice.property.MutablePropertySet
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;

use super::IProp::IProp;
use super::PropBase::PropBase;

// Java: public class Prop extends PropBase implements IProp
pub struct Prop {
    // Java field: PropBase mChangeHandler
    pub(crate) mChangeHandler: Option<PropBase>,
    pub(crate) base: PropBase,
}

// Java: public class Prop extends PropBase implements IProp
impl Prop {
    // Java: Prop()
    pub fn new() -> Self {
        // Java: super();
        Self {
            mChangeHandler: None,
            base: PropBase::new(),
        }
    }

    // Java: IProp getChangeHandler()
    pub fn getChangeHandler(&mut self) -> &mut PropBase {
        // Java: if (null == this.mChangeHandler)
        if self.mChangeHandler.is_none() {
            // Java: this.mChangeHandler = new PropBase();
            self.mChangeHandler = Some(PropBase::new());
        }
        // Java: return this.mChangeHandler;
        self.mChangeHandler.as_mut().unwrap()
    }

    // Java: MutablePropertySet getChangeProp()
    pub fn getChangeProp(&mut self) -> Option<&mut MutablePropertySet> {
        // Java: if (null == this.mChangeHandler)
        if self.mChangeHandler.is_none() {
            // Java: return null;
            None
        }
        // Java: else
        else {
            // Java: return this.mChangeHandler.isContainProp() ? this.mChangeHandler.getProp() : null;
            let handler = self.mChangeHandler.as_mut().unwrap();
            if handler.isContainProp() {
                Some(handler.getProp())
            } else {
                None
            }
        }
    }

    // Java: void clearProp()
    pub fn clearProp(&mut self) {
        // Java: super.clearProp();
        self.base.clearProp();
        // Java: if (null != this.mChangeHandler)
        if let Some(ref mut handler) = self.mChangeHandler {
            // Java: this.mChangeHandler.clearProp();
            handler.clearProp();
        }
    }

    pub fn getProp(&mut self) -> &mut MutablePropertySet {
        self.base.getProp()
    }

    pub fn isContainProp(&self) -> bool {
        self.base.isContainProp()
    }
}

impl IProp for Prop {
    fn getProp(&mut self) -> &mut MutablePropertySet {
        Prop::getProp(self)
    }
    fn isContainProp(&self) -> bool {
        Prop::isContainProp(self)
    }
    fn clearProp(&mut self) {
        Prop::clearProp(self)
    }
}
