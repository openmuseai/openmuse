// Java: import cn.wps.language.Thai
// Java: import cn.wps.language.input.ThaiCorrectionTool
// Java: import cn.wps.text.layout.IResizeCallback
// Java: import cn.wps.text.layout.pool.ObjectPool
// Java: import cn.wps.text.layout.typo.BaseTextLayout
// Java: import java.text.Bidi
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
use super::TextData::TextData;
use super::TextWarp::TextWarp;
use super::TextShape3d::TextShape3d;
use super::Scene3d::Scene3d;
use super::ParaInfo::ParaInfo;

// Java field: float UNIT_FONTSIZE
pub(crate) const UNIT_FONTSIZE: f32 = 1.0;
// Java field: float TEXT_TOLERANCE
pub(crate) const TEXT_TOLERANCE: f32 = 2.0;
// Java field: int MAX_SCALE_TIMES
pub(crate) const MAX_SCALE_TIMES: i32 = 100;
// Java field: int PRECISION
pub(crate) const PRECISION: i32 = 1000;

pub struct ParaSplitInfo;
impl ParaSplitInfo {
    pub fn new() -> Self { ParaSplitInfo }
}

// Java: public class TextBoxData
pub struct TextBoxData {
    // Java field: boolean isFullTypeface
    pub isFullTypeface: bool,
    // Java field: boolean isEditState
    pub isEditState: bool,
    // Java field: List<TextData> textDatas
    pub(crate) textDatas: Vec<TextData>,
    // Java field: int phType
    pub(crate) phType: i32,
    // Java field: float leftInset
    pub(crate) leftInset: f32,
    // Java field: float rightInset
    pub(crate) rightInset: f32,
    // Java field: float topInset
    pub(crate) topInset: f32,
    // Java field: float bottomInset
    pub(crate) bottomInset: f32,
    // Java field: boolean isWrapTextInShape
    pub(crate) isWrapTextInShape: bool,
    // Java field: int scaleTimes
    pub(crate) scaleTimes: i32,
    // Java field: int textautofit
    pub(crate) textautofit: i32,
    // Java field: int oldTextautofit
    pub(crate) oldTextautofit: i32,
    // Java field: int oldAnchor
    pub(crate) oldAnchor: i32,
    // Java field: int textVerticalType
    pub(crate) textVerticalType: i32,
    // Java field: int anchor
    pub(crate) anchor: i32,
    // Java field: boolean isAnchorCenter
    pub(crate) isAnchorCenter: bool,
    // Java field: boolean canChineseVert
    pub(crate) canChineseVert: bool,
    // Java field: float fontScale
    pub(crate) fontScale: f32,
    // Java field: float fontScaleRender
    pub(crate) fontScaleRender: f32,
    // Java field: boolean isFontScaleChange
    pub(crate) isFontScaleChange: bool,
    // Java field: double lnSpcReduction
    pub(crate) lnSpcReduction: f64,
    // Java field: boolean hasSpcReductionChange
    pub(crate) hasSpcReductionChange: bool,
    // Java field: TextShape3d textShape3d
    pub(crate) textShape3d: Option<TextShape3d>,
    // Java field: Scene3d scene3d
    pub(crate) scene3d: Option<Scene3d>,
    // Java field: TextWarp textWarp
    pub(crate) textWarp: Option<TextWarp>,
    // Java field: boolean canRenderCache
    pub(crate) canRenderCache: bool,
    // Java field: int mTextLength
    pub(crate) mTextLength: i32,
    // Java field: float mMinTextSize
    pub(crate) mMinTextSize: f32,
    // Java field: float mUnitTextSize
    pub(crate) mUnitTextSize: f32,
    // Java field: float mTextTolerance
    pub(crate) mTextTolerance: f32,
    // Java field: int presetTextWarp
    pub(crate) presetTextWarp: i32,
    // Java field: float mRotation
    pub(crate) mRotation: f32,
    // Java field: int textVertOverflow
    pub(crate) textVertOverflow: i32,
    // Java field: boolean needClip
    pub needClip: bool,
    // Java field: ParaSplitInfo mParaSplitInfo
    pub(crate) mParaSplitInfo: ParaSplitInfo,
}

// Java: public class TextBoxData
impl TextBoxData {
    // Java: TextBoxData()
    pub fn new() -> Self {
      // Java: super();
      TextBoxData {
          isFullTypeface: true,
          isEditState: false,
          textDatas: Vec::new(),
          phType: -1,
          leftInset: 0.0,
          rightInset: 0.0,
          topInset: 0.0,
          bottomInset: 0.0,
          isWrapTextInShape: true,
          scaleTimes: 0,
          textautofit: 0,
          oldTextautofit: -1,
          oldAnchor: -1,
          textVerticalType: 0,
          anchor: 0,
          isAnchorCenter: false,
          canChineseVert: true,
          fontScale: 1.0,
          fontScaleRender: 1.0,
          isFontScaleChange: false,
          lnSpcReduction: 0.0,
          hasSpcReductionChange: false,
          textShape3d: None,
          scene3d: None,
          textWarp: None,
          canRenderCache: true,
          mTextLength: 0,
          mMinTextSize: 1.0,
          mUnitTextSize: 1.0,
          mTextTolerance: 2.0,
          presetTextWarp: 0,
          mRotation: 0.0,
          textVertOverflow: 1,
          needClip: false,
          mParaSplitInfo: ParaSplitInfo::new(),
      }
    }

