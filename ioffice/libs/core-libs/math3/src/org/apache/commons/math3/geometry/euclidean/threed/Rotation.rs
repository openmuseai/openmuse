#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments, non_upper_case_globals)]

// Java: import java.io.Serializable
// (no direct Rust equivalent)
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays::MathArrays;
use crate::org::apache::commons::math3::exception::util::LocalizedFormats::LocalizedFormats as LocalizedFormatsEnum;

// Java field: Rotation IDENTITY
static IDENTITY: std::sync::OnceLock<Rotation> = std::sync::OnceLock::new();
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -2153622329907944313;

// Java: public class implements implements rotations in a three
pub struct implements {
}

// Java: public class does
pub struct does {
}

// Java: public class provides
pub struct provides {
}

// Java: public class Rotation implements Serializable
pub struct Rotation {
    // Java field: double q0
    pub(crate) q0: f64,
    // Java field: double q1
    pub(crate) q1: f64,
    // Java field: double q2
    pub(crate) q2: f64,
    // Java field: double q3
    pub(crate) q3: f64,
}

// Java: public class implements implements rotations in a three
impl implements {
}

// Java: public class does
impl does {
}

// Java: public class provides
impl provides {
}

fn get_IDENTITY() -> &'static Rotation {
    IDENTITY.get_or_init(|| Rotation { q0: 1.0, q1: 0.0, q2: 0.0, q3: 0.0 })
}

// Java: public class Rotation implements Serializable
impl Rotation {
    // Java: Rotation()
    pub fn new_identity() -> Self {
        Rotation { q0: 1.0, q1: 0.0, q2: 0.0, q3: 0.0 }
    }

    pub fn new(q0: f64, q1: f64, q2: f64, q3: f64, needs_normalization: bool) -> Self {
        if needs_normalization {
            let inv = 1.0 / FastMath::sqrt(q0 * q0 + q1 * q1 + q2 * q2 + q3 * q3);
            Rotation {
                q0: q0 * inv,
                q1: q1 * inv,
                q2: q2 * inv,
                q3: q3 * inv,
            }
        } else {
            Rotation { q0, q1, q2, q3 }
        }
    }

    // Java: Rotation(Vector3D axis, double angle)
    pub fn new_from_axis_angle(_axis: &dyn std::any::Any, _angle: f64) -> Self {
        // Java: this(axis, angle, RotationConvention.VECTOR_OPERATOR);
        todo!()
    }

    // Java: Rotation(final Vector3D axis, final double angle, final RotationConvention convention)
    pub fn new_from_axis_angle_convention(_axis: &dyn std::any::Any, _angle: f64, _convention: &dyn std::any::Any) -> Self {
        todo!()
    }

    // Java: Rotation(double[][] m, double threshold)
    pub fn new_from_matrix(_m: &[&[f64]], _threshold: f64) -> Self {
        todo!()
    }

    // Java: Rotation(Vector3D u1, Vector3D u2, Vector3D v1, Vector3D v2)
    pub fn new_from_vectors_4(_u1: &dyn std::any::Any, _u2: &dyn std::any::Any, _v1: &dyn std::any::Any, _v2: &dyn std::any::Any) -> Self {
        todo!()
    }

    // Java: Rotation(Vector3D u, Vector3D v)
    pub fn new_from_vectors_2(_u: &dyn std::any::Any, _v: &dyn std::any::Any) -> Self {
        // Java: double normProduct = u.getNorm() * v.getNorm();
        // Java: if (normProduct == 0)
        // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NORM_FOR_ROTATION_DEFINING_VECTOR);
        // Java: double dot = u.dotProduct(v);
        // Java: if (dot < ((2.0e-15 - 1.0) * normProduct))
        // Java: // special case u = -v: we select a PI angle rotation around
        // Java: // an arbitrary vector orthogonal to u
        // Java: Vector3D w = u.orthogonal();
        // Java: q0 = 0.0;
        // Java: q1 = -w.getX();
        // Java: q2 = -w.getY();
        // Java: q3 = -w.getZ();
        // Java: else
        // Java: // general case: (u, v) defines a plane, we select
        // Java: // the shortest possible rotation: axis orthogonal to this plane
        // Java: q0 = FastMath.sqrt(0.5 * (1.0 + dot / normProduct));
        // Java: double coeff = 1.0 / (2.0 * q0 * normProduct);
        // Java: Vector3D q = v.crossProduct(u);
        // Java: q1 = coeff * q.getX();
        // Java: q2 = coeff * q.getY();
        // Java: q3 = coeff * q.getZ();
        todo!()
    }

