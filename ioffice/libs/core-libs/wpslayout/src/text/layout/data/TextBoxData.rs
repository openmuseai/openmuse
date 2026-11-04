// Java: import cn.wps.language.Thai
use util::cn::wps::language::Thai;
// Java: import cn.wps.language.input.ThaiCorrectionTool
use util::cn::wps::language::input::ThaiCorrectionTool;
// Java: import cn.wps.language.input.ThaiCorrectionTool.ThaiCharType
use crate::language::input::ThaiCorrectionTool::ThaiCharType;
// Java: import cn.wps.text.layout.IResizeCallback
use wpslayout::cn::wps::text::layout::IResizeCallback;
// Java: import cn.wps.text.layout.pool.ObjectPool
use crate::text::layout::pool::ObjectPool;
// Java: import cn.wps.text.layout.typo.BaseTextLayout
use wpslayout::cn::wps::text::layout::typo::BaseTextLayout;
// Java: import java.text.Bidi
// TODO_RUST_CODE use ???; // java.text.Bidi
// Java: import java.util.ArrayList
// TODO_RUST_CODE use ???; // java.util.ArrayList
// Java: import java.util.HashMap
// TODO_RUST_CODE use ???; // java.util.HashMap
// Java: import java.util.List
// TODO_RUST_CODE use ???; // java.util.List
// Java: import java.util.Map
// TODO_RUST_CODE use ???; // java.util.Map

// Java field: float UNIT_FONTSIZE
pub(crate) const UNIT_FONTSIZE: f32 = /* TODO */;
// Java field: float TEXT_TOLERANCE
pub(crate) const TEXT_TOLERANCE: f32 = /* TODO */;
// Java field: int MAX_SCALE_TIMES
pub(crate) const MAX_SCALE_TIMES: i32 = /* TODO */;
// Java field: int PRECISION
pub(crate) const PRECISION: i32 = /* TODO */;
// Java field: byte FILL
pub(crate) const FILL: i8 = /* TODO */;
// Java field: byte OUTLINE
pub(crate) const OUTLINE: i8 = /* TODO */;
// Java field: byte BLUR
pub(crate) const BLUR: i8 = /* TODO */;
// Java field: byte REFLECTION
pub(crate) const REFLECTION: i8 = /* TODO */;
// Java field: byte OUTERSHADOW
pub(crate) const OUTERSHADOW: i8 = /* TODO */;
// Java field: byte INNERSHADOW
pub(crate) const INNERSHADOW: i8 = /* TODO */;

// Java: public class TextBoxData
pub struct TextBoxData {
    // Java field: boolean isFullTypeface
    pub(crate) isFullTypeface: bool,
    // Java field: boolean isEditState
    pub(crate) isEditState: bool,
    // Java field: List<TextData> textDatas
    pub(crate) textDatas: Arc<Mutex<Vec<TextData>>>,
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
    // Java field: IHolder iHolder
    pub(crate) iHolder: Arc<Mutex<IHolder>>,
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
    isFontScaleChange: bool,
    // Java field: double lnSpcReduction
    pub(crate) lnSpcReduction: f64,
    // Java field: boolean hasSpcReductionChange
    pub(crate) hasSpcReductionChange: bool,
    // Java field: TextShape3d textShape3d
    pub(crate) textShape3d: Arc<Mutex<TextShape3d>>,
    // Java field: Scene3d scene3d
    pub(crate) scene3d: Arc<Mutex<Scene3d>>,
    // Java field: TextWarp textWarp
    pub(crate) textWarp: Arc<Mutex<TextWarp>>,
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
    pub(crate) needClip: bool,
    // Java field: ParaSplitInfo mParaSplitInfo
    pub(crate) mParaSplitInfo: Arc<Mutex<ParaSplitInfo>>,
    // Java field: IResizeCallback mResizeCallback
    pub(crate) mResizeCallback: Arc<Mutex<IResizeCallback>>,
    // Java field: Map<Byte, Boolean> wordartAttr
    pub(crate) wordartAttr: Arc<Mutex<HashMap<Byte, Boolean>>>,
}

// Java: public class SpanNullPointerException
pub struct SpanNullPointerException {
}

// Java: public class SubParaSet
pub struct SubParaSet {
    // Java field: int number
    pub(crate) number: i32,
    // Java field: int indexBase
    pub(crate) indexBase: i32,
}

// Java: public class ParaSplitInfo
pub struct ParaSplitInfo {
    // Java field: List<SubParaSet> mParas
    pub(crate) mParas: Arc<Mutex<Vec<SubParaSet>>>,
}

// Java: public class WORDART_ATTR
pub struct WORDART_ATTR {
}

// Java: public class TextBoxData
impl TextBoxData {
    // Java: TextBoxData()
    pub fn TextBoxData(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: IResizeCallback getResizeCallback()
    pub fn getResizeCallback(&self) -> IResizeCallback {
      // Java: return this.mResizeCallback;
      // TODO_RUST_CODE
    }

    // Java: void setResizeCallback(IResizeCallback callback)
    pub fn setResizeCallback(&self, IResizeCallback callback) {
      // Java: this.mResizeCallback = callback;
      // TODO_RUST_CODE
    }

    // Java: ParaSplitInfo getParaSplitInfo()
    pub fn getParaSplitInfo(&self) -> ParaSplitInfo {
      // Java: return this.mParaSplitInfo;
      // TODO_RUST_CODE
    }

    // Java: IHolder getIHolder()
    pub fn getIHolder(&self) -> IHolder {
      // Java: return this.iHolder;
      // TODO_RUST_CODE
    }

    // Java: void setIHolder(IHolder iHolder)
    pub fn setIHolder(&self, IHolder iHolder) {
      // Java: this.iHolder = iHolder;
      // TODO_RUST_CODE
    }

    // Java: void addScaleTimes()
    pub fn addScaleTimes(&self) {
      // Java: ++this.scaleTimes;
      // TODO_RUST_CODE
    }

    // Java: int getScaleTimes()
    pub fn getScaleTimes(&self) -> i32 {
      // Java: return this.scaleTimes;
      // TODO_RUST_CODE
    }

    // Java: boolean inScaleTimes()
    pub fn inScaleTimes(&self) -> bool {
      // Java: return this.scaleTimes < 100;
      // TODO_RUST_CODE
    }

    // Java: List<TextData> getTextDatas()
    pub fn getTextDatas(&self) -> List<TextData> {
      // Java: return this.textDatas;
      // TODO_RUST_CODE
    }

    // Java: TextData getTextData(int index)
    pub fn getTextData(&self, int index) -> TextData {
      // Java: return (TextData)this.textDatas.get(index);
      // TODO_RUST_CODE
    }

    // Java: void addTextData(TextData data)
    pub fn addTextData(&self, TextData data) {
      // Java: this.textDatas.add(data);
      // TODO_RUST_CODE
    }

    // Java: void setTextDatas(List<TextData> textdatas)
    pub fn setTextDatas(&self, List<TextData> textdatas) {
      // Java: this.textDatas = textdatas;
      // TODO_RUST_CODE
    }

    // Java: void setPHolderType(int type)
    pub fn setPHolderType(&self, int type) {
      // Java: this.phType = type;
      // TODO_RUST_CODE
    }

    // Java: int getPHolderType()
    pub fn getPHolderType(&self) -> i32 {
      // Java: return this.phType;
      // TODO_RUST_CODE
    }

    // Java: void addTextLength(int length)
    pub fn addTextLength(&self, int length) {
      // Java: this.mTextLength += length;
      // TODO_RUST_CODE
    }

    // Java: int getTextLength()
    pub fn getTextLength(&self) -> i32 {
      // Java: return this.mTextLength;
      // TODO_RUST_CODE
    }

    // Java: boolean hasBullets()
    pub fn hasBullets(&self) -> bool {
      // Java: int i = 0;
      // TODO_RUST_CODE
      // Java: for(int size = this.textDatas.size(); i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: if (((TextData)this.textDatas.get(i)).hasBullets())
            // TODO_RUST_CODE
         {
            // Java: return true;
            // TODO_RUST_CODE
         }
      }
      // Java: return false;
      // TODO_RUST_CODE
    }

