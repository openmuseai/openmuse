// Java: import cn.wps.collections.IntervalNode
use crate::cn::wps::collections::IntervalNode::*;

// Java: public abstract class IntervalMapNode extends IntervalNode 
#[derive(Default)]
pub struct IntervalMapNode {
    // Java field: IntervalNode super_;
    pub super_: IntervalNode,
    // Java field:    IntervalMapNode peer;
    pub peer: IntervalMapNode,
}

// Java: public abstract class IntervalMapNode extends IntervalNode 
impl IntervalMapNode {
    // Java: public final IntervalMapNode getPeer() 
    pub fn getPeer(&mut self) -> &IntervalMapNode {
      // Java: return this.peer;
      return self.peer;
    }

}

impl std::ops::Deref for IntervalMapNode {
    type Target = IntervalNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for IntervalMapNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
