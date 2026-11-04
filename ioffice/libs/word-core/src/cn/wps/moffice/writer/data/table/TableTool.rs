#![allow(non_snake_case)]

// Java: import cn.wps.base.io.MetricUtility
use util::base::io::MetricUtility::MetricUtility;
// Java: import cn.wps.moffice.drawing.IPos
use drawing::cn::wps::moffice::drawing::IPos::IPos;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::UnitUtil;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::KStyles;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};
// Java: import cn.wps.moffice.writer.data.ITable
use crate::cn::wps::moffice::writer::data::ITable::Entry as ITableEntry;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::PropertyUtil;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import cn.wps.moffice.writer.data.value.CellMargin
use crate::cn::wps::moffice::writer::data::value::CellMargin::CellMargin;
// Java: import cn.wps.moffice.writer.data.value.DefTable
use crate::cn::wps::moffice::writer::data::value::DefTable::empty as DEF_TABLE_EMPTY;
// Java: import cn.wps.moffice.writer.data.value.DefTableBuilder
use crate::cn::wps::moffice::writer::data::value::DefTableBuilder::DefTableBuilder;
// Java: import cn.wps.moffice.writer.data.table.TablePos
use crate::cn::wps::moffice::writer::data::table::TablePos::TablePos;
// Java: import cn.wps.moffice.writer.data.table.TableJCData
use crate::cn::wps::moffice::writer::data::table::TableJCData::TableJCData;
// Java: import cn.wps.moffice.writer.core.table.Tools
use crate::cn::wps::moffice::writer::core::table::Tools::Tools;
// Java: import cn.wps.moffice.writer.data.value.PositionCode
use crate::cn::wps::moffice::writer::data::value::PositionCode::PositionCode;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
// Java: import cn.wps.moffice.writer.data.value.TableBorders
use crate::cn::wps::moffice::writer::data::value::TableBorders::TableBorders;
// Java: import cn.wps.moffice.writer.data.value.TableCell
use crate::cn::wps::moffice::writer::data::value::TableCell::TableCell;
// Java: import cn.wps.moffice.writer.data.value.TableDefSHD
use crate::cn::wps::moffice::writer::data::value::TableDefSHD::TableDefSHD;
// Java: import cn.wps.moffice.writer.data.value.TableDefSHDBuilder
use crate::cn::wps::moffice::writer::data::value::TableDefSHDBuilder::TableDefSHDBuilder;
// Java: import cn.wps.moffice.writer.data.value.TableLength
use crate::cn::wps::moffice::writer::data::value::TableLength::TableLength;
// Java: import cn.wps.moffice.writer.data.value.TableLook
use crate::cn::wps::moffice::writer::data::value::TableLook::TableLook;
// Java: import cn.wps.moffice.writer.data.value.TableSetBRC
use crate::cn::wps::moffice::writer::data::value::TableSetBRC::TableSetBRC;
// Java: import cn.wps.moffice.writer.data.value.TableStyleProperties
use crate::cn::wps::moffice::writer::data::value::TableStyleProperties::TableStyleProperties;

// Java: protected static int[] SHARE_PROPERTYID_FOR_TABLE = new int[]{292, 354, 343, 312, 313, 314, 317, 318, 315, 316};
const SHARE_PROPERTYID_FOR_TABLE: [i32; 10] = [292, 354, 343, 312, 313, 314, 317, 318, 315, 316];
// Java: protected static Object[] DEFAULT_VALUE_FOR_SHARE_PROPERTY = new Object[]{105, 11, false, new PositionCode(0, 0), 0, 0, 0, 0, 0, 0};
const SHARE_INT_DEFAULT: [i32; 10] = [105, 11, 0, 0, 0, 0, 0, 0, 0, 0];
// Java: protected static int[] SHARE_PROPERTYID_FOR_FIRST_CELL_PARA = new int[]{240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250};
const SHARE_PROPERTYID_FOR_FIRST_CELL_PARA: [i32; 11] = [240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250];
// Java: DEFAULT_VALUE_FOR_FIRST_CELL_PARA = new Object[]{new PositionCode(0, 0), 0, 0, 2, 0, paragraph_wHeightAbs, paragraph_frameTextFlow, 180, 0, false, false};
const FIRST_CELL_INT_DEFAULT: [i32; 11] = [0, 0, 0, 2, 0, 0, 0, 180, 0, 0, 0];

// Java: FinalTableCellProperty, the borders and shading accumulated by mergeRowStyle
struct FinalCellProp {
    shd: Option<SHD>,
    brc_left: Option<BRC>,
    brc_right: Option<BRC>,
    brc_top: Option<BRC>,
    brc_bottom: Option<BRC>,
    brc_h: Option<BRC>,
    brc_v: Option<BRC>,
}

impl FinalCellProp {
    fn new() -> Self {
        Self {
            shd: None,
            brc_left: None,
            brc_right: None,
            brc_top: None,
            brc_bottom: None,
            brc_h: None,
            brc_v: None,
        }
    }
}

// Java: Data.CellData written by setCellSHDAndBRC / calcCellStyle / setMargins / calcRowInfo
#[derive(Clone)]
pub struct CellCalc {
    pub shd: Option<SHD>,
    pub brc_top: Option<BRC>,
    pub brc_left: Option<BRC>,
    pub brc_bottom: Option<BRC>,
    pub brc_right: Option<BRC>,
    pub brc_diag_down: Option<BRC>,
    pub brc_diag_up: Option<BRC>,
    pub margin_left: i32,
    pub margin_top: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub table_cell: Option<TableCell>,
    pub width: i32,
    pub auto: bool,
}

impl CellCalc {
    fn cleared() -> Self {
        Self {
            shd: None,
            brc_top: None,
            brc_left: None,
            brc_bottom: None,
            brc_right: None,
            brc_diag_down: None,
            brc_diag_up: None,
            margin_left: 0,
            margin_top: 0,
            margin_right: 0,
            margin_bottom: 0,
            table_cell: None,
            width: 0,
            auto: false,
        }
    }
}

// Java: Data.RowData plus the per-cell results of calcRowStyle
pub struct RowCalc {
    // Java: calcRowStyle returns the merged row property; calcJC reads it for the first row
    pub merged: PropertySet,
    pub rgdxa: Vec<i32>,
    pub widths: Vec<i32>,
    pub width_before: i32,
    pub row_height: i32,
    pub cant_split: bool,
    // Java: rowFinalProperty.getBoolean(320, false) — table header
    pub table_header: bool,
    pub format_change: bool,
    pub revision_delete: bool,
    pub revision_insert: bool,
    pub cells: Vec<CellCalc>,
}

// Java: Data.TableData written by calcTableStyles
pub struct TableStyleCalc {
    pub pos: Option<TablePos>,
    pub shd: Option<SHD>,
    pub style_id: i32,
    pub table_look: TableLook,
    pub borders: Option<TableBorders>,
    pub is_rtl: bool,
}

// Java: public class TableTool
pub struct TableTool;

