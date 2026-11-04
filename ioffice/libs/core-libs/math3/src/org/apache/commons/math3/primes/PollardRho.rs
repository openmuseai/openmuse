// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class PollardRho
pub struct PollardRho {
}

// Java: public class PollardRho
impl PollardRho {
    // Java: PollardRho()
    pub fn PollardRho(&self) {
    }

    // Java: static List<Integer> primeFactors(int n)
    pub fn primeFactors(&self) -> Box<dyn std::any::Any> {
        // Java: final List<Integer> factors = new ArrayList<Integer>();
        // Java: n = SmallPrimes.smallTrialDivision(n, factors);
        // Java: if (1 == n)
        {
            // Java: return factors;
        }
        // Java: if (SmallPrimes.millerRabinPrimeTest(n))
        {
            // Java: factors.add(n);
            // Java: return factors;
        }
        // Java: int divisor = rhoBrent(n);
        // Java: factors.add(divisor);
        // Java: factors.add(n / divisor);
        // Java: return factors;
        todo!()
    }

    // Java: static int rhoBrent(final int n)
    pub fn rhoBrent(&self) -> i32 {
        // Java: final int x0 = 2;
        // Java: final int m = 25;
        // Java: int cst = SmallPrimes.PRIMES_LAST;
        // Java: int y = x0;
        // Java: int r = 1;
        // Java: do
        {
            // Java: int x = y;
            // Java: for (int i = 0; i < r; i++)
            {
                // Java: final long y2 = ((long) y) * y;
                // Java: y = (int) ((y2 + cst) % n);
            }
            // Java: int k = 0;
            // Java: do
            {
                // Java: final int bound = FastMath.min(m, r - k);
                // Java: int q = 1;
                // Java: for (int i = -3; i < bound; i++) { //start at -3 to ensure we enter this loop at least 3 times
                    // Java: final long y2 = ((long) y) * y;
                    // Java: y = (int) ((y2 + cst) % n);
                    // Java: final long divisor = FastMath.abs(x - y);
                    // Java: if (0 == divisor)
                    {
                        // Java: cst += SmallPrimes.PRIMES_LAST;
                        // Java: k = -m;
                        // Java: y = x0;
                        // Java: r = 1;
                        // Java: break;
                    }
                    // Java: final long prod = divisor * q;
                    // Java: q = (int) (prod % n);
                    // Java: if (0 == q)
                    {
                        // Java: return gcdPositive(FastMath.abs((int) divisor), n);
                    }
                // Java: final int out = gcdPositive(FastMath.abs(q), n);
                // Java: if (1 != out)
                {
                    // Java: return out;
                }
                // Java: k += m;
            }
            // Java: r = 2 * r;
        }
        todo!()
    }

    // Java: static int gcdPositive(int a, int b)
    pub fn gcdPositive(&self) -> i32 {
        // Java: // both a and b must be positive, it is not checked here
        // Java: // gdc(a,0) = a
        // Java: if (a == 0)
        {
            // Java: return b;
        }
        // Java: else if (b == 0)
        {
            // Java: return a;
        }
        // Java: // make a and b odd, keep in mind the common power of twos
        // Java: final int aTwos = Integer.numberOfTrailingZeros(a);
        // Java: a >>= aTwos;
        // Java: final int bTwos = Integer.numberOfTrailingZeros(b);
        // Java: b >>= bTwos;
        // Java: final int shift = FastMath.min(aTwos, bTwos);
        // Java: // a and b >0
        // Java: // if a > b then gdc(a,b) = gcd(a-b,b)
        // Java: // if a < b then gcd(a,b) = gcd(b-a,a)
        // Java: // so next a is the absolute difference and next b is the minimum of current values
        // Java: while (a != b)
        {
            // Java: final int delta = a - b;
            // Java: b = FastMath.min(a, b);
            // Java: a = FastMath.abs(delta);
            // Java: // for speed optimization:
            // Java: // remove any power of two in a as b is guaranteed to be odd throughout all iterations
            // Java: a >>= Integer.numberOfTrailingZeros(a);
        }
        // Java: // gcd(a,a) = a, just "add" the common power of twos
        // Java: return a << shift;
        todo!()
    }

}
