#![allow(dead_code, unused_variables, unused_mut, unreachable_code, non_snake_case, unused_imports, non_camel_case_types)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

type Float = f32;
type Integer = i32;
type List<T> = Vec<T>;

struct Canvas;
struct Matrix;
struct Paint;
struct Path;
struct PointF { x: f32, y: f32 }
struct RectF { left: f32, top: f32, right: f32, bottom: f32 }
struct Direction;
struct VERSION;
struct Platform;
struct Thai;
struct UILanguage;
struct IResizeCallback;
struct DecoLineCache;
struct IHolder;
struct ParaInfo;
struct SpanInfo;
struct TextBoxData;
struct TextData;
struct TextRange;
struct TextSelectionParam;
struct TimingInfo;
struct WpsPath;
struct Emoji;
struct Layout;
struct Spannable;
struct SpannableStringBuilder;
struct AbsoluteSizeSpan;
struct CharacterStyle;
struct Bidi;
struct ArtTransform;
struct WordArtLayout;
struct TypoLayoutGen;
struct BaseTextLayout;
struct FontMetricsInt;

// Java: import android.graphics.Canvas

// Java: import android.graphics.Matrix

// Java: import android.graphics.Paint

// Java: import android.graphics.Path

// Java: import android.graphics.PointF

// Java: import android.graphics.RectF

// Java: import android.graphics.Path.Direction

// Java: import android.os.Build.VERSION

// Java: import cn.wps.core.runtime.Platform

// Java: import cn.wps.language.Thai

// Java: import cn.wps.moffice.define.UILanguage

// Java: import cn.wps.text.layout.IResizeCallback

// Java: import cn.wps.text.layout.data.DecoLineCache

// Java: import cn.wps.text.layout.data.IHolder

// Java: import cn.wps.text.layout.data.ParaInfo

// Java: import cn.wps.text.layout.data.SpanInfo

// Java: import cn.wps.text.layout.data.TextBoxData

// Java: import cn.wps.text.layout.data.TextData

// Java: import cn.wps.text.layout.data.TextRange

// Java: import cn.wps.text.layout.data.TextSelectionParam

// Java: import cn.wps.text.layout.data.TimingInfo

// Java: import cn.wps.text.layout.data.WpsPath

// Java: import cn.wps.text.layout.layouter.Emoji

// Java: import cn.wps.text.layout.typo.layout.Layout

// Java: import cn.wps.text.layout.typo.layout.Spannable

// Java: import cn.wps.text.layout.typo.layout.SpannableStringBuilder

// Java: import cn.wps.text.layout.typo.layout.style.AbsoluteSizeSpan

// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle

// Java: import java.text.Bidi

// Java: import java.util.ArrayList

// Java: import java.util.List


// Java field: int INVALID_INDEX
pub(crate) const INVALID_INDEX: i32 = -1;
// Java field: int SCALE_INIT
pub(crate) const SCALE_INIT: i32 = 0;
// Java field: int SCALE_UP
pub(crate) const SCALE_UP: i32 = 1;
// Java field: int SCALE_DOWN
pub(crate) const SCALE_DOWN: i32 = -1;
// Java field: float FLOAT_DELTA
pub(crate) const FLOAT_DELTA: f32 = 0.0001;
// Java field: int ANDROID_VERSION
pub(crate) const ANDROID_VERSION: i32 = 0;
// Java field: int ANDROID_SDK_INT_5_0
pub(crate) const ANDROID_SDK_INT_5_0: i32 = 21;
// Java field: float MAX_SCALE
pub(crate) const MAX_SCALE: f32 = 1.0;
// Java field: DecoLineCache EMPTY_DECOLINE
static EMPTY_DECOLINE: OnceLock<i32> = OnceLock::new();
// Java field: String ELLIPSIS
pub(crate) const ELLIPSIS: &str = "...";

// Java: public class RichTextLayout extends BaseTextLayout
pub struct RichTextLayout {
    // Java field: int mRotDegrees
    pub(crate) mRotDegrees: i32,
    // Java field: ArtTransform mArtTransform
    pub(crate) mArtTransform: Option<i32>,
    // Java field: List<Layout> mTypoCaches
    pub(crate) mTypoCaches: Option<Vec<i32>>,
    // Java field: WordArtLayout mWordArtTypo
    pub(crate) mWordArtTypo: Option<i32>,
    // Java field: List<List<Layout>> mLayouts
    pub(crate) mLayouts: Option<Vec<Vec<i32>>>,
    // Java field: RectF mTextPos
    mTextPos: Option<i32>,
    // Java field: List<Float> mParaBottoms
    pub(crate) mParaBottoms: Option<Vec<f32>>,
    // Java field: List<Float> mSingleLineParaWidths
    pub(crate) mSingleLineParaWidths: Option<Vec<f32>>,
    // Java field: List<DecoLineCache> mDecoLineCaches
    pub(crate) mDecoLineCaches: Option<Vec<i32>>,
    // Java field: TypoLayoutGen mTypoLayoutGen
    pub(crate) mTypoLayoutGen: Option<i32>,
    // Java field: Paint.FontMetricsInt fm
    fm: Option<i32>,
    // Java field: int mFrom
    mFrom: i32,
}

// Java: public enum SelectionType
pub struct SelectionType {
}

// Java: public class LogicalPosition
pub struct LogicalPosition {
    // Java field: int paraIndex
    pub(crate) paraIndex: i32,
    // Java field: int lineIndex
    pub(crate) lineIndex: i32,
    // Java field: int offset
    pub(crate) offset: i32,
    // Java field: int spaceNum
    pub(crate) spaceNum: i32,
    // Java field: float ascent
    pub(crate) ascent: f32,
    // Java field: boolean isAtLineEnd
    pub(crate) isAtLineEnd: bool,
    // Java field: boolean isAtShapeEnd
    pub(crate) isAtShapeEnd: bool,
    // Java field: int shapeWidth
    pub(crate) shapeWidth: i32,
    // Java field: SpanInfo span
    pub(crate) span: Option<i32>,
}

// Java: public class RichTextLayout extends BaseTextLayout
impl RichTextLayout {
    // Java: RichTextLayout()
    pub fn new(&self) {
      // Java: super();

      // Java: this.mTypoCaches = new ArrayList();

      // Java: this.mParaBottoms = new ArrayList();

      // Java: this.mSingleLineParaWidths = new ArrayList();

      // Java: this.mDecoLineCaches = new ArrayList();

    }

    // Java: void destroy()
    pub fn destroy(&self) {
      // Java: this.mTypoCaches = null;

      // Java: this.mWordArtTypo = null;

      // Java: this.mLayouts = null;

      // Java: this.mDecoLineCaches = null;

      // Java: this.mParaBottoms = null;

      // Java: this.mSingleLineParaWidths = null;

      // Java: this.mDecoLineCaches = null;

      // Java: super.destroy();

    }

    // Java: void typo(TextBoxData textBoxData, float width, float height, boolean forceTypo)
    pub fn typo(&self, _textBoxData: i32, _width: f32, _height: f32, _forceTypo: bool) {
      // Java: super.typo(textBoxData, width, height, forceTypo);

      // Java: if (this.mTypoState != -1)

      {
         // Java: if (textBoxData.presetTextWarp() != 0)

         {
            // Java: this.mArtTransform = new ArtTransform(Math.round(width), Math.round(height), this.mRotDegrees, textBoxData.getTextDatas(), this.mTypoCaches, this.mTypoOffsets);

         }
         // Java: if (textBoxData.checkWordArt())

         {
            // Java: this.mWordArtTypo = new WordArtLayout();

            // Java: this.mWordArtTypo.typo(textBoxData, this.mTypoCaches, this.mTypoOffsets, this.mArtTransform);

         }
      }
    }

    // Java: boolean correctInset(TextBoxData textBoxData, float width, float height)
    pub fn correctInset(&self, _textBoxData: i32, _width: f32, _height: f32) -> bool {
      // Java: if (!textBoxData.isVertical())

      {
         // Java: return false;

      }
      // Java: else

      {
         // Java: this.mRotDegrees = textBoxData.getRoateDegrees();

         // Java: float leftInset;

         // Java: float rightInset;

         // Java: float topInset;

         // Java: float bottomInset;

         // Java: if (90 == this.mRotDegrees)

         {
            // Java: leftInset = textBoxData.topInset();

            // Java: rightInset = textBoxData.bottomInset();

            // Java: topInset = textBoxData.rightInset();

            // Java: bottomInset = textBoxData.leftInset();

         }
         // Java: else

         {
            // Java: leftInset = textBoxData.topInset();

            // Java: rightInset = textBoxData.bottomInset();

            // Java: topInset = textBoxData.leftInset();

            // Java: bottomInset = textBoxData.rightInset();

         }
         // Java: textBoxData.setTopInset(topInset);

         // Java: textBoxData.setBottomInset(bottomInset);

         // Java: textBoxData.setLeftInset(leftInset);

         // Java: textBoxData.setRightInset(rightInset);

         // Java: return true;

      }
      false
    }

    // Java: void doTypo(TextBoxData textBoxData, boolean forceTypo)
    pub fn doTypo(&self, _textBoxData: i32, _forceTypo: bool) {
      // Java: this.mTypoState = -1;

      // Java: List<TextData> dataList = textBoxData.getTextDatas();

      // Java: int dataSize = dataList.size();

      // Java: if (textBoxData.isVerticalText())

      {
         // Java: for(int i = 0; i < dataSize; ++i)

         {
            // Java: ((TextData)dataList.get(i)).mPara.mbulletsInfo.setEastVertical(true);

            // Java: ((TextData)dataList.get(i)).mPara.mbulletsInfo.setAllVertical(textBoxData.isAllVerticalText());

         }
      }
      // Java: if (textBoxData.isNormAutoFit())

      {
         // Java: double reduction = textBoxData.lnSpcReduction();

         // Java: float fontScale = textBoxData.getFontScale();

         // Java: float unitTextSize = textBoxData.getUnitTextSize();

         // Java: textBoxData.setFontScaleRender(fontScale, fontScale);

         // Java: this.scaleTextPara(dataList, fontScale, unitTextSize, reduction, true);

      }
      // Java: boolean isMultiLine = false;

      // Java: IHolder holder = textBoxData.getIHolder();

      // Java: this.mTypoCaches.clear();

      // Java: this.mParaBottoms.clear();

      // Java: this.mSingleLineParaWidths.clear();

      // Java: this.mTypoOffsets.clear();

      // Java: this.mDecoLineCaches.clear();

      // Java: for(int i = 0; i < dataSize; ++i)

      {
         // Java: TextData data = (TextData)dataList.get(i);

         // Java: PointF typoOffset = this.typoOnePara(i, textBoxData);

         // Java: isMultiLine = isMultiLine || data.mPara.mIsWrapText || data.mPara.mIsRightToLeft;

         // Java: if (this.isAutoHeight())

         {
            // Java: Layout typoCache = (Layout)this.mTypoCaches.get(i);

            // Java: float preParaBottom = i == 0 ? 0.0F : (Float)this.mParaBottoms.get(i - 1);

            // Java: float paraBottom = (Float)this.mParaBottoms.get(i);

            // Java: if (this.isAllVertRTl())

            {
               // Java: preParaBottom = -preParaBottom;

               // Java: paraBottom = -paraBottom;

            }
            // Java: float limit = preParaBottom + (typoCache.getLineCount() > 0 ? typoCache.getLineBottom(0) : 0.0F);

            // Java: if (limit > this.mTypoMaxHeight && i > 0)

            {
               // Java: TextData lastPara = (TextData)dataList.get(i - 1);

               // Java: Layout lastTypoCache = (Layout)this.mTypoCaches.get(i - 1);

               // Java: float lastLineWidth = lastTypoCache.getLineWidth(lastTypoCache.getLineCount() - 1);

               // Java: SpanInfo lastSpan = lastPara.getLastSpan();

               // Java: float extraWidth = PaintFontHelper.getCharWidth("...".toCharArray(), lastSpan);

               // Java: if (lastLineWidth + extraWidth > this.mTypoMaxWidth)

               {
                  // Java: String typoText = lastPara.mTypoText;

                  // Java: int length = typoText.length();

                  // Java: if (length <= 2)

                  {
                     // Java: lastPara.mTypoText = typoText.substring(0, 1);

                     // Java: lastSpan.mEndPos -= length - 1;

                  }
                  // Java: else

                  {
                     // Java: lastPara.mTypoText = lastPara.mTypoText.substring(0, length - 2);

                     // Java: lastSpan.mEndPos -= 2;

                  }
               }
               // Java: lastPara.mTypoText = lastPara.mTypoText + "...";

               // Java: lastSpan.mEndPos += "...".length();

               // Java: this.removeLastTypoResult();

               // Java: this.removeLastTypoResult();

               // Java: PointF newTypoOffset = this.typoOnePara(i - 1, textBoxData);

               // Java: ((PointF)this.mTypoOffsets.get(i - 1)).set(newTypoOffset);

               // Java: break;

            }
            // Java: if (paraBottom > this.mTypoMaxHeight)

            {
               // Java: int line = 0;

               // Java: for(int lineCount = typoCache.getLineCount(); line < lineCount; ++line)

               {
                  // Java: if (preParaBottom + typoCache.getLineBottom(line) > this.mTypoMaxHeight)

                  {
                     // Java: --line;

                     // Java: break;

                  }
               }
               // Java: if (line >= 0)

               {
                  // Java: int offset = typoCache.getLineEnd(line);

                  // Java: SpanInfo span = data.getSpanByOffset(offset - 1);

                  // Java: if (span != null)

                  {
                     // Java: float lastLineWidth = typoCache.getLineWidth(line);

                     // Java: float extraWidth = PaintFontHelper.getCharWidth("...".toCharArray(), span);

                     // Java: span.mEndPos = offset + "...".length();

                     // Java: if (lastLineWidth + extraWidth < this.mTypoMaxWidth)

                     {
                        // Java: data.mTypoText = data.mTypoText.substring(0, offset) + "...";

                     }
                     // Java: else if (offset - 2 > 0)

                     {
                        // Java: data.mTypoText = data.mTypoText.substring(0, offset - 2) + "...";

                        // Java: span.mEndPos -= 2;

                     }
                     // Java: else if (data.mTypoText.length() != 0)

                     {
                        // Java: data.mTypoText = data.mTypoText.substring(0, 1) + "...";

                        // Java: span.mEndPos = data.mTypoText.length();

                     }
                     // Java: this.removeLastTypoResult();

                     // Java: typoOffset = this.typoOnePara(i, textBoxData);

                     // Java: this.mTypoOffsets.add(typoOffset);

                     // Java: break;

                  }
               }
            }
         }
         // Java: this.mTypoOffsets.add(typoOffset);

      }
      // Java: if (forceTypo)

      {
         // Java: float textTolerance = textBoxData.getTextTolerance();

         // Java: isMultiLine = this.tryTypoNormalAutoFit(textBoxData, isMultiLine);

         // Java: float textHeight = this.getTextHeight(textBoxData, true);

         // Java: float textWidth = this.getTextWidth(isMultiLine);

         // Java: if (null != holder && (textBoxData.textAutofit() == 2 || textBoxData.oldTextAutoFit() == 2) && textBoxData.presetTextWarp() == 0 && (Math.abs(textHeight - this.mTextRect.height()) > textTolerance || Math.abs(textWidth - this.mTextRect.width()) > textTolerance))

         {
            // Java: IResizeCallback callback = textBoxData.getResizeCallback();

            // Java: if (callback != null)

            {
               // Java: callback.resize(this, this.mTextRect.width(), this.mTextRect.height(), textWidth, textHeight);

               // Java: this.mTypoState = 2;

            }
         }
      }
      // Java: DecoLineCache decoLineCache = new DecoLineCache();

      // Java: for(int i = 0; i < this.mTypoCaches.size(); ++i)

      {
         // Java: TextData data = (TextData)dataList.get(i);

         // Java: Layout layout = (Layout)this.mTypoCaches.get(i);

         // Java: decoLineCache.calcDecoLine(layout, data);

         // Java: if (!decoLineCache.isEmpty())

         {
            // Java: this.mDecoLineCaches.add(decoLineCache);

            // Java: decoLineCache = new DecoLineCache();

         }
         // Java: else

         {
            // Java: this.mDecoLineCaches.add(EMPTY_DECOLINE);

         }
      }
      // Java: float textHeight = this.getTextHeight(textBoxData, false);

      // Java: float textWidth = this.getTextWidth(isMultiLine);

      // Java: this.modifyTextBoxAlign(textBoxData, textHeight);

      // Java: this.setTextRect(textWidth, textHeight);

      // Java: if (null != holder)

      {
         // Java: RectF textRect = new RectF(0.0F, ((PointF)this.mTypoOffsets.get(0)).y, this.mTypoRect.width(), ((PointF)this.mTypoOffsets.get(0)).y + textHeight);

         // Java: if (textBoxData.isVertical())

         {
            // Java: if (textBoxData.verticalText() != 2)

            {
               // Java: float left = this.mTypoRect.bottom - (textHeight + textRect.top);

               // Java: float top = 0.0F;

               // Java: float height = this.mTypoRect.width();

               // Java: float minTextHeight = this.getMaxLineHeight();

               // Java: height = height > minTextHeight ? height : minTextHeight;

               // Java: textRect.set(left, top, left + textHeight, top + height);

            }
            // Java: else

            {
               // Java: textRect.set(textRect.top, 0.0F, textRect.top + textHeight, this.mTypoRect.width());

            }
         }
         // Java: holder.setTextRect(textRect);

      }
      // Java: if (-1 == this.mTypoState)

      {
         // Java: boolean isChangeAutoFitType = textBoxData.updateAuotfit();

         // Java: this.mTypoState = isChangeAutoFitType ? 3 : 1;

      }
    }