    // Java: Rotation()
    pub fn new_default_1() -> Self {
        todo!()
    }

    // Java: Rotation()
    pub fn new_default_2() -> Self {
        todo!()
    }

    // Java: static double[] mat2quat(final double[][] ort)
    pub fn mat2quat(ort: &Vec<Vec<f64>>) -> Vec<f64> {
        // Java: final double[] quat = new double[4];
        let mut quat = vec![0.0_f64; 4];
        // Java: // There are different ways to compute the quaternions elements
        // Java: // from the matrix. They all involve computing one element from
        // Java: // the diagonal of the matrix, and computing the three other ones
        // Java: // using a formula involving a division by the first element,
        // Java: // which unfortunately can be zero. Since the norm of the
        // Java: // quaternion is 1, we know at least one element has an absolute
        // Java: // value greater or equal to 0.5, so it is always possible to
        // Java: // select the right formula and avoid division by zero and even
        // Java: // numerical inaccuracy. Checking the elements in turn and using
        // Java: // the first one greater than 0.45 is safe (this leads to a simple
        // Java: // test since qi = 0.45 implies 4 qi^2 - 1 = -0.19)
        // Java: double s = ort[0][0] + ort[1][1] + ort[2][2];
        let mut s = ort[0][0] + ort[1][1] + ort[2][2];
        // Java: if (s > -0.19)
        if s > -0.19 {
            // Java: // compute q0 and deduce q1, q2 and q3
            // Java: quat[0] = 0.5 * FastMath.sqrt(s + 1.0);
            quat[0] = 0.5 * FastMath::sqrt(s + 1.0);
            // Java: double inv = 0.25 / quat[0];
            let inv = 0.25 / quat[0];
            // Java: quat[1] = inv * (ort[1][2] - ort[2][1]);
            quat[1] = inv * (ort[1][2] - ort[2][1]);
            // Java: quat[2] = inv * (ort[2][0] - ort[0][2]);
            quat[2] = inv * (ort[2][0] - ort[0][2]);
            // Java: quat[3] = inv * (ort[0][1] - ort[1][0]);
            quat[3] = inv * (ort[0][1] - ort[1][0]);
        }
        // Java: else
        else {
            // Java: s = ort[0][0] - ort[1][1] - ort[2][2];
            s = ort[0][0] - ort[1][1] - ort[2][2];
            // Java: if (s > -0.19)
            if s > -0.19 {
                // Java: // compute q1 and deduce q0, q2 and q3
                // Java: quat[1] = 0.5 * FastMath.sqrt(s + 1.0);
                quat[1] = 0.5 * FastMath::sqrt(s + 1.0);
                // Java: double inv = 0.25 / quat[1];
                let inv = 0.25 / quat[1];
                // Java: quat[0] = inv * (ort[1][2] - ort[2][1]);
                quat[0] = inv * (ort[1][2] - ort[2][1]);
                // Java: quat[2] = inv * (ort[0][1] + ort[1][0]);
                quat[2] = inv * (ort[0][1] + ort[1][0]);
                // Java: quat[3] = inv * (ort[0][2] + ort[2][0]);
                quat[3] = inv * (ort[0][2] + ort[2][0]);
            }
            // Java: else
            else {
                // Java: s = ort[1][1] - ort[0][0] - ort[2][2];
                s = ort[1][1] - ort[0][0] - ort[2][2];
                // Java: if (s > -0.19)
                if s > -0.19 {
                    // Java: // compute q2 and deduce q0, q1 and q3
                    // Java: quat[2] = 0.5 * FastMath.sqrt(s + 1.0);
                    quat[2] = 0.5 * FastMath::sqrt(s + 1.0);
                    // Java: double inv = 0.25 / quat[2];
                    let inv = 0.25 / quat[2];
                    // Java: quat[0] = inv * (ort[2][0] - ort[0][2]);
                    quat[0] = inv * (ort[2][0] - ort[0][2]);
                    // Java: quat[1] = inv * (ort[0][1] + ort[1][0]);
                    quat[1] = inv * (ort[0][1] + ort[1][0]);
                    // Java: quat[3] = inv * (ort[2][1] + ort[1][2]);
                    quat[3] = inv * (ort[2][1] + ort[1][2]);
                }
                // Java: else
                else {
                    // Java: // compute q3 and deduce q0, q1 and q2
                    // Java: s = ort[2][2] - ort[0][0] - ort[1][1];
                    s = ort[2][2] - ort[0][0] - ort[1][1];
                    // Java: quat[3] = 0.5 * FastMath.sqrt(s + 1.0);
                    quat[3] = 0.5 * FastMath::sqrt(s + 1.0);
                    // Java: double inv = 0.25 / quat[3];
                    let inv = 0.25 / quat[3];
                    // Java: quat[0] = inv * (ort[0][1] - ort[1][0]);
                    quat[0] = inv * (ort[0][1] - ort[1][0]);
                    // Java: quat[1] = inv * (ort[0][2] + ort[2][0]);
                    quat[1] = inv * (ort[0][2] + ort[2][0]);
                    // Java: quat[2] = inv * (ort[2][1] + ort[1][2]);
                    quat[2] = inv * (ort[2][1] + ort[1][2]);
                }
            }
        }
        // Java: return quat;
        quat
    }

