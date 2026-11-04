// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public class is
pub struct is {
}

// Java: public class BoundaryProjection
pub struct BoundaryProjection {
    // Java field: Point<S> original
    pub(crate) original: Box<dyn std::any::Any>,
    // Java field: Point<S> projected
    pub(crate) projected: Box<dyn std::any::Any>,
    // Java field: double offset
    pub(crate) offset: f64,
}

// Java: public class is
impl is {
}

// Java: public class BoundaryProjection
impl BoundaryProjection {
    // Java: BoundaryProjection(final Point<S> original, final Point<S> projected, final double offset)
    pub fn BoundaryProjection(&self) {
        // Java: this.original  = original;
        // Java: this.projected = projected;
        // Java: this.offset    = offset;
    }

    // Java: Point<S> getOriginal()
    pub fn getOriginal(&self) -> Box<dyn std::any::Any> {
        // Java: return original;
        todo!()
    }

    // Java: Point<S> getProjected()
    pub fn getProjected(&self) -> Box<dyn std::any::Any> {
        // Java: return projected;
        todo!()
    }

    // Java: double getOffset()
    pub fn getOffset(&self) -> f64 {
        // Java: return offset;
        todo!()
    }

}
