#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.geometry.euclidean.threed.Vector3D
use crate::org::apache::commons::math3::geometry::euclidean::threed::Vector3D;
// Java: import org.apache.commons.math3.geometry.partitioning.AbstractSubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::AbstractSubHyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.Hyperplane
use crate::org::apache::commons::math3::geometry::partitioning::Hyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.Region
use crate::org::apache::commons::math3::geometry::partitioning::Region;
// Java: import org.apache.commons.math3.geometry.spherical.oned.Arc
use crate::org::apache::commons::math3::geometry::spherical::oned::Arc;
// Java: import org.apache.commons.math3.geometry.spherical.oned.ArcsSet
use crate::org::apache::commons::math3::geometry::spherical::oned::ArcsSet;
// Java: import org.apache.commons.math3.geometry.spherical.oned.Sphere1D
use crate::org::apache::commons::math3::geometry::spherical::oned::Sphere1D;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;

// Java: public class represents
pub struct represents {
}

// Java: public class SubCircle extends AbstractSubHyperplane
pub struct SubCircle {
}

// Java: public class represents
impl represents {
}

// Java: public class SubCircle extends AbstractSubHyperplane
impl SubCircle {
    // Java: SubCircle()
    pub fn new() -> Self {
        SubCircle {}
    }

    // Java: protected AbstractSubHyperplane<Sphere2D, Sphere1D> buildNew()
    pub fn buildNew(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: SplitSubHyperplane<Sphere2D> split(final Hyperplane<Sphere2D> hyperplane)
    pub fn split(&self, _hyperplane: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final Circle thisCircle   = (Circle) getHyperplane();
        // Java: final Circle otherCircle  = (Circle) hyperplane;
        // Java: final double angle = Vector3D.angle(thisCircle.getPole(), otherCircle.getPole());
        // Java: if (angle < thisCircle.getTolerance() || angle > FastMath.PI - thisCircle.getTolerance())
        // Java: // the two circles are aligned or opposite
        // Java: return new SplitSubHyperplane<Sphere2D>(null, null);
        // Java: else
        // Java: // the two circles intersect each other
        // Java: final Arc    arc          = thisCircle.getInsideArc(otherCircle);
        // Java: final ArcsSet.Split split = ((ArcsSet) getRemainingRegion()).split(arc);
        // Java: final ArcsSet plus        = split.getPlus();
        // Java: final ArcsSet minus       = split.getMinus();
        // Java: return new SplitSubHyperplane<Sphere2D>(plus  == null ? null : new SubCircle(thisCircle.copySelf(), plus),
        // Java: minus == null ? null : new SubCircle(thisCircle.copySelf(), minus));
        todo!()
    }

}
