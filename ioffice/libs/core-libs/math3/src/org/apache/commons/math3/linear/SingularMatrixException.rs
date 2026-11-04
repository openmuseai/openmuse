// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class SingularMatrixException extends MathIllegalArgumentException
pub struct SingularMatrixException {
}

// Java: public class SingularMatrixException extends MathIllegalArgumentException
impl SingularMatrixException {
    // Java: SingularMatrixException()
    pub fn SingularMatrixException(&self) {
        // Java: super(LocalizedFormats.SINGULAR_MATRIX);
    }

}
