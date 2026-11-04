// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::KRange;
// Java: import cn.wps.moffice.writer.core.table.CellsItem
// rust: `core::table::CellsItem` 未登记（`KColRange`/`KTableRangeBase` 闭包属 T7-5b）
//   ⇒ 本函数体走 `todo!("DEFERRED_NON_FUNCTION")`，导入以注释保留以对齐 Java 门面。
// Java: import cn.wps.moffice.writer.core.table.KTableRangeBase
// Java: import cn.wps.moffice.writer.data.table.CellInfo
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
// Disambiguate nested Entry traits (IChpxTable/IPapxTable/ITable 各自声明同名 `Entry`)。
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry as PapxEntry;
use crate::cn::wps::moffice::writer::data::ITable::Entry as ITableEntry;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
// Java: import java.util.ArrayList
// rust: `java.util.ArrayList` → `Vec`（仓库既有约定；见 `RANGE`/`CellsItem` 同类处置）。

// java public class TableTool
pub struct TableTool;

impl TableTool {
    // java public static boolean isInChildTable(PropertySet property, int level)
    pub fn isInChildTable(property: &PropertySet, level: i32) -> bool {
        // java return property.getBoolean(223, false) && property.getInt(224, 1) > level;
        property.getBoolean__int_boolean(223, false) && property.getInt__int_int(224, 1) > level
    }

    // java public static boolean isInTable(PropertySet property, int level)
    pub fn isInTable__propertyset_int(property: &PropertySet, level: i32) -> bool {
        // java return property.getBoolean(223, false) && property.getInt(224, 1) >= level;
        property.getBoolean__int_boolean(223, false) && property.getInt__int_int(224, 1) >= level
    }

    // java public static boolean isInTable(Document document, int cp)
    pub fn isInTable(document: &mut dyn Document, cp: i32) -> bool {
        // java IPapxTable.Entry paragraph = document.getPapxTable().seek(cp);
        let papxTable = match document.getPapxTable() {
            Some(t) => t,
            None => return false,
        };
        let paragraph = match IPapxTable::seek(unsafe { &*papxTable }, cp) {
            Some(p) => p,
            None => return false,
        };
        // java if (paragraph.isEnd())
        if ITableEntry::isEnd(&*paragraph) {
            // java return false;
            return false;
        }
        // java else
        else {
            // java PropertySet prop = paragraph.getProperty();
            let prop = PapxEntry::getProperty(&*paragraph);
            // java return prop.getBoolean(223, false);
            match prop {
                Some(p) => p.getBoolean__int_boolean(223, false),
                None => false,
            }
        }
    }

    // java public static boolean isCellEnd(Document mDocument, int cp, int level)
    pub fn isCellEnd__document_int_int(mDocument: &mut dyn Document, cp: i32, level: i32) -> bool {
        // java IPapxTable.Entry paragraph = mDocument.getPapxTable().seek(cp);
        let papxTable = match mDocument.getPapxTable() {
            Some(t) => t,
            None => return false,
        };
        let paragraph = match IPapxTable::seek(unsafe { &*papxTable }, cp) {
            Some(p) => p,
            None => return false,
        };
        // java PropertySet prop = paragraph.getProperty();
        // java if (prop == null)
        // java prop = PropertySet.empty;
        // rust: Java 侧 `prop` 被 `PropertySet.empty` 兜底后**非空**；Rust 侧门面返回 `Option`，
        //   故按 Java 语义在 `None` 时回退到 `PropertySet::empty()`。
        let propOwned;
        let prop: &PropertySet = match PapxEntry::getProperty(&*paragraph) {
            Some(p) => p,
            None => {
                propOwned = PropertySet::empty();
                &propOwned
            }
        };
        // java if (prop.getInt(224, 1) != level)
        if prop.getInt__int_int(224, 1) != level {
            // java return false;
            return false;
        }
        // java else if (prop.getBoolean(225, false))
        else if prop.getBoolean__int_boolean(225, false) {
            // java return false;
            return false;
        }
        // java else
        else {
            // java char c = mDocument.charAt(cp);
            let c = mDocument.charAt(cp);
            // java if (level == 1)
            if level == 1 {
                // java return c == 7;
                return c == '\u{7}';
            }
            // java else
            else {
                // java return c == '\r' && prop.getBoolean(226, false);
                return c == '\r' && prop.getBoolean__int_boolean(226, false);
            }
        }
    }

