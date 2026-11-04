#![allow(dead_code, unused_variables, unused_mut, unreachable_code, non_snake_case, unused_imports, non_camel_case_types)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Integer = i32;
type List<T> = Vec<T>;
type Map<K, V> = HashMap<K, V>;

struct DecoLineCache;
struct SpanInfo;
struct TextData;
struct TimingInfo;
struct WpsLayout;
struct Layout;
struct Spannable;
struct SpannableStringBuilder;
struct CharacterStyle;
struct ForegroundColorSpan;
struct StrikethroughSpan;
struct UnderlineSpan;
struct Range;
struct TextRangeType;
struct TextUtil;

// Java: import cn.wps.text.layout.data.DecoLineCache

// Java: import cn.wps.text.layout.data.SpanInfo

// Java: import cn.wps.text.layout.data.TextData

// Java: import cn.wps.text.layout.data.TimingInfo

// Java: import cn.wps.text.layout.layouter.WpsLayout

// Java: import cn.wps.text.layout.typo.layout.Layout

// Java: import cn.wps.text.layout.typo.layout.Spannable

// Java: import cn.wps.text.layout.typo.layout.SpannableStringBuilder

// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle

// Java: import cn.wps.text.layout.typo.layout.style.ForegroundColorSpan

// Java: import cn.wps.text.layout.typo.layout.style.StrikethroughSpan

// Java: import cn.wps.text.layout.typo.layout.style.UnderlineSpan

// Java: import cn.wps.text.layout.util.Range

// Java: import cn.wps.text.layout.util.TextRangeType

// Java: import cn.wps.text.layout.util.TextUtil

// Java: import java.util.ArrayList

// Java: import java.util.HashMap

// Java: import java.util.List

// Java: import java.util.Map


// Java: public class TimingLayout
pub struct TimingLayout {
    // Java field: List<DecoLineCache> mBackupDecoLines
    pub(crate) mBackupDecoLines: Option<Vec<i32>>,
    // Java field: List<DecoLineCache> mAlterDecoLines
    pub(crate) mAlterDecoLines: Option<Vec<i32>>,
    // Java field: Map<Integer, List<Layout>> mBackupLys
    mBackupLys: Option<HashMap<i32, Vec<i32>>>,
    // Java field: Map<Integer, List<List<Layout>>> mBackupImgs
    mBackupImgs: Option<HashMap<i32, Vec<Vec<i32>>>>,
    // Java field: Map<Integer, TextData> mAlterDatas
    mAlterDatas: Option<HashMap<i32, i32>>,
}

// Java: public class TimingLayout
impl TimingLayout {
    // Java: TimingLayout(List<DecoLineCache> decoLineCaches)
    pub fn new(&self, _decoLineCaches: i32) {
      // Java: super();

      // Java: this.mBackupLys = new HashMap();

      // Java: this.mBackupImgs = new HashMap();

      // Java: this.mBackupDecoLines = decoLineCaches;

      // Java: this.mAlterDatas = new HashMap();

    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: if (this.mBackupLys != null)

      {
         // Java: this.mBackupLys.clear();

      }
      // Java: if (this.mBackupImgs != null)

      {
         // Java: this.mBackupImgs.clear();

      }
      // Java: if (this.mAlterDecoLines != null)

      {
         // Java: this.mAlterDecoLines.clear();

      }
      // Java: if (this.mAlterDatas != null)

      {
         // Java: this.mAlterDatas.clear();

      }
    }

