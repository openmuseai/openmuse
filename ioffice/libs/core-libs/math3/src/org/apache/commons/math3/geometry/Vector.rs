// Java: import java.text.NumberFormat
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;

// Java: public interface represents
pub trait represents {
}

// Java: public interface Vector
pub trait Vector {
    // Java: abstract Vector<S> getZero()
    fn getZero(&self) -> Box<dyn std::any::Any>;

    // Java: abstract double getNorm1()
    fn getNorm1(&self) -> f64;

    // Java: abstract double getNorm()
    fn getNorm(&self) -> f64;

    // Java: abstract double getNormSq()
    fn getNormSq(&self) -> f64;

    // Java: abstract double getNormInf()
    fn getNormInf(&self) -> f64;

    // Java: abstract Vector<S> add(Vector<S> v)
    fn add(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract Vector<S> add(double factor, Vector<S> v)
    fn add_scaled(&self, factor: f64, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract Vector<S> subtract(Vector<S> v)
    fn subtract(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract Vector<S> subtract(double factor, Vector<S> v)
    fn subtract_scaled(&self, factor: f64, v: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract Vector<S> negate()
    fn negate(&self) -> Box<dyn std::any::Any>;

    // Java: abstract Vector<S> normalize()
    fn normalize(&self) -> Box<dyn std::any::Any>;

    // Java: abstract Vector<S> scalarMultiply(double a)
    fn scalarMultiply(&self, a: f64) -> Box<dyn std::any::Any>;

    // Java: abstract boolean isInfinite()
    fn isInfinite(&self) -> bool;

    // Java: abstract double distance1(Vector<S> v)
    fn distance1(&self, v: &dyn std::any::Any) -> f64;

    // Java: abstract double distance(Vector<S> v)
    fn distance(&self, v: &dyn std::any::Any) -> f64;

    // Java: abstract double distanceInf(Vector<S> v)
    fn distanceInf(&self, v: &dyn std::any::Any) -> f64;

    // Java: abstract double distanceSq(Vector<S> v)
    fn distanceSq(&self, v: &dyn std::any::Any) -> f64;

    // Java: abstract double dotProduct(Vector<S> v)
    fn dotProduct(&self, v: &dyn std::any::Any) -> f64;

    // Java: abstract String toString(final NumberFormat format)
    fn toString_formatted(&self, format: &dyn std::any::Any) -> String;

}
