// Java: import cn.wps.moffice.property.MutablePropertySet
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;

// Java: public interface IProp
pub trait IProp {
    // Java: abstract MutablePropertySet getProp()
    fn getProp(&mut self) -> &mut MutablePropertySet;

    // Java: abstract boolean isContainProp()
    fn isContainProp(&self) -> bool;

    // Java: abstract void clearProp()
    fn clearProp(&mut self);
}