    // Java: void alterAttr(List<Layout> typoCaches, TimingInfo timingInfo, boolean alterClr, List<TextData> datas)
    pub fn alterAttr(&self, _typoCaches: i32, _timingInfo: i32, _alterClr: bool, _datas: i32) {
      // Java: byte alterType = timingInfo.getAlterType();

      // Java: int startPara = timingInfo.getStartPara();

      // Java: int startOffset = timingInfo.getStartOffset();

      // Java: int endPara = timingInfo.getEndPara();

      // Java: int endOffset = timingInfo.getEndOffset();

      // Java: for(int i = startPara; i <= endPara; ++i)

      {
         // Java: Layout layout = (Layout)typoCaches.get(i);

         // Java: if (layout != null)

         {
            // Java: TextData data = (TextData)datas.get(i);

            // Java: if (!layout.getText().toString().trim().equals(""))

            {
               // Java: int start = 0;

               // Java: int end = layout.getText().length();

               // Java: if (i == startPara)

               {
                  // Java: start = startOffset;

               }
               // Java: if (i == endPara)

               {
                  // Java: end = endOffset;

               }
               // Java: Spannable span = (Spannable)layout.getText();

               // Java: this.applyAttr(alterType, span, timingInfo, start, end, alterClr, data);

            }
         }
      }
    }

    // Java: void applyAttr(byte alterType, Spannable span, TimingInfo timingInfo, int start, int end, boolean alterClr, TextData data)
    pub fn applyAttr(&self, _alterType: u8, _span: i32, _timingInfo: i32, _start: i32, _end: i32, _alterClr: bool, _data: i32) {
      // Java: if ((alterType & 2) != 0 && alterClr)

      {
         // Java: int fontclr = timingInfo.getFontcolor();

         // Java: this.alterColor(span, fontclr, start, end, data);

      }
      // Java: if ((alterType & 8) != 0)

      {
         // Java: if (timingInfo.isBold())

         {
            // Java: CharacterStyle charStyle = new WpsStyleSpan(1);

            // Java: span.setSpan(charStyle, start, end, 34);

         }
         // Java: else

         {
            // Java: WpsStyleSpan[] styleSpans = (WpsStyleSpan[])span.getSpans(start, end, WpsStyleSpan.class);

            // Java: for(int i = 0; i < styleSpans.length; ++i)

            {
               // Java: if ((styleSpans[i].getNewStyle() & 1) != 0)

               {
                  // Java: span.removeSpan(styleSpans[i]);

               }
            }
         }
      }
      // Java: if ((alterType & 32) != 0)

      {
         // Java: if (timingInfo.isItalic())

         {
            // Java: CharacterStyle charStyle = new WpsStyleSpan(2);

            // Java: span.setSpan(charStyle, start, end, 34);

         }
         // Java: else

         {
            // Java: WpsStyleSpan[] styleSpans = (WpsStyleSpan[])span.getSpans(start, end, WpsStyleSpan.class);

            // Java: for(int i = 0; i < styleSpans.length; ++i)

            {
               // Java: if ((styleSpans[i].getNewStyle() & 2) != 0)

               {
                  // Java: span.removeSpan(styleSpans[i]);

               }
            }
         }
      }
      // Java: if ((alterType & 4) != 0)

      {
         // Java: if (timingInfo.isUnderline())

         {
            // Java: UnderlineSpan underlineSpan = new UnderlineSpan();

            // Java: span.setSpan(underlineSpan, start, end, 34);

         }
         // Java: else

         {
            // Java: this.removeSpan(span, start, end, UnderlineSpan.class);

         }
      }
      // Java: if ((alterType & 16) != 0)

      {
         // Java: if (timingInfo.isStrikethrough())

         {
            // Java: StrikethroughSpan strikethroughSpan = new StrikethroughSpan();

            // Java: span.setSpan(strikethroughSpan, start, end, 34);

         }
         // Java: else

         {
            // Java: this.removeSpan(span, start, end, StrikethroughSpan.class);

         }
      }
      // Java: if ((alterType & 64) != 0)

      {
         // Java: double fraction = timingInfo.getDynamicDigitFraction();

         // Java: double dest = timingInfo.getDynamicDigitDest();

         // Java: int mode = timingInfo.getDynamicDigitAnimMode();

         // Java: double src = timingInfo.getDynamicDigitSrc();

         // Java: boolean srcAbs = timingInfo.isDynamicDigitIsSrcAbs();

         // Java: boolean destAbs = timingInfo.isDynamicDigitIsDestAbs();

         // Java: boolean optimizeRate = timingInfo.isDynamicDigitOptimizeRate();

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
         // Java: if (destText.length() > data.mTypoText.length())

         {
            // Java: return;

         }
         // Java: if (span instanceof SpannableStringBuilder)

         {
            // Java: SpannableStringBuilder s = (SpannableStringBuilder)span;

            // Java: for(Range rng : digitRange)

            {
               // Java: int _start = start < rng.start() ? rng.start() : start;

               // Java: int _end = end > rng.end() ? rng.end() : end;

               // Java: if (rng.type() == TextRangeType.Text_Digit && _start < _end)

               {
                  // Java: String replaceStr = destText.substring(_start, _end);

                  // Java: for(int i = 0; i < replaceStr.length(); ++i)

                  {
                     // Java: s.replaceChar(replaceStr.charAt(i), _start + i);

                  }
               }
            }
         }
      }
    }

