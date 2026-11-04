// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{
    Entry as IPapxTableEntry, IPapxTable,
};
// Java: import cn.wps.moffice.writer.data.table.ITableManager
use crate::cn::wps::moffice::writer::data::table::ITableManager::*;
// Java: import cn.wps.moffice.writer.data.table.KTableManager
use crate::cn::wps::moffice::writer::data::table::KTableManager::*;
// Java: import cn.wps.moffice.writer.data.table.ObjPools
use crate::cn::wps::moffice::writer::data::table::ObjPools::*;
// Java: import cn.wps.moffice.writer.data.table.TableManagerUtil
use crate::cn::wps::moffice::writer::data::table::TableManagerUtil::{KTableCell, KTableRow, TableManagerUtil};
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::*;
// Java: import cn.wps.moffice.writer.data.value.DefTable
use crate::cn::wps::moffice::writer::data::value::DefTable::*;
// Java: import cn.wps.moffice.writer.data.value.TableCell
use crate::cn::wps::moffice::writer::data::value::TableCell::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.data.table.tree.TreeTableCache
use crate::cn::wps::moffice::writer::data::table::tree::TreeTableCache::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;
// Java: import cn.wps.moffice.writer.data.table.tree.TableInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::*;

// java private static final String TAG = null;
const TAG: &str = "";
// java private static final long serialVersionUID = 1L;
const serialVersionUID: i64 = 1;

// java public class IOCache
#[derive(Default)]
pub struct IOCache {
    // java Document mDocument;
    pub(crate) mDocument: Option<Box<dyn Document>>,
    // java KTableManager mTableManager;
    pub(crate) mTableManager: Option<KTableManager>,
    // java TreeTableCache mTableCache;
    pub(crate) mTableCache: Option<TreeTableCache>,
    // java TableManagerUtil.KTableCell mResolvedRows = ObjPools.getInstance().obtainTableCell().init(0);
    pub(crate) mResolvedRows: Option<KTableCell>,
    // java ArrayList<TableManagerUtil.TableManagerUtil::KTableRow> mUnResolvedRows = new ArrayList();
    pub(crate) mUnResolvedRows: Option<Vec<KTableRow>>,
}

// java private static class PositionUnsafeException extends Exception
#[derive(Default)]
pub struct PositionUnsafeException {
    // java Exception super_
    pub(crate) super_: Exception,
}

// java public class IOCache
impl IOCache {
    // java   IOCache(TreeTableCache tableCache)
    pub fn IOCache(tableCache: &TreeTableCache) -> Self {
      // java this.mTableCache = tableCache;
      Self { mDocument: self.mTableCache.mDocument, mTableManager: self.mTableCache.mTableManager, mTableCache: tableCache, mResolvedRows: Default::default(), mUnResolvedRows: Default::default() }
      // java this.mDocument = this.mTableCache.mDocument;
      // java this.mTableManager = this.mTableCache.mTableManager;
    }

    // java  void collectTo(int cp)
    pub fn collectTo(&mut self, cp: i32) {
          // java this.doCollectTo(cp);
          self.doCollectTo(cp);
          // java PositionUnsafeException var3
          todo!();
    }

