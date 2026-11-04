#![allow(dead_code, unused_variables, non_snake_case)]

// Java: import android.text.TextPaint
struct TextPaint;

// Java: public class StrikethroughSpan extends CharacterStyle implements UpdateAppearance
pub struct StrikethroughSpan {
    // Java field: int strikeThruThickness
    pub(crate) strikeThruThickness: i32,
    // Java field: float strikeThruPosition
    pub(crate) strikeThruPosition: f32,
}

// Java: public class StrikethroughSpan extends CharacterStyle implements UpdateAppearance
impl StrikethroughSpan {
    // Java: StrikethroughSpan()
    pub fn new(&self) {
      // Java: super();
    }

    // Java: void updateDrawState(TextPaint ds)
    pub fn updateDrawState(&self, _ds: i32) {
    }

}

trait UpdateAppearance {}
impl UpdateAppearance for StrikethroughSpan {
}
