#![allow(non_snake_case)]

// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};
// Java: import cn.wps.moffice.writer.data.table.KTableManager
use crate::cn::wps::moffice::writer::data::table::KTableManager::KTableManager;
// Java: import cn.wps.moffice.writer.data.table.TableManagerUtil
use crate::cn::wps::moffice::writer::data::table::TableManagerUtil::KTableRow;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::TableTool;
// Java: import cn.wps.moffice.writer.data.table.tree.TableInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::TableInfoNode;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::RebuildCache;
// Java: import cn.wps.moffice.writer.data.value.DefTableBuilder
use crate::cn::wps::moffice::writer::data::value::DefTableBuilder::DefTableBuilder;

// Java: public class IOCache
pub struct IOCache {
    // Java field: Document mDocument
    mDocument: *const (dyn Document + 'static),
    // Java field: KTableManager mTableManager
    mTableManager: KTableManager,
    // Java field: TreeTableCache.mInvalidCP
    mInvalidCP: i32,
    // Java field: ArrayList<KTableRow> mUnResolvedRows
    mUnResolvedRows: Vec<KTableRow>,
    // Java field: KTableCell mResolvedRows
    mResolvedRows: Vec<KTableRow>,
    tables: Vec<TableInfoNode>,
    // Java field: TreeTableCache.mRebuildCache, bound for the duration of collect/apply
    mRebuildCache: *mut RebuildCache,
}

impl IOCache {
    // Java: IOCache(TreeTableCache tableCache)
    pub fn new(document: &dyn Document) -> Self {
        // Java: this.mDocument = this.mTableCache.mDocument;
        // Java: this.mTableManager = this.mTableCache.mTableManager;
        let mDocument = unsafe {
            std::mem::transmute::<*const dyn Document, *const (dyn Document + 'static)>(
                document as *const dyn Document,
            )
        };
        IOCache {
            mDocument,
            mTableManager: KTableManager::create(document),
            mInvalidCP: 0,
            mUnResolvedRows: Vec::new(),
            mResolvedRows: Vec::new(),
            tables: Vec::new(),
            mRebuildCache: std::ptr::null_mut(),
        }
    }

    // Java: nodes created here call TreeTableCache.getRebuildCache()
    pub fn bind_rebuild(&mut self, cache: *mut RebuildCache) {
        self.mRebuildCache = cache;
    }

    pub fn tables(&self) -> &[TableInfoNode] {
        &self.tables
    }

    pub fn tables_mut(&mut self) -> &mut Vec<TableInfoNode> {
        &mut self.tables
    }

    fn document(&self) -> &dyn Document {
        unsafe { &*self.mDocument }
    }

    fn document_length(&self) -> i32 {
        self.document().getLength()
    }

    // Java: void collectTo(int cp)
    pub fn collectTo(&mut self, cp: i32) {
        // Java: this.doCollectTo(cp);
        let _ = self.doCollectTo(cp);
        // Java: catch (PositionUnsafeException var3) {}
    }

    // Java: void doCollectTo(int cp)
    fn doCollectTo(&mut self, cp: i32) -> Result<(), ()> {
        // Java: int ioInvalidCP = this.getIoInvalidCp();
        let mut io_invalid_cp = self.getIoInvalidCp();
        // Java: if (ioInvalidCP < this.mDocument.getLength())
        let doc_len = self.document_length();
        if io_invalid_cp >= doc_len {
            return Ok(());
        }
        // Java: do { ... } while (ioInvalidCP <= cp);
        loop {
            // Java: if (!this.isInTable(ioInvalidCP))
            if !self.isInTable(io_invalid_cp)? {
                // Java: this.apply(true);
                self.apply(true);
                // Java: IPapxTable.Entry para = this.findNextTableRowStart(ioInvalidCP);
                let Some(para) = self.findNextTableRowStart(io_invalid_cp) else {
                    self.mInvalidCP = doc_len;
                    break;
                };
                // Java: this.mTableCache.mInvalidCP = ioInvalidCP = para.position();
                let next = para.position();
                if para.isEnd() || next <= io_invalid_cp {
                    self.mInvalidCP = doc_len;
                    break;
                }
                io_invalid_cp = next;
                self.mInvalidCP = io_invalid_cp;
            } else {
                // Java: do { createRow; accept; ioInvalidCP = row.getEnd(); } while (isInTable)
                loop {
                    // Java: KTableRow row = this.mTableManager.createRow(ioInvalidCP, 1, true);
                    let row = self.mTableManager.createRow(io_invalid_cp, 1, true);
                    // Java: if (!row.isComplete()) { row.recycle(); return; }
                    if !row.isComplete() {
                        return Ok(());
                    }
                    let end = row.getEnd();
                    // Java: this.accept(row);
                    self.accept(row);
                    // Java: ioInvalidCP = row.getEnd();
                    if end <= io_invalid_cp {
                        return Ok(());
                    }
                    io_invalid_cp = end;
                    // Java: if (this.mTableCache.mInvalidCP > cp) return;
                    if self.mInvalidCP > cp {
                        return Ok(());
                    }
                    if !self.isInTable(io_invalid_cp)? {
                        break;
                    }
                }
                // Java: this.apply(true);
                self.apply(true);
            }
            if io_invalid_cp > cp {
                break;
            }
        }
        Ok(())
    }

