// Java: import java.util.concurrent.TimeUnit
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;

// Java: public class FixedElapsedTime implements StoppingCondition
pub struct FixedElapsedTime {
    // Java field: long maxTimePeriod
    maxTimePeriod: i64,
    // Java field: long endTime
    endTime: i64,
}

// Java: public class FixedElapsedTime implements StoppingCondition
impl FixedElapsedTime {
    // Java: FixedElapsedTime(final long maxTime)
    pub fn new(max_time: i64) -> Self {
        // Java: this(maxTime, TimeUnit.SECONDS);
        Self::new_with_nanos(max_time * 1_000_000_000)
    }

    // Java: FixedElapsedTime(final long maxTime, final TimeUnit unit)
    pub fn new_with_nanos(max_time_nanos: i64) -> Self {
        // Java: if (maxTime < 0)
        if max_time_nanos < 0 {
            // Java: throw new NumberIsTooSmallException(maxTime, 0, true);
            panic!("Max time must be non-negative");
        }
        // Java: maxTimePeriod = unit.toNanos(maxTime);
        FixedElapsedTime { maxTimePeriod: max_time_nanos, endTime: -1 }
    }

    // Java: boolean isSatisfied(final Population population)
    pub fn isSatisfied(&mut self, _population: &dyn std::any::Any) -> bool {
        // Java: if (endTime < 0)
        if self.endTime < 0 {
            // Java: endTime = System.nanoTime() + maxTimePeriod;
            self.endTime = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i64
                + self.maxTimePeriod;
        }
        // Java: return System.nanoTime() >= endTime;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;
        now >= self.endTime
    }

}
