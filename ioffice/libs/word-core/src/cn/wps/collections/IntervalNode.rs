// Java: import cn.wps.collections.PositionNode
use util::collections::PositionNode::PositionNode;
use util::collections::Node::Node;
use util::collections::RBBalancer::RBBalancer;

// java public abstract class IntervalNode extends PositionNode
#[repr(C)]
pub struct IntervalNode {
    // java PositionNode super_
    pub(crate) super_: PositionNode,
}

// java public abstract class IntervalNode extends PositionNode
impl IntervalNode {
    pub fn new() -> Self {
        Self {
            super_: PositionNode::new(),
        }
    }

    // java public abstract int getLength()
    pub fn getLength(&self) -> i32 {
      // abstract — subclasses override; base unused
      0
    }

    // java public abstract void setLength(int var1)
    pub fn setLength(&mut self, _var1: i32) {
      // abstract — subclasses override
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder sb = new StringBuilder();
      let mut sb = String::new();
      // java sb.append(this.getClass().getSimpleName());
      sb.push_str("IntervalNode");
      // java if (this.isHead())
      if self.super_.base.isHead() {
        // java sb.append("(head)");
        sb.push_str("(head)");
      }
      // java if (this.isRoot())
      if self.super_.base.isRoot() {
        // java sb.append("(root)");
        sb.push_str("(root)");
      }
      // java if (RBBalancer.isRed(this))
      let node_ptr = &self.super_.base as *const Node as *mut Node;
      if RBBalancer::isRed(node_ptr) {
        // java sb.append("\ncolor = red");
        sb.push_str("\ncolor = red");
      }
      // java else
      else {
        // java sb.append("\ncolor = black");
        sb.push_str("\ncolor = black");
      }
      // java if (!this.isHead())
      if !self.super_.base.isHead() {
        // java sb.append("\nposition = " + this.position());
        sb.push_str(&format!("\nposition = {}", self.super_.position()));
        // java sb.append("\nlength = " + this.getLength());
        sb.push_str(&format!("\nlength = {}", self.getLength()));
      }
      // java sb.append("\nchildren = (");
      sb.push_str("\nchildren = (");
      // java if (this.left != null)
      let left = self.super_.base.left;
      if !left.is_null() {
        // java sb.append("" + ((IntervalNode)this.left).position());
        unsafe {
          let pn = left as *mut PositionNode;
          sb.push_str(&format!("{}", (*pn).position()));
        }
      }
      // java else
      else {
        // java sb.append("null");
        sb.push_str("null");
      }
      // java sb.append(" ,");
      sb.push_str(" ,");
      // java if (this.right != null)
      let right = self.super_.base.right;
      if !right.is_null() {
        // java sb.append("" + ((IntervalNode)this.right).position());
        unsafe {
          let pn = right as *mut PositionNode;
          sb.push_str(&format!("{}", (*pn).position()));
        }
      }
      // java else
      else {
        // java sb.append("null");
        sb.push_str("null");
      }
      // java sb.append(")");
      sb.push_str(")");
      // java sb.append("\n");
      sb.push_str("\n");
      // java return sb.toString();
      return sb;
    }

    pub fn as_position_node_ptr(&mut self) -> *mut PositionNode {
        &mut self.super_ as *mut PositionNode
    }

    pub fn as_node_ptr(&mut self) -> *mut Node {
        &mut self.super_.base as *mut Node
    }
}

impl std::ops::Deref for IntervalNode {
    type Target = PositionNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for IntervalNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl Default for IntervalNode {
    fn default() -> Self {
        Self::new()
    }
}
