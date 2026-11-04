#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{empty as PROPERTYSET_EMPTY, PropertySet};
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.UUCommentId
use crate::cn::wps::moffice::writer::core::UUCommentId::*;
// Java: import cn.wps.moffice.writer.data.ITable
use crate::cn::wps::moffice::writer::data::ITable::{Entry as ITableEntry, Listener as ITableListener, ITable};
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.internal.KPropertySetRegistry
use crate::cn::wps::moffice::writer::data::internal::KPropertySetRegistry::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.ReferenceTable
use crate::cn::wps::moffice::writer::data::internal_v2::ReferenceTable::{pos_to_tr, tr_as_node, ReferenceTable, ReferenceTableHooks};
// Java: import INotifyTextRope.Listener
use crate::cn::wps::collections::textrope::INotifyTextRope::Listener as TextRopeListener;
// Java: import cn.wps.moffice.writer.data.internal_v2.TextReference
use crate::cn::wps::moffice::writer::data::internal_v2::TextReference::*;
use crate::cn::wps::moffice::writer::data::internal_v2::ListNode::*;
use util::collections::PositionNode::PositionNode;
use util_adapter::cn::wps::core::runtime::IDisposable::IDisposable;
use std::ptr;

// java static final int PROPERTYSET_BITS = 18;
pub(crate) const PROPERTYSET_BITS: i32 = 18;

// java public abstract class CPTable extends ReferenceTable<Integer> implements ITable
#[repr(C)]
pub struct CPTable {
    // java ReferenceTable<Integer> super_
    pub(crate) super_: ReferenceTable<i32>,
    // java KPropertySetRegistry mPropertyRegistry;
    pub(crate) mPropertyRegistry: Option<KPropertySetRegistry>,
    // java Document document;
    pub(crate) document: Option<*mut dyn Document>,
    // java ITable.Listener listener;
    pub(crate) listener: Option<Box<dyn ITableListener>>,
}

// java public abstract class Entry extends TextReference<Integer> implements ITable.Entry
#[derive(Clone, Copy)]
pub struct Entry {
    pub(crate) node: *mut TextReference<i32>,
    pub(crate) table: *mut CPTable,
}

