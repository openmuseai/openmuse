// java package cn.wps.moss.util;

// java import java.util.Collection;
// java import org.apache.poi.ss.SpreadsheetVersion;
// java import org.apache.poi.ss.util.CellReference;
use super::KmoPoint::KmoPoint;

// java public class KmoRect
#[derive(Clone, Debug)]
pub struct KmoRect {
    // java public KmoPoint tl = new KmoPoint();
    pub tl: KmoPoint,
    // java public KmoPoint br = new KmoPoint();
    pub br: KmoPoint,
}

impl PartialEq for KmoRect {
    fn eq(&self, other: &Self) -> bool {
        self.equals(Some(other))
    }
}

impl Eq for KmoRect {}

impl std::hash::Hash for KmoRect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.hashCode());
    }
}

// java public class KmoRect
impl KmoRect {
    // java public static void splitRange(KmoRect opRange, KmoRect splitedRange, Collection<KmoRect> ranges)
    pub fn splitRange(opRange: &KmoRect, splitedRange: &KmoRect, ranges: &mut Vec<KmoRect>) {
        // java if (!opRange.intersects(splitedRange)) {
        if !opRange.intersects(splitedRange) {
            // java ranges.add(new KmoRect(splitedRange));
            ranges.push(KmoRect::KmoRect__copy(splitedRange));
        } else {
            // java KmoRect topRange = new KmoRect(splitedRange);
            let mut topRange = KmoRect::KmoRect__copy(splitedRange);
            // java topRange.br.row = opRange.tl.row - 1;
            topRange.br.row = opRange.tl.row - 1;
            // java topRange = topRange.getIntersectRect(splitedRange);
            let topRange = topRange.getIntersectRect(splitedRange);
            // java KmoRect bottomRange = new KmoRect(splitedRange);
            let mut bottomRange = KmoRect::KmoRect__copy(splitedRange);
            // java bottomRange.tl.row = opRange.br.row + 1;
            bottomRange.tl.row = opRange.br.row + 1;
            // java bottomRange = bottomRange.getIntersectRect(splitedRange);
            let bottomRange = bottomRange.getIntersectRect(splitedRange);
            // java KmoRect leftRange = new KmoRect(opRange);
            let mut leftRange = KmoRect::KmoRect__copy(opRange);
            // java leftRange.tl.col = splitedRange.tl.col;
            leftRange.tl.col = splitedRange.tl.col;
            // java leftRange.br.col = opRange.tl.col - 1;
            leftRange.br.col = opRange.tl.col - 1;
            // java leftRange = leftRange.getIntersectRect(splitedRange);
            let leftRange = leftRange.getIntersectRect(splitedRange);
            // java KmoRect rightRange = new KmoRect(opRange);
            let mut rightRange = KmoRect::KmoRect__copy(opRange);
            // java rightRange.tl.col = opRange.br.col + 1;
            rightRange.tl.col = opRange.br.col + 1;
            // java rightRange.br.col = splitedRange.br.col;
            rightRange.br.col = splitedRange.br.col;
            // java rightRange = rightRange.getIntersectRect(splitedRange);
            let rightRange = rightRange.getIntersectRect(splitedRange);
            // java if (topRange != null && topRange.isValid()) {
            if topRange.as_ref().map(|r| r.isValid()).unwrap_or(false) {
                // java ranges.add(topRange);
                ranges.push(topRange.unwrap());
            }
            // java if (bottomRange != null && bottomRange.isValid()) {
            if bottomRange.as_ref().map(|r| r.isValid()).unwrap_or(false) {
                // java ranges.add(bottomRange);
                ranges.push(bottomRange.unwrap());
            }
            // java if (leftRange != null && leftRange.isValid()) {
            if leftRange.as_ref().map(|r| r.isValid()).unwrap_or(false) {
                // java ranges.add(leftRange);
                ranges.push(leftRange.unwrap());
            }
            // java if (rightRange != null && rightRange.isValid()) {
            if rightRange.as_ref().map(|r| r.isValid()).unwrap_or(false) {
                // java ranges.add(rightRange);
                ranges.push(rightRange.unwrap());
            }
        }
    }

    // java public KmoRect()
    pub fn KmoRect() -> Self {
        Self {
            tl: KmoPoint::KmoPoint(),
            br: KmoPoint::KmoPoint(),
        }
    }

    // java public KmoRect(KmoRect rhs)
    pub fn KmoRect__copy(rhs: &KmoRect) -> Self {
        // java this.copy(rhs);
        let mut rc = Self::KmoRect();
        rc.copy(rhs);
        rc
    }

