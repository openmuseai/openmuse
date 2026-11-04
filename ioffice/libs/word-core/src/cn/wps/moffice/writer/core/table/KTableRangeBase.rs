// Java: import cn.wps.kfc.impl.arrayList.WLongArrayList
use util::kfc::r#impl::arrayList::WLongArrayList::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KParaRunRevision
use crate::cn::wps::moffice::writer::core::KParaRunRevision::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.KRangeBase
use crate::cn::wps::moffice::writer::core::KRangeBase::*;
// Java: import cn.wps.moffice.writer.core.SelectionType
use crate::cn::wps::moffice::writer::core::SelectionType::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{
    Entry as IPapxTableEntry, IPapxTable,
};
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
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
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::*;
// Java: import cn.wps.moffice.writer.data.value.DefTable
use crate::cn::wps::moffice::writer::data::value::DefTable::*;
// Java: import cn.wps.moffice.writer.data.value.DefTableBuilder
use crate::cn::wps::moffice::writer::data::value::DefTableBuilder::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.data.value.TableCell
use crate::cn::wps::moffice::writer::data::value::TableCell::*;
// Java: import cn.wps.moffice.writer.data.value.TableCellBuilder
use crate::cn::wps::moffice::writer::data::value::TableCellBuilder::*;
// Java: import cn.wps.moffice.writer.data.value.TableDefSHD
use crate::cn::wps::moffice::writer::data::value::TableDefSHD::*;
// Java: import cn.wps.moffice.writer.data.value.TableDefSHDBuilder
use crate::cn::wps::moffice::writer::data::value::TableDefSHDBuilder::*;
// Java: import cn.wps.moffice.writer.data.value.TableSetBRC
use crate::cn::wps::moffice::writer::data::value::TableSetBRC::*;
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
use crate::cn::wps::util::concurrent::locks::RangeReadLock::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.table.CellsItem
use crate::cn::wps::moffice::writer::core::table::CellsItem::*;
// Java: import cn.wps.moffice.writer.core.table.RegularChecker
use crate::cn::wps::moffice::writer::core::table::RegularChecker::*;
// Java: import cn.wps.moffice.writer.core.table.KCellIterator
use crate::cn::wps::moffice::writer::core::table::KCellIterator::*;
// Java: import cn.wps.moffice.writer.core.table.KCellIteratorForCol
use crate::cn::wps::moffice::writer::core::table::KCellIteratorForCol::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;
// Java: import cn.wps.moffice.writer.core.table.KComplexTableRange
use crate::cn::wps::moffice::writer::core::table::KComplexTableRange::*;
// Java: import cn.wps.moffice.writer.core.table.DefTableFixer
use crate::cn::wps::moffice::writer::core::table::DefTableFixer::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.core.table.BrcCvFactory
use crate::cn::wps::moffice::writer::core::table::BrcCvFactory::*;
// Java: import cn.wps.moffice.writer.core.table.RowRevisionFixer
use crate::cn::wps::moffice::writer::core::table::RowRevisionFixer::*;

// java public abstract class KTableRangeBase extends KRangeBase
#[derive(Default)]
pub struct KTableRangeBase {
    // java KRangeBase super_
    pub(crate) super_: KRangeBase,
    // java protected ArrayList<CellsItem> mCells = new ArrayList();
    pub(crate) mCells: Option<Vec<CellsItem>>,
    // java private KParaRunRevision mKRevision;
    mKRevision: Option<KParaRunRevision>,
}

// java public static enum RevisionType
#[derive(Copy, Clone, Debug)]
pub enum RevisionType {
}

// java public static class TableRangeContext
#[derive(Default)]
pub struct TableRangeContext {
    // java public SelectionType selType;
    pub(crate) selType: Option<SelectionType>,
    // java public int rootLevel;
    pub(crate) rootLevel: i32,
    // java public boolean startInTable;
    pub(crate) startInTable: bool,
    // java public boolean endInTable;
    pub(crate) endInTable: bool,
    // java public boolean hasTable;
    pub(crate) hasTable: bool,
}

// java public abstract class KTableRangeBase extends KRangeBase
impl KTableRangeBase {
    // java protected KParaRunRevision getKRevision()
    pub fn getKRevision(&mut self) -> KParaRunRevision {
      // java if (this.mKRevision == null)
      if self.mKRevision.is_none() {
        // java this.mKRevision = new KParaRunRevision(this.document);
        self.mKRevision = KParaRunRevision::new(self.document);
      }
      // java return this.mKRevision;
      return self.mKRevision;
    }

    // java public static RevisionType getRevisionType(RowInfo rowInfo)
    pub fn getRevisionType(rowInfo: &RowInfo) -> RevisionType {
      // java if (rowInfo.isRevisionInsert())
      if rowInfo.isRevisionInsert() {
        // java return KTableRangeBase.RevisionType.insert;
        return KTableRangeBase::RevisionType.insert;
      }
      // java else
      else {
        // java return rowInfo.isRevisionDelete() ? KTableRangeBase.RevisionType.delete : KTableRangeBase.RevisionType.normal;
        return (if rowInfo.isRevisionDelete() { KTableRangeBase::RevisionType.delete } else { KTableRangeBase::RevisionType.normal });
      }
    }

