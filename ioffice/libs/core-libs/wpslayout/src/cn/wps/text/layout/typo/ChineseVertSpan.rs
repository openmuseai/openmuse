// Java: import android.graphics.Typeface
use crate::cn::wps::text::layout::typo::layout::style::Typeface::Typeface;
// Java: import android.text.TextPaint
use crate::cn::wps::text::layout::typo::layout::style::CharacterStyle::TextPaint;
// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle

// Java: public class ChineseVertSpan extends CharacterStyle
pub struct ChineseVertSpan {
    // Java field: int start
    pub(crate) start: i32,
    // Java field: int end
    pub(crate) end: i32,
    // Java field: float fontSize
    pub(crate) fontSize: f32,
    // Java field: int fontColor
    pub(crate) fontColor: i32,
    // Java field: Typeface typeface
    pub(crate) typeface: Typeface,
    // Java field: int fontStyle
    pub(crate) fontStyle: i32,
    // Java field: boolean isSuperScript
    pub(crate) isSuperScript: bool,
    // Java field: boolean isSubScript
    pub(crate) isSubScript: bool,
}

// Java: public class ChineseVertSpan extends CharacterStyle
impl ChineseVertSpan {
    // Java: ChineseVertSpan(int start, int end, float fontSize, int fontColor, Typeface typeface, int fontStyle, boolean isSuperScript, boolean isSubScript)
    pub fn new(start: i32, end: i32, fontSize: f32, fontColor: i32, typeface: Typeface, fontStyle: i32, isSuperScript: bool, isSubScript: bool) -> Self {
        // Java: super();
        // Java: this.start = start;
        // Java: this.end = end;
        // Java: this.fontSize = fontSize;
        // Java: this.fontColor = fontColor;
        // Java: this.typeface = typeface;
        // Java: this.fontStyle = fontStyle;
        // Java: this.isSuperScript = isSuperScript;
        // Java: this.isSubScript = isSubScript;
        ChineseVertSpan { start, end, fontSize, fontColor, typeface, fontStyle, isSuperScript, isSubScript }
    }

    // Java: void updateDrawState(TextPaint paint)
    pub fn updateDrawState(&self, _paint: &mut TextPaint) {
    }

    // Java: int getStart()
    pub fn getStart(&self) -> i32 {
        // Java: return this.start;
        self.start
    }

    // Java: int getEnd()
    pub fn getEnd(&self) -> i32 {
        // Java: return this.end;
        self.end
    }

    // Java: float getFontSize()
    pub fn getFontSize(&self) -> f32 {
        // Java: return this.fontSize;
        self.fontSize
    }

    // Java: int getFontColor()
    pub fn getFontColor(&self) -> i32 {
        // Java: return this.fontColor;
        self.fontColor
    }

    // Java: Typeface getTypeface()
    pub fn getTypeface(&self) -> &Typeface {
        // Java: return this.typeface;
        &self.typeface
    }

    // Java: int getFontStyle()
    pub fn getFontStyle(&self) -> i32 {
        // Java: return this.fontStyle;
        self.fontStyle
    }

    // Java: boolean isSuperScript()
    pub fn isSuperScript(&self) -> bool {
        // Java: return this.isSuperScript;
        self.isSuperScript
    }

    // Java: boolean isSubScript()
    pub fn isSubScript(&self) -> bool {
        // Java: return this.isSubScript;
        self.isSubScript
    }
}
