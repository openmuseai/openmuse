#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]
// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.transaction.Command
use util::moffice::transaction::Command::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
use util::moffice::transaction::Action::ActionCommand;
use util::collections::PositionNode::PositionNode;
use util::collections::Node::Node;
// Java: import cn.wps.moffice.writer.data.internal_v2.ListPositionTree
use crate::cn::wps::moffice::writer::data::internal_v2::ListPositionTree::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.TextReference
use crate::cn::wps::moffice::writer::data::internal_v2::TextReference::*;
use std::cell::RefCell;
use std::ptr;

pub(crate) fn tr_as_node<T>(tr: *mut TextReference<T>) -> *mut Node {
    tr as *mut Node
}
pub(crate) fn tr_as_pos<T>(tr: *mut TextReference<T>) -> *mut PositionNode {
    tr as *mut PositionNode
}
pub(crate) fn pos_to_tr<T>(p: *mut PositionNode) -> *mut TextReference<T> {
    p as *mut TextReference<T>
}
fn tr_parent_is_null<T>(tr: *mut TextReference<T>) -> bool {
    if tr.is_null() {
        return true;
    }
    unsafe { (*tr).super_.super_.base.parent.is_null() }
}

/// Ownership glue: Java subclass virtuals (createNode/hooks) via composition.
pub trait ReferenceTableHooks<T> {
    fn createNode(&mut self, var1: i32, var2: Option<T>) -> *mut TextReference<T>;
    fn createHead(&mut self) -> *mut TextReference<T>;
    fn beforeAddReference(&mut self, var1: i32, var2: Option<T>);
    fn afterAddReference(&mut self, var1: *mut TextReference<T>);
    fn beforeRemoveReference(&mut self, var1: *mut TextReference<T>);
    fn afterRemoveReference(&mut self, var1: *mut TextReference<T>);
    fn beforeDataChange(&mut self, var1: *mut TextReference<T>, var2: Option<T>);
    fn afterDataChange(&mut self, var1: *mut TextReference<T>, var2: Option<T>);
}

// java public abstract class ReferenceTable
pub struct ReferenceTable<T> {
    // java ListPositionTree mTree;
    pub(crate) mTree: Option<ListPositionTree>,
    // java Transaction mTransaction;
    pub(crate) mTransaction: *mut Transaction,
    /// Subclass (e.g. CPTable) for Java virtual dispatch.
    pub(crate) hooks: Option<*mut (dyn ReferenceTableHooks<T> + 'static)>,
}

// java  class CmdInsert extends Command
pub struct CmdInsert<T> {
    pub(crate) super_: Command,
    pub(crate) table: *mut ReferenceTable<T>,
    pub(crate) mNext: *mut TextReference<T>,
    pub(crate) mOffset: i32,
    pub(crate) mNew: *mut TextReference<T>,
    pub(crate) mCp: i32,
    pub(crate) mNextCp: i32,
}

pub struct CmdRemove<T> {
    pub(crate) super_: Command,
    pub(crate) table: *mut ReferenceTable<T>,
    pub(crate) mNext: *mut TextReference<T>,
    pub(crate) mTextReference: *mut TextReference<T>,
    pub(crate) cp: i32,
    pub(crate) offset: i32,
    pub(crate) mNextCp: i32,
}

pub struct CmdSetData<T> {
    pub(crate) super_: Command,
    pub(crate) table: *mut ReferenceTable<T>,
    pub(crate) node: *mut TextReference<T>,
    pub(crate) data: Option<T>,
    pub(crate) cp: i32,
}

pub struct CmdInsertSpace<T> {
    pub(crate) super_: Command,
    pub(crate) table: *mut ReferenceTable<T>,
    pub(crate) leaf: *mut TextReference<T>,
    pub(crate) length: i32,
    pub(crate) cp: i32,
}

pub struct CmdMoveRight<T> {
    pub(crate) super_: Command,
    pub(crate) table: *mut ReferenceTable<T>,
    pub(crate) mNode: *mut TextReference<T>,
    pub(crate) mLength: i32,
    pub(crate) cp: i32,
}

impl<T> Default for ReferenceTable<T> {
    fn default() -> Self {
        Self {
            mTree: None,
            mTransaction: ptr::null_mut(),
            hooks: None,
        }
    }
}

// java public abstract class ReferenceTable
impl<T: Clone + 'static> ReferenceTable<T> {
    pub fn ReferenceTable() -> Self {
      let mut this = Self::default();
      this.createTree();
      this
    }

