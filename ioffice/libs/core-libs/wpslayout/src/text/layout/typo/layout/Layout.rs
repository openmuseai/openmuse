// Java: import android.graphics.Canvas
use thirdparty_kInk::com::hp::hpl::inkml::Canvas;
// Java: import android.graphics.Paint
use crate::android::graphics::Paint;
// Java: import android.graphics.Path
use crate::android::graphics::Path;
// Java: import android.graphics.Rect
use util::cn::wps::graphics::Rect;
// Java: import android.text.TextPaint
use crate::android::text::TextPaint;
// Java: import cn.wps.graphics.RectF
use util::cn::wps::graphics::RectF;
// Java: import cn.wps.text.layout.layouter.WpsLayout
use wpslayout::cn::wps::text::layout::layouter::WpsLayout;
// Java: import cn.wps.text.layout.typo.layout.style.LeadingMarginSpan
use wpslayout::cn::wps::text::layout::typo::layout::style::LeadingMarginSpan;
// Java: import cn.wps.text.layout.typo.layout.style.ParagraphStyle
use wpslayout::cn::wps::text::layout::typo::layout::style::ParagraphStyle;
// Java: import cn.wps.text.layout.typo.layout.style.ReplacementSpan
use wpslayout::cn::wps::text::layout::typo::layout::style::ReplacementSpan;
// Java: import java.util.Arrays
// TODO_RUST_CODE use ???; // java.util.Arrays
// Java: import java.util.List
// TODO_RUST_CODE use ???; // java.util.List

// Java field: int BREAK_STRATEGY_SIMPLE
pub(crate) const BREAK_STRATEGY_SIMPLE: i32 = /* TODO */;
// Java field: int BREAK_STRATEGY_HIGH_QUALITY
pub(crate) const BREAK_STRATEGY_HIGH_QUALITY: i32 = /* TODO */;
// Java field: int BREAK_STRATEGY_BALANCED
pub(crate) const BREAK_STRATEGY_BALANCED: i32 = /* TODO */;
// Java field: int HYPHENATION_FREQUENCY_NONE
pub(crate) const HYPHENATION_FREQUENCY_NONE: i32 = /* TODO */;
// Java field: int HYPHENATION_FREQUENCY_NORMAL
pub(crate) const HYPHENATION_FREQUENCY_NORMAL: i32 = /* TODO */;
// Java field: int HYPHENATION_FREQUENCY_FULL
pub(crate) const HYPHENATION_FREQUENCY_FULL: i32 = /* TODO */;
// Java field: ParagraphStyle[] NO_PARA_SPANS
static NO_PARA_SPANS: std::sync::OnceLock<Vec<ParagraphStyle>> = std::sync::OnceLock::new();
// Java field: int JUSTIFICATION_MODE_NONE
pub(crate) const JUSTIFICATION_MODE_NONE: i32 = /* TODO */;
// Java field: int JUSTIFICATION_MODE_INTER_WORD
pub(crate) const JUSTIFICATION_MODE_INTER_WORD: i32 = /* TODO */;
// Java field: float DEFAULT_LINESPACING_MULTIPLIER
pub(crate) const DEFAULT_LINESPACING_MULTIPLIER: f32 = /* TODO */;
// Java field: float DEFAULT_LINESPACING_ADDITION
pub(crate) const DEFAULT_LINESPACING_ADDITION: f32 = /* TODO */;
// Java field: Rect sTempRect
static sTempRect: std::sync::OnceLock<Rect> = std::sync::OnceLock::new();
// Java field: int DIR_LEFT_TO_RIGHT
pub(crate) const DIR_LEFT_TO_RIGHT: i32 = /* TODO */;
// Java field: int DIR_RIGHT_TO_LEFT
pub(crate) const DIR_RIGHT_TO_LEFT: i32 = /* TODO */;
// Java field: int DIR_REQUEST_LTR
pub(crate) const DIR_REQUEST_LTR: i32 = /* TODO */;
// Java field: int DIR_REQUEST_RTL
pub(crate) const DIR_REQUEST_RTL: i32 = /* TODO */;
// Java field: int DIR_REQUEST_DEFAULT_LTR
pub(crate) const DIR_REQUEST_DEFAULT_LTR: i32 = /* TODO */;
// Java field: int DIR_REQUEST_DEFAULT_RTL
pub(crate) const DIR_REQUEST_DEFAULT_RTL: i32 = /* TODO */;
// Java field: int RUN_LENGTH_MASK
pub(crate) const RUN_LENGTH_MASK: i32 = /* TODO */;
// Java field: int RUN_LEVEL_SHIFT
pub(crate) const RUN_LEVEL_SHIFT: i32 = /* TODO */;
// Java field: int RUN_LEVEL_MASK
pub(crate) const RUN_LEVEL_MASK: i32 = /* TODO */;
// Java field: int RUN_RTL_FLAG
pub(crate) const RUN_RTL_FLAG: i32 = /* TODO */;
// Java field: int TAB_INCREMENT
pub(crate) const TAB_INCREMENT: i32 = /* TODO */;
// Java field: Directions DIRS_ALL_LEFT_TO_RIGHT
static DIRS_ALL_LEFT_TO_RIGHT: std::sync::OnceLock<Directions> = std::sync::OnceLock::new();
// Java field: Directions DIRS_ALL_RIGHT_TO_LEFT
static DIRS_ALL_RIGHT_TO_LEFT: std::sync::OnceLock<Directions> = std::sync::OnceLock::new();
// Java field: int TEXT_SELECTION_LAYOUT_RIGHT_TO_LEFT
pub(crate) const TEXT_SELECTION_LAYOUT_RIGHT_TO_LEFT: i32 = /* TODO */;
// Java field: int TEXT_SELECTION_LAYOUT_LEFT_TO_RIGHT
pub(crate) const TEXT_SELECTION_LAYOUT_LEFT_TO_RIGHT: i32 = /* TODO */;

// Java: public class Layout
pub struct Layout {
    // Java field: CharSequence mText
    pub(crate) mText: Arc<Mutex<CharSequence>>,
    // Java field: TextPaint mPaint
    pub(crate) mPaint: Arc<Mutex<TextPaint>>,
    // Java field: TextPaint mWorkPaint
    mWorkPaint: Option<TextPaint>,
    // Java field: int mWidth
    pub(crate) mWidth: i32,
    // Java field: float mOriWidth
    pub(crate) mOriWidth: f32,
    // Java field: int mAngleRectWidth
    pub(crate) mAngleRectWidth: i32,
    // Java field: Alignment mAlignment
    pub(crate) mAlignment: Arc<Mutex<Alignment>>,
    // Java field: float mSpacingMult
    pub(crate) mSpacingMult: f32,
    // Java field: float mSpacingAdd
    pub(crate) mSpacingAdd: f32,
    // Java field: boolean mSpannedText
    mSpannedText: bool,
    // Java field: int mTextDir
    pub(crate) mTextDir: i32,
    // Java field: int mJustificationMode
    mJustificationMode: i32,
    // Java field: int mVertical
    pub(crate) mVertical: i32,
    // Java field: boolean mIsRightToLeft
    pub(crate) mIsRightToLeft: bool,
    // Java field: short mAngle
    pub(crate) mAngle: i16,
    // Java field: List<Float> mLineHeights
    pub(crate) mLineHeights: Arc<Mutex<Vec<Float>>>,
    // Java field: List<Float> mLineWidths
    pub(crate) mLineWidths: Arc<Mutex<Vec<Float>>>,
    // Java field: float sin
    pub(crate) sin: f32,
    // Java field: float cos
    pub(crate) cos: f32,
    // Java field: float tan
    pub(crate) tan: f32,
    // Java field: int mAngleRectHeight
    pub(crate) mAngleRectHeight: i32,
}

// Java: public class HorizontalMeasurementProvider
pub struct HorizontalMeasurementProvider {
    // Java field: int mLine
    pub(crate) mLine: i32,
    // Java field: boolean mPrimary
    pub(crate) mPrimary: bool,
    // Java field: float[] mHorizontals
    pub(crate) mHorizontals: Arc<Mutex<Vec<f32>>>,
    // Java field: int mLineStartOffset
    pub(crate) mLineStartOffset: i32,
}

// Java: public class TabStops
pub struct TabStops {
    // Java field: int[] mStops
    pub(crate) mStops: Arc<Mutex<Vec<i32>>>,
    // Java field: int mNumStops
    pub(crate) mNumStops: i32,
    // Java field: int mIncrement
    pub(crate) mIncrement: i32,
}

// Java: public class Directions
pub struct Directions {
    // Java field: int[] mDirections
    pub(crate) mDirections: Arc<Mutex<Vec<i32>>>,
}

// Java: public enum Alignment
pub struct Alignment {
}

// Java: public class Layout
impl Layout {
    // Java: static float getDesiredWidth(CharSequence source, Paint paint)
    pub fn getDesiredWidth(&self, CharSequence source, Paint paint) -> f32 {
      // Java: return getDesiredWidth(source, 0, source.length(), paint);
      // TODO_RUST_CODE
    }

    // Java: static float getDesiredWidth(CharSequence source, int start, int end, Paint paint)
    pub fn getDesiredWidth(&self, CharSequence source, int start, int end, Paint paint) -> f32 {
      // Java: return getDesiredWidth(source, start, end, paint, 2);
      // TODO_RUST_CODE
    }

    // Java: static float getDesiredWidth(CharSequence source, int start, int end, Paint paint, int textDir)
    pub fn getDesiredWidth(&self, CharSequence source, int start, int end, Paint paint, int textDir) -> f32 {
      // Java: return getDesiredWidthWithLimit(source, start, end, paint, textDir, Float.MAX_VALUE);
      // TODO_RUST_CODE
    }

    // Java: static float getDesiredWidthWithLimit(CharSequence source, int start, int end, Paint paint, int textDir, float upperLimit)
    pub fn getDesiredWidthWithLimit(&self, CharSequence source, int start, int end, Paint paint, int textDir, float upperLimit) -> f32 {
      // Java: float need = 0.0F;
      // TODO_RUST_CODE
      // Java: int next;
      // TODO_RUST_CODE
      // Java: for(int i = start; i <= end; i = next)
        // TODO_RUST_CODE
      {
         // Java: next = TextUtils.indexOf(source, '\n', i, end);
         // TODO_RUST_CODE
         // Java: if (next < 0)
            // TODO_RUST_CODE
         {
            // Java: next = end;
            // TODO_RUST_CODE
         }
         // Java: float w = measurePara(paint, source, i, next, textDir);
         // TODO_RUST_CODE
         // Java: if (w > upperLimit)
            // TODO_RUST_CODE
         {
            // Java: return upperLimit;
            // TODO_RUST_CODE
         }
         // Java: if (w > need)
            // TODO_RUST_CODE
         {
            // Java: need = w;
            // TODO_RUST_CODE
         }
         // Java: ++next;
         // TODO_RUST_CODE
      }
      // Java: return need;
      // TODO_RUST_CODE
    }

    // Java: Layout(CharSequence text, TextPaint paint, float width, Alignment align, float spacingMult, float spacingAdd)
    pub fn Layout(&self, CharSequence text, TextPaint paint, float width, Alignment align, float spacingMult, float spacingAdd) {
      // Java: this(text, paint, width, align, 2, spacingMult, spacingAdd);
      // TODO_RUST_CODE
    }

