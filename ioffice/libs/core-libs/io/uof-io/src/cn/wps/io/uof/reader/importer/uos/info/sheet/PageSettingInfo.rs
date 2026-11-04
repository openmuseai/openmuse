// Java: import cn.wps.io.uof.util.UofUnit
use crate::cn::wps::io::uof::util::UofUnit::UofUnit;

use super::HeaderFooterInfo::HeaderFooterInfo;

// Java: public class PageSettingInfo
pub struct PageSettingInfo {
    // Java field: UofUnit pageLength
    pub pageLength: Option<UofUnit>,
    // Java field: UofUnit pageWidth
    pub pageWidth: Option<UofUnit>,
    // Java field: String pageDirection
    pub pageDirection: Option<String>,
    // Java field: Double pageScale
    pub pageScale: Option<f64>,
    // Java field: UofUnit pageMarginLeft
    pub pageMarginLeft: Option<UofUnit>,
    // Java field: UofUnit pageMarginRight
    pub pageMarginRight: Option<UofUnit>,
    // Java field: UofUnit pageMarginTop
    pub pageMarginTop: Option<UofUnit>,
    // Java field: UofUnit pageMarginBottom
    pub pageMarginBottom: Option<UofUnit>,
    // Java field: boolean isOdd
    pub isOdd: bool,
    // Java field: boolean isFirst
    pub isFirst: bool,
    // Java field: boolean isAlignMargin
    pub isAlignMargin: bool,
    // Java field: boolean isAutoZoom
    pub isAutoZoom: bool,
    // Java field: HeaderFooterInfo footerInfo
    pub footerInfo: HeaderFooterInfo,
    // Java field: HeaderFooterInfo headerInfo
    pub headerInfo: HeaderFooterInfo,
    // Java field: boolean isGrid
    pub isGrid: bool,
    // Java field: boolean isLineNumberColumnMark
    pub isLineNumberColumnMark: bool,
    // Java field: boolean isAccordDraft
    pub isAccordDraft: bool,
    // Java field: boolean isPriorityColumn
    pub isPriorityColumn: bool,
    // Java field: String commentsPrintWay
    pub commentsPrintWay: Option<String>,
    // Java field: String errorCellPrintWay
    pub errorCellPrintWay: Option<String>,
    // Java field: String verticalAlignment
    pub verticalAlignment: Option<String>,
    // Java field: String horizonAlignment
    pub horizonAlignment: Option<String>,
    // Java field: Integer pageHeightTimes
    pub pageHeightTimes: Option<i32>,
    // Java field: Integer pageWidthTimes
    pub pageWidthTimes: Option<i32>,
    // Java field: Integer pageStartNum
    pub pageStartNum: Option<i32>,
    // Java field: boolean isUseStartPageNum
    pub isUseStartPageNum: bool,
    // Java field: Long horizontalResolution
    pub horizontalResolution: Option<i64>,
    // Java field: Long verticalResolution
    pub verticalResolution: Option<i64>,
    // Java field: boolean isShrinkPageCount
    pub isShrinkPageCount: bool,
    // Java field: boolean isBlackAndWhite
    pub isBlackAndWhite: bool,
}

// Java: public class PageSettingInfo
impl PageSettingInfo {
    // Java: PageSettingInfo()
    pub fn PageSettingInfo() -> Self {
      // Java: super();
      PageSettingInfo {
          pageLength: None,
          pageWidth: None,
          pageDirection: None,
          pageScale: None,
          pageMarginLeft: None,
          pageMarginRight: None,
          pageMarginTop: None,
          pageMarginBottom: None,
          isOdd: false,
          isFirst: false,
          isAlignMargin: true,
          isAutoZoom: true,
          footerInfo: HeaderFooterInfo::HeaderFooterInfo(),
          headerInfo: HeaderFooterInfo::HeaderFooterInfo(),
          isGrid: false,
          isLineNumberColumnMark: false,
          isAccordDraft: false,
          isPriorityColumn: true,
          commentsPrintWay: None,
          errorCellPrintWay: None,
          verticalAlignment: None,
          horizonAlignment: None,
          pageHeightTimes: None,
          pageWidthTimes: None,
          pageStartNum: None,
          isUseStartPageNum: false,
          horizontalResolution: None,
          verticalResolution: None,
          isShrinkPageCount: false,
          isBlackAndWhite: false,
      }
    }

}