    // java public KmoRect(int topRow, int leftCol, int bottomRow, int rightCol)
    pub fn KmoRect__bounds(topRow: i32, leftCol: i32, bottomRow: i32, rightCol: i32) -> Self {
        // java this.copy(topRow, leftCol, bottomRow, rightCol);
        let mut rc = Self::KmoRect();
        rc.copy__bounds(topRow, leftCol, bottomRow, rightCol);
        rc
    }

    // java public boolean equals(Object rhs)
    pub fn equals(&self, rhs: Option<&KmoRect>) -> bool {
        // java if (rhs == null) {
        if rhs.is_none() {
            // java return false;
            return false;
        }
        let rhs = rhs.unwrap();
        // java else if (this == rhs) {
        // java return true;
        // rust: no-stmt — 值类型没有单独的引用身份；字段相同则与 Java 的 this == rhs 结果一致
        // java else if (!KmoRect.class.isInstance(rhs)) {
        // java return false;
        // rust: no-stmt — 参数已是 KmoRect
        // java KmoRect rc = (KmoRect)rhs;
        let rc = rhs;
        // java return this.tl.equals(rc.tl) && this.br.equals(rc.br);
        self.tl.equals(Some(&rc.tl)) && self.br.equals(Some(&rc.br))
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
        // java return this.tl.row + this.tl.col + this.br.row + this.br.col;
        self.tl
            .row
            .wrapping_add(self.tl.col)
            .wrapping_add(self.br.row)
            .wrapping_add(self.br.col)
    }

    // java public final boolean isContains(int row, int col)
    pub fn isContains__rc(&self, row: i32, col: i32) -> bool {
        // java return row >= this.tl.row && row <= this.br.row && col >= this.tl.col && col <= this.br.col;
        row >= self.tl.row && row <= self.br.row && col >= self.tl.col && col <= self.br.col
    }

    // java public final boolean isContains(int firstRow, int firstCol, int lastRow, int lastCol)
    pub fn isContains__span(&self, firstRow: i32, firstCol: i32, lastRow: i32, lastCol: i32) -> bool {
        // java return this.isContains(firstRow, firstCol) && this.isContains(lastRow, lastCol);
        self.isContains__rc(firstRow, firstCol) && self.isContains__rc(lastRow, lastCol)
    }

    // java public final boolean isContains(KmoPoint pt)
    pub fn isContains__pt(&self, pt: &KmoPoint) -> bool {
        // java return this.isContains(pt.row, pt.col);
        self.isContains__rc(pt.row, pt.col)
    }

    // java public final boolean isContainsRow(int row)
    pub fn isContainsRow(&self, row: i32) -> bool {
        // java return row >= this.tl.row && row <= this.br.row;
        row >= self.tl.row && row <= self.br.row
    }

    // java public final boolean isContainsCol(int col)
    pub fn isContainsCol(&self, col: i32) -> bool {
        // java return col >= this.tl.col && col <= this.br.col;
        col >= self.tl.col && col <= self.br.col
    }

    // java public boolean isContains(KmoRect rc)
    pub fn isContains(&self, rc: &KmoRect) -> bool {
        // java return this.isContains(rc.tl) && this.isContains(rc.br);
        self.isContains__pt(&rc.tl) && self.isContains__pt(&rc.br)
    }

    // java public final boolean intersects(KmoRect rc)
    pub fn intersects(&self, rc: &KmoRect) -> bool {
        // java return this.tl.col <= rc.br.col && this.br.col >= rc.tl.col && this.tl.row <= rc.br.row && this.br.row >= rc.tl.row;
        self.tl.col <= rc.br.col
            && self.br.col >= rc.tl.col
            && self.tl.row <= rc.br.row
            && self.br.row >= rc.tl.row
    }

    // java public final int area()
    pub fn area(&self) -> i32 {
        // java return (this.br.row - this.tl.row + 1) * (this.br.col - this.tl.col + 1);
        (self.br.row - self.tl.row + 1) * (self.br.col - self.tl.col + 1)
    }

    // java public void intersectCut(KmoRect rect)
    pub fn intersectCut(&mut self, rect: &KmoRect) {
        // java this.tl.row = Math.max(this.tl.row, rect.tl.row);
        self.tl.row = self.tl.row.max(rect.tl.row);
        // java this.tl.col = Math.max(this.tl.col, rect.tl.col);
        self.tl.col = self.tl.col.max(rect.tl.col);
        // java this.br.row = Math.min(this.br.row, rect.br.row);
        self.br.row = self.br.row.min(rect.br.row);
        // java this.br.col = Math.min(this.br.col, rect.br.col);
        self.br.col = self.br.col.min(rect.br.col);
    }

