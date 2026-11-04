#![allow(non_snake_case)]

// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::DocumentEvent;
// Java: import cn.wps.moffice.writer.core.DocumentListener
use crate::cn::wps::moffice::writer::core::DocumentListener::DocumentListener;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::Listener as ChpxListener;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::Listener as PapxListener;
// Java: import cn.wps.moffice.writer.data.ITable
use crate::cn::wps::moffice::writer::data::ITable::{Entry as TableEntry, Listener as TableListener};
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
// Java: import cn.wps.moffice.writer.data.table.KTableManager
use crate::cn::wps::moffice::writer::data::table::KTableManager::KTableManager;
// Java: import cn.wps.moffice.writer.data.table.tree.IOCache
use crate::cn::wps::moffice::writer::data::table::tree::IOCache::IOCache;
// Java: import cn.wps.moffice.writer.data.table.tree.TableInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::TableInfoNode;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::RebuildCache;

// Java: private class AutoResizeRange
#[derive(Default, Clone, Copy)]
struct AutoResizeRange {
    // Java field: long mRangeTimely = 0L
    mRangeTimely: i64,
    // Java field: long mRangeDelay = 0L
    mRangeDelay: i64,
    // Java field: long mLastChangeRangeInSameCell = 0L
    mLastChangeRangeInSameCell: i64,
    // Java field: boolean mNodeChanged = false
    mNodeChanged: bool,
}

struct DeepestCell {
    cell_start: i32,
    cell_end: i32,
    row_end: i32,
    table_start: i32,
}

// Java: public class TreeTableCache
pub struct TreeTableCache {
    // Java field: Document mDocument
    mDocument: *const (dyn Document + 'static),
    // Java field: IOCache mIoCache
    mIoCache: IOCache,
    // Java field: boolean mIoMode
    mIoMode: bool,
    // Java field: long mRange
    mRange: i64,
    // Java field: boolean mNodeChanged
    mNodeChanged: bool,
    // Java field: int mLength on the CellInfoNode base
    mLength: i32,
    // Java field: AutoResizeRange mAutoResizeRange
    mAutoResizeRange: AutoResizeRange,
    // Java field: Data.RebuildCache mRebuildCache
    mRebuildCache: RebuildCache,
}

impl TreeTableCache {
    // Java: public TreeTableCache(Document document)
    pub fn new(document: &dyn Document) -> Self {
        // Java: super(true);
        // Java: this.mDocument = document;
        // Java: this.mCache = this;
        // Java: this.mTableTool = new TableTool(document);
        // rust: TableTool 仍不在产品编译面（登记 §1.7.1）。
        // Java: this.mTableManager = KTableManager.create(document);
        // Java: document.getTextDocument().addTransactionListener(this);
        // Java: document.getDispatcher().addDocumentListener(this);
        // rust: 事务和文档监听仍未接线（登记 §1.7.1）。
        // Java: this.mIoCache = new IOCache(this);
        // Java: this.mIoMode = true;
        let mDocument = unsafe {
            std::mem::transmute::<*const dyn Document, *const (dyn Document + 'static)>(
                document as *const dyn Document,
            )
        };
        TreeTableCache {
            mDocument,
            mIoCache: IOCache::new(document),
            mIoMode: true,
            mRange: 0,
            mNodeChanged: false,
            mLength: 0,
            mAutoResizeRange: AutoResizeRange::default(),
            mRebuildCache: RebuildCache::default(),
        }
    }

    fn rebuild_ptr(&mut self) -> *mut RebuildCache {
        std::ptr::addr_of_mut!(self.mRebuildCache)
    }

    pub fn tables(&self) -> &[TableInfoNode] {
        self.mIoCache.tables()
    }

    // Java: boolean canLookFast()
    pub fn canLookFast(&self) -> bool {
        let document = unsafe { &*self.mDocument };
        // Java: if (this.mDocument.getTextDocument().isInIOState()) return false;
        if document
            .getTextDocument()
            .is_some_and(|text| text.isInIOState())
        {
            return false;
        }
        // Java: if (!this.mIoMode) return true;
        if !self.mIoMode {
            return true;
        }
        // Java: TableInfoNode lastTable = (TableInfoNode)this.mTree.rightmost(); return lastTable.isHead() ? true : lastTable.isComplete();
        match self.tables().last() {
            None => true,
            Some(last) => last.isComplete(),
        }
    }

    // Java: TableInfo getTableByIndex(int index)
    pub fn getTableByIndex(&mut self, index: i32) -> Option<&TableInfoNode> {
        // Java: return index >= 0 && index < this.size() ? super.getTableByIndex(index) : null;
        if index < 0 {
            return None;
        }
        let count = self.size();
        if index >= count {
            return None;
        }
        self.mIoCache.tables().get(index as usize)
    }

    // Java: TableInfo getTableByCP via getNodeByCP
    pub fn getTableByCP(&self, cp: i32) -> Option<&TableInfoNode> {
        self.mIoCache.tables().iter().find(|table| table.contains_cp(cp))
    }

    // Java: TableInfo getRootTable(int startCP)
    pub fn getRootTable(&mut self, start_cp: i32) -> Option<&TableInfoNode> {
        // Java: this.updateCache(startCP); return this.getTableByCP(startCP);
        self.updateCache(start_cp);
        self.getTableByCP(start_cp)
    }

    // Java: public int size()
    pub fn size(&mut self) -> i32 {
        let length = unsafe { &*self.mDocument }.getLength();
        // Java: this.updateCache(this.mDocument.getLength() - 1);
        self.updateCache(length - 1);
        // Java: return super.size();
        // rust: 子节点数等于 IOCache.apply 收成的顶层表。TableInfoNode 树尚未挂上。
        self.mIoCache.tables().len() as i32
    }

    // Java: private synchronized void updateCache(int cp)
    fn updateCache(&mut self, cp: i32) {
        // Java: this.commitModify();
        self.commitModify();
        // Java: if (this.mIoMode) this.mIoCache.collectTo(cp); else this.updateInNormal(cp);
        if self.mIoMode {
            let cache = self.rebuild_ptr();
            self.mIoCache.bind_rebuild(cache);
            self.mIoCache.collectTo(cp);
        } else {
            self.updateInNormal(cp);
        }
    }