    // Java: boolean tryTypoNormalAutoFit(TextBoxData textBoxData, boolean isMultiLine)
    pub fn tryTypoNormalAutoFit(&self, _textBoxData: i32, _isMultiLine: bool) -> bool {
      // Java: int scaleType = 0;

      // Java: boolean needCheckSpcPct = true;

      // Java: List<TextData> dataList = textBoxData.getTextDatas();

      // Java: if (dataList != null && !dataList.isEmpty())

      {
         // Java: float unitFontSize = textBoxData.getUnitTextSize();

         // Java: float textTolerance = textBoxData.getTextTolerance();

         // Java: float textHeight = this.getTextHeight(textBoxData, true);

         // Java: float textWidth = this.getTextWidth(isMultiLine);

         // Java: while(textBoxData.isNormAutoFit() && textBoxData.presetTextWarp() == 0 && textBoxData.inScaleTimes() && (scaleType == 0 || textBoxData.inScaleRender()))

         {
            // Java: float lines = (float)this.getLineCount();

            // Java: float minFontSize = textBoxData.getMinTextSize();

            // Java: boolean isWrapText = textBoxData.isWrapTextInShape();

            // Java: float scaleFontSize;

            // Java: if (this.compareTextBoxSize(textWidth, textHeight, textTolerance, isWrapText, true))

            {
               // Java: if (scaleType == 1)

               {
                  // Java: needCheckSpcPct = false;

               }
               // Java: scaleFontSize = minFontSize - 1.0F;

               // Java: scaleType = -1;

            }
            // Java: else

            {
               // Java: if (!this.compareTextBoxSize(textWidth, textHeight, textTolerance, isWrapText, false) || scaleType == -1)

               {
                  // Java: break;

               }
               // Java: scaleFontSize = minFontSize + 1.0F;

               // Java: scaleType = 1;

            }
            // Java: if (scaleFontSize > textBoxData.getOriginTextSize())

            {
               // Java: if (!(scaleFontSize - 0.5F < textBoxData.getOriginTextSize()))

               {
                  // Java: if (!this.modifyLineSpace(textBoxData, dataList, true))

                  {
                     // Java: break;

                  }
                  // Java: isMultiLine = this.typoParaAgain(isMultiLine, textBoxData);

                  // Java: textHeight = this.getTextHeight(textBoxData, true);

                  // Java: textWidth = this.getTextWidth(isMultiLine);

                  // Java: if (this.compareTextBoxSize(textWidth, textHeight, textTolerance, isWrapText, false))

                  {
                     // Java: this.modifyLineSpace(textBoxData, dataList, false);

                     // Java: this.typoParaAgain(isMultiLine, textBoxData);

                     // Java: break;

                  }
                  // Java: textBoxData.addScaleTimes();

                  // Java: continue;

               }
               // Java: scaleFontSize = textBoxData.getOriginTextSize();

            }
            // Java: float oldScale = textBoxData.getCurrentFontScale();

            // Java: float step = scaleFontSize / minFontSize;

            // Java: float wScale = this.mTextRect.width() / textWidth;

            // Java: float hScale = this.mTextRect.height() / textHeight;

            // Java: float expectScale = (float)Math.sqrt((double)(wScale * hScale));

            // Java: float textScale = lines > 1.0F ? step : expectScale;

            // Java: float scale = textBoxData.setFontScaleRender(textScale, expectScale);

            // Java: if (scaleType == -1 && Math.abs(oldScale - textBoxData.minFontScale()) < 1.0E-4F && Math.abs(textBoxData.getCurrentFontScale() - textBoxData.minFontScale()) < 1.0E-4F)

            {
               // Java: if (!this.modifyLineSpace(textBoxData, dataList, false))

               {
                  // Java: break;

               }
               // Java: isMultiLine = this.typoParaAgain(isMultiLine, textBoxData);

            }
            // Java: else

            {
               // Java: this.scaleTextPara(dataList, scale, unitFontSize, (double)0.0F, false);

               // Java: isMultiLine = this.typoParaAgain(isMultiLine, textBoxData);

            }
            // Java: textHeight = this.getTextHeight(textBoxData, true);

            // Java: textWidth = this.getTextWidth(isMultiLine);

            // Java: boolean needContinue = false;

            // Java: if (this.compareTextBoxSize(textWidth, textHeight, textTolerance, isWrapText, true))

            {
               // Java: if (scaleType == 1)

               {
                  // Java: textBoxData.setFontScaleRender(1.0F / scale, 1.0F / expectScale);

                  // Java: this.scaleTextPara(dataList, 1.0F / scale, unitFontSize, (double)0.0F, false);

                  // Java: isMultiLine = this.typoParaAgain(isMultiLine, textBoxData);

               }
               // Java: else if (needCheckSpcPct)

               {
                  // Java: needContinue = true;

                  // Java: if (this.modifyLineSpace(textBoxData, dataList, false))

                  {
                     // Java: isMultiLine = this.typoParaAgain(isMultiLine, textBoxData);

                  }
               }
            }
            // Java: else if (this.compareTextBoxSize(textWidth, textHeight, textTolerance, isWrapText, false))

            {
               // Java: needContinue = scaleType == 1;

            }
            // Java: if (!needContinue)

            {
               // Java: break;

            }
            // Java: textHeight = this.getTextHeight(textBoxData, true);

            // Java: textWidth = this.getTextWidth(isMultiLine);

            // Java: textBoxData.addScaleTimes();

         }
         // Java: return isMultiLine;

      }
      // Java: else

      {
         // Java: return isMultiLine;

      }
      false
    }

    // Java: boolean modifyLineSpace(TextBoxData textBoxData, List<TextData> dataList, boolean add)
    pub fn modifyLineSpace(&self, _textBoxData: i32, _dataList: i32, _add: bool) -> bool {
      // Java: double reduction = textBoxData.lnSpcReduction();

      // Java: if (!textBoxData.hasLnSpcReductionChange() && reduction <= (double)0.0F && !textBoxData.isPhTitle())

      {
         // Java: reduction = (double)0.0F;

      }
      // Java: double delta = (double)0.0F;

      // Java: if (add)

      {
         // Java: if (reduction > (double)0.0F)

         {
            // Java: double value = reduction - (double)0.1F;

            // Java: if (value >= (double)0.0F)

            {
               // Java: delta = (double)-0.1F;

            }
            // Java: else if (value < (double)0.0F)

            {
               // Java: delta = value > (double)-1.0E-4F ? (double)0.0F : -reduction;

            }
         }
         // Java: else if (reduction < (double)0.0F)

         {
            // Java: delta = -reduction;

         }
      }
      // Java: else if (reduction + (double)0.1F <= (double)0.2001F)

      {
         // Java: delta = (double)0.1F;

      }
      // Java: else if ((double)0.2F - reduction > (double)1.0E-4F)

      {
         // Java: delta = (double)0.2F - reduction;

      }
      // Java: if (Math.abs(delta) < (double)1.0E-4F)

      {
         // Java: return false;

      }
      // Java: else

      {
         // Java: boolean hasScaleSpc = false;

         // Java: for(int i = 0; i < dataList.size(); ++i)

         {
            // Java: ParaInfo para = ((TextData)dataList.get(i)).mPara;

            // Java: double limit = para.mSpcWithinPctOri + (double)(add ? 1.0E-4F : -1.0E-4F);

            // Java: double value = para.mSpcWithinPct + -delta;

            // Java: if (add && value <= limit || !add && value >= limit - (double)0.2F)

            {
               // Java: para.mSpcWithinPct = value;

               // Java: hasScaleSpc = true;

            }
         }
         // Java: if (hasScaleSpc)

         {
            // Java: textBoxData.changeLnSpcReduction(true);

            // Java: textBoxData.setLnSpcReduction(reduction + delta);

         }
         // Java: return hasScaleSpc;

      }
      false
    }

    // Java: boolean compareTextBoxSize(float width, float height, float limit, boolean wrap, boolean toBox)
    pub fn compareTextBoxSize(&self, _width: f32, _height: f32, _limit: f32, _wrap: bool, _toBox: bool) -> bool {
      // Java: if (toBox)

      {
         // Java: return !wrap ? height - this.mTextRect.height() > limit : width - this.mTextRect.width() > limit || height - this.mTextRect.height() > limit;

      }
      // Java: else

      {
         // Java: return !wrap ? this.mTextRect.height() - height > limit : this.mTextRect.width() - width > limit || this.mTextRect.height() - height > limit;

      }
      false
    }

    // Java: void scaleTextPara(List<TextData> dataList, float scale, float unitFontSize, double reduction, boolean needFixFontSize)
    pub fn scaleTextPara(&self, _dataList: i32, _scale: f32, _unitFontSize: f32, _reduction: f64, _needFixFontSize: bool) {
      // Java: int dataSize = dataList != null ? dataList.size() : 0;

      // Java: for(int i = 0; i < dataSize; ++i)

      {
         // Java: TextData data = (TextData)dataList.get(i);

         // Java: ParaInfo para = data.mPara;

         // Java: para.mbulletsInfo.setBulletSize(para.mbulletsInfo.getBulletSize() * scale);

         // Java: para.mSpcAft *= (double)scale;

         // Java: para.mSpcBef *= (double)scale;

         // Java: para.mDefTabSize = (int)((float)para.mDefTabSize * scale);

         // Java: SpanInfo span = data.mFirstSpan;

         // Java: if (needFixFontSize)

         {
            // Java: span.mFontSize = (float)Math.round(span.mFontSize);

         }
         // Java: span.mFontSize = (float)Math.round(span.mFontSize * scale);

         // Java: if (span.mFontSize > span.mFontSizeOri)

         {
            // Java: span.mFontSize = span.mFontSizeOri;

         }
         // Java: if (span.mFontSize < unitFontSize)

         {
            // Java: span.mFontSize = unitFontSize;

         }
         // Java: span.resetAscent();

         // Java: int j = 0;

         // Java: for(int len = data.mOtherSpans.size(); j < len; ++j)

         {
            // Java: span = (SpanInfo)data.mOtherSpans.get(j);

            // Java: if (needFixFontSize)

            {
               // Java: span.mFontSize = (float)Math.round(span.mFontSize);

            }
            // Java: span.mFontSize = (float)Math.round(span.mFontSize * scale);

            // Java: if (span.mFontSize > span.mFontSizeOri)

            {
               // Java: span.mFontSize = span.mFontSizeOri;

            }
            // Java: if (span.mFontSize < unitFontSize)

            {
               // Java: span.mFontSize = unitFontSize;

            }
            // Java: span.resetAscent();

         }
         // Java: if (reduction > (double)0.0F && para.mSpcWithinPct - (double)0.2F > (double)1.0E-4F)

         {
            // Java: para.mSpcWithinPct -= reduction;

         }
      }
    }

    // Java: boolean typoParaAgain(boolean isMultiLine, TextBoxData textBoxData)
    pub fn typoParaAgain(&self, _isMultiLine: bool, _textBoxData: i32) -> bool {
      // Java: this.mTypoCaches.clear();

      // Java: this.mParaBottoms.clear();

      // Java: this.mSingleLineParaWidths.clear();

      // Java: this.mTypoOffsets.clear();

      // Java: this.mDecoLineCaches.clear();

      // Java: List<TextData> dataList = textBoxData.getTextDatas();

      // Java: int dataSize = dataList.size();

      // Java: boolean multiLine = isMultiLine;

      // Java: for(int i = 0; i < dataSize; ++i)

      {
         // Java: TextData data = (TextData)dataList.get(i);

         // Java: multiLine = isMultiLine || data.mPara.mIsWrapText || data.mPara.mIsRightToLeft;

         // Java: PointF typoOffset = this.typoOnePara(i, textBoxData);

         // Java: this.mTypoOffsets.add(typoOffset);

      }
      // Java: return multiLine;

      false
    }

    // Java: void removeLastTypoResult()
    pub fn removeLastTypoResult(&self) {
      // Java: this.mTypoCaches.remove(this.mTypoCaches.size() - 1);

      // Java: this.mParaBottoms.remove(this.mParaBottoms.size() - 1);

      // Java: this.mSingleLineParaWidths.remove(this.mSingleLineParaWidths.size() - 1);

    }

    // Java: void setFrom(int from)
    pub fn setFrom(&self, _from: i32) {
      // Java: this.mFrom = from;

    }

    // Java: PointF typoOnePara(int paraIndex, TextBoxData textBoxData)
    pub fn typoOnePara(&self, _paraIndex: i32, _textBoxData: i32) -> i32 {
      // Java: RectF textRect = new RectF(this.mTextRect);

      // Java: List<TextData> datas = textBoxData.getTextDatas();

      // Java: TextData data = (TextData)datas.get(paraIndex);

      // Java: int firstLead = 0;

      // Java: int restLead = 0;

      // Java: int bulletMoveL = 0;

      // Java: if (data.mPara.mIndent >= 0)

      {
         // Java: boolean var10000 = false;

      }
      // Java: else

      {
         // Java: int var31 = -data.mPara.mIndent;

      }
      // Java: if (data.mPara.mIndent < 0)

      {
         // Java: boolean var32 = false;

      }
      // Java: else

      {
         // Java: short var33 = data.mPara.mIndent;

      }
      // Java: int marginL = data.mPara.mMarginL;

      // Java: double spcBef = paraIndex > 0 ? data.mPara.mSpcBef : (double)0.0F;

      // Java: double lastSpcAft = paraIndex > 0 ? ((TextData)datas.get(paraIndex - 1)).mPara.mSpcAft : (double)0.0F;

      // Java: data.mVertical = textBoxData.verticalText();

      // Java: float mBulletsWid = 0.0F;

      // Java: if (data.hasBullets() && !"\r".equals(data.mSourceText))

      {
         // Java: data.mPara.mbulletsInfo.setRotateDegrees(this.mRotDegrees);

         // Java: data.mPara.mbulletsInfo.calBulletsWidAndHei(data.mFirstSpan.mFontSize, this.isAllVert());

         // Java: mBulletsWid = data.mPara.mbulletsInfo.getWidth();

      }
      // Java: if (data.mPara.mbulletsInfo.isRightToLeft())

      {
         // Java: if (textRect.right > this.mTextRect.right)

         {
            // Java: textRect.right = this.mTextRect.right;

         }
      }
      // Java: else if (textRect.left < this.mTextRect.left)

      {
         // Java: textRect.left = this.mTextRect.left;

      }
      // Java: int leadMargin = data.mPara.mIndent + data.mPara.mMarginL;

      // Java: if (leadMargin < 0)

      {
         // Java: leadMargin = 0;

      }
      // Java: firstLead = leadMargin;

      // Java: if (mBulletsWid > 0.0F)

      {
         // Java: if (data.mPara.mIndent == 0)

         {
            // Java: firstLead = (int)((float)leadMargin + mBulletsWid);

         }
         // Java: if (data.mPara.mIndent < 0)

         {
            // Java: firstLead = (float)(-data.mPara.mIndent) > mBulletsWid ? -data.mPara.mIndent : (int)mBulletsWid;

            // Java: firstLead = firstLead > data.mPara.mMarginL ? firstLead : data.mPara.mMarginL;

            // Java: if ((float)(-data.mPara.mIndent) < mBulletsWid && !data.mPara.mIsRightToLeft)

            {
               // Java: firstLead = (int)((float)firstLead + (float)data.mPara.mIndent + mBulletsWid);

            }
         }
         // Java: if (data.mPara.mIndent > 0 && (float)data.mPara.mIndent < mBulletsWid)

         {
            // Java: firstLead = (int)((float)firstLead + (mBulletsWid - (float)data.mPara.mIndent));

         }
      }
      // Java: if (data.mPara.mIndent >= 0)

      {
         // Java: restLead = data.mPara.mMarginL;

      }
      // Java: else

      {
         // Java: restLead = data.mPara.mMarginL;

      }
      // Java: if (textRect.right <= textRect.left && !this.isAutoWidth())

      {
         // Java: textRect.right = textRect.left + 1.0F;

      }
      // Java: if (textRect.bottom <= textRect.top && !this.isAutoWidth())

      {
         // Java: textRect.bottom = textRect.top + 1.0F;

      }
      // Java: data.mPara.mFirstLead = firstLead;

      // Java: data.mPara.mRestLead = restLead;

      // Java: Layout typoCache = this.mTypoLayoutGen.getLayout(textRect, this.mTextPaint, data, firstLead, restLead, this.mTypoMaxWidth);

      // Java: if (typoCache == null)

      {
         // Java: return new PointF();

      }
      // Java: else

      {
         // Java: this.mSingleLineParaWidths.add(typoCache.getLineWidth(0) + (textRect.left - this.mTextRect.left));

         // Java: boolean noLineSpace = data.mPara.mSpcWithinPct == (double)0.0F && data.mPara.mSpcWithinPts == (double)0.0F;

         // Java: boolean preHasLineSpace = false;

         // Java: for(int i = paraIndex - 1; i >= 0; --i)

         {
            // Java: TextData textData = textBoxData.getTextData(i);

            // Java: if (textData.mPara.mSpcWithinPct != (double)0.0F || textData.mPara.mSpcWithinPts != (double)0.0F)

            {
               // Java: preHasLineSpace = true;

               // Java: break;

            }
         }
         // Java: float preParaBottom = paraIndex == 0 || !preHasLineSpace && noLineSpace ? (this.isAllVertRTl() ? textRect.bottom : textRect.top) : (Float)this.mParaBottoms.get(paraIndex - 1);

         // Java: PointF typoOffset = this.setTextAlignment(paraIndex, data.mPara, textRect, this.mTextPaint, (float)typoCache.getWidth(), preParaBottom);

         // Java: if (this.isAllVertRTl())

         {
            // Java: typoOffset.y = (float)((double)typoOffset.y - (spcBef + lastSpcAft));

         }
         // Java: else

         {
            // Java: typoOffset.y = (float)((double)typoOffset.y + spcBef + lastSpcAft);

         }
         // Java: if (noLineSpace)

         {
            // Java: typoOffset.y -= typoCache.getHeight() * 0.8F;

         }
         // Java: float textWid = typoCache.getLineMax(0);

         // Java: if (mBulletsWid > 0.0F)

         {
            // Java: if (data.mPara.mIndent > 0)

            {
               // Java: bulletMoveL = -data.mPara.mMarginL;

            }
            // Java: else if (data.mPara.mMarginL > -data.mPara.mIndent)

            {
               // Java: bulletMoveL = -(data.mPara.mMarginL + data.mPara.mIndent);

            }
            // Java: else

            {
               // Java: bulletMoveL = 0;

            }
            // Java: float firstLineHeight = typoCache.getLineBottom(0) - typoCache.getLineTop(0);

            // Java: data.mPara.mbulletsInfo.calBulletsXAndY(textRect, (float)bulletMoveL, textWid, data.mPara.mHAlignment, typoOffset.y, typoCache.getLineBaseline(0), data.mFirstSpan.mFontSize, data.mPara.mIsRightToLeft, this.mTextBoxData.verticalText(), firstLineHeight);

         }
         // Java: if (this.isAllVertRTl())

         {
            // Java: this.mParaBottoms.add(paraIndex, typoOffset.y - typoCache.getHeight());

         }
         // Java: else

         {
            // Java: this.mParaBottoms.add(paraIndex, typoOffset.y + typoCache.getHeight());

         }
         // Java: this.mTypoCaches.add(paraIndex, typoCache);

         // Java: return typoOffset;

      }
      0
    }

