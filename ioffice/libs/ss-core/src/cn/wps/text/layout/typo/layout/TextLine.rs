use super::super::super::layouter::WpsLayout::WpsLayout;
use crate::cn::wps::moffice::spreadsheet::control::grid::render::graphics::{Canvas, Paint};
// java class TextLine
pub struct TextLine<'a> {
    layout: &'a WpsLayout,
    line: i32,
}
impl<'a> TextLine<'a> {
    // rust: Borrowed projection of the original TextLine.set + Layout.setTextLineInfo fields.
    pub fn new(layout: &'a WpsLayout, line: i32) -> Self {
        Self { layout, line }
    }
    // java public static boolean isLineEndSpace(char ch) {
    // java return ch == ' ' || ch == '\t' || ch == 5760 || 8192 <= ch && ch <= 8202 && ch != 8199 || ch == 8287 || ch == 12288;
    // java }
    pub fn isLineEndSpace(c: u16) -> bool {
        c == 32
            || c == 9
            || c == 5760
            || (8192..=8202).contains(&c) && c != 8199
            || c == 8287
            || c == 12288
    }
    // java public float getCharSize(int start, int end) {
    // java if (this.charWidths == null) {
    // java return 0.0F;
    // java } else {
    // java float width = 0.0F;
    // java
    // java for(int i = start; i < end; ++i) {
    // java width += this.charWidths[i];
    // java }
    // java
    // java return width;
    // java }
    // java }
    pub fn getCharSize(&self, start: usize, end: usize) -> f32 {
        let mut w = 0.0f32;
        for i in start..end {
            w += self.layout.charWidths[i];
        }
        w
    }
    // java private static void drawStroke(TextPaint wp, Canvas c, int color, float position, float thickness, float xleft, float xright, float baseline) {
    // java float strokeTop = baseline + (float)wp.baselineShift + position;
    // java int previousColor = wp.getColor();
    // java Paint.Style previousStyle = wp.getStyle();
    // java boolean previousAntiAlias = wp.isAntiAlias();
    // java wp.setStyle(Style.FILL);
    // java wp.setAntiAlias(true);
    // java wp.setColor(color);
    // java c.drawRect(xleft, strokeTop, xright, strokeTop + thickness, wp);
    // java wp.setStyle(previousStyle);
    // java wp.setColor(previousColor);
    // java wp.setAntiAlias(previousAntiAlias);
    // java }
    fn drawStroke(
        p: &Paint,
        c: &mut Canvas,
        pos: f32,
        thick: f32,
        left: f32,
        right: f32,
        baseline: f32,
    ) {
        let top = baseline + p.baselineShift as f32 + pos;
        let mut work = p.clone();
        work.style = 0;
        work.flags |= 1;
        c.drawRect__float(left, top, right, top + thick, &work);
    }
    // java private void drawTextRun(Canvas c, Paint wp, int start, int end, boolean runIsRtl, float x, float y) {
    // java if (this.charWidths != null && this.layout != null) {
    // java start += this.mStart;
    // java end += this.mStart;
    // java CharSequence text = this.mText;
    // java float offsetX = x;
    // java if (runIsRtl && this.layout.getLineForOffset(start) == this.layout.getLineCount() - 1) {
    // java offsetX = x - this.charWidths[this.layout.getLineStart(this.mLineCount)];
    // java }
    // java
    // java float top = this.layout.getLineTop(this.mLineCount);
    // java float bottom = this.layout.getLineTop(this.mLineCount + 1);
    // java if (this.layout.isAllVerticalRTLText()) {
    // java top = -top;
    // java bottom = -bottom;
    // java }
    // java
    // java int verX = 0;
    // java float baseLine = y;
    // java int i = start;
    // java
    // java while(i < end) {
    // java if (this.layout.isAllVerticalText() && this.originCharWidths != null) {
    // java if (this.spacBefCharWidths != null) {
    // java baseLine = (float)((int)((double)offsetX + (double)this.spacBefCharWidths[i] * 0.8));
    // java } else {
    // java baseLine = (float)((int)((double)offsetX + (double)this.charWidths[i] * 0.8));
    // java }
    // java
    // java verX = (int)((top + bottom - this.originCharWidths[i]) / 2.0F - (float)((TextPaint)wp).baselineShift);
    // java }
    // java
    // java float drawX = this.layout.isAllVerticalText() ? (float)verX : offsetX;
    // java if (!runIsRtl) {
    // java int length;
    // java for(length = 1; i + length < this.charWidths.length && this.charWidths[i + length] == 0.0F && text.charAt(i + length) != '\r'; ++length) {
    // java }
    // java
    // java c.drawText(text, i, i + length, drawX, baseLine, wp);
    // java offsetX += this.charWidths[i];
    // java i += length;
    // java } else {
    // java int length = 1;
    // java c.drawText(text, i, i + length, drawX, baseLine, wp);
    // java offsetX += this.charWidths[i];
    // java i += length;
    // java }
    // java }
    // java
    // java } else {
    // java if (this.mCharsValid) {
    // java int count = end - start;
    // java c.drawText(String.copyValueOf(this.mChars, start, count), x, y, wp);
    // java } else {
    // java int delta = this.mStart;
    // java c.drawText(this.mText, delta + start, delta + end, (float)((int)x), y, wp);
    // java }
    // java
    // java }
    // java }
    fn drawTextRun(&self, c: &mut Canvas, p: &Paint, start: usize, end: usize, mut x: f32, y: f32) {
        let mut i = start;
        while i < end {
            let mut len = 1;
            while i + len < self.layout.charWidths.len()
                && self.layout.charWidths[i + len] == 0.0
                && self.layout.text[i + len] != 13
            {
                len += 1;
            }
            c.drawText(&self.layout.text[i..i + len], x, y, p);
            x += self.layout.charWidths[i];
            i += len;
        }
    }
    // java void draw(Canvas c, float x, float top, float y, float bottom) {
    // java if (this.mDirections == Layout.DIRS_ALL_LEFT_TO_RIGHT) {
    // java this.drawRun(c, 0, this.mLen, false, x, top, y, bottom, false);
    // java } else if (this.mDirections == Layout.DIRS_ALL_RIGHT_TO_LEFT) {
    // java this.drawRun(c, 0, this.mLen, true, x, top, y, bottom, false);
    // java }
    // java }
    // rust: Original drawRun includes tab UTF-16 units; only measure()/measureAllOffsets() split tab runs.
    // rust: ET read closure has only Typeface / Style / AbsoluteSize / Script / foreground and strike spans; no Word replacement or leading-margin spans.
    pub fn draw(&self, c: &mut Canvas, mut x: f32, y: f32) {
        let mut start = self.layout.starts[self.line as usize] as usize;
        let end = self.layout.getLineVisibleEnd(self.line);
        while start < end {
            let (_, styleEnd, p) = self
                .layout
                .paints
                .iter()
                .find(|(a, b, _)| start >= *a && start < *b)
                .expect("TextLine style span");
            let stop = end.min(*styleEnd);
            for (a, b, thick, pos) in &self.layout.strikeDecorations {
                let a = (*a).max(start);
                let b = (*b).min(stop);
                if a < b {
                    let left = x + p.measureText(&self.layout.text[start..a]);
                    let right = x + p.measureText(&self.layout.text[start..b]);
                    Self::drawStroke(p, c, *pos, (*thick as f32).max(1.0), left, right, y);
                }
            }
            self.drawTextRun(c, p, start, stop, x, y + p.baselineShift as f32);
            x += self.getCharSize(start, stop);
            start = stop;
        }
    }
}
