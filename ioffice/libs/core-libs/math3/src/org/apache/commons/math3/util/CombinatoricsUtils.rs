// Java: import java.util.Iterator
// Java: import java.util.concurrent.atomic.AtomicReference
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: AtomicReference<long[][]> STIRLING_S2
static STIRLING_S2: std::sync::OnceLock<std::sync::Mutex<Vec<Vec<i64>>>> = std::sync::OnceLock::new();

// Java: public class CombinatoricsUtils
pub struct CombinatoricsUtils {
}

// Java: public class CombinatoricsUtils
impl CombinatoricsUtils {
    // Java: CombinatoricsUtils()
    pub fn CombinatoricsUtils(&self) {
    }

    // Java: static long binomialCoefficient(final int n, final int k)
    pub fn binomialCoefficient(&self, n: i32, k: i32) -> i64 {
        todo!();
    }

    // Java: static double binomialCoefficientDouble(final int n, final int k)
    pub fn binomialCoefficientDouble(&self, n: i32, k: i32) -> f64 {
        todo!();
    }

    // Java: static double binomialCoefficientLog(final int n, final int k)
    pub fn binomialCoefficientLog(&self, n: i32, k: i32) -> f64 {
        todo!();
    }

}
