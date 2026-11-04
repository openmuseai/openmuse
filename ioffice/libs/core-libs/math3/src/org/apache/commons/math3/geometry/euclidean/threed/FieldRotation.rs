#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments, non_upper_case_globals)]

// Java: import java.io.Serializable
// (no direct Rust equivalent)
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 20130224;

// Java: public class is
pub struct is {
}

// Java: public class are
pub struct are {
}

// Java: public class FieldRotation
pub struct FieldRotation {
    // Java field: T q0
    pub(crate) q0: f64,
    // Java field: T q1
    pub(crate) q1: f64,
    // Java field: T q2
    pub(crate) q2: f64,
    // Java field: T q3
    pub(crate) q3: f64,
}

// Java: public class is
impl is {
}

// Java: public class are
impl are {
}

// Java: public class FieldRotation
impl FieldRotation {
    // Java: FieldRotation(final T q0, final T q1, final T q2, final T q3, final boolean needsNormalization)
    pub fn new(q0: f64, q1: f64, q2: f64, q3: f64, needsNormalization: bool) -> Self {
        // Java: if (needsNormalization)
        if needsNormalization
        {
            // Java: // normalization preprocessing
            // Java: final T inv =
            // Java: q0.multiply(q0).add(q1.multiply(q1)).add(q2.multiply(q2)).add(q3.multiply(q3)).sqrt().reciprocal();
            let inv = 1.0 / (q0 * q0 + q1 * q1 + q2 * q2 + q3 * q3).sqrt();
            // Java: this.q0 = inv.multiply(q0);
            // Java: this.q1 = inv.multiply(q1);
            // Java: this.q2 = inv.multiply(q2);
            // Java: this.q3 = inv.multiply(q3);
            FieldRotation { q0: inv * q0, q1: inv * q1, q2: inv * q2, q3: inv * q3 }
        }
        // Java: else
        else
        {
            // Java: this.q0 = q0;
            // Java: this.q1 = q1;
            // Java: this.q2 = q2;
            // Java: this.q3 = q3;
            FieldRotation { q0, q1, q2, q3 }
        }
    }

    // Java: FieldRotation(final FieldVector3D<T> axis, final T angle)
    pub fn new_axis_angle(_axis: &dyn std::any::Any, _angle: f64) -> Self {
        todo!()
    }

    // Java: FieldRotation(final FieldVector3D<T> axis, final T angle, final RotationConvention convention)
    pub fn new_axis_angle_convention(_axis: &dyn std::any::Any, _angle: f64, _convention: &dyn std::any::Any) -> Self {
        todo!()
    }

    // Java: FieldRotation(final T[][] m, final double threshold)
    pub fn new_from_matrix(_m: &dyn std::any::Any, _threshold: f64) -> Self {
        todo!()
    }

    // Java: FieldRotation(FieldVector3D<T> u1, FieldVector3D<T> u2, FieldVector3D<T> v1, FieldVector3D<T> v2)
    pub fn new_from_vectors(_u1: &dyn std::any::Any, _u2: &dyn std::any::Any, _v1: &dyn std::any::Any, _v2: &dyn std::any::Any) -> Self {
        todo!()
    }

    // Java: FieldRotation(final FieldVector3D<T> u, final FieldVector3D<T> v)
    pub fn new_from_pair(_u: &dyn std::any::Any, _v: &dyn std::any::Any) -> Self {
        // Java: final T normProduct = u.getNorm().multiply(v.getNorm());
        // Java: if (normProduct.getReal() == 0)
        // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NORM_FOR_ROTATION_DEFINING_VECTOR);
        // Java: final T dot = FieldVector3D.dotProduct(u, v);
        // Java: if (dot.getReal() < ((2.0e-15 - 1.0) * normProduct.getReal()))
        // Java: // special case u = -v: we select a PI angle rotation around
        // Java: // an arbitrary vector orthogonal to u
        // Java: final FieldVector3D<T> w = u.orthogonal();
        // Java: q0 = normProduct.getField().getZero();
        // Java: q1 = w.getX().negate();
        // Java: q2 = w.getY().negate();
        // Java: q3 = w.getZ().negate();
        // Java: else
        // Java: // general case: (u, v) defines a plane, we select
        // Java: // the shortest possible rotation: axis orthogonal to this plane
        // Java: q0 = dot.divide(normProduct).add(1.0).multiply(0.5).sqrt();
        // Java: final T coeff = q0.multiply(normProduct).multiply(2.0).reciprocal();
        // Java: final FieldVector3D<T> q = FieldVector3D.crossProduct(v, u);
        // Java: q1 = coeff.multiply(q.getX());
        // Java: q2 = coeff.multiply(q.getY());
        // Java: q3 = coeff.multiply(q.getZ());
        todo!()
    }

