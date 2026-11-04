#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.Collection
// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Euclidean2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Euclidean2D;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Vector2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Vector2D;
// Java: import org.apache.commons.math3.geometry.hull.ConvexHullGenerator
use crate::org::apache::commons::math3::geometry::hull::ConvexHullGenerator;

use std::any::Any;

// Java: public interface ConvexHullGenerator2D extends ConvexHullGenerator
pub trait ConvexHullGenerator2D {
    // Java: abstract ConvexHull2D generate(Collection<Vector2D> points)
    fn generate(&self, points: &[Box<dyn Any>]) -> Box<dyn Any>;
}
