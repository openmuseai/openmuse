// Java: import cn.wps.collections.PositionNode
use util::collections::PositionNode::*;
// Java: import java.lang.StringBuilder
use stubs::java::lang::StringBuilder::*;
// Java: import cn.wps.collections.RBBalancer
use util::collections::RBBalancer::*;

// Java: public abstract class IntervalNode extends PositionNode 
#[derive(Default)]
pub struct IntervalNode {
    // Java field: PositionNode super_;
    pub super_: PositionNode,
}

// Java: public abstract class IntervalNode extends PositionNode 
impl IntervalNode {
    // Java: public abstract int getLength();
    pub fn getLength(&mut self) -> i32 {
        panic!("IntervalNode::getLength not implemented");
    }

    // Java: public abstract void setLength(int var1);
    pub fn setLength(&mut self, var1: i32) {
        panic!("IntervalNode::setLength not implemented");
    }

    // Java: public String toString() 
    pub fn toString(&mut self) -> String {
      // Java: StringBuilder sb = new StringBuilder();
      let sb: String = StringBuilder::new();
      // Java: sb.append(this.getClass().getSimpleName());
      sb.append(self.getClass().getSimpleName());
      // Java: if (this.isHead())
      if self.isHead() {
         // Java: sb.append("(head)");
         sb.append("(head)");
      }
      // Java: if (this.isRoot())
      if self.isRoot() {
         // Java: sb.append("(root)");
         sb.append("(root)");
      }
      // Java: if (RBBalancer.isRed(this))
      if RBBalancer::isRed(self) {
         // Java: sb.append("\ncolor = red");
         sb.append("\ncolor = red");
      }
      // Java: else
      else {
         // Java: sb.append("\ncolor = black");
         sb.append("\ncolor = black");
      }
      // Java: if (!this.isHead())
      if !self.super_.isHead() {
         // Java: sb.append("\nposition = " + this.position());
         sb.append("\nposition = " + self.super_.position());
         // Java: sb.append("\nlength = " + this.getLength());
         sb.append("\nlength = " + self.getLength());
      }
      // Java: sb.append("\nchildren = (");
      sb.append("\nchildren = (");
      // Java: if (this.left != null)
      if self.left.is_some() {
         // Java: sb.append("" + ((IntervalNode)this.left).position());
         sb.append("" + (self.left).downcast_ref::<IntervalNode>().unwrap().position());
      }
      // Java: else
      else {
         // Java: sb.append("null");
         sb.append("null");
      }
      // Java: sb.append(" ,");
      sb.append(" ,");
      // Java: if (this.right != null)
      if self.right.is_some() {
         // Java: sb.append("" + ((IntervalNode)this.right).position());
         sb.append("" + (self.right).downcast_ref::<IntervalNode>().unwrap().position());
      }
      // Java: else
      else {
         // Java: sb.append("null");
         sb.append("null");
      }
      // Java: sb.append(")");
      sb.append(")");
      // Java: sb.append("\n");
      sb.append("\n");
      // Java: return sb.toString();
      return sb.toString();
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
