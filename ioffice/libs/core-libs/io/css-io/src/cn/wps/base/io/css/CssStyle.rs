// Java: import cn.wps.base.assertion.Assert
// use util::cn::wps::core::runtime::Assert;
// Java: import cn.wps.base.assertion.IOAssert
// use util::cn::wps::base::assertion::IOAssert;
// Java: import cn.wps.base.io.MetricUtility
// use util::cn::wps::base::io::MetricUtility;
// Java: import cn.wps.base.io.TypeConversion
// use util::cn::wps::base::io::TypeConversion;
// Java: import cn.wps.core.runtime.Platform
// use util_adapter::core::runtime::Platform;
// Java: import cn.wps.moffice.util.StringUtil
// use util::org::apache::poi::util::StringUtil;

use super::Flip::Flip;
use super::FontStyle::FontStyle;
use super::FontVariant::FontVariant;
use super::FontWeight::FontWeight;
use super::TextDecoration::TextDecoration;
use super::WrapStyle::WrapStyle;
use super::Position::Position;
use super::PositionH::PositionH;
use super::PositionV::PositionV;
use super::PositionHRel::PositionHRel;
use super::PositionVRel::PositionVRel;
use super::Visibility::Visibility;
use super::WidthRel::WidthRel;
use super::HeightRel::HeightRel;
use super::VerticalTextAnchor::VerticalTextAnchor;
use super::LayoutFlow::LayoutFlow;
use super::MsoLayoutFlowAlt::MsoLayoutFlowAlt;
use super::TextAlign::TextAlign;
use super::TextSpacingMode::TextSpacingMode;
use super::PercentOrFractionUnit::PercentOrFractionUnit;
use super::HeightRule::HeightRule;
use super::TextTransfrom::TextTransfrom;
use super::LayoutGridMode::LayoutGridMode;
use super::VerticalAlign::VerticalAlign;
use super::TextEffect::TextEffect;
use super::TextAnimation::TextAnimation;
use super::FontEmphasize::FontEmphasize;
use super::TextLineThrough::TextLineThrough;
use super::GutterPosition::GutterPosition;
use super::TextJustify::TextJustify;
use super::LineHeightRule::LineHeightRule;
use super::PageOrientation::PageOrientation;
use super::Vjc::Vjc;
use super::SectionOperand::SectionOperand;
use super::SectionBKC::SectionBKC;
use super::BreakType::BreakType;
use super::PageBreakInside::PageBreakInside;
use super::TableAnchorHorizontal::TableAnchorHorizontal;
use super::TableAnchorVertical::TableAnchorVertical;
use super::TableOverlap::TableOverlap;
use super::TableCondition::TableCondition;
use super::MsoElementType::MsoElementType;
use super::SpecialCharacterType::SpecialCharacterType;
use super::MsoTableLayoutAlt::MsoTableLayoutAlt;
use super::MsoCellSpecial::MsoCellSpecial;

// Java: public class CssStyle
pub struct CssStyle {
}

// Java: public class CssUnit
pub struct CssUnit {
    // Java field: Unit mUnit
    pub mUnit: CssUnitKind,
    pub mMagnitude: Option<f32>,
}

// Java: public enum Unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CssUnitKind {
    AUTO,
    MM,
    CM,
    IN,
    PT,
    PC,
    PI,
    PX,
    EM,
    EX,
    PERCENTAGE,
    FRACTION,
    GD,
    EMU,
}

// Java: public class AngleUnit
pub struct AngleUnit {
    // Java field: Unit mUnit
    pub mUnit: AngleUnitKind,
    pub mMagnitude: Option<f32>,
}

// Java: public enum Unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AngleUnitKind {
    FD,
    DEGREE,
    RADIANT,
    F,
}

// Java: public class CssStyle
impl CssStyle {
    // Java: CssStyle()
    pub fn new() -> Self {
      // Java: super();
      CssStyle {}
    }

