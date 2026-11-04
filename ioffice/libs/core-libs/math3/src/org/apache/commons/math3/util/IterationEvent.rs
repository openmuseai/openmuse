// Java: import java.util.EventObject

// Java field: long serialVersionUID

// Java: public class from
pub struct from {
}

// Java: public class IterationEvent extends EventObject
pub struct IterationEvent {
    // Java field: int iterations
    pub(crate) iterations: i32,
}

// Java: public class from
impl from {
}

// Java: public class IterationEvent extends EventObject
impl IterationEvent {
    // Java: IterationEvent(final Object source, final int iterations)
    pub fn IterationEvent(&self) {
        // Java: super(source);
        // Java: this.iterations = iterations;
    }

    // Java: int getIterations()
    pub fn getIterations(&self) -> i32 {
        // Java: return iterations;
        todo!()
    }

}