    // Java: int getLineCount()
    pub fn getLineCount(&self) -> i32 {
      // Java: int lines = 1;

      // Java: int i = 0;

      // Java: for(int size = this.mTypoCaches.size(); i < size; ++i)

      {
         // Java: lines += ((Layout)this.mTypoCaches.get(i)).getLineCount();

      }
      // Java: return lines;

      0
    }

    // Java: float getMaxTextWidth()
    pub fn getMaxTextWidth(&self) -> f32 {
      // Java: float width = 0.0F;

      // Java: int i = 0;

      // Java: for(int size = this.mTypoCaches.size(); i < size; ++i)

      {
         // Java: Layout layout = (Layout)this.mTypoCaches.get(i);

         // Java: float w = (float)layout.getWidth();

         // Java: width = w > width ? w : width;

      }
      // Java: return width;

      0.0
    }

    // Java: float getMaxLineHeight()
    pub fn getMaxLineHeight(&self) -> f32 {
      // Java: float height = 0.0F;

      // Java: int i = 0;

      // Java: for(int size = this.mTypoCaches.size(); i < size; ++i)

      {
         // Java: Layout layout = (Layout)this.mTypoCaches.get(i);

         // Java: int j = 0;

         // Java: for(int count = layout.getLineCount(); j < count; ++j)

         {
            // Java: float h = layout.getLineBottom(j) - layout.getLineTop(j);

            // Java: height = h > height ? h : height;

         }
      }
      // Java: return height;

      0.0
    }

    // Java: float getTextHeight(TextBoxData textBoxData, boolean isForAutoFit)
    pub fn getTextHeight(&self, _textBoxData: i32, _isForAutoFit: bool) -> f32 {
      // Java: int lastPara = this.mParaBottoms.size() - 1;

      // Java: if (isForAutoFit && textBoxData.isNormAutoFit() && !textBoxData.isPhTitle())

      {
         // Java: for(List<TextData> datas = textBoxData.getTextDatas(); lastPara > 0; --lastPara)

         {
            // Java: TextData data = (TextData)datas.get(lastPara);

            // Java: if (data.hasText())

            {
               // Java: break;

            }
         }
      }
      // Java: return this.isAllVertRTl() ? (Float)this.mParaBottoms.get(0) - (Float)this.mParaBottoms.get(lastPara) + ((Layout)this.mTypoCaches.get(0)).getHeight() : (Float)this.mParaBottoms.get(lastPara) - this.mTextRect.top;

      0.0
    }

    // Java: float getTextWidth(boolean isMultiLine)
    pub fn getTextWidth(&self, _isMultiLine: bool) -> f32 {
      // Java: float textWidth = this.mTextRect.width();

      // Java: if (!isMultiLine)

      {
         // Java: textWidth = 0.0F;

         // Java: for(int i = 0; i < this.mSingleLineParaWidths.size(); ++i)

         {
            // Java: float lineWidth = (Float)this.mSingleLineParaWidths.get(i);

            // Java: if (textWidth < lineWidth)

            {
               // Java: textWidth = lineWidth;

            }
         }
      }
      // Java: else if (this.isAutoWidth())

      {
         // Java: textWidth = 0.0F;

         // Java: for(Layout layout : this.mTypoCaches)

         {
            // Java: int i = 0;

            // Java: for(int lineCount = layout.getLineCount(); i < lineCount; ++i)

            {
               // Java: float lineWidth = layout.getLineWidth(i);

               // Java: if (textWidth < lineWidth)

               {
                  // Java: textWidth = lineWidth;

               }
            }
         }
      }
      // Java: return textWidth;

      0.0
    }

    // Java: void renderText(Canvas canvas)
    pub fn renderText(&self, _canvas: i32) {
      // Java: int paraLen = this.mTypoCaches.size();

      // Java: this.renderText(canvas, 0, paraLen, 0, ((Layout)this.mTypoCaches.get(paraLen - 1)).getText().length(), (TimingInfo)null, false);

    }

    // Java: protected <T> void renderText(Canvas canvas, int startPara, int endPara, int startOffset, int endOffset, TimingInfo timInfo, boolean isDragShadow)
    pub fn renderText_2(&self, _canvas: i32, _startPara: i32, _endPara: i32, _startOffset: i32, _endOffset: i32, _timInfo: i32, _isDragShadow: bool) {
      // Java: if (startPara >= 0 && startPara <= endPara - 1 && endPara <= this.mTypoCaches.size())

      {
         // Java: Spannable firstText = (Spannable)((Layout)this.mTypoCaches.get(startPara)).getText();

         // Java: Spannable lastText = (Spannable)((Layout)this.mTypoCaches.get(endPara - 1)).getText();

         // Java: CharacterStyle startMaskSpan = WpsHideSpan.Factory.create();

         // Java: CharacterStyle endMaskSpan = WpsHideSpan.Factory.create();

         // Java: if (startOffset < 0)

         {
            // Java: startOffset = 0;

         }
         // Java: if (startOffset > firstText.length())

         {
            // Java: startOffset = firstText.length();

         }
         // Java: if (endOffset < 0)

         {
            // Java: endOffset = 0;

         }
         // Java: if (endOffset > lastText.length())

         {
            // Java: endOffset = lastText.length();

         }
         // Java: if (firstText == lastText && startOffset > endOffset)

         {
            // Java: startOffset = endOffset;

         }
         // Java: canvas.save();

         // Java: if (this.mTextBoxData.needClip && this.mTextBoxData.getTextVertOverflow() == 1)

         {
            // Java: canvas.clipRect(this.mTypoRect);

         }
         // Java: if (this.mFrom != 1)

         {
            // Java: float width;

            // Java: if (this.isAutoWidth())

            {
               // Java: width = 0.0F;

            }
            // Java: else

            {
               // Java: width = this.mTypoRect.width();

            }
            // Java: float height;

            // Java: if (this.isAutoHeight())

            {
               // Java: height = this.getTextRect().width();

            }
            // Java: else

            {
               // Java: height = this.mTypoRect.height();

            }
            // Java: switch (this.mRotDegrees)

            {
               // Java: case 90:

                  // Java: canvas.translate(height, 0.0F);

                  // Java: canvas.rotate(90.0F, 0.0F, 0.0F);

                  // Java: break;

               // Java: case 270:

                  // Java: canvas.translate(0.0F, width);

                  // Java: canvas.rotate(270.0F, 0.0F, 0.0F);

            }
         }
         // Java: if (this.mTextBoxData.presetTextWarp() != 0 && null != this.mArtTransform)

         {
            // Java: this.mArtTransform.squareText(canvas);

         }
         // Java: boolean wordartRender = this.hasWordArtTypo();

         // Java: if (timInfo != null)

         {
            // Java: this.renderTiming(timInfo, wordartRender);

         }
         // Java: try

         {
            // Java: if (wordartRender)

            {
               // Java: this.mWordArtTypo.beforeRender(timInfo, this.mTextBoxData.getTextDatas());

               // Java: this.mWordArtTypo.render(canvas, this.mTypoCaches, startPara, startOffset, endPara, endOffset, this.mTypoOffsets, this.mDecoLineCaches, this, isDragShadow);

               // Java: this.mWordArtTypo.afterRender(timInfo, this.mTextBoxData.getTextDatas());

            }
            // Java: else

            {
               // Java: if (startOffset > 0)

               {
                  // Java: firstText.setSpan(startMaskSpan, 0, startOffset, 34);

               }
               // Java: if (endOffset < lastText.length())

               {
                  // Java: lastText.setSpan(endMaskSpan, endOffset, lastText.length(), 34);

               }
               // Java: List<TextData> datas = this.mTextBoxData.getTextDatas();

               // Java: float maxWidth = this.getMaxTextWidth();

               // Java: boolean isWidthAutoAdjustSize = this.isAutoWidth();

               // Java: for(int i = startPara; i < endPara && i < this.mTypoCaches.size(); ++i)

               {
                  // Java: float translateX = this.isAllVert() ? ((PointF)this.mTypoOffsets.get(i)).y : ((PointF)this.mTypoOffsets.get(i)).x;

                  // Java: float translateY = this.isAllVert() ? ((PointF)this.mTypoOffsets.get(i)).x : ((PointF)this.mTypoOffsets.get(i)).y;

                  // Java: if (isWidthAutoAdjustSize)

                  {
                     // Java: if (this.isAllVertRTl())

                     {
                        // Java: translateX += this.mTextRect.height();

                     }
                     // Java: else

                     {
                        // Java: int hAlignment = ((TextData)datas.get(i)).mPara.mHAlignment;

                        // Java: switch (hAlignment)

                        {
                           // Java: case 2:

                           // Java: case 7:

                              // Java: translateX += (maxWidth - (float)((Layout)this.mTypoCaches.get(i)).getWidth()) / 2.0F;

                              // Java: break;

                           // Java: case 3:

                              // Java: translateY += maxWidth - (float)((Layout)this.mTypoCaches.get(i)).getWidth();

                        }
                     }
                  }
                  // Java: canvas.translate(translateX, translateY);

                  // Java: ((Layout)this.mTypoCaches.get(i)).draw(canvas, (Path)null, (Paint)null, 0);

                  // Java: if (this.mTextBoxData.verticalText() == 4)

                  {
                     // Java: int start = i == startPara ? startOffset : -1;

                     // Java: int end = i == endPara - 1 ? endOffset : -1;

                     // Java: this.drawChineseVertText((Layout)this.mTypoCaches.get(i), (TextData)datas.get(i), canvas, start, end);

                  }
                  // Java: if (this.mDecoLineCaches != null && i < this.mDecoLineCaches.size())

                  {
                     // Java: DecoLineCache decoLineCache = (DecoLineCache)this.mDecoLineCaches.get(i);

                     // Java: if (decoLineCache != null && !decoLineCache.isEmpty())

                     {
                        // Java: List<RectF> selRects = this.getSelRects(startPara, endPara, startOffset, endOffset, i);

                        // Java: decoLineCache.draw(canvas, selRects, this.isAllVert());

                     }
                  }
                  // Java: canvas.translate(-translateX, -translateY);

               }
               // Java: if (startOffset > 0)

               {
                  // Java: firstText.removeSpan(startMaskSpan);

               }
               // Java: if (endOffset < lastText.length())

               {
                  // Java: lastText.removeSpan(endMaskSpan);

               }
            }
         }
         // Java: catch (Exception e)

         {
            // Java: e.printStackTrace();

         }
         // Java: canvas.restore();

      }
    }

    // Java: void renderText(Canvas canvas, int startTextRange, int endTextRange)
    pub fn renderText_3(&self, _canvas: i32, _startTextRange: i32, _endTextRange: i32) {
      // Java: if (startTextRange < endTextRange && endTextRange >= 0 && null != this.mTextBoxData && !this.mTextBoxData.isEmptyText())

      {
         // Java: int startPara = -1;

         // Java: int endPara = -1;

         // Java: int startOffset = -1;

         // Java: int endOffset = -1;

         // Java: int textOffset = 0;

         // Java: List<TextData> datas = this.mTextBoxData.getTextDatas();

         // Java: for(int paraId = 0; paraId < datas.size(); ++paraId)

         {
            // Java: if (datas.get(paraId) != null && ((TextData)datas.get(paraId)).mSourceText != null)

            {
               // Java: int sourceTextLength = ((TextData)datas.get(paraId)).mSourceText.length();

               // Java: textOffset += sourceTextLength;

               // Java: if (startPara == -1 && startTextRange < textOffset)

               {
                  // Java: startPara = paraId;

                  // Java: startOffset = startTextRange - (textOffset - sourceTextLength);

               }
               // Java: if (endTextRange <= textOffset)

               {
                  // Java: endPara = paraId + 1;

                  // Java: endOffset = endTextRange - (textOffset - sourceTextLength);

                  // Java: break;

               }
            }
         }
         // Java: this.renderText(canvas, startPara, endPara, startOffset, endOffset, (TimingInfo)null, true);

      }
    }

    // Java: void renderTiming(TimingInfo timInfo, boolean wordartRender)
    pub fn renderTiming(&self, _timInfo: i32, _wordartRender: bool) {

    }

    // Java: List<RectF> getSelRects(int startPara, int endPara, int startOffset, int endOffset, int currentPara)
    pub fn getSelRects(&self, _startPara: i32, _endPara: i32, _startOffset: i32, _endOffset: i32, _currentPara: i32) -> Vec<i32> {
      // Java: int startOffsetPara = 0;

      // Java: int endOffset_para = this.mTextBoxData.getTextData(currentPara).mTypoText.length();

      // Java: if (currentPara == startPara)

      {
         // Java: startOffsetPara = startOffset;

      }
      // Java: if (currentPara == endPara - 1)

      {
         // Java: endOffset_para = endOffset;

      }
      // Java: List<RectF> selRects = this.getSelRects(currentPara, startOffsetPara, currentPara, endOffset_para, false, false);

      // Java: if (this.mRotDegrees == 90)

      {
         // Java: Matrix matrix = new Matrix();

         // Java: float[] values = new float[]{0.0F, -1.0F, this.mTypoRect.height(), 1.0F, 0.0F, 0.0F, 0.0F, 0.0F, 1.0F};

         // Java: matrix.setValues(values);

         // Java: Matrix inverse = new Matrix();

         // Java: matrix.invert(inverse);

         // Java: for(int idx = 0; idx < selRects.size(); ++idx)

         {
            // Java: RectF rectTmp = new RectF();

            // Java: inverse.mapRect(rectTmp, (RectF)selRects.get(idx));

            // Java: selRects.set(idx, rectTmp);

         }
      }
      // Java: else if (this.mRotDegrees == 270)

      {
         // Java: Matrix matrix = new Matrix();

         // Java: float[] values = new float[]{0.0F, 1.0F, 0.0F, -1.0F, 0.0F, this.mTypoRect.width(), 0.0F, 0.0F, 1.0F};

         // Java: matrix.setValues(values);

         // Java: Matrix inverse = new Matrix();

         // Java: matrix.invert(inverse);

         // Java: for(int idx = 0; idx < selRects.size(); ++idx)

         {
            // Java: RectF rectTmp = new RectF();

            // Java: inverse.mapRect(rectTmp, (RectF)selRects.get(idx));

            // Java: selRects.set(idx, rectTmp);

         }
      }
      // Java: for(int idx = 0; idx < selRects.size(); ++idx)

      {
         // Java: ((RectF)selRects.get(idx)).offset(-((PointF)this.mTypoOffsets.get(currentPara)).x, -((PointF)this.mTypoOffsets.get(currentPara)).y);

      }
      // Java: return selRects;

      Vec::new()
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: super.clear();

      // Java: this.mRotDegrees = 0;

      // Java: this.mTypoCaches.clear();

      // Java: this.mParaBottoms.clear();

      // Java: this.mSingleLineParaWidths.clear();

      // Java: this.mDecoLineCaches.clear();

      // Java: this.mTypoOffsets.clear();

      // Java: this.mWordArtTypo = null;

    }

    // Java: int[] getTextLayoutPos(int offset)
    pub fn getTextLayoutPos(&self, _offset: i32) -> Vec<i32> {
      // Java: if (this.mTypoState != 1)

      {
         // Java: return null;

      }
      // Java: else

      {
         // Java: int[] pos = new int[3];

         // Java: LogicalPosition lp = this.getLogPosByOffset(offset);

         // Java: pos[0] = lp.paraIndex;

         // Java: pos[1] = lp.lineIndex;

         // Java: Layout layout = (Layout)this.mTypoCaches.get(lp.paraIndex);

         // Java: pos[3] = lp.offset - layout.getLineStart(lp.lineIndex);

         // Java: return pos;

      }
      Vec::new()
    }

    // Java: int getTextGlobalPos(int paraIndex, int lineIndex, int offsetInLine)
    pub fn getTextGlobalPos(&self, _paraIndex: i32, _lineIndex: i32, _offsetInLine: i32) -> i32 {
      // Java: List<TextData> datas = this.mTextBoxData.getTextDatas();

      // Java: if (paraIndex >= 0 && paraIndex < datas.size() && offsetInLine >= 0)

      {
         // Java: int offset = offsetInLine;

         // Java: for(int i = 0; i < paraIndex; ++i)

         {
            // Java: offset += ((TextData)datas.get(i)).mSourceText.length();

         }
         // Java: Layout layout = (Layout)this.mTypoCaches.get(paraIndex);

         // Java: if (lineIndex >= layout.getLineCount())

         {
            // Java: return -1;

         }
         // Java: else

         {
            // Java: offset += layout.getLineStart(lineIndex);

            // Java: return offset;

         }
      }
      // Java: else

      {
         // Java: return -1;

      }
      0
    }

    // Java: RectF getTextRect()
    pub fn getTextRect(&self) -> i32 {
      // Java: return this.mTextBoxData.isVertical() ? new RectF(0.0F, 0.0F, this.mTextRect.height(), this.mTextRect.width()) : this.mTextRect;

      0
    }

    // Java: float calcHorizAlign(int paraIndex, ParaInfo paraInfo, Paint paint, float cellWidth, float typoWidth)
    pub fn calcHorizAlign(&self, _paraIndex: i32, _paraInfo: i32, _paint: i32, _cellWidth: f32, _typoWidth: f32) -> f32 {
      // Java: float offset = 0.0F;

      // Java: boolean isRight = false;

      // Java: switch (paraInfo.mHAlignment)

      {
         // Java: case 0:

         // Java: case 1:

         // Java: case 4:

         // Java: default:

            // Java: break;

         // Java: case 2:

         // Java: case 6:

         // Java: case 7:

            // Java: offset = (cellWidth - typoWidth) / 2.0F;

            // Java: break;

         // Java: case 3:

            // Java: isRight = true;

            // Java: break;

         // Java: case 5:

            // Java: isRight = paraInfo.mIsRightToLeft;

      }
      // Java: if (isRight)

      {
         // Java: offset += cellWidth - typoWidth;

         // Java: float italicFontSize = this.mTextBoxData.getTextData(paraIndex).italicFontSize();

         // Java: if (italicFontSize > 0.0F)

         {
            // Java: offset -= italicFontSize * 0.2F;

         }
      }
      // Java: return offset;

      0.0
    }

