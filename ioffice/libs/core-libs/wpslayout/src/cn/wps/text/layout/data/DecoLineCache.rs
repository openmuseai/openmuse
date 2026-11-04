// Java: import android.graphics.Canvas
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.PointF
// Java: import android.graphics.RectF
// Java: import android.graphics.Paint.Join
// Java: import android.graphics.Paint.Style
// Java: import cn.wps.text.layout.typo.layout.Layout
// Java: import java.text.Bidi
// Java: import java.util.ArrayList
// Java: import java.util.List

use crate::cn::wps::text::layout::typo::layout::Layout::Layout;
use super::TextData::TextData;
use super::SpanInfo::SpanInfo;

pub struct Canvas;
impl Canvas {
    pub fn drawLine(&self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _paint: &Paint) {}
    pub fn drawPath(&self, _path: &Path, _paint: &Paint) {}
}

pub struct Paint {
    color: i32,
    stroke_width: f32,
}
impl Paint {
    pub fn new() -> Self { Paint { color: 0, stroke_width: 0.0 } }
    pub fn reset(&mut self) {}
    pub fn setStyle(&mut self, _style: Style) {}
    pub fn setAntiAlias(&mut self, _aa: bool) {}
    pub fn setStrokeJoin(&mut self, _join: Join) {}
    pub fn setColor(&mut self, color: i32) { self.color = color; }
    pub fn setStrokeWidth(&mut self, width: f32) { self.stroke_width = width; }
}

pub enum Style { STROKE }
pub enum Join { ROUND }

pub struct Path { commands: Vec<(f32, f32)> }
impl Path {
    pub fn new() -> Self { Path { commands: Vec::new() } }
    pub fn moveTo(&mut self, x: f32, y: f32) { self.commands.push((x, y)); }
    pub fn lineTo(&mut self, x: f32, y: f32) { self.commands.push((x, y)); }
}

pub struct PointF { pub x: f32, pub y: f32 }
impl PointF {
    pub fn new(x: f32, y: f32) -> Self { PointF { x, y } }
}

pub struct RectF;

// Java field: short CELL_MARGIN
pub(crate) const CELL_MARGIN: i16 = 2;
// Java field: float DECOLINE_SCALE
pub(crate) const DECOLINE_SCALE: f32 = 0.05;
// Java field: float DOUBLE_LINE_WIDTH_FACTOR
pub(crate) const DOUBLE_LINE_WIDTH_FACTOR: f32 = 0.5;
// Java field: float WAVE_LINE_WIDTH_FACTOR
pub(crate) const WAVE_LINE_WIDTH_FACTOR: f32 = 0.125;
// Java field: float DOUBLE_LINE_VD_FACTOR
pub(crate) const DOUBLE_LINE_VD_FACTOR: f32 = 2.0;
// Java field: float WAVE_LINE_VD_FACTOR
pub(crate) const WAVE_LINE_VD_FACTOR: f32 = 8.0;
// Java field: float DOUBLE_WAVE_LINE_VD_FACTOR
pub(crate) const DOUBLE_WAVE_LINE_VD_FACTOR: f32 = 4.0;
// Java field: float UNDERLINE_MOVEDOWN
pub(crate) const UNDERLINE_MOVEDOWN: f32 = 0.1;
// Java field: short DECOLINE_CLASS_UNDERLINE
pub(crate) const DECOLINE_CLASS_UNDERLINE: i16 = 0;
// Java field: short DECOLINE_CLASS_STRIKETHROUGH
pub(crate) const DECOLINE_CLASS_STRIKETHROUGH: i16 = 1;
// Java field: float HEAVY_FACTOR
pub(crate) const HEAVY_FACTOR: f32 = 1.5;
// Java field: float DOUBLE_WAVE_DISTANCE_FACTOR
pub(crate) const DOUBLE_WAVE_DISTANCE_FACTOR: f32 = 5.0;

// Java: public class DecoLineCache
pub struct DecoLineCache {
    // Java field: ArrayList<DecoLine> mDecoLines
    pub(crate) mDecoLines: Vec<DecoLine>,
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
    pub(crate) r#type: i8,
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
    pub(crate) r#type: i8,
    // Java field: boolean heavy
    pub(crate) heavy: bool,
}

// Java: public class DecoLineCache
impl DecoLineCache {
    // Java: DecoLineCache()
    pub fn new() -> Self {
      // Java: super();
      DecoLineCache { mDecoLines: Vec::new(), mPaint: None }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mDecoLines.clear();
      self.mDecoLines.clear();
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.mDecoLines.isEmpty();
      self.mDecoLines.is_empty()
    }

    // Java: void draw(Canvas canvas, List<RectF> txtRects, boolean isAllVert)
    pub fn draw(&mut self, canvas: &Canvas, _txtRects: &[RectF], isAllVert: bool) {
      // Java: if (!this.mDecoLines.isEmpty())
      if !self.mDecoLines.is_empty()
      {
         // Java: if (this.mPaint == null)
         if self.mPaint.is_none()
         {
            // Java: this.mPaint = new Paint();
            self.mPaint = Some(Paint::new());
         }
         // Java: this.mPaint.reset();
         let paint = self.mPaint.as_mut().unwrap();
         paint.reset();
         // Java: this.mPaint.setStyle(Style.STROKE);
         paint.setStyle(Style::STROKE);
         // Java: this.mPaint.setAntiAlias(true);
         paint.setAntiAlias(true);
         // Java: this.mPaint.setStrokeJoin(Join.ROUND);
         paint.setStrokeJoin(Join::ROUND);
         // Java: for(DecoLine decoLine : this.mDecoLines)
         for i in 0..self.mDecoLines.len()
         {
            // Java: this.mPaint.setColor(decoLine.color);
            let color = self.mDecoLines[i].color;
            let width = self.mDecoLines[i].width;
            let paint = self.mPaint.as_mut().unwrap();
            paint.setColor(color);
            // Java: this.mPaint.setStrokeWidth(decoLine.width);
            paint.setStrokeWidth(width);
            // Java: decoLine.draw(canvas, this.mPaint, isAllVert);
            self.mDecoLines[i].draw(canvas, self.mPaint.as_ref().unwrap(), isAllVert);
         }
      }
    }