    // Java: public void invalidAfter(int cp)
    pub fn invalidAfter(&mut self, mut cp: i32) {
        // Java: this.mIoCache.clear();
        self.mIoCache.clear();
        // Java: if (cp < this.mInvalidCP)
        if cp < self.mIoCache.invalid_cp() {
            // Java: if (!RANGEFactory.isEmpty(this.mRange))
            if !crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory::isEmpty(self.mRange)
            {
                // Java: cp = Math.min(RANGEFactory.getStart(this.mRange), cp); this.mRange = 0L;
                cp = cp.min(
                    crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory::getStart(
                        self.mRange,
                    ),
                );
                self.mRange = 0;
            }
            // Java: this.mInvalidCP = cp;
            self.mIoCache.set_invalid_cp(cp);
            // Java: for (lastTable = rightmost; !lastTable.isHead() && lastTable.getEndCP() > cp; erase)
            let cache = self.rebuild_ptr();
            loop {
                let start = match self.mIoCache.last_table_mut() {
                    Some(last) if last.getEndCP() > cp => last.getStartCP(),
                    _ => break,
                };
                let invalid = self.mIoCache.invalid_cp().min(start);
                self.mIoCache.set_invalid_cp(invalid);
                // Java: eraseSubNode → close() backs AutoFitInfo into RebuildCache
                if let Some(mut last) = self.mIoCache.pop_table() {
                    last.close(unsafe { &mut *cache });
                }
            }
            // Java: this.tryAppend();
            self.try_append();
        }
    }

    // Java: private void tryAppend()
    fn try_append(&mut self) {
        let document = unsafe { &*self.mDocument };
        let length = document.getLength();
        let invalid = self.mIoCache.invalid_cp();
        // Java: if (this.mInvalidCP < this.mDocument.getLength() && this.mTableManager.isInTable(...))
        let manager = crate::cn::wps::moffice::writer::data::table::KTableManager::KTableManager::create(
            document,
        );
        if invalid >= length || !manager.isInTable(invalid, 1) {
            return;
        }
        let Some(last_end) = self.mIoCache.last_table_mut().map(|table| table.getEndCP()) else {
            return;
        };
        // Java: if (!lastTable.isHead() && lastTable.getEndCP() == this.mInvalidCP)
        if last_end != invalid {
            return;
        }
        let cache = self.rebuild_ptr();
        let end = {
            let Some(last) = self.mIoCache.last_table_mut() else {
                return;
            };
            // Java: lastTable = lastTable.tryAppend(); this.mInvalidCP = lastTable.getEndCP();
            last.try_append(&manager, document, i32::MAX, cache);
            last.getEndCP()
        };
        self.mIoCache.set_invalid_cp(end);
        // Java: while (this.mTableManager.isInTable(this.mInvalidCP, 1)) create a new table
        loop {
            let cp = self.mIoCache.invalid_cp();
            if !manager.isInTable(cp, 1) {
                break;
            }
            let row = manager.createRow(cp, 1, false);
            if !row.isComplete() {
                break;
            }
            let mut new_table = TableInfoNode::new();
            let used = new_table.init(std::slice::from_ref(&row), document, 1, true, i32::MAX, cache);
            if used == 0 {
                break;
            }
            let end = new_table.getEndCP();
            if end <= cp {
                break;
            }
            self.mIoCache.set_invalid_cp(end);
            // insertRight is push: the new table is the new rightmost.
            // IOCache keeps tables private; expose push.
            self.mIoCache.push_table(new_table);
        }
    }

    // Java: public GridResizeRequest getGridResizeRequest()
    fn resize_request_is_commiting(&self) -> Option<bool> {
        let document = unsafe { &*self.mDocument };
        // Java: return this.mDocument.getTextDocument().getGridResizeRequest();
        let text = document.getTextDocument()?;
        let request = crate::cn::wps::moffice::writer::core::Document::SoftTextDocument::getGridResizeRequest(text)?;
        // Java: resizeRequest.isCommiting()
        Some(request.isCommiting())
    }

    fn request_with_retypo(&self, range: i64) {
        let document = unsafe { &*self.mDocument };
        // Java: GridResizeRequest resizeRequest = this.getGridResizeRequest();
        let Some(text) = document.getTextDocument() else {
            return;
        };
        let Some(mut request) =
            crate::cn::wps::moffice::writer::core::Document::SoftTextDocument::getGridResizeRequest(text)
        else {
            return;
        };
        // Java: resizeRequest.requestWithReTypo(range, this.mDocument);
        request.requestWithReTypo(range, document);
    }

    // Java: private void changeRange(long range, boolean applyAutoSize)
    pub fn changeRange(&mut self, range: i64, apply_auto_size: bool) {
        // Java: if (resizeRequest == null || !resizeRequest.isCommiting())
        if self.resize_request_is_commiting() != Some(true) {
            // Java: this.mRange = range;
            self.mRange = range;
            // Java: if (applyAutoSize) this.mAutoResizeRange.mRangeTimely = RANGEFactory.include(..., this.mRange);
            if apply_auto_size {
                self.mAutoResizeRange.mRangeTimely =
                    RANGEFactory::include__long_long(self.mAutoResizeRange.mRangeTimely, self.mRange);
            }
        }
    }

    // Java: void markModify(int cp)
    pub fn markModify(&mut self, cp: i32) {
        // Java: this.mIoCache.clear();
        self.mIoCache.clear();
        // Java: this.changeRange(RANGEFactory.include(this.mRange, cp), true);
        let range = RANGEFactory::include__long_int(self.mRange, cp);
        self.changeRange(range, true);
    }

    // Java: private void cleanMark()
    fn cleanMark(&mut self) {
        // Java: this.mRange = 0L; this.mNodeChanged = false;
        self.mRange = 0;
        self.mNodeChanged = false;
    }

    // Java: private void startModify()
    fn startModify(&mut self) {
        // Java: this.cleanMark();
        self.cleanMark();
    }

