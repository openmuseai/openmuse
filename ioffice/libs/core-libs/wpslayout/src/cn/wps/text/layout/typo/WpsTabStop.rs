// Java: import android.text.TextPaint
use crate::cn::wps::text::layout::typo::layout::style::CharacterStyle::TextPaint;
// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle
use crate::cn::wps::text::layout::typo::layout::style::CharacterStyle::CharacterStyleTrait;

// Java: public class WpsTabStop extends CharacterStyle
pub struct WpsTabStop {
    // Java field: int tabStopSize
    pub(crate) tabStopSize: i32,
}

// Java: public class WpsTabStop extends CharacterStyle
impl WpsTabStop {
    // Java: WpsTabStop(int width)
    pub fn new(width: i32) -> Self {
      // Java: super();
      // Java: this.tabStopSize = width;
      WpsTabStop { tabStopSize: width }
    }

    // Java: void setTabWidth(int v)
    pub fn setTabWidth(&mut self, v: i32) {
      // Java: this.tabStopSize = v;
      self.tabStopSize = v;
    }

    // Java: int getTabWidth()
    pub fn getTabWidth(&self) -> i32 {
      // Java: return this.tabStopSize;
      self.tabStopSize
    }

}

impl CharacterStyleTrait for WpsTabStop {
    // Java: void updateDrawState(TextPaint tp)
    fn updateDrawState(&self, _tp: &mut TextPaint) {
    }

    fn getUnderlying(&self) -> &dyn CharacterStyleTrait {
        self
    }
}
