#![allow(dead_code, unused_variables, unused_mut, unreachable_code, non_snake_case, unused_imports, non_camel_case_types)]

use std::collections::HashMap;

type Byte = u8;
type Boolean = bool;
type Integer = i32;
type Float = f32;
type List<T> = Vec<T>;
type Map<K, V> = HashMap<K, V>;

struct TextBoxData;
struct ArtTransform;
struct TypoLayoutGen;
struct Layout;
struct PointF { x: f32, y: f32 }
struct RectF { left: f32, top: f32, right: f32, bottom: f32 }
struct Rect;
struct Canvas;
struct Bitmap;
struct BitmapShader;
struct Matrix;
struct TextPaint;
struct Spannable;
struct SpannableStringBuilder;
struct CharacterStyle;
struct TextData;
#[derive(Clone)]
struct SpanInfo;
struct TextShadow;
struct TextOutline;
struct TimingInfo;
struct ShadowLayoutGroup;
struct ShadowLayoutData;
struct ShadowLayoutType;
struct ShadowRenderData;
struct ShadowTypoData;
struct ShadowImageData;
struct SingleImageSpannable;
struct WordArtVertData;
struct DecoLineCache;
struct FontFill;
struct ForegroundColorSpan;
struct LeadingMarginSpan;
struct Range;
struct TextRangeType;
struct TextUtil;
struct Emoji;
struct WpsLayout;
struct Color;
struct EmbossMaskFilter;
struct PorterDuffXfermode;
struct Xfermode;
struct Config;
struct Join;
struct Style;
struct Mode;
struct TileMode;
struct Point;
struct RichTextLayout;
struct ChineseVertSpan;
struct WpsImageSpan;
struct Bidi;
struct WpsTextOutlineSpan;

static EMPTY_CHAR_STYLE: i32 = 0;

// Java: import android.graphics.Bitmap

// Java: import android.graphics.BitmapShader

// Java: import android.graphics.Canvas

// Java: import android.graphics.Color

// Java: import android.graphics.EmbossMaskFilter

// Java: import android.graphics.Matrix

// Java: import android.graphics.Point

// Java: import android.graphics.PointF

// Java: import android.graphics.PorterDuffXfermode

// Java: import android.graphics.Rect

// Java: import android.graphics.RectF

// Java: import android.graphics.Xfermode

// Java: import android.graphics.Bitmap.Config

// Java: import android.graphics.Paint.Join

// Java: import android.graphics.Paint.Style

// Java: import android.graphics.PorterDuff.Mode

// Java: import android.graphics.Shader.TileMode

// Java: import android.text.TextPaint

// Java: import cn.wps.text.layout.data.DecoLineCache

// Java: import cn.wps.text.layout.data.FontFill

// Java: import cn.wps.text.layout.data.SpanInfo

// Java: import cn.wps.text.layout.data.TextBoxData

// Java: import cn.wps.text.layout.data.TextData

// Java: import cn.wps.text.layout.data.TextOutline

// Java: import cn.wps.text.layout.data.TextShadow

// Java: import cn.wps.text.layout.data.TimingInfo

// Java: import cn.wps.text.layout.layouter.Emoji

// Java: import cn.wps.text.layout.layouter.WpsLayout

// Java: import cn.wps.text.layout.typo.data.ShadowImageData

// Java: import cn.wps.text.layout.typo.data.ShadowLayoutData

// Java: import cn.wps.text.layout.typo.data.ShadowLayoutGroup

// Java: import cn.wps.text.layout.typo.data.ShadowLayoutType

// Java: import cn.wps.text.layout.typo.data.ShadowRenderData

// Java: import cn.wps.text.layout.typo.data.ShadowTypoData

// Java: import cn.wps.text.layout.typo.data.SingleImageSpannable

// Java: import cn.wps.text.layout.typo.data.WordArtVertData

// Java: import cn.wps.text.layout.typo.layout.Layout

// Java: import cn.wps.text.layout.typo.layout.Spannable

// Java: import cn.wps.text.layout.typo.layout.SpannableStringBuilder

// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle

// Java: import cn.wps.text.layout.typo.layout.style.ForegroundColorSpan

// Java: import cn.wps.text.layout.typo.layout.style.LeadingMarginSpan

// Java: import cn.wps.text.layout.util.Range

// Java: import cn.wps.text.layout.util.TextRangeType

// Java: import cn.wps.text.layout.util.TextUtil

// Java: import java.text.Bidi

// Java: import java.util.ArrayList

// Java: import java.util.HashMap

// Java: import java.util.List

// Java: import java.util.Map


// Java field: float LINEHEI_SCALE
pub(crate) const LINEHEI_SCALE: f32 = 1.18;
// Java field: int VERT_RENDER_FILL
pub(crate) const VERT_RENDER_FILL: i32 = 1;
// Java field: int VERT_RENDER_OUTLINE
pub(crate) const VERT_RENDER_OUTLINE: i32 = 2;
// Java field: int VERT_RENDER_BLUR
pub(crate) const VERT_RENDER_BLUR: i32 = 3;
// Java field: int VERT_RENDER_SHADOW
pub(crate) const VERT_RENDER_SHADOW: i32 = 4;
// Java field: int VERT_RENDER_INNER
pub(crate) const VERT_RENDER_INNER: i32 = 5;

// Java: public class WordArtLayout
pub struct WordArtLayout {
    // Java field: TextBoxData mTextBoxData
    mTextBoxData: Option<i32>,
    // Java field: Map<Byte, Boolean> mWordArtAttr
    mWordArtAttr: Option<HashMap<u8, bool>>,
    // Java field: List<PointF> mTypoOffsets
    mTypoOffsets: Option<Vec<i32>>,
    // Java field: ArtTransform mArtTransform
    mArtTransform: Option<i32>,
    // Java field: TypoLayoutGen mTypoLayoutGen
    mTypoLayoutGen: i32,
    // Java field: List<Layout> mTypoFillCaches
    mTypoFillCaches: Option<Vec<i32>>,
    // Java field: List<Layout> mTypoBlurCaches
    mTypoBlurCaches: Option<Vec<i32>>,
    // Java field: List<Layout> mTypoOutlineCaches
    mTypoOutlineCaches: Option<Vec<i32>>,
    // Java field: List<ShadowLayoutGroup> mShadowLayoutCaches
    mShadowLayoutCaches: Option<Vec<i32>>,
    // Java field: Map<Integer, Map<Integer, ShadowTypoData>> mSpanShadowCaches
    mSpanShadowCaches: Option<HashMap<i32, HashMap<i32, i32>>>,
    // Java field: List<Layout> mInnerShadowCaches
    mInnerShadowCaches: Option<Vec<i32>>,
    // Java field: HashMap<Integer, Layout[]> mInnerLayoutMap
    mInnerLayoutMap: Option<HashMap<i32, Vec<i32>>>,
    // Java field: HashMap<Integer, WordArtVertData> mChineseVertCaches
    mChineseVertCaches: Option<HashMap<i32, i32>>,
    // Java field: float mCanvasScaleX
    mCanvasScaleX: f32,
    // Java field: float mCanvasScaleY
    mCanvasScaleY: f32,
    // Java field: boolean hasDynamicDigit
    hasDynamicDigit: bool,
    // Java field: int mRecursionDelta
    mRecursionDelta: i32,
}

// Java: public enum SecondShadowType
pub struct SecondShadowType {
}

// Java: public class WordArtLayout
impl WordArtLayout {
    // Java: WordArtLayout()
    pub fn new(&self) {
      // Java: super();

    }

    // Java: void setScale(float scaleX, float scaleY)
    pub fn setScale(&self, _scaleX: f32, _scaleY: f32) {
      // Java: this.mCanvasScaleX = scaleX;

      // Java: this.mCanvasScaleY = scaleY;

    }

    // Java: boolean checkWordArtAttr(byte attrType)
    pub fn checkWordArtAttr(&self, _attrType: u8) -> bool {
      // Java: if (this.mWordArtAttr == null)

      {
         // Java: return false;

      }
      // Java: else

      {
         // Java: Boolean value = (Boolean)this.mWordArtAttr.get(attrType);

         // Java: return value != null && value;

      }
      false
    }

    // Java: void typo(TextBoxData txtBoxData, List<Layout> typoCaches, List<PointF> typoOffsets, ArtTransform artTransform)
    pub fn typo(&self, _txtBoxData: i32, _typoCaches: i32, _typoOffsets: i32, _artTransform: i32) {
      // Java: this.mTextBoxData = txtBoxData;

      // Java: this.mTypoOffsets = typoOffsets;

      // Java: List<TextData> mDatas = this.mTextBoxData.getTextDatas();

      // Java: this.mArtTransform = artTransform;

      // Java: this.mWordArtAttr = txtBoxData.getWordartAttr();

      // Java: if (this.mTextBoxData.verticalText() == 4)

      {
         // Java: if (this.mChineseVertCaches == null)

         {
            // Java: this.mChineseVertCaches = new HashMap();

         }
         // Java: this.mChineseVertCaches.clear();

         // Java: this.mCanvasScaleX = 1.0F;

         // Java: this.mCanvasScaleY = 1.0F;

      }
      // Java: boolean isFill = this.checkWordArtAttr((byte)1);

      // Java: boolean isOutline = this.checkWordArtAttr((byte)2);

      // Java: boolean isBlur = this.checkWordArtAttr((byte)3);

      // Java: boolean isOuterShadow = this.checkWordArtAttr((byte)5);

      // Java: boolean isInnerShadow = this.checkWordArtAttr((byte)6);

      // Java: if (isFill || isOutline)

      {
         // Java: if (isFill)

         {
            // Java: this.mTypoFillCaches = new ArrayList();

         }
         // Java: if (isOutline)

         {
            // Java: this.mTypoOutlineCaches = new ArrayList();

         }
         // Java: this.typoByLayoutLine(mDatas, typoCaches);

      }
      // Java: if (!isFill)

      {
         // Java: this.mTypoFillCaches = typoCaches;

      }
      // Java: if (isBlur || isInnerShadow || isOuterShadow)

      {
         // Java: if (isBlur)

         {
            // Java: this.mTypoBlurCaches = new ArrayList();

         }
         // Java: if (isInnerShadow)

         {
            // Java: this.mInnerShadowCaches = new ArrayList();

            // Java: this.mInnerLayoutMap = new HashMap();

         }
         // Java: if (isOuterShadow)

         {
            // Java: this.mShadowLayoutCaches = new ArrayList();

            // Java: this.mSpanShadowCaches = new HashMap();

         }
         // Java: this.typoOnlyBySpanInfo(mDatas, typoCaches);

      }
    }

    // Java: void typoByLayoutLine(List<TextData> datas, List<Layout> typoCaches)
    pub fn typoByLayoutLine(&self, _datas: i32, _typoCaches: i32) {
      // Java: if (datas.size() == typoCaches.size())

      {
         // Java: float minTypoX = this.getMinTypoOffsetX();

         // Java: float minTypoY = this.getMinTypoOffsetY();

         // Java: float minLnLeft = this.getMinLeft(typoCaches);

         // Java: for(int i = 0; i < typoCaches.size(); ++i)

         {
            // Java: TextData data = (TextData)datas.get(i);

            // Java: Layout layout = (Layout)typoCaches.get(i);

            // Java: boolean hasFill = data.hasArtFill();

            // Java: boolean hasOutline = data.hasOutline();

            // Java: if (!hasFill && this.mTypoFillCaches != null)

            {
               // Java: this.mTypoFillCaches.add(layout);

            }
            // Java: if (!hasOutline && this.mTypoOutlineCaches != null)

            {
               // Java: this.mTypoOutlineCaches.add(null);

            }
            // Java: if (hasFill || hasOutline)

            {
               // Java: Spannable fillSpannable = hasFill ? new SpannableStringBuilder(layout.getText()) : null;

               // Java: Spannable outlineSpannable = hasOutline ? new SpannableStringBuilder(data.mTypedCharSequence) : null;

               // Java: int flag = 34;

               // Java: int spanInx = 0;

               // Java: int spanSize = data.mOtherSpans.size();

               // Java: int textLen = layout.getText().length();

               // Java: SpanInfo span = data.mFirstSpan;

               // Java: for(int curLine = 0; curLine < layout.getLineCount(); ++curLine)

               {
                  // Java: int lnStartOffset = layout.getLineStart(curLine);

                  // Java: int lnEndOffset = layout.getLineEnd(curLine);

                  // Java: while(span != null && span.mEndPos <= textLen)

                  {
                     // Java: int start = Math.max(lnStartOffset, span.mStartPos);

                     // Java: int end = Math.min(lnEndOffset, span.mEndPos);

                     // Java: float lnTop = layout.getLineTop(curLine);

                     // Java: float lnHei = layout.getLineBaseline(curLine) - lnTop;

                     // Java: float lnWid = layout.getLineWidth(curLine);

                     // Java: float lnLeft = layout.getLineLeft(curLine);

                     // Java: lnHei *= 1.18F;

                     // Java: if (hasFill && fillSpannable != null && span.hasArtFill())

                     {
                        // Java: boolean bRectGrad = span.hasRectShapeGradFill();

                        // Java: lnLeft -= bRectGrad ? minLnLeft - minTypoX : 0.0F;

                        // Java: PointF offset = new PointF();

                        // Java: if (i != 0)

                        {
                           // Java: offset.x = ((PointF)this.mTypoOffsets.get(i)).x - ((PointF)this.mTypoOffsets.get(0)).x;

                           // Java: offset.y = ((PointF)this.mTypoOffsets.get(i)).y - ((PointF)this.mTypoOffsets.get(0)).y;

                        }
                        // Java: CharacterStyle charStyle = new WpsForegroundColorSpan(span.mFontFill, lnHei, lnWid, lnTop, lnLeft, this.mArtTransform, offset, bRectGrad ? minTypoX : minLnLeft, minTypoY);

                        // Java: fillSpannable.setSpan(charStyle, start, end, flag);

                     }
                     // Java: if (hasOutline && outlineSpannable != null)

                     {
                        // Java: CharacterStyle charStyle;

                        // Java: if (span.hasOutline())

                        {
                           // Java: charStyle = this.createOutline(span.mTextOutline, lnWid, lnHei, lnTop, lnLeft);

                        }
                        // Java: else

                        {
                           // Java: charStyle = new ForegroundColorSpan(0);

                        }
                        // Java: outlineSpannable.setSpan(charStyle, start, end, flag);

                     }
                     // Java: if (span.mEndPos <= lnEndOffset)

                     {
                        // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

                        // Java: ++spanInx;

                     }
                     // Java: else

                     {
                        // Java: ++curLine;

                        // Java: lnStartOffset = layout.getLineStart(curLine);

                        // Java: lnEndOffset = layout.getLineEnd(curLine);

                     }
                  }
               }
               // Java: this.createLayout(layout, fillSpannable, data, this.mTypoFillCaches, false);

               // Java: this.createLayout(layout, outlineSpannable, data, this.mTypoOutlineCaches, false);

            }
         }
      }
    }

    // Java: void typoOnlyBySpanInfo(List<TextData> datas, List<Layout> typoCaches)
    pub fn typoOnlyBySpanInfo(&self, _datas: i32, _typoCaches: i32) {
      // Java: if (datas.size() == typoCaches.size())

      {
         // Java: for(int i = 0; i < typoCaches.size(); ++i)

         {
            // Java: TextData data = (TextData)datas.get(i);

            // Java: Layout layout = (Layout)typoCaches.get(i);

            // Java: ShadowLayoutGroup layoutData = new ShadowLayoutGroup();

            // Java: boolean hasBlur = data.hasBlur();

            // Java: boolean hasOutShadow = data.hasOuterShdw();

            // Java: boolean hasInnerShadow = data.hasInnerShdw();

            // Java: if (!hasBlur && this.mTypoBlurCaches != null)

            {
               // Java: this.mTypoBlurCaches.add(null);

            }
            // Java: if (!hasOutShadow && this.mShadowLayoutCaches != null)

            {
               // Java: this.mShadowLayoutCaches.add(null);

            }
            // Java: if (!hasInnerShadow && this.mInnerShadowCaches != null)

            {
               // Java: this.mInnerShadowCaches.add(null);

            }
            // Java: if (hasBlur || hasOutShadow || hasInnerShadow)

            {
               // Java: Spannable innerSpannable = hasInnerShadow ? new SpannableStringBuilder(layout.getText()) : null;

               // Java: Spannable blurSpannable = hasBlur ? new SpannableStringBuilder(layout.getText()) : null;

               // Java: Spannable shadowSpan = null;

               // Java: Spannable outlineShadowSpan = null;

               // Java: Spannable blurShadowSpan = null;

               // Java: if (hasOutShadow)

               {
                  // Java: CharSequence oriChar = data.mTypedCharSequence.subSequence(0, data.mTypedCharSequence.length());

                  // Java: CharSequence renderChar = TypoLayoutGen.processRightToLeftText(oriChar);

                  // Java: shadowSpan = new SpannableStringBuilder(renderChar);

                  // Java: outlineShadowSpan = new SpannableStringBuilder(renderChar);

                  // Java: blurShadowSpan = new SpannableStringBuilder(renderChar);

               }
               // Java: int flag = 34;

               // Java: int spanInx = 0;

               // Java: int spanSize = data.mOtherSpans.size();

               // Java: int textLen = layout.getText().length();

               // Java: SpanInfo span = data.mFirstSpan;

               // Java: boolean hasGenBitmap = false;

               // Java: for(Bitmap innerBitmap = null; span != null && span.mEndPos <= textLen; ++spanInx)

               {
                  // Java: int start = span.mStartPos;

                  // Java: int end = span.mEndPos;

                  // Java: if (hasBlur && blurSpannable != null)

                  {
                     // Java: CharacterStyle charStyle;

                     // Java: if (span.hasBlur() && span.mTextBlur.radius() > 0.0F)

                     {
                        // Java: charStyle = new WpsTextBlurSpan(span.mTextBlur.radius(), span.mTextBlur.blurColor());

                     }
                     // Java: else

                     {
                        // Java: charStyle = new ForegroundColorSpan(0);

                     }
                     // Java: blurSpannable.setSpan(charStyle, start, end, flag);

                  }
                  // Java: if (hasInnerShadow && innerSpannable != null)

                  {
                     // Java: if (!hasGenBitmap)

                     {
                        // Java: hasGenBitmap = true;

                        // Java: Layout[] complexLayout = this.genInnerComplexLayout(data, layout, i);

                        // Java: innerBitmap = this.getInnerTextBitmap(complexLayout[0], complexLayout[1], data);

                        // Java: if (innerBitmap == null)

                        {
                           // Java: layout = (Layout)this.mTypoFillCaches.get(i);

                        }
                     }
                     // Java: float outlineWid = span.hasOutline() ? span.mTextOutline.getLineWid() : 0.0F;

                     // Java: int outlineClr = span.hasOutline() ? span.mTextOutline.getLineClr() : -16777216;

                     // Java: CharacterStyle charStyle;

                     // Java: if (span.hasInnerShdw() && innerBitmap != null)

                     {
                        // Java: float alpha = this.getFillAlpha(span);

                        // Java: charStyle = new WpsInnerShdwSpan(innerBitmap, this.mRecursionDelta, outlineWid, alpha);

                     }
                     // Java: else if (span.hasInnerShdw() && innerBitmap == null)

                     {
                        // Java: charStyle = new WpsInnerShdwSpan(outlineWid, outlineClr, span.hasFill());

                     }
                     // Java: else

                     {
                        // Java: charStyle = new ForegroundColorSpan(0);

                     }
                     // Java: innerSpannable.setSpan(charStyle, start, end, flag);

                  }
                  // Java: if (shadowSpan != null && span.hasOuterShdw())

                  {
                     // Java: this.typoOutShadowBySpan(span, shadowSpan, outlineShadowSpan, blurShadowSpan, i, spanInx);

                  }
                  // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

               }
               // Java: this.createLayout(layout, innerSpannable, data, this.mInnerShadowCaches, false);

               // Java: this.createLayout(layout, blurSpannable, data, this.mTypoBlurCaches, false);

               // Java: if (this.mShadowLayoutCaches != null)

               {
                  // Java: layoutData.mFillShadowLayout = this.createLayout(layout, shadowSpan, data, (List)null, true);

                  // Java: layoutData.mBlurShadowLayout = this.createLayout(layout, blurShadowSpan, data, (List)null, true);

                  // Java: layoutData.mOutlineShadowLayout = this.createLayout(layout, outlineShadowSpan, data, (List)null, true);

                  // Java: this.mShadowLayoutCaches.add(layoutData);

               }
            }
         }
      }
    }

    // Java: Layout createLayout(Layout layout, Spannable spannable, TextData data, List<Layout> layoutCaches, boolean canCopy)
    pub fn createLayout(&self, _layout: i32, _spannable: i32, _data: i32, _layoutCaches: i32, _canCopy: bool) -> i32 {
      // Java: if (layout != null && spannable != null && data != null)

      {
         // Java: Layout createLayout;

         // Java: if (canCopy)

         {
            // Java: if (layout instanceof WpsLayout)

            {
               // Java: createLayout = ((WpsLayout)layout).copy(spannable);

            }
            // Java: else

            {
               // Java: createLayout = this.mTypoLayoutGen.geneLayout(spannable, data, layout.getOriWidth(), layout.getAlignment(), layout.getPaint(), layout.getSpacingMultiplier());

            }
         }
         // Java: else

         {
            // Java: createLayout = this.mTypoLayoutGen.geneLayout(spannable, data, layout.getOriWidth(), layout.getAlignment(), layout.getPaint(), layout.getSpacingMultiplier());

            // Java: if (layoutCaches != null)

            {
               // Java: layoutCaches.add(createLayout);

            }
         }
         // Java: return createLayout;

      }
      // Java: else

      {
         // Java: return null;

      }
      0
    }

