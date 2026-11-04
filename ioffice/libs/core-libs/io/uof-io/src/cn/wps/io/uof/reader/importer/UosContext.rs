// Java: import cn.wps.io.uof.reader.importer.uos.info.BorderLineInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::BorderLineInfo::BorderLineInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.ColorInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::ColorInfo::ColorInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.NameInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::NameInfo::NameInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.filter.DateFilterInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::filter::DateFilterInfo::DateFilterInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.filter.DynamicFilterInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::filter::DynamicFilterInfo::DynamicFilterInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.filter.FilterConditionInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::filter::FilterConditionInfo::FilterConditionInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.hyperLink.HyperLinkInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::hyperLink::HyperLinkInfo::HyperLinkInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.rule.ConditionalFormatInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::rule::ConditionalFormatInfo::ConditionalFormatInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.sheet.HeaderFooterInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::sheet::HeaderFooterInfo::HeaderFooterInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.sheet.PageSettingInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::sheet::PageSettingInfo::PageSettingInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.sheet.ProtectionInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::sheet::ProtectionInfo::ProtectionInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.sheet.ViewInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::sheet::ViewInfo::ViewInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.sort.SortConditionInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::sort::SortConditionInfo::SortConditionInfo;
// Java: import cn.wps.io.uof.reader.importer.uos.info.sort.SortInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::sort::SortInfo::SortInfo;
// Java: import java.util.ArrayList
// Java: import java.util.List

