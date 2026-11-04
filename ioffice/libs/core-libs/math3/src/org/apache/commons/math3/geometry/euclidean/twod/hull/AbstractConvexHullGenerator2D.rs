#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.Collection
// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Vector2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Vector2D;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: double DEFAULT_TOLERANCE
pub(crate) const DEFAULT_TOLERANCE: f64 = 1e-10;

// Java: public class for
pub struct r#for {
}

// Java: public class AbstractConvexHullGenerator2D
pub struct AbstractConvexHullGenerator2D {
    // Java field: double tolerance
    pub(crate) tolerance: f64,
    // Java field: boolean includeCollinearPoints
    pub(crate) includeCollinearPoints: bool,
}

// Java: public class for
impl r#for {
}

// Java: public class AbstractConvexHullGenerator2D
impl AbstractConvexHullGenerator2D {
    // Java: AbstractConvexHullGenerator2D(final boolean includeCollinearPoints)
    pub fn new(includeCollinearPoints: bool) -> Self {
        // Java: this(includeCollinearPoints, DEFAULT_TOLERANCE);
        AbstractConvexHullGenerator2D {
            includeCollinearPoints,
            tolerance: DEFAULT_TOLERANCE,
        }
    }

    // Java: AbstractConvexHullGenerator2D(final boolean includeCollinearPoints, final double tolerance)
    pub fn new_with_tolerance(includeCollinearPoints: bool, tolerance: f64) -> Self {
        // Java: this.includeCollinearPoints = includeCollinearPoints;
        // Java: this.tolerance = tolerance;
        AbstractConvexHullGenerator2D {
            includeCollinearPoints,
            tolerance,
        }
    }

    // Java: double getTolerance()
    pub fn getTolerance(&self) -> f64 {
        // Java: return tolerance;
        self.tolerance
    }

    // Java: boolean isIncludeCollinearPoints()
    pub fn isIncludeCollinearPoints(&self) -> bool {
        // Java: return includeCollinearPoints;
        self.includeCollinearPoints
    }

    // Java: ConvexHull2D generate(final Collection<Vector2D> points)
    pub fn generate(&self, _points: &Vec<Box<dyn std::any::Any>>) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: abstract Collection<Vector2D> findHullVertices(Collection<Vector2D> points)
    pub fn findHullVertices(&self, _points: &Vec<Box<dyn std::any::Any>>) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

}