    // Java: void modifyTextBoxAlign(TextBoxData boxData, float typoHeight)
    pub fn modifyTextBoxAlign(&self, _boxData: i32, _typoHeight: f32) {
      // Java: float offx = 0.0F;

      // Java: float offy = 0.0F;

      // Java: float boxHeight = this.isAutoHeight() ? typoHeight : this.mTextRect.height();

      // Java: switch (boxData.anchor())

      {
         // Java: case 0:

            // Java: offy = 0.0F;

            // Java: break;

         // Java: case 1:

            // Java: offy = boxHeight * 0.5F;

            // Java: offy -= typoHeight * 0.5F;

            // Java: break;

         // Java: case 2:

         // Java: case 3:

         // Java: case 4:

            // Java: offy = boxHeight - typoHeight;

      }
      // Java: float mostl = 0.0F;

      // Java: float mostr = 0.0F;

      // Java: List<TextData> datas = boxData.getTextDatas();

      // Java: for(int i = 0; i < this.mTypoOffsets.size(); ++i)

      {
         // Java: PointF offset = (PointF)this.mTypoOffsets.get(i);

         // Java: Layout layout = (Layout)this.mTypoCaches.get(i);

         // Java: TextData data = (TextData)datas.get(i);

         // Java: int hangIndent = data.mPara.mIndent >= 0 ? 0 : -data.mPara.mIndent;

         // Java: if (hangIndent > data.mPara.mMarginL)

         {
            // Java: hangIndent = data.mPara.mMarginL;

         }
         // Java: float l = data.mPara.mbulletsInfo.getWidth() > 0.0F ? data.mPara.mbulletsInfo.getX() - offset.x - (float)(data.mPara.mMarginL - hangIndent) : layout.getLineLeft(0) - (float)(data.mPara.mMarginL - hangIndent);

         // Java: float r = layout.getLineRight(0);

         // Java: for(int j = 1; j < layout.getLineCount(); ++j)

         {
            // Java: float ll = layout.getLineLeft(j);

            // Java: float lr = layout.getLineRight(j);

            // Java: if (ll < l)

            {
               // Java: l = ll;

            }
            // Java: if (lr > r)

            {
               // Java: r = lr;

            }
         }
         // Java: l += offset.x;

         // Java: r += offset.x;

         // Java: if (i == 0 || l < mostl)

         {
            // Java: mostl = l;

         }
         // Java: if (r > mostr)

         {
            // Java: mostr = r;

         }
      }
      // Java: if (boxData.isAnchorCenter())

      {
         // Java: float ld = mostl - this.mTextRect.left;

         // Java: float rd = this.mTextRect.right - mostr;

         // Java: offx = (rd - ld) / 2.0F;

         // Java: float width = mostr - mostl;

         // Java: this.mTextPos.set(this.mTextRect.centerX() - width / 2.0F, offy, this.mTextRect.centerX() + width / 2.0F, offy + typoHeight);

      }
      // Java: else

      {
         // Java: this.mTextPos.set(mostl, offy, mostr, offy + typoHeight);

      }
      // Java: if (this.isAutoHeight() && this.isAllVert())

      {
         // Java: offx = 0.0F;

         // Java: offy = 0.0F;

      }
      // Java: else if (this.isAllVertRTl())

      {
         // Java: offy = -offy;

      }
      // Java: this.modifyMoffsets(offx, offy, boxData);

    }

    // Java: void modifyMoffsets(float alterX, float alterY, TextBoxData boxData)
    pub fn modifyMoffsets(&self, _alterX: f32, _alterY: f32, _boxData: i32) {
      // Java: List<TextData> datas = boxData.getTextDatas();

      // Java: for(int i = 0; i < this.mTypoOffsets.size(); ++i)

      {
         // Java: PointF var10000 = (PointF)this.mTypoOffsets.get(i);

         // Java: var10000.x += alterX;

         // Java: var10000 = (PointF)this.mTypoOffsets.get(i);

         // Java: var10000.y += alterY;

         // Java: Float bottom = (Float)this.mParaBottoms.get(i);

         // Java: bottom = bottom + alterY;

         // Java: this.mParaBottoms.remove(i);

         // Java: this.mParaBottoms.add(i, bottom);

         // Java: ((TextData)datas.get(i)).mPara.mbulletsInfo.alterX(alterX);

         // Java: ((TextData)datas.get(i)).mPara.mbulletsInfo.alterY(alterY);

      }
    }

    // Java: TextSelectionParam touchCoorToCursor(float touchX, float touchY, TextSelectionParam seleParam)
    pub fn touchCoorToCursor(&self, _touchX: f32, _touchY: f32, _seleParam: i32) -> i32 {
      // Java: if (null == seleParam)

      {
         // Java: seleParam = new TextSelectionParam();

      }
      // Java: LogicalPosition logiPos = this.getLogiPosByCoor(touchX, touchY, false, RichTextLayout.SelectionType.Null, seleParam);

      // Java: this.calCursorByLogPos(logiPos, seleParam);

      // Java: return seleParam;

      0
    }

    // Java: TextSelectionParam getTextHeadCursor(TextSelectionParam seleParam)
    pub fn getTextHeadCursor(&self, _seleParam: i32) -> i32 {
      // Java: if (null == seleParam)

      {
         // Java: seleParam = new TextSelectionParam();

      }
      // Java: LogicalPosition logiPos = new LogicalPosition();

      // Java: logiPos.lineIndex = 0;

      // Java: logiPos.paraIndex = 0;

      // Java: logiPos.offset = 0;

      // Java: this.calcLogPosAscent(logiPos);

      // Java: this.calCursorByLogPos(logiPos, seleParam);

      // Java: return seleParam;

      0
    }

    // Java: TextSelectionParam getTextCursorByPos(int offset, TextSelectionParam seleParam)
    pub fn getTextCursorByPos(&self, _offset: i32, _seleParam: i32) -> i32 {
      // Java: if (null == seleParam)

      {
         // Java: seleParam = new TextSelectionParam();

      }
      // Java: LogicalPosition logiPos = this.getLogPosByOffset(offset);

      // Java: this.calCursorByLogPos(logiPos, seleParam);

      // Java: return seleParam;

      0
    }

    // Java: TextSelectionParam getTextTailCursor(TextSelectionParam seleParam)
    pub fn getTextTailCursor(&self, _seleParam: i32) -> i32 {
      // Java: if (null == seleParam)

      {
         // Java: seleParam = new TextSelectionParam();

      }
      // Java: LogicalPosition logPos = new LogicalPosition();

      // Java: int idxParagraph = this.mTextBoxData.size() - 1;

      // Java: Layout layout = (Layout)this.mTypoCaches.get(idxParagraph);

      // Java: int lineIndex = layout.getLineCount() - 1;

      // Java: int offset = layout.getLineEnd(lineIndex);

      // Java: TextData data = this.mTextBoxData.getTextData(idxParagraph);

      // Java: String text = layout.getText().toString();

      // Java: int lineEnd = layout.getLineEnd(lineIndex);

      // Java: if (null == data.mBidi)

      {
         // Java: logPos.isAtLineEnd = true;

      }
      // Java: else if (data.mBidi.getLevelAt(lineEnd - 1) % 2 == 0)

      {
         // Java: logPos.isAtLineEnd = true;

      }
      // Java: if (offset < 0)

      {
         // Java: offset = 0;

      }
      // Java: else

      {
         // Java: int length = text.length();

         // Java: if (0 < length && length <= offset && ('\r' == text.charAt(length - 1) || ' ' == text.charAt(length - 1)))

         {
            // Java: offset = length - 1;

            // Java: logPos.isAtLineEnd = false;

         }
      }
      // Java: logPos.lineIndex = lineIndex;

      // Java: logPos.paraIndex = idxParagraph;

      // Java: logPos.offset = offset;

      // Java: this.calcLogPosAscent(logPos);

      // Java: this.calCursorByLogPos(logPos, seleParam);

      // Java: return seleParam;

      0
    }

    // Java: int getIndexByOffset(int paraIndex, int offset)
    pub fn getIndexByOffset(&self, _paraIndex: i32, _offset: i32) -> i32 {
      // Java: TextData textData = (TextData)this.mTextBoxData.getTextDatas().get(paraIndex);

      // Java: ParaInfo paraInfo = textData.mPara;

      // Java: return offset >= 0 && paraInfo.indexMap != null && offset < paraInfo.indexMap.length ? paraInfo.indexMap[offset] : offset;

      0
    }

    // Java: int getOffsetByIndex(int paraIndex, int index)
    pub fn getOffsetByIndex(&self, _paraIndex: i32, _index: i32) -> i32 {
      // Java: TextData textData = (TextData)this.mTextBoxData.getTextDatas().get(paraIndex);

      // Java: ParaInfo paraInfo = textData.mPara;

      // Java: return index >= 0 && paraInfo.invertMap != null && index < paraInfo.invertMap.length ? paraInfo.invertMap[index] : index;

      0
    }

    // Java: void calCursorByLogPos(LogicalPosition logPos, TextSelectionParam seleParam)
    pub fn calCursorByLogPos(&self, _logPos: i32, _seleParam: i32) {
      // Java: if (logPos == null)

      {
         // Java: seleParam.setRange(-1, -1);

      }
      // Java: else

      {
         // Java: this.setCursorPathAndMoffset(logPos, seleParam);

         // Java: int offset = logPos.offset;

         // Java: offset = this.offsetBeforeReplace(this.mTextBoxData.getTextData(logPos.paraIndex), offset);

         // Java: offset = this.modifyOffsetInBox(offset, logPos.paraIndex);

         // Java: seleParam.setRange(offset + logPos.spaceNum, offset + logPos.spaceNum);

         // Java: if (logPos.isAtLineEnd)

         {
            // Java: seleParam.setFlag((byte)10);

         }
         // Java: else

         {
            // Java: seleParam.setFlag((byte)1);

         }
      }
    }

    // Java: int get1stTextParagraph()
    pub fn get1stTextParagraph(&self) -> i32 {
      // Java: return 0;

      0
    }

    // Java: LogicalPosition getLogiPosByCoor(float touchX, float touchY, boolean isEndExpand, SelectionType type, TextSelectionParam seleParam)
    pub fn getLogiPosByCoor(&self, _touchX: f32, _touchY: f32, _isEndExpand: bool, _type: i32, _seleParam: i32) -> i32 {
      // Java: float temp = touchX;

      // Java: if (this.mTextBoxData.presetTextWarp() != 0 && null != this.mArtTransform)

      {
         // Java: float[] xy = new float[2];

         // Java: xy[0] = touchX;

         // Java: xy[1] = touchY;

         // Java: this.mArtTransform.squareHitCoor(xy);

         // Java: touchX = xy[0];

         // Java: touchY = xy[1];

      }
      // Java: else if (this.mRotDegrees != 0)

      {
         // Java: float[] xy = new float[]{touchX, touchY};

         // Java: this.rotateBack(xy);

         // Java: touchX = xy[0];

         // Java: touchY = xy[1];

      }
      // Java: else if (this.isAllVert())

      {
         // Java: touchX = touchY;

         // Java: touchY = temp;

      }
      // Java: LogicalPosition logiPos = new LogicalPosition();

      // Java: int _1stTextParagraph = this.get1stTextParagraph();

      // Java: int idxParagraph = this.indexOfParagraphVertical(touchY);

      // Java: if (idxParagraph < _1stTextParagraph)

      {
         // Java: idxParagraph = _1stTextParagraph;

         // Java: touchX = 0.0F;

         // Java: touchY = ((PointF)this.mTypoOffsets.get(_1stTextParagraph)).y;

      }
      // Java: PointF ptParagraph = (PointF)this.mTypoOffsets.get(idxParagraph);

      // Java: touchX -= ptParagraph.x;

      // Java: touchY -= ptParagraph.y;

      // Java: if (this.isAllVertRTl())

      {
         // Java: touchY = -touchY;

      }
      // Java: int offset = 0;

      // Java: int lineIndex = 0;

      // Java: Layout layout = (Layout)this.mTypoCaches.get(idxParagraph);

      // Java: lineIndex = layout.getLineForVertical((int)touchY);

      // Java: offset = layout.getOffsetForHorizontal(lineIndex, (float)((int)touchX));

      // Java: TextData data = this.mTextBoxData.getTextData(idxParagraph);

      // Java: String text = layout.getText().toString();

      // Java: int lineEnd = layout.getLineEnd(lineIndex);

      // Java: float lineTop = layout.getLineTop(lineIndex);

      // Java: float lineBottom = layout.getLineBottom(lineIndex);

      // Java: float lineLeft = layout.getLineLeft(lineIndex);

      // Java: float lineRight = layout.getLineRight(lineIndex);

      // Java: if (seleParam == null || !seleParam.isExactHit() || !(touchX < lineLeft) && !(touchX > lineRight) && !(touchY < lineTop) && !(touchY > lineBottom))

      {
         // Java: if (!data.requiresBidi)

         {
            // Java: if (touchY > lineBottom && touchX > lineLeft && type == RichTextLayout.SelectionType.Null)

            {
               // Java: offset = lineEnd;

            }
            // Java: if (type == RichTextLayout.SelectionType.End)

            {
               // Java: CharSequence string = layout.getText();

               // Java: int lineStart = layout.getLineStart(lineIndex);

               // Java: float firstCharWidth = Layout.getDesiredWidth(string.subSequence(lineStart, lineStart + 1), layout.getPaint());

               // Java: if (touchX < lineLeft + firstCharWidth / 2.0F)

               {
                  // Java: int start = seleParam.getRange().start();

                  // Java: LogicalPosition logicalPos = this.getLogPosByOffset(start);

                  // Java: if (!this.isLineStart(logicalPos))

                  {
                     // Java: if (lineIndex > 0)

                     {
                        // Java: --lineIndex;

                     }
                     // Java: else if (idxParagraph > 0 && lineIndex == 0)

                     {
                        // Java: --idxParagraph;

                        // Java: layout = (Layout)this.mTypoCaches.get(idxParagraph);

                        // Java: lineIndex = layout.getLineCount() - 1;

                        // Java: offset = layout.getLineEnd(lineIndex);

                     }
                  }
               }
            }
            // Java: else if (type == RichTextLayout.SelectionType.Start)

            {
               // Java: boolean isAtEnd = offset == lineEnd && offset >= text.length();

               // Java: if (touchX >= lineRight || isAtEnd)

               {
                  // Java: int end = seleParam.getRange().end();

                  // Java: LogicalPosition endPos = this.getLogPosByOffset(end - 1);

                  // Java: if (!this.isSameLineEnd(endPos, lineIndex, idxParagraph))

                  {
                     // Java: int lineCount = layout.getLineCount();

                     // Java: if (lineIndex < lineCount - 1)

                     {
                        // Java: ++lineIndex;

                        // Java: ++offset;

                     }
                     // Java: else

                     {
                        // Java: int newParaIndex = idxParagraph + 1;

                        // Java: if (newParaIndex < this.mTypoCaches.size())

                        {
                           // Java: layout = (Layout)this.mTypoCaches.get(newParaIndex);

                           // Java: if (layout != null)

                           {
                              // Java: idxParagraph = newParaIndex;

                              // Java: lineIndex = 0;

                              // Java: offset = layout.getLineStart(lineIndex);

                           }
                        }
                     }
                  }
                  // Java: else if (offset >= lineEnd - 1)

                  {
                     // Java: offset = lineEnd - 1;

                  }
               }
            }
         }
         // Java: if (type == RichTextLayout.SelectionType.End && lineEnd > 0 && lineEnd - 1 <= offset && '\n' != text.charAt(lineEnd - 1) && '\r' != text.charAt(lineEnd - 1))

         {
            // Java: float offsetPos = layout.getPrimaryHorizontal(lineEnd - 1);

            // Java: if (touchX > (lineRight + offsetPos) / 2.0F)

            {
               // Java: offset = lineEnd;

               // Java: logiPos.isAtLineEnd = true;

            }
         }
         // Java: if (type == RichTextLayout.SelectionType.Null && lineEnd > 0 && lineEnd - 1 <= offset && '\n' != text.charAt(lineEnd - 1))

         {
            // Java: lineRight = layout.getLineRight(lineIndex);

            // Java: float offsetPos = layout.getPrimaryHorizontal(lineEnd - 1);

            // Java: if (touchX > (lineRight + offsetPos) / 2.0F)

            {
               // Java: offset = lineEnd;

               // Java: logiPos.isAtLineEnd = true;

            }
         }
         // Java: if (offset < 0)

         {
            // Java: offset = 0;

         }
         // Java: else if (offset > lineEnd)

         {
            // Java: offset = lineEnd;

         }
         // Java: else

         {
            // Java: int length = text.length();

            // Java: if (0 < length && length <= offset && !isEndExpand && ('\r' == text.charAt(length - 1) || ' ' == text.charAt(length - 1)))

            {
               // Java: offset = length - 1;

               // Java: logiPos.isAtLineEnd = false;

            }
         }
         // Java: logiPos.lineIndex = lineIndex;

         // Java: logiPos.paraIndex = idxParagraph;

         // Java: logiPos.offset = offset;

         // Java: if (logiPos.isAtLineEnd)

         {
            // Java: --logiPos.offset;

         }
         // Java: this.convertToLogicPoint(logiPos);

         // Java: if (logiPos.isAtLineEnd)

         {
            // Java: ++logiPos.offset;

         }
         // Java: this.calcLogPosAscent(logiPos);

         // Java: if (text.length() - 1 == logiPos.offset && data.mSourceText.length() > text.length())

         {
            // Java: int maxSpaNum = data.mSourceText.length() - text.length();

            // Java: if (null != data.mZeroWidthSpan)

            {
               // Java: --maxSpaNum;

            }
            // Java: float sw = PaintFontHelper.getCharWidth(new char[]{' '}, logiPos.span);

            // Java: if (null != data.mBidi && !data.mBidi.baseIsLeftToRight())

            {
               // Java: lineLeft = layout.getLineLeft(lineIndex);

               // Java: logiPos.spaceNum = touchX < lineLeft ? (int)((double)((lineLeft - touchX) / sw) + (double)0.5F) : 0;

            }
            // Java: else

            {
               // Java: lineRight = layout.getLineRight(lineIndex);

               // Java: logiPos.spaceNum = touchX > lineRight ? (int)((double)((touchX - lineRight) / sw) + (double)0.5F) : 0;

            }
            // Java: if (logiPos.spaceNum > maxSpaNum)

            {
               // Java: logiPos.spaceNum = maxSpaNum;

            }
         }
         // Java: this.expandLayoutLogiPos(layout, logiPos);

         // Java: return logiPos;

      }
      // Java: else

      {
         // Java: return null;

      }
      0
    }

