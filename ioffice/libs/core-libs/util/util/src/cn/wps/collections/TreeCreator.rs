#![allow(non_snake_case, dead_code, unused_variables)]

use std::ptr;
use super::Node::Node;
use super::Tree::Tree;
use super::INodeProvider::INodeProvider;
use super::INodeVisitor::INodeVisitor;
use super::ITreeCreator::ITreeCreator;
use super::VisitorContext::VisitorContext;
use crate::cn::wps::math::MathUtil::MathUtil;

// Java: public class TreeCreatorContext
pub struct TreeCreatorContext {
    pub base: VisitorContext,
    // Java field: int size
    pub(crate) size: i32,
}

impl TreeCreatorContext {
    pub fn new() -> Self {
        Self {
            base: VisitorContext::new(),
            size: 0,
        }
    }

    pub fn getLeftSize(&self) -> i32 {
        TreeCreator::left_size(self.size)
    }

    pub fn getRightSize(&self) -> i32 {
        TreeCreator::right_size(self.size)
    }

    pub fn getSize(&self) -> i32 {
        self.size
    }
}

// Java: public class TreeCreator
pub struct TreeCreator;

// Java: public class NodeVisitorDispatch
pub struct NodeVisitorDispatch {
    // Java field: ITreeCreator[] children
    pub(crate) children: Vec<Box<dyn ITreeCreator>>,
}

// Java: public class Stack
struct InternalStack<T> {
    // Java field: ArrayList<T> elements
    elements: Vec<T>,
}

impl<T> InternalStack<T> {
    fn new() -> Self { Self { elements: Vec::with_capacity(8) } }
    fn with_capacity(cap: usize) -> Self { Self { elements: Vec::with_capacity(cap) } }

    // Java: boolean isEmpty()
    fn is_empty(&self) -> bool {
        // Java: return this.elements.isEmpty();
        self.elements.is_empty()
    }

    // Java: T peek()
    fn peek(&self) -> &T {
        // Java: return (T)this.elements.get(this.elements.size() - 1);
        self.elements.last().unwrap()
    }

    // Java: T pop()
    fn pop(&mut self) -> T {
        // Java: int index = this.elements.size() - 1;
        // Java: T element = (T)this.elements.get(index);
        // Java: this.elements.remove(index);
        // Java: return element;
        self.elements.pop().unwrap()
    }

    // Java: void push(T e)
    fn push(&mut self, e: T) {
        // Java: this.elements.add(e);
        self.elements.push(e);
    }

    // Java: void setTop(T e)
    fn set_top(&mut self, e: T) {
        // Java: this.elements.set(this.elements.size() - 1, e);
        let len = self.elements.len();
        self.elements[len - 1] = e;
    }

    // Java: int size()
    fn size(&self) -> i32 {
        // Java: return this.elements.size();
        self.elements.len() as i32
    }

    // Java: T top()
    fn top(&self) -> &T {
        // Java: return (T)this.elements.get(this.elements.size() - 1);
        self.elements.last().unwrap()
    }
}