    // Java: boolean isInTable(int cp)
    fn isInTable(&self, cp: i32) -> Result<bool, ()> {
        let Some(papx) = self.document().getPapxTable() else {
            return Err(());
        };
        // Java: IPapxTable.Entry para = this.mDocument.getPapxTable().seek(cp);
        let Some(para) = IPapxTable::seek(unsafe { &*papx }, cp) else {
            return Err(());
        };
        // Java: if (para.isEnd()) throw new PositionUnsafeException();
        if para.isEnd() {
            return Err(());
        }
        // Java: if (!this.mTableManager.isSafePara(para)) throw new PositionUnsafeException();
        if !self.mTableManager.isSafePara(para.as_ref()) {
            return Err(());
        }
        // Java: return prop.getBoolean(223, false);
        let in_table = PapxEntry::getProperty(para.as_ref())
            .map(|prop| prop.getBoolean__int_boolean(223, false))
            .unwrap_or(false);
        Ok(in_table)
    }

    // Java: IPapxTable.Entry findNextTableRowStart(int cp)
    fn findNextTableRowStart(&self, cp: i32) -> Option<Box<dyn PapxEntry>> {
        let papx = self.document().getPapxTable()?;
        // Java: for (para = seek(cp); !para.isEnd() && isSafePara(para); para = para.getNext())
        let mut para = IPapxTable::seek(unsafe { &*papx }, cp)?;
        while !para.isEnd() && self.mTableManager.isSafePara(para.as_ref()) {
            // Java: boolean inTable = property.getBoolean(223, false);
            let in_table = PapxEntry::getProperty(para.as_ref())
                .map(|prop| prop.getBoolean__int_boolean(223, false))
                .unwrap_or(false);
            // Java: if (inTable) break;
            if in_table {
                break;
            }
            para = PapxEntry::getNext(para.as_ref())?;
        }
        Some(para)
    }

    // Java: int getIoInvalidCp()
    fn getIoInvalidCp(&self) -> i32 {
        // Java: if (!mUnResolvedRows.isEmpty()) return last unresolved end;
        if let Some(last) = self.mUnResolvedRows.last() {
            return last.getEnd();
        }
        // Java: else if (!mResolvedRows.isEmpty()) return last resolved end;
        if let Some(last) = self.mResolvedRows.last() {
            return last.getEnd();
        }
        // Java: return this.mTableCache.mInvalidCP;
        self.mInvalidCP
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mResolvedRows.clear();
        self.mResolvedRows.clear();
        // Java: for (KTableRow row : this.mUnResolvedRows) row.recycle();
        // Java: this.mUnResolvedRows.clear();
        self.mUnResolvedRows.clear();
    }

    pub fn invalid_cp(&self) -> i32 {
        self.mInvalidCP
    }

    pub fn set_invalid_cp(&mut self, cp: i32) {
        self.mInvalidCP = cp;
    }

    pub fn pop_table(&mut self) -> Option<TableInfoNode> {
        self.tables.pop()
    }

    pub fn push_table(&mut self, table: TableInfoNode) {
        self.tables.push(table);
    }

    pub fn last_table_mut(&mut self) -> Option<&mut TableInfoNode> {
        self.tables.last_mut()
    }

