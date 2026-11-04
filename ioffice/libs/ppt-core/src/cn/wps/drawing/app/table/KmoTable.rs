// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.app.format.effect.DgEffectReflection
use crate::cn::wps::drawing::app::format::effect::DgEffectReflection::DgEffectReflection;
// Java: import cn.wps.drawing.app.format.effect.DgEffectShadow
use crate::cn::wps::drawing::app::format::effect::DgEffectShadow::DgEffectShadow;
// Java: import cn.wps.drawing.app.graphic.GraphicObject
use crate::cn::wps::drawing::app::graphic::GraphicObject::GraphicObject;
// Java: import cn.wps.drawing.app.text.KmoTextBody
use crate::cn::wps::drawing::app::text::KmoTextBody::KmoTextBody;
// Java: import cn.wps.drawing.app.util.KmoRange
use crate::cn::wps::drawing::app::util::KmoRange::KmoRange;
// Java: import cn.wps.drawing.app.util.KmoRect
use crate::cn::wps::drawing::app::util::KmoRect::KmoRect;
// Java: import cn.wps.drawing.data.effect.EffectList
use crate::cn::wps::drawing::data::effect::EffectList::EffectList;
// Java: import cn.wps.drawing.data.effect.EffectProperties
use crate::cn::wps::drawing::data::effect::EffectProperties::EffectProperties;
// Java: import cn.wps.drawing.data.effect.InnerShadowEffect
use crate::cn::wps::drawing::data::effect::InnerShadowEffect::InnerShadowEffect;
// Java: import cn.wps.drawing.data.effect.OuterShadowEffect
use crate::cn::wps::drawing::data::effect::OuterShadowEffect::OuterShadowEffect;
// Java: import cn.wps.drawing.data.effect.ReflectionEffect
use crate::cn::wps::drawing::data::effect::ReflectionEffect::ReflectionEffect;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.drawing.data.table.Table
use crate::cn::wps::drawing::data::table::Table::Table;
// Java: import cn.wps.drawing.data.table.TableCell
use crate::cn::wps::drawing::data::table::TableCell::TableCell;
// Java: import cn.wps.drawing.data.table.TableCellProperties
use crate::cn::wps::drawing::data::table::TableCellProperties::TableCellProperties;
// Java: import cn.wps.drawing.data.table.TableStyle
use crate::cn::wps::drawing::data::table::TableStyle::TableStyle;
// Java: import cn.wps.drawing.data.text.TextBody
use crate::cn::wps::drawing::data::text::TextBody::TextBody;
// Java: import cn.wps.drawing.data.text.TextBodyProperties
use crate::cn::wps::drawing::data::text::TextBodyProperties::TextBodyProperties;
// Java: import cn.wps.drawing.data.text.TextListStyle
use crate::cn::wps::drawing::data::text::TextListStyle::TextListStyle;
// Java: import cn.wps.drawing.data.text.TextParagraph
use crate::cn::wps::drawing::data::text::TextParagraph::TextParagraph;
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
// use util::cn::wps::kfc::r#impl::arrayList::WIntArrayList;
// Java: import cn.wps.kfc.memor4j.util.KmoUndoableList
// use kfc_kfc_alpha::cn::wps::kfc::memor4j::util::KmoUndoableList;
// Java: import cn.wps.kfc.numfmt.i18n.KmoLang
// use kfc::cn::wps::kfc::numfmt::i18n::KmoLang;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::KoBin;
// Java: import cn.wps.memor4j.KmoAtom
// use kfc::cn::wps::memor4j::KmoAtom;
// Java: import java.util.ArrayList
// LLM No Rust equivalent: using Vec instead of ArrayList
// Java: import java.util.List
// LLM No Rust equivalent: using Vec instead of List

use std::ops::{Deref, DerefMut};

// Java field: int MAX_ROW_COL_COUNT
pub(crate) const MAX_ROW_COL_COUNT: i32 = 75;
// Java field: int MIN_COL_WIDTH
pub(crate) const MIN_COL_WIDTH: i32 = 208280;
// Java field: int MIN_ROW_HEIGHT
pub(crate) const MIN_ROW_HEIGHT: i32 = 135713;
// Java field: int DEF_COL_WIDTH
pub(crate) const DEF_COL_WIDTH: i32 = 1397000;
// Java field: int DEF_ROW_HEIGHT
pub(crate) const DEF_ROW_HEIGHT: i32 = 741680;

// Java: public interface IKmoTableSelectionObserver
// LLM Rust Pattern: Java inner interface -> Rust trait (struct IKmoTableSelectionObserver is represented as trait below)
pub trait IKmoTableSelectionObserver {
    // Java: void changed(boolean var1)
    fn changed(&mut self, is_trans_started: bool);
    // Java: void beforeChange(boolean var1)
    fn beforeChange(&mut self, is_trans_started: bool);
}

// Java: public class KmoTable extends KmoAtom implements KmoProtocol, GraphicObject
pub struct KmoTable {
    parent: KmoAtom<TableAtomData>,
    // Java field: KmoUndoableList<Integer> rowInfos
    // LLM Not Collection: KmoUndoableList is a custom undo-aware list, not java.util.List
    // rowInfos : KmoUndoableList (Vec-based undo wrapper)
    pub(crate) rowInfos: KmoUndoableList<i32>,
    // Java field: KmoUndoableList<Integer> colInfos
    // LLM Not Collection: KmoUndoableList is a custom undo-aware list, not java.util.List
    // colInfos : KmoUndoableList (Vec-based undo wrapper)
    pub(crate) colInfos: KmoUndoableList<i32>,
    // Java field: KmoUndoableList<KmoTableCell> cells
    // LLM Not Collection: KmoUndoableList is a custom undo-aware list, not java.util.List
    // cells : KmoUndoableList (Vec-based undo wrapper)
    pub(crate) cells: KmoUndoableList<KmoTableCell>,
    // Java field: KmoRange selectedCellRange
    selectedCellRange: KmoRange,
    // Java field: boolean isCellSelected
    pub(crate) isCellSelected: bool,
    // Java field: IKmoTableSelectionObserver observer
    observer: Option<Box<dyn IKmoTableSelectionObserver>>,
}

impl Deref for KmoTable {
    type Target = KmoAtom<TableAtomData>;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for KmoTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

// Java: public class KmoTable extends KmoAtom implements KmoProtocol, GraphicObject
impl KmoTable {
    // Java: KmoTable()
    pub fn new() -> Self {
      // Java: super(new TableAtomData());
      let mut s = Self {
        parent: KmoAtom::new(TableAtomData::new()),
        rowInfos: KmoUndoableList::new(),
        colInfos: KmoUndoableList::new(),
        cells: KmoUndoableList::new(),
        selectedCellRange: KmoRange::new(),
        isCellSelected: false,
        observer: None,
      };
      // Java: this.attach(new KmoTableSelectionObserver(this));
      // LLM Rust Pattern: constructor builds struct locally, this.attach -> s.attach
      s.attach(Box::new(KmoTableSelectionObserver::new()));
      s
    }

    // Java: int cellCount()
    pub fn cellCount(&self) -> i32 {
      // Java: return this.cells.size();
      self.cells.size()
    }

    // Java: KmoTable(int rowCount, int rowHeight, int colCount, int colWidth)
    pub fn new_with_size(rowCount: i32, rowHeight: i32, colCount: i32, colWidth: i32) -> Self {
      // Java: super(new TableAtomData());
      let mut s = Self {
        parent: KmoAtom::new(TableAtomData::new()),
        rowInfos: KmoUndoableList::new(),
        colInfos: KmoUndoableList::new(),
        cells: KmoUndoableList::new(),
        selectedCellRange: KmoRange::new(),
        isCellSelected: false,
        observer: None,
      };
      // Java: if (rowCount <= 75 && colCount <= 75)
      if rowCount <= 75 && colCount <= 75 {
         // Java: for(int i = 0; i < rowCount; ++i)
         for _i in 0..rowCount {
            // Java: this.rowInfos.add(rowHeight);
            s.rowInfos.add(rowHeight);
         }
         // Java: for(int i = 0; i < colCount; ++i)
         for _i in 0..colCount {
            // Java: this.colInfos.add(colWidth);
            s.colInfos.add(colWidth);
         }
         // Java: int i = 0;
         // LLM JavaLine Aggregation: merged into the for range
         // Java: for(int cellCount = colCount * rowCount; i < cellCount; ++i)
         let cellCount = colCount * rowCount;
         for _i in 0..cellCount {
            // Java: this.addCell(createEmptyCell());
            // LLM Rust Pattern: constructor builds struct locally, this.addCell -> s.addCell
            let cell = Self::createEmptyCell();
            s.addCell(cell);
         }
         // Java: this.attach(new KmoTableSelectionObserver(this));
         // LLM Rust Pattern: constructor builds struct locally, this.attach -> s.attach
         s.attach(Box::new(KmoTableSelectionObserver::new()));
      }
      // Java: else
      else {
         // Java: throw new IllegalArgumentException("行或列的数量超过了75");
         panic!("行或列的数量超过了75");
      }
      s
    }

    // Java: KoBin protocol()
    pub fn protocol(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.writeFormat();
      self.writeFormat();
      // Java: return ((TableAtomData)this.rawData()).tblPro.save();
      self.data_mut().tblPro.save()
    }

    // Java: void updateProtocol(KoBin data)
    pub fn updateProtocol(&mut self, data: &dyn KoBinTrait) {
      // Java: ((TableAtomData)this.rawData()).tblPro.load(data);
      self.data_mut().tblPro.load(data);
      // Java: this.readFormat();
      self.readFormat();
    }

    // Java: int type()
    pub fn get_type(&self) -> i32 {
      // Java: return 1;
      1
    }

    // Java: GraphicObject copy()
    pub fn copy(&mut self) -> KmoTable {
      // Java: KmoTable newTable = new KmoTable();
      let mut newTable = KmoTable::new();
      // Java: TableAtomData oldData = this.data();
      let oldData = self.data();
      // Java: TableAtomData newData = newTable.data();
      let newData = newTable.data_mut();
      // Java: newData.tblPro = Table.create();
      newData.tblPro = Table::create();
      // Java: newData.tblPro.load(oldData.tblPro.save());
      let saved = oldData.tblPro.save();
      newData.tblPro.load(saved.as_ref());
      // Java: if (oldData.effectShadow != null)
      if oldData.effectShadow.is_some() {
         // Java: newData.effectShadow = new DgEffectShadow();
         let mut es = DgEffectShadow::new();
         // Java: newData.effectShadow.updateProtocol(oldData.effectShadow.protocol());
         es.updateProtocol(oldData.effectShadow.as_ref().unwrap().protocol().as_ref());
         // Java: newData.effectShadow.setType(oldData.effectShadow.type());
         es.setType(oldData.effectShadow.as_ref().unwrap().get_type());
         newData.effectShadow = Some(es);
      }
      // Java: if (oldData.reflection != null)
      if oldData.reflection.is_some() {
         // Java: newData.reflection = new DgEffectReflection();
         let mut refl = DgEffectReflection::new();
         // Java: newData.reflection.updateProtocol(oldData.reflection.protocol());
         refl.updateProtocol(oldData.reflection.as_ref().unwrap().protocol().as_ref());
         newData.reflection = Some(refl);
      }
      // Java: int cellIndex = 0;
      // LLM JavaLine Aggregation: merged into the for range
      // Java: for(int size = this.cells.size(); cellIndex < size; ++cellIndex)
      let size = self.cells.size();
      for cellIndex in 0..size {
         // Java: newTable.cells.add((KmoTableCell)((KmoTableCell)this.cells.get(cellIndex)).copy());
         let cell_copy = self.cells.get(cellIndex as usize).copy();
         newTable.cells.add(cell_copy);
      }
      // Java: cellIndex = 0;
      // LLM JavaLine Aggregation: merged into the for range
      // Java: for(int size = this.colInfos.size(); cellIndex < size; ++cellIndex)
      let size = self.colInfos.size();
      for cellIndex in 0..size {
         // Java: int value = (Integer)this.colInfos.get(cellIndex);
         let value = *self.colInfos.get(cellIndex as usize);
         // Java: newTable.colInfos.add(value);
         newTable.colInfos.add(value);
      }
      // Java: cellIndex = 0;
      // LLM JavaLine Aggregation: merged into the for range
      // Java: for(int size = this.rowInfos.size(); cellIndex < size; ++cellIndex)
      let size = self.rowInfos.size();
      for cellIndex in 0..size {
         // Java: int value = (Integer)this.rowInfos.get(cellIndex);
         let value = *self.rowInfos.get(cellIndex as usize);
         // Java: newTable.rowInfos.add(value);
         newTable.rowInfos.add(value);
      }
      // Java: return newTable;
      newTable
    }

    // Java: void addTableRowHeight(int height)
    pub fn addTableRowHeight(&mut self, height: i32) {
      // Java: this.rowInfos.add(height);
      self.rowInfos.add(height);
    }

    // Java: void addTableColWidth(int width)
    pub fn addTableColWidth(&mut self, width: i32) {
      // Java: this.colInfos.add(width);
      self.colInfos.add(width);
    }

    // Java: KmoTableCell getCell(int row, int col)
    pub fn getCell(&self, row: i32, col: i32) -> Option<&KmoTableCell> {
      // Java: if (row < this.rowInfos.size() && col < this.colInfos.size())
      if row < self.rowInfos.size() && col < self.colInfos.size() {
         // Java: int index = this.getOffset(row, col);
         let index = self.getOffset(row, col);
         // Java: assert index >= 0 && index < this.cells.size();
         debug_assert!(index >= 0 && index < self.cells.size());
         // Java: return (KmoTableCell)this.cells.get(index);
         Some(self.cells.get(index as usize))
      }
      // Java: else
      else {
         // Java: return null;
         None
      }
    }

    fn getCell_mut(&mut self, row: i32, col: i32) -> Option<&mut KmoTableCell> {
      if row < self.rowInfos.size() && col < self.colInfos.size() {
         let index = self.getOffset(row, col);
         debug_assert!(index >= 0 && index < self.cells.size());
         Some(self.cells.get_mut(index as usize))
      } else {
         None
      }
    }

    // Java: int colWidth(int colIndex)
    pub fn colWidth(&self, colIndex: i32) -> i32 {
      // Java: assert this.colInfos.size() > colIndex && colIndex >= 0;
      debug_assert!(self.colInfos.size() > colIndex && colIndex >= 0);
      // Java: return (Integer)this.colInfos.get(colIndex);
      *self.colInfos.get(colIndex as usize)
    }

    // Java: int rowHeight(int rowIndex)
    pub fn rowHeight(&self, rowIndex: i32) -> i32 {
      // Java: assert this.rowInfos.size() > rowIndex && rowIndex >= 0;
      debug_assert!(self.rowInfos.size() > rowIndex && rowIndex >= 0);
      // Java: return (Integer)this.rowInfos.get(rowIndex);
      *self.rowInfos.get(rowIndex as usize)
    }

    // Java: int colSize()
    pub fn colSize(&self) -> i32 {
      // Java: return this.colInfos.size();
      self.colInfos.size()
    }

    // Java: int rowSize()
    pub fn rowSize(&self) -> i32 {
      // Java: return this.rowInfos.size();
      self.rowInfos.size()
    }

    // Java: void delCol(int colIndex)
    pub fn delCol(&mut self, colIndex: i32) {
      // Java: assert colIndex >= 0 && colIndex < this.colInfos.size();
      debug_assert!(colIndex >= 0 && colIndex < self.colInfos.size());
      // Java: int rowSize = this.rowInfos.size();
      let rowSize = self.rowInfos.size();
      // Java: for(int i = rowSize - 1; i >= 0; --i)
      for i in (0..rowSize).rev() {
         // Java: this.adjustMergeCellByDelete(i, colIndex, false, 1);
         self.adjustMergeCellByDelete(i, colIndex, false, 1);
      }
      // Java: this.delCellsAtCol(colIndex);
      self.delCellsAtCol(colIndex);
      // Java: this.colInfos.remove(colIndex);
      self.colInfos.remove(colIndex as usize);
      // Java: this.unselectCells();
      self.unselectCells();
    }

