// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import cn.wps.moffice.writer.data.value.CellHideMark
use crate::cn::wps::moffice::writer::data::value::CellHideMark::CellHideMark;
// Java: import cn.wps.moffice.writer.data.value.CellMargin
use crate::cn::wps::moffice::writer::data::value::CellMargin::CellMargin;
// Java: import cn.wps.moffice.writer.data.value.TableCellBuilder
use crate::cn::wps::moffice::writer::data::value::TableCellBuilder::TableCellBuilder;
// Java: import cn.wps.moffice.writer.data.value.TableLength
use crate::cn::wps::moffice::writer::data::value::TableLength::TableLength;
// Java: import cn.wps.moffice.writer.data.value.TableSetBRC
use crate::cn::wps::moffice::writer::data::value::TableSetBRC::TableSetBRC;
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::ImporterBase;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::Xml03Context;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::{Attributes, ConvertHelper};
use std::sync::Arc;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.TablePropertyBaseConverterSimple
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::TablePropertyBaseConverterSimple::TablePropertyBaseConverterSimple;
// Java: import java.lang.Object
use stubs::java::lang::Object::Object;
use std::collections::{HashMap};

// java public final class CellPropertyConverterSimple extends ImporterBase
#[derive(Default, Clone)]
pub struct CellPropertyConverterSimple {
    // java ImporterBase super_
    pub(crate) super_: ImporterBase,
    // java private TablePropertyBaseConverterSimple mTablePropsConverter;
    mTablePropsConverter: Option<TablePropertyBaseConverterSimple>,
}

