// Java: import cn.wps.base.io.MetricUtility
use util::base::io::MetricUtility::*;
// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::*;
// Java: import cn.wps.moffice.drawing.IPos
use drawing::cn::wps::moffice::drawing::IPos::*;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog::*;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.core.PropertyTool
use crate::cn::wps::moffice::writer::core::PropertyTool::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.table.Tools
use crate::cn::wps::moffice::writer::core::table::Tools::*;
// Java: import cn.wps.moffice.writer.data.DefaultValue
use crate::cn::wps::moffice::writer::data::DefaultValue::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.table.tree.RowInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::RowInfoNode::*;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::*;
// Java: import cn.wps.moffice.writer.data.value.CellMargin
use crate::cn::wps::moffice::writer::data::value::CellMargin::*;
// Java: import cn.wps.moffice.writer.data.value.DefTable
use crate::cn::wps::moffice::writer::data::value::DefTable::*;
// Java: import cn.wps.moffice.writer.data.value.DefTableBuilder
use crate::cn::wps::moffice::writer::data::value::DefTableBuilder::*;
// Java: import cn.wps.moffice.writer.data.value.PositionCode
use crate::cn::wps::moffice::writer::data::value::PositionCode::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.data.value.TableBorders
use crate::cn::wps::moffice::writer::data::value::TableBorders::*;
// Java: import cn.wps.moffice.writer.data.value.TableCell
use crate::cn::wps::moffice::writer::data::value::TableCell::*;
// Java: import cn.wps.moffice.writer.data.value.TableCellBuilder
use crate::cn::wps::moffice::writer::data::value::TableCellBuilder::*;
// Java: import cn.wps.moffice.writer.data.value.TableDefSHD
use crate::cn::wps::moffice::writer::data::value::TableDefSHD::*;
// Java: import cn.wps.moffice.writer.data.value.TableLength
use crate::cn::wps::moffice::writer::data::value::TableLength::*;
// Java: import cn.wps.moffice.writer.data.value.TableLook
use crate::cn::wps::moffice::writer::data::value::TableLook::*;
// Java: import cn.wps.moffice.writer.data.value.TableSetBRC
use crate::cn::wps::moffice::writer::data::value::TableSetBRC::*;
// Java: import cn.wps.moffice.writer.data.value.TableStyleProperties
use crate::cn::wps::moffice::writer::data::value::TableStyleProperties::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableJCData
use crate::cn::wps::moffice::writer::data::table::TableJCData::*;
// Java: import cn.wps.moffice.writer.data.table.TablePos
use crate::cn::wps::moffice::writer::data::table::TablePos::*;

// java private static final String TAG = null;
const TAG: &str = "";
// java public static final int MAX_RESIDUAL_MERGE = 6;
pub(crate) const MAX_RESIDUAL_MERGE: i32 = 6;
// java public static final int MAX_RESIDUAL = 40;
pub(crate) const MAX_RESIDUAL: i32 = 40;
// java private static final float DIAGONAL_DOT_DASH_LINE_FIXED_WIDTH = 0.5F;
const DIAGONAL_DOT_DASH_LINE_FIXED_WIDTH: f32 = 0.5f32;
// java protected static int[] SHARE_PROPERTYID_FOR_TABLE = new int[]{292, 354, 343, 312, 313, 314, 317, 318, 315, 316};
pub(crate) static SHARE_PROPERTYID_FOR_TABLE: std::sync::LazyLock<Option<Vec<i32>>> =
                    std::sync::LazyLock::new(|| vec![292, 354, 343, 312, 313, 314, 317, 318, 315, 316]);
// java protected static Object[] DEFAULT_VALUE_FOR_SHARE_PROPERTY = new Object[]{105, 11, false, new PositionCode(0, 0), 0, 0, 0, 0, 0, 0};
pub(crate) static DEFAULT_VALUE_FOR_SHARE_PROPERTY: std::sync::LazyLock<Vec<Box<dyn Object + Send + Sync>>> =
                    std::sync::LazyLock::new(|| vec![
                      Box::new(105) as Box<dyn Object + Send + Sync>, 
                      Box::new(11) as Box<dyn Object + Send + Sync>, 
                      Box::new(false) as Box<dyn Object + Send + Sync>, 
                      Box::new(PositionCode::new(0, 0)) as Box<dyn Object + Send + Sync>, 
                      Box::new(0) as Box<dyn Object + Send + Sync>, Box::new(0) as Box<dyn Object + Send + Sync>, Box::new(0) as Box<dyn Object + Send + Sync>, Box::new(0) as Box<dyn Object + Send + Sync>, Box::new(0) as Box<dyn Object + Send + Sync>, Box::new(0) as Box<dyn Object + Send + Sync>]);
// java protected static int[] SHARE_PROPERTYID_FOR_FIRST_CELL_PARA = new int[]{240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250};
pub(crate) static SHARE_PROPERTYID_FOR_FIRST_CELL_PARA: std::sync::LazyLock<Option<Vec<i32>>> =
                    std::sync::LazyLock::new(|| vec![240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250]);
// java protected static Object[] DEFAULT_VALUE_FOR_FIRST_CELL_PARA;
pub(crate) static DEFAULT_VALUE_FOR_FIRST_CELL_PARA: std::sync::OnceLock<Vec<Box<dyn Object + Send + Sync>>> =
                std::sync::OnceLock::new();

// java public class TableTool
#[derive(Default)]
pub struct TableTool {
    // java PropertyTool mPropertyTool = null;
    pub(crate) mPropertyTool: Option<PropertyTool>,
    // java Document mDocument = null;
    pub(crate) mDocument: Option<Box<dyn Document>>,
}

// java static class FinalTableCellProperty
#[derive(Default)]
pub struct FinalTableCellProperty {
    // java SHD shd = null;
    pub(crate) shd: Option<SHD>,
    // java BRC brcLeft = null;
    pub(crate) brcLeft: Option<BRC>,
    // java BRC brcTop = null;
    pub(crate) brcTop: Option<BRC>,
    // java BRC brcRight = null;
    pub(crate) brcRight: Option<BRC>,
    // java BRC brcBottom = null;
    pub(crate) brcBottom: Option<BRC>,
    // java BRC brcVInside = null;
    pub(crate) brcVInside: Option<BRC>,
    // java BRC brcHInside = null;
    pub(crate) brcHInside: Option<BRC>,
}

// java public class TableTool
impl TableTool {
    // java public static int getResidual(int cellWidth)
    pub fn getResidual(cellWidth: i32) -> i32 {
      // java return cellWidth > 200 ? 40 : (int)Math.min((float)cellWidth / 5.0F, 6.0F);
      return (if cellWidth > 200 { 40 } else { ((Math::min(((cellWidth) as f32) / 5, 6)) as i32) });
    }

    // java public static boolean isInTheSameTable(Document document, int start1, int end1, int start2, int end2)
    pub fn isInTheSameTable(document: &dyn Document, start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
      // java PropertySet property1 = getFinalRowProperty(document, end1 - 1);
      let mut property1: PropertySet = Self::getFinalRowProperty(&document, end1 - 1);
      // java PropertySet property2 = getFinalRowProperty(document, end2 - 1);
      let mut property2: PropertySet = Self::getFinalRowProperty(&document, end2 - 1);
      // java boolean allDefault = true;
      let mut allDefault: bool = true;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (SHARE_PROPERTYID_FOR_TABLE.len() as i32);
      // java for(int size = SHARE_PROPERTYID_FOR_TABLE.length; i < size; ++i)
      while i < size {
        // java Object o1 = property1.getObject(SHARE_PROPERTYID_FOR_TABLE[i], DEFAULT_VALUE_FOR_SHARE_PROPERTY[i]);
        let mut o1: Box<dyn Object> = property1.getObject(SHARE_PROPERTYID_FOR_TABLE[(i) as usize], DEFAULT_VALUE_FOR_SHARE_PROPERTY[(i) as usize]);
        // java Object o2 = property2.getObject(SHARE_PROPERTYID_FOR_TABLE[i], DEFAULT_VALUE_FOR_SHARE_PROPERTY[i]);
        let mut o2: Box<dyn Object> = property2.getObject(SHARE_PROPERTYID_FOR_TABLE[(i) as usize], DEFAULT_VALUE_FOR_SHARE_PROPERTY[(i) as usize]);
        // java if (!o1.equals(o2))
        if !o1.equals(o2) {
          // java return false;
          return false;
        }
        // java if (allDefault)
        if allDefault {
          // java allDefault = o1.equals(DEFAULT_VALUE_FOR_SHARE_PROPERTY[i]);
          allDefault = o1.equals(DEFAULT_VALUE_FOR_SHARE_PROPERTY[(i) as usize]);
        }
      }
      // java if (!allDefault)
      if !allDefault {
        // java return true;
        return true;
      }
      // java else
      else {
        // java property1 = PropertyUtil.getFinalParagraphFormat(document.getPapxTable().seek(start1).getProperty());
        property1 = PropertyUtil::getFinalParagraphFormat(&document.getPapxTable().seek(start1).getProperty());
        // java property2 = PropertyUtil.getFinalParagraphFormat(document.getPapxTable().seek(start2).getProperty());
        property2 = PropertyUtil::getFinalParagraphFormat(&document.getPapxTable().seek(start2).getProperty());
        // java i = 0;
        i = 0;
        let mut size: i32 = (SHARE_PROPERTYID_FOR_FIRST_CELL_PARA.len() as i32);
        // java for(int size = SHARE_PROPERTYID_FOR_FIRST_CELL_PARA.length; i < size; ++i)
        while i < size {
          // java Object o1 = property1.getObject(SHARE_PROPERTYID_FOR_FIRST_CELL_PARA[i], DEFAULT_VALUE_FOR_FIRST_CELL_PARA[i]);
          let mut o1: Box<dyn Object> = property1.getObject(SHARE_PROPERTYID_FOR_FIRST_CELL_PARA[(i) as usize], DEFAULT_VALUE_FOR_FIRST_CELL_PARA[(i) as usize]);
          // java Object o2 = property2.getObject(SHARE_PROPERTYID_FOR_FIRST_CELL_PARA[i], DEFAULT_VALUE_FOR_FIRST_CELL_PARA[i]);
          let mut o2: Box<dyn Object> = property2.getObject(SHARE_PROPERTYID_FOR_FIRST_CELL_PARA[(i) as usize], DEFAULT_VALUE_FOR_FIRST_CELL_PARA[(i) as usize]);
          // java if (!o1.equals(o2))
          if !o1.equals(o2) {
            // java return false;
            return false;
          }
        }
        // java return true;
        return true;
      }
    }

    // java static PropertySet getFinalRowProperty(Document document, int cp)
    pub fn getFinalRowProperty(document: &dyn Document, cp: i32) -> PropertySet {
      // java PropertySet prop = document.getPapxTable().seek(cp).getProperty();
      let mut prop: PropertySet = document.getPapxTable().seek(cp).getProperty();
      // java Object value = prop.getValue(363);
      let mut value: Box<dyn Object> = prop.getValue(363);
      // java if (null != value)
      if value.is_some() {
        // java KStyles styles = document.getTextDocument().getStyles();
        let mut styles: KStyles = document.getTextDocument().getStyles();
        // java return PropertyUtil.getFinalTableFormat(styles, prop);
        return PropertyUtil::getFinalTableFormat(&styles, &prop);
      }
      // java else
      else {
        // java return prop;
        return prop;
      }
    }

    // java public  TableTool(Document document)
    pub fn TableTool(document: &dyn Document) -> Self {
      // java this.mDocument = document;
      Self { mPropertyTool: PropertyTool::new(document), mDocument: document }
      // java this.mPropertyTool = new PropertyTool(document);
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (null != this.mPropertyTool)
      if let Some(mPropertyTool) = self.mPropertyTool.as_mut() {
        // java this.mPropertyTool.dispose();
        mPropertyTool.dispose();
        // java this.mPropertyTool = null;
        mPropertyTool = None;
      }
      // java this.mDocument = null;
      self.mDocument = None;
    }

    // java  PropertySet getBaseTableProperty(PropertySet finalTableProperty)
    pub fn getBaseTableProperty(&self, finalTableProperty: &PropertySet) -> PropertySet {
      // java int styleId = finalTableProperty.getInt(292, 4095);
      let mut styleId: i32 = finalTableProperty.getInt__int_int(292, 4095);
      // java PropertySet baseTableProperty = null;
      let mut baseTableProperty: PropertySet = PropertySet::default();
      // java if (4095 != styleId)
      if 4095 != styleId {
        // java baseTableProperty = this.mPropertyTool.getMergedTableProperty(styleId);
        baseTableProperty = self.mPropertyTool.getMergedTableProperty__int(styleId);
      }
      // java return baseTableProperty;
      return baseTableProperty;
    }

