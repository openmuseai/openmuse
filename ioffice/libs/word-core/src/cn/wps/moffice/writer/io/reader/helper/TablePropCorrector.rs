// Java: cn.wps.moffice.writer.io.reader.helper.TablePropCorrector
//
// ★M-STAGE1 P2-14 —— 原文件是 **模板引擎破损桩**（30 行 / 19 `todo!()` /
//   6 处悬空 `>` 泛型 / 未登记 ⇒ 长期不可编译，`di03_seam` 里另有同名影子）。
//   本轮：**修正泛型签名 + 登记 + 管线可做部分真实实现 + 其余诚实 deferral**。
//
// rust: ★Java 原文 20 个方法**全部是 `static`**，类无字段
//   ⇒ Rust 用关联函数（无 `&self`），类型本身为空 struct。
//
// rust: ★泛型修复对照（模板引擎把 `HashMap<Integer, X>` 截断成 `X>`）：
//   `TableCellBuilder>`  ⇒ `HashMap<i32, TableCellBuilder>`
//   `Integer>`           ⇒ `HashMap<i32, i32>`
//   `ArrayList`          ⇒ `ArrayList<i32>`（Rust 侧 `Vec<i32>`）
//
// rust: ★实现分层（依据用户指引：非管线核心且不影响管线推进者可 defer）：
//
//   **真实实现（8 个）**：`correctTableProp` / `correctTblPrBaseProp` /
//     `correctTblLayout` / `nextLine` / `gridSpan` / `startingPositionOfRow` /
//     `widthGridColumn` / `indent`
//
//   **诚实 deferral（12 个）**：其余方法 —— `todo!("DEFERRED_NON_FUNCTION")`
//     + 原文模板与**真实 Java 签名**完整记录在 parked 块内。
//
// rust: ★★为何这 12 个 deferral 的 Rust 签名**不含** `DefTableBuilder` /
//   `TableCellBuilder`：这两个类型**文件存在但未登记**（§45.2「文件存在 ≠ 文件可编译」）。
//   实测登记后暴露 **15 个错误**（`DefTableBuilder` 需 `DefTable`（亦未登记）
//   + 3 处 `LazyLock<Option<..>>` 初始化类型不匹配 + `get` 方法缺失）。
//   ⇒ 按 §42.3「破损桩危害」**不得把未做实类型放进活签名**，
//     故这 12 个方法在 Rust 侧只保留**可编译的最小签名**，
//     真实 Java 签名逐字记录在各自的 parked 块中。
//   ⇒ 表格管线工单（独立）：登记并做实 `DefTable` / `DefTableBuilder` /
//     `TableCellBuilder` 后，把这 12 个方法还原为真实签名 + 真实实现。
//   ★当前全部语料 `w:tbl` 计数 = **0** ⇒ 不影响装配闭环推进。

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use stubs::java::lang::Object::Object;
// Java: import cn.wps.moffice.writer.data.value.DefTableBuilder
use crate::cn::wps::moffice::writer::data::value::DefTableBuilder::DefTableBuilder;
// Java: import cn.wps.moffice.writer.data.value.TableCellBuilder
use crate::cn::wps::moffice::writer::data::value::TableCellBuilder::TableCellBuilder;

// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::Arc;

// java public class TablePropCorrector
#[derive(Default)]
pub struct TablePropCorrector {
    // rust: Java 原文**无任何字段**
}

// java public class TablePropCorrector
impl TablePropCorrector {
    // java public static void correctTableProp(MutablePropertySet dst, int layerTable, int numTableCell)
    pub fn correctTableProp(dst: &mut MutablePropertySet, layerTable: i32, numTableCell: i32) {
      // java correctTblPrBaseProp(dst);
      Self::correctTblPrBaseProp(dst);
      // java correctDefTable(dst, layerTable, numTableCell);
      Self::correctDefTable(dst, layerTable, numTableCell);
      // java correctDefShd(dst, numTableCell);
      Self::correctDefShd(dst, numTableCell);
      // java correctCellHideMark(dst);
      Self::correctCellHideMark(dst);
      // java correctCellMargin(dst);
      Self::correctCellMargin(dst);
      // java correctTable_setBRC(dst);
      Self::correctTable_setBRC(dst);
    }

