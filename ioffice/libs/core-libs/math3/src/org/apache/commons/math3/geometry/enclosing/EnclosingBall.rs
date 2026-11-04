// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java field: long serialVersionUID

// Java: public class represents
pub struct represents {
}

// Java: public class EnclosingBall
pub struct EnclosingBall {
    // Java field: P center
    pub(crate) center: Box<dyn std::any::Any>,
    // Java field: double radius
    pub(crate) radius: f64,
    // Java field: P[] support
    pub(crate) support: Box<dyn std::any::Any>,
}

// Java: public class represents
impl represents {
}

// Java: public class EnclosingBall
impl EnclosingBall {
    // Java: EnclosingBall(final P center, final double radius, final P ... support)
    pub fn EnclosingBall(&self) {
        // Java: this.center  = center;
        // Java: this.radius  = radius;
        // Java: this.support = support.clone();
    }

    // Java: P getCenter()
    pub fn getCenter(&self) -> Box<dyn std::any::Any> {
        // Java: return center;
        todo!()
    }

    // Java: double getRadius()
    pub fn getRadius(&self) -> f64 {
        // Java: return radius;
        todo!()
    }

    // Java: P[] getSupport()
    pub fn getSupport(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: return support.clone();
        todo!()
    }

    // Java: int getSupportSize()
    pub fn getSupportSize(&self) -> i32 {
        // Java: return support.length;
        todo!()
    }

    // Java: boolean contains(final P point)
    pub fn contains(&self) -> bool {
        // Java: return point.distance(center) <= radius;
        todo!()
    }

    // Java: boolean contains(final P point, final double margin)
    pub fn contains_2(&self) -> bool {
        // Java: return point.distance(center) <= radius + margin;
        todo!()
    }

}