    // Java: boolean hasText()
    pub fn hasText(&self) -> bool {
      // Java: if (this.textDatas.size() == 0)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (this.textDatas.size() == 1)
            // TODO_RUST_CODE
         {
            // Java: TextData data = (TextData)this.textDatas.get(0);
            // TODO_RUST_CODE
            // Java: String source = data.mSourceText;
            // TODO_RUST_CODE
            // Java: if (source == null || source.length() == 0 || source.equals("\r"))
                // TODO_RUST_CODE
            {
               // Java: return false;
               // TODO_RUST_CODE
            }
         }
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasVisibleText()
    pub fn hasVisibleText(&self) -> bool {
      // Java: int i = 0;
      // TODO_RUST_CODE
      // Java: for(int size = this.textDatas.size(); i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: String source = ((TextData)this.textDatas.get(i)).mSourceText;
         // TODO_RUST_CODE
         // Java: if (!this.isInvisibleText(source))
            // TODO_RUST_CODE
         {
            // Java: return true;
            // TODO_RUST_CODE
         }
      }
      // Java: return false;
      // TODO_RUST_CODE
    }

    // Java: boolean isInvisibleText(String source)
    pub fn isInvisibleText(&self, String source) -> bool {
      // Java: return source == null || source.length() == 0 || source.equals(" ") || source.equals("\r");
      // TODO_RUST_CODE
    }

    // Java: boolean isEmptyText()
    pub fn isEmptyText(&self) -> bool {
      // Java: return this.isEmpty() || this.textDatas.size() == 1 && ((TextData)this.textDatas.get(0)).mSourceText.length() == 0;
      // TODO_RUST_CODE
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.textDatas.isEmpty();
      // TODO_RUST_CODE
    }

    // Java: public Map<Byte, Boolean> getWordartAttr()
    pub fn getWordartAttr(&self) -> public Map<Byte, Boolean> {
      // Java: return this.wordartAttr;
      // TODO_RUST_CODE
    }

    // Java: boolean checkWordArt()
    pub fn checkWordArt(&self) -> bool {
      // Java: this.wordartAttr = new HashMap();
      // TODO_RUST_CODE
      // Java: boolean hasArtFill = this.hasArtFill();
      // TODO_RUST_CODE
      // Java: this.wordartAttr.put((byte)1, hasArtFill);
      // TODO_RUST_CODE
      // Java: boolean hasOutln = this.hasOutline();
      // TODO_RUST_CODE
      // Java: this.wordartAttr.put((byte)2, hasOutln);
      // TODO_RUST_CODE
      // Java: boolean hasBlur = this.hasBlur();
      // TODO_RUST_CODE
      // Java: this.wordartAttr.put((byte)3, hasBlur);
      // TODO_RUST_CODE
      // Java: boolean hasRef = this.hasTextImage();
      // TODO_RUST_CODE
      // Java: this.wordartAttr.put((byte)4, hasRef);
      // TODO_RUST_CODE
      // Java: boolean hasOuterShdw = this.hasOuterShdw();
      // TODO_RUST_CODE
      // Java: this.wordartAttr.put((byte)5, hasOuterShdw);
      // TODO_RUST_CODE
      // Java: boolean hasInnerShdw = this.hasInnerShdw();
      // TODO_RUST_CODE
      // Java: this.wordartAttr.put((byte)6, hasInnerShdw);
      // TODO_RUST_CODE
      // Java: return hasArtFill || hasOutln || hasBlur || hasRef || hasOuterShdw || hasInnerShdw;
      // TODO_RUST_CODE
    }