    // Java: void convertToLogicPoint(LogicalPosition logiPos)
    pub fn convertToLogicPoint(&self, _logiPos: i32) {
      // Java: logiPos.offset = this.convertToLogicPoint(logiPos.offset, logiPos.paraIndex, this.mTextBoxData.getTextDatas());

    }

    // Java: void expandLayoutLogiPos(Layout layout, LogicalPosition logiPos)
    pub fn expandLayoutLogiPos(&self, _layout: i32, _logiPos: i32) {

    }

    // Java: int offsetBeforeReplace(TextData data, int offset)
    pub fn offsetBeforeReplace(&self, _data: i32, _offset: i32) -> i32 {
      // Java: int[] positions = data.mReplacePositions;

      // Java: if (null != positions)

      {
         // Java: int delta = 0;

         // Java: for(int i = 0; i < positions.length && offset > positions[i]; i += 2)

         {
            // Java: if (offset <= positions[i + 1])

            {
               // Java: delta += offset - positions[i];

               // Java: break;

            }
            // Java: delta += positions[i + 1] - positions[i];

         }
         // Java: offset -= delta;

      }
      // Java: return offset;

      0
    }

    // Java: int offsetAfterReplace(TextData data, int offset)
    pub fn offsetAfterReplace(&self, _data: i32, _offset: i32) -> i32 {
      // Java: int[] positions = data.mReplacePositions;

      // Java: if (null != positions)

      {
         // Java: for(int i = 0; i < positions.length && offset > positions[i]; i += 2)

         {
            // Java: offset += positions[i + 1] - positions[i];

         }
      }
      // Java: return offset;

      0
    }

    // Java: void calcLogPosAscent(LogicalPosition logPos)
    pub fn calcLogPosAscent(&self, _logPos: i32) {
      // Java: TextData data = this.mTextBoxData.getTextData(logPos.paraIndex);

      // Java: SpanInfo span = data.mFirstSpan;

      // Java: if (logPos.offset > span.mEndPos && data.mOtherSpans.size() > 0)

      {
         // Java: int s = 0;

         // Java: int e = data.mOtherSpans.size() - 1;

         // Java: while(s < e)

         {
            // Java: int m = (s + e) / 2;

            // Java: span = (SpanInfo)data.mOtherSpans.get(m);

            // Java: if (logPos.offset > span.mStartPos && logPos.offset <= span.mEndPos)

            {
               // Java: break;

            }
            // Java: if (logPos.offset <= span.mStartPos)

            {
               // Java: e = m;

            }
            // Java: else

            {
               // Java: if (s == m)

               {
                  // Java: ++m;

               }
               // Java: s = m;

            }
         }
         // Java: if (s == e)

         {
            // Java: span = (SpanInfo)data.mOtherSpans.get(s);

         }
      }
      // Java: logPos.ascent = span.getAscent();

      // Java: logPos.span = span;

    }

    // Java: int modifyOffsetInBox(int offset, int paraIndex)
    pub fn modifyOffsetInBox(&self, _offset: i32, _paraIndex: i32) -> i32 {
      // Java: List<TextData> datas = this.mTextBoxData.getTextDatas();

      // Java: for(int i = 0; i < paraIndex; ++i)

      {
         // Java: offset += ((TextData)datas.get(i)).mSourceText.length();

      }
      // Java: return offset;

      0
    }

    // Java: int convertToLogicPoint(int offset, int paraIndex, List<TextData> datas)
    pub fn convertToLogicPoint_2(&self, _offset: i32, _paraIndex: i32, _datas: i32) -> i32 {
      // Java: if (((TextData)datas.get(paraIndex)).mPara.indexMap != null && ((TextData)datas.get(paraIndex)).mPara.indexMap.length > offset)

      {
         // Java: offset = ((TextData)datas.get(paraIndex)).mPara.indexMap[offset];

      }
      // Java: return offset;

      0
    }

    // Java: int checkEndReturn(LogicalPosition endLogPos)
    pub fn checkEndReturn(&self, _endLogPos: i32) -> i32 {
      // Java: int paraIndex = endLogPos.paraIndex;

      // Java: if (paraIndex != this.mTextBoxData.size() - 1)

      {
         // Java: TextData data = this.mTextBoxData.getTextData(paraIndex);

         // Java: if (data.mTypoText.length() == endLogPos.offset && data.mSourceText.length() > data.mTypoText.length())

         {
            // Java: return data.mSourceText.length() - data.mTypoText.length();

         }
      }
      // Java: return 0;

      0
    }

    // Java: TextSelectionParam touchCoorToSelection(float startX, float startY, float endX, float endY, TextSelectionParam seleParam)
    pub fn touchCoorToSelection(&self, _startX: f32, _startY: f32, _endX: f32, _endY: f32, _seleParam: i32) -> i32 {
      // Java: if (seleParam == null)

      {
         // Java: seleParam = new TextSelectionParam();

      }
      // Java: LogicalPosition startLogPos = null;

      // Java: if (startX == -1.0F)

      {
         // Java: startLogPos = this.getLogPosByOffset(seleParam.getRange().start());

      }
      // Java: else

      {
         // Java: startLogPos = this.getLogiPosByCoor(startX, startY, false, RichTextLayout.SelectionType.Start, seleParam);

      }
      // Java: LogicalPosition endLogPos = null;

      // Java: if (endX == -1.0F)

      {
         // Java: endLogPos = this.getLogPosByOffset(seleParam.getRange().end());

      }
      // Java: else

      {
         // Java: endLogPos = this.getLogiPosByCoor(endX, endY, true, RichTextLayout.SelectionType.End, seleParam);

         // Java: int paraIndex = endLogPos.paraIndex;

         // Java: boolean isLtr = this.isPrimary(this.mTextBoxData.getTextData(paraIndex).mBidi, endLogPos.offset);

         // Java: if (!isLtr && endLogPos.offset < ((Layout)this.mTypoCaches.get(paraIndex)).getText().length() - 1)

         {
            // Java: ++endLogPos.offset;

         }
      }
      // Java: if (startLogPos != null && endLogPos != null)

      {
         // Java: int startOffset = this.offsetBeforeReplace(this.mTextBoxData.getTextData(startLogPos.paraIndex), startLogPos.offset);

         // Java: int endOffset = this.offsetBeforeReplace(this.mTextBoxData.getTextData(endLogPos.paraIndex), endLogPos.offset);

         // Java: startOffset = this.modifyOffsetInBox(startOffset, startLogPos.paraIndex);

         // Java: endOffset = this.modifyOffsetInBox(endOffset, endLogPos.paraIndex);

         // Java: if (startOffset == endOffset)

         {
            // Java: if (startX == -1.0F)

            {
               // Java: endLogPos = this.getLogPosByOffset(this.incrementOffset(endOffset, endLogPos));

               // Java: endOffset = this.offsetBeforeReplace(this.mTextBoxData.getTextData(endLogPos.paraIndex), endLogPos.offset);

               // Java: endOffset = this.modifyOffsetInBox(endOffset, endLogPos.paraIndex);

            }
            // Java: else

            {
               // Java: if (endX != -1.0F)

               {
                  // Java: this.calCursorByLogPos(startLogPos, seleParam);

                  // Java: return seleParam;

               }
               // Java: if (this.isLineStart(endLogPos))

               {
                  // Java: endLogPos = this.getLogPosByOffset(this.incrementOffset(endOffset, endLogPos));

                  // Java: endOffset = this.offsetBeforeReplace(this.mTextBoxData.getTextData(endLogPos.paraIndex), endLogPos.offset);

                  // Java: endOffset = this.modifyOffsetInBox(endOffset, startLogPos.paraIndex);

               }
               // Java: else

               {
                  // Java: startLogPos = this.getLogPosByOffset(this.decrementOffset(startOffset, startLogPos));

                  // Java: startOffset = this.offsetBeforeReplace(this.mTextBoxData.getTextData(startLogPos.paraIndex), startLogPos.offset);

                  // Java: startOffset = this.modifyOffsetInBox(startOffset, startLogPos.paraIndex);

               }
            }
         }
         // Java: else if (startOffset > endOffset)

         {
            // Java: LogicalPosition tempPos = startLogPos;

            // Java: startLogPos = endLogPos;

            // Java: endLogPos = tempPos;

            // Java: int tempOffset = startOffset;

            // Java: startOffset = endOffset;

            // Java: endOffset = tempOffset;

         }
         // Java: endOffset += this.checkEndReturn(endLogPos);

         // Java: seleParam.setRange(startOffset, endOffset);

         // Java: this.setSelPathAndMoffset(startLogPos, endLogPos, seleParam);

         // Java: return seleParam;

      }
      // Java: else

      {
         // Java: return seleParam;

      }
      0
    }

    // Java: boolean isLineStart(LogicalPosition logicPos)
    pub fn isLineStart(&self, _logicPos: i32) -> bool {
      // Java: int offset = logicPos.offset;

      // Java: int lineIndex = logicPos.lineIndex;

      // Java: Layout layout = (Layout)this.mTypoCaches.get(logicPos.paraIndex);

      // Java: int start = layout.getLineStart(lineIndex);

      // Java: return start == offset;

      false
    }

    // Java: boolean isSameLineEnd(LogicalPosition logicPos, int line, int para)
    pub fn isSameLineEnd(&self, _logicPos: i32, _line: i32, _para: i32) -> bool {
      // Java: int offset = logicPos.offset;

      // Java: int lineIndex = logicPos.lineIndex;

      // Java: int paraIndex = logicPos.paraIndex;

      // Java: Layout layout = (Layout)this.mTypoCaches.get(logicPos.paraIndex);

      // Java: int end = layout.getLineEnd(lineIndex);

      // Java: return offset >= end - 1 && line == lineIndex && para == paraIndex;

      false
    }

    // Java: void modifyCursor(TextSelectionParam selParam)
    pub fn modifyCursor(&self, _selParam: i32) {
      // Java: if (null != selParam)

      {
         // Java: int cursor = selParam.getRange().start();

         // Java: LogicalPosition logPos = this.getLogPosByOffset(cursor, selParam.isAtLineEnd());

         // Java: this.setCursorPathAndMoffset(logPos, selParam);

      }
    }

    // Java: PointF expandMoveToEnd(LogicalPosition logPos, Layout layout, Path path, float y1, float y2)
    pub fn expandMoveToEnd(&self, _logPos: i32, _layout: i32, _path: i32, _y1: f32, _y2: f32) -> i32 {
      // Java: return null;

      0
    }

    // Java: int convertToRenderPosition(LogicalPosition logPos)
    pub fn convertToRenderPosition(&self, _logPos: i32) -> i32 {
      // Java: TextData textData = (TextData)this.mTextBoxData.getTextDatas().get(logPos.paraIndex);

      // Java: ParaInfo paraInfo = textData.mPara;

      // Java: return paraInfo.invertMap != null && logPos.offset < paraInfo.invertMap.length ? paraInfo.invertMap[logPos.offset] : logPos.offset;

      0
    }

    // Java: void setCursorPathAndMoffset(LogicalPosition logPos, TextSelectionParam seleParam)
    pub fn setCursorPathAndMoffset(&self, _logPos: i32, _seleParam: i32) {
      // Java: int renderOffset = this.convertToRenderPosition(logPos);

      // Java: WpsPath path = new WpsPath(logPos.paraIndex);

      // Java: Layout layout = (Layout)this.mTypoCaches.get(logPos.paraIndex);

      // Java: float ascent = logPos.ascent;

      // Java: float baseline = layout.getLineBaseline(logPos.lineIndex);

      // Java: float y1 = baseline - ascent / 5.0F;

      // Java: float y2 = baseline + ascent;

      // Java: if (this.isAllVertLTR())

      {
         // Java: y1 = layout.getLineTop(logPos.lineIndex);

         // Java: y2 = layout.getLineBottom(logPos.lineIndex);

      }
      // Java: else if (this.isAllVertRTl())

      {
         // Java: y1 = -layout.getLineTop(logPos.lineIndex);

         // Java: y2 = -layout.getLineBottom(logPos.lineIndex);

      }
      // Java: float cursorHeight = Math.abs(y2 - y1);

      // Java: PointF cursorPoint = null;

      // Java: if ((cursorPoint = this.expandMoveToEnd(logPos, layout, path, y1, y2)) == null)

      {
         // Java: if (logPos.isAtLineEnd)

         {
            // Java: float x = layout.getLineRight(logPos.lineIndex);

            // Java: if (this.isAllVert())

            {
               // Java: path.moveTo(y1, x);

               // Java: path.lineTo(y2, x);

               // Java: cursorPoint = new PointF(y2, x);

            }
            // Java: else

            {
               // Java: path.moveTo(x, y1);

               // Java: path.lineTo(x, y2);

               // Java: cursorPoint = new PointF(x, y1);

            }
         }
         // Java: else

         {
            // Java: float bottom = layout.getLineBottom(logPos.lineIndex);

            // Java: float top = layout.getLineTop(logPos.lineIndex);

            // Java: float sm = 0.0F;

            // Java: List<TextData> datas = this.mTextBoxData.getTextDatas();

            // Java: if (logPos.spaceNum > 0)

            {
               // Java: TextData data = (TextData)datas.get(logPos.paraIndex);

               // Java: char[] space = new char[]{' '};

               // Java: float sw = PaintFontHelper.getCharWidth(space, logPos.span);

               // Java: if (null != data.mBidi && data.mBidi.getLevelAt(renderOffset) % 2 == 1)

               {
                  // Java: sw = -sw;

               }
               // Java: sm = sw * (float)logPos.spaceNum;

            }
            // Java: layout.getCursorPath(renderOffset, path, (CharSequence)null);

            // Java: path = this.recalcBiSelPath((TextData)datas.get(logPos.paraIndex), renderOffset, layout, path);

            // Java: if (this.isAllVert())

            {
               // Java: cursorPoint = new PointF(y2, layout.getPrimaryHorizontal(renderOffset));

            }
            // Java: else

            {
               // Java: cursorPoint = new PointF(layout.getPrimaryHorizontal(renderOffset), y1);

            }
            // Java: float offset = 0.0F;

            // Java: float left = layout.getLineLeft(logPos.lineIndex);

            // Java: if (left < 0.0F)

            {
               // Java: TextData data = (TextData)datas.get(logPos.paraIndex);

               // Java: int lineEnd = layout.getLineEnd(logPos.lineIndex);

               // Java: boolean atLeft = renderOffset < lineEnd;

               // Java: if (null != data.mBidi && data.mBidi.getLevelAt(renderOffset) % 2 == 1)

               {
                  // Java: atLeft = renderOffset == lineEnd;

               }
               // Java: if (atLeft)

               {
                  // Java: offset = left;

               }
            }
            // Java: sm += offset;

            // Java: if (!this.isAllVert())

            {
               // Java: path.offset(sm, y1 - bottom);

               // Java: Matrix matrix = new Matrix();

               // Java: matrix.setScale(1.0F, (y1 - y2) / (bottom - top), 0.0F, y1);

               // Java: path.transform(matrix);

            }
         }
      }
      // Java: List<WpsPath> paths = new ArrayList();

      // Java: List<PointF> offsets = new ArrayList();

      // Java: paths.add(path);

      // Java: PointF offset = (PointF)this.mTypoOffsets.get(logPos.paraIndex);

      // Java: if (this.isAllVert())

      {
         // Java: offset = new PointF(offset.y, offset.x);

      }
      // Java: else

      {
         // Java: offset = new PointF(offset.x, offset.y);

      }
      // Java: cursorPoint.offset(offset.x, offset.y);

      // Java: offsets.add(offset);

      // Java: if (this.mTextBoxData.presetTextWarp() != 0 && null != this.mArtTransform)

      {
         // Java: this.mArtTransform.squarePointF(cursorPoint);

      }
      // Java: if (0 != this.mRotDegrees && this.mTextBoxData.presetTextWarp() == 0)

      {
         // Java: this.rotateSelCur(paths, offsets, cursorPoint);

      }
      // Java: seleParam.setPaths(paths);

      // Java: seleParam.setMoffsets(offsets);

      // Java: seleParam.setSelectDragParam(cursorPoint, (PointF)null, this.mRotDegrees, true, true, cursorHeight, 0.0F, true, this.mTextBoxData.verticalText());

      // Java: if (this.mTextBoxData.presetTextWarp() != 0 && null != this.mArtTransform)

      {
         // Java: this.mArtTransform.squareCurSel(seleParam);

      }
    }

    // Java: void modifySelection(TextSelectionParam seleParam)
    pub fn modifySelection(&self, _seleParam: i32) {
      // Java: TextRange range = seleParam.getRange();

      // Java: LogicalPosition startLogPos = this.getLogPosByOffset(range.start());

      // Java: LogicalPosition endLogPos = this.getLogPosByOffset(range.end() - 1);

      // Java: ++endLogPos.offset;

      // Java: TextData data = this.mTextBoxData.getTextData(endLogPos.paraIndex);

      // Java: if (endLogPos.offset > data.mTypoText.length())

      {
         // Java: endLogPos.offset = data.mTypoText.length();

      }
      // Java: this.setSelPathAndMoffset(startLogPos, endLogPos, seleParam);

    }

