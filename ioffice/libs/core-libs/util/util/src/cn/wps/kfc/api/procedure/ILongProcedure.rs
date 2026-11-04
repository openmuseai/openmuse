// Java: public interface ILongProcedure
pub trait ILongProcedure {
    // Java: abstract boolean execute(long var1)
    fn execute(&mut self, value: i64) -> bool;
}