    // Java: public void beforeInsertText(int cp, int fc, int length)
    pub fn beforeInsertText(&mut self, cp: i32, _fc: i32, length: i32) {
        // Java: this.mIoCache.clear();
        self.mIoCache.clear();
        // Java: if (cp < this.mInvalidCP)
        if cp < self.mIoCache.invalid_cp() {
            // Java: this.mInvalidCP += length;
            let next = self.mIoCache.invalid_cp() + length;
            self.mIoCache.set_invalid_cp(next);
            // Java: this.changeRange(RANGEFactory.insertText(this.mRange, cp, length), true);
            let range = RANGEFactory::insertText(self.mRange, cp, length);
            self.changeRange(range, true);
        }
    }

    // Java: public void beforeStart(String description)
    pub fn beforeStart(&mut self, _description: &str) {
        // Java: this.startModify();
        self.startModify();
    }

    // Java: public void beforeUndo(String description)
    pub fn beforeUndo(&mut self, _description: &str) {
        // Java: this.startModify();
        self.startModify();
    }

    // Java: public void beforeRedo(String description)
    pub fn beforeRedo(&mut self, _description: &str) {
        // Java: this.startModify();
        self.startModify();
    }

    // Java: public void afterCommit(String description)
    pub fn afterCommit(&mut self, _description: &str) {
        // Java: this.commitModify(); this.doGridAutoResize(false);
        self.commitModify();
        self.doGridAutoResize(false);
    }

    // Java: public void afterRollback()
    pub fn afterRollback(&mut self) {
        // Java: this.commitModify();
        self.commitModify();
    }

    // Java: public void afterUndo(String description)
    pub fn afterUndo(&mut self, _description: &str) {
        // Java: this.commitModify(); this.doGridAutoResize(false);
        self.commitModify();
        self.doGridAutoResize(false);
    }

    // Java: public void afterRedo(String description)
    pub fn afterRedo(&mut self, _description: &str) {
        // Java: this.commitModify(); this.doGridAutoResize(false);
        self.commitModify();
        self.doGridAutoResize(false);
    }

    // Java: public void afterInsertText(int cp, int fc, int length)
    pub fn afterInsertText(&mut self, cp: i32, _fc: i32, length: i32) {
        // Java: if (cp < this.mInvalidCP) this.positionFix(cp, length, new int[1]);
        if cp < self.mIoCache.invalid_cp() {
            self.positionFix(cp, length);
        }
    }

    // Java: public void beforeRemoveText(int start, int end)
    pub fn beforeRemoveText(&mut self, start: i32, end: i32) {
        // Java: this.mIoCache.clear();
        self.mIoCache.clear();
        // Java: if (start < this.mInvalidCP)
        if start < self.mIoCache.invalid_cp() {
            // Java: this.mNodeChanged |= this.removeNodeInRange(start, end, new int[1]);
            let changed = self.removeNodeInRange(start, end);
            self.mNodeChanged |= changed;
            // Java: this.mAutoResizeRange.mNodeChanged |= this.mNodeChanged;
            self.mAutoResizeRange.mNodeChanged |= self.mNodeChanged;
        }
    }

    // Java: public void afterRemoveText(int start, int end)
    pub fn afterRemoveText(&mut self, start: i32, end: i32) {
        // Java: if (start < this.mInvalidCP)
        if start < self.mIoCache.invalid_cp() {
            if end >= self.mIoCache.invalid_cp() {
                // Java: this.mInvalidCP = start;
                self.mIoCache.set_invalid_cp(start);
            } else {
                // Java: this.mInvalidCP -= end - start;
                let next = self.mIoCache.invalid_cp() - (end - start);
                self.mIoCache.set_invalid_cp(next);
            }
            // Java: this.changeRange(RANGEFactory.removeText(this.mRange, start, end), true);
            let range = RANGEFactory::removeText(self.mRange, start, end);
            self.changeRange(range, true);
            // Java: if (RANGEFactory.isEmpty(this.mAutoResizeRange.mRangeTimely))
            if RANGEFactory::isEmpty(self.mAutoResizeRange.mRangeTimely) {
                self.mAutoResizeRange.mRangeTimely = RANGEFactory::getRANGE(start, start + 1);
            }
            // Java: this.positionFix(start, start - end, new int[1]);
            self.positionFix(start, start - end);
        }
    }

    // Java: void positionFix(int offsetFromThis, int steps, int[] offsetBuf)
    pub fn positionFix(&mut self, offset_from_this: i32, steps: i32) {
        // Java: this.mLength += steps;
        self.mLength += steps;
        let mut marks = Vec::new();
        {
            let tables = self.mIoCache.tables_mut();
            position_fix_root(tables, offset_from_this, steps, &mut marks);
        }
        for cp in marks {
            self.markModify(cp);
        }
    }

    // Java: boolean removeNodeInRange(int start, int end, int[] offsetBuf)
    pub fn removeNodeInRange(&mut self, start: i32, end: i32) -> bool {
        // Java: TreeTableCache.getStartCP() == 0, so the offsets are absolute CPs.
        let cache = self.rebuild_ptr();
        remove_root_tables(self.mIoCache.tables_mut(), start, end, cache)
    }