    // Java: void calcDecoLine(Layout typoCache, TextData data)
    pub fn calcDecoLine(&mut self, typoCache: &Layout, data: &TextData) {
      // Java: this.mDecoLines.clear();
      self.mDecoLines.clear();
      // Java: if (!this.isBlank(data))
      if !Self::isBlank_static(data)
      {
         // Java: this.calcDecoLine(typoCache, data, (short)0);
         self.calcDecoLine_with_class(typoCache, data, 0);
         // Java: this.calcDecoLine(typoCache, data, (short)1);
         self.calcDecoLine_with_class(typoCache, data, 1);
      }
    }

    // Java: void calcDecoLine(Layout typoCache, TextData data, short lineClass)
    pub fn calcDecoLine_with_class(&mut self, typoCache: &Layout, data: &TextData, lineClass: i16) {
      // Java: boolean isAllVertLtr = data.mVertical == 3;
      let isAllVertLtr = data.mVertical == 3;
      // Java: boolean isAllVertRtl = data.mVertical == 6;
      let isAllVertRtl = data.mVertical == 6;
      // Java: int lineCount = typoCache.getLineCount();
      let lineCount = typoCache.getLineCount();
      // Java: int[] lineEnds = new int[lineCount];
      let mut lineEnds = vec![0i32; lineCount as usize];
      // Java: for(int i = 0; i < lineCount; ++i)
      for i in 0..lineCount
      {
         // Java: lineEnds[i] = typoCache.getLineEnd(i);
         lineEnds[i as usize] = typoCache.getLineEnd(i);
      }
      // Java: ArrayList<LinePos> linesPos = new ArrayList();
      let mut linesPos: Vec<LinePos> = Vec::new();
      // Java: SpanInfo span = data.mFirstSpan;
      let mut span = &data.mFirstSpan;
      // Java: int spanIdx = 0;
      let mut spanIdx: usize = 0;
      // Java: int lineIdx = 0;
      let mut lineIdx = 0i32;
      // Java: while(true)
      loop
      {
         // Java: boolean hasDecoLine = false;
         let mut hasDecoLine = false;
         // Java: int color = span.mFontColor;
         let mut color = span.mFontColor;
         // Java: byte lineType = 0;
         let mut lineType: i8 = 0;
         // Java: boolean isHeavy = false;
         let mut isHeavy = false;
         // Java: switch (lineClass)
         match lineClass
         {
            // Java: case 0:
            0 => {
               // Java: hasDecoLine = span.hasUnderline();
               hasDecoLine = span.hasUnderline();
               // Java: color = span.mUnderlineColor;
               color = span.mUnderlineColor;
               // Java: lineType = span.mUnderline;
               lineType = span.mUnderline as i8;
               // Java: isHeavy = span.isUnderlineHeavy();
               isHeavy = span.isUnderlineHeavy();
               // Java: break;
            }
            // Java: case 1:
            1 => {
               // Java: hasDecoLine = span.hasStrikethroughLine();
               hasDecoLine = span.hasStrikethroughLine();
               // Java: lineType = span.mStrikethrough;
               lineType = span.mStrikethrough as i8;
            }
            _ => {}
         }
         // Java: if (hasDecoLine)
         if hasDecoLine
         {
            // Java: boolean isAccouting = span.isAcounting();
            let isAccouting = span.isAcounting();
            // Java: LinePos pos = new LinePos();
            let mut pos = LinePos::new();
            // Java: pos.start = span.mStartPos;
            pos.start = span.mStartPos;
            // Java: pos.end = span.mEndPos;
            pos.end = span.mEndPos;
            // Java: if (data.mPara.indexMap != null)
            if let Some(ref indexMap) = data.mPara.indexMap
            {
               // Java: int start = pos.start;
               let start = pos.start;
               // Java: int end = pos.end;
               let end = pos.end;
               // Java: pos.start = data.mPara.indexMap[start];
               pos.start = indexMap[start as usize];
               // Java: pos.end = data.mPara.indexMap[start];
               pos.end = indexMap[start as usize];
               // Java: for(int i = start + 1; i < end; ++i)
               for i in (start + 1)..end
               {
                  // Java: int index = data.mPara.indexMap[i];
                  let index = indexMap[i as usize];
                  // Java: if (pos.start > index)
                  if pos.start > index
                  {
                     // Java: pos.start = index;
                     pos.start = index;
                  }
                  // Java: if (pos.end < index)
                  if pos.end < index
                  {
                     // Java: pos.end = index;
                     pos.end = index;
                  }
               }
               // Java: if (pos.start > pos.end)
               if pos.start > pos.end
               {
                  // Java: int temp = pos.start;
                  let temp = pos.start;
                  // Java: pos.start = pos.end;
                  pos.start = pos.end;
                  // Java: pos.end = temp;
                  pos.end = temp;
               }
               // Java: ++pos.end;
               pos.end += 1;
            }
            // Java: pos.color = color;
            pos.color = color;
            // Java: pos.fontSize = span.mFontSize;
            pos.fontSize = span.mFontSize;
            // Java: pos.accounting = isAccouting;
            pos.accounting = isAccouting;
            // Java: pos.ascent = span.getAscent();
            pos.ascent = span.getAscent();
            // Java: pos.type = lineType;
            pos.r#type = lineType;
            // Java: pos.isHeavy = isHeavy;
            pos.isHeavy = isHeavy;
            // Java: pos.isSubscript = span.mIsSubScript;
            pos.isSubscript = span.mIsSubScript;
            // Java: for(pos.isSuperscript = span.mIsSuperScript; lineIdx < lineCount; ++lineIdx)
            pos.isSuperscript = span.mIsSuperScript;
            while lineIdx < lineCount
            {
               // Java: int lineEnd = lineEnds[lineIdx];
               let lineEnd = lineEnds[lineIdx as usize];
               // Java: pos.line = lineIdx;
               pos.line = lineIdx;
               // Java: pos.lineEnd = lineEnd;
               pos.lineEnd = lineEnd;
               // Java: if (pos.start < lineEnd && pos.end <= lineEnd)
               if pos.start < lineEnd && pos.end <= lineEnd
               {
                  // Java: linesPos.add(pos);
                  linesPos.push(pos);
                  // Java: break;
                  lineIdx += 1;
                  break;
               }
               // Java: if (pos.start < lineEnd && pos.end > lineEnd)
               if pos.start < lineEnd && pos.end > lineEnd
               {
                  // Java: LinePos newPos = new LinePos();
                  let mut newPos = LinePos::new();
                  // Java: newPos.start = lineEnd;
                  newPos.start = lineEnd;
                  // Java: newPos.end = pos.end;
                  newPos.end = pos.end;
                  // Java: newPos.color = pos.color;
                  newPos.color = pos.color;
                  // Java: newPos.fontSize = pos.fontSize;
                  newPos.fontSize = pos.fontSize;
                  // Java: newPos.type = pos.type;
                  newPos.r#type = pos.r#type;
                  // Java: newPos.accounting = pos.accounting;
                  newPos.accounting = pos.accounting;
                  // Java: newPos.ascent = pos.ascent;
                  newPos.ascent = pos.ascent;
                  // Java: newPos.isHeavy = pos.isHeavy;
                  newPos.isHeavy = pos.isHeavy;
                  // Java: pos.end = lineEnd;
                  pos.end = lineEnd;
                  // Java: linesPos.add(pos);
                  linesPos.push(pos);
                  // Java: pos = newPos;
                  pos = newPos;
               }
               lineIdx += 1;
            }
         }
         // Java: if (spanIdx >= data.mOtherSpans.size())
         if spanIdx >= data.mOtherSpans.len()
         {
            // Java: float[] moveDownRef = new float[typoCache.getLineCount()];
            let mut moveDownRef = vec![0.0f32; typoCache.getLineCount() as usize];
            // Java: if (0 == lineClass)
            if 0 == lineClass
            {
               // Java: for(int i = 0; i < linesPos.size(); ++i)
               for i in 0..linesPos.len()
               {
                  // Java: LinePos p = (LinePos)linesPos.get(i);
                  let p = &linesPos[i];
                  // Java: if (moveDownRef[p.line] < p.fontSize)
                  if moveDownRef[p.line as usize] < p.fontSize
                  {
                     // Java: moveDownRef[p.line] = p.fontSize;
                     moveDownRef[p.line as usize] = p.fontSize;
                  }
               }
            }
            // Java: CharSequence text = typoCache.getText();
            let text = typoCache.getText();
            // Java: for(int i = 0; i < linesPos.size(); ++i)
            for i in 0..linesPos.len()
            {
               // Java: LinePos p = (LinePos)linesPos.get(i);
               let p = &mut linesPos[i];
               // Java: if (!p.isEmpty(text))
               if !p.isEmpty_text(text)
               {
                  // Java: p.clipEndSpace(text, typoCache.getLineStart(p.line), p.lineEnd);
                  let p_line = p.line;
                  let p_line_end = p.lineEnd;
                  p.clipEndSpace(text, typoCache.getLineStart(p_line), p_line_end);
                  // Java: DecoLine decoLine = new DecoLine();
                  let mut decoLine = DecoLine::new();
                  // Java: decoLine.color = p.color;
                  decoLine.color = p.color;
                  // Java: decoLine.width = p.fontSize * 0.05F;
                  decoLine.width = p.fontSize * 0.05;
                  // Java: decoLine.type = p.type;
                  decoLine.r#type = p.r#type;
                  // Java: decoLine.heavy = p.isHeavy;
                  decoLine.heavy = p.isHeavy;
                  // Java: if (p.isHeavy)
                  if p.isHeavy
                  {
                     // Java: decoLine.width *= 1.5F;
                     decoLine.width *= 1.5;
                  }
                  // Java: switch (p.type)
                  match p.r#type
                  {
                     // Java: case 2:
                     // Java: case 102:
                     2 | 102 => {
                        // Java: decoLine.width *= 0.5F;
                        decoLine.width *= 0.5;
                        // Java: break;
                     }
                     // Java: case 9:
                     // Java: case 10:
                     // Java: case 17:
                     9 | 10 | 17 => {
                        // Java: decoLine.width *= 0.125F;
                        decoLine.width *= 0.125;
                     }
                     _ => {}
                  }
                  // Java: switch (lineClass)
                  match lineClass
                  {
                     // Java: case 0:
                     0 => {
                        // Java: decoLine.yCorrection = moveDownRef[p.line] * 0.1F;
                        decoLine.yCorrection = moveDownRef[p.line as usize] * 0.1;
                        // Java: if (isAllVertLtr)
                        if isAllVertLtr
                        {
                           // Java: decoLine.y = typoCache.getLineTop(p.line);
                           decoLine.y = typoCache.getLineTop(p.line);
                        }
                        // Java: else if (isAllVertRtl)
                        else if isAllVertRtl
                        {
                           // Java: decoLine.y = typoCache.getLineBottom(p.line);
                           decoLine.y = typoCache.getLineBottom(p.line);
                        }
                        // Java: else
                        else
                        {
                           // Java: decoLine.y = typoCache.getLineBaseline(p.line) + decoLine.yCorrection;
                           decoLine.y = typoCache.getLineBaseline(p.line) + decoLine.yCorrection;
                        }
                        // Java: if (isAllVertRtl)
                        if isAllVertRtl
                        {
                           // Java: decoLine.y = -decoLine.y;
                           decoLine.y = -decoLine.y;
                        }
                        // Java: break;
                     }
                     // Java: case 1:
                     1 => {
                        // Java: decoLine.yCorrection = p.ascent / 3.0F + decoLine.width;
                        decoLine.yCorrection = p.ascent / 3.0 + decoLine.width;
                        // Java: if (!isAllVertLtr && !isAllVertRtl)
                        if !isAllVertLtr && !isAllVertRtl
                        {
                           // Java: decoLine.y = typoCache.getLineBaseline(p.line) + decoLine.yCorrection;
                           decoLine.y = typoCache.getLineBaseline(p.line) + decoLine.yCorrection;
                        }
                        // Java: else
                        else
                        {
                           // Java: decoLine.y = (typoCache.getLineTop(p.line) + typoCache.getLineBottom(p.line)) / 2.0F;
                           decoLine.y = (typoCache.getLineTop(p.line) + typoCache.getLineBottom(p.line)) / 2.0;
                        }
                        // Java: if (isAllVertRtl)
                        if isAllVertRtl
                        {
                           // Java: decoLine.y = -decoLine.y;
                           decoLine.y = -decoLine.y;
                        }
                     }
                     _ => {}
                  }
                  // Java: boolean isUp = false;
                  let mut isUp = false;
                  // Java: boolean isDown = false;
                  let mut isDown = false;
                  // Java: if (!isAllVertLtr && !isAllVertRtl)
                  if !isAllVertLtr && !isAllVertRtl
                  {
                     // Java: isUp = p.isSuperscript;
                     isUp = p.isSuperscript;
                     // Java: isDown = p.isSubscript;
                     isDown = p.isSubscript;
                  }
                  // Java: else
                  else
                  {
                     // Java: isUp = p.isSubscript;
                     isUp = p.isSubscript;
                     // Java: isDown = p.isSuperscript;
                     isDown = p.isSuperscript;
                  }
                  // Java: if (isDown)
                  if isDown
                  {
                     // Java: decoLine.y = (float)((double)decoLine.y + (double)(typoCache.getLineBaseline(p.line) - typoCache.getLineTop(p.line)) * 0.3);
                     decoLine.y = (decoLine.y as f64 + (typoCache.getLineBaseline(p.line) - typoCache.getLineTop(p.line)) as f64 * 0.3) as f32;
                  }
                  // Java: else if (isUp)
                  else if isUp
                  {
                     // Java: decoLine.y = (float)((double)decoLine.y - (double)(typoCache.getLineBaseline(p.line) - typoCache.getLineTop(p.line)) * 0.4);
                     decoLine.y = (decoLine.y as f64 - (typoCache.getLineBaseline(p.line) - typoCache.getLineTop(p.line)) as f64 * 0.4) as f32;
                  }
                  // Java: if (p.accounting)
                  if p.accounting
                  {
                     // Java: decoLine.startX = 2.0F;
                     decoLine.startX = 2.0;
                     // Java: decoLine.endX = (float)(typoCache.getWidth() - 2);
                     decoLine.endX = (typoCache.getWidth() - 2) as f32;
                  }
                  // Java: else
                  else
                  {
                     // Java: if (p.primary)
                     if p.primary
                     {
                        // Java: decoLine.startX = typoCache.getPrimaryHorizontal(p.start);
                        decoLine.startX = typoCache.getPrimaryHorizontal(p.start);
                     }
                     // Java: else
                     else
                     {
                        // Java: decoLine.startX = typoCache.getSecondaryHorizontal(p.start);
                        decoLine.startX = typoCache.getSecondaryHorizontal(p.start);
                     }
                     // Java: decoLine.endX = typoCache.getSecondaryHorizontal(p.end > 0 ? p.end - 1 : p.end);
                     decoLine.endX = typoCache.getSecondaryHorizontal(if p.end > 0 { p.end - 1 } else { p.end });
                  }
                  // Java: if (decoLine.startX > decoLine.endX)
                  if decoLine.startX > decoLine.endX
                  {
                     // Java: float temp = decoLine.startX;
                     let temp = decoLine.startX;
                     // Java: decoLine.startX = decoLine.endX;
                     decoLine.startX = decoLine.endX;
                     // Java: decoLine.endX = temp;
                     decoLine.endX = temp;
                  }
                  // Java: this.mDecoLines.add(decoLine);
                  self.mDecoLines.push(decoLine);
               }
            }
            // Java: if (0 == lineClass)
            if 0 == lineClass
            {
               // Java: this.mergeLines(this.mDecoLines);
               Self::mergeLines_static(&mut self.mDecoLines);
            }
            // Java: return;
            return;
         }
         // Java: span = (SpanInfo)data.mOtherSpans.get(spanIdx);
         span = &data.mOtherSpans[spanIdx];
         // Java: ++spanIdx;
         spanIdx += 1;
      }
    }