    // Java: FieldRotation(final RotationOrder order, final T alpha1, final T alpha2, final T alpha3)
    pub fn new_from_euler(_order: &dyn std::any::Any, _alpha1: f64, _alpha2: f64, _alpha3: f64) -> Self {
        // Java: this(order, RotationConvention.VECTOR_OPERATOR, alpha1, alpha2, alpha3);
        todo!()
    }

    // Java: FieldRotation()
    pub fn new_from_euler_convention(_order: &dyn std::any::Any, _convention: &dyn std::any::Any, _alpha1: f64, _alpha2: f64, _alpha3: f64) -> Self {
        todo!()
    }

    // Java: T[] mat2quat(final T[][] ort)
    pub fn mat2quat(&self, _ort: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final T[] quat = MathArrays.buildArray(ort[0][0].getField(), 4);
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
        // Java: T s = ort[0][0].add(ort[1][1]).add(ort[2][2]);
        // Java: if (s.getReal() > -0.19)
        // Java: // compute q0 and deduce q1, q2 and q3
        // Java: quat[0] = s.add(1.0).sqrt().multiply(0.5);
        // Java: T inv = quat[0].reciprocal().multiply(0.25);
        // Java: quat[1] = inv.multiply(ort[1][2].subtract(ort[2][1]));
        // Java: quat[2] = inv.multiply(ort[2][0].subtract(ort[0][2]));
        // Java: quat[3] = inv.multiply(ort[0][1].subtract(ort[1][0]));
        // Java: else
        // Java: s = ort[0][0].subtract(ort[1][1]).subtract(ort[2][2]);
        // Java: if (s.getReal() > -0.19)
        // Java: // compute q1 and deduce q0, q2 and q3
        // Java: quat[1] = s.add(1.0).sqrt().multiply(0.5);
        // Java: T inv = quat[1].reciprocal().multiply(0.25);
        // Java: quat[0] = inv.multiply(ort[1][2].subtract(ort[2][1]));
        // Java: quat[2] = inv.multiply(ort[0][1].add(ort[1][0]));
        // Java: quat[3] = inv.multiply(ort[0][2].add(ort[2][0]));
        // Java: else
        // Java: s = ort[1][1].subtract(ort[0][0]).subtract(ort[2][2]);
        // Java: if (s.getReal() > -0.19)
        // Java: // compute q2 and deduce q0, q1 and q3
        // Java: quat[2] = s.add(1.0).sqrt().multiply(0.5);
        // Java: T inv = quat[2].reciprocal().multiply(0.25);
        // Java: quat[0] = inv.multiply(ort[2][0].subtract(ort[0][2]));
        // Java: quat[1] = inv.multiply(ort[0][1].add(ort[1][0]));
        // Java: quat[3] = inv.multiply(ort[2][1].add(ort[1][2]));
        // Java: else
        // Java: // compute q3 and deduce q0, q1 and q2
        // Java: s = ort[2][2].subtract(ort[0][0]).subtract(ort[1][1]);
        // Java: quat[3] = s.add(1.0).sqrt().multiply(0.5);
        // Java: T inv = quat[3].reciprocal().multiply(0.25);
        // Java: quat[0] = inv.multiply(ort[0][1].subtract(ort[1][0]));
        // Java: quat[1] = inv.multiply(ort[0][2].add(ort[2][0]));
        // Java: quat[2] = inv.multiply(ort[2][1].add(ort[1][2]));
        // Java: return quat;
        todo!()
    }

    // Java: FieldRotation<T> revert()
    pub fn revert(&self) -> FieldRotation {
        // Java: return new FieldRotation<T>(q0.negate(), q1, q2, q3, false);
        FieldRotation::new(-self.q0, self.q1, self.q2, self.q3, false)
    }

    // Java: T getQ0()
    pub fn getQ0(&self) -> f64 {
        // Java: return q0;
        self.q0
    }

    // Java: T getQ1()
    pub fn getQ1(&self) -> f64 {
        // Java: return q1;
        self.q1
    }

    // Java: T getQ2()
    pub fn getQ2(&self) -> f64 {
        // Java: return q2;
        self.q2
    }

    // Java: T getQ3()
    pub fn getQ3(&self) -> f64 {
        // Java: return q3;
        self.q3
    }

    // Java: FieldVector3D<T> getAxis()
    pub fn getAxis(&self) -> Box<dyn std::any::Any> {
        // Java: return getAxis(RotationConvention.VECTOR_OPERATOR);
        todo!()
    }

