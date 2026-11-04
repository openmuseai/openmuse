// Java: import java.util.ArrayList
use std::ptr;
use super::Node::Node;
use super::ITreeBalancer::ITreeBalancer;
use super::ITreeExtension::ITreeExtension;

// Java: public class Tree
pub struct Tree {
    // Java field: ITreeBalancer mBalancer
    pub(crate) mBalancer: Option<Box<dyn ITreeBalancer>>,
    // Java field: ArrayList<Integer> mExtensionMasks
    mExtensionMasks: Vec<i32>,
    // Java field: ArrayList<ITreeExtension> mExtensions
    mExtensions: Vec<Box<dyn ITreeExtension>>,
    // Java field: Node mHead
    pub mHead: *mut Node,
    // Java field: int mSize
    pub(crate) mSize: i32,
}

// Java: public class Iterator1
pub struct Iterator1 {
    // Java field: Node mNode
    pub(crate) mNode: *mut Node,
    // Java field: Tree mTree
    pub(crate) mTree: *mut Tree,
}

// Java: public class Tree
impl Tree {
    // Java: static Node leftmost(Node x)
    pub fn leftmost_static(mut x: *mut Node) -> *mut Node {
      // Java: while(x.left != null)
      unsafe {
        while !(*x).left.is_null()
        {
           // Java: x = x.left;
           x = (*x).left;
        }
      }
      // Java: return x;
      x
    }

    // Java: static Node maximum(Node x)
    pub fn maximum(x: *mut Node) -> *mut Node {
      // Java: return rightmost(x);
      Self::rightmost_static(x)
    }

    // Java: static Node minimum(Node x)
    pub fn minimum(x: *mut Node) -> *mut Node {
      // Java: return leftmost(x);
      Self::leftmost_static(x)
    }

    // Java: static Node rightmost(Node x)
    pub fn rightmost_static(mut x: *mut Node) -> *mut Node {
      // Java: while(x.right != null)
      unsafe {
        while !(*x).right.is_null()
        {
           // Java: x = x.right;
           x = (*x).right;
        }
      }
      // Java: return x;
      x
    }

    // Java: Tree(Node head, ITreeBalancer pBalancer)
    pub fn new(head: *mut Node, p_balancer: Box<dyn ITreeBalancer>) -> Self {
      // Java: super();
      // Java: this.mHead = head;
      // Java: this.resetHead();
      // Java: this.mBalancer = pBalancer;
      let mut tree = Self {
        mBalancer: Some(p_balancer),
        mExtensionMasks: Vec::with_capacity(5),
        mExtensions: Vec::with_capacity(5),
        mHead: head,
        mSize: 0,
      };
      tree.resetHead();
      // Java: this.mBalancer.onCreateHead(this, head);
      let tree_ptr = &mut tree as *mut Tree as *mut u8;
      let head_u8 = head as *mut u8;
      tree.mBalancer.as_mut().unwrap().onCreateHead(tree_ptr, head_u8);
      tree
    }

    // Java: void addExtension(ITreeExtension pExtension, int mask)
    pub fn addExtension(&mut self, p_extension: Box<dyn ITreeExtension>, mask: i32) {
      // Java: this.mExtensions.add(pExtension);
      self.mExtensions.push(p_extension);
      // Java: this.mExtensionMasks.add(mask);
      self.mExtensionMasks.push(mask);
      // Java: if ((mask & 1) != 0)
      if (mask & 1) != 0
      {
         // Java: pExtension.onCreateHead(this, this.mHead);
         let idx = self.mExtensions.len() - 1;
         let tree_ptr = self as *mut Tree as *mut u8;
         let head_u8 = self.mHead as *mut u8;
         self.mExtensions[idx].onCreateHead(tree_ptr, head_u8);
      }
    }

    // Java: void resetHead(Node head)
    pub fn resetHead_with(&mut self, head: *mut Node) {
      // Java: this.mHead = head;
      self.mHead = head;
      // Java: this.resetHead();
      self.resetHead();
      // Java: int n = this.mExtensions.size();
      let n = self.mExtensions.len();
      // Java: for(int i = 0; i < n; ++i)
      for i in 0..n
      {
         // Java: int mask = (Integer)this.mExtensionMasks.get(i);
         let mask = self.mExtensionMasks[i];
         // Java: if ((mask & 1) != 0)
         if (mask & 1) != 0
         {
            // Java: ITreeExtension pExtension = (ITreeExtension)this.mExtensions.get(i);
            // Java: pExtension.onCreateHead(this, this.mHead);
            let tree_ptr = self as *mut Tree as *mut u8;
            let head_u8 = self.mHead as *mut u8;
            self.mExtensions[i].onCreateHead(tree_ptr, head_u8);
         }
      }
    }

