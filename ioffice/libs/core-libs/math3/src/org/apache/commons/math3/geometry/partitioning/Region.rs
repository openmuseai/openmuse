// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;

// Java: public enum Location
pub enum Location {
    INSIDE,
    OUTSIDE,
    BOUNDARY,
}

// Java: public interface represents
pub trait represents {
}

// Java: public interface is
pub trait is {
}

// Java: public interface Region
pub trait Region {
    // Java: abstract Region<S> buildNew(BSPTree<S> newTree)
    fn buildNew(&self) -> Box<dyn std::any::Any>;

    // Java: abstract Region<S> copySelf()
    fn copySelf(&self) -> Box<dyn std::any::Any>;

    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // Java: abstract boolean isEmpty(final BSPTree<S> node)
    fn isEmpty_2(&self) -> bool;

    // Java: abstract boolean isFull()
    fn isFull(&self) -> bool;

    // Java: abstract boolean isFull(final BSPTree<S> node)
    fn isFull_2(&self) -> bool;

    // Java: abstract boolean contains(final Region<S> region)
    fn contains(&self) -> bool;

    // Java: abstract Location checkPoint(final Point<S> point)
    fn checkPoint(&self) -> Box<dyn std::any::Any>;

    // Java: abstract BoundaryProjection<S> projectToBoundary(final Point<S> point)
    fn projectToBoundary(&self) -> Box<dyn std::any::Any>;

    // Java: abstract BSPTree<S> getTree(final boolean includeBoundaryAttributes)
    fn getTree(&self) -> Box<dyn std::any::Any>;

    // Java: abstract double getBoundarySize()
    fn getBoundarySize(&self) -> f64;

    // Java: abstract double getSize()
    fn getSize(&self) -> f64;

    // Java: abstract Point<S> getBarycenter()
    fn getBarycenter(&self) -> Box<dyn std::any::Any>;

    // Java: abstract Side side(final Hyperplane<S> hyperplane)
    fn side(&self) -> Box<dyn std::any::Any>;

    // Java: abstract SubHyperplane<S> intersection(final SubHyperplane<S> sub)
    fn intersection(&self) -> Box<dyn std::any::Any>;

}
