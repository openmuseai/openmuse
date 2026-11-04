// Java: public interface represents
pub trait represents {
}

// Java: public interface as
pub trait r#as {
}

// Java: public interface FixedStepHandler
pub trait FixedStepHandler {
    // Java: abstract void init(double t0, double[] y0, double t)
    fn init(&self);

    // Java: abstract void handleStep(double t, double[] y, double[] yDot, boolean isLast)
    fn handleStep(&self);

}