    // Java: List<RectF> getTextRects(int firstPara, int lastPara)
    pub fn getTextRects(&self, _firstPara: i32, _lastPara: i32) -> Vec<i32> {
      // Java: int startOffset = 0;

      // Java: int endOffset = this.mTextBoxData.getTextData(lastPara).mTypoText.length();

      // Java: LogicalPosition first = new LogicalPosition();

      // Java: LogicalPosition last = new LogicalPosition();

      // Java: TextSelectionParam selParam = new TextSelectionParam();

      // Java: first.paraIndex = firstPara;

      // Java: first.offset = startOffset;

      // Java: last.paraIndex = lastPara;

      // Java: last.offset = endOffset;

      // Java: this.setSelPathAndMoffset(first, last, selParam);

      // Java: List<WpsPath> paths = selParam.getPaths();

      // Java: List<PointF> offsets = selParam.getMoffsets();

      // Java: List<RectF> textRects = new ArrayList();

      // Java: for(int i = 0; i < paths.size(); ++i)

      {
         // Java: PointF offset = (PointF)offsets.get(i);

         // Java: WpsPath path = (WpsPath)paths.get(i);

         // Java: List<RectF> pathRects = path.getRects();

         // Java: for(int j = 0; j < pathRects.size(); ++j)

         {
            // Java: RectF pathRect = (RectF)pathRects.get(j);

            // Java: pathRect.offset(offset.x, offset.y);

            // Java: textRects.add(pathRect);

         }
      }
      // Java: int be = lastPara + 1;

      // Java: int textRectSize = textRects.size();

      // Java: List<TextData> datas = this.mTextBoxData.getTextDatas();

      // Java: int i = firstPara;

      // Java: for(int size = datas.size(); i < be && i < size && i < textRectSize; ++i)

      {
         // Java: TextData data = (TextData)datas.get(i);

         // Java: if (data.mPara.mbulletsInfo.getWidth() > 0.0F)

         {
            // Java: RectF rect = new RectF(0.0F, 0.0F, data.mPara.mbulletsInfo.getWidth(), data.mPara.mbulletsInfo.getHeight());

            // Java: rect.offset(data.mPara.mbulletsInfo.getX(), data.mPara.mbulletsInfo.getY());

            // Java: Matrix matrix = new Matrix();

            // Java: switch (this.mRotDegrees)

            {
               // Java: case 90:

                  // Java: matrix.preTranslate(this.mTypoRect.height(), 0.0F);

                  // Java: matrix.preRotate(90.0F, 0.0F, 0.0F);

                  // Java: break;

               // Java: case 270:

                  // Java: matrix.preTranslate(0.0F, this.mTypoRect.width());

                  // Java: matrix.preRotate(270.0F, 0.0F, 0.0F);

            }
            // Java: matrix.mapRect(rect);

            // Java: RectF textRect = (RectF)textRects.get(i);

            // Java: if (textRect.left > rect.left)

            {
               // Java: textRect.left = rect.left;

            }
            // Java: if (textRect.top > rect.top)

            {
               // Java: textRect.top = rect.top;

            }
         }
      }
      // Java: return textRects;

      Vec::new()
    }

    // Java: List<RectF> getSelRects(int firstPara, int startOffset, int lastPara, int endOffset, boolean bulletIncludeFirst, boolean bulletIncludeLast)
    pub fn getSelRects_2(&self, _firstPara: i32, _startOffset: i32, _lastPara: i32, _endOffset: i32, _bulletIncludeFirst: bool, _bulletIncludeLast: bool) -> Vec<i32> {
      // Java: List<RectF> rects = new ArrayList();

      // Java: List<TextData> datas = this.mTextBoxData.getTextDatas();

      // Java: int bs = bulletIncludeFirst ? firstPara : firstPara + 1;

      // Java: int be = bulletIncludeLast ? lastPara + 1 : lastPara;

      // Java: int i = bs;

      // Java: for(int size = datas.size(); i < be && i < size; ++i)

      {
         // Java: TextData data = (TextData)datas.get(i);

         // Java: if (data.mPara.mbulletsInfo.getWidth() > 0.0F)

         {
            // Java: RectF rect = new RectF(0.0F, 0.0F, data.mPara.mbulletsInfo.getWidth(), data.mPara.mbulletsInfo.getHeight());

            // Java: rect.offset(data.mPara.mbulletsInfo.getX(), data.mPara.mbulletsInfo.getY());

            // Java: Matrix matrix = new Matrix();

            // Java: switch (this.mRotDegrees)

            {
               // Java: case 90:

                  // Java: matrix.preTranslate(this.mTypoRect.height(), 0.0F);

                  // Java: matrix.preRotate(90.0F, 0.0F, 0.0F);

                  // Java: break;

               // Java: case 270:

                  // Java: matrix.preTranslate(0.0F, this.mTypoRect.width());

                  // Java: matrix.preRotate(270.0F, 0.0F, 0.0F);

            }
            // Java: matrix.mapRect(rect);

            // Java: rects.add(rect);

         }
      }
      // Java: LogicalPosition first = new LogicalPosition();

      // Java: LogicalPosition last = new LogicalPosition();

      // Java: TextSelectionParam selParam = new TextSelectionParam();

      // Java: first.paraIndex = firstPara;

      // Java: first.offset = startOffset;

      // Java: last.paraIndex = lastPara;

      // Java: last.offset = endOffset;

      // Java: this.setSelPathAndMoffset(first, last, selParam);

      // Java: List<WpsPath> paths = selParam.getPaths();

      // Java: List<PointF> offsets = selParam.getMoffsets();

      // Java: for(i = 0; i < paths.size(); ++i)

      {
         // Java: PointF offset = (PointF)offsets.get(i);

         // Java: WpsPath path = (WpsPath)paths.get(i);

         // Java: List<RectF> pathRects = path.getRects();

         // Java: for(int j = 0; j < pathRects.size(); ++j)

         {
            // Java: RectF pathRect = (RectF)pathRects.get(j);

            // Java: pathRect.offset(offset.x, offset.y);

            // Java: rects.add(pathRect);

         }
      }
      // Java: return rects;

      Vec::new()
    }

    // Java: RectF getTextDrawAreaFromLayout()
    pub fn getTextDrawAreaFromLayout(&self) -> i32 {
      // Java: if (this.mTypoCaches.size() == 0)

      {
         // Java: return new RectF();

      }
      // Java: else

      {
         // Java: PointF firstOffset = (PointF)this.mTypoOffsets.get(0);

         // Java: RectF drawArea = new RectF(firstOffset.x, firstOffset.y, firstOffset.x, firstOffset.y);

         // Java: for(int i = 0; i < this.mTypoCaches.size(); ++i)

         {
            // Java: Layout layout = (Layout)this.mTypoCaches.get(i);

            // Java: PointF offset = (PointF)this.mTypoOffsets.get(i);

            // Java: RectF rectTmp = new RectF(offset.x, offset.y, offset.x + (float)layout.getWidth(), offset.y + layout.getHeight());

            // Java: drawArea.union(rectTmp);

         }
         // Java: return drawArea;

      }
      0
    }

    // Java: RectF getTextDrawArea()
    pub fn getTextDrawArea(&self) -> i32 {
      // Java: boolean wordartRender = this.hasWordArtTypo();

      // Java: RectF txtDrawArea = this.calcDrawArea(wordartRender, true);

      // Java: return txtDrawArea;

      0
    }

    // Java: ArtTransform getArtTransform()
    pub fn getArtTransform(&self) -> i32 {
      // Java: return this.mArtTransform;

      0
    }

    // Java: RectF calcDrawArea(boolean wordart, boolean artTransform)
    pub fn calcDrawArea(&self, _wordart: bool, _artTransform: bool) -> i32 {
      // Java: if (this.mTypoCaches.size() == 0)

      {
         // Java: return new RectF();

      }
      // Java: else if (artTransform && this.mTextBoxData.presetTextWarp() != 0 && null != this.mArtTransform)

      {
         // Java: return !wordart ? this.mTypoRect : this.mWordArtTypo.adjustTxtRectArtTrans(this.mTypoRect, this.mTypoCaches);

      }
      // Java: else

      {
         // Java: RectF rectTmp = this.getTextDrawAreaFromLayout();

         // Java: float maxCharWid = this.getMaxCharWid();

         // Java: RectF rectSrc = new RectF(rectTmp.left, rectTmp.top, rectTmp.right, rectTmp.bottom);

         // Java: if (rectSrc.width() < maxCharWid)

         {
            // Java: rectSrc.right = rectSrc.right + maxCharWid - rectSrc.width();

         }
         // Java: float boxLeft = this.adjustTextArea(rectSrc);

         // Java: float maxBulSpace = this.getMaxBulSpace();

         // Java: rectSrc.left += maxBulSpace;

         // Java: rectSrc.right += maxBulSpace;

         // Java: if (this.mTextBoxData.isVertical())

         {
            // Java: Matrix matrix = new Matrix();

            // Java: RectF rect = new RectF();

            // Java: if (wordart)

            {
               // Java: this.mWordArtTypo.adjustTxtRect(rectSrc, this.mTypoCaches, this.mTypoOffsets, boxLeft);

            }
            // Java: rectSrc.left = rectSrc.left - maxBulSpace - 3.6F;

            // Java: rectSrc.right -= maxBulSpace;

            // Java: if (this.mRotDegrees == 90)

            {
               // Java: matrix.preTranslate(this.mTypoRect.height(), 0.0F);

               // Java: matrix.preRotate(90.0F);

               // Java: matrix.mapRect(rect, rectSrc);

            }
            // Java: else if (this.mRotDegrees == 270)

            {
               // Java: matrix.preTranslate(0.0F, this.mTypoRect.width());

               // Java: matrix.preRotate(270.0F);

               // Java: matrix.mapRect(rect, rectSrc);

            }
            // Java: return rect;

         }
         // Java: else

         {
            // Java: float minOffsetX = 0.0F;

            // Java: for(int i = 0; i < this.mTypoOffsets.size(); ++i)

            {
               // Java: if (((PointF)this.mTypoOffsets.get(i)).x < minOffsetX)

               {
                  // Java: minOffsetX = ((PointF)this.mTypoOffsets.get(i)).x;

               }
            }
            // Java: rectSrc.offset(minOffsetX < 0.0F ? minOffsetX : 0.0F, ((PointF)this.mTypoOffsets.get(0)).y - rectSrc.top);

            // Java: if (wordart)

            {
               // Java: this.mWordArtTypo.adjustTxtRect(rectSrc, this.mTypoCaches, this.mTypoOffsets, boxLeft);

            }
            // Java: if (maxBulSpace > 0.0F)

            {
               // Java: rectSrc.left -= maxBulSpace;

            }
            // Java: return rectSrc;

         }
      }
      0
    }

    // Java: float getMaxBulSpace()
    pub fn getMaxBulSpace(&self) -> f32 {
      // Java: float maxSpace = 0.0F;

      // Java: for(int para = 0; para < this.mTypoCaches.size(); ++para)

      {
         // Java: float temp = ((PointF)this.mTypoOffsets.get(para)).x - this.mTextRect.left;

         // Java: if (para == 0)

         {
            // Java: maxSpace = temp;

         }
         // Java: else if (temp > maxSpace)

         {
            // Java: maxSpace = temp;

         }
      }
      // Java: return maxSpace;

      0.0
    }

    // Java: float adjustTextArea(RectF rectSrc)
    pub fn adjustTextArea(&self, _rectSrc: i32) -> f32 {
      // Java: float boxLeft = 0.0F;

      // Java: float boxRight = 0.0F;

      // Java: for(int para = 0; para < this.mTypoCaches.size(); ++para)

      {
         // Java: Layout layout = (Layout)this.mTypoCaches.get(para);

         // Java: float paraLeft = layout.getLineLeft(0);

         // Java: float paraRight = layout.getLineRight(0);

         // Java: for(int line = 1; line < layout.getLineCount(); ++line)

         {
            // Java: float lineLeft = layout.getLineLeft(line);

            // Java: if (lineLeft < paraLeft)

            {
               // Java: paraLeft = lineLeft;

            }
            // Java: float lineRight = layout.getLineRight(line);

            // Java: if (lineRight > paraRight)

            {
               // Java: paraRight = lineRight;

            }
         }
         // Java: if (para == 0)

         {
            // Java: boxLeft = paraLeft;

            // Java: boxRight = paraRight;

         }
         // Java: else

         {
            // Java: boxLeft = paraLeft < boxLeft ? paraLeft : boxLeft;

            // Java: boxRight = paraRight > boxRight ? paraRight : boxRight;

         }
      }
      // Java: rectSrc.left += boxLeft;

      // Java: rectSrc.right = rectSrc.left + boxRight - boxLeft;

      // Java: Layout lastLayoutTmp = (Layout)this.mTypoCaches.get(this.mTypoCaches.size() - 1);

      // Java: int lineCount = lastLayoutTmp.getLineCount();

      // Java: float lastBaseline = lastLayoutTmp.getLineBaseline(lineCount - 1);

      // Java: float lastTop = lastLayoutTmp.getLineTop(lineCount - 1);

      // Java: float bottom = ((PointF)this.mTypoOffsets.get(this.mTypoCaches.size() - 1)).y + lastBaseline + (lastBaseline - lastTop) * 0.2F;

      // Java: rectSrc.bottom = bottom;

      // Java: return boxLeft;

      0.0
    }

    // Java: float getMaxCharWid()
    pub fn getMaxCharWid(&self) -> f32 {
      // Java: float maxCharWid = 40.0F;

      // Java: for(int i = 0; i < this.mTypoCaches.size(); ++i)

      {
         // Java: Layout layout = (Layout)this.mTypoCaches.get(i);

         // Java: SpannableStringBuilder spannable = (SpannableStringBuilder)layout.getText();

         // Java: AbsoluteSizeSpan[] spans = (AbsoluteSizeSpan[])spannable.getSpans(0, spannable.length(), AbsoluteSizeSpan.class);

         // Java: for(int idx = 0; idx < spans.length; ++idx)

         {
            // Java: float fontsize = (float)spans[idx].getSize();

            // Java: if (fontsize > maxCharWid)

            {
               // Java: maxCharWid = fontsize;

            }
         }
      }
      // Java: return maxCharWid * 1.25F;

      0.0
    }

    // Java: WpsPath recalcBiSelPath(TextData data, int offset, Layout layout, WpsPath path)
    pub fn recalcBiSelPath(&self, _data: i32, _offset: i32, _layout: i32, _path: i32) -> i32 {
      // Java: RectF rect = new RectF();

      // Java: path.computeBounds(rect, false);

      // Java: if (rect.isEmpty())

      {
         // Java: return path;

      }
      // Java: else

      {
         // Java: WpsPath biPath = new WpsPath(path.getLayoutIndex());

         // Java: float cursor = layout.getPrimaryHorizontal(offset);

         // Java: float midv = (rect.top + rect.bottom) / 2.0F;

         // Java: float midh = (rect.left + rect.right) / 2.0F;

         // Java: if (cursor < midh)

         {
            // Java: biPath.moveTo(rect.left, rect.top);

            // Java: biPath.lineTo(rect.left, midv);

            // Java: biPath.moveTo(rect.right, midv);

            // Java: biPath.lineTo(rect.right, rect.bottom);

         }
         // Java: else

         {
            // Java: biPath.moveTo(rect.left, midv);

            // Java: biPath.lineTo(rect.left, rect.bottom);

            // Java: biPath.moveTo(rect.right, rect.top);

            // Java: biPath.lineTo(rect.right, midv);

         }
         // Java: return biPath;

      }
      0
    }

