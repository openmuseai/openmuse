// Java: import java.util.Collection
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.optimization.GoalType
use crate::org::apache::commons::math3::optimization::GoalType;
// Java: import org.apache.commons.math3.optimization.PointValuePair
use crate::org::apache::commons::math3::optimization::PointValuePair;

// Java: public interface represents
pub trait represents {
}

// Java: public interface LinearOptimizer
pub trait LinearOptimizer {
    // Java: abstract void setMaxIterations(int maxIterations)
    fn setMaxIterations(&self);

    // Java: abstract int getMaxIterations()
    fn getMaxIterations(&self) -> i32;

    // Java: abstract int getIterations()
    fn getIterations(&self) -> i32;

    // Java: abstract PointValuePair optimize()
    fn optimize(&self) -> Box<dyn std::any::Any>;

}
