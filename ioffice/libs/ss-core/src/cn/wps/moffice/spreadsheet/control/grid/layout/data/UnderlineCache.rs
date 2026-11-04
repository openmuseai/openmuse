use super::super::super::render::graphics::{Canvas, FontMetricsInt, Paint};
use super::super::typo::PaintFontHelper::PaintFontHelper;
use super::{CellTextData::CellTextData, TextData::TextData};
use crate::cn::wps::text::layout::{layouter::WpsLayout::WpsLayout, typo::layout::Layout::Layout};
#[derive(Default)]
pub struct UnderlineCache {
    mUnderlines: Vec<Underline>,
}
struct Underline {
    startX: f32,
    endX: f32,
    y: f32,
    color: i32,
    width: f32,
    single: bool,
    lineNum: i32,
}
impl UnderlineCache {
    // java public void clear() {
    // java this.mUnderlines.clear();
    // java }
    pub fn clear(&mut self) {
        self.mUnderlines.clear();
    }
    // java public void calcUnderline(Paint.FontMetricsInt metrics, float width, CellTextData data) {
    // java this.mUnderlines.clear();
    // java if (data.mFirstSpan.hasUnderline()) {
    // java Underline underline = new Underline();
    // java underline.startX = 0.0F;
    // java underline.endX = width;
    // java underline.y = data.mFirstSpan.isAcounting() ? (float)metrics.bottom : 0.0F + (float)metrics.descent * 0.6F;
    // java underline.color = data.mFirstSpan.mFontColor;
    // java underline.width = 1.0F + data.mFirstSpan.mFontSize * 0.01F;
    // java underline.single = data.mFirstSpan.isSUnderline();
    // java this.mUnderlines.add(underline);
    // java }
    // java }
    pub fn calcUnderline(&mut self, fm: &FontMetricsInt, width: f32, data: &CellTextData) {
        self.clear();
        let s = &data.mFirstSpan;
        if s.hasUnderline() {
            self.mUnderlines.push(Underline {
                startX: 0.0,
                endX: width,
                y: if s.isAcounting() {
                    fm.bottom as f32
                } else {
                    fm.descent as f32 * 0.6
                },
                color: s.mFontColor,
                width: 1.0 + s.mFontSize * 0.01,
                single: s.isSUnderline(),
                lineNum: 0,
            });
        }
    }
    // java public void calcUnderline(Layout typoCache, TextData data) {
    // java this.mUnderlines.clear();
    // java int lineCount = typoCache.getLineCount();
    // java int[] lineEnds = new int[lineCount];
    // java
    // java for(int i = 0; i < lineCount; ++i) {
    // java lineEnds[i] = typoCache.getLineEnd(i);
    // java }
    // java
    // java ArrayList<LinePos> linesPos = new ArrayList();
    // java SpanInfo span = data.mFirstSpan;
    // java int spanIdx = 0;
    // java int lineIdx = 0;
    // java boolean showAccounting = true;
    // java
    // java while(true) {
    // java if (span.hasUnderline()) {
    // java boolean isAccouting = span.isAcounting();
    // java if (!isAccouting) {
    // java showAccounting = false;
    // java }
    // java
    // java LinePos pos = new LinePos();
    // java pos.start = span.mStartPos;
    // java
    // java for(pos.end = span.mEndPos; lineIdx < lineCount; ++lineIdx) {
    // java pos.line = lineIdx;
    // java pos.color = span.mFontColor;
    // java pos.fontSize = span.mFontSize;
    // java pos.single = span.isSUnderline();
    // java pos.accounting = isAccouting;
    // java int lineEnd = lineEnds[lineIdx];
    // java if (pos.start < lineEnd && pos.end <= lineEnd) {
    // java linesPos.add(pos);
    // java break;
    // java }
    // java
    // java if (pos.start < lineEnd && pos.end > lineEnd) {
    // java LinePos newPos = new LinePos();
    // java newPos.start = lineEnd;
    // java newPos.end = pos.end;
    // java pos.end = lineEnd;
    // java linesPos.add(pos);
    // java pos = newPos;
    // java }
    // java }
    // java }
    // java
    // java if (spanIdx >= data.mOtherSpans.size()) {
    // java boolean isLeftToRight = !data.mPara.mIsRightToLeft;
    // java
    // java for(int i = 0; i < linesPos.size(); ++i) {
    // java LinePos p = (LinePos)linesPos.get(i);
    // java if (!p.isEmpty()) {
    // java int linestart = typoCache.getLineStart(p.line);
    // java Underline underline = new Underline();
    // java underline.color = p.color;
    // java underline.width = 1.0F + p.fontSize * 0.01F;
    // java underline.single = p.single;
    // java underline.y = typoCache.getLineBottom(p.line) - underline.width * 3.0F;
    // java underline.lineNum = p.line;
    // java if (p.accounting) {
    // java if (showAccounting) {
    // java underline.startX = 2.0F;
    // java underline.endX = (float)(typoCache.getWidth() - 2);
    // java }
    // java } else {
    // java underline.startX = isLeftToRight ? typoCache.getLineLeft(p.line) : typoCache.getLineRight(p.line);
    // java float width = PaintFontHelper.measureCharWidth(data, linestart, p.start);
    // java underline.startX += isLeftToRight ? width : -width;
    // java width = PaintFontHelper.measureCharWidth(data, p.start, p.end);
    // java underline.endX = underline.startX + (isLeftToRight ? width : -width);
    // java }
    // java
    // java this.mUnderlines.add(underline);
    // java }
    // java }
    // java
    // java return;
    // java }
    // java
    // java span = (SpanInfo)data.mOtherSpans.get(spanIdx);
    // java ++spanIdx;
    // java }
    // java }
    pub fn calcUnderline__layout(&mut self, l: &WpsLayout, data: &TextData) {
        self.clear();
        let count = l.getLineCount();
        let mut lineIdx = 0;
        let mut showAccounting = true;
        let mut spans = vec![&data.mFirstSpan];
        spans.extend(data.mOtherSpans.iter());
        let mut parts = vec![];
        for s in spans {
            if s.hasUnderline() {
                let accounting = s.isAcounting();
                if !accounting {
                    showAccounting = false;
                }
                let mut start = s.mStartPos;
                let end = s.mEndPos;
                while lineIdx < count {
                    let lineEnd = l.getLineStart(lineIdx + 1);
                    if start < lineEnd && end <= lineEnd {
                        parts.push((
                            start,
                            end,
                            lineIdx,
                            s.mFontColor,
                            s.mFontSize,
                            s.isSUnderline(),
                            accounting,
                        ));
                        break;
                    }
                    if start < lineEnd && end > lineEnd {
                        parts.push((
                            start,
                            lineEnd,
                            lineIdx,
                            s.mFontColor,
                            s.mFontSize,
                            s.isSUnderline(),
                            accounting,
                        ));
                        start = lineEnd;
                    }
                    lineIdx += 1;
                }
            }
        }
        let ltr = !data.mPara.mIsRightToLeft;
        for (start, end, line, color, fontSize, single, accounting) in parts {
            if start == end {
                continue;
            }
            let width = 1.0 + fontSize * 0.01;
            let (mut x, mut x2) = (0.0, 0.0);
            if accounting {
                if showAccounting {
                    x = 2.0;
                    x2 = (l.getWidth() - 2) as f32;
                }
            } else {
                x = if ltr {
                    l.getLineLeft(line)
                } else {
                    l.getLineRight(line)
                };
                let part = PaintFontHelper::measureCharWidth(
                    data,
                    l.getLineStart(line) as usize,
                    start as usize,
                );
                x += if ltr { part } else { -part };
                let part = PaintFontHelper::measureCharWidth(data, start as usize, end as usize);
                x2 = x + if ltr { part } else { -part };
            }
            self.mUnderlines.push(Underline {
                startX: x,
                endX: x2,
                y: l.getLineTop(line + 1) - width * 3.0,
                color,
                width,
                single,
                lineNum: line,
            });
        }
    }
    // java public void draw(Canvas canvas, Layout layout, short angle) {
    // java if (!this.mUnderlines.isEmpty()) {
    // java if (this.mPaint == null) {
    // java this.mPaint = new Paint();
    // java }
    // java
    // java this.mPaint.reset();
    // java this.mPaint.setStyle(Style.STROKE);
    // java this.mPaint.setTextAlign(Align.RIGHT);
    // java canvas.save();
    // java int curLine = 0;
    // java
    // java for(Underline underline : this.mUnderlines) {
    // java this.mPaint.setColor(underline.color);
    // java this.mPaint.setStrokeWidth(underline.width);
    // java float linespace = underline.width * 2.0F;
    // java if (layout != null && angle != 0 && angle != 90 && angle != -90 && underline.lineNum != 0 && underline.lineNum != curLine) {
    // java if (angle > 0) {
    // java canvas.translate((Float)layout.getLineHeights().get(underline.lineNum) / layout.tan, 0.0F);
    // java } else {
    // java canvas.translate(-(Float)layout.getLineHeights().get(underline.lineNum) / layout.tan, 0.0F);
    // java }
    // java
    // java curLine = underline.lineNum;
    // java }
    // java
    // java if (underline.single) {
    // java canvas.drawLine(underline.startX, underline.y, underline.endX, underline.y, this.mPaint);
    // java } else {
    // java canvas.drawLine(underline.startX, underline.y + linespace, underline.endX, underline.y + linespace, this.mPaint);
    // java canvas.drawLine(underline.startX, underline.y, underline.endX, underline.y, this.mPaint);
    // java }
    // java }
    // java
    // java canvas.restore();
    // java this.clear();
    // java }
    // java }
    pub fn draw__layout(&mut self, c: &mut Canvas, l: Option<&WpsLayout>, angle: i16) {
        if self.mUnderlines.is_empty() {
            return;
        }
        let mut p = Paint {
            style: 1,
            textAlign: 2,
            ..Paint::default()
        };
        c.save();
        let mut line = 0;
        for u in &self.mUnderlines {
            p.color = u.color;
            p.strokeWidth = u.width;
            if let Some(l) = l {
                if angle != 0 && angle != 90 && angle != -90 && u.lineNum != 0 && u.lineNum != line
                {
                    let dx = l.mLineHeights[u.lineNum as usize] / l.tan;
                    c.translate(if angle > 0 { dx } else { -dx }, 0.0);
                    line = u.lineNum;
                }
            }
            if !u.single {
                c.drawLine(
                    u.startX,
                    u.y + u.width * 2.0,
                    u.endX,
                    u.y + u.width * 2.0,
                    &p,
                );
            }
            c.drawLine(u.startX, u.y, u.endX, u.y, &p);
        }
        c.restore();
        self.clear();
    }
    // java public void draw(Canvas canvas) {
    // java this.draw(canvas, (Layout)null, (short)0);
    // java }
    pub fn draw(&mut self, c: &mut Canvas) {
        self.draw__layout(c, None, 0);
    }
}