    // Java: Rotation revert()
    pub fn revert(&self) -> Rotation {
        // Java: return new Rotation(-q0, q1, q2, q3, false);
        Rotation::new(-self.q0, self.q1, self.q2, self.q3, false)
    }

    // Java: double getQ0()
    pub fn getQ0(&self) -> f64 {
        // Java: return q0;
        self.q0
    }

    // Java: double getQ1()
    pub fn getQ1(&self) -> f64 {
        // Java: return q1;
        self.q1
    }

    // Java: double getQ2()
    pub fn getQ2(&self) -> f64 {
        // Java: return q2;
        self.q2
    }

    // Java: double getQ3()
    pub fn getQ3(&self) -> f64 {
        // Java: return q3;
        self.q3
    }

    // Java: Vector3D getAxis()
    pub fn getAxis(&self) -> Box<dyn std::any::Any> {
        // Java: return getAxis(RotationConvention.VECTOR_OPERATOR);
        todo!()
    }

    // Java: Vector3D getAxis(final RotationConvention convention)
    pub fn getAxis_convention(&self, _convention: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final double squaredSine = q1 * q1 + q2 * q2 + q3 * q3;
        // Java: if (squaredSine == 0)
        // Java: return convention == RotationConvention.VECTOR_OPERATOR ? Vector3D.PLUS_I : Vector3D.MINUS_I;
        // Java: else
        // Java: final double sgn = convention == RotationConvention.VECTOR_OPERATOR ? +1 : -1;
        // Java: if (q0 < 0)
        // Java: final double inverse = sgn / FastMath.sqrt(squaredSine);
        // Java: return new Vector3D(q1 * inverse, q2 * inverse, q3 * inverse);
        // Java: final double inverse = -sgn / FastMath.sqrt(squaredSine);
        // Java: return new Vector3D(q1 * inverse, q2 * inverse, q3 * inverse);
        todo!()
    }

