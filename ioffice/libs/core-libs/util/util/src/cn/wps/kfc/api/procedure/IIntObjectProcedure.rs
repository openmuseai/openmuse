// Java: public interface IIntObjectProcedure<T>

pub trait IIntObjectProcedure<T> {
    // Java: boolean execute(int var1, T var2)
    fn execute(&mut self, key: i32, value: &T) -> bool;
}
