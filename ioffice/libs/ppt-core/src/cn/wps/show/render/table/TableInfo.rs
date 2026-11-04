#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.drawing.app.table.KmoTable
// Java: import cn.wps.drawing.app.table.KmoTableCell
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;

pub struct KmoTable { _placeholder: () }
impl KmoTable {
    pub fn colSize(&self) -> i32 { 0 }
    pub fn rowSize(&self) -> i32 { 0 }
    pub fn getCell(&self, _row: i32, _col: i32) -> Option<&KmoTableCell> { None }
}

pub struct KmoTableCell { _placeholder: () }
pub struct CellProperties { _placeholder: () }
pub struct LineProperties { _placeholder: () }

impl KmoTableCell {
    pub fn cellProperties(&self) -> &CellProperties { &CellProperties { _placeholder: () } }
}
impl CellProperties {
    pub fn hasLeftLine(&self) -> bool { false }
    pub fn hasRightLine(&self) -> bool { false }
    pub fn hasTopLine(&self) -> bool { false }
    pub fn hasBottomLine(&self) -> bool { false }
    pub fn leftLine(&self) -> &LineProperties { &LineProperties { _placeholder: () } }
    pub fn rightLine(&self) -> &LineProperties { &LineProperties { _placeholder: () } }
    pub fn topLine(&self) -> &LineProperties { &LineProperties { _placeholder: () } }
    pub fn bottomLine(&self) -> &LineProperties { &LineProperties { _placeholder: () } }
}
impl LineProperties {
    pub fn width(&self) -> i32 { 0 }
}

struct Math;
impl Math {
    fn max(a: f32, b: f32) -> f32 { if a > b { a } else { b } }
}

// Java: public class TableInfo
#[derive(Clone)]
pub struct TableInfo {
    // Java field: float[] colWidth
    pub(crate) colWidth: Vec<f32>,
    // Java field: float[] rowHeight
    pub(crate) rowHeight: Vec<f32>,
    // Java field: float left
    pub(crate) left: f32,
    // Java field: float top
    pub(crate) top: f32,
    // Java field: float width
    pub(crate) width: f32,
    // Java field: float height
    pub(crate) height: f32,
    // Java field: int id
    pub(crate) id: i32,
}

// Java: public class TableInfo
impl TableInfo {
    // Java: TableInfo(KmoTable table, int shapeId)
    pub fn new(table: &KmoTable, shapeId: i32) -> Self {
      // Java: this.colWidth = new float[table.colSize()];
      let colWidth = vec![0.0f32; table.colSize() as usize];
      // Java: this.rowHeight = new float[table.rowSize()];
      let rowHeight = vec![0.0f32; table.rowSize() as usize];
      // Java: this.id = shapeId;
      TableInfo { colWidth, rowHeight, left: 0.0, top: 0.0, width: 0.0, height: 0.0, id: shapeId }
    }

    // Java: int getId()
    pub fn getId(&self) -> i32 {
      // Java: return this.id;
      self.id
    }

    // Java: int rowSize()
    pub fn rowSize(&self) -> i32 {
      // Java: return this.rowHeight.length;
      self.rowHeight.len() as i32
    }

    // Java: int colSize()
    pub fn colSize(&self) -> i32 {
      // Java: return this.colWidth.length;
      self.colWidth.len() as i32
    }

    // Java: float[] getColWidths()
    pub fn getColWidths(&self) -> &[f32] {
      // Java: return this.colWidth;
      &self.colWidth
    }

    // Java: float[] getRowHeights()
    pub fn getRowHeights(&self) -> &[f32] {
      // Java: return this.rowHeight;
      &self.rowHeight
    }

    // Java: float getSumColWidth()
    pub fn getSumColWidth(&self) -> f32 {
      // Java: return this.sumArray(this.colWidth);
      self.sumArray(&self.colWidth)
    }

    // Java: float getSumRowHeight()
    pub fn getSumRowHeight(&self) -> f32 {
      // Java: return this.sumArray(this.rowHeight);
      self.sumArray(&self.rowHeight)
    }

