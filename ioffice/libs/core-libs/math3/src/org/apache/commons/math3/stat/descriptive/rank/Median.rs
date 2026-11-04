#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.stat.ranking.NaNStrategy
use crate::org::apache::commons::math3::stat::ranking::NaNStrategy;
// Java: import org.apache.commons.math3.util.KthSelector
use crate::org::apache::commons::math3::util::KthSelector;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -3961083514872888997i64;
// Java field: double FIXED_QUANTILE_50
pub(crate) const FIXED_QUANTILE_50: f64 = 50.0;

// Java: public class concurrently
pub struct concurrently {
}

// Java: public class Median
pub struct Median {
}

// Java: public class concurrently
impl concurrently {
}

// Java: public class Median
impl Median {
    // Java: Median()
    pub fn new() -> Self {
        // Java: // No try-catch or advertised exception - arg is valid
        // Java: super(FIXED_QUANTILE_50);
        Median {}
    }

    // Java: Median(Median original)
    pub fn new_copy(_original: &Median) -> Self {
        // Java: super(original);
        Median {}
    }

    // Java: Median()
    pub fn new_with_params(_estimationType: &dyn std::any::Any, _nanStrategy: &dyn std::any::Any, _kthSelector: &dyn std::any::Any) -> Self {
        Median {}
    }

    // Java: Median withEstimationType(final EstimationType newEstimationType)
    pub fn withEstimationType(&self, _newEstimationType: &dyn std::any::Any) -> Median {
        // Java: return new Median(newEstimationType, getNaNStrategy(), getKthSelector());
        todo!()
    }

    // Java: Median withNaNStrategy(final NaNStrategy newNaNStrategy)
    pub fn withNaNStrategy(&self, _newNaNStrategy: &dyn std::any::Any) -> Median {
        // Java: return new Median(getEstimationType(), newNaNStrategy, getKthSelector());
        todo!()
    }

    // Java: Median withKthSelector(final KthSelector newKthSelector)
    pub fn withKthSelector(&self, _newKthSelector: &dyn std::any::Any) -> Median {
        // Java: return new Median(getEstimationType(), getNaNStrategy(), newKthSelector);
        todo!()
    }

}
