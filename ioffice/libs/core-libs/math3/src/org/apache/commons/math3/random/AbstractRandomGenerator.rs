// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class implementing
pub struct implementing {
}

// Java: public class AbstractRandomGenerator implements RandomGenerator
pub struct AbstractRandomGenerator {
    // Java field: double cachedNormalDeviate
    cachedNormalDeviate: f64,
}

// Java: public class implementing
impl implementing {
}

// Java: public class AbstractRandomGenerator implements RandomGenerator
impl AbstractRandomGenerator {
    // Java: AbstractRandomGenerator()
    pub fn AbstractRandomGenerator(&self) {
        // Java: super();
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: cachedNormalDeviate = Double.NaN;
    }

    // Java: void setSeed(int seed)
    pub fn setSeed(&self) {
        // Java: setSeed((long) seed);
    }

    // Java: void setSeed(int[] seed)
    pub fn setSeed_2(&self) {
        // Java: // the following number is the largest prime that fits in 32 bits (it is 2^32 - 5)
        // Java: final long prime = 4294967291l;
        // Java: long combined = 0l;
        // Java: for (int s : seed)
        {
            // Java: combined = combined * prime + s;
        }
        // Java: setSeed(combined);
    }

    // Java: abstract void setSeed(long seed)
    pub fn setSeed_3(&self) {
    }

    // Java: void nextBytes(byte[] bytes)
    pub fn nextBytes(&self) {
        // Java: int bytesOut = 0;
        // Java: while (bytesOut < bytes.length)
        {
          // Java: int randInt = nextInt();
          // Java: for (int i = 0; i < 3; i++)
          {
              // Java: if ( i > 0)
              {
                  // Java: randInt >>= 8;
              }
              // Java: bytes[bytesOut++] = (byte) randInt;
              // Java: if (bytesOut == bytes.length)
              {
                  // Java: return;
              }
          }
        }
    }

    // Java: int nextInt()
    pub fn nextInt(&self) -> i32 {
        // Java: return (int) ((2d * nextDouble() - 1d) * Integer.MAX_VALUE);
        todo!()
    }

    // Java: int nextInt(int n)
    pub fn nextInt_2(&self) -> i32 {
        // Java: if (n <= 0 )
        {
            // Java: throw new NotStrictlyPositiveException(n);
        }
        // Java: int result = (int) (nextDouble() * n);
        // Java: return result < n ? result : n - 1;
        todo!()
    }

    // Java: long nextLong()
    pub fn nextLong(&self) -> i64 {
        // Java: return (long) ((2d * nextDouble() - 1d) * Long.MAX_VALUE);
        todo!()
    }

    // Java: boolean nextBoolean()
    pub fn nextBoolean(&self) -> bool {
        // Java: return nextDouble() <= 0.5;
        todo!()
    }

    // Java: float nextFloat()
    pub fn nextFloat(&self) -> f32 {
        // Java: return (float) nextDouble();
        todo!()
    }

    // Java: abstract double nextDouble()
    pub fn nextDouble(&self) -> f64 {
        todo!()
    }

    // Java: double nextGaussian()
    pub fn nextGaussian(&self) -> f64 {
        // Java: if (!Double.isNaN(cachedNormalDeviate))
        {
            // Java: double dev = cachedNormalDeviate;
            // Java: cachedNormalDeviate = Double.NaN;
            // Java: return dev;
        }
        // Java: double v1 = 0;
        // Java: double v2 = 0;
        // Java: double s = 1;
        // Java: while (s >=1 )
        {
            // Java: v1 = 2 * nextDouble() - 1;
            // Java: v2 = 2 * nextDouble() - 1;
            // Java: s = v1 * v1 + v2 * v2;
        }
        // Java: if (s != 0)
        {
            // Java: s = FastMath.sqrt(-2 * FastMath.log(s) / s);
        }
        // Java: cachedNormalDeviate = v2 * s;
        // Java: return v1 * s;
        todo!()
    }

}