    // java public int fillSrcOut(KmoRect rect)
    pub fn fillSrcOut(&mut self, rect: &KmoRect) -> i32 {
        // java int ret = -1;
        let ret: i32 = -1;
        // java if (this.equals(rect)) {
        if self.equals(Some(rect)) {
            // java return ret;
            return ret;
        } else {
            // java if (this.tl.equals(rect.tl)) {
            if self.tl.equals(Some(&rect.tl)) {
                // java if (this.br.col == rect.br.col) {
                if self.br.col == rect.br.col {
                    // java this.tl.row = rect.br.row + 1;
                    self.tl.row = rect.br.row + 1;
                    // java return 1;
                    return 1;
                }
                // java if (this.br.row == rect.br.row) {
                if self.br.row == rect.br.row {
                    // java this.tl.col = rect.br.col + 1;
                    self.tl.col = rect.br.col + 1;
                    // java return 2;
                    return 2;
                }
            // java } else if (this.br.equals(rect.br)) {
            } else if self.br.equals(Some(&rect.br)) {
                // java if (this.tl.col == rect.tl.col) {
                if self.tl.col == rect.tl.col {
                    // java this.br.row = rect.tl.row - 1;
                    self.br.row = rect.tl.row - 1;
                    // java return 1;
                    return 1;
                }
                // java if (this.tl.row == rect.tl.row) {
                if self.tl.row == rect.tl.row {
                    // java this.br.col = rect.tl.col - 1;
                    self.br.col = rect.tl.col - 1;
                    // java return 2;
                    return 2;
                }
            }
            // java return ret;
            return ret;
        }
    }

    // java public KmoRect copy(KmoRect rc)
    pub fn copy(&mut self, rc: &KmoRect) -> &mut Self {
        // java this.tl.copy(rc.tl);
        self.tl.copy(&rc.tl);
        // java this.br.copy(rc.br);
        self.br.copy(&rc.br);
        // java return this;
        self
    }

    // java public KmoRect copy(int topRow, int leftCol, int bottomRow, int rightCol)
    pub fn copy__bounds(&mut self, topRow: i32, leftCol: i32, bottomRow: i32, rightCol: i32) -> &mut Self {
        // java this.tl.row = Math.min(topRow, bottomRow);
        self.tl.row = topRow.min(bottomRow);
        // java this.tl.col = Math.min(leftCol, rightCol);
        self.tl.col = leftCol.min(rightCol);
        // java this.br.row = Math.max(topRow, bottomRow);
        self.br.row = topRow.max(bottomRow);
        // java this.br.col = Math.max(leftCol, rightCol);
        self.br.col = leftCol.max(rightCol);
        // java return this;
        self
    }

    // java public KmoRect combine(KmoRect rc)
    pub fn combine(&mut self, rc: &KmoRect) -> &mut Self {
        // java this.tl.row = Math.min(rc.tl.row, this.tl.row);
        self.tl.row = rc.tl.row.min(self.tl.row);
        // java this.tl.col = Math.min(rc.tl.col, this.tl.col);
        self.tl.col = rc.tl.col.min(self.tl.col);
        // java this.br.row = Math.max(rc.br.row, this.br.row);
        self.br.row = rc.br.row.max(self.br.row);
        // java this.br.col = Math.max(rc.br.col, this.br.col);
        self.br.col = rc.br.col.max(self.br.col);
        // java return this;
        self
    }

    // java public KmoRect combine(int row, int col)
    pub fn combine__rc(&mut self, row: i32, col: i32) -> &mut Self {
        // java this.tl.row = Math.min(row, this.tl.row);
        self.tl.row = row.min(self.tl.row);
        // java this.tl.col = Math.min(col, this.tl.col);
        self.tl.col = col.min(self.tl.col);
        // java this.br.row = Math.max(row, this.br.row);
        self.br.row = row.max(self.br.row);
        // java this.br.col = Math.max(col, this.br.col);
        self.br.col = col.max(self.br.col);
        // java return this;
        self
    }

    // java public void setZero()
    pub fn setZero(&mut self) {
        // java this.tl.row = 0;
        self.tl.row = 0;
        // java this.tl.col = 0;
        self.tl.col = 0;
        // java this.br.row = 0;
        self.br.row = 0;
        // java this.br.col = 0;
        self.br.col = 0;
    }