    pub fn getTextDatas(&self) -> &Vec<TextData> { &self.textDatas }
    pub fn setTextDatas(&mut self, datas: Vec<TextData>) { self.textDatas = datas; }
    pub fn getTextWarp(&self) -> Option<&TextWarp> { self.textWarp.as_ref() }
    pub fn setTextWarp(&mut self, warp: TextWarp) { self.textWarp = Some(warp); }
    pub fn getTextShape3d(&self) -> Option<&TextShape3d> { self.textShape3d.as_ref() }
    pub fn getScene3d(&self) -> Option<&Scene3d> { self.scene3d.as_ref() }
    pub fn getFontScale(&self) -> f32 { self.fontScale }
    pub fn getTextVerticalType(&self) -> i32 { self.textVerticalType }
    pub fn getAnchor(&self) -> i32 { self.anchor }

    pub fn setIHolder<H>(&mut self, _holder: H) {}
    pub fn setRenderCache(&mut self, canRenderCache: bool) { self.canRenderCache = canRenderCache; }
    pub fn setPHolderType(&mut self, phType: i32) { self.phType = phType; }
    pub fn getPHolderType(&self) -> i32 { self.phType }
    pub fn setMinTextSize(&mut self, size: f32) { self.mMinTextSize = size; }
    pub fn addTextData(&mut self, data: TextData) { self.textDatas.push(data); }
    pub fn createTextData(&self) -> TextData { TextData::new() }
    pub fn addTextLength(&mut self, delta: i32) { self.mTextLength += delta; }
    pub fn getTextLength(&self) -> i32 { self.mTextLength }
    pub fn isWrapTextInShape(&self) -> bool { self.isWrapTextInShape }
    pub fn setWrapTextInShape(&mut self, v: bool) { self.isWrapTextInShape = v; }
    pub fn setLeftInset(&mut self, v: f32) { self.leftInset = v; }
    pub fn setRightInset(&mut self, v: f32) { self.rightInset = v; }
    pub fn setTopInset(&mut self, v: f32) { self.topInset = v; }
    pub fn setBottomInset(&mut self, v: f32) { self.bottomInset = v; }
    pub fn getLeftInset(&self) -> f32 { self.leftInset }
    pub fn getRightInset(&self) -> f32 { self.rightInset }
    pub fn getTopInset(&self) -> f32 { self.topInset }
    pub fn getBottomInset(&self) -> f32 { self.bottomInset }
    pub fn leftInset(&self) -> f32 { self.leftInset }
    pub fn rightInset(&self) -> f32 { self.rightInset }
    pub fn topInset(&self) -> f32 { self.topInset }
    pub fn bottomInset(&self) -> f32 { self.bottomInset }
    pub fn isAnchorCenter(&self) -> bool { self.isAnchorCenter }
    pub fn getLnSpcReduction(&self) -> f64 { self.lnSpcReduction }
    pub fn getPresetTextWarp(&self) -> i32 { self.presetTextWarp }
    pub fn getOldTextAutoFit(&self) -> i32 { self.oldTextautofit }
    pub fn getOldAnchor(&self) -> i32 { self.oldAnchor }
    pub fn setAnchor(&mut self, v: i32) { self.anchor = v; }
    pub fn setAnchorCenter(&mut self, v: bool) { self.isAnchorCenter = v; }
    pub fn setVerticalText(&mut self, v: i32) { self.textVerticalType = v; }
    pub fn setTextAutoFit(&mut self, v: i32) { self.textautofit = v; }
    pub fn textAutofit(&self) -> i32 { self.textautofit }
    pub fn setFontScale(&mut self, v: f32) { self.fontScale = v; }
    pub fn setLnSpcReduction(&mut self, v: f64) { self.lnSpcReduction = v; }
    pub fn setPresetTextWarp(&mut self, v: i32) { self.presetTextWarp = v; }
    pub fn setOldTextAutoFit(&mut self, v: i32) { self.oldTextautofit = v; }
    pub fn setOldAnchor(&mut self, v: i32) { self.oldAnchor = v; }
    pub fn isEmpty(&self) -> bool { self.textDatas.is_empty() }
    pub fn isEmptyText(&self) -> bool {
        self.isEmpty() || (self.textDatas.len() == 1 && self.textDatas[0].mSourceText.as_ref().map(|s| s.is_empty()).unwrap_or(true))
    }
    pub fn hasVisibleText(&self) -> bool {
        for td in &self.textDatas {
            let source = td.mSourceText.as_deref().unwrap_or("");
            if !Self::isInvisibleText(source) { return true; }
        }
        false
    }
    fn isInvisibleText(source: &str) -> bool {
        source.is_empty() || source == " " || source == "\r"
    }
}
