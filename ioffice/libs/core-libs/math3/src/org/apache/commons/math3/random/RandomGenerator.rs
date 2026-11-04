// Java: public interface is
#[allow(non_camel_case_types)]
pub trait r#is {
}

// Java: public interface RandomGenerator
#[allow(non_snake_case)]
pub trait RandomGenerator {
    // Java: abstract void setSeed(int seed)
    fn setSeed_int(&mut self, seed: i32);

    // Java: abstract void setSeed(int[] seed)
    fn setSeed_arr(&mut self, seed: &[i32]);

    // Java: abstract void setSeed(long seed)
    fn setSeed(&mut self, seed: i64);

    // Java: abstract void nextBytes(byte[] bytes)
    fn nextBytes(&mut self, bytes: &mut [u8]);

    // Java: abstract int nextInt()
    fn nextInt(&mut self) -> i32;

    // Java: abstract int nextInt(int n)
    fn nextInt_bound(&mut self, n: i32) -> i32;

    // Java: abstract long nextLong()
    fn nextLong(&mut self) -> i64;

    // Java: abstract boolean nextBoolean()
    fn nextBoolean(&mut self) -> bool;

    // Java: abstract float nextFloat()
    fn nextFloat(&mut self) -> f32;

    // Java: abstract double nextDouble()
    fn nextDouble(&mut self) -> f64;

    // Java: abstract double nextGaussian()
    fn nextGaussian(&mut self) -> f64;
}
