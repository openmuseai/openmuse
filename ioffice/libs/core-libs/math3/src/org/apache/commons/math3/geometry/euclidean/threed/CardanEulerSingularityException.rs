#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments, non_upper_case_globals)]

// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class represents
pub struct represents {
}

// Java: public class CardanEulerSingularityException
pub struct CardanEulerSingularityException {
    pub(crate) message: String,
}

// Java: public class represents
impl represents {
}

// Java: public class CardanEulerSingularityException
impl CardanEulerSingularityException {
    // Java: CardanEulerSingularityException(boolean isCardan)
    pub fn new(isCardan: bool) -> Self {
        // Java: super(isCardan ? LocalizedFormats.CARDAN_ANGLES_SINGULARITY : LocalizedFormats.EULER_ANGLES_SINGULARITY);
        let message = if isCardan {
            String::from("Cardan angles singularity")
        } else {
            String::from("Euler angles singularity")
        };
        CardanEulerSingularityException { message }
    }
}
