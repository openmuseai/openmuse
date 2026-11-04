#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: public interface NoCopySpan
pub trait NoCopySpan {
}

// Java: public class Concrete
pub struct Concrete {
}

// Java: public class Concrete
impl Concrete {
    // Java: Concrete()
    pub fn new() -> Self {
        // Java: super();
        Concrete {}
    }
}

impl NoCopySpan for Concrete {}