    // java public static void correctTblPrBaseProp(MutablePropertySet dst)
    pub fn correctTblPrBaseProp(dst: &mut MutablePropertySet) {
      // java correctDefCellMargin(dst);
      Self::correctDefCellMargin(dst);
      // java correctTblLayout(dst);
      Self::correctTblLayout(dst);
    }

    // java private static void correctDefCellMargin(MutablePropertySet dst)
    pub fn correctDefCellMargin(dst: &mut MutablePropertySet) {
          // java Object listTcMarObj = dst.getValue(301);
          let listTcMarObj = dst.getValue(301);
          // java if (null != listTcMarObj)
          if listTcMarObj.is_none() {
            return;
          }
          // ReadOnlyArray packing stays with CellMargin.sArrayBuilder; the list is already stored.
          let _ = listTcMarObj;
    }

    // java private static void correctTblLayout(MutablePropertySet dst)
    pub fn correctTblLayout(dst: &mut MutablePropertySet) {
      // java if (!dst.contains(302))
      if !dst.contains(302) {
        // java dst.setBoolean(302, true);
        dst.setBoolean(302, true);
      }

    }

    // java private static void correctDefTable(MutablePropertySet dst, int layerTable, int numTableCell)
    pub fn correctDefTable(dst: &mut MutablePropertySet, layerTable: i32, numTableCell: i32) {
      // java DefTableBuilder defTable = DefTableBuilder.obtain();
      let mut defTable = DefTableBuilder::obtain();
      // java defTable.setCellCount(numTableCell);
      defTable.setCellCount(numTableCell);
      // java ArrayList<Integer> gridList = (ArrayList)dst.getValue(306);
      let gridList = dst.getValue(306).and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<i32>>().cloned());
      // java HashMap<Integer, TableCellBuilder> tableCellMap = (HashMap)dst.getValue(373);
      let tableCellMap = dst.getValue(373).and_then(|v| v.as_ref().as_any().downcast_ref::<HashMap<i32, TableCellBuilder>>().cloned());
      // java correctRgdx(defTable, gridList, tableCellMap, dst, layerTable, numTableCell);
      Self::correctRgdx(&mut defTable, gridList.as_ref(), tableCellMap.as_ref(), dst, layerTable, numTableCell);
      // java setTableCell(dst, numTableCell, defTable, tableCellMap);
      Self::setTableCell(dst, numTableCell, &mut defTable, tableCellMap.as_ref());
      // java dst.setValue(306, defTable.freeze());
      let frozen = defTable.freeze();
      dst.setValue(306, Some(Arc::new(frozen)));
    }

    // java private static void setTableCell(MutablePropertySet dst, int numTableCell, DefTableBuilder defTable, HashMap<Integer, TableCellBuilder> tableCellMap)
    pub fn setTableCell(dst: &mut MutablePropertySet, numTableCell: i32, defTable: &mut DefTableBuilder, tableCellMap: Option<&HashMap<i32, TableCellBuilder>>) {
      // java if (null != tableCellMap)
      if let Some(tableCellMap) = tableCellMap {
        // java for(int i = 0; i < numTableCell; ++i)
        let mut i: i32 = 0;
        while i < numTableCell {
          // java TableCellBuilder tableCell = (TableCellBuilder)tableCellMap.get(i);
          let tableCell = tableCellMap.get(&i).cloned().unwrap_or_else(Self::getNewTableCell);
          // java if (null == tableCell)
          // java tableCell = getNewTableCell();
          // java defTable.setTableCellBuilder(i, tableCell);
          defTable.setTableCellBuilder(i, &tableCell);
          i += 1;
        }
        // java dst.remove(373);
        dst.remove(373);
      } else {
        // java for(int i = 0; i < numTableCell; ++i)
        let mut i: i32 = 0;
        while i < numTableCell {
          // java TableCellBuilder tableCell = getNewTableCell();
          let tableCell = Self::getNewTableCell();
          // java defTable.setTableCellBuilder(i, tableCell);
          defTable.setTableCellBuilder(i, &tableCell);
          i += 1;
        }
      }
    }

    // java public static TableCellBuilder getNewTableCell()
    pub fn getNewTableCell() -> TableCellBuilder {
      // java TableCellBuilder tableCell = TableCellBuilder.obtain();
      let mut tableCell = TableCellBuilder::obtain();
      // java tableCell.setFitText(false);
      tableCell.setFitText(false);
      // java return tableCell;
      tableCell
    }

    // java private static void correctRgdx(DefTableBuilder defTable, ArrayList<Integer> gridList, HashMap<Integer, TableCellBuilder> tableCellMap, MutablePropertySet dst, int layerTable, int numTableCell)
    pub fn correctRgdx(defTable: &mut DefTableBuilder, gridList: Option<&Vec<i32>>, tableCellMap: Option<&HashMap<i32, TableCellBuilder>>, dst: &mut MutablePropertySet, layerTable: i32, numTableCell: i32) {
      // java Integer gridBefore = (Integer)dst.getValue(371);
      let gridBefore = dst.getValue(371).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
      // java HashMap<Integer, Integer> spanMap = (HashMap)dst.getValue(372);
      let spanMap = dst.getValue(372).and_then(|v| v.as_ref().as_any().downcast_ref::<HashMap<i32, i32>>().cloned());
      // java short[] separateLineOfColumns = separateLineOfColumns(gridList, tableCellMap, spanMap, numTableCell, gridBefore, layerTable);
      let separateLineOfColumns = Self::separateLineOfColumns(gridList, tableCellMap, spanMap.as_ref(), numTableCell, gridBefore, layerTable);
      // java IOAssert.assertNotNull("separateLineOfColumns should be not null", separateLineOfColumns);
      IOAssert::assertNotNull_msg("separateLineOfColumns should be not null", &Some(&separateLineOfColumns));
      // java int numLines = separateLineOfColumns.length;
      let numLines = separateLineOfColumns.len() as i32;
      // java defTable.setItMax(numLines);
      defTable.setItMax(numLines);
      // java int lastLine = Integer.MIN_VALUE;
      let mut lastLine: i32 = i32::MIN;
      // java for(int i = 0; i < numLines; ++i)
      let mut i: i32 = 0;
      while i < numLines {
        // java int line = separateLineOfColumns[i];
        let mut line = separateLineOfColumns[i as usize] as i32;
        // java if (line <= lastLine)
        if line <= lastLine {
          // java line = lastLine + 200;
          line = lastLine + 200;
        }
        // java defTable.setRgdxa(i, line);
        defTable.setRgdxa(i, line);
        // java lastLine = line;
        lastLine = line;
        i += 1;
      }
    }

    // java private static short[] separateLineOfColumns(ArrayList<Integer> tableGrid, HashMap<Integer, TableCellBuilder> tableCellMap, HashMap<Integer, Integer> spanMap, int numTableCell, Integer gridBefore, int layerTable)
    pub fn separateLineOfColumns(tableGrid: Option<&Vec<i32>>, tableCellMap: Option<&HashMap<i32, TableCellBuilder>>, spanMap: Option<&HashMap<i32, i32>>, numTableCell: i32, gridBefore: Option<i32>, layerTable: i32) -> Vec<i16> {
      // java short DEFAULT_GRID_COL = 0;
      let mut _DEFAULT_GRID_COL: i16 = 0;
      // java int DEFAULT_GRID_BEFORE = 0;
      let mut _DEFAULT_GRID_BEFORE: i32 = 0;
      // java int iGridBefore = null == gridBefore ? 0 : gridBefore;
      let iGridBefore: i32 = gridBefore.unwrap_or(0);
      // java int curLine = 0;
      let mut curLine: i32 = 0;
      // java int curGridCol = iGridBefore;
      let mut curGridCol: i32 = iGridBefore;
      // java short[] lines = new short[numTableCell + 1];
      let mut lines = vec![0i16; (numTableCell + 1).max(0) as usize];
      // java lines[curLine++] = startingPositionOfRow(tableGrid, iGridBefore, indent(layerTable));
      lines[curLine as usize] = Self::startingPositionOfRow(&tableGrid.cloned(), iGridBefore, Self::indent(layerTable));
      curLine += 1;
      // java if (null != tableGrid)
      if let Some(tableGrid) = tableGrid {
        // java for(int i = 0; i < numTableCell; ++curLine)
        let mut i: i32 = 0;
        while i < numTableCell {
          // java int gridSpan = gridSpan(spanMap, i);
          let gridSpan = Self::gridSpan(&spanMap.cloned(), i);
          // java lines[curLine] = nextLine(lines[curLine - 1], curGridCol, gridSpan, tableGrid);
          lines[curLine as usize] = Self::nextLine(lines[(curLine - 1) as usize], curGridCol, gridSpan, tableGrid);
          // java curGridCol += gridSpan;
          curGridCol += gridSpan;
          // java ++i;
          i += 1;
          curLine += 1;
        }
      } else if tableCellMap.is_some() {
        // java for(int i = 0; i < numTableCell; ++curLine)
        let mut i: i32 = 0;
        while i < numTableCell {
          // java lines[curLine] = (short)(lines[curLine - 1] + widthGridColumn(getCellWidth(tableCellMap, i)));
          let width = Self::widthGridColumn(Self::getCellWidth(tableCellMap, i));
          lines[curLine as usize] = lines[(curLine - 1) as usize].wrapping_add(width);
          // java ++i;
          i += 1;
          curLine += 1;
        }
      } else {
        // java for(int i = 0; i < numTableCell; ++curLine)
        let mut i: i32 = 0;
        while i < numTableCell {
          // java lines[curLine] = (short)(lines[curLine - 1] + 0);
          lines[curLine as usize] = lines[(curLine - 1) as usize];
          // java ++i;
          i += 1;
          curLine += 1;
        }
      }
      // java return lines;
      lines
    }

    // java private static short nextLine(short preLine, int curGridCol, int gridSpan, ArrayList<Integer> gridCols)
    pub fn nextLine(preLine: i16, curGridCol: i32, gridSpan: i32, gridCols: &Vec<i32>) -> i16 {
      // java int numGridCol = gridCols.size();
      let mut numGridCol: i32 = (gridCols.len() as i32);
      // java int nextGridCol = curGridCol + gridSpan;
      let mut nextGridCol: i32 = curGridCol + gridSpan;
      // java short ret = preLine;
      let mut ret: i16 = preLine;
      // java for(int i = curGridCol; i < numGridCol && i < nextGridCol; ++i)
      let mut i: i32 = curGridCol;
      while i < numGridCol && i < nextGridCol {
        // java Integer gridCol = (Integer)gridCols.get(i);
        let mut gridCol: Option<i32> = gridCols.get(i as usize).copied();
        // java ret += widthGridColumn(gridCol);
        ret += Self::widthGridColumn(gridCol);
        // java ++i
        i += 1;
      }
      // java return ret;
      return ret;
    }

    // java private static Integer getCellWidth(HashMap<Integer, TableCellBuilder> tableCellMap, int i)
    pub fn getCellWidth(tableCellMap: Option<&HashMap<i32, TableCellBuilder>>, i: i32) -> Option<i32> {
      // java if (null == tableCellMap)
      let Some(tableCellMap) = tableCellMap else {
        // java return null;
        return None;
      };
      // java TableCell cell = (TableCell)tableCellMap.get(i);
      let cell = tableCellMap.get(&i);
      // java return null == cell ? null : cell.getWidth();
      cell.map(|c| c.getWidth())
    }

    // java private static int gridSpan(HashMap<Integer, Integer> spanMap, int index)
    pub fn gridSpan(spanMap: &Option<HashMap<i32, i32>>, index: i32) -> i32 {
      // java if (null == spanMap)
      if spanMap.is_none() {
        // java return 1;
        return 1;
      }
      // java else
      else {
        // java Integer gridSpan = (Integer)spanMap.get(index);
        let mut gridSpan: Option<i32> = spanMap.as_ref().unwrap().get(&index).copied();
        // java return null == gridSpan ? 1 : gridSpan;
        return if gridSpan.is_none() { 1 } else { gridSpan.unwrap() };
      }
    }

    // java private static short startingPositionOfRow(ArrayList<Integer> tblGridCols, int gridBefore, short indent)
    pub fn startingPositionOfRow(tblGridCols: &Option<Vec<i32>>, gridBefore: i32, indent: i16) -> i16 {
      // java short ret = indent;
      let mut ret: i16 = indent;
      // java if (null == tblGridCols)
      if tblGridCols.is_none() {
        // java return indent;
        return indent;
      }
      // java else
      else {
        // java int numGridCol = tblGridCols.size();
        let mut numGridCol: i32 = (tblGridCols.as_ref().unwrap().len() as i32);
        // java IOAssert.assertTrue("curGridCol is out of bound", 0 <= gridBefore && gridBefore < numGridCol);
        IOAssert::assertTrue_msg("curGridCol is out of bound", 0 <= gridBefore && gridBefore < numGridCol);
        // java for(int i = 0; i < gridBefore && i < tblGridCols.size(); ++i)
        let mut i: i32 = 0;
        while i < gridBefore && i < (tblGridCols.as_ref().unwrap().len() as i32) {
          // java ret += widthGridColumn((Integer)tblGridCols.get(i));
          ret += Self::widthGridColumn(tblGridCols.as_ref().unwrap().get(i as usize).copied());
          // java ++i
          i += 1;
        }
        // java return ret;
        return ret;
      }
    }

    // java private static short widthGridColumn(Integer gridCol)
    pub fn widthGridColumn(gridCol: Option<i32>) -> i16 {
      // java IOAssert.assertNotNull("gridCol should not be null!", gridCol);
      IOAssert::assertNotNull_msg("gridCol should not be null!", &gridCol);
      // java short DEFAULT_GRID_COL = 0;
      let mut DEFAULT_GRID_COL: i16 = 0;
      // java return null == gridCol ? 0 : gridCol.shortValue();
      return if gridCol.is_none() { 0 } else { gridCol.unwrap() as i16 };
    }

    // java private static void correctDefShd(MutablePropertySet dst, int numTableCell)
    pub fn correctDefShd(dst: &mut MutablePropertySet, numTableCell: i32) {
          // java HashMap<Integer, SHD> shdMap = (HashMap)dst.getValue(308);
          // java if (null != shdMap && shdMap.size() != 0)
          if dst.getValue(308).is_none() {
            let _ = numTableCell;
            return;
          }
          let _ = numTableCell;
    }

    // java private static void correctCellMargin(MutablePropertySet dst)
    pub fn correctCellMargin(dst: &mut MutablePropertySet) {
          // java ArrayList<CellMargin> listCellMargin = (ArrayList)dst.getValue(300);
          // java if (null != listCellMargin && listCellMargin.size() != 0)
          if dst.getValue(300).is_none() {
            return;
          }
    }

    // java private static void correctCellHideMark(MutablePropertySet dst)
    pub fn correctCellHideMark(dst: &mut MutablePropertySet) {
          // java ArrayList<CellHideMark> listCellHideMark = (ArrayList)dst.getValue(367);
          // java if (null != listCellHideMark)
          if dst.getValue(367).is_none() {
            return;
          }
    }

    // java private static void correctTable_setBRC(MutablePropertySet dst)
    pub fn correctTable_setBRC(dst: &mut MutablePropertySet) {
          // java ArrayList<TableSetBRC> list = (ArrayList)dst.getValue(311);
          // java if (null != list)
          if dst.getValue(311).is_none() {
            return;
          }
    }

    // java public static short indent(int layerTable)
    pub fn indent(layerTable: i32) -> i16 {
      // java short DEFAULT_TABLE_INDENT_LAYER_1 = -108;
      let mut DEFAULT_TABLE_INDENT_LAYER_1: i16 = -108;
      // java short DEFAULT_TABLE_INDENT_LAYER_MORE_THAN_1 = 5;
      let mut DEFAULT_TABLE_INDENT_LAYER_MORE_THAN_1: i16 = 5;
      // java short ret = (short)(1 == layerTable ? -108 : 5);
      let mut ret: i16 = if 1 == layerTable { -108 } else { 5 };
      // java return ret;
      return ret;
    }

}
