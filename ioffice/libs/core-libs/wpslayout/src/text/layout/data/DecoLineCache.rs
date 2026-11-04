// Java: import android.graphics.Canvas
use thirdparty_kInk::com::hp::hpl::inkml::Canvas;
// Java: import android.graphics.Paint
use crate::android::graphics::Paint;
// Java: import android.graphics.Path
use crate::android::graphics::Path;
// Java: import android.graphics.PointF
use util::cn::wps::graphics::PointF;
// Java: import android.graphics.RectF
use util::cn::wps::graphics::RectF;
// Java: import android.graphics.Paint.Join
use crate::android::graphics::Paint::Join;
// Java: import android.graphics.Paint.Style
use crate::android::graphics::Paint::Style;
// Java: import cn.wps.text.layout.typo.layout.Layout
use crate::text::layout::typo::layout::Layout;
// Java: import java.text.Bidi
// TODO_RUST_CODE use ???; // java.text.Bidi
// Java: import java.util.ArrayList
// TODO_RUST_CODE use ???; // java.util.ArrayList
// Java: import java.util.List
// TODO_RUST_CODE use ???; // java.util.List

// Java field: short CELL_MARGIN
pub(crate) const CELL_MARGIN: i16 = /* TODO */;
// Java field: float DECOLINE_SCALE
pub(crate) const DECOLINE_SCALE: f32 = /* TODO */;
// Java field: float DOUBLE_LINE_WIDTH_FACTOR
pub(crate) const DOUBLE_LINE_WIDTH_FACTOR: f32 = /* TODO */;
// Java field: float WAVE_LINE_WIDTH_FACTOR
pub(crate) const WAVE_LINE_WIDTH_FACTOR: f32 = /* TODO */;
// Java field: float DOUBLE_LINE_VD_FACTOR
pub(crate) const DOUBLE_LINE_VD_FACTOR: f32 = /* TODO */;
// Java field: float WAVE_LINE_VD_FACTOR
pub(crate) const WAVE_LINE_VD_FACTOR: f32 = /* TODO */;
// Java field: float DOUBLE_WAVE_LINE_VD_FACTOR
pub(crate) const DOUBLE_WAVE_LINE_VD_FACTOR: f32 = /* TODO */;
// Java field: float UNDERLINE_MOVEDOWN
pub(crate) const UNDERLINE_MOVEDOWN: f32 = /* TODO */;
// Java field: short DECOLINE_CLASS_UNDERLINE
pub(crate) const DECOLINE_CLASS_UNDERLINE: i16 = /* TODO */;
// Java field: short DECOLINE_CLASS_STRIKETHROUGH
pub(crate) const DECOLINE_CLASS_STRIKETHROUGH: i16 = /* TODO */;
// Java field: float HEAVY_FACTOR
pub(crate) const HEAVY_FACTOR: f32 = /* TODO */;
// Java field: float DOUBLE_WAVE_DISTANCE_FACTOR
pub(crate) const DOUBLE_WAVE_DISTANCE_FACTOR: f32 = /* TODO */;

// Java: public class DecoLineCache
pub struct DecoLineCache {
    // Java field: ArrayList<DecoLine> mDecoLines
    pub(crate) mDecoLines: Arc<Mutex<Vec<DecoLine>>>,
    // Java field: Paint mPaint
    mPaint: Option<Paint>,
}

// Java: public class LinePos
pub struct LinePos {
    // Java field: int start
    pub(crate) start: i32,
    // Java field: int end
    pub(crate) end: i32,
    // Java field: int line
    pub(crate) line: i32,
    // Java field: int lineEnd
    pub(crate) lineEnd: i32,
    // Java field: int color
    pub(crate) color: i32,
    // Java field: float fontSize
    pub(crate) fontSize: f32,
    // Java field: float ascent
    pub(crate) ascent: f32,
    // Java field: int runLevel
    pub(crate) runLevel: i32,
    // Java field: byte type
    pub(crate) type: i8,
    // Java field: boolean accounting
    pub(crate) accounting: bool,
    // Java field: boolean primary
    pub(crate) primary: bool,
    // Java field: boolean isHeavy
    pub(crate) isHeavy: bool,
    // Java field: boolean isSubscript
    pub(crate) isSubscript: bool,
    // Java field: boolean isSuperscript
    pub(crate) isSuperscript: bool,
}

// Java: public class DecoLine
pub struct DecoLine {
    // Java field: float startX
    pub(crate) startX: f32,
    // Java field: float endX
    pub(crate) endX: f32,
    // Java field: float y
    pub(crate) y: f32,
    // Java field: float yCorrection
    pub(crate) yCorrection: f32,
    // Java field: int color
    pub(crate) color: i32,
    // Java field: float width
    pub(crate) width: f32,
    // Java field: byte type
    pub(crate) type: i8,
    // Java field: boolean heavy
    pub(crate) heavy: bool,
}

