// Java: public interface Spannable extends Spanned
pub trait Spannable: super::Spanned::Spanned {
    // Java: abstract void setSpan(Object var1, int var2, int var3, int var4)
    fn setSpan(&mut self, var1: Box<dyn std::any::Any>, var2: i32, var3: i32, var4: i32);

    // Java: abstract void removeSpan(Object var1)
    fn removeSpan(&mut self, var1: &dyn std::any::Any);

    // Java: abstract default void removeSpan(Object what, int flags)
    fn removeSpan_with_flags(&mut self, what: &dyn std::any::Any, _flags: i32) {
        // Java: this.removeSpan(what);
        self.removeSpan(what);
    }

}

// Java field: Factory sInstance
// Java: public class Factory
pub struct Factory;

// Java: public class Factory
impl Factory {
    // Java: Factory()
    pub fn new() -> Self {
        // Java: super();
        Factory
    }

    // Java: static Factory getInstance()
    pub fn getInstance() -> &'static Factory {
        // Java: return sInstance;
        static INSTANCE: Factory = Factory;
        &INSTANCE
    }

    // Java: Spannable newSpannable(CharSequence source)
    pub fn newSpannable(&self) {
        // Java: return new SpannableString(source);
    }
}
