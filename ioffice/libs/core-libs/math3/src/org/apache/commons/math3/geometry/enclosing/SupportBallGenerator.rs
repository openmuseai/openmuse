// Java: import java.util.List
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public interface SupportBallGenerator extends Space
pub trait SupportBallGenerator {
    // Java: abstract EnclosingBall<S, P> ballOnSupport(List<P> support)
    fn ballOnSupport(&self) -> Box<dyn std::any::Any>;

}