    // Java: private synchronized void commitModify()
    fn commitModify(&mut self) {
        let document_ptr = self.mDocument;
        let in_io = unsafe { &*document_ptr }
            .getTextDocument()
            .is_some_and(|text| text.isInIOState());
        if !self.mNodeChanged && self.isInDeepestCellNoNodeChanged(self.mRange) {
            // Java: this.mAutoResizeRange.mLastChangeRangeInSameCell = this.mRange;
            self.mAutoResizeRange.mLastChangeRangeInSameCell = self.mRange;
        } else if !RANGEFactory::isEmpty(self.mRange) {
            let start = RANGEFactory::getStart(self.mRange);
            let end = RANGEFactory::getEnd(self.mRange);
            match self.root_edit(start, end) {
                None => {
                    // Java: this.invalidAfter(RANGEFactory.getStart(this.mRange));
                    self.invalidAfter(start);
                }
                Some(edit) => {
                    // Java: this.getRebuildCache().start();
                    self.mRebuildCache.start();
                    let document = unsafe { &*document_ptr };
                    match edit {
                        RootEdit::Cache { start, end } => {
                            let _ = self.removeNodeInRange(start, end);
                            self.valid_root(document, start, end);
                        }
                        RootEdit::Table { index, start, end } => {
                            let cache = self.rebuild_ptr();
                            if let Some(table) = self.mIoCache.tables_mut().get_mut(index) {
                                let _ = table.removeNodeInRange(start, end, cache);
                                table.valid__int_int(document, start, end, cache);
                            }
                        }
                        RootEdit::Cell {
                            table_index,
                            row_index,
                            cell_index,
                            start,
                            end,
                            level,
                        } => {
                            let cache = self.rebuild_ptr();
                            if let Some(cell) = self
                                .mIoCache
                                .tables_mut()
                                .get_mut(table_index)
                                .and_then(|table| table.rows_mut().get_mut(row_index))
                                .and_then(|row| row.cells_mut().get_mut(cell_index))
                            {
                                let _ = cell.removeNodeInRange(start, end, cache);
                                cell.valid(document, start, end, level, cache);
                            }
                        }
                    }
                    self.mergeRangFormRebuildCache();
                    // Java: this.getRebuildCache().end();
                    self.mRebuildCache.end();
                    if let Some(last) = self.mIoCache.tables().last() {
                        let end_cp = last.getEndCP();
                        // Java: this.mInvalidCP = Math.max(this.mInvalidCP, lastTable.getEndCP());
                        if end_cp > self.mIoCache.invalid_cp() {
                            self.mIoCache.set_invalid_cp(end_cp);
                        }
                    }
                }
            }
        }
        // Java: this.cleanMark();
        self.cleanMark();
        // Java: if (!this.mDocument.getTextDocument().isInIOState())
        if !in_io {
            if self.mIoMode {
                // Java: this.mIoCache.apply(true);
                let cache = self.rebuild_ptr();
                self.mIoCache.bind_rebuild(cache);
                self.mIoCache.apply(true);
            }
            // Java: this.mIoMode = false; this.tryAppend();
            self.mIoMode = false;
            self.try_append();
        }
    }

    // Java: private void updateInNormal(int cp)
    fn updateInNormal(&mut self, cp: i32) {
        let document_ptr = self.mDocument;
        let document = unsafe { &*document_ptr };
        let length = document.getLength();
        let manager = KTableManager::create(document);
        loop {
            let invalid = self.mIoCache.invalid_cp();
            // Java: if (!(cp >= this.mInvalidCP && this.mInvalidCP < this.mDocument.getLength())) return;
            if !(cp >= invalid && invalid < length) {
                return;
            }
            if !manager.isInTable(invalid, 1) {
                // Java: IPapxTable.Entry para = this.mTableManager.findNextTableRowStart(this.mInvalidCP);
                let Some(para) = manager.findNextTableRowStart(invalid) else {
                    self.mIoCache.set_invalid_cp(length);
                    return;
                };
                let next = para.position();
                if para.isEnd() || next <= invalid {
                    self.mIoCache.set_invalid_cp(length);
                    return;
                }
                self.mIoCache.set_invalid_cp(next);
                continue;
            }
            loop {
                let invalid = self.mIoCache.invalid_cp();
                // Java: IRowEntry row = this.mTableManager.createRow(this.mInvalidCP, 1);
                let row = manager.createRow(invalid, 1, false);
                if !row.isComplete() {
                    return;
                }
                let mut new_table = TableInfoNode::new();
                let cache = self.rebuild_ptr();
                let used = new_table.init(std::slice::from_ref(&row), document, 1, true, i32::MAX, cache);
                if used == 0 {
                    return;
                }
                let end = new_table.getEndCP();
                if end <= invalid {
                    return;
                }
                // Java: this.mTree.insertRight(rightmost, newTable, this.mInvalidCP); this.mInvalidCP = row.getEnd();
                self.mIoCache.push_table(new_table);
                self.mIoCache.set_invalid_cp(end);
                if !manager.isInTable(end, 1) {
                    break;
                }
            }
        }
    }

    fn valid_root(&mut self, document: &dyn Document, start: i32, end: i32) {
        // Java: CellInfoNode.valid on the cache, level 1.
        let manager = KTableManager::create(document);
        let mut start_cp = start;
        while start_cp < end {
            start_cp = manager.findNextRowStart(start_cp, end, 1);
            if start_cp >= end {
                break;
            }
            let row = manager.createRow(start_cp, 1, false);
            if !row.isComplete() {
                break;
            }
            let mut table = TableInfoNode::new();
            let cache = self.rebuild_ptr();
            let used = table.init(std::slice::from_ref(&row), document, 1, true, end, cache);
            if used == 0 {
                break;
            }
            let next_cp = table.getEndCP();
            let at = table.getStartCP();
            let tables = self.mIoCache.tables_mut();
            let idx = tables
                .iter()
                .position(|nested| nested.getStartCP() >= at)
                .unwrap_or(tables.len());
            tables.insert(idx, table);
            if next_cp <= start_cp {
                break;
            }
            start_cp = next_cp;
        }
    }