    // Java: void assignRoot(Node pRoot)
    pub fn assignRoot(&mut self, p_root: *mut Node) {
      // Java: this.resetHead();
      self.resetHead();
      // Java: if (pRoot != null)
      if !p_root.is_null()
      {
         unsafe {
           // Java: this.mHead.left = leftmost(pRoot);
           (*self.mHead).left = Self::leftmost_static(p_root);
           // Java: this.mHead.right = rightmost(pRoot);
           (*self.mHead).right = Self::rightmost_static(p_root);
           // Java: this.mHead.parent = pRoot;
           (*self.mHead).parent = p_root;
           // Java: pRoot.parent = this.mHead;
           (*p_root).parent = self.mHead;
         }
      }
    }

    // Java: void binaryTreeInsertLeft(Node pNode, Node pNew, int arg1, Object arg2)
    pub(crate) fn binaryTreeInsertLeft(&mut self, mut pnode: *mut Node, p_new: *mut Node, _arg1: i32, _arg2: Option<&dyn std::any::Any>) {
      unsafe {
        // Java: Node head = this.mHead;
        let head = self.mHead;
        // Java: pNew.left = null;
        (*p_new).left = ptr::null_mut();
        // Java: pNew.right = null;
        (*p_new).right = ptr::null_mut();
        // Java: if (pNode == head)
        if pnode == head
        {
           // Java: if (head.parent == null)
           if (*head).parent.is_null()
           {
              // Java: head.parent = pNew;
              (*head).parent = p_new;
              // Java: pNew.parent = head;
              (*p_new).parent = head;
              // Java: head.left = pNew;
              (*head).left = p_new;
              // Java: head.right = pNew;
              (*head).right = p_new;
           }
           // Java: else
           else
           {
              // Java: head.right.right = pNew;
              (*(*head).right).right = p_new;
              // Java: pNew.parent = head.right;
              (*p_new).parent = (*head).right;
              // Java: head.right = pNew;
              (*head).right = p_new;
           }
        }
        // Java: else if (pNode.left == null)
        else if (*pnode).left.is_null()
        {
           // Java: pNode.left = pNew;
           (*pnode).left = p_new;
           // Java: pNew.parent = pNode;
           (*p_new).parent = pnode;
           // Java: if (head.left == pNode)
           if (*head).left == pnode
           {
              // Java: head.left = pNew;
              (*head).left = p_new;
           }
        }
        // Java: else
        else
        {
           // Java: pNode = this.treePrev(pNode);
           pnode = self.treePrev(pnode);
           // Java: pNode.right = pNew;
           (*pnode).right = p_new;
           // Java: pNew.parent = pNode;
           (*p_new).parent = pnode;
        }
      }
    }

    // Java: void binaryTreeInsertRight(Node pNode, Node pNew, int arg1, Object arg2)
    pub(crate) fn binaryTreeInsertRight(&mut self, mut pnode: *mut Node, p_new: *mut Node, _arg1: i32, _arg2: Option<&dyn std::any::Any>) {
      unsafe {
        // Java: Node head = this.mHead;
        let head = self.mHead;
        // Java: pNew.left = null;
        (*p_new).left = ptr::null_mut();
        // Java: pNew.right = null;
        (*p_new).right = ptr::null_mut();
        // Java: if (pNode == head)
        if pnode == head
        {
           // Java: if (head.parent == null)
           if (*head).parent.is_null()
           {
              // Java: head.parent = pNew;
              (*head).parent = p_new;
              // Java: pNew.parent = pNode;
              (*p_new).parent = pnode;
              // Java: head.left = pNew;
              (*head).left = p_new;
              // Java: head.right = pNew;
              (*head).right = p_new;
           }
           // Java: else
           else
           {
              // Java: head.left.left = pNew;
              (*(*head).left).left = p_new;
              // Java: pNew.parent = head.left;
              (*p_new).parent = (*head).left;
              // Java: head.left = pNew;
              (*head).left = p_new;
           }
        }
        // Java: else if (pNode.right == null)
        else if (*pnode).right.is_null()
        {
           // Java: pNode.right = pNew;
           (*pnode).right = p_new;
           // Java: pNew.parent = pNode;
           (*p_new).parent = pnode;
           // Java: if (head.right == pNode)
           if (*head).right == pnode
           {
              // Java: head.right = pNew;
              (*head).right = p_new;
           }
        }
        // Java: else
        else
        {
           // Java: pNode = this.treeNext(pNode);
           pnode = self.treeNext(pnode);
           // Java: pNode.left = pNew;
           (*pnode).left = p_new;
           // Java: pNew.parent = pNode;
           (*p_new).parent = pnode;
        }
      }
    }