    pub fn createNode(&mut self, var1: i32, var2: Option<T>) -> *mut TextReference<T> {
      let hooks = self.hooks;
      if let Some(hooks) = hooks {
        return unsafe { (*hooks).createNode(var1, var2) };
      }
      panic!("ReferenceTable::createNode not implemented");
    }

    pub fn createHead(&mut self) -> *mut TextReference<T> {
      let hooks = self.hooks;
      if let Some(hooks) = hooks {
        return unsafe { (*hooks).createHead() };
      }
      panic!("ReferenceTable::createHead not implemented");
    }

    pub fn createTree(&mut self) {
      // java this.mTree = new ListPositionTree(this.createHead());
      let head = self.createHead();
      self.mTree = Some(ListPositionTree::ListPositionTree(head as *mut crate::cn::wps::moffice::writer::data::internal_v2::ListNode::ListNode));
    }

    pub fn setTransaction(&mut self, transaction: *mut Transaction) {
      self.mTransaction = transaction;
    }

    pub fn getTransaction(&self) -> *mut Transaction {
      return self.mTransaction;
    }

    pub fn lowerBound__int(&self, cp: i32) -> *mut TextReference<T> {
      return self.lowerBound__int_intarr(cp, None);
    }

    pub fn lowerBound__int_intarr(&self, cp: i32, offset: Option<&mut [i32]>) -> *mut TextReference<T> {
      let p = match self.mTree.as_ref() {
        Some(tree) => tree.lowerBound_with_offset(cp, offset),
        None => ptr::null_mut(),
      };
      pos_to_tr::<T>(p)
    }

    pub fn upperBound(&self, cp: i32) -> *mut TextReference<T> {
      let p = match self.mTree.as_ref() {
        Some(tree) => tree.upperBound(cp),
        None => ptr::null_mut(),
      };
      pos_to_tr::<T>(p)
    }

    pub fn seek(&self, cp: i32) -> *mut TextReference<T> {
      // java TextReference<T> pLowerBound = this.lowerBound(cp);
      let pLowerBound = self.lowerBound__int(cp);
      if pLowerBound.is_null() {
        return ptr::null_mut();
      }
      unsafe {
        // java if (pLowerBound.position() != cp)
        if (*pLowerBound).position() != cp {
          // java TextReference<T> pPrev = (TextReference)pLowerBound.getPrev();
          let pPrev = (*pLowerBound).getPrev() as *mut TextReference<T>;
          // java if (!pPrev.isHead())
          if !pPrev.is_null() && !(*pPrev).isHead() {
            return pPrev;
          }
        }
      }
      return pLowerBound;
    }

    pub fn size(&self) -> i32 {
      match self.mTree.as_ref() {
        Some(tree) => tree.super_.base.size(),
        None => 0,
      }
    }

    pub fn isEmpty(&self) -> bool {
      return self.size() == 0;
    }

    pub fn head(&self) -> *mut TextReference<T> {
      let p = match self.mTree.as_ref() {
        Some(tree) => tree.super_.base.getHead() as *mut PositionNode,
        None => ptr::null_mut(),
      };
      pos_to_tr::<T>(p)
    }

    pub fn find(&self, cp: i32) -> *mut TextReference<T> {
      let p = self.lowerBound__int(cp);
      if p.is_null() {
        return self.head();
      }
      unsafe {
        if (*p).position() == cp { p } else { self.head() }
      }
    }

    pub fn addReference__int_t(&mut self, cp: i32, data: Option<T>) -> *mut TextReference<T> {
      let pNew = self.createNode(cp, data);
      return self.addReference__int_textreferencet(cp, pNew);
    }

    pub fn position(&self, entry: *mut TextReference<T>) -> i32 {
      match self.mTree.as_ref() {
        Some(tree) => tree.position(entry as *mut crate::cn::wps::moffice::writer::data::internal_v2::ListNode::ListNode),
        None => if entry.is_null() { 0 } else { unsafe { (*entry).position() } },
      }
    }

