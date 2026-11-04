// Java: public interface IDoubleProcedure
pub trait IDoubleProcedure {
    // Java: abstract boolean execute(double var1)
    fn execute(&self, var1: f64) -> bool;
}