    // Java: void doSwap(Node z, Node y)
    fn doSwap(&mut self, z: *mut Node, y: *mut Node) {
      unsafe {
        // Java: z.left.parent = y;
        (*(*z).left).parent = y;
        // Java: y.left = z.left;
        (*y).left = (*z).left;
        // Java: z.left = null;
        (*z).left = ptr::null_mut();
        // Java: Node z_parent = z.parent;
        let z_parent = (*z).parent;
        // Java: Node z_right = z.right;
        let z_right = (*z).right;
        // Java: Node y_right = y.right;
        let y_right = (*y).right;
        // Java: if (y == z_right)
        if y == z_right
        {
           // Java: z.parent = y;
           (*z).parent = y;
           // Java: y.right = z;
           (*y).right = z;
           // Java: y.parent = z_parent;
           (*y).parent = z_parent;
        }
        // Java: else
        else
        {
           // Java: z.parent = y.parent;
           (*z).parent = (*y).parent;
           // Java: y.parent.left = z;
           (*(*y).parent).left = z;
           // Java: y.right = z_right;
           (*y).right = z_right;
           // Java: z_right.parent = y;
           (*z_right).parent = y;
           // Java: y.parent = z_parent;
           (*y).parent = z_parent;
        }
        // Java: if (y_right != null)
        if !y_right.is_null()
        {
           // Java: y_right.parent = z;
           (*y_right).parent = z;
        }
        // Java: z.right = y_right;
        (*z).right = y_right;
        // Java: if (this.mHead.parent == z)
        if (*self.mHead).parent == z
        {
           // Java: this.mHead.parent = y;
           (*self.mHead).parent = y;
        }
        // Java: else if (z_parent.left == z)
        else if (*z_parent).left == z
        {
           // Java: z_parent.left = y;
           (*z_parent).left = y;
        }
        // Java: else
        else
        {
           // Java: z_parent.right = y;
           (*z_parent).right = y;
        }
      }
    }

    // Java: Node erase(Node z)
    pub fn erase(&mut self, z: *mut Node) -> *mut Node {
      unsafe {
        // Java: Node head = this.mHead;
        let head = self.mHead;
        // Java: if (z == head)
        if z == head
        {
           // Java: return z;
           return z;
        }
        // Java: else
        else
        {
           // Java: Node swp = z;
           let mut swp = z;
           // Java: if (z.left != null && z.right != null)
           if !(*z).left.is_null() && !(*z).right.is_null()
           {
              // Java: swp = minimum(z.right);
              swp = Self::minimum((*z).right);
              // Java: this.fireBeforeSwap(z, swp);
              self.fireBeforeSwap(z, swp);
              // Java: this.doSwap(z, swp);
              self.doSwap(z, swp);
           }
           // Java: this.fireBeforeErase(z, swp);
           self.fireBeforeErase(z, swp);
           // Java: Node x = null;
           let mut x: *mut Node = ptr::null_mut();
           // Java: Node x_parent = null;
           // Java: if (z.left == null)
           if (*z).left.is_null()
           {
              // Java: x = z.right;
              x = (*z).right;
           }
           // Java: else
           else
           {
              // Java: x = z.left;
              x = (*z).left;
           }
           // Java: x_parent = z.parent;
           let x_parent = (*z).parent;
           // Java: if (x != null)
           if !x.is_null()
           {
              // Java: x.parent = z.parent;
              (*x).parent = (*z).parent;
           }
           // Java: if (head.parent == z)
           if (*head).parent == z
           {
              // Java: head.parent = x;
              (*head).parent = x;
           }
           // Java: else if (z.parent.left == z)
           else if (*(*z).parent).left == z
           {
              // Java: z.parent.left = x;
              (*(*z).parent).left = x;
           }
           // Java: else
           else
           {
              // Java: z.parent.right = x;
              (*(*z).parent).right = x;
           }
           // Java: if (head.left == z)
           if (*head).left == z
           {
              // Java: if (z.right == null)
              if (*z).right.is_null()
              {
                 // Java: head.left = z.parent;
                 (*head).left = (*z).parent;
              }
              // Java: else
              else
              {
                 // Java: head.left = minimum(x);
                 (*head).left = Self::minimum(x);
              }
           }
           // Java: if (head.right == z)
           if (*head).right == z
           {
              // Java: if (z.left == null)
              if (*z).left.is_null()
              {
                 // Java: head.right = z.parent;
                 (*head).right = (*z).parent;
              }
              // Java: else
              else
              {
                 // Java: head.right = maximum(x);
                 (*head).right = Self::maximum(x);
              }
           }
           // Java: this.fireOnBinaryTreeErased(z, x, x_parent);
           self.fireOnBinaryTreeErased(z, x, x_parent);
           // Java: return z;
           z
        }
      }
    }

