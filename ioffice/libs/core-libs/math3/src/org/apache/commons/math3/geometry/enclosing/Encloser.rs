// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public interface Encloser extends Space
pub trait Encloser {
    // Java: abstract EnclosingBall<S, P> enclose(Iterable<P> points)
    fn enclose(&self) -> Box<dyn std::any::Any>;

}