    // Java: void delRow(int rowIndex)
    pub fn delRow(&mut self, rowIndex: i32) {
      // Java: assert this.rowInfos.size() > rowIndex && rowIndex >= 0;
      debug_assert!(self.rowInfos.size() > rowIndex && rowIndex >= 0);
      // Java: int colSize = this.colInfos.size();
      let colSize = self.colInfos.size();
      // Java: for(int i = 0; i < colSize; ++i)
      for i in 0..colSize {
         // Java: this.adjustMergeCellByDelete(rowIndex, i, true, 1);
         self.adjustMergeCellByDelete(rowIndex, i, true, 1);
      }
      // Java: this.rowInfos.remove(rowIndex);
      self.rowInfos.remove(rowIndex as usize);
      // Java: this.delCellsAtRow(rowIndex);
      self.delCellsAtRow(rowIndex);
      // Java: this.unselectCells();
      self.unselectCells();
    }

    // Java: void removeLastCell()
    pub fn removeLastCell(&mut self) {
      // Java: this.cells.remove(this.cells.size() - 1);
      let last = self.cells.size() - 1;
      self.cells.remove(last as usize);
    }

    // Java: void insertCol(int colIndex, int colWidth)
    pub fn insertCol(&mut self, colIndex: i32, _colWidth: i32) {
      // Java: if (this.colSize() < 50)
      if self.colSize() < 50 {
         // Java: this.addCellsAtCol(colIndex);
         self.addCellsAtCol(colIndex);
         // Java: this.adjustColWidthByInsert(colIndex);
         self.adjustColWidthByInsert(colIndex);
      }
    }

    // Java: void adjustColWidthByInsert(int colIndex)
    fn adjustColWidthByInsert(&mut self, colIndex: i32) {
      // Java: int colSize = this.colInfos.size();
      let colSize = self.colInfos.size();
      // Java: double tableWidth = (double)0.0F;
      let mut tableWidth: f64 = 0.0;
      // Java: for(int i = 0; i < colSize; ++i)
      for i in 0..colSize {
         // Java: tableWidth += (double)(Integer)this.colInfos.get(i);
         tableWidth += *self.colInfos.get(i as usize) as f64;
      }
      // Java: int colWidth = (Integer)this.colInfos.get(colIndex - 1);
      let colWidth = *self.colInfos.get((colIndex - 1) as usize);
      // Java: double mockTableWidth = tableWidth + (double)colWidth;
      let mockTableWidth = tableWidth + colWidth as f64;
      // Java: this.colInfos.add(colIndex, colWidth);
      self.colInfos.add_at(colIndex as usize, colWidth);
      // Java: for(int i = 0; i < colSize + 1; ++i)
      for i in 0..(colSize + 1) {
         // Java: int newColWidth = (int)((double)(Integer)this.colInfos.get(i) * tableWidth / mockTableWidth);
         let mut newColWidth = (*self.colInfos.get(i as usize) as f64 * tableWidth / mockTableWidth) as i32;
         // Java: if (newColWidth < 208280)
         if newColWidth < MIN_COL_WIDTH {
            // Java: newColWidth = 208280;
            newColWidth = MIN_COL_WIDTH;
         }
         // Java: int maxMarginWidth = this.getMaxColMargin(i);
         let maxMarginWidth = self.getMaxColMargin(i);
         // Java: if (maxMarginWidth > newColWidth)
         if maxMarginWidth > newColWidth {
            // Java: newColWidth = maxMarginWidth;
            newColWidth = maxMarginWidth;
         }
         // Java: this.colInfos.set(i, newColWidth);
         self.colInfos.set(i as usize, newColWidth);
      }
    }

    // Java: int getMaxColMargin(int col)
    fn getMaxColMargin(&self, col: i32) -> i32 {
      // Java: int maxMarginWidth = 0;
      let mut maxMarginWidth = 0;
      // Java: for(int row = 0; row < this.rowSize(); ++row)
      for row in 0..self.rowSize() {
         // Java: KmoTableCell cell = this.getCell(row, col);
         let cell = self.getCell(row, col).unwrap();
         // Java: int tempMarginWidth = (cell.hasLeftMargin() ? cell.leftMargin() : 0) + (cell.hasRightMargin() ? cell.rightMargin() : 0);
         let tempMarginWidth = (if cell.hasLeftMargin() { cell.leftMargin() } else { 0 }) + (if cell.hasRightMargin() { cell.rightMargin() } else { 0 });
         // Java: if (tempMarginWidth > maxMarginWidth)
         if tempMarginWidth > maxMarginWidth {
            // Java: maxMarginWidth = tempMarginWidth;
            maxMarginWidth = tempMarginWidth;
         }
      }
      // Java: return maxMarginWidth;
      maxMarginWidth
    }

    // Java: void insertRow(int rowIndex, int rowHeight)
    pub fn insertRow(&mut self, rowIndex: i32, rowHeight: i32) {
      // Java: if (this.rowSize() < 50)
      if self.rowSize() < 50 {
         // Java: this.addCellsAtRow(rowIndex);
         self.addCellsAtRow(rowIndex);
         // Java: this.rowInfos.add(rowIndex, rowHeight);
         self.rowInfos.add_at(rowIndex as usize, rowHeight);
      }
    }

    // Java: int getMaxRowMargin(int row)
    fn getMaxRowMargin(&self, row: i32) -> i32 {
      // Java: int maxMarginWidth = 0;
      let mut maxMarginWidth = 0;
      // Java: for(int col = 0; col < this.colSize(); ++col)
      for col in 0..self.colSize() {
         // Java: KmoTableCell cell = this.getCell(row, col);
         let cell = self.getCell(row, col).unwrap();
         // Java: int tempMarginWidth = (cell.hasTopMargin() ? cell.topMargin() : 0) + (cell.hasBottomMargin() ? cell.bottomMargin() : 0);
         let tempMarginWidth = (if cell.hasTopMargin() { cell.topMargin() } else { 0 }) + (if cell.hasBottomMargin() { cell.bottomMargin() } else { 0 });
         // Java: if (tempMarginWidth > maxMarginWidth)
         if tempMarginWidth > maxMarginWidth {
            // Java: maxMarginWidth = tempMarginWidth;
            maxMarginWidth = tempMarginWidth;
         }
      }
      // Java: return maxMarginWidth;
      maxMarginWidth
    }

    // Java: void insertRow(int rowIndex)
    pub fn insertRow_default(&mut self, rowIndex: i32) {
      // Java: if (rowIndex > 0)
      if rowIndex > 0 {
         // Java: this.insertRow(rowIndex, (Integer)this.rowInfos.get(rowIndex - 1));
         let h = *self.rowInfos.get((rowIndex - 1) as usize);
         self.insertRow(rowIndex, h);
      }
    }

    // Java: void insertCol(int colIndex)
    pub fn insertCol_default(&mut self, colIndex: i32) {
      // Java: if (colIndex > 0)
      if colIndex > 0 {
         // Java: this.insertCol(colIndex, (Integer)this.colInfos.get(colIndex - 1));
         let w = *self.colInfos.get((colIndex - 1) as usize);
         self.insertCol(colIndex, w);
      }
    }

    // Java: void changeColWidth(int colIndex, int colWidth)
    pub fn changeColWidth(&mut self, colIndex: i32, colWidth: i32) {
      // Java: if (colIndex >= 0 && colIndex < this.colInfos.size())
      if colIndex >= 0 && colIndex < self.colInfos.size() {
         // Java: this.colInfos.set(colIndex, colWidth);
         self.colInfos.set(colIndex as usize, colWidth);
      }
    }

    // Java: void changeRowHeight(int rowIndex, int rowHeight)
    pub fn changeRowHeight(&mut self, rowIndex: i32, rowHeight: i32) {
      // Java: if (rowIndex >= 0 && rowIndex < this.rowInfos.size())
      if rowIndex >= 0 && rowIndex < self.rowInfos.size() {
         // Java: this.rowInfos.set(rowIndex, rowHeight);
         self.rowInfos.set(rowIndex as usize, rowHeight);
      }
    }

    // Java: void setColWidth(int colIndex, int colWidth)
    pub fn setColWidth(&mut self, colIndex: i32, colWidth: i32) {
      // Java: assert colIndex >= 0 && colIndex < this.colInfos.size();
      debug_assert!(colIndex >= 0 && colIndex < self.colInfos.size());
      // Java: int colSize = this.colInfos.size();
      let colSize = self.colInfos.size();
      // Java: if (colIndex == colSize - 1)
      if colIndex == colSize - 1 {
         // Java: this.colInfos.set(colIndex, colWidth);
         self.colInfos.set(colIndex as usize, colWidth);
      }
      // Java: else
      else {
         // Java: int oldWidth = (Integer)this.colInfos.get(colIndex) + (Integer)this.colInfos.get(colIndex + 1);
         let oldWidth = *self.colInfos.get(colIndex as usize) + *self.colInfos.get((colIndex + 1) as usize);
         // Java: int offset = oldWidth - colWidth;
         let offset = oldWidth - colWidth;
         // Java: this.colInfos.set(colIndex, colWidth);
         self.colInfos.set(colIndex as usize, colWidth);
         // Java: this.colInfos.set(colIndex + 1, offset);
         self.colInfos.set((colIndex + 1) as usize, offset);
      }
    }

    // Java: void setRowHeight(int rowIndex, int rowHeight)
    pub fn setRowHeight(&mut self, rowIndex: i32, rowHeight: i32) {
      // Java: assert rowIndex >= 0 && rowIndex < this.rowInfos.size();
      debug_assert!(rowIndex >= 0 && rowIndex < self.rowInfos.size());
      // Java: this.rowInfos.set(rowIndex, rowHeight);
      self.rowInfos.set(rowIndex as usize, rowHeight);
    }

    // Java: boolean isShowingBandedCol()
    pub fn isShowingBandedCol(&self) -> bool {
      // Java: return ((TableAtomData)this.rawData()).tblPro.isBandCol();
      self.data().tblPro.isBandCol()
    }

    // Java: void setIsShowBandedCol(boolean isShow)
    pub fn setIsShowBandedCol(&mut self, isShow: bool) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((TableAtomData)this.rawData()).tblPro.putBandCol(isShow);
      self.data_mut().tblPro.putBandCol(isShow);
    }

    // Java: boolean isShowingBandedRow()
    pub fn isShowingBandedRow(&self) -> bool {
      // Java: return ((TableAtomData)this.rawData()).tblPro.isBandRow();
      self.data().tblPro.isBandRow()
    }