    // Java: void fireBeforeErase(Node z, Node swp)
    pub(crate) fn fireBeforeErase(&mut self, z: *mut Node, swp: *mut Node) {
      // Java: int n = this.mExtensions.size();
      let n = self.mExtensions.len();
      // Java: for(int i = 0; i < n; ++i)
      for i in 0..n
      {
         // Java: int mask = (Integer)this.mExtensionMasks.get(i);
         let mask = self.mExtensionMasks[i];
         // Java: if ((mask & 16) != 0)
         if (mask & 16) != 0
         {
            // Java: ITreeExtension ext = (ITreeExtension)this.mExtensions.get(i);
            // Java: ext.beforeErase(this, z, swp);
            let tree_ptr = self as *mut Tree as *mut u8;
            self.mExtensions[i].beforeErase(tree_ptr, z as *mut u8, swp as *mut u8);
         }
      }
    }

    // Java: void fireBeforeRotateLeft(Node x, Node y)
    pub(crate) fn fireBeforeRotateLeft(&mut self, x: *mut Node, y: *mut Node) {
      // Java: int n = this.mExtensions.size();
      let n = self.mExtensions.len();
      // Java: for(int i = 0; i < n; ++i)
      for i in 0..n
      {
         // Java: int mask = (Integer)this.mExtensionMasks.get(i);
         let mask = self.mExtensionMasks[i];
         // Java: if ((mask & 32) != 0)
         if (mask & 32) != 0
         {
            // Java: ITreeExtension ext = (ITreeExtension)this.mExtensions.get(i);
            // Java: ext.beforeRotateLeft(this, x, y);
            let tree_ptr = self as *mut Tree as *mut u8;
            self.mExtensions[i].beforeRotateLeft(tree_ptr, x as *mut u8, y as *mut u8);
         }
      }
    }

    // Java: void fireBeforeRotateRight(Node y, Node x)
    pub(crate) fn fireBeforeRotateRight(&mut self, y: *mut Node, x: *mut Node) {
      // Java: int n = this.mExtensions.size();
      let n = self.mExtensions.len();
      // Java: for(int i = 0; i < n; ++i)
      for i in 0..n
      {
         // Java: int mask = (Integer)this.mExtensionMasks.get(i);
         let mask = self.mExtensionMasks[i];
         // Java: if ((mask & 32) != 0)
         if (mask & 32) != 0
         {
            // Java: ITreeExtension ext = (ITreeExtension)this.mExtensions.get(i);
            // Java: ext.beforeRotateRight(this, y, x);
            let tree_ptr = self as *mut Tree as *mut u8;
            self.mExtensions[i].beforeRotateRight(tree_ptr, y as *mut u8, x as *mut u8);
         }
      }
    }

