// Java: import java.util.Iterator
use crate::org::jaxen::{Object, NodeIterator};
use crate::org::jaxen::Navigator::Navigator;
use crate::org::jaxen::UnsupportedAxisException::UnsupportedAxisException;

// Java: public interface NamedAccessNavigator extends Navigator
pub trait NamedAccessNavigator: Navigator {
    // Java: abstract Iterator getChildAxisIterator(Object var1, String var2, String var3, String var4)
    fn getNamedChildAxisIterator(&self, var1: &Object, var2: &str, var3: &str, var4: &str) -> Result<NodeIterator, UnsupportedAxisException>;

    // Java: abstract Iterator getAttributeAxisIterator(Object var1, String var2, String var3, String var4)
    fn getNamedAttributeAxisIterator(&self, var1: &Object, var2: &str, var3: &str, var4: &str) -> Result<NodeIterator, UnsupportedAxisException>;
}
