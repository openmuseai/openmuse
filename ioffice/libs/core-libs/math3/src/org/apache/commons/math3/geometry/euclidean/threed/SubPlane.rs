#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Euclidean1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Euclidean1D;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Vector1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Vector1D;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Euclidean2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Euclidean2D;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.PolygonsSet
use crate::org::apache::commons::math3::geometry::euclidean::twod::PolygonsSet;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Vector2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Vector2D;
// Java: import org.apache.commons.math3.geometry.partitioning.AbstractSubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::AbstractSubHyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTree
use crate::org::apache::commons::math3::geometry::partitioning::BSPTree;
// Java: import org.apache.commons.math3.geometry.partitioning.Hyperplane
use crate::org::apache::commons::math3::geometry::partitioning::Hyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.Region
use crate::org::apache::commons::math3::geometry::partitioning::Region;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane;

// Java: public class represents
pub struct represents {
}

// Java: public class SubPlane extends AbstractSubHyperplane
pub struct SubPlane {
}

// Java: public class represents
impl represents {
}

// Java: public class SubPlane extends AbstractSubHyperplane
impl SubPlane {
    // Java: SubPlane()
    pub fn new() -> Self {
        SubPlane {}
    }

    // Java: protected AbstractSubHyperplane<Euclidean3D, Euclidean2D> buildNew()
    pub fn buildNew(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: SplitSubHyperplane<Euclidean3D> split(Hyperplane<Euclidean3D> hyperplane)
    pub fn split(&self, _hyperplane: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final Plane otherPlane = (Plane) hyperplane;
        // Java: final Plane thisPlane  = (Plane) getHyperplane();
        // Java: final Line  inter      = otherPlane.intersection(thisPlane);
        // Java: final double tolerance = thisPlane.getTolerance();
        // Java: if (inter == null)
        // Java: // the hyperplanes are parallel
        // Java: final double global = otherPlane.getOffset(thisPlane);
        // Java: if (global < -tolerance)
        // Java: return new SplitSubHyperplane<Euclidean3D>(null, this);
        // Java: else if (global > tolerance)
        // Java: return new SplitSubHyperplane<Euclidean3D>(this, null);
        // Java: else
        // Java: return new SplitSubHyperplane<Euclidean3D>(null, null);
        // Java: // the hyperplanes do intersect
        // Java: Vector2D p = thisPlane.toSubSpace((Point<Euclidean3D>) inter.toSpace((Point<Euclidean1D>) Vector1D.ZERO));
        // Java: Vector2D q = thisPlane.toSubSpace((Point<Euclidean3D>) inter.toSpace((Point<Euclidean1D>) Vector1D.ONE));
        // Java: Vector3D crossP = Vector3D.crossProduct(inter.getDirection(), thisPlane.getNormal());
        // Java: if (crossP.dotProduct(otherPlane.getNormal()) < 0)
        // Java: final Vector2D tmp = p;
        // Java: p           = q;
        // Java: q           = tmp;
        // Java: final SubHyperplane<Euclidean2D> l2DMinus =
        // Java: new org.apache.commons.math3.geometry.euclidean.twod.Line(p, q, tolerance).wholeHyperplane();
        // Java: final SubHyperplane<Euclidean2D> l2DPlus =
        // Java: new org.apache.commons.math3.geometry.euclidean.twod.Line(q, p, tolerance).wholeHyperplane();
        // Java: final BSPTree<Euclidean2D> splitTree = getRemainingRegion().getTree(false).split(l2DMinus);
        // Java: final BSPTree<Euclidean2D> plusTree  = getRemainingRegion().isEmpty(splitTree.getPlus()) ?
        // Java: new BSPTree<Euclidean2D>(Boolean.FALSE) :
        // Java: new BSPTree<Euclidean2D>(l2DPlus, new BSPTree<Euclidean2D>(Boolean.FALSE),
        // Java: splitTree.getPlus(), null);
        // Java: final BSPTree<Euclidean2D> minusTree = getRemainingRegion().isEmpty(splitTree.getMinus()) ?
        // Java: new BSPTree<Euclidean2D>(Boolean.FALSE) :
        // Java: new BSPTree<Euclidean2D>(l2DMinus, new BSPTree<Euclidean2D>(Boolean.FALSE),
        // Java: splitTree.getMinus(), null);
        // Java: return new SplitSubHyperplane<Euclidean3D>(new SubPlane(thisPlane.copySelf(), new PolygonsSet(plusTree, tolerance)),
        // Java: new SubPlane(thisPlane.copySelf(), new PolygonsSet(minusTree, tolerance)));
        todo!()
    }

}
