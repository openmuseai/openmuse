// Java: import java.util.Collection
// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public interface ConvexHullGenerator extends Space
pub trait ConvexHullGenerator {
    // Java: abstract ConvexHull<S, P> generate(Collection<P> points)
    fn generate(&self) -> Box<dyn std::any::Any>;

}