    // Java: void mergeLines(ArrayList<DecoLine> lines)
    fn mergeLines_static(lines: &mut Vec<DecoLine>) {
      // Java: int i = 0;
      let mut i = 0usize;
      // Java: if (null != lines)
      {
         // Java: while(i < lines.size() - 1)
         while lines.len() > 0 && i < lines.len() - 1
         {
            // Java: DecoLine line1 = (DecoLine)lines.get(i);
            // Java: DecoLine line2 = (DecoLine)lines.get(i + 1);
            // Java: if (this.canMergeLines(line1, line2))
            if Self::canMergeLines_static(&lines[i], &lines[i + 1])
            {
               // Java: line2.startX = line1.startX < line2.startX ? line1.startX : line2.startX;
               let new_startX = if lines[i].startX < lines[i + 1].startX { lines[i].startX } else { lines[i + 1].startX };
               // Java: line2.endX = line1.endX > line2.endX ? line1.endX : line2.endX;
               let new_endX = if lines[i].endX > lines[i + 1].endX { lines[i].endX } else { lines[i + 1].endX };
               // Java: line2.width = line1.width > line2.width ? line1.width : line2.width;
               let new_width = if lines[i].width > lines[i + 1].width { lines[i].width } else { lines[i + 1].width };
               lines[i + 1].startX = new_startX;
               lines[i + 1].endX = new_endX;
               lines[i + 1].width = new_width;
               // Java: lines.remove(i);
               lines.remove(i);
            }
            // Java: else
            else
            {
               // Java: ++i;
               i += 1;
            }
         }
      }
    }