    // java public void set(int topRow, int leftCol, int bottomRow, int rightCol)
    pub fn set(&mut self, topRow: i32, leftCol: i32, bottomRow: i32, rightCol: i32) {
        // java this.tl.row = Math.min(topRow, bottomRow);
        self.tl.row = topRow.min(bottomRow);
        // java this.tl.col = Math.min(leftCol, rightCol);
        self.tl.col = leftCol.min(rightCol);
        // java this.br.row = Math.max(topRow, bottomRow);
        self.br.row = topRow.max(bottomRow);
        // java this.br.col = Math.max(leftCol, rightCol);
        self.br.col = leftCol.max(rightCol);
    }

    // java public KmoRect combineEx(KmoRect rc)
    pub fn combineEx(&mut self, rc: &KmoRect) -> &mut Self {
        // java this.tl.row = Math.min(rc.tl.row, this.tl.row);
        self.tl.row = rc.tl.row.min(self.tl.row);
        // java this.tl.col = Math.min(rc.tl.col, this.tl.col);
        self.tl.col = rc.tl.col.min(self.tl.col);
        // java this.br.row = Math.max(rc.br.row + 1, this.br.row);
        self.br.row = (rc.br.row + 1).max(self.br.row);
        // java this.br.col = Math.max(rc.br.col + 1, this.br.col);
        self.br.col = (rc.br.col + 1).max(self.br.col);
        // java return this;
        self
    }

    // java public KmoRect combineEx(int row, int col)
    pub fn combineEx__rc(&mut self, row: i32, col: i32) -> &mut Self {
        // java this.tl.row = Math.min(row, this.tl.row);
        self.tl.row = row.min(self.tl.row);
        // java this.tl.col = Math.min(col, this.tl.col);
        self.tl.col = col.min(self.tl.col);
        // java this.br.row = Math.max(row + 1, this.br.row);
        self.br.row = (row + 1).max(self.br.row);
        // java this.br.col = Math.max(col + 1, this.br.col);
        self.br.col = (col + 1).max(self.br.col);
        // java return this;
        self
    }

    // java public int height()
    pub fn height(&self) -> i32 {
        // java return Math.abs(this.br.row - this.tl.row) + 1;
        (self.br.row - self.tl.row).abs() + 1
    }

    // java public int width()
    pub fn width(&self) -> i32 {
        // java return Math.abs(this.br.col - this.tl.col) + 1;
        (self.br.col - self.tl.col).abs() + 1
    }

    // java public KmoRect getIntersectRect(KmoRect rect)
    pub fn getIntersectRect(&self, rect: &KmoRect) -> Option<KmoRect> {
        // java if (!this.intersects(rect)) {
        if !self.intersects(rect) {
            // java return null;
            return None;
        } else {
            // java KmoRect ret = new KmoRect();
            let mut ret = KmoRect::KmoRect();
            // java ret.tl.row = Math.max(this.tl.row, rect.tl.row);
            ret.tl.row = self.tl.row.max(rect.tl.row);
            // java ret.tl.col = Math.max(this.tl.col, rect.tl.col);
            ret.tl.col = self.tl.col.max(rect.tl.col);
            // java ret.br.row = Math.min(this.br.row, rect.br.row);
            ret.br.row = self.br.row.min(rect.br.row);
            // java ret.br.col = Math.min(this.br.col, rect.br.col);
            ret.br.col = self.br.col.min(rect.br.col);
            // java return ret;
            Some(ret)
        }
    }

    // java public void getIntersectRect(KmoRect rect, KmoRect outRect)
    pub fn getIntersectRect__out(&self, rect: &KmoRect, outRect: Option<&mut KmoRect>) {
        // java if (outRect != null && this.intersects(rect)) {
        if let Some(outRect) = outRect {
            if self.intersects(rect) {
                // java outRect.tl.row = Math.max(this.tl.row, rect.tl.row);
                outRect.tl.row = self.tl.row.max(rect.tl.row);
                // java outRect.tl.col = Math.max(this.tl.col, rect.tl.col);
                outRect.tl.col = self.tl.col.max(rect.tl.col);
                // java outRect.br.row = Math.min(this.br.row, rect.br.row);
                outRect.br.row = self.br.row.min(rect.br.row);
                // java outRect.br.col = Math.min(this.br.col, rect.br.col);
                outRect.br.col = self.br.col.min(rect.br.col);
            }
        }
    }

