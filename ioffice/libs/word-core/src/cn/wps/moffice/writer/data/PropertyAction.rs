// Java: import java.lang.Iterable
use stubs::java::lang::Iterable::*;
// Java: import cn.wps.moffice.writer.data.PropertyOperation
use crate::cn::wps::moffice::writer::data::PropertyOperation::*;

// java public interface PropertyAction
// Java: extends Iterable<PropertyOperation>
pub trait PropertyAction {
    // java  void addOperation(PropertyOperation var1)
    fn addOperation(&mut self, var1: Box<dyn PropertyOperation>);

    /// Java for-each over Iterable<PropertyOperation> (dyn-friendly).
    /// Concrete actions iterate their owned operations in Java insertion order.
    fn for_each_op(&self, _f: &mut dyn FnMut(&dyn PropertyOperation)) {}
}