    // Java: boolean canMergeLines(DecoLine line1, DecoLine line2)
    fn canMergeLines_static(line1: &DecoLine, line2: &DecoLine) -> bool {
      // Java: float ALLOW_ERROR = 1.0E-10F;
      // Java: return line1.color == line2.color && line1.type == line2.type && (line1.startX < line2.startX && line1.endX >= line2.startX || line2.startX < line1.startX && line2.endX >= line1.startX) && Math.abs(line1.y - line2.y) < 1.0E-10F;
      line1.color == line2.color && line1.r#type == line2.r#type
          && (line1.startX < line2.startX && line1.endX >= line2.startX
              || line2.startX < line1.startX && line2.endX >= line1.startX)
          && (line1.y - line2.y).abs() < 1.0E-10
    }

    // Java: void calcDirection(TextData data, ArrayList<LinePos> poses)
    pub fn calcDirection(&self, _data: &TextData, _poses: &mut Vec<LinePos>) {
      // Java: Bidi bd = data.mBidi;
      // Java: if (null != bd)
      {
         // Java: int baseLevel = bd.getBaseLevel();
         // Java: for(int i = 0; i < poses.size(); ++i)
         {
            // Java: LinePos pos = (LinePos)poses.get(i);
            // Java: pos.runLevel = bd.getLevelAt(pos.start);
            // Java: pos.primary = pos.runLevel % 2 == baseLevel % 2;
         }
      }
    }

