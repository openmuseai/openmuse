// Java: import java.util.AbstractList
// Java: import java.util.Collection
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.dom4j.Node
// use crate::org::dom4j::Node;

// Java: public class ContentListFacade extends Node
pub struct ContentListFacade {
    // Java field: List<T> branchContent
    pub(crate) branchContent: Vec<i32>,
    // Java field: AbstractBranch branch
    branch: i32,
}

// Java: public class ContentListFacade extends Node
impl ContentListFacade {
    // Java: ContentListFacade(AbstractBranch branch, List<T> branchContent)
    pub fn new(_branch: i32, _branchContent: Vec<i32>) -> Self {
      // Java: super();
      // Java: this.branch = branch;
      // Java: this.branchContent = branchContent;
      Self { branch: _branch, branchContent: _branchContent }
    }

    // Java: boolean add(T node)
    pub fn add(&mut self, _node: i32) -> bool {
      // Java: this.branch.childAdded(node);
      // Java: return this.branchContent.add(node);
      self.branchContent.push(_node);
      true
    }

    // Java: void add(int index, T node)
    pub fn add_at(&mut self, _index: i32, _node: i32) {
      // Java: this.branch.childAdded(node);
      // Java: this.branchContent.add(index, node);
      self.branchContent.insert(_index as usize, _node);
    }

    // Java: T set(int index, T node)
    pub fn set(&mut self, _index: i32, _node: i32) -> i32 {
      // Java: this.branch.childAdded(node);
      // Java: return (T)(this.branchContent.set(index, node));
      let old = self.branchContent[_index as usize];
      self.branchContent[_index as usize] = _node;
      old
    }

    // Java: boolean remove(T node)
    pub fn remove(&mut self, _node: i32) -> bool {
      // Java: this.branch.childRemoved(node);
      // Java: return this.branchContent.remove(node);
      if let Some(pos) = self.branchContent.iter().position(|&x| x == _node) {
          self.branchContent.remove(pos);
          true
      } else {
          false
      }
    }

    // Java: T remove(int index)
    pub fn remove_at(&mut self, _index: i32) -> i32 {
      // Java: T node = (T)(this.branchContent.remove(index));
      // Java: if (node != null)
      {
         // Java: this.branch.childRemoved(node);
      }
      // Java: return node;
      self.branchContent.remove(_index as usize)
    }

    // Java: boolean addAll(Collection<? extends T> collection)
    pub fn addAll(&mut self, _collection: &[i32]) -> bool {
      // Java: int count = this.branchContent.size();
      // Java: for(Iterator<? extends T> iter = collection.iterator(); iter.hasNext(); ++count)
      {
         // Java: this.add(iter.next());
      }
      // Java: return count == this.branchContent.size();
      for &item in _collection {
          self.branchContent.push(item);
      }
      true
    }

    // Java: boolean addAll(int index, Collection<? extends T> collection)
    pub fn addAll_at(&mut self, _index: i32, _collection: &[i32]) -> bool {
      // Java: int count = this.branchContent.size();
      // Java: for(Iterator<? extends T> iter = collection.iterator(); iter.hasNext(); --count)
      {
         // Java: this.add(index++, iter.next());
      }
      // Java: return count == this.branchContent.size();
      let mut idx = _index as usize;
      for &item in _collection {
          self.branchContent.insert(idx, item);
          idx += 1;
      }
      true
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: for(T node : this)
      {
         // Java: this.branch.childRemoved(node);
      }
      // Java: this.branchContent.clear();
      self.branchContent.clear();
    }

    // Java: boolean removeAll(Collection<?> c)
    pub fn removeAll(&mut self, _c: &[i32]) -> bool {
      // Java: for(Object object : c)
      {
         // Java: if (object instanceof Node)
         {
            // Java: this.branch.childRemoved((Node)object);
         }
      }
      // Java: return this.branchContent.removeAll(c);
      self.branchContent.retain(|x| !_c.contains(x));
      true
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.branchContent.size();
      self.branchContent.len() as i32
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.branchContent.isEmpty();
      self.branchContent.is_empty()
    }

    // Java: boolean contains(T node)
    pub fn contains(&self, _node: i32) -> bool {
      // Java: return this.branchContent.contains(node);
      self.branchContent.contains(&_node)
    }

    // Java: Node[] toArray()
    pub fn toArray(&self) -> Vec<i32> {
      // Java: return (Node[])this.branchContent.toArray(new Node[this.branchContent.size()]);
      self.branchContent.clone()
    }

    // Java: Node[] toArray(Node[] a)
    pub fn toArray_into(&self, _a: &mut Vec<i32>) -> Vec<i32> {
      // Java: return (Node[])this.branchContent.toArray(a);
      self.branchContent.clone()
    }

    // Java: boolean containsAll(Collection<?> c)
    pub fn containsAll(&self, _c: &[i32]) -> bool {
      // Java: return this.branchContent.containsAll(c);
      _c.iter().all(|item| self.branchContent.contains(item))
    }

    // Java: T get(int index)
    pub fn get(&self, _index: i32) -> i32 {
      // Java: return (T)(this.branchContent.get(index));
      self.branchContent[_index as usize]
    }

    // Java: int indexOf(T node)
    pub fn indexOf(&self, _node: i32) -> i32 {
      // Java: return this.branchContent.indexOf(node);
      self.branchContent.iter().position(|&x| x == _node).map(|p| p as i32).unwrap_or(-1)
    }

    // Java: int lastIndexOf(T node)
    pub fn lastIndexOf(&self, _node: i32) -> i32 {
      // Java: return this.branchContent.lastIndexOf(node);
      self.branchContent.iter().rposition(|&x| x == _node).map(|p| p as i32).unwrap_or(-1)
    }

    // Java: List<T> getBackingList()
    pub fn getBackingList(&self) -> &Vec<i32> {
      // Java: return this.branchContent;
      &self.branchContent
    }

}