// Java: public class TreeCreator
impl TreeCreator {
    // Java: TreeCreator()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static Node createBalancedTreeFromSortedList(INodeProvider list, INodeVisitor visitor)
    pub fn createBalancedTreeFromSortedList_visitor(list: &mut dyn INodeProvider, visitor: &mut dyn INodeVisitor) -> *mut Node {
        // Java: Node root = null;
        let mut root: *mut Node = ptr::null_mut();
        // Java: if (list != null && visitor != null)
        {
            // Java: int size = list.getNodeCount();
            let mut size = list.getNodeCount();
            // Java: visitor.init(size);
            visitor.init(size);
            // Java: if (size == 0)
            if size == 0
            {
                // Java: visitor.finish(root);
                visitor.finish(root);
                // Java: return root;
                return root;
            }
            // Java: else
            {
                // Java: int max_depth = 32;
                let _max_depth = 32;
                // Java: Stack<Integer> stack_size = new Stack<Integer>(32);
                let mut stack_size: InternalStack<i32> = InternalStack::with_capacity(32);
                // Java: Stack<Node> stack_left_holder = new Stack<Node>(32);
                let mut stack_left_holder: InternalStack<*mut Node> = InternalStack::with_capacity(32);
                // Java: Stack<Node> rstack = new Stack<Node>();
                let mut rstack: InternalStack<*mut Node> = InternalStack::new();
                // Java: Stack<Integer> levels = new Stack<Integer>();
                let mut levels: InternalStack<i32> = InternalStack::new();
                // Java: levels.push(-1);
                levels.push(-1);
                // Java: TreeCreatorContext ctx = new TreeCreatorContext();
                let mut ctx = VisitorContext::new();
                // Java: ctx.totalSize = size;
                ctx.totalSize = size;
                // Java: while(size > 0 || !stack_size.isEmpty())
                while size > 0 || !stack_size.is_empty()
                {
                    // Java: while(size > 0)
                    while size > 0
                    {
                        // Java: stack_size.push(size);
                        stack_size.push(size);
                        // Java: stack_left_holder.push(null);
                        stack_left_holder.push(ptr::null_mut());
                        // Java: levels.push((Integer)levels.peek() + 1);
                        let lev = *levels.peek() + 1;
                        levels.push(lev);
                        // Java: size = left_size(size);
                        size = Self::left_size(size);
                    }
                    // Java: size = (Integer)stack_size.pop();
                    size = stack_size.pop();
                    // Java: Node left = stack_left_holder.pop();
                    let left = stack_left_holder.pop();
                    // Java: int level = (Integer)levels.pop();
                    let level = levels.pop();
                    // Java: Node n_root = list.next();
                    let n_root = list.next();
                    // Java: n_root.left = left;
                    unsafe { (*n_root).left = left; }
                    // Java: if (n_root.left != null)
                    if !unsafe { (*n_root).left }.is_null()
                    {
                        // Java: n_root.left.parent = n_root;
                        unsafe { (*(*n_root).left).parent = n_root; }
                    }
                    // Java: if (!stack_left_holder.isEmpty() && stack_left_holder.peek() == null)
                    if !stack_left_holder.is_empty() && (*stack_left_holder.peek()).is_null()
                    {
                        // Java: stack_left_holder.setTop(n_root);
                        stack_left_holder.set_top(n_root);
                        // Java: ctx.nodeType = 1;
                        ctx.nodeType = 1;
                    }
                    // Java: else if (!rstack.isEmpty() && (rstack.peek()).right == null)
                    else if !rstack.is_empty() && unsafe { (**rstack.peek()).right }.is_null()
                    {
                        // Java: Node parent = rstack.pop();
                        let parent = rstack.pop();
                        // Java: parent.right = n_root;
                        unsafe { (*parent).right = n_root; }
                        // Java: parent.right.parent = parent;
                        unsafe { (*(*parent).right).parent = parent; }
                        // Java: levels.pop();
                        levels.pop();
                        // Java: ctx.nodeType = 2;
                        ctx.nodeType = 2;
                    }
                    // Java: else
                    else
                    {
                        // Java: root = n_root;
                        root = n_root;
                        // Java: ctx.nodeType = 0;
                        ctx.nodeType = 0;
                    }
                    // Java: if (visitor != null)
                    {
                        // Java: ctx.level = level;
                        ctx.level = level;
                        // Java: ctx.size = size;
                        // Java: visitor.visit(n_root, ctx);
                        visitor.visit(n_root, &mut ctx);
                    }
                    // Java: size = right_size(size);
                    size = Self::right_size(size);
                    // Java: if (size > 0)
                    if size > 0
                    {
                        // Java: rstack.push(n_root);
                        rstack.push(n_root);
                        // Java: levels.push(level);
                        levels.push(level);
                    }
                    // Java: else
                    else
                    {
                        // Java: n_root.right = null;
                        unsafe { (*n_root).right = ptr::null_mut(); }
                    }
                }
                // Java: if (visitor != null)
                {
                    // Java: visitor.finish(root);
                    visitor.finish(root);
                }
                // Java: return root;
                return root;
            }
        }
        // Java: else
        {
            // Java: return root;
            return root;
        }
    }