    // Java: private TableNode getRootNode(int[] offsetBuf)
    fn root_edit(&mut self, mut start: i32, mut end: i32) -> Option<RootEdit> {
        let document = unsafe { &*self.mDocument };
        let manager = KTableManager::create(document);
        // Java: int rootLevel = this.mTableManager.getMinLevel(start, end);
        let root_level = manager.getMinLevel(start, end);
        let min_level = root_level.max(1);
        // Java: start = Math.min(start, getRowStart); end = Math.max(end, getRowEnd);
        start = start.min(manager.getRowStart(start, min_level));
        if end > 0 {
            end = end.max(manager.getRowEnd(end - 1, min_level));
        }
        if min_level == 1 {
            let start_table = self.table_index_at(start);
            let end_table = self.table_index_at(end.saturating_sub(1));
            if let Some(index) = start_table {
                if start_table == end_table {
                    let table = &self.mIoCache.tables()[index];
                    if let Some(row) = table.getRowByCP(start) {
                        start = row.getStartCP();
                    }
                    let table = &self.mIoCache.tables()[index];
                    if let Some(row) = table.getRowByCP(end.saturating_sub(1)) {
                        end = row.getEndCP();
                    }
                    let table_start = self.mIoCache.tables()[index].getStartCP();
                    let table_end = self.mIoCache.tables()[index].getEndCP();
                    // Java: if (rootLevel == 0) end = startTable.getEndCP();
                    if root_level == 0 {
                        end = table_end;
                    } else if start != table_start && end != table_end {
                        // Java: this.mRange = RANGEFactory.getRANGE(start, end); return startTable;
                        self.mRange = RANGEFactory::getRANGE(start, end);
                        return Some(RootEdit::Table {
                            index,
                            start: start - table_start,
                            end: end - table_start,
                        });
                    }
                }
            }
            // Java: this.changeRange(RANGEFactory.getRANGE(start, end), false); return this;
            let range = RANGEFactory::getRANGE(start, end);
            self.changeRange(range, false);
            return Some(RootEdit::Cache { start, end });
        }
        // Java: getRootCell walks down to the cell whose table level is minLevel - 1.
        let target_level = min_level - 1;
        let Some(slot) = self.cell_slot(start, target_level) else {
            return None;
        };
        let range = RANGEFactory::getRANGE(start, end);
        self.changeRange(range, false);
        let cell_start = slot.cell_start;
        Some(RootEdit::Cell {
            table_index: slot.table_index,
            row_index: slot.row_index,
            cell_index: slot.cell_index,
            start: start - cell_start,
            end: end - cell_start,
            level: target_level + 1,
        })
    }

    fn table_index_at(&self, cp: i32) -> Option<usize> {
        self.mIoCache
            .tables()
            .iter()
            .position(|table| table.contains_cp(cp))
    }

    fn cell_slot(&self, cp: i32, table_level: i32) -> Option<CellSlot> {
        let table_index = self.table_index_at(cp)?;
        let table = &self.mIoCache.tables()[table_index];
        if table.getLevel() == table_level {
            let row_index = table.rows().iter().position(|row| row.contains_cp(cp))?;
            let row = &table.rows()[row_index];
            let cell_index = row.cells().iter().position(|cell| cell.contains_cp(cp))?;
            let cell_start = row.cells()[cell_index].getStartCP();
            return Some(CellSlot {
                table_index,
                row_index,
                cell_index,
                cell_start,
            });
        }
        // Nested tables live under the level-1 cell. One more step covers level 2.
        let row_index = table.rows().iter().position(|row| row.contains_cp(cp))?;
        let cell_index = table.rows()[row_index]
            .cells()
            .iter()
            .position(|cell| cell.contains_cp(cp))?;
        let cell = &table.rows()[row_index].cells()[cell_index];
        let _ = cell.getTableByCP(cp);
        Some(CellSlot {
            table_index,
            row_index,
            cell_index,
            cell_start: cell.getStartCP(),
        })
    }

    fn mergeRangFormRebuildCache(&mut self) {
        // Java: this.mRangeTimely = RANGEFactory.include(this.mRangeTimely, getRebuildCache().getInvalidRange());
        self.mAutoResizeRange.mRangeTimely = RANGEFactory::include__long_long(
            self.mAutoResizeRange.mRangeTimely,
            self.mRebuildCache.getInvalidRange(),
        );
        // Java: if (getInvalidCount() > 1L && !RANGEFactory.isEmpty(this.mRange))
        if self.mRebuildCache.getInvalidCount() > 1 && !RANGEFactory::isEmpty(self.mRange) {
            self.mAutoResizeRange.mRangeTimely = RANGEFactory::include__long_long(
                self.mAutoResizeRange.mRangeTimely,
                self.mRange,
            );
        }
    }

    // Java: private boolean isInDeepestCellNoNodeChanged(long changeRange)
    fn isInDeepestCellNoNodeChanged(&self, change_range: i64) -> bool {
        // Java: if (RANGEFactory.isEmpty(changeRange)) return false;
        if RANGEFactory::isEmpty(change_range) {
            return false;
        }
        let start = RANGEFactory::getStart(change_range);
        let end = RANGEFactory::getEnd(change_range);
        let document = unsafe { &*self.mDocument };
        let manager = KTableManager::create(document);
        // Java: int sameCellLevel = this.mTableManager.getSameCellDeepestLevel(start, end);
        let same_level = manager.getSameCellDeepestLevel(start, end);
        if same_level == 0 {
            return false;
        }
        let Some(cell) = self.deepest_cell_detail(start) else {
            return false;
        };
        // Java: cellInfo.getParent().getParent().getLevel() == sameCellLevel
        if cell.table_level != same_level {
            return false;
        }
        let cell_range = RANGEFactory::getRANGE(cell.cell_start, cell.cell_end);
        // Java: if (!RANGEFactory.isSubset(cellInfo.getRange(), changeRange)) return false;
        if !RANGEFactory::isSubset(cell_range, change_range) {
            return false;
        }
        for (nested_start, nested_end) in cell.nested {
            // Java: if (RANGEFactory.isIntersect(table.getRange(), changeRange)) return false;
            let nested = RANGEFactory::getRANGE(nested_start, nested_end);
            if RANGEFactory::isIntersect__long_long(nested, change_range) {
                return false;
            }
        }
        true
    }

    fn deepest_cell_detail(&self, cp: i32) -> Option<CellDetail> {
        let root = self.getTableByCP(cp)?;
        detail_in_table(root, cp)
    }

    // Java: public void doGridAutoResize(boolean fourceRequest)
    pub fn doGridAutoResize(&mut self, fource_request: bool) {
        // Java: long autoSieRange = this.mAutoResizeRange.finishRangeDelay();
        let auto_size_range = self.finishRangeDelay();
        // Java: if (!RANGEFactory.isEmpty(autoSieRange))
        if !RANGEFactory::isEmpty(auto_size_range) {
            // Java: resizeRequest.requestWithReTypo(autoSieRange, this.mDocument);
            self.request_with_retypo(auto_size_range);
        } else if fource_request {
            // Java: resizeRequest.requestWithReTypo(0L, this.mDocument);
            self.request_with_retypo(0);
        }
    }

