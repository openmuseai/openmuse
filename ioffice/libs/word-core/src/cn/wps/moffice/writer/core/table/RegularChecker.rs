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
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.table.CellsItem
use crate::cn::wps::moffice::writer::core::table::CellsItem::*;

// java public class RegularChecker
#[derive(Default)]
pub struct RegularChecker {
    // java private Document document;
    document: Option<Box<dyn Document>>,
    // java private TableInfo tableInfo;
    tableInfo: Option<Box<dyn TableInfo>>,
    // java private int startRow;
    startRow: i32,
    // java private int endRow;
    endRow: i32,
    // java private boolean[][] marked;
    marked: Option<Vec<Vec<bool>>>,
    // java private int[] starts;
    starts: Option<Vec<i32>>,
    // java private int[] ends;
    ends: Option<Vec<i32>>,
}

// java public class RegularChecker
impl RegularChecker {
    // java public  RegularChecker(Document document, TableInfo tableInfo, ArrayList<CellsItem> cells)
    pub fn RegularChecker(document: &dyn Document, tableInfo: &dyn TableInfo, cells: &Vec<CellsItem>) -> Self {
      // java this.document = document;
      let mut this = Self { document: document, tableInfo: tableInfo, startRow: tableInfo.getRowByCP(RANGEFactory::getStart((cells.get(0)).downcast_ref::<CellsItem>().unwrap().rowRange)).getIndex(), endRow: self.startRow + (cells.len() as i32) - 1, marked: vec![Default::default(); (tableSize) as usize], starts: vec![Default::default(); (tableSize) as usize], ends: vec![Default::default(); (tableSize) as usize] };
      // java this.tableInfo = tableInfo;
      // java int tableSize = tableInfo.size();
      let mut tableSize: i32 = tableInfo.size();
      // java this.marked = new boolean[tableSize][];
      // java this.starts = new int[tableSize];
      // java this.ends = new int[tableSize];
      // java this.startRow = tableInfo.getRowByCP(RANGEFactory.getStart(((CellsItem)cells.get(0)).rowRange)).getIndex();
      // java this.endRow = this.startRow + cells.size() - 1;
      // java CellsItem cellsItem = (CellsItem)cells.get(i); this.starts[this.startRow + i] = cellsItem.startCellIndex; this.ends[this.startRow + i] = cellsItem.endCellIndex;
      this.starts.as_ref().unwrap()[(this.startRow + i) as usize] = cellsItem.startCellIndex;
      this
    }

    // java public boolean check(int row, CellInfo mergedCell)
    pub fn check(&mut self, row: i32, mergedCell: &CellInfo) -> bool {
      // java if (this.mark(row, mergedCell))
      if self.mark(row, &mergedCell) {
        // java return true;
        return true;
      }
      // java else
      else {
        // java CellInfo cellInfo = mergedCell.getNextVertMergeCell();
        let mut cellInfo: CellInfo = mergedCell.getNextVertMergeCell();
        let mut i: i32 = row + 1;
        // java for(int i = row + 1; cellInfo != null; ++i)
        while cellInfo.is_some() {
          // java if (!this.contains(i, cellInfo))
          if !self.contains(i, &cellInfo) {
            // java return false;
            return false;
          }
          // java this.mark(i, cellInfo);
          self.mark(i, &cellInfo);
          // java cellInfo = cellInfo.getNextVertMergeCell();
          cellInfo = cellInfo.getNextVertMergeCell();
        }
        // java cellInfo = mergedCell.getPrevVertMergeCell();
        cellInfo = mergedCell.getPrevVertMergeCell();
        let mut i: i32 = row - 1;
        // java for(int i = row - 1; cellInfo != null --i)
        while cellInfo.is_some() {
          // java if (!this.contains(i, cellInfo))
          if !self.contains(i, &cellInfo) {
            // java return false;
            return false;
          }
          // java this.mark(i, cellInfo);
          self.mark(i, &cellInfo);
          // java cellInfo = cellInfo.getPrevVertMergeCell();
          cellInfo = cellInfo.getPrevVertMergeCell();
        }
        // java return true;
        return true;
      }
    }