    // Java: FieldVector3D<T> getAxis(final RotationConvention convention)
    pub fn getAxis_convention(&self, _convention: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final T squaredSine = q1.multiply(q1).add(q2.multiply(q2)).add(q3.multiply(q3));
        // Java: if (squaredSine.getReal() == 0)
        // Java: final Field<T> field = squaredSine.getField();
        // Java: return new FieldVector3D<T>(convention == RotationConvention.VECTOR_OPERATOR ? field.getOne(): field.getOne().negate(),
        // Java: field.getZero(),
        // Java: field.getZero());
        // Java: else
        // Java: final double sgn = convention == RotationConvention.VECTOR_OPERATOR ? +1 : -1;
        // Java: if (q0.getReal() < 0)
        // Java: T inverse = squaredSine.sqrt().reciprocal().multiply(sgn);
        // Java: return new FieldVector3D<T>(q1.multiply(inverse), q2.multiply(inverse), q3.multiply(inverse));
        // Java: final T inverse = squaredSine.sqrt().reciprocal().negate().multiply(sgn);
        // Java: return new FieldVector3D<T>(q1.multiply(inverse), q2.multiply(inverse), q3.multiply(inverse));
        todo!()
    }

    // Java: T getAngle()
    pub fn getAngle(&self) -> f64 {
        // Java: if ((q0.getReal() < -0.1) || (q0.getReal() > 0.1))
        if (self.q0 < -0.1) || (self.q0 > 0.1)
        {
            // Java: return q1.multiply(q1).add(q2.multiply(q2)).add(q3.multiply(q3)).sqrt().asin().multiply(2);
            return (self.q1 * self.q1 + self.q2 * self.q2 + self.q3 * self.q3).sqrt().asin() * 2.0;
        }
        // Java: else if (q0.getReal() < 0)
        else if self.q0 < 0.0
        {
            // Java: return q0.negate().acos().multiply(2);
            return (-self.q0).acos() * 2.0;
        }
        // Java: return q0.acos().multiply(2);
        self.q0.acos() * 2.0
    }