    // java protected PropertySet getFinalChpx(PropertySet chpx, RevisionType type)
    pub fn getFinalChpx(&self, chpx: &mut PropertySet, r#type: &RevisionType) -> PropertySet {
      // java chpx = PropertyUtil.getFinalNormalRunFormat(chpx);
      chpx = PropertyUtil::getFinalNormalRunFormat(&chpx);
      // java KParaRunRevision kRevision = this.getKRevision();
      let mut kRevision: KParaRunRevision = self.getKRevision();
      // java if (type == KTableRangeBase.RevisionType.insert)
      if r#type == KTableRangeBase::RevisionType.insert {
        // java chpx = kRevision.setRunInsertRevision(chpx);
        chpx = kRevision.setRunInsertRevision(chpx);
      }
      // java else if (type == KTableRangeBase.RevisionType.delete)
      else if r#type == KTableRangeBase::RevisionType.delete {
        // java chpx = kRevision.setRunDeleteRevision(chpx);
        chpx = kRevision.setRunDeleteRevision(chpx);
      }
      // java MutablePropertySet propertySet = new MutablePropertySet(chpx);
      let mut propertySet: MutablePropertySet = MutablePropertySet::new(chpx);
      // java if (propertySet.getInt(2, 0) == 4095)
      if propertySet.getInt(2, 0) == 4095 {
        // java propertySet.remove(2);
        propertySet.remove(2);
      }
      // java return propertySet.freeze();
      return propertySet.freeze();
    }

    // java public boolean isRegular(TableInfo tableInfo)
    pub fn isRegular(&self, tableInfo: &dyn TableInfo) -> bool {
      // java RegularChecker checker = new RegularChecker(this.document, tableInfo, this.mCells);
      let mut checker: RegularChecker = RegularChecker::new(self.document, tableInfo, self.mCells);
      // java int firstRowIndex = tableInfo.getRowByCP(RANGEFactory.getStart(((CellsItem)this.mCells.get(0)).rowRange)).getIndex();
      let mut firstRowIndex: i32 = tableInfo.getRowByCP(RANGEFactory::getStart((self.mCells.get(0)).downcast_ref::<CellsItem>().unwrap().rowRange)).getIndex();
      let mut i: i32 = (self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) - 1;
      // java for(int i = this.mCells.size() - 1; i >= 0 --i)
      while i >= 0 {
        // java CellsItem cellsItem = (CellsItem)this.mCells.get(i);
        let mut cellsItem: CellsItem = (self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap();
        // java if (cellsItem.startCellIndex != -1)
        if cellsItem.startCellIndex != -1 {
          // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getStart(cellsItem.rowRange));
          let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart(cellsItem.rowRange));
          let mut j: i32 = cellsItem.startCellIndex;
          // java for(int j = cellsItem.startCellIndex; j <= cellsItem.endCellIndex ++j)
          while j <= cellsItem.endCellIndex {
            // java CellInfo cellInfo = rowInfo.getCellByIndex(j);
            let mut cellInfo: CellInfo = rowInfo.getCellByIndex(j);
            // java if (cellInfo.isVertMergeCell() && !checker.check(firstRowIndex + i, cellInfo))
            if cellInfo.isVertMergeCell() && !checker.check(firstRowIndex + i, cellInfo) {
              // java return false;
              return false;
            }
          }
        }
      }
      // java return true;
      return true;
    }

    // java public static ArrayList<CellsItem> regularize(Document document, ArrayList<CellsItem> cells)
    pub fn regularize__document_arraylistcellsitem(document: &dyn Document, cells: &Vec<CellsItem>) -> Vec<CellsItem> {
      // java RangeReadLock readLock = document.getMainDocument().readLock();
      let mut readLock: RangeReadLock = document.getMainDocument().readLock();
      // java ArrayList var16;
      let mut var16: Vec<()> = vec![];
          // java int rowEndCP = RANGEFactory.getEnd(((CellsItem)cells.get(0)).rowRange) - 1;
          let mut rowEndCP: i32 = RANGEFactory::getEnd((cells.get(0)).downcast_ref::<CellsItem>().unwrap().rowRange) - 1;
          // java TableInfo tableInfo = document.getTableInfos().getTableInfoByRange(rowEndCP, rowEndCP);
          let mut tableInfo: TableInfo = document.getTableInfos().getTableInfoByRange(rowEndCP, rowEndCP);
          // java RegularChecker checker = new RegularChecker(document, tableInfo, cells);
          let mut checker: RegularChecker = RegularChecker::new(document, tableInfo, cells);
          // java int firstRowIndex = tableInfo.getRowByCP(RANGEFactory.getStart(((CellsItem)cells.get(0)).rowRange)).getIndex();
          let mut firstRowIndex: i32 = tableInfo.getRowByCP(RANGEFactory::getStart((cells.get(0)).downcast_ref::<CellsItem>().unwrap().rowRange)).getIndex();
          let mut i: i32 = (cells.len() as i32) - 1;
          // java for(int i = cells.size() - 1; i >= 0 --i)
          while i >= 0 {
            // java CellsItem cellsItem = (CellsItem)cells.get(i);
            let mut cellsItem: CellsItem = (cells.get(i)).downcast_ref::<CellsItem>().unwrap();
            // java int rowIndex = firstRowIndex + i;
            let mut rowIndex: i32 = firstRowIndex + i;
            // java RowInfo rowInfo = tableInfo.getRowByIndex(rowIndex);
            let mut rowInfo: RowInfo = tableInfo.getRowByIndex(rowIndex);
            // java if (cellsItem.startCellIndex != -1)
            if cellsItem.startCellIndex != -1 {
              let mut j: i32 = cellsItem.startCellIndex;
              // java for(int j = cellsItem.startCellIndex; j <= cellsItem.endCellIndex ++j)
              while j <= cellsItem.endCellIndex {
                // java CellInfo cellInfo = rowInfo.getCellByIndex(j);
                let mut cellInfo: CellInfo = rowInfo.getCellByIndex(j);
                // java if (cellInfo.isVertMergeCell())
                if cellInfo.isVertMergeCell() {
                  // java checker.add(rowIndex, cellInfo);
                  checker.add(rowIndex, cellInfo);
                }
              }
            }
          }
          // java var16 = checker.getCells();
          var16 = checker.getCells();
            // java readLock.unlock();
            readLock.unlock();
      // java return var16;
      return var16;
    }

    // java public void regularize()
    pub fn regularize(&mut self) {
      // java this.mCells = regularize(this.document, this.mCells);
      self.mCells = Self::regularize__document_arraylistcellsitem(&self.document, &self.mCells);
    }

    // java public long getLeftTop()
    pub fn getLeftTop(&mut self) -> i64 {
      // java RangeReadLock readLock = this.document.readLock();
      let mut readLock: RangeReadLock = self.document.readLock();
      // java long var5;
      let mut var5: i64;
          // java TableInfo tableInfo = this.getTableInfo();
          let mut tableInfo: TableInfo = self.getTableInfo();
          // java CellsItem cellsItem = (CellsItem)this.mCells.get(0);
          let mut cellsItem: CellsItem = (self.mCells.get(0)).downcast_ref::<CellsItem>().unwrap();
          // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getStart(cellsItem.rowRange));
          let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart(cellsItem.rowRange));
          // java var5 = rowInfo.getCellByIndex(cellsItem.startCellIndex).getRange();
          var5 = rowInfo.getCellByIndex(cellsItem.startCellIndex).getRange();
            // java readLock.unlock();
            readLock.unlock();
      // java return var5;
      return var5;
    }

    // java public long getRightBottom()
    pub fn getRightBottom(&mut self) -> i64 {
      // java RangeReadLock readLock = this.document.readLock();
      let mut readLock: RangeReadLock = self.document.readLock();
      // java long var5;
      let mut var5: i64;
          // java TableInfo tableInfo = this.getTableInfo();
          let mut tableInfo: TableInfo = self.getTableInfo();
          // java CellsItem cellsItem = (CellsItem)this.mCells.get(this.mCells.size() - 1);
          let mut cellsItem: CellsItem = (self.mCells.get((self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) - 1)).downcast_ref::<CellsItem>().unwrap();
          // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getStart(cellsItem.rowRange));
          let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart(cellsItem.rowRange));
          // java var5 = rowInfo.getCellByIndex(cellsItem.endCellIndex).getRange();
          var5 = rowInfo.getCellByIndex(cellsItem.endCellIndex).getRange();
            // java readLock.unlock();
            readLock.unlock();
      // java return var5;
      return var5;
    }

    // java public int getLevel()
    pub fn getLevel(&mut self) -> i32 {
      // java RangeReadLock readLock = this.document.readLock();
      let mut readLock: RangeReadLock = self.document.readLock();
      // java int var2;
      let mut var2: i32;
          // java var2 = this.getTableInfo().getLevel();
          var2 = self.getTableInfo().getLevel();
            // java readLock.unlock();
            readLock.unlock();
      // java return var2;
      return var2;
    }

    // java public boolean isRTL()
    pub fn isRTL(&mut self) -> bool {
      // java RangeReadLock readLock = this.document.readLock();
      let mut readLock: RangeReadLock = self.document.readLock();
      // java boolean var2;
      let mut var2: bool;
          // java var2 = this.getTableInfo().isRTL();
          var2 = self.getTableInfo().isRTL();
            // java readLock.unlock();
            readLock.unlock();
      // java return var2;
      return var2;
    }

    // java public TableInfo getTableInfo()
    pub fn getTableInfo(&self) -> TableInfo {
      // java long rowRange = ((CellsItem)this.mCells.get(0)).rowRange;
      let mut rowRange: i64 = (self.mCells.get(0)).downcast_ref::<CellsItem>().unwrap().rowRange;
      // java int rowEnd = RANGEFactory.getEnd(rowRange) - 1;
      let mut rowEnd: i32 = RANGEFactory::getEnd(rowRange) - 1;
      // java return this.document.getTableInfos().getTableInfoByRange(rowEnd, rowEnd);
      return self.document.getTableInfos().getTableInfoByRange(rowEnd, rowEnd);
    }

    // java public ArrayList<WLongArrayList> getCellsRange()
    pub fn getCellsRange(&self) -> Vec<WLongArrayList> {
      // java TableInfo tableInfo = this.getTableInfo();
      let mut tableInfo: TableInfo = self.getTableInfo();
      // java ArrayList<WLongArrayList> cellsRange = new ArrayList();
      let mut cellsRange: Vec<WLongArrayList> = Vec::new();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int size = this.mCells.size(); i < size; ++i)
      while i < size {
        // java CellsItem cellsItem = (CellsItem)this.mCells.get(i);
        let mut cellsItem: CellsItem = (self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap();
        // java WLongArrayList rowCells = new WLongArrayList();
        let mut rowCells: WLongArrayList = WLongArrayList::new();
        // java if (cellsItem.startCellIndex != -1)
        if cellsItem.startCellIndex != -1 {
          // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getStart(cellsItem.rowRange));
          let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart(cellsItem.rowRange));
          let mut j: i32 = cellsItem.startCellIndex;
          // java for(int j = cellsItem.startCellIndex; j <= cellsItem.endCellIndex ++j)
          while j <= cellsItem.endCellIndex {
            // java CellInfo cellInfo = rowInfo.getCellByIndex(j);
            let mut cellInfo: CellInfo = rowInfo.getCellByIndex(j);
            // java rowCells.add(cellInfo.getRange());
            rowCells.add(cellInfo.getRange());
          }
        }
        // java cellsRange.add(rowCells);
        cellsRange.add(rowCells);
      }
      // java return cellsRange;
      return cellsRange;
    }

    // java public WLongArrayList getCells()
    pub fn getCells(&self) -> WLongArrayList {
      // java TableInfo tableInfo = this.getTableInfo();
      let mut tableInfo: TableInfo = self.getTableInfo();
      // java WLongArrayList cellRanges = new WLongArrayList();
      let mut cellRanges: WLongArrayList = WLongArrayList::new();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int size = this.mCells.size(); i < size; ++i)
      while i < size {
        // java CellsItem cellsItem = (CellsItem)this.mCells.get(i);
        let mut cellsItem: CellsItem = (self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap();
        // java if (cellsItem.startCellIndex != -1)
        if cellsItem.startCellIndex != -1 {
          // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getStart(cellsItem.rowRange));
          let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart(cellsItem.rowRange));
          let mut j: i32 = cellsItem.startCellIndex;
          // java for(int j = cellsItem.startCellIndex; j <= cellsItem.endCellIndex ++j)
          while j <= cellsItem.endCellIndex {
            // java CellInfo cellInfo = rowInfo.getCellByIndex(j);
            let mut cellInfo: CellInfo = rowInfo.getCellByIndex(j);
            // java cellRanges.add(cellInfo.getRange());
            cellRanges.add(cellInfo.getRange());
          }
        }
      }
      // java return cellRanges;
      return cellRanges;
    }

    // java public KCellIterator getCellIterator()
    pub fn getCellIterator(&self) -> Box<dyn KCellIterator> {
      // java return new KCellIteratorForCol(this.document, this.mCells);
      return KCellIteratorForCol::new(self.document, self.mCells);
    }

    // java public ArrayList<CellsItem> getCellsItems()
    pub fn getCellsItems(&self) -> Vec<CellsItem> {
      // java return this.mCells;
      return self.mCells;
    }

    // java public int calcMaxInsertColumns(int colCount)
    pub fn calcMaxInsertColumns(&self, colCount: i32) -> i32 {
      // java if (colCount == 0)
      if colCount == 0 {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java TableInfo tableInfo = this.getTableInfo();
        let mut tableInfo: TableInfo = self.getTableInfo();
        let mut i: i32 = (self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) - 1;
        // java for(int i = this.mCells.size() - 1; i >= 0 --i)
        while i >= 0 {
          // java CellsItem cellsItem = (CellsItem)this.mCells.get(i);
          let mut cellsItem: CellsItem = (self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap();
          // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getStart(cellsItem.rowRange));
          let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart(cellsItem.rowRange));
          // java colCount = Math.min(colCount, 63 - rowInfo.size());
          colCount = Math::min(colCount, 63 - (rowInfo.len() as i32));
        }
        // java return Math.max(0, colCount);
        return Math::max(0, colCount);
      }
    }

    // java static int getRangeLevel(Document document, int cp)
    pub fn getRangeLevel(document: &dyn Document, cp: i32) -> i32 {
      // java IPapxTable.Entry paragraph = document.getPapxTable().seek(cp);
      let mut paragraph: Box<dyn IPapxTableEntry> = document.getPapxTable().seek(cp);
      // java PropertySet prop = paragraph.getProperty();
      let mut prop: PropertySet = paragraph.getProperty();
      // java if (!prop.getBoolean(223, false))
      if !prop.getBoolean(223, false) {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java int level = prop.getInt(224, 1);
        let mut level: i32 = prop.getInt(224, 1);
        // java return level;
        return level;
      }
    }

    // java public static TableRangeContext normalize(Document document, RANGE range)
    pub fn normalize(document: &dyn Document, range: &mut RANGE) -> Option<TableRangeContext> {
      // java if (range.start > range.end)
      if range.start > range.end {
        // java int t = range.start;
        let mut t: i32 = range.start;
        // java range.start = range.end;
        range.start = range.end;
        // java range.end = t;
        range.end = t;
      }
      // java if (range.start != range.end && range.start >= 0 && range.end <= document.getLength())
      if range.start != range.end && range.start >= 0 && range.end <= document.getLength() {
        // java TableInfo tableInfo = document.getTableInfos().getTableInfoByRange(range.start, range.end);
        let mut tableInfo: TableInfo = document.getTableInfos().getTableInfoByRange(range.start, range.end);
        // java boolean withinRootCell;
        let mut withinRootCell: bool;
        // java if (tableInfo != null && tableInfo.getEndCP() >= range.end)
        if tableInfo.is_some() && tableInfo.getEndCP() >= range.end {
          // java RowInfo rowInfo = tableInfo.getRowByCP(range.start);
          let mut rowInfo: RowInfo = tableInfo.getRowByCP(range.start);
          // java CellInfo cellInfo = rowInfo.getCellByCP(range.start);
          let mut cellInfo: CellInfo = rowInfo.getCellByCP(range.start);
          // java withinRootCell = cellInfo == null ? false : range.end <= cellInfo.getEndCP();
          withinRootCell = (if cellInfo.is_none() { Some(false) } else { Some(range.end <= cellInfo.getEndCP()) });
        }
        // java else
        else {
          // java withinRootCell = true;
          withinRootCell = true;
          // java tableInfo = null;
          tableInfo = None;
        }
        // java return expandRange(document, tableInfo, range, withinRootCell);
        return Self::expandRange(&document, Some(&tableInfo), &mut range, withinRootCell);
      }
      // java else
      else {
        // java return new TableRangeContext(SelectionType.NORMAL);
        return TableRangeContext::new(SelectionType::NORMAL);
      }
    }

    // java static TableRangeContext expandRange(Document document, TableInfo tableInfo, RANGE range, boolean withinRootCell)
    pub fn expandRange(document: &dyn Document, tableInfo: Option<&dyn TableInfo>, range: &mut RANGE, withinRootCell: bool) -> Option<TableRangeContext> {
      // java int rootLevel = tableInfo == null ? 0 : tableInfo.getLevel();
      let mut rootLevel: i32 = (if tableInfo.is_none() { 0 } else { tableInfo.getLevel() });
      // java TableRangeContext context = new TableRangeContext(SelectionType.NORMAL);
      let mut context: TableRangeContext = TableRangeContext::new(SelectionType::NORMAL);
      // java context.rootLevel = rootLevel;
      context.rootLevel = rootLevel;
      // java if (withinRootCell)
      if withinRootCell {
        // java if (getRangeLevel(document, range.start) >= rootLevel + 1)
        if Self::getRangeLevel(&document, range.start) >= rootLevel + 1 {
          // java TableInfo subTable = document.getTableInfos().getTableInfoByLevel(range.start, rootLevel + 1);
          let mut subTable: TableInfo = document.getTableInfos().getTableInfoByLevel(range.start, rootLevel + 1);
          // java range.start = subTable.getRowByCP(range.start).getStartCP();
          range.start = subTable.getRowByCP(range.start).getStartCP();
          // java context.startInTable = true;
          context.startInTable = true;
          // java context.hasTable = true;
          context.hasTable = true;
        }
        // java if (getRangeLevel(document, range.end - 1) >= rootLevel + 1)
        if Self::getRangeLevel(&document, range.end - 1) >= rootLevel + 1 {
          // java TableInfo subTable = document.getTableInfos().getTableInfoByLevel(range.end - 1, rootLevel + 1);
          let mut subTable: TableInfo = document.getTableInfos().getTableInfoByLevel(range.end - 1, rootLevel + 1);
          // java range.end = subTable.getRowByCP(range.end - 1).getEndCP();
          range.end = subTable.getRowByCP(range.end - 1).getEndCP();
          // java context.endInTable = true;
          context.endInTable = true;
          // java context.hasTable = true;
          context.hasTable = true;
        }
        // java if (rootLevel >= 1)
        if rootLevel >= 1 {
          // java RowInfo rowInfo = tableInfo.getRowByCP(range.start);
          let mut rowInfo: RowInfo = tableInfo.getRowByCP(range.start);
          // java CellInfo cellInfo = rowInfo.getCellByCP(range.start);
          let mut cellInfo: CellInfo = rowInfo.getCellByCP(range.start);
          // java if (range.end == cellInfo.getEndCP())
          if range.end == cellInfo.getEndCP() {
            // java range.start = cellInfo.getStartCP();
            range.start = cellInfo.getStartCP();
            // java context.selType = SelectionType.TABLECOLUMN;
            context.selType = SelectionType::TABLECOLUMN;
            // java return context;
            return context;
          }
        }
        // java if (!context.hasTable)
        if !context.hasTable {
          // java context.hasTable = document.getTableInfos().hasTable(range.start, range.end, rootLevel + 1);
          context.hasTable = document.getTableInfos().hasTable(range.start, range.end, rootLevel + 1);
        }
      }
      // java else
      else {
        // java context.selType = SelectionType.TABLECOLUMN;
        context.selType = SelectionType::TABLECOLUMN;
        // java context.startInTable = true;
        context.startInTable = true;
        // java context.endInTable = true;
        context.endInTable = true;
        // java context.hasTable = true;
        context.hasTable = true;
        // java RowInfo startRow = tableInfo.getRowByCP(range.start);
        let mut startRow: RowInfo = tableInfo.getRowByCP(range.start);
        // java if (range.start == startRow.getEndCP() - 1)
        if range.start == startRow.getEndCP() - 1 {
          // java --range.start;
          { let _tmp = range.start; range.start -= 1; _tmp };
        }
        // java CellInfo cellInfo = startRow.getCellByCP(range.start);
        let mut cellInfo: CellInfo = startRow.getCellByCP(range.start);
        // java range.start = cellInfo.getStartCP();
        range.start = cellInfo.getStartCP();
        // java RowInfo endRow = tableInfo.getRowByCP(range.end - 1);
        let mut endRow: RowInfo = tableInfo.getRowByCP(range.end - 1);
        // java if (range.end != endRow.getEndCP())
        if range.end != endRow.getEndCP() {
          // java range.end = endRow.getCellByCP(range.end - 1).getEndCP();
          range.end = endRow.getCellByCP(range.end - 1).getEndCP();
        }
        // java else if (range.start == startRow.getStartCP())
        else if range.start == startRow.getStartCP() {
          // java context.selType = SelectionType.TABLEROW;
          context.selType = SelectionType::TABLEROW;
        }
        // java else
        else {
          // java --range.end;
          { let _tmp = range.end; range.end -= 1; _tmp };
        }
      }
      // java return context;
      return context;
    }

    // java public static KTableRangeBase createTableRange(Document document, int start, int end)
    pub fn createTableRange(document: &dyn Document, start: i32, end: i32) -> Option<KTableRangeBase> {
      // java RANGE range = RANGE.obtain(start, end);
      let mut range: RANGE = RANGE::obtain__int_int(start, end);
      // java TableRangeContext context = normalize(document, range);
      let mut context: TableRangeContext = Self::normalize(&document, &mut range);
      // java SelectionType type = context.selType;
      let mut r#type: SelectionType = context.selType;
      // java if (type == SelectionType.TABLEROW)
      if r#type == SelectionType::TABLEROW {
        // java return KRowRange.createRowRange(document, range.start, range.end);
        return KRowRange::createRowRange(&document, range.start, range.end);
      }
      // java else if (type == SelectionType.TABLECOLUMN)
      else if r#type == SelectionType::TABLECOLUMN {
        // java return KColRange.createRectRange(document, range.start, range.end);
        return KColRange::createRectRange(&document, range.start, range.end);
      }
      // java else
      else {
        // java return !context.hasTable ? null : new KComplexTableRange(document, range.start, range.end, context.rootLevel + 1);
        return (if !context.hasTable { None } else { Some(KComplexTableRange::new(document, range.start, range.end, context.rootLevel + 1)) });
      }
    }

    // java public int delete(KRange.DeleteRangeReason reason)
    pub fn delete(&mut self, reason: &KRange::DeleteRangeReason) -> i32 {
      // java if (reason == KRange.DeleteRangeReason.delete_key)
      if reason == KRange::DeleteRangeReason.delete_key {
        // java return this.clearCellsContent();
        return self.clearCellsContent();
      }
      // java else if (reason == KRange.DeleteRangeReason.typetext)
      else if reason == KRange::DeleteRangeReason.typetext {
        // java CellsItem cellsItem = (CellsItem)this.mCells.get(0);
        let mut cellsItem: CellsItem = (self.mCells.get(0)).downcast_ref::<CellsItem>().unwrap();
        // java TableInfo tableInfo = this.getTableInfo();
        let mut tableInfo: TableInfo = self.getTableInfo();
        // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getStart(cellsItem.rowRange));
        let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart(cellsItem.rowRange));
        // java CellInfo cellInfo = rowInfo.getCellByIndex(cellsItem.startCellIndex);
        let mut cellInfo: CellInfo = rowInfo.getCellByIndex(cellsItem.startCellIndex);
        // java return this.clearCellContent(cellInfo);
        return self.clearCellContent(&cellInfo);
      }
      // java else
      else {
        // java return 0;
        return 0;
      }
    }

    // java private int clearCellsContent()
    pub fn clearCellsContent(&self) -> i32 {
      // java TableInfo tableInfo = this.getTableInfo();
      let mut tableInfo: TableInfo = self.getTableInfo();
      // java int subTableLevel = tableInfo.getLevel() + 1;
      let mut subTableLevel: i32 = tableInfo.getLevel() + 1;
      // java KComplexTableRange cells = new KComplexTableRange(this.document);
      let mut cells: KComplexTableRange = KComplexTableRange::new(self.document);
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int size = this.mCells.size(); i < size; ++i)
      while i < size {
        // java CellsItem cellsItem = (CellsItem)this.mCells.get(i);
        let mut cellsItem: CellsItem = (self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap();
        // java if (cellsItem.startCellIndex != -1)
        if cellsItem.startCellIndex != -1 {
          // java RowInfo rowInfo = tableInfo.getRowByCP(RANGEFactory.getStart(cellsItem.rowRange));
          let mut rowInfo: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart(cellsItem.rowRange));
          let mut j: i32 = cellsItem.startCellIndex;
          // java for(int j = cellsItem.startCellIndex; j <= cellsItem.endCellIndex ++j)
          while j <= cellsItem.endCellIndex {
            // java CellInfo cellInfo = rowInfo.getCellByIndex(j);
            let mut cellInfo: CellInfo = rowInfo.getCellByIndex(j);
            // java int cellStart = cellInfo.getStartCP();
            let mut cellStart: i32 = cellInfo.getStartCP();
            // java int cellEnd = cellInfo.getEndCP() - 1;
            let mut cellEnd: i32 = cellInfo.getEndCP() - 1;
            // java if (cellStart < cellEnd)
            if cellStart < cellEnd {
              // java cells.add(cellStart, cellEnd, subTableLevel);
              cells.add(cellStart, cellEnd, subTableLevel);
            }
          }
        }
      }
      // java return cells.delete(KRange.DeleteRangeReason.backspace_key);
      return cells.delete(KRange::DeleteRangeReason.backspace_key);
    }

    // java private int clearCellContent(CellInfo cellInfo)
    pub fn clearCellContent(&mut self, cellInfo: &CellInfo) -> i32 {
      // java if (cellInfo.size() == 1)
      if cellInfo.size() == 1 {
        // java TableInfo subTable = cellInfo.getTableByIndex(0);
        let mut subTable: TableInfo = cellInfo.getTableByIndex(0);
        // java if (subTable.getStartCP() == cellInfo.getStartCP() && subTable.getEndCP() == cellInfo.getEndCP() - 1)
        if subTable.getStartCP() == cellInfo.getStartCP() && subTable.getEndCP() == cellInfo.getEndCP() - 1 {
          // java CellInfo subCell = subTable.getRowByIndex(0).getCellByIndex(0);
          let mut subCell: CellInfo = subTable.getRowByIndex(0).getCellByIndex(0);
          // java return this.clearCellContent(subCell);
          return self.clearCellContent(&subCell);
        }
      }
      // java int cellStart = cellInfo.getStartCP();
      let mut cellStart: i32 = cellInfo.getStartCP();
      // java int cellEnd = cellInfo.getEndCP() - 1;
      let mut cellEnd: i32 = cellInfo.getEndCP() - 1;
      // java return cellStart < cellEnd ? this.safeRemoveText(this.document, cellStart, cellEnd) : 0;
      return (if cellStart < cellEnd { self.safeRemoveText(&self.document, cellStart, cellEnd) } else { 0 });
    }

    // java private int safeRemoveText(Document document, int start, int end)
    pub fn safeRemoveText(&mut self, document: &dyn Document, start: i32, end: i32) -> Option<i32> {
      // java KTableRangeBase tableRange = createTableRange(document, start, end);
      let mut tableRange: KTableRangeBase = Self::createTableRange(&document, start, end);
      // java return tableRange != null ? tableRange.delete(KRange.DeleteRangeReason.backspace_key) : document.getRange(start, end).removeText();
      return (if tableRange.is_some() { Some(tableRange.delete(KRange::DeleteRangeReason.backspace_key)) } else { Some(document.getRange__int_int(start, end).removeText()) });
    }

    // java public int removeCells()
    pub fn removeCells(&mut self) -> i32 {
      // java int deleteCount = 0;
      let mut deleteCount: i32 = 0;
      // java this.getTextDocument().start();
      self.super_.getTextDocument().start();
          // java TableInfo tableInfo = this.getTableInfo();
          let mut tableInfo: TableInfo = self.getTableInfo();
          // java int tableStart = tableInfo.getStartCP();
          let mut tableStart: i32 = tableInfo.getStartCP();
          // java int level = tableInfo.getLevel();
          let mut level: i32 = tableInfo.getLevel();
          // java IPapxTable papxTable = this.document.getPapxTable();
          let mut papxTable: Box<dyn IPapxTable> = self.document.getPapxTable();
          // java DefTableFixer fixer = new DefTableFixer(this.document);
          let mut fixer: DefTableFixer = DefTableFixer::new(self.document);
          // java int lastRowEnd = RANGEFactory.getEnd(((CellsItem)this.mCells.get(this.mCells.size() - 1)).rowRange);
          let mut lastRowEnd: i32 = RANGEFactory::getEnd((self.mCells.get((self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) - 1)).downcast_ref::<CellsItem>().unwrap().rowRange);
          // java int firstRowStart = RANGEFactory.getStart(((CellsItem)this.mCells.get(0)).rowRange);
          let mut firstRowStart: i32 = RANGEFactory::getStart((self.mCells.get(0)).downcast_ref::<CellsItem>().unwrap().rowRange);
          // java fixer.init(tableInfo.getRowByCP(lastRowEnd));
          fixer.init(tableInfo.getRowByCP(lastRowEnd));
          let mut i: i32 = (self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) - 1;
          // java for(int i = this.mCells.size() - 1; i >= 0 --i)
          while i >= 0 {
            // java RowInfo row = tableInfo.getRowByCP(RANGEFactory.getStart(((CellsItem)this.mCells.get(i)).rowRange));
            let mut row: RowInfo = tableInfo.getRowByCP(RANGEFactory::getStart((self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap().rowRange));
            // java PropertySet prop = row.getRowProperty();
            let mut prop: PropertySet = row.getRowProperty();
            // java Revision revision = (Revision)prop.getValue(363);
            let mut revision: Revision = (prop.getValue(363)).downcast_ref::<Revision>().unwrap();
            // java if (null != revision)
            if revision.is_some() {
              // java PropertySet finalRowProp = row.getFinalProperty();
              let mut finalRowProp: PropertySet = row.getFinalProperty();
              // java papxTable.seek(row.getEndCP() - 1).setProperty(finalRowProp);
              papxTable.seek(row.getEndCP() - 1).setProperty(finalRowProp);
            }
            // java deleteCount += this.removeCells(i);
            deleteCount = deleteCount + self.removeCells__int(i);
          }
          let mut i: i32 = (self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) - 1;
          // java for(int i = this.mCells.size() - 1; i >= 0 --i)
          while i >= 0 {
            // java fixer.addSurvivorRow(((CellsItem)this.mCells.get(i)).rowEndIndex);
            fixer.addSurvivorRow((self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap().rowEndIndex);
          }
          // java RowInfo beforeRow = tableInfo.getRowByCP(firstRowStart - 1);
          let mut beforeRow: RowInfo = tableInfo.getRowByCP(firstRowStart - 1);
          // java if (beforeRow != null)
          if beforeRow.is_some() {
            // java fixer.addBeforeRow(this.document.getTextRope().cp2fc(firstRowStart - 1));
            fixer.addBeforeRow(self.document.getTextRope().cp2fc(firstRowStart - 1));
          }
          // java else
          else {
            // java fixer.noBeforeRow();
            fixer.noBeforeRow();
          }
          // java if (this.mCells.size() > 0)
          if (self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 {
            // java tableInfo = this.document.getTableInfos().getTableInfoByLevel(tableStart, level);
            tableInfo = self.document.getTableInfos().getTableInfoByLevel(tableStart, level);
            // java int firstRowCP = this.document.getTextRope().fc2cp(((CellsItem)this.mCells.get(0)).rowEndIndex);
            let mut firstRowCP: i32 = self.document.getTextRope().fc2cp((self.mCells.get(0)).downcast_ref::<CellsItem>().unwrap().rowEndIndex);
            // java int firstRow = tableInfo.getRowByCP(firstRowCP).getIndex();
            let mut firstRow: i32 = tableInfo.getRowByCP(firstRowCP).getIndex();
            let mut i: i32 = (self.mCells.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) - 1;
            // java for(int i = this.mCells.size() - 1; i >= 0 --i)
            while i >= 0 {
              // java ((CellsItem)this.mCells.get(i)).rowRange = tableInfo.getRowByIndex(firstRow + i).getRange();
              (self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap().rowRange = tableInfo.getRowByIndex(firstRow + i).getRange();
            }
            // java this.removeRedundantRows(this.mCells, (MutablePropertySet[])null, false);
            self.removeRedundantRows(&mut self.mCells, None, false);
          }
          // java tableInfo = this.document.getTableInfos().getTableInfoByLevel(tableStart, level);
          tableInfo = self.document.getTableInfos().getTableInfoByLevel(tableStart, level);
          // java if (tableInfo != null)
          if tableInfo.is_some() {
            // java Tools.fixTableWidth(this.document, tableInfo, false);
            Tools::fixTableWidth(self.document, tableInfo, false);
            // java if (tableInfo.isTableFit())
            if tableInfo.isTableFit() {
              // java Tools.fixColumnWidths(this.document, tableInfo);
              Tools::fixColumnWidths(self.document, tableInfo);
              // java this.document.getTableInfos().doGridAutoResize();
              self.document.getTableInfos().doGridAutoResize();
            }
          }
            // java this.getTextDocument().commit("remove column");
            self.super_.getTextDocument().commit("remove column");
      // java return deleteCount;
      return deleteCount;
    }

    // java protected int removeRedundantRows(ArrayList<CellsItem> cells, MutablePropertySet[] props, boolean deleteAllMergedRow)
    pub fn removeRedundantRows(&mut self, cells: &mut Vec<CellsItem>, props: Option<&Vec<MutablePropertySet>>, deleteAllMergedRow: bool) -> i32 {
      // java boolean really = props == null;
      let mut really: bool = props.is_none();
      // java IPapxTable papxTable = this.document.getPapxTable();
      let mut papxTable: Box<dyn IPapxTable> = self.document.getPapxTable();
      // java boolean fixing = false;
      let mut fixing: bool = false;
      // java int deletedRows = 0;
      let mut deletedRows: i32 = 0;
      // java DefTable lastSurvivor = null;
      let mut lastSurvivor: DefTable = DefTable::default();
      // java int size = cells.size();
      let mut size: i32 = (cells.len() as i32);
      // java int lastRowEnd = RANGEFactory.getEnd(((CellsItem)cells.get(size - 1)).rowRange) - 1;
      let mut lastRowEnd: i32 = RANGEFactory::getEnd((cells.get(size - 1)).downcast_ref::<CellsItem>().unwrap().rowRange) - 1;
      // java TableInfo tableInfo = this.document.getTableInfos().getTableInfoByRange(lastRowEnd, lastRowEnd);
      let mut tableInfo: TableInfo = self.document.getTableInfos().getTableInfoByRange(lastRowEnd, lastRowEnd);
      // java RowInfo rowAfter = tableInfo.getRowByCP(lastRowEnd + 1);
      let mut rowAfter: RowInfo = tableInfo.getRowByCP(lastRowEnd + 1);
      // java if (rowAfter != null)
      if rowAfter.is_some() {
        // java lastSurvivor = (DefTable)rowAfter.getFinalProperty().getValue(306);
        lastSurvivor = (rowAfter.getFinalProperty().getValue(306)).downcast_ref::<DefTable>().unwrap();
      }
      let mut i: i32 = size - 1;
      // java for(int i = size - 1; i >= 0 --i)
      while i >= 0 {
        // java MutablePropertySet prop;
        let mut prop: MutablePropertySet = MutablePropertySet::default();
        // java if (really)
        if really {
          // java PropertySet oldProp = ((CellsItem)cells.get(i)).getFinalRowProperty();
          let mut oldProp: PropertySet = (cells.get(i)).downcast_ref::<CellsItem>().unwrap().getFinalRowProperty();
          // java DefTable oldDefTable = (DefTable)oldProp.getObject(306);
          let mut oldDefTable: DefTable = (oldProp.getObject(306)).downcast_ref::<DefTable>().unwrap();
          // java prop = new MutablePropertySet(oldProp);
          prop = MutablePropertySet::new(oldProp);
          // java prop.setValue(306, DefTableBuilder.obtain(oldDefTable));
          prop.setValue(306, DefTableBuilder::obtain__deftable(&oldDefTable));
        }
        // java else
        else {
          // java prop = props[i];
          prop = props.as_mut().unwrap()[(i) as usize];
        }
        // java DefTableBuilder defTable = (DefTableBuilder)prop.getObject(306);
        let mut defTable: DefTableBuilder = (prop.getObject(306)).downcast_ref::<DefTableBuilder>().unwrap();
        // java int notVertMerged = 0;
        let mut notVertMerged: i32 = 0;
        // java int noneMerged = 0;
        let mut noneMerged: i32 = 0;
        let mut j: i32 = defTable.getCellCount() - 1;
        // java for(int j = defTable.getCellCount() - 1; j >= 0 --j)
        while j >= 0 {
          // java TableCellBuilder tc = defTable.getTableCellBuilder(j);
          let mut tc: TableCellBuilder = defTable.getTableCellBuilder(j);
          // java int vMerged = tc.getVertMerge();
          let mut vMerged: i32 = tc.getVertMerge();
          // java if (vMerged == 0)
          if vMerged == 0 {
            // java ++noneMerged;
            { let _tmp = noneMerged; noneMerged += 1; _tmp };
          }
          // java if (vMerged != 1)
          if vMerged != 1 {
            // java ++notVertMerged;
            { let _tmp = notVertMerged; notVertMerged += 1; _tmp };
          }
          // java if (fixing && really && vMerged == 3 && null == TableTool.getNextMergeCell(defTable, j, lastSurvivor))
          if fixing && really && vMerged == 3 && TableTool::getNextMergeCell__deftable_int_deftable(&defTable, j, Some(&lastSurvivor)).is_none() {
            // java tc.setVertMerge(0);
            tc.setVertMerge(0);
            // java ++noneMerged;
            { let _tmp = noneMerged; noneMerged += 1; _tmp };
          }
        }
        // java if (notVertMerged == 0 && deleteAllMergedRow)
        if notVertMerged == 0 && deleteAllMergedRow {
          // java if (really)
          if really {
            // java long rowRange = ((CellsItem)cells.get(i)).rowRange;
            let mut rowRange: i64 = (cells.get(i)).downcast_ref::<CellsItem>().unwrap().rowRange;
            // java this.removeRangeDisableRevision(RANGEFactory.getStart(rowRange), RANGEFactory.getEnd(rowRange));
            self.removeRangeDisableRevision(RANGEFactory::getStart(rowRange), RANGEFactory::getEnd(rowRange));
            // java cells.remove(i);
            cells.remove(i);
          }
          // java ++deletedRows;
          { let _tmp = deletedRows; deletedRows += 1; _tmp };
          // java fixing = true;
          fixing = true;
        }
        // java else
        else {
          // java fixing = false;
          fixing = false;
          // java if (noneMerged == 0 && really && !prop.contains(305))
          if noneMerged == 0 && really && !prop.contains(305) {
            // java prop.setValue(305, UnitUtil.PT2TWIPS(15.6F));
            prop.setValue(305, UnitUtil::PT2TWIPS(15.6));
          }
          // java if (really)
          if really {
            // java papxTable.seek(RANGEFactory.getEnd(((CellsItem)cells.get(i)).rowRange) - 1).setProperty(prop.freeze());
            papxTable.seek(RANGEFactory::getEnd((cells.get(i)).downcast_ref::<CellsItem>().unwrap().rowRange) - 1).setProperty(prop.freeze());
          }
          // java lastSurvivor = defTable;
          lastSurvivor = defTable;
        }
      }
      // java return deletedRows;
      return deletedRows;
    }

    // java private int removeCells(int i)
    pub fn removeCells__int(&mut self, i: i32) -> i32 {
      // java CellsItem cellsItem = (CellsItem)this.mCells.get(i);
      let mut cellsItem: CellsItem = (self.mCells.get(i)).downcast_ref::<CellsItem>().unwrap();
      // java if (-1 == cellsItem.startCellIndex)
      if -1 == cellsItem.startCellIndex {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java PropertySet oldProp = cellsItem.getRowProperty();
        let mut oldProp: PropertySet = cellsItem.getRowProperty();
        // java DefTable oldDefTable = (DefTable)oldProp.getValue(306);
        let mut oldDefTable: DefTable = (oldProp.getValue(306)).downcast_ref::<DefTable>().unwrap();
        // java if (0 == cellsItem.startCellIndex && cellsItem.endCellIndex == oldDefTable.getCellCount() - 1)
        if 0 == cellsItem.startCellIndex && cellsItem.endCellIndex == oldDefTable.getCellCount() - 1 {
          // java this.mCells.remove(i);
          self.mCells.remove(i);
          // java return this.removeRangeDisableRevision(RANGEFactory.getStart(cellsItem.rowRange), RANGEFactory.getEnd(cellsItem.rowRange));
          return self.removeRangeDisableRevision(RANGEFactory::getStart(cellsItem.rowRange), RANGEFactory::getEnd(cellsItem.rowRange));
        }
        // java else
        else {
          // java MutablePropertySet propertySet = new MutablePropertySet(oldProp);
          let mut propertySet: MutablePropertySet = MutablePropertySet::new(oldProp);
          // java this.removeCells(propertySet, cellsItem.startCellIndex, cellsItem.endCellIndex);
          self.removeCells__mutablepropertyset_int_int(&mut propertySet, cellsItem.startCellIndex, cellsItem.endCellIndex);
          // java int rowEndCP = RANGEFactory.getEnd(cellsItem.rowRange);
          let mut rowEndCP: i32 = RANGEFactory::getEnd(cellsItem.rowRange);
          // java this.document.getPapxTable().seek(rowEndCP - 1).setProperty(propertySet.freeze());
          self.document.getPapxTable().seek(rowEndCP - 1).setProperty(propertySet.freeze());
          // java return this.removeRangeDisableRevision(RANGEFactory.getStart(cellsItem.cellsRange), RANGEFactory.getEnd(cellsItem.cellsRange));
          return self.removeRangeDisableRevision(RANGEFactory::getStart(cellsItem.cellsRange), RANGEFactory::getEnd(cellsItem.cellsRange));
        }
      }
    }

    // java protected void removeCells(MutablePropertySet propertySet, int startCellIndex, int endCellIndex)
    pub fn removeCells__mutablepropertyset_int_int(&mut self, propertySet: &mut MutablePropertySet, startCellIndex: i32, endCellIndex: i32) {
      // java DefTable oldDefTable = (DefTable)propertySet.getValue(306);
      let mut oldDefTable: DefTable = (propertySet.getValue(306)).downcast_ref::<DefTable>().unwrap();
      // java DefTable newDefTable = this.fixDefTable(oldDefTable, startCellIndex, endCellIndex);
      let mut newDefTable: DefTable = self.fixDefTable(&oldDefTable, startCellIndex, endCellIndex);
      // java propertySet.setValue(306, newDefTable);
      propertySet.setValue(306, &newDefTable);
      // java TableDefSHD oldDefSHD = (TableDefSHD)propertySet.getObject(308, (Object)null);
      let mut oldDefSHD: TableDefSHD = (propertySet.getObject__int_object(308, Default::default())).downcast_ref::<TableDefSHD>().unwrap();
      // java if (null != oldDefSHD)
      if oldDefSHD.is_some() {
        // java TableDefSHD newDefSHD = this.fixDefSHD(oldDefSHD, startCellIndex, endCellIndex, oldDefTable.getCellCount());
        let mut newDefSHD: TableDefSHD = self.fixDefSHD(&oldDefSHD, startCellIndex, endCellIndex, oldDefTable.getCellCount());
        // java propertySet.setValue(308, newDefSHD);
        propertySet.setValue(308, &newDefSHD);
      }
      // java TableSetBRC[] tableSetBRCs = (TableSetBRC[])propertySet.getValue(311);
      let mut tableSetBRCs: Vec<TableSetBRC> = todo!("cast → Vec<TableSetBRC>");
      // java if (null != tableSetBRCs)
      if tableSetBRCs.is_some() {
        // java TableSetBRC[] newBrcs = this.fixTableBrcs(tableSetBRCs, startCellIndex, endCellIndex);
        let mut newBrcs: Vec<TableSetBRC> = self.fixTableBrcs(&tableSetBRCs, startCellIndex, endCellIndex);
        // java propertySet.setValue(311, newBrcs);
        propertySet.setValue(311, &newBrcs);
      }
      // java BrcCvFactory cvFactory = new BrcCvFactory(propertySet.get());
      let mut cvFactory: BrcCvFactory = BrcCvFactory::new(propertySet.get());
      // java cvFactory.remove(startCellIndex, endCellIndex + 1);
      cvFactory.remove(startCellIndex, endCellIndex + 1);
      // java cvFactory.commit(propertySet);
      cvFactory.commit(propertySet);
    }

    // java private DefTable fixDefTable(DefTable oldDefTable, int startCellIndex, int endCellIndex)
    pub fn fixDefTable(&mut self, oldDefTable: &DefTable, startCellIndex: i32, endCellIndex: i32) -> DefTable {
      // java DefTableBuilder newDefTable = DefTableBuilder.obtain();
      let mut newDefTable: DefTableBuilder = DefTableBuilder::obtain();
      // java int oldCellsCount = oldDefTable.getCellCount();
      let mut oldCellsCount: i32 = oldDefTable.getCellCount();
      // java int newCellsCount = oldCellsCount - (endCellIndex - startCellIndex + 1);
      let mut newCellsCount: i32 = oldCellsCount - endCellIndex - startCellIndex + 1;
      // java newDefTable.setCellCount(newCellsCount);
      newDefTable.setCellCount(newCellsCount);
      // java newDefTable.setItMax(newCellsCount + 1);
      newDefTable.setItMax(newCellsCount + 1);
      // java int dist = oldDefTable.getRgdxa(endCellIndex + 1) - oldDefTable.getRgdxa(startCellIndex);
      let mut dist: i32 = oldDefTable.getRgdxa(endCellIndex + 1) - oldDefTable.getRgdxa(startCellIndex);
      // java int oldDefTablePos = 0;
      let mut oldDefTablePos: i32 = 0;
      // java int newDefTablePos;
      let mut newDefTablePos: i32;
      newDefTablePos = 0;
      // java for(newDefTablePos = 0 oldDefTablePos < startCellIndex ++oldDefTablePos)
      while oldDefTablePos < startCellIndex {
        // java newDefTable.setTableCellBuilder(newDefTablePos++, TableCellBuilder.obtain(oldDefTable.getTableCell(oldDefTablePos)));
        newDefTable.setTableCellBuilder({ let _tmp = newDefTablePos; newDefTablePos += 1; _tmp }, TableCellBuilder::obtain__tablecell(&oldDefTable.getTableCell(oldDefTablePos)));
      }
      let mut var12: i32 = endCellIndex + 1;
      // java for(int var12 = endCellIndex + 1; var12 < oldCellsCount ++var12)
      while var12 < oldCellsCount {
        // java newDefTable.setTableCellBuilder(newDefTablePos++, TableCellBuilder.obtain(oldDefTable.getTableCell(var12)));
        newDefTable.setTableCellBuilder({ let _tmp = newDefTablePos; newDefTablePos += 1; _tmp }, TableCellBuilder::obtain__tablecell(&oldDefTable.getTableCell(var12)));
      }
      // java if (0 != startCellIndex)
      if 0 != startCellIndex {
        // java TableCellBuilder tc = newDefTable.getTableCellBuilder(startCellIndex - 1);
        let mut tc: TableCellBuilder = newDefTable.getTableCellBuilder(startCellIndex - 1);
        // java if (3 == tc.getHoriMerge())
        if 3 == tc.getHoriMerge() {
          // java if (startCellIndex == newCellsCount)
          if startCellIndex == newCellsCount {
            // java tc.setVertMerge(0);
            tc.setVertMerge(0);
          }
          // java else
          else {
            // java TableCell tc2 = newDefTable.getTableCell(startCellIndex);
            let mut tc2: TableCell = newDefTable.getTableCell(startCellIndex);
            // java if (2 != tc2.getHoriMerge())
            if 2 != tc2.getHoriMerge() {
              // java tc.setVertMerge(0);
              tc.setVertMerge(0);
            }
          }
        }
      }
      // java if (startCellIndex != newCellsCount)
      if startCellIndex != newCellsCount {
        // java TableCellBuilder tc = newDefTable.getTableCellBuilder(startCellIndex);
        let mut tc: TableCellBuilder = newDefTable.getTableCellBuilder(startCellIndex);
        // java if (2 == tc.getHoriMerge())
        if 2 == tc.getHoriMerge() {
          // java if (0 == startCellIndex)
          if 0 == startCellIndex {
            // java tc.setVertMerge(0);
            tc.setVertMerge(0);
          }
          // java else
          else {
            // java TableCell tc2 = newDefTable.getTableCell(startCellIndex - 1);
            let mut tc2: TableCell = newDefTable.getTableCell(startCellIndex - 1);
            // java if (0 == tc2.getHoriMerge())
            if 0 == tc2.getHoriMerge() {
              // java tc.setVertMerge(0);
              tc.setVertMerge(0);
            }
          }
        }
      }
      // java oldDefTablePos = 0;
      oldDefTablePos = 0;
      newDefTablePos = 0;
      // java for(newDefTablePos = 0 oldDefTablePos < startCellIndex ++oldDefTablePos)
      while oldDefTablePos < startCellIndex {
        // java newDefTable.setRgdxa(newDefTablePos++, oldDefTable.getRgdxa(oldDefTablePos));
        newDefTable.setRgdxa({ let _tmp = newDefTablePos; newDefTablePos += 1; _tmp }, oldDefTable.getRgdxa(oldDefTablePos));
      }
      let mut var14: i32 = endCellIndex + 1;
      // java for(int var14 = endCellIndex + 1; var14 <= oldCellsCount ++var14)
      while var14 <= oldCellsCount {
        // java newDefTable.setRgdxa(newDefTablePos++, oldDefTable.getRgdxa(var14) - dist);
        newDefTable.setRgdxa({ let _tmp = newDefTablePos; newDefTablePos += 1; _tmp }, oldDefTable.getRgdxa(var14) - dist);
      }
      // java return newDefTable;
      return newDefTable;
    }

    // java private TableDefSHD fixDefSHD(TableDefSHD oldDefSHD, int startCellIndex, int endCellIndex, int oldCellsCount)
    pub fn fixDefSHD(&mut self, oldDefSHD: &TableDefSHD, startCellIndex: i32, endCellIndex: i32, oldCellsCount: i32) -> TableDefSHD {
      // java TableDefSHDBuilder newDefSHD = new TableDefSHDBuilder(oldCellsCount - (endCellIndex - startCellIndex + 1));
      let mut newDefSHD: TableDefSHDBuilder = TableDefSHDBuilder::new(oldCellsCount - endCellIndex - startCellIndex + 1);
      // java int oldPos = 0;
      let mut oldPos: i32 = 0;
      // java int newPos;
      let mut newPos: i32;
      newPos = 0;
      // java for(newPos = 0 oldPos < startCellIndex ++newPos)
      while oldPos < startCellIndex {
        // java newDefSHD.setCellSHD(newPos, oldDefSHD.getCellSHD(oldPos));
        newDefSHD.setCellSHD(newPos, oldDefSHD.getCellSHD(oldPos));
        // java ++oldPos;
        { let _tmp = oldPos; oldPos += 1; _tmp };
      }
      // java oldPos = endCellIndex + 1;
      oldPos = endCellIndex + 1;
      let mut size: i32 = oldDefSHD.getCount();
      // java for(int size = oldDefSHD.getCount(); oldPos < size ++newPos)
      while oldPos < size {
        // java newDefSHD.setCellSHD(newPos, oldDefSHD.getCellSHD(oldPos));
        newDefSHD.setCellSHD(newPos, oldDefSHD.getCellSHD(oldPos));
        // java ++oldPos;
        { let _tmp = oldPos; oldPos += 1; _tmp };
      }
      // java return newDefSHD;
      return newDefSHD;
    }

    // java private TableSetBRC[] fixTableBrcs(TableSetBRC[] tableSetBRCs, int startCellIndex, int endCellIndex)
    pub fn fixTableBrcs(&mut self, tableSetBRCs: &Vec<TableSetBRC>, startCellIndex: i32, endCellIndex: i32) -> Option<Vec<TableSetBRC>> {
      // java ArrayList<TableSetBRC> brcList = new ArrayList();
      let mut brcList: Vec<TableSetBRC> = Vec::new();
      let mut i: i32 = 0;
      // java for(int i = 0; i < tableSetBRCs.length; ++i)
      while i < (tableSetBRCs.len() as i32) {
        // java TableSetBRC oldSetBRC = tableSetBRCs[i];
        let mut oldSetBRC: TableSetBRC = tableSetBRCs[(i) as usize];
        // java long range = RANGEFactory.getRANGE(oldSetBRC.getFirst(), oldSetBRC.getLast());
        let mut range: i64 = RANGEFactory::getRANGE(oldSetBRC.getFirst(), oldSetBRC.getLast());
        // java range = RANGEFactory.removeText(range, startCellIndex, endCellIndex + 1);
        range = RANGEFactory::removeText(range, startCellIndex, endCellIndex + 1);
        // java if (!RANGEFactory.isEmpty(range))
        if !RANGEFactory::isEmpty(range) {
          // java TableSetBRC newSetBRC = new TableSetBRC();
          let mut newSetBRC: TableSetBRC = TableSetBRC::new();
          // java newSetBRC.setBRC(oldSetBRC.getBRC());
          newSetBRC.setBRC(oldSetBRC.getBRC());
          // java newSetBRC.setBordersToApply(oldSetBRC.getBordersToApply());
          newSetBRC.setBordersToApply(oldSetBRC.getBordersToApply());
          // java newSetBRC.setRange(RANGEFactory.getStart(range), RANGEFactory.getEnd(range));
          newSetBRC.setRange(RANGEFactory::getStart(range), RANGEFactory::getEnd(range));
          // java brcList.add(newSetBRC);
          brcList.add(newSetBRC);
        }
      }
      // java if (brcList.size() == 0)
      if (brcList.len() as i32) == 0 {
        // java return null;
        return None;
      }
      // java else
      else {
        // java TableSetBRC[] newBrcs = new TableSetBRC[brcList.size()];
        let mut newBrcs: Vec<TableSetBRC> = vec![Default::default(); (brcList.len() as i32) as usize];
        // java brcList.toArray(newBrcs);
        brcList.toArray(newBrcs);
        // java return newBrcs;
        return newBrcs;
      }
    }

    // java protected int removeRangeDisableRevision(int start, int end)
    pub fn removeRangeDisableRevision(&self, start: i32, end: i32) -> i32 {
      // java boolean originEnabled = this.document.getTextStream().getRevisionFileter();
      let mut originEnabled: bool = self.document.getTextStream().getRevisionFileter();
      // java this.document.getTextStream().disableRevisionFileter();
      self.document.getTextStream().disableRevisionFileter();
      // java int removed = this.document.getRange(start, end).removeText();
      let mut removed: i32 = self.document.getRange(start, end).removeText();
      // java if (originEnabled)
      if originEnabled {
        // java this.document.getTextStream().enableRevisionFileter();
        self.document.getTextStream().enableRevisionFileter();
      }
      // java return removed;
      return removed;
    }

    // java protected void beforeRemoveRowInRevision(RowRevisionFixer fixer)
    pub fn beforeRemoveRowInRevision(&mut self, fixer: &RowRevisionFixer) {
    }

}

impl std::ops::Deref for KTableRangeBase {
    type Target = KRangeBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KTableRangeBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static enum RevisionType
impl RevisionType {
}

// java public static class TableRangeContext
impl TableRangeContext {
    // java public  TableRangeContext(SelectionType type)
    pub fn TableRangeContext(r#type: &SelectionType) -> Self {
      // java this.selType = type;
      Self { selType: r#type, rootLevel: Default::default(), startInTable: Default::default(), endInTable: Default::default(), hasTable: Default::default() }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java return "Context:" + this.selType + "," + this.hasTable + "," + this.rootLevel;
      return "Context:" + self.selType + "," + self.hasTable + "," + self.rootLevel;
    }

}
