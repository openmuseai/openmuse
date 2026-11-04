// Java: public interface representing
pub trait representing {
}

// Java: public interface UnivariateDfpFunction
pub trait UnivariateDfpFunction {
    // Java: abstract Dfp value(Dfp x)
    fn value(&self, x: &super::Dfp::Dfp) -> super::Dfp::Dfp;

}
