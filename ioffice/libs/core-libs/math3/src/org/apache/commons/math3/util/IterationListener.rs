// Java: import java.util.EventListener

// Java: public interface for
pub trait r#for {
}

// Java: public interface IterationListener
pub trait IterationListener {
    // Java: abstract void initializationPerformed(IterationEvent e)
    fn initializationPerformed(&self);

    // Java: abstract void iterationPerformed(IterationEvent e)
    fn iterationPerformed(&self);

    // Java: abstract void iterationStarted(IterationEvent e)
    fn iterationStarted(&self);

    // Java: abstract void terminationPerformed(IterationEvent e)
    fn terminationPerformed(&self);

}