// java public final class CellPropertyConverterSimple extends ImporterBase
impl CellPropertyConverterSimple {
    // java public  CellPropertyConverterSimple(TablePropertyBaseConverterSimple tablePropsConverter, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn CellPropertyConverterSimple(tablePropsConverter: TablePropertyBaseConverterSimple, mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(mode, xml07Context, xml03Context);
      // java this.mTablePropsConverter = tablePropsConverter;
      Self { super_: ImporterBase::ImporterBase(mode, xml07Context, xml03Context), mTablePropsConverter: Some(tablePropsConverter) }
    }

    // java public void convertCellPropBase(int elementId, MutablePropertySet dstTableProps, Attributes attributes, int cellIndex)
    pub fn convertCellPropBase(&self, elementId: i32, dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java IOAssert.assertNotNull("dstTableProps should not be null !", dstTableProps);
      IOAssert::assertNotNull__string_object(&"dstTableProps should not be null !", Some(&*dstTableProps));
      // java IOAssert.assertNotNull("rowPropertyBase should not be null !", attributes);
      IOAssert::assertNotNull__string_object(&"rowPropertyBase should not be null !", Some(&0i32));
      // java switch (elementId)
      // java break
      match elementId {
        // java case -2065786911:
        -2065786911 => {
          // java ConvertHelper.convertTableRevision(dstTableProps, attributes);
          ConvertHelper::convertTableRevision(dstTableProps, attributes);
        }
        // java case -1950215313:
        -1950215313 => {
          // java setCellFitText(dstTableProps, attributes, cellIndex);
          Self::setCellFitText(dstTableProps, attributes, cellIndex);
        }
        // java case -1774131409:
        -1774131409 => {
          // java setCellHideMark(dstTableProps, attributes, cellIndex);
          Self::setCellHideMark(dstTableProps, attributes, cellIndex);
        }
        // java case -1713877738: case -1243302736: case 32: case 35: case 795307910:
        -1713877738 | -1243302736 | 32 | 35 | 795307910 => {
        }
        // java case -1040545365:
        -1040545365 => {
          // java setTcNoWrap(dstTableProps, attributes, cellIndex);
          Self::setTcNoWrap(dstTableProps, attributes, cellIndex);
        }
        // java case -1004053509: case -687708974:
        -1004053509 | -687708974 => {
          // java this.setTextFlow(dstTableProps, attributes, cellIndex);
          self.setTextFlow(dstTableProps, attributes, cellIndex);
        }
        // java case -853376977:
        -853376977 => {
          // java setVertAlign(dstTableProps, attributes, cellIndex);
          Self::setVertAlign(dstTableProps, attributes, cellIndex);
        }
        // java case -842494622: case -812941950:
        -842494622 | -812941950 => {
          // java setVertMerge(dstTableProps, attributes, cellIndex);
          Self::setVertMerge(dstTableProps, attributes, cellIndex);
        }
        // java case 31:
        31 => {
          // java addCellMargin4Normal(dstTableProps, this.mTablePropsConverter.toTopCellMargin(cellIndex, attributes));
          let margin = self.table_props().toTopCellMargin__int_attributes(cellIndex, attributes);
          Self::addCellMargin4Normal(dstTableProps, &margin);
        }
        // java case 33:
        33 => {
          // java addCellMargin4Normal(dstTableProps, this.mTablePropsConverter.toLeftCellMargin(cellIndex, attributes));
          let margin = self.table_props().toLeftCellMargin__int_attributes(cellIndex, attributes);
          Self::addCellMargin4Normal(dstTableProps, &margin);
        }
        // java case 34:
        34 => {
          // java addCellMargin4Normal(dstTableProps, this.mTablePropsConverter.toBottomCellMargin(cellIndex, attributes));
          let margin = self.table_props().toBottomCellMargin__int_attributes(cellIndex, attributes);
          Self::addCellMargin4Normal(dstTableProps, &margin);
        }
        // java case 36:
        36 => {
          // java addCellMargin4Normal(dstTableProps, this.mTablePropsConverter.toRightCellMargin(cellIndex, attributes));
          let margin = self.table_props().toRightCellMargin__int_attributes(cellIndex, attributes);
          Self::addCellMargin4Normal(dstTableProps, &margin);
        }
        // java case 37:
        37 => {
          // java this.setTcBorderTop(dstTableProps, cellIndex, attributes);
          self.setTcBorderTop(dstTableProps, cellIndex, attributes);
        }
        // java case 38:
        38 => {
          // java this.setTcBorderStart(dstTableProps, cellIndex, attributes);
          self.setTcBorderStart(dstTableProps, cellIndex, attributes);
        }
        // java case 39:
        39 => {
          // java this.setTcBorderLeft(dstTableProps, cellIndex, attributes);
          self.setTcBorderLeft(dstTableProps, cellIndex, attributes);
        }
        // java case 40:
        40 => {
          // java this.setTcBorderBottom(dstTableProps, cellIndex, attributes);
          self.setTcBorderBottom(dstTableProps, cellIndex, attributes);
        }
        // java case 41:
        41 => {
          // java this.setTcBorderEnd(dstTableProps, cellIndex, attributes);
          self.setTcBorderEnd(dstTableProps, cellIndex, attributes);
        }
        // java case 42:
        42 => {
          // java this.setTcBorderRight(dstTableProps, cellIndex, attributes);
          self.setTcBorderRight(dstTableProps, cellIndex, attributes);
        }
        // java case 113839:
        113839 => {
          // java setDefShd(dstTableProps, attributes, cellIndex);
          Self::setDefShd(dstTableProps, attributes, cellIndex);
        }
        // java case 114632:
        114632 => {
          // java this.setTcWidth(dstTableProps, attributes, cellIndex);
          self.setTcWidth(dstTableProps, attributes, cellIndex);
        }
        // java case 110397066:
        110397066 => {
          // java this.setTableBorderTl2br(dstTableProps, attributes, cellIndex);
          self.setTableBorderTl2br(dstTableProps, attributes, cellIndex);
        }
        // java case 110575806:
        110575806 => {
          // java this.setTableBorderTr2bl(dstTableProps, attributes, cellIndex);
          self.setTableBorderTr2bl(dstTableProps, attributes, cellIndex);
        }
        // java case 318038960:
        318038960 => {
          // java addGridSpan(dstTableProps, attributes, cellIndex);
          Self::addGridSpan(dstTableProps, attributes, cellIndex);
        }
        // java default:
        _ => {
          // java IOAssert.fail("It should not reach here! " + Integer.toString(elementId));
          IOAssert::fail__string(&format!("It should not reach here! {}", elementId));
        }
      }
    }

    fn table_props(&self) -> &TablePropertyBaseConverterSimple {
      self.mTablePropsConverter.as_ref().expect("mTablePropsConverter")
    }

    fn update_cell(dst: &mut MutablePropertySet, cellIndex: i32, f: impl FnOnce(&mut TableCellBuilder)) {
      let mut tableCell = Self::getTableCell(dst, cellIndex);
      f(&mut tableCell);
      Self::store_cell(dst, cellIndex, tableCell);
    }

    fn store_cell(dst: &mut MutablePropertySet, cellIndex: i32, tableCell: TableCellBuilder) {
      let mut tableCellMap = Self::cell_map(dst);
      tableCellMap.insert(cellIndex, tableCell);
      dst.setValue(373, Some(Arc::new(tableCellMap)));
    }

    fn cell_map(dst: &MutablePropertySet) -> HashMap<i32, TableCellBuilder> {
      dst.getValue(373)
        .and_then(|v| v.as_ref().as_any().downcast_ref::<HashMap<i32, TableCellBuilder>>().cloned())
        .unwrap_or_default()
    }

    // java private static void setCellHideMark(MutablePropertySet dst, Attributes attributes, int cellIndex)
    pub fn setCellHideMark(dst: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java IOAssert.assertNotNull("dst should not be null!", dst);
      IOAssert::assertNotNull__string_object(&"dst should not be null!", Some(&*dst));
      // java ArrayList<CellHideMark> listCellHideMark = (ArrayList)dst.getValue(367);
      let mut listCellHideMark = dst.getValue(367)
        .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<CellHideMark>>().cloned())
        .unwrap_or_default();
      // java if (null == listCellHideMark)
      // java listCellHideMark = new ArrayList();
      // java dst.setValue(367, listCellHideMark);
      // java boolean hideMark = ConvertHelper.toBoolean(attributes);
      let hideMark = ConvertHelper::toBoolean(attributes);
      // java listCellHideMark.add(TablePropsConverter.toCellHideMark(cellIndex, hideMark));
      listCellHideMark.push(CellHideMark::obtain(cellIndex, cellIndex + 1, hideMark));
      dst.setValue(367, Some(Arc::new(listCellHideMark)));
    }

    // java private static void addGridSpan(MutablePropertySet dst, Attributes attributes, int cellIndex)
    pub fn addGridSpan(dst: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java HashMap<Integer, Integer> value = (HashMap)dst.getValue(372);
      let mut spanMap = dst.getValue(372)
        .and_then(|v| v.as_ref().as_any().downcast_ref::<HashMap<i32, i32>>().cloned())
        .unwrap_or_default();
      // java HashMap<Integer, Integer> spanMap = value;
      // java if (null == value)
      // java spanMap = new HashMap();
      // java dst.setValue(372, spanMap);
      // java Integer intValue = ConvertHelper.toInteger(attributes);
      let intValue = ConvertHelper::toInteger(attributes);
      // java if (null != intValue)
      if let Some(intValue) = intValue {
        // java spanMap.put(cellIndex, intValue);
        spanMap.insert(cellIndex, intValue);
      }
      dst.setValue(372, Some(Arc::new(spanMap)));
    }

    // java private static void setDefShd(MutablePropertySet dst, Attributes attributes, int cellIndex)
    pub fn setDefShd(dst: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java IOAssert.assertNotNull("dst should not be null!", dst);
      IOAssert::assertNotNull__string_object(&"dst should not be null!", Some(&*dst));
      // java Object value = dst.getValue(308);
      // java HashMap<Integer, SHD> shdMap = (HashMap)value;
      let mut shdMap = dst.getValue(308)
        .and_then(|v| v.as_ref().as_any().downcast_ref::<HashMap<i32, SHD>>().cloned())
        .unwrap_or_default();
      // java if (null == shdMap)
      // java shdMap = new HashMap();
      // java dst.setValue(308, shdMap);
      // java IOAssert.assertTrue(shdMap instanceof HashMap);
      IOAssert::assertTrue_msg("shdMap instanceof HashMap", true);
      // java shdMap.put(cellIndex, ConvertHelper.toSHD(attributes));
      shdMap.insert(cellIndex, ConvertHelper::toSHD(attributes));
      dst.setValue(308, Some(Arc::new(shdMap)));
    }

    // java private void setTcBorderTop(MutablePropertySet dst, int cellIndex, Attributes attributes)
    pub fn setTcBorderTop(&self, dst: &mut MutablePropertySet, cellIndex: i32, attributes: &dyn Attributes) {
      // java TableCellBuilder tableCell = getTableCell(dst, cellIndex);
      // java tableCell.setBrcTop(ConvertHelper.toBRC(attributes, this.isXml03()));
      let brc = ConvertHelper::toBRC(attributes, self.isXml03());
      Self::update_cell(dst, cellIndex, |tableCell| tableCell.setBrcTop(&brc));
    }

    // java private void setTcBorderStart(MutablePropertySet dst, int cellIndex, Attributes attributes)
    pub fn setTcBorderStart(&self, dst: &mut MutablePropertySet, cellIndex: i32, attributes: &dyn Attributes) {
      // java TableCellBuilder tableCell = getTableCell(dst, cellIndex);
      // java if (null == tableCell.getBrcLeft())
      // java tableCell.setBrcLeft(ConvertHelper.toBRC(attributes, this.isXml03()));
      let brc = ConvertHelper::toBRC(attributes, self.isXml03());
      Self::update_cell(dst, cellIndex, |tableCell| {
        if tableCell.getBrcLeft().is_none() {
          tableCell.setBrcLeft(&brc);
        }
      });
    }

    // java private void setTcBorderLeft(MutablePropertySet dst, int cellIndex, Attributes attributes)
    pub fn setTcBorderLeft(&self, dst: &mut MutablePropertySet, cellIndex: i32, attributes: &dyn Attributes) {
      // java TableCellBuilder tableCell = getTableCell(dst, cellIndex);
      // java tableCell.setBrcLeft(ConvertHelper.toBRC(attributes, this.isXml03()));
      let brc = ConvertHelper::toBRC(attributes, self.isXml03());
      Self::update_cell(dst, cellIndex, |tableCell| tableCell.setBrcLeft(&brc));
    }

    // java private void setTcBorderBottom(MutablePropertySet dst, int cellIndex, Attributes attributes)
    pub fn setTcBorderBottom(&self, dst: &mut MutablePropertySet, cellIndex: i32, attributes: &dyn Attributes) {
      // java TableCellBuilder tableCell = getTableCell(dst, cellIndex);
      // java tableCell.setBrcBottom(ConvertHelper.toBRC(attributes, this.isXml03()));
      let brc = ConvertHelper::toBRC(attributes, self.isXml03());
      Self::update_cell(dst, cellIndex, |tableCell| tableCell.setBrcBottom(&brc));
    }

    // java private void setTcBorderEnd(MutablePropertySet dst, int cellIndex, Attributes attributes)
    pub fn setTcBorderEnd(&self, dst: &mut MutablePropertySet, cellIndex: i32, attributes: &dyn Attributes) {
      // java TableCellBuilder tableCell = getTableCell(dst, cellIndex);
      // java if (null == tableCell.getBrcRight())
      // java tableCell.setBrcRight(ConvertHelper.toBRC(attributes, this.isXml03()));
      let brc = ConvertHelper::toBRC(attributes, self.isXml03());
      Self::update_cell(dst, cellIndex, |tableCell| {
        if tableCell.getBrcRight().is_none() {
          tableCell.setBrcRight(&brc);
        }
      });
    }

    // java private void setTcBorderRight(MutablePropertySet dst, int cellIndex, Attributes attributes)
    pub fn setTcBorderRight(&self, dst: &mut MutablePropertySet, cellIndex: i32, attributes: &dyn Attributes) {
      // java TableCellBuilder tableCell = getTableCell(dst, cellIndex);
      // java tableCell.setBrcRight(ConvertHelper.toBRC(attributes, this.isXml03()));
      let brc = ConvertHelper::toBRC(attributes, self.isXml03());
      Self::update_cell(dst, cellIndex, |tableCell| tableCell.setBrcRight(&brc));
    }

    // java private static TableCellBuilder getTableCell(MutablePropertySet dst, int cellIndex)
    pub fn getTableCell(dst: &mut MutablePropertySet, cellIndex: i32) -> TableCellBuilder {
      // java HashMap<Integer, TableCellBuilder> tableCellMap = (HashMap)dst.getValue(373);
      let mut tableCellMap = Self::cell_map(dst);
      // java if (null == tableCellMap)
      // java tableCellMap = new HashMap();
      // java dst.setValue(373, tableCellMap);
      // java TableCellBuilder tableCell = (TableCellBuilder)tableCellMap.get(cellIndex);
      if !tableCellMap.contains_key(&cellIndex) {
        // java if (null == tableCell)
        // java tableCell = getNewTableCell();
        // java tableCellMap.put(cellIndex, tableCell);
        tableCellMap.insert(cellIndex, Self::getNewTableCell());
        dst.setValue(373, Some(Arc::new(tableCellMap.clone())));
      }
      // java return tableCell;
      tableCellMap.remove(&cellIndex).unwrap_or_else(Self::getNewTableCell)
    }

    // java static TableCellBuilder getNewTableCell()
    pub fn getNewTableCell() -> TableCellBuilder {
      // java TableCellBuilder tableCell = TableCellBuilder.obtain();
      let mut tableCell = TableCellBuilder::obtain();
      // java tableCell.setFitText(false);
      tableCell.setFitText(false);
      // java return tableCell;
      tableCell
    }

    // java private static ArrayList<TableSetBRC> getTableSetBRC(MutablePropertySet dst)
    pub fn getTableSetBRC(dst: &MutablePropertySet) -> Vec<TableSetBRC> {
      // java ArrayList<TableSetBRC> list = (ArrayList)dst.getValue(311);
      // java if (null == list)
      // java list = new ArrayList();
      // java dst.setValue(311, list);
      // java return list;
      dst.getValue(311)
        .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<TableSetBRC>>().cloned())
        .unwrap_or_default()
    }

    // java private static TableSetBRC toTableSetBRC(int index, BRC brc)
    pub fn toTableSetBRC(index: i32, brc: &BRC) -> TableSetBRC {
      // java TableSetBRC ret = new TableSetBRC();
      let mut ret = TableSetBRC::new();
      // java ret.setRange(index, index + 1);
      ret.setRange(index, index + 1);
      // java ret.setBRC(brc);
      ret.setBRC(Some(brc));
      // java return ret;
      ret
    }

    // java private void setTableBorderTl2br(MutablePropertySet dst, Attributes attributes, int cellIndex)
    pub fn setTableBorderTl2br(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java TableSetBRC tableSetBRC = toTableSetBRC(cellIndex, ConvertHelper.toBRC(attributes, this.isXml03()));
      let mut tableSetBRC = Self::toTableSetBRC(cellIndex, &ConvertHelper::toBRC(attributes, self.isXml03()));
      // java tableSetBRC.setBordersToApply(16);
      tableSetBRC.setBordersToApply(16);
      // java ArrayList<TableSetBRC> list = getTableSetBRC(dst);
      let mut list = Self::getTableSetBRC(dst);
      // java list.add(tableSetBRC);
      list.push(tableSetBRC);
      dst.setValue(311, Some(Arc::new(list)));
    }

    // java private void setTableBorderTr2bl(MutablePropertySet dst, Attributes attributes, int cellIndex)
    pub fn setTableBorderTr2bl(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java TableSetBRC tableSetBRC = toTableSetBRC(cellIndex, ConvertHelper.toBRC(attributes, this.isXml03()));
      let mut tableSetBRC = Self::toTableSetBRC(cellIndex, &ConvertHelper::toBRC(attributes, self.isXml03()));
      // java tableSetBRC.setBordersToApply(32);
      tableSetBRC.setBordersToApply(32);
      // java ArrayList<TableSetBRC> list = getTableSetBRC(dst);
      let mut list = Self::getTableSetBRC(dst);
      // java list.add(tableSetBRC);
      list.push(tableSetBRC);
      dst.setValue(311, Some(Arc::new(list)));
    }

    // java private void setTextFlow(MutablePropertySet dstTableProps, Attributes attributes, int cellIndex)
    pub fn setTextFlow(&self, dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java String str = ConvertHelper.getString(attributes);
      let str = ConvertHelper::getString(attributes);
      // java TableCellBuilder tableCell = getTableCell(dstTableProps, cellIndex);
      // java tableCell.setTextFlow(ConvertHelper.toTextFlow(str, this.isXml03()));
      let flow = ConvertHelper::toTextFlow(str.as_deref().unwrap_or(""), self.isXml03());
      Self::update_cell(dstTableProps, cellIndex, |tableCell| tableCell.setTextFlow(flow));
    }

    // java private static void setVertAlign(MutablePropertySet dstTableProps, Attributes attributes, int cellIndex)
    pub fn setVertAlign(dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java String str = ConvertHelper.getString(attributes);
      let str = ConvertHelper::getString(attributes);
      // java TableCellBuilder tableCell = getTableCell(dstTableProps, cellIndex);
      // java tableCell.setVertAlign(toVertAlign(str));
      let align = Self::toVertAlign(str.as_deref());
      Self::update_cell(dstTableProps, cellIndex, |tableCell| tableCell.setVertAlign(align));
    }

    // java public static int toVertAlign(String vertJustifition)
    pub fn toVertAlign(vertJustifition: Option<&str>) -> i32 {
      // java if (null != vertJustifition && vertJustifition.length() != 0)
      if let Some(vertJustifition) = vertJustifition {
        if !vertJustifition.is_empty() {
          // java if (vertJustifition.equalsIgnoreCase("top"))
          if vertJustifition.eq_ignore_ascii_case("top") {
            // java return 0;
            return 0;
          }
          // java else if (vertJustifition.equalsIgnoreCase("center"))
          else if vertJustifition.eq_ignore_ascii_case("center") {
            // java return 1;
            return 1;
          }
          // java else if (vertJustifition.equalsIgnoreCase("both"))
          else if vertJustifition.eq_ignore_ascii_case("both") {
            // java return 0;
            return 0;
          }
          // java else if (vertJustifition.equalsIgnoreCase("bottom"))
          else if vertJustifition.eq_ignore_ascii_case("bottom") {
            // java return 2;
            return 2;
          }
          // java else
          else {
            // java IOAssert.fail("It should not reach here!");
            IOAssert::fail__string("It should not reach here!");
            // java return 0;
            return 0;
          }
        }
      }
      // java else
      // java return 0;
      return 0;
    }

    // java private static void setVertMerge(MutablePropertySet dstTableProps, Attributes attributes, int cellIndex)
    pub fn setVertMerge(dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java String str = ConvertHelper.getString(attributes);
      let str = ConvertHelper::getString(attributes);
      // java TableCellBuilder tableCell = getTableCell(dstTableProps, cellIndex);
      // java tableCell.setVertMerge(toVertMerge(str));
      let merge = Self::toVertMerge(str.as_deref());
      Self::update_cell(dstTableProps, cellIndex, |tableCell| tableCell.setVertMerge(merge));
    }

    // java private static int toVertMerge(String vertMerge)
    pub fn toVertMerge(vertMerge: Option<&str>) -> i32 {
      // java if (null != vertMerge && vertMerge.length() != 0)
      if let Some(vertMerge) = vertMerge {
        if !vertMerge.is_empty() {
          // java if (vertMerge.equalsIgnoreCase("continue"))
          if vertMerge.eq_ignore_ascii_case("continue") {
            // java return 1;
            return 1;
          }
          // java else if (vertMerge.equalsIgnoreCase("restart"))
          else if vertMerge.eq_ignore_ascii_case("restart") {
            // java return 3;
            return 3;
          }
          // java else
          else {
            // java IOAssert.fail("It should not reach here!");
            IOAssert::fail__string("It should not reach here!");
            // java return 0;
            return 0;
          }
        }
      }
      // java else
      // java return 1;
      return 1;
    }

    // java private static void setCellFitText(MutablePropertySet dstTableProps, Attributes attributes, int cellIndex)
    pub fn setCellFitText(dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java TableCellBuilder tableCell = getTableCell(dstTableProps, cellIndex);
      // java tableCell.setFitText(ConvertHelper.toBoolean(attributes));
      let fit = ConvertHelper::toBoolean(attributes);
      Self::update_cell(dstTableProps, cellIndex, |tableCell| tableCell.setFitText(fit));
    }

    // java private static void setTcNoWrap(MutablePropertySet dstTableProps, Attributes attributes, int cellIndex)
    pub fn setTcNoWrap(dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java TableCellBuilder tableCell = getTableCell(dstTableProps, cellIndex);
      // java tableCell.setNoWrap(ConvertHelper.toBoolean(attributes));
      let noWrap = ConvertHelper::toBoolean(attributes);
      Self::update_cell(dstTableProps, cellIndex, |tableCell| tableCell.setNoWrap(noWrap));
    }

    // java private void setTcWidth(MutablePropertySet dstTableProps, Attributes attributes, int cellIndex)
    pub fn setTcWidth(&self, dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes, cellIndex: i32) {
      // java TableLength tableLength = this.mTablePropsConverter.toTableLength(attributes);
      let tableLength = self.table_props().toTableLength(attributes);
      // java TableCellBuilder tableCell = getTableCell(dstTableProps, cellIndex);
      // java if (tableLength != null)
      if let Some(tableLength) = tableLength {
        // java tableCell.setWidth(tableLength.getValue());
        // java tableCell.setWidthType(tableLength.getType());
        let width = tableLength.getValue();
        let widthType = tableLength.getType();
        Self::update_cell(dstTableProps, cellIndex, |tableCell| {
          tableCell.setWidth(width);
          tableCell.setWidthType(widthType);
        });
      }
    }

    // java public void convertTcStylePr(int elementId, Attributes attributes, MutablePropertySet dst)
    pub fn convertTcStylePr(&self, elementId: i32, attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
      // java switch (elementId)
      // java break
      match elementId {
        // java case -1040545365:
        -1040545365 => {
          // java dst.setBoolean(356, ConvertHelper.toBoolean(attributes));
          dst.setBoolean(356, ConvertHelper::toBoolean(attributes));
        }
        // java case -853376977:
        -853376977 => {
          // java dst.setInt(344, toVertAlign(ConvertHelper.getString(attributes)));
          let str = ConvertHelper::getString(attributes);
          dst.setInt(344, Self::toVertAlign(str.as_deref()));
        }
        // java case 31:
        31 => {
          // java addCellMargin4Style(dst, this.mTablePropsConverter.toTopCellMargin(0, attributes));
          let margin = self.table_props().toTopCellMargin__int_attributes(0, attributes);
          Self::addCellMargin4Style(dst, &margin);
        }
        // java case 33:
        33 => {
          // java addCellMargin4Style(dst, this.mTablePropsConverter.toLeftCellMargin(0, attributes));
          let margin = self.table_props().toLeftCellMargin__int_attributes(0, attributes);
          Self::addCellMargin4Style(dst, &margin);
        }
        // java case 34:
        34 => {
          // java addCellMargin4Style(dst, this.mTablePropsConverter.toBottomCellMargin(0, attributes));
          let margin = self.table_props().toBottomCellMargin__int_attributes(0, attributes);
          Self::addCellMargin4Style(dst, &margin);
        }
        // java case 36:
        36 => {
          // java addCellMargin4Style(dst, this.mTablePropsConverter.toRightCellMargin(0, attributes));
          let margin = self.table_props().toRightCellMargin__int_attributes(0, attributes);
          Self::addCellMargin4Style(dst, &margin);
        }
        // java case 37:
        37 => {
          // java dst.setValue(346, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(346, Some(Arc::new(ConvertHelper::toBRC(attributes, self.isXml03()))));
        }
        // java case 38:
        38 => {
          // java if (!dst.contains(347))
          if !dst.contains(347) {
            // java dst.setValue(347, ConvertHelper.toBRC(attributes, this.isXml03()));
            dst.setValue(347, Some(Arc::new(ConvertHelper::toBRC(attributes, self.isXml03()))));
          }
        }
        // java case 39:
        39 => {
          // java dst.setValue(347, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(347, Some(Arc::new(ConvertHelper::toBRC(attributes, self.isXml03()))));
        }
        // java case 40:
        40 => {
          // java dst.setValue(348, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(348, Some(Arc::new(ConvertHelper::toBRC(attributes, self.isXml03()))));
        }
        // java case 41:
        41 => {
          // java if (!dst.contains(349))
          if !dst.contains(349) {
            // java dst.setValue(349, ConvertHelper.toBRC(attributes, this.isXml03()));
            dst.setValue(349, Some(Arc::new(ConvertHelper::toBRC(attributes, self.isXml03()))));
          }
        }
        // java case 42:
        42 => {
          // java dst.setValue(349, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(349, Some(Arc::new(ConvertHelper::toBRC(attributes, self.isXml03()))));
        }
        // java case 113839:
        113839 => {
          // java setTcStylePrShd(attributes, dst);
          Self::setTcStylePrShd(attributes, dst);
        }
        // java case 110397066:
        110397066 => {
          // java dst.setValue(327, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(327, Some(Arc::new(ConvertHelper::toBRC(attributes, self.isXml03()))));
        }
        // java case 110575806:
        110575806 => {
          // java dst.setValue(328, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(328, Some(Arc::new(ConvertHelper::toBRC(attributes, self.isXml03()))));
        }
        // java case 1957244876:
        1957244876 => {
          // java dst.setValue(350, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(350, Some(Arc::new(ConvertHelper::toBRC(attributes, self.isXml03()))));
        }
        // java case 1957244890:
        1957244890 => {
          // java dst.setValue(351, ConvertHelper.toBRC(attributes, this.isXml03()));
          dst.setValue(351, Some(Arc::new(ConvertHelper::toBRC(attributes, self.isXml03()))));
        }
        _ => {
        }
      }
    }

    // java private static void setTcStylePrShd(Attributes attributes, MutablePropertySet dst)
    pub fn setTcStylePrShd(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
      // java SHD tcShd = ConvertHelper.toSHD(attributes);
      let tcShd = ConvertHelper::toSHD(attributes);
      // java dst.setValue(345, tcShd);
      dst.setValue(345, Some(Arc::new(tcShd.clone())));
      // java dst.setValue(309, tcShd);
      dst.setValue(309, Some(Arc::new(tcShd)));
    }

    // java private static void addCellMargin4Normal(MutablePropertySet dst, CellMargin cellMargin)
    pub fn addCellMargin4Normal(dst: &mut MutablePropertySet, cellMargin: &CellMargin) {
      // java addCellMargin(dst, 300, cellMargin);
      Self::addCellMargin(dst, 300, cellMargin);
    }

    // java private static void addCellMargin4Style(MutablePropertySet dst, CellMargin cellMargin)
    pub fn addCellMargin4Style(dst: &mut MutablePropertySet, cellMargin: &CellMargin) {
      // java addCellMargin(dst, 355, cellMargin);
      Self::addCellMargin(dst, 355, cellMargin);
    }

    // java private static void addCellMargin(MutablePropertySet dst, int propId, CellMargin cellMargin)
    pub fn addCellMargin(dst: &mut MutablePropertySet, propId: i32, cellMargin: &CellMargin) {
      // java IOAssert.assertNotNull("dst should not be null!", dst);
      IOAssert::assertNotNull__string_object(&"dst should not be null!", Some(&*dst));
      // java IOAssert.assertNotNull("cellMargin should not be null!", cellMargin);
      IOAssert::assertNotNull__string_object(&"cellMargin should not be null!", Some(cellMargin));
      // java ArrayList<CellMargin> listCellMargin = (ArrayList)dst.getValue(propId);
      let mut listCellMargin = dst.getValue(propId)
        .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<CellMargin>>().cloned())
        .unwrap_or_default();
      // java if (null == listCellMargin)
      // java listCellMargin = new ArrayList();
      // java dst.setValue(propId, listCellMargin);
      // java listCellMargin.add(cellMargin);
      listCellMargin.push(cellMargin.clone());
      dst.setValue(propId, Some(Arc::new(listCellMargin)));
    }

}

impl std::ops::Deref for CellPropertyConverterSimple {
    type Target = ImporterBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CellPropertyConverterSimple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