    // Java: boolean isBlank(TextData data)
    fn isBlank_static(data: &TextData) -> bool {
      // Java: for(int i = 0; i < data.mSourceText.length(); ++i)
      if let Some(ref source_text) = data.mSourceText {
          for c in source_text.chars()
          {
             // Java: char c = data.mSourceText.charAt(i);
             // Java: if (c != ' ' && c != '\r' && c != '\n')
             if c != ' ' && c != '\r' && c != '\n'
             {
                // Java: return false;
                return false;
             }
          }
      }
      // Java: return true;
      true
    }

}

// Java: public class LinePos
impl LinePos {
    pub fn new() -> Self {
        LinePos {
            start: 0, end: 0, line: -1, lineEnd: 0,
            color: -1, fontSize: 0.0, ascent: 0.0, runLevel: 0,
            r#type: 0, accounting: false, primary: true,
            isHeavy: false, isSubscript: false, isSuperscript: false,
        }
    }

    // Java: boolean isEmpty(CharSequence text)
    pub fn isEmpty_text(&self, text: &str) -> bool {
         // Java: if (this.start >= this.end)
         if self.start >= self.end
         {
            // Java: return true;
            return true;
         }
         // Java: else
         {
            // Java: int len = text.length();
            let len = text.len() as i32;
            // Java: for(int i = this.start; i < len && i < this.end; ++i)
            for (idx, c) in text.chars().enumerate() {
                let i = idx as i32;
                if i < self.start { continue; }
                if i >= len || i >= self.end { break; }
               // Java: char c = text.charAt(i);
               // Java: if (c != '\r' && c != '\n')
               if c != '\r' && c != '\n'
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: return true;
            true
         }
    }

    // Java: void clipEndSpace(CharSequence text, int lineStart, int lineEnd)
    pub fn clipEndSpace(&mut self, text: &str, _lineStart: i32, lineEnd: i32) {
         // Java: int i;
         let mut i = lineEnd - 1;
         // Java: for(i = lineEnd - 1; i >= this.start; --i)
         let chars: Vec<char> = text.chars().collect();
         while i >= self.start
         {
            // Java: char c = text.charAt(i);
            let c = if (i as usize) < chars.len() { chars[i as usize] } else { break };
            // Java: if (' ' != c && '\r' != c && '\n' != c)
            if ' ' != c && '\r' != c && '\n' != c
            {
               // Java: break;
               break;
            }
            i -= 1;
         }
         // Java: if (i < this.end)
         if i < self.end
         {
            // Java: this.end = i + 1;
            self.end = i + 1;
         }
    }

}

// Java: public class DecoLine
impl DecoLine {
    // Java: DecoLine()
    pub fn new() -> Self {
         // Java: super();
         DecoLine { startX: 0.0, endX: 0.0, y: 0.0, yCorrection: 0.0, color: 0, width: 0.0, r#type: 0, heavy: false }
    }

    // Java: void draw(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn draw(&self, canvas: &Canvas, paint: &Paint, isAllVert: bool) {
         // Java: switch (this.type)
         match self.r#type
         {
            // Java: case 1:
            // Java: case 4:
            // Java: case 101:
            1 | 4 | 101 => {
               // Java: this.drawSingleline(canvas, paint, isAllVert);
               self.drawSingleline(canvas, paint, isAllVert);
               // Java: break;
            }
            // Java: case 2:
            2 => {
               // Java: this.drawDoubleline(canvas, paint, isAllVert, false);
               self.drawDoubleline(canvas, paint, isAllVert, false);
               // Java: break;
            }
            // Java: case 5:
            // Java: case 11:
            5 | 11 => {
               // Java: this.drawDotline(canvas, paint, isAllVert);
               self.drawDotline(canvas, paint, isAllVert);
               // Java: break;
            }
            // Java: case 6:
            // Java: case 12:
            6 | 12 => {
               // Java: this.drawDashline(canvas, paint, isAllVert);
               self.drawDashline(canvas, paint, isAllVert);
               // Java: break;
            }
            // Java: case 7:
            // Java: case 13:
            7 | 13 => {
               // Java: this.drawDashdotline(canvas, paint, isAllVert);
               self.drawDashdotline(canvas, paint, isAllVert);
               // Java: break;
            }
            // Java: case 8:
            // Java: case 14:
            8 | 14 => {
               // Java: this.drawDashdotdotline(canvas, paint, isAllVert);
               self.drawDashdotdotline(canvas, paint, isAllVert);
               // Java: break;
            }
            // Java: case 9:
            // Java: case 10:
            9 | 10 => {
               // Java: this.drawWaveline(canvas, paint, isAllVert);
               self.drawWaveline(canvas, paint, isAllVert);
               // Java: break;
            }
            // Java: case 15:
            // Java: case 16:
            15 | 16 => {
               // Java: this.drawLongdashline(canvas, paint, isAllVert);
               self.drawLongdashline(canvas, paint, isAllVert);
               // Java: break;
            }
            // Java: case 17:
            17 => {
               // Java: this.drawDoubleWaveline(canvas, paint, isAllVert);
               self.drawDoubleWaveline(canvas, paint, isAllVert);
               // Java: break;
            }
            // Java: case 102:
            102 => {
               // Java: this.drawDoubleline(canvas, paint, isAllVert, true);
               self.drawDoubleline(canvas, paint, isAllVert, true);
            }
            _ => {}
         }
    }

