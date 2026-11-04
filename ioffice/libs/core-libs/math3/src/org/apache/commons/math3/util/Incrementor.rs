// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;

// Java: public interface MaxCountExceededCallback
pub trait MaxCountExceededCallback {
    // Java: abstract void trigger(int maximalCount)
    fn trigger(&self, maximal_count: i32);
}

struct DefaultMaxCountExceededCallback;

impl MaxCountExceededCallback for DefaultMaxCountExceededCallback {
    fn trigger(&self, maximal_count: i32) {
        panic!("MaxCountExceededException: {}", maximal_count);
    }
}

// Java: public class Incrementor
pub struct Incrementor {
    // Java field: int maximalCount
    pub(crate) maximalCount: i32,
    // Java field: int count
    pub(crate) count: i32,
    // Java field: MaxCountExceededCallback maxCountCallback
    maxCountCallback: Box<dyn MaxCountExceededCallback>,
}

// Java: public class Incrementor
impl Incrementor {
    // Java: Incrementor()
    pub fn new() -> Self {
        // Java: this(0);
        Self::new_with_max(0)
    }

    // Java: Incrementor(int max)
    pub fn new_with_max(max: i32) -> Self {
        // Java: this(max,
             // Java: new MaxCountExceededCallback()
             // Java: /** {@inheritDoc} */
             // Java: public void trigger(int max) throws MaxCountExceededException
             // Java: throw new MaxCountExceededException(max);
        Self::new_with_max_and_callback(max, Box::new(DefaultMaxCountExceededCallback))
    }

    // Java: Incrementor(int max, MaxCountExceededCallback cb)
    pub fn new_with_max_and_callback(max: i32, cb: Box<dyn MaxCountExceededCallback>) -> Self {
        Incrementor {
            maximalCount: max,
            count: 0,
            maxCountCallback: cb,
        }
    }

    // Java: void setMaximalCount(int max)
    pub fn setMaximalCount(&mut self, max: i32) {
        // Java: maximalCount = max;
        self.maximalCount = max;
    }

    // Java: int getMaximalCount()
    pub fn getMaximalCount(&self) -> i32 {
        // Java: return maximalCount;
        self.maximalCount
    }

    // Java: int getCount()
    pub fn getCount(&self) -> i32 {
        // Java: return count;
        self.count
    }

    // Java: boolean canIncrement()
    pub fn canIncrement(&self) -> bool {
        // Java: return count < maximalCount;
        self.count < self.maximalCount
    }

    // Java: void incrementCount(int value)
    pub fn incrementCount_value(&mut self, value: i32) {
        // Java: for (int i = 0; i < value; i++)
        for _i in 0..value {
            // Java: incrementCount();
            self.incrementCount();
        }
    }

    // Java: void incrementCount()
    pub fn incrementCount(&mut self) {
        // Java: if (++count > maximalCount)
        self.count += 1;
        if self.count > self.maximalCount {
            // Java: maxCountCallback.trigger(maximalCount);
            self.maxCountCallback.trigger(self.maximalCount);
        }
    }

    // Java: void resetCount()
    pub fn resetCount(&mut self) {
        // Java: count = 0;
        self.count = 0;
    }

    // Java: static Incrementor wrap(final IntegerSequence.Incrementor incrementor)
    pub fn wrap(maximal_count: i32, count: i32) -> Incrementor {
        // Java: return new Incrementor()
        // Java: /** Underlying incrementor. */
        // Java: private IntegerSequence.Incrementor delegate;
        // Java: // set up matching values at initialization
        // Java: delegate = incrementor;
        // Java: super.setMaximalCount(delegate.getMaximalCount());
        // Java: super.incrementCount(delegate.getCount());
        // Java: /** {@inheritDoc} */
        // Java: @Override
        // Java: public void setMaximalCount(int max)
        // Java: super.setMaximalCount(max);
        // Java: delegate = delegate.withMaximalCount(max);
        // Java: /** {@inheritDoc} */
        // Java: @Override
        // Java: public void resetCount()
        // Java: super.resetCount();
        // Java: delegate = delegate.withStart(0);
        // Java: /** {@inheritDoc} */
        // Java: @Override
        // Java: public void incrementCount()
        // Java: super.incrementCount();
        // Java: delegate.increment();
        let mut inc = Incrementor::new_with_max(maximal_count);
        inc.count = count;
        inc
    }

}