    pub fn addReference__int_textreferencet(&mut self, cp: i32, pNew: *mut TextReference<T>) -> *mut TextReference<T> {
      // java int[] offset = new int[]{0};
      let mut offset: [i32; 1] = [0];
      // java TextReference<T> pos = this.lowerBound(cp, offset);
      let pos = self.lowerBound__int_intarr(cp, Some(&mut offset));
      // java this.addReferenceImpl(cp, offset[0], pos, pNew);
      self.addReferenceImpl(cp, offset[0], pos, pNew);
      return pNew;
    }

    pub fn leftmost(&self) -> *mut TextReference<T> {
      let p = match self.mTree.as_ref() {
        Some(tree) => tree.super_.base.leftmost() as *mut PositionNode,
        None => ptr::null_mut(),
      };
      pos_to_tr::<T>(p)
    }

    pub fn addReferenceImpl(&mut self, cp: i32, mut offset: i32, pos: *mut TextReference<T>, pNew: *mut TextReference<T>) -> i32 {
      let data = if pNew.is_null() { None } else { unsafe { (*pNew).data.clone() } };
      self.beforeAddReference(cp, data);
      if offset == -1 {
        let arg = RefCell::new([0i32]);
        if let Some(tree) = self.mTree.as_mut() {
          tree.super_.base.insertLeft(tr_as_node(pos), tr_as_node(pNew), cp, Some(&arg as &dyn std::any::Any));
        }
        offset = arg.borrow()[0];
      } else {
        if !pNew.is_null() {
          unsafe { (*pNew).setOffset(offset); }
        }
        if let Some(tree) = self.mTree.as_mut() {
          tree.super_.base.insertLeft(tr_as_node(pos), tr_as_node(pNew), cp, None);
        }
      }
      let t = self.getTransaction();
      if !t.is_null() && unsafe { (*t).getState() } == 1 {
        let table = self as *mut ReferenceTable<T>;
        let cmd = CmdInsert::CmdInsert(table, pNew, pos, cp, offset);
        unsafe { (*t).submitCommand(Some(Box::new(cmd))); }
      }
      self.afterAddReference(pNew);
      return offset;
    }

    pub fn removeReference(&mut self, pNode: *mut TextReference<T>) {
      if pNode.is_null() {
        return;
      }
      let cp = unsafe { (*pNode).position() };
      let next = unsafe { (*pNode).getNext() as *mut TextReference<T> };
      self.removeReferenceImpl(pNode, cp, next);
    }

    pub fn removeReferenceImpl(&mut self, pNode: *mut TextReference<T>, cp: i32, pNext: *mut TextReference<T>) {
      self.beforeRemoveReference(pNode);
      if let Some(tree) = self.mTree.as_mut() {
        let _ = tree.super_.base.erase(tr_as_node(pNode));
      }
      let t = self.getTransaction();
      if !t.is_null() && unsafe { (*t).getState() } == 1 {
        let table = self as *mut ReferenceTable<T>;
        let cmd = CmdRemove::CmdRemove(table, pNode, pNext, cp);
        unsafe { (*t).submitCommand(Some(Box::new(cmd))); }
      }
      self.afterRemoveReference(pNode);
    }

    pub fn insertSpaceBefore(&mut self, pFrom: *mut TextReference<T>, length: i32) {
      if pFrom.is_null() {
        return;
      }
      unsafe {
        if !(*pFrom).isHead() {
          if let Some(tree) = self.mTree.as_mut() {
            tree.insertSpaceBefore(tr_as_pos(pFrom), length);
          }
          self.recordCmdInsertSpace(pFrom, length);
        }
      }
    }

    pub fn eraseSpaceBefore(&mut self, pFrom: *mut TextReference<T>, length: i32) {
      self.insertSpaceBefore(pFrom, -length);
    }

    pub fn recordCmdInsertSpace(&mut self, pFrom: *mut TextReference<T>, length: i32) {
      let t = self.getTransaction();
      if !t.is_null() && unsafe { (*t).getState() } == 1 {
        let table = self as *mut ReferenceTable<T>;
        let cmd = CmdInsertSpace::CmdInsertSpace(table, pFrom, length);
        unsafe { (*t).submitCommand(Some(Box::new(cmd))); }
      }
    }

