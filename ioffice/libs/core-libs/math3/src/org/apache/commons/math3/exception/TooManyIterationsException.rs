// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class TooManyIterationsException extends MaxCountExceededException
pub struct TooManyIterationsException {
}

// Java: public class TooManyIterationsException extends MaxCountExceededException
impl TooManyIterationsException {
    // Java: TooManyIterationsException(Number max)
    pub fn TooManyIterationsException(&self, max: i64) {
        // Java: super(max);
        // Java: getContext().addMessage(LocalizedFormats.ITERATIONS);
    }

}
