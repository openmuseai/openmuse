// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.command.CTableRowModifyCmd
use crate::cn::wps::moffice::writer::core::command::CTableRowModifyCmd::*;
// Java: import cn.wps.moffice.writer.data.RANGE
use crate::cn::wps::moffice::writer::data::RANGE::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableJCData
use crate::cn::wps::moffice::writer::data::table::TableJCData::*;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::*;
// Java: import cn.wps.moffice.writer.data.value.DefTable
use crate::cn::wps::moffice::writer::data::value::DefTable::*;
// Java: import cn.wps.moffice.writer.data.value.DefTableBuilder
use crate::cn::wps::moffice::writer::data::value::DefTableBuilder::*;
// Java: import cn.wps.moffice.writer.data.value.TableCellBuilder
use crate::cn::wps::moffice::writer::data::value::TableCellBuilder::*;
// Java: import cn.wps.moffice.writer.data.value.TableDefSHD
use crate::cn::wps::moffice::writer::data::value::TableDefSHD::*;
// Java: import cn.wps.moffice.writer.data.value.TableDefSHDBuilder
use crate::cn::wps::moffice::writer::data::value::TableDefSHDBuilder::*;
// Java: import cn.wps.moffice.writer.data.value.TableLength
use crate::cn::wps::moffice::writer::data::value::TableLength::*;
// Java: import cn.wps.moffice.writer.data.value.TableSetBRC
use crate::cn::wps::moffice::writer::data::value::TableSetBRC::*;
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
use crate::cn::wps::util::concurrent::locks::RangeReadLock::*;
// Java: import java.util.ArrayDeque
use stubs::java::util::ArrayDeque::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.table.MergeSpilit
use crate::cn::wps::moffice::writer::core::table::MergeSpilit::*;
// Java: import cn.wps.moffice.writer.core.table.CellsItem
use crate::cn::wps::moffice::writer::core::table::CellsItem::*;
// Java: import cn.wps.moffice.writer.core.table.KCellIterator
use crate::cn::wps::moffice::writer::core::table::KCellIterator::*;
// Java: import cn.wps.moffice.writer.core.table.KCellIteratorForCol
use crate::cn::wps::moffice::writer::core::table::KCellIteratorForCol::*;
// Java: import cn.wps.moffice.writer.core.table.KColIterator
use crate::cn::wps::moffice::writer::core::table::KColIterator::*;
// Java: import cn.wps.moffice.writer.core.table.KColIteratorImpl
use crate::cn::wps::moffice::writer::core::table::KColIteratorImpl::*;
// Java: import cn.wps.moffice.writer.core.table.KTableRangeBase
use crate::cn::wps::moffice::writer::core::table::KTableRangeBase::*;
// Java: import cn.wps.moffice.writer.core.table.BrcCvFactory
use crate::cn::wps::moffice::writer::core::table::BrcCvFactory::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.core.KRange.DeleteRangeReason
use crate::cn::wps::moffice::writer::core::KRange::DeleteRangeReason;

// java public static final int MAX_COLS = 63;
pub(crate) const MAX_COLS: i32 = 63;

// java public class KColRange extends MergeSpilit
#[derive(Default)]
pub struct KColRange {
    // java MergeSpilit super_
    pub(crate) super_: MergeSpilit,
    // java public int mColLeft;
    pub(crate) mColLeft: i32,
    // java public int mColRight;
    pub(crate) mColRight: i32,
    // java private int mInsertCellWidth;
    mInsertCellWidth: i32,
    // java private Float mCompressPercent;
    mCompressPercent: Option<f32>,
    // java private int mTableLeftRgdxa;
    mTableLeftRgdxa: i32,
    // java private int mTableWidthLen;
    mTableWidthLen: i32,
    // java private TableLength mTableWidthProp;
    mTableWidthProp: Option<TableLength>,
    // java private int mStartCP;
    mStartCP: i32,
}

// java public class KColRange extends MergeSpilit
impl KColRange {
    // java public static KColRange createColRange(Document document, int startCP, int endCP)
    pub fn createColRange__document_int_int(document: &dyn Document, startCP: i32, endCP: i32) -> KColRange {
      // java KColRange colRange = new KColRange(document, startCP, endCP, true);
      let mut colRange: KColRange = KColRange::new(document, startCP, endCP, true);
      // java if (0 == colRange.mCells.size())
      if 0 == (colRange.mCells.len() as i32) {
        // java colRange = null;
        colRange = None;
      }
      // java return colRange;
      return colRange;
    }

    // java public static KColRange createColRange(Document document, int startCP, int endCP, int colLeft, int colRight)
    pub fn createColRange__document_int_int_int_int(document: &dyn Document, startCP: i32, endCP: i32, colLeft: i32, colRight: i32) -> KColRange {
      // java KColRange colRange = new KColRange(document, startCP, endCP, colLeft, colRight);
      let mut colRange: KColRange = KColRange::new(document, startCP, endCP, colLeft, colRight);
      // java if (0 == colRange.mCells.size())
      if 0 == (colRange.mCells.len() as i32) {
        // java colRange = null;
        colRange = None;
      }
      // java return colRange;
      return colRange;
    }

    // java public static KColRange createRectRange(Document document, int startCP, int endCP)
    pub fn createRectRange(document: &dyn Document, startCP: i32, endCP: i32) -> KColRange {
      // java KColRange colRange = new KColRange(document, startCP, endCP, false);
      let mut colRange: KColRange = KColRange::new(document, startCP, endCP, false);
      // java if (0 == colRange.mCells.size())
      if 0 == (colRange.mCells.len() as i32) {
        // java colRange = null;
        colRange = None;
      }
      // java return colRange;
      return colRange;
    }

    // java   KColRange(Document document, ArrayList<CellsItem> cells)
    pub fn new__document_arraylistcellsitem(document: &dyn Document, cells: &Vec<CellsItem>) -> Self {
      // java this.document = document;
      Self { super_: MergeSpilit::new(), mColLeft: Default::default(), mColRight: Default::default(), mInsertCellWidth: Default::default(), mCompressPercent: Default::default(), mTableLeftRgdxa: Default::default(), mTableWidthLen: Default::default(), mTableWidthProp: Default::default(), mStartCP: Default::default() }
      // java this.mCells = cells;
    }

    // java   KColRange(Document document, int startCP, int endCP, boolean allRows)
    pub fn new__document_int_int_boolean(document: &dyn Document, startCP: i32, endCP: i32, allRows: bool) -> Self {
      // java this.document = document;
      let mut this = Self { super_: MergeSpilit::new(), mColLeft: Default::default(), mColRight: Default::default(), mInsertCellWidth: Default::default(), mCompressPercent: Default::default(), mTableLeftRgdxa: Default::default(), mTableWidthLen: Default::default(), mTableWidthProp: Default::default(), mStartCP: startCP };
      // java this.mStartCP = startCP;
      // java this.collectColumns(startCP, endCP, allRows);
      this.collectColumns__int_int_boolean(startCP, endCP, allRows);
      this
    }