    // Java: boolean LE(float l, float r)
    pub fn LE(&self, l: f32, r: f32) -> bool {
         // Java: return (int)(r - l) >= 0;
         (r - l) as i32 >= 0
    }

    // Java: void drawSingleline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawSingleline(&self, canvas: &Canvas, paint: &Paint, isAllVert: bool) {
         // Java: if (isAllVert)
         if isAllVert
         {
            // Java: canvas.drawLine(this.y, this.startX, this.y, this.endX, paint);
            canvas.drawLine(self.y, self.startX, self.y, self.endX, paint);
         }
         // Java: else
         else
         {
            // Java: canvas.drawLine(this.startX, this.y, this.endX, this.y, paint);
            canvas.drawLine(self.startX, self.y, self.endX, self.y, paint);
         }
    }

    // Java: void drawDoubleline(Canvas canvas, Paint paint, boolean isAllVert, boolean deleteLine)
    pub fn drawDoubleline(&self, canvas: &Canvas, paint: &Paint, isAllVert: bool, deleteLine: bool) {
         // Java: float vdst = this.width * 2.0F;
         let vdst = self.width * 2.0;
         // Java: float y1 = this.y;
         let mut y1 = self.y;
         // Java: if (deleteLine)
         if deleteLine
         {
            // Java: y1 -= vdst;
            y1 -= vdst;
         }
         // Java: if (isAllVert)
         if isAllVert
         {
            // Java: float y2 = deleteLine ? this.y + vdst : this.y - vdst;
            let y2 = if deleteLine { self.y + vdst } else { self.y - vdst };
            // Java: canvas.drawLine(y1, this.startX, y1, this.endX, paint);
            canvas.drawLine(y1, self.startX, y1, self.endX, paint);
            // Java: canvas.drawLine(y2, this.startX, y2, this.endX, paint);
            canvas.drawLine(y2, self.startX, y2, self.endX, paint);
         }
         // Java: else
         else
         {
            // Java: float y2 = this.y + vdst;
            let y2 = self.y + vdst;
            // Java: canvas.drawLine(this.startX, y1, this.endX, y1, paint);
            canvas.drawLine(self.startX, y1, self.endX, y1, paint);
            // Java: canvas.drawLine(this.startX, y2, this.endX, y2, paint);
            canvas.drawLine(self.startX, y2, self.endX, y2, paint);
         }
    }

