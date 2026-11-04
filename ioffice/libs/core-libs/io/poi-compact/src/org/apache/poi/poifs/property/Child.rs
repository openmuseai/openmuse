use std::sync::Arc;

// Java field: String _0_TABLE
pub const _0_TABLE: &str = "0Table";
// Java field: String _1_TABLE
pub const _1_TABLE: &str = "1Table";
// Java field: String _WORD_DOCUMENT
pub const _WORD_DOCUMENT: &str = "WordDocument";
// Java field: String _DATA
pub const _DATA: &str = "Data";

// Java: public interface Child
pub trait Child {
    // Java: abstract Child getNextChild()
    fn getNextChild(&self) -> Option<Arc<dyn Child>>;

    // Java: abstract Child getPreviousChild()
    fn getPreviousChild(&self) -> Option<Arc<dyn Child>>;

    // Java: abstract void setNextChild(Child var1)
    fn setNextChild(&mut self, var1: Option<Arc<dyn Child>>);

    // Java: abstract void setPreviousChild(Child var1)
    fn setPreviousChild(&mut self, var1: Option<Arc<dyn Child>>);
}
