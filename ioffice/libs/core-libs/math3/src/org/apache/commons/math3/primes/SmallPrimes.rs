// Java: import java.math.BigInteger
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: int PRIMES_LAST
pub(crate) const PRIMES_LAST: i32 = 0;

// Java: public class SmallPrimes
pub struct SmallPrimes {
}

// Java: public class SmallPrimes
impl SmallPrimes {
    // Java: SmallPrimes()
    pub fn SmallPrimes(&self) {
    }

    // Java: static int smallTrialDivision(int n, final List<Integer> factors)
    pub fn smallTrialDivision(&self) -> i32 {
        // Java: for (int p : PRIMES)
        {
            // Java: while (0 == n % p)
            {
                // Java: n /= p;
                // Java: factors.add(p);
            }
        }
        // Java: return n;
        todo!()
    }

    // Java: static int boundedTrialDivision(int n, int maxFactor, List<Integer> factors)
    pub fn boundedTrialDivision(&self) -> i32 {
        // Java: int f = PRIMES_LAST + 2;
        // Java: // no check is done about n >= f
        // Java: while (f <= maxFactor)
        {
            // Java: if (0 == n % f)
            {
                // Java: n /= f;
                // Java: factors.add(f);
                // Java: break;
            }
            // Java: f += 4;
            // Java: if (0 == n % f)
            {
                // Java: n /= f;
                // Java: factors.add(f);
                // Java: break;
            }
            // Java: f += 2;
        }
        // Java: if (n != 1)
        {
            // Java: factors.add(n);
        }
        // Java: return n;
        todo!()
    }

    // Java: static List<Integer> trialDivision(int n)
    pub fn trialDivision(&self) -> Box<dyn std::any::Any> {
        // Java: final List<Integer> factors = new ArrayList<Integer>(32);
        // Java: n = smallTrialDivision(n, factors);
        // Java: if (1 == n)
        {
            // Java: return factors;
        }
        // Java: // here we are sure that n is either a prime or a semi prime
        // Java: final int bound = (int) FastMath.sqrt(n);
        // Java: boundedTrialDivision(n, bound, factors);
        // Java: return factors;
        todo!()
    }

    // Java: static boolean millerRabinPrimeTest(final int n)
    pub fn millerRabinPrimeTest(&self) -> bool {
        // Java: final int nMinus1 = n - 1;
        // Java: final int s = Integer.numberOfTrailingZeros(nMinus1);
        // Java: final int r = nMinus1 >> s;
        // Java: //r must be odd, it is not checked here
        // Java: int t = 1;
        // Java: if (n >= 2047)
        {
            // Java: t = 2;
        }
        // Java: if (n >= 1373653)
        {
            // Java: t = 3;
        }
        // Java: if (n >= 25326001)
        {
            // Java: t = 4;
        }
        // Java: BigInteger br = BigInteger.valueOf(r);
        // Java: BigInteger bn = BigInteger.valueOf(n);
        // Java: for (int i = 0; i < t; i++)
        {
            // Java: BigInteger a = BigInteger.valueOf(SmallPrimes.PRIMES[i]);
            // Java: BigInteger bPow = a.modPow(br, bn);
            // Java: int y = bPow.intValue();
            // Java: if ((1 != y) && (y != nMinus1))
            {
                // Java: int j = 1;
                // Java: while ((j <= s - 1) && (nMinus1 != y))
                {
                    // Java: long square = ((long) y) * y;
                    // Java: y = (int) (square % n);
                    // Java: if (1 == y)
                    {
                        // Java: return false;
                    }
                    // Java: j++;
                }
                // Java: if (nMinus1 != y)
                {
                    // Java: return false;
                }
            }
        }
        // Java: return true; // definitely prime
        todo!()
    }

}