    // Java: void setSelPathAndMoffset(LogicalPosition startLogPos, LogicalPosition endLogPos, TextSelectionParam seleParam)
    pub fn setSelPathAndMoffset(&self, _startLogPos: i32, _endLogPos: i32, _seleParam: i32) {
      // Java: List<WpsPath> paths = new ArrayList();

      // Java: List<PointF> moffsets = new ArrayList();

      // Java: PointF selDragStartP = null;

      // Java: PointF selDragEndP = null;

      // Java: PointF offsetStPoint = null;

      // Java: PointF offsetEndPoint = null;

      // Java: boolean isDragStartLtr = true;

      // Java: boolean isDragEndLtr = true;

      // Java: float startLineHeight = 0.0F;

      // Java: float endLineHeight = 0.0F;

      // Java: int startline = 0;

      // Java: int endline = 0;

      // Java: List<TextData> datas = this.mTextBoxData.getTextDatas();

      // Java: for(int i = startLogPos.paraIndex; i <= endLogPos.paraIndex; ++i)

      {
         // Java: TextData data = (TextData)datas.get(i);

         // Java: int startInPara = i == startLogPos.paraIndex ? startLogPos.offset : 0;

         // Java: int endInPara = i == endLogPos.paraIndex ? endLogPos.offset : ((TextData)datas.get(i)).mTypoText.length();

         // Java: if (startInPara == endInPara)

         {
            // Java: break;

         }
         // Java: if (data.requiresBidi)

         {
            // Java: Layout layout = (Layout)this.mTypoCaches.get(i);

            // Java: PointF typoOffset = (PointF)this.mTypoOffsets.get(i);

            // Java: typoOffset = new PointF(typoOffset.x, typoOffset.y);

            // Java: int firstPosition = 0;

            // Java: int endPosition = 0;

            // Java: if (endLogPos.paraIndex != i && endInPara > startInPara + 1)

            {
               // Java: --endInPara;

            }
            // Java: int startlineInPara = layout.getLineForOffset(startInPara);

            // Java: int endlineInPara = layout.getLineForOffset(endInPara > startInPara ? endInPara - 1 : endInPara);

            // Java: for(int line = startlineInPara; line <= endlineInPara; ++line)

            {
               // Java: int startOffsetInLine = line == startlineInPara ? startInPara : layout.getLineStart(line);

               // Java: int endOffsetInLine = line == endlineInPara ? endInPara : layout.getLineEnd(line);

               // Java: int offsetStart = startOffsetInLine;

               // Java: String selectString = ((TextData)datas.get(i)).mTypoText.substring(startOffsetInLine, endOffsetInLine);

               // Java: Bidi bidi = new Bidi(selectString, data.mPara.mIsRightToLeft ? 1 : 0);

               // Java: int runCount = bidi.getRunCount();

               // Java: int lineFirst = -1;

               // Java: int lineEnd = -1;

               // Java: for(int run = 0; run < runCount; ++run)

               {
                  // Java: bidi.getRunLevel(run);

                  // Java: int runStart = bidi.getRunStart(run) + offsetStart;

                  // Java: int runEnd = bidi.getRunLimit(run) - 1 + offsetStart;

                  // Java: int renderStart = data.mPara.invertMap == null ? runStart : data.mPara.invertMap[runStart];

                  // Java: int renderEnd = data.mPara.invertMap == null ? runEnd : data.mPara.invertMap[runEnd];

                  // Java: if (run == 0 && line == startlineInPara)

                  {
                     // Java: firstPosition = renderStart;

                  }
                  // Java: if (run == runCount - 1 && line == endlineInPara)

                  {
                     // Java: endPosition = renderEnd;

                  }
                  // Java: if (renderStart > renderEnd)

                  {
                     // Java: int temp = renderStart;

                     // Java: renderStart = renderEnd;

                     // Java: renderEnd = temp;

                  }
                  // Java: if (lineFirst == -1 || lineFirst > renderStart)

                  {
                     // Java: lineFirst = renderStart;

                  }
                  // Java: if (lineEnd == -1 || lineEnd < renderEnd)

                  {
                     // Java: lineEnd = renderEnd;

                  }
                  // Java: int startLineInPara = layout.getLineForOffset(renderStart);

                  // Java: int endLineInPara = layout.getLineForOffset(renderEnd);

                  // Java: float befInc = 0.0F;

                  // Java: float aftInc = 0.0F;

                  // Java: float botIncRat = 0.0F;

                  // Java: float botInc = 0.0F;

                  // Java: if (null != data.mBidi && ANDROID_VERSION < 21 && data.mPara.mSpcWithinPct < 0.9 && data.mPara.mSpcWithinPct > (double)0.0F)

                  {
                     // Java: botIncRat = (float)((double)1.0F / data.mPara.mSpcWithinPct - 1.1111111111111112);

                  }
                  // Java: if (0 == startLineInPara && i > 0)

                  {
                     // Java: befInc = (float)data.mPara.mSpcBef;

                     // Java: if (botIncRat > 0.0F)

                     {
                        // Java: Layout befLayout = (Layout)this.mTypoCaches.get(i - 1);

                        // Java: int befLine = befLayout.getLineCount() - 1;

                        // Java: float befBot = befLayout.getLineBottom(befLine);

                        // Java: float befTop = befLayout.getLineTop(befLine);

                        // Java: befInc -= (befBot - befTop) * botIncRat;

                        // Java: if (befInc < 0.0F)

                        {
                           // Java: befInc = 0.0F;

                        }
                     }
                  }
                  // Java: if (i < datas.size() - 1 && layout.getLineCount() - 1 == endLineInPara)

                  {
                     // Java: aftInc = (float)data.mPara.mSpcAft;

                  }
                  // Java: float endInc = aftInc;

                  // Java: if (startLineInPara == endLineInPara)

                  {
                     // Java: endInc = aftInc + befInc;

                  }
                  // Java: WpsPath path = new WpsPath(i);

                  // Java: this.getSelectionAreas(data, path, layout, renderStart, renderEnd);

                  // Java: PointF pos = new PointF(typoOffset.x, typoOffset.y);

                  // Java: if (endInc > 0.0F || botIncRat > 0.0F)

                  {
                     // Java: float bottom = layout.getLineBottom(endLineInPara);

                     // Java: float top = layout.getLineTop(endLineInPara);

                     // Java: botInc = (bottom - top) * botIncRat;

                     // Java: endInc += botInc;

                     // Java: Matrix matrix = new Matrix();

                     // Java: this.scalePath(matrix, 1.0F, (float)((double)(endInc / (bottom - top)) + (double)1.0F), 0.0F, top);

                     // Java: path.transform(matrix);

                     // Java: if (startLineInPara == endLineInPara)

                     {
                        // Java: this.incTypoOffset(pos, befInc);

                     }
                  }
                  // Java: paths.add(path);

                  // Java: moffsets.add(pos);

               }
            }
            // Java: float befInc = 0.0F;

            // Java: float aftInc = 0.0F;

            // Java: float botIncRat = 0.0F;

            // Java: float botInc = 0.0F;

            // Java: if (null != data.mBidi && ANDROID_VERSION < 21 && data.mPara.mSpcWithinPct < 0.9 && data.mPara.mSpcWithinPct > (double)0.0F)

            {
               // Java: botIncRat = (float)((double)1.0F / data.mPara.mSpcWithinPct - 1.1111111111111112);

            }
            // Java: if (0 == startlineInPara && i > 0)

            {
               // Java: befInc = (float)data.mPara.mSpcBef;

               // Java: if (botIncRat > 0.0F)

               {
                  // Java: Layout befLayout = (Layout)this.mTypoCaches.get(i - 1);

                  // Java: int befLine = befLayout.getLineCount() - 1;

                  // Java: float befBot = befLayout.getLineBottom(befLine);

                  // Java: float befTop = befLayout.getLineTop(befLine);

                  // Java: befInc -= (befBot - befTop) * botIncRat;

                  // Java: if (befInc < 0.0F)

                  {
                     // Java: befInc = 0.0F;

                  }
               }
            }
            // Java: if (i < datas.size() - 1 && layout.getLineCount() - 1 == endPosition)

            {
               // Java: aftInc = (float)data.mPara.mSpcAft;

            }
            // Java: if (i == startLogPos.paraIndex)

            {
               // Java: startline = layout.getLineForOffset(firstPosition);

               // Java: int logicOffset = data.mPara.invertMap == null ? firstPosition : data.mPara.invertMap[firstPosition];

               // Java: float dist = this.isPrimary(data.mBidi, logicOffset) ? layout.getPrimaryHorizontal(firstPosition) : layout.getSecondaryHorizontal(firstPosition);

               // Java: if (this.isAllVertLTR())

               {
                  // Java: selDragStartP = new PointF(typoOffset.x + layout.getLineBottom(startline), typoOffset.y + dist);

               }
               // Java: else if (this.isAllVertRTl())

               {
                  // Java: selDragStartP = new PointF(typoOffset.x - layout.getLineBottom(startline), typoOffset.y + dist);

               }
               // Java: else

               {
                  // Java: selDragStartP = new PointF(typoOffset.x + dist, typoOffset.y + layout.getLineBottom(startline));

               }
               // Java: startLineHeight = layout.getLineBottom(startline) - layout.getLineTop(startline) + befInc;

               // Java: if (startline == layout.getLineForOffset(endPosition))

               {
                  // Java: startLineHeight += aftInc + botInc;

                  // Java: selDragStartP.y += aftInc + botInc;

               }
               // Java: isDragStartLtr = this.isLeftToRight(data.mBidi, startInPara);

            }
            // Java: if (i == endLogPos.paraIndex)

            {
               // Java: endline = layout.getLineForOffset(endPosition);

               // Java: int lineEnd = layout.getLineEnd(endline);

               // Java: layout.getLineStart(endline);

               // Java: endPosition = endPosition >= lineEnd ? lineEnd - 1 : endPosition;

               // Java: int logicOffset = data.mPara.invertMap == null ? endPosition : data.mPara.invertMap[endPosition];

               // Java: float dist = this.isPrimary(data.mBidi, logicOffset) ? layout.getSecondaryHorizontal(endPosition) : layout.getPrimaryHorizontal(endPosition);

               // Java: if (this.isAllVertLTR())

               {
                  // Java: selDragEndP = new PointF(typoOffset.x + layout.getLineBottom(endline), typoOffset.y + dist + aftInc + botInc);

               }
               // Java: else if (this.isAllVertRTl())

               {
                  // Java: selDragEndP = new PointF(typoOffset.x - layout.getLineBottom(endline), typoOffset.y + dist + aftInc + botInc);

               }
               // Java: else

               {
                  // Java: selDragEndP = new PointF(typoOffset.x + dist, typoOffset.y + layout.getLineBottom(endline) + aftInc + botInc);

               }
               // Java: endLineHeight = layout.getLineBottom(endline) - layout.getLineTop(endline) + aftInc + botInc;

               // Java: if (endline == layout.getLineForOffset(firstPosition))

               {
                  // Java: endLineHeight += befInc;

               }
               // Java: isDragEndLtr = this.isLeftToRight(data.mBidi, endPosition);

            }
         }
         // Java: else

         {
            // Java: if (0 == endInPara && ((TextData)datas.get(i)).mTypoText.length() > 0)

            {
               // Java: endInPara = 1;

            }
            // Java: int restStart = startInPara;

            // Java: int restEnd = endInPara - 1;

            // Java: Layout layout = (Layout)this.mTypoCaches.get(i);

            // Java: int startLineInPara = layout.getLineForOffset(startInPara);

            // Java: int endLineInPara = layout.getLineForOffset(restEnd);

            // Java: PointF typoOffset = (PointF)this.mTypoOffsets.get(i);

            // Java: if (this.isAllVert())

            {
               // Java: typoOffset = new PointF(typoOffset.y, typoOffset.x);

            }
            // Java: else

            {
               // Java: typoOffset = new PointF(typoOffset.x, typoOffset.y);

            }
            // Java: float befInc = 0.0F;

            // Java: float aftInc = 0.0F;

            // Java: float botIncRat = 0.0F;

            // Java: float botInc = 0.0F;

            // Java: if (null != data.mBidi && ANDROID_VERSION < 21 && data.mPara.mSpcWithinPct < 0.9 && data.mPara.mSpcWithinPct > (double)0.0F)

            {
               // Java: botIncRat = (float)((double)1.0F / data.mPara.mSpcWithinPct - 1.1111111111111112);

            }
            // Java: if (0 == startLineInPara && i > 0)

            {
               // Java: befInc = (float)data.mPara.mSpcBef;

               // Java: if (botIncRat > 0.0F)

               {
                  // Java: Layout befLayout = (Layout)this.mTypoCaches.get(i - 1);

                  // Java: int befLine = befLayout.getLineCount() - 1;

                  // Java: float befBot = befLayout.getLineBottom(befLine);

                  // Java: float befTop = befLayout.getLineTop(befLine);

                  // Java: befInc -= (befBot - befTop) * botIncRat;

                  // Java: if (befInc < 0.0F)

                  {
                     // Java: befInc = 0.0F;

                  }
               }
            }
            // Java: if (i < datas.size() - 1 && layout.getLineCount() - 1 == endLineInPara)

            {
               // Java: aftInc = (float)data.mPara.mSpcAft;

            }
            // Java: if (befInc > 0.0F && startLineInPara < endLineInPara)

            {
               // Java: WpsPath path = new WpsPath(i);

               // Java: int end = layout.getLineEnd(startLineInPara) - 1;

               // Java: this.getSelectionAreas(data, path, layout, startInPara, end);

               // Java: float bottom = layout.getLineBottom(startLineInPara);

               // Java: float top = layout.getLineTop(startLineInPara);

               // Java: Matrix matrix = new Matrix();

               // Java: this.scalePath(matrix, 1.0F, (float)((double)(befInc / (bottom - top)) + (double)1.0F), 0.0F, top);

               // Java: path.transform(matrix);

               // Java: paths.add(path);

               // Java: PointF pos = new PointF(typoOffset.x, typoOffset.y);

               // Java: this.incTypoOffset(pos, befInc);

               // Java: moffsets.add(pos);

               // Java: restStart = layout.getLineStart(startLineInPara + 1);

            }
            // Java: float endInc = aftInc;

            // Java: if (startLineInPara == endLineInPara)

            {
               // Java: endInc = aftInc + befInc;

            }
            // Java: if (endInc > 0.0F || botIncRat > 0.0F)

            {
               // Java: int start = endLineInPara > startLineInPara ? layout.getLineStart(endLineInPara) : startInPara;

               // Java: int end = endInPara - 1;

               // Java: WpsPath path = new WpsPath(i);

               // Java: this.getSelectionAreas(data, path, layout, start, end);

               // Java: float bottom = layout.getLineBottom(endLineInPara);

               // Java: float top = layout.getLineTop(endLineInPara);

               // Java: botInc = (bottom - top) * botIncRat;

               // Java: endInc += botInc;

               // Java: Matrix matrix = new Matrix();

               // Java: this.scalePath(matrix, 1.0F, (float)((double)(endInc / (bottom - top)) + (double)1.0F), 0.0F, top);

               // Java: path.transform(matrix);

               // Java: paths.add(path);

               // Java: restEnd = endLineInPara > 0 ? layout.getLineEnd(endLineInPara - 1) - 1 : -1;

               // Java: PointF pos = new PointF(typoOffset.x, typoOffset.y);

               // Java: if (startLineInPara == endLineInPara)

               {
                  // Java: this.incTypoOffset(pos, befInc);

               }
               // Java: moffsets.add(pos);

            }
            // Java: if (restStart <= restEnd)

            {
               // Java: WpsPath path = new WpsPath(i);

               // Java: this.getSelectionAreas(data, path, layout, restStart, restEnd);

               // Java: PointF pos = new PointF(typoOffset.x, typoOffset.y);

               // Java: paths.add(path);

               // Java: moffsets.add(pos);

            }
            // Java: if (i == startLogPos.paraIndex)

            {
               // Java: startline = startLineInPara;

               // Java: float dist = this.isPrimary(data.mBidi, startInPara) ? layout.getPrimaryHorizontal(startInPara) : layout.getSecondaryHorizontal(startInPara);

               // Java: if (this.isAllVertLTR())

               {
                  // Java: selDragStartP = new PointF(typoOffset.x + layout.getLineBottom(startLineInPara), typoOffset.y + dist);

               }
               // Java: else if (this.isAllVertRTl())

               {
                  // Java: selDragStartP = new PointF(typoOffset.x - layout.getLineBottom(startLineInPara), typoOffset.y + dist);

               }
               // Java: else

               {
                  // Java: selDragStartP = new PointF(typoOffset.x + dist, typoOffset.y + layout.getLineBottom(startLineInPara));

               }
               // Java: startLineHeight = layout.getLineBottom(startLineInPara) - layout.getLineTop(startLineInPara) + befInc;

               // Java: if (startLineInPara == endLineInPara)

               {
                  // Java: startLineHeight += aftInc + botInc;

                  // Java: selDragStartP.y += aftInc + botInc;

               }
               // Java: isDragStartLtr = this.isLeftToRight(data.mBidi, startInPara);

            }
            // Java: if (i == endLogPos.paraIndex)

            {
               // Java: endline = endLineInPara;

               // Java: int includeEnd = endInPara > startInPara ? endInPara - 1 : endInPara;

               // Java: int lineEnd = layout.getLineEnd(endLineInPara);

               // Java: int lineStart = layout.getLineStart(endLineInPara);

               // Java: float dist;

               // Java: if (endInPara != lineEnd)

               {
                  // Java: dist = this.isPrimary(data.mBidi, includeEnd) ? layout.getPrimaryHorizontal(endInPara) : layout.getSecondaryHorizontal(endInPara);

               }
               // Java: else if (null != data.mBidi && Bidi.requiresBidi(data.mTypoText.toCharArray(), lineStart, lineEnd) && (data.mBidi.getBaseLevel() % 2 != 0 || data.mBidi.getLevelAt(includeEnd) % 2 != 0))

               {
                  // Java: if (data.mBidi.getBaseLevel() % 2 == 1 && data.mBidi.getLevelAt(includeEnd) % 2 == 1)

                  {
                     // Java: dist = layout.getLineLeft(endLineInPara);

                  }
                  // Java: else

                  {
                     // Java: float xbef = this.isPrimary(data.mBidi, includeEnd) ? layout.getPrimaryHorizontal(includeEnd) : layout.getSecondaryHorizontal(includeEnd);

                     // Java: float cw = PaintFontHelper.measureCharWidth(data, includeEnd, endInPara);

                     // Java: dist = data.mBidi.getLevelAt(includeEnd) == 0 ? xbef + cw : xbef - cw;

                  }
               }
               // Java: else

               {
                  // Java: dist = layout.getLineRight(endLineInPara);

               }
               // Java: if (this.isAllVertLTR())

               {
                  // Java: selDragEndP = new PointF(typoOffset.x + layout.getLineBottom(endLineInPara), typoOffset.y + dist + aftInc + botInc);

               }
               // Java: else if (this.isAllVertRTl())

               {
                  // Java: selDragEndP = new PointF(typoOffset.x - layout.getLineBottom(endLineInPara), typoOffset.y + dist + aftInc + botInc);

               }
               // Java: else

               {
                  // Java: selDragEndP = new PointF(typoOffset.x + dist, typoOffset.y + layout.getLineBottom(endLineInPara) + aftInc + botInc);

               }
               // Java: endLineHeight = layout.getLineBottom(endLineInPara) - layout.getLineTop(endLineInPara) + aftInc + botInc;

               // Java: if (endLineInPara == startLineInPara)

               {
                  // Java: endLineHeight += befInc;

               }
               // Java: isDragEndLtr = this.isLeftToRight(data.mBidi, includeEnd);

            }
         }
      }
      // Java: if (0 != this.mRotDegrees && this.mTextBoxData.presetTextWarp() == 0)

      {
         // Java: List<PointF> offsets = new ArrayList(moffsets);

         // Java: if (this.isThaiLanguage())

         {
            // Java: offsets.add(offsetStPoint);

            // Java: offsets.add(offsetEndPoint);

         }
         // Java: else

         {
            // Java: offsets.add(selDragStartP);

            // Java: offsets.add(selDragEndP);

         }
         // Java: this.rotateSelCur(paths, offsets, (PointF)null);

      }
      // Java: seleParam.setPaths(paths);

      // Java: seleParam.setMoffsets(moffsets);

      // Java: seleParam.setSelectDragParam(selDragStartP, selDragEndP, this.mRotDegrees, isDragStartLtr, isDragEndLtr, startLineHeight, endLineHeight, startline == endline && startLogPos.paraIndex == endLogPos.paraIndex, this.mTextBoxData.verticalText());

      // Java: if (this.mTextBoxData.presetTextWarp() != 0 && null != this.mArtTransform)

      {
         // Java: this.mArtTransform.squareCurSel(seleParam);

      }
    }

    // Java: void scalePath(Matrix matrix, float sx, float sy, float px, float py)
    pub fn scalePath(&self, _matrix: i32, _sx: f32, _sy: f32, _px: f32, _py: f32) {
      // Java: if (this.isAllVertLTR())

      {
         // Java: matrix.setScale(sy, sx, py, px);

      }
      // Java: else if (this.isAllVertRTl())

      {
         // Java: matrix.setScale(sy, sx, -py, px);

      }
      // Java: else

      {
         // Java: matrix.setScale(sx, sy, px, py);

      }
    }

    // Java: void incTypoOffset(PointF pos, float befInc)
    pub fn incTypoOffset(&self, _pos: i32, _befInc: f32) {
      // Java: if (this.isAllVertLTR())

      {
         // Java: pos.x -= befInc;

      }
      // Java: else if (this.isAllVertRTl())

      {
         // Java: pos.x += befInc;

      }
      // Java: else

      {
         // Java: pos.y -= befInc;

      }
    }

    // Java: boolean isThaiLanguage()
    pub fn isThaiLanguage(&self) -> bool {
      // Java: return UILanguage.UILanguage_Thai == Platform.getLanguage();

      false
    }

    // Java: float getTextOffsetY(int index, float offsetY)
    pub fn getTextOffsetY(&self, _index: i32, _offsetY: f32) -> f32 {
      // Java: if (index == 0 && UILanguage.UILanguage_Thai == Platform.getLanguage())

      {
         // Java: float textOffsetY = Math.min(4.0F, offsetY);

         // Java: return textOffsetY;

      }
      // Java: else

      {
         // Java: return 0.0F;

      }
      0.0
    }

    // Java: float expandDist(Layout layout, float dist)
    pub fn expandDist(&self, _layout: i32, _dist: f32) -> f32 {
      // Java: return dist;

      0.0
    }

    // Java: boolean shouldSelectionContinue(Layout layout, WpsPath path)
    pub fn shouldSelectionContinue(&self, _layout: i32, _path: i32) -> bool {
      // Java: return true;

      false
    }

