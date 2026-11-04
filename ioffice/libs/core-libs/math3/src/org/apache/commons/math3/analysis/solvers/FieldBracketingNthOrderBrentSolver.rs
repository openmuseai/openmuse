// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.analysis.RealFieldUnivariateFunction
use crate::org::apache::commons::math3::analysis::RealFieldUnivariateFunction;
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.NoBracketingException
use crate::org::apache::commons::math3::exception::NoBracketingException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.util.IntegerSequence
use crate::org::apache::commons::math3::util::IntegerSequence;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

use crate::org::apache::commons::math3::analysis::solvers::AllowedSolution::AllowedSolution;

// Java: public class implements implements a modification of the
pub struct implements {
}

// Java: public class FieldBracketingNthOrderBrentSolver extends RealFieldElement
pub struct FieldBracketingNthOrderBrentSolver {
    pub(crate) maximal_order: i32,
    pub(crate) max_evaluations: i32,
    pub(crate) evaluations: i32,
    pub(crate) allowed: AllowedSolution,
}

// Java: public class implements implements a modification of the
impl implements {
}

// Java: public class FieldBracketingNthOrderBrentSolver extends RealFieldElement
impl FieldBracketingNthOrderBrentSolver {
    pub fn new(maximal_order: i32) -> Self {
        FieldBracketingNthOrderBrentSolver {
            maximal_order,
            max_evaluations: i32::MAX,
            evaluations: 0,
            allowed: AllowedSolution::ANY_SIDE,
        }
    }
}