    // java public void calcTableStyles(TableInfo tableInfoImpl)
    pub fn calcTableStyles(&self, tableInfoImpl: &TableInfo) {
      // java Data.TableData tableData = tableInfoImpl.getData();
      let mut tableData: Data::TableData = tableInfoImpl.getData();
      // java tableData.clear();
      tableData.clear();
      // java PropertySet tableProperty = tableInfoImpl.getTableProperty();
      let mut tableProperty: PropertySet = tableInfoImpl.getTableProperty();
      // java tableProperty = Tools.getFinalProperty(tableProperty);
      tableProperty = Tools::getFinalProperty(&tableProperty);
      // java tableData.mPos = this.getTablePos(tableProperty);
      tableData.mPos = self.getTablePos(&tableProperty);
      // java PropertySet baseTableProperty = this.getBaseTableProperty(tableProperty);
      let mut baseTableProperty: PropertySet = self.getBaseTableProperty(&tableProperty);
      // java if (baseTableProperty != null && baseTableProperty.contains(309))
      if baseTableProperty.is_some() && baseTableProperty.contains(309) {
        // java tableData.mShd = (SHD)baseTableProperty.getObject(309, (Object)null);
        tableData.mShd = (baseTableProperty.getObject(309, None)).downcast_ref::<SHD>().unwrap();
      }
      // java if (tableProperty.contains(309))
      if tableProperty.contains(309) {
        // java tableData.mShd = (SHD)tableProperty.getObject(309, (Object)null);
        tableData.mShd = (tableProperty.getObject(309, None)).downcast_ref::<SHD>().unwrap();
      }
      // java tableData.mStyleId = tableProperty.getInt(292, 4095);
      tableData.mStyleId = tableProperty.getInt(292, 4095);
      // java tableData.mTableLook = (TableLook)tableProperty.getObject(342, DefaultValue.table_look);
      tableData.mTableLook = (tableProperty.getObject(342, DefaultValue::table_look)).downcast_ref::<TableLook>().unwrap();
      // java tableData.mTableBorders = (TableBorders)tableProperty.getObject(310, (Object)null);
      tableData.mTableBorders = (tableProperty.getObject(310, None)).downcast_ref::<TableBorders>().unwrap();
      // java tableData.mIsRTL = tableProperty.getBoolean(343, false);
      tableData.mIsRTL = tableProperty.getBoolean(343, false);
    }

    // java public PropertySet calcRowStyle(RowInfo row)
    pub fn calcRowStyle(&mut self, row: &RowInfo) -> PropertySet {
      // java PropertySet finalRowProperty = row.getFinalProperty();
      let mut finalRowProperty: PropertySet = row.getFinalProperty();
      // java PropertySet baseProp = this.getBaseTableProperty(finalRowProperty);
      let mut baseProp: PropertySet = self.getBaseTableProperty(&finalRowProperty);
      // java Data.RowData rowData = row.getData();
      let mut rowData: Data::RowData = row.getData();
      // java rowData.clear();
      rowData.clear();
      // java CellInfo firstCell = row.getCellByIndex(0);
      let mut firstCell: CellInfo = row.getCellByIndex(0);
      // java CellInfo lastCell = firstCell;
      let mut lastCell: CellInfo = firstCell;
      let mut cellInfo: CellInfo = firstCell;
      // java for(CellInfo cellInfo = firstCell; cellInfo != null cellInfo = cellInfo.getNextCell())
      while cellInfo.is_some() {
        // java cellInfo.getData().clear();
        cellInfo.getData().clear();
        // java lastCell = cellInfo;
        lastCell = cellInfo;
      }
      // java MutablePropertySet finalRowStyle = new MutablePropertySet();
      let mut finalRowStyle: MutablePropertySet = MutablePropertySet::new();
      // java FinalTableCellProperty finalTableCellProp = new FinalTableCellProperty();
      let mut finalTableCellProp: FinalTableCellProperty = FinalTableCellProperty::new();
      // java TableLook tableLook = (TableLook)row.getFinalProperty().getObject(342, DefaultValue.table_look);
      let mut tableLook: TableLook = (row.getFinalProperty().getObject(342, DefaultValue::table_look)).downcast_ref::<TableLook>().unwrap();
      // java int rowIndex = row.getIndex();
      let mut rowIndex: i32 = row.getIndex();
      // java TableBorders tableBorders = row.getParent().getTableBorders();
      let mut tableBorders: TableBorders = row.getParent().getTableBorders();
      // java if (null != baseProp)
      if baseProp.is_some() {
        // java this.mergeRowStyle(baseProp, finalRowStyle, finalTableCellProp, tableBorders);
        self.mergeRowStyle(&baseProp, &finalRowStyle, &mut finalTableCellProp, Some(&tableBorders));
        // java TableStyleProperties wholeTableProp = (TableStyleProperties)baseProp.getValue(329);
        let mut wholeTableProp: TableStyleProperties = (baseProp.getValue(329)).downcast_ref::<TableStyleProperties>().unwrap();
        // java if (null != wholeTableProp)
        if wholeTableProp.is_some() {
          // java this.mergeRowStyle(wholeTableProp.getProperty(), finalRowStyle, finalTableCellProp, tableBorders);
          self.mergeRowStyle(&wholeTableProp.getProperty(), &finalRowStyle, &mut finalTableCellProp, Some(&tableBorders));
        }
        // java if (!tableLook.getNoHBand())
        if !tableLook.getNoHBand() {
          // java boolean oddRow = tableLook.getFirstRow() && baseProp.contains(330) ? 0 != rowIndex % 2 : 0 == rowIndex % 2;
          let mut oddRow: bool = (if tableLook.getFirstRow() && baseProp.contains(330) { 0 != rowIndex % 2 } else { 0 == rowIndex % 2 });
          // java if (oddRow)
          if oddRow {
            // java TableStyleProperties oddRowProp = (TableStyleProperties)baseProp.getValue(332);
            let mut oddRowProp: TableStyleProperties = (baseProp.getValue(332)).downcast_ref::<TableStyleProperties>().unwrap();
            // java if (null != oddRowProp)
            if oddRowProp.is_some() {
              // java this.mergeRowStyle(oddRowProp.getProperty(), finalRowStyle, finalTableCellProp, tableBorders);
              self.mergeRowStyle(&oddRowProp.getProperty(), &finalRowStyle, &mut finalTableCellProp, Some(&tableBorders));
            }
          }
          // java else
          else {
            // java TableStyleProperties evenRowProp = (TableStyleProperties)baseProp.getValue(333);
            let mut evenRowProp: TableStyleProperties = (baseProp.getValue(333)).downcast_ref::<TableStyleProperties>().unwrap();
            // java if (null != evenRowProp)
            if evenRowProp.is_some() {
              // java this.mergeRowStyle(evenRowProp.getProperty(), finalRowStyle, finalTableCellProp, tableBorders);
              self.mergeRowStyle(&evenRowProp.getProperty(), &finalRowStyle, &mut finalTableCellProp, Some(&tableBorders));
            }
          }
        }
      }
      // java boolean firstRow = 0 == rowIndex;
      let mut firstRow: bool = 0 == rowIndex;
      // java boolean lastRow = TableInfoTool.isLastRow(row);
      let mut lastRow: bool = TableInfoTool::isLastRow(&row);
      // java boolean useFirstRow = tableLook.getFirstRow() && firstRow;
      let mut useFirstRow: bool = tableLook.getFirstRow() && firstRow;
      // java boolean useLastRow = tableLook.getLastRow() && lastRow;
      let mut useLastRow: bool = tableLook.getLastRow() && lastRow;
      // java if (null != baseProp)
      if baseProp.is_some() {
        // java if (useFirstRow)
        if useFirstRow {
          // java TableStyleProperties firstRowProp = (TableStyleProperties)baseProp.getValue(330);
          let mut firstRowProp: TableStyleProperties = (baseProp.getValue(330)).downcast_ref::<TableStyleProperties>().unwrap();
          // java if (null != firstRowProp)
          if firstRowProp.is_some() {
            // java this.mergeRowStyle(firstRowProp.getProperty(), finalRowStyle, finalTableCellProp, tableBorders);
            self.mergeRowStyle(&firstRowProp.getProperty(), &finalRowStyle, &mut finalTableCellProp, Some(&tableBorders));
          }
        }
        // java if (useLastRow)
        if useLastRow {
          // java TableStyleProperties lastRowProp = (TableStyleProperties)baseProp.getValue(331);
          let mut lastRowProp: TableStyleProperties = (baseProp.getValue(331)).downcast_ref::<TableStyleProperties>().unwrap();
          // java if (null != lastRowProp)
          if lastRowProp.is_some() {
            // java this.mergeRowStyle(lastRowProp.getProperty(), finalRowStyle, finalTableCellProp, tableBorders);
            self.mergeRowStyle(&lastRowProp.getProperty(), &finalRowStyle, &mut finalTableCellProp, Some(&tableBorders));
          }
        }
      }
      // java this.mergeRowStyle(finalRowProperty, finalRowStyle, finalTableCellProp, tableBorders);
      self.mergeRowStyle(&finalRowProperty, &finalRowStyle, &mut finalTableCellProp, Some(&tableBorders));
      // java setCellSHDAndBRC(row, finalRowStyle, finalTableCellProp, lastRow, firstRow);
      Self::setCellSHDAndBRC(&row, &finalRowStyle, &finalTableCellProp, lastRow, firstRow);
      // java if (null != baseProp && !tableLook.getNoVBand())
      if baseProp.is_some() && !tableLook.getNoVBand() {
        // java int cellIndex = 0;
        let mut cellIndex: i32 = 0;
        let mut size: i32 = row.size();
        // java for(int size = row.size(); cellIndex < size ++cellIndex)
        while cellIndex < size {
          // java CellInfo cell = row.getCellByIndex(cellIndex);
          let mut cell: CellInfo = row.getCellByIndex(cellIndex);
          // java boolean oddColumn = tableLook.getFirstCol() && baseProp.contains(334) ? 0 != cellIndex % 2 : 0 == cellIndex % 2;
          let mut oddColumn: bool = (if tableLook.getFirstCol() && baseProp.contains(334) { 0 != cellIndex % 2 } else { 0 == cellIndex % 2 });
          // java if (oddColumn)
          if oddColumn {
            // java TableStyleProperties oddColProp = (TableStyleProperties)baseProp.getValue(336);
            let mut oddColProp: TableStyleProperties = (baseProp.getValue(336)).downcast_ref::<TableStyleProperties>().unwrap();
            // java if (null != oddColProp)
            if oddColProp.is_some() {
              // java calcCellStyle(cell, oddColProp.getProperty());
              Self::calcCellStyle__cellinfo_propertyset(&cell, &oddColProp.getProperty());
            }
          }
          // java else
          else {
            // java TableStyleProperties evenColProp = (TableStyleProperties)baseProp.getValue(337);
            let mut evenColProp: TableStyleProperties = (baseProp.getValue(337)).downcast_ref::<TableStyleProperties>().unwrap();
            // java if (null != evenColProp)
            if evenColProp.is_some() {
              // java calcCellStyle(cell, evenColProp.getProperty());
              Self::calcCellStyle__cellinfo_propertyset(&cell, &evenColProp.getProperty());
            }
          }
        }
      }
      // java if (null != baseProp && !useFirstRow && !useLastRow)
      if baseProp.is_some() && !useFirstRow && !useLastRow {
        // java if (tableLook.getLastCol())
        if tableLook.getLastCol() {
          // java TableStyleProperties lastColProp = (TableStyleProperties)baseProp.getValue(335);
          let mut lastColProp: TableStyleProperties = (baseProp.getValue(335)).downcast_ref::<TableStyleProperties>().unwrap();
          // java if (null != lastColProp)
          if lastColProp.is_some() {
            // java calcCellStyle(lastCell, lastColProp.getProperty());
            Self::calcCellStyle__cellinfo_propertyset(&lastCell, &lastColProp.getProperty());
          }
        }
        // java if (tableLook.getFirstCol())
        if tableLook.getFirstCol() {
          // java TableStyleProperties firstColProp = (TableStyleProperties)baseProp.getValue(334);
          let mut firstColProp: TableStyleProperties = (baseProp.getValue(334)).downcast_ref::<TableStyleProperties>().unwrap();
          // java if (null != firstColProp)
          if firstColProp.is_some() {
            // java calcCellStyle(firstCell, firstColProp.getProperty());
            Self::calcCellStyle__cellinfo_propertyset(&firstCell, &firstColProp.getProperty());
          }
        }
      }
      // java if (null != baseProp)
      if baseProp.is_some() {
        // java if (tableLook.getLastRow() && lastRow && baseProp.contains(331))
        if tableLook.getLastRow() && lastRow && baseProp.contains(331) {
          // java this.mergeRowStyle(((TableStyleProperties)baseProp.getObject(331)).getProperty(), finalRowStyle, finalTableCellProp, tableBorders);
          self.mergeRowStyle(&(baseProp.getObject(331)).downcast_ref::<TableStyleProperties>().unwrap().getProperty(), &finalRowStyle, &mut finalTableCellProp, Some(&tableBorders));
          // java setCellSHDAndBRC(row, finalRowStyle, finalTableCellProp, lastRow, firstRow);
          Self::setCellSHDAndBRC(&row, &finalRowStyle, &finalTableCellProp, lastRow, firstRow);
        }
        // java if (tableLook.getFirstRow() && firstRow && baseProp.contains(330))
        if tableLook.getFirstRow() && firstRow && baseProp.contains(330) {
          // java this.mergeRowStyle(((TableStyleProperties)baseProp.getObject(330)).getProperty(), finalRowStyle, finalTableCellProp, tableBorders);
          self.mergeRowStyle(&(baseProp.getObject(330)).downcast_ref::<TableStyleProperties>().unwrap().getProperty(), &finalRowStyle, &mut finalTableCellProp, Some(&tableBorders));
          // java setCellSHDAndBRC(row, finalRowStyle, finalTableCellProp, lastRow, firstRow);
          Self::setCellSHDAndBRC(&row, &finalRowStyle, &finalTableCellProp, lastRow, firstRow);
        }
      }
      // java if (null != baseProp)
      if baseProp.is_some() {
        // java if (useLastRow)
        if useLastRow {
          // java TableStyleProperties seCellProp = (TableStyleProperties)baseProp.getValue(340);
          let mut seCellProp: TableStyleProperties = (baseProp.getValue(340)).downcast_ref::<TableStyleProperties>().unwrap();
          // java if (null != seCellProp)
          if seCellProp.is_some() {
            // java calcCellStyle(lastCell, seCellProp.getProperty());
            Self::calcCellStyle__cellinfo_propertyset(&lastCell, &seCellProp.getProperty());
          }
          // java TableStyleProperties swCellProp = (TableStyleProperties)baseProp.getValue(341);
          let mut swCellProp: TableStyleProperties = (baseProp.getValue(341)).downcast_ref::<TableStyleProperties>().unwrap();
          // java if (null != swCellProp)
          if swCellProp.is_some() {
            // java calcCellStyle(firstCell, swCellProp.getProperty());
            Self::calcCellStyle__cellinfo_propertyset(&firstCell, &swCellProp.getProperty());
          }
        }
        // java if (useFirstRow)
        if useFirstRow {
          // java TableStyleProperties neCellProp = (TableStyleProperties)baseProp.getValue(338);
          let mut neCellProp: TableStyleProperties = (baseProp.getValue(338)).downcast_ref::<TableStyleProperties>().unwrap();
          // java if (null != neCellProp)
          if neCellProp.is_some() {
            // java calcCellStyle(lastCell, neCellProp.getProperty());
            Self::calcCellStyle__cellinfo_propertyset(&lastCell, &neCellProp.getProperty());
          }
          // java TableStyleProperties nwCellProp = (TableStyleProperties)baseProp.getValue(339);
          let mut nwCellProp: TableStyleProperties = (baseProp.getValue(339)).downcast_ref::<TableStyleProperties>().unwrap();
          // java if (null != nwCellProp)
          if nwCellProp.is_some() {
            // java calcCellStyle(firstCell, nwCellProp.getProperty());
            Self::calcCellStyle__cellinfo_propertyset(&firstCell, &nwCellProp.getProperty());
          }
        }
      }
      // java this.calcRowInfo(row, finalRowStyle.get());
      self.calcRowInfo(&row, &finalRowStyle.get());
      // java if (row.getRowProperty().contains(363))
      if row.getRowProperty().contains(363) {
        // java rowData.mIsFormatChange = true;
        rowData.mIsFormatChange = true;
      }
      // java IChpxTable.Entry run = this.mDocument.getChpxTable().seek(row.getEndCP() - 1);
      let mut run: Box<dyn Entry> = self.mDocument.getChpxTable().seek(row.getEndCP() - 1);
      // java if (run != null && !run.isEnd())
      if run.is_some() && !run.isEnd() {
        // java if (run.getProperty().contains(45))
        if run.getProperty().contains(45) {
          // java rowData.mIsRevisionDelete = true;
          rowData.mIsRevisionDelete = true;
        }
        // java else if (run.getProperty().contains(46))
        else if run.getProperty().contains(46) {
          // java rowData.mIsRevisionInsert = true;
          rowData.mIsRevisionInsert = true;
        }
      }
      // java return finalRowStyle.freeze();
      return finalRowStyle.freeze();
    }