    // java  void doCollectTo(int cp)
    pub fn doCollectTo(&mut self, cp: i32) {
      // java int ioInvalidCP = this.getIoInvalidCp();
      let mut ioInvalidCP: i32 = self.getIoInvalidCp();
      // java if (ioInvalidCP < this.mDocument.getLength())
      if ioInvalidCP < self.mDocument.getLength() {
        // java do (ioInvalidCP <= cp)
        loop {
            // java if (!this.isInTable(ioInvalidCP))
            if !self.isInTable(ioInvalidCP) {
              // java this.apply(true);
              self.apply(true);
              // java IPapxTable.Entry para = this.findNextTableRowStart(ioInvalidCP);
              let mut para: Box<dyn IPapxTableEntry> = self.findNextTableRowStart(ioInvalidCP);
              // java this.mTableCache.mInvalidCP = ioInvalidCP = para.position();
              self.mTableCache.mInvalidCP = { ioInvalidCP = para.position(); ioInvalidCP };
            }
            // java else
            else {
              // java do (this.isInTable(ioInvalidCP))
              loop {
                  // java TableManagerUtil.TableManagerUtil::KTableRow row = this.mTableManager.createRow(ioInvalidCP, 1, true);
                  let mut row: TableManagerUtil::KTableRow = self.mTableManager.createRow__int_int_boolean(ioInvalidCP, 1, true);
                  // java if (!row.isComplete())
                  if !row.isComplete() {
                    // java row.recycle();
                    row.recycle();
                    // java return;
                    return;
                  }
                  // java this.accept(row);
                  self.accept(&row);
                  // java ioInvalidCP = row.getEnd();
                  ioInvalidCP = row.getEnd();
                  // java if (this.mTableCache.mInvalidCP > cp)
                  if self.mTableCache.mInvalidCP > cp {
                    // java return;
                    return;
                  }
              }
              // java this.apply(true);
              self.apply(true);
            }
        }
      }
    }

    // java private boolean isInTable(int cp)
    pub fn isInTable(&self, cp: i32) -> bool {
      // java IPapxTable.Entry para = this.mDocument.getPapxTable().seek(cp);
      let mut para: Box<dyn IPapxTableEntry> = self.mDocument.getPapxTable().seek(cp);
      // java if (para.isEnd())
      if para.isEnd() {
        // java throw new PositionUnsafeException();
        panic!("{}", PositionUnsafeException::new());
      }
      // java else if (!this.mTableManager.isSafePara(para))
      else if !self.mTableManager.isSafePara(&para) {
        // java throw new PositionUnsafeException();
        panic!("{}", PositionUnsafeException::new());
      }
      // java else
      else {
        // java PropertySet prop = para.getProperty();
        let mut prop: PropertySet = para.getProperty();
        // java return prop.getBoolean(223, false);
        return prop.getBoolean(223, false);
      }
    }

    // java private IPapxTable.Entry findNextTableRowStart(int cp)
    pub fn findNextTableRowStart(&self, cp: i32) -> Box<dyn IPapxTableEntry> {
      // java IPapxTable.Entry para;
      let mut para: Box<dyn IPapxTableEntry> = Box::new(()) as Box<dyn IPapxTableEntry>;
      para = self.mDocument.getPapxTable().seek(cp);
      // java for(para = this.mDocument.getPapxTable().seek(cp) !para.isEnd() && this.mTableManager.isSafePara(para) para = para.getNext())
      while !para.isEnd() && self.mTableManager.isSafePara(&para) {
        // java PropertySet property = para.getProperty();
        let mut property: PropertySet = para.getProperty();
        // java boolean inTable = property.getBoolean(223, false);
        let mut inTable: bool = property.getBoolean(223, false);
        // java if (inTable)
        if inTable {
          // java break;
          break;
        }
      }
      // java return para;
      return para;
    }

    // java private synchronized int getIoInvalidCp()
    pub fn getIoInvalidCp(&mut self) -> i32 {
      // java int ioInvalidCP;
      let mut ioInvalidCP: i32;
      // java if (!this.mUnResolvedRows.isEmpty())
      if !self.mUnResolvedRows.is_empty() {
        // java TableManagerUtil.TableManagerUtil::KTableRow lastRow = (TableManagerUtil.TableManagerUtil::KTableRow)this.mUnResolvedRows.get(this.mUnResolvedRows.size() - 1);
        let mut lastRow: TableManagerUtil::KTableRow = (self.mUnResolvedRows.get((self.mUnResolvedRows.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) - 1)).downcast_ref::<TableManagerUtil::KTableRow>().unwrap();
        // java ioInvalidCP = lastRow.getEnd();
        ioInvalidCP = lastRow.getEnd();
      }
      // java else if (!this.mResolvedRows.isEmpty())
      else if !self.mResolvedRows.is_empty() {
        // java TableManagerUtil.TableManagerUtil::KTableRow lastRow = this.mResolvedRows.end().getPrev();
        let mut lastRow: TableManagerUtil::KTableRow = self.mResolvedRows.end().getPrev();
        // java ioInvalidCP = lastRow.getEnd();
        ioInvalidCP = lastRow.getEnd();
      }
      // java else
      else {
        // java ioInvalidCP = this.mTableCache.mInvalidCP;
        ioInvalidCP = self.mTableCache.mInvalidCP;
      }
      // java return ioInvalidCP;
      return ioInvalidCP;
    }

