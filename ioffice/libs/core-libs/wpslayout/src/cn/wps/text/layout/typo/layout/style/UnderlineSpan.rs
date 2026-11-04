#![allow(dead_code, unused_variables, non_snake_case)]

// Java: import android.text.TextPaint
struct TextPaint;

// Java: public class UnderlineSpan extends CharacterStyle implements UpdateAppearance
pub struct UnderlineSpan {
    // Java field: int underlineColor
    pub(crate) underlineColor: i32,
    // Java field: float underlineThickness
    pub(crate) underlineThickness: f32,
    // Java field: float underlinePosition
    pub(crate) underlinePosition: f32,
}

// Java: public class UnderlineSpan extends CharacterStyle implements UpdateAppearance
impl UnderlineSpan {
    // Java: UnderlineSpan()
    pub fn new(&self) {
      // Java: super();
    }

    // Java: void updateDrawState(TextPaint ds)
    pub fn updateDrawState(&self, _ds: i32) {
    }

}

trait UpdateAppearance {}
impl UpdateAppearance for UnderlineSpan {
}
