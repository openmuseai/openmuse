#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]
// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.collections.textrope.INotifyTextRope
use crate::cn::wps::collections::textrope::INotifyTextRope::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.ListIndexNode
use crate::cn::wps::moffice::writer::data::internal_v2::ListIndexNode::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.ListIndexPositionTree
use crate::cn::wps::moffice::writer::data::internal_v2::ListIndexPositionTree::{ListIndexPositionTree, CPIterator};
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
use util::collections::PositionNode::PositionNode;
use std::cell::RefCell;
use std::ptr;

/// Heap layout for Java `PLC.Node` / `PLC.Head` (first field is ListIndexNode).
#[repr(C)]
pub(crate) struct HeapNode {
    pub(crate) super_: ListIndexNode,
    pub(crate) document: Option<*mut (dyn Document + 'static)>,
    // java FootnoteEndnoteRefNode.numStart lives on the node object.
    pub(crate) numStart: i32,
    // Java FieldNode extends this same PLC.Node object. Keep its scalar state
    // on the shared heap node so a copied Rust handle observes mutations.
    pub(crate) field_char: i32,
    pub(crate) field_data: i32,
    pub(crate) field_level: i32,
}

// java public abstract class PLC extends ListIndexPositionTree implements INotifyTextRope.Listener
pub struct PLC {
    // java ListIndexPositionTree super_
    pub(crate) super_: ListIndexPositionTree,
    // java Document mDocument;
    pub(crate) mDocument: Option<*mut (dyn Document + 'static)>,
    // java Transaction mTransaction;
    pub(crate) mTransaction: *mut Transaction,
    // java Head head;
    pub(crate) head: Head,
}

// java public static class Node extends ListIndexNode implements Cloneable
#[derive(Clone, Copy)]
pub struct Node {
    pub(crate) ptr: *mut HeapNode,
}

// java public static class Head extends Node
#[derive(Clone, Copy)]
pub struct Head {
    pub(crate) ptr: *mut HeapNode,
}

// java public class Iterator extends ListIndexPositionTree.CPIterator
pub struct Iterator {
    pub(crate) super_: CPIterator,
}

impl Default for Node {
    fn default() -> Self {
        Self { ptr: ptr::null_mut() }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        !self.ptr.is_null() && self.ptr == other.ptr
    }
}

impl PartialEq for Head {
    fn eq(&self, other: &Self) -> bool {
        !self.ptr.is_null() && self.ptr == other.ptr
    }
}

fn alloc_head(document: Option<&dyn Document>) -> *mut HeapNode {
    let doc = document.map(|d| unsafe {
        std::mem::transmute::<*const dyn Document, *mut (dyn Document + 'static)>(
            d as *const dyn Document,
        )
    });
    let heap = Box::into_raw(Box::new(HeapNode {
        super_: ListIndexNode::new(-1, -1),
        document: doc,
        numStart: 0,
        field_char: 0,
        field_data: 0,
        field_level: 0,
    }));
    unsafe {
        (*heap).super_.super_.super_.base.setHead();
    }
    heap
}

fn as_pos(n: Node) -> *mut PositionNode {
    n.ptr as *mut PositionNode
}

pub(crate) fn nodes_same(a: Node, b: Node) -> bool {
    !a.ptr.is_null() && a.ptr == b.ptr
}

// rust-only: `Default` 门面 —— 供 `#[derive(Default)]` 的子类（`PLCField` →
//   `PLC_Char` → `PLC`）使用。语义即 Java 的 `PLC(null)`：空文档、空事务、
//   空头节点（与既有 `PLC::new(None)` 逐字相同）。
impl Default for PLC {
    fn default() -> Self {
        PLC::new(None)
    }
}

// java public abstract class PLC extends ListIndexPositionTree implements INotifyTextRope.Listener
impl PLC {
    // java public  PLC(Document document)
    pub fn PLC(document: Option<&dyn Document>) -> Self {
      // java super(new Head(document));
      let head_ptr = alloc_head(document);
      let mut this = Self {
        super_: ListIndexPositionTree::new(head_ptr as *mut ListIndexNode),
        mDocument: document.map(|d| unsafe {
          std::mem::transmute::<*const dyn Document, *mut (dyn Document + 'static)>(
            d as *const dyn Document,
          )
        }),
        mTransaction: ptr::null_mut(),
        head: Head { ptr: head_ptr },
      };
      // java this.setTransaction(null != document ? document.getTextDocument().getTransaction() : null);
      let tx = document.and_then(|d| d.getTextDocument()).and_then(|td| td.getTransaction());
      this.setTransaction(tx);
      this
    }