    // java public static boolean isRowEnd(Document document, int cp)
    pub fn isRowEnd__document_int(document: &mut dyn Document, cp: i32) -> bool {
        // java IPapxTable.Entry paragraph = document.getPapxTable().seek(cp);
        let papxTable = match document.getPapxTable() {
            Some(t) => t,
            None => return false,
        };
        let paragraph = match IPapxTable::seek(unsafe { &*papxTable }, cp) {
            Some(p) => p,
            None => return false,
        };
        // java PropertySet prop = paragraph.getProperty();
        let prop = match PapxEntry::getProperty(&*paragraph) {
            Some(p) => p,
            None => return false,
        };
        // java boolean inTable = prop.getBoolean(223, false);
        let inTable = prop.getBoolean__int_boolean(223, false);
        // java boolean rowEnd = prop.getBoolean(225, false);
        let rowEnd = prop.getBoolean__int_boolean(225, false);
        // java if (!inTable && !rowEnd)
        if !inTable && !rowEnd {
            // java return false;
            return false;
        }
        // java else
        else {
            // java int level = prop.getInt(224, 1);
            let level = prop.getInt__int_int(224, 1);
            // java char c = document.charAt(cp);
            let c = document.charAt(cp);
            // java if (c == 7 && level == 1 && rowEnd)
            if c == '\u{7}' && level == 1 && rowEnd {
                // java return true;
                return true;
            }
            // java else
            else {
                // java return c == '\r' && rowEnd;
                return c == '\r' && rowEnd;
            }
        }
    }

    // java public static boolean isCellEnd(Document document, int cp)
    pub fn isCellEnd__document_int(document: &mut dyn Document, cp: i32) -> bool {
        // java IPapxTable.Entry paragraph = document.getPapxTable().seek(cp);
        let papxTable = match document.getPapxTable() {
            Some(t) => t,
            None => return false,
        };
        let paragraph = match IPapxTable::seek(unsafe { &*papxTable }, cp) {
            Some(p) => p,
            None => return false,
        };
        // java PropertySet prop = paragraph.getProperty();
        let prop = match PapxEntry::getProperty(&*paragraph) {
            Some(p) => p,
            None => return false,
        };
        // java boolean inTable = prop.getBoolean(223, false);
        let inTable = prop.getBoolean__int_boolean(223, false);
        // java boolean cellEnd = prop.getBoolean(226, false);
        let cellEnd = prop.getBoolean__int_boolean(226, false);
        // java if (!inTable && !cellEnd)
        if !inTable && !cellEnd {
            // java return false;
            return false;
        }
        // java else if (prop.getBoolean(225, false))
        else if prop.getBoolean__int_boolean(225, false) {
            // java return false;
            return false;
        }
        // java else
        else {
            // java int level = prop.getInt(224, 1);
            let level = prop.getInt__int_int(224, 1);
            // java char c = document.charAt(cp);
            let c = document.charAt(cp);
            // java if (c == 7 && level == 1)
            if c == '\u{7}' && level == 1 {
                // java return true;
                return true;
            }
            // java else
            else {
                // java return c == '\r' && cellEnd;
                return c == '\r' && cellEnd;
            }
        }
    }

    // java public static boolean isRowEnd(Document mDocument, int cp, int level)
    pub fn isRowEnd__document_int_int(mDocument: &mut dyn Document, cp: i32, level: i32) -> bool {
        // java IPapxTable.Entry paragraph = mDocument.getPapxTable().seek(cp);
        let papxTable = match mDocument.getPapxTable() {
            Some(t) => t,
            None => return false,
        };
        let paragraph = match IPapxTable::seek(unsafe { &*papxTable }, cp) {
            Some(p) => p,
            None => return false,
        };
        // java PropertySet prop = paragraph.getProperty();
        let prop = match PapxEntry::getProperty(&*paragraph) {
            Some(p) => p,
            None => return false,
        };
        // java if (prop.getInt(224, 1) != level)
        if prop.getInt__int_int(224, 1) != level {
            // java return false;
            return false;
        }
        // java else
        else {
            // java char ch = mDocument.charAt(cp);
            let ch = mDocument.charAt(cp);
            // java if (level == 1)
            if level == 1 {
                // java return ch == 7 && prop.getBoolean(225, false);
                return ch == '\u{7}' && prop.getBoolean__int_boolean(225, false);
            }
            // java else
            else {
                // java return ch == '\r' && prop.getBoolean(225, false);
                return ch == '\r' && prop.getBoolean__int_boolean(225, false);
            }
        }
    }

