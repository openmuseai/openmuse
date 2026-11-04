#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: public interface SpanWatcher extends NoCopySpan
pub trait SpanWatcher {
    // Java: abstract void onSpanAdded(Spannable var1, Object var2, int var3, int var4)
    fn onSpanAdded(&self, _var1: i32, _var2: i32, _var3: i32, _var4: i32);

    // Java: abstract void onSpanRemoved(Spannable var1, Object var2, int var3, int var4)
    fn onSpanRemoved(&self, _var1: i32, _var2: i32, _var3: i32, _var4: i32);

    // Java: abstract void onSpanChanged(Spannable var1, Object var2, int var3, int var4, int var5, int var6)
    fn onSpanChanged(&self, _var1: i32, _var2: i32, _var3: i32, _var4: i32, _var5: i32, _var6: i32);
}
