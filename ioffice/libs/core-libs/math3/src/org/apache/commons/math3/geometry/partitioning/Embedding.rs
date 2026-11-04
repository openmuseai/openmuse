// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public interface defines
pub trait defines {
}

// Java: public interface is
pub trait is {
}

// Java: public interface Embedding
pub trait Embedding {
    // Java: abstract Point<T> toSubSpace(Point<S> point)
    fn toSubSpace(&self) -> Box<dyn std::any::Any>;

    // Java: abstract Point<S> toSpace(Point<T> point)
    fn toSpace(&self) -> Box<dyn std::any::Any>;

}
