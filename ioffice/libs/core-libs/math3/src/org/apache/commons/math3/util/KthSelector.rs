// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;

// Java field: long serialVersionUID
// Java field: int MIN_SELECT_SIZE
pub(crate) const MIN_SELECT_SIZE: i32 = 0;

// Java: public class KthSelector implements Serializable
pub struct KthSelector {
    // Java field: PivotingStrategyInterface pivotingStrategy
    pub(crate) pivotingStrategy: Box<dyn std::any::Any>,
}

// Java: public class KthSelector implements Serializable
impl KthSelector {
    // Java: KthSelector()
    pub fn KthSelector(&self) {
        // Java: this.pivotingStrategy = new MedianOf3PivotingStrategy();
    }

    // Java: KthSelector(final PivotingStrategyInterface pivotingStrategy)
    pub fn KthSelector_2(&self) {
    }

    // Java: PivotingStrategyInterface getPivotingStrategy()
    pub fn getPivotingStrategy(&self) -> Box<dyn std::any::Any> {
        // Java: return pivotingStrategy;
        todo!()
    }

    // Java: double select(final double[] work, final int[] pivotsHeap, final int k)
    pub fn select(&self) -> f64 {
        // Java: int begin = 0;
        // Java: int end = work.length;
        // Java: int node = 0;
        // Java: final boolean usePivotsHeap = pivotsHeap != null;
        // Java: while (end - begin > MIN_SELECT_SIZE)
        {
            // Java: final int pivot;
            // Java: if (usePivotsHeap && node < pivotsHeap.length &&
                    // Java: pivotsHeap[node] >= 0)
                    {
                // Java: // the pivot has already been found in a previous call
                // Java: // and the array has already been partitioned around it
                // Java: pivot = pivotsHeap[node];
                    }
            // Java: else
            {
                // Java: // select a pivot and partition work array around it
                // Java: pivot = partition(work, begin, end, pivotingStrategy.pivotIndex(work, begin, end));
                // Java: if (usePivotsHeap && node < pivotsHeap.length)
                {
                    // Java: pivotsHeap[node] = pivot;
                }
            }
            // Java: if (k == pivot)
            {
                // Java: // the pivot was exactly the element we wanted
                // Java: return work[k];
            }
            // Java: else if (k < pivot)
            {
                // Java: // the element is in the left partition
                // Java: end  = pivot;
                // Java: node = FastMath.min(2 * node + 1, usePivotsHeap ? pivotsHeap.length : end);
            }
            // Java: else
            {
                // Java: // the element is in the right partition
                // Java: begin = pivot + 1;
                // Java: node  = FastMath.min(2 * node + 2, usePivotsHeap ? pivotsHeap.length : end);
            }
        }
        // Java: Arrays.sort(work, begin, end);
        // Java: return work[k];
        todo!()
    }

    // Java: int partition(final double[] work, final int begin, final int end, final int pivot)
    pub fn partition(&self) -> i32 {
        // Java: final double value = work[pivot];
        // Java: work[pivot] = work[begin];
        // Java: int i = begin + 1;
        // Java: int j = end - 1;
        // Java: while (i < j)
        {
            // Java: while (i < j && work[j] > value)
            {
                // Java: --j;
            }
            // Java: while (i < j && work[i] < value)
            {
                // Java: ++i;
            }
            // Java: if (i < j)
            {
                // Java: final double tmp = work[i];
                // Java: work[i++] = work[j];
                // Java: work[j--] = tmp;
            }
        }
        // Java: if (i >= end || work[i] > value)
        {
            // Java: --i;
        }
        // Java: work[begin] = work[i];
        // Java: work[i] = value;
        // Java: return i;
        todo!()
    }

}