    pub fn new(document: Option<&dyn Document>) -> Self {
        Self::PLC(document)
    }

    pub fn as_pos_tree(&self) -> *mut util::collections::PositionTree::PositionTree {
        self.super_.as_position_tree()
    }

    // java public final void setTransaction(Transaction transaction)
    pub fn setTransaction(&mut self, transaction: Option<&Transaction>) {
      self.mTransaction = match transaction {
        Some(t) => t as *const Transaction as *mut Transaction,
        None => ptr::null_mut(),
      };
    }

    // java public Transaction getTransaction()
    pub fn getTransaction(&self) -> Option<&Transaction> {
      if self.mTransaction.is_null() {
        None
      } else {
        Some(unsafe { &*self.mTransaction })
      }
    }

    // rust-only: Java 的 `getTransaction()` 返回对象引用、天然可写；
    //   Rust 门面把 `mTransaction` 存为裸指针（与 `getDocument` 同），
    //   `CmdUpdateFieldLevel` 需 `submitCommand(&mut self)` ⇒ 提供可变访问器。
    pub fn getTransaction_mut(&mut self) -> Option<&mut Transaction> {
      if self.mTransaction.is_null() {
        None
      } else {
        Some(unsafe { &mut *self.mTransaction })
      }
    }

    // java public final Document getDocument()
    pub fn getDocument(&self) -> Option<&dyn Document> {
      self.mDocument.map(|p| unsafe { &*p })
    }

    // rust-only: Java 的 `getDocument()` 返回对象引用、天然可写；Rust 门面把
    //   `mDocument` 存为裸指针（`PLC.rs:32`）。`Document::charAt` 等取
    //   `&mut self` ⇒ 提供可变访问器，避免 `&T as &mut T` 的未定义行为。
    pub fn getDocument_mut(&mut self) -> Option<&mut (dyn Document + 'static)> {
      self.mDocument.map(|p| unsafe { &mut *p })
    }

    pub fn isEmpty(&self) -> bool {
        self.super_.isEmpty()
    }

    pub fn size(&self) -> i32 {
        self.super_.size()
    }

    // java public final Node begin()
    pub fn begin(&self) -> Node {
      self.head.getNext()
    }

    // java public final Node last()
    pub fn last(&self) -> Node {
      self.head.getPrev()
    }

    // java public final boolean isEnd(Node node)
    pub fn isEnd(&self, node: Node) -> bool {
      nodes_same(node, self.head.as_node())
    }

    // java public final boolean isHead(Node node)
    pub fn isHead(&self, node: Node) -> bool {
      nodes_same(node, self.head.as_node())
    }

    // java public Head getHead()
    pub fn getHead(&self) -> Head {
      self.head
    }

    // java public Iterator createIterator()
    pub fn createIterator(&self) -> Iterator {
      Iterator::new__listindexpositiontree(self)
    }

    // java public Iterator createIterator(Node node)
    pub fn createIterator__node(&self, node: Node) -> Iterator {
      Iterator::new__listindexpositiontree_node(self, node)
    }

    // java public final Node find(int cp)
    pub fn find(&self, cp: i32) -> Node {
      // java Node p = this.lowerBound(cp);
      let p: Node = self.lowerBound__int(cp);
      // java return (Node)(p.position() == cp ? p : this.head);
      return if p.position() == cp { p } else { self.head.as_node() };
    }

    // java public Node lowerBound(int cp)
    // java public Node findUpper(int cp)
    pub fn findUpper(&self, cp: i32) -> Node {
      // java return this.lowerBound(cp);
      return self.lowerBound__int(cp);
    }

    // java public Node lowerBound(int cp)
    pub fn lowerBound__int(&self, cp: i32) -> Node {
      self.lowerBound__int_intarr(cp, None)
    }