    // java public void makeValid()
    pub fn makeValid(&mut self) {
        // java int firstRow = Math.min(this.tl.row, this.br.row);
        let firstRow = self.tl.row.min(self.br.row);
        // java int lastRow = Math.max(this.tl.row, this.br.row);
        let lastRow = self.tl.row.max(self.br.row);
        // java int firstCol = Math.min(this.tl.col, this.br.col);
        let firstCol = self.tl.col.min(self.br.col);
        // java int lastCol = Math.max(this.tl.col, this.br.col);
        let lastCol = self.tl.col.max(self.br.col);
        // java this.tl.row = firstRow;
        self.tl.row = firstRow;
        // java this.tl.col = firstCol;
        self.tl.col = firstCol;
        // java this.br.row = lastRow;
        self.br.row = lastRow;
        // java this.br.col = lastCol;
        self.br.col = lastCol;
    }

    // java public boolean isValid()
    pub fn isValid(&self) -> bool {
        // java if (0 <= this.tl.row && this.tl.row <= this.br.row && this.br.row <= SpreadsheetVersion.CUR_VERSION.getLastRowIndex() && 0 <= this.tl.col && this.tl.col <= this.br.col && this.br.col <= SpreadsheetVersion.CUR_VERSION.getLastColumnIndex()) {
        let ver = crate::org::apache::poi::ss::SpreadsheetVersion::CUR_VERSION
            .lock()
            .unwrap();
        if 0 <= self.tl.row
            && self.tl.row <= self.br.row
            && self.br.row <= ver.getLastRowIndex()
            && 0 <= self.tl.col
            && self.tl.col <= self.br.col
            && self.br.col <= ver.getLastColumnIndex()
        {
            // java return this.tl.row <= this.br.row && this.tl.col <= this.br.col;
            return self.tl.row <= self.br.row && self.tl.col <= self.br.col;
        } else {
            // java return false;
            return false;
        }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (0 <= this.tl.row && this.tl.row <= this.br.row && this.br.row <= SpreadsheetVersion.CUR_VERSION.getLastRowIndex() && 0 <= this.tl.col && this.tl.col <= this.br.col && this.br.col <= SpreadsheetVersion.CUR_VERSION.getLastColumnIndex()) {
        // java CellReference crA = new CellReference(this.tl.row, this.tl.col);
        // java CellReference crB = new CellReference(this.br.row, this.br.col);
        // java return this.getClass().getName() + " [" + crA.formatAsString() + ":" + crB.formatAsString() + "]";
        // java return "#REF!";
        */
    }

    // java public boolean isSingleCell()
    pub fn isSingleCell(&self) -> bool {
        // java return this.tl.row == this.br.row && this.tl.col == this.br.col;
        self.tl.row == self.br.row && self.tl.col == self.br.col
    }

    // java public boolean isFullRow(int maxCol)
    pub fn isFullRow(&self, maxCol: i32) -> bool {
        // java int firstCol = this.tl.col;
        let firstCol = self.tl.col;
        // java int lastCol = this.br.col;
        let lastCol = self.br.col;
        // java return lastCol - firstCol == maxCol - 1;
        lastCol - firstCol == maxCol - 1
    }

    // java public boolean isFullCol(int maxRow)
    pub fn isFullCol(&self, maxRow: i32) -> bool {
        // java int firstRow = this.tl.row;
        let firstRow = self.tl.row;
        // java int lastRow = this.br.row;
        let lastRow = self.br.row;
        // java return lastRow - firstRow == maxRow - 1;
        lastRow - firstRow == maxRow - 1
    }

    // java public boolean isFullSheet(int maxCol, int maxRow)
    pub fn isFullSheet(&self, maxCol: i32, maxRow: i32) -> bool {
        // java return this.isFullRow(maxCol) && this.isFullCol(maxRow);
        self.isFullRow(maxCol) && self.isFullCol(maxRow)
    }

    // java public boolean isSingleCol()
    pub fn isSingleCol(&self) -> bool {
        // java int firstCol = this.tl.col;
        let firstCol = self.tl.col;
        // java int lastCol = this.br.col;
        let lastCol = self.br.col;
        // java return lastCol == firstCol;
        lastCol == firstCol
    }

    // java public boolean isSingleRow()
    pub fn isSingleRow(&self) -> bool {
        // java int firstRow = this.tl.row;
        let firstRow = self.tl.row;
        // java int lastRow = this.br.row;
        let lastRow = self.br.row;
        // java return lastRow == firstRow;
        lastRow == firstRow
    }
}
