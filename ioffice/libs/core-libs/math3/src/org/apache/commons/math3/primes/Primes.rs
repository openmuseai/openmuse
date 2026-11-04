// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import java.util.List

// Java: public class Primes
pub struct Primes {
}

// Java: public class Primes
impl Primes {
    // Java: Primes()
    pub fn Primes(&self) {
    }

    // Java: static boolean isPrime(int n)
    pub fn isPrime(&self) -> bool {
        // Java: if (n < 2)
        {
            // Java: return false;
        }
        // Java: for (int p : SmallPrimes.PRIMES)
        {
            // Java: if (0 == (n % p))
            {
                // Java: return n == p;
            }
        }
        // Java: return SmallPrimes.millerRabinPrimeTest(n);
        todo!()
    }

    // Java: static int nextPrime(int n)
    pub fn nextPrime(&self) -> i32 {
        // Java: if (n < 0)
        {
            // Java: throw new MathIllegalArgumentException(LocalizedFormats.NUMBER_TOO_SMALL, n, 0);
        }
        // Java: if (n == 2)
        {
            // Java: return 2;
        }
        // Java: n |= 1;//make sure n is odd
        // Java: if (n == 1)
        {
            // Java: return 2;
        }
        // Java: if (isPrime(n))
        {
            // Java: return n;
        }
        // Java: // prepare entry in the +2, +4 loop:
        // Java: // n should not be a multiple of 3
        // Java: final int rem = n % 3;
        // Java: if (0 == rem) { // if n % 3 == 0
            // Java: n += 2; // n % 3 == 2
        // Java: } else if (1 == rem) { // if n % 3 == 1
            // Java: // if (isPrime(n)) return n;
            // Java: n += 4; // n % 3 == 2
        // Java: while (true) { // this loop skips all multiple of 3
            // Java: if (isPrime(n))
            {
                // Java: return n;
            }
            // Java: n += 2; // n % 3 == 1
            // Java: if (isPrime(n))
            {
                // Java: return n;
            }
            // Java: n += 4; // n % 3 == 2
        todo!()
    }

    // Java: static List<Integer> primeFactors(int n)
    pub fn primeFactors(&self) -> Box<dyn std::any::Any> {
        // Java: if (n < 2)
        {
            // Java: throw new MathIllegalArgumentException(LocalizedFormats.NUMBER_TOO_SMALL, n, 2);
        }
        // Java: // slower than trial div unless we do an awful lot of computation
        // Java: // (then it finally gets JIT-compiled efficiently
        // Java: // List<Integer> out = PollardRho.primeFactors(n);
        // Java: return SmallPrimes.trialDivision(n);
        todo!()
    }

}
