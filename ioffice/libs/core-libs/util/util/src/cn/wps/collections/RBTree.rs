#![allow(non_snake_case, dead_code, unused_variables)]

use std::ptr;
use super::RBNode::RBNode;

// Java: public class RBTree
pub struct RBTree {
    // Java field: RBNode mHead
    pub(crate) mHead: *mut RBNode,
}

// Java: public class RBTree
impl RBTree {
    // Java: RBTree(RBNode head)
    pub fn new(head: *mut RBNode) -> Self {
        // Java: super();
        // Java: this.mHead = head;
        // Java: this.mHead.setRed();
        unsafe { (*head).setRed(); }
        // Java: this.mHead.left = this.mHead.right = this.mHead;
        unsafe {
            (*head).left = head;
            (*head).right = head;
        }
        // Java: this.mHead.parent = null;
        unsafe { (*head).parent = ptr::null_mut(); }
        Self { mHead: head }
    }

    // Java: RBNode getHead()
    pub fn getHead(&self) -> *mut RBNode {
        // Java: return this.mHead;
        self.mHead
    }

    // Java: RBNode getRoot()
    pub fn getRoot(&self) -> *mut RBNode {
        // Java: return this.mHead.parent;
        unsafe { (*self.mHead).parent }
    }

    // Java: static RBNode leftmost(RBNode x)
    pub fn leftmost_static(mut x: *mut RBNode) -> *mut RBNode {
        // Java: while(x.left != null)
        while !unsafe { (*x).left }.is_null()
        {
            // Java: x = x.left;
            x = unsafe { (*x).left };
        }
        // Java: return x;
        x
    }

    // Java: static RBNode rightmost(RBNode x)
    pub fn rightmost_static(mut x: *mut RBNode) -> *mut RBNode {
        // Java: while(x.right != null)
        while !unsafe { (*x).right }.is_null()
        {
            // Java: x = x.right;
            x = unsafe { (*x).right };
        }
        // Java: return x;
        x
    }

    // Java: static RBNode minimum(RBNode x)
    pub fn minimum(x: *mut RBNode) -> *mut RBNode {
        // Java: return leftmost(x);
        Self::leftmost_static(x)
    }

    // Java: static RBNode maximum(RBNode x)
    pub fn maximum(x: *mut RBNode) -> *mut RBNode {
        // Java: return rightmost(x);
        Self::rightmost_static(x)
    }

    // Java: RBNode leftmost()
    pub fn leftmost(&self) -> *mut RBNode {
        // Java: return this.mHead.left;
        unsafe { (*self.mHead).left }
    }

    // Java: RBNode rightmost()
    pub fn rightmost(&self) -> *mut RBNode {
        // Java: return this.mHead.right;
        unsafe { (*self.mHead).right }
    }

    // Java: void onBinaryTreeInserted(RBNode pNew)
    pub fn onBinaryTreeInserted(&self, _p_new: *mut RBNode) {
    }