    // java public Node lowerBound(int cp, int[] offset)
    pub fn lowerBound__int_intarr(&self, cp: i32, offset: Option<&mut [i32]>) -> Node {
      Node { ptr: self.super_.lowerBound_with_offset(cp, offset) as *mut HeapNode }
    }

    // java public Node upperBound(int cp)
    pub fn upperBound__int(&self, cp: i32) -> Node {
      self.upperBound__int_intarr(cp, None)
    }

    // java public Node upperBound(int cp, int[] offset)
    pub fn upperBound__int_intarr(&self, cp: i32, offset: Option<&mut [i32]>) -> Node {
      Node { ptr: self.super_.upperBound_with_offset(cp, offset) as *mut HeapNode }
    }

    // java public long getRange(Node node)
    pub fn getRange(&self, node: Node) -> i64 {
      if self.isHead(node) {
        return 0;
      }
      let start = node.position();
      let next = node.getNext();
      RANGEFactory::getRANGE(start, next.position())
    }

    // java public void insert(int cp, Node pNew)
    pub fn insert__int_node(&mut self, cp: i32, pNew: Node) {
      let mut offset: [i32; 1] = [0];
      let pos = self.lowerBound__int_intarr(cp, Some(&mut offset));
      self.addReferenceImpl(cp, offset[0], pos, pNew);
    }

    // java public void insertAtUpperBound(int cp, Node pNew)
    pub fn insertAtUpperBound(&mut self, cp: i32, pNew: Node) {
      let mut offset: [i32; 1] = [0];
      let pos = self.upperBound__int_intarr(cp, Some(&mut offset));
      self.addReferenceImpl(cp, offset[0], pos, pNew);
    }

    // java public void push_back(int cp, Node pNew)
    pub fn push_back(&mut self, cp: i32, pNew: Node) {
      self.addReferenceImpl(cp, -1, self.head.as_node(), pNew);
    }

    pub fn get(&self, index: i32) -> Node {
      let mut n = self.begin();
      let mut i: i32 = 0;
      while i < index && !self.isHead(n) {
        n = n.getNext();
        i += 1;
      }
      n
    }

    pub fn index(&self, node: Node) -> i32 {
      let mut n = self.begin();
      let mut i: i32 = 0;
      while !self.isHead(n) {
        if nodes_same(n, node) {
          return i;
        }
        n = n.getNext();
        i += 1;
      }
      -1
    }

    // java public void insert(Node pNew, int cp, int index)
    pub fn insert__node_int_int(&mut self, pNew: Node, cp: i32, index: i32) {
      let pos = self.get(index);
      self.addReferenceImpl(cp, -1, pos, pNew);
    }

    pub(crate) fn alloc_node() -> Node {
      Node {
        ptr: Box::into_raw(Box::new(HeapNode {
          super_: ListIndexNode::new(-1, -1),
          document: None,
          numStart: 0,
          field_char: 0,
          field_data: 0,
          field_level: 0,
        })),
      }
    }

    // java private int addReferenceImpl(int cp, int offset, Node pos, Node pNew)
    pub fn addReferenceImpl(&mut self, cp: i32, mut offset: i32, pos: Node, pNew: Node) -> i32 {
      if offset == -1 {
        let arg = RefCell::new([0i32]);
        self.super_.insertLeft(as_pos(pos), as_pos(pNew), cp, Some(&arg as &dyn std::any::Any));
        offset = arg.borrow()[0];
      } else {
        pNew.setOffset(offset);
        self.super_.insertLeft(as_pos(pos), as_pos(pNew), cp, None);
      }
      self.notifyOnAdd(pNew);
      offset
    }

    // java public void remove(Node entry)
    pub fn remove(&mut self, entry: Node) {
      let _ = self.super_.erase(as_pos(entry));
      self.notifyOnRemoved(entry);
    }

    // java public void insertSpaceBefore(Node pFrom, int length)
    pub fn insertSpaceBefore(&mut self, pFrom: Node, length: i32) {
      if !pFrom.isHead() {
        self.super_.insertSpaceBefore(as_pos(pFrom), length);
      }
    }

