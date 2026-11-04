#![allow(non_snake_case, non_upper_case_globals)]

// Java: import cn.wps.base.io.MetricUtility
use core_libs_util::base::io::MetricUtility::MetricUtility;
// Java: import cn.wps.moffice.drawing.effects.ExtrusionPlane
use super::effects::ExtrusionPlane::ExtrusionPlane;
// Java: import cn.wps.moffice.drawing.effects.ShadowPos
use super::effects::ShadowPos::ShadowPos;
// Java: import cn.wps.moffice.drawing.graphics.TileFlipMode
use super::graphics::TileFlipMode::TileFlipMode;
use std::sync::LazyLock;

// Java: public class DefaultShapeValue
pub struct DefaultShapeValue {
}

// Java: public class DefaultShapeValue
impl DefaultShapeValue {
    // Java field: float shape_wrapDistLeft
    pub const shape_wrapDistLeft: f32 = 9.0;
    // Java field: float shape_wrapDistTop
    pub const shape_wrapDistTop: f32 = 0.0;
    // Java field: float shape_wrapDistRight
    pub const shape_wrapDistRight: f32 = 9.0;
    // Java field: float shape_wrapDistBottom
    pub const shape_wrapDistBottom: f32 = 0.0;
    // Java field: int zIndex
    pub const zIndex: i32 = 0;
    // Java field: boolean is08Shape
    pub const is08Shape: bool = true;
    // Java field: boolean rotWithShape
    pub const rotWithShape: bool = false;
    // Java field: boolean hidden
    pub const hidden: bool = false;
    // Java field: String shapeName
    pub const shapeName: Option<&'static str> = None;
    // Java field: int shapeID
    pub const shapeID: i32 = 0;
    // Java field: int shapeType
    pub const shapeType: i32 = 0;
    // Java field: String description
    pub const description: Option<&'static str> = None;
    // Java field: int textWrap
    pub const textWrap: i32 = 0;
    // Java field: int textWrapType
    pub const textWrapType: i32 = 0;
    // Java field: boolean bolowText
    pub const bolowText: bool = false;
    // Java field: boolean anchorLock
    pub const anchorLock: bool = false;
    // Java field: boolean behindDocument
    pub const behindDocument: bool = false;
    // Java field: boolean fInlineShape
    pub const fInlineShape: bool = false;
    // Java field: boolean fUseShapeAnchor
    pub const fUseShapeAnchor: bool = false;
    // Java field: boolean fRecolorFillAsPicture
    pub const fRecolorFillAsPicture: bool = false;
    // Java field: boolean fEditedWrap
    pub const fEditedWrap: bool = false;
    // Java field: boolean fIsBullet
    pub const fIsBullet: bool = false;
    // Java field: boolean fIsInTextBox
    pub const fIsInTextBox: bool = false;
    // Java field: boolean fGroup
    pub const fGroup: bool = false;
    // Java field: boolean fChild
    pub const fChild: bool = false;
    // Java field: boolean fPatriarch
    pub const fPatriarch: bool = false;
    // Java field: boolean fDeleted
    pub const fDeleted: bool = false;
    // Java field: boolean fOleShape
    pub const fOleShape: bool = false;
    // Java field: boolean fHaveMaster
    pub const fHaveMaster: bool = false;
    // Java field: boolean fFlipH
    pub const fFlipH: bool = false;
    // Java field: boolean fFlipV
    pub const fFlipV: bool = false;
    // Java field: boolean fConnector
    pub const fConnector: bool = false;
    // Java field: boolean fHaveAnchor
    pub const fHaveAnchor: bool = true;
    // Java field: boolean fBackground
    pub const fBackground: bool = false;
    // Java field: boolean fHaveSpt
    pub const fHaveSpt: bool = true;
    // Java field: boolean lockRotation
    pub const lockRotation: bool = false;
    // Java field: boolean lockAspectRatio
    pub const lockAspectRatio: bool = false;
    // Java field: boolean lockPosition
    pub const lockPosition: bool = false;
    // Java field: boolean lockAgainstSelect
    pub const lockAgainstSelect: bool = false;
    // Java field: boolean lockCropping
    pub const lockCropping: bool = false;
    // Java field: boolean lockVertices
    pub const lockVertices: bool = false;
    // Java field: boolean lockText
    pub const lockText: bool = false;
    // Java field: boolean lockAdjustHandles
    pub const lockAdjustHandles: bool = false;
    // Java field: boolean lockAgainstGrouping
    pub const lockAgainstGrouping: bool = false;
    // Java field: boolean lockAgainstUngrouping
    pub const lockAgainstUngrouping: bool = false;
    // Java field: float rotation
    pub const rotation: f32 = 0.0;
    // Java field: float scaleH
    pub const scaleH: f32 = 1.0;
    // Java field: float scaleV
    pub const scaleV: f32 = 1.0;
    // Java field: int posRelH
    pub const posRelH: i32 = 2;
    // Java field: int posRelV
    pub const posRelV: i32 = 2;
    // Java field: int relativeH
    pub const relativeH: i32 = 1;
    // Java field: int relativeV
    pub const relativeV: i32 = 1;
    // Java field: int alignmentH
    pub const alignmentH: i32 = 0;
    // Java field: int alignmentV
    pub const alignmentV: i32 = 0;
    // Java field: int sizeRelH
    pub const sizeRelH: i32 = 1;
    // Java field: int sizeRelV
    pub const sizeRelV: i32 = 1;
    // Java field: float pctHorizPos
    pub const pctHorizPos: f32 = 0.0;
    // Java field: float pctVertPos
    pub const pctVertPos: f32 = 0.0;
    // Java field: float pctHorizSize
    pub const pctHorizSize: f32 = 0.0;
    // Java field: float pctVertSize
    pub const pctVertSize: f32 = 0.0;
    // Java field: boolean allowOverlap
    pub const allowOverlap: bool = true;
    // Java field: boolean layoutInCell
    pub const layoutInCell: bool = true;
    // Java field: float geoLeft
    pub const geoLeft: f32 = 0.0;
    // Java field: float geoTop
    pub const geoTop: f32 = 0.0;
    // Java field: float geoRight
    pub const geoRight: f32 = 21600.0;
    // Java field: float geoBottom
    pub const geoBottom: f32 = 21600.0;
    // Java field: int geometryVerticesCbElem
    pub const geometryVerticesCbElem: i32 = 8;
    // Java field: boolean fShadowOK
    pub const fShadowOK: bool = true;
    // Java field: boolean f3DOK
    pub const f3DOK: bool = true;
    // Java field: boolean fLineOK
    pub const fLineOK: bool = true;
    // Java field: boolean fGtextOK
    pub const fGtextOK: bool = false;
    // Java field: boolean fFillShadeShapeOK
    pub const fFillShadeShapeOK: bool = false;
    // Java field: boolean fFillOK
    pub const fFillOK: bool = true;
    // Java field: boolean fColumnLineOK
    pub const fColumnLineOK: bool = false;
    // Java field: int limoX
    pub const limoX: i32 = i32::MIN;
    // Java field: int limoY
    pub const limoY: i32 = i32::MIN;
    // Java field: int connectionSitesType
    pub const connectionSitesType: i32 = 1;
    // Java field: int callout_type
    pub const callout_type: i32 = 3;
    // Java field: int callout_gap
    pub const callout_gap: i32 = 6;
    // Java field: int callout_angle
    pub const callout_angle: i32 = 0;
    // Java field: int callout_dropType
    pub const callout_dropType: i32 = 3;
    // Java field: float callout_dropSpecified
    pub const callout_dropSpecified: f32 = 9.0;
    // Java field: float callout_lengthSpecified
    pub const callout_lengthSpecified: f32 = 0.0;
    // Java field: boolean fCalloutAccentBar
    pub const fCalloutAccentBar: bool = false;
    // Java field: boolean fCalloutTextBorder
    pub const fCalloutTextBorder: bool = true;
    // Java field: boolean fCalloutMinusX
    pub const fCalloutMinusX: bool = false;
    // Java field: boolean fCalloutMinusY
    pub const fCalloutMinusY: bool = false;
    // Java field: boolean fCalloutDropAuto
    pub const fCalloutDropAuto: bool = false;
    // Java field: boolean fCalloutLengthSpecified
    pub const fCalloutLengthSpecified: bool = false;
    // Java field: boolean fCalloutOn
    pub const fCalloutOn: bool = false;
    // Java field: String gtextUNICODE
    pub const gtextUNICODE: Option<&'static str> = None;
    // Java field: String gtextRTF
    pub const gtextRTF: Option<&'static str> = None;
    // Java field: int gtextAlign
    pub const gtextAlign: i32 = 1;
    // Java field: float gtextSize
    pub const gtextSize: f32 = 36.0;
    // Java field: float gtextSpacing
    pub const gtextSpacing: f32 = 1.0;
    // Java field: String gtextFont
    pub const gtextFont: Option<&'static str> = None;
    // Java field: String gtextCSSFont
    pub const gtextCSSFont: Option<&'static str> = None;
    // Java field: boolean gtextFReverseRows
    pub const gtextFReverseRows: bool = false;
    // Java field: boolean gtextfGtext
    pub const gtextfGtext: bool = false;
    // Java field: boolean gtextFVertical
    pub const gtextFVertical: bool = false;
    // Java field: boolean gtextFKern
    pub const gtextFKern: bool = false;
    // Java field: boolean gtextFTight
    pub const gtextFTight: bool = false;
    // Java field: boolean gtextFStretch
    pub const gtextFStretch: bool = false;
    // Java field: boolean gtextFShrinkFit
    pub const gtextFShrinkFit: bool = false;
    // Java field: boolean gtextFBestFit
    pub const gtextFBestFit: bool = false;
    // Java field: boolean gtextFNormalize
    pub const gtextFNormalize: bool = false;
    // Java field: boolean gtextFDxMeasure
    pub const gtextFDxMeasure: bool = false;
    // Java field: boolean gtextFBold
    pub const gtextFBold: bool = false;
    // Java field: boolean gtextFItalic
    pub const gtextFItalic: bool = false;
    // Java field: boolean gtextFUnderline
    pub const gtextFUnderline: bool = false;
    // Java field: boolean gtextFShadow
    pub const gtextFShadow: bool = false;
    // Java field: boolean gtextFSmallcaps
    pub const gtextFSmallcaps: bool = false;
    // Java field: boolean gtextFStrikethrough
    pub const gtextFStrikethrough: bool = false;
    // Java field: int prstTextShapeType
    pub const prstTextShapeType: i32 = 0;
    // Java field: int fillType
    pub const fillType: i32 = 0;
    // Java field: float transparency
    pub const transparency: f32 = 0.0;
    // Java field: float backTransparency
    pub const backTransparency: f32 = 0.0;
    // Java field: String blipName
    pub const blipName: Option<&'static str> = None;
    // Java field: int mediaID
    pub const mediaID: i32 = -1;
    // Java field: int fillColor
    pub const fillColor: i32 = -1;
    // Java field: int fillBackColor
    pub const fillBackColor: i32 = -1;
    // Java field: float fillAngle
    pub const fillAngle: f32 = 0.0;
    // Java field: float fillFocus
    pub const fillFocus: f32 = 0.0;
    // Java field: float fillWidth
    pub const fillWidth: f32 = 0.0;
    // Java field: float fillHeight
    pub const fillHeight: f32 = 0.0;
    // Java field: int filldztype
    pub const filldztype: i32 = 0;
    // Java field: boolean fillShape
    pub const fillShape: bool = true;
    // Java field: int fillShadeType
    pub const fillShadeType: i32 = 3;
    // Java field: boolean fFillHitTest
    pub const fFillHitTest: bool = true;
    // Java field: boolean fFillOn
    pub const fFillOn: bool = true;
    // Java field: boolean fNoFillHitTest
    pub const fNoFillHitTest: bool = false;
    // Java field: boolean fGrpFill
    pub const fGrpFill: bool = false;
    // Java field: boolean fFilled
    pub const fFilled: bool = true;
    // Java field: float fillOriginX
    pub const fillOriginX: f32 = 0.0;
    // Java field: float fillOriginY
    pub const fillOriginY: f32 = 0.0;
    // Java field: float fillShapeOriginX
    pub const fillShapeOriginX: f32 = 0.0;
    // Java field: float fillShapeOriginY
    pub const fillShapeOriginY: f32 = 0.0;
    // Java field: boolean fillUserBgFill
    pub const fillUserBgFill: bool = false;
    // Java field: int fOverlayBlendMode
    pub const fOverlayBlendMode: i32 = 1;
    // Java field: float tileOffsetX
    pub const tileOffsetX: f32 = 0.0;
    // Java field: float tileOffsetY
    pub const tileOffsetY: f32 = 0.0;
    // Java field: float tileScaleX
    pub const tileScaleX: f32 = 1.0;
    // Java field: float tileScaleY
    pub const tileScaleY: f32 = 1.0;
    // Java field: int tileAlign
    pub const tileAlign: i32 = 7;
    // Java field: TileFlipMode tileFlipMode
    pub fn get_tileFlipMode() -> TileFlipMode { *tileFlipMode }
    // Java field: float cropTop
    pub const cropTop: f32 = 0.0;
    // Java field: float cropLeft
    pub const cropLeft: f32 = 0.0;
    // Java field: float cropBottom
    pub const cropBottom: f32 = 0.0;
    // Java field: float cropRight
    pub const cropRight: f32 = 0.0;
    // Java field: float picContrast
    pub const picContrast: f32 = 50.0;
    // Java field: float picBrightness
    pub const picBrightness: f32 = 50.0;
    // Java field: float picSoftenSharpen
    pub const picSoftenSharpen: f32 = 0.0;
    // Java field: float picSaturation
    pub const picSaturation: f32 = 1.0;
    // Java field: int picTransparent
    pub const picTransparent: i32 = -1;
    // Java field: float picTemperature
    pub const picTemperature: f32 = 6500.0;
    // Java field: int picID
    pub const picID: i32 = 0;
    // Java field: boolean picBiLevel
    pub const picBiLevel: bool = false;
    // Java field: int embosscolor
    pub const embosscolor: i32 = 16777215;
    // Java field: boolean picGray
    pub const picGray: bool = false;
    // Java field: float picGamma
    pub const picGamma: f32 = 1.0;
    // Java field: int pictureRecolor
    pub const pictureRecolor: i32 = -1;
    // Java field: boolean preferRelativeResize
    pub const preferRelativeResize: bool = false;
    // Java field: boolean LossyFormat
    pub const LossyFormat: bool = false;
    // Java field: float picBiLeveThresh
    pub const picBiLeveThresh: f32 = 0.5;
    // Java field: int noLetterPaper
    pub const noLetterPaper: i32 = -1;
    // Java field: int picArtistic
    pub const picArtistic: i32 = 0;
    // Java field: int artisticSize
    pub const artisticSize: i32 = 0;
    // Java field: int artisticTransparent
    pub const artisticTransparent: i32 = 0;
    // Java field: int artisticBlurSize
    pub const artisticBlurSize: i32 = 10;
    // Java field: int artisticMarkerSize
    pub const artisticMarkerSize: i32 = 97;
    // Java field: int artisticPencilGrayscaleSize
    pub const artisticPencilGrayscaleSize: i32 = 27;
    // Java field: int artisticPencilSketchPressure
    pub const artisticPencilSketchPressure: i32 = 22;
    // Java field: int artisticLineDrawingTransparent
    pub const artisticLineDrawingTransparent: i32 = 25;
    // Java field: int artisticLineDrawingSize
    pub const artisticLineDrawingSize: i32 = 0;
    // Java field: int artisticChalkSketchPressure
    pub const artisticChalkSketchPressure: i32 = 0;
    // Java field: int artisticPaintStrokesIntensity
    pub const artisticPaintStrokesIntensity: i32 = 5;
    // Java field: int artisticPaintBrushSize
    pub const artisticPaintBrushSize: i32 = 2;
    // Java field: int artisticGlowDiffusedIntensity
    pub const artisticGlowDiffusedIntensity: i32 = 5;
    // Java field: int artisticLightScreenSize
    pub const artisticLightScreenSize: i32 = 4;
    // Java field: int artisticWatercolorSpongeSize
    pub const artisticWatercolorSpongeSize: i32 = 2;
    // Java field: int artisticFilmGrainSize
    pub const artisticFilmGrainSize: i32 = 40;
    // Java field: int artisticMosiaicBubblesSize
    pub const artisticMosiaicBubblesSize: i32 = 14;
    // Java field: int artisticGlassSize
    pub const artisticGlassSize: i32 = 34;
    // Java field: int artisticCementSize
    pub const artisticCementSize: i32 = 24;
    // Java field: int artisticTexturizerSize
    pub const artisticTexturizerSize: i32 = 34;
    // Java field: int artisticCrisscrossEtchingTransparent
    pub const artisticCrisscrossEtchingTransparent: i32 = 75;
    // Java field: int artisticCrisscrossEtchingSize
    pub const artisticCrisscrossEtchingSize: i32 = 30;
    // Java field: int artisticPastelsSmoothSize
    pub const artisticPastelsSmoothSize: i32 = 34;
    // Java field: int artisticPlasticWrapSmoothness
    pub const artisticPlasticWrapSmoothness: i32 = 5;
    // Java field: int artisticCutoutShade
    pub const artisticCutoutShade: i32 = 2;
    // Java field: int artisticPhotocopyTransparent
    pub const artisticPhotocopyTransparent: i32 = 30;
    // Java field: int artisticPhotocopyDetail
    pub const artisticPhotocopyDetail: i32 = 3;
    // Java field: int artisticGlowEdgesTransparent
    pub const artisticGlowEdgesTransparent: i32 = 15;
    // Java field: int artisticGlowEdgesSmoothness
    pub const artisticGlowEdgesSmoothness: i32 = 3;
    // Java field: int lineColor
    pub const lineColor: i32 = 0;
    // Java field: int lineBackColor
    pub const lineBackColor: i32 = 16777215;
    // Java field: float lineTransparency
    pub const lineTransparency: f32 = 0.0;
    // Java field: boolean fLine
    pub const fLine: bool = true;
    // Java field: boolean fLineChild
    pub const fLineChild: bool = false;
    // Java field: float lineWidth
    pub const lineWidth: f32 = 0.75;
    // Java field: int lineType
    pub const lineType: i32 = 0;
    // Java field: int lineStyle
    pub const lineStyle: i32 = 0;
    // Java field: int lineDashing
    pub const lineDashing: i32 = 0;
    // Java field: int lineJoinStyle
    pub const lineJoinStyle: i32 = 2;
    // Java field: int lineEndCapStyle
    pub const lineEndCapStyle: i32 = 2;
    // Java field: int arrowStyle
    pub const arrowStyle: i32 = 0;
    // Java field: int arrowWidth
    pub const arrowWidth: i32 = 1;
    // Java field: int arrowLength
    pub const arrowLength: i32 = 1;
    // Java field: boolean fInsetPenOK
    pub const fInsetPenOK: bool = true;
    // Java field: boolean fInsetPen
    pub const fInsetPen: bool = false;
    // Java field: boolean fArrowHeadsOk
    pub const fArrowHeadsOk: bool = false;
    // Java field: boolean fNoLineDrawDash
    pub const fNoLineDrawDash: bool = false;
    // Java field: boolean fImageAlignShape
    pub const fImageAlignShape: bool = true;
    // Java field: int lineFillBlipFlags
    pub const lineFillBlipFlags: i32 = 0;
    // Java field: float lineMiterLimit
    pub const lineMiterLimit: f32 = 0.5;
    // Java field: float textbox_margin_left
    pub const textbox_margin_left: f32 = 7.2;
    // Java field: float textbox_margin_top
    pub const textbox_margin_top: f32 = 3.6;
    // Java field: float textbox_margin_right
    pub const textbox_margin_right: f32 = 7.2;
    // Java field: float textbox_margin_bottom
    pub const textbox_margin_bottom: f32 = 3.6;
    // Java field: int wrapText
    pub const wrapText: i32 = 0;
    // Java field: int textAnchorV
    pub const textAnchorV: i32 = 0;
    // Java field: int textFlow
    pub const textFlow: i32 = 0;
    // Java field: int textNext
    pub const textNext: i32 = -1;
    // Java field: boolean fitShape2Text
    pub const fitShape2Text: bool = false;
    // Java field: boolean fitText2Shape
    pub const fitText2Shape: bool = false;
    // Java field: boolean fTextRotate
    pub const fTextRotate: bool = true;
    // Java field: int textDirFont
    pub const textDirFont: i32 = 0;
    // Java field: boolean wrapOutShape
    pub const wrapOutShape: bool = false;
    // Java field: int textID
    pub const textID: i32 = 0;
    // Java field: int textLinkedID
    pub const textLinkedID: i32 = -1;
    // Java field: int textLinkedSeq
    pub const textLinkedSeq: i32 = 0;
    // Java field: int textDir
    pub const textDir: i32 = 0;
    // Java field: int shadowType
    pub const shadowType: i32 = 0;
    // Java field: int shadowColor
    pub const shadowColor: i32 = 8421504;
    // Java field: int shadowHighLight
    pub const shadowHighLight: i32 = 13355979;
    // Java field: float shadowOpacity
    pub const shadowOpacity: f32 = 1.0;
    // Java field: float shadowOffsetX
    pub const shadowOffsetX: f32 = 2.0;
    // Java field: float shadowOffsetY
    pub const shadowOffsetY: f32 = 2.0;
    // Java field: float shadowOffsetXFrac
    pub const shadowOffsetXFrac: f32 = 0.0;
    // Java field: float shadowOffsetYFrac
    pub const shadowOffsetYFrac: f32 = 0.0;
    // Java field: float shadowSecondOffsetX
    pub const shadowSecondOffsetX: f32 = 0.0;
    // Java field: float shadowSecondOffsetY
    pub const shadowSecondOffsetY: f32 = 0.0;
    // Java field: float shadowScaleXToX
    pub const shadowScaleXToX: f32 = 1.0;
    // Java field: float shadowScaleYToX
    pub const shadowScaleYToX: f32 = 0.0;
    // Java field: float shadowScaleXToY
    pub const shadowScaleXToY: f32 = 0.0;
    // Java field: float shadowScaleYToY
    pub const shadowScaleYToY: f32 = 1.0;
    // Java field: float shadowPerspectiveX
    pub const shadowPerspectiveX: f32 = 0.0;
    // Java field: float shadowPerspectiveY
    pub const shadowPerspectiveY: f32 = 0.0;
    // Java field: float shadowOriginX
    pub const shadowOriginX: f32 = 0.0;
    // Java field: float shadowOriginY
    pub const shadowOriginY: f32 = 0.0;
    // Java field: float shadowWeight
    pub const shadowWeight: f32 = 1.0;
    // Java field: float shadowBlurRadius
    pub const shadowBlurRadius: f32 = 0.0;
    // Java field: boolean fshadowObscured
    pub const fshadowObscured: bool = false;
    // Java field: boolean fshadowOn
    pub const fshadowOn: bool = false;
    // Java field: int shadowCrMod
    pub const shadowCrMod: i32 = 536870912;
    // Java field: int shadowColorExt
    pub const shadowColorExt: i32 = -1;
    // Java field: int shadowColorExtMod
    pub const shadowColorExtMod: i32 = 536870912;
    // Java field: int shadowHighlightExt
    pub const shadowHighlightExt: i32 = -1;
    // Java field: int shadowHighlightMod
    pub const shadowHighlightMod: i32 = 536870912;
    // Java field: int shadowSoftness
    pub const shadowSoftness: i32 = 0;
    // Java field: int pictureDblCrMod
    pub const pictureDblCrMod: i32 = 268435700;
    // Java field: float shadowDir
    pub const shadowDir: f32 = 0.0;
    // Java field: float shadowDist
    pub const shadowDist: f32 = 0.0;
    // Java field: boolean shadowRotWithShape
    pub const shadowRotWithShape: bool = true;
    // Java field: int shadowAlgn
    pub const shadowAlgn: i32 = 0;
    // Java field: ShadowPos shadowPos
    pub fn get_shadowPos() -> ShadowPos { *shadowPos }
    // Java field: int reflectionAlgn
    pub const reflectionAlgn: i32 = 0;
    // Java field: float reflectionDir
    pub const reflectionDir: f32 = 0.0;
    // Java field: float reflectionDist
    pub const reflectionDist: f32 = 0.0;
    // Java field: float reflectionEndAlpha
    pub const reflectionEndAlpha: f32 = 0.0;
    // Java field: float reflectionEndPos
    pub const reflectionEndPos: f32 = 1.0;
    // Java field: float reflectionStartAlpha
    pub const reflectionStartAlpha: f32 = 1.0;
    // Java field: float reflectionStartPos
    pub const reflectionStartPos: f32 = 0.0;
    // Java field: float reflectionFadeDir
    pub const reflectionFadeDir: f32 = 90.0;
    // Java field: float reflectionSkewX
    pub const reflectionSkewX: f32 = 0.0;
    // Java field: float reflectionSkewY
    pub const reflectionSkewY: f32 = 0.0;
    // Java field: boolean reflectionRotWithShape
    pub const reflectionRotWithShape: bool = true;
    // Java field: float reflectionScaleX
    pub const reflectionScaleX: f32 = 1.0;
    // Java field: float reflectionScaleY
    pub const reflectionScaleY: f32 = 1.0;
    // Java field: float reflectionBlurRad
    pub const reflectionBlurRad: f32 = 0.0;
    // Java field: int perspectiveType
    pub const perspectiveType: i32 = 1;
    // Java field: float perspectiveOffsetX
    pub const perspectiveOffsetX: f32 = 0.0;
    // Java field: float perspectiveOffsetY
    pub const perspectiveOffsetY: f32 = 0.0;
    // Java field: float perspectiveScaleXToX
    pub const perspectiveScaleXToX: f32 = 1.0;
    // Java field: float perspectiveScaleYToX
    pub const perspectiveScaleYToX: f32 = 0.0;
    // Java field: float perspectiveScaleXToY
    pub const perspectiveScaleXToY: f32 = 0.0;
    // Java field: float perspectiveScaleYToY
    pub const perspectiveScaleYToY: f32 = 1.0;
    // Java field: float perspectivePerspectiveX
    pub const perspectivePerspectiveX: f32 = 0.0;
    // Java field: float perspectivePerspectiveY
    pub const perspectivePerspectiveY: f32 = 0.0;
    // Java field: float perspectiveOriginX
    pub const perspectiveOriginX: f32 = 1.0;
    // Java field: float perspectiveOriginY
    pub const perspectiveOriginY: f32 = 1.0;
    // Java field: float perspectiveWeight
    pub const perspectiveWeight: f32 = 1.0;
    // Java field: float specularAmt
    pub const specularAmt: f32 = 0.0;
    // Java field: float diffuseAmt
    pub const diffuseAmt: f32 = 1.0;
    // Java field: float shininess
    pub const shininess: f32 = 5.0;
    // Java field: float edgeThickness
    pub const edgeThickness: f32 = 1.0;
    // Java field: float extrudeForward
    pub const extrudeForward: f32 = 0.0;
    // Java field: float extrudeBackward
    pub const extrudeBackward: f32 = 36.0;
    // Java field: ExtrusionPlane extrudePlane
    pub fn get_extrudePlane() -> ExtrusionPlane { *extrudePlane }
    // Java field: int extrusionColor
    pub const extrusionColor: i32 = 0;
    // Java field: boolean f3DMetallic
    pub const f3DMetallic: bool = false;
    // Java field: boolean f3DUseExtrusionColor
    pub const f3DUseExtrusionColor: bool = false;
    // Java field: boolean f3DLightFace
    pub const f3DLightFace: bool = true;
    // Java field: boolean f3D
    pub const f3D: bool = false;
    // Java field: float c3DXRotationAngle
    pub const c3DXRotationAngle: f32 = 0.0;
    // Java field: float c3DYRotationAngle
    pub const c3DYRotationAngle: f32 = 0.0;
    // Java field: float c3DRotationAngle
    pub const c3DRotationAngle: f32 = 0.0;
    // Java field: float c3DRotationAxisX
    pub const c3DRotationAxisX: f32 = 100.0;
    // Java field: float c3DRotationAxisY
    pub const c3DRotationAxisY: f32 = 0.0;
    // Java field: float c3DRotationAxisZ
    pub const c3DRotationAxisZ: f32 = 0.0;
    // Java field: float c3DRotationCenterX
    pub const c3DRotationCenterX: f32 = 0.0;
    // Java field: float c3DRotationCenterY
    pub const c3DRotationCenterY: f32 = 0.0;
    // Java field: float c3DRotationCenterZ
    pub const c3DRotationCenterZ: f32 = 0.0;
    // Java field: int c3DRenderMode
    pub const c3DRenderMode: i32 = 0;
    // Java field: float c3DTolerance
    pub const c3DTolerance: f32 = 30000.0;
    // Java field: float c3DXViewpoint
    pub fn get_c3DXViewpoint() -> f32 { *c3DXViewpoint }
    // Java field: float c3DYViewpoint
    pub fn get_c3DYViewpoint() -> f32 { *c3DYViewpoint }
    // Java field: float c3DZViewpoint
    pub fn get_c3DZViewpoint() -> f32 { *c3DZViewpoint }
    // Java field: float c3DOriginX
    pub const c3DOriginX: f32 = 0.5;
    // Java field: float c3DOriginY
    pub const c3DOriginY: f32 = -0.5;
    // Java field: float c3DSkewAngle
    pub const c3DSkewAngle: f32 = 225.0;
    // Java field: float c3DSkewAmount
    pub const c3DSkewAmount: f32 = 50.0;
    // Java field: float c3DAmbientIntensity
    pub const c3DAmbientIntensity: f32 = 0.3;
    // Java field: float c3DKeyX
    pub fn get_c3DKeyX() -> f32 { *c3DKeyX }
    // Java field: float c3DKeyY
    pub const c3DKeyY: f32 = 0.0;
    // Java field: float c3DKeyZ
    pub fn get_c3DKeyZ() -> f32 { *c3DKeyZ }
    // Java field: float c3DKeyIntensity
    pub const c3DKeyIntensity: f32 = 0.6;
    // Java field: float c3DFillX
    pub fn get_c3DFillX() -> f32 { *c3DFillX }
    // Java field: float c3DFillY
    pub const c3DFillY: f32 = 0.0;
    // Java field: float c3DFillZ
    pub fn get_c3DFillZ() -> f32 { *c3DFillZ }
    // Java field: float c3DFillIntensity
    pub const c3DFillIntensity: f32 = 0.6;
    // Java field: boolean fc3DConstrainRotation
    pub const fc3DConstrainRotation: bool = true;
    // Java field: boolean fc3DRotationCenterAuto
    pub const fc3DRotationCenterAuto: bool = false;
    // Java field: boolean fc3DParallel
    pub const fc3DParallel: bool = true;
    // Java field: boolean fc3DKeyHarsh
    pub const fc3DKeyHarsh: bool = true;
    // Java field: boolean fc3DFillHarsh
    pub const fc3DFillHarsh: bool = false;
    // Java field: float blurRadius
    pub const blurRadius: f32 = 0.0;
    // Java field: float effectExtent_leftEdge
    pub const effectExtent_leftEdge: f32 = 0.0;
    // Java field: float effectExtent_topEdge
    pub const effectExtent_topEdge: f32 = 0.0;
    // Java field: float effectExtent_rightEdge
    pub const effectExtent_rightEdge: f32 = 0.0;
    // Java field: float effectExtent_bottomEdge
    pub const effectExtent_bottomEdge: f32 = 0.0;
    // Java field: int glowColor
    pub const glowColor: i32 = 16777215;
    // Java field: float glowOpacity
    pub const glowOpacity: f32 = 1.0;
    // Java field: int glowSize
    pub const glowSize: i32 = 0;
    // Java field: int softEdgeRad
    pub const softEdgeRad: i32 = 0;
    // Java field: int diagramType
    pub const diagramType: i32 = 4095;
    // Java field: float horzRule_pct
    pub const horzRule_pct: f32 = 1.0;
    // Java field: int horzRule_align
    pub const horzRule_align: i32 = 0;
    // Java field: float horzRule_height
    pub const horzRule_height: f32 = 0.0;
    // Java field: float horzRule_width
    pub const horzRule_width: f32 = 0.0;
    // Java field: boolean horzRule_fHorzRule
    pub const horzRule_fHorzRule: bool = false;
    // Java field: boolean horzRule_fNoShade
    pub const horzRule_fNoShade: bool = false;
    // Java field: boolean horzRule_fStandard
    pub const horzRule_fStandard: bool = false;
    // Java field: boolean ink_useInk
    pub const ink_useInk: bool = true;
    // Java field: boolean colortransform_threeRGB
    pub const colortransform_threeRGB: bool = false;
    // Java field: boolean colortransform_hexRGB
    pub const colortransform_hexRGB: bool = false;
    // Java field: boolean colortransform_HSL
    pub const colortransform_HSL: bool = false;
    // Java field: boolean colortransform_sysClr
    pub const colortransform_sysClr: bool = false;
    // Java field: boolean colortransform_prstClr
    pub const colortransform_prstClr: bool = false;
    // Java field: boolean colortransform_themeClr
    pub const colortransform_themeClr: bool = false;
    // Java field: int colortransform_srgbClr
    pub const colortransform_srgbClr: i32 = 0;
    // Java field: float c3dBevelHeight
    pub const c3dBevelHeight: f32 = 6.0;
    // Java field: float c3dBevelWidth
    pub const c3dBevelWidth: f32 = 6.0;
    // Java field: float c3dBevelCrossWidth
    pub const c3dBevelCrossWidth: f32 = 11.0;
    // Java field: float c3dBevelCoolSlantWidth
    pub const c3dBevelCoolSlantWidth: f32 = 13.0;
    // Java field: float c3dBevelSoftRoundWidth
    pub const c3dBevelSoftRoundWidth: f32 = 12.0;
    // Java field: float c3dBevelSoftRoundHeight
    pub const c3dBevelSoftRoundHeight: f32 = 4.0;
    // Java field: float c3dBevelDivotWidth
    pub const c3dBevelDivotWidth: f32 = 11.0;
    // Java field: float c3dBevelDivotHeight
    pub const c3dBevelDivotHeight: f32 = 11.0;
    // Java field: float c3dBevelRibletWidth
    pub const c3dBevelRibletWidth: f32 = 8.0;
    // Java field: float c3dBevelHardEdgeWidth
    pub const c3dBevelHardEdgeWidth: f32 = 9.0;
    // Java field: float c3dBevelArtDecoWidth
    pub const c3dBevelArtDecoWidth: f32 = 9.0;
    // Java field: int c3dBevelPreset
    pub const c3dBevelPreset: i32 = 0;
    // Java field: int c3dContourClr
    pub const c3dContourClr: i32 = 0;
    // Java field: int c3dExtrusionClr
    pub const c3dExtrusionClr: i32 = 0;
    // Java field: float c3dContourWidth
    pub const c3dContourWidth: f32 = 0.0;
    // Java field: float c3dExtrusionHeight
    pub const c3dExtrusionHeight: f32 = 0.0;
    // Java field: int c3dPrstMaterial
    pub const c3dPrstMaterial: i32 = 0;
    // Java field: float c3dShapeDepth
    pub const c3dShapeDepth: f32 = 0.0;
    // Java field: float c3dFov
    pub const c3dFov: f32 = 0.0;
    // Java field: float c3dZoom
    pub const c3dZoom: f32 = 1.0;
    // Java field: int c3dPrstCamera
    pub const c3dPrstCamera: i32 = 0;
    // Java field: int c3dLightDir
    pub const c3dLightDir: i32 = 6;
    // Java field: int c3dPrstLight
    pub const c3dPrstLight: i32 = 1;
    // Java field: String vbaName
    pub const vbaName: &'static str = "";