    // java public static boolean isCellEndOrRowEnd(Document document, int cp)
    pub fn isCellEndOrRowEnd(document: &mut dyn Document, cp: i32) -> bool {
        // java IPapxTable.Entry paragraph = document.getPapxTable().seek(cp);
        let papxTable = match document.getPapxTable() {
            Some(t) => t,
            None => return false,
        };
        let paragraph = match IPapxTable::seek(unsafe { &*papxTable }, cp) {
            Some(p) => p,
            None => return false,
        };
        // java if (paragraph.isEnd())
        if ITableEntry::isEnd(&*paragraph) {
            // java return false;
            return false;
        }
        // java else
        else {
            // java PropertySet prop = paragraph.getProperty();
            let prop = match PapxEntry::getProperty(&*paragraph) {
                Some(p) => p,
                None => return false,
            };
            // java boolean inTable = prop.getBoolean(223, false);
            let inTable = prop.getBoolean__int_boolean(223, false);
            // java boolean cellEnd = prop.getBoolean(226, false);
            let cellEnd = prop.getBoolean__int_boolean(226, false);
            // java boolean rowEnd = prop.getBoolean(225, false);
            let rowEnd = prop.getBoolean__int_boolean(225, false);
            // java if (!inTable && !cellEnd && !rowEnd)
            if !inTable && !cellEnd && !rowEnd {
                // java return false;
                return false;
            }
            // java else
            else {
                // java int level = prop.getInt(224, 1);
                let level = prop.getInt__int_int(224, 1);
                // java char c = document.charAt(cp);
                let c = document.charAt(cp);
                // java if (c == 7 && level == 1)
                if c == '\u{7}' && level == 1 {
                    // java return true;
                    return true;
                }
                // java else if (c == '\r' && cellEnd)
                else if c == '\r' && cellEnd {
                    // java return true;
                    return true;
                }
                // java else
                else {
                    // java return c == '\r' && rowEnd;
                    return c == '\r' && rowEnd;
                }
            }
        }
    }

    // java private static void check_table(Document document, IPapxTable.Entry paragraph, int level)
    fn check_table(_document: &mut dyn Document, paragraph: &dyn PapxEntry, level: i32) {
        // java PropertySet p = PapxEntry::getProperty(&*paragraph);
        // java if (!p.getBoolean(223, false) || p.getInt(224, 1) < level)
        // java throw new RuntimeException("You must call the caller of check_table() after some check");
        let ok = match PapxEntry::getProperty(&*paragraph) {
            Some(p) => p.getBoolean__int_boolean(223, false) && p.getInt__int_int(224, 1) >= level,
            None => false,
        };
        if !ok {
            panic!("You must call the caller of check_table() after some check");
        }
    }

