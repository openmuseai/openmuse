#![allow(non_snake_case, dead_code)]

// Java: public interface IIntProcedure
pub trait IIntProcedure {
    // Java: abstract boolean execute(int var1)
    fn execute(&mut self, var1: i32) -> bool;
}