// Java: public class UosContext
pub struct UosContext {
    // Java field: int rowIndex
    pub rowIndex: i32,
    // Java field: int colIndex
    pub colIndex: i32,
    // Java field: boolean isRowHidden
    pub isRowHidden: bool,
    // Java field: String cellDataType
    pub cellDataType: Option<String>,
    // Java field: String cellValue
    pub cellValue: Option<String>,
    // Java field: Integer cellStyleIdentifier
    pub cellStyleIdentifier: Option<i32>,
    // Java field: Integer top
    pub top: Option<i32>,
    // Java field: Integer bottom
    pub bottom: Option<i32>,
    // Java field: Integer left
    pub left: Option<i32>,
    // Java field: Integer right
    pub right: Option<i32>,
    // Java field: String identifier
    pub identifier: Option<String>,
    // Java field: String sharedStringText
    pub sharedStringText: Option<String>,
    // Java field: int activeSpreadSheetRef
    pub activeSpreadSheetRef: i32,
    // Java field: String formula
    pub formula: Option<String>,
    // Java field: String formulaType
    pub formulaType: Option<String>,
    // Java field: String sharedFormulaRef
    pub sharedFormulaRef: Option<String>,
    // Java field: String formulaArea
    pub formulaArea: Option<String>,
    // Java field: String filterRange
    pub filterRange: Option<String>,
    // Java field: String filterColIndex
    pub filterColIndex: Option<String>,
    // Java field: FilterConditionInfo filterConditionInfo
    pub filterConditionInfo: Option<FilterConditionInfo>,
    // Java field: DateFilterInfo dateFilterInfoCache
    pub dateFilterInfoCache: Option<DateFilterInfo>,
    // Java field: DynamicFilterInfo dynamicFilterCache
    pub dynamicFilterCache: Option<DynamicFilterInfo>,
    // Java field: String filterCustomOp
    pub filterCustomOp: Option<String>,
    // Java field: String filterCustomValue
    pub filterCustomValue: Option<String>,
    // Java field: Integer numberId
    pub numberId: Option<i32>,
    // Java field: Integer numberIdentifier
    pub numberIdentifier: Option<i32>,
    // Java field: String numberFormat
    pub numberFormat: Option<String>,
    // Java field: ColorInfo colorCache
    pub colorCache: Option<ColorInfo>,
    // Java field: Integer fontIdentifier
    pub fontIdentifier: Option<i32>,
    // Java field: ColorInfo fontThemeColorRef
    pub fontThemeColorRef: Option<ColorInfo>,
    // Java field: String fontName
    pub fontName: Option<String>,
    // Java field: Integer fontSize
    pub fontSize: Option<i32>,
    // Java field: Integer fontCharSet
    pub fontCharSet: Option<i32>,
    // Java field: String fontClan
    pub fontClan: Option<String>,
    // Java field: boolean fontIsB
    pub fontIsB: bool,
    // Java field: boolean fontIsI
    pub fontIsI: bool,
    // Java field: String fontStrike
    pub fontStrike: Option<String>,
    // Java field: String fontUnderLine
    pub fontUnderLine: Option<String>,
    // Java field: String fontSuperSubScriptType
    pub fontSuperSubScriptType: Option<String>,
    // Java field: String fontThemeFont
    pub fontThemeFont: Option<String>,
    // Java field: ColorInfo fontColor
    pub fontColor: Option<ColorInfo>,
    // Java field: Integer fillIdentifier
    pub fillIdentifier: Option<i32>,
    // Java field: String fillPatternType
    pub fillPatternType: Option<String>,
    // Java field: ColorInfo fillForegroundColor
    pub fillForegroundColor: Option<ColorInfo>,
    // Java field: ColorInfo fillBackgroundColor
    pub fillBackgroundColor: Option<ColorInfo>,
    // Java field: BorderLineInfo borderCache
    pub borderCache: Option<BorderLineInfo>,
    // Java field: Integer borderIdentifier
    pub borderIdentifier: Option<i32>,
    // Java field: BorderLineInfo borderTop
    pub borderTop: Option<BorderLineInfo>,
    // Java field: BorderLineInfo borderBottom
    pub borderBottom: Option<BorderLineInfo>,
    // Java field: BorderLineInfo borderLeft
    pub borderLeft: Option<BorderLineInfo>,
    // Java field: BorderLineInfo borderRight
    pub borderRight: Option<BorderLineInfo>,
    // Java field: BorderLineInfo borderLDiagonal
    pub borderLDiagonal: Option<BorderLineInfo>,
    // Java field: BorderLineInfo borderRDiagonal
    pub borderRDiagonal: Option<BorderLineInfo>,
    // Java field: Integer xfIdentifier
    pub xfIdentifier: Option<i32>,
    // Java field: boolean xfIsShare
    pub xfIsShare: bool,
    // Java field: boolean xfIsNumberFormat
    pub xfIsNumberFormat: bool,
    // Java field: boolean xfIsFont
    pub xfIsFont: bool,
    // Java field: boolean xfIsFill
    pub xfIsFill: bool,
    // Java field: boolean xfIsBorder
    pub xfIsBorder: bool,
    // Java field: boolean xfIsApplication
    pub xfIsApplication: bool,
    // Java field: boolean xfIsProtect
    pub xfIsProtect: bool,
    // Java field: boolean xfIsTable
    pub xfIsTable: bool,
    // Java field: boolean xfIsCustom
    pub xfIsCustom: bool,
    // Java field: Integer xfNumberFormatRef
    pub xfNumberFormatRef: Option<i32>,
    // Java field: Integer xfFontRef
    pub xfFontRef: Option<i32>,
    // Java field: Integer xfFillRef
    pub xfFillRef: Option<i32>,
    // Java field: Integer xfBorderRef
    pub xfBorderRef: Option<i32>,
    // Java field: String xfVerticalAlignment
    pub xfVerticalAlignment: Option<String>,
    // Java field: String xfHorizonAlignment
    pub xfHorizonAlignment: Option<String>,
    // Java field: SortInfo sortInfo
    pub sortInfo: Option<SortInfo>,
    // Java field: SortConditionInfo sortConditionCache
    pub sortConditionCache: Option<SortConditionInfo>,
    // Java field: PageSettingInfo pageSettingInfo
    pub pageSettingInfo: PageSettingInfo,
    // Java field: HeaderFooterInfo headerFooterCache
    pub headerFooterCache: HeaderFooterInfo,
    // Java field: ViewInfo viewInfo
    pub viewInfo: ViewInfo,
    // Java field: ProtectionInfo protectionInfo
    pub protectionInfo: ProtectionInfo,
    // Java field: List<HyperLinkInfo> hyperLinkInfos
    pub hyperLinkInfos: Vec<HyperLinkInfo>,
    // Java field: HyperLinkInfo hyperLinkCache
    pub hyperLinkCache: HyperLinkInfo,
    // Java field: List<NameInfo> nameList
    pub nameList: Vec<NameInfo>,
    // Java field: ConditionalFormatInfo conditionalFormatCache
    pub conditionalFormatCache: ConditionalFormatInfo,
    // Java field: List<ConditionalFormatInfo> conditionalFormatInfoList
    pub conditionalFormatInfoList: Vec<ConditionalFormatInfo>,
}