    // java public void eraseSpaceBefore(Node pFrom, int length)
    pub fn eraseSpaceBefore(&mut self, pFrom: Node, length: i32) {
      self.insertSpaceBefore(pFrom, -length);
    }

    // java public void moveRight(Node pNode, int length)
    pub fn moveRight(&mut self, pNode: Node, length: i32) {
      if !pNode.isHead() {
        self.super_.moveRight(as_pos(pNode), length);
      }
    }

    // java public void moveLeft(Node pNode, int length)
    pub fn moveLeft(&mut self, pNode: Node, length: i32) {
      self.moveRight(pNode, -length);
    }

    // java abstract void doOnInsertText(int var1, int var2)
    pub fn doOnInsertText(&mut self, _var1: i32, _var2: i32) {
      panic!("PLC::doOnInsertText abstract");
    }

    // java abstract void doOnRemoveText(int var1, int var2)
    pub fn doOnRemoveText(&mut self, _var1: i32, _var2: i32) {
      panic!("PLC::doOnRemoveText abstract");
    }

    // java private boolean isInUserAction()
    pub fn isInUserAction(&self) -> bool {
      let t = self.getTransaction();
      KSAssert::assertNotNull(t);
      self.getTransaction().map(|t| t.getState() == 1).unwrap_or(false)
    }

    // java public void beforeInsertText(int cp, int fc, int length)
    pub fn beforeInsertText(&mut self, _cp: i32, _fc: i32, _length: i32) {}

    // java public void afterInsertText(int cp, int fc, int length)
    pub fn afterInsertText(&mut self, cp: i32, _fc: i32, length: i32) {
      if !self.isEmpty() && self.isInUserAction() {
        self.doOnInsertText(cp, cp + length);
      }
    }

    // java public void beforeRemoveText(int start, int end)
    pub fn beforeRemoveText(&mut self, _start: i32, _end: i32) {}

    // java public void afterRemoveText(int start, int end)
    pub fn afterRemoveText(&mut self, start: i32, end: i32) {
      if self.isInUserAction() {
        self.doOnRemoveText(start, end);
      }
    }

    // java protected void notifyOnAdd(Node pNew)
    pub fn notifyOnAdd(&mut self, _pNew: Node) {}

    // java protected void notifyOnRemoved(Node pNew)
    pub fn notifyOnRemoved(&mut self, _pNew: Node) {}

    // java public void dispose()
    pub fn dispose(&mut self) {
      self.mDocument = None;
    }
}

impl crate::cn::wps::collections::textrope::INotifyTextRope::Listener for PLC {
    fn beforeInsertText(&mut self, var1: i32, var2: i32, var3: i32) {
        PLC::beforeInsertText(self, var1, var2, var3);
    }
    fn afterInsertText(&mut self, var1: i32, var2: i32, var3: i32) {
        PLC::afterInsertText(self, var1, var2, var3);
    }
    fn beforeRemoveText(&mut self, var1: i32, var2: i32) {
        PLC::beforeRemoveText(self, var1, var2);
    }
    fn afterRemoveText(&mut self, var1: i32, var2: i32) {
        PLC::afterRemoveText(self, var1, var2);
    }
}

// java public static class Node extends ListIndexNode implements Cloneable
impl Node {
    pub fn Node() -> Self {
      Self { ptr: ptr::null_mut() }
    }

    pub fn new() -> Self {
        // Java: public Node() { super(-1, -1); }
        // A constructed PLC.Node owns a real ListIndexNode. `Node::Node()` is
        // retained as the Rust-only null sentinel used by lookup failures.
        PLC::alloc_node()
    }

    pub fn as_pos(&self) -> *mut PositionNode {
        self.ptr as *mut PositionNode
    }

    // rust-only: 节点指针访问器 —— 供 `FieldNode` 等**子类**在自身模块内做
    //   `(FieldNode)node` 向下转型（`as_pos` 已为 `PositionNode` 开了同类先例）。
    //   放在此处可避免 `PLC.rs` 反向依赖子类模块（`data::PLCField`）。
    pub fn as_node_ptr(&self) -> *mut core::ffi::c_void {
        self.ptr as *mut core::ffi::c_void
    }

