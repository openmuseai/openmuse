// java package cn.wps.moss.util;

// java public class KmoPoint
#[derive(Clone, Copy, Debug)]
pub struct KmoPoint {
    // java public int row;
    pub row: i32,
    // java public int col;
    pub col: i32,
}

// java public class KmoPoint
impl KmoPoint {
    // java public KmoPoint()
    pub fn KmoPoint() -> Self {
        Self { row: 0, col: 0 }
    }

    // java public KmoPoint(int row, int col)
    pub fn KmoPoint__rc(row: i32, col: i32) -> Self {
        // java this.copy(row, col);
        let mut pt = Self::KmoPoint();
        pt.copy__rc(row, col);
        pt
    }

    // java public KmoPoint copy(KmoPoint pt)
    pub fn copy(&mut self, pt: &KmoPoint) -> &mut Self {
        // java this.row = pt.row;
        self.row = pt.row;
        // java this.col = pt.col;
        self.col = pt.col;
        // java return this;
        self
    }

    // java public KmoPoint copy(int row, int col)
    pub fn copy__rc(&mut self, row: i32, col: i32) -> &mut Self {
        // java this.row = row;
        self.row = row;
        // java this.col = col;
        self.col = col;
        // java return this;
        self
    }

    // java public boolean equals(Object rhs)
    pub fn equals(&self, rhs: Option<&KmoPoint>) -> bool {
        // java if (rhs == null) {
        if rhs.is_none() {
            // java return false;
            return false;
        }
        let rhs = rhs.unwrap();
        // java else if (rhs == this) {
        // java return true;
        // rust: no-stmt — 值类型没有单独的引用身份；字段相同则与 Java 的 this == rhs 结果一致
        // java else if (!KmoPoint.class.isInstance(rhs)) {
        // java return false;
        // rust: no-stmt — 参数已是 KmoPoint
        // java KmoPoint pt = (KmoPoint)rhs;
        let pt = rhs;
        // java return pt.row == this.row && pt.col == this.col;
        pt.row == self.row && pt.col == self.col
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
        // java return this.row + this.col;
        self.row.wrapping_add(self.col)
    }
}