// Java: public class UosContext
impl UosContext {
    // Java: UosContext()
    pub fn UosContext() -> Self {
      // Java: super();
      UosContext {
          rowIndex: 0, colIndex: 0, isRowHidden: false,
          cellDataType: None, cellValue: None, cellStyleIdentifier: None,
          top: None, bottom: None, left: None, right: None,
          identifier: None, sharedStringText: None, activeSpreadSheetRef: 0,
          formula: None, formulaType: None, sharedFormulaRef: None, formulaArea: None,
          filterRange: None, filterColIndex: None, filterConditionInfo: None,
          dateFilterInfoCache: None, dynamicFilterCache: None,
          filterCustomOp: None, filterCustomValue: None,
          numberId: None, numberIdentifier: None, numberFormat: None,
          colorCache: None, fontIdentifier: None, fontThemeColorRef: None,
          fontName: None, fontSize: None, fontCharSet: None, fontClan: None,
          fontIsB: false, fontIsI: false, fontStrike: None, fontUnderLine: None,
          fontSuperSubScriptType: None, fontThemeFont: None, fontColor: None,
          fillIdentifier: None, fillPatternType: None,
          fillForegroundColor: None, fillBackgroundColor: None,
          borderCache: None, borderIdentifier: None,
          borderTop: None, borderBottom: None, borderLeft: None, borderRight: None,
          borderLDiagonal: None, borderRDiagonal: None,
          xfIdentifier: None, xfIsShare: false, xfIsNumberFormat: false,
          xfIsFont: false, xfIsFill: false, xfIsBorder: false,
          xfIsApplication: false, xfIsProtect: false, xfIsTable: false, xfIsCustom: false,
          xfNumberFormatRef: None, xfFontRef: None, xfFillRef: None, xfBorderRef: None,
          xfVerticalAlignment: None, xfHorizonAlignment: None,
          sortInfo: None, sortConditionCache: None,
          pageSettingInfo: PageSettingInfo::PageSettingInfo(),
          headerFooterCache: HeaderFooterInfo::HeaderFooterInfo(),
          viewInfo: ViewInfo::ViewInfo(),
          protectionInfo: ProtectionInfo::ProtectionInfo(),
          hyperLinkInfos: Vec::new(),
          hyperLinkCache: HyperLinkInfo::HyperLinkInfo(),
          nameList: Vec::new(),
          conditionalFormatCache: ConditionalFormatInfo::ConditionalFormatInfo(),
          conditionalFormatInfoList: Vec::new(),
      }
    }

    // Java: void initRow()
    pub fn initRow(&mut self) {
      // Java: this.isRowHidden = false;
      self.isRowHidden = false;
    }

    // Java: void initCellValue()
    pub fn initCellValue(&mut self) {
      // Java: this.cellDataType = null;
      self.cellDataType = None;
      // Java: this.cellValue = null;
      self.cellValue = None;
      // Java: this.cellStyleIdentifier = null;
      self.cellStyleIdentifier = None;
    }

    // Java: void initUsedRange()
    pub fn initUsedRange(&mut self) {
      // Java: this.top = null;
      self.top = None;
      // Java: this.bottom = null;
      self.bottom = None;
      // Java: this.left = null;
      self.left = None;
      // Java: this.right = null;
      self.right = None;
    }

