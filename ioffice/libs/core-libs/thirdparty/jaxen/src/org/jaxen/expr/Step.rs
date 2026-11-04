// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.ContextSupport
use crate::org::jaxen::ContextSupport::ContextSupport;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.UnsupportedAxisException
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;
use crate::org::jaxen::{Object, NodeIterator, List};

// Java: public interface Step extends Predicated
pub trait Step {
    // Java: abstract boolean matches(Object var1, ContextSupport var2)
    fn matches(&self, var1: &Object, var2: &ContextSupport) -> Result<bool, JaxenException>;

    // Java: abstract String getText()
    fn getText(&self) -> String;

    // Java: abstract void simplify()
    fn simplify(&mut self);

    // Java: abstract int getAxis()
    fn getAxis(&self) -> i32;

    // Java: abstract Iterator axisIterator(Object var1, ContextSupport var2)
    fn axisIterator(&self, var1: &Object, var2: &ContextSupport) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract List evaluate(Context var1)
    fn evaluate(&self, var1: &Context) -> Result<List, JaxenException>;
}