    // Java: void typoOutShadowBySpan(SpanInfo span, Spannable shdwSpan, Spannable outlnShdwSpan, Spannable blurShdwSpan, int index, int spanInx)
    pub fn typoOutShadowBySpan(&self, _span: i32, _shdwSpan: i32, _outlnShdwSpan: i32, _blurShdwSpan: i32, _index: i32, _spanInx: i32) {
      // Java: ShadowTypoData shadowData = new ShadowTypoData();

      // Java: CharacterStyle charStyle;

      // Java: CharacterStyle blurCharStyle;

      // Java: CharacterStyle outlineCharStyle;

      // Java: if (span.hasOuterShdw())

      {
         // Java: TextShadow shadow = span.mTextShadow;

         // Java: int shadowAlpha = Color.alpha(shadow.getShdwClr());

         // Java: int fillClr = this.getAlphaColor(shadow.getShdwClr(), (int)((float)shadowAlpha * this.getFillAlpha(span)));

         // Java: float shadowRadScale = this.getOuterShadowRadScale(span.mFontSize);

         // Java: boolean isYCompress = shadow.getShdwYPct() < 1.0F;

         // Java: float renderRad = shadow.getShdwRad() * shadowRadScale;

         // Java: charStyle = new WpsOuterShdwSpan(fillClr, renderRad, 0.0F, span.hasFill(), isYCompress);

         // Java: float skewX = (float)Math.tan(Math.toRadians((double)shadow.getShdwXSkew()));

         // Java: float skewY = (float)Math.tan(Math.toRadians((double)shadow.getShdwYSkew()));

         // Java: shadowData.mShadowScale = new PointF(shadow.getShdwXPct(), shadow.getShdwYPct());

         // Java: shadowData.mShadowSkew = new PointF(skewX, skewY);

         // Java: shadowData.mShadowAngle = shadow.getShdwAngle();

         // Java: shadowData.mShadowDistance = shadow.getShdwDis();

         // Java: shadowData.mShadowAlign = shadow.getShdwAlgn();

         // Java: if (span.hasOutline())

         {
            // Java: int outlineClr = this.getAlphaColor(shadow.getShdwClr(), (int)((float)shadowAlpha * this.getOutlineAlpha(span)));

            // Java: outlineCharStyle = this.createOutline(renderRad, outlineClr, span.mTextOutline, isYCompress);

         }
         // Java: else

         {
            // Java: outlineCharStyle = new ForegroundColorSpan(0);

         }
         // Java: if (span.hasBlur())

         {
            // Java: int color = this.getAlphaColor(shadow.getShdwClr(), (int)((float)shadowAlpha * span.mTextBlur.colorAlpha()));

            // Java: blurCharStyle = new WpsTextBlurSpan(span.mTextBlur.radius(), color);

         }
         // Java: else

         {
            // Java: blurCharStyle = new ForegroundColorSpan(0);

         }
      }
      // Java: else

      {
         // Java: charStyle = new ForegroundColorSpan(0);

         // Java: outlineCharStyle = new ForegroundColorSpan(0);

         // Java: blurCharStyle = new ForegroundColorSpan(0);

         // Java: shadowData.setDefault();

      }
      // Java: Map<Integer, ShadowTypoData> typoDataMap = (Map)this.mSpanShadowCaches.get(index);

      // Java: if (typoDataMap == null)

      {
         // Java: typoDataMap = new HashMap();

         // Java: this.mSpanShadowCaches.put(index, typoDataMap);

      }
      // Java: typoDataMap.put(spanInx, shadowData);

      // Java: shdwSpan.setSpan(charStyle, span.mStartPos, span.mEndPos, 34);

      // Java: outlnShdwSpan.setSpan(outlineCharStyle, span.mStartPos, span.mEndPos, 34);

      // Java: blurShdwSpan.setSpan(blurCharStyle, span.mStartPos, span.mEndPos, 34);

    }

    // Java: float getMinTypoOffsetX()
    pub fn getMinTypoOffsetX(&self) -> f32 {
      // Java: int size = this.mTypoOffsets.size();

      // Java: if (size < 1)

      {
         // Java: return 0.0F;

      }
      // Java: else

      {
         // Java: float min = ((PointF)this.mTypoOffsets.get(0)).x;

         // Java: for(int i = 1; i < size; ++i)

         {
            // Java: if (((PointF)this.mTypoOffsets.get(i)).x < min)

            {
               // Java: min = ((PointF)this.mTypoOffsets.get(i)).x;

            }
         }
         // Java: return min;

      }
      0.0
    }

    // Java: float getMinTypoOffsetY()
    pub fn getMinTypoOffsetY(&self) -> f32 {
      // Java: int size = this.mTypoOffsets.size();

      // Java: if (size < 1)

      {
         // Java: return 0.0F;

      }
      // Java: else

      {
         // Java: float min = ((PointF)this.mTypoOffsets.get(0)).y;

         // Java: for(int i = 1; i < size; ++i)

         {
            // Java: if (((PointF)this.mTypoOffsets.get(i)).y < min)

            {
               // Java: min = ((PointF)this.mTypoOffsets.get(i)).y;

            }
         }
         // Java: return min;

      }
      0.0
    }

    // Java: float getMinLeft(List<Layout> typoCaches)
    pub fn getMinLeft(&self, _typoCaches: i32) -> f32 {
      // Java: float minLnLeft = ((Layout)typoCaches.get(0)).getLineLeft(0);

      // Java: for(int i = 0; i < typoCaches.size(); ++i)

      {
         // Java: Layout layout = (Layout)typoCaches.get(i);

         // Java: for(int ln = 0; ln < layout.getLineCount(); ++ln)

         {
            // Java: float lnLeft = layout.getLineLeft(ln);

            // Java: if (lnLeft < minLnLeft)

            {
               // Java: minLnLeft = lnLeft;

            }
         }
      }
      // Java: return minLnLeft;

      0.0
    }

    // Java: float getFillAlpha(SpanInfo span)
    pub fn getFillAlpha(&self, _span: i32) -> f32 {
      // Java: if (span.mFontFill == null)

      {
         // Java: return span.mColorAlpha;

      }
      // Java: else

      {
         // Java: float alpha = 0.0F;

         // Java: switch (span.mFontFill.mFillType)

         {
            // Java: case COLORFILL:

               // Java: alpha = span.mColorAlpha;

            // Java: case NONE:

            // Java: default:

               // Java: break;

            // Java: case BLIPFILL:

               // Java: alpha = span.mFontFill.mBlipFill.getAlpha();

               // Java: break;

            // Java: case GRADFILL:

               // Java: float[] alphas = span.mFontFill.mGradFill.getGradAttr().colorAlpha();

               // Java: if (alphas.length > 1)

               {
                  // Java: for(int i = 1; i < alphas.length; ++i)

                  {
                     // Java: alpha = alphas[0];

                     // Java: if (alphas[i] != alphas[i - 1])

                     {
                        // Java: alpha = 1.0F;

                        // Java: return alpha;

                     }
                  }
               }
               // Java: else

               {
                  // Java: alpha = 1.0F;

               }
               // Java: break;

            // Java: case PATTERNFILL:

               // Java: alpha = 1.0F;

         }
         // Java: return alpha;

      }
      0.0
    }

    // Java: float getOutlineAlpha(SpanInfo span)
    pub fn getOutlineAlpha(&self, _span: i32) -> f32 {
      // Java: if (span.mTextOutline == null)

      {
         // Java: return 0.0F;

      }
      // Java: else

      {
         // Java: float alpha = 0.0F;

         // Java: switch (span.mTextOutline.getTxtOutlnType())

         {
            // Java: case NONE:

            // Java: default:

               // Java: break;

            // Java: case SOLID_LINE:

               // Java: alpha = span.mTextOutline.getLineClrAlpha();

               // Java: break;

            // Java: case GRAD_LINE:

               // Java: alpha = 1.0F;

         }
         // Java: return alpha;

      }
      0.0
    }

    // Java: Layout[] genInnerComplexLayout(TextData data, Layout layout, int index)
    pub fn genInnerComplexLayout(&self, _data: i32, _layout: i32, _index: i32) -> Vec<i32> {
      // Java: Spannable shadowSpannable = new SpannableStringBuilder(layout.getText());

      // Java: Spannable lightSpannable = new SpannableStringBuilder(layout.getText());

      // Java: int flag = 34;

      // Java: int spanInx = 0;

      // Java: int spanSize = data.mOtherSpans.size();

      // Java: int textLen = layout.getText().length();

      // Java: for(SpanInfo span = data.mFirstSpan; span != null && span.mEndPos <= textLen; ++spanInx)

      {
         // Java: int start = span.mStartPos;

         // Java: int end = span.mEndPos;

         // Java: float outlineWid = span.hasOutline() ? span.mTextOutline.getLineWid() : 0.0F;

         // Java: CharacterStyle outerShadowStyle;

         // Java: if (span.hasOutline() || span.mTextShadow == null || !span.hasInnerShdw() && !span.hasOuterShdw())

         {
            // Java: outerShadowStyle = new ForegroundColorSpan(span.mTextShadow != null ? span.mTextShadow.getShdwClr() : 0);

         }
         // Java: else

         {
            // Java: outerShadowStyle = new WpsOuterShdwSpan(span.mTextShadow.getShdwClr(), 0.0F, outlineWid, span.hasFill(), false);

         }
         // Java: shadowSpannable.setSpan(outerShadowStyle, start, end, flag);

         // Java: CharacterStyle lightStyle;

         // Java: if (span.hasInnerShdw() && span.hasFill())

         {
            // Java: lightStyle = new WpsShdwLightSpan(outlineWid, span.mTextShadow.getShdwRad(), true, span.mFontSize);

            // Java: if (this.mTypoFillCaches.size() > index && span.mFontFill.mFillType != FontFill.FontFillType.NONE && span.mFontFill.mFillType != FontFill.FontFillType.COLORFILL)

            {
               // Java: WpsForegroundColorSpan[] spans = (WpsForegroundColorSpan[])((Spannable)((Layout)this.mTypoFillCaches.get(index)).getText()).getSpans(start, end, WpsForegroundColorSpan.class);

               // Java: if (spans != null && spans.length > 0)

               {
                  // Java: lightSpannable.setSpan(spans[spans.length - 1], start, end, flag);

               }
            }
         }
         // Java: else

         {
            // Java: lightStyle = new ForegroundColorSpan(0);

         }
         // Java: lightSpannable.setSpan(lightStyle, start, end, flag);

         // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

      }
      // Java: Layout shadowLayout = this.createLayout(layout, shadowSpannable, data, (List)null, false);

      // Java: Layout lightLayout = this.createLayout(layout, lightSpannable, data, (List)null, false);

      // Java: Layout[] complex = new Layout[]{shadowLayout, lightLayout};

      // Java: this.mInnerLayoutMap.put(index, complex);

      // Java: return complex;

      Vec::new()
    }

    // Java: Bitmap createSuitBitmap(float width, float height)
    pub fn createSuitBitmap(&self, _width: f32, _height: f32) -> i32 {
      // Java: float scale = (float)(4 - this.mRecursionDelta);

      // Java: int bmpWid = (int)Math.ceil((double)(width * scale));

      // Java: int bmpHei = (int)Math.ceil((double)(height * scale));

      // Java: if (bmpWid > 0 && bmpHei > 0)

      {
         // Java: Bitmap bitmap = null;

         // Java: try

         {
            // Java: bitmap = Bitmap.createBitmap(bmpWid, bmpHei, Config.ARGB_8888);

         }
         // Java: catch (OutOfMemoryError var8)

         {
            // Java: if (this.mRecursionDelta + 1 != 4)

            {
               // Java: ++this.mRecursionDelta;

               // Java: bitmap = this.createSuitBitmap(width, height);

            }
         }
         // Java: catch (Exception var9)

         {
            // Java: return null;

         }
         // Java: return bitmap;

      }
      // Java: else

      {
         // Java: return null;

      }
      0
    }

    // Java: Bitmap getInnerTextBitmap(Layout shadowLayout, Layout lightLayout, TextData data)
    pub fn getInnerTextBitmap(&self, _shadowLayout: i32, _lightLayout: i32, _data: i32) -> i32 {
      // Java: if (shadowLayout != null && lightLayout != null)

      {
         // Java: float width = 0.0F;

         // Java: for(int line = 0; line < shadowLayout.getLineCount(); ++line)

         {
            // Java: float lineRight = shadowLayout.getLineRight(line);

            // Java: if (lineRight > width)

            {
               // Java: width = lineRight;

            }
         }
         // Java: float height = shadowLayout.getHeight();

         // Java: Bitmap bitmap = this.createSuitBitmap(width, height);

         // Java: if (bitmap == null)

         {
            // Java: return null;

         }
         // Java: else

         {
            // Java: float scale = (float)(4 - this.mRecursionDelta);

            // Java: Canvas canvas = new Canvas(bitmap);

            // Java: canvas.scale(scale, scale);

            // Java: shadowLayout.getPaint().setAntiAlias(true);

            // Java: shadowLayout.draw(canvas);

            // Java: Xfermode fillLgOrg = lightLayout.getPaint().getXfermode();

            // Java: lightLayout.getPaint().setXfermode(new PorterDuffXfermode(Mode.SRC_ATOP));

            // Java: lightLayout.getPaint().setAntiAlias(true);

            // Java: Spannable spannable = (Spannable)lightLayout.getText();

            // Java: int flag = 34;

            // Java: int textLen = spannable.length();

            // Java: int spanInx = 0;

            // Java: int spanSize = data.mOtherSpans.size();

            // Java: SpanInfo span = data.mFirstSpan;

            // Java: CharacterStyle startMaskSpan = WpsHideSpan.Factory.create();

            // Java: CharacterStyle endMaskSpan = WpsHideSpan.Factory.create();

            // Java: while(span != null && span.mEndPos <= textLen)

            {
               // Java: int start = span.mStartPos;

               // Java: int end = span.mEndPos;

               // Java: PointF offset = new PointF();

               // Java: if (span.hasInnerShdw())

               {
                  // Java: if (span.mTextShadow.getIsDoc())

                  {
                     // Java: scale = span.mFontSize / 100.0F;

                     // Java: offset.set(-span.mTextShadow.getShdwOffsetX() * scale, -span.mTextShadow.getShdwOffsetY() * scale);

                  }
                  // Java: else

                  {
                     // Java: float shadowDis = span.mTextShadow.getShdwDis();

                     // Java: offset = this.getInnerShadowOfafset(span.mTextShadow.getShdwAngle(), shadowDis, span.mFontSize);

                  }
                  // Java: if (start > 0)

                  {
                     // Java: spannable.setSpan(startMaskSpan, 0, start, flag);

                  }
                  // Java: if (end < textLen)

                  {
                     // Java: spannable.setSpan(endMaskSpan, end, textLen, flag);

                  }
                  // Java: canvas.translate(offset.x, offset.y);

                  // Java: lightLayout.draw(canvas);

                  // Java: canvas.translate(-offset.x, -offset.y);

                  // Java: if (start > 0)

                  {
                     // Java: spannable.removeSpan(startMaskSpan);

                  }
                  // Java: if (end < textLen)

                  {
                     // Java: spannable.removeSpan(endMaskSpan);

                  }
                  // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

                  // Java: ++spanInx;

               }
               // Java: else

               {
                  // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

                  // Java: ++spanInx;

               }
            }
            // Java: lightLayout.getPaint().setXfermode(fillLgOrg);

            // Java: return bitmap;

         }
      }
      // Java: else

      {
         // Java: return null;

      }
      0
    }

    // Java: PointF getInnerShadowOfafset(float angle, float distance, float fontSize)
    pub fn getInnerShadowOfafset(&self, _angle: f32, _distance: f32, _fontSize: f32) -> i32 {
      // Java: angle = (angle + 180.0F) % 360.0F;

      // Java: float x = (float)((double)distance * Math.cos(Math.toRadians((double)angle))) * fontSize / 100.0F;

      // Java: float y = (float)((double)distance * Math.sin(Math.toRadians((double)angle))) * fontSize / 100.0F;

      // Java: return new PointF(x, y);

      0
    }

    // Java: void render(Canvas workCanvas, List<Layout> typoCaches, int startPara, int startOffset, int endPara, int endOffset, List<PointF> mTypoOffsets, List<DecoLineCache> decoLineCaches, RichTextLayout richTextLayout)
    pub fn render(&self, _workCanvas: i32, _typoCaches: i32, _startPara: i32, _startOffset: i32, _endPara: i32, _endOffset: i32, _mTypoOffsets: i32, _decoLineCaches: i32, _richTextLayout: i32) {
      // Java: this.render(workCanvas, typoCaches, startPara, startOffset, endPara, endOffset, mTypoOffsets, decoLineCaches, richTextLayout, false);

    }

    // Java: void beforeRender(TimingInfo timingInfo, List<TextData> dataList)
    pub fn beforeRender(&self, _timingInfo: i32, _dataList: i32) {
      // Java: this.hasDynamicDigit = false;

      // Java: if (timingInfo != null && this.mShadowLayoutCaches != null)

      {
         // Java: int startPara = timingInfo.getStartPara();

         // Java: int endPara = Math.min(timingInfo.getEndPara(), this.mShadowLayoutCaches.size());

         // Java: for(int i = startPara; i <= endPara; ++i)

         {
            // Java: ShadowLayoutGroup layoutData = (ShadowLayoutGroup)this.mShadowLayoutCaches.get(i);

            // Java: if (layoutData != null)

            {
               // Java: this.beforeRender(layoutData.mFillShadowLayout, timingInfo, (TextData)dataList.get(i), i);

               // Java: this.beforeRender(layoutData.mOutlineShadowLayout, timingInfo, (TextData)dataList.get(i), i);

               // Java: this.beforeRender(layoutData.mBlurShadowLayout, timingInfo, (TextData)dataList.get(i), i);

            }
         }
      }
    }

    // Java: void afterRender(TimingInfo timingInfo, List<TextData> datas)
    pub fn afterRender(&self, _timingInfo: i32, _datas: i32) {
      // Java: if (timingInfo != null)

      {
         // Java: int startPara = timingInfo.getStartPara();

         // Java: int endPara = this.mShadowLayoutCaches.size();

         // Java: for(int i = startPara; i < endPara; ++i)

         {
            // Java: ShadowLayoutGroup layoutData = (ShadowLayoutGroup)this.mShadowLayoutCaches.get(i);

            // Java: if (layoutData != null)

            {
               // Java: this.restoreTextData(layoutData.mFillShadowLayout, timingInfo, (TextData)datas.get(i));

               // Java: this.restoreTextData(layoutData.mOutlineShadowLayout, timingInfo, (TextData)datas.get(i));

               // Java: this.restoreTextData(layoutData.mBlurShadowLayout, timingInfo, (TextData)datas.get(i));

            }
         }
      }
    }

    // Java: void beforeRender(Layout layout, TimingInfo timingInfo, TextData data, int para)
    pub fn beforeRender_2(&self, _layout: i32, _timingInfo: i32, _data: i32, _para: i32) {
      // Java: if (layout != null && data != null)

      {
         // Java: byte alterType = timingInfo.getAlterType();

         // Java: if ((alterType & 64) != 0 && !"".equals(layout.getText().toString().trim()))

         {
            // Java: Spannable span = (Spannable)layout.getText();

            // Java: double fraction = timingInfo.getDynamicDigitFraction();

            // Java: double dest = timingInfo.getDynamicDigitDest();

            // Java: int mode = timingInfo.getDynamicDigitAnimMode();

            // Java: double src = timingInfo.getDynamicDigitSrc();

            // Java: boolean srcAbs = timingInfo.isDynamicDigitIsSrcAbs();

            // Java: boolean destAbs = timingInfo.isDynamicDigitIsDestAbs();

            // Java: boolean optimizeRate = timingInfo.isDynamicDigitOptimizeRate();

            // Java: int start = para == timingInfo.getStartPara() ? timingInfo.getStartOffset() : 0;

            // Java: int end = para == timingInfo.getEndPara() ? timingInfo.getEndOffset() : layout.getText().length();

            // Java: ArrayList<Range> digitRange = new ArrayList();

            // Java: String destText;

            // Java: try

            {
               // Java: destText = TextUtil.updateDynamicTextContent(data.mTypoText, fraction, dest, src, destAbs, srcAbs, mode, digitRange, optimizeRate);

            }
            // Java: catch (StringIndexOutOfBoundsException var28)

            {
               // Java: destText = data.mTypoText;

            }
            // Java: if (destText.length() <= data.mTypoText.length())

            {
               // Java: this.hasDynamicDigit = true;

               // Java: if (span instanceof SpannableStringBuilder)

               {
                  // Java: SpannableStringBuilder s = (SpannableStringBuilder)span;

                  // Java: for(Range rng : digitRange)

                  {
                     // Java: int fixStart = Math.max(start, rng.start());

                     // Java: int fixEnd = Math.min(end, rng.end());

                     // Java: if (rng.type() == TextRangeType.Text_Digit && fixStart < fixEnd)

                     {
                        // Java: String replaceStr = destText.substring(fixStart, fixEnd);

                        // Java: for(int j = 0; j < replaceStr.length(); ++j)

                        {
                           // Java: s.replaceChar(replaceStr.charAt(j), fixStart + j);

                        }
                     }
                  }
               }
            }
         }
      }
    }