    // Java: void fireBeforeSwap(Node z, Node y)
    pub(crate) fn fireBeforeSwap(&mut self, z: *mut Node, y: *mut Node) {
      // Java: int n = this.mExtensions.size();
      let n = self.mExtensions.len();
      // Java: for(int i = 0; i < n; ++i)
      for i in 0..n
      {
         // Java: int mask = (Integer)this.mExtensionMasks.get(i);
         let mask = self.mExtensionMasks[i];
         // Java: if ((mask & 8) != 0)
         if (mask & 8) != 0
         {
            // Java: ITreeExtension ext = (ITreeExtension)this.mExtensions.get(i);
            // Java: ext.beforeSwap(this, z, y);
            let tree_ptr = self as *mut Tree as *mut u8;
            self.mExtensions[i].beforeSwap(tree_ptr, z as *mut u8, y as *mut u8);
         }
      }
      // Java: this.mBalancer.beforeSwap(this, z, y);
      let tree_ptr = self as *mut Tree as *mut u8;
      self.mBalancer.as_mut().unwrap().beforeSwap(tree_ptr, z as *mut u8, y as *mut u8);
    }

    // Java: void fireOnBinaryTreeErased(Node z, Node x, Node x_parent)
    pub(crate) fn fireOnBinaryTreeErased(&mut self, z: *mut Node, x: *mut Node, x_parent: *mut Node) {
      // Java: --this.mSize;
      self.mSize -= 1;
      // Java: int n = this.mExtensions.size();
      let n = self.mExtensions.len();
      // Java: for(int i = 0; i < n; ++i)
      for i in 0..n
      {
         // Java: int mask = (Integer)this.mExtensionMasks.get(i);
         let mask = self.mExtensionMasks[i];
         // Java: if ((mask & 4) != 0)
         if (mask & 4) != 0
         {
            // Java: ITreeExtension ext = (ITreeExtension)this.mExtensions.get(i);
            // Java: ext.onBinaryTreeErased(this, z, x, x_parent);
            let tree_ptr = self as *mut Tree as *mut u8;
            self.mExtensions[i].onBinaryTreeErased(tree_ptr, z as *mut u8, x as *mut u8, x_parent as *mut u8);
         }
      }
      // Java: this.mBalancer.onBinaryTreeErased(this, z, x, x_parent);
      let tree_ptr = self as *mut Tree as *mut u8;
      self.mBalancer.as_mut().unwrap().onBinaryTreeErased(tree_ptr, z as *mut u8, x as *mut u8, x_parent as *mut u8);
    }

    // Java: void fireOnBinaryTreeInsertedLeft(Node pNode, Node pNew, int arg1, Object arg2)
    pub(crate) fn fireOnBinaryTreeInsertedLeft(&mut self, pnode: *mut Node, p_new: *mut Node, arg1: i32, arg2: Option<&dyn std::any::Any>) {
      // Java: ++this.mSize;
      self.mSize += 1;
      // Java: int n = this.mExtensions.size();
      let n = self.mExtensions.len();
      // Java: for(int i = 0; i < n; ++i)
      for i in 0..n
      {
         // Java: int mask = (Integer)this.mExtensionMasks.get(i);
         let mask = self.mExtensionMasks[i];
         // Java: if ((mask & 2) != 0)
         if (mask & 2) != 0
         {
            // Java: ITreeExtension ext = (ITreeExtension)this.mExtensions.get(i);
            // Java: ext.onBinaryTreeInsertedLeft(this, pNode, pNew, arg1, arg2);
            let tree_ptr = self as *mut Tree as *mut u8;
            self.mExtensions[i].onBinaryTreeInsertedLeft(tree_ptr, pnode as *mut u8, p_new as *mut u8, arg1, arg2);
         }
      }
      // Java: this.mBalancer.onBinaryTreeInserted(this, pNew);
      let tree_ptr = self as *mut Tree as *mut u8;
      self.mBalancer.as_mut().unwrap().onBinaryTreeInserted(tree_ptr, p_new as *mut u8);
    }

