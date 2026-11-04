use ss_core::java_runtime::RefCell;
use ss_core::java_runtime::Rc;

use ss_core::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;
use ss_core::cn::wps::moss::engine::grid::KmoGrid::KmoGrid;
use ss_core::cn::wps::moss::engine::grid::KmoMatrix::{
    KmoMatrix, KmoRangeBlockInfo, BYTES_PER_NODE, H, W,
};
use ss_core::cn::wps::moss::engine::grid::KmoMatrixToolkit::KmoMatrixToolkit;
use ss_core::cn::wps::moss::util::KmoRect::KmoRect;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

fn cell(ixf: i32, id: i32) -> KmoCellData {
    let mut cell = KmoCellData::default();
    cell.ixf = ixf;
    cell.id = id;
    cell.formulaType = 1;
    cell
}

fn rect_of(matrix: &KmoMatrix) -> (i32, i32, i32, i32) {
    let ur = matrix.getUsedRange();
    let g = ur.borrow();
    (g.tl.row, g.tl.col, g.br.row, g.br.col)
}

#[test]
fn sg1_block_boundaries_and_readback() {
    assert_eq!(BYTES_PER_NODE, 7);
    assert_eq!(H, 32);
    assert_eq!(W, 16);
    assert_eq!(KmoMatrixToolkit::newBuff(), [0u8; 7]);

    let grid = KmoGrid::KmoGrid();
    assert_eq!(grid.getMatrix().borrow().getMaxRow(), 65536);
    assert_eq!(grid.getMatrix().borrow().getMaxCol(), 256);

    let sample = cell(15, 42);
    grid.cellDele().write(0, 0, &sample);
    grid.cellDele().write(31, 15, &sample);
    grid.cellDele().write(32, 16, &sample);
    grid.cellDele().write(31, 16, &sample);

    let mut back = KmoCellData::default();
    grid.cellDele().read(0, 0, &mut back);
    assert_eq!(back.ixf, 15);
    assert_eq!(back.id, 42);
    assert_eq!(back.formulaType, 1);

    grid.cellDele().read(31, 15, &mut back);
    assert_eq!(back.id, 42);
    grid.cellDele().read(32, 16, &mut back);
    assert_eq!(back.id, 42);
    grid.cellDele().read(31, 16, &mut back);
    assert_eq!(back.id, 42);

    let matrix = grid.getMatrix();
    let matrix = matrix.borrow();
    assert!(matrix.getRow(0).is_some());
    assert!(matrix.getRow(1).is_some());
    assert!(matrix.getRow(2).is_none());
    assert_eq!(matrix.getOffset(31, 15), ((31 << 4) + 15) * 7);
    assert_eq!(matrix.getOffset(32, 16), 0);
    assert_eq!(matrix.getBlk(0, 0).unwrap().rawByteData().len(), 3584);
    assert_eq!(matrix.getId(0, 0), 42);
    assert_eq!(matrix.getId(1, 1), 0);
    assert_eq!(matrix.getId(0, 32), -1);
    assert!(matrix.isEmptyNode(1, 1));
    assert!(!matrix.isEmptyNode(0, 0));
    assert!(!matrix.isBlankCell(0, 0));
    assert!(grid.cellDele().exists(0, 0));
    assert!(!grid.cellDele().exists(1, 1));
    assert!(!grid.cellDele().exists(-1, 0));

    let mut dirty = [0xFFu8; 7];
    assert!(!matrix.get(0, 32, &mut dirty));
    assert_eq!(dirty, [0xFFu8; 7]);
    assert!(!matrix.get(-1, 0, &mut dirty));
    assert_eq!(dirty, [0xFFu8; 7]);
}

#[test]
fn sg1_iterators_skip_empty_blocks_and_preserve_java_scan_order() {
    let grid = KmoGrid::KmoGrid();
    let sample = cell(15, 42);
    for (row, col) in [(0, 0), (31, 15), (31, 16), (32, 16), (65, 33)] {
        grid.cellDele().write(row, col, &sample);
    }
    let matrix = grid.getMatrix();
    let range = KmoRect::KmoRect__bounds(0, 0, 100, 60);
    let cases = [
        (true, true, true, vec![(0, 0), (31, 15), (31, 16), (32, 16), (65, 33)]),
        (true, false, false, vec![(65, 33), (32, 16), (31, 16), (31, 15), (0, 0)]),
        (false, true, true, vec![(0, 0), (31, 15), (31, 16), (32, 16), (65, 33)]),
        (false, false, false, vec![(65, 33), (32, 16), (31, 16), (31, 15), (0, 0)]),
    ];
    for (row_first, left_to_right, top_to_bottom, expected) in cases {
        let mut iter = KmoMatrix::getCellDataIterator(&matrix, &range, row_first, left_to_right, top_to_bottom);
        let mut actual = Vec::new();
        while iter.hasNext() {
            iter.next();
            assert_eq!(KmoCellData::getId(iter.data()), 42);
            actual.push((iter.row(), iter.col()));
        }
        assert_eq!(actual, expected);
    }
    let empty = KmoRect::KmoRect__bounds(40, 0, 63, 15);
    assert!(!KmoMatrix::getCellDataIterator(&matrix, &empty, true, true, true).hasNext());
}