    // Java: void alterColor(Spannable span, int fontclr, int start, int end, TextData data)
    pub fn alterColor(&self, _span: i32, _fontclr: i32, _start: i32, _end: i32, _data: i32) {
      // Java: CharacterStyle charStyle = new ForegroundColorSpan(fontclr);

      // Java: if (!WpsSpanHelper.isGEAndroid6_0())

      {
         // Java: span.setSpan(charStyle, start, end, 34);

      }
      // Java: else

      {
         // Java: SpanInfo spanInfo = data.getSpanByOffset(start);

         // Java: if (spanInfo != null)

         {
            // Java: if (end <= spanInfo.mEndPos)

            {
               // Java: this.removeSpan(span, start, end, ForegroundColorSpan.class);

               // Java: span.setSpan(charStyle, start, end, 34);

            }
            // Java: else

            {
               // Java: this.removeSpan(span, start, spanInfo.mEndPos, ForegroundColorSpan.class);

               // Java: span.setSpan(charStyle, start, spanInfo.mEndPos, 34);

               // Java: for(SpanInfo var9 = data.getSpanByOffset(spanInfo.mEndPos); var9 != null; var9 = data.getSpanByOffset(var9.mEndPos))

               {
                  // Java: CharacterStyle var8 = new ForegroundColorSpan(fontclr);

                  // Java: if (end <= var9.mEndPos)

                  {
                     // Java: this.removeSpan(span, var9.mStartPos, end, ForegroundColorSpan.class);

                     // Java: span.setSpan(var8, var9.mStartPos, end, 34);

                     // Java: break;

                  }
                  // Java: this.removeSpan(span, var9.mStartPos, var9.mEndPos, ForegroundColorSpan.class);

                  // Java: span.setSpan(var8, var9.mStartPos, var9.mEndPos, 34);

               }
            }
         }
      }
    }

    // Java: void removeSpan(Spannable span, int start, int end, Class clazz)
    pub fn removeSpan(&self, _span: i32, _start: i32, _end: i32, _clazz: i32) {
      // Java: Object[] spans = span.getSpans(start, end, clazz);

      // Java: int i = 0;

      // Java: for(int size = spans.length; i < size; ++i)

      {
         // Java: Object item = spans[i];

         // Java: int s = span.getSpanStart(item);

         // Java: int e = span.getSpanEnd(item);

         // Java: span.removeSpan(item);

         // Java: if (s < start)

         {
            // Java: span.setSpan(item, s, start, 34);

         }
         // Java: if (end < e)

         {
            // Java: span.setSpan(item, end, e, 34);

         }
      }
    }

    // Java: void beforeRender(List<List<Layout>> layouts, boolean[] alterClrLys, TimingInfo timInfo, TypoLayoutGen typoLayoutGen, List<TextData> datas, List<DecoLineCache> decoLineCaches, int verticalText)
    pub fn beforeRender(&self, _layouts: i32, _alterClrLys: i32, _timInfo: i32, _typoLayoutGen: i32, _datas: i32, _decoLineCaches: i32, _verticalText: i32) {
      // Java: if (layouts != null)

      {
         // Java: this.cloneTypo(timInfo.getStartPara(), timInfo.getEndPara(), layouts, typoLayoutGen, datas, timInfo);

         // Java: for(int i = 0; i < layouts.size(); ++i)

         {
            // Java: this.alterAttr((List)layouts.get(i), timInfo, alterClrLys[i], datas);

         }
         // Java: byte alterType = timInfo.getAlterType();

         // Java: boolean underln = (alterType & 4) != 0;

         // Java: boolean st = (alterType & 16) != 0;

         // Java: if (underln || st)

         {
            // Java: this.alterDecoLine((List)layouts.get(0), timInfo, datas, decoLineCaches, verticalText);

         }
      }
    }