    // java public static IPapxTable.Entry getCellStart(Document document, IPapxTable.Entry paragraph, int level)
    pub fn getCellStart<'a>(
        document: &mut dyn Document,
        paragraph: &'a dyn PapxEntry,
        level: i32,
    ) -> &'a dyn PapxEntry {
        // java check_table(document, paragraph, level);
        Self::check_table(document, paragraph, level);
        // java IPapxTable.Entry pStart = paragraph;
        let mut pStart: &dyn PapxEntry = paragraph;
        // java while(true)
        loop {
            // java IPapxTable.Entry prev = pStart.getPrev();
            let prev = match PapxEntry::getPrev(&*pStart) {
                Some(p) => p,
                None => break,
            };
            // java if (ITableEntry::isHead(&*prev))
            if ITableEntry::isHead(&*prev) {
                // java break;
                break;
            }
            // java pStart = prev;
            pStart = Box::leak(prev) as &dyn PapxEntry;
            // java int end = prev.endCp();
            let end = ITableEntry::endCp(&*pStart);
            // java if (!isInTable(prev.getProperty(), level) || isRowEnd(document, end - 1, level) || isCellEnd(document, end - 1, level))
            let notInTable = match PapxEntry::getProperty(&*pStart) {
                Some(p) => !Self::isInTable__propertyset_int(p, level),
                None => true,
            };
            if notInTable
                || Self::isRowEnd__document_int_int(document, end - 1, level)
                || Self::isCellEnd__document_int_int(document, end - 1, level)
            {
                // java pStart = prev.getNext();
                if let Some(next) = PapxEntry::getNext(&*pStart) {
                    pStart = Box::leak(next) as &dyn PapxEntry;
                }
                // java break;
                break;
            }
        }
        // java return pStart;
        pStart
    }

    // java public static IPapxTable.Entry getCellBreak(Document document, IPapxTable.Entry paragraph, int level)
    pub fn getCellBreak<'a>(
        document: &mut dyn Document,
        paragraph: &'a dyn PapxEntry,
        level: i32,
    ) -> &'a dyn PapxEntry {
        // java check_table(document, paragraph, level);
        Self::check_table(document, paragraph, level);
        // java IPapxTable.Entry pEnd = paragraph;
        let mut pEnd: &dyn PapxEntry = paragraph;
        // java do
        loop {
            // java int end = ITableEntry::endCp(&*pEnd);
            let end = ITableEntry::endCp(&*pEnd);
            // java if (isCellEnd(document, end - 1, level))
            if Self::isCellEnd__document_int_int(document, end - 1, level) {
                // java return pEnd;
                return pEnd;
            }
            // java pEnd = PapxEntry::getNext(&*pEnd);
            match PapxEntry::getNext(&*pEnd) {
                Some(n) => pEnd = Box::leak(n) as &dyn PapxEntry,
                None => break,
            }
            // java while(!ITableEntry::isEnd(&*pEnd));
            if ITableEntry::isEnd(&*pEnd) {
                break;
            }
        }
        // java throw new TableException("Invalid table: no cell end");
        panic!("Invalid table: no cell end");
    }

    // java public static Long getCellRange(Document document, int cp, int level)
    pub fn getCellRange(document: &mut dyn Document, cp: i32, level: i32) -> Option<i64> {
        // java if (level <= 0)
        if level <= 0 {
            // java return null;
            return None;
        }
        // java else
        else {
            // java IPapxTable pPapxTable = document.getPapxTable();
            let pPapxTable = match document.getPapxTable() {
                Some(t) => t,
                None => return None,
            };
            let papxTable: &dyn IPapxTable = unsafe { &*pPapxTable };
            // java IPapxTable.Entry iter = pPapxTable.seek(cp);
            let iter = match IPapxTable::seek(papxTable, cp) {
                Some(i) => i,
                None => return None,
            };
            // java check_table(document, iter, level);
            Self::check_table(document, &*iter, level);
            // java IPapxTable.Entry pStart = iter;
            let iterRef: &dyn PapxEntry = Box::leak(iter);
            let mut pStart: &dyn PapxEntry = iterRef;
            // java while(true)
            loop {
                // java IPapxTable.Entry prev = pStart.getPrev();
                let prev = match PapxEntry::getPrev(&*pStart) {
                    Some(p) => p,
                    None => break,
                };
                // java if (ITableEntry::isHead(&*prev))
                if ITableEntry::isHead(&*prev) {
                    // java break;
                    break;
                }
                // java pStart = prev;
                pStart = Box::leak(prev) as &dyn PapxEntry;
                // java int end = prev.endCp();
                let end = ITableEntry::endCp(&*pStart);
                // java if (!isInTable(prev.getProperty(), level) || isRowEnd(document, end - 1, level) || isCellEnd(document, end - 1, level))
                let notInTable = match PapxEntry::getProperty(&*pStart) {
                    Some(p) => !Self::isInTable__propertyset_int(p, level),
                    None => true,
                };
                if notInTable
                    || Self::isRowEnd__document_int_int(document, end - 1, level)
                    || Self::isCellEnd__document_int_int(document, end - 1, level)
                {
                    // java pStart = prev.getNext();
                    if let Some(next) = PapxEntry::getNext(&*pStart) {
                        pStart = Box::leak(next) as &dyn PapxEntry;
                    }
                    // java break;
                    break;
                }
            }
            // java int start = ITableEntry::position(&*pStart);
            let start = ITableEntry::position(&*pStart);
            // java IPapxTable.Entry pEnd = iter;
            let mut pEnd: &dyn PapxEntry = iterRef;
            // java do
            loop {
                // java int end = ITableEntry::endCp(&*pEnd);
                let end = ITableEntry::endCp(&*pEnd);
                // java if (isCellEnd(document, end - 1, level))
                if Self::isCellEnd__document_int_int(document, end - 1, level) {
                    // java return RANGEFactory.getRANGEObject(start, end);
                    return Some(RANGEFactory::getRANGEObject(start, end));
                }
                // java pEnd = PapxEntry::getNext(&*pEnd);
                match PapxEntry::getNext(&*pEnd) {
                    Some(n) => pEnd = Box::leak(n) as &dyn PapxEntry,
                    None => break,
                }
                // java while(!ITableEntry::isEnd(&*pEnd));
                if ITableEntry::isEnd(&*pEnd) {
                    break;
                }
            }
            // java throw new TableException("Invalid table: no cell end");
            panic!("Invalid table: no cell end");
        }
    }

    // java public static IPapxTable.Entry getRowStart(Document document, IPapxTable.Entry paragraph, int level)
    pub fn getRowStart<'a>(
        document: &mut dyn Document,
        paragraph: &'a dyn PapxEntry,
        level: i32,
    ) -> &'a dyn PapxEntry {
        // java check_table(document, paragraph, level);
        Self::check_table(document, paragraph, level);
        // java IPapxTable.Entry pStart = paragraph;
        let mut pStart: &dyn PapxEntry = paragraph;
        // java while(true)
        loop {
            // java IPapxTable.Entry prev = pStart.getPrev();
            let prev = match PapxEntry::getPrev(&*pStart) {
                Some(p) => p,
                None => break,
            };
            // java if (ITableEntry::isHead(&*prev))
            if ITableEntry::isHead(&*prev) {
                // java break;
                break;
            }
            // java pStart = prev;
            pStart = Box::leak(prev) as &dyn PapxEntry;
            // java int end = prev.endCp();
            let end = ITableEntry::endCp(&*pStart);
            // java if (!isInTable(prev.getProperty(), level) || isRowEnd(document, end - 1, level))
            let notInTable = match PapxEntry::getProperty(&*pStart) {
                Some(p) => !Self::isInTable__propertyset_int(p, level),
                None => true,
            };
            if notInTable || Self::isRowEnd__document_int_int(document, end - 1, level) {
                // java pStart = prev.getNext();
                if let Some(next) = PapxEntry::getNext(&*pStart) {
                    pStart = Box::leak(next) as &dyn PapxEntry;
                }
                // java break;
                break;
            }
        }
        // java return pStart;
        pStart
    }

    // java public static IPapxTable.Entry getRowBreak(Document document, IPapxTable.Entry paragraph, int level)
    pub fn getRowBreak<'a>(
        document: &mut dyn Document,
        paragraph: &'a dyn PapxEntry,
        level: i32,
    ) -> &'a dyn PapxEntry {
        // java check_table(document, paragraph, level);
        Self::check_table(document, paragraph, level);
        // java IPapxTable.Entry pEnd = paragraph;
        let mut pEnd: &dyn PapxEntry = paragraph;
        // java do
        loop {
            // java int end = ITableEntry::endCp(&*pEnd);
            let end = ITableEntry::endCp(&*pEnd);
            // java if (isRowEnd(document, end - 1, level))
            if Self::isRowEnd__document_int_int(document, end - 1, level) {
                // java return pEnd;
                return pEnd;
            }
            // java pEnd = PapxEntry::getNext(&*pEnd);
            match PapxEntry::getNext(&*pEnd) {
                Some(n) => pEnd = Box::leak(n) as &dyn PapxEntry,
                None => break,
            }
            // java while(!ITableEntry::isHead(&*pEnd));
            if ITableEntry::isHead(&*pEnd) {
                break;
            }
        }
        // java throw new TableException("");
        panic!();
    }

    // java public static Long getRowRange(Document document, int cp, int level)
    pub fn getRowRange(document: &mut dyn Document, cp: i32, level: i32) -> Option<i64> {
        // java if (level <= 0)
        if level <= 0 {
            // java return null;
            return None;
        }
        // java else
        else {
            // java IPapxTable paragraphs = document.getPapxTable();
            let paragraphs = match document.getPapxTable() {
                Some(t) => t,
                None => return None,
            };
            let papxTable: &dyn IPapxTable = unsafe { &*paragraphs };
            // java IPapxTable.Entry iter = paragraphs.seek(cp);
            let iter = match IPapxTable::seek(papxTable, cp) {
                Some(i) => i,
                None => return None,
            };
            // java check_table(document, iter, level);
            Self::check_table(document, &*iter, level);
            // java PropertySet property = PapxEntry::getProperty(&*iter);
            let property = match PapxEntry::getProperty(&*iter) {
                Some(p) => p,
                None => return None,
            };
            // java if (!property.getBoolean(223, false))
            if !property.getBoolean__int_boolean(223, false) {
                // java return null;
                return None;
            }
            // java else
            else {
                // java IPapxTable.Entry pStart = iter;
                let iterRef: &dyn PapxEntry = Box::leak(iter);
                let mut pStart: &dyn PapxEntry = iterRef;
                // java while(true)
                loop {
                    // java IPapxTable.Entry prev = pStart.getPrev();
                    let prev = match PapxEntry::getPrev(&*pStart) {
                        Some(p) => p,
                        None => break,
                    };
                    // java if (ITableEntry::isHead(&*prev))
                    if ITableEntry::isHead(&*prev) {
                        // java break;
                        break;
                    }
                    // java pStart = prev;
                    pStart = Box::leak(prev) as &dyn PapxEntry;
                    // java int end = prev.endCp();
                    let end = ITableEntry::endCp(&*pStart);
                    // java if (!isInTable(prev.getProperty(), level) || isRowEnd(document, end - 1, level))
                    let notInTable = match PapxEntry::getProperty(&*pStart) {
                        Some(p) => !Self::isInTable__propertyset_int(p, level),
                        None => true,
                    };
                    if notInTable || Self::isRowEnd__document_int_int(document, end - 1, level) {
                        // java pStart = prev.getNext();
                        if let Some(next) = PapxEntry::getNext(&*pStart) {
                            pStart = Box::leak(next) as &dyn PapxEntry;
                        }
                        // java break;
                        break;
                    }
                }
                // java int start = ITableEntry::position(&*pStart);
                let start = ITableEntry::position(&*pStart);
                // java IPapxTable.Entry pEnd = iter;
                let mut pEnd: &dyn PapxEntry = iterRef;
                // java do
                loop {
                    // java int end = ITableEntry::endCp(&*pEnd);
                    let end = ITableEntry::endCp(&*pEnd);
                    // java if (isRowEnd(document, end - 1, level))
                    if Self::isRowEnd__document_int_int(document, end - 1, level) {
                        // java return RANGEFactory.getRANGEObject(start, end);
                        return Some(RANGEFactory::getRANGEObject(start, end));
                    }
                    // java pEnd = PapxEntry::getNext(&*pEnd);
                    match PapxEntry::getNext(&*pEnd) {
                        Some(n) => pEnd = Box::leak(n) as &dyn PapxEntry,
                        None => break,
                    }
                    // java while(!ITableEntry::isHead(&*pEnd));
                    if ITableEntry::isHead(&*pEnd) {
                        break;
                    }
                }
                // java throw new TableException("");
                panic!();
            }
        }
    }

    // java private static boolean isParagraphBreak(Document document, IChpxTable.Entry frag)
    fn isParagraphBreak(document: &mut dyn Document, frag: &dyn ChpxEntry) -> bool {
        // java IPapxTable pPapxTable = document.getPapxTable();
        let pPapxTable = match document.getPapxTable() {
            Some(t) => t,
            None => return false,
        };
        // java int cp = frag.position();
        let cp = frag.position();
        // java return pPapxTable.seek(cp).endCp() - 1 == cp;
        match IPapxTable::seek(unsafe { &*pPapxTable }, cp) {
            Some(e) => e.endCp() - 1 == cp,
            None => false,
        }
    }

    // java public static Long preprocessTableBeforeSelect(Document document, int start, int end)
    #[deprecated]
    pub fn preprocessTableBeforeSelect(
        document: &mut dyn Document,
        start: i32,
        end: i32,
    ) -> Option<i64> {
        // java IPapxTable pPapxTable = document.getPapxTable();
        let _pPapxTable = document.getPapxTable();
        // java Long pPreprocessedRange = null;
        let mut pPreprocessedRange: Option<i64> = None;
        // java int normalparaEnd = start;
        let _normalparaEnd: i32 = start;
        // java boolean hasNormalPara = false;
        let _hasNormalPara: bool = false;
        // java int[] offsetStart = new int[1];
        let _offsetStart: Vec<i32> = vec![0; 1];
        // java IChpxTable.Entry startFrag = document.getPieceTable().fragmentAtPosition(start, offsetStart);
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int[] offsetEnd = new int[1];
        let _offsetEnd: Vec<i32> = vec![0; 1];
        // java IChpxTable.Entry endFrag = document.getPieceTable().fragmentAtPosition(end, offsetEnd);
        // java if (startFrag != endFrag) {
        // java IChpxTable.Entry next = startFrag;
        // java for(int tempEnd = start + (startFrag.length() - offsetStart[0]); next != endFrag; tempEnd += next.length()) {
        // java if (isParagraphBreak(document, next)) {
        // java IPapxTable.Entry para = pPapxTable.seek(next.position());
        // java boolean isInTable = PapxEntry::getProperty(&*para).getBoolean(223, false);
        // java if (isInTable) {
        // java if (hasNormalPara) { pPreprocessedRange = RANGEFactory.getRANGEObject(start, normalparaEnd); break; }
        // java boolean isTableEnd = PapxEntry::getProperty(&*para).getBoolean(226, false) || PapxEntry::getProperty(&*para).getBoolean(225, false);
        // java if (!isTableEnd) { isTableEnd = 7 == document.charAt(para.endCp() - 1); }
        // java if (isTableEnd) { pPreprocessedRange = RANGEFactory.getRANGEObject(start, tempEnd - 1); break; }
        // java } else { hasNormalPara = true; normalparaEnd = tempEnd; }
        // java }
        // java next = next.getNext();
        // java }
        // java if (pPreprocessedRange == null && hasNormalPara && !next.isHead()) {
        // java IPapxTable.Entry para = pPapxTable.seek(next.position());
        // java boolean isInTable = PapxEntry::getProperty(&*para).getBoolean(223, false);
        // java if (isInTable) { pPreprocessedRange = RANGEFactory.getRANGEObject(start, normalparaEnd); }
        // java }
        // java }
        */
        // java return pPreprocessedRange;
        pPreprocessedRange
    }

    // java public static ArrayList<Long> getColumRanges(Document document, KTableRangeBase tableRange)
    //
    /* DEFERRED_NON_FUNCTION —— 形参 `KTableRangeBase` 与其依赖的
     *   `data::table` 子树（`TableInfo`/`RowInfo`/`CellsItem`）整体延后：
     *   该子树多为自动生成桩，且经 `core::PropertyTool` 拉入 `core::shape`。
     *   与 docx 文本解析→排版→渲染必径无关（仅交互式表格列范围查询）。
     *   **无调用者**（全仓库仅在 `core/table/Tools.rs` 的延后代码中被提及）。 */
    pub fn getColumRanges(
        _document: &mut dyn Document,
        _tableRange: Option<&dyn std::any::Any>,
    ) -> Vec<Option<i64>> {
        // java ArrayList<Long> columRanges = new ArrayList();
        let columRanges: Vec<Option<i64>> = Vec::new();
        // java return columRanges;
        return columRanges;
    }

    // java public static int getTableLevel(Document document, IPapxTable.Entry para)
    pub fn getTableLevel(_document: &mut dyn Document, para: &dyn PapxEntry) -> i32 {
        // java return PapxEntry::getProperty(&*para).getInt(224, 0);
        match PapxEntry::getProperty(&*para) {
            Some(p) => p.getInt__int_int(224, 0),
            None => 0,
        }
    }

    // java public static CellInfo getLastCell(KRange range)
    pub fn getLastCell(
        _range: &KRange,
    // rust: ★T7 —— 返回类型 `CellInfo` 属延后子树；唯一调用点
    //   （`KFootnoteEndnote.fixRange` 的 TABLECOLUMN 分支）已随之延后，
    //   故返回擦除为 `()`。
    ) -> Option<()> {
        // java TableInfos tableInfos = range.getSubDocument().getTableInfos();
        // java TableInfo tableInfo = tableInfos.getTableInfoByRange(range.getEnd(), range.getEnd());
        // java if (tableInfo == null)
        // java return null;
        // java RowInfo tableRow = tableInfo.getRowByCP(range.getEnd() - 1);
        // java if (tableRow == null)
        // java return null;
        // java CellInfo cellInfo = tableRow.getCellByCP(range.getEnd() - 1);
        // java if (cellInfo.isVertMergeCell()) {
        // java while(!cellInfo.isFirstVertMergeCell()) { cellInfo = cellInfo.getPrevVertMergeCell(); }
        // java }
        // java return cellInfo;
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        */
    }
}