    pub fn moveRight(&mut self, pNode: *mut TextReference<T>, length: i32) {
      if pNode.is_null() {
        return;
      }
      unsafe {
        Assert::assertTrue_msg("pNode shpuld not be head!", !(*pNode).isHead());
      }
      if let Some(tree) = self.mTree.as_mut() {
        tree.moveRight(tr_as_pos(pNode), length);
      }
      self.recordMoveRight(pNode, length);
    }

    pub fn recordMoveRight(&mut self, pNode: *mut TextReference<T>, length: i32) {
      let t = self.getTransaction();
      if !t.is_null() && unsafe { (*t).getState() } == 1 {
        let table = self as *mut ReferenceTable<T>;
        let cmd = CmdMoveRight::CmdMoveRight(table, pNode, length);
        unsafe { (*t).submitCommand(Some(Box::new(cmd))); }
      }
    }

    pub fn moveLeft(&mut self, pNode: *mut TextReference<T>, length: i32) {
      self.moveRight(pNode, -length);
    }

    pub fn recordModify(&mut self, entry: *mut TextReference<T>, data: Option<T>) {
      let t = self.getTransaction();
      if !t.is_null() && unsafe { (*t).getState() } == 1 {
        let table = self as *mut ReferenceTable<T>;
        let cmd = CmdSetData::CmdSetData(table, entry, data);
        unsafe { (*t).submitCommand(Some(Box::new(cmd))); }
      }
    }

    pub fn setData(&mut self, pNode: *mut TextReference<T>, data: Option<T>) {
      if pNode.is_null() {
        return;
      }
      self.beforeDataChange(pNode, data.clone());
      let pOldData = unsafe { (*pNode).data.clone() };
      unsafe { (*pNode).data = data; }
      self.recordModify(pNode, pOldData.clone());
      self.afterDataChange(pNode, pOldData);
    }

    pub fn isInUndoRedo(&self) -> bool {
      let t = self.getTransaction();
      if t.is_null() {
        return false;
      }
      let state = unsafe { (*t).getState() };
      return state == 2 || state == 3;
    }

    pub fn dispose(&mut self) {
      self.mTransaction = ptr::null_mut();
      if self.mTree.is_some() {
        if let Some(ref mut mTree) = self.mTree {
          mTree.dispose();
        }
        self.mTree = None;
      }
    }

    pub fn beforeAddReference(&mut self, var1: i32, var2: Option<T>) {
      if let Some(hooks) = self.hooks {
        unsafe { (*hooks).beforeAddReference(var1, var2) };
      }
    }

    pub fn afterAddReference(&mut self, var1: *mut TextReference<T>) {
      if let Some(hooks) = self.hooks {
        unsafe { (*hooks).afterAddReference(var1) };
      }
    }

    pub fn beforeRemoveReference(&mut self, var1: *mut TextReference<T>) {
      if let Some(hooks) = self.hooks {
        unsafe { (*hooks).beforeRemoveReference(var1) };
      }
    }

    pub fn afterRemoveReference(&mut self, var1: *mut TextReference<T>) {
      if let Some(hooks) = self.hooks {
        unsafe { (*hooks).afterRemoveReference(var1) };
      }
    }

    pub fn beforeDataChange(&mut self, var1: *mut TextReference<T>, var2: Option<T>) {
      if let Some(hooks) = self.hooks {
        unsafe { (*hooks).beforeDataChange(var1, var2) };
      }
    }

    pub fn afterDataChange(&mut self, var1: *mut TextReference<T>, var2: Option<T>) {
      if let Some(hooks) = self.hooks {
        unsafe { (*hooks).afterDataChange(var1, var2) };
      }
    }

}

impl<T: Clone + 'static> CmdInsert<T> {
    pub fn CmdInsert(table: *mut ReferenceTable<T>, pNew: *mut TextReference<T>, pos: *mut TextReference<T>, cp: i32, offset: i32) -> Self {
      let next_cp = if pos.is_null() { 0 } else { unsafe { (*pos).position() } };
      Self {
        super_: Command::new(),
        table,
        mNext: pos,
        mOffset: offset,
        mNew: pNew,
        mCp: cp,
        mNextCp: next_cp,
      }
    }

    pub fn execute(&self) {
      if !self.table.is_null() && !self.mNext.is_null() && !self.mNew.is_null() {
        unsafe {
          (*self.table).addReferenceImpl(self.mCp, self.mOffset, self.mNext, self.mNew);
        }
      }
    }
}

