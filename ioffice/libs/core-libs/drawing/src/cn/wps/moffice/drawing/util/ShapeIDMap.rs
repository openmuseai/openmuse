#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::collections::HashMap;
use core_libs_util::base::assertion::KSAssert::KSAssert;

// Java field: int NODESIZE
pub(crate) const NODESIZE: i32 = 1024;
// Java field: int SUBNODESIZE
pub(crate) const SUBNODESIZE: i32 = 128;
// Java field: int SUBCOUNT
pub(crate) const SUBCOUNT: i32 = 8;

// Java: public interface IMapEntryNode
pub trait IMapEntryNodeTrait {
    // Java: abstract int getKey()
    fn getKey(&self) -> i32;
    fn as_any(&self) -> &dyn std::any::Any;
}

pub struct IMapEntryNode {
    key: i32,
}

impl IMapEntryNodeTrait for IMapEntryNode {
    fn getKey(&self) -> i32 {
        self.key
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub trait IHashMapExtension {
    fn onCreateHead(&mut self, tree: *mut u8, head: *mut u8);
    fn onBinaryTreeInsertedLeft(&mut self, tree: *mut u8, pNode: *mut u8, pNew: *mut u8, arg1: i32, arg2: Option<&dyn std::any::Any>);
    fn onBinaryTreeInsertedRight(&mut self, tree: *mut u8, pNode: *mut u8, pNew: *mut u8, arg1: i32, arg2: Option<&dyn std::any::Any>);
    fn beforeSwap(&mut self, tree: *mut u8, z: *mut u8, y: *mut u8);
    fn beforeErase(&mut self, tree: *mut u8, z: *mut u8, swp: *mut u8);
    fn onBinaryTreeErased(&mut self, tree: *mut u8, z: *mut u8, x: *mut u8, x_parent: *mut u8);
    fn beforeRotateLeft(&mut self, tree: *mut u8, x: *mut u8, y: *mut u8);
    fn beforeRotateRight(&mut self, tree: *mut u8, y: *mut u8, x: *mut u8);
}

pub trait Listener {
    fn beforeKeyChange(&mut self, key: i32);
    fn afterKeyChange(&mut self, key: i32, value: &dyn std::any::Any);
}

// Java field: int mask
#[allow(non_upper_case_globals)]
pub(crate) const mask: i32 = 7;

// Java: public class ShapeIDMap implements IHashMapExtension
pub struct ShapeIDMap {
    // Java field: IHashMapExtension.Listener mListener
    mListener: Vec<Box<dyn Listener>>,
    // Java field: List<Node> mNodes
    mNodes: Vec<Node>,
    // Java field: IMapEntryNode mInvalidObject
    pub(crate) mInvalidObject: HashMap<i32, Box<dyn IMapEntryNodeTrait>>,
}

// Java: public class Node
struct Node {
    // Java field: int start
    pub(crate) start: i32,
    // Java field: SubNode[] subNodes
    pub(crate) subNodes: [Option<SubNode>; 8],
}

// Java: public class SubNode
struct SubNode {
    // Java field: IMapEntryNode[] objects
    pub(crate) objects: HashMap<usize, Box<dyn IMapEntryNodeTrait>>,
}

// Java: public class HashMapTreeCreator
pub struct HashMapTreeCreator {
}

// Java: public class KHashMapListener
pub struct KHashMapListener {
    outer: *mut ShapeIDMap,
}

// Java: public class ShapeIDMap implements IHashMapExtension
impl ShapeIDMap {
    // Java: ShapeIDMap()
    pub fn new() -> Self {
        // Java: super();
        ShapeIDMap {
            mListener: Vec::new(),
            mNodes: Vec::new(),
            mInvalidObject: HashMap::new(),
        }
    }

    // Java: static IMapEntryNode cast(cn.wps.collections.Node node)
    fn cast(node: *mut u8) -> *mut u8 {
        // Java: return (IMapEntryNode)node;
        node
    }

    // Java: void put(int shapeID, IMapEntryNode object)
    pub fn put(&mut self, shapeID: i32, object: Box<dyn IMapEntryNodeTrait>) {
        // Java: if (shapeID < 0)
        if shapeID < 0 {
            // Java: this.mInvalidObject = object;
            self.mInvalidObject = { let mut m = HashMap::new(); m.insert(shapeID, object); m };
        }
        // Java: else
        else {
            // Java: Node node = this.getNode(shapeID);
            let node = self.get_node_index(shapeID);
            // Java: if (node == null)
            if node.is_none() {
                // Java: node = this.addNode(shapeID);
                self.addNode(shapeID);
            }
            let node = self.get_node_index(shapeID).unwrap();
            // Java: node.add(shapeID, object);
            self.mNodes[node].add(shapeID, object);
        }
    }

    // Java: Node addNode(int shapeID)
    fn addNode(&mut self, shapeID: i32) -> usize {
        // Java: Node node = new Node(shapeID - shapeID % 1024);
        let node = Node::new_with_start(shapeID - shapeID % 1024);
        // Java: this.mNodes.add(node);
        self.mNodes.push(node);
        // Java: return node;
        self.mNodes.len() - 1
    }

    // Java: boolean inNode(Node node, int id)
    fn inNode(node: &Node, id: i32) -> bool {
        // Java: return node.start <= id && id < node.start + 1024;
        node.start <= id && id < node.start + 1024
    }

    fn get_node_index(&self, shapeID: i32) -> Option<usize> {
        for (i, node) in self.mNodes.iter().enumerate() {
            if Self::inNode(node, shapeID) {
                return Some(i);
            }
        }
        None
    }

    // Java: Node getNode(int shapeID)
    fn getNode(&self, shapeID: i32) -> Option<&Node> {
        // Java: for(Node node : this.mNodes)
        for node in &self.mNodes {
            // Java: if (node != null && this.inNode(node, shapeID))
            if true && Self::inNode(node, shapeID) {
                // Java: return node;
                return Some(node);
            }
        }
        // Java: return null;
        None
    }

    // Java: IMapEntryNode get(int shapeID)
    pub fn get(&self, shapeID: i32) -> Option<&dyn IMapEntryNodeTrait> {
        // Java: if (shapeID < 0)
        if shapeID < 0 {
            // Java: return this.mInvalidObject;
            return self.mInvalidObject.get(&shapeID).map(|v| v.as_ref());
        }
        // Java: else
        else {
            // Java: Node node = this.getNode(shapeID);
            let node = self.getNode(shapeID);
            // Java: return node != null ? node.get(shapeID) : null;
            match node {
                Some(n) => n.get(shapeID),
                None => None,
            }
        }
    }

    // Java: void remove(int shapeID)
    pub fn remove(&mut self, shapeID: i32) {
        // Java: if (shapeID < 0)
        if shapeID < 0 {
            // Java: this.mInvalidObject = null;
            self.mInvalidObject = HashMap::new();
        }
        // Java: Node node = this.getNode(shapeID);
        let node = self.get_node_index(shapeID);
        // Java: if (node != null)
        if let Some(i) = node {
            // Java: node.delete(shapeID);
            self.mNodes[i].delete(shapeID);
        }
    }

    // Java: ITreeCreator getTreeCreator()
    pub fn getTreeCreator(&self) -> HashMapTreeCreator {
        // Java: return new HashMapTreeCreator();
        HashMapTreeCreator {}
    }

    // Java: Object getValue(int key)
    pub fn getValue(&self, key: i32) -> Option<&dyn IMapEntryNodeTrait> {
        // Java: return this.get(key);
        self.get(key)
    }

    // Java: IHashMapExtension.Listener getListener()
    pub fn getListener(&self) -> &Vec<Box<dyn Listener>> {
        // Java: return this.mListener;
        &self.mListener
    }

    // Java: void setListener(IHashMapExtension.Listener listener)
    pub fn setListener(&mut self, listener: Box<dyn Listener>) {
        // Java: this.mListener = listener;
        self.mListener = vec![listener];
    }
}

impl IHashMapExtension for ShapeIDMap {
    // Java: void onCreateHead(Tree tree, cn.wps.collections.Node head)
    fn onCreateHead(&mut self, _tree: *mut u8, _head: *mut u8) {
    }

    // Java: void onBinaryTreeInsertedLeft(Tree tree, cn.wps.collections.Node pNode, cn.wps.collections.Node pNew, int arg1, Object arg2)
    fn onBinaryTreeInsertedLeft(&mut self, _tree: *mut u8, _pNode: *mut u8, pNew: *mut u8, _arg1: i32, _arg2: Option<&dyn std::any::Any>) {
        // Java: this.put(cast(pNew).getKey(), cast(pNew));
        let casted = Self::cast(pNew);
        self.put(unsafe { (*(casted as *mut IMapEntryNode)).key }, Box::new(IMapEntryNode { key: unsafe { (*(casted as *mut IMapEntryNode)).key } }));
    }

    // Java: void onBinaryTreeInsertedRight(Tree tree, cn.wps.collections.Node pNode, cn.wps.collections.Node pNew, int arg1, Object arg2)
    fn onBinaryTreeInsertedRight(&mut self, _tree: *mut u8, _pNode: *mut u8, pNew: *mut u8, _arg1: i32, _arg2: Option<&dyn std::any::Any>) {
        // Java: this.put(cast(pNew).getKey(), cast(pNew));
        let casted = Self::cast(pNew);
        self.put(unsafe { (*(casted as *mut IMapEntryNode)).key }, Box::new(IMapEntryNode { key: unsafe { (*(casted as *mut IMapEntryNode)).key } }));
    }

    // Java: void beforeSwap(Tree tree, cn.wps.collections.Node z, cn.wps.collections.Node y)
    fn beforeSwap(&mut self, _tree: *mut u8, _z: *mut u8, _y: *mut u8) {
    }

    // Java: void beforeErase(Tree tree, cn.wps.collections.Node z, cn.wps.collections.Node swp)
    fn beforeErase(&mut self, _tree: *mut u8, _z: *mut u8, _swp: *mut u8) {
    }

    // Java: void onBinaryTreeErased(Tree tree, cn.wps.collections.Node z, cn.wps.collections.Node x, cn.wps.collections.Node x_parent)
    fn onBinaryTreeErased(&mut self, _tree: *mut u8, z: *mut u8, _x: *mut u8, _x_parent: *mut u8) {
        // Java: this.remove(cast(z).getKey());
        let casted = Self::cast(z);
        self.remove(unsafe { (*(casted as *mut IMapEntryNode)).key });
    }

    // Java: void beforeRotateLeft(Tree tree, cn.wps.collections.Node x, cn.wps.collections.Node y)
    fn beforeRotateLeft(&mut self, _tree: *mut u8, _x: *mut u8, _y: *mut u8) {
    }

    // Java: void beforeRotateRight(Tree tree, cn.wps.collections.Node y, cn.wps.collections.Node x)
    fn beforeRotateRight(&mut self, _tree: *mut u8, _y: *mut u8, _x: *mut u8) {
    }
}

// Java: public class Node
impl Node {
    // Java: Node()
    fn new() -> Self {
        // Java: super();
        Node { start: 0, subNodes: Default::default() }
    }

    // Java: Node(int start)
    fn new_with_start(start: i32) -> Self {
        // Java: this.start = start;
        // LLM JavaLine Aggregation: merged into struct initializer below
        // Java: super();
        Node { start, subNodes: Default::default() }
    }

    // Java: IMapEntryNode get(int id)
    fn get(&self, id: i32) -> Option<&dyn IMapEntryNodeTrait> {
        // Java: int offset = id - this.start;
        let offset = (id - self.start) as usize;
        // Java: KSAssert.assertTrue(offset >= 0 && offset < 1024);
        KSAssert::assertTrue(offset < 1024);
        // Java: int index = offset / 128;
        let index = offset / 128;
        // Java: SubNode subNode = this.subNodes[index];
        let subNode = &self.subNodes[index];
        // Java: return subNode != null ? subNode.objects[offset % 128] : null;
        match subNode {
            Some(sub_node) => sub_node.objects.get(&(offset % 128)).map(|v| v.as_ref()),
            None => None,
        }
    }

    // Java: void delete(int id)
    fn delete(&mut self, id: i32) {
        // Java: int offset = id - this.start;
        let offset = (id - self.start) as usize;
        // Java: KSAssert.assertTrue(offset >= 0 && offset < 1024);
        KSAssert::assertTrue(offset < 1024);
        // Java: int index = offset / 128;
        let index = offset / 128;
        // Java: SubNode subNode = this.subNodes[index];
        let subNode = &mut self.subNodes[index];
        // Java: if (subNode != null)
        if let Some(sub_node) = subNode {
            // Java: subNode.objects[offset % 128] = null;
            sub_node.objects.remove(&(offset % 128));
        }
    }

    // Java: void add(int id, IMapEntryNode object)
    fn add(&mut self, id: i32, object: Box<dyn IMapEntryNodeTrait>) {
        // Java: int offset = id - this.start;
        let offset = (id - self.start) as usize;
        // Java: KSAssert.assertTrue(offset >= 0 && offset < 1024);
        KSAssert::assertTrue(offset < 1024);
        // Java: int index = offset / 128;
        let index = offset / 128;
        // Java: SubNode subNode = this.subNodes[index];
        let subNode = &mut self.subNodes[index];
        // Java: if (subNode == null)
        if subNode.is_none() {
            // Java: subNode = ShapeIDMap.this.new SubNode();
            let new_sub = SubNode::new();
            // Java: this.subNodes[index] = subNode;
            *subNode = Some(new_sub);
        }
        // Java: subNode.objects[offset % 128] = object;
        subNode.as_mut().unwrap().objects.insert(offset % 128, object);
    }
}

// Java: public class SubNode
impl SubNode {
    fn new() -> Self {
        // Java: super();
        SubNode { objects: HashMap::new() }
    }
}


// Java: public class HashMapTreeCreator
impl HashMapTreeCreator {
    // Java: HashMapTreeCreator()
    pub fn new() -> Self {
        // Java: super();
        HashMapTreeCreator {}
    }

    // Java: void init(int nTreeSize)
    pub fn init(&self, nTreeSize: i32) {
    }

    // Java: void visit(cn.wps.collections.Node node, VisitorContext ctx)
    pub fn visit(&self, _node: *mut u8, _ctx: *mut u8) {
    }

    // Java: void finish(cn.wps.collections.Node root)
    pub fn finish(&self, _root: *mut u8) {
    }

    // Java: void createTree(Tree tree, cn.wps.collections.Node root, INodeProvider list)
    pub fn createTree(&self, _tree: *mut u8, _root: *mut u8, _list: *mut u8) {
    }
}

// Java: public class KHashMapListener
impl KHashMapListener {
    // Java: KHashMapListener()
    pub unsafe fn new(outer: *mut ShapeIDMap) -> Self {
        // Java: super();
        KHashMapListener { outer }
    }

    // Java: void beforeKeyChange(int key)
    pub unsafe fn beforeKeyChange(&self, key: i32) {
        // Java: ShapeIDMap.this.remove(key);
        (*self.outer).remove(key);
    }

    // Java: void afterKeyChange(int key, Object value)
    pub unsafe fn afterKeyChange(&self, key: i32, value: &dyn std::any::Any) {
        // Java: ShapeIDMap.this.put(key, (IMapEntryNode)value);
        (*self.outer).put(key, Box::new(IMapEntryNode { key }));
    }
}
