// Java: import java.util.Random
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;

// Java: public class RandomGeneratorFactory
pub struct RandomGeneratorFactory {
}

// Java: public class RandomGeneratorFactory
impl RandomGeneratorFactory {
    // Java: RandomGeneratorFactory()
    pub fn RandomGeneratorFactory(&self) {
    }

    // Java: static RandomGenerator createRandomGenerator(final Random rng)
    pub fn createRandomGenerator(&self) -> Box<dyn std::any::Any> {
        // Java: return new RandomGenerator()
        {
            // Java: /** {@inheritDoc} */
            // Java: public void setSeed(int seed)
            {
                // Java: rng.setSeed((long) seed);
            }
            // Java: /** {@inheritDoc} */
            // Java: public void setSeed(int[] seed)
            {
                // Java: rng.setSeed(convertToLong(seed));
            }
            // Java: /** {@inheritDoc} */
            // Java: public void setSeed(long seed)
            {
                // Java: rng.setSeed(seed);
            }
            // Java: /** {@inheritDoc} */
            // Java: public void nextBytes(byte[] bytes)
            {
                // Java: rng.nextBytes(bytes);
            }
            // Java: /** {@inheritDoc} */
            // Java: public int nextInt()
            {
                // Java: return rng.nextInt();
            }
            // Java: /** {@inheritDoc} */
            // Java: public int nextInt(int n)
            {
                // Java: if (n <= 0)
                {
                    // Java: throw new NotStrictlyPositiveException(n);
                }
                // Java: return rng.nextInt(n);
            }
            // Java: /** {@inheritDoc} */
            // Java: public long nextLong()
            {
                // Java: return rng.nextLong();
            }
            // Java: /** {@inheritDoc} */
            // Java: public boolean nextBoolean()
            {
                // Java: return rng.nextBoolean();
            }
            // Java: /** {@inheritDoc} */
            // Java: public float nextFloat()
            {
                // Java: return rng.nextFloat();
            }
            // Java: /** {@inheritDoc} */
            // Java: public double nextDouble()
            {
                // Java: return rng.nextDouble();
            }
            // Java: /** {@inheritDoc} */
            // Java: public double nextGaussian()
            {
                // Java: return rng.nextGaussian();
            }
        }
        todo!()
    }

    // Java: static long convertToLong(int[] seed)
    pub fn convertToLong(&self) -> i64 {
        // Java: // The following number is the largest prime that fits
        // Java: // in 32 bits (i.e. 2^32 - 5).
        // Java: final long prime = 4294967291l;
        // Java: long combined = 0l;
        // Java: for (int s : seed)
        {
            // Java: combined = combined * prime + s;
        }
        // Java: return combined;
        todo!()
    }

}
