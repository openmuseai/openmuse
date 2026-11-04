#![allow(non_snake_case)]
#![allow(dead_code)]

// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
use stubs::java::lang::Object::Object;

use super::TableManagerUtil::{KTableCell, KTableRow};

// Java: public class KTableManager
pub struct KTableManager {
    // Java field: Document mDocument
    mDocument: *const (dyn Document + 'static),
}

struct ParaFlags {
    in_table: bool,
    level: i32,
    row_end: bool,
    cell_end: bool,
}

impl KTableManager {
    // Java: KTableManager(Document document)
    fn new(document: &dyn Document) -> Self {
        Self {
            mDocument: unsafe {
                std::mem::transmute::<*const dyn Document, *const (dyn Document + 'static)>(
                    document as *const dyn Document,
                )
            },
        }
    }

    // Java: KTableManager create(Document document)
    pub fn create(document: &dyn Document) -> Self {
        // Java: return new KTableManager(document);
        Self::new(document)
    }

    fn document(&self) -> &dyn Document {
        unsafe { &*self.mDocument }
    }

    fn papx(&self) -> Option<&dyn IPapxTable> {
        self.document().getPapxTable().map(|p| unsafe { &*p })
    }

    fn seek(&self, cp: i32) -> Option<Box<dyn PapxEntry>> {
        IPapxTable::seek(self.papx()?, cp)
    }

    fn char_at(&self, cp: i32) -> char {
        // Java: document.getTextRope().charAt(break_cp)
        let Some(rope) = self.document().getTextRope() else {
            return '\0';
        };
        unsafe { &mut *rope }.charAt(cp)
    }

    fn flags(prop: &PropertySet, assign_in_table: bool) -> ParaFlags {
        let mut in_table = prop.getBoolean__int_boolean(223, false);
        let mut level = prop.getInt__int_int(224, 1);
        let mut row_end = prop.getBoolean__int_boolean(225, false);
        let mut cell_end = prop.getBoolean__int_boolean(226, false);
        // Java: if (property.contains(237))
        if prop.contains(237) {
            if let Some(value) = prop.getValue(237) {
                if let Some(revision) = value.as_any().downcast_ref::<Revision>() {
                    if let Some(revision_property) = revision.getProperty() {
                        if assign_in_table {
                            in_table = revision_property.getBoolean__int_boolean(223, in_table);
                        } else {
                            let _ = revision_property.getBoolean__int_boolean(223, in_table);
                        }
                        level = revision_property.getInt__int_int(224, level);
                        row_end = revision_property.getBoolean__int_boolean(225, row_end);
                        cell_end = revision_property.getBoolean__int_boolean(226, cell_end);
                    }
                }
            }
        }
        ParaFlags {
            in_table,
            level,
            row_end,
            cell_end,
        }
    }

    // Java: IPapxTable.Entry findNextTableRowStart(int cp)
    pub fn findNextTableRowStart(&self, cp: i32) -> Option<Box<dyn PapxEntry>> {
        let mut para = self.seek(cp)?;
        // Java: for (; !para.isEnd(); para = PapxEntry::getNext(para.as_ref()))
        while !para.isEnd() {
            let Some(property) = PapxEntry::getProperty(para.as_ref()) else {
                break;
            };
            let flags = Self::flags(property, true);
            // Java: if (inTable) break;
            if flags.in_table {
                break;
            }
            let Some(next) = PapxEntry::getNext(para.as_ref()) else {
                break;
            };
            para = next;
        }
        Some(para)
    }

    // Java: int findNextRowStart(int start, int end, int level)
    pub fn findNextRowStart(&self, start: i32, end: i32, level: i32) -> i32 {
        let Some(mut para) = self.seek(start) else {
            return end;
        };
        while !para.isEnd() && para.position() < end {
            if let Some(prop) = PapxEntry::getProperty(para.as_ref()) {
                let flags = Self::flags(prop, true);
                // Java: if (inTable && nParaLevel >= level) return para.position();
                if flags.in_table && flags.level >= level {
                    return para.position();
                }
            }
            let Some(next) = PapxEntry::getNext(para.as_ref()) else {
                break;
            };
            para = next;
        }
        end
    }