    // Java: void alterDecoLine(List<Layout> typoCaches, TimingInfo timingInfo, List<TextData> datas, List<DecoLineCache> decoLineCaches, int vertText)
    pub fn alterDecoLine(&self, _typoCaches: i32, _timingInfo: i32, _datas: i32, _decoLineCaches: i32, _vertText: i32) {
      // Java: byte alterType = timingInfo.getAlterType();

      // Java: int startPara = timingInfo.getStartPara();

      // Java: int startOffset = timingInfo.getStartOffset();

      // Java: int endPara = timingInfo.getEndPara();

      // Java: int endOffset = timingInfo.getEndOffset();

      // Java: boolean isFirst = this.mAlterDecoLines == null;

      // Java: if (isFirst)

      {
         // Java: this.mAlterDecoLines = new ArrayList();

         // Java: for(int i = 0; i < startPara; ++i)

         {
            // Java: this.mAlterDecoLines.add((DecoLineCache)decoLineCaches.get(i));

         }
      }
      // Java: boolean underln = timingInfo.isUnderline();

      // Java: boolean st = timingInfo.isStrikethrough();

      // Java: if (vertText != 4 && vertText != 3 && vertText != 6)

      {
         // Java: this.alterNormalDecoln(startPara, endPara, startOffset, endOffset, datas, typoCaches, underln, st, timingInfo, isFirst);

      }
      // Java: else

      {
         // Java: this.alterVertDecoln(startPara, endPara, startOffset, endOffset, datas, typoCaches, underln, st, timingInfo, isFirst);

      }
      // Java: if (isFirst && endPara + 1 < datas.size())

      {
         // Java: for(int i = endPara + 1; i < datas.size(); ++i)

         {
            // Java: this.mAlterDecoLines.add((DecoLineCache)decoLineCaches.get(i));

         }
      }
    }

    // Java: void alterVertDecoln(int startPara, int endPara, int startOffset, int endOffset, List<TextData> datas, List<Layout> typoCaches, boolean underln, boolean st, TimingInfo timInfo, boolean isFirst)
    pub fn alterVertDecoln(&self, _startPara: i32, _endPara: i32, _startOffset: i32, _endOffset: i32, _datas: i32, _typoCaches: i32, _underln: bool, _st: bool, _timInfo: i32, _isFirst: bool) {
      // Java: for(int i = startPara; i <= endPara; ++i)

      {
         // Java: Layout layout = (Layout)typoCaches.get(i);

         // Java: if (layout != null && !layout.getText().toString().trim().equals(""))

         {
            // Java: int start = 0;

            // Java: int end = layout.getText().length();

            // Java: if (i == startPara)

            {
               // Java: start = startOffset;

            }
            // Java: if (i == endPara)

            {
               // Java: end = endOffset;

            }
            // Java: TextData dataTmp = new TextData();

            // Java: TextData data = (TextData)datas.get(i);

            // Java: dataTmp.copyFrom(data);

            // Java: this.processDecoLn(dataTmp, start, end, underln, st, true);

            // Java: DecoLineCache decoLineCache = new DecoLineCache();

            // Java: decoLineCache.calcDecoLine(layout, dataTmp);

            // Java: this.addDecoLn(decoLineCache, i, isFirst);

            // Java: this.mAlterDatas.put(i, dataTmp);

         }
      }
    }

