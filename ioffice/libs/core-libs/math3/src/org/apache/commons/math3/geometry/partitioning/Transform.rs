// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public interface represents
pub trait represents {
}

// Java: public interface Transform extends Space
pub trait Transform {
    // Java: abstract Point<S> apply(Point<S> point)
    fn apply(&self) -> Box<dyn std::any::Any>;

    // Java: abstract Hyperplane<S> apply(Hyperplane<S> hyperplane)
    fn apply_2(&self) -> Box<dyn std::any::Any>;

    // Java: abstract SubHyperplane<T> apply(SubHyperplane<T> sub, Hyperplane<S> original, Hyperplane<S> transformed)
    fn apply_3(&self) -> Box<dyn std::any::Any>;

}