    // Java: long popRangeTimely()
    fn popRangeTimely(&mut self) -> i64 {
        // Java: long range = this.mRangeTimely; this.mRangeTimely = 0L; return range;
        let range = self.mAutoResizeRange.mRangeTimely;
        self.mAutoResizeRange.mRangeTimely = 0;
        range
    }

    // Java: long popRangeDelay()
    fn popRangeDelay(&mut self) -> i64 {
        // Java: long range = RANGEFactory.include(this.mRangeDelay, this.popRangeTimely());
        let timely = self.popRangeTimely();
        let range = RANGEFactory::include__long_long(self.mAutoResizeRange.mRangeDelay, timely);
        // Java: this.mRangeDelay = 0L;
        self.mAutoResizeRange.mRangeDelay = 0;
        range
    }

    // Java: long popLastChangeRangeInSameCell()
    fn popLastChangeRangeInSameCell(&mut self) -> i64 {
        let range = self.mAutoResizeRange.mLastChangeRangeInSameCell;
        self.mAutoResizeRange.mLastChangeRangeInSameCell = 0;
        range
    }

    // Java: long finishRangeDelay()
    fn finishRangeDelay(&mut self) -> i64 {
        // Java: long range = this.intersectAndInvalidGrid(this.popRangeDelay());
        let range = self.popRangeDelay();
        self.intersectAndInvalidGrid(range)
    }

    // Java: boolean isTableFit() via mCache.getRootTable(start).getTableProperty()
    fn isTableFit(&mut self, start_cp: i32) -> bool {
        let document = self.mDocument;
        // Java: TableInfo rootTable = this.mCache.getRootTable(this.getStartCP());
        let Some(root) = self.getRootTable(start_cp) else {
            return false;
        };
        // Java: return rootTable.getTableProperty().getBoolean(302, DefaultValue.table_fit);
        root.isTableFit(unsafe { &*document })
    }

    // Java: private CellInfo getDeepestCell(int cp)
    fn getDeepestCell(&self, cp: i32) -> Option<DeepestCell> {
        let root = self.getTableByCP(cp)?;
        deepest_in_table(root, cp)
    }

    // Java: long intersectAndInvalidGrid(long changeRange)
    fn intersectAndInvalidGrid(&mut self, mut change_range: i64) -> i64 {
        // Java: long lastChangeRangeInSameCell = this.popLastChangeRangeInSameCell();
        let last_change = self.popLastChangeRangeInSameCell();
        // Java: boolean nodeChanged = this.mNodeChanged; this.mNodeChanged = false;
        let node_changed = self.mAutoResizeRange.mNodeChanged;
        self.mAutoResizeRange.mNodeChanged = false;
        // Java: if (RANGEFactory.isEmpty(changeRange)) return 0L;
        if RANGEFactory::isEmpty(change_range) {
            return 0;
        }
        // Java: if (lastChangeRangeInSameCell != 0L && lastChangeRangeInSameCell == changeRange)
        if last_change != 0 && last_change == change_range {
            let start = RANGEFactory::getStart(change_range);
            // Java: TableInfo tableInfo = getTableByCP(start); return tableInfo != null && tableInfo.isTableFit() ? changeRange : 0L;
            if self.getTableByCP(start).is_some() && self.isTableFit(start) {
                return change_range;
            }
            return 0;
        }
        let mut change_start = RANGEFactory::getStart(change_range);
        let mut change_end = RANGEFactory::getEnd(change_range);
        // Java: if (nodeChanged)
        if node_changed {
            let prev = self
                .getTableByCP(change_start - 1)
                .map(|table| (table.getStartCP(), table.getEndCP()));
            if let Some((table_start, table_end)) = prev {
                // Java: if (tableInfo != null && tableInfo.isTableFit() && tableInfo.getEndCP() == changeStart) --changeStart;
                if table_end == change_start && self.isTableFit(table_start) {
                    change_start -= 1;
                }
            }
            if let Some(cell) = self.getDeepestCell(change_start) {
                let cell_range = RANGEFactory::getRANGE(cell.cell_start, cell.cell_end);
                // Java: cellInfo.getParent().getParent().isTableFit() && RANGEFactory.isSubset(cellInfo.getRange(), changeRange)
                if self.isTableFit(cell.table_start)
                    && RANGEFactory::isSubset(cell_range, change_range)
                {
                    // Java: changeRange = RANGEFactory.include(changeRange, cellInfo.getParent().getEndCP() - 1);
                    change_range = RANGEFactory::include__long_int(change_range, cell.row_end - 1);
                    change_start = RANGEFactory::getStart(change_range);
                    change_end = RANGEFactory::getEnd(change_range);
                }
            }
        }
        // Java: tblSize = this.mTree.size(); if (tblSize == 0) return 0L;
        let (first_start, last_end, tbl_size) = {
            let tables = self.mIoCache.tables();
            let tbl_size = tables.len();
            if tbl_size == 0 {
                return 0;
            }
            (
                tables[0].getStartCP(),
                tables[tbl_size - 1].getEndCP(),
                tbl_size,
            )
        };
        // Java: if (!(lastEnd > changeStart && firstStart < changeEnd)) return 0L;
        if !(last_end > change_start && first_start < change_end) {
            return 0;
        }
        // Java: if (firstStart >= changeStart) changeStart = firstStart; else findNextRowStart
        if first_start >= change_start {
            change_start = first_start;
        } else {
            let document = unsafe { &*self.mDocument };
            let manager = KTableManager::create(document);
            // Java: int nextRowStart = this.mTableManager.findNextRowStart(changeStart, changeEnd, 1);
            let next_row_start = manager.findNextRowStart(change_start, change_end, 1);
            if next_row_start > change_start {
                change_start = next_row_start;
            }
        }
        // Java: changeStart = Math.max(changeStart, firstStart); changeEnd = Math.min(changeEnd, lastEnd);
        change_start = change_start.max(first_start);
        change_end = change_end.min(last_end);
        // Java: if (changeEnd <= changeStart) return 0L;
        if change_end <= change_start {
            return 0;
        }
        // Java: TableInfo table = this.getTableByCP(changeStart);
        let Some(mut table_index) = self
            .mIoCache
            .tables()
            .iter()
            .position(|table| table.contains_cp(change_start))
        else {
            return 0;
        };
        // Java: for (; table != null && table.getStartCP() < changeEnd; table = mTree.get(tableIndex + 1))
        let mut find_has_fit = false;
        loop {
            let table_start = {
                let tables = self.mIoCache.tables();
                if table_index >= tables.len() {
                    break;
                }
                let table = &tables[table_index];
                if table.getStartCP() >= change_end {
                    break;
                }
                // Java: Data.TableData data = table.getData();
                // Java: if (data.mGridCols != null) data.mGridCols.invalid();
                // mGridCols 在 getGrid() 之前为 null，这里不进入 invalid。
                table.getStartCP()
            };
            // Java: if (table.isTableFit() && !findHasFit)
            if !find_has_fit && self.isTableFit(table_start) {
                find_has_fit = true;
                // Java: tableIndex = table.getStartCP(); if (tableIndex > changeStart) changeStart = tableIndex;
                if table_start > change_start {
                    change_start = table_start;
                }
            }
            // Java: tableIndex = ((TableInfoNode)table).index(); if (tableIndex >= tblSize - 1) break;
            if table_index >= tbl_size - 1 {
                break;
            }
            table_index += 1;
        }
        // Java: if (findHasFit && changeEnd > changeStart)
        if find_has_fit && change_end > change_start {
            let document = unsafe { &*self.mDocument };
            let manager = KTableManager::create(document);
            let next_row_start = manager.findNextRowStart(change_start, change_end, 1);
            if next_row_start > change_start {
                change_start = next_row_start;
            }
            if change_end <= change_start {
                return 0;
            }
            // Java: changeRange = RANGEFactory.include(0L, changeStart, changeEnd); return changeRange;
            RANGEFactory::include__long_int_int(0, change_start, change_end)
        } else {
            0
        }
    }