    // Java: void initDateFilterCache()
    pub fn initDateFilterCache(&mut self) {
      // Java: this.dateFilterInfoCache = new DateFilterInfo();
      self.dateFilterInfoCache = Some(DateFilterInfo::DateFilterInfo());
    }

    // Java: void initDynamicFilterCache()
    pub fn initDynamicFilterCache(&mut self) {
      // Java: this.dynamicFilterCache = new DynamicFilterInfo();
      self.dynamicFilterCache = Some(DynamicFilterInfo::DynamicFilterInfo());
    }

    // Java: void initCustomFilter()
    pub fn initCustomFilter(&mut self) {
      // Java: this.filterCustomOp = null;
      self.filterCustomOp = None;
      // Java: this.filterCustomValue = null;
      self.filterCustomValue = None;
    }

    // Java: void initFilter()
    pub fn initFilter(&mut self) {
      // Java: this.filterConditionInfo = new FilterConditionInfo();
      self.filterConditionInfo = Some(FilterConditionInfo::FilterConditionInfo());
    }

    // Java: void initNumberFormat()
    pub fn initNumberFormat(&mut self) {
      // Java: this.numberId = null;
      self.numberId = None;
      // Java: this.numberIdentifier = null;
      self.numberIdentifier = None;
      // Java: this.numberFormat = null;
      self.numberFormat = None;
    }

    // Java: void initColorCache()
    pub fn initColorCache(&mut self) {
      // Java: this.colorCache = new ColorInfo();
      self.colorCache = Some(ColorInfo::ColorInfo());
    }

    // Java: void initFont()
    pub fn initFont(&mut self) {
      // Java: this.fontIdentifier = null;
      self.fontIdentifier = None;
      // Java: this.fontThemeColorRef = null;
      self.fontThemeColorRef = None;
      // Java: this.fontName = null;
      self.fontName = None;
      // Java: this.fontSize = null;
      self.fontSize = None;
      // Java: this.fontCharSet = null;
      self.fontCharSet = None;
      // Java: this.fontClan = null;
      self.fontClan = None;
      // Java: this.fontIsB = false;
      self.fontIsB = false;
      // Java: this.fontIsI = false;
      self.fontIsI = false;
      // Java: this.fontStrike = null;
      self.fontStrike = None;
      // Java: this.fontUnderLine = null;
      self.fontUnderLine = None;
      // Java: this.fontSuperSubScriptType = null;
      self.fontSuperSubScriptType = None;
      // Java: this.fontThemeFont = null;
      self.fontThemeFont = None;
      // Java: this.fontColor = null;
      self.fontColor = None;
    }

    // Java: void initFill()
    pub fn initFill(&mut self) {
      // Java: this.fillIdentifier = null;
      self.fillIdentifier = None;
      // Java: this.fillPatternType = null;
      self.fillPatternType = None;
      // Java: this.fillForegroundColor = null;
      self.fillForegroundColor = None;
      // Java: this.fillBackgroundColor = null;
      self.fillBackgroundColor = None;
    }

    // Java: void initBorderCache()
    pub fn initBorderCache(&mut self) {
      // Java: this.borderCache = new BorderLineInfo();
      self.borderCache = Some(BorderLineInfo::BorderLineInfo());
    }

    // Java: void initBorder()
    pub fn initBorder(&mut self) {
      // Java: this.borderIdentifier = null;
      self.borderIdentifier = None;
      // Java: this.borderTop = null;
      self.borderTop = None;
      // Java: this.borderBottom = null;
      self.borderBottom = None;
      // Java: this.borderLeft = null;
      self.borderLeft = None;
      // Java: this.borderRight = null;
      self.borderRight = None;
      // Java: this.borderLDiagonal = null;
      self.borderLDiagonal = None;
      // Java: this.borderRDiagonal = null;
      self.borderRDiagonal = None;
    }