    // Java: void accept(KTableRow row)
    fn accept(&mut self, row: KTableRow) {
        // Java: DefTable finalDefTable = (DefTable)row.getFinalProperty().getObject(306, DefTable.empty);
        let prop = TableTool::getFinalRowProperty(self.document(), row.getEnd() - 1);
        let def_table = prop
            .getValue(306)
            .and_then(|value| value.as_any().downcast_ref::<DefTableBuilder>());
        // Java: int merged = 0; int firstMerged = 0;
        let mut merged = 0i32;
        let mut first_merged = 0i32;
        let cell_count = def_table.map(|table| table.getCellCount()).unwrap_or(0);
        // Java: for (int i = finalDefTable.getCellCount() - 1; i >= 0; --i)
        let mut i = cell_count - 1;
        while i >= 0 {
            // Java: TableCell tc = finalDefTable.getTableCell(i);
            // Java: int vMerged = tc == null ? 0 : tc.getVertMerge();
            let v_merged = def_table
                .and_then(|table| table.getTableCell(i))
                .map(|cell| cell.getVertMerge())
                .unwrap_or(0);
            // Java: if (vMerged == 1) ++merged; else if (vMerged == 3) ++firstMerged;
            if v_merged == 1 {
                merged += 1;
            } else if v_merged == 3 {
                first_merged += 1;
            }
            i -= 1;
        }
        // Java: if (merged == 0) { resolved(); appendRow(row); if (firstMerged == 0) resolved(); } else appendRow(row);
        if merged == 0 {
            self.resolved();
            self.appendRow(row);
            if first_merged == 0 {
                self.resolved();
            }
        } else {
            self.appendRow(row);
        }
    }

    // Java: void appendRow(KTableRow row)
    fn appendRow(&mut self, row: KTableRow) {
        // Java: this.mUnResolvedRows.add(row);
        self.mUnResolvedRows.push(row);
        // Java: if (!this.mResolvedRows.isEmpty()) this.apply(false);
        if !self.mResolvedRows.is_empty() {
            self.apply(false);
        }
    }

    // Java: void resolved()
    fn resolved(&mut self) {
        // Java: mResolvedRows.appendChild(row.getStart(), row); mUnResolvedRows.clear();
        let pending = std::mem::take(&mut self.mUnResolvedRows);
        self.mResolvedRows.extend(pending);
    }

    // Java: void apply(boolean applyAllRows)
    pub(crate) fn apply(&mut self, apply_all_rows: bool) {
        // Java: if (applyAllRows) this.resolved();
        if apply_all_rows {
            self.resolved();
        }
        let resolved = std::mem::take(&mut self.mResolvedRows);
        let document = self.mDocument;
        let mut index = 0usize;
        // Java: if (row.isValid() && !lastTable.isHead() && row.getStart() == lastTable.getEndCP())
        if index < resolved.len() {
            let extend = self
                .tables
                .last()
                .is_some_and(|last| resolved[index].getStart() == last.getEndCP());
            if extend {
                let used = {
                    let last = self.tables.last_mut().unwrap();
                    last.init(&resolved[index..], unsafe { &*document }, 1, false, 0, self.mRebuildCache)
                };
                if used > 0 {
                    // Java: this.mTableCache.mInvalidCP = lastRow.getEnd(); row = lastRow.getNext();
                    self.mInvalidCP = self.tables.last().unwrap().getEndCP();
                    index += used;
                }
            }
        }
        // Java: while (row.isValid()) { newTable.init(row, 1, false, 0); ... }
        while index < resolved.len() {
            if let Some(last) = self.tables.last_mut() {
                // Java: if (!lastTable.isHead()) lastTable.setComplete(true);
                last.setComplete(true);
            }
            let mut new_table = TableInfoNode::new();
            // Java: newTable.setComplete(false);
            new_table.setComplete(false);
            let used = new_table.init(&resolved[index..], unsafe { &*document }, 1, false, 0, self.mRebuildCache);
            if used == 0 {
                break;
            }
            // Java: this.mTableCache.mInvalidCP = row.getEnd();
            self.mInvalidCP = new_table.getEndCP();
            self.tables.push(new_table);
            index += used;
        }
        // Java: if (!lastTable.isHead()) { if (applyAllRows) lastTable.setComplete(true); else if (!isInTheSameTable) lastTable.setComplete(true); }
        let mark_complete = if self.tables.is_empty() {
            false
        } else if apply_all_rows {
            true
        } else if let (Some(last_row), Some(next_row)) =
            (resolved.last(), self.mUnResolvedRows.first())
        {
            !TableTool::isInTheSameTable(
                unsafe { &*document },
                last_row.getStart(),
                last_row.getEnd(),
                next_row.getStart(),
                next_row.getEnd(),
            )
        } else {
            false
        };
        if mark_complete {
            if let Some(last) = self.tables.last_mut() {
                last.setComplete(true);
            }
        }
        // Java: this.mResolvedRows.clear();
    }
}