    // Java: void getSelectionAreas(TextData data, WpsPath path, Layout layout, int start, int end)
    pub fn getSelectionAreas(&self, _data: i32, _path: i32, _layout: i32, _start: i32, _end: i32) {
      // Java: int startLine = layout.getLineForOffset(start);

      // Java: int endLine = layout.getLineForOffset(end);

      // Java: float spcWidth = PaintFontHelper.getCharWidth(new char[]{' '}, data.getLastSpan());

      // Java: if (this.shouldSelectionContinue(layout, path))

      {
         // Java: for(int i = startLine; i <= endLine; ++i)

         {
            // Java: int lineEnd = layout.getLineEnd(i);

            // Java: int offStart = i == startLine ? start : layout.getLineStart(i);

            // Java: int offEnd = i == endLine ? end : lineEnd;

            // Java: float left = 0.0F;

            // Java: float right = 0.0F;

            // Java: float top = layout.getLineTop(i);

            // Java: float bottom = layout.getLineBottom(i);

            // Java: if (this.isAllVertRTl())

            {
               // Java: top = -top;

               // Java: bottom = -bottom;

            }
            // Java: if (i > startLine && i < endLine)

            {
               // Java: left = layout.getLineLeft(i);

               // Java: right = layout.getLineRight(i);

               // Java: if (0 == Float.compare(left, right) && !data.hasText())

               {
                  // Java: right += spcWidth;

               }
            }
            // Java: else

            {
               // Java: if (i > startLine)

               {
                  // Java: left = layout.getLineLeft(i);

               }
               // Java: else

               {
                  // Java: left = layout.getPrimaryHorizontal(offStart);

               }
               // Java: if (offEnd != lineEnd && i >= endLine)

               {
                  // Java: right = layout.getSecondaryHorizontal(offEnd);

               }
               // Java: else

               {
                  // Java: right = layout.getLineRight(i);

               }
               // Java: if (0 == Float.compare(left, right) && !data.hasText())

               {
                  // Java: right += spcWidth;

               }
            }
            // Java: if (this.isAllVert())

            {
               // Java: path.addRect(top, left, bottom, right, Direction.CCW);

            }
            // Java: else

            {
               // Java: path.addRect(left, top, right, bottom, Direction.CCW);

            }
         }
         // Java: path.mergeRects(this.isAllVert());

      }
    }

    // Java: boolean isPrimary(Bidi bidi, int offset)
    pub fn isPrimary(&self, _bidi: i32, _offset: i32) -> bool {
      // Java: if (null == bidi)

      {
         // Java: return true;

      }
      // Java: else

      {
         // Java: return bidi.getLevelAt(offset) % 2 == 0;

      }
      false
    }

    // Java: boolean isLeftToRight(Bidi bidi, int offset)
    pub fn isLeftToRight(&self, _bidi: i32, _offset: i32) -> bool {
      // Java: if (null == bidi)

      {
         // Java: return true;

      }
      // Java: else

      {
         // Java: return bidi.getLevelAt(offset) % 2 == 0;

      }
      false
    }

    // Java: boolean isRangeRightToLeft(Bidi bidi, int start, int end)
    pub fn isRangeRightToLeft(&self, _bidi: i32, _start: i32, _end: i32) -> bool {
      // Java: if (null == bidi)

      {
         // Java: return false;

      }
      // Java: else

      {
         // Java: if (start > end)

         {
            // Java: int temp = start;

            // Java: start = end;

            // Java: end = temp;

         }
         // Java: for(int i = start; i < end; ++i)

         {
            // Java: if (bidi.getLevelAt(i) % 2 == 0)

            {
               // Java: return false;

            }
         }
         // Java: return true;

      }
      false
    }

    // Java: int indexOfParagraphVertical(float touchY)
    pub fn indexOfParagraphVertical(&self, _touchY: f32) -> i32 {
      // Java: int count = this.mParaBottoms.size() - 1;

      // Java: if (count < 1)

      {
         // Java: return count;

      }
      // Java: else if (touchY <= (Float)this.mParaBottoms.get(0) && !this.isAllVertRTl())

      {
         // Java: return 0;

      }
      // Java: else if (touchY >= (Float)this.mParaBottoms.get(0) && this.isAllVertRTl())

      {
         // Java: return 0;

      }
      // Java: else

      {
         // Java: for(int i = 1; i < count; ++i)

         {
            // Java: if (this.isAllVertRTl())

            {
               // Java: if ((Float)this.mParaBottoms.get(i - 1) >= touchY && touchY > (Float)this.mParaBottoms.get(i))

               {
                  // Java: return i;

               }
            }
            // Java: else if ((Float)this.mParaBottoms.get(i - 1) < touchY && touchY <= (Float)this.mParaBottoms.get(i))

            {
               // Java: return i;

            }
         }
         // Java: return count;

      }
      0
    }

    // Java: LogicalPosition getLogPosByOffset(int offset)
    pub fn getLogPosByOffset(&self, _offset: i32) -> i32 {
      // Java: return this.getLogPosByOffset(offset, false);

      0
    }

    // Java: LogicalPosition getLogPosByOffset(int offset, boolean atLineEnd)
    pub fn getLogPosByOffset_2(&self, _offset: i32, _atLineEnd: bool) -> i32 {
      // Java: LogicalPosition logPos = new LogicalPosition();

      // Java: int curOffset = 0;

      // Java: int preCurOffset = 0;

      // Java: boolean isLastPara = false;

      // Java: List<TextData> datas = this.mTextBoxData.getTextDatas();

      // Java: for(int i = 0; i < datas.size(); ++i)

      {
         // Java: String sourceText = ((TextData)datas.get(i)).mSourceText;

         // Java: int textlength = sourceText.length();

         // Java: curOffset += textlength;

         // Java: if (datas.size() - 1 == i && curOffset <= offset)

         {
            // Java: offset = curOffset;

            // Java: isLastPara = true;

         }
         // Java: if (offset < curOffset || isLastPara)

         {
            // Java: logPos.paraIndex = i;

            // Java: logPos.offset = offset - preCurOffset;

            // Java: break;

         }
         // Java: preCurOffset = curOffset;

      }
      // Java: logPos.offset = this.offsetAfterReplace((TextData)datas.get(logPos.paraIndex), logPos.offset);

      // Java: Layout layout = (Layout)this.mTypoCaches.get(logPos.paraIndex);

      // Java: CharSequence text = layout.getText();

      // Java: if (logPos.offset >= text.length())

      {
         // Java: logPos.spaceNum = logPos.offset - text.length();

         // Java: logPos.offset = text.length();

         // Java: if (text.length() > 0 && ' ' == text.charAt(text.length() - 1))

         {
            // Java: ++logPos.spaceNum;

            // Java: --logPos.offset;

         }
      }
      // Java: logPos.lineIndex = layout.getLineForOffset(logPos.offset);

      // Java: if (atLineEnd)

      {
         // Java: int step = logPos.lineIndex > 0 ? 1 : 0;

         // Java: int end = layout.getLineEnd(logPos.lineIndex - step);

         // Java: if (end == logPos.offset)

         {
            // Java: logPos.lineIndex -= step;

            // Java: logPos.isAtLineEnd = true;

         }
         // Java: else

         {
            // Java: end = layout.getLineEnd(logPos.lineIndex);

            // Java: logPos.isAtLineEnd = end == logPos.offset;

         }
      }
      // Java: this.calcLogPosAscent(logPos);

      // Java: return logPos;

      0
    }

    // Java: void setTextRect(float width, float height)
    pub fn setTextRect(&self, _width: f32, _height: f32) {
      // Java: if (this.isAutoWidth())

      {
         // Java: this.mTextRect.set(this.mTextPos);

      }
      // Java: else

      {
         // Java: float left = this.mTextRect.left;

         // Java: float top = this.mTextRect.top;

         // Java: float right = left + width;

         // Java: float bottom = top + height;

         // Java: this.mTextRect = new RectF(left, top, right, bottom);

      }
    }

    // Java: void drawChineseVertText(Layout layout, TextData data, Canvas canvas, int start, int end)
    pub fn drawChineseVertText(&self, _layout: i32, _data: i32, _canvas: i32, _start: i32, _end: i32) {
      // Java: Paint paint = new Paint(layout.getPaint());

      // Java: Spannable text = (Spannable)layout.getText();

      // Java: ChineseVertSpan[] spans = (ChineseVertSpan[])text.getSpans(0, text.length(), ChineseVertSpan.class);

      // Java: for(int i = 0; i < spans.length; ++i)

      {
         // Java: ChineseVertSpan span = spans[i];

         // Java: paint.setTextSize(span.getFontSize());

         // Java: paint.setColor(span.getFontColor());

         // Java: paint.setTypeface(span.getTypeface());

         // Java: if ((span.getFontStyle() & 1) != 0)

         {
            // Java: paint.setFakeBoldText(true);

         }
         // Java: if ((span.getFontStyle() & 2) != 0)

         {
            // Java: paint.setTextSkewX(-0.25F);

         }
         // Java: paint.getFontMetricsInt(this.fm);

         // Java: int spanStart = span.getStart();

         // Java: int spanEnd = span.getEnd();

         // Java: if (-1 != start && spanStart < start)

         {
            // Java: spanStart = start;

         }
         // Java: if (-1 != end && spanEnd > end)

         {
            // Java: spanEnd = end;

         }
         // Java: boolean canRotate = this.mTextBoxData.canChineseVert();

         // Java: for(int j = spanStart; j < spanEnd; ++j)

         {
            // Java: int offset = 1;

            // Java: if (Emoji.isEmojiChar(text.charAt(j)))

            {
               // Java: int emoji = Character.codePointAt(text, j);

               // Java: offset = Character.charCount(emoji);

            }
            // Java: float x = this.isPrimary(data.mBidi, j) ? layout.getPrimaryHorizontal(j) : layout.getSecondaryHorizontal(j);

            // Java: float cw = PaintFontHelper.measureCharWidth(data, j, j + offset);

            // Java: x = (float)((double)x + (double)cw * (canRotate ? 0.8 : (double)0.0F));

            // Java: float y;

            // Java: if (TextData.isChinese(text.charAt(j)))

            {
               // Java: y = layout.getLineBaseline(layout.getLineForOffset(j));

            }
            // Java: else

            {
               // Java: int lineOffset = layout.getLineForOffset(j);

               // Java: y = (layout.getLineTop(lineOffset) + layout.getLineBottom(lineOffset)) / 2.0F;

            }
            // Java: if (span.isSuperScript())

            {
               // Java: y = (float)((double)y - (double)cw * 0.2);

            }
            // Java: else if (span.isSubScript())

            {
               // Java: y = (float)((double)y + (double)cw * 0.4);

            }
            // Java: else

            {
               // Java: y = (float)((double)y + (double)cw * (canRotate ? 0.2 : (double)0.0F));

            }
            // Java: if (canRotate)

            {
               // Java: canvas.rotate(-90.0F, x, y);

               // Java: canvas.drawText(text.subSequence(j, j + offset).toString(), x, y, paint);

               // Java: canvas.rotate(90.0F, x, y);

            }
            // Java: else

            {
               // Java: canvas.drawText(text.subSequence(j, j + offset).toString(), x, y, paint);

            }
            // Java: j += offset - 1;

         }
      }
    }

    // Java: boolean isCoorInRun(float x, int start, int end, Layout layout, TextData data)
    pub fn isCoorInRun(&self, _x: f32, _start: i32, _end: i32, _layout: i32, _data: i32) -> bool {
      // Java: int line = layout.getLineForOffset(start);

      // Java: int lineEnd = layout.getLineEnd(line);

      // Java: if (end > lineEnd)

      {
         // Java: end = lineEnd;

      }
      // Java: float x1 = this.getRunStartBorder(start, layout, data);

      // Java: float x2 = this.getRunEndBorder(end, layout, data);

      // Java: return x1 <= x && x <= x2 || x1 >= x && x >= x2;

      false
    }

    // Java: float getRunEndBorder(int end, Layout layout, TextData data)
    pub fn getRunEndBorder(&self, _end: i32, _layout: i32, _data: i32) -> f32 {
      // Java: int offset = end > 0 ? end - 1 : 0;

      // Java: float x = 0.0F;

      // Java: x = this.isPrimary(data.mBidi, offset) ? layout.getPrimaryHorizontal(offset) : layout.getSecondaryHorizontal(offset);

      // Java: float w = PaintFontHelper.measureCharWidth(data, offset, end);

      // Java: x = data.mBidi.getLevelAt(offset) % 2 == 0 ? x + w : x - w;

      // Java: return x;

      0.0
    }

    // Java: float getRunStartBorder(int start, Layout layout, TextData data)
    pub fn getRunStartBorder(&self, _start: i32, _layout: i32, _data: i32) -> f32 {
      // Java: float x = this.isPrimary(data.mBidi, start) ? layout.getPrimaryHorizontal(start) : layout.getSecondaryHorizontal(start);

      // Java: return x;

      0.0
    }

    // Java: void rotateSelCur(List<WpsPath> paths, List<PointF> offsets, PointF cursorPoint)
    pub fn rotateSelCur(&self, _paths: i32, _offsets: i32, _cursorPoint: i32) {
      // Java: if (0 != this.mRotDegrees && null != paths && null != offsets)

      {
         // Java: Matrix matrix = new Matrix();

         // Java: switch (this.mRotDegrees)

         {
            // Java: case 90:

               // Java: matrix.preTranslate(this.mTypoRect.height(), 0.0F);

               // Java: matrix.preRotate((float)this.mRotDegrees, 0.0F, 0.0F);

               // Java: break;

            // Java: case 270:

               // Java: matrix.preTranslate(0.0F, this.mTypoRect.width());

               // Java: matrix.preRotate((float)this.mRotDegrees, 0.0F, 0.0F);

               // Java: break;

            // Java: default:

               // Java: return;

         }
         // Java: float[] xy = new float[2];

         // Java: for(int i = 0; i < offsets.size(); ++i)

         {
            // Java: PointF offset = (PointF)offsets.get(i);

            // Java: if (offset != null)

            {
               // Java: xy[0] = offset.x;

               // Java: xy[1] = offset.y;

               // Java: matrix.mapPoints(xy);

               // Java: offset.x = xy[0];

               // Java: offset.y = xy[1];

            }
         }
         // Java: if (cursorPoint != null)

         {
            // Java: xy[0] = cursorPoint.x;

            // Java: xy[1] = cursorPoint.y;

            // Java: matrix.mapPoints(xy);

            // Java: cursorPoint.set(xy[0], xy[1]);

         }
         // Java: matrix.reset();

         // Java: matrix.preRotate((float)this.mRotDegrees);

         // Java: for(int i = 0; i < paths.size(); ++i)

         {
            // Java: WpsPath path = (WpsPath)paths.get(i);

            // Java: path.transform(matrix);

         }
      }
    }

    // Java: void rotateBack(float[] pts)
    pub fn rotateBack(&self, _pts: Vec<f32>) {
      // Java: if (this.mRotDegrees != 0)

      {
         // Java: switch (this.mRotDegrees)

         {
            // Java: case 90:

               // Java: Matrix matrix = new Matrix();

               // Java: matrix.preRotate((float)(-this.mRotDegrees));

               // Java: matrix.preTranslate(-this.mTypoRect.height(), 0.0F);

               // Java: matrix.mapPoints(pts);

               // Java: break;

            // Java: case 270:

               // Java: matrix = new Matrix();

               // Java: matrix.preRotate((float)(-this.mRotDegrees));

               // Java: matrix.preTranslate(0.0F, -this.mTypoRect.width());

               // Java: matrix.mapPoints(pts);

         }
      }
    }

    // Java: boolean hasWordArtTypo()
    pub fn hasWordArtTypo(&self) -> bool {
      // Java: return this.mWordArtTypo != null;

      false
    }

    // Java: int incrementOffset(int glOffset, LogicalPosition logPos)
    pub fn incrementOffset(&self, _glOffset: i32, _logPos: i32) -> i32 {
      // Java: String text = this.mTextBoxData.getTextData(logPos.paraIndex).mSourceText;

      // Java: int offset = logPos.offset;

      // Java: int end = text.length();

      // Java: if (offset >= end)

      {
         // Java: return glOffset;

      }
      // Java: else

      {
         // Java: char before;

         // Java: char current;

         // Java: do

         {
            // Java: ++offset;

            // Java: if (offset >= end - 1)

            {
               // Java: break;

            }
            // Java: before = text.charAt(offset - 1);

            // Java: current = text.charAt(offset);

         }
         // Java: return glOffset + offset - logPos.offset;

      }
      0
    }

    // Java: int decrementOffset(int glOffset, LogicalPosition logPos)
    pub fn decrementOffset(&self, _glOffset: i32, _logPos: i32) -> i32 {
      // Java: String text = this.mTextBoxData.getTextData(logPos.paraIndex).mSourceText;

      // Java: int offset = logPos.offset;

      // Java: if (offset <= 0)

      {
         // Java: return glOffset;

      }
      // Java: else

      {
         // Java: char current;

         // Java: char before;

         // Java: do

         {
            // Java: --offset;

            // Java: if (offset <= 0)

            {
               // Java: break;

            }
            // Java: current = text.charAt(offset);

            // Java: before = text.charAt(offset - 1);

         }
         // Java: return glOffset + offset - logPos.offset;

      }
      0
    }

    // Java: boolean isAllVert()
    pub fn isAllVert(&self) -> bool {
      // Java: return this.mTextBoxData.verticalText() == 3 || this.mTextBoxData.verticalText() == 6;

      false
    }

    // Java: boolean isAllVertLTR()
    pub fn isAllVertLTR(&self) -> bool {
      // Java: return this.mTextBoxData.verticalText() == 3;

      false
    }

    // Java: boolean isAllVertRTl()
    pub fn isAllVertRTl(&self) -> bool {
      // Java: return this.mTextBoxData.verticalText() == 6;

      false
    }

}

// Java: public enum SelectionType
impl SelectionType {
    // Java: SelectionType()
    pub fn SelectionType(&self) {

    }

}

// Java: public class LogicalPosition
impl LogicalPosition {
    // Java: LogicalPosition()
    pub fn LogicalPosition(&self) {
         // Java: super();

    }

    // Java: boolean equals(LogicalPosition logPos)
    pub fn equals(&self, _logPos: i32) -> bool {
         // Java: return this.paraIndex == logPos.paraIndex && this.offset == logPos.offset;

      false
    }

}