    // Java: void alterNormalDecoln(int startPara, int endPara, int startOffset, int endOffset, List<TextData> datas, List<Layout> typoCaches, boolean underln, boolean st, TimingInfo timInfo, boolean isFirst)
    pub fn alterNormalDecoln(&self, _startPara: i32, _endPara: i32, _startOffset: i32, _endOffset: i32, _datas: i32, _typoCaches: i32, _underln: bool, _st: bool, _timInfo: i32, _isFirst: bool) {
      // Java: for(int i = startPara; i <= endPara; ++i)

      {
         // Java: Layout layout = (Layout)typoCaches.get(i);

         // Java: if (layout != null && !layout.getText().toString().trim().equals(""))

         {
            // Java: Spannable span = (Spannable)layout.getText();

            // Java: int start = 0;

            // Java: int end = layout.getText().length();

            // Java: if (i > startPara && i < endPara)

            {
               // Java: this.addDecoLn((DecoLineCache)null, i, isFirst);

            }
            // Java: else

            {
               // Java: TextData data = (TextData)datas.get(i);

               // Java: if (i == startPara || i == endPara)

               {
                  // Java: if (i == startPara)

                  {
                     // Java: start = startOffset;

                  }
                  // Java: if (i == endPara)

                  {
                     // Java: end = endOffset;

                  }
                  // Java: if (start == 0 && end == layout.getText().length())

                  {
                     // Java: this.addDecoLn((DecoLineCache)null, i, isFirst);

                  }
                  // Java: else

                  {
                     // Java: TextData dataTmp = null;

                     // Java: if (this.mAlterDatas.containsKey(i))

                     {
                        // Java: dataTmp = (TextData)this.mAlterDatas.get(i);

                     }
                     // Java: else

                     {
                        // Java: dataTmp = new TextData();

                        // Java: dataTmp.copyFrom(data);

                     }
                     // Java: this.processDecoLn(dataTmp, start, end, underln, st, false);

                     // Java: DecoLineCache decoLineCache = new DecoLineCache();

                     // Java: decoLineCache.calcDecoLine(layout, dataTmp);

                     // Java: this.addDecoLn(decoLineCache, i, isFirst);

                     // Java: this.mAlterDatas.put(i, dataTmp);

                  }
               }
            }
         }
      }
    }

    // Java: void addDecoLn(DecoLineCache decoLineCache, int para, boolean isFirst)
    pub fn addDecoLn(&self, _decoLineCache: i32, _para: i32, _isFirst: bool) {
      // Java: if (isFirst)

      {
         // Java: this.mAlterDecoLines.add(decoLineCache);

      }
      // Java: else

      {
         // Java: this.mAlterDecoLines.set(para, decoLineCache);

      }
    }

    // Java: List<DecoLineCache> getAlterDecoLines()
    pub fn getAlterDecoLines(&self) -> Vec<i32> {
      // Java: return this.mAlterDecoLines;

      Vec::new()
    }