    // Java: void fireOnBinaryTreeInsertedRight(Node pNode, Node pNew, int arg1, Object arg2)
    pub(crate) fn fireOnBinaryTreeInsertedRight(&mut self, pnode: *mut Node, p_new: *mut Node, arg1: i32, arg2: Option<&dyn std::any::Any>) {
      // Java: ++this.mSize;
      self.mSize += 1;
      // Java: int n = this.mExtensions.size();
      let n = self.mExtensions.len();
      // Java: for(int i = 0; i < n; ++i)
      for i in 0..n
      {
         // Java: int mask = (Integer)this.mExtensionMasks.get(i);
         let mask = self.mExtensionMasks[i];
         // Java: if ((mask & 2) != 0)
         if (mask & 2) != 0
         {
            // Java: ITreeExtension ext = (ITreeExtension)this.mExtensions.get(i);
            // Java: ext.onBinaryTreeInsertedRight(this, pNode, pNew, arg1, arg2);
            let tree_ptr = self as *mut Tree as *mut u8;
            self.mExtensions[i].onBinaryTreeInsertedRight(tree_ptr, pnode as *mut u8, p_new as *mut u8, arg1, arg2);
         }
      }
      // Java: this.mBalancer.onBinaryTreeInserted(this, pNew);
      let tree_ptr = self as *mut Tree as *mut u8;
      self.mBalancer.as_mut().unwrap().onBinaryTreeInserted(tree_ptr, p_new as *mut u8);
    }

    // Java: ITreeBalancer getBalancer()
    pub fn getBalancer(&self) -> &dyn ITreeBalancer {
      // Java: return this.mBalancer;
      self.mBalancer.as_ref().unwrap().as_ref()
    }

    // Java: ITreeExtension[] getExtensions()
    pub fn getExtensions(&self) -> &[Box<dyn ITreeExtension>] {
      // Java: ITreeExtension[] p = new ITreeExtension[this.mExtensions.size()];
      // Java: return (ITreeExtension[])this.mExtensions.toArray(p);
      &self.mExtensions
    }

    // Java: Node getHead()
    pub fn getHead(&self) -> *mut Node {
      // Java: return this.mHead;
      self.mHead
    }

    // Java: Node getLeftRoot(Node pNode)
    pub fn getLeftRoot(&self, mut pnode: *mut Node) -> *mut Node {
      unsafe {
        // Java: Node pHead = this.mHead;
        let p_head = self.mHead;
        // Java: Node p;
        // Java: for(p = pNode.parent; p != pHead && pNode == p.left; p = p.parent)
        let mut p = (*pnode).parent;
        while p != p_head && pnode == (*p).left
        {
           // Java: pNode = p;
           pnode = p;
           p = (*p).parent;
        }
        // Java: return p;
        p
      }
    }

    // Java: Node getRoot()
    pub fn getRoot(&self) -> *mut Node {
      // Java: return this.mHead.parent;
      unsafe { (*self.mHead).parent }
    }

    // Java: void insertLeft(Node pos, Node pNew)
    pub fn insertLeft_simple(&mut self, pos: *mut Node, p_new: *mut Node) {
      // Java: this.insertLeft(pos, pNew, 0, null);
      self.insertLeft(pos, p_new, 0, None);
    }

    // Java: void insertLeft(Node pos, Node pNew, int arg1, Object arg2)
    pub fn insertLeft(&mut self, pos: *mut Node, p_new: *mut Node, arg1: i32, arg2: Option<&dyn std::any::Any>) {
      // Java: this.binaryTreeInsertLeft(pos, pNew, arg1, arg2);
      self.binaryTreeInsertLeft(pos, p_new, arg1, arg2);
      // Java: this.fireOnBinaryTreeInsertedLeft(pos, pNew, arg1, arg2);
      self.fireOnBinaryTreeInsertedLeft(pos, p_new, arg1, arg2);
    }

    // Java: void insertRight(Node pos, Node pNew)
    pub fn insertRight_simple(&mut self, pos: *mut Node, p_new: *mut Node) {
      // Java: this.insertRight(pos, pNew, 0, null);
      self.insertRight(pos, p_new, 0, None);
    }

    // Java: void insertRight(Node pos, Node pNew, int arg1, Object arg2)
    pub fn insertRight(&mut self, pos: *mut Node, p_new: *mut Node, arg1: i32, arg2: Option<&dyn std::any::Any>) {
      // Java: this.binaryTreeInsertRight(pos, pNew, arg1, arg2);
      self.binaryTreeInsertRight(pos, p_new, arg1, arg2);
      // Java: this.fireOnBinaryTreeInsertedRight(pos, pNew, arg1, arg2);
      self.fireOnBinaryTreeInsertedRight(pos, p_new, arg1, arg2);
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.mHead.left == this.mHead;
      unsafe { (*self.mHead).left == self.mHead }
    }