    // java  void mergeRowStyle(PropertySet property, MutablePropertySet finalRowStyle, FinalTableCellProperty finalTableCellProp, TableBorders baseBorders)
    pub fn mergeRowStyle(&self, property: &PropertySet, finalRowStyle: &MutablePropertySet, finalTableCellProp: &mut FinalTableCellProperty, baseBorders: Option<&TableBorders>) {
      // java this.mPropertyTool.getMergedTableProperty(finalRowStyle, property, true);
      self.mPropertyTool.getMergedTableProperty__mutablepropertyset_propertyset_boolean(&mut finalRowStyle, &property, true);
      // java SHD shd = (SHD)finalRowStyle.getValue(345);
      let mut shd: SHD = (finalRowStyle.getValue(345)).downcast_ref::<SHD>().unwrap();
      // java if (null != shd)
      if shd.is_some() {
        // java finalTableCellProp.shd = shd;
        finalTableCellProp.shd = shd;
      }
      // java TableBorders borders = (TableBorders)finalRowStyle.getValue(310);
      let mut borders: TableBorders = (finalRowStyle.getValue(310)).downcast_ref::<TableBorders>().unwrap();
      // java if (null != borders)
      if borders.is_some() {
        // java BRC brc = borders.getBorder(1);
        let mut brc: BRC = borders.getBorder(1);
        // java if (null != brc)
        if brc.is_some() {
          // java finalTableCellProp.brcLeft = brc;
          finalTableCellProp.brcLeft = brc;
        }
        // java brc = borders.getBorder(3);
        brc = borders.getBorder(3);
        // java if (null != brc)
        if brc.is_some() {
          // java finalTableCellProp.brcRight = brc;
          finalTableCellProp.brcRight = brc;
        }
        // java brc = borders.getBorder(0);
        brc = borders.getBorder(0);
        // java if (null != brc)
        if brc.is_some() {
          // java finalTableCellProp.brcTop = brc;
          finalTableCellProp.brcTop = brc;
        }
        // java brc = borders.getBorder(2);
        brc = borders.getBorder(2);
        // java if (null != brc)
        if brc.is_some() {
          // java finalTableCellProp.brcBottom = brc;
          finalTableCellProp.brcBottom = brc;
        }
        // java brc = borders.getBorder(4);
        brc = borders.getBorder(4);
        // java if (null != brc)
        if brc.is_some() {
          // java finalTableCellProp.brcHInside = brc;
          finalTableCellProp.brcHInside = brc;
        }
        // java brc = borders.getBorder(5);
        brc = borders.getBorder(5);
        // java if (null != brc)
        if brc.is_some() {
          // java finalTableCellProp.brcVInside = brc;
          finalTableCellProp.brcVInside = brc;
        }
      }
      // java if (baseBorders != null)
      if let Some(baseBorders) = baseBorders {
        // java if (finalTableCellProp.brcLeft == null)
        if finalTableCellProp.brcLeft.is_none() {
          // java finalTableCellProp.brcLeft = baseBorders.getBorder(1);
          finalTableCellProp.brcLeft = baseBorders.getBorder(1);
        }
        // java if (finalTableCellProp.brcRight == null)
        if finalTableCellProp.brcRight.is_none() {
          // java finalTableCellProp.brcRight = baseBorders.getBorder(3);
          finalTableCellProp.brcRight = baseBorders.getBorder(3);
        }
        // java if (finalTableCellProp.brcTop == null)
        if finalTableCellProp.brcTop.is_none() {
          // java finalTableCellProp.brcTop = baseBorders.getBorder(0);
          finalTableCellProp.brcTop = baseBorders.getBorder(0);
        }
        // java if (finalTableCellProp.brcBottom == null)
        if finalTableCellProp.brcBottom.is_none() {
          // java finalTableCellProp.brcBottom = baseBorders.getBorder(2);
          finalTableCellProp.brcBottom = baseBorders.getBorder(2);
        }
        // java if (finalTableCellProp.brcHInside == null)
        if finalTableCellProp.brcHInside.is_none() {
          // java finalTableCellProp.brcHInside = baseBorders.getBorder(4);
          finalTableCellProp.brcHInside = baseBorders.getBorder(4);
        }
        // java if (finalTableCellProp.brcVInside == null)
        if finalTableCellProp.brcVInside.is_none() {
          // java finalTableCellProp.brcVInside = baseBorders.getBorder(5);
          finalTableCellProp.brcVInside = baseBorders.getBorder(5);
        }
      }
    }

    // java static void setCellSHDAndBRC(RowInfo row, MutablePropertySet finalRowStyle, FinalTableCellProperty finalTableCellProperty, boolean isLastRow, boolean isFirstRow)
    pub fn setCellSHDAndBRC(row: &RowInfo, finalRowStyle: &MutablePropertySet, finalTableCellProperty: &FinalTableCellProperty, isLastRow: bool, isFirstRow: bool) {
      // java BRC brcTopStyle = (BRC)finalRowStyle.getValue(346);
      let mut brcTopStyle: BRC = (finalRowStyle.getValue(346)).downcast_ref::<BRC>().unwrap();
      // java BRC brcBottomStyle = (BRC)finalRowStyle.getValue(348);
      let mut brcBottomStyle: BRC = (finalRowStyle.getValue(348)).downcast_ref::<BRC>().unwrap();
      // java BRC brcLeftStyle = (BRC)finalRowStyle.getValue(347);
      let mut brcLeftStyle: BRC = (finalRowStyle.getValue(347)).downcast_ref::<BRC>().unwrap();
      // java BRC brcRightStyle = (BRC)finalRowStyle.getValue(349);
      let mut brcRightStyle: BRC = (finalRowStyle.getValue(349)).downcast_ref::<BRC>().unwrap();
      // java BRC brcHStyle = (BRC)finalRowStyle.getValue(350);
      let mut brcHStyle: BRC = (finalRowStyle.getValue(350)).downcast_ref::<BRC>().unwrap();
      // java BRC brcVStyle = (BRC)finalRowStyle.getValue(351);
      let mut brcVStyle: BRC = (finalRowStyle.getValue(351)).downcast_ref::<BRC>().unwrap();
      // java SHD shd = finalTableCellProperty.shd;
      let mut cellShd: SHD = finalTableCellProperty.shd;
      // java BRC brcTop = finalTableCellProperty.brcTop;
      let mut cellBrcTop: BRC = finalTableCellProperty.brcTop;
      // java BRC brcBottom = finalTableCellProperty.brcBottom;
      let mut cellBrcBottom: BRC = finalTableCellProperty.brcBottom;
      // java BRC brcLeft = finalTableCellProperty.brcLeft;
      let mut cellBrcLeft: BRC = finalTableCellProperty.brcLeft;
      // java if (null != brcLeftStyle)
      if brcLeftStyle.is_some() {
        // java brcLeft = brcLeftStyle;
        cellBrcLeft = brcLeftStyle;
      }
      // java BRC brcRight = finalTableCellProperty.brcRight;
      let mut cellBrcRight: BRC = finalTableCellProperty.brcRight;
      // java if (null != brcRightStyle)
      if brcRightStyle.is_some() {
        // java brcRight = brcRightStyle;
        cellBrcRight = brcRightStyle;
      }
      // java BRC brcH = finalTableCellProperty.brcHInside;
      let mut brcH: BRC = finalTableCellProperty.brcHInside;
      // java if (null != brcHStyle)
      if brcHStyle.is_some() {
        // java brcH = brcHStyle;
        brcH = brcHStyle;
      }
      // java BRC brcV = finalTableCellProperty.brcVInside;
      let mut brcV: BRC = finalTableCellProperty.brcVInside;
      // java if (null != brcVStyle)
      if brcVStyle.is_some() {
        // java brcV = brcVStyle;
        brcV = brcVStyle;
      }
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = row.size();
      // java for(int size = row.size(); i < size; ++i)
      while i < size {
        // java CellInfo cell = row.getCellByIndex(i);
        let mut cell: CellInfo = row.getCellByIndex(i);
        // java Data.CellData cellData = cell.getData();
        let mut cellData: Data::CellData = cell.getData();
        // java if (null != shd)
        if cellShd.is_some() {
          // java cellData.mShd = shd;
          cellData.mShd = cellShd;
        }
        // java if (null != brcH)
        if brcH.is_some() {
          // java cellData.setBrcTop(brcH);
          cellData.setBrcTop(brcH);
          // java cellData.setBrcBottom(brcH);
          cellData.setBrcBottom(brcH);
        }
        // java if (null != brcV)
        if brcV.is_some() {
          // java cellData.setBrcLeft(brcV);
          cellData.setBrcLeft(brcV);
          // java cellData.setBrcRight(brcV);
          cellData.setBrcRight(brcV);
        }
        // java if (i == 0)
        if i == 0 {
          // java cellData.setBrcLeft(brcLeft);
          cellData.setBrcLeft(cellBrcLeft);
        }
        // java if (i == size - 1)
        if i == size - 1 {
          // java cellData.setBrcRight(brcRight);
          cellData.setBrcRight(cellBrcRight);
        }
        // java if (isFirstRow)
        if isFirstRow {
          // java cellData.setBrcTop(brcTop);
          cellData.setBrcTop(cellBrcTop);
        }
        // java if (null != brcTopStyle)
        if brcTopStyle.is_some() {
          // java cellData.setBrcTop(brcTopStyle);
          cellData.setBrcTop(brcTopStyle);
        }
        // java if (isLastRow)
        if isLastRow {
          // java cellData.setBrcBottom(brcBottom);
          cellData.setBrcBottom(cellBrcBottom);
        }
        // java if (null != brcBottomStyle)
        if brcBottomStyle.is_some() {
          // java cellData.setBrcBottom(brcBottomStyle);
          cellData.setBrcBottom(brcBottomStyle);
        }
      }
    }