    // Java: int getRowStart(int cp, int level)
    pub fn getRowStart(&self, cp: i32, level: i32) -> i32 {
        let Some(mut para) = self.seek(cp).and_then(|p| PapxEntry::getPrev(p.as_ref())) else {
            return 0;
        };
        while !para.isHead() {
            if let Some(prop) = PapxEntry::getProperty(para.as_ref()) {
                let flags = Self::flags(prop, true);
                // Java: if (!inTable || nParaLevel < level || nParaLevel == level && isRowEnd)
                if !flags.in_table || flags.level < level || (flags.level == level && flags.row_end) {
                    return PapxEntry::getNext(para.as_ref()).map(|n| n.position()).unwrap_or(0);
                }
            }
            let Some(prev) = PapxEntry::getPrev(para.as_ref()) else {
                break;
            };
            para = prev;
        }
        0
    }

    // Java: int getRowEnd(int cp, int level)
    pub fn getRowEnd(&self, cp: i32, level: i32) -> i32 {
        // Java: if (!this.isInTable(cp, level)) return cp;
        if !self.isInTable(cp, level) {
            return cp;
        }
        let Some(mut para) = self.seek(cp) else {
            return self.document().getLength();
        };
        while !para.isEnd() {
            if let Some(prop) = PapxEntry::getProperty(para.as_ref()) {
                let flags = Self::flags(prop, true);
                if flags.level == level && flags.row_end {
                    return PapxEntry::getNext(para.as_ref()).map(|n| n.position()).unwrap_or(cp);
                }
            }
            let Some(next) = PapxEntry::getNext(para.as_ref()) else {
                break;
            };
            para = next;
        }
        self.document().getLength()
    }

    // Java: int getMinLevel(int start, int end)
    pub fn getMinLevel(&self, start: i32, end: i32) -> i32 {
        let mut min_level = i32::MAX;
        let Some(mut s) = self.seek(start) else {
            return min_level;
        };
        let Some(mut e) = self.seek(end) else {
            return min_level;
        };
        if !e.isEnd() && e.position() < end {
            if let Some(next) = PapxEntry::getNext(e.as_ref()) {
                e = next;
            }
        }
        while s.position() != e.position() || s.isEnd() != e.isEnd() {
            let Some(prop) = PapxEntry::getProperty(s.as_ref()) else {
                break;
            };
            let flags = Self::flags(prop, true);
            if !flags.in_table {
                return 0;
            }
            if flags.level < min_level {
                min_level = flags.level;
            }
            let Some(next) = PapxEntry::getNext(s.as_ref()) else {
                break;
            };
            s = next;
        }
        min_level
    }

    // Java: int getSameCellDeepestLevel(int start, int end)
    pub fn getSameCellDeepestLevel(&self, start: i32, end: i32) -> i32 {
        let mut same_level = i32::MAX;
        let Some(mut s) = self.seek(start) else {
            return 0;
        };
        let Some(mut e) = self.seek(end) else {
            return 0;
        };
        if !e.isEnd() && e.position() < end {
            if let Some(next) = PapxEntry::getNext(e.as_ref()) {
                e = next;
            }
        }
        while s.position() != e.position() || s.isEnd() != e.isEnd() {
            let Some(prop) = PapxEntry::getProperty(s.as_ref()) else {
                return 0;
            };
            let flags = Self::flags(prop, true);
            if !flags.in_table {
                return 0;
            }
            if flags.level != same_level {
                if same_level != i32::MAX {
                    return 0;
                }
                same_level = flags.level;
            }
            let next_is_end = PapxEntry::getNext(s.as_ref())
                .map(|n| n.position() == e.position() && n.isEnd() == e.isEnd())
                .unwrap_or(false);
            if self.isCellEnd(s.as_ref(), s.endCp() - 1, same_level) && !next_is_end {
                return 0;
            }
            let Some(next) = PapxEntry::getNext(s.as_ref()) else {
                break;
            };
            s = next;
        }
        if same_level == i32::MAX { 0 } else { same_level }
    }

    // Java: boolean isSafePara(IPapxTable.Entry para)
    pub fn isSafePara(&self, para: &dyn PapxEntry) -> bool {
        // Java: IPapxTable.Entry next = para.getNext();
        let Some(next) = PapxEntry::getNext(para) else {
            return false;
        };
        // Java: if (next.isEnd()) return false;
        if next.isEnd() {
            return false;
        }
        // Java: return !tableCorrector.hasError() || tableCorrector.getMinErrorCp() >= paraEnd;
        // rust: DocumentCorrector.getTableCorrector 仍停车。docx 导入没有表格纠错错误，等价于 !hasError()。
        let _ = next.position();
        true
    }