    // Java: boolean hasArtFill()
    pub fn hasArtFill(&self) -> bool {
      // Java: if (this.textDatas.size() == 0)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < this.textDatas.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: if (((TextData)this.textDatas.get(i)).hasArtFill())
                // TODO_RUST_CODE
            {
               // Java: return true;
               // TODO_RUST_CODE
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasOutline()
    pub fn hasOutline(&self) -> bool {
      // Java: if (this.textDatas.size() == 0)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < this.textDatas.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: if (((TextData)this.textDatas.get(i)).hasOutline())
                // TODO_RUST_CODE
            {
               // Java: return true;
               // TODO_RUST_CODE
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasBlur()
    pub fn hasBlur(&self) -> bool {
      // Java: if (this.textDatas.size() == 0)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < this.textDatas.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: if (((TextData)this.textDatas.get(i)).hasBlur())
                // TODO_RUST_CODE
            {
               // Java: return true;
               // TODO_RUST_CODE
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasTextImage()
    pub fn hasTextImage(&self) -> bool {
      // Java: if (this.textDatas.size() == 0)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < this.textDatas.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: if (((TextData)this.textDatas.get(i)).hasTextImage())
                // TODO_RUST_CODE
            {
               // Java: return true;
               // TODO_RUST_CODE
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasOuterShdw()
    pub fn hasOuterShdw(&self) -> bool {
      // Java: if (this.textDatas.size() == 0)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < this.textDatas.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: if (((TextData)this.textDatas.get(i)).hasOuterShdw())
                // TODO_RUST_CODE
            {
               // Java: return true;
               // TODO_RUST_CODE
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasInnerShdw()
    pub fn hasInnerShdw(&self) -> bool {
      // Java: if (this.textDatas.size() == 0)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < this.textDatas.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: if (((TextData)this.textDatas.get(i)).hasInnerShdw())
                // TODO_RUST_CODE
            {
               // Java: return true;
               // TODO_RUST_CODE
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: float leftInset()
    pub fn leftInset(&self) -> f32 {
      // Java: return this.leftInset;
      // TODO_RUST_CODE
    }

    // Java: void setLeftInset(float leftInset)
    pub fn setLeftInset(&self, float leftInset) {
      // Java: this.leftInset = leftInset;
      // TODO_RUST_CODE
    }

    // Java: float rightInset()
    pub fn rightInset(&self) -> f32 {
      // Java: return this.rightInset;
      // TODO_RUST_CODE
    }

    // Java: void setRightInset(float rightInset)
    pub fn setRightInset(&self, float rightInset) {
      // Java: this.rightInset = rightInset;
      // TODO_RUST_CODE
    }

    // Java: float topInset()
    pub fn topInset(&self) -> f32 {
      // Java: return this.topInset;
      // TODO_RUST_CODE
    }

    // Java: void setTopInset(float topInset)
    pub fn setTopInset(&self, float topInset) {
      // Java: this.topInset = topInset;
      // TODO_RUST_CODE
    }

    // Java: float bottomInset()
    pub fn bottomInset(&self) -> f32 {
      // Java: return this.bottomInset;
      // TODO_RUST_CODE
    }

    // Java: void setBottomInset(float bottomInset)
    pub fn setBottomInset(&self, float bottomInset) {
      // Java: this.bottomInset = bottomInset;
      // TODO_RUST_CODE
    }

    // Java: boolean isWrapTextInShape()
    pub fn isWrapTextInShape(&self) -> bool {
      // Java: return this.isWrapTextInShape;
      // TODO_RUST_CODE
    }

    // Java: void setWrapTextInShape(boolean isWrapTextInShape)
    pub fn setWrapTextInShape(&self, boolean isWrapTextInShape) {
      // Java: this.isWrapTextInShape = isWrapTextInShape;
      // TODO_RUST_CODE
    }

    // Java: void setTextAutoFit(int textautofit)
    pub fn setTextAutoFit(&self, int textautofit) {
      // Java: this.textautofit = textautofit;
      // TODO_RUST_CODE
    }

    // Java: int textAutofit()
    pub fn textAutofit(&self) -> i32 {
      // Java: return this.textautofit;
      // TODO_RUST_CODE
    }

    // Java: void setOldTextAutoFit(int textautofit)
    pub fn setOldTextAutoFit(&self, int textautofit) {
      // Java: this.oldTextautofit = textautofit;
      // TODO_RUST_CODE
    }

    // Java: int oldTextAutoFit()
    pub fn oldTextAutoFit(&self) -> i32 {
      // Java: return this.oldTextautofit;
      // TODO_RUST_CODE
    }

    // Java: boolean isNormAutoFit()
    pub fn isNormAutoFit(&self) -> bool {
      // Java: return this.textautofit == 1;
      // TODO_RUST_CODE
    }

    // Java: void setFontScale(float scale)
    pub fn setFontScale(&self, float scale) {
      // Java: this.fontScale = scale;
      // TODO_RUST_CODE
    }

    // Java: float getFontScale()
    pub fn getFontScale(&self) -> f32 {
      // Java: return this.fontScale;
      // TODO_RUST_CODE
    }

    // Java: void setRotation(float rotation)
    pub fn setRotation(&self, float rotation) {
      // Java: this.mRotation = rotation;
      // TODO_RUST_CODE
    }

    // Java: float rotation()
    pub fn rotation(&self) -> f32 {
      // Java: return this.mRotation;
      // TODO_RUST_CODE
    }

    // Java: int getRoateDegrees()
    pub fn getRoateDegrees(&self) -> i32 {
      // Java: int rotDegrees = 0;
      // TODO_RUST_CODE
      // Java: switch (this.textVerticalType)
        // TODO_RUST_CODE
      {
         // Java: case 1:
         // TODO_RUST_CODE
         // Java: case 4:
         // TODO_RUST_CODE
            // Java: rotDegrees = 90;
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
            // Java: rotDegrees = 270;
            // TODO_RUST_CODE
         // Java: case 3:
         // TODO_RUST_CODE
      }
      // Java: return rotDegrees;
      // TODO_RUST_CODE
    }

    // Java: float setFontScaleRender(float scale, float expectScale)
    pub fn setFontScaleRender(&self, float scale, float expectScale) -> f32 {
      // Java: float minScale = this.minFontScale();
      // TODO_RUST_CODE
      // Java: float scaleRender = this.fontScaleRender;
      // TODO_RUST_CODE
      // Java: if (this.scaleTimes == 99 || expectScale < minScale * 0.5F)
        // TODO_RUST_CODE
      {
         // Java: scale = minScale;
         // TODO_RUST_CODE
      }
      // Java: this.fontScaleRender *= scale;
      // TODO_RUST_CODE
      // Java: if (this.fontScaleRender < minScale)
        // TODO_RUST_CODE
      {
         // Java: this.fontScaleRender = minScale;
         // TODO_RUST_CODE
         // Java: scale = minScale / scaleRender;
         // TODO_RUST_CODE
      }
      // Java: return scale;
      // TODO_RUST_CODE
    }

    // Java: boolean inScaleRender()
    pub fn inScaleRender(&self) -> bool {
      // Java: return (int)(this.fontScaleRender * 100.0F + 0.5F) > (int)(this.minFontScale() * 100.0F + 0.5F);
      // TODO_RUST_CODE
    }

    // Java: float getCurrentFontScale()
    pub fn getCurrentFontScale(&self) -> f32 {
      // Java: return this.fontScaleRender;
      // TODO_RUST_CODE
    }

    // Java: void setLnSpcReduction(double lnSpcReduction)
    pub fn setLnSpcReduction(&self, double lnSpcReduction) {
      // Java: this.lnSpcReduction = lnSpcReduction;
      // TODO_RUST_CODE
    }

    // Java: double lnSpcReduction()
    pub fn lnSpcReduction(&self) -> f64 {
      // Java: return this.lnSpcReduction;
      // TODO_RUST_CODE
    }

    // Java: void changeLnSpcReduction(boolean change)
    pub fn changeLnSpcReduction(&self, boolean change) {
      // Java: this.hasSpcReductionChange = change;
      // TODO_RUST_CODE
    }

    // Java: boolean hasLnSpcReductionChange()
    pub fn hasLnSpcReductionChange(&self) -> bool {
      // Java: return this.hasSpcReductionChange;
      // TODO_RUST_CODE
    }

    // Java: boolean updateAuotfit()
    pub fn updateAuotfit(&self) -> bool {
      // Java: if (null != this.iHolder && 1 == this.textautofit)
        // TODO_RUST_CODE
      {
         // Java: boolean isChangeAutoFitType = false;
         // TODO_RUST_CODE
         // Java: int scale = (int)(this.fontScale * 1000.0F + 0.5F);
         // TODO_RUST_CODE
         // Java: int scaleRender = (int)(this.fontScaleRender * 1000.0F + 0.5F);
         // TODO_RUST_CODE
         // Java: boolean isChangeFontScale = scale != scaleRender;
         // TODO_RUST_CODE
         // Java: if (!isChangeFontScale && scaleRender != 1000)
            // TODO_RUST_CODE
         {
            // Java: isChangeFontScale = this.iHolder.hasFontSizeFit();
            // TODO_RUST_CODE
         }
         // Java: if (isChangeFontScale)
            // TODO_RUST_CODE
         {
            // Java: double changeLnSpcReduction = this.lnSpcReduction;
            // TODO_RUST_CODE
            // Java: if (this.phType == 0 || this.phType == 2)
                // TODO_RUST_CODE
            {
               // Java: changeLnSpcReduction = (double)0.0F;
               // TODO_RUST_CODE
            }
            // Java: isChangeAutoFitType = this.iHolder.setFontScale(this.fontScaleRender, changeLnSpcReduction);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: this.iHolder.clearFontSizeFit();
            // TODO_RUST_CODE
         }
         // Java: return isChangeAutoFitType;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean isVertical()
    pub fn isVertical(&self) -> bool {
      // Java: switch (this.textVerticalType)
        // TODO_RUST_CODE
      {
         // Java: case 1:
         // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
         // Java: case 3:
         // TODO_RUST_CODE
         // Java: case 4:
         // TODO_RUST_CODE
         // Java: case 6:
         // TODO_RUST_CODE
            // Java: return true;
            // TODO_RUST_CODE
         // Java: case 5:
         // TODO_RUST_CODE
         // Java: default:
         // TODO_RUST_CODE
            // Java: return false;
            // TODO_RUST_CODE
      }
    }

    // Java: boolean isVerticalText()
    pub fn isVerticalText(&self) -> bool {
      // Java: switch (this.textVerticalType)
        // TODO_RUST_CODE
      {
         // Java: case 3:
         // TODO_RUST_CODE
         // Java: case 4:
         // TODO_RUST_CODE
         // Java: case 6:
         // TODO_RUST_CODE
            // Java: return true;
            // TODO_RUST_CODE
         // Java: case 5:
         // TODO_RUST_CODE
         // Java: default:
         // TODO_RUST_CODE
            // Java: return false;
            // TODO_RUST_CODE
      }
    }

    // Java: boolean isAllVerticalText()
    pub fn isAllVerticalText(&self) -> bool {
      // Java: switch (this.textVerticalType)
        // TODO_RUST_CODE
      {
         // Java: case 3:
         // TODO_RUST_CODE
         // Java: case 6:
         // TODO_RUST_CODE
            // Java: return true;
            // TODO_RUST_CODE
         // Java: default:
         // TODO_RUST_CODE
            // Java: return false;
            // TODO_RUST_CODE
      }
    }

    // Java: int verticalText()
    pub fn verticalText(&self) -> i32 {
      // Java: return this.textVerticalType;
      // TODO_RUST_CODE
    }

    // Java: void setVerticalText(int verticalText)
    pub fn setVerticalText(&self, int verticalText) {
      // Java: this.textVerticalType = verticalText;
      // TODO_RUST_CODE
    }

    // Java: boolean canChineseVert()
    pub fn canChineseVert(&self) -> bool {
      // Java: return this.canChineseVert;
      // TODO_RUST_CODE
    }

    // Java: void setCanChineseVert(boolean canChineseVert)
    pub fn setCanChineseVert(&self, boolean canChineseVert) {
      // Java: this.canChineseVert = canChineseVert;
      // TODO_RUST_CODE
    }

    // Java: boolean isVerticalCenter()
    pub fn isVerticalCenter(&self) -> bool {
      // Java: return this.anchor == 1;
      // TODO_RUST_CODE
    }

    // Java: int anchor()
    pub fn anchor(&self) -> i32 {
      // Java: return this.anchor;
      // TODO_RUST_CODE
    }

    // Java: void setAnchor(int anchor)
    pub fn setAnchor(&self, int anchor) {
      // Java: this.anchor = anchor;
      // TODO_RUST_CODE
    }

    // Java: int oldAnchor()
    pub fn oldAnchor(&self) -> i32 {
      // Java: return this.oldAnchor;
      // TODO_RUST_CODE
    }

    // Java: void setOldAnchor(int anchor)
    pub fn setOldAnchor(&self, int anchor) {
      // Java: this.oldAnchor = anchor;
      // TODO_RUST_CODE
    }

    // Java: boolean isAnchorCenter()
    pub fn isAnchorCenter(&self) -> bool {
      // Java: return this.isAnchorCenter;
      // TODO_RUST_CODE
    }

    // Java: void setAnchorCenter(boolean isAnchorCenter)
    pub fn setAnchorCenter(&self, boolean isAnchorCenter) {
      // Java: this.isAnchorCenter = isAnchorCenter;
      // TODO_RUST_CODE
    }

    // Java: TextShape3d getTextShape3d()
    pub fn getTextShape3d(&self) -> TextShape3d {
      // Java: return this.textShape3d;
      // TODO_RUST_CODE
    }

    // Java: void setTextShape3d(TextShape3d textShape3d)
    pub fn setTextShape3d(&self, TextShape3d textShape3d) {
      // Java: this.textShape3d = textShape3d;
      // TODO_RUST_CODE
    }

    // Java: Scene3d getScene3d()
    pub fn getScene3d(&self) -> Scene3d {
      // Java: return this.scene3d;
      // TODO_RUST_CODE
    }

    // Java: void setScene3d(Scene3d scene3d)
    pub fn setScene3d(&self, Scene3d scene3d) {
      // Java: this.scene3d = scene3d;
      // TODO_RUST_CODE
    }

    // Java: TextWarp getTextWarp()
    pub fn getTextWarp(&self) -> TextWarp {
      // Java: return this.textWarp;
      // TODO_RUST_CODE
    }

    // Java: void setTextWarp(TextWarp textWarp)
    pub fn setTextWarp(&self, TextWarp textWarp) {
      // Java: this.textWarp = textWarp;
      // TODO_RUST_CODE
    }

    // Java: float getUnitTextSize()
    pub fn getUnitTextSize(&self) -> f32 {
      // Java: return this.mUnitTextSize;
      // TODO_RUST_CODE
    }

    // Java: void setUnitTextSize(float unitTextSize)
    pub fn setUnitTextSize(&self, float unitTextSize) {
      // Java: this.mUnitTextSize = unitTextSize;
      // TODO_RUST_CODE
    }

    // Java: float getTextTolerance()
    pub fn getTextTolerance(&self) -> f32 {
      // Java: return this.mTextTolerance;
      // TODO_RUST_CODE
    }

    // Java: void setTextTolerance(float textTolerance)
    pub fn setTextTolerance(&self, float textTolerance) {
      // Java: this.mTextTolerance = textTolerance;
      // TODO_RUST_CODE
    }

    // Java: int getTextVertOverflow()
    pub fn getTextVertOverflow(&self) -> i32 {
      // Java: return this.textVertOverflow;
      // TODO_RUST_CODE
    }

    // Java: void setTextVertOverflow(int textVertOverflow)
    pub fn setTextVertOverflow(&self, int textVertOverflow) {
      // Java: this.textVertOverflow = textVertOverflow;
      // TODO_RUST_CODE
    }

    // Java: void copyFrom(TextBoxData srcData)
    pub fn copyFrom(&self, TextBoxData srcData) {
      // Java: int i = 0;
      // TODO_RUST_CODE
      // Java: for(int size = srcData.textDatas.size(); i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: TextData textData = new TextData();
         // TODO_RUST_CODE
         // Java: textData.copyFrom((TextData)srcData.textDatas.get(i));
         // TODO_RUST_CODE
         // Java: this.textDatas.add(i, textData);
         // TODO_RUST_CODE
      }
      // Java: this.phType = srcData.phType;
      // TODO_RUST_CODE
      // Java: this.leftInset = srcData.leftInset;
      // TODO_RUST_CODE
      // Java: this.rightInset = srcData.rightInset;
      // TODO_RUST_CODE
      // Java: this.topInset = srcData.topInset;
      // TODO_RUST_CODE
      // Java: this.bottomInset = srcData.bottomInset;
      // TODO_RUST_CODE
      // Java: this.isWrapTextInShape = srcData.isWrapTextInShape;
      // TODO_RUST_CODE
      // Java: this.iHolder = srcData.iHolder;
      // TODO_RUST_CODE
      // Java: this.scaleTimes = srcData.scaleTimes;
      // TODO_RUST_CODE
      // Java: this.textautofit = srcData.textautofit;
      // TODO_RUST_CODE
      // Java: this.textVerticalType = srcData.textVerticalType;
      // TODO_RUST_CODE
      // Java: this.canChineseVert = srcData.canChineseVert;
      // TODO_RUST_CODE
      // Java: this.anchor = srcData.anchor;
      // TODO_RUST_CODE
      // Java: this.isAnchorCenter = srcData.isAnchorCenter;
      // TODO_RUST_CODE
      // Java: this.fontScale = srcData.fontScale;
      // TODO_RUST_CODE
      // Java: this.isFontScaleChange = srcData.isFontScaleChange;
      // TODO_RUST_CODE
      // Java: this.mTextLength = srcData.mTextLength;
      // TODO_RUST_CODE
      // Java: this.mMinTextSize = srcData.mMinTextSize;
      // TODO_RUST_CODE
      // Java: this.textShape3d = srcData.textShape3d;
      // TODO_RUST_CODE
      // Java: this.scene3d = srcData.scene3d;
      // TODO_RUST_CODE
      // Java: this.textWarp = srcData.textWarp;
      // TODO_RUST_CODE
      // Java: this.mUnitTextSize = srcData.mUnitTextSize;
      // TODO_RUST_CODE
      // Java: this.mTextTolerance = srcData.mTextTolerance;
      // TODO_RUST_CODE
      // Java: this.isFullTypeface = srcData.isFullTypeface;
      // TODO_RUST_CODE
      // Java: this.isEditState = srcData.isEditState;
      // TODO_RUST_CODE
      // Java: this.textVertOverflow = srcData.textVertOverflow;
      // TODO_RUST_CODE
    }

    // Java: void processBeforeTypo(boolean forceProcessCrlf)
    pub fn processBeforeTypo(&self, boolean forceProcessCrlf) {
      // Java: this.mParaSplitInfo.clear();
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < this.textDatas.size(); ++i)
        // TODO_RUST_CODE
      {
         // Java: this.mParaSplitInfo.addSubPara(new SubParaSet(1, i));
         // TODO_RUST_CODE
      }
      // Java: if (forceProcessCrlf)
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < this.textDatas.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: String s = "\r(\n)|\n";
            // TODO_RUST_CODE
            // Java: int subInc = this.splitDataBySeparator(this.textDatas, i, s);
            // TODO_RUST_CODE
            // Java: this.mParaSplitInfo.setSubPara(i, subInc);
            // TODO_RUST_CODE
            // Java: i += subInc;
            // TODO_RUST_CODE
         }
      }
      // Java: DataObjectPool.CharArray charArray = DataObjectPool.charArrayObjectPool.borrowObject();
      // TODO_RUST_CODE
      // Java: int i = 0;
      // TODO_RUST_CODE
      // Java: for(int size = this.textDatas.size(); i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: TextData data = (TextData)this.textDatas.get(i);
         // TODO_RUST_CODE
         // Java: data.mTypoText = data.mSourceText;
         // TODO_RUST_CODE
         // Java: this.insertThaiPlaceHolder(data);
         // TODO_RUST_CODE
         // Java: data.mTypoText = data.mTypoText.replace("\u0085", "\u200b");
         // TODO_RUST_CODE
         // Java: this.removeTrailingSpace(data);
         // TODO_RUST_CODE
         // Java: this.mergeSpan(data);
         // TODO_RUST_CODE
         // Java: int len = data.mSourceText.length();
         // TODO_RUST_CODE
         // Java: char[] values = charArray.toCharArray(data.mSourceText, 0, len);
         // TODO_RUST_CODE
         // Java: data.requiresBidi = Bidi.requiresBidi(values, 0, len);
         // TODO_RUST_CODE
         // Java: if (Bidi.requiresBidi(values, 0, len))
            // TODO_RUST_CODE
         {
            // Java: int direction = data.mPara.mIsRightToLeft ? 1 : 0;
            // TODO_RUST_CODE
            // Java: Bidi bd = new Bidi(values, 0, (byte[])null, 0, len, direction);
            // TODO_RUST_CODE
            // Java: data.mBidi = bd;
            // TODO_RUST_CODE
         }
      }
      // Java: DataObjectPool.charArrayObjectPool.returnObject(charArray);
      // TODO_RUST_CODE
    }

    // Java: float minFontScale()
    pub fn minFontScale(&self) -> f32 {
      // Java: float minFontScale = 1.0F;
      // TODO_RUST_CODE
      // Java: switch (this.phType)
        // TODO_RUST_CODE
      {
         // Java: case 0:
         // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
            // Java: minFontScale = 0.9F;
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: default:
         // TODO_RUST_CODE
            // Java: minFontScale = 0.25F;
            // TODO_RUST_CODE
      }
      // Java: return minFontScale;
      // TODO_RUST_CODE
    }

    // Java: boolean isPhTitle()
    pub fn isPhTitle(&self) -> bool {
      // Java: return this.phType == 0 || this.phType == 2;
      // TODO_RUST_CODE
    }

    // Java: void setMinTextSize(float size)
    pub fn setMinTextSize(&self, float size) {
      // Java: if (this.mMinTextSize == this.mUnitTextSize)
        // TODO_RUST_CODE
      {
         // Java: this.mMinTextSize = size;
         // TODO_RUST_CODE
      }
      // Java: else if (size < this.mMinTextSize)
        // TODO_RUST_CODE
      {
         // Java: this.mMinTextSize = size;
         // TODO_RUST_CODE
      }
    }

    // Java: float getMinTextSize()
    pub fn getMinTextSize(&self) -> f32 {
      // Java: return this.mMinTextSize * this.fontScaleRender;
      // TODO_RUST_CODE
    }

    // Java: float getOriginTextSize()
    pub fn getOriginTextSize(&self) -> f32 {
      // Java: return this.mMinTextSize;
      // TODO_RUST_CODE
    }

    // Java: boolean canRenderCache()
    pub fn canRenderCache(&self) -> bool {
      // Java: return this.canRenderCache;
      // TODO_RUST_CODE
    }

    // Java: void setRenderCache(boolean canRenderCache)
    pub fn setRenderCache(&self, boolean canRenderCache) {
      // Java: this.canRenderCache = canRenderCache;
      // TODO_RUST_CODE
    }

    // Java: int presetTextWarp()
    pub fn presetTextWarp(&self) -> i32 {
      // Java: return this.presetTextWarp;
      // TODO_RUST_CODE
    }

    // Java: void setPresetTextWarp(int presetTextWarp)
    pub fn setPresetTextWarp(&self, int presetTextWarp) {
      // Java: this.presetTextWarp = presetTextWarp;
      // TODO_RUST_CODE
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: ObjectPool<TextData> pool = DataObjectPool.textDataObjectPool;
      // TODO_RUST_CODE
      // Java: int i = 0;
      // TODO_RUST_CODE
      // Java: for(int size = this.textDatas.size(); i < size; ++i)
        // TODO_RUST_CODE
      {
         // Java: pool.returnObject((TextData)this.textDatas.get(i));
         // TODO_RUST_CODE
      }
      // Java: this.textDatas.clear();
      // TODO_RUST_CODE
      // Java: this.mParaSplitInfo.clear();
      // TODO_RUST_CODE
      // Java: this.phType = -1;
      // TODO_RUST_CODE
      // Java: this.leftInset = 0.0F;
      // TODO_RUST_CODE
      // Java: this.rightInset = 0.0F;
      // TODO_RUST_CODE
      // Java: this.topInset = 0.0F;
      // TODO_RUST_CODE
      // Java: this.bottomInset = 0.0F;
      // TODO_RUST_CODE
      // Java: this.isWrapTextInShape = true;
      // TODO_RUST_CODE
      // Java: this.iHolder = null;
      // TODO_RUST_CODE
      // Java: this.scaleTimes = 0;
      // TODO_RUST_CODE
      // Java: this.textautofit = 0;
      // TODO_RUST_CODE
      // Java: this.textVerticalType = 0;
      // TODO_RUST_CODE
      // Java: this.canChineseVert = true;
      // TODO_RUST_CODE
      // Java: this.anchor = 0;
      // TODO_RUST_CODE
      // Java: this.isAnchorCenter = false;
      // TODO_RUST_CODE
      // Java: this.fontScale = 1.0F;
      // TODO_RUST_CODE
      // Java: this.fontScaleRender = 1.0F;
      // TODO_RUST_CODE
      // Java: this.isFontScaleChange = false;
      // TODO_RUST_CODE
      // Java: this.lnSpcReduction = (double)0.0F;
      // TODO_RUST_CODE
      // Java: this.textShape3d = null;
      // TODO_RUST_CODE
      // Java: this.scene3d = null;
      // TODO_RUST_CODE
      // Java: this.textWarp = null;
      // TODO_RUST_CODE
      // Java: this.canRenderCache = true;
      // TODO_RUST_CODE
      // Java: this.mTextLength = 0;
      // TODO_RUST_CODE
      // Java: this.mMinTextSize = 1.0F;
      // TODO_RUST_CODE
      // Java: this.presetTextWarp = 0;
      // TODO_RUST_CODE
      // Java: this.mResizeCallback = null;
      // TODO_RUST_CODE
      // Java: this.isFullTypeface = true;
      // TODO_RUST_CODE
      // Java: this.textVertOverflow = 1;
      // TODO_RUST_CODE
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.textDatas.size();
      // TODO_RUST_CODE
    }

    // Java: TextData createTextData()
    pub fn createTextData(&self) -> TextData {
      // Java: return DataObjectPool.textDataObjectPool.borrowObject();
      // TODO_RUST_CODE
    }

    // Java: boolean foundConsonant(StringBuilder text, int index)
    pub fn foundConsonant(&self, StringBuilder text, int index) -> bool {
      // Java: int searchLen = 2;
      // TODO_RUST_CODE
      // Java: int i = index - 1;
      // TODO_RUST_CODE
      // Java: while(true)
        // TODO_RUST_CODE
      {
         // Java: if (i >= 0 && searchLen > 0)
            // TODO_RUST_CODE
         {
            // Java: char ch = text.charAt(i);
            // TODO_RUST_CODE
            // Java: if (!Thai.isThai(ch))
                // TODO_RUST_CODE
            {
               // Java: return false;
               // TODO_RUST_CODE
            }
            // Java: ThaiCorrectionTool.ThaiCharType type = ThaiCorrectionTool.getThaiCharType(ch);
            // TODO_RUST_CODE
            // Java: if (type != ThaiCharType.Consonant && type != ThaiCharType.ConsonantVowel)
                // TODO_RUST_CODE
            {
               // Java: if (!Thai.isHat(ch) && !Thai.isShoe(ch) && !Thai.isTone(ch))
                    // TODO_RUST_CODE
               {
                  // Java: return false;
                  // TODO_RUST_CODE
               }
               // Java: --i;
               // TODO_RUST_CODE
               // Java: --searchLen;
               // TODO_RUST_CODE
               // Java: continue;
               // TODO_RUST_CODE
            }
            // Java: return true;
            // TODO_RUST_CODE
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: void insertThaiPlaceHolder(TextData data)
    pub fn insertThaiPlaceHolder(&self, TextData data) {
      // Java: int[] positions = new int[256];
      // TODO_RUST_CODE
      // Java: int index = 0;
      // TODO_RUST_CODE
      // Java: StringBuilder text = new StringBuilder(data.mTypoText);
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < text.length(); ++i)
        // TODO_RUST_CODE
      {
         // Java: char ch = text.charAt(i);
         // TODO_RUST_CODE
         // Java: if (Thai.isThai(ch) && (Thai.isHat(ch) || Thai.isShoe(ch) || Thai.isTone(ch)) && !this.foundConsonant(text, i))
            // TODO_RUST_CODE
         {
            // Java: if (index >= positions.length)
                // TODO_RUST_CODE
            {
               // Java: int[] grow = new int[index * 2];
               // TODO_RUST_CODE
               // Java: System.arraycopy(positions, 0, grow, 0, positions.length);
               // TODO_RUST_CODE
               // Java: positions = grow;
               // TODO_RUST_CODE
            }
            // Java: positions[index++] = i;
            // TODO_RUST_CODE
            // Java: positions[index++] = i + 1;
            // TODO_RUST_CODE
            // Java: this.expandSpan(data, i, 1);
            // TODO_RUST_CODE
            // Java: text.insert(i, String.valueOf('◌'));
            // TODO_RUST_CODE
            // Java: ++i;
            // TODO_RUST_CODE
         }
      }
      // Java: if (index > 0)
        // TODO_RUST_CODE
      {
         // Java: int[] result = new int[index];
         // TODO_RUST_CODE
         // Java: System.arraycopy(positions, 0, result, 0, index);
         // TODO_RUST_CODE
         // Java: data.mTypoText = text.toString();
         // TODO_RUST_CODE
         // Java: data.mReplacePositions = result;
         // TODO_RUST_CODE
      }
    }

    // Java: void removeTrailingSpace(TextData data)
    pub fn removeTrailingSpace(&self, TextData data) {
      // Java: String text = data.mTypoText;
      // TODO_RUST_CODE
      // Java: int oldLen = text.length();
      // TODO_RUST_CODE
      // Java: if (oldLen > 0 && (text.charAt(oldLen - 1) != ' ' || oldLen >= 2 && text.charAt(oldLen - 2) == ' '))
        // TODO_RUST_CODE
      {
         // Java: int i;
         // TODO_RUST_CODE
         // Java: for(i = oldLen - 1; i >= 0 && '\t' >= text.charAt(i) && (' ' >= text.charAt(i) || 12288 == text.charAt(i)); --i)
            // TODO_RUST_CODE
         {
         }
         // Java: int len;
         // TODO_RUST_CODE
         // Java: if (i < 0)
            // TODO_RUST_CODE
         {
            // Java: len = 1;
            // TODO_RUST_CODE
            // Java: text = " ";
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: len = i + 1;
            // TODO_RUST_CODE
            // Java: text = text.substring(0, i + 1);
            // TODO_RUST_CODE
         }
         // Java: if (len < oldLen)
            // TODO_RUST_CODE
         {
            // Java: this.removeSpan(data, len);
            // TODO_RUST_CODE
         }
         // Java: data.mTypoText = text;
         // TODO_RUST_CODE
      }
    }

    // Java: void expandSpan(TextData data, int index, int delta)
    pub fn expandSpan(&self, TextData data, int index, int delta) {
      // Java: SpanInfo span;
      // TODO_RUST_CODE
      // Java: for(span = data.getFirstSpan(); null != span; span = data.getNextSpan())
        // TODO_RUST_CODE
      {
         // Java: if (span.mStartPos >= index || span.mEndPos > index)
            // TODO_RUST_CODE
         {
            // Java: if (span.mStartPos > index)
                // TODO_RUST_CODE
            {
               // Java: span.mStartPos += delta;
               // TODO_RUST_CODE
            }
            // Java: span.mEndPos += delta;
            // TODO_RUST_CODE
            // Java: span = data.getNextSpan();
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         }
      }
      // Java: while(null != span)
        // TODO_RUST_CODE
      {
         // Java: span.mStartPos += delta;
         // TODO_RUST_CODE
         // Java: span.mEndPos += delta;
         // TODO_RUST_CODE
         // Java: span = data.getNextSpan();
         // TODO_RUST_CODE
      }
    }

    // Java: void removeSpan(TextData data, int end)
    pub fn removeSpan(&self, TextData data, int end) {
      // Java: for(SpanInfo span = data.getFirstSpan(); null != span; span = data.getNextSpan())
        // TODO_RUST_CODE
      {
         // Java: if (span.mEndPos > end)
            // TODO_RUST_CODE
         {
            // Java: if (span.mStartPos < end)
                // TODO_RUST_CODE
            {
               // Java: span.mEndPos = end;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: data.removeCurrentSpan();
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: void mergeSpan(TextData data)
    pub fn mergeSpan(&self, TextData data) {
      // Java: SpanInfo befSpan = data.getFirstSpan();
      // TODO_RUST_CODE
      // Java: SpanInfo aftSpan = data.getNextSpan();
      // TODO_RUST_CODE
      // Java: while(null != befSpan && null != aftSpan)
        // TODO_RUST_CODE
      {
         // Java: if (befSpan.equalsAttr(aftSpan))
            // TODO_RUST_CODE
         {
            // Java: befSpan.mEndPos = aftSpan.mEndPos;
            // TODO_RUST_CODE
            // Java: data.removeCurrentSpan();
            // TODO_RUST_CODE
            // Java: aftSpan = data.getNextSpan();
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: befSpan = aftSpan;
            // TODO_RUST_CODE
            // Java: aftSpan = data.getNextSpan();
            // TODO_RUST_CODE
         }
      }
    }

    // Java: int splitDataBySeparator(List<TextData> datas, int index, String separator)
    pub fn splitDataBySeparator(&self, List<TextData> datas, int index, String separator) -> i32 {
      // Java: int orgIndex = index;
      // TODO_RUST_CODE
      // Java: TextData data = (TextData)datas.get(index);
      // TODO_RUST_CODE
      // Java: String[] texts = data.mSourceText.split(separator, -1);
      // TODO_RUST_CODE
      // Java: int size = texts.length;
      // TODO_RUST_CODE
      // Java: if (size > 1)
        // TODO_RUST_CODE
      {
         // Java: datas.remove(index);
         // TODO_RUST_CODE
         // Java: int j;
         // TODO_RUST_CODE
         // Java: for(j = 0; j < size - 1; ++j)
            // TODO_RUST_CODE
         {
            // Java: texts[j] = texts[j] + "\r";
            // TODO_RUST_CODE
         }
         // Java: if (texts[j].isEmpty())
            // TODO_RUST_CODE
         {
            // Java: texts[j] = "\r";
            // TODO_RUST_CODE
            // Java: SpanInfo lastSpan = new SpanInfo(data.getLastSpan());
            // TODO_RUST_CODE
            // Java: lastSpan.mStartPos = data.mSourceText.length();
            // TODO_RUST_CODE
            // Java: lastSpan.mEndPos = lastSpan.mStartPos + 1;
            // TODO_RUST_CODE
            // Java: data.mOtherSpans.add(lastSpan);
            // TODO_RUST_CODE
         }
         // Java: data.resetSpanIndex();
         // TODO_RUST_CODE
         // Java: for(int var10 = 0; var10 < size; ++var10)
            // TODO_RUST_CODE
         {
            // Java: TextData subData = this.createSubData(data, texts[var10], 0 == var10, size - 1 == var10);
            // TODO_RUST_CODE
            // Java: datas.add(index++, subData);
            // TODO_RUST_CODE
         }
      }
      // Java: return index > orgIndex ? index - orgIndex - 1 : 0;
      // TODO_RUST_CODE
    }

    // Java: TextData createSubData(TextData data, String text, boolean isFirst, boolean isLast)
    pub fn createSubData(&self, TextData data, String text, boolean isFirst, boolean isLast) -> TextData {
      // Java: TextData subData = new TextData(data);
      // TODO_RUST_CODE
      // Java: subData.mSourceText = text;
      // TODO_RUST_CODE
      // Java: subData.mPara = new ParaInfo(subData.mPara);
      // TODO_RUST_CODE
      // Java: subData.mPara.mbulletsInfo = new BulletsInfo(subData.mPara.mbulletsInfo);
      // TODO_RUST_CODE
      // Java: subData.mOtherSpans = new ArrayList();
      // TODO_RUST_CODE
      // Java: if (!isFirst)
        // TODO_RUST_CODE
      {
         // Java: subData.mPara.mIndent = 0;
         // TODO_RUST_CODE
         // Java: subData.mPara.mSpcBef = (double)0.0F;
         // TODO_RUST_CODE
         // Java: subData.mPara.mbulletsInfo.setBulletType(-1);
         // TODO_RUST_CODE
      }
      // Java: if (!isLast)
        // TODO_RUST_CODE
      {
         // Java: subData.mPara.mSpcAft = (double)0.0F;
         // TODO_RUST_CODE
      }
      // Java: SpanInfo span = data.getCurrentSpan();
      // TODO_RUST_CODE
      // Java: if (null == span)
        // TODO_RUST_CODE
      {
         // Java: throw new SpanNullPointerException("Span data missed!!");
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: SpanInfo subSpan = new SpanInfo(span);
         // TODO_RUST_CODE
         // Java: subData.mFirstSpan = subSpan;
         // TODO_RUST_CODE
         // Java: int txtlen = text.length();
         // TODO_RUST_CODE
         // Java: if (subSpan.mEndPos > txtlen)
            // TODO_RUST_CODE
         {
            // Java: subSpan.mEndPos = txtlen;
            // TODO_RUST_CODE
            // Java: span.mStartPos = txtlen;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: for(SpanInfo var9 = data.getNextSpan(); null != var9 && subSpan.mEndPos < txtlen; var9 = data.getNextSpan())
                // TODO_RUST_CODE
            {
               // Java: subSpan = new SpanInfo(var9);
               // TODO_RUST_CODE
               // Java: subData.mOtherSpans.add(subSpan);
               // TODO_RUST_CODE
               // Java: if (subSpan.mEndPos > txtlen)
                    // TODO_RUST_CODE
               {
                  // Java: subSpan.mEndPos = txtlen;
                  // TODO_RUST_CODE
                  // Java: var9.mStartPos = txtlen;
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: this.offsetSpan(data, txtlen, data.getSpanIndex());
         // TODO_RUST_CODE
         // Java: if (BaseTextLayout.isShape(text))
            // TODO_RUST_CODE
         {
            // Java: SpanInfo var10000 = subData.mFirstSpan;
            // TODO_RUST_CODE
            // Java: var10000.mFontSize += subData.mFirstSpan.mFontSize * 0.2F;
            // TODO_RUST_CODE
         }
         // Java: return subData;
         // TODO_RUST_CODE
      }
    }

    // Java: void offsetSpan(TextData data, int offset, int index)
    pub fn offsetSpan(&self, TextData data, int offset, int index) {
      // Java: SpanInfo span = null;
      // TODO_RUST_CODE
      // Java: if (index < 0)
        // TODO_RUST_CODE
      {
         // Java: index = -1;
         // TODO_RUST_CODE
         // Java: span = data.mFirstSpan;
         // TODO_RUST_CODE
      }
      // Java: else if (index < data.mOtherSpans.size())
        // TODO_RUST_CODE
      {
         // Java: span = (SpanInfo)data.mOtherSpans.get(index);
         // TODO_RUST_CODE
      }
      // Java: ++index;
      // TODO_RUST_CODE
      // Java: for(; null != span; span = index < data.mOtherSpans.size() ? (SpanInfo)data.mOtherSpans.get(index++) : null)
        // TODO_RUST_CODE
      {
         // Java: if (span.mStartPos >= offset)
            // TODO_RUST_CODE
         {
            // Java: span.mStartPos -= offset;
            // TODO_RUST_CODE
            // Java: span.mEndPos -= offset;
            // TODO_RUST_CODE
         }
      }
    }

}

// Java: public class SpanNullPointerException
impl SpanNullPointerException {
    // Java: SpanNullPointerException()
    pub fn SpanNullPointerException(&self) {
         // Java: super();
         // TODO_RUST_CODE
    }

    // Java: SpanNullPointerException(String detailMsg)
    pub fn SpanNullPointerException(&self, String detailMsg) {
         // Java: super(detailMsg);
         // TODO_RUST_CODE
    }

}

// Java: public class SubParaSet
impl SubParaSet {
    // Java: SubParaSet(int number, int indexBase)
    pub fn SubParaSet(&self, int number, int indexBase) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.number = number;
         // TODO_RUST_CODE
         // Java: this.indexBase = indexBase;
         // TODO_RUST_CODE
    }

}

// Java: public class ParaSplitInfo
impl ParaSplitInfo {
    // Java: ParaSplitInfo()
    pub fn ParaSplitInfo(&self) {
         // Java: super();
         // TODO_RUST_CODE
    }

    // Java: void addSubPara(SubParaSet subParaSet)
    pub fn addSubPara(&self, SubParaSet subParaSet) {
         // Java: this.mParas.add(subParaSet);
         // TODO_RUST_CODE
    }

    // Java: void removeSubPara(int paraIndex)
    pub fn removeSubPara(&self, int paraIndex) {
         // Java: if (paraIndex >= 0 && paraIndex < this.mParas.size())
        // TODO_RUST_CODE
         {
            // Java: this.mParas.remove(paraIndex);
            // TODO_RUST_CODE
         }
    }

    // Java: void clear()
    pub fn clear(&self) {
         // Java: this.mParas = new ArrayList();
         // TODO_RUST_CODE
    }

    // Java: int getParaNumber()
    pub fn getParaNumber(&self) -> i32 {
         // Java: return this.mParas.size();
         // TODO_RUST_CODE
    }

    // Java: void setSubPara(int subParaIndex, int subInc)
    pub fn setSubPara(&self, int subParaIndex, int subInc) {
         // Java: int paraIndex = this.getParaIndex(subParaIndex);
         // TODO_RUST_CODE
         // Java: if (paraIndex >= 0 && paraIndex < this.mParas.size())
        // TODO_RUST_CODE
         {
            // Java: SubParaSet sub = (SubParaSet)this.mParas.get(paraIndex);
            // TODO_RUST_CODE
            // Java: sub.number += subInc;
            // TODO_RUST_CODE
            // Java: for(int i = paraIndex + 1; i < this.mParas.size(); ++i)
            // TODO_RUST_CODE
            {
               // Java: SubParaSet var10000 = (SubParaSet)this.mParas.get(i);
               // TODO_RUST_CODE
               // Java: var10000.indexBase += subInc;
               // TODO_RUST_CODE
            }
         }
    }

    // Java: int getParaIndex(int subParaIndex)
    pub fn getParaIndex(&self, int subParaIndex) -> i32 {
         // Java: int i = 0;
         // TODO_RUST_CODE
         // Java: for(int subnum = 0; i < this.mParas.size(); ++i)
        // TODO_RUST_CODE
         {
            // Java: subnum += ((SubParaSet)this.mParas.get(i)).number;
            // TODO_RUST_CODE
            // Java: if (subnum > subParaIndex || i == this.mParas.size() - 1)
            // TODO_RUST_CODE
            {
               // Java: break;
               // TODO_RUST_CODE
            }
         }
         // Java: return i;
         // TODO_RUST_CODE
    }

    // Java: SubParaSet getSubParaSet(int paraIndex)
    pub fn getSubParaSet(&self, int paraIndex) -> SubParaSet {
         // Java: return (SubParaSet)this.mParas.get(paraIndex);
         // TODO_RUST_CODE
    }

    // Java: int getSubParaIndex(int paraIndex)
    pub fn getSubParaIndex(&self, int paraIndex) -> i32 {
         // Java: return ((SubParaSet)this.mParas.get(paraIndex)).indexBase;
         // TODO_RUST_CODE
    }

    // Java: int[] getSubParaIndexOffset(int paraIndex, int offset)
    pub fn getSubParaIndexOffset(&self, int paraIndex, int offset) -> int[] {
         // Java: int subParaIndex = ((SubParaSet)this.mParas.get(paraIndex)).indexBase;
         // TODO_RUST_CODE
         // Java: int indexMax = subParaIndex + ((SubParaSet)this.mParas.get(paraIndex)).number;
         // TODO_RUST_CODE
         // Java: for(int txtlen = 0; subParaIndex < indexMax; ++subParaIndex)
        // TODO_RUST_CODE
         {
            // Java: TextData data = (TextData)TextBoxData.this.textDatas.get(subParaIndex);
            // TODO_RUST_CODE
            // Java: txtlen += data.mTypoText.length();
            // TODO_RUST_CODE
            // Java: if (txtlen > offset || indexMax - 1 == subParaIndex)
            // TODO_RUST_CODE
            {
               // Java: offset -= txtlen - data.mTypoText.length();
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            }
         }
         // Java: return new int[]{subParaIndex, offset};
         // TODO_RUST_CODE
    }

}

// Java: public class WORDART_ATTR
impl WORDART_ATTR {
    // Java: WORDART_ATTR()
    pub fn WORDART_ATTR(&self) {
         // Java: super();
         // TODO_RUST_CODE
    }

}
