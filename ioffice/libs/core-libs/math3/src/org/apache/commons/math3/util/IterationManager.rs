// Java: import java.util.Collection
// Java: import java.util.concurrent.CopyOnWriteArrayList
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;

// Java: public class provides
pub struct provides {
}

// Java: public class IterationManager
pub struct IterationManager {
    // Java field: IntegerSequence.Incrementor iterations
    pub(crate) iterations: Box<dyn std::any::Any>,
    // Java field: Collection<IterationListener> listeners
    pub(crate) listeners: Vec<Box<dyn std::any::Any>>,
}

// Java: public class provides
impl provides {
}

// Java: public class IterationManager
impl IterationManager {
    // Java: IterationManager(final int maxIterations)
    pub fn IterationManager(&self) {
        // Java: this.iterations = IntegerSequence.Incrementor.create().withMaximalCount(maxIterations);
        // Java: this.listeners = new CopyOnWriteArrayList<IterationListener>();
    }

    // Java: IterationManager()
    pub fn IterationManager_2(&self) {
    }

    // Java: IterationManager()
    pub fn IterationManager_3(&self) {
    }

    // Java: void addIterationListener(final IterationListener listener)
    pub fn addIterationListener(&self) {
        // Java: listeners.add(listener);
    }

    // Java: void fireInitializationEvent(final IterationEvent e)
    pub fn fireInitializationEvent(&self) {
        // Java: for (IterationListener l : listeners)
        {
            // Java: l.initializationPerformed(e);
        }
    }

    // Java: void fireIterationPerformedEvent(final IterationEvent e)
    pub fn fireIterationPerformedEvent(&self) {
        // Java: for (IterationListener l : listeners)
        {
            // Java: l.iterationPerformed(e);
        }
    }

    // Java: void fireIterationStartedEvent(final IterationEvent e)
    pub fn fireIterationStartedEvent(&self) {
        // Java: for (IterationListener l : listeners)
        {
            // Java: l.iterationStarted(e);
        }
    }

    // Java: void fireTerminationEvent(final IterationEvent e)
    pub fn fireTerminationEvent(&self) {
        // Java: for (IterationListener l : listeners)
        {
            // Java: l.terminationPerformed(e);
        }
    }

    // Java: int getIterations()
    pub fn getIterations(&self) -> i32 {
        // Java: return iterations.getCount();
        todo!()
    }

    // Java: int getMaxIterations()
    pub fn getMaxIterations(&self) -> i32 {
        // Java: return iterations.getMaximalCount();
        todo!()
    }

    // Java: void incrementIterationCount()
    pub fn incrementIterationCount(&self) {
    }

    // Java: void removeIterationListener(final IterationListener listener)
    pub fn removeIterationListener(&self) {
        // Java: listeners.remove(listener);
    }

    // Java: void resetIterationCount()
    pub fn resetIterationCount(&self) {
        // Java: iterations = iterations.withStart(0);
    }

}