impl Default for CPTable {
    fn default() -> Self {
        Self {
            super_: ReferenceTable::default(),
            mPropertyRegistry: None,
            document: None,
            listener: None,
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self { node: ptr::null_mut(), table: ptr::null_mut() }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

pub fn entry_from_ptr(table: *mut CPTable, node: *mut TextReference<i32>) -> Entry {
    Entry { node, table }
}

fn doc_length(table: *mut CPTable) -> i32 {
    if table.is_null() {
        return 0;
    }
    unsafe {
        match (*table).document {
            Some(d) if !d.is_null() => (*d).getLength(),
            _ => 0,
        }
    }
}

// java public interface ITable
impl ITable for CPTable {
    fn isEmpty(&self) -> bool {
      self.super_.isEmpty()
    }

    fn begin(&self) -> Option<Box<dyn ITableEntry>> {
      Some(Box::new(CPTable::begin_entry(self)))
    }

    fn end(&self) -> Option<Box<dyn ITableEntry>> {
      Some(Box::new(CPTable::end_entry(self)))
    }

    fn last(&self) -> Option<Box<dyn ITableEntry>> {
      Some(Box::new(CPTable::last_entry(self)))
    }

    fn seek(&self, cp: i32) -> Option<Box<dyn ITableEntry>> {
      Some(Box::new(CPTable::seek_entry(self, cp)))
    }

    fn insert(&mut self, cp: i32, property: Option<&PropertySet>) -> Option<Box<dyn ITableEntry>> {
      let chpx = CPTable::insert_entry(self, cp, property);
      Some(Box::new(chpx))
    }

    fn remove(&mut self, var1: &dyn ITableEntry) {
      CPTable::remove(self, var1);
    }

    fn size(&self) -> i32 {
      self.super_.size()
    }

    fn setListener(&mut self, var1: Option<Box<dyn ITableListener>>) {
      CPTable::setListener(self, var1);
    }

    fn getListener(&self) -> Option<&dyn ITableListener> {
      self.listener.as_deref()
    }

    fn treeNext(&self, var1: &dyn ITableEntry) -> Option<Box<dyn ITableEntry>> {
      CPTable::treeNext(self, var1).map(|e| Box::new(e) as Box<dyn ITableEntry>)
    }

    fn treePrev(&self, var1: &dyn ITableEntry) -> Option<Box<dyn ITableEntry>> {
      CPTable::treePrev(self, var1).map(|e| Box::new(e) as Box<dyn ITableEntry>)
    }

}

impl CPTable {
    pub fn bind_hooks(&mut self) {
      self.super_.hooks = Some(self as *mut CPTable as *mut (dyn ReferenceTableHooks<i32> + 'static));
    }

    pub fn begin_entry(&self) -> Entry {
      let table = self as *const CPTable as *mut CPTable;
      let head = self.super_.head();
      let next = if head.is_null() { ptr::null_mut() } else { unsafe { (*head).getNext() as *mut TextReference<i32> } };
      entry_from_ptr(table, next)
    }
    pub fn end_entry(&self) -> Entry {
      let table = self as *const CPTable as *mut CPTable;
      entry_from_ptr(table, self.super_.head())
    }
    pub fn last_entry(&self) -> Entry {
      let table = self as *const CPTable as *mut CPTable;
      let head = self.super_.head();
      let prev = if head.is_null() { ptr::null_mut() } else { unsafe { (*head).getPrev() as *mut TextReference<i32> } };
      entry_from_ptr(table, prev)
    }
    pub fn seek_entry(&self, cp: i32) -> Entry {
      let table = self as *const CPTable as *mut CPTable;
      let tr = unsafe { (*table).super_.seek(cp) };
      entry_from_ptr(table, tr)
    }
    pub fn insert_entry(&mut self, cp: i32, property: Option<&PropertySet>) -> Entry {
      let data = match self.mPropertyRegistry.as_mut() {
        Some(reg) => reg.register(property),
        None => 0,
      };
      let tr = self.super_.addReference__int_t(cp, Some(data));
      entry_from_ptr(self as *mut CPTable, tr)
    }

    pub fn remove_entry(&mut self, entry: &Entry) {
      if entry.isHead() {
        panic!("Removing the head node!!");
      }
      self.super_.removeReference(entry.node);
    }

    pub fn setPropertyImpl(&mut self, entry: &mut Entry, prop: Option<&PropertySet>) {
      let data = match self.mPropertyRegistry.as_mut() {
        Some(reg) => reg.register(prop),
        None => 0,
      };
      let old_data = if entry.node.is_null() { 0 } else { unsafe { (*entry.node).getData().copied().unwrap_or(0) } };
      if data != old_data {
        self.super_.setData(entry.node, Some(data));
      }
    }

    // java public  CPTable(Document document)
    pub fn CPTable(document: *mut dyn Document) -> Self {
      let mut this = Self::default();
      this.document = if document.is_null() { None } else { Some(document) };
      // StorageDoc has no TextDocument: still need a registry so insert data != 0.
      this.mPropertyRegistry = Some(KPropertySetRegistry::KPropertySetRegistry(0));
      if !document.is_null() {
        let textDocument = unsafe { (*document).getTextDocument() };
        if let Some(td) = textDocument {
          if let Some(tx) = td.getTransaction() {
            this.super_.setTransaction(tx as *const _ as *mut _);
          }
          let mut uuid: UUCommentId = td.getUUPropertySetRegistryId();
          this.mPropertyRegistry = Some(KPropertySetRegistry::KPropertySetRegistry(uuid.getUUCommentId()));
        }
      }
      this
    }

    pub fn setListener(&mut self, listener: Option<Box<dyn ITableListener>>) {
      self.listener = listener;
    }

    pub fn remove(&mut self, entry: &dyn ITableEntry) {
      if entry.isHead() {
        panic!("Removing the head node!!");
      }
      let p = self.super_.find(entry.position());
      if !p.is_null() {
        unsafe {
          if !(*p).isHead() {
            self.super_.removeReference(p);
          }
        }
      }
    }

    pub fn beforeInsertText(&mut self, cp: i32, fc: i32, length: i32) {
      let _ = (cp, fc, length);
    }

    pub fn afterInsertText(&mut self, cp: i32, fc: i32, length: i32) {
      if !self.super_.isInUndoRedo() {
        let chpx = entry_from_ptr(self as *mut CPTable, self.super_.upperBound(cp));
        if !chpx.isHead() {
          self.super_.insertSpaceBefore(chpx.node, length);
        }
      }
    }

    pub fn afterRemoveText(&mut self, start: i32, end: i32) {
      let _ = (start, end);
    }

    pub fn beforeRemoveText(&mut self, start: i32, end: i32) {
      let _ = (start, end);
    }

    pub fn treeNext(&self, entry: &dyn ITableEntry) -> Option<Entry> {
      let p = self.super_.find(entry.position());
      let n = match self.super_.mTree.as_ref() {
        Some(tree) => tree.super_.base.treeNext(tr_as_node(p)),
        None => ptr::null_mut(),
      };
      if n.is_null() { return None; }
      Some(entry_from_ptr(self as *const CPTable as *mut CPTable, pos_to_tr::<i32>(n as *mut PositionNode)))
    }

    pub fn treePrev(&self, entry: &dyn ITableEntry) -> Option<Entry> {
      let p = self.super_.find(entry.position());
      let n = match self.super_.mTree.as_ref() {
        Some(tree) => tree.super_.base.treePrev(tr_as_node(p)),
        None => ptr::null_mut(),
      };
      if n.is_null() { return None; }
      Some(entry_from_ptr(self as *const CPTable as *mut CPTable, pos_to_tr::<i32>(n as *mut PositionNode)))
    }

    pub fn toString(&self) -> String {
      let mut sb: String = String::new();
      sb.push_str("CPTable\n\ncp order\n");
      let nMax: i32 = std::cmp::min(self.super_.size(), i32::MAX);
      let mut chpx: Entry = self.begin_entry();
      let mut i: i32 = 0;
      while i < nMax {
        sb.push_str(&chpx.toString());
        sb.push_str("\n");
        chpx = chpx.getNext();
        i += 1;
      }
      return sb;
    }

    pub fn beforeAddReference(&mut self, cp: i32, data: Option<i32>) {
      let property: PropertySet = match (self.mPropertyRegistry.as_ref(), data) {
        (Some(reg), Some(d)) => reg.get(d),
        _ => (*PROPERTYSET_EMPTY).clone(),
      };
      if let Some(listener) = self.listener.as_mut() {
        listener.beforeAddEntry(cp, Some(&property));
      }
    }

    pub fn afterAddReference(&mut self, pNode: *mut TextReference<i32>) {
      let e = entry_from_ptr(self as *mut CPTable, pNode);
      if let Some(listener) = self.listener.as_mut() {
        listener.afterAddEntry(&e);
      }
    }

    pub fn beforeRemoveReference(&mut self, pNode: *mut TextReference<i32>) {
      let e = entry_from_ptr(self as *mut CPTable, pNode);
      if let Some(listener) = self.listener.as_mut() {
        listener.beforeRemoveEntry(&e);
      }
    }

    pub fn afterRemoveReference(&mut self, pNode: *mut TextReference<i32>) {
      let e = entry_from_ptr(self as *mut CPTable, pNode);
      if let Some(listener) = self.listener.as_mut() {
        listener.afterRemoveEntry(&e);
      }
    }

    pub fn beforeDataChange(&mut self, pNode: *mut TextReference<i32>, data: Option<i32>) {
      let e = entry_from_ptr(self as *mut CPTable, pNode);
      let prop = match (self.mPropertyRegistry.as_ref(), data) {
        (Some(reg), Some(d)) => Some(reg.get(d)),
        _ => None,
      };
      if let Some(listener) = self.listener.as_mut() {
        listener.beforeDataChange(&e, prop.as_ref());
      }
    }

    pub fn afterDataChange(&mut self, pNode: *mut TextReference<i32>, data: Option<i32>) {
      let e = entry_from_ptr(self as *mut CPTable, pNode);
      let prop = match (self.mPropertyRegistry.as_ref(), data) {
        (Some(reg), Some(d)) => Some(reg.get(d)),
        _ => None,
      };
      if let Some(listener) = self.listener.as_mut() {
        listener.afterDataChange(&e, prop.as_ref());
      }
    }

    pub fn dispose(&mut self) {
      self.super_.dispose();
      self.document = None;
      self.listener = None;
      if self.mPropertyRegistry.is_some() {
        if let Some(ref mut reg) = self.mPropertyRegistry {
          reg.dispose();
        }
        self.mPropertyRegistry = None;
      }
    }

}

impl ReferenceTableHooks<i32> for CPTable {
    fn createNode(&mut self, var1: i32, var2: Option<i32>) -> *mut TextReference<i32> {
        TextReference::alloc(var1, var2, self as *mut CPTable as *mut ())
    }
    fn createHead(&mut self) -> *mut TextReference<i32> {
        TextReference::alloc(-1, None, self as *mut CPTable as *mut ())
    }
    fn beforeAddReference(&mut self, var1: i32, var2: Option<i32>) {
        CPTable::beforeAddReference(self, var1, var2);
    }
    fn afterAddReference(&mut self, var1: *mut TextReference<i32>) {
        CPTable::afterAddReference(self, var1);
    }
    fn beforeRemoveReference(&mut self, var1: *mut TextReference<i32>) {
        CPTable::beforeRemoveReference(self, var1);
    }
    fn afterRemoveReference(&mut self, var1: *mut TextReference<i32>) {
        CPTable::afterRemoveReference(self, var1);
    }
    fn beforeDataChange(&mut self, var1: *mut TextReference<i32>, var2: Option<i32>) {
        CPTable::beforeDataChange(self, var1, var2);
    }
    fn afterDataChange(&mut self, var1: *mut TextReference<i32>, var2: Option<i32>) {
        CPTable::afterDataChange(self, var1, var2);
    }
}

impl std::ops::Deref for CPTable {
    type Target = ReferenceTable<i32>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}

impl std::ops::DerefMut for CPTable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl IDisposable for CPTable {
    fn dispose(&self) {}
}

impl TextRopeListener for CPTable {
    fn beforeInsertText(&mut self, var1: i32, var2: i32, var3: i32) {
        CPTable::beforeInsertText(self, var1, var2, var3);
    }
    fn afterInsertText(&mut self, var1: i32, var2: i32, var3: i32) {
        CPTable::afterInsertText(self, var1, var2, var3);
    }
    fn beforeRemoveText(&mut self, var1: i32, var2: i32) {
        CPTable::beforeRemoveText(self, var1, var2);
    }
    fn afterRemoveText(&mut self, var1: i32, var2: i32) {
        CPTable::afterRemoveText(self, var1, var2);
    }
}

impl Entry {
    pub fn Entry(table: *mut CPTable, cp: i32, data: Option<i32>) -> Self {
      let node = TextReference::alloc(cp, data, table as *mut ());
      Self { node, table }
    }

    pub fn endCp(&self) -> i32 {
      return self.getNext().position();
    }

    pub fn range(&self) -> i64 {
      return RANGEFactory::getRANGE(self.position(), self.endCp());
    }

    pub fn length(&self) -> i32 {
      return self.endCp() - self.position();
    }

    pub fn isEnd(&self) -> bool {
      return false;
    }

    pub fn isLast(&self) -> bool {
      if self.table.is_null() { return false; }
      unsafe {
        let prev = (*self.table).end_entry().getPrev();
        self.node == prev.node
      }
    }

    pub fn isHead(&self) -> bool {
      if self.node.is_null() {
        return true;
      }
      unsafe { (*self.node).isHead() }
    }

    pub fn getProperty(&self) -> PropertySet {
      if self.table.is_null() || self.node.is_null() {
        return (*PROPERTYSET_EMPTY).clone();
      }
      unsafe {
        match ((*self.table).mPropertyRegistry.as_ref(), (*self.node).getData()) {
          (Some(reg), Some(d)) => reg.get(*d),
          _ => (*PROPERTYSET_EMPTY).clone(),
        }
      }
    }

    pub fn setProperty(&mut self, prop: Option<&PropertySet>) {
      if !self.table.is_null() {
        unsafe { (*self.table).setPropertyImpl(self, prop); }
      }
    }

    pub fn getPropertySetId(&self) -> i32 {
      let registryId: i32 = if self.table.is_null() {
        0
      } else {
        unsafe { (*self.table).mPropertyRegistry.as_ref().map(|r| r.getRegistryId()).unwrap_or(0) }
      };
      let data = if self.node.is_null() { 0 } else { unsafe { (*self.node).getData().copied().unwrap_or(0) } };
      return (registryId << 18) + data;
    }

    pub fn position(&self) -> i32 {
      if self.node.is_null() {
        return 0;
      }
      unsafe {
        if (*self.node).isHead() {
          return doc_length(self.table);
        }
        if self.table.is_null() {
          return (*self.node).position();
        }
        (*self.table).super_.position(self.node)
      }
    }

    pub fn getNext(&self) -> Entry {
      let next = if self.node.is_null() {
        ptr::null_mut()
      } else {
        unsafe { (*self.node).getNext() as *mut TextReference<i32> }
      };
      Entry { node: next, table: self.table }
    }

    pub fn getPrev(&self) -> Entry {
      let prev = if self.node.is_null() {
        ptr::null_mut()
      } else {
        unsafe { (*self.node).getPrev() as *mut TextReference<i32> }
      };
      Entry { node: prev, table: self.table }
    }

    pub fn toString(&self) -> String {
      return format!(
        "Entry: cp = {}, start = {}, end = {}, property = {}",
        self.position(), self.position(), self.endCp(), "PropertySet"
      );
    }

}

impl ITableEntry for Entry {
    fn position(&self) -> i32 { Entry::position(self) }
    fn endCp(&self) -> i32 { Entry::endCp(self) }
    fn range(&self) -> i64 { Entry::range(self) }
    fn length(&self) -> i32 { Entry::length(self) }
    fn getPrev(&self) -> Option<Box<dyn ITableEntry>> { Some(Box::new(Entry::getPrev(self))) }
    fn getNext(&self) -> Option<Box<dyn ITableEntry>> { Some(Box::new(Entry::getNext(self))) }
    fn isHead(&self) -> bool { Entry::isHead(self) }
    fn isEnd(&self) -> bool { Entry::isEnd(self) }
    fn isLast(&self) -> bool { Entry::isLast(self) }
    fn getProperty(&self) -> Option<&PropertySet> { None }
    fn setProperty(&mut self, var1: Option<&PropertySet>) { Entry::setProperty(self, var1); }
    fn getPropertySetId(&self) -> i32 { Entry::getPropertySetId(self) }
}
