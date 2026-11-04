// java public interface INumericValueEval extends IValueEval
pub trait INumericValueEval: super::IValueEval {
    fn getNumberValue(&self) -> f64;
}
