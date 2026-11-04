// Java: import android.graphics.Paint
// Java: import android.text.style.ReplacementSpan

pub struct Paint;
pub struct FontMetricsInt;

// Java: public class LooseReplacementSpan extends ReplacementSpan
pub struct LooseReplacementSpan {
}

// Java: public class LooseReplacementSpan extends ReplacementSpan
impl LooseReplacementSpan {
    // Java: LooseReplacementSpan()
    pub fn new() -> Self {
        // Java: super();
        LooseReplacementSpan {}
    }

    // Java: abstract int[] getSizes(Paint var1, CharSequence var2, int var3, int var4, Paint.FontMetricsInt var5)
    pub fn getSizes(&self, _var1: &Paint, _var2: &str, _var3: i32, _var4: i32, _var5: Option<&mut FontMetricsInt>) -> Vec<i32> {
        Vec::new()
    }
}
