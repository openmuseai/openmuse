use std::sync::Arc;

use super::Child::Child;

// Java: public interface Parent extends Child
pub trait Parent: Child {
    // Java: abstract Iterator getChildren()
    fn getChildren(&self) -> Vec<Arc<dyn Child>>;

    // Java: abstract void addChild(Property var1)
    fn addChild(&mut self, var1: Arc<dyn Child>);

    // Java: abstract void setPreviousChild(Child var1)
    fn setPreviousChild_parent(&mut self, var1: Option<Arc<dyn Child>>);

    // Java: abstract void setNextChild(Child var1)
    fn setNextChild_parent(&mut self, var1: Option<Arc<dyn Child>>);
}