    // java public void add(int row, CellInfo mergedCell)
    pub fn add(&mut self, row: i32, mergedCell: &CellInfo) {
      // java if (!this.mark(row, mergedCell))
      if !self.mark(row, &mergedCell) {
        // java CellInfo cellInfo = mergedCell.getNextVertMergeCell();
        let mut cellInfo: CellInfo = mergedCell.getNextVertMergeCell();
        let mut i: i32 = row + 1;
        // java for(int i = row + 1; cellInfo != null; ++i)
        while cellInfo.is_some() {
          // java this.addCell(i, cellInfo);
          self.addCell(i, &cellInfo);
          // java cellInfo = cellInfo.getNextVertMergeCell();
          cellInfo = cellInfo.getNextVertMergeCell();
        }
        // java cellInfo = mergedCell.getPrevVertMergeCell();
        cellInfo = mergedCell.getPrevVertMergeCell();
        let mut i: i32 = row - 1;
        // java for(int i = row - 1; cellInfo != null --i)
        while cellInfo.is_some() {
          // java this.addCell(i, cellInfo);
          self.addCell(i, &cellInfo);
          // java cellInfo = cellInfo.getPrevVertMergeCell();
          cellInfo = cellInfo.getPrevVertMergeCell();
        }
      }
    }

    // java public ArrayList<CellsItem> getCells()
    pub fn getCells(&self) -> Vec<CellsItem> {
      // java ArrayList<CellsItem> cells = new ArrayList(this.endRow - this.startRow + 1);
      let mut cells: Vec<CellsItem> = Vec::new();
      let mut i: i32 = self.startRow;
      // java for(int i = this.startRow; i <= this.endRow; ++i)
      while i <= self.endRow {
        // java RowInfo rowInfo = this.tableInfo.getRowByIndex(i);
        let mut rowInfo: RowInfo = self.tableInfo.getRowByIndex(i);
        // java CellsItem cellsItem = new CellsItem(this.document, rowInfo.getRange());
        let mut cellsItem: CellsItem = CellsItem::new(self.document, rowInfo.getRange());
        // java cellsItem.startCellIndex = this.starts[i];
        cellsItem.startCellIndex = self.starts.as_ref().unwrap()[(i) as usize];
        // java cellsItem.endCellIndex = this.ends[i];
        cellsItem.endCellIndex = self.ends.as_ref().unwrap()[(i) as usize];
        // java if (cellsItem.startCellIndex != -1)
        if cellsItem.startCellIndex != -1 {
          // java cellsItem.cellsRange = RANGEFactory.getRANGE(rowInfo.getCellByIndex(cellsItem.startCellIndex).getStartCP(), rowInfo.getCellByIndex(cellsItem.endCellIndex).getEndCP());
          cellsItem.cellsRange = RANGEFactory::getRANGE(rowInfo.getCellByIndex(cellsItem.startCellIndex).getStartCP(), rowInfo.getCellByIndex(cellsItem.endCellIndex).getEndCP());
        }
        // java cells.add(cellsItem);
        cells.add(cellsItem);
      }
      // java return cells;
      return cells;
    }

    // java private boolean mark(int row, CellInfo mergedCell)
    pub fn mark(&mut self, row: i32, mergedCell: &CellInfo) -> bool {
      // java if (this.marked[row] == null)
      if self.marked.as_mut().unwrap()[(row) as usize].is_none() {
        // java this.marked[row] = new boolean[mergedCell.getParent().size()];
        self.marked.as_mut().unwrap()[(row) as usize] = vec![Default::default(); (mergedCell.getParent().len() as i32) as usize];
      }
      // java int col = mergedCell.getIndex();
      let mut col: i32 = mergedCell.getIndex();
      // java boolean b = this.marked[row][col];
      let mut b: bool = self.marked.as_mut().unwrap()[(row) as usize][(col) as usize];
      // java this.marked[row][col] = true;
      self.marked.as_mut().unwrap()[(row) as usize][(col) as usize] = true;
      // java return b;
      return b;
    }

