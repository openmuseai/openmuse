// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
use crate::cn::wps::util::concurrent::locks::RangeReadLock::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.table.KCellIterator
use crate::cn::wps::moffice::writer::core::table::KCellIterator::*;
// Java: import cn.wps.moffice.writer.core.table.CellsItem
use crate::cn::wps::moffice::writer::core::table::CellsItem::*;

// java public class KCellIteratorForCol implements KCellIterator
#[derive(Default)]
pub struct KCellIteratorForCol {
    // java private Document mDocument;
    mDocument: Option<Box<dyn Document>>,
    // java private ArrayList<CellsItem> mCells;
    mCells: Option<Vec<CellsItem>>,
    // java private int mItemIndex = -1;
    mItemIndex: i32,
    // java private int mIndexOfItem = -1;
    mIndexOfItem: i32,
    // java private boolean mFindNextCell = false;
    mFindNextCell: bool,
    // java private int mNextItemIndex = -1;
    mNextItemIndex: i32,
    // java private int mNextIndexOfItem = -1;
    mNextIndexOfItem: i32,
}

// java public interface KCellIterator
impl KCellIterator for KCellIteratorForCol {
    // java public boolean hasNext()
    fn hasNext(&self) -> bool {
      // java return this.findNextCell();
      return self.findNextCell();
    }

    // java public boolean next()
    fn next(&mut self) -> bool {
      // java if (this.findNextCell())
      if self.findNextCell() {
        // java this.mItemIndex = this.mNextItemIndex;
        self.mItemIndex = self.mNextItemIndex;
        // java this.mIndexOfItem = this.mNextIndexOfItem;
        self.mIndexOfItem = self.mNextIndexOfItem;
        // java this.mFindNextCell = false;
        self.mFindNextCell = false;
        // java return true;
        return true;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public int getCellEndIndex()
    fn getCellEndIndex(&self) -> i32 {
      // java if (-1 == this.mItemIndex)
      if -1 == self.mItemIndex {
        // java return -1;
        return -1;
      }
      // java else
      else {
        // java CellsItem cellItem = (CellsItem)this.mCells.get(this.mItemIndex);
        let mut cellItem: CellsItem = (self.mCells.get(self.mItemIndex)).downcast_ref::<CellsItem>().unwrap();
        // java int cellEndIndex = -1;
        let mut cellEndIndex: i32 = -1;
        // java int rowStart = RANGEFactory.getStart(cellItem.rowRange);
        let mut rowStart: i32 = RANGEFactory::getStart(cellItem.rowRange);
        // java int rowEnd = RANGEFactory.getEnd(cellItem.rowRange);
        let mut rowEnd: i32 = RANGEFactory::getEnd(cellItem.rowRange);
        // java RangeReadLock readLock = this.mDocument.getMainDocument().readLock();
        let mut readLock: RangeReadLock = self.mDocument.getMainDocument().readLock();
        // java int var12;
        let mut var12: i32;
            // java TableInfo tableInfo = this.mDocument.getTableInfos().getTableInfoByRange(rowStart, rowEnd);
            let mut tableInfo: TableInfo = self.mDocument.getTableInfos().getTableInfoByRange(rowStart, rowEnd);
            // java if (null != tableInfo)
            if tableInfo.is_some() {
              // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getStart(cellItem.rowRange));
              let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart(cellItem.rowRange));
              // java CellInfo cellInfo = rowInfo.getCellByIndex(this.mIndexOfItem);
              let mut cellInfo: CellInfo = rowInfo.getCellByIndex(self.mIndexOfItem);
              // java if (null != cellInfo)
              if cellInfo.is_some() {
                // java cellEndIndex = this.mDocument.getTextRope().cp2fc(cellInfo.getEndCP() - 1);
                cellEndIndex = self.mDocument.getTextRope().cp2fc(cellInfo.getEndCP() - 1);
              }
            }
            // java var12 = cellEndIndex;
            var12 = cellEndIndex;
              // java readLock.unlock();
              readLock.unlock();
        // java return var12;
        return var12;
      }
    }

}

// java public class KCellIteratorForCol implements KCellIterator
impl KCellIteratorForCol {
    // java public  KCellIteratorForCol(Document document, ArrayList<CellsItem> cells)
    pub fn KCellIteratorForCol(document: &dyn Document, cells: &Vec<CellsItem>) -> Self {
      // java this.mDocument = document;
      Self { mDocument: document, mCells: cells, mItemIndex: Default::default(), mIndexOfItem: Default::default(), mFindNextCell: Default::default(), mNextItemIndex: Default::default(), mNextIndexOfItem: Default::default() }
      // java this.mCells = cells;
    }

    // java private boolean findNextCell()
    pub fn findNextCell(&mut self) -> bool {
      // java if (this.mFindNextCell)
      if self.mFindNextCell {
        // java return true;
        return true;
      }
      // java else
      else {
        // java if (-1 == this.mItemIndex)
        if -1 == self.mItemIndex {
          // java this.mNextItemIndex = 0;
          self.mNextItemIndex = 0;
          let mut size: i32 = (self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
          // java for(int size = this.mCells.size(); this.mNextItemIndex < size ++this.mNextItemIndex)
          while self.mNextItemIndex < size {
            // java CellsItem cellItem = (CellsItem)this.mCells.get(this.mNextItemIndex);
            let mut cellItem: CellsItem = (self.mCells.get(self.mNextItemIndex)).downcast_ref::<CellsItem>().unwrap();
            // java if (cellItem.startCellIndex != -1)
            if cellItem.startCellIndex != -1 {
              // java this.mNextIndexOfItem = cellItem.startCellIndex;
              self.mNextIndexOfItem = cellItem.startCellIndex;
              // java this.mFindNextCell = true;
              self.mFindNextCell = true;
              // java break;
              break;
            }
          }
        }
        // java else
        else {
          // java CellsItem cellItem = (CellsItem)this.mCells.get(this.mItemIndex);
          let mut cellItem: CellsItem = (self.mCells.get(self.mItemIndex)).downcast_ref::<CellsItem>().unwrap();
          // java if (this.mIndexOfItem < cellItem.endCellIndex)
          if self.mIndexOfItem < cellItem.endCellIndex {
            // java this.mNextIndexOfItem = this.mIndexOfItem + 1;
            self.mNextIndexOfItem = self.mIndexOfItem + 1;
            // java this.mFindNextCell = true;
            self.mFindNextCell = true;
          }
          // java else
          else {
            // java this.mNextItemIndex = this.mItemIndex + 1;
            self.mNextItemIndex = self.mItemIndex + 1;
            let mut size: i32 = (self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
            // java for(int size = this.mCells.size(); this.mNextItemIndex < size ++this.mNextItemIndex)
            while self.mNextItemIndex < size {
              // java cellItem = (CellsItem)this.mCells.get(this.mNextItemIndex);
              cellItem = (self.mCells.get(self.mNextItemIndex)).downcast_ref::<CellsItem>().unwrap();
              // java if (cellItem.startCellIndex != -1)
              if cellItem.startCellIndex != -1 {
                // java this.mNextIndexOfItem = cellItem.startCellIndex;
                self.mNextIndexOfItem = cellItem.startCellIndex;
                // java this.mFindNextCell = true;
                self.mFindNextCell = true;
                // java break;
                break;
              }
            }
          }
        }
        // java return this.mFindNextCell;
        return self.mFindNextCell;
      }
    }

}
