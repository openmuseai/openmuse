// Java: public class KmoRange
pub struct KmoRange {
    // Java field: int topRow
    pub(crate) topRow: i32,
    // Java field: int leftCol
    pub(crate) leftCol: i32,
    // Java field: int bottomRow
    pub(crate) bottomRow: i32,
    // Java field: int rightCol
    pub(crate) rightCol: i32,
}

// Java: public class KmoRange
impl KmoRange {
    // Java: KmoRange()
    pub fn new() -> Self {
      Self { topRow: 0, leftCol: 0, bottomRow: 0, rightCol: 0 }
    }

    // Java: KmoRange(int topRow, int leftCol, int bottomRow, int rightCol)
    pub fn new_from(topRow: i32, leftCol: i32, bottomRow: i32, rightCol: i32) -> Self {
      let mut inst = Self::new();
      inst.KmoRange_init(topRow, leftCol, bottomRow, rightCol);
      inst
    }

    fn KmoRange_init(&mut self, topRow: i32, leftCol: i32, bottomRow: i32, rightCol: i32) {
      // Java: this.copy(topRow, leftCol, bottomRow, rightCol);
      self.copy(topRow, leftCol, bottomRow, rightCol);
    }

    // Java: KmoRange copy(int topRow, int leftCol, int bottomRow, int rightCol)
    pub fn copy(&mut self, topRow: i32, leftCol: i32, bottomRow: i32, rightCol: i32) -> &Self {
      // Java: this.topRow = topRow;
      self.topRow = topRow;
      // Java: this.leftCol = leftCol;
      self.leftCol = leftCol;
      // Java: this.bottomRow = bottomRow;
      self.bottomRow = bottomRow;
      // Java: this.rightCol = rightCol;
      self.rightCol = rightCol;
      // Java: return this;
      self
    }

    // Java: int area()
    pub fn area(&self) -> i32 {
      // Java: return (this.bottomRow - this.topRow + 1) * (this.rightCol - this.leftCol + 1);
      (self.bottomRow - self.topRow + 1) * (self.rightCol - self.leftCol + 1)
    }

    // Java: int width()
    pub fn width(&self) -> i32 {
      // Java: return this.rightCol - this.leftCol + 1;
      self.rightCol - self.leftCol + 1
    }

    // Java: int height()
    pub fn height(&self) -> i32 {
      // Java: return this.bottomRow - this.topRow + 1;
      self.bottomRow - self.topRow + 1
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "(row1:" + this.topRow + ", col1:" + this.leftCol + ") (row2:" + this.bottomRow + ", col2:" + this.rightCol + ")";
      format!("(row1:{}, col1:{}) (row2:{}, col2:{})", self.topRow, self.leftCol, self.bottomRow, self.rightCol)
    }

    // Java: boolean isSingleCell()
    pub fn isSingleCell(&self) -> bool {
      // Java: return this.topRow == this.bottomRow && this.leftCol == this.rightCol;
      self.topRow == self.bottomRow && self.leftCol == self.rightCol
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return this.topRow + this.leftCol + this.bottomRow + this.rightCol;
      self.topRow + self.leftCol + self.bottomRow + self.rightCol
    }

    // Java: boolean equals(Object rhs)
    pub fn equals(&self, rhs: Option<&KmoRange>) -> bool {
      // Java: if (rhs == null)
      if rhs.is_none() {
         // Java: return false;
         return false;
      }
      // Java: else if (this == rhs)
      // LLM Rust No Need: reference equality not applicable in Rust
      else if false {
         // Java: return true;
         return true;
      }
      // Java: else if (!KmoRange.class.isInstance(rhs))
      // LLM Rust No Need: type check not needed, parameter is already typed
      else if false {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: KmoRange range = (KmoRange)rhs;
         let range = rhs.unwrap();
         // Java: return range.topRow == this.topRow && range.leftCol == this.leftCol && range.bottomRow == this.bottomRow && range.rightCol == this.rightCol;
         return range.topRow == self.topRow && range.leftCol == self.leftCol && range.bottomRow == self.bottomRow && range.rightCol == self.rightCol;
      }
    }

}
