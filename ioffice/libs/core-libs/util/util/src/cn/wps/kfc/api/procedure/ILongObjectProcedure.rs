// Java: public interface ILongObjectProcedure<T>
pub trait ILongObjectProcedure<T> {
    // Java: abstract boolean execute(long var1, T var3)
    fn execute(&mut self, key: i64, value: &T) -> bool;
}
