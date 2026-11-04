// rust: ★T7 —— `data/table/TableTool.rs` 中**两个自包含**的合并单元格查询 helper。
//   拆分理由：`data::table::TableTool` 的结构体持有 `mPropertyTool: Option<PropertyTool>`
//   字段，整体登记会经 `core::PropertyTool` 拉入 `core::shape` 闭包（实测 24+ 错误）；
//   而 `core::table::DefTableFixer` 只需要下面这两个 `DefTable` 维度的静态 helper ——
//   它们只依赖 `DefTable`/`TableCell`/`Math`，不触碰 `PropertyTool`/`shape`。
//   Java 原位置：`cn.wps.moffice.writer.data.TableTool`。
//
// Java: import cn.wps.moffice.writer.data.value.DefTable
use crate::cn::wps::moffice::writer::data::value::DefTable::DefTable;
// Java: import cn.wps.moffice.writer.data.value.TableCell
use crate::cn::wps::moffice::writer::data::value::TableCell::TableCell;
// Java: import java.lang.Math
use stubs::java::lang::Math::*;

pub struct TableToolMerge;

impl TableToolMerge {
    pub fn getNextMergeCell__deftable_int_deftable(thisRow: &DefTable, cellIndex: i32, nextRow: Option<&DefTable>) -> Option<i32> {
      // java if (null == nextRow)
      if nextRow.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java float thisLeft = (float)thisRow.getRgdxa(cellIndex);
        let mut thisLeft: f32 = ((thisRow.getRgdxa(cellIndex)) as f32);
        // java float thisRight = (float)thisRow.getRgdxa(cellIndex + 1);
        let mut thisRight: f32 = ((thisRow.getRgdxa(cellIndex + 1)) as f32);
        // java int meetLastCellIndex = -1;
        let mut meetLastCellIndex: i32 = -1;
        let mut curIndex: i32 = nextRow.getCellCount() - 1;
        // java for(int curIndex = nextRow.getCellCount() - 1; curIndex >= 0 --curIndex)
        while curIndex >= 0 {
          // java TableCell rawCell = nextRow.getTableCell(curIndex);
          let mut rawCell: TableCell = nextRow.getTableCell(curIndex);
          // java if (null != rawCell)
          if rawCell.is_some() {
            // java int mergeCellType = rawCell.getVertMerge();
            let mut mergeCellType: i32 = rawCell.getVertMerge();
            // java if (1 == mergeCellType)
            if 1 == mergeCellType {
              // java float leftDiff = Math.abs((float)nextRow.getRgdxa(curIndex) - thisLeft);
              let mut leftDiff: f32 = Math::abs(((nextRow.getRgdxa(curIndex)) as f32) - thisLeft);
              // java float rightDiff = Math.abs((float)nextRow.getRgdxa(curIndex + 1) - thisRight);
              let mut rightDiff: f32 = Math::abs(((nextRow.getRgdxa(curIndex + 1)) as f32) - thisRight);
              // java if (leftDiff < 6.0F && rightDiff < 6.0F && (-1 == meetLastCellIndex || leftDiff <= Math.abs((float)nextRow.getRgdxa(meetLastCellIndex) - thisLeft) && rightDiff <= Math.abs((float)nextRow.getRgdxa(meetLastCellIndex + 1) - thisRight)))
              if leftDiff < 6 && rightDiff < 6 && -1 == meetLastCellIndex || leftDiff <= Math::abs(((nextRow.getRgdxa(meetLastCellIndex)) as f32) - thisLeft) && rightDiff <= Math::abs(((nextRow.getRgdxa(meetLastCellIndex + 1)) as f32) - thisRight) {
                // java meetLastCellIndex = curIndex;
                meetLastCellIndex = curIndex;
              }
            }
          }
        }
        // java return -1 == meetLastCellIndex ? null : meetLastCellIndex;
        return (if -1 == meetLastCellIndex { None } else { Some(meetLastCellIndex) });
      }
    }

    pub fn getPrevMergeCell__deftable_int_deftable(thisRow: &DefTable, cellIndex: i32, prevRow: Option<&DefTable>) -> Option<i32> {
      // java if (null == prevRow)
      if prevRow.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java float thisLeft = (float)thisRow.getRgdxa(cellIndex);
        let mut thisLeft: f32 = ((thisRow.getRgdxa(cellIndex)) as f32);
        // java float thisRight = (float)thisRow.getRgdxa(cellIndex + 1);
        let mut thisRight: f32 = ((thisRow.getRgdxa(cellIndex + 1)) as f32);
        // java int meetLastCellIndex = -1;
        let mut meetLastCellIndex: i32 = -1;
        let mut curIndex: i32 = prevRow.getCellCount() - 1;
        // java for(int curIndex = prevRow.getCellCount() - 1; curIndex >= 0 --curIndex)
        while curIndex >= 0 {
          // java TableCell rawCell = prevRow.getTableCell(curIndex);
          let mut rawCell: TableCell = prevRow.getTableCell(curIndex);
          // java if (null != rawCell)
          if rawCell.is_some() {
            // java int mergeCellType = rawCell.getVertMerge();
            let mut mergeCellType: i32 = rawCell.getVertMerge();
            // java if (1 == mergeCellType || 3 == mergeCellType)
            if 1 == mergeCellType || 3 == mergeCellType {
              // java float leftDiff = Math.abs((float)prevRow.getRgdxa(curIndex) - thisLeft);
              let mut leftDiff: f32 = Math::abs(((prevRow.getRgdxa(curIndex)) as f32) - thisLeft);
              // java float rightDiff = Math.abs((float)prevRow.getRgdxa(curIndex + 1) - thisRight);
              let mut rightDiff: f32 = Math::abs(((prevRow.getRgdxa(curIndex + 1)) as f32) - thisRight);
              // java if (leftDiff < 6.0F && rightDiff < 6.0F && (-1 == meetLastCellIndex || leftDiff <= Math.abs((float)prevRow.getRgdxa(meetLastCellIndex) - thisLeft) && rightDiff <= Math.abs((float)prevRow.getRgdxa(meetLastCellIndex + 1) - thisRight)))
              if leftDiff < 6 && rightDiff < 6 && -1 == meetLastCellIndex || leftDiff <= Math::abs(((prevRow.getRgdxa(meetLastCellIndex)) as f32) - thisLeft) && rightDiff <= Math::abs(((prevRow.getRgdxa(meetLastCellIndex + 1)) as f32) - thisRight) {
                // java meetLastCellIndex = curIndex;
                meetLastCellIndex = curIndex;
              }
            }
          }
        }
        // java return -1 == meetLastCellIndex ? null : meetLastCellIndex;
        return (if -1 == meetLastCellIndex { None } else { Some(meetLastCellIndex) });
      }
    }

    // java public static CellInfo getNextMergeCell(RowInfo row, float rgdxaLeft, float rgdxaRight)
}