impl TableTool {
    // Java: public static boolean isInTheSameTable(Document document, int start1, int end1, int start2, int end2)
    pub fn isInTheSameTable(
        document: &dyn Document,
        start1: i32,
        end1: i32,
        start2: i32,
        end2: i32,
    ) -> bool {
        // Java: PropertySet property1 = getFinalRowProperty(document, end1 - 1);
        let mut property1 = Self::getFinalRowProperty(document, end1 - 1);
        // Java: PropertySet property2 = getFinalRowProperty(document, end2 - 1);
        let mut property2 = Self::getFinalRowProperty(document, end2 - 1);
        // Java: boolean allDefault = true;
        let mut all_default = true;
        // Java: for (int i = 0; i < SHARE_PROPERTYID_FOR_TABLE.length; ++i)
        for i in 0..SHARE_PROPERTYID_FOR_TABLE.len() {
            let id = SHARE_PROPERTYID_FOR_TABLE[i];
            let (equal, is_default) = if id == 343 {
                // Java: Object o1 = property1.getObject(id, false); o1.equals(o2)
                let o1 = property1.getBoolean__int_boolean(id, false);
                let o2 = property2.getBoolean__int_boolean(id, false);
                (o1 == o2, o1 == false)
            } else {
                Self::same_value(&property1, &property2, id, SHARE_INT_DEFAULT[i])
            };
            // Java: if (!o1.equals(o2)) return false;
            if !equal {
                return false;
            }
            // Java: if (allDefault) allDefault = o1.equals(DEFAULT_VALUE_FOR_SHARE_PROPERTY[i]);
            if all_default {
                all_default = is_default;
            }
        }
        // Java: if (!allDefault) return true;
        if !all_default {
            return true;
        }
        // Java: property1 = PropertyUtil.getFinalParagraphFormat(document.getPapxTable().seek(start1).getProperty());
        property1 = PropertyUtil::getFinalParagraphFormat(&Self::paragraph_at(document, start1));
        // Java: property2 = PropertyUtil.getFinalParagraphFormat(document.getPapxTable().seek(start2).getProperty());
        property2 = PropertyUtil::getFinalParagraphFormat(&Self::paragraph_at(document, start2));
        // Java: for (int i = 0; i < SHARE_PROPERTYID_FOR_FIRST_CELL_PARA.length; ++i)
        for i in 0..SHARE_PROPERTYID_FOR_FIRST_CELL_PARA.len() {
            let id = SHARE_PROPERTYID_FOR_FIRST_CELL_PARA[i];
            let (equal, _is_default) = if id == 249 || id == 250 {
                let o1 = property1.getBoolean__int_boolean(id, false);
                let o2 = property2.getBoolean__int_boolean(id, false);
                (o1 == o2, o1 == false)
            } else {
                Self::same_value(&property1, &property2, id, FIRST_CELL_INT_DEFAULT[i])
            };
            // Java: if (!o1.equals(o2)) return false;
            if !equal {
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: public static int getResidual(int cellWidth)
    pub fn getResidual(cell_width: i32) -> i32 {
        // Java: return cellWidth > 200 ? 40 : (int)Math.min((float)cellWidth / 5.0F, 6.0F);
        if cell_width > 200 {
            40
        } else {
            ((cell_width as f32) / 5.0).min(6.0) as i32
        }
    }

    // Java: public static int getFixedTableWidth(TableInfo tableInfo)
    pub fn getFixedTableWidth(prop: &PropertySet) -> i32 {
        // Java: if (finalRowProperty.contains(295)) { type == 3 return value; }
        if prop.contains(295) {
            if let Some(length) = Self::table_length(prop, 295) {
                if length.getType() == 3 {
                    return length.getValue();
                }
            }
        }
        // Java: return 0;
        0
    }

    // Java: public static boolean isTableWithPercent(TableInfo tableInfo)
    pub fn isTableWithPercent(prop: &PropertySet) -> bool {
        // Java: if (finalRowProperty.contains(295)) return tableLength.getType() == 2;
        if !prop.contains(295) {
            return false;
        }
        prop.getValue(295)
            .and_then(|value| {
                value
                    .as_any()
                    .downcast_ref::<crate::cn::wps::moffice::writer::data::value::TableLength::TableLength>()
            })
            .is_some_and(|length| length.getType() == 2)
    }

    // Java: static PropertySet getFinalRowProperty(Document document, int cp)
    pub fn getFinalRowProperty(document: &dyn Document, cp: i32) -> PropertySet {
        // Java: PropertySet prop = document.getPapxTable().seek(cp).getProperty();
        let prop = Self::paragraph_at(document, cp);
        // Java: Object value = prop.getValue(363);
        let value = prop.getValue(363);
        // Java: if (null != value)
        if value.is_some() {
            // Java: KStyles styles = document.getTextDocument().getStyles();
            // Java: return PropertyUtil.getFinalTableFormat(styles, prop);
            if let Some(styles) = document.getStyles() {
                let styles: &KStyles = unsafe { &*styles };
                return PropertyUtil::getFinalTableFormat(styles, &prop);
            }
        }
        // Java: return prop;
        prop
    }

    fn paragraph_at(document: &dyn Document, cp: i32) -> PropertySet {
        let Some(papx) = document.getPapxTable() else {
            return PropertySet::new();
        };
        let Some(para) = IPapxTable::seek(unsafe { &*papx }, cp) else {
            return PropertySet::new();
        };
        PapxEntry::getProperty(para.as_ref())
            .cloned()
            .unwrap_or_else(PropertySet::new)
    }

    /// Java `getObject(id, default).equals`. 缺省按默认值。312 / 240 的 `PositionCode` 当前导入不写入，缺省即 `(0, 0)`。
    fn same_value(a: &PropertySet, b: &PropertySet, id: i32, default_int: i32) -> (bool, bool) {
        let left = a.getInt__int_int(id, default_int);
        let right = b.getInt__int_int(id, default_int);
        (left == right, left == default_int)
    }

    // Java: public PropertySet calcRowStyle(RowInfo row)
    pub fn calcRowStyle(
        document: &dyn Document,
        row_index: i32,
        row_count: i32,
        table_complete: bool,
        row_end_cp: i32,
        cell_count: i32,
        base_borders: Option<&TableBorders>,
    ) -> RowCalc {
        // Java: PropertySet finalRowProperty = row.getFinalProperty();
        let final_row_property = Self::getFinalRowProperty(document, row_end_cp - 1);
        // Java: PropertySet baseProp = this.getBaseTableProperty(finalRowProperty);
        let base_prop = Self::getBaseTableProperty(document, &final_row_property);
        // Java: rowData.clear(); each cellData.clear();
        let mut cells = Vec::new();
        let mut i = 0;
        while i < cell_count {
            cells.push(CellCalc::cleared());
            i += 1;
        }
        // Java: MutablePropertySet finalRowStyle = new MutablePropertySet();
        let mut final_row_style = MutablePropertySet::new();
        // Java: FinalTableCellProperty finalTableCellProp = new FinalTableCellProperty();
        let mut cell_prop = FinalCellProp::new();
        // Java: TableLook tableLook = (TableLook)row.getFinalProperty().getObject(342, DefaultValue.table_look);
        let table_look = Self::table_look(&final_row_property);
        // Java: TableBorders tableBorders = row.getParent().getTableBorders();
        if let Some(base) = base_prop.as_ref() {
            // Java: this.mergeRowStyle(baseProp, ...)
            Self::mergeRowStyle(document, &mut final_row_style, base, &mut cell_prop, base_borders);
            // Java: TableStyleProperties wholeTableProp = (TableStyleProperties)baseProp.getValue(329);
            if let Some(whole) = Self::style_props(base, 329) {
                Self::mergeRowStyle(document, &mut final_row_style, &whole, &mut cell_prop, base_borders);
            }
            // Java: if (!tableLook.getNoHBand())
            if !table_look.getNoHBand() {
                // Java: oddRow = firstRow && base.contains(330) ? rowIndex % 2 != 0 : rowIndex % 2 == 0
                let odd_row = if table_look.getFirstRow() && base.contains(330) {
                    row_index % 2 != 0
                } else {
                    row_index % 2 == 0
                };
                let band_id = if odd_row { 332 } else { 333 };
                if let Some(band) = Self::style_props(base, band_id) {
                    Self::mergeRowStyle(document, &mut final_row_style, &band, &mut cell_prop, base_borders);
                }
            }
        }
        // Java: boolean firstRow = 0 == rowIndex;
        let first_row = row_index == 0;
        // Java: boolean lastRow = TableInfoTool.isLastRow(row);
        let last_row = table_complete && row_count > 0 && row_index == row_count - 1;
        // Java: boolean useFirstRow = tableLook.getFirstRow() && firstRow;
        let use_first_row = table_look.getFirstRow() && first_row;
        // Java: boolean useLastRow = tableLook.getLastRow() && lastRow;
        let use_last_row = table_look.getLastRow() && last_row;
        if let Some(base) = base_prop.as_ref() {
            if use_first_row {
                if let Some(first) = Self::style_props(base, 330) {
                    Self::mergeRowStyle(document, &mut final_row_style, &first, &mut cell_prop, base_borders);
                }
            }
            if use_last_row {
                if let Some(last) = Self::style_props(base, 331) {
                    Self::mergeRowStyle(document, &mut final_row_style, &last, &mut cell_prop, base_borders);
                }
            }
        }
        // Java: this.mergeRowStyle(finalRowProperty, ...)
        Self::mergeRowStyle(
            document,
            &mut final_row_style,
            &final_row_property,
            &mut cell_prop,
            base_borders,
        );
        // Java: setCellSHDAndBRC(row, finalRowStyle, finalTableCellProp, lastRow, firstRow);
        Self::setCellSHDAndBRC(&mut cells, final_row_style.get(), &cell_prop, last_row, first_row);
        if let Some(base) = base_prop.as_ref() {
            // Java: if (!tableLook.getNoVBand())
            if !table_look.getNoVBand() {
                let mut cell_index = 0;
                while cell_index < cell_count {
                    // Java: oddColumn = firstCol && base.contains(334) ? cellIndex % 2 != 0 : cellIndex % 2 == 0
                    let odd_column = if table_look.getFirstCol() && base.contains(334) {
                        cell_index % 2 != 0
                    } else {
                        cell_index % 2 == 0
                    };
                    let band_id = if odd_column { 336 } else { 337 };
                    if let Some(band) = Self::style_props(base, band_id) {
                        Self::calcCellStyle(&mut cells[cell_index as usize], &band);
                    }
                    cell_index += 1;
                }
            }
        }
        if let Some(base) = base_prop.as_ref() {
            // Java: if (!useFirstRow && !useLastRow)
            if !use_first_row && !use_last_row && cell_count > 0 {
                if table_look.getLastCol() {
                    if let Some(last_col) = Self::style_props(base, 335) {
                        Self::calcCellStyle(&mut cells[(cell_count - 1) as usize], &last_col);
                    }
                }
                if table_look.getFirstCol() {
                    if let Some(first_col) = Self::style_props(base, 334) {
                        Self::calcCellStyle(&mut cells[0], &first_col);
                    }
                }
            }
        }
        if let Some(base) = base_prop.as_ref() {
            // Java: last-row style is merged again and can override the row DefTable
            if table_look.getLastRow() && last_row && base.contains(331) {
                if let Some(last) = Self::style_props(base, 331) {
                    Self::mergeRowStyle(document, &mut final_row_style, &last, &mut cell_prop, base_borders);
                    Self::setCellSHDAndBRC(&mut cells, final_row_style.get(), &cell_prop, last_row, first_row);
                }
            }
            // Java: first-row style is merged again and can override the row DefTable
            if table_look.getFirstRow() && first_row && base.contains(330) {
                if let Some(first) = Self::style_props(base, 330) {
                    Self::mergeRowStyle(document, &mut final_row_style, &first, &mut cell_prop, base_borders);
                    Self::setCellSHDAndBRC(&mut cells, final_row_style.get(), &cell_prop, last_row, first_row);
                }
            }
        }
        if let Some(base) = base_prop.as_ref() {
            if use_last_row && cell_count > 0 {
                if let Some(se) = Self::style_props(base, 340) {
                    Self::calcCellStyle(&mut cells[(cell_count - 1) as usize], &se);
                }
                if let Some(sw) = Self::style_props(base, 341) {
                    Self::calcCellStyle(&mut cells[0], &sw);
                }
            }
            if use_first_row && cell_count > 0 {
                if let Some(ne) = Self::style_props(base, 338) {
                    Self::calcCellStyle(&mut cells[(cell_count - 1) as usize], &ne);
                }
                if let Some(nw) = Self::style_props(base, 339) {
                    Self::calcCellStyle(&mut cells[0], &nw);
                }
            }
        }
        // Java: this.calcRowInfo(row, finalRowStyle.get()); return finalRowStyle.freeze();
        let merged = final_row_style.freeze();
        Self::calcRowInfo(&mut cells, &merged);
        let merged_for_jc = merged.clone();
        // Java: if (row.getRowProperty().contains(363)) rowData.mIsFormatChange = true;
        let original = Self::paragraph_at(document, row_end_cp - 1);
        let format_change = original.contains(363);
        // Java: IChpxTable.Entry run = document.getChpxTable().seek(row.getEndCP() - 1);
        let (revision_delete, revision_insert) = Self::row_revision(document, row_end_cp - 1);
        RowCalc {
            merged: merged_for_jc,
            rgdxa: Self::rgdxa_of(&merged),
            widths: Vec::new(),
            width_before: Self::width_before_of(&merged),
            row_height: if merged.contains(305) {
                merged.getInt__int_int(305, 0)
            } else {
                0
            },
            cant_split: merged.getBoolean__int_boolean(304, false),
            table_header: merged.getBoolean__int_boolean(320, false),
            format_change,
            revision_delete,
            revision_insert,
            cells,
        }
    }

    // Java: PropertySet getBaseTableProperty(PropertySet finalTableProperty)
    fn getBaseTableProperty(document: &dyn Document, final_table_property: &PropertySet) -> Option<PropertySet> {
        // Java: int styleId = finalTableProperty.getInt(292, 4095);
        let style_id = final_table_property.getInt__int_int(292, 4095);
        // Java: if (4095 != styleId) return mPropertyTool.getMergedTableProperty(styleId);
        if style_id == 4095 {
            return None;
        }
        // Java: KStyle style = styleSheet.getStyleById(styleId); return style.getFinalProperty();
        Self::para_property_set(document, style_id)
    }

    // Java: public void calcTableStyles(TableInfo tableInfoImpl)
    pub fn calcTableStyles(document: &dyn Document, row_end_cp: i32) -> TableStyleCalc {
        // Java: PropertySet tableProperty = tableInfoImpl.getTableProperty();
        // Java: getTableProperty() is the first row's getFinalProperty()
        let table_property = Self::getFinalRowProperty(document, row_end_cp - 1);
        // Java: tableProperty = Tools.getFinalProperty(tableProperty);
        let table_property = Self::getFinalProperty(&table_property);
        // Java: tableData.mPos = this.getTablePos(tableProperty);
        let pos = Self::getTablePos(&table_property);
        // Java: PropertySet baseTableProperty = this.getBaseTableProperty(tableProperty);
        let base = Self::getBaseTableProperty(document, &table_property);
        let mut shd = None;
        if let Some(base) = base.as_ref() {
            // Java: if (baseTableProperty != null && baseTableProperty.contains(309))
            if base.contains(309) {
                shd = base.getValue(309).and_then(|value| value.as_any().downcast_ref::<SHD>().cloned());
            }
        }
        // Java: if (tableProperty.contains(309)) tableData.mShd = (SHD)tableProperty.getObject(309, null);
        if table_property.contains(309) {
            shd = table_property
                .getValue(309)
                .and_then(|value| value.as_any().downcast_ref::<SHD>().cloned());
        }
        TableStyleCalc {
            pos,
            shd,
            // Java: tableData.mStyleId = tableProperty.getInt(292, 4095);
            style_id: table_property.getInt__int_int(292, 4095),
            // Java: tableData.mTableLook = (TableLook)tableProperty.getObject(342, DefaultValue.table_look);
            table_look: Self::table_look(&table_property),
            // Java: tableData.mTableBorders = (TableBorders)tableProperty.getObject(310, null);
            borders: table_property
                .getValue(310)
                .and_then(|value| value.as_any().downcast_ref::<TableBorders>().cloned()),
            // Java: tableData.mIsRTL = tableProperty.getBoolean(343, false);
            is_rtl: table_property.getBoolean__int_boolean(343, false),
        }
    }

    // Java: public static PropertySet Tools.getFinalProperty(PropertySet originalRowProp)
    fn getFinalProperty(original_row_prop: &PropertySet) -> PropertySet {
        // Java: Revision revision = (Revision)originalRowProp.getValue(363);
        let Some(revision) = original_row_prop
            .getValue(363)
            .and_then(|value| value.as_any().downcast_ref::<Revision>())
        else {
            return original_row_prop.clone();
        };
        // Java: return PropertySet.merge(originalRowProp, revision.getProperty());
        match revision.getProperty() {
            Some(revision_prop) => PropertySet::merge(original_row_prop, &revision_prop),
            None => original_row_prop.clone(),
        }
    }

    // Java: IPos getTablePos(PropertySet property)
    fn getTablePos(property: &PropertySet) -> Option<TablePos> {
        // Java: if (!contains(313) && !contains(314) && !contains(312)) return null;
        if !property.contains(313) && !property.contains(314) && !property.contains(312) {
            return None;
        }
        let dxa_abs = property.getInt__int_int(313, 0);
        let dya_abs = property.getInt__int_int(314, 0);
        // Java: PositionCode pc = (PositionCode)property.getObject(312, new PositionCode(0, 0));
        let pc = property
            .getValue(312)
            .and_then(|value| value.as_ref().as_any().downcast_ref::<PositionCode>().cloned())
            .unwrap_or_else(|| PositionCode::new(0, 0));
        // Java: if (pc.isNotWrap() && dxaAbs == 0 && dyaAbs == 0 && !property.getBoolean(319, false)) return null;
        if pc.isNotWrap() && dxa_abs == 0 && dya_abs == 0 && !property.getBoolean__int_boolean(319, false) {
            return None;
        }
        let mut pos = TablePos::default();
        // Java: pos.setAllowOverlap(!property.getBoolean(319, false));
        IPos::setAllowOverlap(&mut pos, !property.getBoolean__int_boolean(319, false));
        IPos::setWrapDistLeft(&mut pos, UnitUtil::TWIPS2PT(property.getInt__int_int(315, 0)));
        IPos::setWrapDistRight(&mut pos, UnitUtil::TWIPS2PT(property.getInt__int_int(316, 0)));
        IPos::setWrapDistTop(&mut pos, UnitUtil::TWIPS2PT(property.getInt__int_int(317, 0)));
        IPos::setWrapDistBottom(&mut pos, UnitUtil::TWIPS2PT(property.getInt__int_int(318, 0)));
        let is_rtl = property.getBoolean__int_boolean(343, false);
        match dxa_abs {
            -16 => IPos::setAlignmentH(&mut pos, 5),
            -12 => IPos::setAlignmentH(&mut pos, 4),
            -8 => IPos::setAlignmentH(&mut pos, if is_rtl { 1 } else { 3 }),
            -4 => IPos::setAlignmentH(&mut pos, 2),
            0 => IPos::setAlignmentH(&mut pos, if is_rtl { 3 } else { 1 }),
            _ => {
                IPos::setAlignmentH(&mut pos, 0);
                // Java: RectF rt = pos.getShapeRect(); rt.left = (float)MetricUtility.Twips2Points(dxaAbs - 1);
                let mut rt = IPos::getShapeRect(&pos);
                rt.left = MetricUtility::Twips2Points_i32(dxa_abs - 1) as f32;
                IPos::setShapeRect(&mut pos, rt);
            }
        }
        match dya_abs {
            -20 => IPos::setAlignmentV(&mut pos, 5),
            -16 => IPos::setAlignmentV(&mut pos, 4),
            -12 => IPos::setAlignmentV(&mut pos, 3),
            -8 => IPos::setAlignmentV(&mut pos, 2),
            -4 => IPos::setAlignmentV(&mut pos, 1),
            _ => {
                // Java: case 0 and default both set alignment 0 and the top offset
                IPos::setAlignmentV(&mut pos, 0);
                let mut rt = IPos::getShapeRect(&pos);
                rt.top = MetricUtility::Twips2Points_i32(dya_abs - 1) as f32;
                IPos::setShapeRect(&mut pos, rt);
            }
        }
        IPos::setRelativeH(&mut pos, 2);
        IPos::setRelativeV(&mut pos, 0);
        if property.contains(312) {
            match pc.getVertPos() {
                0 => IPos::setRelativeV(&mut pos, 0),
                1 => IPos::setRelativeV(&mut pos, 1),
                2 => IPos::setRelativeV(&mut pos, 2),
                _ => IPos::setRelativeV(&mut pos, 0),
            }
            match pc.getHorzPos() {
                0 => IPos::setRelativeH(&mut pos, 2),
                1 => IPos::setRelativeH(&mut pos, 0),
                2 => IPos::setRelativeH(&mut pos, 1),
                _ => IPos::setRelativeH(&mut pos, 2),
            }
        }
        Some(pos)
    }

    // Java: void mergeRowStyle(PropertySet property, MutablePropertySet finalRowStyle, ...)
    fn mergeRowStyle(
        document: &dyn Document,
        dest: &mut MutablePropertySet,
        property: &PropertySet,
        cell_prop: &mut FinalCellProp,
        base_borders: Option<&TableBorders>,
    ) {
        // Java: mPropertyTool.getMergedTableProperty(dest, property, true)
        // Java: getMergedParaProperty then, if dest contains 363, addLimit(revision, 291, 376)
        let style_name = property.getInt__int_int(191, 0);
        if let Some(base_para) = Self::para_property_set(document, style_name) {
            dest.addLimit(&base_para, 191, 376);
        }
        dest.addLimit(property, 191, 376);
        // Java: Revision revision = (Revision)paraProperty.getValue(237); dest.add(revisionProp);
        if let Some(revision) = property.getValue(237).and_then(|value| value.as_any().downcast_ref::<Revision>()) {
            if let Some(revision_prop) = revision.getProperty() {
                dest.add(&revision_prop);
            }
        }
        if dest.contains(363) {
            if let Some(revision) = dest.getValue(363).and_then(|value| value.as_any().downcast_ref::<Revision>()) {
                if let Some(revision_prop) = revision.getProperty() {
                    dest.addLimit(&revision_prop, 291, 376);
                }
            }
        }
        // Java: SHD shd = (SHD)finalRowStyle.getValue(345);
        if let Some(shd) = dest.getValue(345).and_then(|value| value.as_any().downcast_ref::<SHD>()) {
            cell_prop.shd = Some(shd.clone());
        }
        // Java: TableBorders borders = (TableBorders)finalRowStyle.getValue(310);
        if let Some(borders) = dest.getValue(310).and_then(|value| value.as_any().downcast_ref::<TableBorders>()) {
            if let Some(brc) = borders.getBorder(1) {
                cell_prop.brc_left = Some(brc);
            }
            if let Some(brc) = borders.getBorder(3) {
                cell_prop.brc_right = Some(brc);
            }
            if let Some(brc) = borders.getBorder(0) {
                cell_prop.brc_top = Some(brc);
            }
            if let Some(brc) = borders.getBorder(2) {
                cell_prop.brc_bottom = Some(brc);
            }
            if let Some(brc) = borders.getBorder(4) {
                cell_prop.brc_h = Some(brc);
            }
            if let Some(brc) = borders.getBorder(5) {
                cell_prop.brc_v = Some(brc);
            }
        }
        // Java: missing cell borders fall back to the table borders from calcTableStyles
        if let Some(base_borders) = base_borders {
            if cell_prop.brc_left.is_none() {
                cell_prop.brc_left = base_borders.getBorder(1);
            }
            if cell_prop.brc_right.is_none() {
                cell_prop.brc_right = base_borders.getBorder(3);
            }
            if cell_prop.brc_top.is_none() {
                cell_prop.brc_top = base_borders.getBorder(0);
            }
            if cell_prop.brc_bottom.is_none() {
                cell_prop.brc_bottom = base_borders.getBorder(2);
            }
            if cell_prop.brc_h.is_none() {
                cell_prop.brc_h = base_borders.getBorder(4);
            }
            if cell_prop.brc_v.is_none() {
                cell_prop.brc_v = base_borders.getBorder(5);
            }
        }
    }

    // Java: static void setCellSHDAndBRC(...)
    fn setCellSHDAndBRC(
        cells: &mut [CellCalc],
        style: &PropertySet,
        prop: &FinalCellProp,
        is_last_row: bool,
        is_first_row: bool,
    ) {
        let brc_top_style = Self::brc_at(style, 346);
        let brc_bottom_style = Self::brc_at(style, 348);
        let brc_left_style = Self::brc_at(style, 347);
        let brc_right_style = Self::brc_at(style, 349);
        let brc_h_style = Self::brc_at(style, 350);
        let brc_v_style = Self::brc_at(style, 351);
        let shd = prop.shd.clone();
        let brc_top = prop.brc_top.clone();
        let brc_bottom = prop.brc_bottom.clone();
        let mut brc_left = prop.brc_left.clone();
        if brc_left_style.is_some() {
            brc_left = brc_left_style.clone();
        }
        let mut brc_right = prop.brc_right.clone();
        if brc_right_style.is_some() {
            brc_right = brc_right_style.clone();
        }
        let mut brc_h = prop.brc_h.clone();
        if brc_h_style.is_some() {
            brc_h = brc_h_style.clone();
        }
        let mut brc_v = prop.brc_v.clone();
        if brc_v_style.is_some() {
            brc_v = brc_v_style.clone();
        }
        let size = cells.len();
        for (i, cell) in cells.iter_mut().enumerate() {
            if let Some(shd) = shd.clone() {
                cell.shd = Some(shd);
            }
            if let Some(brc) = brc_h.clone() {
                cell.brc_top = Some(brc.clone());
                cell.brc_bottom = Some(brc);
            }
            if let Some(brc) = brc_v.clone() {
                cell.brc_left = Some(brc.clone());
                cell.brc_right = Some(brc);
            }
            if i == 0 {
                cell.brc_left = brc_left.clone();
            }
            if i + 1 == size {
                cell.brc_right = brc_right.clone();
            }
            if is_first_row {
                cell.brc_top = brc_top.clone();
            }
            if let Some(brc) = brc_top_style.clone() {
                cell.brc_top = Some(brc);
            }
            if is_last_row {
                cell.brc_bottom = brc_bottom.clone();
            }
            if let Some(brc) = brc_bottom_style.clone() {
                cell.brc_bottom = Some(brc);
            }
        }
    }

    // Java: static void calcCellStyle(CellInfo cell, PropertySet property)
    fn calcCellStyle(cell: &mut CellCalc, property: &PropertySet) {
        if let Some(shd) = property.getValue(345).and_then(|value| value.as_any().downcast_ref::<SHD>()) {
            cell.shd = Some(shd.clone());
        }
        let brc_top = Self::brc_at(property, 346);
        let brc_bottom = Self::brc_at(property, 348);
        let brc_left = Self::brc_at(property, 347);
        let brc_right = Self::brc_at(property, 349);
        let brc_h = Self::brc_at(property, 350);
        let brc_v = Self::brc_at(property, 351);
        if let Some(brc) = brc_h {
            cell.brc_top = Some(brc.clone());
            cell.brc_bottom = Some(brc);
        }
        if let Some(brc) = brc_v {
            cell.brc_left = Some(brc.clone());
            cell.brc_right = Some(brc);
        }
        if let Some(brc) = brc_top {
            cell.brc_top = Some(brc);
        }
        if let Some(brc) = brc_bottom {
            cell.brc_bottom = Some(brc);
        }
        if let Some(brc) = brc_left {
            cell.brc_left = Some(brc);
        }
        if let Some(brc) = brc_right {
            cell.brc_right = Some(brc);
        }
    }

    // Java: void calcRowInfo(RowInfo row, PropertySet finalRowProperty)
    fn calcRowInfo(cells: &mut [CellCalc], prop: &PropertySet) {
        let def = Self::def_table(prop);
        let row_size = cells.len() as i32;
        // Java: if (finalRowProperty.contains(308)) { revision gate; then TableDefSHD }
        if prop.contains(308) {
            let apply_shd = match prop.getValue(363).and_then(|value| value.as_any().downcast_ref::<Revision>()) {
                None => true,
                Some(revision) => match revision.getProperty() {
                    None => false,
                    Some(revision_prop) => !revision_prop.contains(292) || revision_prop.contains(308),
                },
            };
            if apply_shd {
                if let Some(def_shd) = Self::def_shd(prop) {
                    let size = row_size.min(def_shd.getCount());
                    let mut i = 0;
                    while i < size {
                        if let Some(shd) = def_shd.getCellSHD(i) {
                            // Java: if (shd != null && shd.getPattern() != 65535)
                            if shd.getPattern() != 65535 {
                                cells[i as usize].shd = Some(shd);
                            }
                        }
                        i += 1;
                    }
                }
            }
        }
        let color_left = Self::int_array(prop, 324);
        let color_top = Self::int_array(prop, 323);
        let color_right = Self::int_array(prop, 326);
        let color_bottom = Self::int_array(prop, 325);
        let mut i = 0;
        while i < row_size {
            let table_cell = def.getTableCell(i).cloned();
            if let Some(table_cell) = table_cell.as_ref() {
                // Java: calcCellStyle(cell, tableCell, i, colorRef...)
                Self::calcCellStyleFromTableCell(
                    &mut cells[i as usize],
                    table_cell,
                    i,
                    color_left.as_deref(),
                    color_top.as_deref(),
                    color_right.as_deref(),
                    color_bottom.as_deref(),
                );
            }
            cells[i as usize].table_cell = table_cell.clone();
            // Java: isAutoWidth reads TableCell.getWidthType() == 1. Widths are calcCellWidths.
            let width_type = table_cell.as_ref().map(|cell| cell.getWidthType()).unwrap_or(0);
            cells[i as usize].auto = width_type == 1;
            // Java: setMargins(cell, i, finalRowProperty);
            Self::setMargins(&mut cells[i as usize], i, prop);
            i += 1;
        }
        // Java: TableSetBRC[] tableSetBRCs = (TableSetBRC[])finalRowProperty.getValue(311);
        if let Some(table_set_brcs) = prop
            .getValue(311)
            .and_then(|value| value.as_any().downcast_ref::<Vec<TableSetBRC>>().cloned())
        {
            for table_set_brc in table_set_brcs {
                let borders_to_apply = table_set_brc.getBordersToApply();
                // Java: isDiagonalBRC: bordersToApply == 16 || bordersToApply == 32
                if borders_to_apply == 16 || borders_to_apply == 32 {
                    let start = table_set_brc.getFirst();
                    let end = table_set_brc.getLast().min(row_size);
                    let diagonal = Self::normalizeDiagonalCellBRC(table_set_brc.getBRC());
                    let mut j = start;
                    while j < end {
                        if j >= 0 {
                            if borders_to_apply == 16 {
                                cells[j as usize].brc_diag_down = diagonal.clone();
                            } else {
                                cells[j as usize].brc_diag_up = diagonal.clone();
                            }
                        }
                        j += 1;
                    }
                }
            }
        }
    }

    // Java: static void calcCellStyle(CellInfo, TableCell, int, int[], int[], int[], int[])
    fn calcCellStyleFromTableCell(
        cell: &mut CellCalc,
        table_cell: &TableCell,
        cell_index: i32,
        color_left: Option<&[i32]>,
        color_top: Option<&[i32]>,
        color_right: Option<&[i32]>,
        color_bottom: Option<&[i32]>,
    ) {
        if let Some(brc) = table_cell.getBrcTop() {
            if Self::isValidBRC(Some(&brc)) {
                cell.brc_top = Self::correctBRCColorInDocFormat(Some(brc), cell_index, color_top);
            }
        }
        if let Some(brc) = table_cell.getBrcLeft() {
            if Self::isValidBRC(Some(&brc)) {
                cell.brc_left = Self::correctBRCColorInDocFormat(Some(brc), cell_index, color_left);
            }
        }
        if let Some(brc) = table_cell.getBrcBottom() {
            if Self::isValidBRC(Some(&brc)) {
                cell.brc_bottom = Self::correctBRCColorInDocFormat(Some(brc), cell_index, color_bottom);
            }
        }
        if let Some(brc) = table_cell.getBrcRight() {
            if Self::isValidBRC(Some(&brc)) {
                cell.brc_right = Self::correctBRCColorInDocFormat(Some(brc), cell_index, color_right);
            }
        }
    }

    // Java: public static BRC correctBRCColorInDocFormat(BRC brc, int cellIdx, int[] colorRef)
    fn correctBRCColorInDocFormat(brc: Option<BRC>, cell_index: i32, color_ref: Option<&[i32]>) -> Option<BRC> {
        match (brc, color_ref) {
            (Some(brc), Some(colors)) if cell_index >= 0 && (cell_index as usize) < colors.len() => {
                Some(BRC::obtain__brc_int(&brc, colors[cell_index as usize]))
            }
            (brc, _) => brc,
        }
    }

    // Java: public static boolean isValidBRC(BRC brc)
    fn isValidBRC(brc: Option<&BRC>) -> bool {
        brc.is_some_and(|brc| brc.getBorderType() != 0)
    }

    // Java: static void setMargins(CellInfo cell, int cellIndex, PropertySet finalRowProperty)
    fn setMargins(cell: &mut CellCalc, cell_index: i32, prop: &PropertySet) {
        cell.margin_left = 108;
        cell.margin_top = 0;
        cell.margin_right = 108;
        cell.margin_bottom = 0;
        if let Some(margins) = Self::cell_margins(prop, 301) {
            for margin in margins {
                Self::calcMargin(cell, &margin);
            }
        }
        if let Some(margins) = Self::cell_margins(prop, 300) {
            for margin in margins {
                if cell_index >= margin.getFirst() && cell_index < margin.getBeyond() {
                    Self::calcMargin(cell, &margin);
                }
            }
        }
    }

    // Java: static void calcMargin(CellInfo cell, CellMargin cellMargin)
    fn calcMargin(cell: &mut CellCalc, cell_margin: &CellMargin) {
        let table_length = cell_margin.getCellSpacingValue();
        if !table_length.isValid() {
            return;
        }
        // Java: types 0, 1, 2 and 3 all take tableLength.getValue()
        let value = table_length.getValue();
        let grbit = cell_margin.getGrbitChg();
        if grbit & 1 != 0 {
            cell.margin_top = value;
        }
        if grbit & 2 != 0 {
            cell.margin_left = value;
        }
        if grbit & 4 != 0 {
            cell.margin_bottom = value;
        }
        if grbit & 8 != 0 {
            cell.margin_right = value;
        }
    }

    // Java: private static BRC normalizeDiagonalCellBRC(BRC brc)
    fn normalizeDiagonalCellBRC(brc: Option<BRC>) -> Option<BRC> {
        let brc = brc?;
        match brc.getBorderType() {
            3 => {
                if brc.getColor() != -16777216 {
                    Some(BRC::obtain__brc_float_int(&brc, brc.getLineWidth() * 3.0, 1))
                } else {
                    Some(brc)
                }
            }
            6 | 7 | 8 | 9 | 22 | 23 => Some(BRC::obtain__brc_float_int(&brc, 0.5, brc.getBorderType())),
            _ => Some(brc),
        }
    }

    fn para_property_set(document: &dyn Document, style_id: i32) -> Option<PropertySet> {
        let styles = document.getStyles()?;
        let styles = unsafe { &mut *styles };
        let mut style = styles.getStyleById__int(style_id)?;
        style.getFinalProperty()
    }

    fn style_props(base: &PropertySet, id: i32) -> Option<PropertySet> {
        base.getValue(id)
            .and_then(|value| value.as_any().downcast_ref::<TableStyleProperties>())
            .and_then(|props| props.getProperty())
    }

    fn table_look(prop: &PropertySet) -> TableLook {
        if let Some(look) = prop.getValue(342).and_then(|value| value.as_any().downcast_ref::<TableLook>()) {
            return look.clone();
        }
        crate::cn::wps::moffice::writer::data::DefaultValue::table_look
            .as_ref()
            .as_ref()
            .cloned()
            .unwrap_or_else(TableLook::new)
    }

    fn def_table(prop: &PropertySet) -> DefTableBuilder {
        prop.getValue(306)
            .and_then(|value| value.as_any().downcast_ref::<DefTableBuilder>().cloned())
            .unwrap_or_else(|| (*DEF_TABLE_EMPTY).clone())
    }

    fn rgdxa_of(prop: &PropertySet) -> Vec<i32> {
        let def = Self::def_table(prop);
        let count = def.getItMax();
        let mut rgdxa = Vec::new();
        if count > 0 {
            let mut i = 0;
            while i < count {
                rgdxa.push(def.getRgdxa(i));
                i += 1;
            }
        }
        rgdxa
    }

    fn width_before_of(prop: &PropertySet) -> i32 {
        if !prop.contains(296) {
            return 0;
        }
        prop.getValue(296)
            .and_then(|value| value.as_any().downcast_ref::<TableLength>())
            .map(|length| length.getValue())
            .unwrap_or(0)
    }

    fn def_shd(prop: &PropertySet) -> Option<TableDefSHD> {
        let value = prop.getValue(308)?;
        if let Some(builder) = value.as_any().downcast_ref::<TableDefSHDBuilder>() {
            return Some((&**builder).clone());
        }
        value.as_any().downcast_ref::<TableDefSHD>().cloned()
    }

    fn brc_at(prop: &PropertySet, id: i32) -> Option<BRC> {
        prop.getValue(id).and_then(|value| value.as_any().downcast_ref::<BRC>().cloned())
    }

    fn cell_margins(prop: &PropertySet, id: i32) -> Option<Vec<CellMargin>> {
        prop.getValue(id)
            .and_then(|value| value.as_any().downcast_ref::<Vec<CellMargin>>().cloned())
    }

    fn int_array(prop: &PropertySet, id: i32) -> Option<Vec<i32>> {
        prop.getValue(id).and_then(|value| value.as_any().downcast_ref::<Vec<i32>>().cloned())
    }

    fn row_revision(document: &dyn Document, cp: i32) -> (bool, bool) {
        let Some(chpx) = document.getChpxTable() else {
            return (false, false);
        };
        let Some(run) = IChpxTable::seek(unsafe { &*chpx }, cp) else {
            return (false, false);
        };
        if ITableEntry::isEnd(run.as_ref()) {
            return (false, false);
        }
        let Some(prop) = ITableEntry::getProperty(run.as_ref()) else {
            return (false, false);
        };
        if prop.contains(45) {
            (true, false)
        } else if prop.contains(46) {
            (false, true)
        } else {
            (false, false)
        }
    }

    // Java: public TableJCData calcJC(TableInfo table)
    pub fn calcJC(document: &dyn Document, first_merged: &PropertySet, rows: &[RowJcInput]) -> TableJCData {
        let Some(first) = rows.first() else {
            return TableJCData::default();
        };
        // java boolean alignTableRowByRow = compatibility.isBefore2013Mode() && property.getBoolean(389, false);
        let align_row_by_row = Tools::compatibility_mode(document).isBefore2013Mode()
            && Tools::document_property_boolean(document, 389, false);
        let mut table_before = 0;
        // java if (alignTableRowByRow && finalRowProperty.contains(296))
        if align_row_by_row && first_merged.contains(296) {
            table_before = Self::table_length_value(first_merged, 296);
        }
        // java int tableJC = finalRowProperty.getInt(303, 0);
        let table_jc = first_merged.getInt__int_int(303, 0);
        // java TableLength tableLength = (TableLength)finalRowProperty.getValue(298);
        let table_indent = Self::table_length_value(first_merged, 298);
        // java new TableJCData(tableJC, firstRow.getRgdxa(0) - talbeBefore, firstRow.getRgdxa(firstRow.size()) - talbeBefore, tableIndent)
        let mut jc_data = TableJCData::TableJCData(
            table_jc,
            first.rgdxa_left - table_before,
            first.rgdxa_right - table_before,
            table_indent,
        );
        for row in rows {
            table_before = 0;
            if align_row_by_row {
                // java PropertySet finalyProperty = row.getFinalProperty();
                let final_property = Self::getFinalRowProperty(document, row.end_cp - 1);
                if final_property.contains(296) {
                    table_before = Self::table_length_value(&final_property, 296);
                }
            }
            // java jcData.jcLeft = Math.min(jcData.jcLeft, row.getRgdxa(0) - talbeBefore);
            jc_data.jcLeft = jc_data.jcLeft.min(row.rgdxa_left - table_before);
            // java jcData.jcRight = Math.max(jcData.jcRight, row.getRgdxa(row.size()) - talbeBefore);
            jc_data.jcRight = jc_data.jcRight.max(row.rgdxa_right - table_before);
        }
        jc_data
    }

    // Java: public static int[] calcCellWidths(...)
    pub fn calcCellWidths(
        document: &dyn Document,
        level: i32,
        row_start_cp: i32,
        final_row_property: &PropertySet,
        table_property: &PropertySet,
        margins: &[(i32, i32)],
        jc_data: &TableJCData,
        take_table_width_from_jc_data: bool,
        parent_cell: Option<ParentCellFit>,
    ) -> Option<Vec<i32>> {
        let def_table = Self::def_table(final_row_property);
        let cell_count = def_table.getCellCount();
        if cell_count == 0 {
            return None;
        }
        let mut table_width = 0;
        if take_table_width_from_jc_data {
            table_width = (jc_data.jcRight - jc_data.jcLeft).abs();
        }
        let mut is_table_width_auto = false;
        if table_width <= 0 {
            if final_row_property.contains(295) {
                if let Some(table_length) = Self::table_length(final_row_property, 295) {
                    if table_length.getType() == 2 {
                        let rgdx_total = def_table.getRgdxa(cell_count) - def_table.getRgdxa(0);
                        if level <= 1 {
                            let core_width = Self::getTableFitMaxWidthAt(
                                document,
                                row_start_cp,
                                final_row_property,
                                cell_count - 1,
                            );
                            table_width = Self::round_half_up(
                                (table_length.getValue() as f32) / 5000.0 * (core_width as f32),
                            );
                            let doc_type = document.getType__on_IDocument();
                            if rgdx_total < table_width && doc_type != 5 && doc_type != 6 {
                                table_width = rgdx_total;
                            }
                        } else {
                            let core_width = Self::getParentcellFitMaxWidth(parent_cell, table_property);
                            table_width = Self::round_half_up(
                                (table_length.getValue() as f32) / 5000.0 * (core_width as f32),
                            );
                        }
                    } else if table_length.getType() == 3 {
                        table_width = table_length.getValue();
                    } else if table_length.getType() == 1 {
                        is_table_width_auto = true;
                    }
                }
            } else {
                is_table_width_auto = true;
            }
        }
        let first_cell = def_table.getTableCell(0);
        let first_width_type = first_cell.map(|cell| cell.getWidthType()).unwrap_or(0);
        if first_width_type == 2 && table_width == 0 {
            table_width = def_table.getRgdxa(cell_count) - def_table.getRgdxa(0);
        }
        let mut cell_widths = vec![0; cell_count as usize];
        let mut row_width_total = 0;
        if first_width_type == 2 {
            let mut pct_total = 0.0f32;
            let mut i = 0;
            while i < cell_count {
                let cell = def_table.getTableCell(i);
                let pct = cell.map(|cell| cell.getWidth() as f32).unwrap_or(0.0) / 5000.0;
                if pct <= 1.0 - pct_total {
                    cell_widths[i as usize] = (pct * (table_width as f32)) as i32;
                    pct_total += pct;
                } else {
                    let leave_pct = 1.0 - pct_total;
                    if leave_pct > 0.0 {
                        cell_widths[i as usize] = (leave_pct * (table_width as f32)) as i32;
                        pct_total = 1.0;
                    } else {
                        cell_widths[i as usize] = 0;
                    }
                }
                row_width_total += cell_widths[i as usize];
                i += 1;
            }
        } else {
            let mut auto_force_width_total = 0.0f32;
            let mut i = 0;
            while i < cell_count {
                let cell = def_table.getTableCell(i);
                let width_type = cell.map(|cell| cell.getWidthType()).unwrap_or(0);
                let mut width = cell.map(|cell| cell.getWidth()).unwrap_or(0);
                if width_type == 0 && width == 0 {
                    width = def_table.getRgdxa(i + 1) - def_table.getRgdxa(i);
                    auto_force_width_total += width as f32;
                } else if !is_table_width_auto && width_type == 1 {
                    width = def_table.getRgdxa(i + 1) - def_table.getRgdxa(i);
                    auto_force_width_total += width as f32;
                }
                cell_widths[i as usize] = width;
                row_width_total += width;
                i += 1;
            }
            if table_width > 0 && row_width_total != table_width {
                if row_width_total > table_width && auto_force_width_total > 0.0 {
                    let adjust_auto_total = ((row_width_total - table_width) as f32).min(auto_force_width_total);
                    row_width_total = 0;
                    let mut i = 0;
                    while i < cell_count {
                        if def_table.getTableCell(i).map(|cell| cell.getWidthType()).unwrap_or(0) == 1 {
                            cell_widths[i as usize] = (cell_widths[i as usize] as f32
                                - adjust_auto_total * (cell_widths[i as usize] as f32) / auto_force_width_total)
                                as i32;
                        }
                        row_width_total += cell_widths[i as usize];
                        i += 1;
                    }
                }
                if row_width_total != table_width
                    && row_width_total != 0
                    && (jc_data.jcLeft - def_table.getRgdxa(0)).abs() < 40
                    && (jc_data.jcRight - def_table.getRgdxa(cell_count)).abs() < 40
                {
                    let mut i = 0;
                    while i < cell_count {
                        let width = cell_widths[i as usize];
                        cell_widths[i as usize] = table_width * width / row_width_total;
                        i += 1;
                    }
                    row_width_total = table_width;
                }
            }
        }
        let mut need_reduce = 0;
        let mut leave_width = 0;
        let mut i = 0;
        while i < cell_count {
            let (margin_left, margin_right) = margins.get(i as usize).copied().unwrap_or((0, 0));
            let ori_width = cell_widths[i as usize];
            let min_width = margin_left + margin_right;
            let diff = ori_width - min_width;
            if diff < 0 {
                need_reduce += -diff;
            } else {
                leave_width += diff;
            }
            i += 1;
        }
        if need_reduce > 0 {
            let core_width = Self::getTableFitMaxWidthAt(
                document,
                row_start_cp,
                final_row_property,
                cell_count - 1,
            );
            if is_table_width_auto && core_width >= row_width_total + need_reduce {
                let mut i = 0;
                while i < cell_count {
                    let (margin_left, margin_right) = margins.get(i as usize).copied().unwrap_or((0, 0));
                    let min_width = margin_left + margin_right;
                    if cell_widths[i as usize] - min_width < 0 {
                        cell_widths[i as usize] = min_width;
                    }
                    i += 1;
                }
            } else {
                if need_reduce > leave_width {
                    need_reduce = leave_width;
                }
                let mut i = 0;
                while i < cell_count {
                    let (margin_left, margin_right) = margins.get(i as usize).copied().unwrap_or((0, 0));
                    let ori_width = cell_widths[i as usize];
                    let min_width = margin_left + margin_right;
                    let diff = ori_width - min_width;
                    if diff < 0 {
                        cell_widths[i as usize] = min_width;
                    } else if leave_width > 0 && diff > 0 {
                        cell_widths[i as usize] -= diff * need_reduce / leave_width;
                    }
                    i += 1;
                }
            }
        }
        Some(cell_widths)
    }

    // Java: public static int getTableFitMaxWidth(Document document, RowInfo firstRowInfo)
    pub fn getTableFitMaxWidthAt(
        document: &dyn Document,
        cp: i32,
        row_property: &PropertySet,
        last_cell_index: i32,
    ) -> i32 {
        let mut core_width = 0;
        match document.getType__on_IDocument() {
            0 | 1 | 2 | 4 => {
                core_width = Tools::getMainDocumentCoreWidth(document, cp);
                if Tools::compatibility_mode(document).isTableIndentedByOldStyle() {
                    let left_margin = Self::getDefaultCellMarginLeft(row_property);
                    let right_margin = Self::getDefaultCellMarginRightAt(row_property, last_cell_index);
                    core_width += left_margin + right_margin;
                }
            }
            5 | 6 => {
                core_width = Tools::getTextBoxCoreWidth(document, cp);
            }
            _ => {}
        }
        if core_width > 0 && row_property.getInt__int_int(303, 0) == 0 {
            if let Some(table_length) = Self::table_length(row_property, 298) {
                core_width -= table_length.getTWIPS();
            }
        }
        core_width
    }

    // Java: public static int getParentcellFitMaxWidth(RowInfo rowInfo)
    pub fn getParentcellFitMaxWidth(parent_cell: Option<ParentCellFit>, table_property: &PropertySet) -> i32 {
        let mut core_width = 0;
        if let Some(parent_cell) = parent_cell {
            core_width = parent_cell.rgdxa_right - parent_cell.rgdxa_left - parent_cell.margin_left - parent_cell.margin_right;
        }
        if core_width > 0 && table_property.getInt__int_int(303, 0) == 0 {
            if let Some(table_length) = Self::table_length(table_property, 298) {
                core_width -= table_length.getValue();
            }
        }
        core_width
    }

    // Java: public static int getDefaultCellMarginLeft(RowInfo firstRowInfo)
    fn getDefaultCellMarginLeft(row_property: &PropertySet) -> i32 {
        if let Some(margins) = Self::cell_margins(row_property, 301) {
            for margin in &margins {
                if 0 >= margin.getFirst() && 0 < margin.getBeyond() {
                    let table_length = margin.getCellSpacingValue();
                    if margin.getGrbitChg() & 2 != 0 && table_length.isValid() {
                        return table_length.getValue();
                    }
                }
            }
        }
        108
    }

    // Java: public static int getDefaultCellMarginRight(RowInfo firstRowInfo)
    pub fn getDefaultCellMarginRightAt(row_property: &PropertySet, last_cell_index: i32) -> i32 {
        let Some(margins) = Self::cell_margins(row_property, 301) else {
            return 108;
        };
        for margin in &margins {
            if last_cell_index >= margin.getFirst() && last_cell_index < margin.getBeyond() {
                let table_length = margin.getCellSpacingValue();
                if margin.getGrbitChg() & 8 != 0 && table_length.isValid() {
                    return table_length.getValue();
                }
            }
        }
        let mut i = 0;
        while i < margins.len() {
            let margin = &margins[i];
            if margin.getFirst() == 0 && margin.getBeyond() == 1 {
                let table_length = margin.getCellSpacingValue();
                if margin.getGrbitChg() & 8 != 0 && table_length.isValid() {
                    return table_length.getValue();
                }
            }
            i += 1;
        }
        108
    }

    fn table_length(prop: &PropertySet, id: i32) -> Option<TableLength> {
        prop.getValue(id).and_then(|value| value.as_any().downcast_ref::<TableLength>().cloned())
    }

    fn table_length_value(prop: &PropertySet, id: i32) -> i32 {
        if !prop.contains(id) {
            return 0;
        }
        Self::table_length(prop, id).map(|length| length.getValue()).unwrap_or(0)
    }

    fn round_half_up(value: f32) -> i32 {
        (value + 0.5).floor() as i32
    }
}

// Java: row geometry calcJC walks after calcRowStyle
pub struct RowJcInput {
    pub end_cp: i32,
    pub rgdxa_left: i32,
    pub rgdxa_right: i32,
}

// Java: parentCell.getRgdxaRight/Left and margins
#[derive(Clone, Copy)]
pub struct ParentCellFit {
    pub rgdxa_left: i32,
    pub rgdxa_right: i32,
    pub margin_left: i32,
    pub margin_right: i32,
}
