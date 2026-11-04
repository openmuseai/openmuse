// Java: import java.util.NoSuchElementException
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;

// Java: public class MultidimensionalCounter implements Iterable
pub struct MultidimensionalCounter {
    // Java field: int dimension
    pub(crate) dimension: i32,
    // Java field: int[] uniCounterOffset
    uniCounterOffset: Vec<i32>,
    // Java field: int[] size
    pub(crate) size: Box<dyn std::any::Any>,
    // Java field: int totalSize
    pub(crate) totalSize: i32,
    // Java field: int last
    pub(crate) last: i32,
}

// Java: public class Iterator
pub struct Iterator {
    // Java field: int[] counter
    pub(crate) counter: Box<dyn std::any::Any>,
    // Java field: int count
    pub(crate) count: i32,
    // Java field: int maxCount
    pub(crate) maxCount: i32,
}

// Java: public class MultidimensionalCounter implements Iterable
impl MultidimensionalCounter {
    // Java: MultidimensionalCounter(int ... size)
    pub fn MultidimensionalCounter(&self) {
        // Java: dimension = size.length;
        // Java: this.size = MathArrays.copyOf(size);
        // Java: uniCounterOffset = new int[dimension];
        // Java: last = dimension - 1;
        // Java: int tS = size[last];
        // Java: for (int i = 0; i < last; i++)
        {
            // Java: int count = 1;
            // Java: for (int j = i + 1; j < dimension; j++)
            {
                // Java: count *= size[j];
            }
            // Java: uniCounterOffset[i] = count;
            // Java: tS *= size[i];
        }
        // Java: uniCounterOffset[last] = 0;
        // Java: if (tS <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(tS);
        }
        // Java: totalSize = tS;
    }

    // Java: Iterator iterator()
    pub fn iterator(&self) -> Box<dyn std::any::Any> {
        // Java: return new Iterator();
        todo!()
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return dimension;
        todo!()
    }

    // Java: int[] getCounts(int index)
    pub fn getCounts(&self) -> Vec<i32> {
        // Java: if (index < 0 ||
            // Java: index >= totalSize)
            {
            // Java: throw new OutOfRangeException(index, 0, totalSize);
            }
        // Java: final int[] indices = new int[dimension];
        // Java: int count = 0;
        // Java: for (int i = 0; i < last; i++)
        {
            // Java: int idx = 0;
            // Java: final int offset = uniCounterOffset[i];
            // Java: while (count <= index)
            {
                // Java: count += offset;
                // Java: ++idx;
            }
            // Java: --idx;
            // Java: count -= offset;
            // Java: indices[i] = idx;
        }
        // Java: indices[last] = index - count;
        // Java: return indices;
        todo!()
    }

    // Java: int getCount(int ... c)
    pub fn getCount(&self) -> i32 {
        todo!()
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return totalSize;
        todo!()
    }

    // Java: int[] getSizes()
    pub fn getSizes(&self) -> Vec<i32> {
        // Java: return MathArrays.copyOf(size);
        todo!()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: final StringBuilder sb = new StringBuilder();
        // Java: for (int i = 0; i < dimension; i++)
        {
            // Java: sb.append("[").append(getCount(i)).append("]");
        }
        // Java: return sb.toString();
        todo!()
    }

}

// Java: public class Iterator
impl Iterator {
    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
            // Java: return count < maxCount;
        todo!()
    }

    // Java: Integer next()
    pub fn next(&self) -> Box<dyn std::any::Any> {
            // Java: if (!hasNext())
            {
                // Java: throw new NoSuchElementException();
            }
            // Java: for (int i = last; i >= 0; i--)
            {
                // Java: if (counter[i] == size[i] - 1)
                {
                    // Java: counter[i] = 0;
                }
                // Java: else
                {
                    // Java: ++counter[i];
                    // Java: break;
                }
            }
            // Java: return ++count;
        todo!()
    }

    // Java: int getCount()
    pub fn getCount_2(&self) -> i32 {
            // Java: return count;
        todo!()
    }

    // Java: int[] getCounts()
    pub fn getCounts_2(&self) -> Vec<i32> {
            // Java: return MathArrays.copyOf(counter);
        todo!()
    }

    // Java: int getCount(int dim)
    pub fn getCount_3(&self) -> i32 {
            // Java: return counter[dim];
        todo!()
    }

    // Java: void remove()
    pub fn remove(&self) {
            // Java: throw new UnsupportedOperationException();
    }

}

