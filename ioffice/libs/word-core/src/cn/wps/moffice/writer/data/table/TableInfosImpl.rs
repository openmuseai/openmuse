#![allow(non_snake_case, dead_code, unused_imports)]
use std::cell::RefCell;

use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};
use crate::cn::wps::moffice::writer::data::table::KTableManager::KTableManager;
use crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos;
use crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode;
use crate::cn::wps::moffice::writer::data::table::tree::RowInfoNode::RowInfoNode;
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::TableInfoNode;
use crate::cn::wps::moffice::writer::data::table::tree::TreeTableCache::TreeTableCache;

/// Java `TableInfosImpl`. 构造时持有 `TreeTableCache`。
pub struct TableInfosImpl {
    // java Document mDocument;
    pub(crate) mDocument: Option<*mut (dyn Document + 'static)>,
    // java TreeTableCache mTableCache;
    mTableCache: RefCell<TreeTableCache>,
}

impl TableInfosImpl {
    // java TableInfosImpl(Document document)
    pub fn TableInfosImpl(document: &dyn Document) -> Self {
        Self {
            // java this.mDocument = document;
            mDocument: Some(unsafe {
                std::mem::transmute::<*const dyn Document, *mut (dyn Document + 'static)>(
                    document as *const dyn Document,
                )
            }),
            // java this.mTableCache = new TreeTableCache(document);
            mTableCache: RefCell::new(TreeTableCache::new(document)),
        }
    }

    pub fn new(document: &dyn Document) -> Self {
        Self::TableInfosImpl(document)
    }

    // Java: TreeTableCache is stable once TableInfosImpl is boxed.
    pub fn cache_ptr(&self) -> *mut TreeTableCache {
        self.mTableCache.as_ptr()
    }

    fn document(&self) -> Option<&dyn Document> {
        self.mDocument.map(|document| unsafe { &*document })
    }

    // Java: private TableInfo getRootTable(int startCP)
    fn root_table(&self, mut start_cp: i32) -> Option<TableInfoNode> {
        // Java: if (startCP < 0) return null;
        if start_cp < 0 {
            return None;
        }
        let document_ptr = self.mDocument?;
        let document = unsafe { &*document_ptr };
        let mut cache = self.mTableCache.borrow_mut();
        // Java: if (!this.mTableCache.canLookFast())
        if !cache.canLookFast() {
            // Java: startCP = Math.min(startCP, this.mDocument.getLength() - 1);
            start_cp = start_cp.min(document.getLength() - 1);
            // Java: return this.mTableCache.getRootTable(startCP);
            return cache.getRootTable(start_cp).cloned();
        }
        // Java: if (startCP >= this.mDocument.getLength()) return null;
        if start_cp >= document.getLength() {
            return None;
        }
        // Java: IPapxTable.Entry entry = this.mDocument.getPapxTable().seek(startCP);
        let in_table = document.getPapxTable().and_then(|papx| {
            let para = IPapxTable::seek(unsafe { &*papx }, start_cp)?;
            if para.isEnd() {
                return None;
            }
            PapxEntry::getProperty(para.as_ref()).map(|prop| prop.getBoolean__int_boolean(223, false))
        });
        // Java: if (!entry.isEnd() && entry.getProperty().getBoolean(223, false)) return getRootTable;
        if in_table == Some(true) {
            cache.getRootTable(start_cp).cloned()
        } else {
            None
        }
    }
}

impl TableInfos for TableInfosImpl {
    fn size(&self) -> i32 {
        // java return this.mTableCache.size();
        self.mTableCache.borrow_mut().size()
    }

    fn nestedTableCount(&self) -> i32 {
        let _ = self.size();
        self.mTableCache
            .borrow()
            .tables()
            .iter()
            .map(|table| table.nested_table_count())
            .sum()
    }

    fn getTableByIndex(&self, index: i32) -> Option<TableInfoNode> {
        // Java: return this.mTableCache.getTableByIndex(index);
        self.mTableCache.borrow_mut().getTableByIndex(index).cloned()
    }