    // java private void accept(TableManagerUtil.TableManagerUtil::KTableRow row)
    pub fn accept(&mut self, row: &TableManagerUtil::KTableRow) {
      // java DefTable finalDefTable = (DefTable)row.getFinalProperty().getObject(306, DefTable.empty);
      let mut finalDefTable: DefTable = (row.getFinalProperty().getObject(306, DefTable::empty)).downcast_ref::<DefTable>().unwrap();
      // java int merged = 0;
      let mut merged: i32 = 0;
      // java int firstMerged = 0;
      let mut firstMerged: i32 = 0;
      let mut i: i32 = finalDefTable.getCellCount() - 1;
      // java for(int i = finalDefTable.getCellCount() - 1; i >= 0 --i)
      while i >= 0 {
        // java TableCell tc = finalDefTable.getTableCell(i);
        let mut tc: TableCell = finalDefTable.getTableCell(i);
        // java int vMerged = tc == null ? 0 : tc.getVertMerge();
        let mut vMerged: i32 = (if tc.is_none() { 0 } else { tc.getVertMerge() });
        // java if (vMerged == 1)
        if vMerged == 1 {
          // java ++merged;
          { let _tmp = merged; merged += 1; _tmp };
        }
        // java else if (vMerged == 3)
        else if vMerged == 3 {
          // java ++firstMerged;
          { let _tmp = firstMerged; firstMerged += 1; _tmp };
        }
      }
      // java if (merged == 0)
      if merged == 0 {
        // java this.resolved();
        self.resolved();
        // java this.appendRow(row);
        self.appendRow(&row);
        // java if (firstMerged == 0)
        if firstMerged == 0 {
          // java this.resolved();
          self.resolved();
        }
      }
      // java else
      else {
        // java this.appendRow(row);
        self.appendRow(&row);
      }
    }

    // java private synchronized void appendRow(TableManagerUtil.TableManagerUtil::KTableRow row)
    pub fn appendRow(&mut self, row: &TableManagerUtil::KTableRow) {
      // java this.mUnResolvedRows.add(row);
      self.mUnResolvedRows.add(row);
      // java if (!this.mResolvedRows.isEmpty())
      if !self.mResolvedRows.is_empty() {
        // java this.apply(false);
        self.apply(false);
      }
    }

    // java synchronized void resolved()
    pub fn resolved(&mut self) {
      // java int i = 0;
      let mut i: i32 = 0;
      let mut n: i32 = (self.mUnResolvedRows.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int n = this.mUnResolvedRows.size(); i < n; ++i)
      while i < n {
        // java TableManagerUtil.TableManagerUtil::KTableRow row = (TableManagerUtil.TableManagerUtil::KTableRow)this.mUnResolvedRows.get(i);
        let mut row: TableManagerUtil::KTableRow = (self.mUnResolvedRows.get(i)).downcast_ref::<TableManagerUtil::KTableRow>().unwrap();
        // java this.mResolvedRows.appendChild(row.getStart(), row);
        self.mResolvedRows.appendChild(row.getStart(), row);
      }
      // java this.mUnResolvedRows.clear();
      self.mUnResolvedRows.clear();
    }

    // java synchronized void clear()
    pub fn clear(&mut self) {
      // java this.mResolvedRows.clear();
      self.mResolvedRows.clear();
      // java for(TableManagerUtil.TableManagerUtil::KTableRow row : this.mUnResolvedRows)
      for row in self.mUnResolvedRows.iter() {
        // java row.recycle();
        row.recycle();
      }
      // java this.mUnResolvedRows.clear();
      self.mUnResolvedRows.clear();
    }