    // Java: double getAngle()
    pub fn getAngle(&self) -> f64 {
        // Java: if ((q0 < -0.1) || (q0 > 0.1))
        if (self.q0 < -0.1) || (self.q0 > 0.1) {
            // Java: return 2 * FastMath.asin(FastMath.sqrt(q1 * q1 + q2 * q2 + q3 * q3));
            return 2.0 * FastMath::asin(FastMath::sqrt(self.q1 * self.q1 + self.q2 * self.q2 + self.q3 * self.q3));
        }
        // Java: else if (q0 < 0)
        else if self.q0 < 0.0 {
            // Java: return 2 * FastMath.acos(-q0);
            return 2.0 * FastMath::acos(-self.q0);
        }
        // Java: return 2 * FastMath.acos(q0);
        2.0 * FastMath::acos(self.q0)
    }

    // Java: double[] getAngles(RotationOrder order)
    pub fn getAngles(&self, _order: &dyn std::any::Any) -> Vec<f64> {
        todo!()
    }

    // Java: double[] getAngles(RotationOrder order, RotationConvention convention)
    pub fn getAngles_convention(&self, _order: &dyn std::any::Any, _convention: &dyn std::any::Any) -> Vec<f64> {
        todo!()
    }

    // Java: double[][] getMatrix()
    pub fn getMatrix(&self) -> Vec<Vec<f64>> {
        // Java: // products
        // Java: double q0q0  = q0 * q0;
        let q0q0 = self.q0 * self.q0;
        // Java: double q0q1  = q0 * q1;
        let q0q1 = self.q0 * self.q1;
        // Java: double q0q2  = q0 * q2;
        let q0q2 = self.q0 * self.q2;
        // Java: double q0q3  = q0 * q3;
        let q0q3 = self.q0 * self.q3;
        // Java: double q1q1  = q1 * q1;
        let q1q1 = self.q1 * self.q1;
        // Java: double q1q2  = q1 * q2;
        let q1q2 = self.q1 * self.q2;
        // Java: double q1q3  = q1 * q3;
        let q1q3 = self.q1 * self.q3;
        // Java: double q2q2  = q2 * q2;
        let q2q2 = self.q2 * self.q2;
        // Java: double q2q3  = q2 * q3;
        let q2q3 = self.q2 * self.q3;
        // Java: double q3q3  = q3 * q3;
        let q3q3 = self.q3 * self.q3;
        // Java: // create the matrix
        // Java: double[][] m = new double[3][];
        // Java: m[0] = new double[3];
        // Java: m[1] = new double[3];
        // Java: m[2] = new double[3];
        let mut m = vec![vec![0.0_f64; 3]; 3];
        // Java: m [0][0] = 2.0 * (q0q0 + q1q1) - 1.0;
        m[0][0] = 2.0 * (q0q0 + q1q1) - 1.0;
        // Java: m [1][0] = 2.0 * (q1q2 - q0q3);
        m[1][0] = 2.0 * (q1q2 - q0q3);
        // Java: m [2][0] = 2.0 * (q1q3 + q0q2);
        m[2][0] = 2.0 * (q1q3 + q0q2);
        // Java: m [0][1] = 2.0 * (q1q2 + q0q3);
        m[0][1] = 2.0 * (q1q2 + q0q3);
        // Java: m [1][1] = 2.0 * (q0q0 + q2q2) - 1.0;
        m[1][1] = 2.0 * (q0q0 + q2q2) - 1.0;
        // Java: m [2][1] = 2.0 * (q2q3 - q0q1);
        m[2][1] = 2.0 * (q2q3 - q0q1);
        // Java: m [0][2] = 2.0 * (q1q3 - q0q2);
        m[0][2] = 2.0 * (q1q3 - q0q2);
        // Java: m [1][2] = 2.0 * (q2q3 + q0q1);
        m[1][2] = 2.0 * (q2q3 + q0q1);
        // Java: m [2][2] = 2.0 * (q0q0 + q3q3) - 1.0;
        m[2][2] = 2.0 * (q0q0 + q3q3) - 1.0;
        // Java: return m;
        m
    }