    fn include_entry_range(&mut self, entry: &dyn TableEntry) {
        // Java: int start = entry.position(); int end = entry.getNext().position();
        let start = entry.position();
        let Some(next) = entry.getNext() else {
            return;
        };
        let mut end = next.position();
        // Java: if (end > this.mInvalidCP) end = this.mInvalidCP;
        let invalid = self.mIoCache.invalid_cp();
        if end > invalid {
            end = invalid;
        }
        // Java: if (start < end) this.changeRange(RANGEFactory.include(this.mRange, start, end), true);
        if start < end {
            let range = RANGEFactory::include__long_int_int(self.mRange, start, end);
            self.changeRange(range, true);
        }
    }
}

struct CellDetail {
    cell_start: i32,
    cell_end: i32,
    table_level: i32,
    nested: Vec<(i32, i32)>,
}

struct CellSlot {
    table_index: usize,
    row_index: usize,
    cell_index: usize,
    cell_start: i32,
}

enum RootEdit {
    Cache { start: i32, end: i32 },
    Table { index: usize, start: i32, end: i32 },
    Cell {
        table_index: usize,
        row_index: usize,
        cell_index: usize,
        start: i32,
        end: i32,
        level: i32,
    },
}

fn detail_in_table(table: &TableInfoNode, cp: i32) -> Option<CellDetail> {
    let row = table.getRowByCP(cp)?;
    let cell = row.getCellByCP(cp)?;
    if let Some(child) = cell.getTableByCP(cp) {
        return detail_in_table(child, cp);
    }
    let nested = cell
        .tables()
        .iter()
        .map(|nested| (nested.getStartCP(), nested.getEndCP()))
        .collect();
    Some(CellDetail {
        cell_start: cell.getStartCP(),
        cell_end: cell.getEndCP(),
        table_level: table.getLevel(),
        nested,
    })
}

fn position_fix_root(tables: &mut [TableInfoNode], offset_from_this: i32, steps: i32, marks: &mut Vec<i32>) {
    if tables.is_empty() {
        return;
    }
    let edit = offset_from_this;
    let prev_i = tables.iter().rposition(|table| table.getStartCP() <= edit);
    if steps > 0 {
        if let Some(i) = prev_i {
            let within = edit - tables[i].getStartCP();
            let len = tables[i].getEndCP() - tables[i].getStartCP();
            if within < len {
                tables[i].positionFix(within, steps, marks);
            }
        }
    } else {
        let deleted = -steps;
        if let Some(i) = prev_i {
            let within = edit - tables[i].getStartCP();
            let len = tables[i].getEndCP() - tables[i].getStartCP();
            if within < len {
                let sub_deleted = deleted.min(len - within);
                tables[i].positionFix(within, -sub_deleted, marks);
            }
        }
        if let Some(upper_i) = tables.iter().position(|table| table.getStartCP() > edit) {
            let upper_off = tables[upper_i].getStartCP();
            let upper_deleted = offset_from_this + deleted - upper_off;
            if upper_deleted > 0 {
                tables[upper_i].shift_cp(upper_deleted);
                tables[upper_i].positionFix(0, -upper_deleted, marks);
            }
        }
    }
    for table in tables.iter_mut() {
        if table.getStartCP() > edit {
            table.shift_cp(steps);
        }
    }
}

fn remove_root_tables(
    tables: &mut Vec<TableInfoNode>,
    start: i32,
    end: i32,
    rebuild: *mut RebuildCache,
) -> bool {
    if tables.is_empty() || end <= start {
        return false;
    }
    let mut changed = false;
    let mut i = 0;
    while i < tables.len() {
        let lo = tables[i].getStartCP();
        let hi = tables[i].getEndCP();
        if hi <= start || lo >= end {
            i += 1;
            continue;
        }
        if lo >= start && hi <= end {
            // Java: eraseSubNode → close() then erase
            let mut table = tables.remove(i);
            if !rebuild.is_null() {
                table.close(unsafe { &mut *rebuild });
            }
            changed = true;
            continue;
        }
        let sub_start = (start - lo).max(0);
        let sub_end = (end - lo).min(hi - lo);
        changed |= tables[i].removeNodeInRange(sub_start, sub_end, rebuild);
        i += 1;
    }
    changed
}

fn deepest_in_table(table: &TableInfoNode, cp: i32) -> Option<DeepestCell> {
    // Java: RowInfo row = table.getRowByCP(cp); CellInfo cell = row.getCellByCP(cp);
    let row = table.getRowByCP(cp)?;
    let cell = row.getCellByCP(cp)?;
    // Java: for (child = cell.getTableByCP(cp); child != null; child = cell.getTableByCP(cp))
    if let Some(child) = cell.getTableByCP(cp) {
        return deepest_in_table(child, cp);
    }
    Some(DeepestCell {
        cell_start: cell.getStartCP(),
        cell_end: cell.getEndCP(),
        row_end: row.getEndCP(),
        table_start: table.getStartCP(),
    })
}

impl TableListener for TreeTableCache {
    // Java: public void beforeAddEntry(int cp, PropertySet data)
    fn beforeAddEntry(&mut self, _cp: i32, _data: Option<&PropertySet>) {
        // Java: this.mIoCache.clear();
        self.mIoCache.clear();
    }

