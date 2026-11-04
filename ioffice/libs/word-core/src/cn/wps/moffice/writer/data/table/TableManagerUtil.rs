// Java: public class TableManagerUtil
// rust: 行/格节点是 KTableManager.createRow 的返回值。其余链表池方法仍未进产品编译面。

#[derive(Clone)]
pub struct KTableRow {
    pub mOffset: i32,
    pub mLength: i32,
    pub mIsComplete: bool,
    cells: Vec<KTableCell>,
}

#[derive(Clone)]
pub struct KTableCell {
    pub mOffset: i32,
    pub mLength: i32,
    pub mIsComplete: bool,
    rows: Vec<KTableRow>,
}

impl KTableRow {
    pub fn init(offset: i32) -> Self {
        Self {
            mOffset: offset,
            mLength: 0,
            mIsComplete: false,
            cells: Vec::new(),
        }
    }

    pub fn getStart(&self) -> i32 {
        self.mOffset
    }

    pub fn getEnd(&self) -> i32 {
        self.mOffset + self.mLength
    }

    pub fn isComplete(&self) -> bool {
        self.mIsComplete
    }

    pub fn appendChild(&mut self, _cp: i32, cell: KTableCell) {
        self.cells.push(cell);
    }

    pub fn cells(&self) -> &[KTableCell] {
        &self.cells
    }
}

impl KTableCell {
    pub fn init(offset: i32) -> Self {
        Self {
            mOffset: offset,
            mLength: 0,
            mIsComplete: false,
            rows: Vec::new(),
        }
    }

    pub fn isComplete(&self) -> bool {
        self.mIsComplete
    }

    pub fn getStart(&self) -> i32 {
        self.mOffset
    }

    pub fn getEnd(&self) -> i32 {
        self.mOffset + self.mLength
    }

    pub fn appendChild(&mut self, _cp: i32, row: KTableRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[KTableRow] {
        &self.rows
    }
}