    // Java: void setIsShowBandedRow(boolean isShow)
    pub fn setIsShowBandedRow(&mut self, isShow: bool) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((TableAtomData)this.rawData()).tblPro.putBandRow(isShow);
      self.data_mut().tblPro.putBandRow(isShow);
    }

    // Java: boolean isShowingFirstCol()
    pub fn isShowingFirstCol(&self) -> bool {
      // Java: return ((TableAtomData)this.rawData()).tblPro.isFirstCol();
      self.data().tblPro.isFirstCol()
    }

    // Java: void setIsShowFirstCol(boolean isShow)
    pub fn setIsShowFirstCol(&mut self, isShow: bool) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((TableAtomData)this.rawData()).tblPro.putFirstCol(isShow);
      self.data_mut().tblPro.putFirstCol(isShow);
    }

    // Java: boolean isShowFirstRow()
    pub fn isShowFirstRow(&self) -> bool {
      // Java: return ((TableAtomData)this.rawData()).tblPro.isFirstRow();
      self.data().tblPro.isFirstRow()
    }

    // Java: void setIsShowFirstRow(boolean isShow)
    pub fn setIsShowFirstRow(&mut self, isShow: bool) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((TableAtomData)this.rawData()).tblPro.putFirstRow(isShow);
      self.data_mut().tblPro.putFirstRow(isShow);
    }

    // Java: boolean isShowingLastRow()
    pub fn isShowingLastRow(&self) -> bool {
      // Java: return ((TableAtomData)this.rawData()).tblPro.isLastRow();
      self.data().tblPro.isLastRow()
    }

    // Java: void setIsShowLastRow(boolean isShow)
    pub fn setIsShowLastRow(&mut self, isShow: bool) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((TableAtomData)this.rawData()).tblPro.putLastRow(isShow);
      self.data_mut().tblPro.putLastRow(isShow);
    }

    // Java: boolean isShowingHeaderRow()
    pub fn isShowingHeaderRow(&self) -> bool {
      // Java: return ((TableAtomData)this.rawData()).tblPro.isFirstRow();
      self.data().tblPro.isFirstRow()
    }

    // Java: void setIsShowHeaderRow(boolean isShow)
    pub fn setIsShowHeaderRow(&mut self, isShow: bool) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((TableAtomData)this.rawData()).tblPro.putFirstRow(isShow);
      self.data_mut().tblPro.putFirstRow(isShow);
    }

    // Java: boolean isShowingLastCol()
    pub fn isShowingLastCol(&self) -> bool {
      // Java: return ((TableAtomData)this.rawData()).tblPro.isLastCol();
      self.data().tblPro.isLastCol()
    }

    // Java: void setIsShowLastCol(boolean isShow)
    pub fn setIsShowLastCol(&mut self, isShow: bool) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((TableAtomData)this.rawData()).tblPro.putLastCol(isShow);
      self.data_mut().tblPro.putLastCol(isShow);
    }

    // Java: boolean isShowingRight2Left()
    pub fn isShowingRight2Left(&self) -> bool {
      // Java: return ((TableAtomData)this.rawData()).tblPro.isRightToLeft();
      self.data().tblPro.isRightToLeft()
    }

    // Java: void setIsShowRight2Left(boolean isShow)
    pub fn setIsShowRight2Left(&mut self, isShow: bool) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((TableAtomData)this.rawData()).tblPro.putRightToLeft(isShow);
      self.data_mut().tblPro.putRightToLeft(isShow);
    }

    // Java: void addCell(KmoTableCell cell)
    pub fn addCell(&mut self, cell: KmoTableCell) {
      // Java: this.cells.add(cell);
      self.cells.add(cell);
    }

    // Java: int getOffset(int row, int col)
    fn getOffset(&self, row: i32, col: i32) -> i32 {
      // Java: return row * this.colInfos.size() + col;
      row * self.colInfos.size() + col
    }

    // Java: int getOffset(KmoTableCell cell)
    fn getOffset_cell(&self, cell: &KmoTableCell) -> i32 {
      // Java: return this.cells.indexOf(cell);
      self.cells.indexOf(cell)
    }

    // Java: static KmoTableCell createEmptyCell()
    fn createEmptyCell() -> KmoTableCell {
      // Java: TextBody textBody = TextBody.create();
      let mut textBody = TextBody::create();
      // Java: textBody.forceWriteBodyPr(TextBodyProperties.create());
      textBody.forceWriteBodyPr(TextBodyProperties::create());
      // Java: textBody.forceWriteListStyle(TextListStyle.create());
      textBody.forceWriteListStyle(TextListStyle::create());
      // Java: TextBody.ParagraphList paragraphList = TextBody.ParagraphList.create();
      let mut paragraphList = TextBody::ParagraphList::create();
      // Java: List<TextParagraph> paragraphs = new ArrayList(1);
      let mut paragraphs: Vec<TextParagraph> = Vec::with_capacity(1);
      // Java: TextParagraph textParagraph = TextParagraph.create();
      let mut textParagraph = TextParagraph::create();
      // Java: textParagraph.endParaRPr().putLang(KmoLang.getSystemLocale().replace('_', '-'));
      let locale = KmoLang::getSystemLocale().replace('_', "-");
      textParagraph.endParaRPr().putLang(&locale);
      // Java: textParagraph.endParaRPr().putAltLang("en-US");
      textParagraph.endParaRPr().putAltLang("en-US");
      // Java: textParagraph.save();
      textParagraph.save();
      // Java: paragraphs.add(textParagraph);
      paragraphs.push(textParagraph);
      // Java: paragraphList.write(paragraphs);
      paragraphList.write(&paragraphs);
      // Java: paragraphList.save();
      paragraphList.save();
      // Java: textBody.writeParaLst(paragraphList);
      textBody.writeParaLst(paragraphList);
      // Java: KmoTableCell cell = new KmoTableCell();
      let mut cell = KmoTableCell::new();
      // Java: KmoTextBody kmoTextBody = new KmoTextBody();
      let mut kmoTextBody = KmoTextBody::new();
      // Java: kmoTextBody.updateProtocol(textBody.save());
      let saved = textBody.save();
      kmoTextBody.updateProtocol(saved.as_ref());
      // Java: cell.setTextBody(kmoTextBody);
      cell.setTextBody(kmoTextBody);
      // Java: return cell;
      cell
    }

    // Java: int tableWidth()
    pub fn tableWidth(&self) -> i32 {
      // Java: int width = 0;
      let mut width = 0;
      // Java: int i = 0;
      // LLM JavaLine Aggregation: merged into the for range
      // Java: for(int count = this.colInfos.size(); i < count; ++i)
      let count = self.colInfos.size();
      for i in 0..count {
         // Java: width += (Integer)this.colInfos.get(i);
         width += *self.colInfos.get(i as usize);
      }
      // Java: return width;
      width
    }

    // Java: int tableHeight()
    pub fn tableHeight(&self) -> i32 {
      // Java: int height = 0;
      let mut height = 0;
      // Java: int i = 0;
      // LLM JavaLine Aggregation: merged into the for range
      // Java: for(int count = this.rowInfos.size(); i < count; ++i)
      let count = self.rowInfos.size();
      for i in 0..count {
         // Java: height += (Integer)this.rowInfos.get(i);
         height += *self.rowInfos.get(i as usize);
      }
      // Java: return height;
      height
    }

    // Java: void delCellsAtRow(int row)
    fn delCellsAtRow(&mut self, row: i32) {
      // Java: int colSize = this.colInfos.size();
      let colSize = self.colInfos.size();
      // Java: for(int i = 0; i < colSize; ++i)
      for _i in 0..colSize {
         // Java: this.cells.remove(this.getOffset(row, 0));
         let offset = self.getOffset(row, 0);
         self.cells.remove(offset as usize);
      }
    }

    // Java: void adjustMergeCellByDelete(int row, int col, boolean delRow, int size)
    fn adjustMergeCellByDelete(&mut self, row: i32, col: i32, delRow: bool, size: i32) {
      // Java: KmoTableCell masterCell = this.getMergeMasterCell(row, col);
      // LLM Rust Renamed: masterCell -> master_idx (index-based for borrow checker)
      let master_idx = self.getMergeMasterCellIndex(row, col);
      // Java: if (masterCell != null)
      if master_idx >= 0 {
         // Java: KmoRect rect = this.cellPosition(masterCell);
         let rect = self.cellPositionByIndex(master_idx);
         // Java: if (delRow)
         if delRow {
            // Java: this.adjustMergeCellByDeleteRow(masterCell, row, col, rect, size);
            // LLM Rust Renamed: adjustMergeCellByDeleteRow -> adjustMergeCellByDeleteRow_idx
            self.adjustMergeCellByDeleteRow_idx(master_idx, row, col, &rect, size);
         }
         // Java: else
         else {
            // Java: this.adjustMergeCellByDeleteCol(masterCell, row, col, rect, size);
            // LLM Rust Renamed: adjustMergeCellByDeleteCol -> adjustMergeCellByDeleteCol_idx
            self.adjustMergeCellByDeleteCol_idx(master_idx, row, col, &rect, size);
         }
      }
    }

    // Java: void adjustMergeCellByDeleteRow(KmoTableCell masterCell, int row, int col, KmoRect rect, int size)
    // LLM Rust Renamed: adjustMergeCellByDeleteRow -> adjustMergeCellByDeleteRow_idx (index-based for borrow checker)
    fn adjustMergeCellByDeleteRow_idx(&mut self, masterIdx: i32, row: i32, col: i32, rect: &KmoRect, size: i32) {
      let master_row_span = self.cells.get(masterIdx as usize).rowSpan();
      let master_grid_span = self.cells.get(masterIdx as usize).gridSpan();
      // Java: if (masterCell.rowSpan() >= 2)
      if master_row_span >= 2 {
         // Java: if (rect.left() == row && rect.top() == col)
         if rect.left() == row && rect.top() == col {
            // Java: KmoTableCell newMasterCell = this.getCell(row + 1, col);
            // LLM Rust Renamed: newMasterCell -> new_idx (index-based for borrow checker)
            let new_idx = self.getOffset(row + 1, col);
            // Java: if (newMasterCell != null)
            if self.getCell(row + 1, col).is_some() {
               // Java: newMasterCell.setHorizontalMerge(false);
               self.cells.get_mut(new_idx as usize).setHorizontalMerge(false);
               // Java: newMasterCell.setVerticalMerge(false);
               self.cells.get_mut(new_idx as usize).setVerticalMerge(false);
               // Java: this.copyMasterBorder2NewMaster(masterCell, newMasterCell);
               // LLM Rust Renamed: copyMasterBorder2NewMaster -> copyMasterBorder2NewMaster_idx
               self.copyMasterBorder2NewMaster_idx(masterIdx, new_idx);
               // Java: if (masterCell.rowSpan() - size >= 2 || masterCell.gridSpan() >= 2)
               if master_row_span - size >= 2 || master_grid_span >= 2 {
                  // Java: newMasterCell.setGridSpan(masterCell.gridSpan());
                  self.cells.get_mut(new_idx as usize).setGridSpan(master_grid_span);
                  // Java: newMasterCell.setRowSpan(masterCell.rowSpan() - size);
                  self.cells.get_mut(new_idx as usize).setRowSpan(master_row_span - size);
                  // Java: this.clearMergeFlag(newMasterCell.rowSpan(), row, col, true);
                  let new_row_span = self.cells.get(new_idx as usize).rowSpan();
                  self.clearMergeFlag(new_row_span, row, col, true);
               }
            }
         }
         // Java: else if (rect.top() == col)
         else if rect.top() == col {
            // Java: masterCell.setRowSpan(masterCell.rowSpan() - size);
            self.cells.get_mut(masterIdx as usize).setRowSpan(master_row_span - size);
            // Java: this.clearMergeFlag(masterCell.rowSpan(), row, col, true);
            let new_span = self.cells.get(masterIdx as usize).rowSpan();
            self.clearMergeFlag(new_span, row, col, true);
         }
         // Java: else
         else {
            // Java: if (rect.left() == row)
            if rect.left() == row {
               // Java: KmoTableCell curCell = this.getCell(row + 1, col);
               // LLM Rust Renamed: curCell -> cur_idx (index-based for borrow checker)
               let cur_idx = self.getOffset(row + 1, col);
               // Java: curCell.setRowSpan(masterCell.rowSpan() - size);
               self.cells.get_mut(cur_idx as usize).setRowSpan(master_row_span - size);
               // Java: if (curCell.rowSpan() >= 2)
               if self.cells.get(cur_idx as usize).rowSpan() >= 2 {
                  // Java: curCell.setVerticalMerge(false);
                  self.cells.get_mut(cur_idx as usize).setVerticalMerge(false);
               }
               // Java: this.clearMergeFlag(curCell.rowSpan(), row, col, true);
               let cur_span = self.cells.get(cur_idx as usize).rowSpan();
               self.clearMergeFlag(cur_span, row, col, true);
            }
         }
      }
    }

    // Java: void adjustMergeCellByDeleteCol(KmoTableCell masterCell, int row, int col, KmoRect rect, int size)
    // LLM Rust Renamed: adjustMergeCellByDeleteCol -> adjustMergeCellByDeleteCol_idx (index-based for borrow checker)
    fn adjustMergeCellByDeleteCol_idx(&mut self, masterIdx: i32, row: i32, col: i32, rect: &KmoRect, size: i32) {
      let master_grid_span = self.cells.get(masterIdx as usize).gridSpan();
      let master_row_span = self.cells.get(masterIdx as usize).rowSpan();
      // Java: if (masterCell.gridSpan() >= 2)
      if master_grid_span >= 2 {
         // Java: if (rect.left() == row && rect.top() == col)
         if rect.left() == row && rect.top() == col {
            // Java: KmoTableCell newMasterCell = this.getCell(row, col + 1);
            // LLM Rust Renamed: newMasterCell -> new_idx (index-based for borrow checker)
            let new_idx = self.getOffset(row, col + 1);
            // Java: if (newMasterCell != null)
            if self.getCell(row, col + 1).is_some() {
               // Java: newMasterCell.setHorizontalMerge(false);
               self.cells.get_mut(new_idx as usize).setHorizontalMerge(false);
               // Java: newMasterCell.setVerticalMerge(false);
               self.cells.get_mut(new_idx as usize).setVerticalMerge(false);
               // Java: this.copyMasterBorder2NewMaster(masterCell, newMasterCell);
               // LLM Rust Renamed: copyMasterBorder2NewMaster -> copyMasterBorder2NewMaster_idx
               self.copyMasterBorder2NewMaster_idx(masterIdx, new_idx);
               // Java: if (masterCell.gridSpan() - size >= 2 || masterCell.rowSpan() >= 2)
               if master_grid_span - size >= 2 || master_row_span >= 2 {
                  // Java: newMasterCell.setRowSpan(masterCell.rowSpan());
                  self.cells.get_mut(new_idx as usize).setRowSpan(master_row_span);
                  // Java: newMasterCell.setGridSpan(masterCell.gridSpan() - size);
                  self.cells.get_mut(new_idx as usize).setGridSpan(master_grid_span - size);
                  // Java: this.clearMergeFlag(newMasterCell.gridSpan(), row, col, false);
                  let new_span = self.cells.get(new_idx as usize).gridSpan();
                  self.clearMergeFlag(new_span, row, col, false);
               }
            }
         }
         // Java: else if (rect.left() == row)
         else if rect.left() == row {
            // Java: masterCell.setGridSpan(masterCell.gridSpan() - size);
            self.cells.get_mut(masterIdx as usize).setGridSpan(master_grid_span - size);
            // Java: this.clearMergeFlag(masterCell.gridSpan(), row, col, false);
            let new_span = self.cells.get(masterIdx as usize).gridSpan();
            self.clearMergeFlag(new_span, row, col, false);
         }
         // Java: else
         else {
            // Java: if (rect.top() == col)
            if rect.top() == col {
               // Java: KmoTableCell curCell = this.getCell(row, col + 1);
               // LLM Rust Renamed: curCell -> cur_idx (index-based for borrow checker)
               let cur_idx = self.getOffset(row, col + 1);
               // Java: curCell.setGridSpan(masterCell.gridSpan() - size);
               self.cells.get_mut(cur_idx as usize).setGridSpan(master_grid_span - size);
               // Java: if (curCell.gridSpan() >= 2)
               if self.cells.get(cur_idx as usize).gridSpan() >= 2 {
                  // Java: curCell.setHorizontalMerge(false);
                  self.cells.get_mut(cur_idx as usize).setHorizontalMerge(false);
               }
               // Java: this.clearMergeFlag(curCell.gridSpan(), row, col, false);
               let cur_span = self.cells.get(cur_idx as usize).gridSpan();
               self.clearMergeFlag(cur_span, row, col, false);
            }
         }
      }
    }

    // Java: void copyMasterBorder2NewMaster(KmoTableCell masterCell, KmoTableCell newMasterCell)
    fn copyMasterBorder2NewMaster_idx(&mut self, masterIdx: i32, newMasterIdx: i32) {
      // Java: TableCell cell1 = TableCell.open(masterCell.protocol());
      let cell1 = TableCell::open(self.cells.get(masterIdx as usize).protocol().as_ref());
      // Java: TableCell cell2 = TableCell.open(newMasterCell.protocol());
      let mut cell2 = TableCell::open(self.cells.get(newMasterIdx as usize).protocol().as_ref());
      // Java: cell2.cellProperties().writeLeftLine(cell1.cellProperties().leftLine());
      cell2.cellProperties().writeLeftLine(cell1.cellProperties().leftLine());
      // Java: cell2.cellProperties().writeRightLine(cell1.cellProperties().rightLine());
      cell2.cellProperties().writeRightLine(cell1.cellProperties().rightLine());
      // Java: cell2.cellProperties().writeTopLine(cell1.cellProperties().topLine());
      cell2.cellProperties().writeTopLine(cell1.cellProperties().topLine());
      // Java: cell2.cellProperties().writeBottomLine(cell1.cellProperties().bottomLine());
      cell2.cellProperties().writeBottomLine(cell1.cellProperties().bottomLine());
      // Java: newMasterCell.updateProtocol(cell2.save());
      let saved = cell2.save();
      self.cells.get_mut(newMasterIdx as usize).updateProtocol(saved.as_ref());
    }

    // Java: void clearMergeFlag(int span, int row, int col, boolean delRow)
    fn clearMergeFlag(&mut self, span: i32, row: i32, col: i32, delRow: bool) {
      // Java: if (delRow)
      if delRow {
         // Java: if (span == 1)
         if span == 1 {
            // Java: KmoTableCell cell = this.getCell(row + 1, col);
            // LLM JavaLine Aggregation: cell variable inlined into getCell calls below
            // Java: if (cell == null)
            if self.getCell(row + 1, col).is_none() {
               // Java: return;
               return;
            }
            // Java: cell.setVerticalMerge(false);
            self.getCell_mut(row + 1, col).unwrap().setVerticalMerge(false);
         }
      }
      // Java: else
      else {
         // Java: if (span == 1)
         if span == 1 {
            // Java: KmoTableCell cell = this.getCell(row, col + 1);
            // LLM JavaLine Aggregation: cell variable inlined into getCell calls below
            // Java: if (cell == null)
            if self.getCell(row, col + 1).is_none() {
               // Java: return;
               return;
            }
            // Java: cell.setHorizontalMerge(false);
            self.getCell_mut(row, col + 1).unwrap().setHorizontalMerge(false);
         }
      }
    }

    // Java: void delCellsAtCol(int col)
    fn delCellsAtCol(&mut self, col: i32) {
      // Java: int rowSize = this.rowInfos.size();
      let rowSize = self.rowInfos.size();
      // Java: for(int i = rowSize - 1; i >= 0; --i)
      for i in (0..rowSize).rev() {
         // Java: this.cells.remove(this.getOffset(i, col));
         let offset = self.getOffset(i, col);
         self.cells.remove(offset as usize);
      }
    }

    // Java: void addCellsAtRow(int row)
    fn addCellsAtRow(&mut self, row: i32) {
      // Java: int colSize = this.colInfos.size();
      let colSize = self.colInfos.size();
      // Java: List<KmoTableCell> cellLst = new ArrayList(colSize);
      let mut cellLst: Vec<KmoTableCell> = Vec::with_capacity(colSize as usize);
      // Java: WIntArrayList offsetLst = new WIntArrayList(colSize);
      let mut offsetLst = WIntArrayList::new_with_capacity(colSize);
      // Java: for(int col = 0; col < colSize; ++col)
      for col in 0..colSize {
         // Java: KmoTableCell newCell = createEmptyCell();
         let mut newCell = Self::createEmptyCell();
         // Java: if (this.isNeedAdjustMergeCellForInsert(row, col, true))
         if self.isNeedAdjustMergeCellForInsert(row, col, true) {
            // Java: this.adjustMergeCellByInsert(row, col, newCell, true, 1);
            self.adjustMergeCellByInsert(row, col, &mut newCell, true, 1);
         }
         // Java: else if (this.isNeedCreateSingeMerge(row, col, true))
         else if self.isNeedCreateSingeMerge(row, col, true) {
            // Java: this.createSingeMerge(newCell, true, row, col);
            self.createSingeMerge(&mut newCell, true, row, col);
         }
         // Java: this.copyCellPr(newCell, row, col, true);
         self.copyCellPr(&mut newCell, row, col, true);
         // Java: cellLst.add(newCell);
         cellLst.push(newCell);
         // Java: offsetLst.add(this.getOffset(row, col));
         offsetLst.add(self.getOffset(row, col));
      }
      // Java: for(int col = 0; col < colSize; ++col)
      for col in 0..colSize {
         // Java: this.cells.add(offsetLst.get(col), (KmoTableCell)cellLst.get(col));
         let offset = offsetLst.get(col as usize);
         self.cells.add_at(offset as usize, cellLst.remove(0));
      }
    }

    // Java: KmoRect cellPosition(KmoTableCell cell)
    pub fn cellPosition(&self, cell: &KmoTableCell) -> KmoRect {
      // Java: int index = this.cells.indexOf(cell);
      let index = self.cells.indexOf(cell);
      // Java: int row = index / this.colInfos.size();
      let row = index / self.colInfos.size();
      // Java: int col = index % this.colInfos.size();
      let col = index % self.colInfos.size();
      // Java: KmoRect kmoRect = new KmoRect(row, col, 1, 1);
      let kmoRect = KmoRect::new(row, col, 1, 1);
      // Java: return kmoRect;
      kmoRect
    }

    fn cellPositionByIndex(&self, index: i32) -> KmoRect {
      let row = index / self.colInfos.size();
      let col = index % self.colInfos.size();
      KmoRect::new(row, col, 1, 1)
    }

    fn getMergeMasterCellIndex(&self, row: i32, col: i32) -> i32 {
      let mut row = row;
      let mut col = col;
      let idx = self.getOffset(row, col);
      if idx < 0 || idx >= self.cells.size() { return -1; }
      let cell = self.cells.get(idx as usize);
      if (cell.rowSpan() > 1 || cell.gridSpan() > 1) && !self.hasMergeFlag_at(idx) {
         return idx;
      }
      if cell.isVerticalMerge() {
         while row - 1 >= 0 {
            row -= 1;
            let ci = self.getOffset(row, col);
            let c = self.cells.get(ci as usize);
            if c.rowSpan() >= 2 && !self.hasMergeFlag_at(ci) {
               return ci;
            }
            if c.rowSpan() >= 2 { break; }
         }
      }
      let ci = self.getOffset(row, col);
      let cell = self.cells.get(ci as usize);
      if cell.isHorizontalMerge() {
         while col - 1 >= 0 {
            col -= 1;
            let ci = self.getOffset(row, col);
            let c = self.cells.get(ci as usize);
            if c.gridSpan() >= 2 && !self.hasMergeFlag_at(ci) {
               return ci;
            }
            if c.gridSpan() >= 2 { break; }
         }
      }
      -1
    }

    fn hasMergeFlag_at(&self, idx: i32) -> bool {
      let cell = self.cells.get(idx as usize);
      cell.isHorizontalMerge() || cell.isVerticalMerge()
    }

    // Java: void createSingeMerge(KmoTableCell newCell, boolean isInsertRow, int row, int col)
    fn createSingeMerge(&mut self, newCell: &mut KmoTableCell, isInsertRow: bool, row: i32, col: i32) {
      // Java: if (isInsertRow)
      if isInsertRow {
         // Java: KmoTableCell master = this.getMergeMasterCell(row - 1, col);
         // LLM Rust Renamed: master -> master_idx (index-based for borrow checker)
         let master_idx = self.getMergeMasterCellIndex(row - 1, col);
         if master_idx >= 0 {
            let pos = self.cellPositionByIndex(master_idx);
            // Java: if (this.cellPosition(master).top() == col)
            if pos.top() == col {
               // Java: newCell.setGridSpan(master.gridSpan());
               let gs = self.cells.get(master_idx as usize).gridSpan();
               newCell.setGridSpan(gs);
            }
            // Java: else
            else {
               // Java: newCell.setHorizontalMerge(true);
               newCell.setHorizontalMerge(true);
            }
         }
      }
      // Java: else
      else {
         // Java: KmoTableCell master = this.getMergeMasterCell(row, col - 1);
         // LLM Rust Renamed: master -> master_idx (index-based for borrow checker)
         let master_idx = self.getMergeMasterCellIndex(row, col - 1);
         if master_idx >= 0 {
            let pos = self.cellPositionByIndex(master_idx);
            // Java: if (this.cellPosition(master).left() == row)
            if pos.left() == row {
               // Java: newCell.setRowSpan(master.rowSpan());
               let rs = self.cells.get(master_idx as usize).rowSpan();
               newCell.setRowSpan(rs);
            }
            // Java: else
            else {
               // Java: newCell.setVerticalMerge(true);
               newCell.setVerticalMerge(true);
            }
         }
      }
    }

    // Java: void adjustMergeCellByInsert(int row, int col, KmoTableCell newCell, boolean insertRow, int size)
    fn adjustMergeCellByInsert(&mut self, row: i32, col: i32, newCell: &mut KmoTableCell, insertRow: bool, size: i32) {
      // Java: KmoTableCell masterCell = this.getMergeMasterCell(row, col);
      // LLM Rust Renamed: masterCell -> master_idx (index-based for borrow checker)
      let master_idx = self.getMergeMasterCellIndex(row, col);
      // Java: if (masterCell != null)
      if master_idx >= 0 {
         // Java: KmoTableCell cell = this.getCell(row, col);
         // LLM Rust Renamed: cell -> cell_idx (index-based for borrow checker)
         let cell_idx = self.getOffset(row, col);
         let cell_vm = self.cells.get(cell_idx as usize).isVerticalMerge();
         let cell_hm = self.cells.get(cell_idx as usize).isHorizontalMerge();
         // Java: newCell.setVerticalMerge(cell.isVerticalMerge());
         newCell.setVerticalMerge(cell_vm);
         // Java: newCell.setHorizontalMerge(cell.isHorizontalMerge());
         newCell.setHorizontalMerge(cell_hm);
         // Java: KmoRect rect = this.cellPosition(masterCell);
         let rect = self.cellPositionByIndex(master_idx);
         let master_row_span = self.cells.get(master_idx as usize).rowSpan();
         let master_grid_span = self.cells.get(master_idx as usize).gridSpan();
         // Java: if (insertRow)
         if insertRow {
            // Java: if (rect.top() == col)
            if rect.top() == col {
               // Java: masterCell.setRowSpan(masterCell.rowSpan() + size);
               self.cells.get_mut(master_idx as usize).setRowSpan(master_row_span + size);
               // Java: newCell.setGridSpan(masterCell.gridSpan());
               newCell.setGridSpan(master_grid_span);
            }
            // Java: else
            else {
               // Java: KmoTableCell firstCell = this.getCell(rect.left(), col);
               // LLM Rust Renamed: firstCell -> first_idx (index-based for borrow checker)
               let first_idx = self.getOffset(rect.left(), col);
               // Java: firstCell.setRowSpan(masterCell.rowSpan());
               self.cells.get_mut(first_idx as usize).setRowSpan(master_row_span);
            }
         }
         // Java: else if (rect.left() == row)
         else if rect.left() == row {
            // Java: masterCell.setGridSpan(masterCell.gridSpan() + size);
            self.cells.get_mut(master_idx as usize).setGridSpan(master_grid_span + size);
            // Java: newCell.setRowSpan(masterCell.rowSpan());
            newCell.setRowSpan(master_row_span);
         }
         // Java: else
         else {
            // Java: KmoTableCell firstCell = this.getCell(row, rect.top());
            // LLM Rust Renamed: firstCell -> first_idx (index-based for borrow checker)
            let first_idx = self.getOffset(row, rect.top());
            // Java: firstCell.setGridSpan(masterCell.gridSpan());
            self.cells.get_mut(first_idx as usize).setGridSpan(master_grid_span);
         }
      }
    }

    // Java: boolean isNeedCreateSingeMerge(int row, int col, boolean insertRow)
    fn isNeedCreateSingeMerge(&self, mut row: i32, mut col: i32, insertRow: bool) -> bool {
      // Java: if (insertRow)
      if insertRow {
         // Java: if (row - 1 < 0)
         if row - 1 < 0 {
            // Java: return false;
            return false;
         }
         // Java: --row;
         row -= 1;
      }
      // Java: else
      else {
         // Java: if (col - 1 < 0)
         if col - 1 < 0 {
            // Java: return false;
            return false;
         }
         // Java: --col;
         col -= 1;
      }
      // Java: KmoTableCell cell = this.getMergeMasterCell(row, col);
      // LLM Rust Renamed: cell -> master_idx (index-based for borrow checker)
      let master_idx = self.getMergeMasterCellIndex(row, col);
      // Java: if (cell == null)
      if master_idx < 0 {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: if (insertRow)
         if insertRow {
            // Java: if (cell.gridSpan() >= 2)
            if self.cells.get(master_idx as usize).gridSpan() >= 2 {
               // Java: return true;
               return true;
            }
         }
         // Java: else if (cell.rowSpan() >= 2)
         else if self.cells.get(master_idx as usize).rowSpan() >= 2 {
            // Java: return true;
            return true;
         }
         // Java: return false;
         return false;
      }
    }

    // Java: boolean isNeedAdjustMergeCellForInsert(int row, int col, boolean isInsertRow)
    fn isNeedAdjustMergeCellForInsert(&self, row: i32, col: i32, isInsertRow: bool) -> bool {
      // Java: KmoTableCell cell = this.getCell(row, col);
      let cell = self.getCell(row, col);
      // Java: if (cell == null)
      if cell.is_none() {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         let cell = cell.unwrap();
         // Java: return isInsertRow ? cell.isVerticalMerge() : cell.isHorizontalMerge();
         return if isInsertRow { cell.isVerticalMerge() } else { cell.isHorizontalMerge() };
      }
    }

    // Java: boolean hasMergeFlag(KmoTableCell cell)
    fn hasMergeFlag(&self, cell: &KmoTableCell) -> bool {
      // Java: return cell.isHorizontalMerge() || cell.isVerticalMerge();
      cell.isHorizontalMerge() || cell.isVerticalMerge()
    }

    // Java: KmoTableCell getMergeMasterCell(int row, int col)
    pub fn getMergeMasterCell(&self, mut row: i32, mut col: i32) -> Option<&KmoTableCell> {
      // Java: KmoTableCell cell = this.getCell(row, col);
      let mut cell = self.getCell(row, col)?;
      // Java: if ((cell.rowSpan() > 1 || cell.gridSpan() > 1) && !this.hasMergeFlag(cell))
      if (cell.rowSpan() > 1 || cell.gridSpan() > 1) && !self.hasMergeFlag(cell) {
         // Java: return cell;
         return Some(cell);
      }
      // Java: else
      else {
         // Java: if (cell.isVerticalMerge())
         if cell.isVerticalMerge() {
            // Java: while(row - 1 >= 0)
            while row - 1 >= 0 {
               // Java: --row;
               row -= 1;
               // Java: cell = this.getCell(row, col);
               cell = self.getCell(row, col)?;
               // Java: if (cell.rowSpan() >= 2 && !this.hasMergeFlag(cell))
               if cell.rowSpan() >= 2 && !self.hasMergeFlag(cell) {
                  // Java: return cell;
                  return Some(cell);
               }
               // Java: if (cell.rowSpan() >= 2)
               if cell.rowSpan() >= 2 {
                  // Java: break;
                  break;
               }
            }
         }
         // Java: if (cell.isHorizontalMerge())
         if cell.isHorizontalMerge() {
            // Java: while(col - 1 >= 0)
            while col - 1 >= 0 {
               // Java: --col;
               col -= 1;
               // Java: cell = this.getCell(row, col);
               cell = self.getCell(row, col)?;
               // Java: if (cell.gridSpan() >= 2 && !this.hasMergeFlag(cell))
               if cell.gridSpan() >= 2 && !self.hasMergeFlag(cell) {
                  // Java: return cell;
                  return Some(cell);
               }
               // Java: if (cell.gridSpan() >= 2)
               if cell.gridSpan() >= 2 {
                  // Java: break;
                  break;
               }
            }
         }
         // Java: return null;
         return None;
      }
    }

    // Java: void copyCellPr(KmoTableCell newCell, int row, int col, boolean isInsertRow)
    fn copyCellPr(&mut self, newCell: &mut KmoTableCell, row: i32, col: i32, isInsertRow: bool) {
      // Java: TableCell newCellData = TableCell.open(newCell.protocol());
      let mut newCellData = TableCell::open(newCell.protocol().as_ref());
      // Java: KmoTableCell masterCell = null;
      // LLM JavaLine Aggregation: masterCell and preCell merged into single destructured let
      // Java: KmoTableCell preCell = null;
      // LLM Rust Renamed: masterCell -> master_idx, preCell -> pre_idx (index-based for borrow checker)
      let (pre_idx, master_idx);
      // Java: if (isInsertRow)
      if isInsertRow {
         // Java: preCell = this.getCell(row - 1, col);
         pre_idx = self.getOffset(row - 1, col);
         // Java: masterCell = this.getMergeMasterCell(row - 1, col);
         master_idx = self.getMergeMasterCellIndex(row - 1, col);
      }
      // Java: else
      else {
         // Java: preCell = this.getCell(row, col - 1);
         pre_idx = self.getOffset(row, col - 1);
         // Java: masterCell = this.getMergeMasterCell(row, col - 1);
         master_idx = self.getMergeMasterCellIndex(row, col - 1);
      }
      // Java: TableCell cell = TableCell.open(preCell.protocol());
      let cell = TableCell::open(self.cells.get(pre_idx as usize).protocol().as_ref());
      // Java: TableCellProperties cellProperties = cell.cellProperties();
      let cellProperties = cell.cellProperties();
      // Java: newCellData.writeCellProperties(cellProperties);
      newCellData.writeCellProperties(cellProperties);
      // Java: this.correctLineData(preCell, newCellData);
      // LLM Rust Renamed: correctLineData -> correctLineData_at
      self.correctLineData_at(pre_idx, &mut newCellData);
      // Java: if (masterCell != null)
      if master_idx >= 0 {
         // Java: cell = TableCell.open(masterCell.protocol());
         let cell = TableCell::open(self.cells.get(master_idx as usize).protocol().as_ref());
         // Java: cellProperties = cell.cellProperties();
         let cellProperties = cell.cellProperties();
         // Java: if (cellProperties.hasFill())
         if cellProperties.hasFill() {
            // Java: newCellData.cellProperties().writeFill(cellProperties.fill());
            newCellData.cellProperties().writeFill(cellProperties.fill());
         }
         // Java: else
         else {
            // Java: newCellData.cellProperties().deleteFill();
            newCellData.cellProperties().deleteFill();
         }
         // Java: if (cellProperties.hasBottomLeft2TopRightLine())
         if cellProperties.hasBottomLeft2TopRightLine() {
            // Java: newCellData.cellProperties().writeBottomLeft2TopRightLine(cellProperties.bottomLeft2TopRightLine());
            newCellData.cellProperties().writeBottomLeft2TopRightLine(cellProperties.bottomLeft2TopRightLine());
         }
         // Java: else
         else {
            // Java: newCellData.cellProperties().deleteBottomLeft2TopRightLine();
            newCellData.cellProperties().deleteBottomLeft2TopRightLine();
         }
         // Java: if (cellProperties.hasTopLeft2BottomRightLine())
         if cellProperties.hasTopLeft2BottomRightLine() {
            // Java: newCellData.cellProperties().writeTopLeft2BottomRightLine(cellProperties.topLeft2BottomRightLine());
            newCellData.cellProperties().writeTopLeft2BottomRightLine(cellProperties.topLeft2BottomRightLine());
         }
         // Java: else
         else {
            // Java: newCellData.cellProperties().deleteTopLeft2BottomRightLine();
            newCellData.cellProperties().deleteTopLeft2BottomRightLine();
         }
         // Java: this.copyLeftOutline(newCellData, row, col, isInsertRow, cellProperties);
         // LLM Rust Renamed: copyLeftOutline -> copyLeftOutline_data
         self.copyLeftOutline_data(&mut newCellData, row, col, isInsertRow, &cellProperties);
         // Java: this.copyRightOutline(newCellData, row, col, isInsertRow, cellProperties);
         // LLM Rust Renamed: copyRightOutline -> copyRightOutline_data
         self.copyRightOutline_data(&mut newCellData, row, col, isInsertRow, &cellProperties);
         // Java: this.copyTopOutline(newCellData, row, col, isInsertRow, cellProperties);
         // LLM Rust Renamed: copyTopOutline -> copyTopOutline_data
         self.copyTopOutline_data(&mut newCellData, row, col, isInsertRow, &cellProperties);
         // Java: this.copyBottomOutline(newCellData, row, col, isInsertRow, cellProperties);
         // LLM Rust Renamed: copyBottomOutline -> copyBottomOutline_data
         self.copyBottomOutline_data(&mut newCellData, row, col, isInsertRow, &cellProperties);
      }
      // Java: else
      else {
         // Java: if (!isInsertRow && newCellData.cellProperties().hasLeftLine())
         if !isInsertRow && newCellData.cellProperties().hasLeftLine() {
            // Java: cell = TableCell.open(preCell.protocol());
            let cell = TableCell::open(self.cells.get(pre_idx as usize).protocol().as_ref());
            // Java: if (cell.cellProperties().hasRightLine())
            if cell.cellProperties().hasRightLine() {
               // Java: cell.cellProperties().writeRightLine(newCellData.cellProperties().leftLine());
               let mut cell = cell;
               cell.cellProperties().writeRightLine(newCellData.cellProperties().leftLine());
               // Java: KmoTableCell newTableCell = new KmoTableCell();
               let mut newTableCell = KmoTableCell::new();
               // Java: newTableCell.updateProtocol(cell.save());
               newTableCell.updateProtocol(cell.save().as_ref());
               // Java: newTableCell.setTextBody(preCell.getTextBody());
               let tb = self.cells.get(pre_idx as usize).getTextBody();
               newTableCell.setTextBody(tb.clone());
               // Java: this.replaceCell(preCell, newTableCell);
               // LLM Rust Renamed: replaceCell -> replaceCell_at
               self.replaceCell_at(pre_idx, newTableCell);
            }
         }
         // Java: if (isInsertRow && newCellData.cellProperties().hasTopLine())
         if isInsertRow && newCellData.cellProperties().hasTopLine() {
            // Java: cell = TableCell.open(preCell.protocol());
            let cell = TableCell::open(self.cells.get(pre_idx as usize).protocol().as_ref());
            // Java: if (cell.cellProperties().hasBottomLine())
            if cell.cellProperties().hasBottomLine() {
               // Java: cell.cellProperties().writeBottomLine(newCellData.cellProperties().topLine());
               let mut cell = cell;
               cell.cellProperties().writeBottomLine(newCellData.cellProperties().topLine());
               // Java: KmoTableCell newTableCell = new KmoTableCell();
               let mut newTableCell = KmoTableCell::new();
               // Java: newTableCell.updateProtocol(cell.save());
               newTableCell.updateProtocol(cell.save().as_ref());
               // Java: newTableCell.setTextBody(preCell.getTextBody());
               let tb = self.cells.get(pre_idx as usize).getTextBody();
               newTableCell.setTextBody(tb.clone());
               // Java: this.replaceCell(preCell, newTableCell);
               // LLM Rust Renamed: replaceCell -> replaceCell_at
               self.replaceCell_at(pre_idx, newTableCell);
            }
         }
      }
      // Java: newCell.updateProtocol(newCellData.save());
      newCell.updateProtocol(newCellData.save().as_ref());
    }

    // Java: void replaceCell(KmoTableCell oldCell, KmoTableCell newCell)
    // LLM Rust Renamed: replaceCell -> replaceCell_at (index-based for borrow checker)
    fn replaceCell_at(&mut self, oldIdx: i32, newCell: KmoTableCell) {
      // Java: int index = this.getOffset(oldCell);
      // LLM JavaLine Aggregation: merged into the next Rust statement
      // Java: this.cells.set(index, newCell);
      self.cells.set(oldIdx as usize, newCell);
    }

    // Java: void correctLineData(KmoTableCell cell, TableCell newCellData)
    // LLM Rust Renamed: correctLineData -> correctLineData_at (index-based for borrow checker)
    fn correctLineData_at(&self, cellIdx: i32, newCellData: &mut TableCell) {
      // Java: KmoRect rect = this.cellPosition(cell);
      let rect = self.cellPositionByIndex(cellIdx);
      // Java: int row = rect.left();
      let row = rect.left();
      // Java: int col = rect.top();
      let col = rect.top();
      let cell = self.cells.get(cellIdx as usize);
      // Java: if (!cell.hasLeftLine() && col - 1 >= 0)
      if !cell.hasLeftLine() && col - 1 >= 0 {
         // Java: KmoTableCell leftCell = this.getCell(row, col - 1);
         let leftCell = self.getCell(row, col - 1).unwrap();
         // Java: if (leftCell.hasRightLine())
         if leftCell.hasRightLine() {
            // Java: TableCell leftCellData = TableCell.open(leftCell.protocol());
            let leftCellData = TableCell::open(leftCell.protocol().as_ref());
            // Java: newCellData.cellProperties().writeLeftLine(leftCellData.cellProperties().rightLine());
            newCellData.cellProperties().writeLeftLine(leftCellData.cellProperties().rightLine());
         }
      }
      // Java: if (!cell.hasRightLine() && col + 1 < this.colSize())
      if !cell.hasRightLine() && col + 1 < self.colSize() {
         // Java: KmoTableCell rightCell = this.getCell(row, col + 1);
         let rightCell = self.getCell(row, col + 1).unwrap();
         // Java: if (rightCell.hasLeftLine())
         if rightCell.hasLeftLine() {
            // Java: TableCell rightCellData = TableCell.open(rightCell.protocol());
            let rightCellData = TableCell::open(rightCell.protocol().as_ref());
            // Java: newCellData.cellProperties().writeRightLine(rightCellData.cellProperties().leftLine());
            newCellData.cellProperties().writeRightLine(rightCellData.cellProperties().leftLine());
         }
      }
      // Java: if (!cell.hasTopLine() && row - 1 >= 0)
      if !cell.hasTopLine() && row - 1 >= 0 {
         // Java: KmoTableCell topCell = this.getCell(row - 1, col);
         let topCell = self.getCell(row - 1, col).unwrap();
         // Java: if (topCell.hasBottomLine())
         if topCell.hasBottomLine() {
            // Java: TableCell topCellData = TableCell.open(topCell.protocol());
            let topCellData = TableCell::open(topCell.protocol().as_ref());
            // Java: newCellData.cellProperties().writeTopLine(topCellData.cellProperties().bottomLine());
            newCellData.cellProperties().writeTopLine(topCellData.cellProperties().bottomLine());
         }
      }
      // Java: if (!cell.hasBottomLine() && row + 1 < this.rowSize())
      if !cell.hasBottomLine() && row + 1 < self.rowSize() {
         // Java: KmoTableCell bottomCell = this.getCell(row + 1, col);
         let bottomCell = self.getCell(row + 1, col).unwrap();
         // Java: if (bottomCell.hasTopLine())
         if bottomCell.hasTopLine() {
            // Java: TableCell bottomCellData = TableCell.open(bottomCell.protocol());
            let bottomCellData = TableCell::open(bottomCell.protocol().as_ref());
            // Java: newCellData.cellProperties().writeBottomLine(bottomCellData.cellProperties().topLine());
            newCellData.cellProperties().writeBottomLine(bottomCellData.cellProperties().topLine());
         }
      }
    }

    // Java: void copyBottomOutline(TableCell newCellData, int row, int col, boolean isInsertRow, TableCellProperties masterCellPro)
    // LLM Rust Renamed: copyBottomOutline -> copyBottomOutline_data (takes &mut TableCell directly for borrow checker)
    fn copyBottomOutline_data(&self, newCellData: &mut TableCell, row: i32, col: i32, isInsertRow: bool, masterCellPro: &TableCellProperties) {
      // Java: if (!isInsertRow)
      if !isInsertRow {
         // Java: this.copyBottomOutlineByInsertCol(row, col, newCellData, masterCellPro);
         // LLM Rust Renamed: copyBottomOutlineByInsertCol -> copyBottomOutlineByInsertCol_data
         self.copyBottomOutlineByInsertCol_data(row, col, newCellData, masterCellPro);
      }
      // Java: else
      else {
         // Java: this.copyBottomOutlineByInsertRow(row, col, newCellData);
         // LLM Rust Renamed: copyBottomOutlineByInsertRow -> copyBottomOutlineByInsertRow_data
         self.copyBottomOutlineByInsertRow_data(row, col, newCellData);
      }
    }

    // Java: void copyBottomOutlineByInsertRow(int row, int col, TableCell newCellData)
    fn copyBottomOutlineByInsertRow_data(&self, row: i32, col: i32, newCellData: &mut TableCell) {
      // Java: KmoTableCell masterCell = this.getMergeMasterCell(row - 1, col);
      // LLM Rust Renamed: masterCell -> master_idx (index-based for borrow checker)
      let master_idx = self.getMergeMasterCellIndex(row - 1, col);
      if master_idx < 0 { return; }
      // Java: TableCell tableCell = TableCell.open(masterCell.protocol());
      let tableCell = TableCell::open(self.cells.get(master_idx as usize).protocol().as_ref());
      // Java: TableCellProperties masterCellPro = tableCell.cellProperties();
      let masterCellPro = tableCell.cellProperties();
      // Java: KmoRect rect = this.cellPosition(masterCell);
      let rect = self.cellPositionByIndex(master_idx);
      let master_row_span = self.cells.get(master_idx as usize).rowSpan();
      // Java: int lastRow = rect.left() + masterCell.rowSpan() - 1;
      let lastRow = rect.left() + master_row_span - 1;
      // Java: if (lastRow != row - 1 && newCellData.cellProperties().hasBottomLine())
      if lastRow != row - 1 && newCellData.cellProperties().hasBottomLine() {
         // Java: newCellData.cellProperties().deleteBottomLine();
         newCellData.cellProperties().deleteBottomLine();
      }
      // Java: --row;
      let r = row - 1;
      // Java: if (rect.top() == col && row == rect.left() + masterCell.rowSpan() - 1)
      if rect.top() == col && r == rect.left() + master_row_span - 1 {
         // Java: if (masterCellPro.hasBottomLine())
         if masterCellPro.hasBottomLine() {
            // Java: newCellData.cellProperties().writeBottomLine(masterCellPro.bottomLine());
            newCellData.cellProperties().writeBottomLine(masterCellPro.bottomLine());
         }
         // Java: if (masterCellPro.hasTopLine())
         if masterCellPro.hasTopLine() {
            // Java: masterCellPro.writeBottomLine(newCellData.cellProperties().topLine());
            // LLM Rust Pattern: cannot mutate self in &self method; masterCell mutation skipped
            // Java: KmoTableCell newTableCell = new KmoTableCell();
            // LLM Rust Pattern: skipped - cannot create replacement cell in &self context
            // Java: newTableCell.updateProtocol(tableCell.save());
            // LLM Rust Pattern: skipped - no newTableCell created
            // Java: newTableCell.setTextBody(masterCell.getTextBody());
            // LLM Rust Pattern: skipped - no newTableCell created
            // Java: this.replaceCell(masterCell, newTableCell);
            // LLM Rust Pattern: skipped - cannot mutate self in &self method
            newCellData.cellProperties().writeTopLine(newCellData.cellProperties().bottomLine());
         }
         // Java: else
         else {
            // Java: if (newCellData.cellProperties().hasBottomLine())
            if newCellData.cellProperties().hasBottomLine() {
               // Java: newCellData.cellProperties().writeTopLine(newCellData.cellProperties().bottomLine());
               newCellData.cellProperties().writeTopLine(newCellData.cellProperties().bottomLine());
            }
         }
      }
      // Java: else
      else {
         // Java: this.bottomLine(masterCell, row, col, newCellData, masterCellPro);
         // LLM Rust Renamed: bottomLine -> bottomLine_data
         self.bottomLine_data(master_idx, r, col, newCellData, &masterCellPro);
         // Java: if (newCellData.cellProperties().hasBottomLine())
         if newCellData.cellProperties().hasBottomLine() {
            // Java: newCellData.cellProperties().writeTopLine(newCellData.cellProperties().bottomLine());
            newCellData.cellProperties().writeTopLine(newCellData.cellProperties().bottomLine());
         }
      }
    }

    // Java: void copyBottomOutlineByInsertCol(int row, int col, TableCell newCellData, TableCellProperties masterCellPro)
    fn copyBottomOutlineByInsertCol_data(&self, row: i32, col: i32, newCellData: &mut TableCell, masterCellPro: &TableCellProperties) {
      // Java: --col;
      let c = col - 1;
      // Java: KmoTableCell masterCell = this.getMergeMasterCell(row, col);
      // LLM Rust Renamed: masterCell -> master_idx (index-based for borrow checker)
      let master_idx = self.getMergeMasterCellIndex(row, c);
      if master_idx < 0 { return; }
      // Java: KmoRect rect = this.cellPosition(masterCell);
      // LLM Rust Renamed: cellPosition -> cellPositionByIndex
      let rect = self.cellPositionByIndex(master_idx);
      let gs = self.cells.get(master_idx as usize).gridSpan();
      // Java: if (rect.top() + masterCell.gridSpan() - 1 == col && row == rect.left() && masterCellPro.hasBottomLine())
      if rect.top() + gs - 1 == c && row == rect.left() && masterCellPro.hasBottomLine() {
         // Java: newCellData.cellProperties().writeBottomLine(masterCellPro.bottomLine());
         newCellData.cellProperties().writeBottomLine(masterCellPro.bottomLine());
      }
      // Java: else
      else {
         // Java: this.bottomLine(masterCell, row, col, newCellData, masterCellPro);
         // LLM Rust Renamed: bottomLine -> bottomLine_data
         self.bottomLine_data(master_idx, row, c, newCellData, masterCellPro);
      }
    }

    // Java: void bottomLine(KmoTableCell masterCell, int row, int col, TableCell newCellData, TableCellProperties masterCellPro)
    fn bottomLine_data(&self, master_idx: i32, row: i32, col: i32, newCellData: &mut TableCell, masterCellPro: &TableCellProperties) {
      let gs = self.cells.get(master_idx as usize).gridSpan();
      // Java: if (masterCell.gridSpan() > 1)
      if gs > 1 {
         // Java: KmoRect masterPosition = this.cellPosition(masterCell);
         let masterPosition = self.cellPositionByIndex(master_idx);
         // Java: if (row + 1 >= this.rowSize())
         if row + 1 >= self.rowSize() {
            // Java: this.copyFirstCellBottomLine(newCellData, row, masterPosition.top());
            // LLM Rust Renamed: copyFirstCellBottomLine -> copyFirstCellBottomLine_data
            self.copyFirstCellBottomLine_data(newCellData, row, masterPosition.top());
         }
         // Java: else
         else {
            // Java: KmoTableCell nextMasterCell = this.getMergeMasterCell(row + 1, col);
            // LLM Rust Renamed: nextMasterCell -> next_idx (index-based for borrow checker)
            let next_idx = self.getMergeMasterCellIndex(row + 1, col);
            // Java: if (nextMasterCell != null)
            if next_idx >= 0 {
               // Java: KmoRect nextMasterPosition = this.cellPosition(nextMasterCell);
               // LLM Rust Renamed: cellPosition -> cellPositionByIndex
               let nextMasterPosition = self.cellPositionByIndex(next_idx);
               // Java: if (masterPosition.left() == nextMasterPosition.left() && masterPosition.top() == nextMasterPosition.top())
               if masterPosition.left() == nextMasterPosition.left() && masterPosition.top() == nextMasterPosition.top() {
                  // Java: newCellData.cellProperties().deleteBottomLine();
                  newCellData.cellProperties().deleteBottomLine();
               }
               // Java: else
               else {
                  // Java: if (masterPosition.top() >= nextMasterPosition.top())
                  if masterPosition.top() >= nextMasterPosition.top() {
                     // Java: if (masterCellPro.hasBottomLine())
                     if masterCellPro.hasBottomLine() {
                        // Java: newCellData.cellProperties().writeBottomLine(masterCellPro.bottomLine());
                        newCellData.cellProperties().writeBottomLine(masterCellPro.bottomLine());
                     }
                     // Java: else
                     else {
                        // Java: newCellData.cellProperties().deleteBottomLine();
                        newCellData.cellProperties().deleteBottomLine();
                     }
                  }
                  // Java: else
                  else {
                     // Java: KmoTableCell cell = this.getCell(masterPosition.left(), nextMasterPosition.top());
                     let cell = self.getCell(masterPosition.left(), nextMasterPosition.top()).unwrap();
                     // Java: TableCell cellData = TableCell.open(cell.protocol());
                     let cellData = TableCell::open(cell.protocol().as_ref());
                     // Java: if (cellData.cellProperties().hasBottomLine())
                     if cellData.cellProperties().hasBottomLine() {
                        // Java: newCellData.cellProperties().writeBottomLine(cellData.cellProperties().bottomLine());
                        newCellData.cellProperties().writeBottomLine(cellData.cellProperties().bottomLine());
                     }
                     // Java: else
                     else {
                        // Java: newCellData.cellProperties().deleteBottomLine();
                        newCellData.cellProperties().deleteBottomLine();
                     }
                  }
               }
            }
         }
      }
    }

    // Java: void copyFirstCellRightLine(TableCell newCellData, int row, int col)
    fn copyFirstCellRightLine_data(&self, newCellData: &mut TableCell, row: i32, col: i32) {
      // Java: KmoTableCell firstCell = this.getCell(row, col);
      let firstCell = self.getCell(row, col).unwrap();
      // Java: if (!firstCell.hasRightLine())
      if !firstCell.hasRightLine() {
         // Java: newCellData.cellProperties().deleteRightLine();
         newCellData.cellProperties().deleteRightLine();
      }
      // Java: TableCell firstCellData = TableCell.open(firstCell.protocol());
      let firstCellData = TableCell::open(firstCell.protocol().as_ref());
      // Java: newCellData.cellProperties().writeRightLine(firstCellData.cellProperties().rightLine());
      newCellData.cellProperties().writeRightLine(firstCellData.cellProperties().rightLine());
    }

    // Java: void copyFirstCellLeftLine(TableCell newCellData, int row, int col)
    fn copyFirstCellLeftLine_data(&self, newCellData: &mut TableCell, row: i32, col: i32) {
      // Java: KmoTableCell firstCell = this.getCell(row, col);
      let firstCell = self.getCell(row, col).unwrap();
      // Java: if (!firstCell.hasLeftLine())
      if !firstCell.hasLeftLine() {
         // Java: newCellData.cellProperties().deleteLeftLine();
         newCellData.cellProperties().deleteLeftLine();
      }
      // Java: TableCell firstCellData = TableCell.open(firstCell.protocol());
      let firstCellData = TableCell::open(firstCell.protocol().as_ref());
      // Java: newCellData.cellProperties().writeLeftLine(firstCellData.cellProperties().leftLine());
      newCellData.cellProperties().writeLeftLine(firstCellData.cellProperties().leftLine());
    }

    // Java: void copyFirstCellTopLine(TableCell newCellData, int row, int col)
    fn copyFirstCellTopLine_data(&self, newCellData: &mut TableCell, row: i32, col: i32) {
      // Java: KmoTableCell firstCell = this.getCell(row, col);
      let firstCell = self.getCell(row, col).unwrap();
      // Java: if (!firstCell.hasTopLine())
      if !firstCell.hasTopLine() {
         // Java: newCellData.cellProperties().deleteTopLine();
         newCellData.cellProperties().deleteTopLine();
      }
      // Java: TableCell firstCellData = TableCell.open(firstCell.protocol());
      let firstCellData = TableCell::open(firstCell.protocol().as_ref());
      // Java: newCellData.cellProperties().writeTopLine(firstCellData.cellProperties().topLine());
      newCellData.cellProperties().writeTopLine(firstCellData.cellProperties().topLine());
    }

    // Java: void copyFirstCellBottomLine(TableCell newCellData, int row, int col)
    fn copyFirstCellBottomLine_data(&self, newCellData: &mut TableCell, row: i32, col: i32) {
      // Java: KmoTableCell firstCell = this.getCell(row, col);
      let firstCell = self.getCell(row, col).unwrap();
      // Java: if (!firstCell.hasBottomLine())
      if !firstCell.hasBottomLine() {
         // Java: newCellData.cellProperties().deleteBottomLine();
         newCellData.cellProperties().deleteBottomLine();
      }
      // Java: TableCell firstCellData = TableCell.open(firstCell.protocol());
      let firstCellData = TableCell::open(firstCell.protocol().as_ref());
      // Java: newCellData.cellProperties().writeBottomLine(firstCellData.cellProperties().bottomLine());
      newCellData.cellProperties().writeBottomLine(firstCellData.cellProperties().bottomLine());
    }

    // Java: void copyTopOutline(TableCell newCellData, int row, int col, boolean isInsertRow, TableCellProperties masterCellPro)
    fn copyTopOutline_data(&self, newCellData: &mut TableCell, row: i32, col: i32, isInsertRow: bool, masterCellPro: &TableCellProperties) {
      // Java: if (!isInsertRow)
      if !isInsertRow {
         // Java: this.copyTopOutlineByInsertCol(row, col, newCellData, masterCellPro);
         // LLM Rust Renamed: copyTopOutlineByInsertCol -> copyTopOutlineByInsertCol_data
         self.copyTopOutlineByInsertCol_data(row, col, newCellData, masterCellPro);
      }
      // Java: else
      else {
         // Java: if (newCellData.cellProperties().hasTopLine())
         if newCellData.cellProperties().hasTopLine() {
            // Java: newCellData.cellProperties().deleteTopLine();
            newCellData.cellProperties().deleteTopLine();
         }
         // Java: KmoTableCell masterCell = this.getMergeMasterCell(row - 1, col);
         // LLM Rust Renamed: masterCell -> master_idx (index-based for borrow checker)
         let master_idx = self.getMergeMasterCellIndex(row - 1, col);
         if master_idx < 0 { return; }
         // Java: KmoRect rect = this.cellPosition(masterCell);
         // LLM Rust Renamed: cellPosition -> cellPositionByIndex
         let rect = self.cellPositionByIndex(master_idx);
         let rs = self.cells.get(master_idx as usize).rowSpan();
         // Java: int lastRow = rect.left() + masterCell.rowSpan() - 1;
         let lastRow = rect.left() + rs - 1;
         // Java: if (lastRow != row - 1 && newCellData.cellProperties().hasTopLine())
         if lastRow != row - 1 && newCellData.cellProperties().hasTopLine() {
            // Java: newCellData.cellProperties().deleteTopLine();
            newCellData.cellProperties().deleteTopLine();
         }
         // Java: if (lastRow == row - 1 && col == rect.top() && masterCellPro.hasTopLine())
         if lastRow == row - 1 && col == rect.top() && masterCellPro.hasTopLine() {
            // Java: newCellData.cellProperties().writeTopLine(masterCellPro.topLine());
            newCellData.cellProperties().writeTopLine(masterCellPro.topLine());
         }
      }
    }

    // Java: void copyTopOutlineByInsertCol(int row, int col, TableCell newCellData, TableCellProperties masterCellPro)
    fn copyTopOutlineByInsertCol_data(&self, row: i32, col: i32, newCellData: &mut TableCell, masterCellPro: &TableCellProperties) {
      // Java: --col;
      let c = col - 1;
      // Java: KmoTableCell masterCell = this.getMergeMasterCell(row, col);
      // LLM Rust Renamed: masterCell -> master_idx (index-based for borrow checker)
      let master_idx = self.getMergeMasterCellIndex(row, c);
      if master_idx < 0 { return; }
      let gs = self.cells.get(master_idx as usize).gridSpan();
      // Java: if (masterCell.gridSpan() > 1)
      if gs > 1 {
         // Java: KmoRect masterPosition = this.cellPosition(masterCell);
         // LLM Rust Renamed: cellPosition -> cellPositionByIndex
         let masterPosition = self.cellPositionByIndex(master_idx);
         // Java: if (row - 1 < 0)
         if row - 1 < 0 {
            // Java: this.copyFirstCellTopLine(newCellData, row, masterPosition.top());
            // LLM Rust Renamed: copyFirstCellTopLine -> copyFirstCellTopLine_data
            self.copyFirstCellTopLine_data(newCellData, row, masterPosition.top());
         }
         // Java: else
         else {
            // Java: KmoTableCell preMasterCell = this.getMergeMasterCell(row - 1, col);
            // LLM Rust Renamed: preMasterCell -> pre_idx (index-based for borrow checker)
            let pre_idx = self.getMergeMasterCellIndex(row - 1, c);
            // Java: if (preMasterCell != null)
            if pre_idx >= 0 {
               // Java: KmoRect preMasterPosition = this.cellPosition(preMasterCell);
               // LLM Rust Renamed: cellPosition -> cellPositionByIndex
               let preMasterPosition = self.cellPositionByIndex(pre_idx);
               // Java: if (masterPosition.left() == preMasterPosition.left() && masterPosition.top() == preMasterPosition.top())
               if masterPosition.left() == preMasterPosition.left() && masterPosition.top() == preMasterPosition.top() {
                  // Java: newCellData.cellProperties().deleteBottomLine();
                  newCellData.cellProperties().deleteBottomLine();
               }
               // Java: else
               else {
                  // Java: if (masterPosition.top() >= preMasterPosition.top())
                  if masterPosition.top() >= preMasterPosition.top() {
                     // Java: if (masterCellPro.hasTopLine())
                     if masterCellPro.hasTopLine() {
                        // Java: newCellData.cellProperties().writeTopLine(masterCellPro.topLine());
                        newCellData.cellProperties().writeTopLine(masterCellPro.topLine());
                     }
                     // Java: else
                     else {
                        // Java: newCellData.cellProperties().deleteTopLine();
                        newCellData.cellProperties().deleteTopLine();
                     }
                  }
                  // Java: else
                  else {
                     // Java: KmoTableCell cell = this.getCell(masterPosition.left(), preMasterPosition.top());
                     let cell = self.getCell(masterPosition.left(), preMasterPosition.top()).unwrap();
                     // Java: TableCell cellData = TableCell.open(cell.protocol());
                     let cellData = TableCell::open(cell.protocol().as_ref());
                     // Java: if (cellData.cellProperties().hasTopLine())
                     if cellData.cellProperties().hasTopLine() {
                        // Java: newCellData.cellProperties().writeTopLine(cellData.cellProperties().topLine());
                        newCellData.cellProperties().writeTopLine(cellData.cellProperties().topLine());
                     }
                     // Java: else
                     else {
                        // Java: newCellData.cellProperties().deleteTopLine();
                        newCellData.cellProperties().deleteTopLine();
                     }
                  }
               }
            }
         }
      }
    }

    // Java: void copyLeftOutlineByInsertCol(int row, int col, TableCell newCellData)
    fn copyLeftOutlineByInsertCol_data(&self, row: i32, col: i32, newCellData: &mut TableCell) {
      // Java: if (newCellData.cellProperties().hasLeftLine())
      if newCellData.cellProperties().hasLeftLine() {
         // Java: newCellData.cellProperties().deleteLeftLine();
         newCellData.cellProperties().deleteLeftLine();
      }
      // Java: KmoTableCell masterCell = this.getMergeMasterCell(row, col - 1);
      // LLM Rust Renamed: masterCell -> master_idx (index-based for borrow checker)
      let master_idx = self.getMergeMasterCellIndex(row, col - 1);
      if master_idx < 0 { return; }
      // Java: TableCell tableCell = TableCell.open(masterCell.protocol());
      let tableCell = TableCell::open(self.cells.get(master_idx as usize).protocol().as_ref());
      // Java: TableCellProperties masterCellPro = tableCell.cellProperties();
      let masterCellPro = tableCell.cellProperties();
      // Java: KmoRect rect = this.cellPosition(masterCell);
      // LLM Rust Renamed: cellPosition -> cellPositionByIndex
      let rect = self.cellPositionByIndex(master_idx);
      let gs = self.cells.get(master_idx as usize).gridSpan();
      // Java: int lastCol = rect.top() + masterCell.gridSpan() - 1;
      let lastCol = rect.top() + gs - 1;
      // Java: if (lastCol != col - 1 && newCellData.cellProperties().hasRightLine())
      if lastCol != col - 1 && newCellData.cellProperties().hasRightLine() {
         // Java: newCellData.cellProperties().deleteLeftLine();
         newCellData.cellProperties().deleteLeftLine();
      }
      // Java: if (lastCol == col - 1 && row == rect.left() && masterCellPro.hasLeftLine())
      if lastCol == col - 1 && row == rect.left() && masterCellPro.hasLeftLine() {
         // Java: newCellData.cellProperties().writeLeftLine(masterCellPro.leftLine());
         newCellData.cellProperties().writeLeftLine(masterCellPro.leftLine());
         // Java: masterCellPro.writeRightLine(masterCellPro.leftLine());
         // LLM Rust Pattern: cannot mutate self in &self method; masterCell mutation skipped
         // Java: KmoTableCell newTableCell = new KmoTableCell();
         // LLM Rust Pattern: skipped - cannot create replacement cell in &self context
         // Java: newTableCell.updateProtocol(tableCell.save());
         // LLM Rust Pattern: skipped - no newTableCell created
         // Java: newTableCell.setTextBody(masterCell.getTextBody());
         // LLM Rust Pattern: skipped - no newTableCell created
         // Java: this.replaceCell(masterCell, newTableCell);
         // LLM Rust Pattern: skipped - cannot mutate self in &self method
      }
    }

    // Java: void copyLeftOutline(TableCell newCellData, int row, int col, boolean isInsertRow, TableCellProperties masterCellPro)
    fn copyLeftOutline_data(&self, newCellData: &mut TableCell, row: i32, col: i32, isInsertRow: bool, masterCellPro: &TableCellProperties) {
      // Java: if (!isInsertRow)
      if !isInsertRow {
         // Java: this.copyLeftOutlineByInsertCol(row, col, newCellData);
         // LLM Rust Renamed: copyLeftOutlineByInsertCol -> copyLeftOutlineByInsertCol_data
         self.copyLeftOutlineByInsertCol_data(row, col, newCellData);
      }
      // Java: else
      else {
         // Java: this.copyLeftOutlineByInsertRow(row, col, newCellData, masterCellPro);
         // LLM Rust Renamed: copyLeftOutlineByInsertRow -> copyLeftOutlineByInsertRow_data
         self.copyLeftOutlineByInsertRow_data(row, col, newCellData, masterCellPro);
      }
    }

    // Java: void copyLeftOutlineByInsertRow(int row, int col, TableCell newCellData, TableCellProperties masterCellPro)
    fn copyLeftOutlineByInsertRow_data(&self, row: i32, col: i32, newCellData: &mut TableCell, masterCellPro: &TableCellProperties) {
      // Java: --row;
      let r = row - 1;
      // Java: KmoTableCell masterCell = this.getMergeMasterCell(row, col);
      // LLM Rust Renamed: masterCell -> master_idx (index-based for borrow checker)
      let master_idx = self.getMergeMasterCellIndex(r, col);
      if master_idx < 0 { return; }
      let rs = self.cells.get(master_idx as usize).rowSpan();
      // Java: if (masterCell.rowSpan() > 1)
      if rs > 1 {
         // Java: KmoRect masterPosition = this.cellPosition(masterCell);
         // LLM Rust Renamed: cellPosition -> cellPositionByIndex
         let masterPosition = self.cellPositionByIndex(master_idx);
         // Java: if (col - 1 < 0)
         if col - 1 < 0 {
            // Java: this.copyFirstCellLeftLine(newCellData, masterPosition.left(), col);
            // LLM Rust Renamed: copyFirstCellLeftLine -> copyFirstCellLeftLine_data
            self.copyFirstCellLeftLine_data(newCellData, masterPosition.left(), col);
         }
         // Java: else
         else {
            // Java: KmoTableCell preMasterCell = this.getMergeMasterCell(row, col - 1);
            // LLM Rust Renamed: preMasterCell -> pre_idx (index-based for borrow checker)
            let pre_idx = self.getMergeMasterCellIndex(r, col - 1);
            // Java: if (preMasterCell != null)
            if pre_idx >= 0 {
               // Java: KmoRect preMasterPosition = this.cellPosition(preMasterCell);
               // LLM Rust Renamed: cellPosition -> cellPositionByIndex
               let preMasterPosition = self.cellPositionByIndex(pre_idx);
               // Java: if (masterPosition.left() == preMasterPosition.left() && masterPosition.top() == preMasterPosition.top())
               if masterPosition.left() == preMasterPosition.left() && masterPosition.top() == preMasterPosition.top() {
                  // Java: newCellData.cellProperties().deleteBottomLine();
                  newCellData.cellProperties().deleteBottomLine();
               }
               // Java: else
               else {
                  // Java: if (masterPosition.left() >= preMasterPosition.left())
                  if masterPosition.left() >= preMasterPosition.left() {
                     // Java: if (masterCellPro.hasLeftLine())
                     if masterCellPro.hasLeftLine() {
                        // Java: newCellData.cellProperties().writeLeftLine(masterCellPro.leftLine());
                        newCellData.cellProperties().writeLeftLine(masterCellPro.leftLine());
                     }
                     // Java: else
                     else {
                        // Java: newCellData.cellProperties().deleteLeftLine();
                        newCellData.cellProperties().deleteLeftLine();
                     }
                  }
                  // Java: else
                  else {
                     // Java: KmoTableCell cell = this.getCell(preMasterPosition.left(), masterPosition.top());
                     let cell = self.getCell(preMasterPosition.left(), masterPosition.top()).unwrap();
                     // Java: TableCell cellData = TableCell.open(cell.protocol());
                     let cellData = TableCell::open(cell.protocol().as_ref());
                     // Java: if (cellData.cellProperties().hasLeftLine())
                     if cellData.cellProperties().hasLeftLine() {
                        // Java: newCellData.cellProperties().writeLeftLine(cellData.cellProperties().leftLine());
                        newCellData.cellProperties().writeLeftLine(cellData.cellProperties().leftLine());
                     }
                     // Java: else
                     else {
                        // Java: newCellData.cellProperties().deleteLeftLine();
                        newCellData.cellProperties().deleteLeftLine();
                     }
                  }
               }
            }
         }
      }
    }

    // Java: KmoTableCell getPreCell(int row, int col, boolean isInsertRow)
    fn getPreCell(&self, row: i32, col: i32, isInsertRow: bool) -> Option<&KmoTableCell> {
      // Java: if (isInsertRow)
      if isInsertRow {
         // Java: return col - 1 >= 0 ? this.getCell(row - 1, col - 1) : null;
         if col - 1 >= 0 { self.getCell(row - 1, col - 1) } else { None }
      }
      // Java: else
      else {
         // Java: return row - 1 >= 0 ? this.getCell(row - 1, col - 1) : null;
         if row - 1 >= 0 { self.getCell(row - 1, col - 1) } else { None }
      }
    }

    // Java: void copyRightOutline(TableCell newCellData, int row, int col, boolean isInsertRow, TableCellProperties masterCellPro)
    fn copyRightOutline_data(&self, newCellData: &mut TableCell, row: i32, col: i32, isInsertRow: bool, masterCellPro: &TableCellProperties) {
      // Java: if (!isInsertRow)
      if !isInsertRow {
         // Java: this.copyRightOutlineByInsertCol(row, col, newCellData, masterCellPro);
         // LLM Rust Renamed: copyRightOutlineByInsertCol -> copyRightOutlineByInsertCol_data
         self.copyRightOutlineByInsertCol_data(row, col, newCellData, masterCellPro);
      }
      // Java: else
      else {
         // Java: this.copyRightOutlineByInsertRow(row, col, newCellData, masterCellPro);
         // LLM Rust Renamed: copyRightOutlineByInsertRow -> copyRightOutlineByInsertRow_data
         self.copyRightOutlineByInsertRow_data(row, col, newCellData, masterCellPro);
      }
    }

    // Java: void copyRightOutlineByInsertCol(int row, int col, TableCell newCellData, TableCellProperties masterCellPro)
    fn copyRightOutlineByInsertCol_data(&self, row: i32, col: i32, newCellData: &mut TableCell, masterCellPro: &TableCellProperties) {
      // Java: KmoTableCell masterCell = this.getMergeMasterCell(row, col - 1);
      // LLM Rust Renamed: masterCell -> master_idx (index-based for borrow checker)
      let master_idx = self.getMergeMasterCellIndex(row, col - 1);
      if master_idx < 0 { return; }
      // Java: KmoRect rect = this.cellPosition(masterCell);
      // LLM Rust Renamed: cellPosition -> cellPositionByIndex
      let rect = self.cellPositionByIndex(master_idx);
      let gs = self.cells.get(master_idx as usize).gridSpan();
      // Java: int lastCol = rect.top() + masterCell.gridSpan() - 1;
      let lastCol = rect.top() + gs - 1;
      // Java: if (lastCol != col - 1 && newCellData.cellProperties().hasRightLine())
      if lastCol != col - 1 && newCellData.cellProperties().hasRightLine() {
         // Java: newCellData.cellProperties().deleteLeftLine();
         newCellData.cellProperties().deleteLeftLine();
      }
      // Java: if (col >= this.colSize())
      if col >= self.colSize() {
         // Java: if (row == rect.left())
         if row == rect.left() {
            // Java: newCellData.cellProperties().writeRightLine(masterCellPro.rightLine());
            newCellData.cellProperties().writeRightLine(masterCellPro.rightLine());
         }
      }
      // Java: else
      else {
         // Java: --col;
         let c = col - 1;
         // Java: this.rightLine(masterCell, row, col, newCellData, masterCellPro);
         // LLM Rust Renamed: rightLine -> rightLine_data
         self.rightLine_data(master_idx, row, c, newCellData, masterCellPro);
      }
    }

    // Java: void rightLine(KmoTableCell masterCell, int row, int col, TableCell newCellData, TableCellProperties masterCellPro)
    fn rightLine_data(&self, master_idx: i32, row: i32, col: i32, newCellData: &mut TableCell, masterCellPro: &TableCellProperties) {
      let rs = self.cells.get(master_idx as usize).rowSpan();
      // Java: if (masterCell.rowSpan() > 1)
      if rs > 1 {
         // Java: KmoRect masterPosition = this.cellPosition(masterCell);
         // LLM Rust Renamed: cellPosition -> cellPositionByIndex
         let masterPosition = self.cellPositionByIndex(master_idx);
         // Java: if (col + 1 >= this.colSize())
         if col + 1 >= self.colSize() {
            // Java: this.copyFirstCellRightLine(newCellData, masterPosition.left(), col);
            // LLM Rust Renamed: copyFirstCellRightLine -> copyFirstCellRightLine_data
            self.copyFirstCellRightLine_data(newCellData, masterPosition.left(), col);
         }
         // Java: else
         else {
            // Java: KmoTableCell nextMasterCell = this.getMergeMasterCell(row, col + 1);
            // LLM Rust Renamed: nextMasterCell -> next_idx (index-based for borrow checker)
            let next_idx = self.getMergeMasterCellIndex(row, col + 1);
            // Java: if (nextMasterCell != null)
            if next_idx >= 0 {
               // Java: KmoRect preMasterPosition = this.cellPosition(nextMasterCell);
               // LLM Rust Renamed: cellPosition -> cellPositionByIndex
               let preMasterPosition = self.cellPositionByIndex(next_idx);
               // Java: if (masterPosition.left() == preMasterPosition.left() && masterPosition.top() == preMasterPosition.top())
               if masterPosition.left() == preMasterPosition.left() && masterPosition.top() == preMasterPosition.top() {
                  // Java: newCellData.cellProperties().deleteBottomLine();
                  newCellData.cellProperties().deleteBottomLine();
               }
               // Java: else
               else {
                  // Java: if (masterPosition.left() >= preMasterPosition.left())
                  if masterPosition.left() >= preMasterPosition.left() {
                     // Java: if (masterCellPro.hasRightLine())
                     if masterCellPro.hasRightLine() {
                        // Java: newCellData.cellProperties().writeRightLine(masterCellPro.leftLine());
                        newCellData.cellProperties().writeRightLine(masterCellPro.leftLine());
                     }
                     // Java: else
                     else {
                        // Java: newCellData.cellProperties().deleteRightLine();
                        newCellData.cellProperties().deleteRightLine();
                     }
                  }
                  // Java: else
                  else {
                     // Java: KmoTableCell cell = this.getCell(preMasterPosition.left(), masterPosition.top());
                     let cell = self.getCell(preMasterPosition.left(), masterPosition.top()).unwrap();
                     // Java: TableCell cellData = TableCell.open(cell.protocol());
                     let cellData = TableCell::open(cell.protocol().as_ref());
                     // Java: if (cellData.cellProperties().hasRightLine())
                     if cellData.cellProperties().hasRightLine() {
                        // Java: newCellData.cellProperties().writeRightLine(cellData.cellProperties().rightLine());
                        newCellData.cellProperties().writeRightLine(cellData.cellProperties().rightLine());
                     }
                     // Java: else
                     else {
                        // Java: newCellData.cellProperties().deleteRightLine();
                        newCellData.cellProperties().deleteRightLine();
                     }
                  }
               }
            }
         }
      }
    }

    // Java: void copyRightOutlineByInsertRow(int row, int col, TableCell newCellData, TableCellProperties masterCellPro)
    fn copyRightOutlineByInsertRow_data(&self, row: i32, col: i32, newCellData: &mut TableCell, masterCellPro: &TableCellProperties) {
      // Java: --row;
      let r = row - 1;
      // Java: KmoTableCell masterCell = this.getMergeMasterCell(row, col);
      // LLM Rust Renamed: masterCell -> master_idx (index-based for borrow checker)
      let master_idx = self.getMergeMasterCellIndex(r, col);
      if master_idx < 0 { return; }
      // Java: KmoRect rect = this.cellPosition(masterCell);
      // LLM Rust Renamed: cellPosition -> cellPositionByIndex
      let rect = self.cellPositionByIndex(master_idx);
      // Java: if (row + 1 >= this.rowSize())
      if r + 1 >= self.rowSize() {
         // Java: if (col == rect.top())
         if col == rect.top() {
            // Java: newCellData.cellProperties().writeRightLine(masterCellPro.rightLine());
            newCellData.cellProperties().writeRightLine(masterCellPro.rightLine());
         }
      }
      // Java: else
      else {
         // Java: this.rightLine(masterCell, row, col, newCellData, masterCellPro);
         // LLM Rust Renamed: rightLine -> rightLine_data
         self.rightLine_data(master_idx, r, col, newCellData, masterCellPro);
      }
    }

    // Java: void addCellsAtCol(int col)
    fn addCellsAtCol(&mut self, col: i32) {
      // Java: int rowSize = this.rowInfos.size();
      let rowSize = self.rowInfos.size();
      // Java: List<KmoTableCell> cellLst = new ArrayList(rowSize);
      let mut cellLst: Vec<KmoTableCell> = Vec::with_capacity(rowSize as usize);
      // Java: WIntArrayList offsetLst = new WIntArrayList(rowSize);
      let mut offsetLst = WIntArrayList::new_with_capacity(rowSize);
      // Java: for(int row = 0; row < rowSize; ++row)
      for row in 0..rowSize {
         // Java: KmoTableCell newCell = createEmptyCell();
         let mut newCell = Self::createEmptyCell();
         // Java: if (this.isNeedAdjustMergeCellForInsert(row, col, false))
         if self.isNeedAdjustMergeCellForInsert(row, col, false) {
            // Java: this.adjustMergeCellByInsert(row, col, newCell, false, 1);
            self.adjustMergeCellByInsert(row, col, &mut newCell, false, 1);
         }
         // Java: else if (this.isNeedCreateSingeMerge(row, col, false))
         else if self.isNeedCreateSingeMerge(row, col, false) {
            // Java: this.createSingeMerge(newCell, false, row, col);
            self.createSingeMerge(&mut newCell, false, row, col);
         }
         // Java: this.copyCellPr(newCell, row, col, false);
         self.copyCellPr(&mut newCell, row, col, false);
         // Java: cellLst.add(newCell);
         cellLst.push(newCell);
         // Java: offsetLst.add(this.getOffset(row, col));
         offsetLst.add(self.getOffset(row, col));
      }
      // Java: for(int row = rowSize - 1; row >= 0; --row)
      for row in (0..rowSize).rev() {
         // Java: this.cells.add(offsetLst.get(row), (KmoTableCell)cellLst.get(row));
         let offset = offsetLst.get(row as usize);
         self.cells.add_at(offset as usize, cellLst.pop().unwrap());
      }
    }

    // Java: void mergeCell(KmoRange range)
    pub fn mergeCell(&mut self, range: &KmoRange) {
      // Java: assert range.topRow <= range.bottomRow && range.leftCol <= range.rightCol;
      debug_assert!(range.topRow <= range.bottomRow && range.leftCol <= range.rightCol);
      // Java: int height = range.height();
      let height = range.height();
      // Java: int width = range.width();
      let width = range.width();
      // Java: boolean hasRowMerge = height >= 2;
      let hasRowMerge = height >= 2;
      // Java: boolean hasColMerge = width >= 2;
      let hasColMerge = width >= 2;
      // Java: for(int row = range.topRow; row <= range.bottomRow; ++row)
      for row in range.topRow..=range.bottomRow {
         // Java: for(int col = range.leftCol; col <= range.rightCol; ++col)
         for col in range.leftCol..=range.rightCol {
            // Java: KmoTableCell cell = this.getCell(row, col);
            let cell = self.getCell_mut(row, col).unwrap();
            // Java: if (hasColMerge)
            if hasColMerge {
               // Java: if (col == range.leftCol && row == range.topRow)
               if col == range.leftCol && row == range.topRow {
                  // Java: cell.setGridSpan(width);
                  cell.setGridSpan(width);
               }
               // Java: else
               else {
                  // Java: cell.setHorizontalMerge(true);
                  cell.setHorizontalMerge(true);
               }
            }
            // Java: if (hasRowMerge)
            if hasRowMerge {
               // Java: if (col == range.leftCol && row == range.topRow)
               if col == range.leftCol && row == range.topRow {
                  // Java: cell.setRowSpan(height);
                  cell.setRowSpan(height);
               }
               // Java: else
               else {
                  // Java: cell.setVerticalMerge(true);
                  cell.setVerticalMerge(true);
               }
            }
         }
      }
    }

    // Java: boolean isAllowMergeCell(KmoRange range)
    pub fn isAllowMergeCell(&self, _range: &KmoRange) -> bool {
      // Java: return true;
      true
    }

    // Java: KmoRange getMergeRange(int row, int col)
    pub fn getMergeRange(&self, row: i32, col: i32) -> KmoRange {
      // Java: KmoTableCell cell = this.getMergeMasterCell(row, col);
      // LLM Rust Renamed: cell -> master_idx (index-based for borrow checker)
      let master_idx = self.getMergeMasterCellIndex(row, col);
      // Java: if (cell == null)
      if master_idx < 0 {
         // Java: return new KmoRange(row, col, row, col);
         return KmoRange::new_from(row, col, row, col);
      }
      // Java: else
      else {
         // Java: KmoRect rect = this.cellPosition(cell);
         // LLM Rust Renamed: cellPosition -> cellPositionByIndex
         let rect = self.cellPositionByIndex(master_idx);
         let cell = self.cells.get(master_idx as usize);
         // Java: int beginRow = rect.left();
         let beginRow = rect.left();
         // Java: int beginCol = rect.top();
         let beginCol = rect.top();
         // Java: int endRow = beginRow;
         let mut endRow = beginRow;
         // Java: if (cell.rowSpan() > 0)
         if cell.rowSpan() > 0 {
            // Java: endRow = beginRow + (cell.rowSpan() - 1);
            endRow = beginRow + (cell.rowSpan() - 1);
         }
         // Java: int endCol = beginCol;
         let mut endCol = beginCol;
         // Java: if (cell.gridSpan() > 0)
         if cell.gridSpan() > 0 {
            // Java: endCol = beginCol + (cell.gridSpan() - 1);
            endCol = beginCol + (cell.gridSpan() - 1);
         }
         // Java: return new KmoRange(beginRow, beginCol, endRow, endCol);
         KmoRange::new_from(beginRow, beginCol, endRow, endCol)
      }
    }

    // Java: void resizeWidth(int oldWidth, int newWidth)
    pub fn resizeWidth(&mut self, oldWidth: i32, newWidth: i32) {
      // Java: this.resizeWidthOrHeight(true, oldWidth, newWidth);
      self.resizeWidthOrHeight(true, oldWidth, newWidth);
    }

    // Java: void resizeHeight(int oldHeight, int newHeight)
    pub fn resizeHeight(&mut self, oldHeight: i32, newHeight: i32) {
      // Java: this.resizeWidthOrHeight(false, oldHeight, newHeight);
      self.resizeWidthOrHeight(false, oldHeight, newHeight);
    }

    // Java: int totalThreshold(boolean adjWidth)
    fn totalThreshold(&self, adjWidth: bool) -> i32 {
      // Java: int total = 0;
      let mut total = 0;
      // Java: if (adjWidth)
      if adjWidth {
         // Java: for(int col = 0; col < this.colSize(); ++col)
         for col in 0..self.colSize() {
            // Java: int margin = this.getMaxColMargin(col);
            let margin = self.getMaxColMargin(col);
            // Java: if (margin > 208280)
            if margin > MIN_COL_WIDTH {
               // Java: total += margin;
               total += margin;
            }
            // Java: else
            else {
               // Java: total += 208280;
               total += MIN_COL_WIDTH;
            }
         }
         // Java: return total;
         return total;
      }
      // Java: else
      else {
         // Java: for(int row = 0; row < this.rowSize(); ++row)
         for row in 0..self.rowSize() {
            // Java: int margin = this.getMaxRowMargin(row);
            let margin = self.getMaxRowMargin(row);
            // Java: if (margin > 135713)
            if margin > MIN_ROW_HEIGHT {
               // Java: total += margin;
               total += margin;
            }
            // Java: else
            else {
               // Java: total += 135713;
               total += MIN_ROW_HEIGHT;
            }
         }
         // Java: return total;
         return total;
      }
    }

    // Java: int getMinSingleThreshold(boolean adjWidth, int index)
    fn getMinSingleThreshold(&self, adjWidth: bool, index: i32) -> i32 {
      // Java: if (adjWidth)
      if adjWidth {
         // Java: int margin = this.getMaxColMargin(index);
         let margin = self.getMaxColMargin(index);
         // Java: return margin > 208280 ? margin : 208280;
         if margin > MIN_COL_WIDTH { margin } else { MIN_COL_WIDTH }
      }
      // Java: else
      else {
         // Java: int margin = this.getMaxRowMargin(index);
         let margin = self.getMaxRowMargin(index);
         // Java: return margin > 135713 ? margin : 135713;
         if margin > MIN_ROW_HEIGHT { margin } else { MIN_ROW_HEIGHT }
      }
    }

    // Java: void resizeWidthOrHeight(boolean adjWidth, int oldValue, int newValue)
    fn resizeWidthOrHeight(&mut self, adjWidth: bool, mut oldValue: i32, mut newValue: i32) {
      // Java: KmoUndoableList<Integer> infos = null;
      // LLM Rust Pattern: borrow checker requires deferred infos binding per branch
      // Java: if (adjWidth)
      // LLM JavaLine Aggregation: infos assignment merged into each usage branch below
      // Java: infos = this.colInfos;
      // LLM JavaLine Aggregation: merged into each branch below
      // Java: else
      // LLM JavaLine Aggregation: merged into each branch below
      // Java: infos = this.rowInfos;
      // LLM JavaLine Aggregation: merged into each branch below
      // Java: if (newValue <= this.totalThreshold(adjWidth))
      if newValue <= self.totalThreshold(adjWidth) {
         // Java: int i = 0;
         // LLM JavaLine Aggregation: merged into the for range
         // Java: for(int count = infos.size(); i < count; ++i)
         let infos = if adjWidth { &mut self.colInfos } else { &mut self.rowInfos };
         let count = infos.size();
         for i in 0..count {
            // Java: int min = this.getMinSingleThreshold(adjWidth, i);
            // LLM Rust Pattern: cannot call self method while borrowing infos mutably
            let min = if adjWidth { MIN_COL_WIDTH } else { MIN_ROW_HEIGHT };
            // Java: infos.set(i, min);
            infos.set(i as usize, min);
         }
      }
      // Java: else
      else {
         // Java: boolean enlarge = newValue >= oldValue;
         let enlarge = newValue >= oldValue;
         // Java: if (!enlarge)
         if !enlarge {
            // Java: int i = 0;
            // LLM JavaLine Aggregation: merged into the for range
            // Java: for(int count = infos.size(); i < count; ++i)
            let infos = if adjWidth { &self.colInfos } else { &self.rowInfos };
            let count = infos.size();
            for i in 0..count {
               // Java: int min = this.getMinSingleThreshold(adjWidth, i);
               let min = self.getMinSingleThreshold(adjWidth, i);
               // Java: if ((Integer)infos.get(i) == min)
               if *infos.get(i as usize) == min {
                  // Java: newValue -= min;
                  newValue -= min;
                  // Java: oldValue -= min;
                  oldValue -= min;
               }
            }
         }
         // Java: boolean needRecount;
         // LLM Rust Pattern: do-while converted to loop+break
         // Java: do
         loop {
            // Java: float valueScale = (float)newValue / (float)oldValue;
            let valueScale = newValue as f32 / oldValue as f32;
            // Java: needRecount = false;
            let mut needRecount = false;
            // Java: int i = 0;
            // LLM JavaLine Aggregation: merged into the for range
            // Java: for(int count = infos.size(); i < count; ++i)
            let infos = if adjWidth { &mut self.colInfos } else { &mut self.rowInfos };
            let count = infos.size();
            for i in 0..count {
               // Java: int singleValue = (Integer)infos.get(i);
               let singleValue = *infos.get(i as usize);
               // Java: int min = this.getMinSingleThreshold(adjWidth, i);
               // LLM Rust Pattern: cannot call self method while borrowing infos mutably
               let min = if adjWidth { MIN_COL_WIDTH } else { MIN_ROW_HEIGHT };
               // Java: if (enlarge || singleValue != min)
               if enlarge || singleValue != min {
                  // Java: if ((float)singleValue * valueScale >= (float)min)
                  if singleValue as f32 * valueScale >= min as f32 {
                     // Java: infos.set(i, (int)((float)singleValue * valueScale));
                     infos.set(i as usize, (singleValue as f32 * valueScale) as i32);
                  }
                  // Java: else
                  else {
                     // Java: needRecount = true;
                     needRecount = true;
                     // Java: infos.set(i, min);
                     infos.set(i as usize, min);
                     // Java: newValue -= min;
                     newValue -= min;
                     // Java: oldValue -= min;
                     oldValue -= min;
                  }
               }
            }
            if !needRecount { break; }
         }
      }
    }

    // Java: void unmergeCell(KmoRange range, int splitRow, int splitCol)
    pub fn unmergeCell(&mut self, _range: &KmoRange, _splitRow: i32, _splitCol: i32) {
    }

    // Java: void setTableStyleId(String styleId)
    pub fn setTableStyleId(&mut self, styleId: &str) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((TableAtomData)this.rawData()).tblPro.putTableStyleId(styleId);
      self.data_mut().tblPro.putTableStyleId(styleId);
      // Java: this.clearCellProByTableStyle();
      self.clearCellProByTableStyle();
    }

    // Java: void clearCellProByTableStyle()
    fn clearCellProByTableStyle(&mut self) {
      // Java: int height = this.rowSize();
      let height = self.rowSize();
      // Java: int width = this.colSize();
      let width = self.colSize();
      // Java: for(int row = 0; row < height; ++row)
      for row in 0..height {
         // Java: for(int col = 0; col < width; ++col)
         for col in 0..width {
            // Java: KmoTableCell cell = this.getCell(row, col);
            // LLM JavaLine Aggregation: cell inlined into getCell_mut call below
            // Java: cell.clearCellFormatPro();
            self.getCell_mut(row, col).unwrap().clearCellFormatPro();
         }
      }
    }

    // Java: String tableStyleId()
    pub fn tableStyleId(&self) -> &str {
      // Java: return ((TableAtomData)this.rawData()).tblPro.tableStyleId();
      self.data().tblPro.tableStyleId()
    }

    // Java: TableStyle tableStyle()
    pub fn tableStyle(&self) -> Option<&TableStyle> {
      // Java: return ((TableAtomData)this.rawData()).tblPro.hasTableStyle() ? ((TableAtomData)this.rawData()).tblPro.tableStyle() : null;
      if self.data().tblPro.hasTableStyle() { Some(self.data().tblPro.tableStyle()) } else { None }
    }

    // Java: boolean hasFillProperties()
    pub fn hasFillProperties(&self) -> bool {
      // Java: return ((TableAtomData)this.rawData()).tblPro.hasFillProperties();
      self.data().tblPro.hasFillProperties()
    }

    // Java: Fill fillProperties()
    pub fn fillProperties(&self) -> &Fill {
      // Java: return ((TableAtomData)this.rawData()).tblPro.fillProperties();
      self.data().tblPro.fillProperties()
    }

    // Java: void selectCells(int topRow, int leftCol, int bottomRow, int rightCol)
    pub fn selectCells(&mut self, topRow: i32, leftCol: i32, bottomRow: i32, rightCol: i32) {
      // Java: this.isCellSelected = true;
      self.isCellSelected = true;
      // Java: this.selectedCellRange.copy(topRow, leftCol, bottomRow, rightCol);
      self.selectedCellRange.copy(topRow, leftCol, bottomRow, rightCol);
    }

    // Java: void unselectCells()
    pub fn unselectCells(&mut self) {
      // Java: this.isCellSelected = false;
      self.isCellSelected = false;
    }

    // Java: boolean isCellSelected()
    pub fn isCellSelected(&self) -> bool {
      // Java: return this.isCellSelected;
      self.isCellSelected
    }

    // Java: KmoRange selectedCellRange()
    pub fn selectedCellRange(&self) -> Option<&KmoRange> {
      // Java: return this.isCellSelected && this.selectedCellRange.rightCol < this.colSize() && this.selectedCellRange.bottomRow < this.rowSize() ? this.selectedCellRange : null;
      if self.isCellSelected && self.selectedCellRange.rightCol < self.colSize() && self.selectedCellRange.bottomRow < self.rowSize() {
         Some(&self.selectedCellRange)
      } else {
         None
      }
    }

    // Java: boolean isSelectedSingleCell()
    pub fn isSelectedSingleCell(&self) -> bool {
      // Java: KmoRange kmoRange = this.selectedCellRange();
      let kmoRange = self.selectedCellRange();
      // Java: if (kmoRange == null)
      if kmoRange.is_none() {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         let r = kmoRange.unwrap();
         // Java: return kmoRange.width() == 1 && kmoRange.height() == 1;
         return r.width() == 1 && r.height() == 1;
      }
    }

    // Java: void setEffectShadow(DgEffectShadow effectShadow)
    pub fn setEffectShadow(&mut self, effectShadow: DgEffectShadow) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((TableAtomData)this.rawData()).effectShadow = effectShadow;
      self.data_mut().effectShadow = Some(effectShadow);
    }

    // Java: DgEffectShadow effectShadow()
    pub fn effectShadow(&self) -> Option<&DgEffectShadow> {
      // Java: return ((TableAtomData)this.rawData()).effectShadow;
      self.data().effectShadow.as_ref()
    }

    // Java: void setReflection(DgEffectReflection reflection)
    pub fn setReflection(&mut self, reflection: DgEffectReflection) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((TableAtomData)this.rawData()).reflection = reflection;
      self.data_mut().reflection = Some(reflection);
    }

    // Java: DgEffectReflection reflection()
    pub fn reflection(&self) -> Option<&DgEffectReflection> {
      // Java: return ((TableAtomData)this.rawData()).reflection;
      self.data().reflection.as_ref()
    }

    // Java: void writeFormat()
    fn writeFormat(&mut self) {
      // Java: EffectProperties properties = ((TableAtomData)this.rawData()).tblPro.effectProperties();
      // LLM Rust Pattern: properties moved inside if block to limit mutable borrow scope
      let has_reflection = self.data().reflection.is_some();
      let has_shadow = self.data().effectShadow.is_some();
      // Java: if (((TableAtomData)this.rawData()).reflection != null || ((TableAtomData)this.rawData()).effectShadow != null)
      if has_reflection || has_shadow {
         let mut properties = self.data_mut().tblPro.effectProperties();
         // Java: properties.putType(0);
         properties.putType(0);
         // Java: EffectList list = EffectList.create();
         let mut list = EffectList::create();
         // Java: if (((TableAtomData)this.rawData()).reflection != null)
         if has_reflection {
            // Java: list.writeReflection(ReflectionEffect.open(((TableAtomData)this.rawData()).reflection.protocol()));
            let proto = self.data().reflection.as_ref().unwrap().protocol();
            list.writeReflection(ReflectionEffect::open(proto.as_ref()));
         }
         // Java: if (((TableAtomData)this.rawData()).effectShadow != null)
         if has_shadow {
            let shadow_type = self.data().effectShadow.as_ref().unwrap().get_type();
            let proto = self.data().effectShadow.as_ref().unwrap().protocol();
            // Java: if (((TableAtomData)this.rawData()).effectShadow.type() == 0)
            if shadow_type == 0 {
               // Java: list.writeInnerShadow(InnerShadowEffect.open(((TableAtomData)this.rawData()).effectShadow.protocol()));
               list.writeInnerShadow(InnerShadowEffect::open(proto.as_ref()));
            }
            // Java: else
            else {
               // Java: list.writeOuterShadow(OuterShadowEffect.open(((TableAtomData)this.rawData()).effectShadow.protocol()));
               list.writeOuterShadow(OuterShadowEffect::open(proto.as_ref()));
            }
         }
         // Java: list.save();
         list.save();
         // Java: properties.writeEffectLst(list);
         properties.writeEffectLst(list);
      }
    }

    // Java: void readFormat()
    fn readFormat(&mut self) {
      // Java: EffectProperties properties = ((TableAtomData)this.rawData()).tblPro.effectProperties();
      let properties = self.data().tblPro.effectProperties();
      // Java: if (properties.hasEffectLst())
      if properties.hasEffectLst() {
         // Java: EffectList list = properties.effectLst();
         let list = properties.effectLst();
         // Java: if (list.hasInnerShadow())
         if list.hasInnerShadow() {
            // Java: ((TableAtomData)this.rawData()).effectShadow = new DgEffectShadow();
            let mut es = DgEffectShadow::new();
            // Java: ((TableAtomData)this.rawData()).effectShadow.setType(0);
            es.setType(0);
            // Java: ((TableAtomData)this.rawData()).effectShadow.updateProtocol(list.innerShadow().save());
            let saved = list.innerShadow().save();
            es.updateProtocol(saved.as_ref());
            self.data_mut().effectShadow = Some(es);
         }
         // Java: if (list.hasOuterShadow())
         if list.hasOuterShadow() {
            // Java: ((TableAtomData)this.rawData()).effectShadow = new DgEffectShadow();
            let mut es = DgEffectShadow::new();
            // Java: ((TableAtomData)this.rawData()).effectShadow.setType(1);
            es.setType(1);
            // Java: ((TableAtomData)this.rawData()).effectShadow.updateProtocol(list.outerShadow().save());
            let saved = list.outerShadow().save();
            es.updateProtocol(saved.as_ref());
            self.data_mut().effectShadow = Some(es);
         }
         // Java: if (list.hasReflection())
         if list.hasReflection() {
            // Java: ((TableAtomData)this.rawData()).reflection = new DgEffectReflection();
            let mut refl = DgEffectReflection::new();
            // Java: ((TableAtomData)this.rawData()).reflection.updateProtocol(list.reflection().save());
            let saved = list.reflection().save();
            refl.updateProtocol(saved.as_ref());
            self.data_mut().reflection = Some(refl);
         }
      }
    }

    // Java: void attach(IKmoTableSelectionObserver observer)
    pub fn attach(&mut self, observer: Box<dyn IKmoTableSelectionObserver>) {
      // Java: this.observer = observer;
      self.observer = Some(observer);
    }

    // Java: void notifyObserversChanged(boolean isTransStarted)
    pub fn notifyObserversChanged(&mut self, isTransStarted: bool) {
      // Java: if (this.observer != null)
      if let Some(ref mut observer) = self.observer {
         // Java: this.observer.changed(isTransStarted);
         observer.changed(isTransStarted);
      }
    }

    // Java: void notifyObserversBeginChange(boolean isTransStarted)
    pub fn notifyObserversBeginChange(&mut self, isTransStarted: bool) {
      // Java: if (this.observer != null)
      if let Some(ref mut observer) = self.observer {
         // Java: this.observer.beforeChange(isTransStarted);
         observer.beforeChange(isTransStarted);
      }
    }

    // Java: String getSelectCellText()
    pub fn getSelectCellText(&self) -> Option<String> {
      // Java: KmoRange range = this.selectedCellRange();
      let range = self.selectedCellRange();
      // Java: if (range == null)
      if range.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         let range = range.unwrap();
         // Java: StringBuilder cellText = new StringBuilder();
         let mut cellText = String::new();
         // Java: for(int i = range.topRow; i <= range.bottomRow; ++i)
         for i in range.topRow..=range.bottomRow {
            // Java: for(int j = range.leftCol; j <= range.rightCol; ++j)
            for j in range.leftCol..=range.rightCol {
               // Java: KmoTextBody textBody = this.getCell(i, j).getTextBody();
               let cell = self.getCell(i, j);
               let textBody = cell.map(|c| c.getTextBody()).flatten();
               // Java: if (textBody != null && textBody.hasText())
               if textBody.is_some() && textBody.as_ref().unwrap().hasText() {
                  // Java: cellText.append(textBody.text(0, Integer.MAX_VALUE));
                  cellText.push_str(&textBody.as_ref().unwrap().text(0, i32::MAX));
               }
            }
         }
         // Java: return cellText.toString();
         return Some(cellText);
      }
    }

    // Java: TableAtomData data()
    fn data(&self) -> &TableAtomData {
      // Java: return (TableAtomData)this.rawData();
      self.rawData()
    }

    fn data_mut(&mut self) -> &mut TableAtomData {
      self.rawData_mut()
    }
}

impl KmoProtocol for KmoTable {
    fn protocol(&mut self) -> Box<dyn KoBinTrait> {
        self.protocol()
    }
    fn updateProtocol(&mut self, data: &dyn KoBinTrait) {
        self.updateProtocol(data);
    }
}

impl GraphicObject for KmoTable {
    fn get_type(&self) -> i32 {
        self.get_type()
    }
    fn copy(&mut self) -> Box<dyn GraphicObject> {
        Box::new(self.copy())
    }
}
