// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class TooManyEvaluationsException extends MaxCountExceededException
pub struct TooManyEvaluationsException {
}

// Java: public class TooManyEvaluationsException extends MaxCountExceededException
impl TooManyEvaluationsException {
    // Java: TooManyEvaluationsException(Number max)
    pub fn TooManyEvaluationsException(&self, max: i64) {
        // Java: super(max);
        // Java: getContext().addMessage(LocalizedFormats.EVALUATIONS);
    }

}