    // Java: Vector3D applyTo(Vector3D u)
    pub fn applyTo_vec(&self, _u: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: double x = u.getX();
        // Java: double y = u.getY();
        // Java: double z = u.getZ();
        // Java: double s = q1 * x + q2 * y + q3 * z;
        // Java: return new Vector3D(2 * (q0 * (x * q0 - (q2 * z - q3 * y)) + s * q1) - x,
        // Java: 2 * (q0 * (y * q0 - (q3 * x - q1 * z)) + s * q2) - y,
        // Java: 2 * (q0 * (z * q0 - (q1 * y - q2 * x)) + s * q3) - z);
        todo!()
    }

    // Java: void applyTo(final double[] in, final double[] out)
    pub fn applyTo_arr(&self, input: &[f64], out: &mut [f64]) {
        // Java: final double x = in[0];
        let x = input[0];
        // Java: final double y = in[1];
        let y = input[1];
        // Java: final double z = in[2];
        let z = input[2];
        // Java: final double s = q1 * x + q2 * y + q3 * z;
        let s = self.q1 * x + self.q2 * y + self.q3 * z;
        // Java: out[0] = 2 * (q0 * (x * q0 - (q2 * z - q3 * y)) + s * q1) - x;
        out[0] = 2.0 * (self.q0 * (x * self.q0 - (self.q2 * z - self.q3 * y)) + s * self.q1) - x;
        // Java: out[1] = 2 * (q0 * (y * q0 - (q3 * x - q1 * z)) + s * q2) - y;
        out[1] = 2.0 * (self.q0 * (y * self.q0 - (self.q3 * x - self.q1 * z)) + s * self.q2) - y;
        // Java: out[2] = 2 * (q0 * (z * q0 - (q1 * y - q2 * x)) + s * q3) - z;
        out[2] = 2.0 * (self.q0 * (z * self.q0 - (self.q1 * y - self.q2 * x)) + s * self.q3) - z;
    }

    // Java: Vector3D applyInverseTo(Vector3D u)
    pub fn applyInverseTo_vec(&self, _u: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: double x = u.getX();
        // Java: double y = u.getY();
        // Java: double z = u.getZ();
        // Java: double s = q1 * x + q2 * y + q3 * z;
        // Java: double m0 = -q0;
        // Java: return new Vector3D(2 * (m0 * (x * m0 - (q2 * z - q3 * y)) + s * q1) - x,
        // Java: 2 * (m0 * (y * m0 - (q3 * x - q1 * z)) + s * q2) - y,
        // Java: 2 * (m0 * (z * m0 - (q1 * y - q2 * x)) + s * q3) - z);
        todo!()
    }

    // Java: void applyInverseTo(final double[] in, final double[] out)
    pub fn applyInverseTo_arr(&self, input: &[f64], out: &mut [f64]) {
        // Java: final double x = in[0];
        let x = input[0];
        // Java: final double y = in[1];
        let y = input[1];
        // Java: final double z = in[2];
        let z = input[2];
        // Java: final double s = q1 * x + q2 * y + q3 * z;
        let s = self.q1 * x + self.q2 * y + self.q3 * z;
        // Java: final double m0 = -q0;
        let m0 = -self.q0;
        // Java: out[0] = 2 * (m0 * (x * m0 - (q2 * z - q3 * y)) + s * q1) - x;
        out[0] = 2.0 * (m0 * (x * m0 - (self.q2 * z - self.q3 * y)) + s * self.q1) - x;
        // Java: out[1] = 2 * (m0 * (y * m0 - (q3 * x - q1 * z)) + s * q2) - y;
        out[1] = 2.0 * (m0 * (y * m0 - (self.q3 * x - self.q1 * z)) + s * self.q2) - y;
        // Java: out[2] = 2 * (m0 * (z * m0 - (q1 * y - q2 * x)) + s * q3) - z;
        out[2] = 2.0 * (m0 * (z * m0 - (self.q1 * y - self.q2 * x)) + s * self.q3) - z;
    }

    // Java: Rotation applyTo(Rotation r)
    pub fn applyTo_rotation(&self, r: &Rotation) -> Rotation {
        // Java: return compose(r, RotationConvention.VECTOR_OPERATOR);
        self.composeInternal(r)
    }