    // java   KColRange(Document document, int startCP, int endCP, int colLeft, int colRight)
    pub fn new__document_int_int_int_int(document: &dyn Document, startCP: i32, endCP: i32, colLeft: i32, colRight: i32) -> Self {
      // java this.document = document;
      let mut this = Self { super_: MergeSpilit::new(), mColLeft: colLeft, mColRight: colRight, mInsertCellWidth: Default::default(), mCompressPercent: Default::default(), mTableLeftRgdxa: Default::default(), mTableWidthLen: Default::default(), mTableWidthProp: Default::default(), mStartCP: startCP };
      // java RangeReadLock readLock = document.getMainDocument().readLock();
      let mut readLock: RangeReadLock = document.getMainDocument().readLock();
      // java try { TableInfo tableInfo = document.getTableInfos().getTableInfoByRange(startCP, endCP); if (tableInfo != null) { this.collectColumns(tableInfo, 0, tableInfo.size() - 1, colLeft, colRight, colRight - colLeft); } } finally { readLock.unlock();
      this.collectColumns__tableinfo_int_int_int_int_int(&tableInfo, 0, (tableInfo.len() as i32) - 1, colLeft, colRight, colRight - colLeft);
      this
    }

    // java public long getLastRowSelectRange()
    pub fn getLastRowSelectRange(&self) -> i64 {
      // java TableInfo tableInfo = this.getTableInfo();
      let mut tableInfo: TableInfo = self.super_.getTableInfo();
      // java CellsItem cellsItem = (CellsItem)this.mCells.get(this.mCells.size() - 1);
      let mut cellsItem: CellsItem = (self.mCells.get((self.mCells.len() as i32) - 1)).downcast_ref::<CellsItem>().unwrap();
      // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getStart(cellsItem.rowRange));
      let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart(cellsItem.rowRange));
      // java int start = rowInfo.getCellByIndex(cellsItem.startCellIndex).getStartCP();
      let mut start: i32 = rowInfo.getCellByIndex(cellsItem.startCellIndex).getStartCP();
      // java int end = rowInfo.getCellByIndex(cellsItem.endCellIndex).getEndCP();
      let mut end: i32 = rowInfo.getCellByIndex(cellsItem.endCellIndex).getEndCP();
      // java return RANGEFactory.getRANGE(start, end);
      return RANGEFactory::getRANGE(start, end);
    }

    // java public int insertColumn(boolean after, int colCount)
    pub fn insertColumn(&mut self, after: bool, colCount: i32) -> i32 {
      // java if (0 != this.mCells.size() && 0 < colCount)
      if 0 != (self.mCells.len() as i32) && 0 < colCount {
        // java int var9;
        let mut var9: i32;
            // java this.getTextDocument().start();
            self.super_.getTextDocument().start();
            // java colCount = this.calcMaxInsertColumns(colCount);
            colCount = self.super_.calcMaxInsertColumns(colCount);
            // java if (colCount > 0)
            if colCount > 0 {
              // java this.calcInsertCellWidth(after, colCount);
              self.calcInsertCellWidth(after, colCount);
              // java TableInfo tableInfo = this.getTableInfo();
              let mut tableInfo: TableInfo = self.super_.getTableInfo();
              let mut i: i32 = (self.mCells.len() as i32) - 1;
              // java for(int i = this.mCells.size() - 1; i >= 0 --i)
              while i >= 0 {
                // java this.insertCells(tableInfo, (CellsItem)this.mCells.get(i), after, colCount);
                self.insertCells(&tableInfo, &(self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap(), after, colCount);
              }
            }
            // java var9 = colCount;
            var9 = colCount;
              // java this.getSubDocument().getTableInfos().doGridAutoResize();
              self.super_.getSubDocument().getTableInfos().doGridAutoResize();
              // java this.getTextDocument().commit("insert column");
              self.super_.getTextDocument().commit("insert column");
        // java return var9;
        return var9;
      }
      // java else
      else {
        // java return 0;
        return 0;
      }
    }

    // java public KCellIterator getCellIterator()
    pub fn getCellIterator(&self) -> Box<dyn KCellIterator> {
      // java return new KCellIteratorForCol(this.document, this.mCells);
      return KCellIteratorForCol::new(self.document, self.mCells);
    }

    // java public KColIterator getColIterator()
    pub fn getColIterator(&self) -> Box<dyn KColIterator> {
      // java return new KColIteratorImpl(this.document, (CellsItem)this.mCells.get(0));
      return KColIteratorImpl::new(self.document, (self.mCells.get(0)).downcast_ref::<CellsItem>().unwrap());
    }

    // java private void collectColumns(int startCP, int endCP, boolean allRows)
    pub fn collectColumns__int_int_boolean(&mut self, startCP: i32, endCP: i32, allRows: bool) {
      // java if (startCP <= endCP)
      if startCP <= endCP {
        // java this.mCells.clear();
        self.mCells.clear();
        // java RangeReadLock readLock = this.getMainDocument().readLockAll();
        let mut readLock: RangeReadLock = self.super_.getMainDocument().readLockAll();
            // java TableInfo tableInfo = this.document.getTableInfos().getTableInfoByRange(startCP, endCP);
            let mut tableInfo: TableInfo = self.document.getTableInfos().getTableInfoByRange(startCP, endCP);
            // java if (tableInfo != null)
            if tableInfo.is_some() {
              // java RANGE range = RANGE.obtain(startCP, endCP);
              let mut range: RANGE = RANGE::obtain__int_int(startCP, endCP);
              // java normalize(tableInfo, range);
              Self::normalize(&tableInfo, &mut range);
              // java RowInfo firstRowInfo = tableInfo.getRowByCP(range.start);
              let mut firstRowInfo: RowInfo = tableInfo.getRowByCP(range.start);
              // java CellInfo startCell = firstRowInfo.getCellByCP(range.start);
              let mut startCell: CellInfo = firstRowInfo.getCellByCP(range.start);
              // java RowInfo lastRowInfo = tableInfo.getRowByCP(range.end);
              let mut lastRowInfo: RowInfo = tableInfo.getRowByCP(range.end);
              // java CellInfo endCell = lastRowInfo.getCellByCP(range.end - 1);
              let mut endCell: CellInfo = lastRowInfo.getCellByCP(range.end - 1);
              // java range.recycle();
              range.recycle();
              // java int rgdxaLeft = Math.min(startCell.getRgdxaLeft(), endCell.getRgdxaLeft());
              let mut rgdxaLeft: i32 = Math::min(startCell.getRgdxaLeft(), endCell.getRgdxaLeft());
              // java int rgdxaRight = Math.max(startCell.getRgdxaRight(), endCell.getRgdxaRight());
              let mut rgdxaRight: i32 = Math::max(startCell.getRgdxaRight(), endCell.getRgdxaRight());
              // java int startCellWidth = startCell.getRgdxaRight() - startCell.getRgdxaLeft();
              let mut startCellWidth: i32 = startCell.getRgdxaRight() - startCell.getRgdxaLeft();
              // java int endCellWidth = endCell.getRgdxaRight() - endCell.getRgdxaLeft();
              let mut endCellWidth: i32 = endCell.getRgdxaRight() - endCell.getRgdxaLeft();
              // java int minWidth = Math.min(startCellWidth, endCellWidth);
              let mut minWidth: i32 = Math::min(startCellWidth, endCellWidth);
              // java if (allRows)
              if allRows {
                // java this.collectColumns(tableInfo, 0, tableInfo.size() - 1, rgdxaLeft, rgdxaRight, minWidth);
                self.collectColumns__tableinfo_int_int_int_int_int(&tableInfo, 0, (tableInfo.len() as i32) - 1, rgdxaLeft, rgdxaRight, minWidth);
              }
              // java else
              else {
                // java this.collectColumns(tableInfo, firstRowInfo.getIndex(), lastRowInfo.getIndex(), rgdxaLeft, rgdxaRight, minWidth);
                self.collectColumns__tableinfo_int_int_int_int_int(&tableInfo, firstRowInfo.getIndex(), lastRowInfo.getIndex(), rgdxaLeft, rgdxaRight, minWidth);
              }
              // java return;
              return;
            }
              // java readLock.unlock();
              readLock.unlock();
      }
    }

    // java public static void normalize(TableInfo tableInfo, RANGE range)
    pub fn normalize(tableInfo: &TableInfo, range: &mut RANGE) {
      // java RowInfo firstRowInfo = tableInfo.getRowByCP(range.start);
      let mut firstRowInfo: RowInfo = tableInfo.getRowByCP(range.start);
      // java CellInfo startCell = firstRowInfo.getCellByCP(range.start);
      let mut startCell: CellInfo = firstRowInfo.getCellByCP(range.start);
      // java if (startCell == null)
      if startCell.is_none() {
        // java startCell = firstRowInfo.getCellByIndex(0);
        startCell = firstRowInfo.getCellByIndex(0);
      }
      // java if (startCell.isVertMergeCell())
      if startCell.isVertMergeCell() {
        // java while (!startCell.isFirstVertMergeCell())
        while !startCell.isFirstVertMergeCell() {
          // java startCell = startCell.getPrevVertMergeCell();
          startCell = startCell.getPrevVertMergeCell();
        }
        // java firstRowInfo = startCell.getParent();
        firstRowInfo = startCell.getParent();
      }
      // java int endCellCP = Math.max(range.start, range.end - 1);
      let mut endCellCP: i32 = Math::max(range.start, range.end - 1);
      // java RowInfo lastRowInfo;
      let mut lastRowInfo: RowInfo = RowInfo::default();
      // java if (tableInfo.getEndCP() >= range.end)
      if tableInfo.getEndCP() >= range.end {
        // java lastRowInfo = tableInfo.getRowByCP(endCellCP);
        lastRowInfo = tableInfo.getRowByCP(endCellCP);
      }
      // java else
      else {
        // java lastRowInfo = tableInfo.getRowByIndex(tableInfo.size() - 1);
        lastRowInfo = tableInfo.getRowByIndex(tableInfo.size() - 1);
      }
      // java CellInfo endCell = lastRowInfo.getCellByCP(endCellCP);
      let mut endCell: CellInfo = lastRowInfo.getCellByCP(endCellCP);
      // java if (endCell == null)
      if endCell.is_none() {
        // java endCell = lastRowInfo.getCellByIndex(lastRowInfo.size() - 1);
        endCell = lastRowInfo.getCellByIndex((lastRowInfo.len() as i32) - 1);
      }
      // java if (endCell.isVertMergeCell())
      if endCell.isVertMergeCell() {
        // java while (!endCell.isLastVertMergeCell())
        while !endCell.isLastVertMergeCell() {
          // java endCell = endCell.getNextVertMergeCell();
          endCell = endCell.getNextVertMergeCell();
        }
        // java lastRowInfo = endCell.getParent();
        lastRowInfo = endCell.getParent();
      }
      // java range.start = startCell.getStartCP();
      range.start = startCell.getStartCP();
      // java range.end = endCell.getEndCP();
      range.end = endCell.getEndCP();
    }

    // java private void collectColumns(TableInfo tableInfo, int firstRow, int lastRow, int rgdxaLeft, int rgdxaRight, int minWidth)
    pub fn collectColumns__tableinfo_int_int_int_int_int(&mut self, tableInfo: &TableInfo, firstRow: i32, lastRow: i32, rgdxaLeft: i32, rgdxaRight: i32, minWidth: i32) {
      // java this.mColLeft = rgdxaLeft;
      self.mColLeft = rgdxaLeft;
      // java this.mColRight = rgdxaRight;
      self.mColRight = rgdxaRight;
      // java int residual = TableTool.getResidual(minWidth);
      let mut residual: i32 = TableTool::getResidual(minWidth);
      // java rgdxaLeft += residual;
      rgdxaLeft = rgdxaLeft + residual;
      // java rgdxaRight -= residual;
      rgdxaRight = rgdxaRight - residual;
      // java ArrayDeque<CellsItem> cells = new ArrayDeque();
      let mut cells: ArrayDeque = ArrayDeque::new();
      let mut i: i32 = firstRow;
      // java for(int i = firstRow; i <= lastRow; ++i)
      while i <= lastRow {
        // java RowInfo rowInfo = tableInfo.getRowByIndex(i);
        let mut rowInfo: RowInfo = tableInfo.getRowByIndex(i);
        // java CellsItem cellItem = new CellsItem(this.document, rowInfo.getRange());
        let mut cellItem: CellsItem = CellsItem::new(self.document, rowInfo.getRange());
        // java boolean findStart = false;
        let mut findStart: bool = false;
        // java int j = 0;
        let mut j: i32 = 0;
        let mut sizeOfCell: i32 = (rowInfo.len() as i32);
        // java for(int sizeOfCell = rowInfo.size(); j < sizeOfCell ++j)
        while j < sizeOfCell {
          // java CellInfo cellInfo = rowInfo.getCellByIndex(j);
          let mut cellInfo: CellInfo = rowInfo.getCellByIndex(j);
          // java if (cellInfo.getRgdxaRight() > rgdxaLeft && cellInfo.getRgdxaLeft() < rgdxaRight)
          if cellInfo.getRgdxaRight() > rgdxaLeft && cellInfo.getRgdxaLeft() < rgdxaRight {
            // java if (findStart)
            if findStart {
              // java cellItem.endCellIndex = j;
              cellItem.endCellIndex = j;
              // java cellItem.cellsRange = RANGEFactory.getRANGE(RANGEFactory.getStart(cellItem.cellsRange), cellInfo.getEndCP());
              cellItem.cellsRange = RANGEFactory::getRANGE(RANGEFactory::getStart(cellItem.cellsRange), cellInfo.getEndCP());
            }
            // java else
            else {
              // java cellItem.startCellIndex = j;
              cellItem.startCellIndex = j;
              // java cellItem.endCellIndex = j;
              cellItem.endCellIndex = j;
              // java cellItem.cellsRange = RANGEFactory.getRANGE(cellInfo.getStartCP(), cellInfo.getEndCP());
              cellItem.cellsRange = RANGEFactory::getRANGE(cellInfo.getStartCP(), cellInfo.getEndCP());
              // java findStart = true;
              findStart = true;
            }
          }
        }
        // java cells.add(cellItem);
        cells.add(cellItem);
      }
      let mut i: i32 = (cells.len() as i32) - 1;
      // java for(int i = cells.size() - 1; i >= 0 && ((CellsItem)cells.getLast()).startCellIndex == -1 --i)
      while i >= 0 && (cells.getLast()).downcast_ref::<CellsItem>().unwrap().startCellIndex == -1 {
        // java cells.pollLast();
        cells.pollLast();
      }
      let mut i: i32 = (cells.len() as i32) - 1;
      // java for(int i = cells.size() - 1; i >= 0 && ((CellsItem)cells.getFirst()).startCellIndex == -1 --i)
      while i >= 0 && (cells.getFirst()).downcast_ref::<CellsItem>().unwrap().startCellIndex == -1 {
        // java cells.pollFirst();
        cells.pollFirst();
      }
      // java this.mCells.addAll(cells);
      self.mCells.addAll(cells);
    }

    // java private void calcInsertCellWidth(boolean after, int colCount)
    pub fn calcInsertCellWidth(&mut self, after: bool, colCount: i32) {
      // java this.mInsertCellWidth = 0;
      self.mInsertCellWidth = 0;
      // java this.mCompressPercent = null;
      self.mCompressPercent = None;
      // java this.mTableLeftRgdxa = 0;
      self.mTableLeftRgdxa = 0;
      // java CellsItem cellsItem = null;
      let mut cellsItem: CellsItem = CellsItem::default();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (self.mCells.len() as i32);
      // java for(int size = this.mCells.size(); i < size; ++i)
      while i < size {
        // java cellsItem = (CellsItem)this.mCells.get(i);
        cellsItem = (self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap();
        // java if (RANGEFactory.contains(cellsItem.rowRange, this.mStartCP))
        if RANGEFactory::contains(cellsItem.rowRange, self.mStartCP) {
          // java break;
          break;
        }
      }
      // java DefTable defTable = (DefTable)cellsItem.getFinalRowProperty().getValue(306);
      let mut defTable: DefTable = (cellsItem.getFinalRowProperty().getValue(306)).downcast_ref::<DefTable>().unwrap();
      // java if (after)
      if after {
        // java if (cellsItem.endCellIndex != defTable.getCellCount() - 1)
        if cellsItem.endCellIndex != defTable.getCellCount() - 1 {
          // java int cellIndex = cellsItem.endCellIndex + 1;
          let mut cellIndex: i32 = cellsItem.endCellIndex + 1;
          // java this.mInsertCellWidth = defTable.getRgdxa(cellIndex + 1) - defTable.getRgdxa(cellIndex);
          self.mInsertCellWidth = defTable.getRgdxa(cellIndex + 1) - defTable.getRgdxa(cellIndex);
        }
        // java else
        else {
          // java this.mInsertCellWidth = defTable.getRgdxa(cellsItem.endCellIndex + 1) - defTable.getRgdxa(cellsItem.endCellIndex);
          self.mInsertCellWidth = defTable.getRgdxa(cellsItem.endCellIndex + 1) - defTable.getRgdxa(cellsItem.endCellIndex);
        }
      }
      // java else
      else {
        // java this.mInsertCellWidth = defTable.getRgdxa(cellsItem.startCellIndex + 1) - defTable.getRgdxa(cellsItem.startCellIndex);
        self.mInsertCellWidth = defTable.getRgdxa(cellsItem.startCellIndex + 1) - defTable.getRgdxa(cellsItem.startCellIndex);
      }
      // java int maxRgdxa = 0;
      let mut maxRgdxa: i32 = 0;
      // java TableInfo tableInfo = this.document.getTableInfos().getTableInfoByRange(RANGEFactory.getStart(cellsItem.rowRange), RANGEFactory.getEnd(cellsItem.rowRange));
      let mut tableInfo: TableInfo = self.document.getTableInfos().getTableInfoByRange(RANGEFactory::getStart(cellsItem.rowRange), RANGEFactory::getEnd(cellsItem.rowRange));
      // java if (tableInfo.getLevel() > 1)
      if tableInfo.getLevel() > 1 {
        // java CellInfo parentCell = tableInfo.getParentCell();
        let mut parentCell: CellInfo = tableInfo.getParentCell();
        // java maxRgdxa = parentCell.getRgdxaRight() - parentCell.getRgdxaLeft() - 5 - parentCell.getMarginLeft() - parentCell.getMarginRight();
        maxRgdxa = parentCell.getRgdxaRight() - parentCell.getRgdxaLeft() - 5 - parentCell.getMarginLeft() - parentCell.getMarginRight();
        // java maxRgdxa = Math.max(maxRgdxa, tableInfo.getJC().jcRight);
        maxRgdxa = Math::max(maxRgdxa, tableInfo.getJC().jcRight);
      }
      // java else
      else {
        // java int coreWidth = Tools.getMainDocumentCoreWidth(this.document, RANGEFactory.getStart(((CellsItem)this.mCells.get(0)).rowRange));
        let mut coreWidth: i32 = Tools::getMainDocumentCoreWidth(self.document, RANGEFactory::getStart((self.mCells.get(0)).downcast_ref::<CellsItem>().unwrap().rowRange));
        // java maxRgdxa = coreWidth + 108;
        maxRgdxa = coreWidth + 108;
        // java maxRgdxa = Math.max(maxRgdxa, tableInfo.getJC().jcRight);
        maxRgdxa = Math::max(maxRgdxa, tableInfo.getJC().jcRight);
      }
      // java boolean limitTableWidth = true;
      let mut limitTableWidth: bool = true;
      // java TableJCData jcData = tableInfo.getJC();
      let mut jcData: TableJCData = tableInfo.getJC();
      // java this.mTableLeftRgdxa = jcData.jcLeft;
      self.mTableLeftRgdxa = jcData.jcLeft;
      // java int newRight = jcData.jcRight + this.mInsertCellWidth * colCount;
      let mut newRight: i32 = jcData.jcRight + self.mInsertCellWidth * colCount;
      // java if (limitTableWidth && newRight > maxRgdxa)
      if limitTableWidth && newRight > maxRgdxa {
        // java this.mCompressPercent = (float)(maxRgdxa - this.mTableLeftRgdxa) * 1.0F / (float)(newRight - this.mTableLeftRgdxa);
        self.mCompressPercent = ((maxRgdxa - self.mTableLeftRgdxa) as f32) * 1 / ((newRight - self.mTableLeftRgdxa) as f32);
        // java newRight = maxRgdxa;
        newRight = maxRgdxa;
      }
      // java this.mTableWidthLen = newRight - this.mTableLeftRgdxa;
      self.mTableWidthLen = newRight - self.mTableLeftRgdxa;
      // java PropertySet finalProp = tableInfo.getTableProperty();
      let mut finalProp: PropertySet = tableInfo.getTableProperty();
      // java if (finalProp.contains(295))
      if finalProp.contains(295) {
        // java TableLength tableWidth = (TableLength)finalProp.getValue(295);
        let mut tableWidth: TableLength = (finalProp.getValue(295)).downcast_ref::<TableLength>().unwrap();
        // java switch (tableWidth.getType())
        match tableWidth.getType() {
          // java case 1:
          1 => {
            // java this.mTableWidthProp = tableWidth;
            self.mTableWidthProp = tableWidth;
          }
          // java case 2:
          2 => {
            // java this.mTableWidthProp = new TableLength(2, (int)((float)(this.mTableWidthLen / (maxRgdxa - this.mTableLeftRgdxa)) * 5000.0F));
            self.mTableWidthProp = TableLength::new(2, ((((self.mTableWidthLen / maxRgdxa - self.mTableLeftRgdxa) as f32) * 5000) as i32));
          }
          // java default:
          _ => {
            // java this.mTableWidthProp = new TableLength(tableWidth.getType(), this.mTableWidthLen);
            self.mTableWidthProp = TableLength::new(tableWidth.getType(), self.mTableWidthLen);
          }
        }
      }
    }

    // java private void insertCells(TableInfo tableInfo, CellsItem cellsItem, boolean after, int colCount)
    pub fn insertCells(&mut self, tableInfo: &TableInfo, cellsItem: &CellsItem, after: bool, colCount: i32) {
      // java if (-1 != cellsItem.startCellIndex || null != this.mCompressPercent)
      if -1 != cellsItem.startCellIndex || self.mCompressPercent.is_some() {
        // java PropertySet finalProperty = cellsItem.getFinalRowProperty();
        let mut finalProperty: PropertySet = cellsItem.getFinalRowProperty();
        // java DefTable defTable = (DefTable)finalProperty.getValue(306);
        let mut defTable: DefTable = (finalProperty.getValue(306)).downcast_ref::<DefTable>().unwrap();
        // java TableDefSHD defShds = (TableDefSHD)finalProperty.getValue(308);
        let mut defShds: TableDefSHD = (finalProperty.getValue(308)).downcast_ref::<TableDefSHD>().unwrap();
        // java TableSetBRC[] setBRCs = (TableSetBRC[])finalProperty.getValue(311);
        let mut setBRCs: Vec<TableSetBRC> = todo!("cast → Vec<TableSetBRC>");
        // java this.setDefTable4InsertCells(defTable, cellsItem, after, colCount);
        self.setDefTable4InsertCells(&defTable, &cellsItem, after, colCount);
        // java this.setDefSHD4InsertCells(defShds, cellsItem, after, colCount);
        self.setDefSHD4InsertCells(Some(&defShds), &cellsItem, after, colCount);
        // java this.setTableSetBrcs(setBRCs, cellsItem, after, colCount, defTable.getCellCount());
        self.setTableSetBrcs(Some(&setBRCs), &cellsItem, after, colCount, defTable.getCellCount());
        // java this.setBrcCv(finalProperty, cellsItem, after, colCount, defTable.getCellCount());
        self.setBrcCv(&finalProperty, &cellsItem, after, colCount, defTable.getCellCount());
        // java Tools.submitCMD(this.document, cellsItem.rowRange, 295, finalProperty.getValue(295), this.mTableWidthProp);
        Tools::submitCMD(self.document, cellsItem.rowRange, 295, finalProperty.getValue(295), self.mTableWidthProp);
        // java if (-1 != cellsItem.startCellIndex)
        if -1 != cellsItem.startCellIndex {
          // java int insertPos = 0;
          let mut insertPos: i32 = 0;
          // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getEnd(cellsItem.rowRange) - 1);
          let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getEnd(cellsItem.rowRange) - 1);
          // java int templateCell;
          let mut templateCell: i32;
          // java if (after)
          if after {
            // java insertPos = RANGEFactory.getEnd(cellsItem.cellsRange);
            insertPos = RANGEFactory::getEnd(cellsItem.cellsRange);
            // java if (cellsItem.endCellIndex == rowInfo.size() - 1)
            if cellsItem.endCellIndex == (rowInfo.len() as i32) - 1 {
              // java templateCell = cellsItem.endCellIndex;
              templateCell = cellsItem.endCellIndex;
            }
            // java else
            else {
              // java templateCell = cellsItem.endCellIndex + 1;
              templateCell = cellsItem.endCellIndex + 1;
            }
          }
          // java else
          else {
            // java insertPos = RANGEFactory.getStart(cellsItem.cellsRange);
            insertPos = RANGEFactory::getStart(cellsItem.cellsRange);
            // java templateCell = cellsItem.startCellIndex;
            templateCell = cellsItem.startCellIndex;
          }
          // java int level = tableInfo.getLevel();
          let mut level: i32 = tableInfo.getLevel();
          // java KTableRangeBase.RevisionType type = getRevisionType(rowInfo);
          let mut r#type: KTableRangeBase::RevisionType = KTableRangeBase::getRevisionType(&rowInfo);
          // java int cellStart = rowInfo.getCellByIndex(templateCell).getStartCP();
          let mut cellStart: i32 = rowInfo.getCellByIndex(templateCell).getStartCP();
          // java PropertySet originProp = this.document.getPapxTable().seek(cellStart).getProperty();
          let mut originProp: PropertySet = self.document.getPapxTable().seek(cellStart).getProperty();
          // java PropertySet papx = Tools.getDefaultCellPropertySet(this.document, level, originProp);
          let mut papx: PropertySet = Tools::getDefaultCellPropertySet(self.document, level, originProp);
          // java PropertySet chpx = this.getFinalChpx(this.document.getChpxTable().seek(cellStart).getProperty(), type);
          let mut chpx: PropertySet = self.super_.getFinalChpx(&mut self.document.getChpxTable().seek(cellStart).getProperty(), &r#type);
          // java this.document.getTextStream().disableRevisionFileter();
          self.document.getTextStream().disableRevisionFileter();
          let mut i: i32 = 0;
          // java for(int i = 0; i < colCount; ++i)
          while i < colCount {
            // java papx = Tools.createParaId(papx, this.document);
            papx = Tools::createParaId(papx, self.document);
            // java int cp = insertPos;
            let mut cp: i32 = insertPos;
            // java Tools.createCell(this.document, insertPos++, chpx, papx);
            Tools::createCell(self.document, { let _tmp = insertPos; insertPos += 1; _tmp }, chpx, papx);
            // java Tools.addParaId2Map(this.document, cp, papx);
            Tools::addParaId2Map(self.document, cp, papx);
          }
          // java this.document.getTextStream().enableRevisionFileter();
          self.document.getTextStream().enableRevisionFileter();
        }
      }
    }

    // java private void setDefTable4InsertCells(DefTable oldDefTable, CellsItem cellsItem, boolean after, int colCount)
    pub fn setDefTable4InsertCells(&mut self, oldDefTable: &DefTable, cellsItem: &CellsItem, after: bool, colCount: i32) {
      // java if (Tools.checkTableFormatChange(this.document, cellsItem))
      if Tools::checkTableFormatChange(self.document, cellsItem) {
        // java DefTable old = (DefTable)cellsItem.getRowProperty().getValue(306);
        let mut old: DefTable = (cellsItem.getRowProperty().getValue(306)).downcast_ref::<DefTable>().unwrap();
        // java DefTable defTable = this.insertColumns(old, cellsItem, after, colCount);
        let mut defTable: DefTable = self.insertColumns(&old, &cellsItem, after, colCount);
        // java CTableRowModifyCmd cmd = new CTableRowModifyCmd(this.document, cellsItem.rowRange, 306, old, defTable);
        let mut cmd: CTableRowModifyCmd = CTableRowModifyCmd::new(self.document, cellsItem.rowRange, 306, old, defTable);
        // java cmd.setModifyRevision(false);
        cmd.setModifyRevision(false);
        // java cmd.execute();
        cmd.execute();
        // java this.getTextDocument().submit(cmd);
        self.super_.getTextDocument().submit(cmd);
      }
      // java DefTableBuilder newDefTable = this.insertColumns(oldDefTable, cellsItem, after, colCount);
      let mut newDefTable: DefTableBuilder = self.insertColumns(&oldDefTable, &cellsItem, after, colCount);
      // java if (null != this.mCompressPercent)
      if let Some(mCompressPercent) = self.mCompressPercent.as_mut() {
        // java int i = 0;
        let mut i: i32 = 0;
        let mut size: i32 = newDefTable.getItMax();
        // java for(int size = newDefTable.getItMax(); i < size; ++i)
        while i < size {
          // java float rgdxa = (float)this.mTableLeftRgdxa + (float)(newDefTable.getRgdxa(i) - this.mTableLeftRgdxa) * this.mCompressPercent;
          let mut rgdxa: f32 = ((self.mTableLeftRgdxa) as f32) + ((newDefTable.getRgdxa(i) - self.mTableLeftRgdxa) as f32) * mCompressPercent;
          // java newDefTable.setRgdxa(i, (int)rgdxa);
          newDefTable.setRgdxa(i, ((rgdxa) as i32));
        }
      }
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = newDefTable.getCellCount();
      // java for(int size = newDefTable.getCellCount(); i < size; ++i)
      while i < size {
        // java TableCellBuilder newTableCell = newDefTable.getTableCellBuilder(i);
        let mut newTableCell: TableCellBuilder = newDefTable.getTableCellBuilder(i);
        // java if (newTableCell.getWidthType() == 2)
        if newTableCell.getWidthType() == 2 {
          // java float rgdxa = (float)(newDefTable.getRgdxa(i + 1) - newDefTable.getRgdxa(i));
          let mut rgdxa: f32 = ((newDefTable.getRgdxa(i + 1) - newDefTable.getRgdxa(i)) as f32);
          // java newTableCell.setWidth((int)(rgdxa / (float)this.mTableWidthLen * 5000.0F));
          newTableCell.setWidth(((rgdxa / ((self.mTableWidthLen) as f32) * 5000) as i32));
        }
        // java else if (this.mCompressPercent != null)
        else if let Some(mCompressPercent) = self.mCompressPercent.as_mut() {
          // java newTableCell.setWidth((int)((float)newTableCell.getWidth() * this.mCompressPercent));
          newTableCell.setWidth(((((newTableCell.getWidth()) as f32) * mCompressPercent) as i32));
        }
      }
      // java Tools.setDefTable(this.document, cellsItem, oldDefTable, newDefTable);
      Tools::setDefTable(self.document, cellsItem, oldDefTable, newDefTable);
    }

    // java private DefTableBuilder insertColumns(DefTable oldDefTable, CellsItem cellsItem, boolean after, int colCount)
    pub fn insertColumns(&self, oldDefTable: &DefTable, cellsItem: &CellsItem, after: bool, colCount: i32) -> DefTableBuilder {
      // java DefTableBuilder newDefTable;
      let mut newDefTable: DefTableBuilder = DefTableBuilder::default();
      // java if (-1 == cellsItem.startCellIndex)
      if -1 == cellsItem.startCellIndex {
        // java newDefTable = DefTableBuilder.obtain(oldDefTable);
        newDefTable = DefTableBuilder::obtain__deftable(&oldDefTable);
      }
      // java else
      else {
        // java newDefTable = DefTableBuilder.obtain();
        newDefTable = DefTableBuilder::obtain();
        // java int oldCellCount = oldDefTable.getCellCount();
        let mut oldCellCount: i32 = oldDefTable.getCellCount();
        // java int newCellCount = oldCellCount + colCount;
        let mut newCellCount: i32 = oldCellCount + colCount;
        // java newDefTable.setCellCount(newCellCount);
        newDefTable.setCellCount(newCellCount);
        // java newDefTable.setItMax(newCellCount + 1);
        newDefTable.setItMax(newCellCount + 1);
        // java int oldPos = 0;
        let mut oldPos: i32 = 0;
        // java int newPos = 0;
        let mut newPos: i32 = 0;
        // java int maxPosBeforeInsert;
        let mut maxPosBeforeInsert: i32;
        // java if (after)
        if after {
          // java maxPosBeforeInsert = cellsItem.endCellIndex + 1;
          maxPosBeforeInsert = cellsItem.endCellIndex + 1;
        }
        // java else
        else {
          // java maxPosBeforeInsert = cellsItem.startCellIndex;
          maxPosBeforeInsert = cellsItem.startCellIndex;
        }
        // java while (oldPos < maxPosBeforeInsert)
        while oldPos < maxPosBeforeInsert {
          // java newDefTable.setTableCellBuilder(newPos++, TableCellBuilder.obtain(oldDefTable.getTableCell(oldPos)));
          newDefTable.setTableCellBuilder({ let _tmp = newPos; newPos += 1; _tmp }, TableCellBuilder::obtain__tablecell(&oldDefTable.getTableCell(oldPos)));
          // java ++oldPos;
          { let _tmp = oldPos; oldPos += 1; _tmp };
        }
        // java TableCellBuilder newCell;
        let mut newCell: TableCellBuilder = TableCellBuilder::default();
        // java if (after)
        if after {
          // java if (oldCellCount - 1 == cellsItem.endCellIndex)
          if oldCellCount - 1 == cellsItem.endCellIndex {
            // java newCell = TableCellBuilder.obtain(oldDefTable.getTableCell(cellsItem.endCellIndex));
            newCell = TableCellBuilder::obtain__tablecell(&oldDefTable.getTableCell(cellsItem.endCellIndex));
            // java newCell.setHoriMerge(0);
            newCell.setHoriMerge(0);
          }
          // java else
          else {
            // java newCell = TableCellBuilder.obtain(oldDefTable.getTableCell(cellsItem.endCellIndex + 1));
            newCell = TableCellBuilder::obtain__tablecell(&oldDefTable.getTableCell(cellsItem.endCellIndex + 1));
            // java newCell.setBrcRight(newCell.getBrcLeft());
            newCell.setBrcRight(newCell.getBrcLeft());
          }
        }
        // java else
        else {
          // java newCell = TableCellBuilder.obtain(oldDefTable.getTableCell(cellsItem.startCellIndex));
          newCell = TableCellBuilder::obtain__tablecell(&oldDefTable.getTableCell(cellsItem.startCellIndex));
          // java if (cellsItem.startCellIndex != 0)
          if cellsItem.startCellIndex != 0 {
            // java newCell.setBrcRight(newCell.getBrcLeft());
            newCell.setBrcRight(newCell.getBrcLeft());
          }
          // java if (3 == newCell.getHoriMerge())
          if 3 == newCell.getHoriMerge() {
            // java newCell.setHoriMerge(0);
            newCell.setHoriMerge(0);
          }
        }
        // java newCell.setVertMerge(0);
        newCell.setVertMerge(0);
        let mut i: i32 = 0;
        // java for(int i = 0; i < colCount; ++i)
        while i < colCount {
          // java newDefTable.setTableCellBuilder(newPos++, TableCellBuilder.obtain(newCell));
          newDefTable.setTableCellBuilder({ let _tmp = newPos; newPos += 1; _tmp }, TableCellBuilder::obtain__tablecell(&newCell));
        }
        // java while (oldPos < oldCellCount)
        while oldPos < oldCellCount {
          // java newDefTable.setTableCellBuilder(newPos++, TableCellBuilder.obtain(oldDefTable.getTableCell(oldPos)));
          newDefTable.setTableCellBuilder({ let _tmp = newPos; newPos += 1; _tmp }, TableCellBuilder::obtain__tablecell(&oldDefTable.getTableCell(oldPos)));
          // java ++oldPos;
          { let _tmp = oldPos; oldPos += 1; _tmp };
        }
        // java oldPos = 0;
        oldPos = 0;
        newPos = 0;
        // java for(newPos = 0 oldPos < maxPosBeforeInsert ++oldPos)
        while oldPos < maxPosBeforeInsert {
          // java newDefTable.setRgdxa(newPos++, oldDefTable.getRgdxa(oldPos));
          newDefTable.setRgdxa({ let _tmp = newPos; newPos += 1; _tmp }, oldDefTable.getRgdxa(oldPos));
        }
        let mut i: i32 = 0;
        // java for(int i = 0; i < colCount; ++i)
        while i < colCount {
          // java newDefTable.setRgdxa(newPos++, oldDefTable.getRgdxa(oldPos) + this.mInsertCellWidth * i);
          newDefTable.setRgdxa({ let _tmp = newPos; newPos += 1; _tmp }, oldDefTable.getRgdxa(oldPos) + self.mInsertCellWidth * i);
        }
        let mut offset: i32 = self.mInsertCellWidth * colCount;
        // java for(int offset = this.mInsertCellWidth * colCount; oldPos <= oldCellCount ++oldPos)
        while oldPos <= oldCellCount {
          // java newDefTable.setRgdxa(newPos++, oldDefTable.getRgdxa(oldPos) + offset);
          newDefTable.setRgdxa({ let _tmp = newPos; newPos += 1; _tmp }, oldDefTable.getRgdxa(oldPos) + offset);
        }
      }
      // java return newDefTable;
      return newDefTable;
    }

    // java private void setDefSHD4InsertCells(TableDefSHD defShds, CellsItem cellsItem, boolean after, int colCount)
    pub fn setDefSHD4InsertCells(&mut self, defShds: Option<&TableDefSHD>, cellsItem: &CellsItem, after: bool, colCount: i32) {
      // java TableDefSHD oldDefShd = defShds;
      let mut oldDefShd: TableDefSHD = defShds;
      // java if (null != defShds && -1 != cellsItem.startCellIndex)
      if let Some(defShds) = defShds {
        if -1 != cellsItem.startCellIndex {
          // java int oldCellCount = defShds.getCount();
          let mut oldCellCount: i32 = defShds.getCount();
          // java int newCellCount = oldCellCount + colCount;
          let mut newCellCount: i32 = oldCellCount + colCount;
          // java TableDefSHDBuilder newDefShd = new TableDefSHDBuilder(newCellCount);
          let mut newDefShd: TableDefSHDBuilder = TableDefSHDBuilder::new(newCellCount);
          // java int oldPos = 0;
          let mut oldPos: i32 = 0;
          // java int newPos = 0;
          let mut newPos: i32 = 0;
          // java int maxPosBeforeInsert;
          let mut maxPosBeforeInsert: i32;
          // java if (after)
          if after {
            // java maxPosBeforeInsert = cellsItem.endCellIndex + 1;
            maxPosBeforeInsert = cellsItem.endCellIndex + 1;
          }
          // java else
          else {
            // java maxPosBeforeInsert = cellsItem.startCellIndex;
            maxPosBeforeInsert = cellsItem.startCellIndex;
          }
          // java while (oldPos < maxPosBeforeInsert)
          while oldPos < maxPosBeforeInsert {
            // java newDefShd.setCellSHD(newPos++, oldDefShd.getCellSHD(oldPos));
            newDefShd.setCellSHD({ let _tmp = newPos; newPos += 1; _tmp }, oldDefShd.getCellSHD(oldPos));
            // java ++oldPos;
            { let _tmp = oldPos; oldPos += 1; _tmp };
          }
          // java SHD newSHD;
          let mut newSHD: SHD = SHD::default();
          // java if (after)
          if after {
            // java if (oldCellCount - 1 == cellsItem.endCellIndex)
            if oldCellCount - 1 == cellsItem.endCellIndex {
              // java newSHD = oldDefShd.getCellSHD(cellsItem.endCellIndex);
              newSHD = oldDefShd.getCellSHD(cellsItem.endCellIndex);
            }
            // java else
            else {
              // java newSHD = oldDefShd.getCellSHD(cellsItem.endCellIndex + 1);
              newSHD = oldDefShd.getCellSHD(cellsItem.endCellIndex + 1);
            }
          }
          // java else
          else {
            // java newSHD = oldDefShd.getCellSHD(cellsItem.startCellIndex);
            newSHD = oldDefShd.getCellSHD(cellsItem.startCellIndex);
          }
          // java int insertPos = newPos;
          let mut insertPos: i32 = newPos;
          let mut i: i32 = 0;
          // java for(int i = 0; i < colCount; ++i)
          while i < colCount {
            // java newDefShd.setCellSHD(newPos++, newSHD);
            newDefShd.setCellSHD({ let _tmp = newPos; newPos += 1; _tmp }, newSHD);
          }
          // java while (oldPos < oldCellCount)
          while oldPos < oldCellCount {
            // java newDefShd.setCellSHD(newPos++, oldDefShd.getCellSHD(oldPos));
            newDefShd.setCellSHD({ let _tmp = newPos; newPos += 1; _tmp }, oldDefShd.getCellSHD(oldPos));
            // java ++oldPos;
            { let _tmp = oldPos; oldPos += 1; _tmp };
          }
          // java if (Tools.checkTableFormatChange(this.document, cellsItem))
          if Tools::checkTableFormatChange(self.document, cellsItem) {
            // java TableDefSHD defShd = this.calculateOldDefShd(newDefShd, insertPos, colCount);
            let mut defShd: TableDefSHD = self.calculateOldDefShd(&newDefShd, insertPos, colCount);
            // java CTableRowModifyCmd cmd = new CTableRowModifyCmd(this.document, cellsItem.rowRange, 308, oldDefShd, defShd);
            let mut cmd: CTableRowModifyCmd = CTableRowModifyCmd::new(self.document, cellsItem.rowRange, 308, oldDefShd, defShd);
            // java cmd.setModifyRevision(false);
            cmd.setModifyRevision(false);
            // java cmd.execute();
            cmd.execute();
            // java this.getTextDocument().submit(cmd);
            self.super_.getTextDocument().submit(cmd);
          }
          // java Tools.setDefSHD(this.getTextDocument(), this.document, cellsItem, newDefShd, oldDefShd);
          Tools::setDefSHD(self.super_.getTextDocument(), self.document, cellsItem, newDefShd, oldDefShd);
        }
      }
    }

    // java private TableDefSHD calculateOldDefShd(TableDefSHD newDefShd, int insertPos, int colCount)
    pub fn calculateOldDefShd(&mut self, newDefShd: &TableDefSHD, insertPos: i32, colCount: i32) -> TableDefSHD {
      // java TableDefSHDBuilder defShd = new TableDefSHDBuilder(newDefShd);
      let mut defShd: TableDefSHDBuilder = TableDefSHDBuilder::new(newDefShd);
      // java SHD shd = SHD.obtain(-2, -2, 0);
      let mut shd: SHD = SHD::obtain__int_int_int(-2, -2, 0);
      let mut i: i32 = 0;
      // java for(int i = 0; i < colCount; ++i)
      while i < colCount {
        // java defShd.setCellSHD(insertPos++, shd);
        defShd.setCellSHD({ let _tmp = insertPos; insertPos += 1; _tmp }, shd);
      }
      // java return defShd;
      return defShd;
    }

    // java private void setTableSetBrcs(TableSetBRC[] setBRCs, CellsItem cellsItem, boolean after, int colCount, int oldCellCount)
    pub fn setTableSetBrcs(&self, setBRCs: Option<&Vec<TableSetBRC>>, cellsItem: &CellsItem, after: bool, colCount: i32, oldCellCount: i32) {
      // java if (setBRCs != null && cellsItem.startCellIndex != -1)
      if let Some(setBRCs) = setBRCs {
        if cellsItem.startCellIndex != -1 {
          // java int insertPos;
          let mut insertPos: i32;
          // java if (after)
          if after {
            // java insertPos = cellsItem.endCellIndex + 1;
            insertPos = cellsItem.endCellIndex + 1;
          }
          // java else
          else {
            // java insertPos = cellsItem.startCellIndex;
            insertPos = cellsItem.startCellIndex;
          }
          // java TableSetBRC[] newBrcs = new TableSetBRC[setBRCs.length];
          let mut newBrcs: Vec<TableSetBRC> = vec![Default::default(); (setBRCs.len() as i32) as usize];
          let mut i: i32 = 0;
          // java for(int i = 0; i < setBRCs.length; ++i)
          while i < (setBRCs.len() as i32) {
            // java TableSetBRC oldSetBRC = setBRCs[i];
            let mut oldSetBRC: TableSetBRC = setBRCs[(i) as usize];
            // java int start = oldSetBRC.getFirst();
            let mut start: i32 = oldSetBRC.getFirst();
            // java int end = oldSetBRC.getLast();
            let mut end: i32 = oldSetBRC.getLast();
            // java if (insertPos < start)
            if insertPos < start {
              // java start += colCount;
              start = start + colCount;
              // java end += colCount;
              end = end + colCount;
            }
            // java else if (insertPos < end)
            else if insertPos < end {
              // java end += colCount;
              end = end + colCount;
            }
            // java else if (insertPos == end && end == oldCellCount)
            else if insertPos == end && end == oldCellCount {
              // java end += colCount;
              end = end + colCount;
            }
            // java TableSetBRC newSetBRC = new TableSetBRC();
            let mut newSetBRC: TableSetBRC = TableSetBRC::new();
            // java newSetBRC.setBRC(oldSetBRC.getBRC());
            newSetBRC.setBRC(oldSetBRC.getBRC());
            // java newSetBRC.setBordersToApply(oldSetBRC.getBordersToApply());
            newSetBRC.setBordersToApply(oldSetBRC.getBordersToApply());
            // java newSetBRC.setRange(start, end);
            newSetBRC.setRange(start, end);
            // java newBrcs[i] = newSetBRC;
            newBrcs[(i) as usize] = newSetBRC;
          }
          // java if (Tools.checkTableFormatChange(this.document, cellsItem))
          if Tools::checkTableFormatChange(self.document, cellsItem) {
            // java CTableRowModifyCmd cmd = new CTableRowModifyCmd(this.document, cellsItem.rowRange, 311, setBRCs, newBrcs);
            let mut cmd: CTableRowModifyCmd = CTableRowModifyCmd::new(self.document, cellsItem.rowRange, 311, setBRCs, newBrcs);
            // java cmd.setModifyRevision(false);
            cmd.setModifyRevision(false);
            // java cmd.execute();
            cmd.execute();
            // java this.getTextDocument().submit(cmd);
            self.super_.getTextDocument().submit(cmd);
          }
          // java Tools.submitCMD(this.document, cellsItem.rowRange, 311, setBRCs, newBrcs);
          Tools::submitCMD(self.document, cellsItem.rowRange, 311, setBRCs, newBrcs);
        }
      }
    }

    // java private void setBrcCv(PropertySet finalProperty, CellsItem cellsItem, boolean after, int colCount, int oldCellCount)
    pub fn setBrcCv(&mut self, finalProperty: &PropertySet, cellsItem: &CellsItem, after: bool, colCount: i32, oldCellCount: i32) {
      // java int insertPos;
      let mut insertPos: i32;
      // java if (after)
      if after {
        // java insertPos = cellsItem.endCellIndex + 1;
        insertPos = cellsItem.endCellIndex + 1;
      }
      // java else
      else {
        // java insertPos = cellsItem.startCellIndex;
        insertPos = cellsItem.startCellIndex;
      }
      // java BrcCvFactory cvFactory = new BrcCvFactory(finalProperty);
      let mut cvFactory: BrcCvFactory = BrcCvFactory::new(finalProperty);
      // java cvFactory.insert(insertPos, colCount, oldCellCount);
      cvFactory.insert(insertPos, colCount, oldCellCount);
      // java cvFactory.commit(cellsItem);
      cvFactory.commit(cellsItem);
    }

    // java public void setSHD(SHD shd)
    pub fn setSHD(&self, shd: &SHD) {
      // java if (0 != this.mCells.size())
      if 0 != (self.mCells.len() as i32) {
        // java this.getTextDocument().start();
        self.super_.getTextDocument().start();
            // java TableInfo tableInfo = this.getTableInfo();
            let mut tableInfo: TableInfo = self.super_.getTableInfo();
            // java int i = 0;
            let mut i: i32 = 0;
            let mut size: i32 = (self.mCells.len() as i32);
            // java for(int size = this.mCells.size(); i < size; ++i)
            while i < size {
              // java CellsItem cellItem = (CellsItem)this.mCells.get(i);
              let mut cellItem: CellsItem = (self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap();
              // java PropertySet prop = cellItem.getFinalRowProperty();
              let mut prop: PropertySet = cellItem.getFinalRowProperty();
              // java Assert.assertNotNull("prop should not be null!", prop);
              Assert::assertNotNull__string_object(&"prop should not be null!", Some(&prop));
              // java TableDefSHD oldTableDefSHD = (TableDefSHD)prop.getObject(308, (Object)null);
              let mut oldTableDefSHD: TableDefSHD = (prop.getObject(308, None)).downcast_ref::<TableDefSHD>().unwrap();
              // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getStart(cellItem.rowRange));
              let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart(cellItem.rowRange));
              // java int cellCount = rowInfo.size();
              let mut cellCount: i32 = (rowInfo.len() as i32);
              // java TableDefSHDBuilder newTableDefSHD = new TableDefSHDBuilder(cellCount);
              let mut newTableDefSHD: TableDefSHDBuilder = TableDefSHDBuilder::new(cellCount);
              // java if (null != oldTableDefSHD)
              if oldTableDefSHD.is_some() {
                let mut j: i32 = 0;
                // java for(int j = 0; j < cellCount ++j)
                while j < cellCount {
                  // java newTableDefSHD.setCellSHD(j, oldTableDefSHD.getCellSHD(j));
                  newTableDefSHD.setCellSHD(j, oldTableDefSHD.getCellSHD(j));
                }
              }
              // java int j = cellItem.startCellIndex;
              let mut j: i32 = cellItem.startCellIndex;
              let mut maxIndex: i32 = cellItem.endCellIndex;
              // java for(int maxIndex = cellItem.endCellIndex; j <= maxIndex ++j)
              while j <= maxIndex {
                // java newTableDefSHD.setCellSHD(j, shd);
                newTableDefSHD.setCellSHD(j, shd);
              }
              // java Tools.setDefSHD(this.getTextDocument(), this.document, cellItem, newTableDefSHD, oldTableDefSHD);
              Tools::setDefSHD(self.super_.getTextDocument(), self.document, cellItem, newTableDefSHD, oldTableDefSHD);
            }
              // java this.getTextDocument().commit("set col shd");
              self.super_.getTextDocument().commit("set col shd");
      }
    }

    // java public int delete(KRange.DeleteRangeReason reason)
    pub fn delete(&mut self, reason: &KRange::DeleteRangeReason) -> i32 {
      // java return reason == KRange.DeleteRangeReason.backspace_key ? this.removeCells() : super.delete(reason);
      return (if reason == KRange::DeleteRangeReason.backspace_key { self.super_.removeCells() } else { self.super_.delete(reason) });
    }

}

impl std::ops::Deref for KColRange {
    type Target = MergeSpilit;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KColRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
