// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable;

// Java field: long serialVersionUID

// Java: public class InvalidRepresentationException extends MathIllegalArgumentException
pub struct InvalidRepresentationException {
    pub(crate) message: String,
}

// Java: public class InvalidRepresentationException extends MathIllegalArgumentException
impl InvalidRepresentationException {
    // Java: InvalidRepresentationException(Localizable pattern, Object ... args)
    pub fn new(pattern: &str) -> Self {
       // Java: super(pattern, args);
       InvalidRepresentationException { message: pattern.to_string() }
    }

}
