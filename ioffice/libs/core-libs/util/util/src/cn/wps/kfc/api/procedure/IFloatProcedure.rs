// Java: public interface IFloatProcedure
pub trait IFloatProcedure {
    // Java: abstract boolean execute(float var1)
    fn execute(&self, var1: f32) -> bool;
}
