// Java: import cn.wps.moffice.property.MutablePropertySet
// use util::cn::wps::moffice::property::MutablePropertySet;

// Java: public interface IPropHandler
pub trait IPropHandler {
    // Java: abstract MutablePropertySet getProp()
    fn getProp(&self) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract void clearProp()
    fn clearProp(&self);

}
