// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.base.io.TypeConversion
use util::base::io::TypeConversion::TypeConversion;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import cn.wps.moffice.writer.data.value.CellMargin
use crate::cn::wps::moffice::writer::data::value::CellMargin::CellMargin;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
// Java: import cn.wps.moffice.writer.data.value.TableBorders
use crate::cn::wps::moffice::writer::data::value::TableBorders::TableBorders;
// Java: import cn.wps.moffice.writer.data.value.TableLength
use crate::cn::wps::moffice::writer::data::value::TableLength::TableLength;
// Java: import cn.wps.moffice.writer.data.value.TableLook
use crate::cn::wps::moffice::writer::data::value::TableLook::TableLook;
// Java: import cn.wps.moffice.writer.data.value.TableLookBuilder
use crate::cn::wps::moffice::writer::data::value::TableLookBuilder::TableLookBuilder;
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::ImporterBase;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
// Java: import cn.wps.moffice.writer.io.reader.helper.TablePropCorrector
use crate::cn::wps::moffice::writer::io::reader::helper::TablePropCorrector::TablePropCorrector;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::Xml03Context;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
// Java: import java.util.ArrayList
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::{Attributes, ConvertHelper};
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ImporterHelper::ImporterHelper;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::ShareConvertHelper;
use std::sync::Arc;
// Java: import java.lang.Object
use stubs::java::lang::Object::Object;

// java public class TablePropertyBaseConverterSimple extends ImporterBase
#[derive(Default, Clone)]
pub struct TablePropertyBaseConverterSimple {
    // java ImporterBase super_
    pub(crate) super_: ImporterBase,
}