    // java static void calcCellStyle(CellInfo cell, PropertySet property)
    pub fn calcCellStyle__cellinfo_propertyset(cell: &CellInfo, property: &PropertySet) {
      // java Data.CellData cellData = cell.getData();
      let mut cellData: Data::CellData = cell.getData();
      // java SHD shd = (SHD)property.getValue(345);
      let mut shd: SHD = (property.getValue(345)).downcast_ref::<SHD>().unwrap();
      // java if (null != shd)
      if shd.is_some() {
        // java cellData.mShd = shd;
        cellData.mShd = shd;
      }
      // java BRC brcTopStyle = (BRC)property.getValue(346);
      let mut brcTopStyle: BRC = (property.getValue(346)).downcast_ref::<BRC>().unwrap();
      // java BRC brcBottomStyle = (BRC)property.getValue(348);
      let mut brcBottomStyle: BRC = (property.getValue(348)).downcast_ref::<BRC>().unwrap();
      // java BRC brcLeftStyle = (BRC)property.getValue(347);
      let mut brcLeftStyle: BRC = (property.getValue(347)).downcast_ref::<BRC>().unwrap();
      // java BRC brcRightStyle = (BRC)property.getValue(349);
      let mut brcRightStyle: BRC = (property.getValue(349)).downcast_ref::<BRC>().unwrap();
      // java BRC brcHStyle = (BRC)property.getValue(350);
      let mut brcHStyle: BRC = (property.getValue(350)).downcast_ref::<BRC>().unwrap();
      // java BRC brcVStyle = (BRC)property.getValue(351);
      let mut brcVStyle: BRC = (property.getValue(351)).downcast_ref::<BRC>().unwrap();
      // java if (null != brcHStyle)
      if brcHStyle.is_some() {
        // java cellData.setBrcTop(brcHStyle);
        cellData.setBrcTop(brcHStyle);
        // java cellData.setBrcBottom(brcHStyle);
        cellData.setBrcBottom(brcHStyle);
      }
      // java if (null != brcVStyle)
      if brcVStyle.is_some() {
        // java cellData.setBrcLeft(brcVStyle);
        cellData.setBrcLeft(brcVStyle);
        // java cellData.setBrcRight(brcVStyle);
        cellData.setBrcRight(brcVStyle);
      }
      // java if (null != brcTopStyle)
      if brcTopStyle.is_some() {
        // java cellData.setBrcTop(brcTopStyle);
        cellData.setBrcTop(brcTopStyle);
      }
      // java if (null != brcBottomStyle)
      if brcBottomStyle.is_some() {
        // java cellData.setBrcBottom(brcBottomStyle);
        cellData.setBrcBottom(brcBottomStyle);
      }
      // java if (null != brcLeftStyle)
      if brcLeftStyle.is_some() {
        // java cellData.setBrcLeft(brcLeftStyle);
        cellData.setBrcLeft(brcLeftStyle);
      }
      // java if (null != brcRightStyle)
      if brcRightStyle.is_some() {
        // java cellData.setBrcRight(brcRightStyle);
        cellData.setBrcRight(brcRightStyle);
      }
    }

    // java static void calcCellStyle(CellInfo cell, TableCell tableCell, int cellIdx, int[] colorRefLefts, int[] colorRefTops, int[] colorRefRights, int[] colorRefBottoms)
    pub fn calcCellStyle__cellinfo_tablecell_int_intarr_intarr_intarr_intarr(cell: &CellInfo, tableCell: Option<&TableCell>, cellIdx: i32, colorRefLefts: &Vec<i32>, colorRefTops: &Vec<i32>, colorRefRights: &Vec<i32>, colorRefBottoms: &Vec<i32>) {
      // java if (null != tableCell)
      if let Some(tableCell) = tableCell {
        // java Data.CellData cellData = cell.getData();
        let mut cellData: Data::CellData = cell.getData();
        // java BRC brcTop = tableCell.getBrcTop();
        let mut cellBrcTop: BRC = tableCell.getBrcTop();
        // java if (isValidBRC(brcTop))
        if Self::isValidBRC(Some(&cellBrcTop)) {
          // java cellData.setBrcTop(correctBRCColorInDocFormat(brcTop, cellIdx, colorRefTops));
          cellData.setBrcTop(Self::correctBRCColorInDocFormat(Some(&cellBrcTop), cellIdx, Some(&colorRefTops)));
        }
        // java BRC brcLeft = tableCell.getBrcLeft();
        let mut cellBrcLeft: BRC = tableCell.getBrcLeft();
        // java if (isValidBRC(brcLeft))
        if Self::isValidBRC(Some(&cellBrcLeft)) {
          // java cellData.setBrcLeft(correctBRCColorInDocFormat(brcLeft, cellIdx, colorRefLefts));
          cellData.setBrcLeft(Self::correctBRCColorInDocFormat(Some(&cellBrcLeft), cellIdx, Some(&colorRefLefts)));
        }
        // java BRC brcBottom = tableCell.getBrcBottom();
        let mut cellBrcBottom: BRC = tableCell.getBrcBottom();
        // java if (isValidBRC(brcBottom))
        if Self::isValidBRC(Some(&cellBrcBottom)) {
          // java cellData.setBrcBottom(correctBRCColorInDocFormat(brcBottom, cellIdx, colorRefBottoms));
          cellData.setBrcBottom(Self::correctBRCColorInDocFormat(Some(&cellBrcBottom), cellIdx, Some(&colorRefBottoms)));
        }
        // java BRC brcRight = tableCell.getBrcRight();
        let mut cellBrcRight: BRC = tableCell.getBrcRight();
        // java if (isValidBRC(brcRight))
        if Self::isValidBRC(Some(&cellBrcRight)) {
          // java cellData.setBrcRight(correctBRCColorInDocFormat(brcRight, cellIdx, colorRefRights));
          cellData.setBrcRight(Self::correctBRCColorInDocFormat(Some(&cellBrcRight), cellIdx, Some(&colorRefRights)));
        }
      }
    }

    // java public static BRC correctBRCColorInDocFormat(BRC brc, int cellIdx, int[] colorRef)
    pub fn correctBRCColorInDocFormat(brc: Option<&BRC>, cellIdx: i32, colorRef: Option<&Vec<i32>>) -> Option<BRC> {
      // java return null != brc && null != colorRef && cellIdx < colorRef.length ? BRC.obtain(brc, colorRef[cellIdx]) : brc;
      return (if brc.is_some() && colorRef.is_some() && cellIdx < (colorRef.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) { Some(BRC::obtain__brc_float(&brc, colorRef.as_ref().unwrap()[(cellIdx) as usize])) } else { Some(brc) });
    }

    // java public static boolean isValidBRC(BRC brc)
    pub fn isValidBRC(brc: Option<&BRC>) -> Option<bool> {
      // java return null != brc && brc.getBorderType() != 0;
      return brc.is_some() && brc.getBorderType() != 0;
    }

    // java public static boolean isStrictValidBRC(BRC brc)
    pub fn isStrictValidBRC(brc: Option<&BRC>) -> Option<bool> {
      // java return null != brc && brc.getBorderType() != 0 && brc.getBorderType() != 255;
      return brc.is_some() && brc.getBorderType() != 0 && brc.getBorderType() != 255;
    }

    // java public TableJCData calcJC(TableInfo table)
    pub fn calcJC(&mut self, table: &TableInfo) -> TableJCData {
      // java RowInfo firstRow = table.getRowByIndex(0);
      let mut firstRow: RowInfo = table.getRowByIndex(0);
      // java PropertySet finalRowProperty = this.calcRowStyle(firstRow);
      let mut finalRowProperty: PropertySet = self.calcRowStyle(&firstRow);
      // java TextDocument textDocument = this.mDocument.getTextDocument();
      let mut textDocument: TextDocument = self.mDocument.getTextDocument();
      // java boolean alignTableRowByRow = textDocument.getCompatibilityMode().isBefore2013Mode() && textDocument.getProperty().getBoolean(389, false);
      let mut alignTableRowByRow: bool = textDocument.getCompatibilityMode().isBefore2013Mode() && textDocument.getProperty().getBoolean(389, false);
      // java int talbeBefore = 0;
      let mut talbeBefore: i32 = 0;
      // java if (alignTableRowByRow && finalRowProperty.contains(296))
      if alignTableRowByRow && finalRowProperty.contains(296) {
        // java TableLength tableLength = (TableLength)finalRowProperty.getValue(296);
        let mut tableLength: TableLength = (finalRowProperty.getValue(296)).downcast_ref::<TableLength>().unwrap();
        // java talbeBefore = tableLength.getValue();
        talbeBefore = tableLength.getValue();
      }
      // java int tableJC = finalRowProperty.getInt(303, 0);
      let mut tableJC: i32 = finalRowProperty.getInt(303, 0);
      // java TableLength tableLength = (TableLength)finalRowProperty.getValue(298);
      let mut tableLength: TableLength = (finalRowProperty.getValue(298)).downcast_ref::<TableLength>().unwrap();
      // java int tableIndent = 0;
      let mut tableIndent: i32 = 0;
      // java if (null != tableLength)
      if tableLength.is_some() {
        // java tableIndent = tableLength.getValue();
        tableIndent = tableLength.getValue();
      }
      // java TableJCData jcData = new TableJCData(tableJC, firstRow.getRgdxa(0) - talbeBefore, firstRow.getRgdxa(firstRow.size()) - talbeBefore, tableIndent);
      let mut jcData: TableJCData = TableJCData::new(tableJC, firstRow.getRgdxa(0) - talbeBefore, firstRow.getRgdxa((firstRow.len() as i32)) - talbeBefore, tableIndent);
      let mut row: RowInfo = table.getRowByIndex(0);
      // java for(RowInfo row = table.getRowByIndex(0); row != null row = row.getNextRow())
      while row.is_some() {
        // java talbeBefore = 0;
        talbeBefore = 0;
        // java if (alignTableRowByRow)
        if alignTableRowByRow {
          // java PropertySet finalyProperty = row.getFinalProperty();
          let mut finalyProperty: PropertySet = row.getFinalProperty();
          // java if (finalyProperty.contains(296))
          if finalyProperty.contains(296) {
            // java TableLength tableLength2 = (TableLength)finalyProperty.getValue(296);
            let mut tableLength2: TableLength = (finalyProperty.getValue(296)).downcast_ref::<TableLength>().unwrap();
            // java talbeBefore = tableLength2.getValue();
            talbeBefore = tableLength2.getValue();
          }
        }
        // java jcData.jcLeft = Math.min(jcData.jcLeft, row.getRgdxa(0) - talbeBefore);
        jcData.jcLeft = Math::min(jcData.jcLeft, row.getRgdxa(0) - talbeBefore);
        // java jcData.jcRight = Math.max(jcData.jcRight, row.getRgdxa(row.size()) - talbeBefore);
        jcData.jcRight = Math::max(jcData.jcRight, row.getRgdxa((row.len() as i32)) - talbeBefore);
      }
      // java return jcData;
      return jcData;
    }

