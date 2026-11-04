// Java: public interface Spannable extends Spanned
pub trait Spannable {
    // Java: abstract void setSpan(Object var1, int var2, int var3, int var4)
    fn setSpan(&self, Object var1, int var2, int var3, int var4);

    // Java: abstract void removeSpan(Object var1)
    fn removeSpan(&self, Object var1);

    // Java: abstract default void removeSpan(Object what, int flags)
    fn removeSpan(&self, Object what, int flags) -> default void;

}