    // Java: void binaryTreeInsertLeft(RBNode pNode, RBNode pNew)
    pub fn binaryTreeInsertLeft(&mut self, _p_node: *mut RBNode, p_new: *mut RBNode) {
        // Java: RBNode head = this.mHead;
        let head = self.mHead;
        // Java: pNew.left = null;
        unsafe { (*p_new).left = ptr::null_mut(); }
        // Java: pNew.right = null;
        unsafe { (*p_new).right = ptr::null_mut(); }
        // Java: if (pNode == head)
        if p_node == head
        {
            // Java: if (head.parent == null)
            if unsafe { (*head).parent }.is_null()
            {
                // Java: head.parent = pNew;
                unsafe { (*head).parent = p_new; }
                // Java: pNew.parent = head;
                unsafe { (*p_new).parent = head; }
                // Java: head.left = pNew;
                unsafe { (*head).left = p_new; }
                // Java: head.right = pNew;
                unsafe { (*head).right = p_new; }
            }
            // Java: else
            else
            {
                // Java: head.right.right = pNew;
                unsafe { (*(*head).right).right = p_new; }
                // Java: pNew.parent = head.right;
                unsafe { (*p_new).parent = (*head).right; }
                // Java: head.right = pNew;
                unsafe { (*head).right = p_new; }
            }
        }
        // Java: else if (pNode.left == null)
        else if unsafe { (*p_node).left }.is_null()
        {
            // Java: pNode.left = pNew;
            unsafe { (*p_node).left = p_new; }
            // Java: pNew.parent = pNode;
            unsafe { (*p_new).parent = p_node; }
            // Java: if (head.left == pNode)
            if unsafe { (*head).left } == p_node
            {
                // Java: head.left = pNew;
                unsafe { (*head).left = p_new; }
            }
        }
        // Java: else
        else
        {
            // Java: pNode = pNode.getPrev();
            p_node = unsafe { (*p_node).getPrev() };
            // Java: pNode.right = pNew;
            unsafe { (*p_node).right = p_new; }
            // Java: pNew.parent = pNode;
            unsafe { (*p_new).parent = p_node; }
        }
        // Java: this.onBinaryTreeInserted(pNew);
        self.onBinaryTreeInserted(p_new);
    }

    // Java: void binaryTreeInsertRight(RBNode pNode, RBNode pNew)
    pub fn binaryTreeInsertRight(&mut self, _p_node: *mut RBNode, p_new: *mut RBNode) {
        // Java: RBNode head = this.mHead;
        let head = self.mHead;
        // Java: pNew.left = null;
        unsafe { (*p_new).left = ptr::null_mut(); }
        // Java: pNew.right = null;
        unsafe { (*p_new).right = ptr::null_mut(); }
        // Java: if (pNode == head)
        if p_node == head
        {
            // Java: if (head.parent == null)
            if unsafe { (*head).parent }.is_null()
            {
                // Java: head.parent = pNew;
                unsafe { (*head).parent = p_new; }
                // Java: pNew.parent = pNode;
                unsafe { (*p_new).parent = p_node; }
                // Java: head.left = pNew;
                unsafe { (*head).left = p_new; }
                // Java: head.right = pNew;
                unsafe { (*head).right = p_new; }
            }
            // Java: else
            else
            {
                // Java: head.left.left = pNew;
                unsafe { (*(*head).left).left = p_new; }
                // Java: pNew.parent = head.left;
                unsafe { (*p_new).parent = (*head).left; }
                // Java: head.left = pNew;
                unsafe { (*head).left = p_new; }
            }
        }
        // Java: else if (pNode.right == null)
        else if unsafe { (*p_node).right }.is_null()
        {
            // Java: pNode.right = pNew;
            unsafe { (*p_node).right = p_new; }
            // Java: pNew.parent = pNode;
            unsafe { (*p_new).parent = p_node; }
            // Java: if (head.right == pNode)
            if unsafe { (*head).right } == p_node
            {
                // Java: head.right = pNew;
                unsafe { (*head).right = p_new; }
            }
        }
        // Java: else
        else
        {
            // Java: pNode = pNode.getNext();
            p_node = unsafe { (*p_node).getNext() };
            // Java: pNode.left = pNew;
            unsafe { (*p_node).left = p_new; }
            // Java: pNew.parent = pNode;
            unsafe { (*p_new).parent = p_node; }
        }
        // Java: this.onBinaryTreeInserted(pNew);
        self.onBinaryTreeInserted(p_new);
    }

    // Java: void insertLeft(RBNode pos, RBNode pNew)
    pub fn insertLeft(&mut self, pos: *mut RBNode, p_new: *mut RBNode) {
        // Java: this.binaryTreeInsertLeft(pos, pNew);
        self.binaryTreeInsertLeft(pos, p_new);
        // Java: this.insertFixup(pNew);
        self.insertFixup(p_new);
    }

