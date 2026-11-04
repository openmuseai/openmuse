// Java: import android.text.TextPaint
use crate::cn::wps::text::layout::typo::layout::style::CharacterStyle::TextPaint;
// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle
use crate::cn::wps::text::layout::typo::layout::style::CharacterStyle::CharacterStyleTrait;

// Java: public class SpacingSpan extends CharacterStyle
pub struct SpacingSpan {
    // Java field: float spc
    pub(crate) spc: f32,
}

// Java: public class SpacingSpan extends CharacterStyle
impl SpacingSpan {
    // Java: SpacingSpan(float kern)
    pub fn new(kern: f32) -> Self {
      // Java: super();
      // Java: this.spc = kern;
      SpacingSpan { spc: kern }
    }

    // Java: void updateDrawState(TextPaint textPaint)
    pub fn updateDrawState(&self, _textPaint: &TextPaint) {
    }

    // Java: float getSpacing()
    pub fn getSpacing(&self) -> f32 {
      // Java: return this.spc;
      self.spc
    }

}