    // java  void calcRowInfo(RowInfo row, PropertySet finalRowProperty)
    pub fn calcRowInfo(&self, row: &RowInfo, finalRowProperty: &PropertySet) {
      // java Data.RowData rowData = row.getData();
      let mut rowData: Data::RowData = row.getData();
      // java if (finalRowProperty.contains(305))
      if finalRowProperty.contains(305) {
        // java rowData.mRowHeight = finalRowProperty.getInt(305);
        rowData.mRowHeight = finalRowProperty.getInt__int(305);
      }
      // java rowData.mCantSplit = finalRowProperty.getBoolean(304, false);
      rowData.mCantSplit = finalRowProperty.getBoolean__int_boolean(304, false);
      // java DefTable defTable = (DefTable)finalRowProperty.getObject(306, DefTable.empty);
      let mut defTable: DefTable = (finalRowProperty.getObject__int_object(306, &DefTable::empty)).downcast_ref::<DefTable>().unwrap();
      // java int rgdxaCount = defTable.getItMax();
      let mut rgdxaCount: i32 = defTable.getItMax();
      // java if (rgdxaCount > 0)
      if rgdxaCount > 0 {
        // java int[] rgdxaCenter = new int[defTable.getItMax()];
        let mut rgdxaCenter: Vec<i32> = vec![Default::default(); (defTable.getItMax()) as usize];
        let mut i: i32 = 0;
        // java for(int i = 0; i < rgdxaCount; ++i)
        while i < rgdxaCount {
          // java rgdxaCenter[i] = defTable.getRgdxa(i);
          rgdxaCenter[(i) as usize] = defTable.getRgdxa(i);
        }
        // java rowData.mRgdxaCenter = rgdxaCenter;
        rowData.mRgdxaCenter = rgdxaCenter;
      }
      // java if (finalRowProperty.contains(296))
      if finalRowProperty.contains(296) {
        // java TableLength tableLength = (TableLength)finalRowProperty.getValue(296);
        let mut tableLength: TableLength = (finalRowProperty.getValue(296)).downcast_ref::<TableLength>().unwrap();
        // java rowData.mWidthBefore = tableLength.getValue();
        rowData.mWidthBefore = tableLength.getValue();
      }
      // java int rowSize = row.size();
      let mut rowSize: i32 = row.size();
      // java if (finalRowProperty.contains(308))
      if finalRowProperty.contains(308) {
        // java Revision revision = (Revision)finalRowProperty.getValue(363);
        let mut revision: Revision = (finalRowProperty.getValue(363)).downcast_ref::<Revision>().unwrap();
        // java if (null == revision || !revision.getProperty().contains(292) || revision.getProperty().contains(292) && revision.getProperty().contains(308))
        if revision.is_none() || !revision.getProperty().contains(292) || revision.getProperty().contains(292) && revision.getProperty().contains(308) {
          // java TableDefSHD defShd = (TableDefSHD)finalRowProperty.getValue(308);
          let mut defShd: TableDefSHD = (finalRowProperty.getValue(308)).downcast_ref::<TableDefSHD>().unwrap();
          // java if (rowSize != defShd.getCount())
          if rowSize != defShd.getCount() {
            // java KSLog.d(TAG, "row.size() != defShd.size()");
            KSLog::d__string_string(&TAG, &"row.size() != defShd.size()");
          }
          // java int i = 0;
          let mut i: i32 = 0;
          let mut size: i32 = Math::min(rowSize, defShd.getCount());
          // java for(int size = Math.min(rowSize, defShd.getCount()); i < size; ++i)
          while i < size {
            // java CellInfo cell = row.getCellByIndex(i);
            let mut cell: CellInfo = row.getCellByIndex(i);
            // java SHD shd = defShd.getCellSHD(i);
            let mut shd: SHD = defShd.getCellSHD(i);
            // java if (null != shd && shd.getPattern() != 65535)
            if shd.is_some() && shd.getPattern() != 65535 {
              // java cell.getData().mShd = shd;
              cell.getData().mShd = shd;
            }
          }
        }
      }
      // java int[] colorRefLefts = (int[])finalRowProperty.getObject(324, (Object)null);
      let mut colorRefLefts: Vec<i32> = todo!("cast → Vec<i32>");
      // java int[] colorRefTops = (int[])finalRowProperty.getObject(323, (Object)null);
      let mut colorRefTops: Vec<i32> = todo!("cast → Vec<i32>");
      // java int[] colorRefRights = (int[])finalRowProperty.getObject(326, (Object)null);
      let mut colorRefRights: Vec<i32> = todo!("cast → Vec<i32>");
      // java int[] colorRefBottoms = (int[])finalRowProperty.getObject(325, (Object)null);
      let mut colorRefBottoms: Vec<i32> = todo!("cast → Vec<i32>");
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = rowSize;
      // java for(int size = rowSize; i < size; ++i)
      while i < size {
        // java CellInfo cell = row.getCellByIndex(i);
        let mut cell: CellInfo = row.getCellByIndex(i);
        // java TableCell tableCell = defTable.getTableCell(i);
        let mut tableCell: TableCell = defTable.getTableCell(i);
        // java cell.getData().mTableCell = tableCell;
        cell.getData().mTableCell = tableCell;
        // java calcCellStyle(cell, tableCell, i, colorRefLefts, colorRefTops, colorRefRights, colorRefBottoms);
        Self::calcCellStyle__cellinfo_tablecell_int_intarr_intarr_intarr_intarr(&cell, Some(&tableCell), i, &colorRefLefts, &colorRefTops, &colorRefRights, &colorRefBottoms);
        // java setMargins(cell, i, finalRowProperty);
        Self::setMargins(&cell, i, &finalRowProperty);
      }
      // java TableSetBRC[] tableSetBRCs = (TableSetBRC[])finalRowProperty.getValue(311);
      let mut tableSetBRCs: Vec<TableSetBRC> = todo!("cast → Vec<TableSetBRC>");
      // java if (null != tableSetBRCs)
      if tableSetBRCs.is_some() {
        // java i = 0;
        i = 0;
        let mut length: i32 = (tableSetBRCs.len() as i32);
        // java for(int length = tableSetBRCs.length; i < length; ++i)
        while i < length {
          // java TableSetBRC tableSetBRC = tableSetBRCs[i];
          let mut tableSetBRC: TableSetBRC = tableSetBRCs[(i) as usize];
          // java int bordersToApply = tableSetBRC.getBordersToApply();
          let mut bordersToApply: i32 = tableSetBRC.getBordersToApply();
          // java if (this.isDiagonalBRC(bordersToApply))
          if self.isDiagonalBRC(bordersToApply) {
            // java int start = tableSetBRC.getFirst();
            let mut start: i32 = tableSetBRC.getFirst();
            // java int end = tableSetBRC.getLast();
            let mut end: i32 = tableSetBRC.getLast();
            // java int j = start;
            let mut j: i32 = start;
            let mut size: i32 = Math::min(rowSize, end);
            // java for(int size = Math.min(rowSize, end); j < size ++j)
            while j < size {
              // java CellInfo cell = row.getCellByIndex(j);
              let mut cell: CellInfo = row.getCellByIndex(j);
              // java BRC diagonalBRC = normalizeDiagonalCellBRC(tableSetBRC.getBRC());
              let mut diagonalBRC: BRC = Self::normalizeDiagonalCellBRC(Some(&tableSetBRC.getBRC()));
              // java if (bordersToApply == 16)
              if bordersToApply == 16 {
                // java cell.getData().setBrcFromTopLeftToBottomRight(diagonalBRC);
                cell.getData().setBrcFromTopLeftToBottomRight(diagonalBRC);
              }
              // java else
              else {
                // java cell.getData().setBrcFromTopRightToBottomLeft(diagonalBRC);
                cell.getData().setBrcFromTopRightToBottomLeft(diagonalBRC);
              }
            }
          }
        }
      }
    }

    // java public static int getDefaultCellMarginLeft(RowInfo firstRowInfo)
    pub fn getDefaultCellMarginLeft(firstRowInfo: &RowInfo) -> i32 {
      // java PropertySet rowFinalProperty = firstRowInfo.getFinalProperty();
      let mut rowFinalProperty: PropertySet = firstRowInfo.getFinalProperty();
      // java CellMargin[] cellMargins = (CellMargin[])rowFinalProperty.getObject(301, (Object)null);
      let mut cellMargins: Vec<CellMargin> = todo!("cast → Vec<CellMargin>");
      // java if (null != cellMargins)
      if cellMargins.is_some() {
        // java for(CellMargin cellMargin : cellMargins)
        for cellMargin in cellMargins.iter() {
          // java if (0 >= cellMargin.getFirst() && 0 < cellMargin.getBeyond())
          if 0 >= cellMargin.getFirst() && 0 < cellMargin.getBeyond() {
            // java int grbitChg = cellMargin.getGrbitChg();
            let mut grbitChg: i32 = cellMargin.getGrbitChg();
            // java TableLength tableLength = cellMargin.getCellSpacingValue();
            let mut tableLength: TableLength = cellMargin.getCellSpacingValue();
            // java if (0 != (grbitChg & 2) && tableLength.isValid())
            if 0 != grbitChg & 2 && tableLength.isValid() {
              // java return tableLength.getValue();
              return tableLength.getValue();
            }
          }
        }
      }
      // java return 108;
      return 108;
    }

    // java public static int getDefaultCellMarginRight(RowInfo firstRowInfo)
    pub fn getDefaultCellMarginRight(firstRowInfo: &RowInfo) -> i32 {
      // java PropertySet rowFinalProperty = firstRowInfo.getFinalProperty();
      let mut rowFinalProperty: PropertySet = firstRowInfo.getFinalProperty();
      // java CellMargin[] cellMargins = (CellMargin[])rowFinalProperty.getObject(301, (Object)null);
      let mut cellMargins: Vec<CellMargin> = todo!("cast → Vec<CellMargin>");
      // java int lastCellIndex = firstRowInfo.size() - 1;
      let mut lastCellIndex: i32 = firstRowInfo.size() - 1;
      // java if (null != cellMargins)
      if cellMargins.is_some() {
        // java for(CellMargin cellMargin : cellMargins)
        for cellMargin in cellMargins.iter() {
          // java if (lastCellIndex >= cellMargin.getFirst() && lastCellIndex < cellMargin.getBeyond())
          if lastCellIndex >= cellMargin.getFirst() && lastCellIndex < cellMargin.getBeyond() {
            // java int grbitChg = cellMargin.getGrbitChg();
            let mut grbitChg: i32 = cellMargin.getGrbitChg();
            // java TableLength tableLength = cellMargin.getCellSpacingValue();
            let mut tableLength: TableLength = cellMargin.getCellSpacingValue();
            // java if (0 != (grbitChg & 8) && tableLength.isValid())
            if 0 != grbitChg & 8 && tableLength.isValid() {
              // java return tableLength.getValue();
              return tableLength.getValue();
            }
          }
        }
        // java int cellMarginCount = cellMargins.length;
        let mut cellMarginCount: i32 = (cellMargins.len() as i32);
        let mut i: i32 = 0;
        // java for(int i = 0; i < cellMarginCount; ++i)
        while i < cellMarginCount {
          // java CellMargin cellMargin = cellMargins[i];
          let mut cellMargin: CellMargin = cellMargins[(i) as usize];
          // java if (cellMargin.getFirst() == 0 && cellMargin.getBeyond() == 1)
          if cellMargin.getFirst() == 0 && cellMargin.getBeyond() == 1 {
            // java int grbitChg = cellMargin.getGrbitChg();
            let mut grbitChg: i32 = cellMargin.getGrbitChg();
            // java TableLength tableLength = cellMargin.getCellSpacingValue();
            let mut tableLength: TableLength = cellMargin.getCellSpacingValue();
            // java if (0 != (grbitChg & 8) && tableLength.isValid())
            if 0 != grbitChg & 8 && tableLength.isValid() {
              // java return tableLength.getValue();
              return tableLength.getValue();
            }
          }
        }
      }
      // java return 108;
      return 108;
    }

    // java public static int getTableFitMaxWidth(Document document, RowInfo firstRowInfo)
    pub fn getTableFitMaxWidth(document: &dyn Document, firstRowInfo: &RowInfo) -> i32 {
      // java int coreWidth = 0;
      let mut coreWidth: i32 = 0;
      // java int cp = firstRowInfo.getStartCP();
      let mut cp: i32 = firstRowInfo.getStartCP();
      // java switch (document.getType())
      match document.getType() {
        // java case 0: case 1: case 2: case 4:
        0 | 1 | 2 | 4 => {
          // java coreWidth = Tools.getMainDocumentCoreWidth(document, cp);
          coreWidth = Tools::getMainDocumentCoreWidth(&document, cp);
          // java if (document.getTextDocument().getCompatibilityMode().isTableIndentedByOldStyle())
          if document.getTextDocument().getCompatibilityMode().isTableIndentedByOldStyle() {
            // java int leftMargin = getDefaultCellMarginLeft(firstRowInfo);
            let mut leftMargin: i32 = Self::getDefaultCellMarginLeft(&firstRowInfo);
            // java int rightMargin = getDefaultCellMarginRight(firstRowInfo);
            let mut rightMargin: i32 = Self::getDefaultCellMarginRight(&firstRowInfo);
            // java coreWidth += leftMargin + rightMargin;
            coreWidth = coreWidth + leftMargin + rightMargin;
          }
        }
        // java case 3: default:
        3 | _ => {
        }
        // java case 5: case 6:
        5 | 6 => {
          // java coreWidth = Tools.getTextBoxCoreWidth(document, cp);
          coreWidth = Tools::getTextBoxCoreWidth(&document, cp);
        }
      }
      // java if (coreWidth > 0)
      if coreWidth > 0 {
        // java PropertySet propertySet = firstRowInfo.getFinalProperty();
        let mut propertySet: PropertySet = firstRowInfo.getFinalProperty();
        // java int tableJC = propertySet.getInt(303, 0);
        let mut tableJC: i32 = propertySet.getInt(303, 0);
        // java if (tableJC == 0)
        if tableJC == 0 {
          // java TableLength tableLength = (TableLength)propertySet.getValue(298);
          let mut tableLength: TableLength = (propertySet.getValue(298)).downcast_ref::<TableLength>().unwrap();
          // java if (null != tableLength)
          if tableLength.is_some() {
            // java coreWidth -= tableLength.getTWIPS();
            coreWidth = coreWidth - tableLength.getTWIPS();
          }
        }
      }
      // java return coreWidth;
      return coreWidth;
    }

