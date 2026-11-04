// Java: import java.util.Iterator
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.MathUnsupportedOperationException
use crate::org::apache::commons::math3::exception::MathUnsupportedOperationException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;

// Java: public interface MaxCountExceededCallback
pub trait MaxCountExceededCallback {
    // Java: abstract void trigger(int maximalCount)
    fn trigger(&self, max: i32);
}

struct DefaultCallback;
impl MaxCountExceededCallback for DefaultCallback {
    fn trigger(&self, max: i32) {
        panic!("MaxCountExceededException: {}", max);
    }
}

// Java: public class IntegerSequence
pub struct IntegerSequence {
}

// Java: public class Range
pub struct Range {
    // Java field: int size
    pub(crate) size: i32,
    // Java field: int start
    pub(crate) start: i32,
    // Java field: int max
    pub(crate) max: i32,
    // Java field: int step
    pub(crate) step: i32,
}

// Java: public class Incrementor
pub struct Incrementor {
    // Java field: int init
    pub(crate) init: i32,
    // Java field: int maximalCount
    pub(crate) maximalCount: i32,
    // Java field: int increment
    pub(crate) increment: i32,
    // Java field: MaxCountExceededCallback maxCountCallback
    pub(crate) maxCountCallback: Box<dyn MaxCountExceededCallback>,
    // Java field: int count
    pub(crate) count: i32,
}

// Java: public class IntegerSequence
impl IntegerSequence {
    // Java: IntegerSequence()
    pub fn IntegerSequence() -> Self {
        IntegerSequence {}
    }

    // Java: static Range range()
    pub fn range(start: i32, end: i32) -> Range {
        Range::Range(start, end, 1)
    }

    // Java: static Range range()
    pub fn range_step(start: i32, end: i32, step: i32) -> Range {
        Range::Range(start, end, step)
    }
}

// Java: public class Range
impl Range {
    // Java: Range()
    pub fn Range(start: i32, end: i32, step: i32) -> Self {
        let size = if step > 0 {
            ((end - start) / step) + 1
        } else {
            ((start - end) / (-step)) + 1
        };
        Range { size: size.max(0), start, max: end, step }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
            // Java: return size;
            self.size
    }

    // Java: Iterator<Integer> iterator()
    pub fn iterator(&self) -> Incrementor {
            // Java: return Incrementor.create()
                // Java: .withStart(start)
                // Java: .withMaximalCount(max + (step > 0 ? 1 : -1))
                // Java: .withIncrement(step);
            Incrementor::create()
                .withStart(self.start)
                .withMaximalCount(self.max + if self.step > 0 { 1 } else { -1 })
                .withIncrement(self.step)
    }
}

// Java: public class Incrementor
impl Incrementor {
    // Java: Incrementor()
    fn new(init: i32, max: i32, increment: i32, callback: Box<dyn MaxCountExceededCallback>) -> Self {
        Incrementor { init, maximalCount: max, increment, maxCountCallback: callback, count: init }
    }

    // Java: static Incrementor create()
    pub fn create() -> Incrementor {
            // Java: return new Incrementor(0, 0, 1, CALLBACK);
            Self::new(0, 0, 1, Box::new(DefaultCallback))
    }

    // Java: Incrementor withStart(int start)
    pub fn withStart(self, start: i32) -> Incrementor {
            // Java: return new Incrementor(start,
                                   // Java: this.maximalCount,
                                   // Java: this.increment,
                                   // Java: this.maxCountCallback);
            Incrementor { init: start, count: start, maximalCount: self.maximalCount, increment: self.increment, maxCountCallback: self.maxCountCallback }
    }

    // Java: Incrementor withMaximalCount(int max)
    pub fn withMaximalCount(self, max: i32) -> Incrementor {
            // Java: return new Incrementor(this.init,
                                   // Java: max,
                                   // Java: this.increment,
                                   // Java: this.maxCountCallback);
            Incrementor { maximalCount: max, ..self }
    }

    // Java: Incrementor withIncrement(int step)
    pub fn withIncrement(self, step: i32) -> Incrementor {
            // Java: if (step == 0)
            if step == 0 {
                // Java: throw new ZeroException();
                panic!("ZeroException");
            }
            // Java: return new Incrementor(this.init,
                                   // Java: this.maximalCount,
                                   // Java: step,
                                   // Java: this.maxCountCallback);
            Incrementor { increment: step, ..self }
    }

    // Java: Incrementor withCallback(MaxCountExceededCallback cb)
    pub fn withCallback(self, cb: Box<dyn MaxCountExceededCallback>) -> Incrementor {
            // Java: return new Incrementor(this.init,
                                   // Java: this.maximalCount,
                                   // Java: this.increment,
                                   // Java: cb);
            Incrementor { maxCountCallback: cb, ..self }
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
            // Java: return canIncrement(1);
            self.canIncrement_n(1)
    }

    // Java: boolean canIncrement(int nTimes)
    pub fn canIncrement_n(&self, n_times: i32) -> bool {
            // Java: final int finalCount = count + nTimes * increment;
            let final_count = self.count + n_times * self.increment;
            // Java: return increment < 0 ?
                // Java: finalCount > maximalCount :
                // Java: finalCount < maximalCount;
            if self.increment < 0 { final_count > self.maximalCount } else { final_count < self.maximalCount }
    }

    // Java: void increment(int nTimes)
    pub fn increment_n(&mut self, n_times: i32) {
            // Java: if (nTimes <= 0)
            if n_times <= 0 {
                // Java: throw new NotStrictlyPositiveException(nTimes);
                panic!("NotStrictlyPositiveException: {}", n_times);
            }
            // Java: if (!canIncrement(0))
            if !self.canIncrement_n(0) {
                // Java: maxCountCallback.trigger(maximalCount);
                self.maxCountCallback.trigger(self.maximalCount);
            }
            // Java: count += nTimes * increment;
            self.count += n_times * self.increment;
    }

    // Java: void increment()
    pub fn increment_once(&mut self) {
            // Java: increment(1);
            self.increment_n(1);
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
            // Java: return canIncrement(0);
            self.canIncrement_n(0)
    }

    // Java: Integer next()
    pub fn next(&mut self) -> i32 {
            // Java: final int value = count;
            let value = self.count;
            // Java: increment();
            self.increment_once();
            // Java: return value;
            value
    }

    // Java: void remove()
    pub fn remove(&self) {
            // Java: throw new MathUnsupportedOperationException();
            panic!("MathUnsupportedOperationException");
    }
}
