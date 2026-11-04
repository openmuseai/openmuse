// Java: public interface CharsetListener
pub trait CharsetListener {
    // Java: abstract void report(String var1)
    fn report(&self, charset: &str);
}
