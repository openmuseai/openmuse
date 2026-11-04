#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{empty as PROPERTYSET_EMPTY, PropertySet};
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.StyleListenerBase
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::KStyles;
use crate::cn::wps::moffice::writer::core::StyleListenerBase::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{
    Entry as IChpxEntry, IChpxTable, Listener as IChpxListener,
};
// Java: import cn.wps.moffice.writer.data.ITable
use crate::cn::wps::moffice::writer::data::ITable::{
    Entry as ITableEntry, Listener as ITableListener, ITable,
};
// Java: import cn.wps.moffice.writer.data.internal_v2.CPTable
use crate::cn::wps::moffice::writer::data::internal_v2::CPTable::{
    entry_from_ptr, Entry as CPTableEntry, CPTable,
};
// Java: import cn.wps.moffice.writer.data.internal_v2.CPIterator
use crate::cn::wps::moffice::writer::data::internal_v2::CPIterator::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.TextReference
use crate::cn::wps::moffice::writer::data::internal_v2::TextReference::*;
use crate::cn::wps::moffice::writer::data::internal_v2::ReferenceTable::{
    ReferenceTableHooks,
};
use crate::cn::wps::collections::textrope::INotifyTextRope::Listener as TextRopeListener;
use util_adapter::cn::wps::core::runtime::IDisposable::IDisposable;
use std::ptr;

// java final class ChpxTable extends CPTable implements IChpxTable
#[repr(C)]
#[derive(Default)]
pub struct ChpxTable {
    // java CPTable super_
    pub(crate) super_: CPTable,
}

// java public class Head extends Entry
#[derive(Default)]
pub struct Head {
    pub(crate) super_: Entry,
}

// java public class Entry extends CPTable.Entry implements IChpxTable.Entry
#[derive(Clone, Copy, Default)]
pub struct Entry {
    pub(crate) super_: CPTableEntry,
    pub(crate) table: *mut ChpxTable,
}

fn entry_from_cp(table: *mut ChpxTable, e: CPTableEntry) -> Entry {
    Entry { super_: e, table }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.super_.node == other.super_.node
    }
}

impl IChpxTable for ChpxTable {
    fn begin(&self) -> Option<Box<dyn IChpxEntry>> {
      Some(Box::new(ChpxTable::begin_entry(self)))
    }

    fn end(&self) -> Option<Box<dyn IChpxEntry>> {
      Some(Box::new(ChpxTable::end_entry(self)))
    }

    fn last(&self) -> Option<Box<dyn IChpxEntry>> {
      Some(Box::new(ChpxTable::last_entry(self)))
    }

    fn seek(&self, cp: i32) -> Option<Box<dyn IChpxEntry>> {
      Some(Box::new(ChpxTable::seek_entry(self, cp)))
    }

    fn insert(&mut self, cp: i32, property: Option<&PropertySet>) -> Option<Box<dyn IChpxEntry>> {
      Some(Box::new(ChpxTable::insert_entry(self, cp, property)))
    }

    fn remove(&mut self, var1: &dyn ITableEntry) {
      self.super_.remove(var1);
    }

    fn createCpIterator(&self) -> Option<CPIterator> {
      Some(CPIterator::new())
    }

}