    // Java: T[] getAngles(final RotationOrder order)
    pub fn getAngles(&self, _order: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T[] getAngles(final RotationOrder order, RotationConvention convention)
    pub fn getAngles_convention(&self, _order: &dyn std::any::Any, _convention: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T[] buildArray(final T a0, final T a1, final T a2)
    pub fn buildArray(&self, a0: f64, a1: f64, a2: f64) -> Vec<f64> {
        // Java: final T[] array = MathArrays.buildArray(a0.getField(), 3);
        // Java: array[0] = a0;
        // Java: array[1] = a1;
        // Java: array[2] = a2;
        // Java: return array;
        vec![a0, a1, a2]
    }

    // Java: FieldVector3D<T> vector(final double x, final double y, final double z)
    pub fn vector(&self, x: f64, y: f64, z: f64) -> Box<dyn std::any::Any> {
        // Java: final T zero = q0.getField().getZero();
        // Java: return new FieldVector3D<T>(zero.add(x), zero.add(y), zero.add(z));
        todo!()
    }

    // Java: T[][] getMatrix()
    pub fn getMatrix(&self) -> Vec<Vec<f64>> {
        // Java: // products
        // Java: final T q0q0  = q0.multiply(q0);
        let q0q0 = self.q0 * self.q0;
        // Java: final T q0q1  = q0.multiply(q1);
        let q0q1 = self.q0 * self.q1;
        // Java: final T q0q2  = q0.multiply(q2);
        let q0q2 = self.q0 * self.q2;
        // Java: final T q0q3  = q0.multiply(q3);
        let q0q3 = self.q0 * self.q3;
        // Java: final T q1q1  = q1.multiply(q1);
        let q1q1 = self.q1 * self.q1;
        // Java: final T q1q2  = q1.multiply(q2);
        let q1q2 = self.q1 * self.q2;
        // Java: final T q1q3  = q1.multiply(q3);
        let q1q3 = self.q1 * self.q3;
        // Java: final T q2q2  = q2.multiply(q2);
        let q2q2 = self.q2 * self.q2;
        // Java: final T q2q3  = q2.multiply(q3);
        let q2q3 = self.q2 * self.q3;
        // Java: final T q3q3  = q3.multiply(q3);
        let q3q3 = self.q3 * self.q3;
        // Java: // create the matrix
        // Java: final T[][] m = MathArrays.buildArray(q0.getField(), 3, 3);
        let mut m = vec![vec![0.0f64; 3]; 3];
        // Java: m [0][0] = q0q0.add(q1q1).multiply(2).subtract(1);
        m[0][0] = (q0q0 + q1q1) * 2.0 - 1.0;
        // Java: m [1][0] = q1q2.subtract(q0q3).multiply(2);
        m[1][0] = (q1q2 - q0q3) * 2.0;
        // Java: m [2][0] = q1q3.add(q0q2).multiply(2);
        m[2][0] = (q1q3 + q0q2) * 2.0;
        // Java: m [0][1] = q1q2.add(q0q3).multiply(2);
        m[0][1] = (q1q2 + q0q3) * 2.0;
        // Java: m [1][1] = q0q0.add(q2q2).multiply(2).subtract(1);
        m[1][1] = (q0q0 + q2q2) * 2.0 - 1.0;
        // Java: m [2][1] = q2q3.subtract(q0q1).multiply(2);
        m[2][1] = (q2q3 - q0q1) * 2.0;
        // Java: m [0][2] = q1q3.subtract(q0q2).multiply(2);
        m[0][2] = (q1q3 - q0q2) * 2.0;
        // Java: m [1][2] = q2q3.add(q0q1).multiply(2);
        m[1][2] = (q2q3 + q0q1) * 2.0;
        // Java: m [2][2] = q0q0.add(q3q3).multiply(2).subtract(1);
        m[2][2] = (q0q0 + q3q3) * 2.0 - 1.0;
        // Java: return m;
        m
    }

    // Java: Rotation toRotation()
    pub fn toRotation(&self) -> Box<dyn std::any::Any> {
        // Java: return new Rotation(q0.getReal(), q1.getReal(), q2.getReal(), q3.getReal(), false);
        todo!()
    }

    // Java: FieldVector3D<T> applyTo(final FieldVector3D<T> u)
    pub fn applyTo_field_vec(&self, _u: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final T x = u.getX();
        // Java: final T y = u.getY();
        // Java: final T z = u.getZ();
        // Java: final T s = q1.multiply(x).add(q2.multiply(y)).add(q3.multiply(z));
        // Java: return new FieldVector3D<T>(q0.multiply(x.multiply(q0).subtract(q2.multiply(z).subtract(q3.multiply(y)))).add(s.multiply(q1)).multiply(2).subtract(x),
        // Java: q0.multiply(y.multiply(q0).subtract(q3.multiply(x).subtract(q1.multiply(z)))).add(s.multiply(q2)).multiply(2).subtract(y),
        // Java: q0.multiply(z.multiply(q0).subtract(q1.multiply(y).subtract(q2.multiply(x)))).add(s.multiply(q3)).multiply(2).subtract(z));
        todo!()
    }

    // Java: FieldVector3D<T> applyTo(final Vector3D u)
    pub fn applyTo_vec3d(&self, _u: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final double x = u.getX();
        // Java: final double y = u.getY();
        // Java: final double z = u.getZ();
        // Java: final T s = q1.multiply(x).add(q2.multiply(y)).add(q3.multiply(z));
        // Java: return new FieldVector3D<T>(q0.multiply(q0.multiply(x).subtract(q2.multiply(z).subtract(q3.multiply(y)))).add(s.multiply(q1)).multiply(2).subtract(x),
        // Java: q0.multiply(q0.multiply(y).subtract(q3.multiply(x).subtract(q1.multiply(z)))).add(s.multiply(q2)).multiply(2).subtract(y),
        // Java: q0.multiply(q0.multiply(z).subtract(q1.multiply(y).subtract(q2.multiply(x)))).add(s.multiply(q3)).multiply(2).subtract(z));
        todo!()
    }

    // Java: void applyTo(final T[] in, final T[] out)
    pub fn applyTo_arr(&self, _input: &[f64], _output: &mut [f64]) {
        // Java: final T x = in[0];
        // Java: final T y = in[1];
        // Java: final T z = in[2];
        // Java: final T s = q1.multiply(x).add(q2.multiply(y)).add(q3.multiply(z));
        // Java: out[0] = q0.multiply(x.multiply(q0).subtract(q2.multiply(z).subtract(q3.multiply(y)))).add(s.multiply(q1)).multiply(2).subtract(x);
        // Java: out[1] = q0.multiply(y.multiply(q0).subtract(q3.multiply(x).subtract(q1.multiply(z)))).add(s.multiply(q2)).multiply(2).subtract(y);
        // Java: out[2] = q0.multiply(z.multiply(q0).subtract(q1.multiply(y).subtract(q2.multiply(x)))).add(s.multiply(q3)).multiply(2).subtract(z);
        todo!()
    }

    // Java: void applyTo(final double[] in, final T[] out)
    pub fn applyTo_darr(&self, _input: &[f64], _output: &mut [f64]) {
        // Java: final double x = in[0];
        // Java: final double y = in[1];
        // Java: final double z = in[2];
        // Java: final T s = q1.multiply(x).add(q2.multiply(y)).add(q3.multiply(z));
        // Java: out[0] = q0.multiply(q0.multiply(x).subtract(q2.multiply(z).subtract(q3.multiply(y)))).add(s.multiply(q1)).multiply(2).subtract(x);
        // Java: out[1] = q0.multiply(q0.multiply(y).subtract(q3.multiply(x).subtract(q1.multiply(z)))).add(s.multiply(q2)).multiply(2).subtract(y);
        // Java: out[2] = q0.multiply(q0.multiply(z).subtract(q1.multiply(y).subtract(q2.multiply(x)))).add(s.multiply(q3)).multiply(2).subtract(z);
        todo!()
    }

    // Java: static public static <T extends RealFieldElement<T>> FieldVector3D<T> applyTo(final Rotation r, final FieldVector3D<T> u)
    pub fn applyTo_rotation_vec(_r: &dyn std::any::Any, _u: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final T x = u.getX();
        // Java: final T y = u.getY();
        // Java: final T z = u.getZ();
        // Java: final T s = x.multiply(r.getQ1()).add(y.multiply(r.getQ2())).add(z.multiply(r.getQ3()));
        // Java: return new FieldVector3D<T>(x.multiply(r.getQ0()).subtract(z.multiply(r.getQ2()).subtract(y.multiply(r.getQ3()))).multiply(r.getQ0()).add(s.multiply(r.getQ1())).multiply(2).subtract(x),
        // Java: y.multiply(r.getQ0()).subtract(x.multiply(r.getQ3()).subtract(z.multiply(r.getQ1()))).multiply(r.getQ0()).add(s.multiply(r.getQ2())).multiply(2).subtract(y),
        // Java: z.multiply(r.getQ0()).subtract(y.multiply(r.getQ1()).subtract(x.multiply(r.getQ2()))).multiply(r.getQ0()).add(s.multiply(r.getQ3())).multiply(2).subtract(z));
        todo!()
    }

    // Java: FieldVector3D<T> applyInverseTo(final FieldVector3D<T> u)
    pub fn applyInverseTo_field_vec(&self, _u: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final T x = u.getX();
        // Java: final T y = u.getY();
        // Java: final T z = u.getZ();
        // Java: final T s  = q1.multiply(x).add(q2.multiply(y)).add(q3.multiply(z));
        // Java: final T m0 = q0.negate();
        // Java: return new FieldVector3D<T>(m0.multiply(x.multiply(m0).subtract(q2.multiply(z).subtract(q3.multiply(y)))).add(s.multiply(q1)).multiply(2).subtract(x),
        // Java: m0.multiply(y.multiply(m0).subtract(q3.multiply(x).subtract(q1.multiply(z)))).add(s.multiply(q2)).multiply(2).subtract(y),
        // Java: m0.multiply(z.multiply(m0).subtract(q1.multiply(y).subtract(q2.multiply(x)))).add(s.multiply(q3)).multiply(2).subtract(z));
        todo!()
    }

    // Java: FieldVector3D<T> applyInverseTo(final Vector3D u)
    pub fn applyInverseTo_vec3d(&self, _u: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final double x = u.getX();
        // Java: final double y = u.getY();
        // Java: final double z = u.getZ();
        // Java: final T s  = q1.multiply(x).add(q2.multiply(y)).add(q3.multiply(z));
        // Java: final T m0 = q0.negate();
        // Java: return new FieldVector3D<T>(m0.multiply(m0.multiply(x).subtract(q2.multiply(z).subtract(q3.multiply(y)))).add(s.multiply(q1)).multiply(2).subtract(x),
        // Java: m0.multiply(m0.multiply(y).subtract(q3.multiply(x).subtract(q1.multiply(z)))).add(s.multiply(q2)).multiply(2).subtract(y),
        // Java: m0.multiply(m0.multiply(z).subtract(q1.multiply(y).subtract(q2.multiply(x)))).add(s.multiply(q3)).multiply(2).subtract(z));
        todo!()
    }

    // Java: void applyInverseTo(final T[] in, final T[] out)
    pub fn applyInverseTo_arr(&self, _input: &[f64], _output: &mut [f64]) {
        // Java: final T x = in[0];
        // Java: final T y = in[1];
        // Java: final T z = in[2];
        // Java: final T s = q1.multiply(x).add(q2.multiply(y)).add(q3.multiply(z));
        // Java: final T m0 = q0.negate();
        // Java: out[0] = m0.multiply(x.multiply(m0).subtract(q2.multiply(z).subtract(q3.multiply(y)))).add(s.multiply(q1)).multiply(2).subtract(x);
        // Java: out[1] = m0.multiply(y.multiply(m0).subtract(q3.multiply(x).subtract(q1.multiply(z)))).add(s.multiply(q2)).multiply(2).subtract(y);
        // Java: out[2] = m0.multiply(z.multiply(m0).subtract(q1.multiply(y).subtract(q2.multiply(x)))).add(s.multiply(q3)).multiply(2).subtract(z);
        todo!()
    }

    // Java: void applyInverseTo(final double[] in, final T[] out)
    pub fn applyInverseTo_darr(&self, _input: &[f64], _output: &mut [f64]) {
        // Java: final double x = in[0];
        // Java: final double y = in[1];
        // Java: final double z = in[2];
        // Java: final T s = q1.multiply(x).add(q2.multiply(y)).add(q3.multiply(z));
        // Java: final T m0 = q0.negate();
        // Java: out[0] = m0.multiply(m0.multiply(x).subtract(q2.multiply(z).subtract(q3.multiply(y)))).add(s.multiply(q1)).multiply(2).subtract(x);
        // Java: out[1] = m0.multiply(m0.multiply(y).subtract(q3.multiply(x).subtract(q1.multiply(z)))).add(s.multiply(q2)).multiply(2).subtract(y);
        // Java: out[2] = m0.multiply(m0.multiply(z).subtract(q1.multiply(y).subtract(q2.multiply(x)))).add(s.multiply(q3)).multiply(2).subtract(z);
        todo!()
    }

    // Java: static public static <T extends RealFieldElement<T>> FieldVector3D<T> applyInverseTo(final Rotation r, final FieldVector3D<T> u)
    pub fn applyInverseTo_rotation_vec(_r: &dyn std::any::Any, _u: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final T x = u.getX();
        // Java: final T y = u.getY();
        // Java: final T z = u.getZ();
        // Java: final T s  = x.multiply(r.getQ1()).add(y.multiply(r.getQ2())).add(z.multiply(r.getQ3()));
        // Java: final double m0 = -r.getQ0();
        // Java: return new FieldVector3D<T>(x.multiply(m0).subtract(z.multiply(r.getQ2()).subtract(y.multiply(r.getQ3()))).multiply(m0).add(s.multiply(r.getQ1())).multiply(2).subtract(x),
        // Java: y.multiply(m0).subtract(x.multiply(r.getQ3()).subtract(z.multiply(r.getQ1()))).multiply(m0).add(s.multiply(r.getQ2())).multiply(2).subtract(y),
        // Java: z.multiply(m0).subtract(y.multiply(r.getQ1()).subtract(x.multiply(r.getQ2()))).multiply(m0).add(s.multiply(r.getQ3())).multiply(2).subtract(z));
        todo!()
    }

    // Java: FieldRotation<T> applyTo(final FieldRotation<T> r)
    pub fn applyTo_field_rot(&self, _r: &FieldRotation) -> FieldRotation {
        // Java: return compose(r, RotationConvention.VECTOR_OPERATOR);
        todo!()
    }

    // Java: FieldRotation<T> compose(final FieldRotation<T> r, final RotationConvention convention)
    pub fn compose(&self, _r: &FieldRotation, _convention: &dyn std::any::Any) -> FieldRotation {
        // Java: return convention == RotationConvention.VECTOR_OPERATOR ?
        // Java: composeInternal(r) : r.composeInternal(this);
        todo!()
    }

    // Java: FieldRotation<T> composeInternal(final FieldRotation<T> r)
    pub fn composeInternal(&self, r: &FieldRotation) -> FieldRotation {
        // Java: return new FieldRotation<T>(r.q0.multiply(q0).subtract(r.q1.multiply(q1).add(r.q2.multiply(q2)).add(r.q3.multiply(q3))),
        // Java: r.q1.multiply(q0).add(r.q0.multiply(q1)).add(r.q2.multiply(q3).subtract(r.q3.multiply(q2))),
        // Java: r.q2.multiply(q0).add(r.q0.multiply(q2)).add(r.q3.multiply(q1).subtract(r.q1.multiply(q3))),
        // Java: r.q3.multiply(q0).add(r.q0.multiply(q3)).add(r.q1.multiply(q2).subtract(r.q2.multiply(q1))),
        // Java: false);
        FieldRotation::new(
            r.q0 * self.q0 - (r.q1 * self.q1 + r.q2 * self.q2 + r.q3 * self.q3),
            r.q1 * self.q0 + r.q0 * self.q1 + (r.q2 * self.q3 - r.q3 * self.q2),
            r.q2 * self.q0 + r.q0 * self.q2 + (r.q3 * self.q1 - r.q1 * self.q3),
            r.q3 * self.q0 + r.q0 * self.q3 + (r.q1 * self.q2 - r.q2 * self.q1),
            false,
        )
    }

    // Java: FieldRotation<T> applyTo(final Rotation r)
    pub fn applyTo_rot(&self, _r: &dyn std::any::Any) -> FieldRotation {
        // Java: return compose(r, RotationConvention.VECTOR_OPERATOR);
        todo!()
    }

    // Java: FieldRotation<T> compose(final Rotation r, final RotationConvention convention)
    pub fn compose_rot(&self, _r: &dyn std::any::Any, _convention: &dyn std::any::Any) -> FieldRotation {
        // Java: return convention == RotationConvention.VECTOR_OPERATOR ?
        // Java: composeInternal(r) : applyTo(r, this);
        todo!()
    }

    // Java: FieldRotation<T> composeInternal(final Rotation r)
    pub fn composeInternal_rot(&self, _r: &dyn std::any::Any) -> FieldRotation {
        // Java: return new FieldRotation<T>(q0.multiply(r.getQ0()).subtract(q1.multiply(r.getQ1()).add(q2.multiply(r.getQ2())).add(q3.multiply(r.getQ3()))),
        // Java: q0.multiply(r.getQ1()).add(q1.multiply(r.getQ0())).add(q3.multiply(r.getQ2()).subtract(q2.multiply(r.getQ3()))),
        // Java: q0.multiply(r.getQ2()).add(q2.multiply(r.getQ0())).add(q1.multiply(r.getQ3()).subtract(q3.multiply(r.getQ1()))),
        // Java: q0.multiply(r.getQ3()).add(q3.multiply(r.getQ0())).add(q2.multiply(r.getQ1()).subtract(q1.multiply(r.getQ2()))),
        // Java: false);
        todo!()
    }

    // Java: static public static <T extends RealFieldElement<T>> FieldRotation<T> applyTo(final Rotation r1, final FieldRotation<T> rInner)
    pub fn applyTo_rotation_field(_r1: &dyn std::any::Any, _rInner: &FieldRotation) -> FieldRotation {
        // Java: return new FieldRotation<T>(rInner.q0.multiply(r1.getQ0()).subtract(rInner.q1.multiply(r1.getQ1()).add(rInner.q2.multiply(r1.getQ2())).add(rInner.q3.multiply(r1.getQ3()))),
        // Java: rInner.q1.multiply(r1.getQ0()).add(rInner.q0.multiply(r1.getQ1())).add(rInner.q2.multiply(r1.getQ3()).subtract(rInner.q3.multiply(r1.getQ2()))),
        // Java: rInner.q2.multiply(r1.getQ0()).add(rInner.q0.multiply(r1.getQ2())).add(rInner.q3.multiply(r1.getQ1()).subtract(rInner.q1.multiply(r1.getQ3()))),
        // Java: rInner.q3.multiply(r1.getQ0()).add(rInner.q0.multiply(r1.getQ3())).add(rInner.q1.multiply(r1.getQ2()).subtract(rInner.q2.multiply(r1.getQ1()))),
        // Java: false);
        todo!()
    }

    // Java: FieldRotation<T> applyInverseTo(final FieldRotation<T> r)
    pub fn applyInverseTo_field_rot(&self, _r: &FieldRotation) -> FieldRotation {
        // Java: return composeInverse(r, RotationConvention.VECTOR_OPERATOR);
        todo!()
    }

    // Java: FieldRotation<T> composeInverse(final FieldRotation<T> r, final RotationConvention convention)
    pub fn composeInverse(&self, _r: &FieldRotation, _convention: &dyn std::any::Any) -> FieldRotation {
        // Java: return convention == RotationConvention.VECTOR_OPERATOR ?
        // Java: composeInverseInternal(r) : r.composeInternal(revert());
        todo!()
    }

    // Java: FieldRotation<T> composeInverseInternal(FieldRotation<T> r)
    pub fn composeInverseInternal(&self, r: &FieldRotation) -> FieldRotation {
        // Java: return new FieldRotation<T>(r.q0.multiply(q0).add(r.q1.multiply(q1).add(r.q2.multiply(q2)).add(r.q3.multiply(q3))).negate(),
        // Java: r.q0.multiply(q1).add(r.q2.multiply(q3).subtract(r.q3.multiply(q2))).subtract(r.q1.multiply(q0)),
        // Java: r.q0.multiply(q2).add(r.q3.multiply(q1).subtract(r.q1.multiply(q3))).subtract(r.q2.multiply(q0)),
        // Java: r.q0.multiply(q3).add(r.q1.multiply(q2).subtract(r.q2.multiply(q1))).subtract(r.q3.multiply(q0)),
        // Java: false);
        FieldRotation::new(
            -(r.q0 * self.q0 + (r.q1 * self.q1 + r.q2 * self.q2 + r.q3 * self.q3)),
            r.q0 * self.q1 + (r.q2 * self.q3 - r.q3 * self.q2) - r.q1 * self.q0,
            r.q0 * self.q2 + (r.q3 * self.q1 - r.q1 * self.q3) - r.q2 * self.q0,
            r.q0 * self.q3 + (r.q1 * self.q2 - r.q2 * self.q1) - r.q3 * self.q0,
            false,
        )
    }

    // Java: FieldRotation<T> applyInverseTo(final Rotation r)
    pub fn applyInverseTo_rot(&self, _r: &dyn std::any::Any) -> FieldRotation {
        // Java: return composeInverse(r, RotationConvention.VECTOR_OPERATOR);
        todo!()
    }

    // Java: FieldRotation<T> composeInverse(final Rotation r, final RotationConvention convention)
    pub fn composeInverse_rot(&self, _r: &dyn std::any::Any, _convention: &dyn std::any::Any) -> FieldRotation {
        // Java: return convention == RotationConvention.VECTOR_OPERATOR ?
        // Java: composeInverseInternal(r) : applyTo(r, revert());
        todo!()
    }

    // Java: FieldRotation<T> composeInverseInternal(Rotation r)
    pub fn composeInverseInternal_rot(&self, _r: &dyn std::any::Any) -> FieldRotation {
        // Java: return new FieldRotation<T>(q0.multiply(r.getQ0()).add(q1.multiply(r.getQ1()).add(q2.multiply(r.getQ2())).add(q3.multiply(r.getQ3()))).negate(),
        // Java: q1.multiply(r.getQ0()).add(q3.multiply(r.getQ2()).subtract(q2.multiply(r.getQ3()))).subtract(q0.multiply(r.getQ1())),
        // Java: q2.multiply(r.getQ0()).add(q1.multiply(r.getQ3()).subtract(q3.multiply(r.getQ1()))).subtract(q0.multiply(r.getQ2())),
        // Java: q3.multiply(r.getQ0()).add(q2.multiply(r.getQ1()).subtract(q1.multiply(r.getQ2()))).subtract(q0.multiply(r.getQ3())),
        // Java: false);
        todo!()
    }

    // Java: static public static <T extends RealFieldElement<T>> FieldRotation<T> applyInverseTo(final Rotation rOuter, final FieldRotation<T> rInner)
    pub fn applyInverseTo_rotation_field(_rOuter: &dyn std::any::Any, _rInner: &FieldRotation) -> FieldRotation {
        // Java: return new FieldRotation<T>(rInner.q0.multiply(rOuter.getQ0()).add(rInner.q1.multiply(rOuter.getQ1()).add(rInner.q2.multiply(rOuter.getQ2())).add(rInner.q3.multiply(rOuter.getQ3()))).negate(),
        // Java: rInner.q0.multiply(rOuter.getQ1()).add(rInner.q2.multiply(rOuter.getQ3()).subtract(rInner.q3.multiply(rOuter.getQ2()))).subtract(rInner.q1.multiply(rOuter.getQ0())),
        // Java: rInner.q0.multiply(rOuter.getQ2()).add(rInner.q3.multiply(rOuter.getQ1()).subtract(rInner.q1.multiply(rOuter.getQ3()))).subtract(rInner.q2.multiply(rOuter.getQ0())),
        // Java: rInner.q0.multiply(rOuter.getQ3()).add(rInner.q1.multiply(rOuter.getQ2()).subtract(rInner.q2.multiply(rOuter.getQ1()))).subtract(rInner.q3.multiply(rOuter.getQ0())),
        // Java: false);
        todo!()
    }

    // Java: T[][] orthogonalizeMatrix(final T[][] m, final double threshold)
    pub fn orthogonalizeMatrix(&self, _m: &dyn std::any::Any, _threshold: f64) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static public static <T extends RealFieldElement<T>> T distance(final FieldRotation<T> r1, final FieldRotation<T> r2)
    pub fn distance(r1: &FieldRotation, r2: &FieldRotation) -> f64 {
        // Java: return r1.composeInverseInternal(r2).getAngle();
        r1.composeInverseInternal(r2).getAngle()
    }

}
