use super::CharacterStyle::TextPaint;

// Java: public class AbsoluteSizeSpan extends MetricAffectingSpan
pub struct AbsoluteSizeSpan {
    // Java field: int mSize
    pub(crate) mSize: i32,
}

// Java: public class AbsoluteSizeSpan extends MetricAffectingSpan
impl AbsoluteSizeSpan {
    // Java: AbsoluteSizeSpan(int size)
    pub fn new(size: i32) -> Self {
      // Java: super();
      // Java: this.mSize = size;
      AbsoluteSizeSpan { mSize: size }
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this.mSize;
      self.mSize
    }

    // Java: void updateDrawState(TextPaint ds)
    pub fn updateDrawState(&self, ds: &mut TextPaint) {
      // Java: ds.setTextSize((float)this.mSize);
    }

    // Java: void updateMeasureState(TextPaint ds)
    pub fn updateMeasureState(&self, ds: &mut TextPaint) {
      // Java: ds.setTextSize((float)this.mSize);
    }

}
