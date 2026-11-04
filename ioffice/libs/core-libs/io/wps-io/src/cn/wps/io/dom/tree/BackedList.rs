// Java: import cn.wps.io.dom.Node
// Java: import java.util.List
use super::AbstractBranch::AbstractBranch;

// Java: public class BackedList extends Node
pub struct BackedList {
    // Java field: List<Node> branchContent
    branchContent: Vec<Box<dyn std::any::Any>>,
    // Java field: AbstractBranch branch
    branch: Option<Box<AbstractBranch>>,
}

// Java: public class BackedList extends Node
impl BackedList {
    // Java: BackedList(AbstractBranch branch, List<Node> branchContent)
    pub fn new() -> Self {
      // Java: super();
      todo!();
      // Java: this.branch = branch;
      todo!();
      // Java: this.branchContent = branchContent;
      todo!();
    }

    // Java: BackedList(AbstractBranch branch, List<Node> branchContent, List<T> initialContent)
    pub fn new_1() -> Self {
      // Java: this(branch, branchContent);
      todo!();
      // Java: this.addAll(initialContent);
      todo!();
    }

    // Java: boolean add(T node)
    pub fn add(&mut self) -> bool {
      // Java: this.branch.addNode(node);
      todo!();
      // Java: return super.add(node);
      todo!();
    }

    // Java: void add(int index, T node)
    pub fn add_2(&mut self) {
      // Java: if (index < 0)
      todo!();
      {
         // Java: throw new IndexOutOfBoundsException("Index value: " + index + " is less than zero");
      todo!();
      }
      // Java: else if (index > this.size())
      todo!();
      {
         // Java: throw new IndexOutOfBoundsException("Index value: " + index + " cannot be greater than the size: " + this.size());
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: int realIndex;
      todo!();
         // Java: if (this.size() == 0)
      todo!();
         {
            // Java: realIndex = this.branchContent.size();
      todo!();
         }
         // Java: else if (index < this.size())
      todo!();
         {
            // Java: realIndex = this.branchContent.indexOf(this.get(index));
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: realIndex = this.branchContent.indexOf(this.get(this.size() - 1)) + 1;
      todo!();
         }
         // Java: this.branch.addNode(realIndex, node);
      todo!();
         // Java: super.add(index, node);
      todo!();
      }
    }

    // Java: T set(int index, T node)
    pub fn set(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: int realIndex = this.branchContent.indexOf(this.get(index));
      todo!();
      // Java: if (realIndex < 0)
      todo!();
      {
         // Java: realIndex = index == 0 ? 0 : Integer.MAX_VALUE;
      todo!();
      }
      // Java: if (realIndex < this.branchContent.size())
      todo!();
      {
         // Java: this.branch.removeNode((Node)this.get(index));
      todo!();
         // Java: this.branch.addNode(realIndex, node);
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: this.branch.removeNode((Node)this.get(index));
      todo!();
         // Java: this.branch.addNode(node);
      todo!();
      }
      // Java: this.branch.childAdded(node);
      todo!();
      // Java: return (T)(super.set(index, node));
      todo!();
    }

    // Java: boolean remove(Node node)
    pub fn remove(&mut self) -> bool {
      // Java: this.branch.removeNode(node);
      todo!();
      // Java: return super.remove(node);
      todo!();
    }

    // Java: T remove(int index)
    pub fn remove_2(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: T node = (T)(super.remove(index));
      todo!();
      // Java: if (node != null)
      todo!();
      {
         // Java: this.branch.removeNode(node);
      todo!();
      }
      // Java: return node;
      todo!();
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: for(T node : this)
      todo!();
      {
         // Java: this.branchContent.remove(node);
      todo!();
         // Java: this.branch.childRemoved(node);
      todo!();
      }
      // Java: super.clear();
      todo!();
    }

    // Java: void addLocal(T node)
    pub fn addLocal(&mut self) {
      // Java: super.add(node);
      todo!();
    }

}
