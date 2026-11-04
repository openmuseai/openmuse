// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 20120201;

// Java: public class for
pub struct r#for {
}

// Java: public class RootsOfUnity implements Serializable
pub struct RootsOfUnity {
    // Java field: int omegaCount
    pub(crate) omegaCount: i32,
    // Java field: double[] omegaReal
    omegaReal: Option<Vec<f64>>,
    // Java field: double[] omegaImaginaryCounterClockwise
    omegaImaginaryCounterClockwise: Option<Vec<f64>>,
    // Java field: double[] omegaImaginaryClockwise
    omegaImaginaryClockwise: Option<Vec<f64>>,
    // Java field: boolean isCounterClockWise
    isCounterClockWise: bool,
}

// Java: public class for
impl r#for {
}

// Java: public class RootsOfUnity implements Serializable
impl RootsOfUnity {
    // Java: RootsOfUnity()
    pub fn RootsOfUnity() -> Self {
        // Java: omegaCount = 0;
        // Java: omegaReal = null;
        // Java: omegaImaginaryCounterClockwise = null;
        // Java: omegaImaginaryClockwise = null;
        // Java: isCounterClockWise = true;
        RootsOfUnity {
            omegaCount: 0,
            omegaReal: None,
            omegaImaginaryCounterClockwise: None,
            omegaImaginaryClockwise: None,
            isCounterClockWise: true,
        }
    }

    // Java: boolean isCounterClockWise()
    pub fn isCounterClockWise(&self) -> bool {
        self.isCounterClockWise
    }

    // Java: void computeRoots(int n)
    pub fn computeRoots(&mut self, n: i32) {
        // Java: if (n == 0)
        if n == 0 {
            // Java: throw new ZeroException(
                    // Java: LocalizedFormats.CANNOT_COMPUTE_0TH_ROOT_OF_UNITY);
            panic!("ZeroException: CANNOT_COMPUTE_0TH_ROOT_OF_UNITY");
        }
        // Java: isCounterClockWise = n > 0;
        self.isCounterClockWise = n > 0;
        // Java: // avoid repetitive calculations
        // Java: final int absN = FastMath.abs(n);
        let abs_n = n.unsigned_abs() as usize;
        // Java: if (absN == omegaCount)
        if abs_n as i32 == self.omegaCount {
            // Java: return;
            return;
        }
        // Java: // calculate everything from scratch
        // Java: final double t = 2.0 * FastMath.PI / absN;
        let t = 2.0 * std::f64::consts::PI / abs_n as f64;
        // Java: final double cosT = FastMath.cos(t);
        let cos_t = t.cos();
        // Java: final double sinT = FastMath.sin(t);
        let sin_t = t.sin();
        // Java: omegaReal = new double[absN];
        let mut omega_real = vec![0.0f64; abs_n];
        // Java: omegaImaginaryCounterClockwise = new double[absN];
        let mut omega_imag_ccw = vec![0.0f64; abs_n];
        // Java: omegaImaginaryClockwise = new double[absN];
        let mut omega_imag_cw = vec![0.0f64; abs_n];
        // Java: omegaReal[0] = 1.0;
        omega_real[0] = 1.0;
        // Java: omegaImaginaryCounterClockwise[0] = 0.0;
        omega_imag_ccw[0] = 0.0;
        // Java: omegaImaginaryClockwise[0] = 0.0;
        omega_imag_cw[0] = 0.0;
        // Java: for (int i = 1; i < absN; i++)
        for i in 1..abs_n {
            // Java: omegaReal[i] = omegaReal[i - 1] * cosT -
                    // Java: omegaImaginaryCounterClockwise[i - 1] * sinT;
            omega_real[i] = omega_real[i - 1] * cos_t - omega_imag_ccw[i - 1] * sin_t;
            // Java: omegaImaginaryCounterClockwise[i] = omegaReal[i - 1] * sinT +
                    // Java: omegaImaginaryCounterClockwise[i - 1] * cosT;
            omega_imag_ccw[i] = omega_real[i - 1] * sin_t + omega_imag_ccw[i - 1] * cos_t;
            // Java: omegaImaginaryClockwise[i] = -omegaImaginaryCounterClockwise[i];
            omega_imag_cw[i] = -omega_imag_ccw[i];
        }
        // Java: omegaCount = absN;
        self.omegaCount = abs_n as i32;
        self.omegaReal = Some(omega_real);
        self.omegaImaginaryCounterClockwise = Some(omega_imag_ccw);
        self.omegaImaginaryClockwise = Some(omega_imag_cw);
    }

    // Java: double getReal(int k)
    pub fn getReal(&self, k: i32) -> f64 {
        if self.omegaCount == 0 {
            panic!("MathIllegalStateException: ROOTS_OF_UNITY_NOT_COMPUTED");
        }
        let idx = ((k % self.omegaCount + self.omegaCount) % self.omegaCount) as usize;
        self.omegaReal.as_ref().unwrap()[idx]
    }

    // Java: double getImaginary(int k)
    pub fn getImaginary(&self, k: i32) -> f64 {
        if self.omegaCount == 0 {
            panic!("MathIllegalStateException: ROOTS_OF_UNITY_NOT_COMPUTED");
        }
        let idx = ((k % self.omegaCount + self.omegaCount) % self.omegaCount) as usize;
        if self.isCounterClockWise {
            self.omegaImaginaryCounterClockwise.as_ref().unwrap()[idx]
        } else {
            self.omegaImaginaryClockwise.as_ref().unwrap()[idx]
        }
    }

    // Java: int getNumberOfRoots()
    pub fn getNumberOfRoots(&self) -> i32 {
        // Java: return omegaCount;
        self.omegaCount
    }
}
