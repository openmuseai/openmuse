// Java: public class SynchronizedRandomGenerator implements RandomGenerator
pub struct SynchronizedRandomGenerator {
    // Java field: RandomGenerator wrapped
    pub(crate) wrapped: Box<dyn std::any::Any>,
}

// Java: public class SynchronizedRandomGenerator implements RandomGenerator
impl SynchronizedRandomGenerator {
    // Java: SynchronizedRandomGenerator(RandomGenerator rng)
    pub fn SynchronizedRandomGenerator(&self) {
        // Java: wrapped = rng;
    }

    // Java: void setSeed(int seed)
    pub fn setSeed(&self) {
        // Java: wrapped.setSeed(seed);
    }

    // Java: void setSeed(int[] seed)
    pub fn setSeed_2(&self) {
        // Java: wrapped.setSeed(seed);
    }

    // Java: void setSeed(long seed)
    pub fn setSeed_3(&self) {
        // Java: wrapped.setSeed(seed);
    }

    // Java: void nextBytes(byte[] bytes)
    pub fn nextBytes(&self) {
        // Java: wrapped.nextBytes(bytes);
    }

    // Java: int nextInt()
    pub fn nextInt(&self) -> i32 {
        // Java: return wrapped.nextInt();
        todo!()
    }

    // Java: int nextInt(int n)
    pub fn nextInt_2(&self) -> i32 {
        // Java: return wrapped.nextInt(n);
        todo!()
    }

    // Java: long nextLong()
    pub fn nextLong(&self) -> i64 {
        // Java: return wrapped.nextLong();
        todo!()
    }

    // Java: boolean nextBoolean()
    pub fn nextBoolean(&self) -> bool {
        // Java: return wrapped.nextBoolean();
        todo!()
    }

    // Java: float nextFloat()
    pub fn nextFloat(&self) -> f32 {
        // Java: return wrapped.nextFloat();
        todo!()
    }

    // Java: double nextDouble()
    pub fn nextDouble(&self) -> f64 {
        // Java: return wrapped.nextDouble();
        todo!()
    }

    // Java: double nextGaussian()
    pub fn nextGaussian(&self) -> f64 {
        // Java: return wrapped.nextGaussian();
        todo!()
    }

}

