// Java: import cn.wps.base.assertion.Assert
use util::cn::wps::core::runtime::Assert;
// Java: import cn.wps.base.assertion.IOAssert
use util::cn::wps::base::assertion::IOAssert;
// Java: import cn.wps.base.io.MetricUtility
use util::cn::wps::base::io::MetricUtility;
// Java: import cn.wps.base.io.TypeConversion
use util::cn::wps::base::io::TypeConversion;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::core::runtime::Platform;
// Java: import cn.wps.moffice.util.StringUtil
use util::org::apache::poi::util::StringUtil;

// Java: public class CssStyle
pub struct CssStyle {
}

// Java: public class CssUnit
pub struct CssUnit {
    // Java field: Unit mUnit
    pub(crate) mUnit: Arc<Mutex<Unit>>,
}

// Java: public enum Unit
pub struct Unit {
}

// Java: public class AngleUnit
pub struct AngleUnit {
    // Java field: Unit mUnit
    pub(crate) mUnit: Arc<Mutex<Unit>>,
}

// Java: public enum Unit
pub struct Unit {
}

// Java: public class CssStyle
impl CssStyle {
    // Java: CssStyle()
    pub fn CssStyle(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getFontSize()
    pub fn getFontSize(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract String getFontFamily()
    pub fn getFontFamily(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getFont()
    pub fn getFont(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract FontStyle getFontStyle()
    pub fn getFontStyle(&self) -> FontStyle {
    // TODO_RUST_CODE
    }

    // Java: abstract FontVariant getFontVariant()
    pub fn getFontVariant(&self) -> FontVariant {
    // TODO_RUST_CODE
    }

    // Java: abstract FontWeight getFontWeight()
    pub fn getFontWeight(&self) -> FontWeight {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getMsoTextShadow()
    pub fn getMsoTextShadow(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract TextDecoration getTextDecoration()
    pub fn getTextDecoration(&self) -> TextDecoration {
    // TODO_RUST_CODE
    }

    // Java: abstract Flip getFlip()
    pub fn getFlip(&self) -> Flip {
    // TODO_RUST_CODE
    }

    // Java: abstract WrapStyle getWrapStyle()
    pub fn getWrapStyle(&self) -> WrapStyle {
    // TODO_RUST_CODE
    }

    // Java: abstract Position getPosition()
    pub fn getPosition(&self) -> Position {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getWidth()
    pub fn getWidth(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getHeight()
    pub fn getHeight(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getLeft()
    pub fn getLeft(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getTop()
    pub fn getTop(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMarginTop()
    pub fn getMarginTop(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMarginBottom()
    pub fn getMarginBottom(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMarginLeft()
    pub fn getMarginLeft(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMarginRight()
    pub fn getMarginRight(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract PositionH getPositionH()
    pub fn getPositionH(&self) -> PositionH {
    // TODO_RUST_CODE
    }

    // Java: abstract PositionV getPositionV()
    pub fn getPositionV(&self) -> PositionV {
    // TODO_RUST_CODE
    }

    // Java: abstract PositionHRel getPositionHRel()
    pub fn getPositionHRel(&self) -> PositionHRel {
    // TODO_RUST_CODE
    }

    // Java: abstract PositionVRel getPositionVRel()
    pub fn getPositionVRel(&self) -> PositionVRel {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getWrapDistanceLeft()
    pub fn getWrapDistanceLeft(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getWrapDistanceTop()
    pub fn getWrapDistanceTop(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getWrapDistanceRight()
    pub fn getWrapDistanceRight(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getWrapDistanceBottom()
    pub fn getWrapDistanceBottom(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract Visibility getVisibility()
    pub fn getVisibility(&self) -> Visibility {
    // TODO_RUST_CODE
    }

    // Java: abstract AngleUnit getRotation()
    pub fn getRotation(&self) -> AngleUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract Integer getZIndex()
    pub fn getZIndex(&self) -> Integer {
    // TODO_RUST_CODE
    }

    // Java: abstract String getLeftPercent()
    pub fn getLeftPercent(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getTopPercent()
    pub fn getTopPercent(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getWidthPercent()
    pub fn getWidthPercent(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getHeightPercent()
    pub fn getHeightPercent(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract WidthRel getWidthRel()
    pub fn getWidthRel(&self) -> WidthRel {
    // TODO_RUST_CODE
    }

    // Java: abstract HeightRel getHeightRel()
    pub fn getHeightRel(&self) -> HeightRel {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getFitShapeToText()
    pub fn getFitShapeToText(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getFitTextToShape()
    pub fn getFitTextToShape(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract VerticalTextAnchor getVerticalTextAnchor()
    pub fn getVerticalTextAnchor(&self) -> VerticalTextAnchor {
    // TODO_RUST_CODE
    }

    // Java: abstract LayoutFlow getLayoutFlow()
    pub fn getLayoutFlow(&self) -> LayoutFlow {
    // TODO_RUST_CODE
    }

    // Java: abstract MsoLayoutFlowAlt getMsoLayoutFlowAlt()
    pub fn getMsoLayoutFlowAlt(&self) -> MsoLayoutFlowAlt {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getVRotateLetters()
    pub fn getVRotateLetters(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getVSameLetterHeights()
    pub fn getVSameLetterHeights(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoNextTextBox()
    pub fn getMsoNextTextBox(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract TextAlign getVTextAlign()
    pub fn getVTextAlign(&self) -> TextAlign {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getVTextKern()
    pub fn getVTextKern(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getVTextReverse()
    pub fn getVTextReverse(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract TextSpacingMode getVTextSpacingMode()
    pub fn getVTextSpacingMode(&self) -> TextSpacingMode {
    // TODO_RUST_CODE
    }

    // Java: abstract PercentOrFractionUnit getVTextSpacing()
    pub fn getVTextSpacing(&self) -> PercentOrFractionUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getWrapEdited()
    pub fn getWrapEdited(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract HeightRule getMsoHeightRule()
    pub fn getMsoHeightRule(&self) -> HeightRule {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoBidiFontSize()
    pub fn getMsoBidiFontSize(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoAsciiFontFamily()
    pub fn getMsoAsciiFontFamily(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoFareastFontFamily()
    pub fn getMsoFareastFontFamily(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoHansiFontFamily()
    pub fn getMsoHansiFontFamily(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoBidiFontFamily()
    pub fn getMsoBidiFontFamily(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getColor()
    pub fn getColor(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoHide()
    pub fn getMsoHide(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getDisplay()
    pub fn getDisplay(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getBackground()
    pub fn getBackground(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoPattern()
    pub fn getMsoPattern(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getBackgroundColor()
    pub fn getBackgroundColor(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract TextTransfrom getTextTransform()
    pub fn getTextTransform(&self) -> TextTransfrom {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getLetterSpacing()
    pub fn getLetterSpacing(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoFontKerning()
    pub fn getMsoFontKerning(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract LayoutGridMode getLayoutGridMode()
    pub fn getLayoutGridMode(&self) -> LayoutGridMode {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getMsoNoProof()
    pub fn getMsoNoProof(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoAnsiLanguage()
    pub fn getMsoAnsiLanguage(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoFareastLanguage()
    pub fn getMsoFareastLanguage(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoBidiLanguage()
    pub fn getMsoBidiLanguage(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract VerticalAlign getVerticalAlign()
    pub fn getVerticalAlign(&self) -> VerticalAlign {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoFontWidth()
    pub fn getMsoFontWidth(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract TextEffect getTextEffect()
    pub fn getTextEffect(&self) -> TextEffect {
    // TODO_RUST_CODE
    }

    // Java: abstract String getTextShadow()
    pub fn getTextShadow(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract TextAnimation getTextAnimation()
    pub fn getTextAnimation(&self) -> TextAnimation {
    // TODO_RUST_CODE
    }

    // Java: abstract FontEmphasize getFontEmphasize()
    pub fn getFontEmphasize(&self) -> FontEmphasize {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoTextRaise()
    pub fn getMsoTextRaise(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract String getTextUnderline()
    pub fn getTextUnderline(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract TextLineThrough getTextLineThrough()
    pub fn getTextLineThrough(&self) -> TextLineThrough {
    // TODO_RUST_CODE
    }

    // Java: abstract GutterPosition getMsoGutterPosition()
    pub fn getMsoGutterPosition(&self) -> GutterPosition {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoBorderColorAlt()
    pub fn getMsoBorderColorAlt(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoBorderWidthAlt()
    pub fn getMsoBorderWidthAlt(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoBorderTopAlt()
    pub fn getMsoBorderTopAlt(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoBorderLeftAlt()
    pub fn getMsoBorderLeftAlt(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoBorderBottomAlt()
    pub fn getMsoBorderBottomAlt(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoBorderRightAlt()
    pub fn getMsoBorderRightAlt(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getBorderColor()
    pub fn getBorderColor(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getBorderWidth()
    pub fn getBorderWidth(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract String getBorderTop()
    pub fn getBorderTop(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getBorderLeft()
    pub fn getBorderLeft(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getBorderRight()
    pub fn getBorderRight(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getBorderBottom()
    pub fn getBorderBottom(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoBorderAlt()
    pub fn getMsoBorderAlt(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getBorder()
    pub fn getBorder(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getMsoBorderShadow()
    pub fn getMsoBorderShadow(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract TextAlign getTextAlign()
    pub fn getTextAlign(&self) -> TextAlign {
    // TODO_RUST_CODE
    }

    // Java: abstract TextJustify getTextJustify()
    pub fn getTextJustify(&self) -> TextJustify {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getTextKashida()
    pub fn getTextKashida(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getMsoMarginTopAlt()
    pub fn getMsoMarginTopAlt(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getMsoMarginBottomAlt()
    pub fn getMsoMarginBottomAlt(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoParaMarginTop()
    pub fn getMsoParaMarginTop(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoParaMarginBottom()
    pub fn getMsoParaMarginBottom(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getTextIndent()
    pub fn getTextIndent(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoCharIndentCount()
    pub fn getMsoCharIndentCount(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoParaMarginRight()
    pub fn getMsoParaMarginRight(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoParaMarginLeft()
    pub fn getMsoParaMarginLeft(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract String getLineHeight()
    pub fn getLineHeight(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract LineHeightRule getMsoLineHeightRule()
    pub fn getMsoLineHeightRule(&self) -> LineHeightRule {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoOutlineLevel()
    pub fn getMsoOutlineLevel(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getLayoutGridAlign()
    pub fn getLayoutGridAlign(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getMsoMirrorIndents()
    pub fn getMsoMirrorIndents(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract String getTabStops()
    pub fn getTabStops(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoPagination()
    pub fn getMsoPagination(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getPageBreakAfter()
    pub fn getPageBreakAfter(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoLineBreakOverride()
    pub fn getMsoLineBreakOverride(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getPunctuationWrap()
    pub fn getPunctuationWrap(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getWordBreak()
    pub fn getWordBreak(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getPunctuationTrim()
    pub fn getPunctuationTrim(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getTextAutospace()
    pub fn getTextAutospace(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoStyleName()
    pub fn getMsoStyleName(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoStyleParent()
    pub fn getMsoStyleParent(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getTextFit()
    pub fn getTextFit(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getMsoStyleQFormat()
    pub fn getMsoStyleQFormat(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoHeaderMargin()
    pub fn getMsoHeaderMargin(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoFooterMargin()
    pub fn getMsoFooterMargin(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract String getLayoutGrid()
    pub fn getLayoutGrid(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getSize()
    pub fn getSize(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMargin()
    pub fn getMargin(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoRotate()
    pub fn getMsoRotate(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract PageOrientation getMsoPageOrientation()
    pub fn getMsoPageOrientation(&self) -> PageOrientation {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoGutterMargin()
    pub fn getMsoGutterMargin(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract Vjc getMsoVerticalPageAlign()
    pub fn getMsoVerticalPageAlign(&self) -> Vjc {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoLineNumbersStart()
    pub fn getMsoLineNumbersStart(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoLineNumbersCountBy()
    pub fn getMsoLineNumbersCountBy(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoLineNumbersDistance()
    pub fn getMsoLineNumbersDistance(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract SectionOperand getMsoLineNumbersRestart()
    pub fn getMsoLineNumbersRestart(&self) -> SectionOperand {
    // TODO_RUST_CODE
    }

    // Java: abstract SectionBKC getPageBreakBefore()
    pub fn getPageBreakBefore(&self) -> SectionBKC {
    // TODO_RUST_CODE
    }

    // Java: abstract BreakType getMsoBreakType()
    pub fn getMsoBreakType(&self) -> BreakType {
    // TODO_RUST_CODE
    }

    // Java: abstract String getClear()
    pub fn getClear(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getMsoFacingPages()
    pub fn getMsoFacingPages(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getMsoTitlePage()
    pub fn getMsoTitlePage(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract Boolean getMsoMirrorMargins()
    pub fn getMsoMirrorMargins(&self) -> Boolean {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoTabCount()
    pub fn getMsoTabCount(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit[] getMsoPaddingAlt()
    pub fn getMsoPaddingAlt(&self) -> CssUnit[] {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit[] getPadding()
    pub fn getPadding(&self) -> CssUnit[] {
    // TODO_RUST_CODE
    }

    // Java: abstract PageBreakInside getPageBreakInside()
    pub fn getPageBreakInside(&self) -> PageBreakInside {
    // TODO_RUST_CODE
    }

    // Java: abstract TableAnchorHorizontal getMsoTableAnchorHorizontal()
    pub fn getMsoTableAnchorHorizontal(&self) -> TableAnchorHorizontal {
    // TODO_RUST_CODE
    }

    // Java: abstract TableAnchorVertical getMsoTableAnchorVertical()
    pub fn getMsoTableAnchorVertical(&self) -> TableAnchorVertical {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoTableRSpace()
    pub fn getMsoTableRSpace(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoTableLSpace()
    pub fn getMsoTableLSpace(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoTableTSpace()
    pub fn getMsoTableTSpace(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoTableBSpace()
    pub fn getMsoTableBSpace(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoRowMarginLeft()
    pub fn getMsoRowMarginLeft(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getMsoRowMarginRight()
    pub fn getMsoRowMarginRight(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract Object getMsoTableTop()
    pub fn getMsoTableTop(&self) -> Object {
    // TODO_RUST_CODE
    }

    // Java: abstract Object getMsoTableLeft()
    pub fn getMsoTableLeft(&self) -> Object {
    // TODO_RUST_CODE
    }

    // Java: abstract TableOverlap getMsoTableOverlap()
    pub fn getMsoTableOverlap(&self) -> TableOverlap {
    // TODO_RUST_CODE
    }

    // Java: abstract CssUnit getTabInterval()
    pub fn getTabInterval(&self) -> CssUnit {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoTStyleShading()
    pub fn getMsoTStyleShading(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract TableCondition getMsoTableCondition()
    pub fn getMsoTableCondition(&self) -> TableCondition {
    // TODO_RUST_CODE
    }

    // Java: abstract MsoElementType getMsoElementType()
    pub fn getMsoElementType(&self) -> MsoElementType {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoCommentReference()
    pub fn getMsoCommentReference(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoCommentContinuation()
    pub fn getMsoCommentContinuation(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoCommentDate()
    pub fn getMsoCommentDate(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoCommentAuthor()
    pub fn getMsoCommentAuthor(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract SpecialCharacterType getMsoSpecialCharacter()
    pub fn getMsoSpecialCharacter(&self) -> SpecialCharacterType {
    // TODO_RUST_CODE
    }

    // Java: abstract String getMsoBookmark()
    pub fn getMsoBookmark(&self) -> String {
    // TODO_RUST_CODE
    }

    // Java: abstract MsoTableLayoutAlt getMsoTableLayoutAlt()
    pub fn getMsoTableLayoutAlt(&self) -> MsoTableLayoutAlt {
    // TODO_RUST_CODE
    }

    // Java: abstract MsoCellSpecial getMsoCellSpecial()
    pub fn getMsoCellSpecial(&self) -> MsoCellSpecial {
    // TODO_RUST_CODE
    }

}

// Java: public class CssUnit
impl CssUnit {
    // Java: CssUnit()
    pub fn CssUnit(&self) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.mUnit = CssStyle.CssUnit.Unit.PX;
         // TODO_RUST_CODE
         // Java: this.mMagnitude = null;
         // TODO_RUST_CODE
    }

    // Java: CssUnit(String value)
    pub fn CssUnit(&self, String value) {
         // Java: this();
         // TODO_RUST_CODE
         // Java: Assert.assertNotNull("value should not be null", value);
         // TODO_RUST_CODE
         // Java: this.setValue(value);
         // TODO_RUST_CODE
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
         // Java: return (this.mMagnitude == null ? "mMagnitude == null" : this.mMagnitude.toString()) + "  " + (this.mUnit == null ? "mUnit == null" : this.mUnit.toString());
         // TODO_RUST_CODE
    }

    // Java: static Unit convert2Unit(String unit)
    pub fn convert2Unit(&self, String unit) -> Unit {
         // Java: if (null != unit && unit.length() > 0)
        // TODO_RUST_CODE
         {
            // Java: Unit mUnit = null;
            // TODO_RUST_CODE
            // Java: if ("auto".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.AUTO;
               // TODO_RUST_CODE
            }
            // Java: else if ("mm".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.MM;
               // TODO_RUST_CODE
            }
            // Java: else if ("cm".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.CM;
               // TODO_RUST_CODE
            }
            // Java: else if ("in".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.IN;
               // TODO_RUST_CODE
            }
            // Java: else if ("pt".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.PT;
               // TODO_RUST_CODE
            }
            // Java: else if ("pc".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.PC;
               // TODO_RUST_CODE
            }
            // Java: else if ("pi".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.PI;
               // TODO_RUST_CODE
            }
            // Java: else if ("px".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.PX;
               // TODO_RUST_CODE
            }
            // Java: else if ("em".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.EM;
               // TODO_RUST_CODE
            }
            // Java: else if ("ex".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.EX;
               // TODO_RUST_CODE
            }
            // Java: else if ("%".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.PERCENTAGE;
               // TODO_RUST_CODE
            }
            // Java: else if ("f".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.FRACTION;
               // TODO_RUST_CODE
            }
            // Java: else if ("gd".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.GD;
               // TODO_RUST_CODE
            }
            // Java: else if ("emu".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: mUnit = CssStyle.CssUnit.Unit.EMU;
               // TODO_RUST_CODE
            }
            // Java: return mUnit;
            // TODO_RUST_CODE
         }
         // Java: else
        // TODO_RUST_CODE
         {
            // Java: return null;
            // TODO_RUST_CODE
         }
    }

    // Java: void setUnit(String unit)
    pub fn setUnit(&self, String unit) {
         // Java: Assert.assertNotNull("unit should not be true", unit);
         // TODO_RUST_CODE
         // Java: unit = unit.trim();
         // TODO_RUST_CODE
         // Java: if (null != unit && unit.length() > 0)
        // TODO_RUST_CODE
         {
            // Java: this.mUnit = convert2Unit(unit);
            // TODO_RUST_CODE
            // Java: if (this.mUnit == null)
            // TODO_RUST_CODE
            {
               // Java: this.mUnit = CssStyle.CssUnit.Unit.PX;
               // TODO_RUST_CODE
            }
         }
    }

    // Java: float getTwips()
    pub fn getTwips(&self) -> f32 {
         // Java: float pt = this.getPt();
         // TODO_RUST_CODE
         // Java: return MetricUtility.Points2Twips(pt);
         // TODO_RUST_CODE
    }

    // Java: float getPt()
    pub fn getPt(&self) -> f32 {
         // Java: float ret = 0.0F;
         // TODO_RUST_CODE
         // Java: if (null != this.mMagnitude && null != this.mUnit)
        // TODO_RUST_CODE
         {
            // Java: switch (this.mUnit)
            // TODO_RUST_CODE
            {
               // Java: case IN:
               // TODO_RUST_CODE
                  // Java: ret = MetricUtility.Inches2Points(this.mMagnitude);
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               // Java: case PX:
               // TODO_RUST_CODE
                  // Java: ret = MetricUtility.pixels2Points(this.mMagnitude, Platform.getDisplayMetrics().densityDpi);
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               // Java: case CM:
               // TODO_RUST_CODE
                  // Java: ret = MetricUtility.CM2Points(this.mMagnitude);
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               // Java: case MM:
               // TODO_RUST_CODE
                  // Java: IOAssert.logNotNull("cssUnit.mMagnitude should not be null", this.mMagnitude);
                  // TODO_RUST_CODE
                  // Java: ret = null != this.mMagnitude ? MetricUtility.MM2Points(this.mMagnitude) : 0.0F;
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               // Java: case EMU:
               // TODO_RUST_CODE
                  // Java: ret = MetricUtility.EMU2Points(this.mMagnitude);
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               // Java: default:
               // TODO_RUST_CODE
                  // Java: ret = this.mMagnitude;
                  // TODO_RUST_CODE
            }
            // Java: return ret;
            // TODO_RUST_CODE
         }
         // Java: else
        // TODO_RUST_CODE
         {
            // Java: return ret;
            // TODO_RUST_CODE
         }
    }

    // Java: static boolean checkFormat(String value)
    pub fn checkFormat(&self, String value) -> bool {
         // Java: value = cn.wps.base.io.Unit.trim(value);
         // TODO_RUST_CODE
         // Java: if (null != value && value.length() > 0)
        // TODO_RUST_CODE
         {
            // Java: int i = StringUtil.findLastDigitFromEnd(value);
            // TODO_RUST_CODE
            // Java: if (i < 0)
            // TODO_RUST_CODE
            {
               // Java: return false;
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: String magnitude = value.substring(0, i + 1);
               // TODO_RUST_CODE
               // Java: if (magnitude != null && magnitude.length() > 0)
                // TODO_RUST_CODE
               {
                  // Java: String unit = value.substring(i + 1);
                  // TODO_RUST_CODE
                  // Java: if (unit == null)
                    // TODO_RUST_CODE
                  {
                     // Java: return false;
                     // TODO_RUST_CODE
                  }
                  // Java: else
                    // TODO_RUST_CODE
                  {
                     // Java: return convert2Unit(unit) != null && TypeConversion.string2Float(magnitude) != null;
                     // TODO_RUST_CODE
                  }
               }
               // Java: else
                // TODO_RUST_CODE
               {
                  // Java: return false;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: else
        // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
    }

}

// Java: public enum Unit
impl Unit {
    // Java: Unit()
    pub fn Unit(&self) {
    // TODO_RUST_CODE
    }

}

// Java: public class AngleUnit
impl AngleUnit {
    // Java: AngleUnit()
    pub fn AngleUnit(&self) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.mUnit = CssStyle.AngleUnit.Unit.DEGREE;
         // TODO_RUST_CODE
         // Java: this.mMagnitude = null;
         // TODO_RUST_CODE
    }

    // Java: AngleUnit(String value)
    pub fn AngleUnit(&self, String value) {
         // Java: this();
         // TODO_RUST_CODE
         // Java: Assert.assertNotNull("value should not be null", value);
         // TODO_RUST_CODE
         // Java: this.setValue(value);
         // TODO_RUST_CODE
    }

    // Java: void setUnit(String unit)
    pub fn setUnit(&self, String unit) {
         // Java: Assert.assertNotNull("unit should not be true", unit);
         // TODO_RUST_CODE
         // Java: unit = unit.trim();
         // TODO_RUST_CODE
         // Java: if (null != unit && unit.length() > 0)
        // TODO_RUST_CODE
         {
            // Java: if ("fd".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: this.mUnit = CssStyle.AngleUnit.Unit.FD;
               // TODO_RUST_CODE
            }
            // Java: else if ("deg".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: this.mUnit = CssStyle.AngleUnit.Unit.DEGREE;
               // TODO_RUST_CODE
            }
            // Java: else if ("rad".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: this.mUnit = CssStyle.AngleUnit.Unit.RADIANT;
               // TODO_RUST_CODE
            }
            // Java: else if ("f".equalsIgnoreCase(unit))
            // TODO_RUST_CODE
            {
               // Java: this.mUnit = CssStyle.AngleUnit.Unit.F;
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: Assert.fail("unreognized angel unit type is met: " + unit);
               // TODO_RUST_CODE
            }
         }
         // Java: else
        // TODO_RUST_CODE
         {
            // Java: this.mUnit = CssStyle.AngleUnit.Unit.DEGREE;
            // TODO_RUST_CODE
         }
    }

}

// Java: public enum Unit
impl Unit {
    // Java: Unit()
    pub fn Unit(&self) {
    // TODO_RUST_CODE
    }

}
