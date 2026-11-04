// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;

// Java field: throws NullArgumentException, MathArithmeticException
// Java field: throws NullArgumentException, MathArithmeticException
// Java field: throws DimensionMismatchException
// Java field: throws DimensionMismatchException, MathArithmeticException
// Java field: throws DimensionMismatchException, MathArithmeticException
// Java field: throws OutOfRangeException, NotPositiveException

// Java: public interface FieldVector extends FieldElement
pub trait FieldVector {
    // Java: abstract Field<T> getField()
    fn getField(&self) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> copy()
    fn copy(&self) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> add(FieldVector<T> v)
    fn add(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> subtract(FieldVector<T> v)
    fn subtract(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> mapAdd(T d)
    fn mapAdd(&self, d: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> mapAddToSelf(T d)
    fn mapAddToSelf(&mut self, d: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> mapSubtract(T d)
    fn mapSubtract(&self, d: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> mapSubtractToSelf(T d)
    fn mapSubtractToSelf(&mut self, d: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> mapMultiply(T d)
    fn mapMultiply(&self, d: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> mapMultiplyToSelf(T d)
    fn mapMultiplyToSelf(&mut self, d: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> mapDivide(T d)
    fn mapDivide(&self, d: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> mapDivideToSelf(T d)
    fn mapDivideToSelf(&mut self, d: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> mapInv()
    fn mapInv(&self) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> mapInvToSelf()
    fn mapInvToSelf(&mut self) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> ebeMultiply(FieldVector<T> v)
    fn ebeMultiply(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> ebeDivide(FieldVector<T> v)
    fn ebeDivide(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract T[] getData()
    fn getData(&self) -> Vec<Box<dyn std::any::Any>>;

    // Java: abstract T dotProduct(FieldVector<T> v)
    fn dotProduct(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> projection(FieldVector<T> v)
    fn projection(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldMatrix<T> outerProduct(FieldVector<T> v)
    fn outerProduct(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract T getEntry(int index)
    fn getEntry(&self, index: i32) -> Box<dyn std::any::Any>;

    // Java: abstract void setEntry(int index, T value)
    fn setEntry(&mut self, index: i32, value: Box<dyn std::any::Any>);

    // Java: abstract int getDimension()
    fn getDimension(&self) -> i32;

    // Java: abstract FieldVector<T> append(FieldVector<T> v)
    fn append(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> append(T d)
    fn append_element(&self, d: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract FieldVector<T> getSubVector(int index, int n)
    fn getSubVector(&self, index: i32, n: i32) -> Box<dyn std::any::Any>;

    // Java: abstract void setSubVector(int index, FieldVector<T> v)
    fn setSubVector(&mut self, index: i32, v: &dyn std::any::Any);

    // Java: abstract void set(T value)
    fn set(&mut self, value: Box<dyn std::any::Any>);

    // Java: abstract T[] toArray()
    fn toArray(&self) -> Vec<Box<dyn std::any::Any>>;

}
