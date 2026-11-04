// Java: import cn.wps.text.layout.typo.layout.Spannable
use crate::text::layout::typo::layout::Spannable;

// Java: public interface SpanWatcher extends NoCopySpan
pub trait SpanWatcher {
    // Java: abstract void onSpanAdded(Spannable var1, Object var2, int var3, int var4)
    fn onSpanAdded(&self, Spannable var1, Object var2, int var3, int var4);

    // Java: abstract void onSpanRemoved(Spannable var1, Object var2, int var3, int var4)
    fn onSpanRemoved(&self, Spannable var1, Object var2, int var3, int var4);

    // Java: abstract void onSpanChanged(Spannable var1, Object var2, int var3, int var4, int var5, int var6)
    fn onSpanChanged(&self, Spannable var1, Object var2, int var3, int var4, int var5, int var6);

}