impl ITable for ChpxTable {
    fn isEmpty(&self) -> bool {
        self.super_.isEmpty()
    }
    fn begin(&self) -> Option<Box<dyn ITableEntry>> {
        Some(Box::new(ChpxTable::begin_entry(self)))
    }
    fn end(&self) -> Option<Box<dyn ITableEntry>> {
        Some(Box::new(ChpxTable::end_entry(self)))
    }
    fn last(&self) -> Option<Box<dyn ITableEntry>> {
        Some(Box::new(ChpxTable::last_entry(self)))
    }
    fn seek(&self, var1: i32) -> Option<Box<dyn ITableEntry>> {
        Some(Box::new(ChpxTable::seek_entry(self, var1)))
    }
    fn insert(&mut self, var1: i32, var2: Option<&PropertySet>) -> Option<Box<dyn ITableEntry>> {
        Some(Box::new(ChpxTable::insert_entry(self, var1, var2)))
    }
    fn remove(&mut self, var1: &dyn ITableEntry) {
        self.super_.remove(var1);
    }
    fn size(&self) -> i32 {
        self.super_.size()
    }
    fn setListener(&mut self, var1: Option<Box<dyn ITableListener>>) {
        self.super_.setListener(var1);
    }
    fn getListener(&self) -> Option<&dyn ITableListener> {
        self.super_.getListener()
    }
    fn treeNext(&self, var1: &dyn ITableEntry) -> Option<Box<dyn ITableEntry>> {
        self.super_.treeNext(var1).map(|e| Box::new(e) as Box<dyn ITableEntry>)
    }
    fn treePrev(&self, var1: &dyn ITableEntry) -> Option<Box<dyn ITableEntry>> {
        self.super_.treePrev(var1).map(|e| Box::new(e) as Box<dyn ITableEntry>)
    }
}

