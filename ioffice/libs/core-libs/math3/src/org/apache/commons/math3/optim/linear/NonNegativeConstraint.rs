// Java: import org.apache.commons.math3.optim.OptimizationData
use crate::org::apache::commons::math3::optim::OptimizationData;

// Java: public class NonNegativeConstraint implements OptimizationData
pub struct NonNegativeConstraint {
    // Java field: boolean isRestricted
    pub(crate) isRestricted: bool,
}

// Java: public class NonNegativeConstraint implements OptimizationData
impl NonNegativeConstraint {
    // Java: NonNegativeConstraint(boolean restricted)
    pub fn NonNegativeConstraint(&self) {
        // Java: isRestricted = restricted;
    }

    // Java: boolean isRestrictedToNonNegative()
    pub fn isRestrictedToNonNegative(&self) -> bool {
        // Java: return isRestricted;
        todo!()
    }

}