    // Java: abstract CssUnit getFontSize()
    pub fn getFontSize(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getFontFamily()
    pub fn getFontFamily(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getFont()
    pub fn getFont(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract FontStyle getFontStyle()
    pub fn getFontStyle(&self) -> Option<FontStyle> {
    unimplemented!("abstract method")
    }

    // Java: abstract FontVariant getFontVariant()
    pub fn getFontVariant(&self) -> Option<FontVariant> {
    unimplemented!("abstract method")
    }

    // Java: abstract FontWeight getFontWeight()
    pub fn getFontWeight(&self) -> Option<FontWeight> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getMsoTextShadow()
    pub fn getMsoTextShadow(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract TextDecoration getTextDecoration()
    pub fn getTextDecoration(&self) -> Option<TextDecoration> {
    unimplemented!("abstract method")
    }

    // Java: abstract Flip getFlip()
    pub fn getFlip(&self) -> Option<Flip> {
    unimplemented!("abstract method")
    }

    // Java: abstract WrapStyle getWrapStyle()
    pub fn getWrapStyle(&self) -> Option<WrapStyle> {
    unimplemented!("abstract method")
    }

    // Java: abstract Position getPosition()
    pub fn getPosition(&self) -> Option<Position> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getWidth()
    pub fn getWidth(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getHeight()
    pub fn getHeight(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getLeft()
    pub fn getLeft(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getTop()
    pub fn getTop(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMarginTop()
    pub fn getMarginTop(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMarginBottom()
    pub fn getMarginBottom(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMarginLeft()
    pub fn getMarginLeft(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMarginRight()
    pub fn getMarginRight(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract PositionH getPositionH()
    pub fn getPositionH(&self) -> Option<PositionH> {
    unimplemented!("abstract method")
    }

    // Java: abstract PositionV getPositionV()
    pub fn getPositionV(&self) -> Option<PositionV> {
    unimplemented!("abstract method")
    }

    // Java: abstract PositionHRel getPositionHRel()
    pub fn getPositionHRel(&self) -> Option<PositionHRel> {
    unimplemented!("abstract method")
    }

    // Java: abstract PositionVRel getPositionVRel()
    pub fn getPositionVRel(&self) -> Option<PositionVRel> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getWrapDistanceLeft()
    pub fn getWrapDistanceLeft(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getWrapDistanceTop()
    pub fn getWrapDistanceTop(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getWrapDistanceRight()
    pub fn getWrapDistanceRight(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getWrapDistanceBottom()
    pub fn getWrapDistanceBottom(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract Visibility getVisibility()
    pub fn getVisibility(&self) -> Option<Visibility> {
    unimplemented!("abstract method")
    }

    // Java: abstract AngleUnit getRotation()
    pub fn getRotation(&self) -> Option<AngleUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract Integer getZIndex()
    pub fn getZIndex(&self) -> Option<i32> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getLeftPercent()
    pub fn getLeftPercent(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getTopPercent()
    pub fn getTopPercent(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getWidthPercent()
    pub fn getWidthPercent(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getHeightPercent()
    pub fn getHeightPercent(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract WidthRel getWidthRel()
    pub fn getWidthRel(&self) -> Option<WidthRel> {
    unimplemented!("abstract method")
    }

    // Java: abstract HeightRel getHeightRel()
    pub fn getHeightRel(&self) -> Option<HeightRel> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getFitShapeToText()
    pub fn getFitShapeToText(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getFitTextToShape()
    pub fn getFitTextToShape(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract VerticalTextAnchor getVerticalTextAnchor()
    pub fn getVerticalTextAnchor(&self) -> Option<VerticalTextAnchor> {
    unimplemented!("abstract method")
    }

    // Java: abstract LayoutFlow getLayoutFlow()
    pub fn getLayoutFlow(&self) -> Option<LayoutFlow> {
    unimplemented!("abstract method")
    }

    // Java: abstract MsoLayoutFlowAlt getMsoLayoutFlowAlt()
    pub fn getMsoLayoutFlowAlt(&self) -> Option<MsoLayoutFlowAlt> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getVRotateLetters()
    pub fn getVRotateLetters(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getVSameLetterHeights()
    pub fn getVSameLetterHeights(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoNextTextBox()
    pub fn getMsoNextTextBox(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract TextAlign getVTextAlign()
    pub fn getVTextAlign(&self) -> Option<TextAlign> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getVTextKern()
    pub fn getVTextKern(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getVTextReverse()
    pub fn getVTextReverse(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract TextSpacingMode getVTextSpacingMode()
    pub fn getVTextSpacingMode(&self) -> Option<TextSpacingMode> {
    unimplemented!("abstract method")
    }

    // Java: abstract PercentOrFractionUnit getVTextSpacing()
    pub fn getVTextSpacing(&self) -> Option<PercentOrFractionUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getWrapEdited()
    pub fn getWrapEdited(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract HeightRule getMsoHeightRule()
    pub fn getMsoHeightRule(&self) -> Option<HeightRule> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoBidiFontSize()
    pub fn getMsoBidiFontSize(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoAsciiFontFamily()
    pub fn getMsoAsciiFontFamily(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoFareastFontFamily()
    pub fn getMsoFareastFontFamily(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoHansiFontFamily()
    pub fn getMsoHansiFontFamily(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoBidiFontFamily()
    pub fn getMsoBidiFontFamily(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getColor()
    pub fn getColor(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoHide()
    pub fn getMsoHide(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getDisplay()
    pub fn getDisplay(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getBackground()
    pub fn getBackground(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoPattern()
    pub fn getMsoPattern(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getBackgroundColor()
    pub fn getBackgroundColor(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract TextTransfrom getTextTransform()
    pub fn getTextTransform(&self) -> Option<TextTransfrom> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getLetterSpacing()
    pub fn getLetterSpacing(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoFontKerning()
    pub fn getMsoFontKerning(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract LayoutGridMode getLayoutGridMode()
    pub fn getLayoutGridMode(&self) -> Option<LayoutGridMode> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getMsoNoProof()
    pub fn getMsoNoProof(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoAnsiLanguage()
    pub fn getMsoAnsiLanguage(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoFareastLanguage()
    pub fn getMsoFareastLanguage(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoBidiLanguage()
    pub fn getMsoBidiLanguage(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract VerticalAlign getVerticalAlign()
    pub fn getVerticalAlign(&self) -> Option<VerticalAlign> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoFontWidth()
    pub fn getMsoFontWidth(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract TextEffect getTextEffect()
    pub fn getTextEffect(&self) -> Option<TextEffect> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getTextShadow()
    pub fn getTextShadow(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract TextAnimation getTextAnimation()
    pub fn getTextAnimation(&self) -> Option<TextAnimation> {
    unimplemented!("abstract method")
    }

    // Java: abstract FontEmphasize getFontEmphasize()
    pub fn getFontEmphasize(&self) -> Option<FontEmphasize> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoTextRaise()
    pub fn getMsoTextRaise(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getTextUnderline()
    pub fn getTextUnderline(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract TextLineThrough getTextLineThrough()
    pub fn getTextLineThrough(&self) -> Option<TextLineThrough> {
    unimplemented!("abstract method")
    }

    // Java: abstract GutterPosition getMsoGutterPosition()
    pub fn getMsoGutterPosition(&self) -> Option<GutterPosition> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoBorderColorAlt()
    pub fn getMsoBorderColorAlt(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoBorderWidthAlt()
    pub fn getMsoBorderWidthAlt(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoBorderTopAlt()
    pub fn getMsoBorderTopAlt(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoBorderLeftAlt()
    pub fn getMsoBorderLeftAlt(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoBorderBottomAlt()
    pub fn getMsoBorderBottomAlt(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoBorderRightAlt()
    pub fn getMsoBorderRightAlt(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getBorderColor()
    pub fn getBorderColor(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getBorderWidth()
    pub fn getBorderWidth(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getBorderTop()
    pub fn getBorderTop(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getBorderLeft()
    pub fn getBorderLeft(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getBorderRight()
    pub fn getBorderRight(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getBorderBottom()
    pub fn getBorderBottom(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoBorderAlt()
    pub fn getMsoBorderAlt(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getBorder()
    pub fn getBorder(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getMsoBorderShadow()
    pub fn getMsoBorderShadow(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract TextAlign getTextAlign()
    pub fn getTextAlign(&self) -> Option<TextAlign> {
    unimplemented!("abstract method")
    }

    // Java: abstract TextJustify getTextJustify()
    pub fn getTextJustify(&self) -> Option<TextJustify> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getTextKashida()
    pub fn getTextKashida(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getMsoMarginTopAlt()
    pub fn getMsoMarginTopAlt(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getMsoMarginBottomAlt()
    pub fn getMsoMarginBottomAlt(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoParaMarginTop()
    pub fn getMsoParaMarginTop(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoParaMarginBottom()
    pub fn getMsoParaMarginBottom(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getTextIndent()
    pub fn getTextIndent(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoCharIndentCount()
    pub fn getMsoCharIndentCount(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoParaMarginRight()
    pub fn getMsoParaMarginRight(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoParaMarginLeft()
    pub fn getMsoParaMarginLeft(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getLineHeight()
    pub fn getLineHeight(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract LineHeightRule getMsoLineHeightRule()
    pub fn getMsoLineHeightRule(&self) -> Option<LineHeightRule> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoOutlineLevel()
    pub fn getMsoOutlineLevel(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getLayoutGridAlign()
    pub fn getLayoutGridAlign(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getMsoMirrorIndents()
    pub fn getMsoMirrorIndents(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getTabStops()
    pub fn getTabStops(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoPagination()
    pub fn getMsoPagination(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getPageBreakAfter()
    pub fn getPageBreakAfter(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoLineBreakOverride()
    pub fn getMsoLineBreakOverride(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getPunctuationWrap()
    pub fn getPunctuationWrap(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getWordBreak()
    pub fn getWordBreak(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getPunctuationTrim()
    pub fn getPunctuationTrim(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getTextAutospace()
    pub fn getTextAutospace(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoStyleName()
    pub fn getMsoStyleName(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoStyleParent()
    pub fn getMsoStyleParent(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getTextFit()
    pub fn getTextFit(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getMsoStyleQFormat()
    pub fn getMsoStyleQFormat(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoHeaderMargin()
    pub fn getMsoHeaderMargin(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoFooterMargin()
    pub fn getMsoFooterMargin(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getLayoutGrid()
    pub fn getLayoutGrid(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getSize()
    pub fn getSize(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMargin()
    pub fn getMargin(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoRotate()
    pub fn getMsoRotate(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract PageOrientation getMsoPageOrientation()
    pub fn getMsoPageOrientation(&self) -> Option<PageOrientation> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoGutterMargin()
    pub fn getMsoGutterMargin(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract Vjc getMsoVerticalPageAlign()
    pub fn getMsoVerticalPageAlign(&self) -> Option<Vjc> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoLineNumbersStart()
    pub fn getMsoLineNumbersStart(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoLineNumbersCountBy()
    pub fn getMsoLineNumbersCountBy(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoLineNumbersDistance()
    pub fn getMsoLineNumbersDistance(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract SectionOperand getMsoLineNumbersRestart()
    pub fn getMsoLineNumbersRestart(&self) -> Option<SectionOperand> {
    unimplemented!("abstract method")
    }

    // Java: abstract SectionBKC getPageBreakBefore()
    pub fn getPageBreakBefore(&self) -> Option<SectionBKC> {
    unimplemented!("abstract method")
    }

    // Java: abstract BreakType getMsoBreakType()
    pub fn getMsoBreakType(&self) -> Option<BreakType> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getClear()
    pub fn getClear(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getMsoFacingPages()
    pub fn getMsoFacingPages(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getMsoTitlePage()
    pub fn getMsoTitlePage(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract Boolean getMsoMirrorMargins()
    pub fn getMsoMirrorMargins(&self) -> Option<bool> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoTabCount()
    pub fn getMsoTabCount(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit[] getMsoPaddingAlt()
    pub fn getMsoPaddingAlt(&self) -> Option<Vec<CssUnit>> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit[] getPadding()
    pub fn getPadding(&self) -> Option<Vec<CssUnit>> {
    unimplemented!("abstract method")
    }

    // Java: abstract PageBreakInside getPageBreakInside()
    pub fn getPageBreakInside(&self) -> Option<PageBreakInside> {
    unimplemented!("abstract method")
    }

    // Java: abstract TableAnchorHorizontal getMsoTableAnchorHorizontal()
    pub fn getMsoTableAnchorHorizontal(&self) -> Option<TableAnchorHorizontal> {
    unimplemented!("abstract method")
    }

    // Java: abstract TableAnchorVertical getMsoTableAnchorVertical()
    pub fn getMsoTableAnchorVertical(&self) -> Option<TableAnchorVertical> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoTableRSpace()
    pub fn getMsoTableRSpace(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoTableLSpace()
    pub fn getMsoTableLSpace(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoTableTSpace()
    pub fn getMsoTableTSpace(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoTableBSpace()
    pub fn getMsoTableBSpace(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoRowMarginLeft()
    pub fn getMsoRowMarginLeft(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getMsoRowMarginRight()
    pub fn getMsoRowMarginRight(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract Object getMsoTableTop()
    pub fn getMsoTableTop(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract Object getMsoTableLeft()
    pub fn getMsoTableLeft(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract TableOverlap getMsoTableOverlap()
    pub fn getMsoTableOverlap(&self) -> Option<TableOverlap> {
    unimplemented!("abstract method")
    }

    // Java: abstract CssUnit getTabInterval()
    pub fn getTabInterval(&self) -> Option<CssUnit> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoTStyleShading()
    pub fn getMsoTStyleShading(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract TableCondition getMsoTableCondition()
    pub fn getMsoTableCondition(&self) -> Option<TableCondition> {
    unimplemented!("abstract method")
    }

    // Java: abstract MsoElementType getMsoElementType()
    pub fn getMsoElementType(&self) -> Option<MsoElementType> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoCommentReference()
    pub fn getMsoCommentReference(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoCommentContinuation()
    pub fn getMsoCommentContinuation(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoCommentDate()
    pub fn getMsoCommentDate(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoCommentAuthor()
    pub fn getMsoCommentAuthor(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract SpecialCharacterType getMsoSpecialCharacter()
    pub fn getMsoSpecialCharacter(&self) -> Option<SpecialCharacterType> {
    unimplemented!("abstract method")
    }

    // Java: abstract String getMsoBookmark()
    pub fn getMsoBookmark(&self) -> Option<String> {
    unimplemented!("abstract method")
    }

    // Java: abstract MsoTableLayoutAlt getMsoTableLayoutAlt()
    pub fn getMsoTableLayoutAlt(&self) -> Option<MsoTableLayoutAlt> {
    unimplemented!("abstract method")
    }

    // Java: abstract MsoCellSpecial getMsoCellSpecial()
    pub fn getMsoCellSpecial(&self) -> Option<MsoCellSpecial> {
    unimplemented!("abstract method")
    }

}

// Java: public class CssUnit
impl CssUnit {
    // Java: CssUnit()
    pub fn new() -> Self {
         // Java: super();
         // Java: this.mUnit = CssStyle.CssUnit.Unit.PX;
         // Java: this.mMagnitude = null;
         CssUnit {
             mUnit: CssUnitKind::PX,
             mMagnitude: None,
         }
    }

    // Java: CssUnit(String value)
    pub fn new_from_value(value: &str) -> Self {
         // Java: this();
         let mut unit = Self::new();
         // Java: Assert.assertNotNull("value should not be null", value);
         assert!(!value.is_empty(), "value should not be null");
         // Java: this.setValue(value);
         unit.setValue(value);
         unit
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
         // Java: return (this.mMagnitude == null ? "mMagnitude == null" : this.mMagnitude.toString()) + "  " + (this.mUnit == null ? "mUnit == null" : this.mUnit.toString());
         let mag_str = match self.mMagnitude {
             None => "mMagnitude == null".to_string(),
             Some(v) => v.to_string(),
         };
         format!("{}  {:?}", mag_str, self.mUnit)
    }

    // Java: static Unit convert2Unit(String unit)
    pub fn convert2Unit(unit: &str) -> Option<CssUnitKind> {
         // Java: if (null != unit && unit.length() > 0)
         if !unit.is_empty() {
            // Java: Unit mUnit = null;
            let mut m_unit: Option<CssUnitKind> = None;
            // Java: if ("auto".equalsIgnoreCase(unit))
            if unit.eq_ignore_ascii_case("auto") {
               // Java: mUnit = CssStyle.CssUnit.Unit.AUTO;
               m_unit = Some(CssUnitKind::AUTO);
            }
            // Java: else if ("mm".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("mm") {
               // Java: mUnit = CssStyle.CssUnit.Unit.MM;
               m_unit = Some(CssUnitKind::MM);
            }
            // Java: else if ("cm".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("cm") {
               // Java: mUnit = CssStyle.CssUnit.Unit.CM;
               m_unit = Some(CssUnitKind::CM);
            }
            // Java: else if ("in".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("in") {
               // Java: mUnit = CssStyle.CssUnit.Unit.IN;
               m_unit = Some(CssUnitKind::IN);
            }
            // Java: else if ("pt".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("pt") {
               // Java: mUnit = CssStyle.CssUnit.Unit.PT;
               m_unit = Some(CssUnitKind::PT);
            }
            // Java: else if ("pc".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("pc") {
               // Java: mUnit = CssStyle.CssUnit.Unit.PC;
               m_unit = Some(CssUnitKind::PC);
            }
            // Java: else if ("pi".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("pi") {
               // Java: mUnit = CssStyle.CssUnit.Unit.PI;
               m_unit = Some(CssUnitKind::PI);
            }
            // Java: else if ("px".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("px") {
               // Java: mUnit = CssStyle.CssUnit.Unit.PX;
               m_unit = Some(CssUnitKind::PX);
            }
            // Java: else if ("em".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("em") {
               // Java: mUnit = CssStyle.CssUnit.Unit.EM;
               m_unit = Some(CssUnitKind::EM);
            }
            // Java: else if ("ex".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("ex") {
               // Java: mUnit = CssStyle.CssUnit.Unit.EX;
               m_unit = Some(CssUnitKind::EX);
            }
            // Java: else if ("%".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("%") {
               // Java: mUnit = CssStyle.CssUnit.Unit.PERCENTAGE;
               m_unit = Some(CssUnitKind::PERCENTAGE);
            }
            // Java: else if ("f".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("f") {
               // Java: mUnit = CssStyle.CssUnit.Unit.FRACTION;
               m_unit = Some(CssUnitKind::FRACTION);
            }
            // Java: else if ("gd".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("gd") {
               // Java: mUnit = CssStyle.CssUnit.Unit.GD;
               m_unit = Some(CssUnitKind::GD);
            }
            // Java: else if ("emu".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("emu") {
               // Java: mUnit = CssStyle.CssUnit.Unit.EMU;
               m_unit = Some(CssUnitKind::EMU);
            }
            // Java: return mUnit;
            m_unit
         }
         // Java: else
         else {
            // Java: return null;
            None
         }
    }

    // Java: void setUnit(String unit)
    pub fn setUnit(&mut self, unit: &str) {
         // Java: Assert.assertNotNull("unit should not be true", unit);
         assert!(!unit.is_empty() || unit.is_empty(), "unit should not be null");
         // Java: unit = unit.trim();
         let unit = unit.trim();
         // Java: if (null != unit && unit.length() > 0)
         if !unit.is_empty() {
            // Java: this.mUnit = convert2Unit(unit);
            if let Some(u) = Self::convert2Unit(unit) {
                self.mUnit = u;
            }
            // Java: if (this.mUnit == null)
            else {
               // Java: this.mUnit = CssStyle.CssUnit.Unit.PX;
               self.mUnit = CssUnitKind::PX;
            }
         }
    }

    // Java: float getTwips()
    pub fn getTwips(&self) -> f32 {
         // Java: float pt = this.getPt();
         let pt = self.getPt();
         // Java: return MetricUtility.Points2Twips(pt);
         pt * 20.0
    }

    // Java: float getPt()
    pub fn getPt(&self) -> f32 {
         // Java: float ret = 0.0F;
         let mut ret = 0.0f32;
         // Java: if (null != this.mMagnitude && null != this.mUnit)
         if let Some(magnitude) = self.mMagnitude {
            // Java: switch (this.mUnit)
            match self.mUnit {
               // Java: case IN:
               CssUnitKind::IN => {
                  // Java: ret = MetricUtility.Inches2Points(this.mMagnitude);
                  ret = magnitude * 72.0;
                  // Java: break;
               }
               // Java: case PX:
               CssUnitKind::PX => {
                  // Java: ret = MetricUtility.pixels2Points(this.mMagnitude, Platform.getDisplayMetrics().densityDpi);
                  ret = (magnitude / 96.0) * 72.0;
                  // Java: break;
               }
               // Java: case CM:
               CssUnitKind::CM => {
                  // Java: ret = MetricUtility.CM2Points(this.mMagnitude);
                  ret = (magnitude / 2.54) * 72.0;
                  // Java: break;
               }
               // Java: case MM:
               CssUnitKind::MM => {
                  // Java: IOAssert.logNotNull("cssUnit.mMagnitude should not be null", this.mMagnitude);
                  // Java: ret = null != this.mMagnitude ? MetricUtility.MM2Points(this.mMagnitude) : 0.0F;
                  ret = (magnitude / 10.0 / 2.54) * 72.0;
                  // Java: break;
               }
               // Java: case EMU:
               CssUnitKind::EMU => {
                  // Java: ret = MetricUtility.EMU2Points(this.mMagnitude);
                  ret = (magnitude / 914400.0) * 72.0;
                  // Java: break;
               }
               // Java: default:
               _ => {
                  // Java: ret = this.mMagnitude;
                  ret = magnitude;
               }
            }
            // Java: return ret;
            ret
         }
         // Java: else
         else {
            // Java: return ret;
            ret
         }
    }

    // Java: static boolean checkFormat(String value)
    pub fn checkFormat(value: &str) -> bool {
         // Java: value = cn.wps.base.io.Unit.trim(value);
         let value = Self::trim_unit_value(value);
         // Java: if (null != value && value.length() > 0)
         if let Some(ref value) = value {
             if !value.is_empty() {
                // Java: int i = StringUtil.findLastDigitFromEnd(value);
                let i = Self::findLastDigitFromEnd(value);
                // Java: if (i < 0)
                if i < 0 {
                   // Java: return false;
                   return false;
                }
                // Java: else
                else {
                   // Java: String magnitude = value.substring(0, i + 1);
                   let idx = (i + 1) as usize;
                   let magnitude = &value[..idx];
                   // Java: if (magnitude != null && magnitude.length() > 0)
                   if !magnitude.is_empty() {
                      // Java: String unit = value.substring(i + 1);
                      let unit = &value[idx..];
                      // Java: if (unit == null)
                      // Java: return false;
                      // Java: else
                      // Java: return convert2Unit(unit) != null && TypeConversion.string2Float(magnitude) != null;
                      return Self::convert2Unit(unit).is_some()
                          && magnitude.parse::<f32>().is_ok();
                   }
                   // Java: else
                   else {
                      // Java: return false;
                      return false;
                   }
                }
             }
         }
         // Java: else
         // Java: return false;
         false
    }

    pub fn setValue(&mut self, value: &str) {
        let value = Self::trim_unit_value(value);
        if let Some(ref value) = value {
            if !value.is_empty() {
                let i = Self::findLastDigitFromEnd(value);
                if i < 0 {
                    return;
                }
                let idx = (i + 1) as usize;
                let magnitude = &value[..idx];
                if !magnitude.is_empty() {
                    self.mMagnitude = magnitude.parse::<f32>().ok();
                }
                let unit = &value[idx..];
                self.setUnit(unit);
            }
        }
    }

    pub fn trim_unit_value(value: &str) -> Option<String> {
        let value = value.trim();
        let parts: Vec<&str> = value.split(&[' ', ','][..]).collect();
        for part in &parts {
            if !part.is_empty() {
                return Some(part.to_string());
            }
        }
        None
    }

    fn findLastDigitFromEnd(value: &str) -> i32 {
        let bytes = value.as_bytes();
        for i in (0..bytes.len()).rev() {
            if bytes[i].is_ascii_digit() || bytes[i] == b'.' || bytes[i] == b'-' || bytes[i] == b'+' {
                return i as i32;
            }
        }
        -1
    }
}

// Java: public enum Unit
impl CssUnitKind {
    // Java: Unit()
    pub fn new() -> Self {
        CssUnitKind::PX
    }
}

// Java: public class AngleUnit
impl AngleUnit {
    // Java: AngleUnit()
    pub fn new() -> Self {
         // Java: super();
         // Java: this.mUnit = CssStyle.AngleUnit.Unit.DEGREE;
         // Java: this.mMagnitude = null;
         AngleUnit {
             mUnit: AngleUnitKind::DEGREE,
             mMagnitude: None,
         }
    }

    // Java: AngleUnit(String value)
    pub fn new_from_value(value: &str) -> Self {
         // Java: this();
         let mut unit = Self::new();
         // Java: Assert.assertNotNull("value should not be null", value);
         assert!(!value.is_empty(), "value should not be null");
         // Java: this.setValue(value);
         unit.setValue(value);
         unit
    }

    // Java: void setUnit(String unit)
    pub fn setUnit(&mut self, unit: &str) {
         // Java: Assert.assertNotNull("unit should not be true", unit);
         // Java: unit = unit.trim();
         let unit = unit.trim();
         // Java: if (null != unit && unit.length() > 0)
         if !unit.is_empty() {
            // Java: if ("fd".equalsIgnoreCase(unit))
            if unit.eq_ignore_ascii_case("fd") {
               // Java: this.mUnit = CssStyle.AngleUnit.Unit.FD;
               self.mUnit = AngleUnitKind::FD;
            }
            // Java: else if ("deg".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("deg") {
               // Java: this.mUnit = CssStyle.AngleUnit.Unit.DEGREE;
               self.mUnit = AngleUnitKind::DEGREE;
            }
            // Java: else if ("rad".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("rad") {
               // Java: this.mUnit = CssStyle.AngleUnit.Unit.RADIANT;
               self.mUnit = AngleUnitKind::RADIANT;
            }
            // Java: else if ("f".equalsIgnoreCase(unit))
            else if unit.eq_ignore_ascii_case("f") {
               // Java: this.mUnit = CssStyle.AngleUnit.Unit.F;
               self.mUnit = AngleUnitKind::F;
            }
            // Java: else
            else {
               // Java: Assert.fail("unreognized angel unit type is met: " + unit);
               panic!("unreognized angel unit type is met: {}", unit);
            }
         }
         // Java: else
         else {
            // Java: this.mUnit = CssStyle.AngleUnit.Unit.DEGREE;
            self.mUnit = AngleUnitKind::DEGREE;
         }
    }

    pub fn setValue(&mut self, value: &str) {
        let value = CssUnit::trim_unit_value(value);
        if let Some(ref value) = value {
            if !value.is_empty() {
                let i = CssUnit::findLastDigitFromEnd(value);
                if i < 0 {
                    return;
                }
                let idx = (i + 1) as usize;
                let magnitude = &value[..idx];
                if !magnitude.is_empty() {
                    self.mMagnitude = magnitude.parse::<f32>().ok();
                }
                let unit = &value[idx..];
                self.setUnit(unit);
            }
        }
    }
}

// Java: public enum Unit
impl AngleUnitKind {
    // Java: Unit()
    pub fn new() -> Self {
        AngleUnitKind::DEGREE
    }
}
