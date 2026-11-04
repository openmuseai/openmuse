// Java: import android.text.TextPaint
use super::CharacterStyle::{TextPaint, CharacterStyleTrait};
use super::UpdateAppearance::UpdateAppearance;

// Java: public class ForegroundColorSpan extends CharacterStyle implements UpdateAppearance
pub struct ForegroundColorSpan {
    // Java field: int mColor
    pub(crate) mColor: i32,
}

// Java: public class ForegroundColorSpan extends CharacterStyle implements UpdateAppearance
impl ForegroundColorSpan {
    // Java: ForegroundColorSpan(int color)
    pub fn new(color: i32) -> Self {
      // Java: super();
      // Java: this.mColor = color;
      ForegroundColorSpan { mColor: color }
    }

    // Java: int getForegroundColor()
    pub fn getForegroundColor(&self) -> i32 {
      // Java: return this.mColor;
      self.mColor
    }

}

impl CharacterStyleTrait for ForegroundColorSpan {
    // Java: void updateDrawState(TextPaint textPaint)
    fn updateDrawState(&self, textPaint: &mut TextPaint) {
      // Java: textPaint.setColor(this.mColor);
    }

    fn getUnderlying(&self) -> &dyn CharacterStyleTrait {
        self
    }
}

impl UpdateAppearance for ForegroundColorSpan {
}
