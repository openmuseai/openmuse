use crate::cn::wps::moffice::spreadsheet::control::grid::render::graphics::Canvas;
// java public abstract class Layout
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    ALIGN_NORMAL,
    ALIGN_OPPOSITE,
    ALIGN_CENTER,
    ALIGN_JUSTIFY,
    ALIGN_DISTRIBUTED,
    ALIGN_LEFT,
    ALIGN_RIGHT,
}
pub trait Layout {
    fn getWidth(&self) -> i32;
    fn getLineCount(&self) -> i32;
    fn getLineTop(&self, line: i32) -> f32;
    fn getLineDescent(&self, line: i32) -> f32;
    fn getLineStart(&self, line: i32) -> i32;
    fn getLineWidth(&self, line: i32) -> f32;
    fn getLineMax(&self, line: i32) -> f32;
    fn getAlignment(&self) -> Alignment;
    fn getLineForVertical(&self, y: i32) -> i32;
    fn draw(&self, canvas: &mut Canvas);
    // java public float getHeight() {
    // java return this.getLineTop(this.getLineCount());
    // java }
    fn getHeight(&self) -> f32 {
        self.getLineTop(self.getLineCount())
    }
    // java public final float getLineBaseline(int line) {
    // java return this.getLineTop(line + 1) - this.getLineDescent(line);
    // java }
    fn getLineBaseline(&self, line: i32) -> f32 {
        self.getLineTop(line + 1) - self.getLineDescent(line)
    }
    // java protected float getAlignmentOffset(int line) {
    // java float alignmentOffset = 0.0F;
    // java switch (this.getAlignment()) {
    // java case ALIGN_DISTRIBUTED:
    // java case ALIGN_CENTER:
    // java int left = this.getParagraphLeft(line);
    // java int right = this.getParagraphRight(line);
    // java int max = (int)this.getLineMax(line) & -2;
    // java alignmentOffset += (float)((right - left - max) / 2);
    // java break;
    // java case ALIGN_OPPOSITE:
    // java alignmentOffset += (float)this.getWidth() - this.getLineMax(line);
    // java }
    // java
    // java return alignmentOffset;
    // java }
    fn getAlignmentOffset(&self, line: i32) -> f32 {
        match self.getAlignment() {
            Alignment::ALIGN_DISTRIBUTED | Alignment::ALIGN_CENTER => {
                ((self.getWidth() - ((self.getLineMax(line) as i32) & -2)) / 2) as f32
            }
            Alignment::ALIGN_OPPOSITE => self.getWidth() as f32 - self.getLineMax(line),
            _ => 0.0,
        }
    }
    // java public float getLineLeft(int line) {
    // java int dir = this.getParagraphDirection(line);
    // java Alignment align = this.getParagraphAlignment(line);
    // java if (align != Layout.Alignment.ALIGN_LEFT && align != Layout.Alignment.ALIGN_JUSTIFY) {
    // java if (align == Layout.Alignment.ALIGN_NORMAL) {
    // java return dir == -1 ? (float)this.getParagraphRight(line) - this.getLineMax(line) : 0.0F;
    // java } else if (align == Layout.Alignment.ALIGN_RIGHT) {
    // java return (float)this.mWidth - this.getLineMax(line);
    // java } else if (align == Layout.Alignment.ALIGN_OPPOSITE) {
    // java return dir == -1 ? 0.0F : (float)this.mWidth - this.getLineMax(line);
    // java } else {
    // java int left = this.getParagraphLeft(line);
    // java int right = this.getParagraphRight(line);
    // java int max = (int)this.getLineMax(line) & -2;
    // java return (float)(left + (right - left - max) / 2);
    // java }
    // java } else {
    // java return 0.0F;
    // java }
    // java }
    fn getLineLeft(&self, line: i32) -> f32 {
        match self.getAlignment() {
            Alignment::ALIGN_LEFT | Alignment::ALIGN_JUSTIFY | Alignment::ALIGN_NORMAL => 0.0,
            Alignment::ALIGN_RIGHT | Alignment::ALIGN_OPPOSITE => {
                self.getWidth() as f32 - self.getLineMax(line)
            }
            _ => ((self.getWidth() - ((self.getLineMax(line) as i32) & -2)) / 2) as f32,
        }
    }
    // java public float getLineRight(int line) {
    // java int dir = this.getParagraphDirection(line);
    // java Alignment align = this.getParagraphAlignment(line);
    // java if (align == Layout.Alignment.ALIGN_LEFT) {
    // java return (float)this.getParagraphLeft(line) + this.getLineMax(line);
    // java } else if (align == Layout.Alignment.ALIGN_NORMAL) {
    // java return dir == -1 ? (float)this.mWidth : (float)this.getParagraphLeft(line) + this.getLineMax(line);
    // java } else if (align == Layout.Alignment.ALIGN_RIGHT) {
    // java return (float)this.mWidth;
    // java } else if (align == Layout.Alignment.ALIGN_OPPOSITE) {
    // java return dir == -1 ? this.getLineMax(line) : (float)this.mWidth;
    // java } else {
    // java int left = this.getParagraphLeft(line);
    // java int right = this.getParagraphRight(line);
    // java int max = (int)this.getLineMax(line) & -2;
    // java return (float)(right - (right - left - max) / 2);
    // java }
    // java }
    fn getLineRight(&self, line: i32) -> f32 {
        match self.getAlignment() {
            Alignment::ALIGN_LEFT | Alignment::ALIGN_NORMAL => self.getLineMax(line),
            Alignment::ALIGN_RIGHT | Alignment::ALIGN_OPPOSITE => self.getWidth() as f32,
            _ => {
                (self.getWidth() - (self.getWidth() - ((self.getLineMax(line) as i32) & -2)) / 2)
                    as f32
            }
        }
    }
}