    pub(crate) fn field_state(&self) -> (i32, i32, i32) {
        if self.ptr.is_null() { return (0, 0, 0); }
        unsafe { ((*self.ptr).field_char, (*self.ptr).field_data, (*self.ptr).field_level) }
    }

    pub(crate) fn set_field_state(&self, field_char: i32, field_data: i32, field_level: i32) {
        if self.ptr.is_null() { return; }
        unsafe {
            (*self.ptr).field_char = field_char;
            (*self.ptr).field_data = field_data;
            (*self.ptr).field_level = field_level;
        }
    }

    // java public Node getPrev()
    pub fn getPrev(&self) -> Node {
      if self.ptr.is_null() {
        return Node::Node();
      }
      let prev = unsafe { (*self.ptr).super_.getPrev() };
      Node { ptr: prev as *mut HeapNode }
    }

    // java public Node getNext()
    pub fn getNext(&self) -> Node {
      if self.ptr.is_null() {
        return Node::Node();
      }
      let next = unsafe { (*self.ptr).super_.getNext() };
      Node { ptr: next as *mut HeapNode }
    }

    // java public int getCP()
    pub fn getCP(&self) -> i32 {
      self.position()
    }

    pub fn isHead(&self) -> bool {
      if self.ptr.is_null() {
        return false;
      }
      unsafe { (*self.ptr).super_.isHead() }
    }

    pub fn position(&self) -> i32 {
      if self.ptr.is_null() {
        return 0;
      }
      unsafe {
        if (*self.ptr).super_.isHead() {
          match (*self.ptr).document {
            Some(p) if !p.is_null() => (*p).getLength(),
            _ => 0,
          }
        } else {
          (*self.ptr).super_.position()
        }
      }
    }

    pub fn setOffset(&self, offset: i32) {
      if !self.ptr.is_null() {
        unsafe { (*self.ptr).super_.setOffset(offset); }
      }
    }

    pub fn isEnd(&self) -> bool {
      self.isHead()
    }

    pub fn dispose(&mut self) {}

    // java public void moveRight(int steps)
    // rust: ★M-STAGE2 P1-D1 —— 镜像 `ListIndexNode.moveRight(int steps)`，
    //   Java 原文该方法体为**空**（`PLC.java:378`），此处如实复现。
    pub fn moveRight(&mut self, _steps: i32) {}
}

// java public static class Head extends Node
impl Head {
    pub fn as_node(&self) -> Node {
      Node { ptr: self.ptr }
    }

    pub fn getNext(&self) -> Node {
      self.as_node().getNext()
    }

    pub fn getPrev(&self) -> Node {
      self.as_node().getPrev()
    }

    // java public int position()
    pub fn position(&self) -> i32 {
      self.as_node().position()
    }

    // java public boolean isEnd()
    pub fn isEnd(&self) -> bool {
      true
    }
}

// java public class Iterator extends ListIndexPositionTree.CPIterator
impl Iterator {
    pub fn new__listindexpositiontree(plc: &PLC) -> Self {
      Self {
        super_: CPIterator::new__positiontree_listindexnode(
          plc.as_pos_tree(),
          plc.head.ptr as *mut ListIndexNode,
        ),
      }
    }

    pub fn new__listindexpositiontree_node(plc: &PLC, node: Node) -> Self {
      Self {
        super_: CPIterator::new__positiontree_listindexnode(
          plc.as_pos_tree(),
          node.ptr as *mut ListIndexNode,
        ),
      }
    }

    pub fn next(&mut self) -> Node {
      Node { ptr: self.super_.next() as *mut HeapNode }
    }

    pub fn getNode(&self) -> Node {
      Node { ptr: self.super_.getNode() as *mut HeapNode }
    }

    pub fn hasNext(&self) -> bool {
      self.super_.hasNext()
    }

    pub fn prev(&mut self) -> Node {
      Node { ptr: self.super_.prev() as *mut HeapNode }
    }

    pub fn hasPrev(&self) -> bool {
      self.super_.hasPrev()
    }

    pub fn getStart(&self) -> i32 {
      self.super_.getStart()
    }
}

impl std::ops::Deref for PLC {
    type Target = ListIndexPositionTree;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl std::ops::DerefMut for PLC {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}
