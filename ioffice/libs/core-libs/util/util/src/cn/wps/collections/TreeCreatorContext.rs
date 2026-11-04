#![allow(non_snake_case, dead_code, unused_variables)]

use super::VisitorContext::VisitorContext;
use super::TreeCreator::TreeCreator;

// Java: public class TreeCreatorContext extends VisitorContext
pub struct TreeCreatorContext {
    pub base: VisitorContext,
    // Java field: int size
    pub(crate) size: i32,
}

// Java: public class TreeCreatorContext extends VisitorContext
impl TreeCreatorContext {
    // Java: TreeCreatorContext()
    pub fn new() -> Self {
        // Java: super();
        Self {
            base: VisitorContext::new(),
            size: 0,
        }
    }

    // Java: int getLeftSize()
    pub fn getLeftSize(&self) -> i32 {
        // Java: return TreeCreator.left_size(this.size);
        TreeCreator::left_size(self.size)
    }

    // Java: int getRightSize()
    pub fn getRightSize(&self) -> i32 {
        // Java: return TreeCreator.right_size(this.size);
        TreeCreator::right_size(self.size)
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return this.size;
        self.size
    }
}