    // Java: void restoreTextData(Layout layout, TimingInfo timingInfo, TextData data)
    pub fn restoreTextData(&self, _layout: i32, _timingInfo: i32, _data: i32) {
      // Java: if (layout != null && data != null && timingInfo != null)

      {
         // Java: byte alterType = timingInfo.getAlterType();

         // Java: if ((alterType & 64) != 0 && !"".equals(layout.getText().toString().trim()))

         {
            // Java: Spannable span = (Spannable)layout.getText();

            // Java: String destText = data.mTypoText;

            // Java: ArrayList<Range> digitRange = new ArrayList();

            // Java: TextUtil.splitTextDigit(destText, digitRange);

            // Java: if (span instanceof SpannableStringBuilder)

            {
               // Java: SpannableStringBuilder s = (SpannableStringBuilder)span;

               // Java: for(Range rng : digitRange)

               {
                  // Java: if (rng.type() == TextRangeType.Text_Digit)

                  {
                     // Java: String replaceStr = destText.substring(rng.start(), rng.end());

                     // Java: s.replace(rng.start(), rng.end(), replaceStr);

                  }
               }
            }
         }
      }
    }

    // Java: boolean isAllVert()
    pub fn isAllVert(&self) -> bool {
      // Java: return this.mTextBoxData.verticalText() == 3 || this.mTextBoxData.verticalText() == 6;

      false
    }

    // Java: void render(Canvas workCanvas, List<Layout> typoCaches, int startPara, int startOffset, int endPara, int endOffset, List<PointF> mTypoOffsets, List<DecoLineCache> decoLineCaches, RichTextLayout richTextLayout, boolean isDragShadow)
    pub fn render_2(&self, _workCanvas: i32, _typoCaches: i32, _startPara: i32, _startOffset: i32, _endPara: i32, _endOffset: i32, _mTypoOffsets: i32, _decoLineCaches: i32, _richTextLayout: i32, _isDragShadow: bool) {
      // Java: Spannable lastText = (Spannable)((Layout)typoCaches.get(endPara - 1)).getText();

      // Java: CharacterStyle startMaskSpan = WpsHideSpan.Factory.create();

      // Java: CharacterStyle endMaskSpan = WpsHideSpan.Factory.create();

      // Java: if (startOffset > 0)

      {
         // Java: this.setStartMaskSpan(startPara, startOffset, startMaskSpan);

      }
      // Java: if (endOffset < lastText.length())

      {
         // Java: this.setEndMaskSpan(endPara, endOffset, endMaskSpan);

      }
      // Java: int shdwSpanIdx = 0;

      // Java: workCanvas.save();

      // Java: for(int i = startPara; i < endPara && i < typoCaches.size(); ++i)

      {
         // Java: float translateX = this.isAllVert() ? ((PointF)mTypoOffsets.get(i)).y : ((PointF)mTypoOffsets.get(i)).x;

         // Java: float translateY = this.isAllVert() ? ((PointF)mTypoOffsets.get(i)).x : ((PointF)mTypoOffsets.get(i)).y;

         // Java: workCanvas.translate(translateX, translateY);

         // Java: if (!isDragShadow && this.mTextBoxData.getTextData(i).hasTextImage())

         {
            // Java: this.renderInvertImage(workCanvas, (Layout)typoCaches.get(i), i);

         }
         // Java: if (!isDragShadow && this.mShadowLayoutCaches != null && this.mShadowLayoutCaches.get(i) != null && !((ShadowLayoutGroup)this.mShadowLayoutCaches.get(i)).isEmpty())

         {
            // Java: shdwSpanIdx = this.renderOuterShadow(workCanvas, i, typoCaches, shdwSpanIdx);

         }
         // Java: if (!isDragShadow && this.mTypoBlurCaches != null && this.mTypoBlurCaches.get(i) != null)

         {
            // Java: ((Layout)this.mTypoBlurCaches.get(i)).draw(workCanvas);

            // Java: this.renderChineseVertText((Layout)this.mTypoBlurCaches.get(i), i, workCanvas, Integer.MIN_VALUE, Integer.MAX_VALUE, 3);

         }
         // Java: if (this.mTypoOutlineCaches != null && this.mTypoOutlineCaches.get(i) != null)

         {
            // Java: ((Layout)this.mTypoOutlineCaches.get(i)).draw(workCanvas);

            // Java: this.renderChineseVertText((Layout)this.mTypoOutlineCaches.get(i), i, workCanvas, Integer.MIN_VALUE, Integer.MAX_VALUE, 2);

         }
         // Java: ((Layout)this.mTypoFillCaches.get(i)).draw(workCanvas);

         // Java: this.renderChineseVertText((Layout)this.mTypoFillCaches.get(i), i, workCanvas, Integer.MIN_VALUE, Integer.MAX_VALUE, 1);

         // Java: if (!isDragShadow && this.mInnerShadowCaches != null && this.mInnerShadowCaches.get(i) != null)

         {
            // Java: ((Layout)this.mInnerShadowCaches.get(i)).draw(workCanvas);

            // Java: this.renderChineseVertText((Layout)this.mInnerShadowCaches.get(i), i, workCanvas, Integer.MIN_VALUE, Integer.MAX_VALUE, 5);

         }
         // Java: if (decoLineCaches != null && i < decoLineCaches.size() && decoLineCaches.get(i) != null)

         {
            // Java: List<RectF> selRects = richTextLayout.getSelRects(startPara, endPara, startOffset, endOffset, i);

            // Java: ((DecoLineCache)decoLineCaches.get(i)).draw(workCanvas, selRects, this.mTextBoxData.isAllVerticalText());

         }
         // Java: workCanvas.translate(-translateX, -translateY);

      }
      // Java: workCanvas.restore();

      // Java: if (startOffset > 0)

      {
         // Java: this.removeStartMaskSpan(startPara, startMaskSpan);

      }
      // Java: if (endOffset < lastText.length())

      {
         // Java: this.removeEndMaskSpan(endPara, endMaskSpan);

      }
    }

    // Java: void renderChineseVertText(Layout layout, int layoutIndex, Canvas canvas, int start, int end, int type)
    pub fn renderChineseVertText(&self, _layout: i32, _layoutIndex: i32, _canvas: i32, _start: i32, _end: i32, _type: i32) {
      // Java: TextData data = this.mTextBoxData.getTextData(layoutIndex);

      // Java: if (data != null && data.mVertical == 4)

      {
         // Java: TextPaint paint = new TextPaint();

         // Java: Spannable text = (Spannable)layout.getText();

         // Java: int checkStart = Math.max(start, 0);

         // Java: int checkEnd = Math.min(end, text.length());

         // Java: ChineseVertSpan[] spans = (ChineseVertSpan[])text.getSpans(checkStart, checkEnd, ChineseVertSpan.class);

         // Java: int spanIndex = 0;

         // Java: SpanInfo spanInfo = data.mFirstSpan;

         // Java: for(ChineseVertSpan vertSpan : spans)

         {
            // Java: int spanStart = Math.max(start, vertSpan.getStart());

            // Java: int spanEnd = Math.min(end, vertSpan.getEnd());

            // Java: boolean needFixPos;

            // Java: for(needFixPos = false; spanInfo != null && (vertSpan.getStart() != spanInfo.mStartPos || vertSpan.getEnd() != spanInfo.mEndPos); ++spanIndex)

            {
               // Java: if (vertSpan.getStart() >= spanInfo.mStartPos && vertSpan.getEnd() <= spanInfo.mEndPos)

               {
                  // Java: needFixPos = true;

                  // Java: break;

               }
               // Java: spanInfo = spanIndex >= data.mOtherSpans.size() ? null : (SpanInfo)data.mOtherSpans.get(spanIndex);

            }
            // Java: if (spanInfo != null)

            {
               // Java: SpanInfo curSpanInfo = new SpanInfo(spanInfo);

               // Java: if (needFixPos)

               {
                  // Java: curSpanInfo.mStartPos = vertSpan.getStart();

                  // Java: curSpanInfo.mEndPos = vertSpan.getEnd();

                  // Java: checkStart = Math.max(checkStart, curSpanInfo.mStartPos);

                  // Java: checkEnd = Math.min(checkEnd, curSpanInfo.mEndPos);

               }
               // Java: this.initVertTextPaint(paint, vertSpan);

               // Java: boolean enableCache = start == Integer.MIN_VALUE && end == Integer.MAX_VALUE;

               // Java: if (type == 1)

               {
                  // Java: ForegroundColorSpan[] colorSpans = (ForegroundColorSpan[])text.getSpans(spanStart, spanEnd, WpsForegroundColorSpan.class);

                  // Java: if (colorSpans.length <= 0)

                  {
                     // Java: colorSpans = (ForegroundColorSpan[])text.getSpans(spanStart, spanEnd, ForegroundColorSpan.class);

                  }
                  // Java: if (colorSpans.length > 0)

                  {
                     // Java: colorSpans[colorSpans.length - 1].updateDrawState(paint);

                  }
                  // Java: if (enableCache)

                  {
                     // Java: this.drawChineseVertSpan(layout, layoutIndex, spanIndex, curSpanInfo, canvas, paint, (WpsImageSpan)null);

                  }
                  // Java: else

                  {
                     // Java: this.drawChineseVertSection(layout, layoutIndex, curSpanInfo, canvas, paint, checkStart, checkEnd, (WpsImageSpan)null);

                  }
               }
               // Java: if (type == 2)

               {
                  // Java: WpsTextOutlineSpan[] outlineSpans = (WpsTextOutlineSpan[])text.getSpans(spanStart, spanEnd, WpsTextOutlineSpan.class);

                  // Java: if (outlineSpans.length > 0)

                  {
                     // Java: outlineSpans[outlineSpans.length - 1].updateDrawState(paint);

                     // Java: if (enableCache)

                     {
                        // Java: this.drawChineseVertSpan(layout, layoutIndex, spanIndex, curSpanInfo, canvas, paint, (WpsImageSpan)null);

                     }
                     // Java: else

                     {
                        // Java: this.drawChineseVertSection(layout, layoutIndex, curSpanInfo, canvas, paint, checkStart, checkEnd, (WpsImageSpan)null);

                     }
                  }
               }
               // Java: if (type == 3)

               {
                  // Java: WpsTextBlurSpan[] blurSpans = (WpsTextBlurSpan[])text.getSpans(spanStart, spanEnd, WpsTextBlurSpan.class);

                  // Java: if (blurSpans.length > 0)

                  {
                     // Java: blurSpans[blurSpans.length - 1].updateDrawState(paint);

                     // Java: if (enableCache)

                     {
                        // Java: this.drawChineseVertSpan(layout, layoutIndex, spanIndex, curSpanInfo, canvas, paint, (WpsImageSpan)null);

                     }
                     // Java: else

                     {
                        // Java: this.drawChineseVertSection(layout, layoutIndex, curSpanInfo, canvas, paint, checkStart, checkEnd, (WpsImageSpan)null);

                     }
                  }
               }
               // Java: if (type == 4)

               {
                  // Java: WpsOuterShdwSpan[] outerShdwSpans = (WpsOuterShdwSpan[])text.getSpans(spanStart, spanEnd, WpsOuterShdwSpan.class);

                  // Java: if (outerShdwSpans.length > 0)

                  {
                     // Java: outerShdwSpans[outerShdwSpans.length - 1].updateDrawState(paint);

                     // Java: if (enableCache)

                     {
                        // Java: this.drawChineseVertSpan(layout, layoutIndex, spanIndex, curSpanInfo, canvas, paint, (WpsImageSpan)null);

                     }
                     // Java: else

                     {
                        // Java: this.drawChineseVertSection(layout, layoutIndex, curSpanInfo, canvas, paint, checkStart, checkEnd, (WpsImageSpan)null);

                     }
                  }
               }
               // Java: if (type == 5)

               {
                  // Java: WpsInnerShdwSpan[] innerSpans = (WpsInnerShdwSpan[])text.getSpans(spanStart, spanEnd, WpsInnerShdwSpan.class);

                  // Java: if (innerSpans.length > 0)

                  {
                     // Java: this.drawChineseVertInnerSpan(layout, layoutIndex, curSpanInfo, canvas, paint, vertSpan);

                  }
               }
            }
         }
      }
    }

    // Java: void updateEmbossPaint(TextPaint paint, SpanInfo spanInfo, PointF offset)
    pub fn updateEmbossPaint(&self, _paint: i32, _spanInfo: i32, _offset: i32) {
      // Java: float scale = Math.max(this.mCanvasScaleX, this.mCanvasScaleY);

      // Java: int scaleFactor = (int)(2.0F * scale);

      // Java: int directionZ = 8;

      // Java: offset.x = this.getNumberSign(offset.x) * (float)scaleFactor;

      // Java: offset.y = this.getNumberSign(offset.y) * (float)scaleFactor;

      // Java: int directionX;

      // Java: int directionY;

      // Java: if (offset.x != 0.0F && offset.y != 0.0F)

      {
         // Java: directionX = (int)offset.x;

         // Java: directionY = (int)offset.y;

      }
      // Java: else if (offset.x == 0.0F && offset.y != 0.0F)

      {
         // Java: directionX = 0;

         // Java: directionY = (int)offset.y;

      }
      // Java: else if (offset.x != 0.0F && offset.y == 0.0F)

      {
         // Java: directionX = (int)offset.x;

         // Java: directionY = 0;

      }
      // Java: else

      {
         // Java: directionX = scaleFactor;

         // Java: directionY = scaleFactor;

      }
      // Java: float outlineWid = spanInfo.hasOutline() ? spanInfo.mTextOutline.getLineWid() : 0.0F;

      // Java: int outlineClr = spanInfo.hasOutline() ? spanInfo.mTextOutline.getLineClr() : 0;

      // Java: if (outlineWid > 0.0F && !spanInfo.hasFill())

      {
         // Java: paint.setColor(outlineClr);

         // Java: paint.setStyle(Style.STROKE);

         // Java: paint.setStrokeWidth(Math.max(outlineWid, 0.25F));

         // Java: paint.setStrokeJoin(Join.ROUND);

      }
      // Java: else

      {
         // Java: paint.setStyle(Style.FILL);

      }
      // Java: float[] direction = new float[]{(float)directionX, (float)directionY, (float)directionZ};

      // Java: EmbossMaskFilter filter = new EmbossMaskFilter(direction, 0.8F, 1.0F, 2.0F);

      // Java: paint.setMaskFilter(filter);

    }

    // Java: float getNumberSign(float num)
    pub fn getNumberSign(&self, _num: f32) -> f32 {
      // Java: float err = 1.0E-6F;

      // Java: return (!(num > 0.0F) || !(num - err > 0.0F)) && (!(num < 0.0F) || !(num + err < 0.0F)) ? 0.0F : num;

      0.0
    }

    // Java: float[] calVertTextInfo(TextData data, Layout layout, Spannable text, SpanInfo spanInfo, int index, int offset, boolean canRotate)
    pub fn calVertTextInfo(&self, _data: i32, _layout: i32, _text: i32, _spanInfo: i32, _index: i32, _offset: i32, _canRotate: bool) -> Vec<f32> {
      // Java: float x = this.isPrimary(data.mBidi, index) ? layout.getPrimaryHorizontal(index) : layout.getSecondaryHorizontal(index);

      // Java: float cw = PaintFontHelper.measureCharWidth(data, index, index + offset);

      // Java: x = (float)((double)x + (double)cw * (canRotate ? 0.88 : (double)0.0F));

      // Java: float y;

      // Java: if (TextData.isChinese(text.charAt(index)))

      {
         // Java: y = layout.getLineBaseline(layout.getLineForOffset(index));

      }
      // Java: else

      {
         // Java: int lineOffset = layout.getLineForOffset(index);

         // Java: y = (layout.getLineTop(lineOffset) + layout.getLineBottom(lineOffset)) / 2.0F;

      }
      // Java: float delta;

      // Java: if (spanInfo.mIsSuperScript)

      {
         // Java: delta = cw * 0.2F;

         // Java: y -= delta;

      }
      // Java: else if (spanInfo.mIsSubScript)

      {
         // Java: delta = cw * 0.4F;

         // Java: y += delta;

      }
      // Java: else

      {
         // Java: delta = cw * (canRotate ? 0.15F : 0.0F);

         // Java: y += delta;

      }
      // Java: return new float[]{cw, x, y, delta};

      Vec::new()
    }

    // Java: void drawChineseVertSpan(Layout layout, int layoutIndex, int chSpanIndex, SpanInfo spanInfo, Canvas canvas, TextPaint paint, WpsImageSpan imageStyle)
    pub fn drawChineseVertSpan(&self, _layout: i32, _layoutIndex: i32, _chSpanIndex: i32, _spanInfo: i32, _canvas: i32, _paint: i32, _imageStyle: i32) {
      // Java: if (this.mChineseVertCaches == null)

      {
         // Java: this.mChineseVertCaches = new HashMap();

      }
      // Java: boolean useCache = false;

      // Java: boolean canRotate = this.mTextBoxData.canChineseVert();

      // Java: WordArtVertData vertInfo = (WordArtVertData)this.mChineseVertCaches.get(layoutIndex);

      // Java: if (vertInfo != null && vertInfo.getLayoutIndex() == layoutIndex)

      {
         // Java: List<WordArtVertData.TextUnitRenderData> dataList = vertInfo.getTextUnitList();

         // Java: if (dataList.size() >= spanInfo.mEndPos - spanInfo.mStartPos)

         {
            // Java: for(WordArtVertData.TextUnitRenderData data : dataList)

            {
               // Java: if (data != null && data.mSpanIndex == chSpanIndex && spanInfo.mStartPos <= data.mStart && spanInfo.mEndPos >= data.mEnd)

               {
                  // Java: useCache = true;

                  // Java: this.drawRotateText(spanInfo, canvas, data.mText, paint, data.mWidth, data.mStartX, data.mStartY, imageStyle);

               }
            }
         }
      }
      // Java: if (!useCache)

      {
         // Java: vertInfo = new WordArtVertData(layoutIndex);

         // Java: TextData data = this.mTextBoxData.getTextData(layoutIndex);

         // Java: Spannable text = (Spannable)layout.getText();

         // Java: int spanStart = Math.max(0, spanInfo.mStartPos);

         // Java: int spanEnd = Math.min(text.length(), spanInfo.mEndPos);

         // Java: for(int index = spanStart; index < spanEnd; ++index)

         {
            // Java: WordArtVertData.TextUnitRenderData unitData = new WordArtVertData.TextUnitRenderData();

            // Java: int offset = 1;

            // Java: if (Emoji.isEmojiChar(text.charAt(index)))

            {
               // Java: int emoji = Character.codePointAt(text, index);

               // Java: offset = Character.charCount(emoji);

            }
            // Java: float[] calData = this.calVertTextInfo(data, layout, text, spanInfo, index, offset, canRotate);

            // Java: String drawText = text.subSequence(index, index + offset).toString();

            // Java: this.drawRotateText(spanInfo, canvas, drawText, paint, calData[0], calData[1], calData[2], imageStyle);

            // Java: index += offset - 1;

            // Java: unitData.mSpanIndex = chSpanIndex;

            // Java: unitData.mStart = index;

            // Java: unitData.mEnd = index + offset;

            // Java: unitData.mWidth = calData[0];

            // Java: unitData.mStartX = calData[1];

            // Java: unitData.mStartY = calData[2];

            // Java: unitData.mText = drawText;

            // Java: vertInfo.addTextUnitData(unitData);

         }
         // Java: this.mChineseVertCaches.put(layoutIndex, vertInfo);

      }
    }

    // Java: void drawChineseVertSection(Layout layout, int layoutIndex, SpanInfo spanInfo, Canvas canvas, TextPaint paint, int start, int end, WpsImageSpan imageStyle)
    pub fn drawChineseVertSection(&self, _layout: i32, _layoutIndex: i32, _spanInfo: i32, _canvas: i32, _paint: i32, _start: i32, _end: i32, _imageStyle: i32) {
      // Java: boolean canRotate = this.mTextBoxData.canChineseVert();

      // Java: TextData data = this.mTextBoxData.getTextData(layoutIndex);

      // Java: Spannable text = (Spannable)layout.getText();

      // Java: int validStart = Math.max(start, 0);

      // Java: int validEnd = Math.min(end, text.length());

      // Java: for(int index = validStart; index < validEnd; ++index)

      {
         // Java: int offset = 1;

         // Java: if (Emoji.isEmojiChar(text.charAt(index)))

         {
            // Java: int emoji = Character.codePointAt(text, index);

            // Java: offset = Character.charCount(emoji);

         }
         // Java: float[] calData = this.calVertTextInfo(data, layout, text, spanInfo, index, offset, canRotate);

         // Java: String drawText = text.subSequence(index, index + offset).toString();

         // Java: this.drawRotateText(spanInfo, canvas, drawText, paint, calData[0], calData[1], calData[2], imageStyle);

         // Java: index += offset - 1;

      }
    }