    // Java: static void createBalancedTreeFromSortedList(Tree tree, INodeProvider list)
    pub fn createBalancedTreeFromSortedList_tree(tree: &mut Tree, list: &mut dyn INodeProvider) {
        // Java: ITreeExtension[] pExtensions = tree.getExtensions();
        // Java: ITreeCreator[] pVisitors = new ITreeCreator[pExtensions.length + 1];
        // Java: for(int i = 0; i < pExtensions.length; ++i)
        // Java: pVisitors[i] = pExtensions[i].getTreeCreator();
        // Java: pVisitors[pVisitors.length - 1] = tree.getBalancer().getTreeCreator();
        // Java: NodeVisitorDispatch dispatcher = new NodeVisitorDispatch(pVisitors);
        // Java: Node root = createBalancedTreeFromSortedList((INodeProvider)list, (INodeVisitor)dispatcher);
        // Java: tree.assignRoot(root);
        // Java: dispatcher.createTree(tree, root, list);
    }

    // Java: static int left_size(int size)
    pub fn left_size(size: i32) -> i32 {
        // Java: return size / 2;
        size / 2
    }

    // Java: static int maxLevel(int nTreeSize)
    pub fn maxLevel(n_tree_size: i32) -> i32 {
        // Java: return MathUtil.log2(nTreeSize);
        MathUtil::log2(n_tree_size)
    }

    // Java: static int right_size(int size)
    pub fn right_size(size: i32) -> i32 {
        // Java: return size - size / 2 - 1;
        size - size / 2 - 1
    }
}

// Java: public class NodeVisitorDispatch
impl NodeVisitorDispatch {
    // Java: NodeVisitorDispatch(ITreeCreator[] children)
    pub fn new(children: Vec<Box<dyn ITreeCreator>>) -> Self {
        // Java: super();
        // Java: this.children = children;
        Self { children }
    }

    // Java: void createTree(Tree tree, Node root, INodeProvider list)
    pub fn createTree(&mut self, tree: &mut Tree, root: *mut Node, list: &dyn INodeProvider) {
        // Java: int n = this.children.length;
        let n = self.children.len();
        // Java: for(int i = 0; i < n; ++i)
        for i in 0..n
        {
            // Java: this.children[i].createTree(tree, root, list);
            self.children[i].createTree(tree, root, list);
        }
    }
}

impl INodeVisitor for NodeVisitorDispatch {
    // Java: void finish(Node root)
    fn finish(&mut self, root: *mut Node) {
        // Java: int n = this.children.length;
        let n = self.children.len();
        // Java: for(int i = 0; i < n; ++i)
        for i in 0..n
        {
            // Java: this.children[i].finish(root);
            self.children[i].finish(root);
        }
    }

    // Java: void init(int nTreeSize)
    fn init(&mut self, n_tree_size: i32) {
        // Java: int n = this.children.length;
        let n = self.children.len();
        // Java: for(int i = 0; i < n; ++i)
        for i in 0..n
        {
            // Java: this.children[i].init(i);
            self.children[i].init(i as i32);
        }
    }

    // Java: void visit(Node node, VisitorContext ctx)
    fn visit(&mut self, node: *mut Node, ctx: &mut VisitorContext) {
        // Java: int n = this.children.length;
        let n = self.children.len();
        // Java: for(int i = 0; i < n; ++i)
        for i in 0..n
        {
            // Java: this.children[i].visit(node, ctx);
            self.children[i].visit(node, ctx);
        }
    }
}
