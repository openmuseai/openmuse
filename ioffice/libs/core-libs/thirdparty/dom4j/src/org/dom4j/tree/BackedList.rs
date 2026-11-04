// Java: import java.util.List
// Java: import org.dom4j.Node

// Java: public class BackedList extends Node
pub struct BackedList<T> {
    // Java field: List<Node> branchContent
    branchContent: Vec<T>,
    // Java field: AbstractBranch branch
    items: Vec<T>,
}

// Java: public class BackedList extends Node
impl<T: Clone> BackedList<T> {
    // Java: BackedList(AbstractBranch branch, List<Node> branchContent)
    pub fn new(branch_content: Vec<T>) -> Self {
        // Java: super();
        // Java: this.branch = branch;
        // Java: this.branchContent = branchContent;
        BackedList {
            branchContent: branch_content,
            items: Vec::new(),
        }
    }

    // Java: BackedList(AbstractBranch branch, List<Node> branchContent, List<T> initialContent)
    pub fn new_with_content(branch_content: Vec<T>, initial_content: Vec<T>) -> Self {
        // Java: this(branch, branchContent);
        // Java: this.addAll(initialContent);
        BackedList {
            branchContent: branch_content,
            items: initial_content,
        }
    }

    // Java: boolean add(T node)
    pub fn add(&mut self, node: T) -> bool {
        // Java: this.branch.addNode(node);
        // Java: return super.add(node);
        self.items.push(node);
        true
    }

    // Java: void add(int index, T node)
    pub fn add_at(&mut self, index: usize, node: T) {
        // Java: if (index < 0)
        // Java: throw new IndexOutOfBoundsException("Index value: " + index + " is less than zero");
        // Java: else if (index > this.size())
        // Java: throw new IndexOutOfBoundsException("Index value: " + index + " cannot be greater than the size: " + this.size());
        if index > self.items.len() {
            panic!("Index value: {} is out of bounds", index);
        }
        // Java: else
        {
            // Java: int realIndex;
            // Java: if (this.size() == 0)
            if self.items.is_empty() {
                // Java: realIndex = this.branchContent.size();
            }
            // Java: else if (index < this.size())
            else if index < self.items.len() {
                // Java: realIndex = this.branchContent.indexOf(this.get(index));
            }
            // Java: else
            else {
                // Java: realIndex = this.branchContent.indexOf(this.get(this.size() - 1)) + 1;
            }
            // Java: this.branch.addNode(realIndex, node);
            // Java: super.add(index, node);
            self.items.insert(index, node);
        }
    }

    // Java: T set(int index, T node)
    pub fn set(&mut self, index: usize, node: T) -> T {
        // Java: int realIndex = this.branchContent.indexOf(this.get(index));
        // Java: if (realIndex < 0)
        // Java: realIndex = index == 0 ? 0 : Integer.MAX_VALUE;
        // Java: if (realIndex < this.branchContent.size())
        // Java: this.branch.removeNode((Node)this.get(index));
        // Java: this.branch.addNode(realIndex, node);
        // Java: else
        // Java: this.branch.removeNode((Node)this.get(index));
        // Java: this.branch.addNode(node);
        // Java: this.branch.childAdded(node);
        // Java: return (T)(super.set(index, node));
        let old = self.items[index].clone();
        self.items[index] = node;
        old
    }

    // Java: boolean remove(Node node)
    pub fn remove(&mut self, node: &T) -> bool
    where T: PartialEq {
        // Java: this.branch.removeNode(node);
        // Java: return super.remove(node);
        if let Some(pos) = self.items.iter().position(|x| x == node) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    // Java: T remove(int index)
    pub fn remove_at(&mut self, index: usize) -> T {
        // Java: T node = (T)(super.remove(index));
        // Java: if (node != null)
        // Java: this.branch.removeNode(node);
        // Java: return node;
        self.items.remove(index)
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: for(T node : this)
        // Java: this.branchContent.remove(node);
        // Java: this.branch.childRemoved(node);
        // Java: super.clear();
        self.items.clear();
    }

    // Java: void addLocal(T node)
    pub fn addLocal(&mut self, node: T) {
        // Java: super.add(node);
        self.items.push(node);
    }
}