    // Java: void insertRight(RBNode pos, RBNode pNew)
    pub fn insertRight(&mut self, pos: *mut RBNode, p_new: *mut RBNode) {
        // Java: this.binaryTreeInsertRight(pos, pNew);
        self.binaryTreeInsertRight(pos, p_new);
        // Java: this.insertFixup(pNew);
        self.insertFixup(p_new);
    }

    // Java: RBNode getLeftRoot(RBNode pNode)
    pub fn getLeftRoot(&self, _p_node: *mut RBNode) -> *mut RBNode {
        // Java: RBNode pHead = this.mHead;
        let p_head = self.mHead;
        // Java: RBNode p;
        // Java: for(p = pNode.parent; p != pHead && pNode == p.left; p = p.parent)
        let mut p = unsafe { (*p_node).parent };
        while p != p_head && p_node == unsafe { (*p).left }
        {
            // Java: pNode = p;
            p_node = p;
            p = unsafe { (*p).parent };
        }
        // Java: return p;
        p
    }

    // Java: void insertFixup(RBNode x)
    fn insertFixup(&mut self, mut x: *mut RBNode) {
        // Java: RBNode head = this.mHead;
        let head = self.mHead;
        // Java: x.setRed();
        unsafe { (*x).setRed(); }
        // Java: while(x != head.parent && x.parent.isRed())
        while x != unsafe { (*head).parent } && unsafe { (*(*x).parent).isRed() }
        {
            // Java: if (x.parent == x.parent.parent.left)
            if unsafe { (*x).parent } == unsafe { (*(*(*x).parent).parent).left }
            {
                // Java: RBNode y = x.parent.parent.right;
                let y = unsafe { (*(*(*x).parent).parent).right };
                // Java: if (y != null && y.isRed())
                if !y.is_null() && unsafe { (*y).isRed() }
                {
                    // Java: x.parent.setBlack();
                    unsafe { (*(*x).parent).setBlack(); }
                    // Java: y.setBlack();
                    unsafe { (*y).setBlack(); }
                    // Java: x.parent.parent.setRed();
                    unsafe { (*(*(*x).parent).parent).setRed(); }
                    // Java: x = x.parent.parent;
                    x = unsafe { (*(*x).parent).parent };
                }
                // Java: else
                else
                {
                    // Java: if (x == x.parent.right)
                    if x == unsafe { (*(*x).parent).right }
                    {
                        // Java: x = x.parent;
                        x = unsafe { (*x).parent };
                        // Java: this.rotateLeft(x, head);
                        self.rotateLeft(x, head);
                    }
                    // Java: x.parent.setBlack();
                    unsafe { (*(*x).parent).setBlack(); }
                    // Java: x.parent.parent.setRed();
                    unsafe { (*(*(*x).parent).parent).setRed(); }
                    // Java: this.rotateRight(x.parent.parent, head);
                    self.rotateRight(unsafe { (*(*x).parent).parent }, head);
                }
            }
            // Java: else
            else
            {
                // Java: RBNode y = x.parent.parent.left;
                let y = unsafe { (*(*(*x).parent).parent).left };
                // Java: if (y != null && y.isRed())
                if !y.is_null() && unsafe { (*y).isRed() }
                {
                    // Java: x.parent.setBlack();
                    unsafe { (*(*x).parent).setBlack(); }
                    // Java: y.setBlack();
                    unsafe { (*y).setBlack(); }
                    // Java: x.parent.parent.setRed();
                    unsafe { (*(*(*x).parent).parent).setRed(); }
                    // Java: x = x.parent.parent;
                    x = unsafe { (*(*x).parent).parent };
                }
                // Java: else
                else
                {
                    // Java: if (x == x.parent.left)
                    if x == unsafe { (*(*x).parent).left }
                    {
                        // Java: x = x.parent;
                        x = unsafe { (*x).parent };
                        // Java: this.rotateRight(x, head);
                        self.rotateRight(x, head);
                    }
                    // Java: x.parent.setBlack();
                    unsafe { (*(*x).parent).setBlack(); }
                    // Java: x.parent.parent.setRed();
                    unsafe { (*(*(*x).parent).parent).setRed(); }
                    // Java: this.rotateLeft(x.parent.parent, head);
                    self.rotateLeft(unsafe { (*(*x).parent).parent }, head);
                }
            }
        }
        // Java: head.parent.setBlack();
        unsafe { (*(*head).parent).setBlack(); }
    }