impl ChpxTable {
    pub fn bind_hooks(&mut self) {
        self.super_.super_.hooks =
            Some(self as *mut ChpxTable as *mut (dyn ReferenceTableHooks<i32> + 'static));
    }

    pub fn begin_entry(&self) -> Entry {
        entry_from_cp(self as *const ChpxTable as *mut ChpxTable, self.super_.begin_entry())
    }
    pub fn end_entry(&self) -> Entry {
        entry_from_cp(self as *const ChpxTable as *mut ChpxTable, self.super_.end_entry())
    }
    pub fn last_entry(&self) -> Entry {
        entry_from_cp(self as *const ChpxTable as *mut ChpxTable, self.super_.last_entry())
    }
    pub fn seek_entry(&self, cp: i32) -> Entry {
        entry_from_cp(self as *const ChpxTable as *mut ChpxTable, self.super_.seek_entry(cp))
    }
    pub fn insert_entry(&mut self, cp: i32, property: Option<&PropertySet>) -> Entry {
        self.bind_hooks();
        entry_from_cp(self as *mut ChpxTable, self.super_.insert_entry(cp, property))
    }

    // rust: ★T7-9 —— `Document::getStyles()` 门面已从伪造的 `Option<*mut ()>` 迁移为
    //   真实 `Option<*mut KStyles>` ⇒ 本辅助函数随之返回真实类型（不再擦除）。
    fn styles_ptr(&self) -> Option<*mut KStyles> {
        match self.super_.document {
            Some(d) if !d.is_null() => unsafe { (*d).getStyles() },
            _ => None,
        }
    }

    // java   ChpxTable(Document document)
    pub fn ChpxTable(document: *mut dyn Document) -> Self {
      let mut this = Self {
            super_: CPTable::CPTable(document),
        };
        this.bind_hooks();
        this.super_.super_.createTree();
        this.bind_hooks();
        this
    }

    pub fn createHead(&mut self) -> *mut TextReference<i32> {
      TextReference::alloc(-1, None, &mut self.super_ as *mut CPTable as *mut ())
    }

    pub fn createNode(&mut self, cp: i32, data: Option<i32>) -> *mut TextReference<i32> {
      TextReference::alloc(cp, data, &mut self.super_ as *mut CPTable as *mut ())
    }

    pub fn beforeRemoveText(&mut self, start: i32, end: i32) {
      self.bind_hooks();
      if !self.super_.super_.isInUndoRedo() {
        let mut s: Entry = self.split(start);
        let e: Entry = self.split(end);
        while s != e {
          let chpx_next = s.getNext();
          self.super_.remove_entry(&s.super_);
          s = chpx_next;
        }
        self.super_.super_.eraseSpaceBefore(e.super_.node, end - start);
      }
    }

    pub fn split(&mut self, cp: i32) -> Entry {
      let mut chpx: Entry = self.seek_entry(cp);
      if chpx != self.end_entry() && chpx.position() != cp {
        let prop = chpx.getProperty();
        chpx = self.insert_entry(cp, Some(&prop));
      }
      return chpx;
    }

    pub fn beforeAddReference(&mut self, cp: i32, data: Option<i32>) {
      self.super_.beforeAddReference(cp, data);
      let prop = match (self.super_.mPropertyRegistry.as_ref(), data) {
            (Some(reg), Some(d)) => Some(reg.get(d)),
            _ => None,
        };
        let prop_owned = prop;
        // java StyleListenerBase.addStyleRef(this.document.getStyles(), this.mPropertyRegistry.get(data), (Integer)null, 2);
        // rust: ★T7-9 —— **真实接线恢复**。原调用被 M-STAGE2 P1-D1/T2 消除，理由写在
        //   这里：「`styles_ptr()` 来自 `Document::getStyles()`，其返回类型是伪造的
        //   `Option<*mut ()>`……须待软 `Document` 图把 `getStyles()` 从 `*mut ()`
        //   迁移为真实 `KStyles` 后恢复」。
        //   ★ 该前置**现已满足**：`Document::getStyles()` 返回 `Option<*mut KStyles>`，
        //   且 `StyleListenerBase::addStyleRef/removeStyleRef` 已是真实移植
        //   （`StyleListenerBase.rs:28/49`，形参 `&mut KStyles`）⇒ 按 Java 逐字恢复调用。
        let styles = self.styles_ptr();
        if let (Some(styles), Some(prop_ref)) = (styles, prop_owned.as_ref()) {
            StyleListenerBase::addStyleRef(unsafe { &mut *styles }, Some(prop_ref), None, 2);
        }
    }

    pub fn beforeRemoveReference(&mut self, pNode: *mut TextReference<i32>) {
      self.super_.beforeRemoveReference(pNode);
      let e = entry_from_cp(
            self as *mut ChpxTable,
            entry_from_ptr(&mut self.super_ as *mut CPTable, pNode),
        );
        let prop = if e.isHead() {
            None
        } else {
            Some(e.getProperty())
        };
        // java StyleListenerBase.removeStyleRef(this.document.getStyles(), ((ITable.Entry)pNode).getProperty(), (Integer)null, 2);
        // rust: ★T7-9 —— 同 `beforeAddReference`：前置已满足 ⇒ 按 Java 逐字恢复真实调用。
        let styles = self.styles_ptr();
        if let Some(styles) = styles {
            StyleListenerBase::removeStyleRef(unsafe { &mut *styles }, prop.as_ref(), None, 2);
        }
    }

}

impl ReferenceTableHooks<i32> for ChpxTable {
    fn createNode(&mut self, var1: i32, var2: Option<i32>) -> *mut TextReference<i32> {
        ChpxTable::createNode(self, var1, var2)
    }
    fn createHead(&mut self) -> *mut TextReference<i32> {
        ChpxTable::createHead(self)
    }
    fn beforeAddReference(&mut self, var1: i32, var2: Option<i32>) {
        ChpxTable::beforeAddReference(self, var1, var2);
    }
    fn afterAddReference(&mut self, var1: *mut TextReference<i32>) {
        self.super_.afterAddReference(var1);
    }
    fn beforeRemoveReference(&mut self, var1: *mut TextReference<i32>) {
        ChpxTable::beforeRemoveReference(self, var1);
    }
    fn afterRemoveReference(&mut self, var1: *mut TextReference<i32>) {
        self.super_.afterRemoveReference(var1);
    }
    fn beforeDataChange(&mut self, var1: *mut TextReference<i32>, var2: Option<i32>) {
        self.super_.beforeDataChange(var1, var2);
    }
    fn afterDataChange(&mut self, var1: *mut TextReference<i32>, var2: Option<i32>) {
        self.super_.afterDataChange(var1, var2);
    }
}

impl std::ops::Deref for ChpxTable {
    type Target = CPTable;
    fn deref(&self) -> &Self::Target { &self.super_ }
}

impl std::ops::DerefMut for ChpxTable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl IDisposable for ChpxTable {
    fn dispose(&self) {}
}

impl TextRopeListener for ChpxTable {
    fn beforeInsertText(&mut self, var1: i32, var2: i32, var3: i32) {
        self.super_.beforeInsertText(var1, var2, var3);
    }
    fn afterInsertText(&mut self, var1: i32, var2: i32, var3: i32) {
        self.super_.afterInsertText(var1, var2, var3);
    }
    fn beforeRemoveText(&mut self, var1: i32, var2: i32) {
        ChpxTable::beforeRemoveText(self, var1, var2);
    }
    fn afterRemoveText(&mut self, var1: i32, var2: i32) {
        self.super_.afterRemoveText(var1, var2);
    }
}

impl Head {
    pub fn Head(table: *mut ChpxTable) -> Self {
      let mut e = Entry::Entry(table, -1, None);
      Self { super_: e }
    }
}

impl std::ops::Deref for Head {
    type Target = Entry;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl std::ops::DerefMut for Head {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl Entry {
    pub fn Entry(table: *mut ChpxTable, cp: i32, data: Option<i32>) -> Self {
      let cp_table = if table.is_null() {
            ptr::null_mut()
        } else {
            unsafe { &mut (*table).super_ as *mut CPTable }
        };
      Self {
            super_: CPTableEntry::Entry(cp_table, cp, data),
            table,
        }
    }

    pub fn position(&self) -> i32 {
        self.super_.position()
    }

    pub fn endCp(&self) -> i32 {
        if self.isHead() {
            panic!("Do NOT call this method on Head node");
        }
        self.super_.endCp()
    }

    pub fn range(&self) -> i64 {
        if self.isHead() {
            panic!("Do NOT call this method on Head node");
        }
        self.super_.range()
    }

    pub fn length(&self) -> i32 {
        if self.isHead() {
            panic!("Do NOT call this method on Head node");
        }
        self.super_.length()
    }

    pub fn isEnd(&self) -> bool {
        self.isHead()
    }

    pub fn isLast(&self) -> bool {
        if self.isHead() {
            return false;
        }
        self.super_.isLast()
    }

    pub fn isHead(&self) -> bool {
        self.super_.isHead()
    }

    pub fn getProperty(&self) -> PropertySet {
        if self.isHead() {
            panic!("Do NOT call this method on Head node");
        }
        self.super_.getProperty()
    }

    pub fn setProperty(&mut self, prop: Option<&PropertySet>) {
        if self.isHead() {
            panic!("Do NOT call this method on Head node");
        }
        self.super_.setProperty(prop);
    }

    pub fn getPropertySetId(&self) -> i32 {
        if self.isHead() {
            return 0;
        }
        self.super_.getPropertySetId()
    }

    pub fn getNext(&self) -> Entry {
      entry_from_cp(self.table, self.super_.getNext())
    }

    pub fn getPrev(&self) -> Entry {
      entry_from_cp(self.table, self.super_.getPrev())
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

impl IChpxEntry for Entry {
    fn getPrev(&self) -> Option<Box<dyn IChpxEntry>> { Some(Box::new(Entry::getPrev(self))) }
    fn getNext(&self) -> Option<Box<dyn IChpxEntry>> { Some(Box::new(Entry::getNext(self))) }
    fn getProperty(&self) -> Option<&PropertySet> {
      if self.isHead() {
        return None;
      }
      Some(Box::leak(Box::new(Entry::getProperty(self))))
    }
    fn setProperty(&mut self, var1: Option<&PropertySet>) { Entry::setProperty(self, var1); }
}
