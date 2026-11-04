// Java: import java.util.Random

// Java field: long serialVersionUID

// Java: public class JDKRandomGenerator extends Random implements RandomGenerator
pub struct JDKRandomGenerator {
}

// Java: public class JDKRandomGenerator extends Random implements RandomGenerator
impl JDKRandomGenerator {
    // Java: JDKRandomGenerator()
    pub fn new() -> Self {
        // Java: super();
        JDKRandomGenerator {}
    }

    // Java: JDKRandomGenerator(int seed)
    pub fn new_with_seed(_seed: i32) -> Self {
        // Java: setSeed(seed);
        JDKRandomGenerator {}
    }

    // Java: void setSeed(int seed)
    pub fn setSeed(&mut self, _seed: i32) {
        // Java: setSeed((long) seed);
    }

    // Java: void setSeed(int[] seed)
    pub fn setSeed_array(&mut self, _seed: &[i32]) {
        // Java: setSeed(RandomGeneratorFactory.convertToLong(seed));
    }

}