    // Java: static void swapColor(RBNode a, RBNode b)
    fn swapColor(a: *mut RBNode, b: *mut RBNode) {
        // Java: int ac = a.getColor();
        let ac = unsafe { (*a).getColor() };
        // Java: int bc = b.getColor();
        let bc = unsafe { (*b).getColor() };
        // Java: a.setColor(bc);
        unsafe { (*a).setColor(bc); }
        // Java: b.setColor(ac);
        unsafe { (*b).setColor(ac); }
    }

    // Java: void beforeSwap(RBNode z, RBNode y)
    pub fn beforeSwap(&self, _z: *mut RBNode, _y: *mut RBNode) {
    }

    // Java: void beforeErase(RBNode z, RBNode swp)
    pub fn beforeErase(&self, _z: *mut RBNode, _swp: *mut RBNode) {
    }

    // Java: void doSwap(RBNode z, RBNode y)
    fn doSwap(&mut self, z: *mut RBNode, y: *mut RBNode) {
        // Java: swapColor(y, z);
        Self::swapColor(y, z);
        // Java: z.left.parent = y;
        unsafe { (*(*z).left).parent = y; }
        // Java: y.left = z.left;
        unsafe { (*y).left = (*z).left; }
        // Java: z.left = null;
        unsafe { (*z).left = ptr::null_mut(); }
        // Java: RBNode z_parent = z.parent;
        let z_parent = unsafe { (*z).parent };
        // Java: RBNode z_right = z.right;
        let z_right = unsafe { (*z).right };
        // Java: RBNode y_right = y.right;
        let y_right = unsafe { (*y).right };
        // Java: if (y == z_right)
        if y == z_right
        {
            // Java: z.parent = y;
            unsafe { (*z).parent = y; }
            // Java: y.right = z;
            unsafe { (*y).right = z; }
            // Java: y.parent = z_parent;
            unsafe { (*y).parent = z_parent; }
        }
        // Java: else
        else
        {
            // Java: z.parent = y.parent;
            unsafe { (*z).parent = (*y).parent; }
            // Java: y.parent.left = z;
            unsafe { (*(*y).parent).left = z; }
            // Java: y.right = z_right;
            unsafe { (*y).right = z_right; }
            // Java: z_right.parent = y;
            unsafe { (*z_right).parent = y; }
            // Java: y.parent = z_parent;
            unsafe { (*y).parent = z_parent; }
        }
        // Java: if (y_right != null)
        if !y_right.is_null()
        {
            // Java: y_right.parent = z;
            unsafe { (*y_right).parent = z; }
        }
        // Java: z.right = y_right;
        unsafe { (*z).right = y_right; }
        // Java: if (this.mHead.parent == z)
        if unsafe { (*self.mHead).parent } == z
        {
            // Java: this.mHead.parent = y;
            unsafe { (*self.mHead).parent = y; }
        }
        // Java: else if (z_parent.left == z)
        else if unsafe { (*z_parent).left } == z
        {
            // Java: z_parent.left = y;
            unsafe { (*z_parent).left = y; }
        }
        // Java: else
        else
        {
            // Java: z_parent.right = y;
            unsafe { (*z_parent).right = y; }
        }
    }