    // java public static int getParentcellFitMaxWidth(RowInfo rowInfo)
    pub fn getParentcellFitMaxWidth(rowInfo: &RowInfo) -> i32 {
      // java TableInfo tableInfo = rowInfo.getParent();
      let mut tableInfo: TableInfo = rowInfo.getParent();
      // java CellInfo parentCell = tableInfo.getParentCell();
      let mut parentCell: CellInfo = tableInfo.getParentCell();
      // java int coreWidth = 0;
      let mut coreWidth: i32 = 0;
      // java if (parentCell != null)
      if parentCell.is_some() {
        // java coreWidth = parentCell.getRgdxaRight() - parentCell.getRgdxaLeft() - parentCell.getMarginLeft() - parentCell.getMarginRight();
        coreWidth = parentCell.getRgdxaRight() - parentCell.getRgdxaLeft() - parentCell.getMarginLeft() - parentCell.getMarginRight();
      }
      // java if (coreWidth > 0)
      if coreWidth > 0 {
        // java PropertySet propertySet = tableInfo.getTableProperty();
        let mut propertySet: PropertySet = tableInfo.getTableProperty();
        // java int tableJC = propertySet.getInt(303, 0);
        let mut tableJC: i32 = propertySet.getInt(303, 0);
        // java if (tableJC == 0)
        if tableJC == 0 {
          // java TableLength tableLength = (TableLength)propertySet.getValue(298);
          let mut tableLength: TableLength = (propertySet.getValue(298)).downcast_ref::<TableLength>().unwrap();
          // java if (null != tableLength)
          if tableLength.is_some() {
            // java coreWidth -= tableLength.getValue();
            coreWidth = coreWidth - tableLength.getValue();
          }
        }
      }
      // java return coreWidth;
      return coreWidth;
    }

    // java public static int getFixedTableWidth(TableInfo tableInfo)
    pub fn getFixedTableWidth(tableInfo: &TableInfo) -> i32 {
      // java PropertySet finalRowProperty = tableInfo.getTableProperty();
      let mut finalRowProperty: PropertySet = tableInfo.getTableProperty();
      // java if (finalRowProperty.contains(295))
      if finalRowProperty.contains(295) {
        // java TableLength tableLength = (TableLength)finalRowProperty.getValue(295);
        let mut tableLength: TableLength = (finalRowProperty.getValue(295)).downcast_ref::<TableLength>().unwrap();
        // java if (tableLength.getType() == 3)
        if tableLength.getType() == 3 {
          // java return tableLength.getValue();
          return tableLength.getValue();
        }
      }
      // java return 0;
      return 0;
    }