    // Java: float sumArray(float[] array)
    pub fn sumArray(&self, array: &[f32]) -> f32 {
      // Java: float sum = 0.0F;
      let mut sum = 0.0f32;
      // Java: int length = array.length;
      let length = array.len();
      // Java: for(int i = 0; i < length; ++i)
      for i in 0..length {
         // Java: sum += array[i];
         sum += array[i];
      }
      // Java: return sum;
      sum
    }

    // Java: float getLeft()
    pub fn getLeft(&self) -> f32 {
      // Java: return this.left;
      self.left
    }

    // Java: void setLeft(float left)
    pub fn setLeft(&mut self, left: f32) {
      // Java: this.left = left;
      self.left = left;
    }

    // Java: float getTop()
    pub fn getTop(&self) -> f32 {
      // Java: return this.top;
      self.top
    }

    // Java: void setTop(float top)
    pub fn setTop(&mut self, top: f32) {
      // Java: this.top = top;
      self.top = top;
    }

    // Java: float getWidth()
    pub fn getWidth(&self) -> f32 {
      // Java: return this.width;
      self.width
    }

    // Java: void setWidth(float width)
    pub fn setWidth(&mut self, width: f32) {
      // Java: this.width = width;
      self.width = width;
    }

    // Java: float getHeight()
    pub fn getHeight(&self) -> f32 {
      // Java: return this.height;
      self.height
    }

    // Java: void setHeight(float height)
    pub fn setHeight(&mut self, height: f32) {
      // Java: this.height = height;
      self.height = height;
    }

    // Java: boolean isSameRowCol(KmoShowShape shape)
    pub fn isSameRowCol(&self, shape: &KmoShowShape) -> bool {
      // Java: if (this.id != shape.id())
      if self.id != shape.dgShapeId() {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: KmoTable table = (KmoTable)shape.graphic();
         // Java: return this.colWidth.length == table.colSize() && this.rowHeight.length == table.rowSize();
         false
      }
    }

    // Java: float[] calAroundMaxBorderWidth(KmoTable table)
    pub fn calAroundMaxBorderWidth(&self, table: &KmoTable) -> Vec<f32> {
      // Java: int rowSize = this.rowSize();
      let rowSize = self.rowSize();
      // Java: int colSize = this.colSize();
      let colSize = self.colSize();
      // Java: float left = 0.0F;
      let mut left = 0.0f32;
      // Java: float top = 0.0F;
      let mut top = 0.0f32;
      // Java: float right = 0.0F;
      let mut right = 0.0f32;
      // Java: float bottom = 0.0F;
      let mut bottom = 0.0f32;
      // Java: for(int row = 0; row < rowSize; ++row)
      for row in 0..rowSize {
         // Java: for(int col = 0; col < colSize; ++col)
         for col in 0..colSize {
            // Java: KmoTableCell cell = table.getCell(row, col);
            let cell = table.getCell(row, col);
            if let Some(cell) = cell {
            // Java: if (col == 0)
            if col == 0 {
               // Java: if (cell.cellProperties().hasLeftLine())
               if cell.cellProperties().hasLeftLine() {
                  // Java: left = Math.max(left, (float)cell.cellProperties().leftLine().width());
                  left = Math::max(left, cell.cellProperties().leftLine().width() as f32);
               }
            }
            // Java: else if (col == colSize - 1 && cell.cellProperties().hasRightLine())
            else if col == colSize - 1 && cell.cellProperties().hasRightLine() {
               // Java: right = Math.max(right, (float)cell.cellProperties().rightLine().width());
               right = Math::max(right, cell.cellProperties().rightLine().width() as f32);
            }
            // Java: if (row == 0)
            if row == 0 {
               // Java: if (cell.cellProperties().hasTopLine())
               if cell.cellProperties().hasTopLine() {
                  // Java: top = Math.max(top, (float)cell.cellProperties().topLine().width());
                  top = Math::max(top, cell.cellProperties().topLine().width() as f32);
               }
            }
            // Java: else if (row == rowSize - 1 && cell.cellProperties().hasBottomLine())
            else if row == rowSize - 1 && cell.cellProperties().hasBottomLine() {
               // Java: bottom = Math.max(bottom, (float)cell.cellProperties().bottomLine().width());
               bottom = Math::max(bottom, cell.cellProperties().bottomLine().width() as f32);
            }
            }
         }
      }
      // Java: return new float[]{left, top, right, bottom};
      vec![left, top, right, bottom]
    }

}
