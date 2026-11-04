// Java: import java.util.Random

// Java field: long serialVersionUID

// Java: public class RandomAdaptor extends Random implements RandomGenerator
pub struct RandomAdaptor {
    // Java field: RandomGenerator randomGenerator
    pub(crate) randomGenerator: Box<dyn std::any::Any>,
}

// Java: public class RandomAdaptor extends Random implements RandomGenerator
impl RandomAdaptor {
    // Java: RandomAdaptor()
    pub fn RandomAdaptor(&self) {
    }

    // Java: RandomAdaptor(RandomGenerator randomGenerator)
    pub fn RandomAdaptor_2(&self) {
        // Java: this.randomGenerator = randomGenerator;
    }

    // Java: static Random createAdaptor(RandomGenerator randomGenerator)
    pub fn createAdaptor(&self) -> Box<dyn std::any::Any> {
        // Java: return new RandomAdaptor(randomGenerator);
        todo!()
    }

    // Java: boolean nextBoolean()
    pub fn nextBoolean(&self) -> bool {
        // Java: return randomGenerator.nextBoolean();
        todo!()
    }

    // Java: void nextBytes(byte[] bytes)
    pub fn nextBytes(&self) {
        // Java: randomGenerator.nextBytes(bytes);
    }

    // Java: double nextDouble()
    pub fn nextDouble(&self) -> f64 {
        // Java: return randomGenerator.nextDouble();
        todo!()
    }

    // Java: float nextFloat()
    pub fn nextFloat(&self) -> f32 {
        // Java: return randomGenerator.nextFloat();
        todo!()
    }

    // Java: double nextGaussian()
    pub fn nextGaussian(&self) -> f64 {
        // Java: return randomGenerator.nextGaussian();
        todo!()
    }

    // Java: int nextInt()
    pub fn nextInt(&self) -> i32 {
        // Java: return randomGenerator.nextInt();
        todo!()
    }

    // Java: int nextInt(int n)
    pub fn nextInt_2(&self) -> i32 {
        // Java: return randomGenerator.nextInt(n);
        todo!()
    }

    // Java: long nextLong()
    pub fn nextLong(&self) -> i64 {
        // Java: return randomGenerator.nextLong();
        todo!()
    }

    // Java: void setSeed(int seed)
    pub fn setSeed(&self) {
        // Java: if (randomGenerator != null) {  // required to avoid NPE in constructor
            // Java: randomGenerator.setSeed(seed);
    }

    // Java: void setSeed(int[] seed)
    pub fn setSeed_2(&self) {
        // Java: if (randomGenerator != null) {  // required to avoid NPE in constructor
            // Java: randomGenerator.setSeed(seed);
    }

    // Java: void setSeed(long seed)
    pub fn setSeed_3(&self) {
        // Java: if (randomGenerator != null) {  // required to avoid NPE in constructor
            // Java: randomGenerator.setSeed(seed);
    }

}