    // java synchronized void apply(boolean applyAllRows)
    pub fn apply(&mut self, applyAllRows: bool) {
      // java if (applyAllRows)
      if applyAllRows {
        // java this.resolved();
        self.resolved();
      }
      // java TableInfoNode lastTable = (TableInfoNode)this.mTableCache.mTree.rightmost();
      let mut lastTable: TableInfoNode = (self.mTableCache.mTree.rightmost()).downcast_ref::<TableInfoNode>().unwrap();
      // java ITableManager.IRowEntry row = this.mResolvedRows.begin();
      let mut row: ITableManager::IRowEntry = self.mResolvedRows.begin();
      // java if (row.isValid() && !lastTable.isHead() && row.getStart() == lastTable.getEndCP())
      if row.isValid() && !lastTable.isHead() && row.getStart() == lastTable.getEndCP() {
        // java ITableManager.IRowEntry lastRow = lastTable.init(row, 1, false, 0);
        let mut lastRow: ITableManager::IRowEntry = lastTable.init(row, 1, false, 0);
        // java if (lastRow != null)
        if lastRow.is_some() {
          // java this.mTableCache.mInvalidCP = lastRow.getEnd();
          self.mTableCache.mInvalidCP = lastRow.getEnd();
          // java row = lastRow.getNext();
          row = lastRow.getNext();
        }
      }
      // java while (row.isValid())
      while row.isValid() {
        // java if (!lastTable.isHead())
        if !lastTable.isHead() {
          // java lastTable.setComplete(true);
          lastTable.setComplete(true);
        }
        // java int tableStart = row.getStart();
        let mut tableStart: i32 = row.getStart();
        // java TableInfoNode newTable = (TableInfoNode)this.mTableCache.createSubNode();
        let mut newTable: TableInfoNode = (self.mTableCache.createSubNode()).downcast_ref::<TableInfoNode>().unwrap();
        // java newTable.setComplete(false);
        newTable.setComplete(false);
        // java row = newTable.init(row, 1, false, 0);
        row = newTable.init(row, 1, false, 0);
        // java this.mTableCache.mTree.insertRight(lastTable, newTable, tableStart);
        self.mTableCache.mTree.insertRight(lastTable, newTable, tableStart);
        // java this.mTableCache.mInvalidCP = row.getEnd();
        self.mTableCache.mInvalidCP = row.getEnd();
        // java lastTable = newTable;
        lastTable = newTable;
        // java row = row.getNext();
        row = row.getNext();
      }
      // java if (!lastTable.isHead())
      if !lastTable.isHead() {
        // java if (applyAllRows)
        if applyAllRows {
          // java lastTable.setComplete(true);
          lastTable.setComplete(true);
        }
        // java else
        else {
          // java ITableManager.IRowEntry lastRow = row.getPrev();
          let mut lastRow: ITableManager::IRowEntry = row.getPrev();
          // java TableManagerUtil.TableManagerUtil::KTableRow nextRow = (TableManagerUtil.TableManagerUtil::KTableRow)this.mUnResolvedRows.get(0);
          let mut nextRow: TableManagerUtil::KTableRow = (self.mUnResolvedRows.get(0)).downcast_ref::<TableManagerUtil::KTableRow>().unwrap();
          // java if (!TableTool.isInTheSameTable(this.mDocument, lastRow.getStart(), lastRow.getEnd(), nextRow.getStart(), nextRow.getEnd()))
          if !TableTool::isInTheSameTable(&self.mDocument, lastRow.getStart(), lastRow.getEnd(), nextRow.getStart(), nextRow.getEnd()) {
            // java lastTable.setComplete(true);
            lastTable.setComplete(true);
          }
        }
      }
      // java this.mResolvedRows.clear();
      self.mResolvedRows.clear();
    }

}

// java private static class PositionUnsafeException extends Exception
impl PositionUnsafeException {
    // java private  PositionUnsafeException()
    pub fn PositionUnsafeException() -> Self {
      Self { super_: Exception::new() }
    }

}

impl std::ops::Deref for PositionUnsafeException {
    type Target = Exception;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PositionUnsafeException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