    // Java: DefaultShapeValue()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: static float getBevelWidth(int prst)
    pub fn getBevelWidth(prst: i32) -> f32 {
        // Java: float width = 6.0F;
        let mut width: f32 = 6.0;
        // Java: switch (prst)
        match prst {
            // Java: case 2:
            2 => {
                // Java: width = 9.0F;
                width = 9.0;
            }
            // Java: case 3:
            // Java: case 4:
            // Java: case 9:
            // Java: case 11:
            // Java: default:
            // LLM Rust Match: default handled by _ => {} arm
            3 | 4 | 9 | 11 => {
                // Java: break;
                // LLM Rust Match: break is implicit in Rust match arms
            }
            // Java: case 5:
            5 => {
                // Java: width = 13.0F;
                width = 13.0;
                // Java: break;
                // LLM Rust Match: break is implicit in Rust match arms
            }
            // Java: case 6:
            6 => {
                // Java: width = 11.0F;
                width = 11.0;
                // Java: break;
                // LLM Rust Match: break is implicit in Rust match arms
            }
            // Java: case 7:
            7 => {
                // Java: width = 11.0F;
                width = 11.0;
                // Java: break;
                // LLM Rust Match: break is implicit in Rust match arms
            }
            // Java: case 8:
            8 => {
                // Java: width = 9.0F;
                width = 9.0;
                // Java: break;
                // LLM Rust Match: break is implicit in Rust match arms
            }
            // Java: case 10:
            10 => {
                // Java: width = 8.0F;
                width = 8.0;
                // Java: break;
                // LLM Rust Match: break is implicit in Rust match arms
            }
            // Java: case 12:
            12 => {
                // Java: width = 12.0F;
                width = 12.0;
            }
            _ => {}
        }
        // Java: return width;
        width
    }