// Java: public class DecoLineCache
impl DecoLineCache {
    // Java: DecoLineCache()
    pub fn DecoLineCache(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: this.mDecoLines.clear();
      // TODO_RUST_CODE
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.mDecoLines.isEmpty();
      // TODO_RUST_CODE
    }

    // Java: void draw(Canvas canvas, List<RectF> txtRects, boolean isAllVert)
    pub fn draw(&self, Canvas canvas, List<RectF> txtRects, boolean isAllVert) {
      // Java: if (!this.mDecoLines.isEmpty())
        // TODO_RUST_CODE
      {
         // Java: if (this.mPaint == null)
            // TODO_RUST_CODE
         {
            // Java: this.mPaint = new Paint();
            // TODO_RUST_CODE
         }
         // Java: this.mPaint.reset();
         // TODO_RUST_CODE
         // Java: this.mPaint.setStyle(Style.STROKE);
         // TODO_RUST_CODE
         // Java: this.mPaint.setAntiAlias(true);
         // TODO_RUST_CODE
         // Java: this.mPaint.setStrokeJoin(Join.ROUND);
         // TODO_RUST_CODE
         // Java: for(DecoLine decoLine : this.mDecoLines)
            // TODO_RUST_CODE
         {
            // Java: this.mPaint.setColor(decoLine.color);
            // TODO_RUST_CODE
            // Java: this.mPaint.setStrokeWidth(decoLine.width);
            // TODO_RUST_CODE
            // Java: decoLine.draw(canvas, this.mPaint, isAllVert);
            // TODO_RUST_CODE
         }
      }
    }

    // Java: void calcDecoLine(Layout typoCache, TextData data)
    pub fn calcDecoLine(&self, Layout typoCache, TextData data) {
      // Java: this.mDecoLines.clear();
      // TODO_RUST_CODE
      // Java: if (!this.isBlank(data))
        // TODO_RUST_CODE
      {
         // Java: this.calcDecoLine(typoCache, data, (short)0);
         // TODO_RUST_CODE
         // Java: this.calcDecoLine(typoCache, data, (short)1);
         // TODO_RUST_CODE
      }
    }

