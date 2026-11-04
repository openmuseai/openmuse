// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{PropertySet, PropertyValue};
// Java: import cn.wps.moffice.property.SetPropertyMode
use util::moffice::property::SetPropertyMode::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public interface PropertyOperation
pub trait PropertyOperation {
    // java  int getOp()
    fn getOp(&self) -> i32;

    // java  void setOp(int var1)
    fn setOp(&mut self, var1: i32);

    // java  PropertySet getOperand()
    fn getOperand(&self) -> &PropertySet;

    // java  Object getHint(Object var1)
    fn getHint(&self, var1: &dyn Object) -> Option<&dyn Object>;

    // java  void setHint(Object var1, Object var2)
    fn setHint(&mut self, var1: PropertyValue, var2: PropertyValue);

    // java  void removeHint(Object var1)
    fn removeHint(&mut self, var1: &dyn Object);

}
