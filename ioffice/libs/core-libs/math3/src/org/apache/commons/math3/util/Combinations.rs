// Java: import java.util.Iterator
// Java: import java.util.Comparator
// Java: import java.util.Arrays
// Java: import java.util.NoSuchElementException
// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;

// Java: public class Combinations implements Iterable
pub struct Combinations {
    // Java field: int n
    pub(crate) n: i32,
    // Java field: int k
    pub(crate) k: i32,
    // Java field: IterationOrder iterationOrder
    iterationOrder: IterationOrder,
}

// Java: public enum IterationOrder
pub struct IterationOrder {
}

// Java: public class LexicographicIterator
pub struct LexicographicIterator {
    // Java field: int k
    pub(crate) k: i32,
    // Java field: int[] c
    pub(crate) c: Box<dyn std::any::Any>,
    // Java field: boolean more
    pub(crate) more: bool,
    // Java field: int j
    pub(crate) j: i32,
}

// Java: public class SingletonIterator
pub struct SingletonIterator {
    // Java field: int[] singleton
    pub(crate) singleton: Box<dyn std::any::Any>,
    // Java field: boolean more
    pub(crate) more: bool,
}

// Java: public class LexicographicComparator
pub struct LexicographicComparator {
}

// Java: public class Combinations implements Iterable
impl Combinations {
    // Java: Combinations()
    pub fn Combinations(&self) {
    }

    // Java: Combinations()
    pub fn Combinations_2(&self) {
    }

    // Java: int getN()
    pub fn getN(&self) -> i32 {
        // Java: return n;
        todo!()
    }

    // Java: int getK()
    pub fn getK(&self) -> i32 {
        // Java: return k;
        todo!()
    }

    // Java: Iterator<int[]> iterator()
    pub fn iterator(&self) -> Box<dyn std::any::Any> {
        // Java: if (k == 0 ||
            // Java: k == n)
            {
            // Java: return new SingletonIterator(MathArrays.natural(k));
            }
        // Java: switch (iterationOrder)
        {
        // Java: case LEXICOGRAPHIC:
            // Java: return new LexicographicIterator(n, k);
        // Java: default:
            // Java: throw new MathInternalError(); // Should never happen.
        }
        todo!()
    }

    // Java: Comparator<int[]> comparator()
    pub fn comparator(&self) -> Box<dyn std::any::Any> {
        // Java: return new LexicographicComparator(n, k);
        todo!()
    }

}

// Java: public enum IterationOrder
impl IterationOrder {
}

// Java: public class LexicographicIterator
impl LexicographicIterator {
    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
            // Java: return more;
        todo!()
    }

    // Java: int[] next()
    pub fn next(&self) -> Vec<i32> {
            // Java: if (!more)
            {
                // Java: throw new NoSuchElementException();
            }
            // Java: // Copy return value (prepared by last activation)
            // Java: final int[] ret = new int[k];
            // Java: System.arraycopy(c, 1, ret, 0, k);
            // Java: // Prepare next iteration
            // Java: // T2 and T6 loop
            // Java: int x = 0;
            // Java: if (j > 0)
            {
                // Java: x = j;
                // Java: c[j] = x;
                // Java: j--;
                // Java: return ret;
            }
            // Java: // T3
            // Java: if (c[1] + 1 < c[2])
            {
                // Java: c[1]++;
                // Java: return ret;
            }
            // Java: else
            {
                // Java: j = 2;
            }
            // Java: // T4
            // Java: boolean stepDone = false;
            // Java: while (!stepDone)
            {
                // Java: c[j - 1] = j - 2;
                // Java: x = c[j] + 1;
                // Java: if (x == c[j + 1])
                {
                    // Java: j++;
                }
                // Java: else
                {
                    // Java: stepDone = true;
                }
            }
            // Java: // T5
            // Java: if (j > k)
            {
                // Java: more = false;
                // Java: return ret;
            }
            // Java: // T6
            // Java: c[j] = x;
            // Java: j--;
            // Java: return ret;
        todo!()
    }

    // Java: void remove()
    pub fn remove(&self) {
            // Java: throw new UnsupportedOperationException();
    }

}

// Java: public class SingletonIterator
impl SingletonIterator {
    // Java: boolean hasNext()
    pub fn hasNext_2(&self) -> bool {
            // Java: return more;
        todo!()
    }

    // Java: int[] next()
    pub fn next_2(&self) -> Vec<i32> {
            // Java: if (more)
            {
                // Java: more = false;
                // Java: return singleton;
            }
            // Java: else
            {
                // Java: throw new NoSuchElementException();
            }
        todo!()
    }

    // Java: void remove()
    pub fn remove_2(&self) {
            // Java: throw new UnsupportedOperationException();
    }

}

// Java: public class LexicographicComparator
impl LexicographicComparator {
}