    // Java: RBNode erase(RBNode z)
    pub fn erase(&mut self, z: *mut RBNode) -> *mut RBNode {
        // Java: RBNode head = this.mHead;
        let head = self.mHead;
        // Java: if (z == head)
        if z == head
        {
            // Java: return z;
            return z;
        }
        // Java: else
        {
            // Java: RBNode swp = z;
            let mut swp = z;
            // Java: if (z.left != null && z.right != null)
            if !unsafe { (*z).left }.is_null() && !unsafe { (*z).right }.is_null()
            {
                // Java: swp = minimum(z.right);
                swp = Self::minimum(unsafe { (*z).right });
                // Java: this.beforeSwap(z, swp);
                self.beforeSwap(z, swp);
                // Java: this.doSwap(z, swp);
                self.doSwap(z, swp);
            }
            // Java: this.beforeErase(z, swp);
            self.beforeErase(z, swp);
            // Java: RBNode x = null;
            let mut x: *mut RBNode;
            // Java: RBNode x_parent = null;
            let mut x_parent: *mut RBNode;
            // Java: if (z.left == null)
            if unsafe { (*z).left }.is_null()
            {
                // Java: x = z.right;
                x = unsafe { (*z).right };
            }
            // Java: else
            else
            {
                // Java: x = z.left;
                x = unsafe { (*z).left };
            }
            // Java: x_parent = z.parent;
            x_parent = unsafe { (*z).parent };
            // Java: if (x != null)
            if !x.is_null()
            {
                // Java: x.parent = z.parent;
                unsafe { (*x).parent = (*z).parent; }
            }
            // Java: if (head.parent == z)
            if unsafe { (*head).parent } == z
            {
                // Java: head.parent = x;
                unsafe { (*head).parent = x; }
            }
            // Java: else if (z.parent.left == z)
            else if unsafe { (*(*z).parent).left } == z
            {
                // Java: z.parent.left = x;
                unsafe { (*(*z).parent).left = x; }
            }
            // Java: else
            else
            {
                // Java: z.parent.right = x;
                unsafe { (*(*z).parent).right = x; }
            }
            // Java: if (head.left == z)
            if unsafe { (*head).left } == z
            {
                // Java: if (z.right == null)
                if unsafe { (*z).right }.is_null()
                {
                    // Java: head.left = z.parent;
                    unsafe { (*head).left = (*z).parent; }
                }
                // Java: else
                else
                {
                    // Java: head.left = minimum(x);
                    unsafe { (*head).left = Self::minimum(x); }
                }
            }
            // Java: if (head.right == z)
            if unsafe { (*head).right } == z
            {
                // Java: if (z.left == null)
                if unsafe { (*z).left }.is_null()
                {
                    // Java: head.right = z.parent;
                    unsafe { (*head).right = (*z).parent; }
                }
                // Java: else
                else
                {
                    // Java: head.right = maximum(x);
                    unsafe { (*head).right = Self::maximum(x); }
                }
            }
            // Java: if (z.isRed())
            if unsafe { (*z).isRed() }
            {
                // Java: return z;
                return z;
            }
            // Java: else
            {
                // Java: while(x != head.parent && (x == null || x.isBlack()))
                while x != unsafe { (*head).parent } && (x.is_null() || unsafe { (*x).isBlack() })
                {
                    // Java: if (x == x_parent.left)
                    if x == unsafe { (*x_parent).left }
                    {
                        // Java: RBNode w = x_parent.right;
                        let mut w = unsafe { (*x_parent).right };
                        // Java: if (w.isRed())
                        if unsafe { (*w).isRed() }
                        {
                            // Java: w.setBlack();
                            unsafe { (*w).setBlack(); }
                            // Java: x_parent.setRed();
                            unsafe { (*x_parent).setRed(); }
                            // Java: this.rotateLeft(x_parent, head);
                            self.rotateLeft(x_parent, head);
                            // Java: w = x_parent.right;
                            w = unsafe { (*x_parent).right };
                        }
                        // Java: if (w.left != null && !w.left.isBlack() || w.right != null && !w.right.isBlack())
                        if (!unsafe { (*w).left }.is_null() && !unsafe { (*(*w).left).isBlack() })
                            || (!unsafe { (*w).right }.is_null() && !unsafe { (*(*w).right).isBlack() })
                        {
                            // Java: if (w.right == null || w.right.isBlack())
                            if unsafe { (*w).right }.is_null() || unsafe { (*(*w).right).isBlack() }
                            {
                                // Java: if (w.left != null)
                                if !unsafe { (*w).left }.is_null()
                                {
                                    // Java: w.left.setBlack();
                                    unsafe { (*(*w).left).setBlack(); }
                                }
                                // Java: w.setRed();
                                unsafe { (*w).setRed(); }
                                // Java: this.rotateRight(w, head);
                                self.rotateRight(w, head);
                                // Java: w = x_parent.right;
                                w = unsafe { (*x_parent).right };
                            }
                            // Java: w.setColor(x_parent.getColor());
                            let color = unsafe { (*x_parent).getColor() };
                            unsafe { (*w).setColor(color); }
                            // Java: x_parent.setBlack();
                            unsafe { (*x_parent).setBlack(); }
                            // Java: if (w.right != null)
                            if !unsafe { (*w).right }.is_null()
                            {
                                // Java: w.right.setBlack();
                                unsafe { (*(*w).right).setBlack(); }
                            }
                            // Java: this.rotateLeft(x_parent, head);
                            self.rotateLeft(x_parent, head);
                            // Java: break;
                            break;
                        }
                        // Java: w.setRed();
                        unsafe { (*w).setRed(); }
                        // Java: x = x_parent;
                        x = x_parent;
                        // Java: x_parent = x_parent.parent;
                        x_parent = unsafe { (*x_parent).parent };
                    }
                    // Java: else
                    else
                    {
                        // Java: RBNode w = x_parent.left;
                        let mut w = unsafe { (*x_parent).left };
                        // Java: if (w.isRed())
                        if unsafe { (*w).isRed() }
                        {
                            // Java: w.setBlack();
                            unsafe { (*w).setBlack(); }
                            // Java: x_parent.setRed();
                            unsafe { (*x_parent).setRed(); }
                            // Java: this.rotateRight(x_parent, head);
                            self.rotateRight(x_parent, head);
                            // Java: w = x_parent.right;
                            w = unsafe { (*x_parent).right };
                        }
                        // Java: if (w.right != null && !w.right.isBlack() || w.left != null && !w.left.isBlack())
                        if (!unsafe { (*w).right }.is_null() && !unsafe { (*(*w).right).isBlack() })
                            || (!unsafe { (*w).left }.is_null() && !unsafe { (*(*w).left).isBlack() })
                        {
                            // Java: if (w.left == null || w.left.isBlack())
                            if unsafe { (*w).left }.is_null() || unsafe { (*(*w).left).isBlack() }
                            {
                                // Java: if (w.right != null)
                                if !unsafe { (*w).right }.is_null()
                                {
                                    // Java: w.right.setBlack();
                                    unsafe { (*(*w).right).setBlack(); }
                                }
                                // Java: w.setRed();
                                unsafe { (*w).setRed(); }
                                // Java: this.rotateLeft(w, head);
                                self.rotateLeft(w, head);
                                // Java: w = x_parent.left;
                                w = unsafe { (*x_parent).left };
                            }
                            // Java: w.setColor(x_parent.getColor());
                            let color = unsafe { (*x_parent).getColor() };
                            unsafe { (*w).setColor(color); }
                            // Java: x_parent.setBlack();
                            unsafe { (*x_parent).setBlack(); }
                            // Java: if (w.left != null)
                            if !unsafe { (*w).left }.is_null()
                            {
                                // Java: w.left.setBlack();
                                unsafe { (*(*w).left).setBlack(); }
                            }
                            // Java: this.rotateRight(x_parent, head);
                            self.rotateRight(x_parent, head);
                            // Java: break;
                            break;
                        }
                        // Java: w.setRed();
                        unsafe { (*w).setRed(); }
                        // Java: x = x_parent;
                        x = x_parent;
                        // Java: x_parent = x_parent.parent;
                        x_parent = unsafe { (*x_parent).parent };
                    }
                }
                // Java: if (x != null)
                if !x.is_null()
                {
                    // Java: x.setBlack();
                    unsafe { (*x).setBlack(); }
                }
                // Java: return z;
                z
            }
        }
    }

