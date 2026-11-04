// Java: import org.apache.commons.math3.exception.MathArithmeticException
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::MathArithmeticException::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::NullArgumentException::NullArgumentException;

// Java: public interface FieldElement
pub trait FieldElement {
    type T;
    type FieldType;

    // Java: abstract T add(T a)
    fn add(&self, a: &Self::T) -> Self::T;

    // Java: abstract T subtract(T a)
    fn subtract(&self, a: &Self::T) -> Self::T;

    // Java: abstract T negate()
    fn negate(&self) -> Self::T;

    // Java: abstract T multiply(int n)
    fn multiply_int(&self, n: i32) -> Self::T;

    // Java: abstract T multiply(T a)
    fn multiply(&self, a: &Self::T) -> Self::T;

    // Java: abstract T divide(T a)
    fn divide(&self, a: &Self::T) -> Result<Self::T, MathArithmeticException>;

    // Java: abstract T reciprocal()
    fn reciprocal(&self) -> Result<Self::T, MathArithmeticException>;

    // Java: abstract Field<T> getField()
    fn getField(&self) -> &Self::FieldType;
}