// java public class TablePropertyBaseConverterSimple extends ImporterBase
impl TablePropertyBaseConverterSimple {
    // java public  TablePropertyBaseConverterSimple(int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn TablePropertyBaseConverterSimple(mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(mode, xml07Context, xml03Context);
      Self { super_: ImporterBase::ImporterBase(mode, xml07Context, xml03Context) }
    }

    // java public void toCoreTableProps(MutablePropertySet dstTableProps, int elementId, Attributes attributes, StyleId2Sti styleId2Sti)
    pub fn toCoreTableProps(&self, dstTableProps: &mut MutablePropertySet, elementId: i32, attributes: &dyn Attributes, styleId2Sti: &StyleId2Sti) {
      // java IOAssert.assertNotNull("dstTableProps should not be null", dstTableProps);
      IOAssert::assertNotNull__string_object(&"dstTableProps should not be null", Some(&dstTableProps));
      // java IOAssert.assertNotNull("tablePropertyBase should not be null", attributes);
      IOAssert::assertNotNull__string_object(&"tablePropertyBase should not be null", Some(&0i32));
      // java IOAssert.assertNotNull("styleId2Sti should not be null", styleId2Sti);
      IOAssert::assertNotNull__string_object(&"styleId2Sti should not be null", Some(&styleId2Sti));
      // java switch (elementId)
      // java break
      match elementId {
        // java case -1986705949:
        -1986705949 => {
          // java dstTableProps.setValue(299, ConvertHelper.toCellSpacing(attributes));
          dstTableProps.setValue(299, Some(Arc::new(ConvertHelper::toCellSpacing(attributes))));
        }
        // java case -1515984003:
        -1515984003 => {
          // java this.convertTableLook(dstTableProps, attributes);
          self.convertTableLook(dstTableProps, attributes);
        }
        // java case -1042798877:
        -1042798877 => {
          // java this.convertInt(358, dstTableProps, attributes);
          self.convertInt(358, dstTableProps, attributes);
        }
        // java case -895426488:
        -895426488 => {
          // java this.convertString(352, dstTableProps, attributes);
          self.convertString(352, dstTableProps, attributes);
        }
        // java case -880189631:
        -880189631 => {
          // java dstTableProps.setValue(298, this.toTableLength(attributes));
          dstTableProps.setValue(298, self.toTableLength(attributes).map(|v| Arc::new(v) as _));
        }
        // java case -880153068:
        -880153068 => {
          // java this.setFloatTablePosition(dstTableProps, attributes);
          self.setFloatTablePosition(dstTableProps, attributes);
        }
        // java case -879337336:
        -879337336 => {
          // java dstTableProps.setBoolean(302, this.toFit(attributes));
          dstTableProps.setBoolean(302, self.toFit(attributes));
        }
        // java case -784869634: default:
        -784869634 => {
        }
        // java case -634428099:
        -634428099 => {
          // java this.convertInt(357, dstTableProps, attributes);
          self.convertInt(357, dstTableProps, attributes);
        }
        // java case -499594914:
        -499594914 => {
          // java this.convertString(353, dstTableProps, attributes);
          self.convertString(353, dstTableProps, attributes);
        }
        // java case 31:
        31 => {
          // java this.setDefCellMargin(dstTableProps, this.toTopCellMargin(attributes));
          self.setDefCellMargin(dstTableProps, &self.toTopCellMargin__attributes(attributes));
        }
        // java case 32: case 33:
        32 | 33 => {
          // java this.setDefCellMargin(dstTableProps, this.toLeftCellMargin(attributes));
          self.setDefCellMargin(dstTableProps, &self.toLeftCellMargin__attributes(attributes));
        }
        // java case 34:
        34 => {
          // java this.setDefCellMargin(dstTableProps, this.toBottomCellMargin(attributes));
          self.setDefCellMargin(dstTableProps, &self.toBottomCellMargin__attributes(attributes));
        }
        // java case 35: case 36:
        35 | 36 => {
          // java this.setDefCellMargin(dstTableProps, this.toRightCellMargin(attributes));
          self.setDefCellMargin(dstTableProps, &self.toRightCellMargin__attributes(attributes));
        }
        // java case 37:
        37 => {
          // java this.setTableBorders(dstTableProps, 0, attributes);
          self.setTableBorders(dstTableProps, 0, attributes);
        }
        // java case 38:
        38 => {
          // java this.setTableBordersStart(dstTableProps, attributes);
          self.setTableBordersStart(dstTableProps, attributes);
        }
        // java case 39:
        39 => {
          // java this.setTableBorders(dstTableProps, 1, attributes);
          self.setTableBorders(dstTableProps, 1, attributes);
        }
        // java case 40:
        40 => {
          // java this.setTableBorders(dstTableProps, 2, attributes);
          self.setTableBorders(dstTableProps, 2, attributes);
        }
        // java case 41:
        41 => {
          // java this.setTableBordersEnd(dstTableProps, attributes);
          self.setTableBordersEnd(dstTableProps, attributes);
        }
        // java case 42:
        42 => {
          // java this.setTableBorders(dstTableProps, 3, attributes);
          self.setTableBorders(dstTableProps, 3, attributes);
        }
        // java case 3385:
        3385 => {
          // java dstTableProps.setInt(303, this.toTableJC(attributes));
          dstTableProps.setInt(303, self.toTableJC(attributes));
        }
        // java case 113839:
        113839 => {
          // java dstTableProps.setValue(309, ConvertHelper.toSHD(attributes));
          dstTableProps.setValue(309, Some(Arc::new(ConvertHelper::toSHD(attributes))));
        }
        // java case 3553369:
        3553369 => {
          // java dstTableProps.setValue(295, this.toTableLength(attributes));
          dstTableProps.setValue(295, self.toTableLength(attributes).map(|v| Arc::new(v) as _));
        }
        // java case 96307760:
        96307760 => {
          // java ConvertHelper.convertTableRevision(dstTableProps, attributes);
          ConvertHelper::convertTableRevision(dstTableProps, attributes);
        }
        // java case 255759507:
        255759507 => {
          // java this.convertTableStyle(dstTableProps, attributes, styleId2Sti);
          self.convertTableStyle(dstTableProps, attributes, styleId2Sti);
        }
        // java case 860953964:
        860953964 => {
          // java dstTableProps.setValue(343, ConvertHelper.toBoolean(attributes));
          dstTableProps.setValue(343, Some(Arc::new(ConvertHelper::toBoolean(attributes))));
        }
        // java case 1755679401:
        1755679401 => {
          // java dstTableProps.setBoolean(319, this.convertCantOverlap(attributes));
          dstTableProps.setBoolean(319, self.convertCantOverlap(attributes));
        }
        // java case 1957244876:
        1957244876 => {
          // java this.setTableBorders(dstTableProps, 4, attributes);
          self.setTableBorders(dstTableProps, 4, attributes);
        }
        // java case 1957244890:
        1957244890 => {
          // java this.setTableBorders(dstTableProps, 5, attributes);
          self.setTableBorders(dstTableProps, 5, attributes);
        }
        // java case 1998672067:
        1998672067 => {
          // java ConvertHelper.convertTableRevision(dstTableProps, attributes);
          ConvertHelper::convertTableRevision(dstTableProps, attributes);
        }
        _ => {
        }
      }
    }

    // java public void convertRowProp(TextDocument doc, int elementId, MutablePropertySet dstTableProps, Attributes attributes)
    pub fn convertRowProp(&self, doc: Option<*mut TextDocument>, elementId: i32, dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes) {
      let _ = doc;
      // java IOAssert.assertNotNull("dstTableProps should not be null !", dstTableProps);
      IOAssert::assertNotNull__string_object(&"dstTableProps should not be null !", Some(&dstTableProps));
      // java IOAssert.assertNotNull("rowPropertyBase should not be null !", attributes);
      IOAssert::assertNotNull__string_object(&"rowPropertyBase should not be null !", Some(&0i32));
      // java switch (elementId)
      // java break
      match elementId {
        // java case -1986705949:
        -1986705949 => {
          // java dstTableProps.setValue(299, ConvertHelper.toCellSpacing(attributes));
          dstTableProps.setValue(299, Some(Arc::new(ConvertHelper::toCellSpacing(attributes))));
        }
        // java case -1713877738: case -1217487446: case 95595916: case 1252369974:
        -1713877738 | -1217487446 | 95595916 | 1252369974 => {
        }
        // java case -990885909:
        -990885909 => {
          // java dstTableProps.setBoolean(320, ConvertHelper.toBoolean(attributes));
          dstTableProps.setBoolean(320, ConvertHelper::toBoolean(attributes));
        }
        // java case -824916059:
        -824916059 => {
          // java dstTableProps.setValue(297, this.toTableLength(attributes));
          dstTableProps.setValue(297, self.toTableLength(attributes).map(|v| Arc::new(v) as _));
        }
        // java case 3385:
        3385 => {
          // java dstTableProps.setValue(303, this.toTableJC(attributes));
          dstTableProps.setValue(303, Some(Arc::new(self.toTableJC(attributes))));
        }
        // java case 99339:
        99339 => {
          // java ConvertHelper.convertDel(doc, dstTableProps, attributes);
          let revision = ConvertHelper::convertDelRevision(attributes);
          dstTableProps.setValue(45, Some(Arc::new(revision)));
          dstTableProps.setBoolean(54, true);
        }
        // java case 104430:
        104430 => {
          // java ConvertHelper.convertIns(doc, dstTableProps, attributes);
          let revision = ConvertHelper::convertInsRevision(attributes);
          dstTableProps.setValue(46, Some(Arc::new(revision)));
          dstTableProps.setBoolean(55, true);
        }
        // java case 196061797:
        196061797 => {
          // java Integer intValue = ConvertHelper.toInteger(attributes);
          let intValue = ConvertHelper::toInteger(attributes);
          // java if (null != intValue && intValue > 0)
          if let Some(intValue) = intValue {
            if intValue > 0 {
            // java dstTableProps.setInt(371, intValue);
            dstTableProps.setInt(371, intValue);
            }
          }
        }
        // java case 224704214:
        224704214 => {
          // java dstTableProps.setValue(296, this.toTableLength(attributes));
          dstTableProps.setValue(296, self.toTableLength(attributes).map(|v| Arc::new(v) as _));
        }
        // java case 556545349:
        556545349 => {
          // java dstTableProps.setInt(305, (int)this.toCoreRowHeight(attributes));
          dstTableProps.setInt(305, self.toCoreRowHeight(attributes) as i32);
        }
        // java case 887167216:
        887167216 => {
          // java ConvertHelper.convertTableRevision(dstTableProps, attributes);
          ConvertHelper::convertTableRevision(dstTableProps, attributes);
        }
        // java case 946779862:
        946779862 => {
          // java dstTableProps.setBoolean(304, ConvertHelper.toBoolean(attributes));
          dstTableProps.setBoolean(304, ConvertHelper::toBoolean(attributes));
        }
        // java default:
        _ => {
          // java IOAssert.fail("It should not reach here! " + Integer.toString(elementId));
          IOAssert::fail__string(&format!("It should not reach here! {}", elementId));
        }
      }
    }

    // java public void convertTableGrid(int elementId, MutablePropertySet dstTable, Attributes attributes)
    pub fn convertTableGrid(&self, elementId: i32, dstTable: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull(dstTable);
      IOAssert::assertNotNull__object(&Some(&*dstTable));
      // java switch (elementId)
      // java break
      match elementId {
        // java case 287338586:
        287338586 => {
          // java this.addGrid(dstTable, attributes);
          self.addGrid(dstTable, attributes);
        }
        // java case 1513751796:
        1513751796 => {
          // java ConvertHelper.convertTableRevision(dstTable, attributes);
          ConvertHelper::convertTableRevision(dstTable, attributes);
        }
        // java default:
        _ => {
          // java IOAssert.fail("It should not reach here!");
          IOAssert::fail__string(&"It should not reach here!");
        }
      }
    }

    // java private void convertTableLook(MutablePropertySet dstTableProps, Attributes attributes)
    pub fn convertTableLook(&self, dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java TableLook tableLook = this.toCoreTableLook(attributes);
      let tableLook = self.toCoreTableLook(attributes);
      // java if (null != tableLook)
      if let Some(tableLook) = tableLook {
        // java dstTableProps.setValue(342, tableLook);
        dstTableProps.setValue(342, Some(Arc::new(tableLook)));
      }
    }

    // java private void convertInt(int propertyId, MutablePropertySet dstTableProps, Attributes attributes)
    pub fn convertInt(&self, propertyId: i32, dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java Integer intValue = ConvertHelper.toInteger(attributes);
      let intValue = ConvertHelper::toInteger(attributes);
      // java if (null != intValue)
      if let Some(intValue) = intValue {
        // java dstTableProps.setInt(propertyId, intValue);
        dstTableProps.setInt(propertyId, intValue);
      }
    }

    // java private void convertString(int propertyId, MutablePropertySet dstTableProps, Attributes attributes)
    pub fn convertString(&self, propertyId: i32, dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java String tableDescription = ConvertHelper.getString(attributes);
      let tableDescription = ConvertHelper::getString(attributes);
      // java if (null != tableDescription)
      if let Some(tableDescription) = tableDescription {
        // java dstTableProps.setString(propertyId, tableDescription);
        dstTableProps.setString(propertyId, tableDescription);
      }
    }

    // java private void convertTableStyle(MutablePropertySet dstTableProps, Attributes attributes, StyleId2Sti styleId2Sti)
    pub fn convertTableStyle(&self, dstTableProps: &mut MutablePropertySet, attributes: &dyn Attributes, styleId2Sti: &StyleId2Sti) {
      // java String strTableStyleId = ConvertHelper.getString(attributes);
      let strTableStyleId = ConvertHelper::getString(attributes);
      // java Integer tableStyleId = null != strTableStyleId ? styleId2Sti.getSti(strTableStyleId) : null;
      let tableStyleId = strTableStyleId.as_deref().and_then(|id| styleId2Sti.getSti(id));
      // java if (null != tableStyleId)
      if let Some(tableStyleId) = tableStyleId {
        // java dstTableProps.setInt(292, tableStyleId);
        dstTableProps.setInt(292, tableStyleId);
      }
    }

    // java private void addGrid(MutablePropertySet dst, Attributes attributes)
    pub fn addGrid(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull(dst);
      IOAssert::assertNotNull__object(&Some(&*dst));
      // java ArrayList<Integer> gridList = (ArrayList)dst.getValue(306);
      let mut gridList = dst.getValue(306).and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<i32>>().cloned());
      // java if (null == gridList)
      if gridList.is_none() {
        // java gridList = new ArrayList();
        gridList = Some(Vec::new());
        // java dst.setValue(306, gridList);
        dst.setValue(306, Some(Arc::new(gridList.clone().unwrap_or_default())));
      }
      // java Integer intValue = ConvertHelper.getInt(attributes, "w");
      let intValue = ConvertHelper::getInt(attributes, "w");
      // java if (null != intValue)
      if let Some(intValue) = intValue {
        // java gridList.add(intValue);
        gridList.get_or_insert_with(Vec::new).push(intValue);
        dst.setValue(306, Some(Arc::new(gridList.unwrap_or_default())));
      }
    }

    // java private float toCoreRowHeight(Attributes attributes)
    pub fn toCoreRowHeight(&self, attributes: &dyn Attributes) -> f32 {
      // java float CORE_AUTO = 0.0F;
      let CORE_AUTO: f32 = 0.0;
      let _ = CORE_AUTO;
      // java if (null != attributes && attributes.getLength() != 0)
      if attributes.getLength() != 0 {
          // java String heightRule = ConvertHelper.getString(attributes, "hRule");
          let heightRule = ConvertHelper::getString__String(attributes, "hRule");
          // java Float value = ConvertHelper.toFloat(attributes);
          let mut value = ConvertHelper::toFloat(attributes);
          // java if (null == value)
          if value.is_none() {
            // java float DEFAULT_HEIGHT = 0.0F;
            let DEFAULT_HEIGHT: f32 = 0.0;
            let _ = DEFAULT_HEIGHT;
            // java value = 0.0F;
            value = Some(0.0);
          }
          let value = value.unwrap_or(0.0);
          // java if (null != heightRule && heightRule.length() != 0)
          if let Some(heightRule) = heightRule.as_deref() {
            if !heightRule.is_empty() {
            // java if (heightRule.equalsIgnoreCase("auto"))
            if heightRule.eq_ignore_ascii_case("auto") {
              // java return 0.0F;
              return 0.0;
            }
            // java else if (heightRule.equalsIgnoreCase("exact"))
            else if heightRule.eq_ignore_ascii_case("exact") {
              // java return -value;
              return -value;
            }
            // java else if (heightRule.equalsIgnoreCase("atLeast"))
            else if heightRule.eq_ignore_ascii_case("atLeast") {
              // java return value;
              return value;
            }
            // java else
            else {
              // java IOAssert.fail("It should not reach here!");
              IOAssert::fail__string(&"It should not reach here!");
              // java return 0.0F;
              return 0.0;
            }
            }
          }
          // java else
          // java return value;
          return value;
      }
      // java else
      // java return 0.0F;
      return 0.0;
    }

    // java private void setTableBorders(MutablePropertySet dst, int id, Attributes attributes)
    pub fn setTableBorders(&self, dst: &mut MutablePropertySet, id: i32, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null!", dst);
      IOAssert::assertNotNull__string_object(&"dst should not be null!", Some(&dst));
      // java IOAssert.assertNotNull("attributes should not be null!", attributes);
      IOAssert::assertNotNull__string_object(&"attributes should not be null!", Some(&0i32));
      // java TableBorders coreData = (TableBorders)dst.getValue(310);
      let mut coreData = dst.getValue(310).and_then(|v| v.as_ref().as_any().downcast_ref::<TableBorders>().cloned());
      // java if (null == coreData)
      if coreData.is_none() {
        // java coreData = new TableBorders();
        coreData = Some(TableBorders::new());
        // java dst.setValue(310, coreData);
        dst.setValue(310, Some(Arc::new(coreData.clone().unwrap_or_else(TableBorders::new))));
      }
      // java BRC brcTop = ConvertHelper.toBRC(attributes, this.isXml03());
      let brcTop = ConvertHelper::toBRC(attributes, self.isXml03());
      // java IOAssert.assertNotNull("brcTop should not be null!", brcTop);
      IOAssert::assertNotNull__string_object(&"brcTop should not be null!", Some(&brcTop));
      // java coreData.setBorder(id, brcTop);
      let mut coreData = coreData.unwrap_or_else(TableBorders::new);
      coreData.setBorder(id, &brcTop);
      dst.setValue(310, Some(Arc::new(coreData)));
    }

    // java private void setTableBordersStart(MutablePropertySet dst, Attributes attributes)
    pub fn setTableBordersStart(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null!", dst);
      IOAssert::assertNotNull__string_object(&"dst should not be null!", Some(&dst));
      // java IOAssert.assertNotNull("attributes should not be null!", attributes);
      IOAssert::assertNotNull__string_object(&"attributes should not be null!", Some(&0i32));
      // java TableBorders coreData = (TableBorders)dst.getValue(310);
      let coreData = dst.getValue(310).and_then(|v| v.as_ref().as_any().downcast_ref::<TableBorders>().cloned());
      // java BRC brc = null == coreData ? null : coreData.getBorder(1);
      let brc = coreData.as_ref().and_then(|core| core.getBorder(1));
      // java if (null == brc)
      if brc.is_none() {
        // java this.setTableBorders(dst, 1, attributes);
        self.setTableBorders(dst, 1, attributes);
      }
    }

    // java private void setTableBordersEnd(MutablePropertySet dst, Attributes attributes)
    pub fn setTableBordersEnd(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null!", dst);
      IOAssert::assertNotNull__string_object(&"dst should not be null!", Some(&dst));
      // java IOAssert.assertNotNull("attributes should not be null!", attributes);
      IOAssert::assertNotNull__string_object(&"attributes should not be null!", Some(&0i32));
      // java TableBorders coreData = (TableBorders)dst.getValue(310);
      let coreData = dst.getValue(310).and_then(|v| v.as_ref().as_any().downcast_ref::<TableBorders>().cloned());
      // java BRC brc = null == coreData ? null : coreData.getBorder(3);
      let brc = coreData.as_ref().and_then(|core| core.getBorder(3));
      // java if (null == brc)
      if brc.is_none() {
        // java this.setTableBorders(dst, 3, attributes);
        self.setTableBorders(dst, 3, attributes);
      }
    }

    // java  void setDefCellMargin(MutablePropertySet dst, CellMargin cellMargin)
    pub fn setDefCellMargin(&self, dst: &mut MutablePropertySet, cellMargin: &CellMargin) {
      // java IOAssert.assertNotNull("cellMargin should not be null", cellMargin);
      IOAssert::assertNotNull__string_object(&"cellMargin should not be null", Some(&cellMargin));
      // java ArrayList<CellMargin> listCellMargin = (ArrayList)dst.getValue(301);
      let mut listCellMargin = dst.getValue(301).and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<CellMargin>>().cloned());
      // java if (null == listCellMargin)
      if listCellMargin.is_none() {
        // java listCellMargin = new ArrayList();
        listCellMargin = Some(Vec::new());
        // java dst.setValue(301, listCellMargin);
        dst.setValue(301, Some(Arc::new(listCellMargin.clone().unwrap_or_default())));
      }
      // java listCellMargin.add(cellMargin);
      listCellMargin.get_or_insert_with(Vec::new).push(cellMargin.clone());
      dst.setValue(301, Some(Arc::new(listCellMargin.unwrap_or_default())));
    }

    // java private CellMargin toTopCellMargin(Attributes margin)
    pub fn toTopCellMargin__attributes(&self, margin: &dyn Attributes) -> CellMargin {
      // java return this.toTopCellMargin(0, margin);
      return self.toTopCellMargin__int_attributes(0, margin);
    }

    // java private CellMargin toLeftCellMargin(Attributes margin)
    pub fn toLeftCellMargin__attributes(&self, margin: &dyn Attributes) -> CellMargin {
      // java return this.toLeftCellMargin(0, margin);
      return self.toLeftCellMargin__int_attributes(0, margin);
    }

    // java private CellMargin toBottomCellMargin(Attributes margin)
    pub fn toBottomCellMargin__attributes(&self, margin: &dyn Attributes) -> CellMargin {
      // java return this.toBottomCellMargin(0, margin);
      return self.toBottomCellMargin__int_attributes(0, margin);
    }

    // java private CellMargin toRightCellMargin(Attributes margin)
    pub fn toRightCellMargin__attributes(&self, margin: &dyn Attributes) -> CellMargin {
      // java return this.toRightCellMargin(0, margin);
      return self.toRightCellMargin__int_attributes(0, margin);
    }

    // java public CellMargin toTopCellMargin(int cellPosition, Attributes margin)
    pub fn toTopCellMargin__int_attributes(&self, cellPosition: i32, margin: &dyn Attributes) -> CellMargin {
      // java IOAssert.assertTrue("cellPosition >= 0 should be true!", cellPosition >= 0);
      IOAssert::assertTrue_msg(&"cellPosition >= 0 should be true!", cellPosition >= 0);
      // java IOAssert.assertNotNull("margin is not null!", margin);
      IOAssert::assertNotNull__string_object(&"margin is not null!", Some(&margin));
      // java return this.toCellMargin(cellPosition, margin, 1);
      return self.toCellMargin(cellPosition, margin, 1);
    }

    // java  CellMargin toLeftCellMargin(int cellPosition, Attributes margin)
    pub fn toLeftCellMargin__int_attributes(&self, cellPosition: i32, margin: &dyn Attributes) -> CellMargin {
      // java IOAssert.assertTrue("cellPosition >= 0 should be true!", cellPosition >= 0);
      IOAssert::assertTrue_msg(&"cellPosition >= 0 should be true!", cellPosition >= 0);
      // java IOAssert.assertNotNull("margin is not null!", margin);
      IOAssert::assertNotNull__string_object(&"margin is not null!", Some(&margin));
      // java return this.toCellMargin(cellPosition, margin, 2);
      return self.toCellMargin(cellPosition, margin, 2);
    }

    // java  CellMargin toBottomCellMargin(int cellPosition, Attributes margin)
    pub fn toBottomCellMargin__int_attributes(&self, cellPosition: i32, margin: &dyn Attributes) -> CellMargin {
      // java IOAssert.assertTrue("cellPosition >= 0 should be true!", cellPosition >= 0);
      IOAssert::assertTrue_msg(&"cellPosition >= 0 should be true!", cellPosition >= 0);
      // java IOAssert.assertNotNull("margin is not null!", margin);
      IOAssert::assertNotNull__string_object(&"margin is not null!", Some(&margin));
      // java return this.toCellMargin(cellPosition, margin, 4);
      return self.toCellMargin(cellPosition, margin, 4);
    }

    // java  CellMargin toRightCellMargin(int cellPosition, Attributes margin)
    pub fn toRightCellMargin__int_attributes(&self, cellPosition: i32, margin: &dyn Attributes) -> CellMargin {
      // java IOAssert.assertTrue("cellPosition >= 0 should be true!", cellPosition >= 0);
      IOAssert::assertTrue_msg(&"cellPosition >= 0 should be true!", cellPosition >= 0);
      // java IOAssert.assertNotNull("margin is not null!", margin);
      IOAssert::assertNotNull__string_object(&"margin is not null!", Some(&margin));
      // java return this.toCellMargin(cellPosition, margin, 8);
      return self.toCellMargin(cellPosition, margin, 8);
    }

    // java private CellMargin toCellMargin(int cellPosition, Attributes attributes, int grbitChg)
    pub fn toCellMargin(&self, cellPosition: i32, attributes: &dyn Attributes, grbitChg: i32) -> CellMargin {
      // java IOAssert.assertTrue("cellPosition >= 0 should be true!", cellPosition >= 0);
      IOAssert::assertTrue_msg(&"cellPosition >= 0 should be true!", cellPosition >= 0);
      // java IOAssert.assertNotNull("margin is not null!", attributes);
      IOAssert::assertNotNull__string_object(&"margin is not null!", Some(&0i32));
      // java int unit = ConvertHelper.getStringHashCode(attributes, "type");
      let unit: i32 = ConvertHelper::getStringHashCode__String(attributes, "type");
      // java int type = ConvertHelper.toTableLengthType(unit);
      let r#type: i32 = ConvertHelper::toTableLengthType(unit);
      // java IOAssert.assertTrue("(TableLengthType.vetNil <= type && TableLengthType.vetDxa <= type) should not be false!", 0 <= type && type <= 3);
      IOAssert::assertTrue_msg(&"(TableLengthType.vetNil <= type && TableLengthType.vetDxa <= type) should not be false!", 0 <= r#type && r#type <= 3);
      // java float DEFAULT_W = 0.0F;
      let DEFAULT_W: f32 = 0.0;
      let _ = DEFAULT_W;
      // java float value = 0.0F;
      let mut value: f32 = 0.0;
      // java if (null != attributes && 1 != type && 2 != type)
      if 1 != r#type && 2 != r#type {
          // java Float getValue = ShareConvertHelper.getFloatSTMeasurementOrPercent(attributes, "w");
          let getValue = ShareConvertHelper::getFloatSTMeasurementOrPercent(attributes, "w");
          // java value = null == getValue ? 0.0F : getValue;
          value = getValue.unwrap_or(0.0);
      }
      // java return CellMargin.obtain(cellPosition, cellPosition + 1, grbitChg, type, (int)value);
      return CellMargin::obtain__int_int_int_int_int(cellPosition, cellPosition + 1, grbitChg, r#type, value as i32);
    }

    // java private void setFloatTablePosition(MutablePropertySet dst, Attributes attributes)
    pub fn setFloatTablePosition(&self, dst: &mut MutablePropertySet, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("dst should not be null!", dst);
      IOAssert::assertNotNull__string_object(&"dst should not be null!", Some(&dst));
      // java if (null != attributes && attributes.getLength() != 0)
      if attributes.getLength() != 0 {
          // java String hAnchorType = ConvertHelper.getString(attributes, "horzAnchor");
          let hAnchorType = ConvertHelper::getString__String(attributes, "horzAnchor");
          // java String vAnchorType = ConvertHelper.getString(attributes, "vertAnchor");
          let vAnchorType = ConvertHelper::getString__String(attributes, "vertAnchor");
          // java if (hAnchorType != null || vAnchorType != null)
          if hAnchorType.is_some() || vAnchorType.is_some() {
            // java dst.setValue(312, ConvertHelper.convertHorzAnchorAndVertAnchor(hAnchorType, vAnchorType));
            let position_code = ConvertHelper::convertHorzAnchorAndVertAnchor(hAnchorType.as_deref(), vAnchorType.as_deref());
            dst.setValue(312, Some(Arc::new(position_code)));
          }
          // java dst.setInt(313, this.convertPositionX(ConvertHelper.getInt(attributes, "tblpX"), ConvertHelper.getString(attributes, "tblpXSpec")));
          dst.setInt(313, self.convertPositionX(ConvertHelper::getInt(attributes, "tblpX"), ConvertHelper::getString__String(attributes, "tblpXSpec")));
          // java dst.setInt(314, this.convertPositionY(ConvertHelper.getInt(attributes, "tblpY"), ConvertHelper.getString(attributes, "tblpYSpec")));
          dst.setInt(314, self.convertPositionY(ConvertHelper::getInt(attributes, "tblpY"), ConvertHelper::getString__String(attributes, "tblpYSpec")));
          // java dst.setInt(315, this.fromtText(ConvertHelper.getFloat(attributes, "leftFromText")));
          dst.setInt(315, self.fromtText(ConvertHelper::getFloat(attributes, "leftFromText")));
          // java dst.setInt(316, this.fromtText(ConvertHelper.getFloat(attributes, "rightFromText")));
          dst.setInt(316, self.fromtText(ConvertHelper::getFloat(attributes, "rightFromText")));
          // java dst.setInt(317, this.fromtText(ConvertHelper.getFloat(attributes, "topFromText")));
          dst.setInt(317, self.fromtText(ConvertHelper::getFloat(attributes, "topFromText")));
          // java dst.setInt(318, this.fromtText(ConvertHelper.getFloat(attributes, "bottomFromText")));
          dst.setInt(318, self.fromtText(ConvertHelper::getFloat(attributes, "bottomFromText")));
      }
    }

    // java private int convertPositionX(Integer absolutePosX, String relativePosX)
    pub fn convertPositionX(&self, absolutePosX: Option<i32>, relativePosX: Option<String>) -> i32 {
      // java if (null != relativePosX)
      if let Some(relativePosX) = relativePosX.as_deref() {
        // java if (relativePosX.equalsIgnoreCase("left"))
        if relativePosX.eq_ignore_ascii_case("left") {
          // java return 0;
          return 0;
        }
        // java else if (relativePosX.equalsIgnoreCase("center"))
        else if relativePosX.eq_ignore_ascii_case("center") {
          // java return -4;
          return -4;
        }
        // java else if (relativePosX.equalsIgnoreCase("right"))
        else if relativePosX.eq_ignore_ascii_case("right") {
          // java return -8;
          return -8;
        }
        // java else if (relativePosX.equalsIgnoreCase("inside"))
        else if relativePosX.eq_ignore_ascii_case("inside") {
          // java return -12;
          return -12;
        }
        // java else if (relativePosX.equalsIgnoreCase("outside"))
        else if relativePosX.eq_ignore_ascii_case("outside") {
          // java return -16;
          return -16;
        }
        // java else
        else {
          // java IOAssert.fail("It should not reach here!");
          IOAssert::fail__string(&"It should not reach here!");
          // java return 0;
          return 0;
        }
      }
      // java else
      else {
        // java return null == absolutePosX ? 0 : absolutePosX;
        return absolutePosX.unwrap_or(0);
      }
    }

    // java private int convertPositionY(Integer absolutePosY, String relativePosY)
    pub fn convertPositionY(&self, absolutePosY: Option<i32>, relativePosY: Option<String>) -> i32 {
      // java if (null != relativePosY)
      if let Some(relativePosY) = relativePosY.as_deref() {
        // java if (relativePosY.equalsIgnoreCase("inline"))
        if relativePosY.eq_ignore_ascii_case("inline") {
          // java return 0;
          return 0;
        }
        // java else if (relativePosY.equalsIgnoreCase("top"))
        else if relativePosY.eq_ignore_ascii_case("top") {
          // java return -4;
          return -4;
        }
        // java else if (relativePosY.equalsIgnoreCase("center"))
        else if relativePosY.eq_ignore_ascii_case("center") {
          // java return -8;
          return -8;
        }
        // java else if (relativePosY.equalsIgnoreCase("bottom"))
        else if relativePosY.eq_ignore_ascii_case("bottom") {
          // java return -12;
          return -12;
        }
        // java else if (relativePosY.equalsIgnoreCase("inside"))
        else if relativePosY.eq_ignore_ascii_case("inside") {
          // java return -16;
          return -16;
        }
        // java else if (relativePosY.equalsIgnoreCase("outside"))
        else if relativePosY.eq_ignore_ascii_case("outside") {
          // java return -20;
          return -20;
        }
        // java else
        else {
          // java IOAssert.fail("It should not reach here!" + relativePosY);
          IOAssert::fail__string(&format!("It should not reach here!{}", relativePosY));
          // java return 0;
          return 0;
        }
      }
      // java else
      else {
        // java return null == absolutePosY ? 0 : absolutePosY;
        return absolutePosY.unwrap_or(0);
      }
    }

    // java private int fromtText(Float value)
    pub fn fromtText(&self, value: Option<f32>) -> i32 {
      // java int DEFAULT_RIGHT_FROM_TEXT = 0;
      let DEFAULT_RIGHT_FROM_TEXT: i32 = 0;
      let _ = DEFAULT_RIGHT_FROM_TEXT;
      // java return null != value ? value.intValue() : 0;
      return value.map(|v| v as i32).unwrap_or(0);
    }

    // java private boolean convertCantOverlap(Attributes attributes)
    pub fn convertCantOverlap(&self, attributes: &dyn Attributes) -> bool {
      // java String type = ConvertHelper.getString(attributes);
      let r#type = ConvertHelper::getString(attributes);
      // java if (null == type)
      if r#type.is_none() {
        // java return false;
        return false;
      }
      // java else if (type.equalsIgnoreCase("never"))
      else if r#type.as_deref().unwrap_or("").eq_ignore_ascii_case("never") {
        // java return true;
        return true;
      }
      // java else if (type.equalsIgnoreCase("overlap"))
      else if r#type.as_deref().unwrap_or("").eq_ignore_ascii_case("overlap") {
        // java return false;
        return false;
      }
      // java else
      else {
        // java IOAssert.fail("It should not reach here!");
        IOAssert::fail__string(&"It should not reach here!");
        // java return true;
        return true;
      }
    }

    // java  TableLook toCoreTableLook(Attributes attributes)
    pub fn toCoreTableLook(&self, attributes: &dyn Attributes) -> Option<TableLook> {
      // java if (attributes.getLength() == 0)
      if attributes.getLength() == 0 {
        // java return null;
        return None;
      }
      // java else
      else {
        // java TableLookBuilder ret = new TableLookBuilder();
        let mut ret: TableLookBuilder = TableLookBuilder::new();
        // java Boolean firstRow = ConvertHelper.getBoolean(attributes, "firstRow");
        let firstRow = ConvertHelper::getBoolean(attributes, "firstRow");
        // java if (null != firstRow)
        if let Some(firstRow) = firstRow {
          // java ret.setFirstRow(firstRow);
          ret.setFirstRow(firstRow);
        }
        // java Boolean lastRow = ConvertHelper.getBoolean(attributes, "lastRow");
        let lastRow = ConvertHelper::getBoolean(attributes, "lastRow");
        // java if (null != lastRow)
        if let Some(lastRow) = lastRow {
          // java ret.setLastRow(lastRow);
          ret.setLastRow(lastRow);
        }
        // java Boolean firstColumn = ConvertHelper.getBoolean(attributes, "firstColumn");
        let firstColumn = ConvertHelper::getBoolean(attributes, "firstColumn");
        // java if (null != firstColumn)
        if let Some(firstColumn) = firstColumn {
          // java ret.setFirstCol(firstColumn);
          ret.setFirstCol(firstColumn);
        }
        // java Boolean lastColumn = ConvertHelper.getBoolean(attributes, "lastColumn");
        let lastColumn = ConvertHelper::getBoolean(attributes, "lastColumn");
        // java if (null != lastColumn)
        if let Some(lastColumn) = lastColumn {
          // java ret.setLastCol(lastColumn);
          ret.setLastCol(lastColumn);
        }
        // java Boolean noHBand = ConvertHelper.getBoolean(attributes, "noHBand");
        let noHBand = ConvertHelper::getBoolean(attributes, "noHBand");
        // java if (null != noHBand)
        if let Some(noHBand) = noHBand {
          // java ret.setNoHBand(noHBand);
          ret.setNoHBand(noHBand);
        }
        // java Boolean noVBand = ConvertHelper.getBoolean(attributes, "noVBand");
        let noVBand = ConvertHelper::getBoolean(attributes, "noVBand");
        // java if (null != noVBand)
        if let Some(noVBand) = noVBand {
          // java ret.setNoVBand(noVBand);
          ret.setNoVBand(noVBand);
        }
        // java if (null == firstRow && null == lastRow && null == firstColumn && null == lastColumn && null == noHBand && null == noVBand)
        if firstRow.is_none() && lastRow.is_none() && firstColumn.is_none() && lastColumn.is_none() && noHBand.is_none() && noVBand.is_none() {
          // java this.setTableLook(attributes, ret);
          self.setTableLook(attributes, &mut ret);
          // java return ret.freeze();
          return Some(ret.freeze());
        }
        // java else
        else {
          // java return ret.freeze();
          return Some(ret.freeze());
        }
      }
    }

    // java private void setTableLook(Attributes attributes, TableLookBuilder ret)
    pub fn setTableLook(&self, attributes: &dyn Attributes, ret: &mut TableLookBuilder) {
      // java String value = ConvertHelper.getString(attributes);
      let value = ConvertHelper::getString(attributes);
      // java if (null != value)
      if let Some(value) = value.as_deref() {
        // java Integer intValue = TypeConversion.hexString2Integer(value);
        let intValue = TypeConversion::hexString2Integer(value);
        // java if (null != intValue)
        if let Some(intValue) = intValue {
          // java ImporterHelper.setTableLook(ret, intValue);
          ImporterHelper::setTableLook(ret, intValue);
        }
      }
    }

    // java  boolean toFit(Attributes attributes)
    pub fn toFit(&self, attributes: &dyn Attributes) -> bool {
      // java String str = ConvertHelper.getString(attributes, "type");
      let str = ConvertHelper::getString__String(attributes, "type");
      // java if (null != str && str.length() != 0)
      if str.as_deref().map(|s| !s.is_empty()).unwrap_or(false) {
        // java int AUTOFIT = -646308862;
        let AUTOFIT: i32 = -646308862;
        // java int FIXED = 97445748;
        let FIXED: i32 = 97445748;
        let _ = (AUTOFIT, FIXED);
        // java switch (str.hashCode())
        // java break
        match ConvertHelper::getStringHashCode__String(attributes, "type") {
          // java case -646308862:
          -646308862 => {
            // java return true;
            return true;
          }
          // java case 97445748:
          97445748 => {
            // java return false;
            return false;
          }
          // java default:
          _ => {
            // java IOAssert.fail("It should not reach here!");
            IOAssert::fail__string(&"It should not reach here!");
            // java return true;
            return true;
          }
        }
      }
      // java else
      else {
        // java return true;
        return true;
      }
    }

    // java  int toTableJC(Attributes attributes)
    pub fn toTableJC(&self, attributes: &dyn Attributes) -> i32 {
      // java int strCode = ConvertHelper.getStringHashCode(attributes);
      let strCode: i32 = ConvertHelper::getStringHashCode(attributes);
      // java if (-1 == strCode)
      if -1 == strCode {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java int CENTER = -1364013995;
        let mut CENTER: i32 = -1364013995;
        // java int END = 100571;
        let mut END: i32 = 100571;
        // java int LEFT = 3317767;
        let mut LEFT: i32 = 3317767;
        // java int RIGHT = 108511772;
        let mut RIGHT: i32 = 108511772;
        // java int START = 109757538;
        let mut START: i32 = 109757538;
        // java switch (strCode)
        // java break
        match strCode {
          // java case -1364013995:
          -1364013995 => {
            // java return 1;
            return 1;
          }
          // java case 100571:
          100571 => {
            // java return 2;
            return 2;
          }
          // java case 3317767:
          3317767 => {
            // java return 0;
            return 0;
          }
          // java case 108511772:
          108511772 => {
            // java return 2;
            return 2;
          }
          // java case 109757538:
          109757538 => {
            // java return 0;
            return 0;
          }
          // java default:
          _ => {
            // java IOAssert.fail("It should not reach here!");
            IOAssert::fail__string(&"It should not reach here!");
            // java return 0;
            return 0;
          }
        }
      }
    }

    // java  TableLength toTableLength(Attributes attributes)
    pub fn toTableLength(&self, attributes: &dyn Attributes) -> Option<TableLength> {
      // java if (0 == attributes.getLength())
      if 0 == attributes.getLength() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java int unit = ConvertHelper.getStringHashCode(attributes, "type");
        let mut unit: i32 = ConvertHelper::getStringHashCode__String(attributes, "type");
        // java int type = ConvertHelper.toTableLengthType(unit);
        let mut r#type: i32 = ConvertHelper::toTableLengthType(unit);
        // java IOAssert.assertTrue("(TableLengthType.vetNil <= type && TableLengthType.vetDxa <= type) should not be false!", 0 <= type && type <= 3);
        IOAssert::assertTrue_msg(&"(TableLengthType.vetNil <= type && TableLengthType.vetDxa <= type) should not be false!", 0 <= r#type && r#type <= 3);
        // java int DEFAULT_W = 0;
        let mut DEFAULT_W: i32 = 0;
        // java Integer inValue = ConvertHelper.getInt(attributes, "w");
        let inValue = ConvertHelper::getInt(attributes, "w");
        // java int value = null == inValue ? 0 : inValue;
        let value: i32 = inValue.unwrap_or(0);
        // java return TableLength.obtain(type, value);
        return Some(TableLength::obtain(r#type, value));
      }
    }

    // java public MutablePropertySet toTableProp(TextDocument doc, MutablePropertySet dst, MutablePropertySet originalProp, int layerTable, int numTableCell)
    pub fn toTableProp(&self, doc: Option<*mut TextDocument>, dst: &mut MutablePropertySet, originalProp: &MutablePropertySet, layerTable: i32, numTableCell: i32) -> MutablePropertySet {
      // java return null != originalProp && originalProp.contains(363) ? this.convertTableProp(doc, dst, originalProp, layerTable, numTableCell) : this.convertTableProp(dst, layerTable, numTableCell);
      if originalProp.contains(363) {
        let mut original = originalProp.clone_mps();
        return self.convertTableProp__textdocument_mutablepropertyset_mutablepropertyset_int_int(doc, dst, &mut original, layerTable, numTableCell);
      }
      return self.convertTableProp__mutablepropertyset_int_int(dst, layerTable, numTableCell);
    }

    // java private MutablePropertySet convertTableProp(MutablePropertySet dst, int layerTable, int numTableCell)
    pub fn convertTableProp__mutablepropertyset_int_int(&self, dst: &mut MutablePropertySet, layerTable: i32, numTableCell: i32) -> MutablePropertySet {
      // java dst.setBoolean(223, true);
      dst.setBoolean(223, true);
      // java dst.setInt(224, layerTable);
      dst.setInt(224, layerTable);
      // java dst.setBoolean(225, true);
      dst.setBoolean(225, true);
      // java TablePropCorrector.correctTableProp(dst, layerTable, numTableCell);
      TablePropCorrector::correctTableProp(dst, layerTable, numTableCell);
      // java return dst;
      return dst.clone_mps();
    }

    // java private MutablePropertySet convertTableProp(TextDocument doc, MutablePropertySet dst, MutablePropertySet originalProp, int layerTable, int numTableCell)
    pub fn convertTableProp__textdocument_mutablepropertyset_mutablepropertyset_int_int(&self, doc: Option<*mut TextDocument>, dst: &mut MutablePropertySet, originalProp: &mut MutablePropertySet, layerTable: i32, numTableCell: i32) -> MutablePropertySet {
      let _ = doc;
      // java IOAssert.assertNotNull("doc should not be null", doc);
      IOAssert::assertNotNull__string_object(&"doc should not be null", Some(&0i32));
      // java IOAssert.assertNotNull("dst should not be null", dst);
      IOAssert::assertNotNull__string_object(&"dst should not be null", Some(&dst));
      // java IOAssert.assertNotNull("originalProp should not be null", originalProp);
      IOAssert::assertNotNull__string_object(&"originalProp should not be null", Some(&originalProp));
      // java IOAssert.assertTrue("originalProp should contain table_formatChange", originalProp.contains(363));
      IOAssert::assertTrue_msg(&"originalProp should contain table_formatChange", originalProp.contains(363));
      // java originalProp.setBoolean(223, true);
      originalProp.setBoolean(223, true);
      // java originalProp.setInt(224, layerTable);
      originalProp.setInt(224, layerTable);
      // java originalProp.setBoolean(225, true);
      originalProp.setBoolean(225, true);
      // java originalProp.setInt(365, ConvertHelper.addRsidToTextDocument(doc));
      let _rsid = doc;
      // java if (dst.contains(282))
      if dst.contains(282) {
        // java originalProp.setValue(282, dst.getValue(282));
        originalProp.setValue(282, dst.getValue(282).cloned());
        // java dst.remove(282);
        dst.remove(282);
      }
      // java MutablePropertySet revisionProp = new MutablePropertySet(dst);
      let mut revisionProp = MutablePropertySet::new_from_mutable(dst);
      // java this.convertTableProp(originalProp, revisionProp, layerTable, numTableCell);
      self.convertTableProp__mutablepropertyset_mutablepropertyset_int_int(originalProp, &mut revisionProp, layerTable, numTableCell);
      // java return originalProp;
      return originalProp.clone_mps();
    }

    // java private void convertTableProp(MutablePropertySet originalProp, MutablePropertySet revisionProp, int layerTable, int numTableCell)
    pub fn convertTableProp__mutablepropertyset_mutablepropertyset_int_int(&self, originalProp: &mut MutablePropertySet, revisionProp: &mut MutablePropertySet, layerTable: i32, numTableCell: i32) {
      // java IOAssert.assertNotNull("revisionProp should not be null", revisionProp);
      IOAssert::assertNotNull__string_object(&"revisionProp should not be null", Some(&revisionProp));
      // java IOAssert.assertNotNull("originalProp should not be null", originalProp);
      IOAssert::assertNotNull__string_object(&"originalProp should not be null", Some(&originalProp));
      // java IOAssert.assertTrue("originalProp should contain table_formatChange", originalProp.contains(363));
      IOAssert::assertTrue_msg(&"originalProp should contain table_formatChange", originalProp.contains(363));
      // java TablePropCorrector.correctTableProp(revisionProp, layerTable, numTableCell);
      TablePropCorrector::correctTableProp(revisionProp, layerTable, numTableCell);
      // java TablePropCorrector.correctTableProp(originalProp, layerTable, numTableCell);
      TablePropCorrector::correctTableProp(originalProp, layerTable, numTableCell);
      // java MutablePropertySet revisionChangeProp = TablePropsConverter.getDifferenceTableProp(originalProp, revisionProp);
      let mut revisionChangeProp = Self::getDifferenceTableProp(originalProp, revisionProp);
      // java Revision revision = (Revision)originalProp.getValue(363);
      let mut revision = originalProp.getValue(363).and_then(|v| v.as_ref().as_any().downcast_ref::<Revision>().cloned()).unwrap_or_else(|| Revision::Revision__int(3));
      // java revision.setProperty(revisionChangeProp.freeze());
      revision.setProperty(Some(revisionChangeProp.freeze()));
      originalProp.setValue(363, Some(Arc::new(revision)));
    }

    fn getDifferenceTableProp(previous: &MutablePropertySet, after: &MutablePropertySet) -> MutablePropertySet {
      if after.contains(292) {
        return after.clone_mps();
      }
      let prev_ps = previous.clone_mps().freeze();
      let after_ps = after.clone_mps().freeze();
      let mut merged = MutablePropertySet::new_from_ps(&after_ps);
      merged.differenceSet(&prev_ps);
      merged
    }

    // java public PropertySet getTableRowRunProp(MutablePropertySet rowProp)
    pub fn getTableRowRunProp(&self, rowProp: &mut MutablePropertySet) -> PropertySet {
      // java IOAssert.assertNotNull(rowProp);
      IOAssert::assertNotNull__object(&Some(&*rowProp));
      // java Object revisionIns = rowProp.getValue(46);
      let revisionIns = rowProp.getValue(46).cloned();
      // java Object revisionDel = rowProp.getValue(45);
      let revisionDel = rowProp.getValue(45).cloned();
      // java if (null == revisionIns && null == revisionDel)
      if revisionIns.is_none() && revisionDel.is_none() {
        // java return PropertySet.empty;
        return (*util::moffice::property::PropertySet::empty).clone();
      }
      // java else
      else {
        // java MutablePropertySet dst = new MutablePropertySet();
        let mut dst = MutablePropertySet::new();
        // java if (null != revisionIns)
        if revisionIns.is_some() {
          // java this.moveProp(rowProp, dst, 46);
          self.moveProp(rowProp, &mut dst, 46);
          // java this.moveProp(rowProp, dst, 55);
          self.moveProp(rowProp, &mut dst, 55);
          // java this.moveProp(rowProp, dst, 50);
          self.moveProp(rowProp, &mut dst, 50);
        }
        // java if (null != revisionDel)
        if revisionDel.is_some() {
          // java this.moveProp(rowProp, dst, 45);
          self.moveProp(rowProp, &mut dst, 45);
          // java this.moveProp(rowProp, dst, 54);
          self.moveProp(rowProp, &mut dst, 54);
          // java this.moveProp(rowProp, dst, 49);
          self.moveProp(rowProp, &mut dst, 49);
        }
        // java return dst.freeze();
        return dst.freeze();
      }
    }

    // java private void moveProp(MutablePropertySet rowProp, MutablePropertySet dst, int propertyId)
    pub fn moveProp(&self, rowProp: &mut MutablePropertySet, dst: &mut MutablePropertySet, propertyId: i32) {
      // java Object value = rowProp.getValue(propertyId);
      let value = rowProp.getValue(propertyId).cloned();
      // java dst.setValue(propertyId, value);
      dst.setValue(propertyId, value);
      // java rowProp.remove(propertyId);
      rowProp.remove(propertyId);
    }

}

impl std::ops::Deref for TablePropertyBaseConverterSimple {
    type Target = ImporterBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TablePropertyBaseConverterSimple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