    // Java: TableManagerUtil.KTableRow createRow(int row_start, int level, boolean ioMode)
    pub fn createRow(&self, row_start: i32, level: i32, io_mode: bool) -> KTableRow {
        let Some(para) = self.seek(row_start) else {
            return KTableRow::init(row_start);
        };
        let mut row = KTableRow::init(row_start);
        // Java: if (para.isEnd()) return row;
        if para.isEnd() {
            return row;
        }
        // Java: else if (ioMode && !this.isSafePara(para)) return row;
        if io_mode && !self.isSafePara(para.as_ref()) {
            return row;
        }
        let mut cell_start = row_start;
        loop {
            // Java: for (KTableCell cell = this.createCell(...); cell != null; cell = this.createCell(...))
            let Some(cell) = self.createCell(cell_start, level, io_mode) else {
                break;
            };
            // Java: if (!cell.isComplete()) { cell.recycle(); return row; }
            if !cell.isComplete() {
                return row;
            }
            let length = cell.mLength;
            row.appendChild(cell_start, cell);
            cell_start += length;
        }
        // Java: row.mLength = this.mDocument.getPapxTable().seek(cell_start).endCp() - row_start;
        if let Some(end_para) = self.seek(cell_start) {
            row.mLength = end_para.endCp() - row_start;
        }
        // Java: row.mIsComplete = true;
        row.mIsComplete = true;
        row
    }

    // Java: KTableCell createCell(int cell_start, int level, boolean ioMode)
    fn createCell(&self, cell_start: i32, level: i32, io_mode: bool) -> Option<KTableCell> {
        let mut cell = KTableCell::init(cell_start);
        let Some(mut para) = self.seek(cell_start) else {
            return Some(cell);
        };
        if para.isEnd() {
            return Some(cell);
        }
        if io_mode && !self.isSafePara(para.as_ref()) {
            return Some(cell);
        }
        let Some(prop) = PapxEntry::getProperty(para.as_ref()) else {
            return Some(cell);
        };
        let mut flags = Self::flags(prop, true);
        // Java: if (isRowEnd && nParaLevel == level) { cell.recycle(); return null; }
        if flags.row_end && flags.level == level {
            return None;
        }
        while !self.isCellEnd(para.as_ref(), para.endCp() - 1, level) {
            if flags.level > level {
                let nested = self.createRow(para.position(), level + 1, io_mode);
                if !nested.isComplete() {
                    return Some(cell);
                }
                let end = nested.getEnd();
                cell.appendChild(para.position(), nested);
                let Some(next_para) = self.seek(end) else {
                    return Some(cell);
                };
                para = next_para;
            } else {
                let Some(next_para) = PapxEntry::getNext(para.as_ref()) else {
                    return Some(cell);
                };
                para = next_para;
            }
            if io_mode && !self.isSafePara(para.as_ref()) {
                return Some(cell);
            }
            let Some(prop) = PapxEntry::getProperty(para.as_ref()) else {
                return Some(cell);
            };
            // Java: 循环里修订分支丢掉了 inTable 的赋值，只更新 level 和 isRowEnd。
            flags = Self::flags(prop, false);
            flags.in_table = prop.getBoolean__int_boolean(223, false);
        }
        // Java: cell.mLength = para.endCp() - cell_start;
        cell.mLength = para.endCp() - cell_start;
        // Java: if (cell.mLength > 1 && isRowEnd) --cell.mLength;
        if cell.mLength > 1 && flags.row_end {
            cell.mLength -= 1;
        }
        cell.mIsComplete = true;
        Some(cell)
    }

    // Java: boolean isInTable(int cp, int level)
    pub fn isInTable(&self, cp: i32, level: i32) -> bool {
        let Some(para) = self.seek(cp) else {
            return false;
        };
        // Java: if (para.isEnd()) return false;
        if para.isEnd() {
            return false;
        }
        let Some(prop) = PapxEntry::getProperty(para.as_ref()) else {
            return false;
        };
        let flags = Self::flags(prop, true);
        // Java: return inTable && nParaLevel >= level;
        flags.in_table && flags.level >= level
    }

    // Java: boolean isCellEnd(Document document, IPapxTable.Entry paragraph, int break_cp, int level)
    fn isCellEnd(&self, paragraph: &dyn PapxEntry, break_cp: i32, level: i32) -> bool {
        let Some(prop) = PapxEntry::getProperty(paragraph) else {
            return false;
        };
        let flags = Self::flags(prop, true);
        // Java: if (nParaLevel != level) return false;
        if flags.level != level {
            return false;
        }
        let c = self.char_at(break_cp);
        // Java: return c == 7 || c == '\r' && isCellEnd;
        c == '\u{0007}' || (c == '\r' && flags.cell_end)
    }
}