    // Java: void calcDecoLine(Layout typoCache, TextData data, short lineClass)
    pub fn calcDecoLine(&self, Layout typoCache, TextData data, short lineClass) {
      // Java: boolean isAllVertLtr = data.mVertical == 3;
      // TODO_RUST_CODE
      // Java: boolean isAllVertRtl = data.mVertical == 6;
      // TODO_RUST_CODE
      // Java: int lineCount = typoCache.getLineCount();
      // TODO_RUST_CODE
      // Java: int[] lineEnds = new int[lineCount];
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < lineCount; ++i)
        // TODO_RUST_CODE
      {
         // Java: lineEnds[i] = typoCache.getLineEnd(i);
         // TODO_RUST_CODE
      }
      // Java: ArrayList<LinePos> linesPos = new ArrayList();
      // TODO_RUST_CODE
      // Java: SpanInfo span = data.mFirstSpan;
      // TODO_RUST_CODE
      // Java: int spanIdx = 0;
      // TODO_RUST_CODE
      // Java: int lineIdx = 0;
      // TODO_RUST_CODE
      // Java: while(true)
        // TODO_RUST_CODE
      {
         // Java: boolean hasDecoLine = false;
         // TODO_RUST_CODE
         // Java: int color = span.mFontColor;
         // TODO_RUST_CODE
         // Java: byte lineType = 0;
         // TODO_RUST_CODE
         // Java: boolean isHeavy = false;
         // TODO_RUST_CODE
         // Java: switch (lineClass)
            // TODO_RUST_CODE
         {
            // Java: case 0:
            // TODO_RUST_CODE
               // Java: hasDecoLine = span.hasUnderline();
               // TODO_RUST_CODE
               // Java: color = span.mUnderlineColor;
               // TODO_RUST_CODE
               // Java: lineType = span.mUnderline;
               // TODO_RUST_CODE
               // Java: isHeavy = span.isUnderlineHeavy();
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 1:
            // TODO_RUST_CODE
               // Java: hasDecoLine = span.hasStrikethroughLine();
               // TODO_RUST_CODE
               // Java: lineType = span.mStrikethrough;
               // TODO_RUST_CODE
         }
         // Java: if (hasDecoLine)
            // TODO_RUST_CODE
         {
            // Java: boolean isAccouting = span.isAcounting();
            // TODO_RUST_CODE
            // Java: LinePos pos = new LinePos();
            // TODO_RUST_CODE
            // Java: pos.start = span.mStartPos;
            // TODO_RUST_CODE
            // Java: pos.end = span.mEndPos;
            // TODO_RUST_CODE
            // Java: if (data.mPara.indexMap != null)
                // TODO_RUST_CODE
            {
               // Java: int start = pos.start;
               // TODO_RUST_CODE
               // Java: int end = pos.end;
               // TODO_RUST_CODE
               // Java: pos.start = data.mPara.indexMap[start];
               // TODO_RUST_CODE
               // Java: pos.end = data.mPara.indexMap[start];
               // TODO_RUST_CODE
               // Java: for(int i = start + 1; i < end; ++i)
                    // TODO_RUST_CODE
               {
                  // Java: int index = data.mPara.indexMap[i];
                  // TODO_RUST_CODE
                  // Java: if (pos.start > index)
                        // TODO_RUST_CODE
                  {
                     // Java: pos.start = index;
                     // TODO_RUST_CODE
                  }
                  // Java: if (pos.end < index)
                        // TODO_RUST_CODE
                  {
                     // Java: pos.end = index;
                     // TODO_RUST_CODE
                  }
               }
               // Java: if (pos.start > pos.end)
                    // TODO_RUST_CODE
               {
                  // Java: int temp = pos.start;
                  // TODO_RUST_CODE
                  // Java: pos.start = pos.end;
                  // TODO_RUST_CODE
                  // Java: pos.end = temp;
                  // TODO_RUST_CODE
               }
               // Java: ++pos.end;
               // TODO_RUST_CODE
            }
            // Java: pos.color = color;
            // TODO_RUST_CODE
            // Java: pos.fontSize = span.mFontSize;
            // TODO_RUST_CODE
            // Java: pos.accounting = isAccouting;
            // TODO_RUST_CODE
            // Java: pos.ascent = span.getAscent();
            // TODO_RUST_CODE
            // Java: pos.type = lineType;
            // TODO_RUST_CODE
            // Java: pos.isHeavy = isHeavy;
            // TODO_RUST_CODE
            // Java: pos.isSubscript = span.mIsSubScript;
            // TODO_RUST_CODE
            // Java: for(pos.isSuperscript = span.mIsSuperScript; lineIdx < lineCount; ++lineIdx)
                // TODO_RUST_CODE
            {
               // Java: int lineEnd = lineEnds[lineIdx];
               // TODO_RUST_CODE
               // Java: pos.line = lineIdx;
               // TODO_RUST_CODE
               // Java: pos.lineEnd = lineEnd;
               // TODO_RUST_CODE
               // Java: if (pos.start < lineEnd && pos.end <= lineEnd)
                    // TODO_RUST_CODE
               {
                  // Java: linesPos.add(pos);
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               }
               // Java: if (pos.start < lineEnd && pos.end > lineEnd)
                    // TODO_RUST_CODE
               {
                  // Java: LinePos newPos = new LinePos();
                  // TODO_RUST_CODE
                  // Java: newPos.start = lineEnd;
                  // TODO_RUST_CODE
                  // Java: newPos.end = pos.end;
                  // TODO_RUST_CODE
                  // Java: newPos.color = pos.color;
                  // TODO_RUST_CODE
                  // Java: newPos.fontSize = pos.fontSize;
                  // TODO_RUST_CODE
                  // Java: newPos.type = pos.type;
                  // TODO_RUST_CODE
                  // Java: newPos.accounting = pos.accounting;
                  // TODO_RUST_CODE
                  // Java: newPos.ascent = pos.ascent;
                  // TODO_RUST_CODE
                  // Java: newPos.isHeavy = pos.isHeavy;
                  // TODO_RUST_CODE
                  // Java: pos.end = lineEnd;
                  // TODO_RUST_CODE
                  // Java: linesPos.add(pos);
                  // TODO_RUST_CODE
                  // Java: pos = newPos;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: if (spanIdx >= data.mOtherSpans.size())
            // TODO_RUST_CODE
         {
            // Java: float[] moveDownRef = new float[typoCache.getLineCount()];
            // TODO_RUST_CODE
            // Java: if (0 == lineClass)
                // TODO_RUST_CODE
            {
               // Java: for(int i = 0; i < linesPos.size(); ++i)
                    // TODO_RUST_CODE
               {
                  // Java: LinePos p = (LinePos)linesPos.get(i);
                  // TODO_RUST_CODE
                  // Java: if (moveDownRef[p.line] < p.fontSize)
                        // TODO_RUST_CODE
                  {
                     // Java: moveDownRef[p.line] = p.fontSize;
                     // TODO_RUST_CODE
                  }
               }
            }
            // Java: CharSequence text = typoCache.getText();
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < linesPos.size(); ++i)
                // TODO_RUST_CODE
            {
               // Java: LinePos p = (LinePos)linesPos.get(i);
               // TODO_RUST_CODE
               // Java: if (!p.isEmpty(text))
                    // TODO_RUST_CODE
               {
                  // Java: p.clipEndSpace(text, typoCache.getLineStart(p.line), p.lineEnd);
                  // TODO_RUST_CODE
                  // Java: DecoLine decoLine = new DecoLine();
                  // TODO_RUST_CODE
                  // Java: decoLine.color = p.color;
                  // TODO_RUST_CODE
                  // Java: decoLine.width = p.fontSize * 0.05F;
                  // TODO_RUST_CODE
                  // Java: decoLine.type = p.type;
                  // TODO_RUST_CODE
                  // Java: decoLine.heavy = p.isHeavy;
                  // TODO_RUST_CODE
                  // Java: if (p.isHeavy)
                        // TODO_RUST_CODE
                  {
                     // Java: decoLine.width *= 1.5F;
                     // TODO_RUST_CODE
                  }
                  // Java: switch (p.type)
                        // TODO_RUST_CODE
                  {
                     // Java: case 2:
                     // TODO_RUST_CODE
                     // Java: case 102:
                     // TODO_RUST_CODE
                        // Java: decoLine.width *= 0.5F;
                        // TODO_RUST_CODE
                        // Java: break;
                        // TODO_RUST_CODE
                     // Java: case 9:
                     // TODO_RUST_CODE
                     // Java: case 10:
                     // TODO_RUST_CODE
                     // Java: case 17:
                     // TODO_RUST_CODE
                        // Java: decoLine.width *= 0.125F;
                        // TODO_RUST_CODE
                  }
                  // Java: switch (lineClass)
                        // TODO_RUST_CODE
                  {
                     // Java: case 0:
                     // TODO_RUST_CODE
                        // Java: decoLine.yCorrection = moveDownRef[p.line] * 0.1F;
                        // TODO_RUST_CODE
                        // Java: if (isAllVertLtr)
                            // TODO_RUST_CODE
                        {
                           // Java: decoLine.y = typoCache.getLineTop(p.line);
                           // TODO_RUST_CODE
                        }
                        // Java: else if (isAllVertRtl)
                            // TODO_RUST_CODE
                        {
                           // Java: decoLine.y = typoCache.getLineBottom(p.line);
                           // TODO_RUST_CODE
                        }
                        // Java: else
                            // TODO_RUST_CODE
                        {
                           // Java: decoLine.y = typoCache.getLineBaseline(p.line) + decoLine.yCorrection;
                           // TODO_RUST_CODE
                        }
                        // Java: if (isAllVertRtl)
                            // TODO_RUST_CODE
                        {
                           // Java: decoLine.y = -decoLine.y;
                           // TODO_RUST_CODE
                        }
                        // Java: break;
                        // TODO_RUST_CODE
                     // Java: case 1:
                     // TODO_RUST_CODE
                        // Java: decoLine.yCorrection = p.ascent / 3.0F + decoLine.width;
                        // TODO_RUST_CODE
                        // Java: if (!isAllVertLtr && !isAllVertRtl)
                            // TODO_RUST_CODE
                        {
                           // Java: decoLine.y = typoCache.getLineBaseline(p.line) + decoLine.yCorrection;
                           // TODO_RUST_CODE
                        }
                        // Java: else
                            // TODO_RUST_CODE
                        {
                           // Java: decoLine.y = (typoCache.getLineTop(p.line) + typoCache.getLineBottom(p.line)) / 2.0F;
                           // TODO_RUST_CODE
                        }
                        // Java: if (isAllVertRtl)
                            // TODO_RUST_CODE
                        {
                           // Java: decoLine.y = -decoLine.y;
                           // TODO_RUST_CODE
                        }
                  }
                  // Java: boolean isUp = false;
                  // TODO_RUST_CODE
                  // Java: boolean isDown = false;
                  // TODO_RUST_CODE
                  // Java: if (!isAllVertLtr && !isAllVertRtl)
                        // TODO_RUST_CODE
                  {
                     // Java: isUp = p.isSuperscript;
                     // TODO_RUST_CODE
                     // Java: isDown = p.isSubscript;
                     // TODO_RUST_CODE
                  }
                  // Java: else
                        // TODO_RUST_CODE
                  {
                     // Java: isUp = p.isSubscript;
                     // TODO_RUST_CODE
                     // Java: isDown = p.isSuperscript;
                     // TODO_RUST_CODE
                  }
                  // Java: if (isDown)
                        // TODO_RUST_CODE
                  {
                     // Java: decoLine.y = (float)((double)decoLine.y + (double)(typoCache.getLineBaseline(p.line) - typoCache.getLineTop(p.line)) * 0.3);
                     // TODO_RUST_CODE
                  }
                  // Java: else if (isUp)
                        // TODO_RUST_CODE
                  {
                     // Java: decoLine.y = (float)((double)decoLine.y - (double)(typoCache.getLineBaseline(p.line) - typoCache.getLineTop(p.line)) * 0.4);
                     // TODO_RUST_CODE
                  }
                  // Java: if (p.accounting)
                        // TODO_RUST_CODE
                  {
                     // Java: decoLine.startX = 2.0F;
                     // TODO_RUST_CODE
                     // Java: decoLine.endX = (float)(typoCache.getWidth() - 2);
                     // TODO_RUST_CODE
                  }
                  // Java: else
                        // TODO_RUST_CODE
                  {
                     // Java: if (p.primary)
                            // TODO_RUST_CODE
                     {
                        // Java: decoLine.startX = typoCache.getPrimaryHorizontal(p.start);
                        // TODO_RUST_CODE
                     }
                     // Java: else
                            // TODO_RUST_CODE
                     {
                        // Java: decoLine.startX = typoCache.getSecondaryHorizontal(p.start);
                        // TODO_RUST_CODE
                     }
                     // Java: decoLine.endX = typoCache.getSecondaryHorizontal(p.end > 0 ? p.end - 1 : p.end);
                     // TODO_RUST_CODE
                  }
                  // Java: if (decoLine.startX > decoLine.endX)
                        // TODO_RUST_CODE
                  {
                     // Java: float temp = decoLine.startX;
                     // TODO_RUST_CODE
                     // Java: decoLine.startX = decoLine.endX;
                     // TODO_RUST_CODE
                     // Java: decoLine.endX = temp;
                     // TODO_RUST_CODE
                  }
                  // Java: this.mDecoLines.add(decoLine);
                  // TODO_RUST_CODE
               }
            }
            // Java: if (0 == lineClass)
                // TODO_RUST_CODE
            {
               // Java: this.mergeLines(this.mDecoLines);
               // TODO_RUST_CODE
            }
            // Java: return;
            // TODO_RUST_CODE
         }
         // Java: span = (SpanInfo)data.mOtherSpans.get(spanIdx);
         // TODO_RUST_CODE
         // Java: ++spanIdx;
         // TODO_RUST_CODE
      }
    }

    // Java: void mergeLines(ArrayList<DecoLine> lines)
    pub fn mergeLines(&self, ArrayList<DecoLine> lines) {
      // Java: int i = 0;
      // TODO_RUST_CODE
      // Java: if (null != lines)
        // TODO_RUST_CODE
      {
         // Java: while(i < lines.size() - 1)
            // TODO_RUST_CODE
         {
            // Java: DecoLine line1 = (DecoLine)lines.get(i);
            // TODO_RUST_CODE
            // Java: DecoLine line2 = (DecoLine)lines.get(i + 1);
            // TODO_RUST_CODE
            // Java: if (this.canMergeLines(line1, line2))
                // TODO_RUST_CODE
            {
               // Java: line2.startX = line1.startX < line2.startX ? line1.startX : line2.startX;
               // TODO_RUST_CODE
               // Java: line2.endX = line1.endX > line2.endX ? line1.endX : line2.endX;
               // TODO_RUST_CODE
               // Java: line2.width = line1.width > line2.width ? line1.width : line2.width;
               // TODO_RUST_CODE
               // Java: lines.remove(i);
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: ++i;
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: boolean canMergeLines(DecoLine line1, DecoLine line2)
    pub fn canMergeLines(&self, DecoLine line1, DecoLine line2) -> bool {
      // Java: float ALLOW_ERROR = 1.0E-10F;
      // TODO_RUST_CODE
      // Java: return line1.color == line2.color && line1.type == line2.type && (line1.startX < line2.startX && line1.endX >= line2.startX || line2.startX < line1.startX && line2.endX >= line1.startX) && Math.abs(line1.y - line2.y) < 1.0E-10F;
      // TODO_RUST_CODE
    }

    // Java: void calcDirection(TextData data, ArrayList<LinePos> poses)
    pub fn calcDirection(&self, TextData data, ArrayList<LinePos> poses) {
      // Java: Bidi bd = data.mBidi;
      // TODO_RUST_CODE
      // Java: if (null != bd)
        // TODO_RUST_CODE
      {
         // Java: int baseLevel = bd.getBaseLevel();
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < poses.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: LinePos pos = (LinePos)poses.get(i);
            // TODO_RUST_CODE
            // Java: pos.runLevel = bd.getLevelAt(pos.start);
            // TODO_RUST_CODE
            // Java: pos.primary = pos.runLevel % 2 == baseLevel % 2;
            // TODO_RUST_CODE
         }
      }
    }

    // Java: boolean isBlank(TextData data)
    pub fn isBlank(&self, TextData data) -> bool {
      // Java: for(int i = 0; i < data.mSourceText.length(); ++i)
        // TODO_RUST_CODE
      {
         // Java: char c = data.mSourceText.charAt(i);
         // TODO_RUST_CODE
         // Java: if (c != ' ' && c != '\r' && c != '\n')
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

// Java: public class LinePos
impl LinePos {
    // Java: boolean isEmpty(CharSequence text)
    pub fn isEmpty(&self, CharSequence text) -> bool {
         // Java: if (this.start >= this.end)
        // TODO_RUST_CODE
         {
            // Java: return true;
            // TODO_RUST_CODE
         }
         // Java: else
        // TODO_RUST_CODE
         {
            // Java: int len = text.length();
            // TODO_RUST_CODE
            // Java: for(int i = this.start; i < len && i < this.end; ++i)
            // TODO_RUST_CODE
            {
               // Java: char c = text.charAt(i);
               // TODO_RUST_CODE
               // Java: if (c != '\r' && c != '\n')
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

    // Java: void clipEndSpace(CharSequence text, int lineStart, int lineEnd)
    pub fn clipEndSpace(&self, CharSequence text, int lineStart, int lineEnd) {
         // Java: int i;
         // TODO_RUST_CODE
         // Java: for(i = lineEnd - 1; i >= this.start; --i)
        // TODO_RUST_CODE
         {
            // Java: char c = text.charAt(i);
            // TODO_RUST_CODE
            // Java: if (' ' != c && '\r' != c && '\n' != c)
            // TODO_RUST_CODE
            {
               // Java: break;
               // TODO_RUST_CODE
            }
         }
         // Java: if (i < this.end)
        // TODO_RUST_CODE
         {
            // Java: this.end = i + 1;
            // TODO_RUST_CODE
         }
    }

}

// Java: public class DecoLine
impl DecoLine {
    // Java: DecoLine()
    pub fn DecoLine(&self) {
         // Java: super();
         // TODO_RUST_CODE
    }

    // Java: void draw(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn draw(&self, Canvas canvas, Paint paint, boolean isAllVert) {
         // Java: switch (this.type)
        // TODO_RUST_CODE
         {
            // Java: case 1:
            // TODO_RUST_CODE
            // Java: case 4:
            // TODO_RUST_CODE
            // Java: case 101:
            // TODO_RUST_CODE
               // Java: this.drawSingleline(canvas, paint, isAllVert);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 2:
            // TODO_RUST_CODE
               // Java: this.drawDoubleline(canvas, paint, isAllVert, false);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 5:
            // TODO_RUST_CODE
            // Java: case 11:
            // TODO_RUST_CODE
               // Java: this.drawDotline(canvas, paint, isAllVert);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 6:
            // TODO_RUST_CODE
            // Java: case 12:
            // TODO_RUST_CODE
               // Java: this.drawDashline(canvas, paint, isAllVert);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 7:
            // TODO_RUST_CODE
            // Java: case 13:
            // TODO_RUST_CODE
               // Java: this.drawDashdotline(canvas, paint, isAllVert);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 8:
            // TODO_RUST_CODE
            // Java: case 14:
            // TODO_RUST_CODE
               // Java: this.drawDashdotdotline(canvas, paint, isAllVert);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 9:
            // TODO_RUST_CODE
            // Java: case 10:
            // TODO_RUST_CODE
               // Java: this.drawWaveline(canvas, paint, isAllVert);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 15:
            // TODO_RUST_CODE
            // Java: case 16:
            // TODO_RUST_CODE
               // Java: this.drawLongdashline(canvas, paint, isAllVert);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 17:
            // TODO_RUST_CODE
               // Java: this.drawDoubleWaveline(canvas, paint, isAllVert);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 102:
            // TODO_RUST_CODE
               // Java: this.drawDoubleline(canvas, paint, isAllVert, true);
               // TODO_RUST_CODE
         }
    }

    // Java: boolean LE(float l, float r)
    pub fn LE(&self, float l, float r) -> bool {
         // Java: return (int)(r - l) >= 0;
         // TODO_RUST_CODE
    }

    // Java: void drawSingleline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawSingleline(&self, Canvas canvas, Paint paint, boolean isAllVert) {
         // Java: if (isAllVert)
        // TODO_RUST_CODE
         {
            // Java: canvas.drawLine(this.y, this.startX, this.y, this.endX, paint);
            // TODO_RUST_CODE
         }
         // Java: else
        // TODO_RUST_CODE
         {
            // Java: canvas.drawLine(this.startX, this.y, this.endX, this.y, paint);
            // TODO_RUST_CODE
         }
    }

    // Java: void drawDoubleline(Canvas canvas, Paint paint, boolean isAllVert, boolean deleteLine)
    pub fn drawDoubleline(&self, Canvas canvas, Paint paint, boolean isAllVert, boolean deleteLine) {
         // Java: float vdst = this.width * 2.0F;
         // TODO_RUST_CODE
         // Java: float y1 = this.y;
         // TODO_RUST_CODE
         // Java: if (deleteLine)
        // TODO_RUST_CODE
         {
            // Java: y1 -= vdst;
            // TODO_RUST_CODE
         }
         // Java: if (isAllVert)
        // TODO_RUST_CODE
         {
            // Java: float y2 = deleteLine ? this.y + vdst : this.y - vdst;
            // TODO_RUST_CODE
            // Java: canvas.drawLine(y1, this.startX, y1, this.endX, paint);
            // TODO_RUST_CODE
            // Java: canvas.drawLine(y2, this.startX, y2, this.endX, paint);
            // TODO_RUST_CODE
         }
         // Java: else
        // TODO_RUST_CODE
         {
            // Java: float y2 = this.y + vdst;
            // TODO_RUST_CODE
            // Java: canvas.drawLine(this.startX, y1, this.endX, y1, paint);
            // TODO_RUST_CODE
            // Java: canvas.drawLine(this.startX, y2, this.endX, y2, paint);
            // TODO_RUST_CODE
         }
    }

    // Java: void drawWaveline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawWaveline(&self, Canvas canvas, Paint paint, boolean isAllVert) {
         // Java: float linespace = this.width * 8.0F;
         // TODO_RUST_CODE
         // Java: if (!this.heavy)
        // TODO_RUST_CODE
         {
            // Java: linespace *= 1.5F;
            // TODO_RUST_CODE
         }
         // Java: float y1 = this.y;
         // TODO_RUST_CODE
         // Java: float y2 = this.y + linespace;
         // TODO_RUST_CODE
         // Java: float step = y2 - y1;
         // TODO_RUST_CODE
         // Java: boolean up = true;
         // TODO_RUST_CODE
         // Java: PointF point = new PointF(this.startX, y1);
         // TODO_RUST_CODE
         // Java: Path path = new Path();
         // TODO_RUST_CODE
         // Java: if (isAllVert)
        // TODO_RUST_CODE
         {
            // Java: path.moveTo(point.y, point.x);
            // TODO_RUST_CODE
         }
         // Java: else
        // TODO_RUST_CODE
         {
            // Java: path.moveTo(point.x, point.y);
            // TODO_RUST_CODE
         }
         // Java: while(point.x < this.endX)
        // TODO_RUST_CODE
         {
            // Java: point.x += step;
            // TODO_RUST_CODE
            // Java: point.y = up ? y2 : y1;
            // TODO_RUST_CODE
            // Java: up = !up;
            // TODO_RUST_CODE
            // Java: if (isAllVert)
            // TODO_RUST_CODE
            {
               // Java: path.lineTo(point.y, point.x);
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: path.lineTo(point.x, point.y);
               // TODO_RUST_CODE
            }
         }
         // Java: canvas.drawPath(path, paint);
         // TODO_RUST_CODE
    }

    // Java: void drawDoubleWaveline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawDoubleWaveline(&self, Canvas canvas, Paint paint, boolean isAllVert) {
         // Java: float linespace = this.width * 4.0F;
         // TODO_RUST_CODE
         // Java: if (!this.heavy)
        // TODO_RUST_CODE
         {
            // Java: linespace *= 1.5F;
            // TODO_RUST_CODE
         }
         // Java: float y1 = this.y;
         // TODO_RUST_CODE
         // Java: float y2 = this.y + linespace;
         // TODO_RUST_CODE
         // Java: float step = y2 - y1;
         // TODO_RUST_CODE
         // Java: float vdst = this.width * 4.0F;
         // TODO_RUST_CODE
         // Java: boolean up = true;
         // TODO_RUST_CODE
         // Java: PointF point = new PointF(this.startX, y1);
         // TODO_RUST_CODE
         // Java: Path path1 = new Path();
         // TODO_RUST_CODE
         // Java: Path path2 = new Path();
         // TODO_RUST_CODE
         // Java: if (isAllVert)
        // TODO_RUST_CODE
         {
            // Java: path1.moveTo(point.y, point.x);
            // TODO_RUST_CODE
            // Java: path2.moveTo(point.y + vdst, point.x);
            // TODO_RUST_CODE
         }
         // Java: else
        // TODO_RUST_CODE
         {
            // Java: path1.moveTo(point.x, point.y);
            // TODO_RUST_CODE
            // Java: path2.moveTo(point.x, point.y + vdst);
            // TODO_RUST_CODE
         }
         // Java: while(point.x < this.endX)
        // TODO_RUST_CODE
         {
            // Java: point.x += step;
            // TODO_RUST_CODE
            // Java: point.y = up ? y2 : y1;
            // TODO_RUST_CODE
            // Java: up = !up;
            // TODO_RUST_CODE
            // Java: if (isAllVert)
            // TODO_RUST_CODE
            {
               // Java: path1.lineTo(point.y, point.x);
               // TODO_RUST_CODE
               // Java: path2.lineTo(point.y + vdst, point.x);
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: path1.lineTo(point.x, point.y);
               // TODO_RUST_CODE
               // Java: path2.lineTo(point.x, point.y + vdst);
               // TODO_RUST_CODE
            }
         }
         // Java: canvas.drawPath(path1, paint);
         // TODO_RUST_CODE
         // Java: canvas.drawPath(path2, paint);
         // TODO_RUST_CODE
    }

    // Java: void drawDotline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawDotline(&self, Canvas canvas, Paint paint, boolean isAllVert) {
         // Java: float step = 2.0F * this.width;
         // TODO_RUST_CODE
         // Java: for(float posX = this.startX; posX <= this.endX; posX += step)
        // TODO_RUST_CODE
         {
            // Java: if (isAllVert)
            // TODO_RUST_CODE
            {
               // Java: canvas.drawLine(this.y, posX, this.y, posX + this.width, paint);
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: canvas.drawLine(posX, this.y, posX + this.width, this.y, paint);
               // TODO_RUST_CODE
            }
         }
    }

    // Java: void drawDashline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawDashline(&self, Canvas canvas, Paint paint, boolean isAllVert) {
         // Java: float step = 5.0F * this.width;
         // TODO_RUST_CODE
         // Java: float len = 3.0F * this.width;
         // TODO_RUST_CODE
         // Java: for(float posX = this.startX; posX < this.endX; posX += step)
        // TODO_RUST_CODE
         {
            // Java: if (isAllVert)
            // TODO_RUST_CODE
            {
               // Java: canvas.drawLine(this.y, posX, this.y, posX + len, paint);
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: canvas.drawLine(posX, this.y, posX + len, this.y, paint);
               // TODO_RUST_CODE
            }
         }
    }

    // Java: void drawLongdashline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawLongdashline(&self, Canvas canvas, Paint paint, boolean isAllVert) {
         // Java: float step = 8.0F * this.width;
         // TODO_RUST_CODE
         // Java: float len = 5.0F * this.width;
         // TODO_RUST_CODE
         // Java: for(float posX = this.startX; posX < this.endX; posX += step)
        // TODO_RUST_CODE
         {
            // Java: if (isAllVert)
            // TODO_RUST_CODE
            {
               // Java: canvas.drawLine(this.y, posX, this.y, posX + len, paint);
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: canvas.drawLine(posX, this.y, posX + len, this.y, paint);
               // TODO_RUST_CODE
            }
         }
    }

    // Java: void drawDashdotline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawDashdotline(&self, Canvas canvas, Paint paint, boolean isAllVert) {
         // Java: float step = 8.0F * this.width;
         // TODO_RUST_CODE
         // Java: float len = 3.0F * this.width;
         // TODO_RUST_CODE
         // Java: float dotdst = 5.0F * this.width;
         // TODO_RUST_CODE
         // Java: for(float posX = this.startX; posX < this.endX; posX += step)
        // TODO_RUST_CODE
         {
            // Java: if (isAllVert)
            // TODO_RUST_CODE
            {
               // Java: canvas.drawLine(this.y, posX, this.y, posX + len, paint);
               // TODO_RUST_CODE
               // Java: canvas.drawLine(this.y, posX + dotdst, this.y, posX + dotdst + this.width, paint);
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: canvas.drawLine(posX, this.y, posX + len, this.y, paint);
               // TODO_RUST_CODE
               // Java: canvas.drawLine(posX + dotdst, this.y, posX + dotdst + this.width, this.y, paint);
               // TODO_RUST_CODE
            }
         }
    }

    // Java: void drawDashdotdotline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawDashdotdotline(&self, Canvas canvas, Paint paint, boolean isAllVert) {
         // Java: float step = 11.0F * this.width;
         // TODO_RUST_CODE
         // Java: float len = 3.0F * this.width;
         // TODO_RUST_CODE
         // Java: float dotdst1 = 5.0F * this.width;
         // TODO_RUST_CODE
         // Java: float dotdst2 = 8.0F * this.width;
         // TODO_RUST_CODE
         // Java: for(float posX = this.startX; posX < this.endX; posX += step)
        // TODO_RUST_CODE
         {
            // Java: if (isAllVert)
            // TODO_RUST_CODE
            {
               // Java: canvas.drawLine(this.y, posX, this.y, posX + len, paint);
               // TODO_RUST_CODE
               // Java: canvas.drawLine(this.y, posX + dotdst1, this.y, posX + dotdst1 + this.width, paint);
               // TODO_RUST_CODE
               // Java: canvas.drawLine(this.y, posX + dotdst2, this.y, posX + dotdst2 + this.width, paint);
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: canvas.drawLine(posX, this.y, posX + len, this.y, paint);
               // TODO_RUST_CODE
               // Java: canvas.drawLine(posX + dotdst1, this.y, posX + dotdst1 + this.width, this.y, paint);
               // TODO_RUST_CODE
               // Java: canvas.drawLine(posX + dotdst2, this.y, posX + dotdst2 + this.width, this.y, paint);
               // TODO_RUST_CODE
            }
         }
    }

}