    // Java: Rotation compose(final Rotation r, final RotationConvention convention)
    pub fn compose(&self, r: &Rotation, _convention: &dyn std::any::Any) -> Rotation {
        // Java: return convention == RotationConvention.VECTOR_OPERATOR ?
        // Java: composeInternal(r) : r.composeInternal(this);
        self.composeInternal(r)
    }

    // Java: Rotation composeInternal(final Rotation r)
    pub fn composeInternal(&self, r: &Rotation) -> Rotation {
        // Java: return new Rotation(r.q0 * q0 - (r.q1 * q1 + r.q2 * q2 + r.q3 * q3),
        // Java: r.q1 * q0 + r.q0 * q1 + (r.q2 * q3 - r.q3 * q2),
        // Java: r.q2 * q0 + r.q0 * q2 + (r.q3 * q1 - r.q1 * q3),
        // Java: r.q3 * q0 + r.q0 * q3 + (r.q1 * q2 - r.q2 * q1),
        // Java: false);
        Rotation::new(
            r.q0 * self.q0 - (r.q1 * self.q1 + r.q2 * self.q2 + r.q3 * self.q3),
            r.q1 * self.q0 + r.q0 * self.q1 + (r.q2 * self.q3 - r.q3 * self.q2),
            r.q2 * self.q0 + r.q0 * self.q2 + (r.q3 * self.q1 - r.q1 * self.q3),
            r.q3 * self.q0 + r.q0 * self.q3 + (r.q1 * self.q2 - r.q2 * self.q1),
            false,
        )
    }

    // Java: Rotation applyInverseTo(Rotation r)
    pub fn applyInverseTo_rotation(&self, r: &Rotation) -> Rotation {
        // Java: return composeInverse(r, RotationConvention.VECTOR_OPERATOR);
        self.composeInverseInternal(r)
    }

    // Java: Rotation composeInverse(final Rotation r, final RotationConvention convention)
    pub fn composeInverse(&self, r: &Rotation, _convention: &dyn std::any::Any) -> Rotation {
        // Java: return convention == RotationConvention.VECTOR_OPERATOR ?
        // Java: composeInverseInternal(r) : r.composeInternal(revert());
        self.composeInverseInternal(r)
    }

    // Java: Rotation composeInverseInternal(Rotation r)
    pub fn composeInverseInternal(&self, r: &Rotation) -> Rotation {
        // Java: return new Rotation(-r.q0 * q0 - (r.q1 * q1 + r.q2 * q2 + r.q3 * q3),
        // Java: -r.q1 * q0 + r.q0 * q1 + (r.q2 * q3 - r.q3 * q2),
        // Java: -r.q2 * q0 + r.q0 * q2 + (r.q3 * q1 - r.q1 * q3),
        // Java: -r.q3 * q0 + r.q0 * q3 + (r.q1 * q2 - r.q2 * q1),
        // Java: false);
        Rotation::new(
            -r.q0 * self.q0 - (r.q1 * self.q1 + r.q2 * self.q2 + r.q3 * self.q3),
            -r.q1 * self.q0 + r.q0 * self.q1 + (r.q2 * self.q3 - r.q3 * self.q2),
            -r.q2 * self.q0 + r.q0 * self.q2 + (r.q3 * self.q1 - r.q1 * self.q3),
            -r.q3 * self.q0 + r.q0 * self.q3 + (r.q1 * self.q2 - r.q2 * self.q1),
            false,
        )
    }

    // Java: double[][] orthogonalizeMatrix(double[][] m, double threshold)
    pub fn orthogonalizeMatrix(_m: &Vec<Vec<f64>>, _threshold: f64) -> Vec<Vec<f64>> {
        todo!()
    }

    // Java: static double distance(Rotation r1, Rotation r2)
    pub fn distance(r1: &Rotation, r2: &Rotation) -> f64 {
        // Java: return r1.composeInverseInternal(r2).getAngle();
        r1.composeInverseInternal(r2).getAngle()
    }

}
