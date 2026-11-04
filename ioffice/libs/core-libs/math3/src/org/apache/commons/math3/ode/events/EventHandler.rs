// Java: public interface represents
pub trait represents {
}

// Java: public enum Action
pub enum Action {
    STOP,
    RESET_STATE,
    RESET_DERIVATIVES,
    CONTINUE,
}

// Java: public interface EventHandler
pub trait EventHandler {
    // Java: abstract void init(double t0, double[] y0, double t)
    fn init(&self);

    // Java: abstract double g(double t, double[] y)
    fn g(&self) -> f64;

    // Java: abstract Action eventOccurred(double t, double[] y, boolean increasing)
    fn eventOccurred(&self) -> Box<dyn std::any::Any>;

    // Java: abstract void resetState(double t, double[] y)
    fn resetState(&self);

}