    // Java: public void afterAddEntry(ITable.Entry entry)
    fn afterAddEntry(&mut self, entry: &dyn TableEntry) {
        self.include_entry_range(entry);
    }

    // Java: public void beforeRemoveEntry(ITable.Entry entry)
    fn beforeRemoveEntry(&mut self, entry: &dyn TableEntry) {
        // Java: this.mIoCache.clear();
        self.mIoCache.clear();
        self.include_entry_range(entry);
    }

    // Java: public void afterRemoveEntry(ITable.Entry entry)
    fn afterRemoveEntry(&mut self, _entry: &dyn TableEntry) {}

    // Java: public void beforeDataChange(ITable.Entry entry, PropertySet data)
    fn beforeDataChange(&mut self, entry: &dyn TableEntry, _data: Option<&PropertySet>) {
        // Java: this.mIoCache.clear();
        self.mIoCache.clear();
        self.include_entry_range(entry);
    }

    // Java: public void afterDataChange(ITable.Entry entry, PropertySet data)
    fn afterDataChange(&mut self, _entry: &dyn TableEntry, _data: Option<&PropertySet>) {}
}

impl PapxListener for TreeTableCache {}

impl ChpxListener for TreeTableCache {}

impl DocumentListener for TreeTableCache {
    // Java: public void update(DocumentEvent e)
    fn update(&mut self, event: &DocumentEvent) {
        // Java: if (e.getType() == 5)
        if event.getType() == 5 {
            // Java: int cp = this.mDocument.getTextRope().fc2cp(e.getOffset());
            let cp = {
                let document = unsafe { &mut *(self.mDocument as *mut (dyn Document + 'static)) };
                let Some(rope) = document.getTextRope() else {
                    return;
                };
                unsafe { &mut *rope }.fc2cp(event.getOffset())
            };
            // Java: if (cp < this.mInvalidCP) this.markModify(cp);
            if cp < self.mIoCache.invalid_cp() {
                self.markModify(cp);
            }
        } else if event.getType() == 6 {
            // Java: int cp = e.getOffset();
            let cp = event.getOffset();
            // Java: if (cp >= 0 && cp < this.mInvalidCP)
            if cp >= 0 && cp < self.mIoCache.invalid_cp() {
                // Java: this.mAutoResizeRange.mRangeDelay = RANGEFactory.include(this.mAutoResizeRange.mRangeDelay, cp);
                self.mAutoResizeRange.mRangeDelay =
                    RANGEFactory::include__long_int(self.mAutoResizeRange.mRangeDelay, cp);
            }
        }
    }
}

/// Java `TreeTableCache` registered as `INotifyTextRope.Listener` after the cache is boxed.
pub struct CacheRopeListener {
    pub cache: *mut TreeTableCache,
}

impl crate::cn::wps::collections::textrope::INotifyTextRope::Listener for CacheRopeListener {
    fn beforeInsertText(&mut self, cp: i32, fc: i32, length: i32) {
        if !self.cache.is_null() {
            unsafe { &mut *self.cache }.beforeInsertText(cp, fc, length);
        }
    }
    fn afterInsertText(&mut self, cp: i32, fc: i32, length: i32) {
        if !self.cache.is_null() {
            unsafe { &mut *self.cache }.afterInsertText(cp, fc, length);
        }
    }
    fn beforeRemoveText(&mut self, start: i32, end: i32) {
        if !self.cache.is_null() {
            unsafe { &mut *self.cache }.beforeRemoveText(start, end);
        }
    }
    fn afterRemoveText(&mut self, start: i32, end: i32) {
        if !self.cache.is_null() {
            unsafe { &mut *self.cache }.afterRemoveText(start, end);
        }
    }
}

/// Java `TreeTableCache` registered as `Transaction.Listener` after the cache is boxed.
pub struct CacheTxListener {
    pub cache: *mut TreeTableCache,
}

impl util::moffice::transaction::Transaction::Listener for CacheTxListener {
    fn beforeStart(&mut self, description: &str) {
        if !self.cache.is_null() {
            unsafe { &mut *self.cache }.beforeStart(description);
        }
    }
    fn afterCommit(&mut self, description: &str) {
        if !self.cache.is_null() {
            unsafe { &mut *self.cache }.afterCommit(description);
        }
    }
    fn afterRollback(&mut self) {
        if !self.cache.is_null() {
            unsafe { &mut *self.cache }.afterRollback();
        }
    }
    fn beforeUndo(&mut self, description: &str) {
        if !self.cache.is_null() {
            unsafe { &mut *self.cache }.beforeUndo(description);
        }
    }
    fn afterUndo(&mut self, description: &str) {
        if !self.cache.is_null() {
            unsafe { &mut *self.cache }.afterUndo(description);
        }
    }
    fn beforeRedo(&mut self, description: &str) {
        if !self.cache.is_null() {
            unsafe { &mut *self.cache }.beforeRedo(description);
        }
    }
    fn afterRedo(&mut self, description: &str) {
        if !self.cache.is_null() {
            unsafe { &mut *self.cache }.afterRedo(description);
        }
    }
}