    // Java: void drawChineseVertInnerSpan(Layout layout, int layoutIndex, SpanInfo spanInfo, Canvas canvas, TextPaint paint, ChineseVertSpan vertSpan)
    pub fn drawChineseVertInnerSpan(&self, _layout: i32, _layoutIndex: i32, _spanInfo: i32, _canvas: i32, _paint: i32, _vertSpan: i32) {
      // Java: boolean canRotate = this.mTextBoxData.canChineseVert();

      // Java: TextData data = this.mTextBoxData.getTextData(layoutIndex);

      // Java: Spannable text = (Spannable)layout.getText();

      // Java: int spanStart = Math.max(0, spanInfo.mStartPos);

      // Java: int spanEnd = Math.min(text.length(), spanInfo.mEndPos);

      // Java: PointF shadowOffset = new PointF();

      // Java: if (spanInfo.hasInnerShdw())

      {
         // Java: if (spanInfo.mTextShadow.getIsDoc())

         {
            // Java: float scale = spanInfo.mFontSize / 100.0F;

            // Java: shadowOffset.set(-spanInfo.mTextShadow.getShdwOffsetX() * scale, -spanInfo.mTextShadow.getShdwOffsetY() * scale);

         }
         // Java: else

         {
            // Java: float shadowDis = spanInfo.mTextShadow.getShdwDis();

            // Java: shadowOffset = this.getInnerShadowOfafset(spanInfo.mTextShadow.getShdwAngle(), shadowDis, spanInfo.mFontSize);

         }
      }
      // Java: CharacterStyle[] shadowStyle = null;

      // Java: CharacterStyle[] lightStyle = null;

      // Java: Layout[] innerLayout = (Layout[])this.mInnerLayoutMap.get(layoutIndex);

      // Java: if (innerLayout != null && innerLayout.length == 2)

      {
         // Java: Spannable shadowSpan = (Spannable)innerLayout[0].getText();

         // Java: CharacterStyle[] styles = (CharacterStyle[])shadowSpan.getSpans(spanStart, spanEnd, WpsOuterShdwSpan.class);

         // Java: if (styles == null || styles.length == 0)

         {
            // Java: styles = (CharacterStyle[])shadowSpan.getSpans(spanStart, spanEnd, ForegroundColorSpan.class);

         }
         // Java: if (styles != null && styles.length > 0)

         {
            // Java: shadowStyle = new CharacterStyle[]{styles[styles.length - 1]};

         }
         // Java: CharacterStyle normalStyle = null;

         // Java: Spannable lightSpan = (Spannable)innerLayout[1].getText();

         // Java: styles = (CharacterStyle[])lightSpan.getSpans(spanStart, spanEnd, WpsShdwLightSpan.class);

         // Java: if (styles == null || styles.length == 0)

         {
            // Java: styles = (CharacterStyle[])lightSpan.getSpans(spanStart, spanEnd, ForegroundColorSpan.class);

         }
         // Java: if (styles != null && styles.length > 0)

         {
            // Java: normalStyle = styles[styles.length - 1];

         }
         // Java: CharacterStyle fillStyle = null;

         // Java: if (this.mTypoFillCaches.size() > layoutIndex && spanInfo.mFontFill.mFillType != FontFill.FontFillType.NONE && spanInfo.mFontFill.mFillType != FontFill.FontFillType.COLORFILL)

         {
            // Java: WpsForegroundColorSpan[] spans = (WpsForegroundColorSpan[])((Spannable)((Layout)this.mTypoFillCaches.get(layoutIndex)).getText()).getSpans(spanStart, spanEnd, WpsForegroundColorSpan.class);

            // Java: fillStyle = spans[spans.length - 1];

         }
         // Java: if (normalStyle != null || fillStyle != null)

         {
            // Java: lightStyle = new CharacterStyle[]{fillStyle, normalStyle};

         }
      }
      // Java: for(int index = spanStart; index < spanEnd; ++index)

      {
         // Java: int offset = 1;

         // Java: if (Emoji.isEmojiChar(text.charAt(index)))

         {
            // Java: int emoji = Character.codePointAt(text, index);

            // Java: offset = Character.charCount(emoji);

         }
         // Java: int line = layout.getLineForOffset(index);

         // Java: float[] calData = this.calVertTextInfo(data, layout, text, spanInfo, index, offset, canRotate);

         // Java: String drawText = text.subSequence(index, index + offset).toString();

         // Java: if (shadowStyle != null && lightStyle != null)

         {
            // Java: float height = layout.getLineBottom(line) - layout.getLineTop(line);

            // Java: Bitmap bitmap = this.genVertInnerBitmap(layout, shadowStyle, lightStyle, drawText, calData[0], height, vertSpan, line, shadowOffset);

            // Java: if (bitmap != null)

            {
               // Java: BitmapShader shader = new BitmapShader(bitmap, TileMode.CLAMP, TileMode.CLAMP);

               // Java: Matrix matrix = new Matrix();

               // Java: matrix.setTranslate(calData[1], layout.getLineTop(line) + calData[3]);

               // Java: shader.setLocalMatrix(matrix);

               // Java: paint.setShader(shader);

            }
            // Java: else

            {
               // Java: this.updateEmbossPaint(paint, spanInfo, shadowOffset);

            }
         }
         // Java: this.drawRotateText(spanInfo, canvas, drawText, paint, calData[0], calData[1], calData[2], (WpsImageSpan)null);

         // Java: index += offset - 1;

      }
    }

    // Java: Bitmap genVertInnerBitmap(Layout layout, CharacterStyle[] shadowStyle, CharacterStyle[] lightStyle, String drawText, float width, float height, ChineseVertSpan vertSpan, int line, PointF shadowOffset)
    pub fn genVertInnerBitmap(&self, _layout: i32, _shadowStyle: i32, _lightStyle: i32, _drawText: String, _width: f32, _height: f32, _vertSpan: i32, _line: i32, _shadowOffset: i32) -> i32 {
      // Java: if (layout != null && shadowStyle != null && lightStyle != null && !(width <= 0.0F) && !(height <= 0.0F))

      {
         // Java: Bitmap bitmap;

         // Java: try

         {
            // Java: bitmap = Bitmap.createBitmap((int)width, (int)height, Config.ARGB_8888);

         }
         // Java: catch (Exception | OutOfMemoryError var19)

         {
            // Java: return null;

         }
         // Java: Canvas shaderCanvas = new Canvas(bitmap);

         // Java: TextPaint shadowPaint = new TextPaint();

         // Java: this.initVertTextPaint(shadowPaint, vertSpan);

         // Java: for(CharacterStyle style : shadowStyle)

         {
            // Java: if (style != null)

            {
               // Java: style.updateDrawState(shadowPaint);

            }
         }
         // Java: float y = layout.getLineBaseline(line) - layout.getLineTop(line);

         // Java: shaderCanvas.drawText(drawText, 0.0F, y, shadowPaint);

         // Java: TextPaint lightPaint = new TextPaint();

         // Java: this.initVertTextPaint(lightPaint, vertSpan);

         // Java: for(CharacterStyle style : lightStyle)

         {
            // Java: if (style != null)

            {
               // Java: style.updateDrawState(lightPaint);

            }
         }
         // Java: Xfermode oldMode = lightPaint.getXfermode();

         // Java: lightPaint.setXfermode(new PorterDuffXfermode(Mode.SRC_ATOP));

         // Java: shaderCanvas.translate(-shadowOffset.x, -shadowOffset.y);

         // Java: y = layout.getLineBaseline(line) - layout.getLineTop(line);

         // Java: shaderCanvas.drawText(drawText, 0.0F, y, lightPaint);

         // Java: shaderCanvas.translate(shadowOffset.x, shadowOffset.y);

         // Java: lightPaint.setXfermode(oldMode);

         // Java: return bitmap;

      }
      // Java: else

      {
         // Java: return null;

      }
      0
    }

    // Java: void initVertTextPaint(TextPaint paint, ChineseVertSpan vertSpan)
    pub fn initVertTextPaint(&self, _paint: i32, _vertSpan: i32) {
      // Java: paint.reset();

      // Java: paint.setAntiAlias(true);

      // Java: paint.setColor(vertSpan.getFontColor());

      // Java: paint.setTextSize(vertSpan.getFontSize());

      // Java: paint.setTypeface(vertSpan.getTypeface());

      // Java: if ((vertSpan.getFontStyle() & 1) != 0)

      {
         // Java: paint.setFakeBoldText(true);

      }
      // Java: if ((vertSpan.getFontStyle() & 2) != 0)

      {
         // Java: paint.setTextSkewX(-0.25F);

      }
    }