    // java private boolean contains(int row, CellInfo mergedCell)
    pub fn contains(&self, row: i32, mergedCell: &CellInfo) -> bool {
      // java if (row >= this.startRow && row <= this.endRow && this.starts[row] != -1)
      if row >= self.startRow && row <= self.endRow && self.starts.as_ref().unwrap()[(row) as usize] != -1 {
        // java int cellIndex = mergedCell.getIndex();
        let mut cellIndex: i32 = mergedCell.getIndex();
        // java return cellIndex >= this.starts[row] && cellIndex <= this.ends[row];
        return cellIndex >= self.starts.as_ref().unwrap()[(row) as usize] && cellIndex <= self.ends.as_ref().unwrap()[(row) as usize];
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java private void addCell(int row, CellInfo mergedCell)
    pub fn addCell(&mut self, row: i32, mergedCell: &CellInfo) {
      // java this.mark(row, mergedCell);
      self.mark(row, &mergedCell);
      // java int cellIndex = mergedCell.getIndex();
      let mut cellIndex: i32 = mergedCell.getIndex();
      // java if (row < this.startRow)
      if row < self.startRow {
        // java --this.startRow;
        { let _tmp = self.startRow; self.startRow -= 1; _tmp };
        // java this.starts[row] = this.ends[row] = cellIndex;
        self.starts.as_mut().unwrap()[(row) as usize] = { self.ends.as_mut().unwrap()[(row) as usize] = cellIndex; self.ends.as_mut().unwrap()[(row) as usize] };
      }
      // java else if (row > this.endRow)
      else if row > self.endRow {
        // java ++this.endRow;
        { let _tmp = self.endRow; self.endRow += 1; _tmp };
        // java this.starts[row] = this.ends[row] = cellIndex;
        self.starts.as_mut().unwrap()[(row) as usize] = { self.ends.as_mut().unwrap()[(row) as usize] = cellIndex; self.ends.as_mut().unwrap()[(row) as usize] };
      }
      // java else if (this.starts[row] == -1)
      else if self.starts.as_mut().unwrap()[(row) as usize] == -1 {
        // java this.starts[row] = this.ends[row] = cellIndex;
        self.starts.as_mut().unwrap()[(row) as usize] = { self.ends.as_mut().unwrap()[(row) as usize] = cellIndex; self.ends.as_mut().unwrap()[(row) as usize] };
      }
      // java else if (cellIndex < this.starts[row])
      else if cellIndex < self.starts.as_mut().unwrap()[(row) as usize] {
        // java RowInfo rowInfo = mergedCell.getParent();
        let mut rowInfo: RowInfo = mergedCell.getParent();
        // java int i = this.starts[row] - 1;
        let mut i: i32 = self.starts.as_mut().unwrap()[(row) as usize] - 1;
        self.starts.as_mut().unwrap()[(row) as usize] = cellIndex;
        // java for(this.starts[row] = cellIndex i > cellIndex --i)
        while i > cellIndex {
          // java CellInfo cellInfo = rowInfo.getCellByIndex(i);
          let mut cellInfo: CellInfo = rowInfo.getCellByIndex(i);
          // java if (cellInfo.isVertMergeCell())
          if cellInfo.isVertMergeCell() {
            // java this.add(row, cellInfo);
            self.add(row, &cellInfo);
          }
        }
      }
      // java else if (cellIndex > this.ends[row])
      else if cellIndex > self.ends.as_mut().unwrap()[(row) as usize] {
        // java RowInfo rowInfo = mergedCell.getParent();
        let mut rowInfo: RowInfo = mergedCell.getParent();
        // java int i = this.ends[row] + 1;
        let mut i: i32 = self.ends.as_mut().unwrap()[(row) as usize] + 1;
        self.ends.as_mut().unwrap()[(row) as usize] = cellIndex;
        // java for(this.ends[row] = cellIndex i < cellIndex ++i)
        while i < cellIndex {
          // java CellInfo cellInfo = rowInfo.getCellByIndex(i);
          let mut cellInfo: CellInfo = rowInfo.getCellByIndex(i);
          // java if (cellInfo.isVertMergeCell())
          if cellInfo.isVertMergeCell() {
            // java this.add(row, cellInfo);
            self.add(row, &cellInfo);
          }
        }
      }
    }

}