    // java public static boolean isTableWithPercent(TableInfo tableInfo)
    pub fn isTableWithPercent(tableInfo: &TableInfo) -> bool {
      // java PropertySet finalRowProperty = tableInfo.getTableProperty();
      let mut finalRowProperty: PropertySet = tableInfo.getTableProperty();
      // java if (finalRowProperty.contains(295))
      if finalRowProperty.contains(295) {
        // java TableLength tableLength = (TableLength)finalRowProperty.getValue(295);
        let mut tableLength: TableLength = (finalRowProperty.getValue(295)).downcast_ref::<TableLength>().unwrap();
        // java return tableLength.getType() == 2;
        return tableLength.getType() == 2;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public static int[] calcCellWidths(RowInfo row, PropertySet finalRowProperty, TableJCData jcData, boolean takeTableWidthFromJcData)
    pub fn calcCellWidths(row: &RowInfo, finalRowProperty: &PropertySet, jcData: &TableJCData, takeTableWidthFromJcData: bool) -> Option<Vec<i32>> {
      // java DefTable defTable = (DefTable)finalRowProperty.getObject(306, DefTable.empty);
      let mut defTable: DefTable = (finalRowProperty.getObject__int_object(306, &DefTable::empty)).downcast_ref::<DefTable>().unwrap();
      // java int cellCount = defTable.getCellCount();
      let mut cellCount: i32 = defTable.getCellCount();
      // java if (cellCount == 0)
      if cellCount == 0 {
        // java return null;
        return None;
      }
      // java else
      else {
        // java int tableWidth = 0;
        let mut tableWidth: i32 = 0;
        // java if (takeTableWidthFromJcData)
        if takeTableWidthFromJcData {
          // java tableWidth = Math.abs(jcData.jcRight - jcData.jcLeft);
          tableWidth = Math::abs(jcData.jcRight - jcData.jcLeft);
        }
        // java boolean isTableWidthAuto = false;
        let mut isTableWidthAuto: bool = false;
        // java if (tableWidth <= 0)
        if tableWidth <= 0 {
          // java if (finalRowProperty.contains(295))
          if finalRowProperty.contains(295) {
            // java TableLength tableLength = (TableLength)finalRowProperty.getValue(295);
            let mut tableLength: TableLength = (finalRowProperty.getValue(295)).downcast_ref::<TableLength>().unwrap();
            // java if (tableLength.getType() == 2)
            if tableLength.getType() == 2 {
              // java int rgdxTotal = defTable.getRgdxa(cellCount) - defTable.getRgdxa(0);
              let mut rgdxTotal: i32 = defTable.getRgdxa(cellCount) - defTable.getRgdxa(0);
              // java if (row.getParent().getLevel() <= 1)
              if row.getParent().getLevel() <= 1 {
                // java Document doc = ((RowInfoNode)row).getDocument();
                let mut doc: Box<dyn Document> = (row).downcast_ref::<RowInfoNode>().unwrap().getDocument();
                // java int coreWidth = getTableFitMaxWidth(doc, row);
                let mut coreWidth: i32 = Self::getTableFitMaxWidth(&doc, &row);
                // java tableWidth = Math.round((float)tableLength.getValue() / 5000.0F * (float)coreWidth);
                tableWidth = Math::round(((tableLength.getValue()) as f32) / 5000 * ((coreWidth) as f32));
                // java if (rgdxTotal < tableWidth && doc.getType() != 5 && doc.getType() != 6)
                if rgdxTotal < tableWidth && doc.getType() != 5 && doc.getType() != 6 {
                  // java tableWidth = rgdxTotal;
                  tableWidth = rgdxTotal;
                }
              }
              // java else
              else {
                // java int coreWidth = getParentcellFitMaxWidth(row);
                let mut coreWidth: i32 = Self::getParentcellFitMaxWidth(&row);
                // java tableWidth = Math.round((float)tableLength.getValue() / 5000.0F * (float)coreWidth);
                tableWidth = Math::round(((tableLength.getValue()) as f32) / 5000 * ((coreWidth) as f32));
              }
            }
            // java else if (tableLength.getType() == 3)
            else if tableLength.getType() == 3 {
              // java tableWidth = tableLength.getValue();
              tableWidth = tableLength.getValue();
            }
            // java else if (tableLength.getType() == 1)
            else if tableLength.getType() == 1 {
              // java isTableWidthAuto = true;
              isTableWidthAuto = true;
            }
          }
          // java else
          else {
            // java isTableWidthAuto = true;
            isTableWidthAuto = true;
          }
        }
        // java int firstWidthType = defTable.getTableCell(0).getWidthType();
        let mut firstWidthType: i32 = defTable.getTableCell(0).getWidthType();
        // java if (firstWidthType == 2 && tableWidth == 0)
        if firstWidthType == 2 && tableWidth == 0 {
          // java tableWidth = defTable.getRgdxa(cellCount) - defTable.getRgdxa(0);
          tableWidth = defTable.getRgdxa(cellCount) - defTable.getRgdxa(0);
        }
        // java int[] cellWidths = new int[cellCount];
        let mut cellWidths: Vec<i32> = vec![Default::default(); (cellCount) as usize];
        // java int rowWidthTotal = 0;
        let mut rowWidthTotal: i32 = 0;
        // java if (firstWidthType == 2)
        if firstWidthType == 2 {
          // java float pctTotalIn100 = 0.0F;
          let mut pctTotalIn100: f32 = 0;
          let mut i: i32 = 0;
          // java for(int i = 0; i < cellCount; ++i)
          while i < cellCount {
            // java TableCell cell = defTable.getTableCell(i);
            let mut cell: TableCell = defTable.getTableCell(i);
            // java float pct = (float)cell.getWidth() / 5000.0F;
            let mut pct: f32 = ((cell.getWidth()) as f32) / 5000;
            // java if (pct <= 1.0F - pctTotalIn100)
            if pct <= 1 - pctTotalIn100 {
              // java cellWidths[i] = (int)(pct * (float)tableWidth);
              cellWidths[(i) as usize] = ((pct * ((tableWidth) as f32)) as i32);
              // java pctTotalIn100 += pct;
              pctTotalIn100 = pctTotalIn100 + pct;
            }
            // java else
            else {
              // java float leavePct = 1.0F - pctTotalIn100;
              let mut leavePct: f32 = 1 - pctTotalIn100;
              // java if (leavePct > 0.0F)
              if leavePct > 0 {
                // java cellWidths[i] = (int)(leavePct * (float)tableWidth);
                cellWidths[(i) as usize] = ((leavePct * ((tableWidth) as f32)) as i32);
                // java pctTotalIn100 = 1.0F;
                pctTotalIn100 = 1;
              }
              // java else
              else {
                // java cellWidths[i] = 0;
                cellWidths[(i) as usize] = 0;
              }
            }
            // java rowWidthTotal += cellWidths[i];
            rowWidthTotal = rowWidthTotal + cellWidths[(i) as usize];
          }
        }
        // java else
        else {
          // java float autoFourceWidthTotal = 0.0F;
          let mut autoFourceWidthTotal: f32 = 0;
          let mut i: i32 = 0;
          // java for(int i = 0; i < cellCount; ++i)
          while i < cellCount {
            // java TableCell cell = defTable.getTableCell(i);
            let mut cell: TableCell = defTable.getTableCell(i);
            // java int widthType = cell.getWidthType();
            let mut widthType: i32 = cell.getWidthType();
            // java int width = cell.getWidth();
            let mut width: i32 = cell.getWidth();
            // java if (widthType == 0 && width == 0)
            if widthType == 0 && width == 0 {
              // java width = defTable.getRgdxa(i + 1) - defTable.getRgdxa(i);
              width = defTable.getRgdxa(i + 1) - defTable.getRgdxa(i);
              // java autoFourceWidthTotal += (float)width;
              autoFourceWidthTotal = autoFourceWidthTotal + ((width) as f32);
            }
            // java else if (!isTableWidthAuto && widthType == 1)
            else if !isTableWidthAuto && widthType == 1 {
              // java width = defTable.getRgdxa(i + 1) - defTable.getRgdxa(i);
              width = defTable.getRgdxa(i + 1) - defTable.getRgdxa(i);
              // java autoFourceWidthTotal += (float)width;
              autoFourceWidthTotal = autoFourceWidthTotal + ((width) as f32);
            }
            // java cellWidths[i] = width;
            cellWidths[(i) as usize] = width;
            // java rowWidthTotal += width;
            rowWidthTotal = rowWidthTotal + width;
          }
          // java if (tableWidth > 0 && rowWidthTotal != tableWidth)
          if tableWidth > 0 && rowWidthTotal != tableWidth {
            // java if (rowWidthTotal > tableWidth && autoFourceWidthTotal > 0.0F)
            if rowWidthTotal > tableWidth && autoFourceWidthTotal > 0 {
              // java float adjustAutoTotal = Math.min((float)(rowWidthTotal - tableWidth), autoFourceWidthTotal);
              let mut adjustAutoTotal: f32 = Math::min(((rowWidthTotal - tableWidth) as f32), autoFourceWidthTotal);
              // java rowWidthTotal = 0;
              rowWidthTotal = 0;
              let mut i: i32 = 0;
              // java for(int i = 0; i < cellCount; ++i)
              while i < cellCount {
                // java if (defTable.getTableCell(i).getWidthType() == 1)
                if defTable.getTableCell(i).getWidthType() == 1 {
                  // java cellWidths[i] = (int)((float)cellWidths[i] - adjustAutoTotal * (float)cellWidths[i] / autoFourceWidthTotal);
                  cellWidths[(i) as usize] = ((((cellWidths[(i) as usize]) as f32) - adjustAutoTotal * ((cellWidths[(i) as usize]) as f32) / autoFourceWidthTotal) as i32);
                }
                // java rowWidthTotal += cellWidths[i];
                rowWidthTotal = rowWidthTotal + cellWidths[(i) as usize];
              }
            }
            // java if (rowWidthTotal != tableWidth && Math.abs(jcData.jcLeft - defTable.getRgdxa(0)) < 40 && Math.abs(jcData.jcRight - defTable.getRgdxa(cellCount)) < 40)
            if rowWidthTotal != tableWidth && Math::abs(jcData.jcLeft - defTable.getRgdxa(0)) < 40 && Math::abs(jcData.jcRight - defTable.getRgdxa(cellCount)) < 40 {
              let mut i: i32 = 0;
              // java for(int i = 0; i < cellCount; ++i)
              while i < cellCount {
                // java int width = cellWidths[i];
                let mut width: i32 = cellWidths[(i) as usize];
                // java cellWidths[i] = tableWidth * width / rowWidthTotal;
                cellWidths[(i) as usize] = tableWidth * width / rowWidthTotal;
              }
              // java rowWidthTotal = tableWidth;
              rowWidthTotal = tableWidth;
            }
          }
        }
        // java int needReduce = 0;
        let mut needReduce: i32 = 0;
        // java int leaveWidth = 0;
        let mut leaveWidth: i32 = 0;
        // java int i = 0;
        let mut i: i32 = 0;
        let mut size: i32 = cellCount;
        // java for(int size = cellCount; i < size; ++i)
        while i < size {
          // java Data.CellData cellData = row.getCellByIndex(i).getData();
          let mut cellData: Data::CellData = row.getCellByIndex(i).getData();
          // java int oriWidth = cellWidths[i];
          let mut oriWidth: i32 = cellWidths[(i) as usize];
          // java int minWidth = cellData.mMarginLeft + cellData.mMarginRight;
          let mut minWidth: i32 = cellData.mMarginLeft + cellData.mMarginRight;
          // java int diff = oriWidth - minWidth;
          let mut diff: i32 = oriWidth - minWidth;
          // java if (diff < 0)
          if diff < 0 {
            // java needReduce += -diff;
            needReduce = needReduce + -diff;
          }
          // java else
          else {
            // java leaveWidth += diff;
            leaveWidth = leaveWidth + diff;
          }
        }
        // java if (needReduce > 0)
        if needReduce > 0 {
          // java Document doc = ((RowInfoNode)row).getDocument();
          let mut doc: Box<dyn Document> = (row).downcast_ref::<RowInfoNode>().unwrap().getDocument();
          // java int coreWidth = getTableFitMaxWidth(doc, row);
          let mut coreWidth: i32 = Self::getTableFitMaxWidth(&doc, &row);
          // java if (isTableWidthAuto && coreWidth >= rowWidthTotal + needReduce)
          if isTableWidthAuto && coreWidth >= rowWidthTotal + needReduce {
            // java i = 0;
            i = 0;
            let mut size: i32 = cellCount;
            // java for(int size = cellCount; i < size; ++i)
            while i < size {
              // java Data.CellData cellData = row.getCellByIndex(i).getData();
              let mut cellData: Data::CellData = row.getCellByIndex(i).getData();
              // java int oriWidth = cellWidths[i];
              let mut oriWidth: i32 = cellWidths[(i) as usize];
              // java int minWidth = cellData.mMarginLeft + cellData.mMarginRight;
              let mut minWidth: i32 = cellData.mMarginLeft + cellData.mMarginRight;
              // java int diff = oriWidth - minWidth;
              let mut diff: i32 = oriWidth - minWidth;
              // java if (diff < 0)
              if diff < 0 {
                // java cellWidths[i] = minWidth;
                cellWidths[(i) as usize] = minWidth;
              }
            }
          }
          // java else
          else {
            // java if (needReduce > leaveWidth)
            if needReduce > leaveWidth {
              // java needReduce = leaveWidth;
              needReduce = leaveWidth;
            }
            // java i = 0;
            i = 0;
            let mut size: i32 = cellCount;
            // java for(int size = cellCount; i < size; ++i)
            while i < size {
              // java Data.CellData cellData = row.getCellByIndex(i).getData();
              let mut cellData: Data::CellData = row.getCellByIndex(i).getData();
              // java int oriWidth = cellWidths[i];
              let mut oriWidth: i32 = cellWidths[(i) as usize];
              // java int minWidth = cellData.mMarginLeft + cellData.mMarginRight;
              let mut minWidth: i32 = cellData.mMarginLeft + cellData.mMarginRight;
              // java int diff = oriWidth - minWidth;
              let mut diff: i32 = oriWidth - minWidth;
              // java if (diff < 0)
              if diff < 0 {
                // java cellWidths[i] = minWidth;
                cellWidths[(i) as usize] = minWidth;
              }
              // java else if (leaveWidth > 0 && diff > 0)
              else if leaveWidth > 0 && diff > 0 {
                // java cellWidths[i] -= diff * needReduce / leaveWidth;
                cellWidths[(i) as usize] = cellWidths[(i) as usize] - diff * needReduce / leaveWidth;
              }
            }
          }
        }
        // java return cellWidths;
        return cellWidths;
      }
    }

    // java private static BRC normalizeDiagonalCellBRC(BRC brc)
    pub fn normalizeDiagonalCellBRC(brc: Option<&BRC>) -> Option<BRC> {
      // java if (null == brc)
      if brc.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java BRC ret = brc;
        let mut ret: BRC = brc;
        // java switch (brc.getBorderType())
        match brc.getBorderType() {
          // java case 3:
          3 => {
            // java if (brc.getColor() != -16777216)
            if brc.getColor() != -16777216 {
              // java ret = BRC.obtain(brc, brc.getLineWidth() * 3.0F, 1);
              ret = BRC::obtain__float_int_int(brc, brc.getLineWidth() * 3, 1);
            }
          }
          // java case 4: case 5: case 10: case 11: case 12: case 13: case 14: case 15: case 16: case 17: case 18: case 19: case 20: case 21: default:
          4 | 5 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | _ => {
          }
          // java case 6: case 7: case 8: case 9: case 22: case 23:
          6 | 7 | 8 | 9 | 22 | 23 => {
            // java ret = BRC.obtain(brc, 0.5F, brc.getBorderType());
            ret = BRC::obtain__brc_float_int(&brc, 0.5, brc.getBorderType());
          }
        }
        // java return ret;
        return ret;
      }
    }

    // java private final boolean isDiagonalBRC(int bordersToApply)
    pub fn isDiagonalBRC(&self, bordersToApply: i32) -> bool {
      // java return bordersToApply == 16 || bordersToApply == 32;
      return bordersToApply == 16 || bordersToApply == 32;
    }

    // java static void setMargins(CellInfo cell, int cellIndex, PropertySet finalRowProperty)
    pub fn setMargins(cell: &CellInfo, cellIndex: i32, finalRowProperty: &PropertySet) {
      // java Data.CellData cellData = cell.getData();
      let mut cellData: Data::CellData = cell.getData();
      // java cellData.mMarginLeft = 108;
      cellData.mMarginLeft = 108;
      // java cellData.mMarginTop = 0;
      cellData.mMarginTop = 0;
      // java cellData.mMarginRight = 108;
      cellData.mMarginRight = 108;
      // java cellData.mMarginBottom = 0;
      cellData.mMarginBottom = 0;
      // java CellMargin[] cellMargins = (CellMargin[])finalRowProperty.getObject(301, (Object)null);
      let mut cellMargins: Vec<CellMargin> = todo!("cast → Vec<CellMargin>");
      // java if (null != cellMargins)
      if cellMargins.is_some() {
        // java for(CellMargin cellMargin : cellMargins)
        for cellMargin in cellMargins.iter() {
          // java calcMargin(cell, cellMargin);
          Self::calcMargin(Some(&cell), Some(&cellMargin));
        }
      }
      // java cellMargins = (CellMargin[])finalRowProperty.getObject(300, (Object)null);
      cellMargins = todo!("cast → Vec<CellMargin>");
      // java if (null != cellMargins)
      if cellMargins.is_some() {
        // java for(CellMargin cellMargin : cellMargins)
        for cellMargin in cellMargins.iter() {
          // java if (cellIndex >= cellMargin.getFirst() && cellIndex < cellMargin.getBeyond())
          if cellIndex >= cellMargin.getFirst() && cellIndex < cellMargin.getBeyond() {
            // java calcMargin(cell, cellMargin);
            Self::calcMargin(Some(&cell), Some(&cellMargin));
          }
        }
      }
    }

    // java static void calcMargin(CellInfo cell, CellMargin cellMargin)
    pub fn calcMargin(cell: Option<&CellInfo>, cellMargin: Option<&CellMargin>) {
      // java if (cell != null)
      if let Some(cell) = cell {
        // java Data.CellData cellData = cell.getData();
        let mut cellData: Data::CellData = cell.getData();
        // java if (null != cellMargin)
        if let Some(cellMargin) = cellMargin {
          // java int grbitChg = cellMargin.getGrbitChg();
          let mut grbitChg: i32 = cellMargin.getGrbitChg();
          // java TableLength tableLength = cellMargin.getCellSpacingValue();
          let mut tableLength: TableLength = cellMargin.getCellSpacingValue();
          // java if (tableLength.isValid())
          if tableLength.isValid() {
            // java int value = 0;
            let mut value: i32 = 0;
            // java switch (tableLength.getType())
            match tableLength.getType() {
              // java case 0: case 1: case 3:
              0 | 1 | 3 => {
                // java value = tableLength.getValue();
                value = tableLength.getValue();
              }
              // java case 2: default:
              2 | _ => {
                // java value = tableLength.getValue();
                value = tableLength.getValue();
              }
            }
            // java if (0 != (grbitChg & 1))
            if 0 != grbitChg & 1 {
              // java cellData.mMarginTop = value;
              cellData.mMarginTop = value;
            }
            // java if (0 != (grbitChg & 2))
            if 0 != grbitChg & 2 {
              // java cellData.mMarginLeft = value;
              cellData.mMarginLeft = value;
            }
            // java if (0 != (grbitChg & 4))
            if 0 != grbitChg & 4 {
              // java cellData.mMarginBottom = value;
              cellData.mMarginBottom = value;
            }
            // java if (0 != (grbitChg & 8))
            if 0 != grbitChg & 8 {
              // java cellData.mMarginRight = value;
              cellData.mMarginRight = value;
            }
          }
        }
      }
    }

    // java static void mergeCells(TableInfo tableInfoImpl)
    pub fn mergeCells(tableInfoImpl: &TableInfo) {
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = tableInfoImpl.size() - 1;
      // java for(int size = tableInfoImpl.size() - 1; i < size; ++i)
      while i < size {
        // java RowInfo row = tableInfoImpl.getRowByIndex(i);
        let mut row: RowInfo = tableInfoImpl.getRowByIndex(i);
        // java int j = 0;
        let mut j: i32 = 0;
        let mut sizeOfCell: i32 = (row.len() as i32);
        // java for(int sizeOfCell = row.size(); j < sizeOfCell ++j)
        while j < sizeOfCell {
          // java CellInfo cell = row.getCellByIndex(j);
          let mut cell: CellInfo = row.getCellByIndex(j);
          // java Data.CellData cellData = cell.getData();
          let mut cellData: Data::CellData = cell.getData();
          // java TableCell tableCell = cellData.mTableCell;
          let mut tableCell: TableCell = cellData.mTableCell;
          // java if (null != tableCell && 0 != tableCell.getVertMerge() && 0 == tableCell.getHoriMerge())
          if tableCell.is_some() && 0 != tableCell.getVertMerge() && 0 == tableCell.getHoriMerge() {
            // java CellInfo nextMergeCell = getNextMergeCell(tableInfoImpl.getRowByIndex(i + 1), (float)cell.getRgdxaLeft(), (float)cell.getRgdxaRight());
            let mut nextMergeCell: CellInfo = Self::getNextMergeCell__rowinfo_float_float(Some(&tableInfoImpl.getRowByIndex(i + 1)), ((cell.getRgdxaLeft()) as f32), ((cell.getRgdxaRight()) as f32));
            // java if (null != nextMergeCell)
            if nextMergeCell.is_some() {
              // java cellData.mNextVertMergeCell = nextMergeCell;
              cellData.mNextVertMergeCell = nextMergeCell;
              // java nextMergeCell.getData().mPrevVertMergeCell = cell;
              nextMergeCell.getData().mPrevVertMergeCell = cell;
            }
          }
        }
      }
    }

    // java public static Integer getNextMergeCell(DefTable thisRow, int cellIndex, DefTable nextRow)
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

    // java public static void spilitTable(Document document, RowInfo row)
    pub fn spilitTable(document: &dyn Document, row: &RowInfo) {
      // java RowInfo prevRow = row.getPrevRow();
      let mut prevRow: RowInfo = row.getPrevRow();
      // java MutablePropertySet propertySet = new MutablePropertySet();
      let mut propertySet: MutablePropertySet = MutablePropertySet::new();
      // java propertySet.add(prevRow.getFinalProperty());
      propertySet.add(prevRow.getFinalProperty());
      // java DefTable oldDefTable = (DefTable)propertySet.getValue(306);
      let mut oldDefTable: DefTable = (propertySet.getValue(306)).downcast_ref::<DefTable>().unwrap();
      // java DefTableBuilder newDefTable = DefTableBuilder.obtain(oldDefTable);
      let mut newDefTable: DefTableBuilder = DefTableBuilder::obtain__deftable(&oldDefTable);
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = oldDefTable.getCellCount();
      // java for(int size = oldDefTable.getCellCount(); i < size; ++i)
      while i < size {
        // java TableCellBuilder cell = newDefTable.getTableCellBuilder(i);
        let mut cell: TableCellBuilder = newDefTable.getTableCellBuilder(i);
        // java if (cell.getVertMerge() == 3)
        if cell.getVertMerge() == 3 {
          // java cell.setVertMerge(0);
          cell.setVertMerge(0);
        }
      }
      // java propertySet.setValue(306, newDefTable);
      propertySet.setValue(306, newDefTable);
      // java document.getPapxTable().seek(prevRow.getEndCP() - 1).setProperty(propertySet.freeze());
      document.getPapxTable().seek(prevRow.getEndCP() - 1).setProperty(propertySet.freeze());
      // java DefTable nextDefTable = null;
      let mut nextDefTable: DefTable = DefTable::default();
      // java RowInfo nextRow = row.getNextRow();
      let mut nextRow: RowInfo = row.getNextRow();
      // java if (nextRow != null)
      if nextRow.is_some() {
        // java nextDefTable = (DefTable)nextRow.getFinalProperty().getValue(306);
        nextDefTable = (nextRow.getFinalProperty().getValue(306)).downcast_ref::<DefTable>().unwrap();
      }
      // java propertySet = new MutablePropertySet();
      propertySet = MutablePropertySet::new();
      // java propertySet.add(row.getFinalProperty());
      propertySet.add(row.getFinalProperty());
      // java oldDefTable = (DefTable)propertySet.getValue(306);
      oldDefTable = (propertySet.getValue(306)).downcast_ref::<DefTable>().unwrap();
      // java newDefTable = DefTableBuilder.obtain(oldDefTable);
      newDefTable = DefTableBuilder::obtain__deftable(&oldDefTable);
      // java i = 0;
      i = 0;
      let mut size: i32 = oldDefTable.getCellCount();
      // java for(int size = oldDefTable.getCellCount(); i < size; ++i)
      while i < size {
        // java TableCellBuilder cell = newDefTable.getTableCellBuilder(i);
        let mut cell: TableCellBuilder = newDefTable.getTableCellBuilder(i);
        // java if (cell.getVertMerge() == 1)
        if cell.getVertMerge() == 1 {
          // java if (getNextMergeCell(newDefTable, i, nextDefTable) != null)
          if Self::getNextMergeCell__deftable_int_deftable(&newDefTable, i, Some(&nextDefTable)).is_some() {
            // java cell.setVertMerge(3);
            cell.setVertMerge(3);
          }
          // java else
          else {
            // java cell.setVertMerge(0);
            cell.setVertMerge(0);
          }
        }
      }
      // java propertySet.setValue(306, newDefTable);
      propertySet.setValue(306, newDefTable);
      // java document.getPapxTable().seek(row.getEndCP() - 1).setProperty(propertySet.freeze());
      document.getPapxTable().seek(row.getEndCP() - 1).setProperty(propertySet.freeze());
      // java document.getTableInfos().invalidAfter(row.getStartCP());
      document.getTableInfos().invalidAfter(row.getStartCP());
    }

    // java public static Integer getPrevMergeCell(DefTable thisRow, int cellIndex, DefTable prevRow)
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
    pub fn getNextMergeCell__rowinfo_float_float(row: Option<&RowInfo>, rgdxaLeft: f32, rgdxaRight: f32) -> Option<CellInfo> {
      // java if (null == row)
      if row.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java CellInfo meetLastCell = null;
        let mut meetLastCell: CellInfo = CellInfo::default();
        // java int i = 0;
        let mut i: i32 = 0;
        let mut size: i32 = row.size();
        // java for(int size = row.size(); i < size; ++i)
        while i < size {
          // java CellInfo cell = row.getCellByIndex(i);
          let mut cell: CellInfo = row.getCellByIndex(i);
          // java TableCell rawCell = cell.getData().mTableCell;
          let mut rawCell: TableCell = cell.getData().mTableCell;
          // java if (null != rawCell && 1 == rawCell.getVertMerge())
          if rawCell.is_some() && 1 == rawCell.getVertMerge() {
            // java float leftDiff = Math.abs((float)cell.getRgdxaLeft() - rgdxaLeft);
            let mut leftDiff: f32 = Math::abs(((cell.getRgdxaLeft()) as f32) - rgdxaLeft);
            // java float rightDiff = Math.abs((float)cell.getRgdxaRight() - rgdxaRight);
            let mut rightDiff: f32 = Math::abs(((cell.getRgdxaRight()) as f32) - rgdxaRight);
            // java if (leftDiff < 6.0F && rightDiff < 6.0F && (null == meetLastCell || leftDiff <= Math.abs((float)meetLastCell.getRgdxaLeft() - rgdxaLeft) && rightDiff <= Math.abs((float)meetLastCell.getRgdxaRight() - rgdxaRight)))
            if leftDiff < 6 && rightDiff < 6 && meetLastCell.is_none() || leftDiff <= Math::abs(((meetLastCell.getRgdxaLeft()) as f32) - rgdxaLeft) && rightDiff <= Math::abs(((meetLastCell.getRgdxaRight()) as f32) - rgdxaRight) {
              // java meetLastCell = cell;
              meetLastCell = cell;
            }
          }
        }
        // java return meetLastCell;
        return meetLastCell;
      }
    }

