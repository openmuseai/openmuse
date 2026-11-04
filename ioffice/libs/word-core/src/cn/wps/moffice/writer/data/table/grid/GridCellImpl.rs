#![allow(non_snake_case)]

// Java: public class GridCellImpl implements GridCell
#[derive(Clone, Debug, Default)]
pub struct GridCellImpl {
    // Java field: the cell's index inside its row
    pub cell_index: i32,
    // Java field: GridCell mPreSpan
    pub pre_col: Option<usize>,
    // Java field: GridCell mNextSpan
    pub next_col: Option<usize>,
}

impl GridCellImpl {
    pub fn new(cell_index: i32) -> Self {
        Self {
            cell_index,
            pre_col: None,
            next_col: None,
        }
    }
}