    // Java: Node leftmost()
    pub fn leftmost(&self) -> *mut Node {
      // Java: return this.mHead.left;
      unsafe { (*self.mHead).left }
    }

    // Java: void resetHead()
    fn resetHead(&mut self) {
      unsafe {
        // Java: this.mHead.setHead();
        (*self.mHead).setHead();
        // Java: this.mHead.left = this.mHead;
        (*self.mHead).left = self.mHead;
        // Java: this.mHead.right = this.mHead;
        (*self.mHead).right = self.mHead;
        // Java: this.mHead.parent = null;
        (*self.mHead).parent = ptr::null_mut();
        // Java: this.mSize = 0;
        self.mSize = 0;
      }
    }

    // Java: Node rightmost()
    pub fn rightmost(&self) -> *mut Node {
      // Java: return this.mHead.right;
      unsafe { (*self.mHead).right }
    }

    // Java: void rotateLeft(Node x, Node head)
    pub(crate) fn rotateLeft(&mut self, x: *mut Node, head: *mut Node) {
      unsafe {
        // Java: Node y = x.right;
        let y = (*x).right;
        // Java: this.fireBeforeRotateLeft(x, y);
        self.fireBeforeRotateLeft(x, y);
        // Java: x.right = y.left;
        (*x).right = (*y).left;
        // Java: if (y.left != null)
        if !(*y).left.is_null()
        {
           // Java: y.left.parent = x;
           (*(*y).left).parent = x;
        }
        // Java: Node p = x.parent;
        let p = (*x).parent;
        // Java: y.left = x;
        (*y).left = x;
        // Java: x.parent = y;
        (*x).parent = y;
        // Java: if (x == head.parent)
        if x == (*head).parent
        {
           // Java: head.parent = y;
           (*head).parent = y;
        }
        // Java: else if (x == p.left)
        else if x == (*p).left
        {
           // Java: p.left = y;
           (*p).left = y;
        }
        // Java: else
        else
        {
           // Java: p.right = y;
           (*p).right = y;
        }
        // Java: y.parent = p;
        (*y).parent = p;
      }
    }

    // Java: void rotateRight(Node y, Node head)
    pub(crate) fn rotateRight(&mut self, y: *mut Node, head: *mut Node) {
      unsafe {
        // Java: Node x = y.left;
        let x = (*y).left;
        // Java: this.fireBeforeRotateRight(y, x);
        self.fireBeforeRotateRight(y, x);
        // Java: y.left = x.right;
        (*y).left = (*x).right;
        // Java: if (x.right != null)
        if !(*x).right.is_null()
        {
           // Java: x.right.parent = y;
           (*(*x).right).parent = y;
        }
        // Java: Node p = y.parent;
        let p = (*y).parent;
        // Java: x.right = y;
        (*x).right = y;
        // Java: y.parent = x;
        (*y).parent = x;
        // Java: if (y == head.parent)
        if y == (*head).parent
        {
           // Java: head.parent = x;
           (*head).parent = x;
        }
        // Java: else if (y == p.left)
        else if y == (*p).left
        {
           // Java: p.left = x;
           (*p).left = x;
        }
        // Java: else
        else
        {
           // Java: p.right = x;
           (*p).right = x;
        }
        // Java: x.parent = p;
        (*x).parent = p;
      }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.mSize;
      self.mSize
    }

    // Java: static void toList(Node prev, Node root, Node next)
    pub fn toList_nodes(prev: *mut Node, root: *mut Node, next: *mut Node) {
      unsafe {
        // Java: if (root.left == null)
        if (*root).left.is_null()
        {
           // Java: prev.right = root;
           (*prev).right = root;
           // Java: root.left = prev;
           (*root).left = prev;
        }
        // Java: else
        else
        {
           // Java: toList(prev, root.left, root);
           Self::toList_nodes(prev, (*root).left, root);
        }
        // Java: if (root.right == null)
        if (*root).right.is_null()
        {
           // Java: root.right = next;
           (*root).right = next;
           // Java: next.left = root;
           (*next).left = root;
        }
        // Java: else
        else
        {
           // Java: toList(root, root.right, next);
           Self::toList_nodes(root, (*root).right, next);
        }
      }
    }