    // Java: void processDecoLn(TextData data, int startOffset, int endOffset, boolean underln, boolean st, boolean vert)
    pub fn processDecoLn(&self, _data: i32, _startOffset: i32, _endOffset: i32, _underln: bool, _st: bool, _vert: bool) {
      // Java: int textLen = data.mSourceText.length();

      // Java: SpanInfo span = data.getFirstSpan();

      // Java: while(span != null && span.mEndPos <= textLen)

      {
         // Java: int start = span.mStartPos;

         // Java: int end = span.mEndPos;

         // Java: if (start < startOffset)

         {
            // Java: if (end > startOffset)

            {
               // Java: span.mEndPos = startOffset;

               // Java: SpanInfo spanTmp = new SpanInfo(span);

               // Java: spanTmp.mStartPos = startOffset;

               // Java: if (vert)

               {
                  // Java: spanTmp.mUnderline = underln ? 1 : spanTmp.mUnderline;

                  // Java: spanTmp.mStrikethrough = st ? 101 : spanTmp.mStrikethrough;

               }
               // Java: else

               {
                  // Java: spanTmp.mUnderline = underln ? 0 : spanTmp.mUnderline;

                  // Java: spanTmp.mStrikethrough = st ? 100 : spanTmp.mStrikethrough;

               }
               // Java: if (end <= endOffset)

               {
                  // Java: spanTmp.mEndPos = end;

                  // Java: data.insertSpanAft(spanTmp);

               }
               // Java: else

               {
                  // Java: spanTmp.mEndPos = endOffset;

                  // Java: data.insertSpanAft(spanTmp);

                  // Java: SpanInfo spanTmp2 = new SpanInfo(span);

                  // Java: spanTmp2.mStartPos = endOffset;

                  // Java: spanTmp2.mEndPos = end;

                  // Java: data.insertSpanAft(spanTmp2);

               }
            }
            // Java: else

            {
               // Java: span = data.getNextSpan();

            }
         }
         // Java: else

         {
            // Java: if (start >= startOffset && start < endOffset)

            {
               // Java: if (vert)

               {
                  // Java: span.mUnderline = underln ? 1 : span.mUnderline;

                  // Java: span.mStrikethrough = st ? 101 : span.mStrikethrough;

               }
               // Java: else

               {
                  // Java: span.mUnderline = underln ? 0 : span.mUnderline;

                  // Java: span.mStrikethrough = st ? 100 : span.mStrikethrough;

               }
               // Java: if (end > endOffset)

               {
                  // Java: span.mEndPos = endOffset;

                  // Java: SpanInfo spanTmp = new SpanInfo(span);

                  // Java: spanTmp.mStartPos = endOffset;

                  // Java: spanTmp.mEndPos = end;

                  // Java: data.insertSpanAft(spanTmp);

               }
            }
            // Java: span = data.getNextSpan();

         }
      }
    }

    // Java: void cloneTypo(int startPara, int endPara, List<List<Layout>> orgCaches, TypoLayoutGen typoLayoutGen, List<TextData> datas, TimingInfo timInfo)
    pub fn cloneTypo(&self, _startPara: i32, _endPara: i32, _orgCaches: i32, _typoLayoutGen: i32, _datas: i32, _timInfo: i32) {
      // Java: for(int para = startPara; para <= endPara; ++para)

      {
         // Java: if (!this.mBackupLys.containsKey(para))

         {
            // Java: List<Layout> paraBack = new ArrayList();

            // Java: for(int attr = 0; attr < orgCaches.size(); ++attr)

            {
               // Java: Layout layout = (Layout)((List)orgCaches.get(attr)).get(para);

               // Java: if (layout != null)

               {
                  // Java: Layout attrBack;

                  // Java: if (layout instanceof WpsLayout)

                  {
                     // Java: attrBack = ((WpsLayout)layout).copy();

                  }
                  // Java: else

                  {
                     // Java: Spannable span = new SpannableStringBuilder(layout.getText());

                     // Java: attrBack = typoLayoutGen.geneLayout(span, (TextData)datas.get(para), (float)layout.getWidth(), layout.getAlignment(), layout.getPaint(), layout.getSpacingMultiplier());

                  }
                  // Java: paraBack.add(attrBack);

               }
               // Java: else

               {
                  // Java: paraBack.add(null);

               }
            }
            // Java: this.mBackupLys.put(para, paraBack);

         }
      }
    }

    // Java: void restoreOrg(List<List<Layout>> layouts)
    pub fn restoreOrg(&self, _layouts: i32) {
      // Java: this.restoreTypo(layouts);

    }

    // Java: List<DecoLineCache> getBackupDecoLines()
    pub fn getBackupDecoLines(&self) -> Vec<i32> {
      // Java: return this.mBackupDecoLines;

      Vec::new()
    }

    // Java: void restoreTypo(List<List<Layout>> layouts)
    pub fn restoreTypo(&self, _layouts: i32) {
      // Java: Object[] paraIdxs = this.mBackupLys.keySet().toArray();

      // Java: for(int i = 0; i < paraIdxs.length; ++i)

      {
         // Java: int para = (Integer)paraIdxs[i];

         // Java: for(int attr = 0; attr < layouts.size(); ++attr)

         {
            // Java: List<Layout> org = (List)layouts.get(attr);

            // Java: org.set(para, (Layout)((List)this.mBackupLys.get(para)).get(attr));

         }
      }
    }

}