    // Java: void beforeRotateLeft(RBNode x, RBNode y)
    pub fn beforeRotateLeft(&self, _x: *mut RBNode, _y: *mut RBNode) {
    }

    // Java: void beforeRotateRight(RBNode y, RBNode x)
    pub fn beforeRotateRight(&self, _y: *mut RBNode, _x: *mut RBNode) {
    }

    // Java: void rotateLeft(RBNode x, RBNode head)
    fn rotateLeft(&mut self, x: *mut RBNode, head: *mut RBNode) {
        // Java: RBNode y = x.right;
        let y = unsafe { (*x).right };
        // Java: this.beforeRotateLeft(x, y);
        self.beforeRotateLeft(x, y);
        // Java: x.right = y.left;
        unsafe { (*x).right = (*y).left; }
        // Java: if (y.left != null)
        if !unsafe { (*y).left }.is_null()
        {
            // Java: y.left.parent = x;
            unsafe { (*(*y).left).parent = x; }
        }
        // Java: RBNode p = x.parent;
        let p = unsafe { (*x).parent };
        // Java: y.left = x;
        unsafe { (*y).left = x; }
        // Java: x.parent = y;
        unsafe { (*x).parent = y; }
        // Java: if (x == head.parent)
        if x == unsafe { (*head).parent }
        {
            // Java: head.parent = y;
            unsafe { (*head).parent = y; }
        }
        // Java: else if (x == p.left)
        else if x == unsafe { (*p).left }
        {
            // Java: p.left = y;
            unsafe { (*p).left = y; }
        }
        // Java: else
        else
        {
            // Java: p.right = y;
            unsafe { (*p).right = y; }
        }
        // Java: y.parent = p;
        unsafe { (*y).parent = p; }
    }