    fn getTableInfoByLevel(&self, cp: i32, level: i32) -> Option<TableInfoNode> {
        // Java: if (0 >= level) return null;
        if level <= 0 {
            return None;
        }
        // Java: TableInfo rootTable = this.getRootTable(cp);
        let root = self.root_table(cp)?;
        // Java: if (1 == level) return rootTable;
        if level == 1 {
            return Some(root);
        }
        // Java: RowInfo row = rootTable.getRowByCP(cp); CellInfo cell = row.getCellByCP(cp);
        let mut cell = root.getRowByCP(cp)?.getCellByCP(cp)?.clone();
        // Java: for (child = cell.getTableByCP(cp); child != null; child = cell.getTableByCP(cp))
        loop {
            let child = cell.getTableByCP(cp)?.clone();
            // Java: if (level == childTable.getLevel()) return childTable;
            if child.getLevel() == level {
                return Some(child);
            }
            let row = child.getRowByCP(cp)?;
            let next_cell = row.getCellByCP(cp)?;
            // Java: if (null == cell) return null;
            cell = next_cell.clone();
        }
    }

    fn getRowInfoByLevel(&self, cp: i32, level: i32) -> Option<RowInfoNode> {
        // Java: TableInfo tableInfo = this.getTableInfoByLevel(cp, level);
        let table = self.getTableInfoByLevel(cp, level)?;
        // Java: return tableInfo == null ? null : tableInfo.getRowByCP(cp);
        table.getRowByCP(cp).cloned()
    }

    fn getCellInfoByLevel(&self, cp: i32, level: i32) -> Option<CellInfoNode> {
        // Java: RowInfo rowInfo = this.getRowInfoByLevel(cp, level);
        let row = self.getRowInfoByLevel(cp, level)?;
        // Java: return rowInfo == null ? null : rowInfo.getCellByCP(cp);
        row.getCellByCP(cp).cloned()
    }

    fn getTableInfoByRange(&self, start_cp: i32, end_cp: i32) -> Option<TableInfoNode> {
        // Java: TableInfo rootTable = this.getRootTable(startCP);
        let root = self.root_table(start_cp)?;
        // Java: TableInfo curTable = rootTable;
        let mut cur = root.clone();
        // Java: RowInfo row = rootTable.getRowByCP(startCP); CellInfo cell = row.getCellByCP(startCP);
        // Java: if (null == cell) return rootTable;
        let Some(mut cell) = root
            .getRowByCP(start_cp)
            .and_then(|row| row.getCellByCP(start_cp).cloned())
        else {
            return Some(root);
        };
        // Java: for (child = cell.getTableByCP(startCP); child != null && child.getEndCP() >= endCP; ...)
        loop {
            let Some(child) = cell.getTableByCP(start_cp).cloned() else {
                break;
            };
            if child.getEndCP() < end_cp {
                break;
            }
            cur = child.clone();
            let Some(next) = child
                .getRowByCP(start_cp)
                .and_then(|row| row.getCellByCP(start_cp).cloned())
            else {
                break;
            };
            cell = next;
        }
        Some(cur)
    }

    fn hasTable(&self, start_cp: i32, end_cp: i32, level: i32) -> bool {
        let Some(document_ptr) = self.mDocument else {
            return false;
        };
        let document = unsafe { &mut *document_ptr };
        // Java: RangeReadLock readLock = this.mDocument.readLock();
        let mut read_lock = document.readLock();
        // Java: int docLen = this.mDocument.getLength();
        let doc_len = document.getLength();
        // Java: int s = Math.min(startCP, endCP); int e = Math.max(startCP, endCP);
        let mut s = start_cp.min(end_cp);
        let mut e = start_cp.max(end_cp);
        // Java: s = Math.max(0, Math.min(s, docLen)); e = Math.max(0, Math.min(e, docLen));
        s = 0.max(s.min(doc_len));
        e = 0.max(e.min(doc_len));
        // Java: return tableManager.findNextRowStart(s, e, level) < e;
        let manager = KTableManager::create(document);
        let found = manager.findNextRowStart(s, e, level) < e;
        // Java: readLock.unlock();
        read_lock.unlock();
        found
    }

    fn invalidAfter(&self, cp: i32) {
        // Java: this.mTableCache.invalidAfter(cp);
        self.mTableCache.borrow_mut().invalidAfter(cp);
    }

    fn doGridAutoResize(&self) {
        // Java: this.mTableCache.doGridAutoResize(true);
        self.mTableCache.borrow_mut().doGridAutoResize(true);
    }

    fn cache_ptr(&self) -> *mut TreeTableCache {
        TableInfosImpl::cache_ptr(self)
    }
}