    // java public static CellInfo getPrevMergeCell(RowInfo row, float rgdxaLeft, float rgdxaRight)
    pub fn getPrevMergeCell__rowinfo_float_float(row: Option<&RowInfo>, rgdxaLeft: f32, rgdxaRight: f32) -> Option<CellInfo> {
      // java if (null == row)
      if row.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java CellInfo meetLastCell = null;
        let mut meetLastCell: CellInfo = CellInfo::default();
        // java int i = 0;
        let mut i: i32 = 0;
        let mut size: i32 = row.size();
        // java for(int size = row.size(); i < size; ++i)
        while i < size {
          // java CellInfo cell = row.getCellByIndex(i);
          let mut cell: CellInfo = row.getCellByIndex(i);
          // java TableCell rawCell = cell.getData().mTableCell;
          let mut rawCell: TableCell = cell.getData().mTableCell;
          // java if (null != rawCell)
          if rawCell.is_some() {
            // java int vMerged = rawCell.getVertMerge();
            let mut vMerged: i32 = rawCell.getVertMerge();
            // java if (1 == vMerged || 3 == vMerged)
            if 1 == vMerged || 3 == vMerged {
              // java float leftDiff = Math.abs((float)cell.getRgdxaLeft() - rgdxaLeft);
              let mut leftDiff: f32 = Math::abs(((cell.getRgdxaLeft()) as f32) - rgdxaLeft);
              // java float rightDiff = Math.abs((float)cell.getRgdxaRight() - rgdxaRight);
              let mut rightDiff: f32 = Math::abs(((cell.getRgdxaRight()) as f32) - rgdxaRight);
              // java if (leftDiff < 6.0F && rightDiff < 6.0F && (null == meetLastCell || leftDiff <= Math.abs((float)meetLastCell.getRgdxaLeft() - rgdxaLeft) && rightDiff <= Math.abs((float)meetLastCell.getRgdxaRight() - rgdxaRight)))
              if leftDiff < 6 && rightDiff < 6 && meetLastCell.is_none() || leftDiff <= Math::abs(((meetLastCell.getRgdxaLeft()) as f32) - rgdxaLeft) && rightDiff <= Math::abs(((meetLastCell.getRgdxaRight()) as f32) - rgdxaRight) {
                // java meetLastCell = cell;
                meetLastCell = cell;
              }
            }
          }
        }
        // java return meetLastCell;
        return meetLastCell;
      }
    }

    // java  IPos getTablePos(PropertySet property)
    pub fn getTablePos(&self, property: &PropertySet) -> Option<Box<dyn IPos>> {
      // java if (!property.contains(313) && !property.contains(314) && !property.contains(312))
      if !property.contains(313) && !property.contains(314) && !property.contains(312) {
        // java return null;
        return None;
      }
      // java else
      else {
        // java int dxaAbs = property.getInt(313, 0);
        let mut dxaAbs: i32 = property.getInt__int_int(313, 0);
        // java int dyaAbs = property.getInt(314, 0);
        let mut dyaAbs: i32 = property.getInt__int_int(314, 0);
        // java PositionCode pc = (PositionCode)property.getObject(312, new PositionCode(0, 0));
        let mut pc: PositionCode = (property.getObject__int_object(312, &PositionCode::new(0, 0))).downcast_ref::<PositionCode>().unwrap();
        // java if (pc.isNotWrap() && dxaAbs == 0 && dyaAbs == 0 && !property.getBoolean(319, false))
        if pc.isNotWrap() && dxaAbs == 0 && dyaAbs == 0 && !property.getBoolean__int_boolean(319, false) {
          // java return null;
          return None;
        }
        // java else
        else {
          // java IPos pos = new TablePos();
          let mut pos: Box<dyn IPos> = TablePos::new();
          // java pos.setAllowOverlap(!property.getBoolean(319, false));
          pos.setAllowOverlap(!property.getBoolean__int_boolean(319, false));
          // java pos.setWrapDistLeft(UnitUtil.TWIPS2PT(property.getInt(315, 0)));
          pos.setWrapDistLeft(UnitUtil::TWIPS2PT__int(property.getInt__int_int(315, 0)));
          // java pos.setWrapDistRight(UnitUtil.TWIPS2PT(property.getInt(316, 0)));
          pos.setWrapDistRight(UnitUtil::TWIPS2PT__int(property.getInt__int_int(316, 0)));
          // java pos.setWrapDistTop(UnitUtil.TWIPS2PT(property.getInt(317, 0)));
          pos.setWrapDistTop(UnitUtil::TWIPS2PT__int(property.getInt__int_int(317, 0)));
          // java pos.setWrapDistBottom(UnitUtil.TWIPS2PT(property.getInt(318, 0)));
          pos.setWrapDistBottom(UnitUtil::TWIPS2PT__int(property.getInt__int_int(318, 0)));
          // java boolean isRTL = property.getBoolean(343, false);
          let mut isRTL: bool = property.getBoolean__int_boolean(343, false);
          // java switch (dxaAbs)
          match dxaAbs {
            // java case -16:
            -16 => {
              // java pos.setAlignmentH(5);
              pos.setAlignmentH(5);
            }
            // java case -12:
            -12 => {
              // java pos.setAlignmentH(4);
              pos.setAlignmentH(4);
            }
            // java case -8:
            -8 => {
              // java pos.setAlignmentH(isRTL ? 1 : 3);
              pos.setAlignmentH((if isRTL { Some(1) } else { Some(3) }));
            }
            // java case -4:
            -4 => {
              // java pos.setAlignmentH(2);
              pos.setAlignmentH(2);
            }
            // java case 0:
            0 => {
              // java pos.setAlignmentH(isRTL ? 3 : 1);
              pos.setAlignmentH((if isRTL { Some(3) } else { Some(1) }));
            }
            // java default:
            _ => {
              // java pos.setAlignmentH(0);
              pos.setAlignmentH(0);
              // java RectF rt = pos.getShapeRect();
              let mut rt: RectF = pos.getShapeRect();
              // java rt.left = (float)MetricUtility.Twips2Points(dxaAbs - 1);
              rt.left = ((MetricUtility::Twips2Points__int(dxaAbs - 1)) as f32);
            }
          }
          // java switch (dyaAbs)
          match dyaAbs {
            // java case -20:
            -20 => {
              // java pos.setAlignmentV(5);
              pos.setAlignmentV(5);
            }
            // java case -16:
            -16 => {
              // java pos.setAlignmentV(4);
              pos.setAlignmentV(4);
            }
            // java case -12:
            -12 => {
              // java pos.setAlignmentV(3);
              pos.setAlignmentV(3);
            }
            // java case -8:
            -8 => {
              // java pos.setAlignmentV(2);
              pos.setAlignmentV(2);
            }
            // java case -4:
            -4 => {
              // java pos.setAlignmentV(1);
              pos.setAlignmentV(1);
            }
            // java case 0: default:
            0 | _ => {
              // java pos.setAlignmentV(0);
              pos.setAlignmentV(0);
              // java RectF rt = pos.getShapeRect();
              let mut rt: RectF = pos.getShapeRect();
              // java rt.top = (float)MetricUtility.Twips2Points(dyaAbs - 1);
              rt.top = ((MetricUtility::Twips2Points__int(dyaAbs - 1)) as f32);
            }
          }
          // java pos.setRelativeH(2);
          pos.setRelativeH(2);
          // java pos.setRelativeV(0);
          pos.setRelativeV(0);
          // java if (property.contains(312))
          if property.contains(312) {
            // java switch (pc.getVertPos())
            match pc.getVertPos() {
              // java case 0:
              0 => {
                // java pos.setRelativeV(0);
                pos.setRelativeV(0);
              }
              // java case 1:
              1 => {
                // java pos.setRelativeV(1);
                pos.setRelativeV(1);
              }
              // java case 2:
              2 => {
                // java pos.setRelativeV(2);
                pos.setRelativeV(2);
              }
              // java default:
              _ => {
                // java pos.setRelativeV(0);
                pos.setRelativeV(0);
              }
            }
            // java switch (pc.getHorzPos())
            match pc.getHorzPos() {
              // java case 0:
              0 => {
                // java pos.setRelativeH(2);
                pos.setRelativeH(2);
              }
              // java case 1:
              1 => {
                // java pos.setRelativeH(0);
                pos.setRelativeH(0);
              }
              // java case 2:
              2 => {
                // java pos.setRelativeH(1);
                pos.setRelativeH(1);
              }
              // java default:
              _ => {
                // java pos.setRelativeH(2);
                pos.setRelativeH(2);
              }
            }
          }
          // java return pos;
          return pos;
        }
      }
    }

}

// java static class FinalTableCellProperty
impl FinalTableCellProperty {
    // java public boolean isEmpty()
    pub fn isEmpty(&self) -> Option<bool> {
      // java return null == this.shd && null == this.brcLeft && null == this.brcTop && null == this.brcRight && null == this.brcBottom && null == this.brcVInside && null == this.brcHInside;
      return self.shd.is_none() && self.brcLeft.is_none() && self.brcTop.is_none() && self.brcRight.is_none() && self.brcBottom.is_none() && self.brcVInside.is_none() && self.brcHInside.is_none();
    }

}
