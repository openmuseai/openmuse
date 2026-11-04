#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import android.text.TextPaint
use crate::cn::wps::text::layout::typo::layout::style::UpdateAppearance::UpdateAppearance;

// Java: public class BackgroundColorSpan extends CharacterStyle implements UpdateAppearance
pub struct BackgroundColorSpan {
    // Java field: int mColor
    pub(crate) mColor: i32,
}

// Java: public class BackgroundColorSpan extends CharacterStyle implements UpdateAppearance
impl BackgroundColorSpan {
    // Java: BackgroundColorSpan(int color)
    pub fn BackgroundColorSpan(color: i32) -> Self {
      // Java: super();
      // Java: this.mColor = color;
      BackgroundColorSpan { mColor: color }
    }

    // Java: int getBackgroundColor()
    pub fn getBackgroundColor(&self) -> i32 {
      // Java: return this.mColor;
      self.mColor
    }

    // Java: void updateDrawState(TextPaint tp)
    pub fn updateDrawState(&self, tp: i32) {
      // Java: tp.bgColor = this.mColor;
    }

}

impl UpdateAppearance for BackgroundColorSpan {
}