    // Java: static float getBevelHeight(int prst)
    pub fn getBevelHeight(prst: i32) -> f32 {
        // Java: float width = 6.0F;
        let mut width: f32 = 6.0;
        // Java: switch (prst)
        match prst {
            // Java: case 7:
            7 => {
                // Java: width = 11.0F;
                width = 11.0;
                // Java: break;
                // LLM Rust Match: break is implicit in Rust match arms
            }
            // Java: case 12:
            12 => {
                // Java: width = 4.0F;
                width = 4.0;
            }
            _ => {}
        }
        // Java: return width;
        width
    }
}

#[allow(non_upper_case_globals)]
// Java field: TileFlipMode tileFlipMode
static tileFlipMode: LazyLock<TileFlipMode> = LazyLock::new(|| TileFlipMode::none);
#[allow(non_upper_case_globals)]
// Java field: ShadowPos shadowPos
static shadowPos: LazyLock<ShadowPos> = LazyLock::new(|| ShadowPos::None);
#[allow(non_upper_case_globals)]
// Java field: ExtrusionPlane extrudePlane
static extrudePlane: LazyLock<ExtrusionPlane> = LazyLock::new(|| ExtrusionPlane::XY);
#[allow(non_upper_case_globals)]
// Java field: float c3DXViewpoint
static c3DXViewpoint: LazyLock<f32> = LazyLock::new(|| MetricUtility::EMU2Points(1250000.0));
#[allow(non_upper_case_globals)]
// Java field: float c3DYViewpoint
static c3DYViewpoint: LazyLock<f32> = LazyLock::new(|| MetricUtility::EMU2Points(-1250000.0));
#[allow(non_upper_case_globals)]
// Java field: float c3DZViewpoint
static c3DZViewpoint: LazyLock<f32> = LazyLock::new(|| MetricUtility::EMU2Points(9000000.0));
#[allow(non_upper_case_globals)]
// Java field: float c3DKeyX
static c3DKeyX: LazyLock<f32> = LazyLock::new(|| MetricUtility::Fraction2Float(50000.0));
#[allow(non_upper_case_globals)]
// Java field: float c3DKeyZ
static c3DKeyZ: LazyLock<f32> = LazyLock::new(|| MetricUtility::Fraction2Float(10000.0));
#[allow(non_upper_case_globals)]
// Java field: float c3DFillX
static c3DFillX: LazyLock<f32> = LazyLock::new(|| MetricUtility::Fraction2Float(-50000.0));
#[allow(non_upper_case_globals)]
// Java field: float c3DFillZ
static c3DFillZ: LazyLock<f32> = LazyLock::new(|| MetricUtility::Fraction2Float(10000.0));