    // Java: void drawRotateText(SpanInfo span, Canvas canvas, String drawText, TextPaint paint, float width, float x, float y, WpsImageSpan imageStyle)
    pub fn drawRotateText(&self, _span: i32, _canvas: i32, _drawText: String, _paint: i32, _width: f32, _x: f32, _y: f32, _imageStyle: i32) {
      // Java: boolean isDrawTextImage = imageStyle != null;

      // Java: TextPaint finalPaint = new TextPaint(paint);

      // Java: if (isDrawTextImage)

      {
         // Java: imageStyle.setTextWidth(width);

         // Java: imageStyle.updateDrawState(finalPaint);

      }
      // Java: if (this.mTextBoxData.canChineseVert())

      {
         // Java: if (isDrawTextImage)

         {
            // Java: canvas.save();

            // Java: canvas.rotate(-90.0F, x, y);

            // Java: Matrix matrix = new Matrix();

            // Java: matrix.setScale(-1.0F, 1.0F);

            // Java: canvas.concat(matrix);

            // Java: Rect rect = canvas.getClipBounds();

            // Java: rect.left = (int)(-x - width / 4.0F);

            // Java: canvas.clipRect(rect);

            // Java: canvas.translate(-x - width / 4.0F, 0.0F);

            // Java: if (span.mTextImage != null)

            {
               // Java: float ds = width / 18.0F * span.mTextImage.getDist();

               // Java: canvas.translate(ds, 0.0F);

            }
            // Java: canvas.drawText(drawText, 0.0F, y, finalPaint);

            // Java: canvas.restore();

         }
         // Java: else

         {
            // Java: canvas.rotate(-90.0F, x, y);

            // Java: canvas.drawText(drawText, x, y, finalPaint);

            // Java: canvas.rotate(90.0F, x, y);

         }
      }
      // Java: else

      {
         // Java: canvas.drawText(drawText, x, y, finalPaint);

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

    // Java: RectF adjustTxtRectArtTrans(RectF rect, List<Layout> typoCaches)
    pub fn adjustTxtRectArtTrans(&self, _rect: i32, _typoCaches: i32) -> i32 {
      // Java: float dLeft = 0.0F;

      // Java: float dTop = 0.0F;

      // Java: float dRight = 0.0F;

      // Java: float dBottom = 0.0F;

      // Java: Boolean isOutline = (Boolean)this.mWordArtAttr.get((byte)2);

      // Java: Boolean isBlur = (Boolean)this.mWordArtAttr.get((byte)3);

      // Java: Boolean isOuterShadow = (Boolean)this.mWordArtAttr.get((byte)5);

      // Java: if (isOutline != null && isOutline)

      {
         // Java: float outlnWid = this.getMaxOutlineWid();

         // Java: float offset = outlnWid * 0.4F;

         // Java: dLeft = Math.max(offset, dLeft);

         // Java: dTop = Math.max(offset, dTop);

         // Java: dRight = Math.max(offset, dRight);

         // Java: dBottom = Math.max(offset, dBottom);

      }
      // Java: if (isBlur != null && isBlur)

      {
         // Java: float offset = this.getMaxBlurRad();

         // Java: dLeft = Math.max(offset, dLeft);

         // Java: dTop = Math.max(offset, dTop);

         // Java: dRight = Math.max(offset, dRight);

         // Java: dBottom = Math.max(offset, dBottom);

      }
      // Java: RectF rectDst = new RectF(rect.left - dLeft, rect.top - dTop, rect.right + dRight, rect.bottom + dBottom);

      // Java: if (isOuterShadow != null && isOuterShadow)

      {
         // Java: this.adjustByOuterShadow(typoCaches, rectDst);

      }
      // Java: return rectDst;

      0
    }

    // Java: void adjustByOuterShadow(List<Layout> typoCaches, RectF rectSrc)
    pub fn adjustByOuterShadow(&self, _typoCaches: i32, _rectSrc: i32) {
      // Java: for(int para = 0; para < this.mTextBoxData.size(); ++para)

      {
         // Java: TextData data = this.mTextBoxData.getTextData(para);

         // Java: if (data.hasOuterShdw())

         {
            // Java: Layout layout = (Layout)typoCaches.get(para);

            // Java: int spanInx = 0;

            // Java: int spanSize = data.mOtherSpans.size();

            // Java: int textLen = data.mTypoText.length();

            // Java: SpanInfo span = data.mFirstSpan;

            // Java: for(int line = 0; line < layout.getLineCount(); ++line)

            {
               // Java: int lnEndOffset = layout.getLineEnd(line);

               // Java: while(span != null && span.mEndPos <= textLen)

               {
                  // Java: int spanEnd = span.mEndPos;

                  // Java: if (span.hasOuterShdw())

                  {
                     // Java: float startX = 0.0F;

                     // Java: float startY = layout.getLineTop(line);

                     // Java: TextShadow shadow = span.mTextShadow;

                     // Java: float lnHei = layout.getLineBaseline(line) - layout.getLineTop(line);

                     // Java: RectF rect = new RectF(0.0F, 0.0F, layout.getLineRight(line) - layout.getLineLeft(line), lnHei * 1.18F);

                     // Java: if (!shadow.getIsDoc())

                     {
                        // Java: Matrix matrixTmp = new Matrix();

                        // Java: PointF offset = this.getShadowOffset(shadow.getShdwAlgn(), new RectF(0.0F, 0.0F, this.mArtTransform.getTextWidth(), this.mArtTransform.getTextHeight()));

                        // Java: float dis = shadow.getShdwDis();

                        // Java: float angle = shadow.getShdwAngle();

                        // Java: float x = (float)((double)dis * Math.cos(Math.toRadians((double)angle)));

                        // Java: float y = (float)((double)dis * Math.sin(Math.toRadians((double)angle)));

                        // Java: float skewX = (float)Math.tan(Math.toRadians((double)shadow.getShdwXSkew()));

                        // Java: float skewY = (float)Math.tan(Math.toRadians((double)shadow.getShdwYSkew()));

                        // Java: PointF scale = this.mArtTransform.getScale();

                        // Java: matrixTmp.preScale(scale.x, scale.y);

                        // Java: matrixTmp.preTranslate(offset.x, offset.y);

                        // Java: matrixTmp.preTranslate(x, y);

                        // Java: matrixTmp.preScale(shadow.getShdwXPct(), shadow.getShdwYPct());

                        // Java: matrixTmp.preSkew(skewX, skewY);

                        // Java: matrixTmp.preTranslate(-offset.x, -offset.y);

                        // Java: matrixTmp.preTranslate(((PointF)this.mTypoOffsets.get(para)).x + startX, ((PointF)this.mTypoOffsets.get(para)).y + startY);

                        // Java: RectF rectShdw = new RectF();

                        // Java: matrixTmp.mapRect(rectShdw, rect);

                        // Java: rectSrc.union(rectShdw);

                     }
                     // Java: else

                     {
                        // Java: Matrix matrixTmp = new Matrix();

                        // Java: PointF offset = this.getShadowOffsetDoc(shadow.getOrigin(), new RectF(0.0F, 0.0F, this.mArtTransform.getTextWidth(), this.mArtTransform.getTextHeight()));

                        // Java: matrixTmp.preTranslate(shadow.getShdwOffsetX() + offset.x, shadow.getShdwOffsetY() + offset.y);

                        // Java: matrixTmp.preScale(shadow.getShdwXPct(), shadow.getShdwYPct());

                        // Java: matrixTmp.preSkew(shadow.getShdwXSkew(), shadow.getShdwYSkew());

                        // Java: matrixTmp.preTranslate(-offset.x, -offset.y);

                        // Java: matrixTmp.preTranslate(((PointF)this.mTypoOffsets.get(para)).x + startX, ((PointF)this.mTypoOffsets.get(para)).y + startY);

                        // Java: RectF rectShdw = new RectF();

                        // Java: matrixTmp.mapRect(rectShdw, rect);

                        // Java: rectSrc.union(rectShdw);

                     }
                  }
                  // Java: if (spanEnd <= lnEndOffset)

                  {
                     // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

                     // Java: ++spanInx;

                  }
                  // Java: else

                  {
                     // Java: ++line;

                     // Java: lnEndOffset = layout.getLineEnd(line);

                  }
               }
            }
         }
      }
    }

    // Java: void adjustTxtRect(RectF rect, List<Layout> typoCaches, List<PointF> typoOffsets, float boxLeft)
    pub fn adjustTxtRect(&self, _rect: i32, _typoCaches: i32, _typoOffsets: i32, _boxLeft: f32) {
      // Java: float dLeft = 0.0F;

      // Java: float dTop = 0.0F;

      // Java: float dRight = 0.0F;

      // Java: float dBottom = 0.0F;

      // Java: Boolean isOutline = (Boolean)this.mWordArtAttr.get((byte)2);

      // Java: Boolean isBlur = (Boolean)this.mWordArtAttr.get((byte)3);

      // Java: Boolean isReflection = (Boolean)this.mWordArtAttr.get((byte)4);

      // Java: Boolean isOuterShadow = (Boolean)this.mWordArtAttr.get((byte)5);

      // Java: if (isOutline != null && isOutline)

      {
         // Java: float outlnWid = this.getMaxOutlineWid();

         // Java: float offset = outlnWid * 0.4F;

         // Java: dLeft = Math.max(offset, dLeft);

         // Java: dTop = Math.max(offset, dTop);

         // Java: dRight = Math.max(offset, dRight);

         // Java: dBottom = Math.max(offset, dBottom);

      }
      // Java: if (isBlur != null && isBlur)

      {
         // Java: float offset = this.getMaxBlurRad();

         // Java: dLeft = Math.max(offset, dLeft);

         // Java: dTop = Math.max(offset, dTop);

         // Java: dRight = Math.max(offset, dRight);

         // Java: dBottom = Math.max(offset, dBottom);

      }
      // Java: if (isReflection != null && isReflection)

      {
         // Java: float bottomSpace = this.getMaxReflectOffset(rect, typoCaches, typoOffsets);

         // Java: dBottom = Math.max(bottomSpace, dBottom);

      }
      // Java: if (isOuterShadow != null && isOuterShadow)

      {
         // Java: float[] offsets = this.getOuterShdwOffset(typoCaches, rect, boxLeft);

         // Java: if (offsets.length == 4)

         {
            // Java: dLeft = Math.max(offsets[0], dLeft);

            // Java: dTop = Math.max(offsets[1], dTop);

            // Java: dRight = Math.max(offsets[2], dRight);

            // Java: dBottom = Math.max(offsets[3], dBottom);

         }
      }
      // Java: rect.set(rect.left - dLeft, rect.top - dTop, rect.right + dRight, rect.bottom + dBottom);

    }

    // Java: float getMaxOutlineWid()
    pub fn getMaxOutlineWid(&self) -> f32 {
      // Java: if (this.mTextBoxData.size() != this.mTypoOutlineCaches.size())

      {
         // Java: return 0.0F;

      }
      // Java: else

      {
         // Java: float maxOutlnWid = 0.0F;

         // Java: for(int i = 0; i < this.mTypoOutlineCaches.size(); ++i)

         {
            // Java: if (this.mTypoOutlineCaches.get(i) != null)

            {
               // Java: Spannable spannable = (Spannable)((Layout)this.mTypoOutlineCaches.get(i)).getText();

               // Java: WpsTextOutlineSpan[] outlnSpans = (WpsTextOutlineSpan[])spannable.getSpans(0, spannable.length(), WpsTextOutlineSpan.class);

               // Java: for(WpsTextOutlineSpan outlnSpan : outlnSpans)

               {
                  // Java: float outlnWid = outlnSpan.getActualWid_pt();

                  // Java: if (maxOutlnWid < outlnWid)

                  {
                     // Java: maxOutlnWid = outlnWid;

                  }
               }
            }
         }
         // Java: return maxOutlnWid;

      }
      0.0
    }

    // Java: float getMaxBlurRad()
    pub fn getMaxBlurRad(&self) -> f32 {
      // Java: if (this.mTextBoxData.size() != this.mTypoBlurCaches.size())

      {
         // Java: return 0.0F;

      }
      // Java: else

      {
         // Java: float maxBlurRad = 0.0F;

         // Java: for(int i = 0; i < this.mTypoBlurCaches.size(); ++i)

         {
            // Java: if (this.mTypoBlurCaches.get(i) != null)

            {
               // Java: Spannable spannable = (Spannable)((Layout)this.mTypoBlurCaches.get(i)).getText();

               // Java: WpsTextBlurSpan[] spans = (WpsTextBlurSpan[])spannable.getSpans(0, spannable.length(), WpsTextBlurSpan.class);

               // Java: for(WpsTextBlurSpan span : spans)

               {
                  // Java: float blurRad = span.getActualBlurRad();

                  // Java: if (maxBlurRad < blurRad)

                  {
                     // Java: maxBlurRad = blurRad;

                  }
               }
            }
         }
         // Java: return maxBlurRad;

      }
      0.0
    }

    // Java: float getMaxReflectOffset(RectF rect, List<Layout> typoCaches, List<PointF> typoOffsets)
    pub fn getMaxReflectOffset(&self, _rect: i32, _typoCaches: i32, _typoOffsets: i32) -> f32 {
      // Java: if (this.mTextBoxData.size() != typoCaches.size())

      {
         // Java: return 0.0F;

      }
      // Java: else

      {
         // Java: float dBottom = 0.0F;

         // Java: for(int para = 0; para < typoCaches.size(); ++para)

         {
            // Java: TextData data = this.mTextBoxData.getTextData(para);

            // Java: if (data.hasTextImage())

            {
               // Java: Layout layout = (Layout)typoCaches.get(para);

               // Java: int spanInx = 0;

               // Java: int spanSize = data.mOtherSpans.size();

               // Java: int textLen = layout.getText().length();

               // Java: SpanInfo span = data.mFirstSpan;

               // Java: for(int line = 0; line < layout.getLineCount(); ++line)

               {
                  // Java: int lnEndOffset = layout.getLineEnd(line);

                  // Java: while(span != null && span.mEndPos <= textLen)

                  {
                     // Java: int spanEnd = span.mEndPos;

                     // Java: float lnHei = layout.getLineBaseline(line) - layout.getLineTop(line);

                     // Java: if (span.hasTextImage())

                     {
                        // Java: float distance = lnHei / 18.0F * span.mTextImage.getDist();

                        // Java: distance += lnHei * (float)span.mTextImage.getEndPos();

                        // Java: float space = rect.bottom - ((PointF)typoOffsets.get(para)).y - layout.getLineBaseline(line);

                        // Java: if (distance > space && distance - space > dBottom)

                        {
                           // Java: dBottom = distance - space;

                        }
                     }
                     // Java: if (spanEnd < lnEndOffset)

                     {
                        // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

                        // Java: ++spanInx;

                     }
                     // Java: else

                     {
                        // Java: ++line;

                        // Java: if (line >= layout.getLineCount())

                        {
                           // Java: break;

                        }
                        // Java: lnEndOffset = layout.getLineEnd(line);

                     }
                  }
               }
            }
         }
         // Java: return dBottom;

      }
      0.0
    }

    // Java: float[] getOuterShdwOffset(List<Layout> typoCaches, RectF rectSrc, float boxLeft)
    pub fn getOuterShdwOffset(&self, _typoCaches: i32, _rectSrc: i32, _boxLeft: f32) -> Vec<f32> {
      // Java: float[] offsets = new float[4];

      // Java: RectF rectDst = new RectF(rectSrc.left, rectSrc.top, rectSrc.right, rectSrc.bottom);

      // Java: for(int para = 0; para < this.mTextBoxData.size(); ++para)

      {
         // Java: TextData data = this.mTextBoxData.getTextData(para);

         // Java: if (data.hasOuterShdw())

         {
            // Java: Layout layout = (Layout)typoCaches.get(para);

            // Java: int spanInx = 0;

            // Java: int spanSize = data.mOtherSpans.size();

            // Java: int textLen = data.mSourceText.length();

            // Java: SpanInfo span = data.mFirstSpan;

            // Java: for(int line = 0; line < layout.getLineCount(); ++line)

            {
               // Java: int lnEndOffset = layout.getLineEnd(line);

               // Java: while(span != null && span.mEndPos <= textLen)

               {
                  // Java: int spanEnd = span.mEndPos;

                  // Java: if (span.hasOuterShdw())

                  {
                     // Java: float startX = 0.0F;

                     // Java: float startY = layout.getLineTop(line);

                     // Java: float lnHei = layout.getLineBaseline(line) - layout.getLineTop(line);

                     // Java: RectF rect = new RectF(0.0F, 0.0F, layout.getLineRight(line) - layout.getLineLeft(line), lnHei * 1.18F);

                     // Java: PointF offset = this.getShadowOffset(span.mTextShadow.getShdwAlgn(), rect);

                     // Java: float dis = span.mTextShadow.getShdwDis();

                     // Java: float angle = span.mTextShadow.getShdwAngle();

                     // Java: float disScale = this.getOuterShadowDisScale(span.mFontSize);

                     // Java: float x = (float)((double)dis * Math.cos(Math.toRadians((double)angle))) * disScale;

                     // Java: float y = (float)((double)dis * Math.sin(Math.toRadians((double)angle))) * disScale;

                     // Java: rect.offset(rectSrc.left + startX + x, rectSrc.top + startY + y);

                     // Java: float left = rect.left;

                     // Java: float top = rect.top;

                     // Java: rect.offset(-left, -top);

                     // Java: rect.left = rect.left + layout.getLineLeft(line) - boxLeft;

                     // Java: rect.right = rect.right + layout.getLineLeft(line) - boxLeft;

                     // Java: Matrix matrix = new Matrix();

                     // Java: matrix.preTranslate(offset.x, offset.y);

                     // Java: matrix.preScale(span.mTextShadow.getShdwXPct(), span.mTextShadow.getShdwYPct());

                     // Java: matrix.preTranslate(-offset.x, -offset.y);

                     // Java: RectF rectTmp = new RectF(rect.left, rect.top, rect.right, rect.bottom);

                     // Java: matrix.mapRect(rectTmp, rect);

                     // Java: rectTmp.offset(left, top);

                     // Java: rectDst.union(rectTmp);

                  }
                  // Java: if (spanEnd <= lnEndOffset)

                  {
                     // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

                     // Java: ++spanInx;

                  }
                  // Java: else

                  {
                     // Java: ++line;

                     // Java: lnEndOffset = layout.getLineEnd(line);

                  }
               }
            }
         }
      }
      // Java: offsets[0] = rectSrc.left - rectDst.left;

      // Java: offsets[1] = rectSrc.top - rectDst.top;

      // Java: offsets[2] = rectDst.right - rectSrc.right;

      // Java: offsets[3] = rectDst.bottom - rectSrc.bottom;

      // Java: return offsets;

      Vec::new()
    }

    // Java: List<Spannable> getImgSpans(TextData data, int para)
    pub fn getImgSpans(&self, _data: i32, _para: i32) -> Vec<i32> {
      // Java: List<Spannable> spannables = new ArrayList();

      // Java: if (this.mTypoBlurCaches != null && this.mTypoBlurCaches.size() > para && this.mTypoBlurCaches.get(para) != null)

      {
         // Java: Layout blurLayout = (Layout)this.mTypoBlurCaches.get(para);

         // Java: Spannable blurSpan = new SpannableStringBuilder(blurLayout.getText());

         // Java: spannables.add(blurSpan);

      }
      // Java: Layout fillLayout = (Layout)this.mTypoFillCaches.get(para);

      // Java: Spannable fillSpan = new SpannableStringBuilder(fillLayout.getText());

      // Java: spannables.add(fillSpan);

      // Java: if (this.mShadowLayoutCaches != null && this.mShadowLayoutCaches.get(para) != null)

      {
         // Java: Layout shadowLayout = ((ShadowLayoutGroup)this.mShadowLayoutCaches.get(para)).mFillShadowLayout;

         // Java: Spannable shadowSpan = new SpannableStringBuilder(shadowLayout.getText());

         // Java: spannables.add(shadowSpan);

      }
      // Java: if (this.mTypoOutlineCaches != null && this.mTypoOutlineCaches.size() > para && this.mTypoOutlineCaches.get(para) != null)

      {
         // Java: Layout outlnLayout = (Layout)this.mTypoOutlineCaches.get(para);

         // Java: Spannable outlnSpan = new SpannableStringBuilder(outlnLayout.getText());

         // Java: spannables.add(outlnSpan);

      }
      // Java: if (this.mInnerShadowCaches != null && this.mInnerShadowCaches.size() > para && this.mInnerShadowCaches.get(para) != null)

      {
         // Java: Layout innerShdwLayout = (Layout)this.mInnerShadowCaches.get(para);

         // Java: Spannable innerShdwSpan = new SpannableStringBuilder(innerShdwLayout.getText());

         // Java: spannables.add(innerShdwSpan);

      }
      // Java: return spannables;

      Vec::new()
    }

    // Java: List<SingleImageSpannable> getImageSpans(int para)
    pub fn getImageSpans(&self, _para: i32) -> Vec<i32> {
      // Java: List<SingleImageSpannable> needSpanList = new ArrayList();

      // Java: if (this.mShadowLayoutCaches != null && this.mShadowLayoutCaches.size() > para && this.mShadowLayoutCaches.get(para) != null)

      {
         // Java: Layout shadowLayout = ((ShadowLayoutGroup)this.mShadowLayoutCaches.get(para)).mFillShadowLayout;

         // Java: needSpanList.add(new SingleImageSpannable((byte)5, shadowLayout.getText()));

      }
      // Java: if (this.mTypoBlurCaches != null && this.mTypoBlurCaches.size() > para && this.mTypoBlurCaches.get(para) != null)

      {
         // Java: Layout blurLayout = (Layout)this.mTypoBlurCaches.get(para);

         // Java: needSpanList.add(new SingleImageSpannable((byte)3, blurLayout.getText()));

      }
      // Java: if (this.mTypoOutlineCaches != null && this.mTypoOutlineCaches.size() > para && this.mTypoOutlineCaches.get(para) != null)

      {
         // Java: Layout outlineLayout = (Layout)this.mTypoOutlineCaches.get(para);

         // Java: needSpanList.add(new SingleImageSpannable((byte)2, outlineLayout.getText()));

      }
      // Java: if (this.mTypoFillCaches != null && this.mTypoFillCaches.size() > para && this.mTypoFillCaches.get(para) != null)

      {
         // Java: Layout fillLayout = (Layout)this.mTypoFillCaches.get(para);

         // Java: needSpanList.add(new SingleImageSpannable((byte)1, fillLayout.getText()));

      }
      // Java: if (this.mInnerShadowCaches != null && this.mInnerShadowCaches.size() > para && this.mInnerShadowCaches.get(para) != null)

      {
         // Java: Layout innerLayout = (Layout)this.mInnerShadowCaches.get(para);

         // Java: needSpanList.add(new SingleImageSpannable((byte)6, innerLayout.getText()));

      }
      // Java: return needSpanList;

      Vec::new()
    }

    // Java: boolean checkNeedDrawLayout(int effectType, SpanInfo spanInfo)
    pub fn checkNeedDrawLayout(&self, _effectType: i32, _spanInfo: i32) -> bool {
      // Java: boolean need = true;

      // Java: switch (effectType)

      {
         // Java: case 2:

            // Java: need = spanInfo.hasOutline();

            // Java: break;

         // Java: case 3:

            // Java: need = spanInfo.hasBlur();

         // Java: case 4:

         // Java: default:

            // Java: break;

         // Java: case 5:

            // Java: need = spanInfo.hasOuterShdw();

            // Java: break;

         // Java: case 6:

            // Java: need = spanInfo.hasInnerShdw();

      }
      // Java: return need;

      false
    }

    // Java: void renderInvertImage(Canvas workCanvas, Layout layout, int para)
    pub fn renderInvertImage(&self, _workCanvas: i32, _layout: i32, _para: i32) {
      // Java: boolean verticalText = this.mTextBoxData.verticalText() == 4;

      // Java: TextData data = this.mTextBoxData.getTextData(para);

      // Java: List<SingleImageSpannable> spanList = this.getImageSpans(para);

      // Java: int spanInx = 0;

      // Java: int spanSize = data.mOtherSpans.size();

      // Java: int textLen = data.mSourceText.length();

      // Java: SpanInfo span = data.mFirstSpan;

      // Java: CharacterStyle preSpan = WpsHideSpan.Factory.create();

      // Java: CharacterStyle aftSpan = WpsHideSpan.Factory.create();

      // Java: Spannable text = (Spannable)layout.getText();

      // Java: ChineseVertSpan[] vertSpans = verticalText ? (ChineseVertSpan[])text.getSpans(0, text.length(), ChineseVertSpan.class) : null;

      // Java: SpanInfo curSpanInfo = null;

      // Java: int verSpanIndex = 0;

      // Java: for(int line = 0; line < layout.getLineCount(); ++line)

      {
         // Java: layout.getLineStart(line);

         // Java: int lnEndOffset = layout.getLineEnd(line);

         // Java: while(span != null && span.mEndPos <= textLen)

         {
            // Java: int spanStart = span.mStartPos;

            // Java: int spanEnd = span.mEndPos;

            // Java: if (span.hasTextImage())

            {
               // Java: ShadowTypoData shadowInfo = null;

               // Java: if (this.mSpanShadowCaches != null)

               {
                  // Java: Map<Integer, ShadowTypoData> typoDataMap = (Map)this.mSpanShadowCaches.get(para);

                  // Java: if (typoDataMap != null)

                  {
                     // Java: shadowInfo = (ShadowTypoData)typoDataMap.get(spanInx);

                  }
               }
               // Java: ShadowImageData shadowData = null;

               // Java: if (shadowInfo != null)

               {
                  // Java: ShadowRenderData renderData = this.calRenderData(span.mTextShadow, shadowInfo, (PointF)this.mTypoOffsets.get(para), layout, line, span.mFontSize);

                  // Java: shadowData = this.getShadowImageData(span, shadowInfo, renderData);

               }
               // Java: if (verticalText && vertSpans != null && vertSpans.length > verSpanIndex)

               {
                  // Java: int vertStart = vertSpans[verSpanIndex].getStart();

                  // Java: int vertEnd = vertSpans[verSpanIndex].getEnd();

                  // Java: boolean hasFound = false;

                  // Java: for(int i = verSpanIndex; i < vertSpans.length; ++i)

                  {
                     // Java: if (vertStart < span.mStartPos)

                     {
                        // Java: ++verSpanIndex;

                     }
                     // Java: else if (vertEnd <= span.mEndPos)

                     {
                        // Java: hasFound = true;

                        // Java: break;

                     }
                     // Java: if (vertSpans.length > verSpanIndex)

                     {
                        // Java: vertStart = vertSpans[verSpanIndex].getStart();

                        // Java: vertEnd = vertSpans[verSpanIndex].getEnd();

                     }
                  }
                  // Java: if (hasFound && vertSpans.length > verSpanIndex)

                  {
                     // Java: int seqStart = verSpanIndex;

                     // Java: int seqEnd = verSpanIndex;

                     // Java: for(int i = verSpanIndex + 1; i < vertSpans.length; ++i)

                     {
                        // Java: vertEnd = vertSpans[i].getEnd();

                        // Java: if (vertEnd > span.mEndPos)

                        {
                           // Java: break;

                        }
                        // Java: ++seqEnd;

                        // Java: ++verSpanIndex;

                     }
                     // Java: int lastEnd = vertSpans[seqStart].getEnd();

                     // Java: curSpanInfo = new SpanInfo(span);

                     // Java: for(int i = seqStart; i <= seqEnd; ++i)

                     {
                        // Java: vertStart = vertSpans[i].getStart();

                        // Java: vertEnd = vertSpans[i].getEnd();

                        // Java: if (i > seqStart && vertStart - lastEnd > 0)

                        {
                           // Java: SpanInfo splitSpan = new SpanInfo(span);

                           // Java: splitSpan.mStartPos = lastEnd;

                           // Java: splitSpan.mEndPos = vertStart;

                           // Java: curSpanInfo.mStartPos = vertStart;

                           // Java: curSpanInfo.mEndPos = vertEnd;

                           // Java: if (data.mVertical != 6 && data.mVertical != 3)

                           {
                              // Java: this.drawHorizontalTextImage(workCanvas, layout, span, line, data, spanList, shadowData, preSpan, aftSpan);

                           }
                        }
                        // Java: else

                        {
                           // Java: boolean notFixPos = vertStart == span.mStartPos && vertEnd == span.mEndPos;

                           // Java: if (!notFixPos)

                           {
                              // Java: curSpanInfo.mStartPos = vertStart;

                              // Java: curSpanInfo.mEndPos = vertEnd;

                           }
                        }
                        // Java: this.drawVerticalTextImage(workCanvas, layout, para, vertSpans[i], curSpanInfo, spanList, shadowData, spanInx);

                        // Java: lastEnd = vertSpans[i].getEnd();

                     }
                     // Java: if (lastEnd < span.mEndPos)

                     {
                        // Java: SpanInfo splitSpan = new SpanInfo(span);

                        // Java: splitSpan.mStartPos = lastEnd;

                        // Java: splitSpan.mEndPos = span.mEndPos;

                        // Java: if (data.mVertical != 6 && data.mVertical != 3)

                        {
                           // Java: this.drawHorizontalTextImage(workCanvas, layout, span, line, data, spanList, shadowData, preSpan, aftSpan);

                        }
                     }
                  }
                  // Java: else if (data.mVertical != 6 && data.mVertical != 3)

                  {
                     // Java: this.drawHorizontalTextImage(workCanvas, layout, span, line, data, spanList, shadowData, preSpan, aftSpan);

                  }
               }
               // Java: else if (data.mVertical != 6 && data.mVertical != 3)

               {
                  // Java: this.drawHorizontalTextImage(workCanvas, layout, span, line, data, spanList, shadowData, preSpan, aftSpan);

               }
            }
            // Java: if (spanEnd <= lnEndOffset)

            {
               // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

               // Java: ++spanInx;

            }
            // Java: else

            {
               // Java: ++line;

               // Java: layout.getLineStart(line);

               // Java: lnEndOffset = layout.getLineEnd(line);

            }
         }
      }
    }

    // Java: void drawHorizontalTextImage(Canvas workCanvas, Layout layout, SpanInfo span, int line, TextData data, List<SingleImageSpannable> spanList, ShadowImageData shadowImage, CharacterStyle preTransSpan, CharacterStyle aftTransSpan)
    pub fn drawHorizontalTextImage(&self, _workCanvas: i32, _layout: i32, _span: i32, _line: i32, _data: i32, _spanList: i32, _shadowImage: i32, _preTransSpan: i32, _aftTransSpan: i32) {
      // Java: int lnStartOffset = layout.getLineStart(line);

      // Java: int lnEndOffset = layout.getLineEnd(line);

      // Java: int start = Math.max(lnStartOffset, span.mStartPos);

      // Java: int end = Math.min(lnEndOffset, span.mEndPos);

      // Java: float baseLine = layout.getLineBaseline(line);

      // Java: float lnHei = baseLine - layout.getLineTop(line);

      // Java: CharacterStyle charStyle = new WpsImageSpan(span.mTextImage, lnHei);

      // Java: Matrix matrix = new Matrix();

      // Java: matrix.preScale(1.0F, -1.0F);

      // Java: workCanvas.save();

      // Java: workCanvas.concat(matrix);

      // Java: if (line == 1)

      {
         // Java: workCanvas.translate(this.getLeadSpc(layout), 0.0F);

      }
      // Java: Rect clip = workCanvas.getClipBounds();

      // Java: --clip.left;

      // Java: ++clip.right;

      // Java: clip.bottom = (int)(-lnHei);

      // Java: workCanvas.clipRect(clip);

      // Java: float distance = lnHei / 18.0F * span.mTextImage.getDist();

      // Java: float offset = baseLine + lnHei + distance;

      // Java: workCanvas.translate(0.0F, -offset);

      // Java: for(int index = 0; index < spanList.size(); ++index)

      {
         // Java: SingleImageSpannable spannableData = (SingleImageSpannable)spanList.get(index);

         // Java: if (this.checkNeedDrawLayout(spannableData.getType(), span))

         {
            // Java: Spannable subSpan = spannableData.getSpan();

            // Java: subSpan.setSpan(charStyle, start, end, 34);

            // Java: Layout layoutTmp = this.mTypoLayoutGen.geneLayout(subSpan.subSequence(lnStartOffset, lnEndOffset), data, layout.getOriWidth(), layout.getAlignment(), layout.getPaint(), layout.getSpacingMultiplier());

            // Java: Spannable tempSpannable = (Spannable)layoutTmp.getText();

            // Java: if (this.hasDynamicDigit && layoutTmp instanceof WpsLayout && layout instanceof WpsLayout)

            {
               // Java: ((WpsLayout)layout).copyData((WpsLayout)layoutTmp, lnStartOffset, 0, tempSpannable.length());

            }
            // Java: if (start > lnStartOffset)

            {
               // Java: tempSpannable.setSpan(preTransSpan, 0, start - lnStartOffset, 34);

            }
            // Java: if (end < lnEndOffset)

            {
               // Java: tempSpannable.setSpan(aftTransSpan, end - lnStartOffset, lnEndOffset - lnStartOffset, 34);

            }
            // Java: if (spannableData.isOuterShadow() && shadowImage != null)

            {
               // Java: if (shadowImage.isDoubleShadow())

               {
                  // Java: workCanvas.save();

                  // Java: workCanvas.concat(matrix);

                  // Java: workCanvas.translate(shadowImage.getDoubleAfterTrans().x, shadowImage.getDoubleAfterTrans().y);

                  // Java: workCanvas.scale(shadowImage.getScale().x, shadowImage.getScale().y);

                  // Java: workCanvas.skew(shadowImage.getSkew().x, shadowImage.getSkew().y);

                  // Java: workCanvas.translate(shadowImage.getDoubleAfterTrans().x, shadowImage.getDoubleAfterTrans().y);

                  // Java: Spannable doubleSpannable = new SpannableStringBuilder(layoutTmp.getText());

                  // Java: this.drawDoubleShadowImage(span, WordArtLayout.SecondShadowType.FILL, doubleSpannable, layoutTmp, start, end, workCanvas, data);

                  // Java: workCanvas.restore();

               }
               // Java: workCanvas.save();

               // Java: workCanvas.translate(shadowImage.getFirstBeforeTrans().x, shadowImage.getFirstBeforeTrans().y);

               // Java: workCanvas.scale(shadowImage.getScale().x, shadowImage.getScale().y);

               // Java: workCanvas.skew(shadowImage.getSkew().x, shadowImage.getSkew().y);

               // Java: workCanvas.translate(shadowImage.getFirstAfterTrans().x, shadowImage.getFirstAfterTrans().y);

               // Java: layoutTmp.draw(workCanvas);

               // Java: workCanvas.restore();

            }
            // Java: else

            {
               // Java: layoutTmp.draw(workCanvas);

            }
            // Java: if (start > 0)

            {
               // Java: tempSpannable.removeSpan(preTransSpan);

            }
            // Java: if (end < tempSpannable.length())

            {
               // Java: tempSpannable.removeSpan(aftTransSpan);

            }
            // Java: subSpan.removeSpan(charStyle);

         }
      }
      // Java: workCanvas.restore();

    }

    // Java: void drawDoubleShadowImage(SpanInfo span, SecondShadowType type, Spannable shdwSpan, Layout layout, int start, int end, Canvas canvas, TextData data)
    pub fn drawDoubleShadowImage(&self, _span: i32, _type: i32, _shdwSpan: i32, _layout: i32, _start: i32, _end: i32, _canvas: i32, _data: i32) {
      // Java: int shdwAlpha = Color.alpha(span.mTextShadow.getShdwClr());

      // Java: int shdwClr = 0;

      // Java: if (type == WordArtLayout.SecondShadowType.FILL)

      {
         // Java: if (!span.hasFill())

         {
            // Java: return;

         }
         // Java: shdwClr = this.getAlphaColor(span.mTextShadow.getShdwClr(), (int)((float)shdwAlpha * this.getFillAlpha(span)));

      }
      // Java: else if (type == WordArtLayout.SecondShadowType.OUTLINE)

      {
         // Java: if (!span.hasOutline())

         {
            // Java: return;

         }
         // Java: shdwClr = this.getAlphaColor(span.mTextShadow.getShdwClr(), (int)((float)shdwAlpha * this.getOutlineAlpha(span)));

      }
      // Java: else if (type == WordArtLayout.SecondShadowType.BLUR)

      {
         // Java: if (!span.hasBlur())

         {
            // Java: return;

         }
         // Java: shdwClr = this.getAlphaColor(span.mTextShadow.getShdwClr(), (int)((float)shdwAlpha * span.mTextBlur.colorAlpha()));

      }
      // Java: WpsForegroundColorSpan[] styleSpans = (WpsForegroundColorSpan[])shdwSpan.getSpans(start, end, WpsForegroundColorSpan.class);

      // Java: for(WpsForegroundColorSpan styleSpan : styleSpans)

      {
         // Java: shdwSpan.removeSpan(styleSpan);

      }
      // Java: ForegroundColorSpan[] ClrStyleSpans = (ForegroundColorSpan[])shdwSpan.getSpans(start, end, ForegroundColorSpan.class);

      // Java: for(ForegroundColorSpan foregroundColorSpan : ClrStyleSpans)

      {
         // Java: shdwSpan.removeSpan(foregroundColorSpan);

      }
      // Java: int r = Math.min(255, Color.red(shdwClr) + 102);

      // Java: int g = Math.min(255, Color.green(shdwClr) + 102);

      // Java: int b = Math.min(255, Color.blue(shdwClr) + 102);

      // Java: int clr = Color.argb(Color.alpha(shdwClr), r, g, b);

      // Java: CharacterStyle charStyleTmp = new ForegroundColorSpan(clr);

      // Java: shdwSpan.setSpan(charStyleTmp, start, end, 34);

      // Java: Layout sLayout = this.mTypoLayoutGen.geneLayout(shdwSpan, data, layout.getOriWidth(), layout.getAlignment(), layout.getPaint(), layout.getSpacingMultiplier());

      // Java: sLayout.draw(canvas);

    }

    // Java: void drawVerticalTextImage(Canvas canvas, Layout layout, int layoutIndex, ChineseVertSpan vertSpan, SpanInfo curSpanInfo, List<SingleImageSpannable> spanList, ShadowImageData shadowImage, int spanIndex)
    pub fn drawVerticalTextImage(&self, _canvas: i32, _layout: i32, _layoutIndex: i32, _vertSpan: i32, _curSpanInfo: i32, _spanList: i32, _shadowImage: i32, _spanIndex: i32) {
      // Java: TextPaint paint = new TextPaint();

      // Java: int spanStart = Math.max(0, vertSpan.getStart());

      // Java: int spanEnd = Math.min(curSpanInfo.mEndPos, vertSpan.getEnd());

      // Java: this.initVertTextPaint(paint, vertSpan);

      // Java: WpsImageSpan style = new WpsImageSpan(curSpanInfo.mTextImage, true);

      // Java: for(SingleImageSpannable spannable : spanList)

      {
         // Java: if (this.checkNeedDrawLayout(spannable.getType(), curSpanInfo))

         {
            // Java: Spannable layoutSpan = new SpannableStringBuilder(spannable.getSpan());

            // Java: if (spannable.getType() == 1)

            {
               // Java: ForegroundColorSpan[] colorSpans = (ForegroundColorSpan[])layoutSpan.getSpans(spanStart, spanEnd, WpsForegroundColorSpan.class);

               // Java: if (colorSpans.length <= 0)

               {
                  // Java: colorSpans = (ForegroundColorSpan[])layoutSpan.getSpans(spanStart, spanEnd, ForegroundColorSpan.class);

               }
               // Java: if (colorSpans.length > 0)

               {
                  // Java: colorSpans[colorSpans.length - 1].updateDrawState(paint);

               }
               // Java: paint.setStyle(Style.FILL);

               // Java: this.drawChineseVertSpan(layout, layoutIndex, spanIndex, curSpanInfo, canvas, paint, style);

            }
            // Java: if (spannable.getType() == 2)

            {
               // Java: WpsTextOutlineSpan[] outlineSpans = (WpsTextOutlineSpan[])layoutSpan.getSpans(spanStart, spanEnd, WpsTextOutlineSpan.class);

               // Java: if (outlineSpans.length > 0)

               {
                  // Java: outlineSpans[outlineSpans.length - 1].updateDrawState(paint);

                  // Java: this.drawChineseVertSpan(layout, layoutIndex, spanIndex, curSpanInfo, canvas, paint, style);

               }
            }
            // Java: if (spannable.getType() == 3)

            {
               // Java: WpsTextBlurSpan[] blurSpans = (WpsTextBlurSpan[])layoutSpan.getSpans(spanStart, spanEnd, WpsTextBlurSpan.class);

               // Java: if (blurSpans.length > 0)

               {
                  // Java: blurSpans[blurSpans.length - 1].updateDrawState(paint);

                  // Java: this.drawChineseVertSpan(layout, layoutIndex, spanIndex, curSpanInfo, canvas, paint, style);

               }
            }
            // Java: if (spannable.getType() == 5)

            {
               // Java: WpsOuterShdwSpan[] outerShdwSpans = (WpsOuterShdwSpan[])layoutSpan.getSpans(spanStart, spanEnd, WpsOuterShdwSpan.class);

               // Java: if (outerShdwSpans.length > 0)

               {
                  // Java: outerShdwSpans[outerShdwSpans.length - 1].updateDrawState(paint);

                  // Java: canvas.save();

                  // Java: if (shadowImage != null)

                  {
                     // Java: canvas.translate(shadowImage.getFirstBeforeTrans().x, shadowImage.getFirstBeforeTrans().y);

                     // Java: canvas.scale(shadowImage.getScale().x, shadowImage.getScale().y);

                     // Java: canvas.skew(shadowImage.getSkew().x, shadowImage.getSkew().y);

                     // Java: canvas.translate(shadowImage.getFirstAfterTrans().x, shadowImage.getFirstAfterTrans().y);

                  }
                  // Java: this.drawChineseVertSpan(layout, layoutIndex, spanIndex, curSpanInfo, canvas, paint, style);

                  // Java: canvas.restore();

               }
            }
            // Java: if (spannable.getType() == 6)

            {
               // Java: WpsInnerShdwSpan[] innerSpans = (WpsInnerShdwSpan[])layoutSpan.getSpans(spanStart, spanEnd, WpsInnerShdwSpan.class);

               // Java: if (innerSpans.length > 0)

               {
                  // Java: this.drawChineseVertInnerSpan(layout, layoutIndex, curSpanInfo, canvas, paint, vertSpan);

               }
            }
         }
      }
    }

    // Java: void renderTxtImg_para(Canvas workCanvas, int para, List<Layout> typoCaches, List<PointF> typoOffsets)
    pub fn renderTxtImg_para(&self, _workCanvas: i32, _para: i32, _typoCaches: i32, _typoOffsets: i32) {
      // Java: TextData data = this.mTextBoxData.getTextData(para);

      // Java: List<Spannable> spannables = this.getImgSpans(data, para);

      // Java: CharacterStyle charStyle = null;

      // Java: int flag = 34;

      // Java: int start = 0;

      // Java: int end = 0;

      // Java: int spanInx = 0;

      // Java: int spanSize = data.mOtherSpans.size();

      // Java: int textLen = data.mSourceText.length();

      // Java: SpanInfo span = data.mFirstSpan;

      // Java: Layout layout = (Layout)typoCaches.get(para);

      // Java: workCanvas.save();

      // Java: Matrix matrix = new Matrix();

      // Java: matrix.preScale(1.0F, -1.0F);

      // Java: workCanvas.concat(matrix);

      // Java: CharacterStyle preTransSpan = WpsHideSpan.Factory.create();

      // Java: CharacterStyle aftTransSpan = WpsHideSpan.Factory.create();

      // Java: for(int line = 0; line < layout.getLineCount(); ++line)

      {
         // Java: int lnStartOffset = layout.getLineStart(line);

         // Java: int lnEndOffset = layout.getLineEnd(line);

         // Java: while(span != null && span.mEndPos <= textLen)

         {
            // Java: int spanStart = span.mStartPos;

            // Java: int spanEnd = span.mEndPos;

            // Java: start = Math.max(lnStartOffset, spanStart);

            // Java: end = Math.min(lnEndOffset, spanEnd);

            // Java: if (span.hasTextImage())

            {
               // Java: if (line == 1)

               {
                  // Java: workCanvas.translate(this.getLeadSpc(layout), 0.0F);

               }
               // Java: float lnHei = layout.getLineBaseline(line) - layout.getLineTop(line);

               // Java: CharacterStyle var30 = new WpsImageSpan(span.mTextImage, lnHei);

               // Java: workCanvas.save();

               // Java: Rect clip = workCanvas.getClipBounds();

               // Java: --clip.left;

               // Java: ++clip.right;

               // Java: clip.bottom = (int)(-lnHei);

               // Java: workCanvas.clipRect(clip);

               // Java: float offset = ((Layout)typoCaches.get(para)).getLineBaseline(line) + lnHei;

               // Java: workCanvas.translate(0.0F, -offset);

               // Java: float distance = lnHei / 18.0F * span.mTextImage.getDist();

               // Java: workCanvas.translate(0.0F, -distance);

               // Java: for(int index = 0; index < spannables.size(); ++index)

               {
                  // Java: ((Spannable)spannables.get(index)).setSpan(var30, start, end, flag);

                  // Java: Layout layoutTmp = this.mTypoLayoutGen.geneLayout(((Spannable)spannables.get(index)).subSequence(lnStartOffset, lnEndOffset), data, layout.getOriWidth(), layout.getAlignment(), layout.getPaint(), layout.getSpacingMultiplier());

                  // Java: if (start > lnStartOffset)

                  {
                     // Java: ((Spannable)layoutTmp.getText()).setSpan(preTransSpan, 0, start - lnStartOffset, flag);

                  }
                  // Java: if (end < lnEndOffset)

                  {
                     // Java: ((Spannable)layoutTmp.getText()).setSpan(aftTransSpan, end - lnStartOffset, lnEndOffset - lnStartOffset, flag);

                  }
                  // Java: layoutTmp.draw(workCanvas);

                  // Java: if (start > lnStartOffset)

                  {
                     // Java: ((Spannable)layoutTmp.getText()).removeSpan(preTransSpan);

                  }
                  // Java: if (end < lnEndOffset)

                  {
                     // Java: ((Spannable)layoutTmp.getText()).removeSpan(aftTransSpan);

                  }
                  // Java: ((Spannable)spannables.get(index)).removeSpan(var30);

               }
               // Java: workCanvas.restore();

            }
            // Java: if (spanEnd <= lnEndOffset)

            {
               // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

               // Java: ++spanInx;

            }
            // Java: else

            {
               // Java: ++line;

               // Java: lnStartOffset = layout.getLineStart(line);

               // Java: lnEndOffset = layout.getLineEnd(line);

            }
         }
      }
      // Java: workCanvas.restore();

    }

    // Java: float getLeadSpc(Layout layout)
    pub fn getLeadSpc(&self, _layout: i32) -> f32 {
      // Java: LeadingMarginSpan.Standard[] paraSpans = (LeadingMarginSpan.Standard[])((Spannable)layout.getText()).getSpans(0, layout.getText().length(), LeadingMarginSpan.Standard.class);

      // Java: int firstLead = 0;

      // Java: int restLead = 0;

      // Java: if (paraSpans.length > 0)

      {
         // Java: firstLead = paraSpans[0].getLeadingMargin(true);

         // Java: restLead = paraSpans[0].getLeadingMargin(false);

      }
      // Java: return (float)(restLead - firstLead);

      0.0
    }

    // Java: int renderOuterShdw_para(Canvas workCanvas, int para, List<Layout> typoCaches, int shdwSpanIdx)
    pub fn renderOuterShdw_para(&self, _workCanvas: i32, _para: i32, _typoCaches: i32, _shdwSpanIdx: i32) -> i32 {
      // Java: TextData data = this.mTextBoxData.getTextData(para);

      // Java: List<Spannable> firstSpans = new ArrayList();

      // Java: List<Layout> firstLys = new ArrayList();

      // Java: ShadowLayoutGroup layoutData = (ShadowLayoutGroup)this.mShadowLayoutCaches.get(para);

      // Java: Layout shdwLayout = layoutData.mFillShadowLayout;

      // Java: firstSpans.add((Spannable)shdwLayout.getText());

      // Java: firstLys.add(shdwLayout);

      // Java: Layout outlnShdwLayout = layoutData.mOutlineShadowLayout;

      // Java: firstSpans.add((Spannable)outlnShdwLayout.getText());

      // Java: firstLys.add(outlnShdwLayout);

      // Java: Layout blurShdwLayout = layoutData.mBlurShadowLayout;

      // Java: firstSpans.add((Spannable)blurShdwLayout.getText());

      // Java: firstLys.add(blurShdwLayout);

      // Java: List<Spannable> secondSpans = new ArrayList();

      // Java: SecondShadowType[] types = new SecondShadowType[3];

      // Java: Spannable shdwSpan2 = new SpannableStringBuilder(shdwLayout.getText());

      // Java: secondSpans.add(shdwSpan2);

      // Java: types[0] = WordArtLayout.SecondShadowType.FILL;

      // Java: Spannable outlnSpan2 = new SpannableStringBuilder(outlnShdwLayout.getText());

      // Java: secondSpans.add(outlnSpan2);

      // Java: types[1] = WordArtLayout.SecondShadowType.OUTLINE;

      // Java: Spannable blurSpan2 = new SpannableStringBuilder(blurShdwLayout.getText());

      // Java: secondSpans.add(blurSpan2);

      // Java: types[2] = WordArtLayout.SecondShadowType.BLUR;

      // Java: List<ShadowLayoutData> shadowTypoData = layoutData.getTypoData();

      // Java: CharacterStyle preTransSpan = WpsHideSpan.Factory.create();

      // Java: CharacterStyle aftTransSpan = WpsHideSpan.Factory.create();

      // Java: Layout layout = (Layout)typoCaches.get(para);

      // Java: int start = 0;

      // Java: int end = 0;

      // Java: int spanInx = 0;

      // Java: int spanSize = data.mOtherSpans.size();

      // Java: int textLen = layout.getText().length();

      // Java: SpanInfo span = data.mFirstSpan;

      // Java: for(int k = 0; k < layout.getLineCount(); ++k)

      {
         // Java: int lnStartOffset = layout.getLineStart(k);

         // Java: int lnEndOffset = layout.getLineEnd(k);

         // Java: while(span != null && span.mEndPos <= textLen)

         {
            // Java: int spanStart = span.mStartPos;

            // Java: int spanEnd = span.mEndPos;

            // Java: start = Math.max(lnStartOffset, spanStart);

            // Java: end = Math.min(lnEndOffset, spanEnd);

            // Java: if (span.hasOuterShdw())

            {
               // Java: if (start > 0)

               {
                  // Java: this.setSpan(firstSpans, 0, start, preTransSpan);

                  // Java: this.setSpan(secondSpans, 0, start, preTransSpan);

               }
               // Java: if (end < textLen)

               {
                  // Java: this.setSpan(firstSpans, end, textLen, aftTransSpan);

                  // Java: this.setSpan(secondSpans, end, textLen, aftTransSpan);

               }
               // Java: ShadowTypoData shadowInfo = null;

               // Java: if (this.mSpanShadowCaches != null)

               {
                  // Java: Map<Integer, ShadowTypoData> typoDataMap = (Map)this.mSpanShadowCaches.get(para);

                  // Java: if (typoDataMap != null && typoDataMap.size() > shdwSpanIdx)

                  {
                     // Java: shadowInfo = (ShadowTypoData)typoDataMap.get(shdwSpanIdx);

                  }
               }
               // Java: if (shadowInfo == null)

               {
                  // Java: shadowInfo = new ShadowTypoData();

                  // Java: shadowInfo.setDefault();

               }
               // Java: TextShadow shadow = span.mTextShadow;

               // Java: PointF scale = shadowInfo.mShadowScale;

               // Java: PointF skew = shadowInfo.mShadowSkew;

               // Java: if (this.mArtTransform != null && shadow.getIsDoc())

               {
                  // Java: skew.x = shadow.getShdwXSkew();

                  // Java: skew.y = shadow.getShdwYSkew();

               }
               // Java: float lnHei = layout.getLineBaseline(k) - layout.getLineTop(k);

               // Java: RectF rect = new RectF(0.0F, 0.0F, layout.getLineRight(k) - layout.getLineLeft(k), lnHei * 1.18F);

               // Java: PointF offset = this.getShadowOffset(shadowInfo.mShadowAlign, rect);

               // Java: offset.x += layout.getLineLeft(k);

               // Java: float startX = 0.0F;

               // Java: float startY = layout.getLineTop(k);

               // Java: float dis = shadowInfo.mShadowDistance;

               // Java: float angle = shadowInfo.mShadowAngle;

               // Java: float disScale = this.getOuterShadowDisScale(span.mFontSize);

               // Java: float x = (float)((double)dis * Math.cos(Math.toRadians((double)angle))) * disScale;

               // Java: float y = (float)((double)dis * Math.sin(Math.toRadians((double)angle))) * disScale;

               // Java: int prstType = shadow.getPrstShdwType();

               // Java: if (this.isDouble(prstType, shadow))

               {
                  // Java: this.drawShadowSecond(span, workCanvas, new PointF(startX, startY), offset, new PointF(x, y), prstType, scale, skew, para, new Point(start, end), shadowTypoData);

               }
               // Java: workCanvas.save();

               // Java: if (this.mArtTransform != null)

               {
                  // Java: if (!shadow.getIsDoc())

                  {
                     // Java: offset = this.getShadowOffset(shadowInfo.mShadowAlign, new RectF(0.0F, 0.0F, this.mArtTransform.getTextWidth(), this.mArtTransform.getTextHeight()));

                     // Java: workCanvas.translate(-((PointF)this.mTypoOffsets.get(para)).x, -((PointF)this.mTypoOffsets.get(para)).y);

                     // Java: this.drawFirstShadowArtTrans(workCanvas, new PointF(startX, startY), offset, new PointF(x, y), scale, skew, firstLys, this.mTypoOffsets, para);

                     // Java: workCanvas.translate(((PointF)this.mTypoOffsets.get(para)).x, ((PointF)this.mTypoOffsets.get(para)).y);

                  }
                  // Java: else

                  {
                     // Java: workCanvas.save();

                     // Java: workCanvas.translate(offset.x + startX, offset.y + startY);

                     // Java: workCanvas.translate(shadow.getShdwOffsetX() * disScale, shadow.getShdwOffsetY() * disScale);

                     // Java: workCanvas.scale(scale.x, scale.y);

                     // Java: workCanvas.skew(skew.x, skew.y);

                     // Java: workCanvas.translate(-(offset.x + startX), -(offset.y + startY));

                     // Java: for(int i = 0; i < firstLys.size(); ++i)

                     {
                        // Java: ((Layout)firstLys.get(i)).draw(workCanvas);

                     }
                     // Java: workCanvas.restore();

                  }
               }
               // Java: else

               {
                  // Java: this.drawFirstShadow(workCanvas, new PointF(startX, startY), new PointF(offset.x, offset.y), new PointF(x, y), scale, skew, firstLys, para);

               }
               // Java: workCanvas.restore();

               // Java: if (span.hasTextImage())

               {
                  // Java: Matrix matrix = new Matrix();

                  // Java: matrix.preScale(1.0F, -1.0F);

                  // Java: float distance = lnHei / 18.0F * span.mTextImage.getDist();

                  // Java: CharacterStyle charStyle = new WpsImageSpan(span.mTextImage, lnHei);

                  // Java: if (this.isDouble(prstType, shadow))

                  {
                     // Java: workCanvas.save();

                     // Java: workCanvas.concat(matrix);

                     // Java: workCanvas.translate(0.0F, -(((Layout)typoCaches.get(para)).getLineBaseline(k) + lnHei));

                     // Java: workCanvas.translate(0.0F, -distance);

                     // Java: workCanvas.translate(offset.x, offset.y);

                     // Java: if (this.isType13(prstType, shadow))

                     {
                        // Java: workCanvas.translate(2.0F * x, 2.0F * y);

                     }
                     // Java: else

                     {
                        // Java: workCanvas.translate(-x, -y);

                     }
                     // Java: workCanvas.scale(scale.x, scale.y);

                     // Java: workCanvas.skew(skew.x, skew.y);

                     // Java: workCanvas.translate(-offset.x, -offset.y);

                     // Java: this.setSpan(secondSpans, start, end, charStyle);

                     // Java: this.drawSecondShadowImg(span, WordArtLayout.SecondShadowType.BLUR, blurSpan2, blurShdwLayout, start, end, lnStartOffset, lnEndOffset, workCanvas, data);

                     // Java: this.drawSecondShadowImg(span, WordArtLayout.SecondShadowType.FILL, shdwSpan2, shdwLayout, start, end, lnStartOffset, lnEndOffset, workCanvas, data);

                     // Java: this.drawSecondShadowImg(span, WordArtLayout.SecondShadowType.OUTLINE, outlnSpan2, outlnShdwLayout, start, end, lnStartOffset, lnEndOffset, workCanvas, data);

                     // Java: this.removeSpan(secondSpans, charStyle);

                     // Java: workCanvas.restore();

                  }
                  // Java: workCanvas.save();

                  // Java: Rect clip = workCanvas.getClipBounds();

                  // Java: --clip.left;

                  // Java: ++clip.right;

                  // Java: clip.bottom = (int)(-lnHei);

                  // Java: workCanvas.clipRect(clip);

                  // Java: workCanvas.concat(matrix);

                  // Java: workCanvas.translate(0.0F, -(((Layout)typoCaches.get(para)).getLineBaseline(k) + lnHei));

                  // Java: workCanvas.translate(0.0F, -distance);

                  // Java: workCanvas.translate(shadow.getShdwOffsetX() + offset.x, shadow.getShdwOffsetY() + offset.y);

                  // Java: workCanvas.translate(x, y);

                  // Java: workCanvas.scale(scale.x, scale.y);

                  // Java: workCanvas.skew(skew.x, skew.y);

                  // Java: workCanvas.translate(-offset.x, -offset.y);

                  // Java: this.setSpan(firstSpans, start, end, charStyle);

                  // Java: this.drawFirstShadowImg(workCanvas, firstLys, start, end, lnStartOffset, lnEndOffset, data);

                  // Java: this.removeSpan(firstSpans, charStyle);

                  // Java: workCanvas.restore();

               }
               // Java: if (start > 0)

               {
                  // Java: this.removeSpan(firstSpans, preTransSpan);

                  // Java: this.removeSpan(secondSpans, preTransSpan);

               }
               // Java: if (end < textLen)

               {
                  // Java: this.removeSpan(firstSpans, aftTransSpan);

                  // Java: this.removeSpan(secondSpans, aftTransSpan);

               }
            }
            // Java: if (spanEnd <= lnEndOffset)

            {
               // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

               // Java: ++spanInx;

               // Java: ++shdwSpanIdx;

            }
            // Java: else

            {
               // Java: ++k;

               // Java: lnStartOffset = layout.getLineStart(k);

               // Java: lnEndOffset = layout.getLineEnd(k);

            }
         }
      }
      // Java: return shdwSpanIdx;

      0
    }

    // Java: int renderOuterShadow(Canvas workCanvas, int para, List<Layout> typoCaches, int shadowSpanIdx)
    pub fn renderOuterShadow(&self, _workCanvas: i32, _para: i32, _typoCaches: i32, _shadowSpanIdx: i32) -> i32 {
      // Java: TextData data = this.mTextBoxData.getTextData(para);

      // Java: ShadowLayoutGroup layoutData = (ShadowLayoutGroup)this.mShadowLayoutCaches.get(para);

      // Java: List<Spannable> firstSpans = layoutData.getSpanList(true);

      // Java: List<Spannable> secondSpans = layoutData.getSpanList(false);

      // Java: List<ShadowLayoutData> shadowTypoData = layoutData.getTypoData();

      // Java: CharacterStyle preTransSpan = WpsHideSpan.Factory.create();

      // Java: CharacterStyle aftTransSpan = WpsHideSpan.Factory.create();

      // Java: Layout layout = (Layout)typoCaches.get(para);

      // Java: int spanInx = 0;

      // Java: int spanSize = data.mOtherSpans.size();

      // Java: int textLen = layout.getText().length();

      // Java: SpanInfo span = data.mFirstSpan;

      // Java: for(int k = 0; k < layout.getLineCount(); ++k)

      {
         // Java: int lnStartOffset = layout.getLineStart(k);

         // Java: int lnEndOffset = layout.getLineEnd(k);

         // Java: while(span != null && span.mEndPos <= textLen)

         {
            // Java: int spanStart = span.mStartPos;

            // Java: int spanEnd = span.mEndPos;

            // Java: int start = Math.max(lnStartOffset, spanStart);

            // Java: int end = Math.min(lnEndOffset, spanEnd);

            // Java: if (span.hasOuterShdw())

            {
               // Java: if (start > 0)

               {
                  // Java: this.setSpan(firstSpans, 0, start, preTransSpan);

                  // Java: this.setSpan(secondSpans, 0, start, preTransSpan);

               }
               // Java: if (end < textLen)

               {
                  // Java: this.setSpan(firstSpans, end, textLen, aftTransSpan);

                  // Java: this.setSpan(secondSpans, end, textLen, aftTransSpan);

               }
               // Java: TextShadow shadow = span.mTextShadow;

               // Java: ShadowTypoData shadowInfo = null;

               // Java: if (this.mSpanShadowCaches != null)

               {
                  // Java: Map<Integer, ShadowTypoData> typoDataMap = (Map)this.mSpanShadowCaches.get(para);

                  // Java: if (typoDataMap != null)

                  {
                     // Java: shadowInfo = (ShadowTypoData)typoDataMap.get(spanInx);

                  }
               }
               // Java: if (shadowInfo == null)

               {
                  // Java: shadowInfo = new ShadowTypoData();

                  // Java: shadowInfo.setDefault();

               }
               // Java: PointF scale = shadowInfo.mShadowScale;

               // Java: PointF skew = shadowInfo.mShadowSkew;

               // Java: if (this.mArtTransform != null && shadow.getIsDoc())

               {
                  // Java: skew.x = shadow.getShdwXSkew();

                  // Java: skew.y = shadow.getShdwYSkew();

               }
               // Java: ShadowRenderData renderData = this.calRenderData(shadow, shadowInfo, (PointF)this.mTypoOffsets.get(para), layout, k, span.mFontSize);

               // Java: int prstType = shadow.getPrstShdwType();

               // Java: if (this.isDouble(prstType, shadow))

               {
                  // Java: this.drawShadowSecond(span, workCanvas, renderData.getStart(), renderData.getOffset(), renderData.getDistance(), prstType, scale, skew, para, new Point(start, end), shadowTypoData);

               }
               // Java: PointF beforeTrans = renderData.getBeforeTrans();

               // Java: PointF afterTrans = renderData.getAfterTrans();

               // Java: workCanvas.save();

               // Java: if (this.mArtTransform != null && !shadow.getIsDoc())

               {
                  // Java: workCanvas.translate(-((PointF)this.mTypoOffsets.get(para)).x, -((PointF)this.mTypoOffsets.get(para)).y);

                  // Java: this.drawFirstShadow(workCanvas, beforeTrans, scale, skew, afterTrans, layoutData, para, start, end);

                  // Java: workCanvas.translate(((PointF)this.mTypoOffsets.get(para)).x, ((PointF)this.mTypoOffsets.get(para)).y);

               }
               // Java: else

               {
                  // Java: this.drawFirstShadow(workCanvas, beforeTrans, scale, skew, afterTrans, layoutData, para, start, end);

               }
               // Java: workCanvas.restore();

               // Java: if (start > 0)

               {
                  // Java: this.removeSpan(firstSpans, preTransSpan);

                  // Java: this.removeSpan(secondSpans, preTransSpan);

               }
               // Java: if (end < textLen)

               {
                  // Java: this.removeSpan(firstSpans, aftTransSpan);

                  // Java: this.removeSpan(secondSpans, aftTransSpan);

               }
            }
            // Java: if (spanEnd <= lnEndOffset)

            {
               // Java: span = spanInx < spanSize ? (SpanInfo)data.mOtherSpans.get(spanInx) : null;

               // Java: ++spanInx;

               // Java: ++shadowSpanIdx;

            }
            // Java: else

            {
               // Java: ++k;

               // Java: lnStartOffset = layout.getLineStart(k);

               // Java: lnEndOffset = layout.getLineEnd(k);

            }
         }
      }
      // Java: return shadowSpanIdx;

      0
    }

    // Java: ShadowRenderData calRenderData(TextShadow shadow, ShadowTypoData shadowInfo, PointF typoOffset, Layout layout, int line, float textSize)
    pub fn calRenderData(&self, _shadow: i32, _shadowInfo: i32, _typoOffset: i32, _layout: i32, _line: i32, _textSize: f32) -> i32 {
      // Java: ShadowRenderData renderData = new ShadowRenderData();

      // Java: float left = layout.getLineLeft(line);

      // Java: float top = layout.getLineTop(line);

      // Java: float right = layout.getLineRight(line);

      // Java: float baseLine = layout.getLineBaseline(line);

      // Java: RectF rect = new RectF(0.0F, 0.0F, right - left, (baseLine - top) * 1.18F);

      // Java: PointF offset = this.getShadowOffset(shadowInfo.mShadowAlign, rect);

      // Java: offset.x += left;

      // Java: float startX = 0.0F;

      // Java: float dis = shadowInfo.mShadowDistance;

      // Java: float angle = shadowInfo.mShadowAngle;

      // Java: float disScale = this.getOuterShadowDisScale(textSize);

      // Java: float x = (float)((double)dis * Math.cos(Math.toRadians((double)angle))) * disScale;

      // Java: float y = (float)((double)dis * Math.sin(Math.toRadians((double)angle))) * disScale;

      // Java: PointF beforeTrans = new PointF();

      // Java: PointF afterTrans = new PointF();

      // Java: if (this.mArtTransform != null)

      {
         // Java: if (!shadow.getIsDoc())

         {
            // Java: RectF r = new RectF(0.0F, 0.0F, this.mArtTransform.getTextWidth(), this.mArtTransform.getTextHeight());

            // Java: offset = this.getShadowOffset(shadowInfo.mShadowAlign, r);

            // Java: beforeTrans.x = x + offset.x;

            // Java: beforeTrans.y = y + offset.y;

            // Java: afterTrans.x = -offset.x + startX + typoOffset.x;

            // Java: afterTrans.y = -offset.y + top + typoOffset.y;

         }
         // Java: else

         {
            // Java: beforeTrans.x = startX + offset.x + shadow.getShdwOffsetX() * disScale;

            // Java: beforeTrans.y = top + offset.y + shadow.getShdwOffsetY() * disScale;

            // Java: afterTrans.x = -(startX + offset.x);

            // Java: afterTrans.y = -(top + offset.y);

         }
      }
      // Java: else

      {
         // Java: beforeTrans.x = startX + offset.x + x;

         // Java: beforeTrans.y = top + offset.y + y;

         // Java: afterTrans.x = -(startX + offset.x);

         // Java: afterTrans.y = -(top + offset.y);

      }
      // Java: renderData.setScale(shadowInfo.mShadowScale);

      // Java: renderData.setSkew(shadowInfo.mShadowSkew);

      // Java: renderData.setLineBase(baseLine);

      // Java: renderData.setLineHeight(baseLine - top);

      // Java: renderData.setStart(new PointF(0.0F, top));

      // Java: renderData.setOffset(offset);

      // Java: renderData.setDistance(new PointF(x, y));

      // Java: renderData.setBeforeTrans(beforeTrans);

      // Java: renderData.setAfterTrans(afterTrans);

      // Java: return renderData;

      0
    }

    // Java: ShadowImageData getShadowImageData(SpanInfo span, ShadowTypoData shadowInfo, ShadowRenderData renderData)
    pub fn getShadowImageData(&self, _span: i32, _shadowInfo: i32, _renderData: i32) -> i32 {
      // Java: ShadowImageData shadowData = new ShadowImageData();

      // Java: shadowData.setShadow(span.mTextShadow);

      // Java: shadowData.setScale(shadowInfo.mShadowScale);

      // Java: shadowData.setSkew(shadowInfo.mShadowSkew);

      // Java: boolean isDoubleShadow = false;

      // Java: if (span.mTextShadow != null)

      {
         // Java: isDoubleShadow = this.isDouble(span.mTextShadow.getPrstShdwType(), span.mTextShadow);

         // Java: shadowData.setDoubleShadow(isDoubleShadow);

      }
      // Java: if (isDoubleShadow)

      {
         // Java: boolean isType13 = this.isType13(span.mTextShadow.getPrstShdwType(), span.mTextShadow);

         // Java: float deltaX = isType13 ? 2.0F * renderData.getDistance().x : -renderData.getDistance().x;

         // Java: float deltaY = isType13 ? 2.0F * renderData.getDistance().y : -renderData.getDistance().y;

         // Java: float transX = renderData.getOffset().x + deltaX;

         // Java: float TransY = renderData.getOffset().y + deltaY;

         // Java: PointF bTrans = new PointF(transX, TransY);

         // Java: transX = -renderData.getOffset().x;

         // Java: TransY = -renderData.getOffset().y;

         // Java: PointF aTrans = new PointF(transX, TransY);

         // Java: shadowData.setDoubleBeforeTrans(bTrans);

         // Java: shadowData.setDoubleAfterTrans(aTrans);

      }
      // Java: PointF bTrans = new PointF();

      // Java: PointF aTrans = new PointF();

      // Java: bTrans.x = span.mTextShadow.getShdwOffsetX() + renderData.getOffset().x + renderData.getDistance().x;

      // Java: bTrans.y = span.mTextShadow.getShdwOffsetY() + renderData.getOffset().y - renderData.getDistance().y;

      // Java: aTrans.x = -renderData.getOffset().x;

      // Java: aTrans.y = -renderData.getOffset().y;

      // Java: shadowData.setFirstBeforeTrans(bTrans);

      // Java: shadowData.setFirstAfterTrans(aTrans);

      // Java: return shadowData;

      0
    }

    // Java: void drawShadowSecond(SpanInfo span, Canvas canvas, PointF start, PointF offset, PointF dis, int prstType, PointF scale, PointF skew, int para, Point interval, List<ShadowLayoutData> secondTypoData)
    pub fn drawShadowSecond(&self, _span: i32, _canvas: i32, _start: i32, _offset: i32, _dis: i32, _prstType: i32, _scale: i32, _skew: i32, _para: i32, _interval: i32, _secondTypoData: i32) {
      // Java: TextShadow shadow = span.mTextShadow;

      // Java: TextData data = this.mTextBoxData.getTextData(para);

      // Java: canvas.save();

      // Java: if (!shadow.getIsDoc())

      {
         // Java: canvas.translate(start.x + offset.x, start.y + offset.y);

         // Java: if (this.isType13(prstType, shadow))

         {
            // Java: canvas.translate(2.0F * dis.x, 2.0F * dis.y);

         }
         // Java: else

         {
            // Java: canvas.translate(-dis.x, -dis.y);

         }
         // Java: canvas.scale(scale.x, scale.y);

         // Java: canvas.skew(skew.x, skew.y);

         // Java: canvas.translate(-(start.x + offset.x), -(start.y + offset.y));

      }
      // Java: else

      {
         // Java: canvas.translate(-((PointF)this.mTypoOffsets.get(para)).x, -((PointF)this.mTypoOffsets.get(para)).y);

         // Java: offset = this.getShadowOffsetDoc(shadow.getOrigin(), new RectF(0.0F, 0.0F, this.mArtTransform.getTextWidth(), this.mArtTransform.getTextHeight()));

         // Java: if (this.isType13(prstType, shadow))

         {
            // Java: canvas.translate(2.0F * shadow.getShdwOffsetX() + offset.x, 2.0F * shadow.getShdwOffsetY() + offset.y);

         }
         // Java: else

         {
            // Java: canvas.translate(-shadow.getShdwOffsetX() + offset.x, -shadow.getShdwOffsetY() + offset.y);

         }
         // Java: canvas.scale(scale.x, scale.y);

         // Java: canvas.skew(skew.x, skew.y);

         // Java: canvas.translate(-offset.x, -offset.y);

         // Java: canvas.translate(((PointF)this.mTypoOffsets.get(para)).x + start.x, ((PointF)this.mTypoOffsets.get(para)).y + start.y);

      }
      // Java: for(ShadowLayoutData typoData : secondTypoData)

      {
         // Java: this.drawSecondShadow(span, typoData, interval.x, interval.y, canvas, data);

      }
      // Java: canvas.restore();

    }

    // Java: boolean isDouble(int prstType, TextShadow shadow)
    pub fn isDouble(&self, _prstType: i32, _shadow: i32) -> bool {
      // Java: boolean isDoubleInWriter = shadow.getIsDoc() && this.mArtTransform != null && (prstType == 1 || prstType == 5);

      // Java: boolean isDoubleInPresentation = !shadow.getIsDoc() && (prstType == 13 || prstType == 17 || prstType == 18);

      // Java: return isDoubleInPresentation || isDoubleInWriter;

      false
    }

    // Java: boolean isType13(int prstType, TextShadow shadow)
    pub fn isType13(&self, _prstType: i32, _shadow: i32) -> bool {
      // Java: boolean isType13InWriter = shadow.getIsDoc() && this.mArtTransform != null && prstType == 1;

      // Java: boolean isType13InPresentation = !shadow.getIsDoc() && prstType == 13;

      // Java: return isType13InWriter || isType13InPresentation;

      false
    }

    // Java: float getOuterShadowDisScale(float fontsize)
    pub fn getOuterShadowDisScale(&self, _fontsize: f32) -> f32 {
      // Java: float scale = 1.0F;

      // Java: float f = fontsize / 10.0F;

      // Java: if (f <= 5.0F)

      {
         // Java: scale = 0.1243F * f + 0.1394F;

      }
      // Java: else if (f > 5.0F && f <= 15.0F)

      {
         // Java: scale = 0.0836F * f + 0.3611F;

      }
      // Java: else if (f > 15.0F)

      {
         // Java: scale = 0.0652F * f + 0.6435F;

      }
      // Java: return scale;

      0.0
    }

    // Java: float getOuterShadowRadScale(float fontsize)
    pub fn getOuterShadowRadScale(&self, _fontsize: f32) -> f32 {
      // Java: float scale = 1.0F;

      // Java: float f = fontsize / 10.0F;

      // Java: if (f <= 5.0F)

      {
         // Java: scale = 0.1243F * f + 0.0394F;

      }
      // Java: else if (f > 5.0F && f <= 15.0F)

      {
         // Java: scale = 0.0836F * f + 0.23F;

      }
      // Java: else if (f > 15.0F)

      {
         // Java: scale = 0.0652F * f + 0.5035F;

      }
      // Java: return scale;

      0.0
    }

    // Java: void setSpan(List<Spannable> spannables, int start, int end, CharacterStyle charStyle)
    pub fn setSpan(&self, _spannables: i32, _start: i32, _end: i32, _charStyle: i32) {
      // Java: int flag = 34;

      // Java: for(int i = 0; i < spannables.size(); ++i)

      {
         // Java: ((Spannable)spannables.get(i)).setSpan(charStyle, start, end, flag);

      }
    }

    // Java: void removeSpan(List<Spannable> spannables, CharacterStyle charStyle)
    pub fn removeSpan(&self, _spannables: i32, _charStyle: i32) {
      // Java: for(int i = 0; i < spannables.size(); ++i)

      {
         // Java: ((Spannable)spannables.get(i)).removeSpan(charStyle);

      }
    }

    // Java: void drawFirstShadowArtTrans(Canvas canvas, PointF start, PointF offset, PointF dis, PointF scale, PointF skew, List<Layout> layouts, List<PointF> mTypoOffsets, int para)
    pub fn drawFirstShadowArtTrans(&self, _canvas: i32, _start: i32, _offset: i32, _dis: i32, _scale: i32, _skew: i32, _layouts: i32, _mTypoOffsets: i32, _para: i32) {
      // Java: canvas.save();

      // Java: canvas.translate(offset.x, offset.y);

      // Java: canvas.translate(dis.x, dis.y);

      // Java: canvas.scale(scale.x, scale.y);

      // Java: canvas.skew(skew.x, skew.y);

      // Java: canvas.translate(-offset.x, -offset.y);

      // Java: canvas.translate(((PointF)mTypoOffsets.get(para)).x + start.x, ((PointF)mTypoOffsets.get(para)).y + start.y);

      // Java: for(int i = 0; i < layouts.size(); ++i)

      {
         // Java: ((Layout)layouts.get(i)).draw(canvas);

      }
      // Java: canvas.restore();

    }

    // Java: void drawFirstShadow(Canvas canvas, PointF start, PointF offset, PointF dis, PointF scale, PointF skew, List<Layout> layouts, int para)
    pub fn drawFirstShadow(&self, _canvas: i32, _start: i32, _offset: i32, _dis: i32, _scale: i32, _skew: i32, _layouts: i32, _para: i32) {
      // Java: canvas.save();

      // Java: canvas.translate(offset.x + start.x, offset.y + start.y);

      // Java: canvas.translate(dis.x, dis.y);

      // Java: canvas.scale(scale.x, scale.y);

      // Java: canvas.skew(skew.x, skew.y);

      // Java: canvas.translate(-(offset.x + start.x), -(offset.y + start.y));

      // Java: for(int i = 0; i < layouts.size(); ++i)

      {
         // Java: ((Layout)layouts.get(i)).draw(canvas);

      }
      // Java: this.renderChineseVertText((Layout)layouts.get(0), para, canvas, Integer.MIN_VALUE, Integer.MAX_VALUE, 4);

      // Java: canvas.restore();

    }

    // Java: void drawFirstShadow(Canvas canvas, PointF beforeTrans, PointF scale, PointF skew, PointF afterTrans, ShadowLayoutGroup layoutGroup, int para, int start, int end)
    pub fn drawFirstShadow_2(&self, _canvas: i32, _beforeTrans: i32, _scale: i32, _skew: i32, _afterTrans: i32, _layoutGroup: i32, _para: i32, _start: i32, _end: i32) {
      // Java: if (layoutGroup != null && !layoutGroup.isEmpty())

      {
         // Java: canvas.save();

         // Java: canvas.translate(beforeTrans.x, beforeTrans.y);

         // Java: canvas.scale(scale.x, scale.y);

         // Java: canvas.skew(skew.x, skew.y);

         // Java: canvas.translate(afterTrans.x, afterTrans.y);

         // Java: if (layoutGroup.mFillShadowLayout != null)

         {
            // Java: layoutGroup.mFillShadowLayout.draw(canvas);

            // Java: this.renderChineseVertText(layoutGroup.mFillShadowLayout, para, canvas, start, end, 4);

         }
         // Java: if (layoutGroup.mOutlineShadowLayout != null)

         {
            // Java: layoutGroup.mOutlineShadowLayout.draw(canvas);

            // Java: this.renderChineseVertText(layoutGroup.mOutlineShadowLayout, para, canvas, start, end, 2);

         }
         // Java: if (layoutGroup.mBlurShadowLayout != null)

         {
            // Java: layoutGroup.mBlurShadowLayout.draw(canvas);

         }
         // Java: canvas.restore();

      }
    }

    // Java: void drawFirstShadowImg(Canvas canvas, List<Layout> firstLys, int start, int end, int lnStart, int lnEnd, TextData data)
    pub fn drawFirstShadowImg(&self, _canvas: i32, _firstLys: i32, _start: i32, _end: i32, _lnStart: i32, _lnEnd: i32, _data: i32) {
      // Java: CharacterStyle preTrans = WpsHideSpan.Factory.create();

      // Java: CharacterStyle aftTrans = WpsHideSpan.Factory.create();

      // Java: for(int i = 0; i < firstLys.size(); ++i)

      {
         // Java: Layout layoutOrg = (Layout)firstLys.get(i);

         // Java: Layout layout = this.mTypoLayoutGen.geneLayout(layoutOrg.getText().subSequence(lnStart, lnEnd), data, layoutOrg.getOriWidth(), layoutOrg.getAlignment(), layoutOrg.getPaint(), layoutOrg.getSpacingMultiplier());

         // Java: if (start > lnStart)

         {
            // Java: ((Spannable)layout.getText()).setSpan(preTrans, 0, start - lnStart, 34);

         }
         // Java: if (end < lnEnd)

         {
            // Java: ((Spannable)layout.getText()).setSpan(aftTrans, end - lnStart, lnEnd - lnStart, 34);

         }
         // Java: layout.draw(canvas);

         // Java: if (start > lnStart)

         {
            // Java: ((Spannable)layout.getText()).removeSpan(preTrans);

         }
         // Java: if (end < lnEnd)

         {
            // Java: ((Spannable)layout.getText()).removeSpan(aftTrans);

         }
      }
    }

    // Java: void drawSecondShadow(SpanInfo span, ShadowLayoutData typoData, int start, int end, Canvas canvas, TextData data)
    pub fn drawSecondShadow(&self, _span: i32, _typoData: i32, _start: i32, _end: i32, _canvas: i32, _data: i32) {
      // Java: int shdwAlpha = Color.alpha(span.mTextShadow.getShdwClr());

      // Java: int shdwClr = 0;

      // Java: if (typoData.getShadowType() == ShadowLayoutType.fill)

      {
         // Java: if (!span.hasFill())

         {
            // Java: return;

         }
         // Java: shdwClr = this.getAlphaColor(span.mTextShadow.getShdwClr(), (int)((float)shdwAlpha * this.getFillAlpha(span)));

      }
      // Java: else if (typoData.getShadowType() == ShadowLayoutType.outline)

      {
         // Java: if (!span.hasOutline())

         {
            // Java: return;

         }
         // Java: shdwClr = this.getAlphaColor(span.mTextShadow.getShdwClr(), (int)((float)shdwAlpha * this.getOutlineAlpha(span)));

      }
      // Java: else if (typoData.getShadowType() == ShadowLayoutType.blur)

      {
         // Java: if (!span.hasBlur())

         {
            // Java: return;

         }
         // Java: shdwClr = this.getAlphaColor(span.mTextShadow.getShdwClr(), (int)((float)shdwAlpha * span.mTextBlur.colorAlpha()));

      }
      // Java: int r = Math.min(255, Color.red(shdwClr) + 102);

      // Java: int g = Math.min(255, Color.green(shdwClr) + 102);

      // Java: int b = Math.min(255, Color.blue(shdwClr) + 102);

      // Java: int clr = Color.argb(Color.alpha(shdwClr), r, g, b);

      // Java: CharacterStyle charStyleTmp = new ForegroundColorSpan(clr);

      // Java: Spannable shdwSpan = typoData.getSpannable();

      // Java: Layout layout = typoData.getOrigLayout();

      // Java: shdwSpan.setSpan(charStyleTmp, start, end, 34);

      // Java: Layout shdwLayout2 = this.mTypoLayoutGen.geneLayout(shdwSpan, data, layout.getOriWidth(), layout.getAlignment(), layout.getPaint(), layout.getSpacingMultiplier());

      // Java: shdwLayout2.draw(canvas);

      // Java: shdwSpan.removeSpan(charStyleTmp);

    }

    // Java: void drawSecondShadowImg(SpanInfo span, SecondShadowType type, Spannable shdwSpan, Layout layout, int start, int end, int lnStart, int lnEnd, Canvas canvas, TextData data)
    pub fn drawSecondShadowImg(&self, _span: i32, _type: i32, _shdwSpan: i32, _layout: i32, _start: i32, _end: i32, _lnStart: i32, _lnEnd: i32, _canvas: i32, _data: i32) {
      // Java: int shdwAlpha = Color.alpha(span.mTextShadow.getShdwClr());

      // Java: int shdwClr = 0;

      // Java: if (type == WordArtLayout.SecondShadowType.FILL)

      {
         // Java: if (!span.hasFill())

         {
            // Java: return;

         }
         // Java: shdwClr = this.getAlphaColor(span.mTextShadow.getShdwClr(), (int)((float)shdwAlpha * this.getFillAlpha(span)));

      }
      // Java: else if (type == WordArtLayout.SecondShadowType.OUTLINE)

      {
         // Java: if (!span.hasOutline())

         {
            // Java: return;

         }
         // Java: shdwClr = this.getAlphaColor(span.mTextShadow.getShdwClr(), (int)((float)shdwAlpha * this.getOutlineAlpha(span)));

      }
      // Java: else if (type == WordArtLayout.SecondShadowType.BLUR)

      {
         // Java: if (!span.hasBlur())

         {
            // Java: return;

         }
         // Java: shdwClr = this.getAlphaColor(span.mTextShadow.getShdwClr(), (int)((float)shdwAlpha * span.mTextBlur.colorAlpha()));

      }
      // Java: WpsForegroundColorSpan[] styleSpans = (WpsForegroundColorSpan[])shdwSpan.getSpans(start, end, WpsForegroundColorSpan.class);

      // Java: for(WpsForegroundColorSpan styleSpan : styleSpans)

      {
         // Java: shdwSpan.removeSpan(styleSpan);

      }
      // Java: ForegroundColorSpan[] styleSpans2 = (ForegroundColorSpan[])shdwSpan.getSpans(start, end, ForegroundColorSpan.class);

      // Java: for(ForegroundColorSpan foregroundColorSpan : styleSpans2)

      {
         // Java: shdwSpan.removeSpan(foregroundColorSpan);

      }
      // Java: int r = Math.min(255, Color.red(shdwClr) + 102);

      // Java: int g = Math.min(255, Color.green(shdwClr) + 102);

      // Java: int b = Math.min(255, Color.blue(shdwClr) + 102);

      // Java: int clr = Color.argb(Color.alpha(shdwClr), r, g, b);

      // Java: CharacterStyle charStyleTmp = new ForegroundColorSpan(clr);

      // Java: shdwSpan.setSpan(charStyleTmp, start, end, 34);

      // Java: Layout shdwLayout2 = this.mTypoLayoutGen.geneLayout(shdwSpan.subSequence(lnStart, lnEnd), data, layout.getOriWidth(), layout.getAlignment(), layout.getPaint(), layout.getSpacingMultiplier());

      // Java: CharacterStyle preTrans = WpsHideSpan.Factory.create();

      // Java: CharacterStyle aftTrans = WpsHideSpan.Factory.create();

      // Java: if (start > lnStart)

      {
         // Java: ((Spannable)layout.getText()).setSpan(preTrans, 0, start - lnStart, 34);

      }
      // Java: if (end < lnEnd)

      {
         // Java: ((Spannable)layout.getText()).setSpan(aftTrans, end - lnStart, lnEnd - lnStart, 34);

      }
      // Java: shdwLayout2.draw(canvas);

      // Java: if (start > lnStart)

      {
         // Java: ((Spannable)layout.getText()).removeSpan(preTrans);

      }
      // Java: if (end < lnEnd)

      {
         // Java: ((Spannable)layout.getText()).removeSpan(aftTrans);

      }
      // Java: shdwSpan.removeSpan(charStyleTmp);

    }

    // Java: PointF getShadowOffsetDoc(PointF origin, RectF rect)
    pub fn getShadowOffsetDoc(&self, _origin: i32, _rect: i32) -> i32 {
      // Java: float x = (origin.x + 0.5F) * rect.width();

      // Java: float y = (origin.y + 0.5F) * rect.height();

      // Java: return new PointF(x, y);

      0
    }

    // Java: PointF getShadowOffset(int algn, RectF rect)
    pub fn getShadowOffset(&self, _algn: i32, _rect: i32) -> i32 {
      // Java: float x = 0.0F;

      // Java: float y = 0.0F;

      // Java: switch (TextShadow.ShadowAlgn.values()[algn])

      {
         // Java: case CTR:

            // Java: x = rect.width() / 2.0F;

            // Java: y = rect.height() / 2.0F;

            // Java: break;

         // Java: case L:

            // Java: x = 0.0F;

            // Java: y = rect.height() / 2.0F;

            // Java: break;

         // Java: case R:

            // Java: x = rect.width();

            // Java: y = rect.height() / 2.0F;

            // Java: break;

         // Java: case T:

            // Java: x = rect.width() / 2.0F;

            // Java: y = 0.0F;

            // Java: break;

         // Java: case TR:

            // Java: x = rect.width();

            // Java: y = 0.0F;

            // Java: break;

         // Java: case B:

            // Java: x = rect.width() / 2.0F;

            // Java: y = rect.height();

            // Java: break;

         // Java: case BL:

            // Java: x = 0.0F;

            // Java: y = rect.height();

            // Java: break;

         // Java: case BR:

            // Java: x = rect.width();

            // Java: y = rect.height();

            // Java: break;

         // Java: case TL:

         // Java: default:

            // Java: x = 0.0F;

            // Java: y = 0.0F;

      }
      // Java: return new PointF(x, y);

      0
    }

    // Java: void setStartMaskSpan(int startPara, int startOffset, CharacterStyle startMaskSpan)
    pub fn setStartMaskSpan(&self, _startPara: i32, _startOffset: i32, _startMaskSpan: i32) {
      // Java: this.setStartTrans(startPara, startOffset, startMaskSpan, this.mTypoFillCaches);

      // Java: if (this.checkWordArtAttr((byte)3))

      {
         // Java: this.setStartTrans(startPara, startOffset, startMaskSpan, this.mTypoBlurCaches);

      }
      // Java: if (this.checkWordArtAttr((byte)2))

      {
         // Java: this.setStartTrans(startPara, startOffset, startMaskSpan, this.mTypoOutlineCaches);

      }
      // Java: if (this.checkWordArtAttr((byte)5))

      {
         // Java: ShadowLayoutGroup layoutData = (ShadowLayoutGroup)this.mShadowLayoutCaches.get(startPara);

         // Java: if (layoutData != null)

         {
            // Java: int flag = 34;

            // Java: this.setSpanTransform(layoutData.mFillShadowLayout, 0, startOffset, startMaskSpan, flag, false);

            // Java: this.setSpanTransform(layoutData.mOutlineShadowLayout, 0, startOffset, startMaskSpan, flag, false);

            // Java: this.setSpanTransform(layoutData.mBlurShadowLayout, 0, startOffset, startMaskSpan, flag, false);

         }
      }
      // Java: if (this.checkWordArtAttr((byte)6))

      {
         // Java: this.setStartTrans(startPara, startOffset, startMaskSpan, this.mInnerShadowCaches);

      }
    }

    // Java: void setEndMaskSpan(int endPara, int endOffset, CharacterStyle endMaskSpan)
    pub fn setEndMaskSpan(&self, _endPara: i32, _endOffset: i32, _endMaskSpan: i32) {
      // Java: this.setEndTrans(endPara, endOffset, endMaskSpan, this.mTypoFillCaches);

      // Java: if (this.checkWordArtAttr((byte)3))

      {
         // Java: this.setEndTrans(endPara, endOffset, endMaskSpan, this.mTypoBlurCaches);

      }
      // Java: if (this.checkWordArtAttr((byte)2))

      {
         // Java: this.setEndTrans(endPara, endOffset, endMaskSpan, this.mTypoOutlineCaches);

      }
      // Java: if (this.checkWordArtAttr((byte)5))

      {
         // Java: ShadowLayoutGroup layoutData = (ShadowLayoutGroup)this.mShadowLayoutCaches.get(endPara - 1);

         // Java: if (layoutData != null)

         {
            // Java: int flag = 34;

            // Java: this.setSpanTransform(layoutData.mFillShadowLayout, endOffset, endOffset, endMaskSpan, flag, true);

            // Java: this.setSpanTransform(layoutData.mOutlineShadowLayout, endOffset, endOffset, endMaskSpan, flag, true);

            // Java: this.setSpanTransform(layoutData.mBlurShadowLayout, endOffset, endOffset, endMaskSpan, flag, true);

         }
      }
      // Java: if (this.checkWordArtAttr((byte)6))

      {
         // Java: this.setEndTrans(endPara, endOffset, endMaskSpan, this.mInnerShadowCaches);

      }
    }

    // Java: void removeStartMaskSpan(int startPara, CharacterStyle startMaskSpan)
    pub fn removeStartMaskSpan(&self, _startPara: i32, _startMaskSpan: i32) {
      // Java: this.removeStartTrans(startPara, startMaskSpan, this.mTypoFillCaches);

      // Java: if (this.checkWordArtAttr((byte)3))

      {
         // Java: this.removeStartTrans(startPara, startMaskSpan, this.mTypoBlurCaches);

      }
      // Java: if (this.checkWordArtAttr((byte)2))

      {
         // Java: this.removeStartTrans(startPara, startMaskSpan, this.mTypoOutlineCaches);

      }
      // Java: if (this.checkWordArtAttr((byte)5))

      {
         // Java: ShadowLayoutGroup layoutData = (ShadowLayoutGroup)this.mShadowLayoutCaches.get(startPara);

         // Java: if (layoutData != null)

         {
            // Java: if (layoutData.mFillShadowLayout != null)

            {
               // Java: ((Spannable)layoutData.mFillShadowLayout.getText()).removeSpan(startMaskSpan);

            }
            // Java: if (layoutData.mOutlineShadowLayout != null)

            {
               // Java: ((Spannable)layoutData.mOutlineShadowLayout.getText()).removeSpan(startMaskSpan);

            }
            // Java: if (layoutData.mBlurShadowLayout != null)

            {
               // Java: ((Spannable)layoutData.mBlurShadowLayout.getText()).removeSpan(startMaskSpan);

            }
         }
      }
      // Java: if (this.checkWordArtAttr((byte)6))

      {
         // Java: this.removeStartTrans(startPara, startMaskSpan, this.mInnerShadowCaches);

      }
    }

    // Java: void removeEndMaskSpan(int endPara, CharacterStyle endMaskSpan)
    pub fn removeEndMaskSpan(&self, _endPara: i32, _endMaskSpan: i32) {
      // Java: this.removeEndTrans(endPara, endMaskSpan, this.mTypoFillCaches);

      // Java: if (this.checkWordArtAttr((byte)3))

      {
         // Java: this.removeEndTrans(endPara, endMaskSpan, this.mTypoBlurCaches);

      }
      // Java: if (this.checkWordArtAttr((byte)2))

      {
         // Java: this.removeEndTrans(endPara, endMaskSpan, this.mTypoOutlineCaches);

      }
      // Java: if (this.checkWordArtAttr((byte)5))

      {
         // Java: ShadowLayoutGroup layoutData = (ShadowLayoutGroup)this.mShadowLayoutCaches.get(endPara - 1);

         // Java: if (layoutData != null)

         {
            // Java: if (layoutData.mFillShadowLayout != null)

            {
               // Java: ((Spannable)layoutData.mFillShadowLayout.getText()).removeSpan(endMaskSpan);

            }
            // Java: if (layoutData.mOutlineShadowLayout != null)

            {
               // Java: ((Spannable)layoutData.mOutlineShadowLayout.getText()).removeSpan(endMaskSpan);

            }
            // Java: if (layoutData.mBlurShadowLayout != null)

            {
               // Java: ((Spannable)layoutData.mBlurShadowLayout.getText()).removeSpan(endMaskSpan);

            }
         }
      }
      // Java: if (this.checkWordArtAttr((byte)6))

      {
         // Java: this.removeEndTrans(endPara, endMaskSpan, this.mInnerShadowCaches);

      }
    }

    // Java: void setSpanTransform(Layout layout, int start, int end, CharacterStyle maskSpan, int flag, boolean isEnd)
    pub fn setSpanTransform(&self, _layout: i32, _start: i32, _end: i32, _maskSpan: i32, _flag: i32, _isEnd: bool) {
      // Java: if (layout != null && start != end)

      {
         // Java: end = isEnd ? layout.getText().length() : end;

         // Java: ((Spannable)layout.getText()).setSpan(maskSpan, start, end, flag);

      }
    }

    // Java: void setStartTrans(int startPara, int startOffset, CharacterStyle startMaskSpan, List<Layout> typoCaches)
    pub fn setStartTrans(&self, _startPara: i32, _startOffset: i32, _startMaskSpan: i32, _typoCaches: i32) {
      // Java: if (typoCaches != null)

      {
         // Java: int flag = 34;

         // Java: if (typoCaches.get(startPara) != null)

         {
            // Java: ((Spannable)((Layout)typoCaches.get(startPara)).getText()).setSpan(startMaskSpan, 0, startOffset, flag);

         }
      }
    }

    // Java: void setEndTrans(int endPara, int endOffset, CharacterStyle endMaskSpan, List<Layout> typoCaches)
    pub fn setEndTrans(&self, _endPara: i32, _endOffset: i32, _endMaskSpan: i32, _typoCaches: i32) {
      // Java: if (typoCaches != null)

      {
         // Java: int flag = 34;

         // Java: if (typoCaches.get(endPara - 1) != null)

         {
            // Java: ((Spannable)((Layout)typoCaches.get(endPara - 1)).getText()).setSpan(endMaskSpan, endOffset, ((Layout)typoCaches.get(endPara - 1)).getText().length(), flag);

         }
      }
    }

    // Java: void removeStartTrans(int startPara, CharacterStyle startMaskSpan, List<Layout> typoCaches)
    pub fn removeStartTrans(&self, _startPara: i32, _startMaskSpan: i32, _typoCaches: i32) {
      // Java: if (typoCaches != null && typoCaches.get(startPara) != null)

      {
         // Java: ((Spannable)((Layout)typoCaches.get(startPara)).getText()).removeSpan(startMaskSpan);

      }
    }

    // Java: void removeEndTrans(int endPara, CharacterStyle endMaskSpan, List<Layout> typoCaches)
    pub fn removeEndTrans(&self, _endPara: i32, _endMaskSpan: i32, _typoCaches: i32) {
      // Java: if (typoCaches != null && typoCaches.get(endPara - 1) != null)

      {
         // Java: ((Spannable)((Layout)typoCaches.get(endPara - 1)).getText()).removeSpan(endMaskSpan);

      }
    }

    // Java: List<List<Layout>> getTxtLayouts()
    pub fn getTxtLayouts(&self) -> Vec<Vec<i32>> {
      // Java: List<List<Layout>> layouts = new ArrayList();

      // Java: if (this.mTypoFillCaches != null)

      {
         // Java: layouts.add(this.mTypoFillCaches);

      }
      // Java: if (this.mTypoBlurCaches != null)

      {
         // Java: layouts.add(this.mTypoBlurCaches);

      }
      // Java: if (this.mTypoOutlineCaches != null)

      {
         // Java: layouts.add(this.mTypoOutlineCaches);

      }
      // Java: return layouts;

      Vec::new()
    }

    // Java: boolean[] alterClrLys()
    pub fn alterClrLys(&self) -> Vec<bool> {
      // Java: return new boolean[]{true, false, false};

      Vec::new()
    }

    // Java: int getAlphaColor(int color, int alpha)
    pub fn getAlphaColor(&self, _color: i32, _alpha: i32) -> i32 {
      // Java: return alpha << 24 | 16777215 & color;

      0
    }

    // Java: CharacterStyle createOutline(TextOutline textOutline, float lineWid, float lineHei, float lineTop, float lineLeft)
    pub fn createOutline(&self, _textOutline: i32, _lineWid: f32, _lineHei: f32, _lineTop: f32, _lineLeft: f32) -> i32 {
      // Java: try

      {
         // Java: return new WpsTextOutlineSpan(textOutline, lineWid, lineHei, lineTop, lineLeft);

      }
      // Java: catch (Throwable var7)

      {
         // Java: return EMPTY_CHAR_STYLE;

      }
      0
    }

    // Java: CharacterStyle createOutline(float blurRad, int shdwClr, TextOutline textOutline, boolean isYCompress)
    pub fn createOutline_2(&self, _blurRad: f32, _shdwClr: i32, _textOutline: i32, _isYCompress: bool) -> i32 {
      // Java: try

      {
         // Java: return new WpsTextOutlineSpan(blurRad, shdwClr, textOutline, isYCompress);

      }
      // Java: catch (Throwable var6)

      {
         // Java: return EMPTY_CHAR_STYLE;

      }
      0
    }

}

// Java: public enum SecondShadowType
impl SecondShadowType {
    // Java: SecondShadowType()
    pub fn new_2(&self) {

    }

}
