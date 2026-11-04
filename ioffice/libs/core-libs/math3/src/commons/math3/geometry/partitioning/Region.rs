// Java: import org.apache.commons.math3.geometry.Space
use math3::commons::math3::geometry::Space;
// Java: import org.apache.commons.math3.geometry.Point
use math3::commons::math3::geometry::Point;

// Java: public interface represents
pub trait represents {
}

// Java: public interface is
pub trait is {
}

// Java: public interface Region
pub trait Region {
    // Java: abstract Region<S> buildNew(BSPTree<S> newTree)
    fn buildNew(&self, BSPTree<S> newTree) -> Region<S>;

    // Java: abstract Region<S> copySelf()
    fn copySelf(&self) -> Region<S>;

    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // Java: abstract boolean isEmpty(final BSPTree<S> node)
    fn isEmpty(&self, final BSPTree<S> node) -> bool;

    // Java: abstract boolean isFull()
    fn isFull(&self) -> bool;

    // Java: abstract boolean isFull(final BSPTree<S> node)
    fn isFull(&self, final BSPTree<S> node) -> bool;

    // Java: abstract boolean contains(final Region<S> region)
    fn contains(&self, final Region<S> region) -> bool;

    // Java: abstract Location checkPoint(final Point<S> point)
    fn checkPoint(&self, final Point<S> point) -> Location;

    // Java: abstract BoundaryProjection<S> projectToBoundary(final Point<S> point)
    fn projectToBoundary(&self, final Point<S> point) -> BoundaryProjection<S>;

    // Java: abstract BSPTree<S> getTree(final boolean includeBoundaryAttributes)
    fn getTree(&self, final boolean includeBoundaryAttributes) -> BSPTree<S>;

    // Java: abstract double getBoundarySize()
    fn getBoundarySize(&self) -> f64;

    // Java: abstract double getSize()
    fn getSize(&self) -> f64;

    // Java: abstract Point<S> getBarycenter()
    fn getBarycenter(&self) -> Point<S>;

    // Java: abstract Side side(final Hyperplane<S> hyperplane)
    fn side(&self, final Hyperplane<S> hyperplane) -> Side;

    // Java: abstract SubHyperplane<S> intersection(final SubHyperplane<S> sub)
    fn intersection(&self, final SubHyperplane<S> sub) -> SubHyperplane<S>;

}
