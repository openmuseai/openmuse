// Java: import org.apache.commons.math3.exception.DimensionMismatchException
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::DimensionMismatchException::DimensionMismatchException;

// Java field: throws DimensionMismatchException
// Java field: throws DimensionMismatchException
// Java field: throws DimensionMismatchException
// Java field: throws DimensionMismatchException
// Java field: throws DimensionMismatchException
// Java field: throws DimensionMismatchException

// Java: public interface RealFieldElement extends FieldElement
#[allow(non_snake_case)]
pub trait RealFieldElement {
    type T;

    // Java: abstract double getReal()
    fn getReal(&self) -> f64;

    // Java: abstract T add(double a)
    fn add_double(&self, a: f64) -> Self::T;

    // Java: abstract T subtract(double a)
    fn subtract_double(&self, a: f64) -> Self::T;

    // Java: abstract T multiply(double a)
    fn multiply_double(&self, a: f64) -> Self::T;

    // Java: abstract T divide(double a)
    fn divide_double(&self, a: f64) -> Self::T;

    // Java: abstract T remainder(double a)
    fn remainder_double(&self, a: f64) -> Self::T;

    // Java: abstract T remainder(T a)
    fn remainder(&self, a: &Self::T) -> Self::T;

    // Java: abstract T abs()
    fn abs(&self) -> Self::T;

    // Java: abstract T ceil()
    fn ceil(&self) -> Self::T;

    // Java: abstract T floor()
    fn floor(&self) -> Self::T;

    // Java: abstract T rint()
    fn rint(&self) -> Self::T;

    // Java: abstract long round()
    fn round(&self) -> i64;

    // Java: abstract T signum()
    fn signum(&self) -> Self::T;

    // Java: abstract T copySign(T sign)
    fn copySign(&self, sign: &Self::T) -> Self::T;

    // Java: abstract T copySign(double sign)
    fn copySign_double(&self, sign: f64) -> Self::T;

    // Java: abstract T scalb(int n)
    fn scalb(&self, n: i32) -> Self::T;

    // Java: abstract T hypot(T y)
    fn hypot(&self, y: &Self::T) -> Self::T;

    // Java: abstract T reciprocal()
    fn reciprocal(&self) -> Self::T;

    // Java: abstract T sqrt()
    fn sqrt(&self) -> Self::T;

    // Java: abstract T cbrt()
    fn cbrt(&self) -> Self::T;

    // Java: abstract T rootN(int n)
    fn rootN(&self, n: i32) -> Self::T;

    // Java: abstract T pow(double p)
    fn pow_double(&self, p: f64) -> Self::T;

    // Java: abstract T pow(int n)
    fn pow_int(&self, n: i32) -> Self::T;

    // Java: abstract T pow(T e)
    fn pow(&self, e: &Self::T) -> Self::T;

    // Java: abstract T exp()
    fn exp(&self) -> Self::T;

    // Java: abstract T expm1()
    fn expm1(&self) -> Self::T;

    // Java: abstract T log()
    fn log(&self) -> Self::T;

    // Java: abstract T log1p()
    fn log1p(&self) -> Self::T;

    // Java: abstract T cos()
    fn cos(&self) -> Self::T;

    // Java: abstract T sin()
    fn sin(&self) -> Self::T;

    // Java: abstract T tan()
    fn tan(&self) -> Self::T;

    // Java: abstract T acos()
    fn acos(&self) -> Self::T;

    // Java: abstract T asin()
    fn asin(&self) -> Self::T;

    // Java: abstract T atan()
    fn atan(&self) -> Self::T;

    // Java: abstract T atan2(T x)
    fn atan2(&self, x: &Self::T) -> Self::T;

    // Java: abstract T cosh()
    fn cosh(&self) -> Self::T;

    // Java: abstract T sinh()
    fn sinh(&self) -> Self::T;

    // Java: abstract T tanh()
    fn tanh(&self) -> Self::T;

    // Java: abstract T acosh()
    fn acosh(&self) -> Self::T;

    // Java: abstract T asinh()
    fn asinh(&self) -> Self::T;

    // Java: abstract T atanh()
    fn atanh(&self) -> Self::T;

    // Java: abstract T linearCombination(T[] a, T[] b)
    fn linearCombination_arr(&self, a: &[Self::T], b: &[Self::T]) -> Self::T;

    // Java: abstract T linearCombination(double[] a, T[] b)
    fn linearCombination_double_arr(&self, a: &[f64], b: &[Self::T]) -> Self::T;

    // Java: abstract T linearCombination(T a1, T b1, T a2, T b2)
    fn linearCombination_2(&self, a1: &Self::T, b1: &Self::T, a2: &Self::T, b2: &Self::T) -> Self::T;

    // Java: abstract T linearCombination(double a1, T b1, double a2, T b2)
    fn linearCombination_2d(&self, a1: f64, b1: &Self::T, a2: f64, b2: &Self::T) -> Self::T;

    // Java: abstract T linearCombination(T a1, T b1, T a2, T b2, T a3, T b3)
    fn linearCombination_3(&self, a1: &Self::T, b1: &Self::T, a2: &Self::T, b2: &Self::T, a3: &Self::T, b3: &Self::T) -> Self::T;

    // Java: abstract T linearCombination(double a1, T b1, double a2, T b2, double a3, T b3)
    fn linearCombination_3d(&self, a1: f64, b1: &Self::T, a2: f64, b2: &Self::T, a3: f64, b3: &Self::T) -> Self::T;

    // Java: abstract T linearCombination(T a1, T b1, T a2, T b2, T a3, T b3, T a4, T b4)
    fn linearCombination_4(&self, a1: &Self::T, b1: &Self::T, a2: &Self::T, b2: &Self::T, a3: &Self::T, b3: &Self::T, a4: &Self::T, b4: &Self::T) -> Self::T;

    // Java: abstract T linearCombination(double a1, T b1, double a2, T b2, double a3, T b3, double a4, T b4)
    fn linearCombination_4d(&self, a1: f64, b1: &Self::T, a2: f64, b2: &Self::T, a3: f64, b3: &Self::T, a4: f64, b4: &Self::T) -> Self::T;
}