    // Java: void rotateRight(RBNode y, RBNode head)
    fn rotateRight(&mut self, y: *mut RBNode, head: *mut RBNode) {
        // Java: RBNode x = y.left;
        let x = unsafe { (*y).left };
        // Java: this.beforeRotateRight(y, x);
        self.beforeRotateRight(y, x);
        // Java: y.left = x.right;
        unsafe { (*y).left = (*x).right; }
        // Java: if (x.right != null)
        if !unsafe { (*x).right }.is_null()
        {
            // Java: x.right.parent = y;
            unsafe { (*(*x).right).parent = y; }
        }
        // Java: RBNode p = y.parent;
        let p = unsafe { (*y).parent };
        // Java: x.right = y;
        unsafe { (*x).right = y; }
        // Java: y.parent = x;
        unsafe { (*y).parent = x; }
        // Java: if (y == head.parent)
        if y == unsafe { (*head).parent }
        {
            // Java: head.parent = x;
            unsafe { (*head).parent = x; }
        }
        // Java: else if (y == p.left)
        else if y == unsafe { (*p).left }
        {
            // Java: p.left = x;
            unsafe { (*p).left = x; }
        }
        // Java: else
        else
        {
            // Java: p.right = x;
            unsafe { (*p).right = x; }
        }
        // Java: x.parent = p;
        unsafe { (*x).parent = p; }
    }
}