impl<T: Clone + 'static> ActionCommand for CmdInsert<T> {
    fn execute(&self) { CmdInsert::execute(self); }
    fn unexecute(&self) {}
    fn dispose(&self) {}
    fn get_level(&self) -> i32 { self.super_.level }
}

impl<T> std::ops::Deref for CmdInsert<T> {
    type Target = Command;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<T> std::ops::DerefMut for CmdInsert<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<T: Clone + 'static> CmdRemove<T> {
    pub fn CmdRemove(table: *mut ReferenceTable<T>, pTextReference: *mut TextReference<T>, pNext: *mut TextReference<T>, cp: i32) -> Self {
      let next_cp = if pNext.is_null() { 0 } else { unsafe { (*pNext).position() } };
      Self {
        super_: Command::new(),
        table,
        mNext: pNext,
        mTextReference: pTextReference,
        cp,
        offset: -1,
        mNextCp: next_cp,
      }
    }
}

impl<T: Clone + 'static> ActionCommand for CmdRemove<T> {
    fn execute(&self) {
      if !self.table.is_null() && !self.mTextReference.is_null() {
        unsafe { (*self.table).removeReferenceImpl(self.mTextReference, self.cp, self.mNext); }
      }
    }
    fn unexecute(&self) {}
    fn dispose(&self) {}
    fn get_level(&self) -> i32 { self.super_.level }
}

impl<T> std::ops::Deref for CmdRemove<T> {
    type Target = Command;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<T> std::ops::DerefMut for CmdRemove<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<T: Clone + 'static> CmdSetData<T> {
    pub fn CmdSetData(table: *mut ReferenceTable<T>, node: *mut TextReference<T>, pOldData: Option<T>) -> Self {
      let cp = if node.is_null() { 0 } else { unsafe { (*node).position() } };
      Self { super_: Command::new(), table, node, data: pOldData, cp }
    }
}

impl<T: Clone + 'static> ActionCommand for CmdSetData<T> {
    fn execute(&self) {}
    fn unexecute(&self) {}
    fn dispose(&self) {}
    fn get_level(&self) -> i32 { self.super_.level }
}

impl<T> std::ops::Deref for CmdSetData<T> {
    type Target = Command;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<T> std::ops::DerefMut for CmdSetData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<T: Clone + 'static> CmdInsertSpace<T> {
    pub fn CmdInsertSpace(table: *mut ReferenceTable<T>, leaf: *mut TextReference<T>, length: i32) -> Self {
      let cp = if leaf.is_null() { 0 } else { unsafe { (*leaf).position() } };
      Self { super_: Command::new(), table, leaf, length, cp }
    }
}

impl<T: Clone + 'static> ActionCommand for CmdInsertSpace<T> {
    fn execute(&self) {
      if !self.table.is_null() && !self.leaf.is_null() {
        unsafe { (*self.table).insertSpaceBefore(self.leaf, self.length); }
      }
    }
    fn unexecute(&self) {}
    fn dispose(&self) {}
    fn get_level(&self) -> i32 { self.super_.level }
}

impl<T> std::ops::Deref for CmdInsertSpace<T> {
    type Target = Command;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<T> std::ops::DerefMut for CmdInsertSpace<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<T: Clone + 'static> CmdMoveRight<T> {
    pub fn CmdMoveRight(table: *mut ReferenceTable<T>, node: *mut TextReference<T>, length: i32) -> Self {
      let cp = if node.is_null() { 0 } else { unsafe { (*node).position() } };
      Self { super_: Command::new(), table, mNode: node, mLength: length, cp }
    }
}

impl<T: Clone + 'static> ActionCommand for CmdMoveRight<T> {
    fn execute(&self) {
      if !self.table.is_null() && !self.mNode.is_null() {
        unsafe { (*self.table).moveRight(self.mNode, self.mLength); }
      }
    }
    fn unexecute(&self) {}
    fn dispose(&self) {}
    fn get_level(&self) -> i32 { self.super_.level }
}

impl<T> std::ops::Deref for CmdMoveRight<T> {
    type Target = Command;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<T> std::ops::DerefMut for CmdMoveRight<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}
