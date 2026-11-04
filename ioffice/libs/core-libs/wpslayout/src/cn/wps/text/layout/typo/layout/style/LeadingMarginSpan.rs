// Java: import android.graphics.Canvas
// Java: import android.graphics.Paint
// Java: import cn.wps.text.layout.typo.layout.Layout
use super::ParagraphStyle::ParagraphStyle;

pub struct Canvas;
pub struct Paint;
pub struct Layout;

// Java: public interface LeadingMarginSpan extends ParagraphStyle
pub trait LeadingMarginSpan: ParagraphStyle {
    // Java: abstract int getLeadingMargin(boolean var1)
    fn getLeadingMargin(&self, var1: bool) -> i32;

    // Java: abstract void drawLeadingMargin(Canvas var1, Paint var2, int var3, int var4, float var5, float var6, float var7, CharSequence var8, int var9, int var10, boolean var11, Layout var12)
    fn drawLeadingMargin(&self, _var1: &Canvas, _var2: &Paint, _var3: i32, _var4: i32, _var5: f32, _var6: f32, _var7: f32, _var8: &str, _var9: i32, _var10: i32, _var11: bool, _var12: &Layout);
}

// Java: public class Standard
pub struct Standard {
    // Java field: int mFirst
    pub(crate) mFirst: i32,
    // Java field: int mRest
    pub(crate) mRest: i32,
}

// Java: public class Standard
impl Standard {
    // Java: Standard(int first, int rest)
    pub fn new(first: i32, rest: i32) -> Self {
        // Java: super();
        // Java: this.mFirst = first;
        // Java: this.mRest = rest;
        Standard { mFirst: first, mRest: rest }
    }

    // Java: Standard(int every)
    pub fn new_every(every: i32) -> Self {
        // Java: this(every, every);
        Self::new(every, every)
    }
}

impl ParagraphStyle for Standard {}

impl LeadingMarginSpan for Standard {
    // Java: int getLeadingMargin(boolean first)
    fn getLeadingMargin(&self, first: bool) -> i32 {
        // Java: return first ? this.mFirst : this.mRest;
        if first { self.mFirst } else { self.mRest }
    }

    // Java: void drawLeadingMargin(Canvas c, Paint p, int x, int dir, float top, float baseline, float bottom, CharSequence text, int start, int end, boolean first, Layout layout)
    fn drawLeadingMargin(&self, _c: &Canvas, _p: &Paint, _x: i32, _dir: i32, _top: f32, _baseline: f32, _bottom: f32, _text: &str, _start: i32, _end: i32, _first: bool, _layout: &Layout) {
    }
}