    // Java: void drawWaveline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawWaveline(&self, canvas: &Canvas, paint: &Paint, isAllVert: bool) {
         // Java: float linespace = this.width * 8.0F;
         let mut linespace = self.width * 8.0;
         // Java: if (!this.heavy)
         if !self.heavy
         {
            // Java: linespace *= 1.5F;
            linespace *= 1.5;
         }
         // Java: float y1 = this.y;
         let y1 = self.y;
         // Java: float y2 = this.y + linespace;
         let y2 = self.y + linespace;
         // Java: float step = y2 - y1;
         let step = y2 - y1;
         // Java: boolean up = true;
         let mut up = true;
         // Java: PointF point = new PointF(this.startX, y1);
         let mut point = PointF::new(self.startX, y1);
         // Java: Path path = new Path();
         let mut path = Path::new();
         // Java: if (isAllVert)
         if isAllVert
         {
            // Java: path.moveTo(point.y, point.x);
            path.moveTo(point.y, point.x);
         }
         // Java: else
         else
         {
            // Java: path.moveTo(point.x, point.y);
            path.moveTo(point.x, point.y);
         }
         // Java: while(point.x < this.endX)
         while point.x < self.endX
         {
            // Java: point.x += step;
            point.x += step;
            // Java: point.y = up ? y2 : y1;
            point.y = if up { y2 } else { y1 };
            // Java: up = !up;
            up = !up;
            // Java: if (isAllVert)
            if isAllVert
            {
               // Java: path.lineTo(point.y, point.x);
               path.lineTo(point.y, point.x);
            }
            // Java: else
            else
            {
               // Java: path.lineTo(point.x, point.y);
               path.lineTo(point.x, point.y);
            }
         }
         // Java: canvas.drawPath(path, paint);
         canvas.drawPath(&path, paint);
    }

    // Java: void drawDoubleWaveline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawDoubleWaveline(&self, canvas: &Canvas, paint: &Paint, isAllVert: bool) {
         // Java: float linespace = this.width * 4.0F;
         let mut linespace = self.width * 4.0;
         // Java: if (!this.heavy)
         if !self.heavy
         {
            // Java: linespace *= 1.5F;
            linespace *= 1.5;
         }
         // Java: float y1 = this.y;
         let y1 = self.y;
         // Java: float y2 = this.y + linespace;
         let y2 = self.y + linespace;
         // Java: float step = y2 - y1;
         let step = y2 - y1;
         // Java: float vdst = this.width * 4.0F;
         let vdst = self.width * 4.0;
         // Java: boolean up = true;
         let mut up = true;
         // Java: PointF point = new PointF(this.startX, y1);
         let mut point = PointF::new(self.startX, y1);
         // Java: Path path1 = new Path();
         let mut path1 = Path::new();
         // Java: Path path2 = new Path();
         let mut path2 = Path::new();
         // Java: if (isAllVert)
         if isAllVert
         {
            // Java: path1.moveTo(point.y, point.x);
            path1.moveTo(point.y, point.x);
            // Java: path2.moveTo(point.y + vdst, point.x);
            path2.moveTo(point.y + vdst, point.x);
         }
         // Java: else
         else
         {
            // Java: path1.moveTo(point.x, point.y);
            path1.moveTo(point.x, point.y);
            // Java: path2.moveTo(point.x, point.y + vdst);
            path2.moveTo(point.x, point.y + vdst);
         }
         // Java: while(point.x < this.endX)
         while point.x < self.endX
         {
            // Java: point.x += step;
            point.x += step;
            // Java: point.y = up ? y2 : y1;
            point.y = if up { y2 } else { y1 };
            // Java: up = !up;
            up = !up;
            // Java: if (isAllVert)
            if isAllVert
            {
               // Java: path1.lineTo(point.y, point.x);
               path1.lineTo(point.y, point.x);
               // Java: path2.lineTo(point.y + vdst, point.x);
               path2.lineTo(point.y + vdst, point.x);
            }
            // Java: else
            else
            {
               // Java: path1.lineTo(point.x, point.y);
               path1.lineTo(point.x, point.y);
               // Java: path2.lineTo(point.x, point.y + vdst);
               path2.lineTo(point.x, point.y + vdst);
            }
         }
         // Java: canvas.drawPath(path1, paint);
         canvas.drawPath(&path1, paint);
         // Java: canvas.drawPath(path2, paint);
         canvas.drawPath(&path2, paint);
    }

    // Java: void drawDotline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawDotline(&self, canvas: &Canvas, paint: &Paint, isAllVert: bool) {
         // Java: float step = 2.0F * this.width;
         let step = 2.0 * self.width;
         // Java: for(float posX = this.startX; posX <= this.endX; posX += step)
         let mut posX = self.startX;
         while posX <= self.endX
         {
            // Java: if (isAllVert)
            if isAllVert
            {
               // Java: canvas.drawLine(this.y, posX, this.y, posX + this.width, paint);
               canvas.drawLine(self.y, posX, self.y, posX + self.width, paint);
            }
            // Java: else
            else
            {
               // Java: canvas.drawLine(posX, this.y, posX + this.width, this.y, paint);
               canvas.drawLine(posX, self.y, posX + self.width, self.y, paint);
            }
            posX += step;
         }
    }

    // Java: void drawDashline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawDashline(&self, canvas: &Canvas, paint: &Paint, isAllVert: bool) {
         // Java: float step = 5.0F * this.width;
         let step = 5.0 * self.width;
         // Java: float len = 3.0F * this.width;
         let len = 3.0 * self.width;
         // Java: for(float posX = this.startX; posX < this.endX; posX += step)
         let mut posX = self.startX;
         while posX < self.endX
         {
            // Java: if (isAllVert)
            if isAllVert
            {
               // Java: canvas.drawLine(this.y, posX, this.y, posX + len, paint);
               canvas.drawLine(self.y, posX, self.y, posX + len, paint);
            }
            // Java: else
            else
            {
               // Java: canvas.drawLine(posX, this.y, posX + len, this.y, paint);
               canvas.drawLine(posX, self.y, posX + len, self.y, paint);
            }
            posX += step;
         }
    }

    // Java: void drawLongdashline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawLongdashline(&self, canvas: &Canvas, paint: &Paint, isAllVert: bool) {
         // Java: float step = 8.0F * this.width;
         let step = 8.0 * self.width;
         // Java: float len = 5.0F * this.width;
         let len = 5.0 * self.width;
         // Java: for(float posX = this.startX; posX < this.endX; posX += step)
         let mut posX = self.startX;
         while posX < self.endX
         {
            // Java: if (isAllVert)
            if isAllVert
            {
               // Java: canvas.drawLine(this.y, posX, this.y, posX + len, paint);
               canvas.drawLine(self.y, posX, self.y, posX + len, paint);
            }
            // Java: else
            else
            {
               // Java: canvas.drawLine(posX, this.y, posX + len, this.y, paint);
               canvas.drawLine(posX, self.y, posX + len, self.y, paint);
            }
            posX += step;
         }
    }

    // Java: void drawDashdotline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawDashdotline(&self, canvas: &Canvas, paint: &Paint, isAllVert: bool) {
         // Java: float step = 8.0F * this.width;
         let step = 8.0 * self.width;
         // Java: float len = 3.0F * this.width;
         let len = 3.0 * self.width;
         // Java: float dotdst = 5.0F * this.width;
         let dotdst = 5.0 * self.width;
         // Java: for(float posX = this.startX; posX < this.endX; posX += step)
         let mut posX = self.startX;
         while posX < self.endX
         {
            // Java: if (isAllVert)
            if isAllVert
            {
               // Java: canvas.drawLine(this.y, posX, this.y, posX + len, paint);
               canvas.drawLine(self.y, posX, self.y, posX + len, paint);
               // Java: canvas.drawLine(this.y, posX + dotdst, this.y, posX + dotdst + this.width, paint);
               canvas.drawLine(self.y, posX + dotdst, self.y, posX + dotdst + self.width, paint);
            }
            // Java: else
            else
            {
               // Java: canvas.drawLine(posX, this.y, posX + len, this.y, paint);
               canvas.drawLine(posX, self.y, posX + len, self.y, paint);
               // Java: canvas.drawLine(posX + dotdst, this.y, posX + dotdst + this.width, this.y, paint);
               canvas.drawLine(posX + dotdst, self.y, posX + dotdst + self.width, self.y, paint);
            }
            posX += step;
         }
    }

    // Java: void drawDashdotdotline(Canvas canvas, Paint paint, boolean isAllVert)
    pub fn drawDashdotdotline(&self, canvas: &Canvas, paint: &Paint, isAllVert: bool) {
         // Java: float step = 11.0F * this.width;
         let step = 11.0 * self.width;
         // Java: float len = 3.0F * this.width;
         let len = 3.0 * self.width;
         // Java: float dotdst1 = 5.0F * this.width;
         let dotdst1 = 5.0 * self.width;
         // Java: float dotdst2 = 8.0F * this.width;
         let dotdst2 = 8.0 * self.width;
         // Java: for(float posX = this.startX; posX < this.endX; posX += step)
         let mut posX = self.startX;
         while posX < self.endX
         {
            // Java: if (isAllVert)
            if isAllVert
            {
               // Java: canvas.drawLine(this.y, posX, this.y, posX + len, paint);
               canvas.drawLine(self.y, posX, self.y, posX + len, paint);
               // Java: canvas.drawLine(this.y, posX + dotdst1, this.y, posX + dotdst1 + this.width, paint);
               canvas.drawLine(self.y, posX + dotdst1, self.y, posX + dotdst1 + self.width, paint);
               // Java: canvas.drawLine(this.y, posX + dotdst2, this.y, posX + dotdst2 + this.width, paint);
               canvas.drawLine(self.y, posX + dotdst2, self.y, posX + dotdst2 + self.width, paint);
            }
            // Java: else
            else
            {
               // Java: canvas.drawLine(posX, this.y, posX + len, this.y, paint);
               canvas.drawLine(posX, self.y, posX + len, self.y, paint);
               // Java: canvas.drawLine(posX + dotdst1, this.y, posX + dotdst1 + this.width, this.y, paint);
               canvas.drawLine(posX + dotdst1, self.y, posX + dotdst1 + self.width, self.y, paint);
               // Java: canvas.drawLine(posX + dotdst2, this.y, posX + dotdst2 + this.width, this.y, paint);
               canvas.drawLine(posX + dotdst2, self.y, posX + dotdst2 + self.width, self.y, paint);
            }
            posX += step;
         }
    }

}
