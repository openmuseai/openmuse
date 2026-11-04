use crate::cn::wps::moss::util::KmoRect::KmoRect;
// java public final class GridRange
#[derive(Clone, Debug, Default)]
pub struct GridRange {
    pub mFirstRow: i32,
    pub mLastRow: i32,
    pub mFirstColumn: i32,
    pub mLastColumn: i32,
}
impl GridRange {
    // java public GridRange(int firstRow, int lastRow, int firstColumn, int lastColumn) {
    // java this.set(firstRow, lastRow, firstColumn, lastColumn);
    // java }
    pub fn new(fr: i32, lr: i32, fc: i32, lc: i32) -> Self {
        Self {
            mFirstRow: fr,
            mLastRow: lr,
            mFirstColumn: fc,
            mLastColumn: lc,
        }
    }
    // java public void set(int firstRow, int lastRow, int firstColumn, int lastColumn) {
    // java this.mFirstRow = firstRow;
    // java this.mLastRow = lastRow;
    // java this.mFirstColumn = firstColumn;
    // java this.mLastColumn = lastColumn;
    // java }
    pub fn set(&mut self, fr: i32, lr: i32, fc: i32, lc: i32) {
        self.mFirstRow = fr;
        self.mLastRow = lr;
        self.mFirstColumn = fc;
        self.mLastColumn = lc;
    }
    // java public GridRange combine(GridRange src) {
    // java this.mFirstRow = Math.min(src.mFirstRow, this.mFirstRow);
    // java this.mFirstColumn = Math.min(src.mFirstColumn, this.mFirstColumn);
    // java this.mLastRow = Math.max(src.mLastRow, this.mLastRow);
    // java this.mLastColumn = Math.max(src.mLastColumn, this.mLastColumn);
    // java return this;
    // java }
    pub fn combine(&mut self, s: &Self) -> &mut Self {
        self.mFirstRow = self.mFirstRow.min(s.mFirstRow);
        self.mFirstColumn = self.mFirstColumn.min(s.mFirstColumn);
        self.mLastRow = self.mLastRow.max(s.mLastRow);
        self.mLastColumn = self.mLastColumn.max(s.mLastColumn);
        self
    }
    // java public final boolean isContains(int row, int col) {
    // java return row >= this.mFirstRow && row <= this.mLastRow && col >= this.mFirstColumn && col <= this.mLastColumn;
    // java }
    pub fn isContains(&self, r: i32, c: i32) -> bool {
        r >= self.mFirstRow && r <= self.mLastRow && c >= self.mFirstColumn && c <= self.mLastColumn
    }
    // java public final boolean isContains(KmoRect kmoRect) {
    // java return kmoRect.tl.row >= this.mFirstRow && kmoRect.tl.col >= this.mFirstColumn && kmoRect.br.row <= this.mLastRow && kmoRect.br.col <= this.mLastColumn;
    // java }
    pub fn isContains__rect(&self, r: &KmoRect) -> bool {
        r.tl.row >= self.mFirstRow
            && r.tl.col >= self.mFirstColumn
            && r.br.row <= self.mLastRow
            && r.br.col <= self.mLastColumn
    }
    // java public final boolean isEmbed(KmoRect kmoRect) {
    // java return kmoRect.tl.row > this.mFirstRow && kmoRect.tl.col > this.mFirstColumn && kmoRect.br.row < this.mLastRow && kmoRect.br.col < this.mLastColumn;
    // java }
    pub fn isEmbed(&self, r: &KmoRect) -> bool {
        r.tl.row > self.mFirstRow
            && r.tl.col > self.mFirstColumn
            && r.br.row < self.mLastRow
            && r.br.col < self.mLastColumn
    }
    // java public void copyFrom(GridRange otherGridRange) {
    // java if (otherGridRange != null) {
    // java this.mFirstRow = otherGridRange.mFirstRow;
    // java this.mLastRow = otherGridRange.mLastRow;
    // java this.mFirstColumn = otherGridRange.mFirstColumn;
    // java this.mLastColumn = otherGridRange.mLastColumn;
    // java }
    // java }
    pub fn copyFrom(&mut self, s: Option<&Self>) {
        if let Some(s) = s {
            *self = s.clone();
        }
    }
}
