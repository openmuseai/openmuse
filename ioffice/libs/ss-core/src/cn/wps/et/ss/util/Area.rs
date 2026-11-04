// java package cn.wps.et.ss.util;

// java public class Area implements Cloneable
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Area {
    // java private int firstRow;
    firstRow: i32,
    // java private int lastRow;
    lastRow: i32,
    // java private int firstCol;
    firstCol: i32,
    // java private int lastCol;
    lastCol: i32,
}

impl Area {
    // java public Area()
    pub fn new() -> Self { Self::default() }
    // java public Area(int firstRow, int lastRow, int firstCol, int lastCol)
    pub fn Area(firstRow: i32, lastRow: i32, firstCol: i32, lastCol: i32) -> Self {
        Self { firstRow, lastRow, firstCol, lastCol }
    }
    // java public Area(Area a)
    pub fn fromArea(a: &Area) -> Self { a.clone() }
    // java public int getFirstRow()
    pub fn getFirstRow(&self) -> i32 { self.firstRow }
    // java public void setFirstRow(int firstRow)
    pub fn setFirstRow(&mut self, firstRow: i32) { self.firstRow = firstRow; }
    // java public int getLastRow()
    pub fn getLastRow(&self) -> i32 { self.lastRow }
    // java public void setLastRow(int lastRow)
    pub fn setLastRow(&mut self, lastRow: i32) { self.lastRow = lastRow; }
    // java public int getFirstCol()
    pub fn getFirstCol(&self) -> i32 { self.firstCol }
    // java public void setFirstCol(int firstCol)
    pub fn setFirstCol(&mut self, firstCol: i32) { self.firstCol = firstCol; }
    // java public int getLastCol()
    pub fn getLastCol(&self) -> i32 { self.lastCol }
    // java public void setLastCol(int lastCol)
    pub fn setLastCol(&mut self, lastCol: i32) { self.lastCol = lastCol; }
    // java public void set(int firstRow, int lastRow, int firstCol, int lastCol)
    pub fn set(&mut self, firstRow: i32, lastRow: i32, firstCol: i32, lastCol: i32) {
        *self = Self::Area(firstRow, lastRow, firstCol, lastCol);
    }
    // java public int height()
    pub fn height(&self) -> i32 { self.lastRow.wrapping_sub(self.firstRow).wrapping_add(1) }
    // java public int width()
    pub fn width(&self) -> i32 { self.lastCol.wrapping_sub(self.firstCol).wrapping_add(1) }
    // java public int acreage()
    pub fn acreage(&self) -> i32 { self.height().wrapping_mul(self.width()) }
    // java public boolean isSingleCell()
    pub fn isSingleCell(&self) -> bool { self.firstRow == self.lastRow && self.firstCol == self.lastCol }
    // java public boolean isSingleRow()
    pub fn isSingleRow(&self) -> bool { self.firstRow == self.lastRow }
    // java public boolean isSingleCol()
    pub fn isSingleCol(&self) -> bool { self.firstCol == self.lastCol }
    // java public boolean isValid()
    pub fn isValid(&self) -> bool { 0 <= self.firstRow && self.firstRow <= self.lastRow && 0 <= self.firstCol && self.firstCol <= self.lastCol }
    // java public boolean intersects(Area a)
    pub fn intersects(&self, a: &Area) -> bool { self.firstRow <= a.lastRow && self.lastRow >= a.firstRow && self.firstCol <= a.lastCol && self.lastCol >= a.firstCol }
    // java public Area intersectWith(Area a)
    pub fn intersectWith(&self, a: &Area) -> Option<Self> {
        if !self.intersects(a) { return None; }
        Some(Self::Area(self.firstRow.max(a.firstRow), self.lastRow.min(a.lastRow), self.firstCol.max(a.firstCol), self.lastCol.min(a.lastCol)))
    }
    // java public boolean contains(Area a)
    pub fn containsArea(&self, a: &Area) -> bool { self.firstRow <= a.firstRow && a.lastRow <= self.lastRow && self.firstCol <= a.firstCol && a.lastCol <= self.lastCol }
    // java public boolean contains(int row, int col)
    pub fn contains(&self, row: i32, col: i32) -> bool { self.firstRow <= row && row <= self.lastRow && self.firstCol <= col && col <= self.lastCol }
    // java public Area offset(int rowOff, int colOff)
    pub fn offset(&self, rowOff: i32, colOff: i32) -> Self { Self::Area(self.firstRow.wrapping_add(rowOff), self.lastRow.wrapping_add(rowOff), self.firstCol.wrapping_add(colOff), self.lastCol.wrapping_add(colOff)) }
    // java public Area clone()
    pub fn copy(&self) -> Self { self.clone() }
    // java public boolean equals(Object o)
    pub fn equals(&self, o: &Area) -> bool { self == o }
    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
        let mut h: i32 = 17;
        h = h.wrapping_add(self.firstRow).wrapping_mul(17);
        h = h.wrapping_add(self.lastRow).wrapping_mul(19);
        h = h.wrapping_add(self.firstCol).wrapping_mul(31);
        h.wrapping_add(self.lastCol)
    }
}
