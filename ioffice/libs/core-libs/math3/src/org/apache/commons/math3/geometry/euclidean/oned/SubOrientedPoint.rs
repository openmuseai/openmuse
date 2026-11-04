// Java: import org.apache.commons.math3.geometry.partitioning.AbstractSubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::AbstractSubHyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.Hyperplane
use crate::org::apache::commons::math3::geometry::partitioning::Hyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.Region
use crate::org::apache::commons::math3::geometry::partitioning::Region;

// Java: public class represents
// Java: public class are

// Java: public class SubOrientedPoint
pub struct SubOrientedPoint {
    // Java: super(hyperplane, remainingRegion);
    pub hyperplane: Box<dyn std::any::Any>,
    pub remainingRegion: Box<dyn std::any::Any>,
}

// Java: public class SubOrientedPoint
#[allow(non_snake_case)]
impl SubOrientedPoint {
    // Java: SubOrientedPoint()
    pub fn new(hyperplane: Box<dyn std::any::Any>, remainingRegion: Box<dyn std::any::Any>) -> Self {
        // Java: super(hyperplane, remainingRegion);
        SubOrientedPoint { hyperplane, remainingRegion }
    }

    // Java: double getSize()
    pub fn getSize(&self) -> f64 {
        // Java: return 0;
        0.0
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: protected AbstractSubHyperplane<Euclidean1D, Euclidean1D> buildNew()
    pub fn buildNew(&self, hyperplane: Box<dyn std::any::Any>, remainingRegion: Box<dyn std::any::Any>) -> SubOrientedPoint {
        // Java: return new SubOrientedPoint(hyperplane, remainingRegion);
        SubOrientedPoint::new(hyperplane, remainingRegion)
    }

    // Java: SplitSubHyperplane<Euclidean1D> split(final Hyperplane<Euclidean1D> hyperplane)
    pub fn split(&self, _hyperplane: Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
        // Java: final double global = hyperplane.getOffset(((OrientedPoint) getHyperplane()).getLocation());
        // Java: if (global < -1.0e-10)
        // Java: return new SplitSubHyperplane<Euclidean1D>(null, this);
        // Java: else if (global > 1.0e-10)
        // Java: return new SplitSubHyperplane<Euclidean1D>(this, null);
        // Java: else
        // Java: return new SplitSubHyperplane<Euclidean1D>(null, null);
        todo!()
    }
}
