// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public interface represents
pub trait represents {
}

// Java: public interface is
pub trait is {
}

// Java: public interface Hyperplane
pub trait Hyperplane {
    // Java: abstract Hyperplane<S> copySelf()
    fn copySelf(&self) -> Box<dyn std::any::Any>;

    // Java: abstract double getOffset(Point<S> point)
    fn getOffset(&self) -> f64;

    // Java: abstract Point<S> project(Point<S> point)
    fn project(&self) -> Box<dyn std::any::Any>;

    // Java: abstract double getTolerance()
    fn getTolerance(&self) -> f64;

    // Java: abstract boolean sameOrientationAs(Hyperplane<S> other)
    fn sameOrientationAs(&self) -> bool;

    // Java: abstract SubHyperplane<S> wholeHyperplane()
    fn wholeHyperplane(&self) -> Box<dyn std::any::Any>;

    // Java: abstract Region<S> wholeSpace()
    fn wholeSpace(&self) -> Box<dyn std::any::Any>;

}