#[test]
fn sg2_used_range_empty_node_and_init_fill() {
    let used = Rc::new(RefCell::new(KmoRect::KmoRect()));
    let mut matrix = KmoMatrix::KmoMatrix__sheet(
        true,
        Rc::clone(&used),
        SpreadsheetVersion::EXCEL97,
    );
    matrix.set(5, 5, &[0u8; 7]);
    assert_eq!(rect_of(&matrix), (0, 0, 0, 0));
    let mut node = [0xABu8; 7];
    assert!(matrix.get(5, 5, &mut node));
    assert_eq!(node, [0u8; 7]);
    assert!(matrix.get(0, 0, &mut node));
    assert_eq!(node, [0u8; 7]);
    let mut missing = [0xABu8; 7];
    assert!(!matrix.get(0, 16, &mut missing));
    assert_eq!(missing, [0xABu8; 7]);

    let mut mark = [0u8; 7];
    mark[3] = 7;
    matrix.set(5, 3, &mark);
    assert_eq!(rect_of(&matrix), (5, 3, 6, 4));
    matrix.set(5, 8, &mark);
    assert_eq!(rect_of(&matrix), (5, 3, 6, 9));

    let mut fresh = KmoMatrix::KmoMatrix__sheet(
        true,
        Rc::new(RefCell::new(KmoRect::KmoRect())),
        SpreadsheetVersion::EXCEL97,
    );
    fresh.initFill(5, 3, &mark);
    assert_eq!(rect_of(&fresh), (0, 0, 6, 4));
    let mut got = [0u8; 7];
    assert!(fresh.get(5, 3, &mut got));
    assert_eq!(got, mark);
}

#[test]
fn sg3_blank_ixf_clear_move_and_cmd() {
    let grid = KmoGrid::KmoGrid();
    let blank = cell(15, 0);
    grid.cellDele().write(2, 4, &blank);
    let matrix = grid.getMatrix();
    {
        let matrix = matrix.borrow();
        assert!(matrix.isBlankCell(2, 4));
        assert!(!matrix.isEmptyNode(2, 4));
        assert!(!KmoMatrix::isEmptyByte(&[0, 0, 0, 0, 0, 0, 1]));
        assert!(KmoMatrix::isEmptyByte(&[0u8; 7]));
        assert!(matrix.isEmptyRowBlock(40));
        assert!(!matrix.isEmptyRowBlock(2));
        assert!(matrix.isEmptyColBlock(2, 20));
        assert!(!matrix.isEmptyColBlock(2, 4));
    }

    grid.cellDele().delete(2, 4);
    assert!(matrix.borrow().isEmptyNode(2, 4));

    let mut marked = [0u8; 7];
    marked[6] = 9;
    matrix.borrow_mut().set(0, 1, &marked);
    matrix.borrow_mut().set(8, 1, &marked);
    matrix.borrow_mut().moveCol(1, 3, 0, 8);
    let mut moved = [0u8; 7];
    assert!(matrix.borrow().get(0, 3, &mut moved));
    assert_eq!(moved, marked);
    assert!(matrix.borrow().get(8, 3, &mut moved));
    assert_eq!(moved, marked);
    assert!(matrix.borrow().isEmptyNode(0, 1));
    assert!(matrix.borrow().isEmptyNode(8, 1));

    matrix.borrow_mut().moveRow(0, 20, 3, 3, true);
    assert!(matrix.borrow().get(20, 3, &mut moved));
    assert_eq!(moved, marked);
    assert!(matrix.borrow().isEmptyNode(0, 3));
    assert_eq!(matrix.borrow_mut().clearRow(100, 0, 1), 16);

    matrix.borrow_mut().startCmdMode();
    matrix.borrow_mut().set(4, 4, &marked);
    matrix.borrow_mut().endCmdMode();
    assert!(matrix.borrow().get(4, 4, &mut moved));
    assert_eq!(moved, marked);

    matrix.borrow_mut().clearCol(3, 0, 20);
    assert!(matrix.borrow().isEmptyNode(0, 3));
    assert!(matrix.borrow().isEmptyNode(8, 3));
    assert!(matrix.borrow().isEmptyNode(20, 3));

    let info = KmoRangeBlockInfo::KmoRangeBlockInfo(&KmoRect::KmoRect__bounds(31, 15, 32, 16));
    assert_eq!(info.firstBlkRow, 0);
    assert_eq!(info.lastBlkRow, 1);
    assert_eq!(info.firstBlkCol, 0);
    assert_eq!(info.lastBlkCol, 1);
    assert_eq!(info.firstRowInBlk, 31);
    assert_eq!(info.lastRowInBlk, 0);
    assert_eq!(info.firstColInBlk, 15);
    assert_eq!(info.lastColInBlk, 0);

    let wide = KmoMatrix::KmoMatrix__rect(KmoRect::KmoRect());
    assert_eq!(wide.getMaxRow(), 1048576);
    assert_eq!(wide.getMaxCol(), 16384);
}

#[test]
fn sg4_merge_copy_and_memory() {
    let mut src = KmoMatrix::KmoMatrix__sheet(
        true,
        Rc::new(RefCell::new(KmoRect::KmoRect())),
        SpreadsheetVersion::EXCEL97,
    );
    let mut mark = [0u8; 7];
    mark[3] = 3;
    src.set(0, 0, &mark);
    let mut dst = KmoMatrix::KmoMatrix__sheet(
        true,
        Rc::new(RefCell::new(KmoRect::KmoRect())),
        SpreadsheetVersion::EXCEL97,
    );
    let range = KmoRect::KmoRect__bounds(0, 0, 1, 1);
    dst.mergeCopy(10, 0, &range, &src);
    let mut got = [0u8; 7];
    assert!(dst.get(10, 0, &mut got));
    assert_eq!(got, mark);
    assert_eq!(rect_of(&dst), (10, 0, 11, 1));

    src.clearRowsMemory(0, 31);
    assert!(!src.isEmptyRowBlock(0));
    assert!(src.isEmptyColBlock(0, 0));
    src.set(0, 0, &mark);
    src.clearAllRowsMemory();
    assert!(src.getRowList().is_empty());
    assert!(!src.get(0, 0, &mut got));
}
