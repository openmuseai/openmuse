// Java: public interface ILabelProvider<T>
pub trait ILabelProvider<T> {
    // Java: abstract String getText(T var1)
    fn getText(&self, var1: &T) -> String;
}
