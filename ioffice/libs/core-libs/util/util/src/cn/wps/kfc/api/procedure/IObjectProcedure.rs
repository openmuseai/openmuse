// Java: public interface IObjectProcedure<T>
pub trait IObjectProcedure<T> {
    // Java: abstract boolean execute(T var1)
    fn execute(&mut self, val: &T) -> bool;
}