    // Java: Layout(CharSequence text, TextPaint paint, float width, Alignment align, int textDir, float spacingMult, float spacingAdd)
    pub fn Layout(&self, CharSequence text, TextPaint paint, float width, Alignment align, int textDir, float spacingMult, float spacingAdd) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mWorkPaint = new TextPaint();
      // TODO_RUST_CODE
      // Java: this.mAlignment = Layout.Alignment.ALIGN_NORMAL;
      // TODO_RUST_CODE
      // Java: this.sin = 0.0F;
      // TODO_RUST_CODE
      // Java: this.cos = 0.0F;
      // TODO_RUST_CODE
      // Java: this.tan = 0.0F;
      // TODO_RUST_CODE
      // Java: if (width < 0.0F)
        // TODO_RUST_CODE
      {
         // Java: throw new IllegalArgumentException("Layout: " + width + " < 0");
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (paint != null)
            // TODO_RUST_CODE
         {
            // Java: paint.baselineShift = 0;
            // TODO_RUST_CODE
         }
         // Java: this.mText = text;
         // TODO_RUST_CODE
         // Java: this.mPaint = paint;
         // TODO_RUST_CODE
         // Java: this.mOriWidth = width;
         // TODO_RUST_CODE
         // Java: this.mWidth = (int)width;
         // TODO_RUST_CODE
         // Java: this.mAlignment = align;
         // TODO_RUST_CODE
         // Java: this.mSpacingMult = spacingMult;
         // TODO_RUST_CODE
         // Java: this.mSpacingAdd = spacingAdd;
         // TODO_RUST_CODE
         // Java: this.mSpannedText = text instanceof Spanned;
         // TODO_RUST_CODE
         // Java: this.mTextDir = textDir;
         // TODO_RUST_CODE
      }
    }

    // Java: void setJustificationMode(int justificationMode)
    pub fn setJustificationMode(&self, int justificationMode) {
      // Java: this.mJustificationMode = justificationMode;
      // TODO_RUST_CODE
    }

    // Java: void replaceWith(CharSequence text, TextPaint paint, int width, Alignment align, float spacingmult, float spacingadd)
    pub fn replaceWith(&self, CharSequence text, TextPaint paint, int width, Alignment align, float spacingmult, float spacingadd) {
      // Java: if (width < 0)
        // TODO_RUST_CODE
      {
         // Java: throw new IllegalArgumentException("Layout: " + width + " < 0");
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.mText = text;
         // TODO_RUST_CODE
         // Java: this.mPaint = paint;
         // TODO_RUST_CODE
         // Java: this.mWidth = width;
         // TODO_RUST_CODE
         // Java: this.mAlignment = align;
         // TODO_RUST_CODE
         // Java: this.mSpacingMult = spacingmult;
         // TODO_RUST_CODE
         // Java: this.mSpacingAdd = spacingadd;
         // TODO_RUST_CODE
         // Java: this.mSpannedText = text instanceof Spanned;
         // TODO_RUST_CODE
      }
    }

    // Java: void draw(Canvas c)
    pub fn draw(&self, Canvas c) {
      // Java: this.draw(c, (Path)null, (Paint)null, 0);
      // TODO_RUST_CODE
    }

    // Java: void draw(Canvas canvas, Path highlight, Paint highlightPaint, int cursorOffsetVertical)
    pub fn draw(&self, Canvas canvas, Path highlight, Paint highlightPaint, int cursorOffsetVertical) {
      // Java: long lineRange = this.getLineRangeForDraw(canvas);
      // TODO_RUST_CODE
      // Java: int firstLine = TextUtils.unpackRangeStartFromLong(lineRange);
      // TODO_RUST_CODE
      // Java: int lastLine = TextUtils.unpackRangeEndFromLong(lineRange);
      // TODO_RUST_CODE
      // Java: if (lastLine >= 0)
        // TODO_RUST_CODE
      {
         // Java: this.drawText(canvas, firstLine, lastLine);
         // TODO_RUST_CODE
      }
    }

    // Java: boolean isJustificationRequired(int lineNum)
    pub fn isJustificationRequired(&self, int lineNum) -> bool {
      // Java: if (this.mJustificationMode == 0)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int lineEnd = this.getLineEnd(lineNum);
         // TODO_RUST_CODE
         // Java: return lineEnd < this.mText.length() && this.mText.charAt(lineEnd - 1) != '\n';
         // TODO_RUST_CODE
      }
    }

    // Java: float getJustifyWidth(int lineNum)
    pub fn getJustifyWidth(&self, int lineNum) -> f32 {
      // Java: Alignment paraAlign = this.mAlignment;
      // TODO_RUST_CODE
      // Java: int left = 0;
      // TODO_RUST_CODE
      // Java: int right = this.mWidth;
      // TODO_RUST_CODE
      // Java: int dir = this.getParagraphDirection(lineNum);
      // TODO_RUST_CODE
      // Java: ParagraphStyle[] spans = NO_PARA_SPANS;
      // TODO_RUST_CODE
      // Java: if (this.mSpannedText)
        // TODO_RUST_CODE
      {
         // Java: Spanned sp = (Spanned)this.mText;
         // TODO_RUST_CODE
         // Java: int start = this.getLineStart(lineNum);
         // TODO_RUST_CODE
         // Java: boolean isFirstParaLine = start == 0 || this.mText.charAt(start - 1) == '\n';
         // TODO_RUST_CODE
         // Java: if (isFirstParaLine)
            // TODO_RUST_CODE
         {
            // Java: int spanEnd = sp.nextSpanTransition(start, this.mText.length(), ParagraphStyle.class);
            // TODO_RUST_CODE
            // Java: spans = (ParagraphStyle[])getParagraphSpans(sp, start, spanEnd, ParagraphStyle.class);
            // TODO_RUST_CODE
         }
         // Java: int length = spans.length;
         // TODO_RUST_CODE
         // Java: boolean useFirstLineMargin = isFirstParaLine;
         // TODO_RUST_CODE
         // Java: for(int n = 0; n < length; ++n)
            // TODO_RUST_CODE
         {
            // Java: if (spans[n] instanceof LeadingMarginSpan)
                // TODO_RUST_CODE
            {
               // Java: LeadingMarginSpan margin = (LeadingMarginSpan)spans[n];
               // TODO_RUST_CODE
               // Java: if (dir == -1)
                    // TODO_RUST_CODE
               {
                  // Java: right -= margin.getLeadingMargin(useFirstLineMargin);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: left += margin.getLeadingMargin(useFirstLineMargin);
                  // TODO_RUST_CODE
               }
            }
         }
      }
      // Java: Alignment align;
      // TODO_RUST_CODE
      // Java: if (paraAlign == Layout.Alignment.ALIGN_LEFT)
        // TODO_RUST_CODE
      {
         // Java: align = dir == 1 ? Layout.Alignment.ALIGN_NORMAL : Layout.Alignment.ALIGN_OPPOSITE;
         // TODO_RUST_CODE
      }
      // Java: else if (paraAlign == Layout.Alignment.ALIGN_RIGHT)
        // TODO_RUST_CODE
      {
         // Java: align = dir == 1 ? Layout.Alignment.ALIGN_OPPOSITE : Layout.Alignment.ALIGN_NORMAL;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: align = paraAlign;
         // TODO_RUST_CODE
      }
      // Java: int indentWidth;
      // TODO_RUST_CODE
      // Java: if (align != Layout.Alignment.ALIGN_NORMAL && align != Layout.Alignment.ALIGN_JUSTIFY)
        // TODO_RUST_CODE
      {
         // Java: if (align == Layout.Alignment.ALIGN_OPPOSITE)
            // TODO_RUST_CODE
         {
            // Java: if (dir == 1)
                // TODO_RUST_CODE
            {
               // Java: indentWidth = -this.getIndentAdjust(lineNum, Layout.Alignment.ALIGN_RIGHT);
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: indentWidth = this.getIndentAdjust(lineNum, Layout.Alignment.ALIGN_LEFT);
               // TODO_RUST_CODE
            }
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: indentWidth = this.getIndentAdjust(lineNum, Layout.Alignment.ALIGN_CENTER);
            // TODO_RUST_CODE
         }
      }
      // Java: else if (dir == 1)
        // TODO_RUST_CODE
      {
         // Java: indentWidth = this.getIndentAdjust(lineNum, Layout.Alignment.ALIGN_LEFT);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: indentWidth = -this.getIndentAdjust(lineNum, Layout.Alignment.ALIGN_RIGHT);
         // TODO_RUST_CODE
      }
      // Java: return (float)(right - left - indentWidth);
      // TODO_RUST_CODE
    }

    // Java: float getAlignmentOffset(int line)
    pub fn getAlignmentOffset(&self, int line) -> f32 {
      // Java: float alignmentOffset = 0.0F;
      // TODO_RUST_CODE
      // Java: switch (this.getAlignment())
        // TODO_RUST_CODE
      {
         // Java: case ALIGN_DISTRIBUTED:
         // TODO_RUST_CODE
         // Java: case ALIGN_CENTER:
         // TODO_RUST_CODE
            // Java: int left = this.getParagraphLeft(line);
            // TODO_RUST_CODE
            // Java: int right = this.getParagraphRight(line);
            // TODO_RUST_CODE
            // Java: int max = (int)this.getLineMax(line) & -2;
            // TODO_RUST_CODE
            // Java: alignmentOffset += (float)((right - left - max) / 2);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case ALIGN_OPPOSITE:
         // TODO_RUST_CODE
            // Java: alignmentOffset += (float)this.getWidth() - this.getLineMax(line);
            // TODO_RUST_CODE
      }
      // Java: return alignmentOffset;
      // TODO_RUST_CODE
    }

    // Java: void drawText(Canvas canvas, int firstLine, int lastLine)
    pub fn drawText(&self, Canvas canvas, int firstLine, int lastLine) {
      // Java: float previousLineBottom = this.getLineTop(firstLine);
      // TODO_RUST_CODE
      // Java: int previousLineEnd = this.getLineStart(firstLine);
      // TODO_RUST_CODE
      // Java: ParagraphStyle[] spans = NO_PARA_SPANS;
      // TODO_RUST_CODE
      // Java: int spanEnd = 0;
      // TODO_RUST_CODE
      // Java: Paint paint = this.mWorkPaint;
      // TODO_RUST_CODE
      // Java: paint.set(this.mPaint);
      // TODO_RUST_CODE
      // Java: CharSequence buf = this.mText;
      // TODO_RUST_CODE
      // Java: Alignment paraAlign = this.mAlignment;
      // TODO_RUST_CODE
      // Java: TabStops tabStops = null;
      // TODO_RUST_CODE
      // Java: boolean tabStopsIsInitialized = false;
      // TODO_RUST_CODE
      // Java: TextLine tl = TextLine.obtain();
      // TODO_RUST_CODE
      // Java: int count = canvas.save();
      // TODO_RUST_CODE
      // Java: for(int lineNum = firstLine; lineNum <= lastLine; ++lineNum)
        // TODO_RUST_CODE
      {
         // Java: this.setTextLineInfo(tl, lineNum);
         // TODO_RUST_CODE
         // Java: int start = previousLineEnd;
         // TODO_RUST_CODE
         // Java: previousLineEnd = this.getLineStart(lineNum + 1);
         // TODO_RUST_CODE
         // Java: boolean justify = this.isJustificationRequired(lineNum);
         // TODO_RUST_CODE
         // Java: int end = this.getLineVisibleEnd(lineNum, start, previousLineEnd);
         // TODO_RUST_CODE
         // Java: float ltop = previousLineBottom;
         // TODO_RUST_CODE
         // Java: float lbottom = this.getLineTop(lineNum + 1);
         // TODO_RUST_CODE
         // Java: previousLineBottom = lbottom;
         // TODO_RUST_CODE
         // Java: float lbaseline = lbottom - this.getLineDescent(lineNum);
         // TODO_RUST_CODE
         // Java: int dir = this.getParagraphDirection(lineNum);
         // TODO_RUST_CODE
         // Java: int left = 0;
         // TODO_RUST_CODE
         // Java: int right = this.mWidth;
         // TODO_RUST_CODE
         // Java: if (this.mSpannedText)
            // TODO_RUST_CODE
         {
            // Java: Spanned sp = (Spanned)buf;
            // TODO_RUST_CODE
            // Java: int textLength = buf.length();
            // TODO_RUST_CODE
            // Java: boolean isFirstParaLine = start == 0 || buf.charAt(start - 1) == '\n';
            // TODO_RUST_CODE
            // Java: if (start >= spanEnd && (lineNum == firstLine || isFirstParaLine))
                // TODO_RUST_CODE
            {
               // Java: spanEnd = sp.nextSpanTransition(start, textLength, ParagraphStyle.class);
               // TODO_RUST_CODE
               // Java: spans = (ParagraphStyle[])getParagraphSpans(sp, start, spanEnd, ParagraphStyle.class);
               // TODO_RUST_CODE
               // Java: paraAlign = this.mAlignment;
               // TODO_RUST_CODE
               // Java: tabStopsIsInitialized = false;
               // TODO_RUST_CODE
            }
            // Java: int length = spans.length;
            // TODO_RUST_CODE
            // Java: boolean useFirstLineMargin = isFirstParaLine;
            // TODO_RUST_CODE
            // Java: for(int n = 0; n < length; ++n)
                // TODO_RUST_CODE
            {
               // Java: if (spans[n] instanceof LeadingMarginSpan)
                    // TODO_RUST_CODE
               {
                  // Java: LeadingMarginSpan margin = (LeadingMarginSpan)spans[n];
                  // TODO_RUST_CODE
                  // Java: if (dir == -1)
                        // TODO_RUST_CODE
                  {
                     // Java: margin.drawLeadingMargin(canvas, paint, right, dir, ltop, lbaseline, lbottom, buf, start, end, isFirstParaLine, this);
                     // TODO_RUST_CODE
                     // Java: right -= margin.getLeadingMargin(useFirstLineMargin);
                     // TODO_RUST_CODE
                  }
                  // Java: else
                        // TODO_RUST_CODE
                  {
                     // Java: margin.drawLeadingMargin(canvas, paint, left, dir, ltop, lbaseline, lbottom, buf, start, end, isFirstParaLine, this);
                     // TODO_RUST_CODE
                     // Java: left += margin.getLeadingMargin(useFirstLineMargin);
                     // TODO_RUST_CODE
                  }
               }
            }
         }
         // Java: boolean hasTab = this.getLineContainsTab(lineNum);
         // TODO_RUST_CODE
         // Java: if (hasTab && !tabStopsIsInitialized)
            // TODO_RUST_CODE
         {
            // Java: if (tabStops == null)
                // TODO_RUST_CODE
            {
               // Java: tabStops = new TabStops(20, spans);
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: tabStops.reset(20, spans);
               // TODO_RUST_CODE
            }
            // Java: tabStopsIsInitialized = true;
            // TODO_RUST_CODE
         }
         // Java: int indentWidth = 0;
         // TODO_RUST_CODE
         // Java: float x = this.getAlignmentOffset(lineNum);
         // TODO_RUST_CODE
         // Java: if (!this.mIsRightToLeft)
            // TODO_RUST_CODE
         {
            // Java: x += (float)this.getLeadMargin(lineNum);
            // TODO_RUST_CODE
         }
         // Java: if (this.isAngle() && this.mAngle != 90 && this.mAngle != -90 && lineNum != 0)
            // TODO_RUST_CODE
         {
            // Java: if (this.mAngle > 0)
                // TODO_RUST_CODE
            {
               // Java: canvas.translate((Float)this.getLineHeights().get(lineNum) / this.tan, 0.0F);
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: canvas.translate(-(Float)this.getLineHeights().get(lineNum) / this.tan, 0.0F);
               // TODO_RUST_CODE
            }
         }
         // Java: Directions directions = this.getLineDirections(lineNum);
         // TODO_RUST_CODE
         // Java: if (directions == DIRS_ALL_LEFT_TO_RIGHT && !this.mSpannedText && !hasTab && !justify)
            // TODO_RUST_CODE
         {
            // Java: canvas.drawText(buf, start, end, x, lbaseline, paint);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: tl.set(paint, buf, start, end, dir, directions, hasTab, tabStops);
            // TODO_RUST_CODE
            // Java: if (justify)
                // TODO_RUST_CODE
            {
               // Java: tl.justify((float)(right - left - 0));
               // TODO_RUST_CODE
            }
            // Java: tl.draw(canvas, x, ltop, lbaseline, lbottom);
            // TODO_RUST_CODE
         }
      }
      // Java: canvas.restoreToCount(count);
      // TODO_RUST_CODE
      // Java: TextLine.recycle(tl);
      // TODO_RUST_CODE
    }

    // Java: long getLineRangeForDraw(Canvas canvas)
    pub fn getLineRangeForDraw(&self, Canvas canvas) -> i64 {
      // Java: if (!this.isAllVerticalText() && !this.isAngle())
        // TODO_RUST_CODE
      {
         // Java: int dtop;
         // TODO_RUST_CODE
         // Java: int dbottom;
         // TODO_RUST_CODE
         // Java: synchronized(sTempRect)
            // TODO_RUST_CODE
         {
            // Java: if (!canvas.getClipBounds(sTempRect))
                // TODO_RUST_CODE
            {
               // Java: return TextUtils.packRangeInLong(0, -1);
               // TODO_RUST_CODE
            }
            // Java: dtop = sTempRect.top;
            // TODO_RUST_CODE
            // Java: dbottom = sTempRect.bottom;
            // TODO_RUST_CODE
         }
         // Java: int top = Math.max(dtop, 0);
         // TODO_RUST_CODE
         // Java: int bottom = (int)Math.min(this.getLineTop(this.getLineCount()), (float)dbottom);
         // TODO_RUST_CODE
         // Java: return top >= bottom ? TextUtils.packRangeInLong(0, -1) : TextUtils.packRangeInLong(this.getLineForVertical(top), this.getLineForVertical(bottom));
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return TextUtils.packRangeInLong(0, this.getLineCount() - 1);
         // TODO_RUST_CODE
      }
    }

    // Java: int getLineStartPos(int line, int left, int right)
    pub fn getLineStartPos(&self, int line, int left, int right) -> i32 {
      // Java: Alignment align = this.getParagraphAlignment(line);
      // TODO_RUST_CODE
      // Java: int dir = this.getParagraphDirection(line);
      // TODO_RUST_CODE
      // Java: if (align == Layout.Alignment.ALIGN_LEFT)
        // TODO_RUST_CODE
      {
         // Java: align = dir == 1 ? Layout.Alignment.ALIGN_NORMAL : Layout.Alignment.ALIGN_OPPOSITE;
         // TODO_RUST_CODE
      }
      // Java: else if (align == Layout.Alignment.ALIGN_RIGHT)
        // TODO_RUST_CODE
      {
         // Java: align = dir == 1 ? Layout.Alignment.ALIGN_OPPOSITE : Layout.Alignment.ALIGN_NORMAL;
         // TODO_RUST_CODE
      }
      // Java: int x;
      // TODO_RUST_CODE
      // Java: if (align != Layout.Alignment.ALIGN_NORMAL && align != Layout.Alignment.ALIGN_JUSTIFY)
        // TODO_RUST_CODE
      {
         // Java: TabStops tabStops = null;
         // TODO_RUST_CODE
         // Java: int max = (int)this.getLineExtent(line, tabStops, false);
         // TODO_RUST_CODE
         // Java: if (align == Layout.Alignment.ALIGN_OPPOSITE)
            // TODO_RUST_CODE
         {
            // Java: if (dir == 1)
                // TODO_RUST_CODE
            {
               // Java: x = right - max + this.getIndentAdjust(line, Layout.Alignment.ALIGN_RIGHT);
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: x = left - max + this.getIndentAdjust(line, Layout.Alignment.ALIGN_LEFT);
               // TODO_RUST_CODE
            }
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: max &= -2;
            // TODO_RUST_CODE
            // Java: x = left + right - max >> 1 + this.getIndentAdjust(line, Layout.Alignment.ALIGN_CENTER);
            // TODO_RUST_CODE
         }
      }
      // Java: else if (dir == 1)
        // TODO_RUST_CODE
      {
         // Java: x = left + this.getIndentAdjust(line, Layout.Alignment.ALIGN_LEFT);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: x = right + this.getIndentAdjust(line, Layout.Alignment.ALIGN_RIGHT);
         // TODO_RUST_CODE
      }
      // Java: return x;
      // TODO_RUST_CODE
    }

    // Java: CharSequence getText()
    pub fn getText(&self) -> CharSequence {
      // Java: return this.mText;
      // TODO_RUST_CODE
    }

    // Java: TextPaint getPaint()
    pub fn getPaint(&self) -> TextPaint {
      // Java: return this.mPaint;
      // TODO_RUST_CODE
    }

    // Java: int getWidth()
    pub fn getWidth(&self) -> i32 {
      // Java: return this.mWidth;
      // TODO_RUST_CODE
    }

    // Java: float getOriWidth()
    pub fn getOriWidth(&self) -> f32 {
      // Java: return this.mOriWidth;
      // TODO_RUST_CODE
    }

    // Java: int getAngleRectWidth()
    pub fn getAngleRectWidth(&self) -> i32 {
      // Java: return this.mAngleRectWidth;
      // TODO_RUST_CODE
    }

    // Java: abstract int getEllipsisCount(int var1)
    pub fn getEllipsisCount(&self, int var1) -> i32 {
    // TODO_RUST_CODE
    }

    // Java: abstract int getEllipsisStart(int var1)
    pub fn getEllipsisStart(&self, int var1) -> i32 {
    // TODO_RUST_CODE
    }

    // Java: int getEllipsizedWidth()
    pub fn getEllipsizedWidth(&self) -> i32 {
      // Java: return this.mWidth;
      // TODO_RUST_CODE
    }

    // Java: void increaseWidthTo(int wid)
    pub fn increaseWidthTo(&self, int wid) {
      // Java: if (wid < this.mWidth)
        // TODO_RUST_CODE
      {
      }
      // Java: this.mWidth = wid;
      // TODO_RUST_CODE
    }

    // Java: float getHeight()
    pub fn getHeight(&self) -> f32 {
      // Java: return this.getLineTop(this.getLineCount());
      // TODO_RUST_CODE
    }

    // Java: float getHeight(boolean cap)
    pub fn getHeight(&self, boolean cap) -> f32 {
      // Java: return this.getHeight();
      // TODO_RUST_CODE
    }

    // Java: Alignment getAlignment()
    pub fn getAlignment(&self) -> Alignment {
      // Java: return this.mAlignment;
      // TODO_RUST_CODE
    }

    // Java: float getSpacingMultiplier()
    pub fn getSpacingMultiplier(&self) -> f32 {
      // Java: return this.mSpacingMult;
      // TODO_RUST_CODE
    }

    // Java: float getSpacingAdd()
    pub fn getSpacingAdd(&self) -> f32 {
      // Java: return this.mSpacingAdd;
      // TODO_RUST_CODE
    }

    // Java: int getTextDirectionHeuristic()
    pub fn getTextDirectionHeuristic(&self) -> i32 {
      // Java: return this.mTextDir;
      // TODO_RUST_CODE
    }

    // Java: abstract int getLineCount()
    pub fn getLineCount(&self) -> i32 {
    // TODO_RUST_CODE
    }

    // Java: int getLineBounds(int line, RectF bounds)
    pub fn getLineBounds(&self, int line, RectF bounds) -> i32 {
      // Java: if (bounds != null)
        // TODO_RUST_CODE
      {
         // Java: bounds.left = 0.0F;
         // TODO_RUST_CODE
         // Java: bounds.top = this.getLineTop(line);
         // TODO_RUST_CODE
         // Java: bounds.right = (float)this.mWidth;
         // TODO_RUST_CODE
         // Java: bounds.bottom = this.getLineTop(line + 1);
         // TODO_RUST_CODE
      }
      // Java: return (int)this.getLineBaseline(line);
      // TODO_RUST_CODE
    }

    // Java: abstract float getLineTop(int var1)
    pub fn getLineTop(&self, int var1) -> f32 {
    // TODO_RUST_CODE
    }

    // Java: abstract float getLineDescent(int var1)
    pub fn getLineDescent(&self, int var1) -> f32 {
    // TODO_RUST_CODE
    }

    // Java: abstract int getLineStart(int var1)
    pub fn getLineStart(&self, int var1) -> i32 {
    // TODO_RUST_CODE
    }

    // Java: abstract int getParagraphDirection(int var1)
    pub fn getParagraphDirection(&self, int var1) -> i32 {
    // TODO_RUST_CODE
    }

    // Java: abstract boolean getLineContainsTab(int var1)
    pub fn getLineContainsTab(&self, int var1) -> bool {
    // TODO_RUST_CODE
    }

    // Java: abstract Directions getLineDirections(int var1)
    pub fn getLineDirections(&self, int var1) -> Directions {
    // TODO_RUST_CODE
    }

    // Java: abstract int getTopPadding()
    pub fn getTopPadding(&self) -> i32 {
    // TODO_RUST_CODE
    }

    // Java: abstract int getBottomPadding()
    pub fn getBottomPadding(&self) -> i32 {
    // TODO_RUST_CODE
    }

    // Java: abstract int getAngleRectHeight()
    pub fn getAngleRectHeight(&self) -> i32 {
    // TODO_RUST_CODE
    }

    // Java: int getHyphen(int line)
    pub fn getHyphen(&self, int line) -> i32 {
      // Java: return 0;
      // TODO_RUST_CODE
    }

    // Java: int getIndentAdjust(int line, Alignment alignment)
    pub fn getIndentAdjust(&self, int line, Alignment alignment) -> i32 {
      // Java: return 0;
      // TODO_RUST_CODE
    }

    // Java: boolean isLevelBoundary(int offset)
    pub fn isLevelBoundary(&self, int offset) -> bool {
      // Java: int line = this.getLineForOffset(offset);
      // TODO_RUST_CODE
      // Java: Directions dirs = this.getLineDirections(line);
      // TODO_RUST_CODE
      // Java: if (dirs != DIRS_ALL_LEFT_TO_RIGHT && dirs != DIRS_ALL_RIGHT_TO_LEFT)
        // TODO_RUST_CODE
      {
         // Java: int[] runs = dirs.mDirections;
         // TODO_RUST_CODE
         // Java: int lineStart = this.getLineStart(line);
         // TODO_RUST_CODE
         // Java: int lineEnd = this.getLineEnd(line);
         // TODO_RUST_CODE
         // Java: if (offset != lineStart && offset != lineEnd)
            // TODO_RUST_CODE
         {
            // Java: offset -= lineStart;
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < runs.length; i += 2)
                // TODO_RUST_CODE
            {
               // Java: if (offset == runs[i])
                    // TODO_RUST_CODE
               {
                  // Java: return true;
                  // TODO_RUST_CODE
               }
            }
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: int paraLevel = this.getParagraphDirection(line) == 1 ? 0 : 1;
            // TODO_RUST_CODE
            // Java: int runIndex = offset == lineStart ? 0 : runs.length - 2;
            // TODO_RUST_CODE
            // Java: return (runs[runIndex + 1] >>> 26 & 63) != paraLevel;
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

    // Java: boolean isRtlCharAt(int offset)
    pub fn isRtlCharAt(&self, int offset) -> bool {
      // Java: int line = this.getLineForOffset(offset);
      // TODO_RUST_CODE
      // Java: Directions dirs = this.getLineDirections(line);
      // TODO_RUST_CODE
      // Java: if (dirs == DIRS_ALL_LEFT_TO_RIGHT)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else if (dirs == DIRS_ALL_RIGHT_TO_LEFT)
        // TODO_RUST_CODE
      {
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int[] runs = dirs.mDirections;
         // TODO_RUST_CODE
         // Java: int lineStart = this.getLineStart(line);
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < runs.length; i += 2)
            // TODO_RUST_CODE
         {
            // Java: int start = lineStart + runs[i];
            // TODO_RUST_CODE
            // Java: int limit = start + (runs[i + 1] & 67108863);
            // TODO_RUST_CODE
            // Java: if (offset >= start && offset < limit)
                // TODO_RUST_CODE
            {
               // Java: int level = runs[i + 1] >>> 26 & 63;
               // TODO_RUST_CODE
               // Java: return (level & 1) != 0;
               // TODO_RUST_CODE
            }
         }
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: long getRunRange(int offset)
    pub fn getRunRange(&self, int offset) -> i64 {
      // Java: int line = this.getLineForOffset(offset);
      // TODO_RUST_CODE
      // Java: Directions dirs = this.getLineDirections(line);
      // TODO_RUST_CODE
      // Java: if (dirs != DIRS_ALL_LEFT_TO_RIGHT && dirs != DIRS_ALL_RIGHT_TO_LEFT)
        // TODO_RUST_CODE
      {
         // Java: int[] runs = dirs.mDirections;
         // TODO_RUST_CODE
         // Java: int lineStart = this.getLineStart(line);
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < runs.length; i += 2)
            // TODO_RUST_CODE
         {
            // Java: int start = lineStart + runs[i];
            // TODO_RUST_CODE
            // Java: int limit = start + (runs[i + 1] & 67108863);
            // TODO_RUST_CODE
            // Java: if (offset >= start && offset < limit)
                // TODO_RUST_CODE
            {
               // Java: return TextUtils.packRangeInLong(start, limit);
               // TODO_RUST_CODE
            }
         }
         // Java: return TextUtils.packRangeInLong(0, this.getLineEnd(line));
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return TextUtils.packRangeInLong(0, this.getLineEnd(line));
         // TODO_RUST_CODE
      }
    }

    // Java: boolean primaryIsTrailingPrevious(int offset)
    pub fn primaryIsTrailingPrevious(&self, int offset) -> bool {
      // Java: int line = this.getLineForOffset(offset);
      // TODO_RUST_CODE
      // Java: int lineStart = this.getLineStart(line);
      // TODO_RUST_CODE
      // Java: int lineEnd = this.getLineEnd(line);
      // TODO_RUST_CODE
      // Java: int[] runs = this.getLineDirections(line).mDirections;
      // TODO_RUST_CODE
      // Java: int levelAt = -1;
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < runs.length; i += 2)
        // TODO_RUST_CODE
      {
         // Java: int start = lineStart + runs[i];
         // TODO_RUST_CODE
         // Java: int limit = start + (runs[i + 1] & 67108863);
         // TODO_RUST_CODE
         // Java: if (limit > lineEnd)
            // TODO_RUST_CODE
         {
            // Java: limit = lineEnd;
            // TODO_RUST_CODE
         }
         // Java: if (offset >= start && offset < limit)
            // TODO_RUST_CODE
         {
            // Java: if (offset > start)
                // TODO_RUST_CODE
            {
               // Java: return false;
               // TODO_RUST_CODE
            }
            // Java: levelAt = runs[i + 1] >>> 26 & 63;
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         }
      }
      // Java: if (levelAt == -1)
        // TODO_RUST_CODE
      {
         // Java: levelAt = this.getParagraphDirection(line) == 1 ? 0 : 1;
         // TODO_RUST_CODE
      }
      // Java: int levelBefore = -1;
      // TODO_RUST_CODE
      // Java: if (offset == lineStart)
        // TODO_RUST_CODE
      {
         // Java: levelBefore = this.getParagraphDirection(line) == 1 ? 0 : 1;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: --offset;
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < runs.length; i += 2)
            // TODO_RUST_CODE
         {
            // Java: int start = lineStart + runs[i];
            // TODO_RUST_CODE
            // Java: int limit = start + (runs[i + 1] & 67108863);
            // TODO_RUST_CODE
            // Java: if (limit > lineEnd)
                // TODO_RUST_CODE
            {
               // Java: limit = lineEnd;
               // TODO_RUST_CODE
            }
            // Java: if (offset >= start && offset < limit)
                // TODO_RUST_CODE
            {
               // Java: levelBefore = runs[i + 1] >>> 26 & 63;
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            }
         }
      }
      // Java: return levelBefore < levelAt;
      // TODO_RUST_CODE
    }

    // Java: boolean[] primaryIsTrailingPreviousAllLineOffsets(int line)
    pub fn primaryIsTrailingPreviousAllLineOffsets(&self, int line) -> boolean[] {
      // Java: int lineStart = this.getLineStart(line);
      // TODO_RUST_CODE
      // Java: int lineEnd = this.getLineEnd(line);
      // TODO_RUST_CODE
      // Java: int[] runs = this.getLineDirections(line).mDirections;
      // TODO_RUST_CODE
      // Java: boolean[] trailing = new boolean[lineEnd - lineStart + 1];
      // TODO_RUST_CODE
      // Java: byte[] level = new byte[lineEnd - lineStart + 1];
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < runs.length; i += 2)
        // TODO_RUST_CODE
      {
         // Java: int start = lineStart + runs[i];
         // TODO_RUST_CODE
         // Java: int limit = start + (runs[i + 1] & 67108863);
         // TODO_RUST_CODE
         // Java: if (limit > lineEnd)
            // TODO_RUST_CODE
         {
            // Java: limit = lineEnd;
            // TODO_RUST_CODE
         }
         // Java: level[limit - lineStart - 1] = (byte)(runs[i + 1] >>> 26 & 63);
         // TODO_RUST_CODE
      }
      // Java: for(int i = 0; i < runs.length; i += 2)
        // TODO_RUST_CODE
      {
         // Java: int start = lineStart + runs[i];
         // TODO_RUST_CODE
         // Java: byte currentLevel = (byte)(runs[i + 1] >>> 26 & 63);
         // TODO_RUST_CODE
         // Java: trailing[start - lineStart] = currentLevel > (start == lineStart ? (this.getParagraphDirection(line) == 1 ? 0 : 1) : level[start - lineStart - 1]);
         // TODO_RUST_CODE
      }
      // Java: return trailing;
      // TODO_RUST_CODE
    }

    // Java: float getPrimaryHorizontal(int offset)
    pub fn getPrimaryHorizontal(&self, int offset) -> f32 {
      // Java: return this.getPrimaryHorizontal(offset, false);
      // TODO_RUST_CODE
    }

    // Java: float getPrimaryHorizontal(int offset, boolean clamped)
    pub fn getPrimaryHorizontal(&self, int offset, boolean clamped) -> f32 {
      // Java: boolean trailing = this.primaryIsTrailingPrevious(offset);
      // TODO_RUST_CODE
      // Java: return this.getHorizontal(offset, trailing, clamped);
      // TODO_RUST_CODE
    }

    // Java: float getSecondaryHorizontal(int offset)
    pub fn getSecondaryHorizontal(&self, int offset) -> f32 {
      // Java: return this.getSecondaryHorizontal(offset, false);
      // TODO_RUST_CODE
    }

    // Java: float getSecondaryHorizontal(int offset, boolean clamped)
    pub fn getSecondaryHorizontal(&self, int offset, boolean clamped) -> f32 {
      // Java: boolean trailing = this.primaryIsTrailingPrevious(offset);
      // TODO_RUST_CODE
      // Java: return this.getHorizontal(offset, !trailing, clamped);
      // TODO_RUST_CODE
    }

    // Java: float getHorizontal(int offset, boolean primary)
    pub fn getHorizontal(&self, int offset, boolean primary) -> f32 {
      // Java: return primary ? this.getPrimaryHorizontal(offset) : this.getSecondaryHorizontal(offset);
      // TODO_RUST_CODE
    }

    // Java: float getHorizontal(int offset, boolean trailing, boolean clamped)
    pub fn getHorizontal(&self, int offset, boolean trailing, boolean clamped) -> f32 {
      // Java: int line = this.getLineForOffset(offset);
      // TODO_RUST_CODE
      // Java: return this.getHorizontal(offset, trailing, line, clamped);
      // TODO_RUST_CODE
    }

    // Java: float getHorizontal(int offset, boolean trailing, int line, boolean clamped)
    pub fn getHorizontal(&self, int offset, boolean trailing, int line, boolean clamped) -> f32 {
      // Java: int start = this.getLineStart(line);
      // TODO_RUST_CODE
      // Java: int end = this.getLineEnd(line);
      // TODO_RUST_CODE
      // Java: int dir = this.getParagraphDirection(line);
      // TODO_RUST_CODE
      // Java: boolean hasTab = this.getLineContainsTab(line);
      // TODO_RUST_CODE
      // Java: Directions directions = this.getLineDirections(line);
      // TODO_RUST_CODE
      // Java: TabStops tabStops = null;
      // TODO_RUST_CODE
      // Java: if (hasTab && this.mText instanceof Spanned)
        // TODO_RUST_CODE
      {
      }
      // Java: TextLine tl = TextLine.obtain();
      // TODO_RUST_CODE
      // Java: this.setTextLineInfo(tl, line);
      // TODO_RUST_CODE
      // Java: tl.set(this.mPaint, this.mText, start, end, dir, directions, hasTab, tabStops);
      // TODO_RUST_CODE
      // Java: float wid = tl.measure(offset - start, trailing, (Paint.FontMetrics)null);
      // TODO_RUST_CODE
      // Java: TextLine.recycle(tl);
      // TODO_RUST_CODE
      // Java: if (clamped && wid > (float)this.mWidth)
        // TODO_RUST_CODE
      {
         // Java: wid = (float)this.mWidth;
         // TODO_RUST_CODE
      }
      // Java: int left = this.getParagraphLeft(line);
      // TODO_RUST_CODE
      // Java: int right = this.getParagraphRight(line);
      // TODO_RUST_CODE
      // Java: return (float)this.getLineStartPos(line, left, right) + wid;
      // TODO_RUST_CODE
    }

    // Java: float[] getLineHorizontals(int line, boolean clamped, boolean primary)
    pub fn getLineHorizontals(&self, int line, boolean clamped, boolean primary) -> float[] {
      // Java: int start = this.getLineStart(line);
      // TODO_RUST_CODE
      // Java: int end = this.getLineEnd(line);
      // TODO_RUST_CODE
      // Java: int dir = this.getParagraphDirection(line);
      // TODO_RUST_CODE
      // Java: boolean hasTab = this.getLineContainsTab(line);
      // TODO_RUST_CODE
      // Java: Directions directions = this.getLineDirections(line);
      // TODO_RUST_CODE
      // Java: TabStops tabStops = null;
      // TODO_RUST_CODE
      // Java: TextLine tl = TextLine.obtain();
      // TODO_RUST_CODE
      // Java: this.setTextLineInfo(tl, line);
      // TODO_RUST_CODE
      // Java: tl.set(this.mPaint, this.mText, start, end, dir, directions, hasTab, tabStops);
      // TODO_RUST_CODE
      // Java: boolean[] trailings = this.primaryIsTrailingPreviousAllLineOffsets(line);
      // TODO_RUST_CODE
      // Java: if (!primary)
        // TODO_RUST_CODE
      {
         // Java: for(int offset = 0; offset < trailings.length; ++offset)
            // TODO_RUST_CODE
         {
            // Java: trailings[offset] = !trailings[offset];
            // TODO_RUST_CODE
         }
      }
      // Java: float[] wid = tl.measureAllOffsets(trailings, (Paint.FontMetrics)null);
      // TODO_RUST_CODE
      // Java: TextLine.recycle(tl);
      // TODO_RUST_CODE
      // Java: if (clamped)
        // TODO_RUST_CODE
      {
         // Java: for(int offset = 0; offset <= wid.length; ++offset)
            // TODO_RUST_CODE
         {
            // Java: if (wid[offset] > (float)this.mWidth)
                // TODO_RUST_CODE
            {
               // Java: wid[offset] = (float)this.mWidth;
               // TODO_RUST_CODE
            }
         }
      }
      // Java: int left = this.getParagraphLeft(line);
      // TODO_RUST_CODE
      // Java: int right = this.getParagraphRight(line);
      // TODO_RUST_CODE
      // Java: int lineStartPos = this.getLineStartPos(line, left, right);
      // TODO_RUST_CODE
      // Java: float[] horizontal = new float[end - start + 1];
      // TODO_RUST_CODE
      // Java: for(int offset = 0; offset < horizontal.length; ++offset)
        // TODO_RUST_CODE
      {
         // Java: horizontal[offset] = (float)lineStartPos + wid[offset];
         // TODO_RUST_CODE
      }
      // Java: return horizontal;
      // TODO_RUST_CODE
    }

    // Java: float getLineLeft(int line)
    pub fn getLineLeft(&self, int line) -> f32 {
      // Java: int dir = this.getParagraphDirection(line);
      // TODO_RUST_CODE
      // Java: Alignment align = this.getParagraphAlignment(line);
      // TODO_RUST_CODE
      // Java: if (align != Layout.Alignment.ALIGN_LEFT && align != Layout.Alignment.ALIGN_JUSTIFY)
        // TODO_RUST_CODE
      {
         // Java: if (align == Layout.Alignment.ALIGN_NORMAL)
            // TODO_RUST_CODE
         {
            // Java: return dir == -1 ? (float)this.getParagraphRight(line) - this.getLineMax(line) : 0.0F;
            // TODO_RUST_CODE
         }
         // Java: else if (align == Layout.Alignment.ALIGN_RIGHT)
            // TODO_RUST_CODE
         {
            // Java: return (float)this.mWidth - this.getLineMax(line);
            // TODO_RUST_CODE
         }
         // Java: else if (align == Layout.Alignment.ALIGN_OPPOSITE)
            // TODO_RUST_CODE
         {
            // Java: return dir == -1 ? 0.0F : (float)this.mWidth - this.getLineMax(line);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: int left = this.getParagraphLeft(line);
            // TODO_RUST_CODE
            // Java: int right = this.getParagraphRight(line);
            // TODO_RUST_CODE
            // Java: int max = (int)this.getLineMax(line) & -2;
            // TODO_RUST_CODE
            // Java: return (float)(left + (right - left - max) / 2);
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return 0.0F;
         // TODO_RUST_CODE
      }
    }

    // Java: float getLineRight(int line)
    pub fn getLineRight(&self, int line) -> f32 {
      // Java: int dir = this.getParagraphDirection(line);
      // TODO_RUST_CODE
      // Java: Alignment align = this.getParagraphAlignment(line);
      // TODO_RUST_CODE
      // Java: if (align == Layout.Alignment.ALIGN_LEFT)
        // TODO_RUST_CODE
      {
         // Java: return (float)this.getParagraphLeft(line) + this.getLineMax(line);
         // TODO_RUST_CODE
      }
      // Java: else if (align == Layout.Alignment.ALIGN_NORMAL)
        // TODO_RUST_CODE
      {
         // Java: return dir == -1 ? (float)this.mWidth : (float)this.getParagraphLeft(line) + this.getLineMax(line);
         // TODO_RUST_CODE
      }
      // Java: else if (align == Layout.Alignment.ALIGN_RIGHT)
        // TODO_RUST_CODE
      {
         // Java: return (float)this.mWidth;
         // TODO_RUST_CODE
      }
      // Java: else if (align == Layout.Alignment.ALIGN_OPPOSITE)
        // TODO_RUST_CODE
      {
         // Java: return dir == -1 ? this.getLineMax(line) : (float)this.mWidth;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int left = this.getParagraphLeft(line);
         // TODO_RUST_CODE
         // Java: int right = this.getParagraphRight(line);
         // TODO_RUST_CODE
         // Java: int max = (int)this.getLineMax(line) & -2;
         // TODO_RUST_CODE
         // Java: return (float)(right - (right - left - max) / 2);
         // TODO_RUST_CODE
      }
    }

    // Java: float getLineMax(int line)
    pub fn getLineMax(&self, int line) -> f32 {
      // Java: float margin = (float)this.getParagraphLeadingMargin(line);
      // TODO_RUST_CODE
      // Java: float signedExtent = this.getLineExtent(line, false);
      // TODO_RUST_CODE
      // Java: return margin + (signedExtent >= 0.0F ? signedExtent : -signedExtent);
      // TODO_RUST_CODE
    }

    // Java: float getLineWidth(int line)
    pub fn getLineWidth(&self, int line) -> f32 {
      // Java: float margin = (float)this.getParagraphLeadingMargin(line);
      // TODO_RUST_CODE
      // Java: float signedExtent = this.getLineExtent(line, true);
      // TODO_RUST_CODE
      // Java: return margin + (signedExtent >= 0.0F ? signedExtent : -signedExtent);
      // TODO_RUST_CODE
    }

    // Java: float getLineExtent(int line, boolean full)
    pub fn getLineExtent(&self, int line, boolean full) -> f32 {
      // Java: int start = this.getLineStart(line);
      // TODO_RUST_CODE
      // Java: int end = full ? this.getLineEnd(line) : this.getLineVisibleEnd(line);
      // TODO_RUST_CODE
      // Java: float[] charWidths = this.getCharWidths();
      // TODO_RUST_CODE
      // Java: float width = 0.0F;
      // TODO_RUST_CODE
      // Java: for(int i = start; i < end; ++i)
        // TODO_RUST_CODE
      {
         // Java: width += charWidths[i];
         // TODO_RUST_CODE
      }
      // Java: return width;
      // TODO_RUST_CODE
    }

    // Java: float getLineExtent(int line, TabStops tabStops, boolean full)
    pub fn getLineExtent(&self, int line, TabStops tabStops, boolean full) -> f32 {
      // Java: int start = this.getLineStart(line);
      // TODO_RUST_CODE
      // Java: int end = full ? this.getLineEnd(line) : this.getLineVisibleEnd(line);
      // TODO_RUST_CODE
      // Java: float[] charWidths = this.getCharWidths();
      // TODO_RUST_CODE
      // Java: float width = 0.0F;
      // TODO_RUST_CODE
      // Java: for(int i = start; i < end; ++i)
        // TODO_RUST_CODE
      {
         // Java: width += charWidths[i];
         // TODO_RUST_CODE
      }
      // Java: return width;
      // TODO_RUST_CODE
    }

    // Java: int getLineForVertical(int vertical)
    pub fn getLineForVertical(&self, int vertical) -> i32 {
      // Java: int high = this.getLineCount();
      // TODO_RUST_CODE
      // Java: int low = -1;
      // TODO_RUST_CODE
      // Java: while(high - low > 1)
        // TODO_RUST_CODE
      {
         // Java: int guess = (high + low) / 2;
         // TODO_RUST_CODE
         // Java: if (this.getLineTop(guess) > (float)vertical)
            // TODO_RUST_CODE
         {
            // Java: high = guess;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: low = guess;
            // TODO_RUST_CODE
         }
      }
      // Java: if (low < 0)
        // TODO_RUST_CODE
      {
         // Java: return 0;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return low;
         // TODO_RUST_CODE
      }
    }

    // Java: int getLineForOffset(int offset)
    pub fn getLineForOffset(&self, int offset) -> i32 {
      // Java: int high = this.getLineCount();
      // TODO_RUST_CODE
      // Java: int low = -1;
      // TODO_RUST_CODE
      // Java: while(high - low > 1)
        // TODO_RUST_CODE
      {
         // Java: int guess = (high + low) / 2;
         // TODO_RUST_CODE
         // Java: if (this.getLineStart(guess) > offset)
            // TODO_RUST_CODE
         {
            // Java: high = guess;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: low = guess;
            // TODO_RUST_CODE
         }
      }
      // Java: if (low < 0)
        // TODO_RUST_CODE
      {
         // Java: return 0;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return low;
         // TODO_RUST_CODE
      }
    }

    // Java: int getOffsetForHorizontal(int line, float horiz)
    pub fn getOffsetForHorizontal(&self, int line, float horiz) -> i32 {
      // Java: return this.getOffsetForHorizontal(line, horiz, true);
      // TODO_RUST_CODE
    }

    // Java: int getOffsetForHorizontal(int line, float horiz, boolean primary)
    pub fn getOffsetForHorizontal(&self, int line, float horiz, boolean primary) -> i32 {
      // Java: int lineEndOffset = this.getLineEnd(line);
      // TODO_RUST_CODE
      // Java: int lineStartOffset = this.getLineStart(line);
      // TODO_RUST_CODE
      // Java: Directions dirs = this.getLineDirections(line);
      // TODO_RUST_CODE
      // Java: TextLine tl = TextLine.obtain();
      // TODO_RUST_CODE
      // Java: this.setTextLineInfo(tl, line);
      // TODO_RUST_CODE
      // Java: tl.set(this.mPaint, this.mText, lineStartOffset, lineEndOffset, this.getParagraphDirection(line), dirs, false, (TabStops)null);
      // TODO_RUST_CODE
      // Java: HorizontalMeasurementProvider horizontal = new HorizontalMeasurementProvider(line, primary);
      // TODO_RUST_CODE
      // Java: int max;
      // TODO_RUST_CODE
      // Java: if (line == this.getLineCount() - 1)
        // TODO_RUST_CODE
      {
         // Java: max = lineEndOffset;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: max = tl.getOffsetToLeftRightOf(lineEndOffset - lineStartOffset, !this.isRtlCharAt(lineEndOffset - 1)) + lineStartOffset;
         // TODO_RUST_CODE
      }
      // Java: int best = lineStartOffset;
      // TODO_RUST_CODE
      // Java: float bestdist = Math.abs(horizontal.get(lineStartOffset) - horiz);
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < dirs.mDirections.length; i += 2)
        // TODO_RUST_CODE
      {
         // Java: int here = lineStartOffset + dirs.mDirections[i];
         // TODO_RUST_CODE
         // Java: int there = here + (dirs.mDirections[i + 1] & 67108863);
         // TODO_RUST_CODE
         // Java: boolean isRtl = (dirs.mDirections[i + 1] & 67108864) != 0;
         // TODO_RUST_CODE
         // Java: int swap = isRtl ? -1 : 1;
         // TODO_RUST_CODE
         // Java: if (there > max)
            // TODO_RUST_CODE
         {
            // Java: there = max;
            // TODO_RUST_CODE
         }
         // Java: int high = there - 1 + 1;
         // TODO_RUST_CODE
         // Java: int low = here + 1 - 1;
         // TODO_RUST_CODE
         // Java: while(high - low > 1)
            // TODO_RUST_CODE
         {
            // Java: int guess = (high + low) / 2;
            // TODO_RUST_CODE
            // Java: int adguess = this.getOffsetAtStartOf(guess);
            // TODO_RUST_CODE
            // Java: if (horizontal.get(adguess) * (float)swap >= horiz * (float)swap)
                // TODO_RUST_CODE
            {
               // Java: high = guess;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: low = guess;
               // TODO_RUST_CODE
            }
         }
         // Java: if (low < here + 1)
            // TODO_RUST_CODE
         {
            // Java: low = here + 1;
            // TODO_RUST_CODE
         }
         // Java: if (low < there)
            // TODO_RUST_CODE
         {
            // Java: int aft = tl.getOffsetToLeftRightOf(low - lineStartOffset, isRtl) + lineStartOffset;
            // TODO_RUST_CODE
            // Java: low = tl.getOffsetToLeftRightOf(aft - lineStartOffset, !isRtl) + lineStartOffset;
            // TODO_RUST_CODE
            // Java: if (low >= here && low < there)
                // TODO_RUST_CODE
            {
               // Java: float dist = Math.abs(horizontal.get(low) - horiz);
               // TODO_RUST_CODE
               // Java: if (aft < there)
                    // TODO_RUST_CODE
               {
                  // Java: float other = Math.abs(horizontal.get(aft) - horiz);
                  // TODO_RUST_CODE
                  // Java: if (other < dist)
                        // TODO_RUST_CODE
                  {
                     // Java: dist = other;
                     // TODO_RUST_CODE
                     // Java: low = aft;
                     // TODO_RUST_CODE
                  }
               }
               // Java: if (dist < bestdist)
                    // TODO_RUST_CODE
               {
                  // Java: bestdist = dist;
                  // TODO_RUST_CODE
                  // Java: best = low;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: float dist = Math.abs(horizontal.get(here) - horiz);
         // TODO_RUST_CODE
         // Java: if (dist < bestdist)
            // TODO_RUST_CODE
         {
            // Java: bestdist = dist;
            // TODO_RUST_CODE
            // Java: best = here;
            // TODO_RUST_CODE
         }
      }
      // Java: float dist = Math.abs(horizontal.get(max) - horiz);
      // TODO_RUST_CODE
      // Java: if (dist <= bestdist)
        // TODO_RUST_CODE
      {
         // Java: best = max;
         // TODO_RUST_CODE
      }
      // Java: TextLine.recycle(tl);
      // TODO_RUST_CODE
      // Java: return best;
      // TODO_RUST_CODE
    }

    // Java: int getLineEnd(int line)
    pub fn getLineEnd(&self, int line) -> i32 {
      // Java: return this.getLineStart(line + 1);
      // TODO_RUST_CODE
    }

    // Java: int getLineVisibleEnd(int line)
    pub fn getLineVisibleEnd(&self, int line) -> i32 {
      // Java: return this.getLineVisibleEnd(line, this.getLineStart(line), this.getLineStart(line + 1));
      // TODO_RUST_CODE
    }

    // Java: int getLineVisibleEnd(int line, int start, int end)
    pub fn getLineVisibleEnd(&self, int line, int start, int end) -> i32 {
      // Java: CharSequence text = this.mText;
      // TODO_RUST_CODE
      // Java: if (line == this.getLineCount() - 1)
        // TODO_RUST_CODE
      {
         // Java: return end;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: while(end > start)
            // TODO_RUST_CODE
         {
            // Java: char ch = text.charAt(end - 1);
            // TODO_RUST_CODE
            // Java: if (ch == '\n')
                // TODO_RUST_CODE
            {
               // Java: return end - 1;
               // TODO_RUST_CODE
            }
            // Java: if (!TextLine.isLineEndSpace(ch))
                // TODO_RUST_CODE
            {
               // Java: break;
               // TODO_RUST_CODE
            }
            // Java: --end;
            // TODO_RUST_CODE
         }
         // Java: return end;
         // TODO_RUST_CODE
      }
    }

    // Java: float getLineBottom(int line)
    pub fn getLineBottom(&self, int line) -> f32 {
      // Java: return this.getLineTop(line + 1);
      // TODO_RUST_CODE
    }

    // Java: float getLineBottomWithoutSpacing(int line)
    pub fn getLineBottomWithoutSpacing(&self, int line) -> f32 {
      // Java: return this.getLineTop(line + 1) - (float)this.getLineExtra(line);
      // TODO_RUST_CODE
    }

    // Java: float getLineBaseline(int line)
    pub fn getLineBaseline(&self, int line) -> f32 {
      // Java: return this.getLineTop(line + 1) - this.getLineDescent(line);
      // TODO_RUST_CODE
    }

    // Java: float getLineAscent(int line)
    pub fn getLineAscent(&self, int line) -> f32 {
      // Java: return this.getLineTop(line) - (this.getLineTop(line + 1) - this.getLineDescent(line));
      // TODO_RUST_CODE
    }

    // Java: int getLineExtra(int line)
    pub fn getLineExtra(&self, int line) -> i32 {
      // Java: return 0;
      // TODO_RUST_CODE
    }

    // Java: int getOffsetToLeftOf(int offset)
    pub fn getOffsetToLeftOf(&self, int offset) -> i32 {
      // Java: return this.getOffsetToLeftRightOf(offset, true);
      // TODO_RUST_CODE
    }

    // Java: int getOffsetToRightOf(int offset)
    pub fn getOffsetToRightOf(&self, int offset) -> i32 {
      // Java: return this.getOffsetToLeftRightOf(offset, false);
      // TODO_RUST_CODE
    }

    // Java: int getOffsetToLeftRightOf(int caret, boolean toLeft)
    pub fn getOffsetToLeftRightOf(&self, int caret, boolean toLeft) -> i32 {
      // Java: int line = this.getLineForOffset(caret);
      // TODO_RUST_CODE
      // Java: int lineStart = this.getLineStart(line);
      // TODO_RUST_CODE
      // Java: int lineEnd = this.getLineEnd(line);
      // TODO_RUST_CODE
      // Java: int lineDir = this.getParagraphDirection(line);
      // TODO_RUST_CODE
      // Java: boolean lineChanged = false;
      // TODO_RUST_CODE
      // Java: boolean advance = toLeft == (lineDir == -1);
      // TODO_RUST_CODE
      // Java: if (advance)
        // TODO_RUST_CODE
      {
         // Java: if (caret == lineEnd)
            // TODO_RUST_CODE
         {
            // Java: if (line >= this.getLineCount() - 1)
                // TODO_RUST_CODE
            {
               // Java: return caret;
               // TODO_RUST_CODE
            }
            // Java: lineChanged = true;
            // TODO_RUST_CODE
            // Java: ++line;
            // TODO_RUST_CODE
         }
      }
      // Java: else if (caret == lineStart)
        // TODO_RUST_CODE
      {
         // Java: if (line <= 0)
            // TODO_RUST_CODE
         {
            // Java: return caret;
            // TODO_RUST_CODE
         }
         // Java: lineChanged = true;
         // TODO_RUST_CODE
         // Java: --line;
         // TODO_RUST_CODE
      }
      // Java: if (lineChanged)
        // TODO_RUST_CODE
      {
         // Java: lineStart = this.getLineStart(line);
         // TODO_RUST_CODE
         // Java: lineEnd = this.getLineEnd(line);
         // TODO_RUST_CODE
         // Java: int newDir = this.getParagraphDirection(line);
         // TODO_RUST_CODE
         // Java: if (newDir != lineDir)
            // TODO_RUST_CODE
         {
            // Java: toLeft = !toLeft;
            // TODO_RUST_CODE
            // Java: lineDir = newDir;
            // TODO_RUST_CODE
         }
      }
      // Java: Directions directions = this.getLineDirections(line);
      // TODO_RUST_CODE
      // Java: TextLine tl = TextLine.obtain();
      // TODO_RUST_CODE
      // Java: this.setTextLineInfo(tl, line);
      // TODO_RUST_CODE
      // Java: tl.set(this.mPaint, this.mText, lineStart, lineEnd, lineDir, directions, false, (TabStops)null);
      // TODO_RUST_CODE
      // Java: caret = lineStart + tl.getOffsetToLeftRightOf(caret - lineStart, toLeft);
      // TODO_RUST_CODE
      // Java: TextLine.recycle(tl);
      // TODO_RUST_CODE
      // Java: return caret;
      // TODO_RUST_CODE
    }

    // Java: int getOffsetAtStartOf(int offset)
    pub fn getOffsetAtStartOf(&self, int offset) -> i32 {
      // Java: if (offset == 0)
        // TODO_RUST_CODE
      {
         // Java: return 0;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: CharSequence text = this.mText;
         // TODO_RUST_CODE
         // Java: char c = text.charAt(offset);
         // TODO_RUST_CODE
         // Java: if (c >= '\udc00' && c <= '\udfff')
            // TODO_RUST_CODE
         {
            // Java: char c1 = text.charAt(offset - 1);
            // TODO_RUST_CODE
            // Java: if (c1 >= '\ud800' && c1 <= '\udbff')
                // TODO_RUST_CODE
            {
               // Java: --offset;
               // TODO_RUST_CODE
            }
         }
         // Java: if (this.mSpannedText)
            // TODO_RUST_CODE
         {
            // Java: ReplacementSpan[] spans = (ReplacementSpan[])((Spanned)text).getSpans(offset, offset, ReplacementSpan.class);
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < spans.length; ++i)
                // TODO_RUST_CODE
            {
               // Java: int start = ((Spanned)text).getSpanStart(spans[i]);
               // TODO_RUST_CODE
               // Java: int end = ((Spanned)text).getSpanEnd(spans[i]);
               // TODO_RUST_CODE
               // Java: if (start < offset && end > offset)
                    // TODO_RUST_CODE
               {
                  // Java: offset = start;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: return offset;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean shouldClampCursor(int line)
    pub fn shouldClampCursor(&self, int line) -> bool {
      // Java: switch (this.getParagraphAlignment(line))
        // TODO_RUST_CODE
      {
         // Java: case ALIGN_LEFT:
         // TODO_RUST_CODE
            // Java: return true;
            // TODO_RUST_CODE
         // Java: case ALIGN_NORMAL:
         // TODO_RUST_CODE
            // Java: return this.getParagraphDirection(line) > 0;
            // TODO_RUST_CODE
         // Java: default:
         // TODO_RUST_CODE
            // Java: return false;
            // TODO_RUST_CODE
      }
    }

    // Java: void addSelection(int line, int start, int end, float top, float bottom, SelectionRectangleConsumer consumer)
    pub fn addSelection(&self, int line, int start, int end, float top, float bottom, SelectionRectangleConsumer consumer) {
      // Java: int linestart = this.getLineStart(line);
      // TODO_RUST_CODE
      // Java: int lineend = this.getLineEnd(line);
      // TODO_RUST_CODE
      // Java: Directions dirs = this.getLineDirections(line);
      // TODO_RUST_CODE
      // Java: if (lineend > linestart && this.mText.charAt(lineend - 1) == '\n')
        // TODO_RUST_CODE
      {
         // Java: --lineend;
         // TODO_RUST_CODE
      }
      // Java: for(int i = 0; i < dirs.mDirections.length; i += 2)
        // TODO_RUST_CODE
      {
         // Java: int here = linestart + dirs.mDirections[i];
         // TODO_RUST_CODE
         // Java: int there = here + (dirs.mDirections[i + 1] & 67108863);
         // TODO_RUST_CODE
         // Java: if (there > lineend)
            // TODO_RUST_CODE
         {
            // Java: there = lineend;
            // TODO_RUST_CODE
         }
         // Java: if (start <= there && end >= here)
            // TODO_RUST_CODE
         {
            // Java: int st = Math.max(start, here);
            // TODO_RUST_CODE
            // Java: int en = Math.min(end, there);
            // TODO_RUST_CODE
            // Java: if (st != en)
                // TODO_RUST_CODE
            {
               // Java: float h1 = this.getHorizontal(st, false, line, false);
               // TODO_RUST_CODE
               // Java: float h2 = this.getHorizontal(en, true, line, false);
               // TODO_RUST_CODE
               // Java: float left = Math.min(h1, h2);
               // TODO_RUST_CODE
               // Java: float right = Math.max(h1, h2);
               // TODO_RUST_CODE
               // Java: int layout = (dirs.mDirections[i + 1] & 67108864) != 0 ? 0 : 1;
               // TODO_RUST_CODE
               // Java: consumer.accept(left, top, right, bottom, layout);
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: void getSelection(int start, int end, SelectionRectangleConsumer consumer)
    pub fn getSelection(&self, int start, int end, SelectionRectangleConsumer consumer) {
      // Java: if (start != end)
        // TODO_RUST_CODE
      {
         // Java: if (end < start)
            // TODO_RUST_CODE
         {
            // Java: int temp = end;
            // TODO_RUST_CODE
            // Java: end = start;
            // TODO_RUST_CODE
            // Java: start = temp;
            // TODO_RUST_CODE
         }
         // Java: int startline = this.getLineForOffset(start);
         // TODO_RUST_CODE
         // Java: int endline = this.getLineForOffset(end);
         // TODO_RUST_CODE
         // Java: float top = this.getLineTop(startline);
         // TODO_RUST_CODE
         // Java: float bottom = this.getLineBottomWithoutSpacing(endline);
         // TODO_RUST_CODE
         // Java: if (startline == endline)
            // TODO_RUST_CODE
         {
            // Java: this.addSelection(startline, start, end, top, bottom, consumer);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: float width = (float)this.mWidth;
            // TODO_RUST_CODE
            // Java: this.addSelection(startline, start, this.getLineEnd(startline), top, this.getLineBottom(startline), consumer);
            // TODO_RUST_CODE
            // Java: if (this.getParagraphDirection(startline) == -1)
                // TODO_RUST_CODE
            {
               // Java: consumer.accept(this.getLineLeft(startline), top, 0.0F, this.getLineBottom(startline), 0);
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: consumer.accept(this.getLineRight(startline), top, width, this.getLineBottom(startline), 1);
               // TODO_RUST_CODE
            }
            // Java: for(int i = startline + 1; i < endline; ++i)
                // TODO_RUST_CODE
            {
               // Java: top = this.getLineTop(i);
               // TODO_RUST_CODE
               // Java: bottom = this.getLineBottom(i);
               // TODO_RUST_CODE
               // Java: if (this.getParagraphDirection(i) == -1)
                    // TODO_RUST_CODE
               {
                  // Java: consumer.accept(0.0F, top, width, bottom, 0);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: consumer.accept(0.0F, top, width, bottom, 1);
                  // TODO_RUST_CODE
               }
            }
            // Java: top = this.getLineTop(endline);
            // TODO_RUST_CODE
            // Java: bottom = this.getLineBottomWithoutSpacing(endline);
            // TODO_RUST_CODE
            // Java: this.addSelection(endline, this.getLineStart(endline), end, top, bottom, consumer);
            // TODO_RUST_CODE
            // Java: if (this.getParagraphDirection(endline) == -1)
                // TODO_RUST_CODE
            {
               // Java: consumer.accept(width, top, this.getLineRight(endline), bottom, 0);
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: consumer.accept(0.0F, top, this.getLineLeft(endline), bottom, 1);
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: Alignment getParagraphAlignment(int line)
    pub fn getParagraphAlignment(&self, int line) -> Alignment {
      // Java: return this.mAlignment;
      // TODO_RUST_CODE
    }

    // Java: int getParagraphLeft(int line)
    pub fn getParagraphLeft(&self, int line) -> i32 {
      // Java: int left = 0;
      // TODO_RUST_CODE
      // Java: int dir = this.getParagraphDirection(line);
      // TODO_RUST_CODE
      // Java: return dir != -1 && this.mSpannedText ? this.getParagraphLeadingMargin(line) : left;
      // TODO_RUST_CODE
    }

    // Java: int getParagraphRight(int line)
    pub fn getParagraphRight(&self, int line) -> i32 {
      // Java: int right = this.mWidth;
      // TODO_RUST_CODE
      // Java: int dir = this.getParagraphDirection(line);
      // TODO_RUST_CODE
      // Java: return dir != 1 && this.mSpannedText ? right - this.getParagraphLeadingMargin(line) : right;
      // TODO_RUST_CODE
    }

    // Java: int getParagraphLeadingMargin(int line)
    pub fn getParagraphLeadingMargin(&self, int line) -> i32 {
      // Java: if (!this.mSpannedText)
        // TODO_RUST_CODE
      {
         // Java: return 0;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: Spanned spanned = (Spanned)this.mText;
         // TODO_RUST_CODE
         // Java: int lineStart = this.getLineStart(line);
         // TODO_RUST_CODE
         // Java: int lineEnd = this.getLineEnd(line);
         // TODO_RUST_CODE
         // Java: int spanEnd = spanned.nextSpanTransition(lineStart, lineEnd, LeadingMarginSpan.class);
         // TODO_RUST_CODE
         // Java: LeadingMarginSpan[] spans = (LeadingMarginSpan[])getParagraphSpans(spanned, lineStart, spanEnd, LeadingMarginSpan.class);
         // TODO_RUST_CODE
         // Java: if (spans.length == 0)
            // TODO_RUST_CODE
         {
            // Java: return 0;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: int margin = 0;
            // TODO_RUST_CODE
            // Java: boolean useFirstLineMargin = lineStart == 0 || spanned.charAt(lineStart - 1) == '\n';
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < spans.length; ++i)
                // TODO_RUST_CODE
            {
               // Java: LeadingMarginSpan span = spans[i];
               // TODO_RUST_CODE
               // Java: margin += span.getLeadingMargin(useFirstLineMargin);
               // TODO_RUST_CODE
            }
            // Java: return margin;
            // TODO_RUST_CODE
         }
      }
    }

    // Java: static float measurePara(Paint paint, CharSequence text, int start, int end, int textDir)
    pub fn measurePara(&self, Paint paint, CharSequence text, int start, int end, int textDir) -> f32 {
      // Java: TextLine tl = TextLine.obtain();
      // TODO_RUST_CODE
      // Java: float var21;
      // TODO_RUST_CODE
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: char[] chars = ArrayUtils.newUnpaddedCharArray(text.length());
         // TODO_RUST_CODE
         // Java: TextUtils.getChars(text, start, end, chars, 0);
         // TODO_RUST_CODE
         // Java: int len = chars.length;
         // TODO_RUST_CODE
         // Java: boolean hasTabs = false;
         // TODO_RUST_CODE
         // Java: TabStops tabStops = null;
         // TODO_RUST_CODE
         // Java: int margin = 0;
         // TODO_RUST_CODE
         // Java: if (text instanceof Spanned)
            // TODO_RUST_CODE
         {
            // Java: Spanned spanned = (Spanned)text;
            // TODO_RUST_CODE
            // Java: LeadingMarginSpan[] spans = (LeadingMarginSpan[])getParagraphSpans(spanned, start, end, LeadingMarginSpan.class);
            // TODO_RUST_CODE
            // Java: for(LeadingMarginSpan lms : spans)
                // TODO_RUST_CODE
            {
               // Java: margin += lms.getLeadingMargin(true);
               // TODO_RUST_CODE
            }
         }
         // Java: for(int i = 0; i < len; ++i)
            // TODO_RUST_CODE
         {
            // Java: if (chars[i] == '\t')
                // TODO_RUST_CODE
            {
               // Java: hasTabs = true;
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            }
         }
         // Java: tl.set(paint, text, start, end, 1, DIRS_ALL_LEFT_TO_RIGHT, hasTabs, tabStops);
         // TODO_RUST_CODE
         // Java: var21 = (float)margin + Math.abs(tl.metrics((Paint.FontMetrics)null));
         // TODO_RUST_CODE
      }
      // Java: finally
        // TODO_RUST_CODE
      {
         // Java: TextLine.recycle(tl);
         // TODO_RUST_CODE
      }
      // Java: return var21;
      // TODO_RUST_CODE
    }

    // Java: static <T> T[] getParagraphSpans(Spanned text, int start, int end, Class<T> type)
    pub fn getParagraphSpans(&self, Spanned text, int start, int end, Class<T> type) -> <T> T[] {
      // Java: if (start == end && start > 0)
        // TODO_RUST_CODE
      {
         // Java: return (T[])ArrayUtils.emptyArray(type);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return (T[])(text instanceof SpannableStringBuilder ? ((SpannableStringBuilder)text).getSpans(start, end, type) : text.getSpans(start, end, type));
         // TODO_RUST_CODE
      }
    }

    // Java: void getCursorPath(int point, Path dest, CharSequence editingBuffer)
    pub fn getCursorPath(&self, int point, Path dest, CharSequence editingBuffer) {
      // Java: dest.reset();
      // TODO_RUST_CODE
      // Java: int line = this.getLineForOffset(point);
      // TODO_RUST_CODE
      // Java: float top = this.getLineTop(line);
      // TODO_RUST_CODE
      // Java: float bottom = this.getLineBottomWithoutSpacing(line);
      // TODO_RUST_CODE
      // Java: boolean clamped = this.shouldClampCursor(line);
      // TODO_RUST_CODE
      // Java: float h1 = this.getPrimaryHorizontal(point, clamped) - 0.5F;
      // TODO_RUST_CODE
      // Java: float h2 = this.isLevelBoundary(point) ? this.getSecondaryHorizontal(point, clamped) - 0.5F : h1;
      // TODO_RUST_CODE
      // Java: int caps = 0;
      // TODO_RUST_CODE
      // Java: int fn = 0;
      // TODO_RUST_CODE
      // Java: float dist = 0.0F;
      // TODO_RUST_CODE
      // Java: if (caps != 0 || fn != 0)
        // TODO_RUST_CODE
      {
         // Java: dist = (bottom - top) / 4.0F;
         // TODO_RUST_CODE
         // Java: if (fn != 0)
            // TODO_RUST_CODE
         {
            // Java: top += dist;
            // TODO_RUST_CODE
         }
         // Java: if (caps != 0)
            // TODO_RUST_CODE
         {
            // Java: bottom -= dist;
            // TODO_RUST_CODE
         }
      }
      // Java: if (h1 < 0.5F)
        // TODO_RUST_CODE
      {
         // Java: h1 = 0.5F;
         // TODO_RUST_CODE
      }
      // Java: if (h2 < 0.5F)
        // TODO_RUST_CODE
      {
         // Java: h2 = 0.5F;
         // TODO_RUST_CODE
      }
      // Java: if (Float.compare(h1, h2) == 0)
        // TODO_RUST_CODE
      {
         // Java: dest.moveTo(h1, top);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h1, bottom);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: dest.moveTo(h1, top);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h1, (top + bottom) / 2.0F);
         // TODO_RUST_CODE
         // Java: dest.moveTo(h2, (top + bottom) / 2.0F);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h2, bottom);
         // TODO_RUST_CODE
      }
      // Java: if (caps == 2)
        // TODO_RUST_CODE
      {
         // Java: dest.moveTo(h2, bottom);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h2 - dist, bottom + dist);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h2, bottom);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h2 + dist, bottom + dist);
         // TODO_RUST_CODE
      }
      // Java: else if (caps == 1)
        // TODO_RUST_CODE
      {
         // Java: dest.moveTo(h2, bottom);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h2 - dist, bottom + dist);
         // TODO_RUST_CODE
         // Java: dest.moveTo(h2 - dist, bottom + dist - 0.5F);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h2 + dist, bottom + dist - 0.5F);
         // TODO_RUST_CODE
         // Java: dest.moveTo(h2 + dist, bottom + dist);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h2, bottom);
         // TODO_RUST_CODE
      }
      // Java: if (fn == 2)
        // TODO_RUST_CODE
      {
         // Java: dest.moveTo(h1, top);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h1 - dist, top - dist);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h1, top);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h1 + dist, top - dist);
         // TODO_RUST_CODE
      }
      // Java: else if (fn == 1)
        // TODO_RUST_CODE
      {
         // Java: dest.moveTo(h1, top);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h1 - dist, top - dist);
         // TODO_RUST_CODE
         // Java: dest.moveTo(h1 - dist, top - dist + 0.5F);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h1 + dist, top - dist + 0.5F);
         // TODO_RUST_CODE
         // Java: dest.moveTo(h1 + dist, top - dist);
         // TODO_RUST_CODE
         // Java: dest.lineTo(h1, top);
         // TODO_RUST_CODE
      }
    }

    // Java: float[] getCharWidths()
    pub fn getCharWidths(&self) -> float[] {
      // Java: return null;
      // TODO_RUST_CODE
    }

    // Java: float[] getOriginCharWidths()
    pub fn getOriginCharWidths(&self) -> float[] {
      // Java: return null;
      // TODO_RUST_CODE
    }

    // Java: float[] getSpacBefCharWidths()
    pub fn getSpacBefCharWidths(&self) -> float[] {
      // Java: return null;
      // TODO_RUST_CODE
    }

    // Java: List<Integer> getMarginLefts()
    pub fn getMarginLefts(&self) -> List<Integer> {
      // Java: return null;
      // TODO_RUST_CODE
    }

    // Java: int getLeadMargin(int line)
    pub fn getLeadMargin(&self, int line) -> i32 {
      // Java: return 0;
      // TODO_RUST_CODE
    }

    // Java: void setTextLineInfo(TextLine tl, int lineNum)
    pub fn setTextLineInfo(&self, TextLine tl, int lineNum) {
      // Java: if (this instanceof WpsLayout)
        // TODO_RUST_CODE
      {
         // Java: tl.setCharWidths(this.getCharWidths());
         // TODO_RUST_CODE
         // Java: tl.setOriginCharWidths(this.getOriginCharWidths());
         // TODO_RUST_CODE
         // Java: tl.setSpacBefCharWidths(this.getSpacBefCharWidths());
         // TODO_RUST_CODE
         // Java: tl.setLineCount(lineNum);
         // TODO_RUST_CODE
         // Java: List<Integer> marginLefts = this.getMarginLefts();
         // TODO_RUST_CODE
         // Java: int marginL = marginLefts != null && marginLefts.size() > lineNum ? (Integer)marginLefts.get(lineNum) : 0;
         // TODO_RUST_CODE
         // Java: tl.setMarginL(marginL);
         // TODO_RUST_CODE
         // Java: tl.setLayout(this);
         // TODO_RUST_CODE
         // Java: tl.setVertical(this.mVertical);
         // TODO_RUST_CODE
      }
    }

    // Java: boolean isAllVerticalText()
    pub fn isAllVerticalText(&self) -> bool {
      // Java: switch (this.mVertical)
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

    // Java: boolean isAllVerticalLTRText()
    pub fn isAllVerticalLTRText(&self) -> bool {
      // Java: return this.mVertical == 3;
      // TODO_RUST_CODE
    }

    // Java: boolean isAllVerticalRTLText()
    pub fn isAllVerticalRTLText(&self) -> bool {
      // Java: return this.mVertical == 6;
      // TODO_RUST_CODE
    }

    // Java: List<Float> getLineHeights()
    pub fn getLineHeights(&self) -> List<Float> {
      // Java: return this.mLineHeights;
      // TODO_RUST_CODE
    }

    // Java: List<Float> getLineWidths()
    pub fn getLineWidths(&self) -> List<Float> {
      // Java: return this.mLineWidths;
      // TODO_RUST_CODE
    }

    // Java: boolean isAngle()
    pub fn isAngle(&self) -> bool {
      // Java: return this.mAngle != 0;
      // TODO_RUST_CODE
    }

}

// Java: public class HorizontalMeasurementProvider
impl HorizontalMeasurementProvider {
    // Java: void init()
    pub fn init(&self) {
         // Java: Directions dirs = Layout.this.getLineDirections(this.mLine);
         // TODO_RUST_CODE
         // Java: if (dirs != Layout.DIRS_ALL_LEFT_TO_RIGHT)
        // TODO_RUST_CODE
         {
            // Java: this.mHorizontals = Layout.this.getLineHorizontals(this.mLine, false, this.mPrimary);
            // TODO_RUST_CODE
            // Java: this.mLineStartOffset = Layout.this.getLineStart(this.mLine);
            // TODO_RUST_CODE
         }
    }

    // Java: float get(int offset)
    pub fn get(&self, int offset) -> f32 {
         // Java: return this.mHorizontals == null ? Layout.this.getHorizontal(offset, this.mPrimary) : this.mHorizontals[offset - this.mLineStartOffset];
         // TODO_RUST_CODE
    }

}

// Java: public class TabStops
impl TabStops {
    // Java: void reset(int increment, Object[] spans)
    pub fn reset(&self, int increment, Object[] spans) {
         // Java: this.mIncrement = increment;
         // TODO_RUST_CODE
         // Java: int ns = 0;
         // TODO_RUST_CODE
         // Java: if (spans != null)
        // TODO_RUST_CODE
         {
            // Java: int[] stops = this.mStops;
            // TODO_RUST_CODE
            // Java: if (ns > 1)
            // TODO_RUST_CODE
            {
               // Java: Arrays.sort(stops, 0, ns);
               // TODO_RUST_CODE
            }
            // Java: if (stops != this.mStops)
            // TODO_RUST_CODE
            {
               // Java: this.mStops = stops;
               // TODO_RUST_CODE
            }
         }
         // Java: this.mNumStops = ns;
         // TODO_RUST_CODE
    }

    // Java: float nextTab(float h)
    pub fn nextTab(&self, float h) -> f32 {
         // Java: int ns = this.mNumStops;
         // TODO_RUST_CODE
         // Java: if (ns > 0)
        // TODO_RUST_CODE
         {
            // Java: int[] stops = this.mStops;
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < ns; ++i)
            // TODO_RUST_CODE
            {
               // Java: int stop = stops[i];
               // TODO_RUST_CODE
               // Java: if ((float)stop > h)
                // TODO_RUST_CODE
               {
                  // Java: return (float)stop;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: return nextDefaultStop(h, this.mIncrement);
         // TODO_RUST_CODE
    }

    // Java: static float nextDefaultStop(float h, int inc)
    pub fn nextDefaultStop(&self, float h, int inc) -> f32 {
         // Java: return (float)((int)((h + (float)inc) / (float)inc) * inc);
         // TODO_RUST_CODE
    }

}

// Java: public class Directions
impl Directions {
    // Java: Directions(int[] dirs)
    pub fn Directions(&self, int[] dirs) {
         // Java: super();
         // TODO_RUST_CODE
         // Java: this.mDirections = dirs;
         // TODO_RUST_CODE
    }

}

// Java: public enum Alignment
impl Alignment {
    // Java: Alignment()
    pub fn Alignment(&self) {
    // TODO_RUST_CODE
    }

}