    // Java: void initXf()
    pub fn initXf(&mut self) {
      // Java: this.xfIdentifier = null;
      self.xfIdentifier = None;
      // Java: this.xfIsShare = false;
      self.xfIsShare = false;
      // Java: this.xfIsNumberFormat = false;
      self.xfIsNumberFormat = false;
      // Java: this.xfIsFont = false;
      self.xfIsFont = false;
      // Java: this.xfIsFill = false;
      self.xfIsFill = false;
      // Java: this.xfIsBorder = false;
      self.xfIsBorder = false;
      // Java: this.xfIsApplication = false;
      self.xfIsApplication = false;
      // Java: this.xfIsProtect = false;
      self.xfIsProtect = false;
      // Java: this.xfIsTable = false;
      self.xfIsTable = false;
      // Java: this.xfIsCustom = false;
      self.xfIsCustom = false;
      // Java: this.xfNumberFormatRef = null;
      self.xfNumberFormatRef = None;
      // Java: this.xfFontRef = null;
      self.xfFontRef = None;
      // Java: this.xfFillRef = null;
      self.xfFillRef = None;
      // Java: this.xfBorderRef = null;
      self.xfBorderRef = None;
      // Java: this.xfVerticalAlignment = null;
      self.xfVerticalAlignment = None;
      // Java: this.xfHorizonAlignment = null;
      self.xfHorizonAlignment = None;
    }

    // Java: void initSortInfo()
    pub fn initSortInfo(&mut self) {
      // Java: this.sortInfo = new SortInfo();
      self.sortInfo = Some(SortInfo::SortInfo());
    }

    // Java: void initSortConditionCache()
    pub fn initSortConditionCache(&mut self) {
      // Java: this.sortConditionCache = new SortConditionInfo();
      self.sortConditionCache = Some(SortConditionInfo::SortConditionInfo());
    }

    // Java: void initPageSettingInfo()
    pub fn initPageSettingInfo(&mut self) {
      // Java: this.pageSettingInfo = new PageSettingInfo();
      self.pageSettingInfo = PageSettingInfo::PageSettingInfo();
    }

    // Java: void initHeaderFooter()
    pub fn initHeaderFooter(&mut self) {
      // Java: this.headerFooterCache = new HeaderFooterInfo();
      self.headerFooterCache = HeaderFooterInfo::HeaderFooterInfo();
    }

    // Java: void initViewInfo()
    pub fn initViewInfo(&mut self) {
      // Java: this.viewInfo = new ViewInfo();
      self.viewInfo = ViewInfo::ViewInfo();
    }

    // Java: void initProtectionInfo()
    pub fn initProtectionInfo(&mut self) {
      // Java: this.protectionInfo = new ProtectionInfo();
      self.protectionInfo = ProtectionInfo::ProtectionInfo();
    }

    // Java: void initHyperLinkInfos()
    pub fn initHyperLinkInfos(&mut self) {
      // Java: this.hyperLinkInfos = new ArrayList();
      self.hyperLinkInfos = Vec::new();
    }

    // Java: void initHyperLinkInfoCache()
    pub fn initHyperLinkInfoCache(&mut self) {
      // Java: this.hyperLinkCache = new HyperLinkInfo();
      self.hyperLinkCache = HyperLinkInfo::HyperLinkInfo();
    }

    // Java: void initNameInfoList()
    pub fn initNameInfoList(&mut self) {
      // Java: this.nameList = new ArrayList();
      self.nameList = Vec::new();
    }

    // Java: void initConditionalFormatCache()
    pub fn initConditionalFormatCache(&mut self) {
      // Java: this.conditionalFormatCache = new ConditionalFormatInfo();
      self.conditionalFormatCache = ConditionalFormatInfo::ConditionalFormatInfo();
    }

    // Java: void initConditionalFormatInfoList()
    pub fn initConditionalFormatInfoList(&mut self) {
      // Java: this.conditionalFormatInfoList = new ArrayList();
      self.conditionalFormatInfoList = Vec::new();
    }

}