    // Java: static void toList(Tree tree)
    pub fn toList(tree: &mut Tree) {
      unsafe {
        // Java: Node head = tree.mHead;
        let head = tree.mHead;
        // Java: Node root = head.parent;
        let root = (*head).parent;
        // Java: if (root != null)
        if !root.is_null()
        {
           // Java: toList(head, root, head);
           Self::toList_nodes(head, root, head);
           // Java: Node t = head.left;
           let t = (*head).left;
           // Java: head.left = head.right;
           (*head).left = (*head).right;
           // Java: head.right = t;
           (*head).right = t;
        }
      }
    }

    // Java: Node treePrev(Node node)
    pub fn treePrev(&self, mut node: *mut Node) -> *mut Node {
      unsafe {
        // Java: if (node.isHead())
        if (*node).isHead()
        {
           // Java: return node.right;
           return (*node).right;
        }
        // Java: else
        else
        {
           // Java: if (node.left != null)
           if !(*node).left.is_null()
           {
              // Java: for(node = node.left; node.right != null; node = node.right)
              node = (*node).left;
              while !(*node).right.is_null()
              {
                 node = (*node).right;
              }
           }
           // Java: else
           else
           {
              // Java: Node y;
              // Java: for(y = node.parent; node == y.left; y = y.parent)
              let mut y = (*node).parent;
              while node == (*y).left
              {
                 // Java: node = y;
                 node = y;
                 y = (*y).parent;
              }
              // Java: if (node.left != y)
              if (*node).left != y
              {
                 // Java: node = y;
                 node = y;
              }
           }
           // Java: return node;
           node
        }
      }
    }

    // Java: Node treeNext(Node node)
    pub fn treeNext(&self, mut node: *mut Node) -> *mut Node {
      unsafe {
        // Java: if (node.isHead())
        if (*node).isHead()
        {
           // Java: return node.left;
           return (*node).left;
        }
        // Java: else
        else
        {
           // Java: if (node.right != null)
           if !(*node).right.is_null()
           {
              // Java: for(node = node.right; node.left != null; node = node.left)
              node = (*node).right;
              while !(*node).left.is_null()
              {
                 node = (*node).left;
              }
           }
           // Java: else
           else
           {
              // Java: Node y;
              // Java: for(y = node.parent; node == y.right; y = y.parent)
              let mut y = (*node).parent;
              while node == (*y).right
              {
                 // Java: node = y;
                 node = y;
                 y = (*y).parent;
              }
              // Java: if (node.right != y)
              if (*node).right != y
              {
                 // Java: node = y;
                 node = y;
              }
           }
           // Java: return node;
           node
        }
      }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: this.mHead = null;
      self.mHead = ptr::null_mut();
      // Java: this.mBalancer = null;
      self.mBalancer = None;
    }

}

// Java: public class Iterator1
impl Iterator1 {
    // Java: Iterator1(Tree tree, Node node)
    pub fn new(tree: *mut Tree, node: *mut Node) -> Self {
         // Java: super();
         Self {
           // Java: this.mTree = tree;
           mTree: tree,
           // Java: this.mNode = node;
           mNode: node,
         }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
         // Java: return this.mNode != this.mTree.mHead;
         unsafe { self.mNode != (*self.mTree).mHead }
    }

    // Java: boolean hasPrev()
    pub fn hasPrev(&self) -> bool {
         // Java: return this.mNode != this.mTree.mHead;
         unsafe { self.mNode != (*self.mTree).mHead }
    }

    // Java: Node next()
    pub fn next(&mut self) -> *mut Node {
         // Java: Node current = this.mNode;
         let current = self.mNode;
         // Java: this.mNode = this.mNode.getNext();
         unsafe { self.mNode = (*self.mNode).getNext(); }
         // Java: return current;
         current
    }

    // Java: Node prev()
    pub fn prev(&mut self) -> *mut Node {
         // Java: Node current = this.mNode;
         let current = self.mNode;
         // Java: this.mNode = this.mNode.getPrev();
         unsafe { self.mNode = (*self.mNode).getPrev(); }
         // Java: return current;
         current
    }

    // Java: void remove()
    pub fn remove(&mut self) {
         unsafe {
           // Java: Node next = this.mNode.getNext();
           let next = (*self.mNode).getNext();
           // Java: this.mTree.erase(this.mNode);
           (*self.mTree).erase(self.mNode);
           // Java: this.mNode = next;
           self.mNode = next;
         }
    }

}
