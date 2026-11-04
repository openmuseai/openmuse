// Java field: int SPAN_POINT_MARK_MASK
pub const SPAN_POINT_MARK_MASK: i32 = 51;
// Java field: int SPAN_MARK_MARK
pub const SPAN_MARK_MARK: i32 = 17;
// Java field: int SPAN_MARK_POINT
pub const SPAN_MARK_POINT: i32 = 18;
// Java field: int SPAN_POINT_MARK
pub const SPAN_POINT_MARK: i32 = 33;
// Java field: int SPAN_POINT_POINT
pub const SPAN_POINT_POINT: i32 = 34;
// Java field: int SPAN_PARAGRAPH
pub const SPAN_PARAGRAPH: i32 = 51;
// Java field: int SPAN_INCLUSIVE_EXCLUSIVE
pub const SPAN_INCLUSIVE_EXCLUSIVE: i32 = 17;
// Java field: int SPAN_INCLUSIVE_INCLUSIVE
pub const SPAN_INCLUSIVE_INCLUSIVE: i32 = 18;
// Java field: int SPAN_EXCLUSIVE_EXCLUSIVE
pub const SPAN_EXCLUSIVE_EXCLUSIVE: i32 = 33;
// Java field: int SPAN_EXCLUSIVE_INCLUSIVE
pub const SPAN_EXCLUSIVE_INCLUSIVE: i32 = 34;
// Java field: int SPAN_COMPOSING
pub const SPAN_COMPOSING: i32 = 256;
// Java field: int SPAN_INTERMEDIATE
pub const SPAN_INTERMEDIATE: i32 = 512;
// Java field: int SPAN_USER_SHIFT
pub const SPAN_USER_SHIFT: i32 = 24;
// Java field: int SPAN_USER
pub const SPAN_USER: i32 = -16777216;
// Java field: int SPAN_PRIORITY_SHIFT
pub const SPAN_PRIORITY_SHIFT: i32 = 16;
// Java field: int SPAN_PRIORITY
pub const SPAN_PRIORITY: i32 = 16711680;

// Java: public interface Spanned extends CharSequence
pub trait Spanned {
    // Java: abstract <T> T[] getSpans(int var1, int var2, Class<T> var3)
    fn getSpans(&self, var1: i32, var2: i32) -> Vec<Box<dyn std::any::Any>>;

    // Java: abstract int getSpanStart(Object var1)
    fn getSpanStart(&self, var1: &dyn std::any::Any) -> i32;

    // Java: abstract int getSpanEnd(Object var1)
    fn getSpanEnd(&self, var1: &dyn std::any::Any) -> i32;

    // Java: abstract int getSpanFlags(Object var1)
    fn getSpanFlags(&self, var1: &dyn std::any::Any) -> i32;

    // Java: abstract int nextSpanTransition(int var1, int var2, Class var3)
    fn nextSpanTransition(&self, var1: i32, var2: i32) -> i32;

}
