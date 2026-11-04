// Java: public class is
pub struct is {
}

// Java: public class GaussianRandomGenerator implements NormalizedRandomGenerator
pub struct GaussianRandomGenerator {
    // Java field: RandomGenerator generator
    pub(crate) generator: Box<dyn std::any::Any>,
}

// Java: public class is
impl is {
}

// Java: public class GaussianRandomGenerator implements NormalizedRandomGenerator
impl GaussianRandomGenerator {
    // Java: GaussianRandomGenerator(final RandomGenerator generator)
    pub fn GaussianRandomGenerator(&self) {
        // Java: this.generator = generator;
    }

    // Java: double nextNormalizedDouble()
    pub fn nextNormalizedDouble(&self) -> f64 {
        // Java: return generator.nextGaussian();
        todo!()
    }

}

