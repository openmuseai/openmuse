// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import java.util.Iterator
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction::UnivariateFunction;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;
// Java field: RealVectorFormat DEFAULT_FORMAT
static DEFAULT_FORMAT: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();

// Java: public class implements implements the
pub struct implements {
}

// Java: public class ArrayRealVector extends RealVector implements Serializable
pub struct ArrayRealVector {
}

// Java: public class implements implements the
impl implements {
}

// impl the for implements {
// }

// Java: public class ArrayRealVector extends RealVector implements Serializable
impl ArrayRealVector {
    // Java: ArrayRealVector()
    pub fn ArrayRealVector(&self) {
        // Java: data = new double[0];
    }

    // Java: ArrayRealVector(int size)
    pub fn ArrayRealVector_1(&self, size: i32) {
        // Java: data = new double[size];
    }

    // Java: ArrayRealVector(int size, double preset)
    pub fn ArrayRealVector_2(&self, size: i32, preset: f64) {
        // Java: data = new double[size];
        // Java: Arrays.fill(data, preset);
    }

    // Java: ArrayRealVector(double[] d)
    pub fn ArrayRealVector_3(&self, d: &[f64]) {
        // Java: data = d.clone();
    }

    // Java: ArrayRealVector(double[] d, boolean copyArray)
    pub fn ArrayRealVector_4(&self, d: &[f64], copyArray: bool) {
    }

    // Java: ArrayRealVector(double[] d, int pos, int size)
    pub fn ArrayRealVector_5(&self, d: &[f64], pos: i32, size: i32) {
    }

    // Java: ArrayRealVector(Double[] d)
    pub fn ArrayRealVector_6(&self, d: &[f64]) {
        // Java: data = new double[d.length];
        // Java: for (int i = 0; i < d.length; i++)
        {
            // Java: data[i] = d[i].doubleValue();
        }
    }

    // Java: ArrayRealVector(Double[] d, int pos, int size)
    pub fn ArrayRealVector_7(&self, d: &[f64], pos: i32, size: i32) {
    }

    // Java: ArrayRealVector(RealVector v)
    pub fn ArrayRealVector_8(&self, v: &dyn std::any::Any) {
        // Java: if (v == null)
        {
            // Java: throw new NullArgumentException();
        }
        // Java: data = new double[v.getDimension()];
        // Java: for (int i = 0; i < data.length; ++i)
        {
            // Java: data[i] = v.getEntry(i);
        }
    }

    // Java: ArrayRealVector(ArrayRealVector v)
    pub fn ArrayRealVector_9(&self, v: &ArrayRealVector) {
        // Java: this(v, true);
    }

    // Java: ArrayRealVector(ArrayRealVector v, boolean deep)
    pub fn ArrayRealVector_10(&self, v: &ArrayRealVector, deep: bool) {
        // Java: data = deep ? v.data.clone() : v.data;
    }

    // Java: ArrayRealVector(ArrayRealVector v1, ArrayRealVector v2)
    pub fn ArrayRealVector_11(&self, v1: &ArrayRealVector, v2: &ArrayRealVector) {
        // Java: data = new double[v1.data.length + v2.data.length];
        // Java: System.arraycopy(v1.data, 0, data, 0, v1.data.length);
        // Java: System.arraycopy(v2.data, 0, data, v1.data.length, v2.data.length);
    }

    // Java: ArrayRealVector(ArrayRealVector v1, RealVector v2)
    pub fn ArrayRealVector_12(&self, v1: &ArrayRealVector, v2: &dyn std::any::Any) {
        // Java: final int l1 = v1.data.length;
        // Java: final int l2 = v2.getDimension();
        // Java: data = new double[l1 + l2];
        // Java: System.arraycopy(v1.data, 0, data, 0, l1);
        // Java: for (int i = 0; i < l2; ++i)
        {
            // Java: data[l1 + i] = v2.getEntry(i);
        }
    }

    // Java: ArrayRealVector(RealVector v1, ArrayRealVector v2)
    pub fn ArrayRealVector_13(&self, v1: &dyn std::any::Any, v2: &ArrayRealVector) {
        // Java: final int l1 = v1.getDimension();
        // Java: final int l2 = v2.data.length;
        // Java: data = new double[l1 + l2];
        // Java: for (int i = 0; i < l1; ++i)
        {
            // Java: data[i] = v1.getEntry(i);
        }
        // Java: System.arraycopy(v2.data, 0, data, l1, l2);
    }

    // Java: ArrayRealVector(ArrayRealVector v1, double[] v2)
    pub fn ArrayRealVector_14(&self, v1: &ArrayRealVector, v2: &[f64]) {
        // Java: final int l1 = v1.getDimension();
        // Java: final int l2 = v2.length;
        // Java: data = new double[l1 + l2];
        // Java: System.arraycopy(v1.data, 0, data, 0, l1);
        // Java: System.arraycopy(v2, 0, data, l1, l2);
    }

    // Java: ArrayRealVector(double[] v1, ArrayRealVector v2)
    pub fn ArrayRealVector_15(&self, v1: &[f64], v2: &ArrayRealVector) {
        // Java: final int l1 = v1.length;
        // Java: final int l2 = v2.getDimension();
        // Java: data = new double[l1 + l2];
        // Java: System.arraycopy(v1, 0, data, 0, l1);
        // Java: System.arraycopy(v2.data, 0, data, l1, l2);
    }

    // Java: ArrayRealVector(double[] v1, double[] v2)
    pub fn ArrayRealVector_16(&self, v1: &[f64], v2: &[f64]) {
        // Java: final int l1 = v1.length;
        // Java: final int l2 = v2.length;
        // Java: data = new double[l1 + l2];
        // Java: System.arraycopy(v1, 0, data, 0, l1);
        // Java: System.arraycopy(v2, 0, data, l1, l2);
    }

    // Java: ArrayRealVector copy()
    pub fn copy(&self) -> ArrayRealVector {
        // Java: return new ArrayRealVector(this, true);
        todo!();
    }

    // Java: ArrayRealVector add(RealVector v)
    pub fn add(&self, v: &dyn std::any::Any) -> ArrayRealVector {
        todo!();
    }

    // Java: ArrayRealVector subtract(RealVector v)
    pub fn subtract(&self, v: &dyn std::any::Any) -> ArrayRealVector {
        todo!();
    }

    // Java: ArrayRealVector map(UnivariateFunction function)
    pub fn map(&self, function: &dyn UnivariateFunction) -> ArrayRealVector {
        // Java: return copy().mapToSelf(function);
        todo!();
    }

    // Java: ArrayRealVector mapToSelf(UnivariateFunction function)
    pub fn mapToSelf(&self, function: &dyn UnivariateFunction) -> ArrayRealVector {
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: data[i] = function.value(data[i]);
        }
        // Java: return this;
        todo!();
    }

    // Java: RealVector mapAddToSelf(double d)
    pub fn mapAddToSelf(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: data[i] += d;
        }
        // Java: return this;
        todo!();
    }

    // Java: RealVector mapSubtractToSelf(double d)
    pub fn mapSubtractToSelf(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: data[i] -= d;
        }
        // Java: return this;
        todo!();
    }

    // Java: RealVector mapMultiplyToSelf(double d)
    pub fn mapMultiplyToSelf(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: data[i] *= d;
        }
        // Java: return this;
        todo!();
    }

    // Java: RealVector mapDivideToSelf(double d)
    pub fn mapDivideToSelf(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: data[i] /= d;
        }
        // Java: return this;
        todo!();
    }

    // Java: ArrayRealVector ebeMultiply(RealVector v)
    pub fn ebeMultiply(&self, v: &dyn std::any::Any) -> ArrayRealVector {
        todo!();
    }

    // Java: ArrayRealVector ebeDivide(RealVector v)
    pub fn ebeDivide(&self, v: &dyn std::any::Any) -> ArrayRealVector {
        todo!();
    }

    // Java: double[] getDataRef()
    pub fn getDataRef(&self) -> Vec<f64> {
        // Java: return data;
        todo!();
    }

    // Java: double dotProduct(RealVector v)
    pub fn dotProduct(&self, v: &dyn std::any::Any) -> f64 {
        // Java: if (v instanceof ArrayRealVector)
        {
            // Java: final double[] vData = ((ArrayRealVector) v).data;
            // Java: checkVectorDimensions(vData.length);
            // Java: double dot = 0;
            // Java: for (int i = 0; i < data.length; i++)
            {
                // Java: dot += data[i] * vData[i];
            }
            // Java: return dot;
        }
        // Java: return super.dotProduct(v);
        todo!();
    }

    // Java: double getNorm()
    pub fn getNorm(&self) -> f64 {
        // Java: double sum = 0;
        // Java: for (double a : data)
        {
            // Java: sum += a * a;
        }
        // Java: return FastMath.sqrt(sum);
        todo!();
    }

    // Java: double getL1Norm()
    pub fn getL1Norm(&self) -> f64 {
        // Java: double sum = 0;
        // Java: for (double a : data)
        {
            // Java: sum += FastMath.abs(a);
        }
        // Java: return sum;
        todo!();
    }

    // Java: double getLInfNorm()
    pub fn getLInfNorm(&self) -> f64 {
        // Java: double max = 0;
        // Java: for (double a : data)
        {
            // Java: max = FastMath.max(max, FastMath.abs(a));
        }
        // Java: return max;
        todo!();
    }

    // Java: double getDistance(RealVector v)
    pub fn getDistance(&self, v: &dyn std::any::Any) -> f64 {
        // Java: if (v instanceof ArrayRealVector)
        {
            // Java: final double[] vData = ((ArrayRealVector) v).data;
            // Java: checkVectorDimensions(vData.length);
            // Java: double sum = 0;
            // Java: for (int i = 0; i < data.length; ++i)
            {
                // Java: final double delta = data[i] - vData[i];
                // Java: sum += delta * delta;
            }
            // Java: return FastMath.sqrt(sum);
        }
        // Java: else
        {
            // Java: checkVectorDimensions(v);
            // Java: double sum = 0;
            // Java: for (int i = 0; i < data.length; ++i)
            {
                // Java: final double delta = data[i] - v.getEntry(i);
                // Java: sum += delta * delta;
            }
            // Java: return FastMath.sqrt(sum);
        }
        todo!();
    }

    // Java: double getL1Distance(RealVector v)
    pub fn getL1Distance(&self, v: &dyn std::any::Any) -> f64 {
        todo!();
    }

    // Java: double getLInfDistance(RealVector v)
    pub fn getLInfDistance(&self, v: &dyn std::any::Any) -> f64 {
        todo!();
    }

    // Java: RealMatrix outerProduct(RealVector v)
    pub fn outerProduct(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: if (v instanceof ArrayRealVector)
        {
            // Java: final double[] vData = ((ArrayRealVector) v).data;
            // Java: final int m = data.length;
            // Java: final int n = vData.length;
            // Java: final RealMatrix out = MatrixUtils.createRealMatrix(m, n);
            // Java: for (int i = 0; i < m; i++)
            {
                // Java: for (int j = 0; j < n; j++)
                {
                    // Java: out.setEntry(i, j, data[i] * vData[j]);
                }
            }
            // Java: return out;
        }
        // Java: else
        {
            // Java: final int m = data.length;
            // Java: final int n = v.getDimension();
            // Java: final RealMatrix out = MatrixUtils.createRealMatrix(m, n);
            // Java: for (int i = 0; i < m; i++)
            {
                // Java: for (int j = 0; j < n; j++)
                {
                    // Java: out.setEntry(i, j, data[i] * v.getEntry(j));
                }
            }
            // Java: return out;
        }
        todo!();
    }

    // Java: double getEntry(int index)
    pub fn getEntry(&self, index: i32) -> f64 {
        // Java: try
        {
            // Java: return data[index];
        }
        // Java: catch (IndexOutOfBoundsException e)
        {
            // Java: throw new OutOfRangeException(LocalizedFormats.INDEX, index, 0,
                // Java: getDimension() - 1);
        }
        todo!();
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return data.length;
        todo!();
    }

    // Java: RealVector append(RealVector v)
    pub fn append(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: try
        {
            // Java: return new ArrayRealVector(this, (ArrayRealVector) v);
        }
        // Java: catch (ClassCastException cce)
        {
            // Java: return new ArrayRealVector(this, v);
        }
        todo!();
    }

    // Java: ArrayRealVector append(ArrayRealVector v)
    pub fn append_1(&self, v: &ArrayRealVector) -> ArrayRealVector {
        // Java: return new ArrayRealVector(this, v);
        todo!();
    }

    // Java: RealVector append(double in)
    pub fn append_2(&self, r#in: f64) -> Box<dyn std::any::Any> {
        // Java: final double[] out = new double[data.length + 1];
        // Java: System.arraycopy(data, 0, out, 0, data.length);
        // Java: out[data.length] = in;
        // Java: return new ArrayRealVector(out, false);
        todo!();
    }

    // Java: RealVector getSubVector(int index, int n)
    pub fn getSubVector(&self, index: i32, n: i32) -> Box<dyn std::any::Any> {
        todo!();
    }

    // Java: void setEntry(int index, double value)
    pub fn setEntry(&self, index: i32, value: f64) {
        // Java: try
        {
            // Java: data[index] = value;
        }
        // Java: catch (IndexOutOfBoundsException e)
        {
            // Java: checkIndex(index);
        }
    }

    // Java: void addToEntry(int index, double increment)
    pub fn addToEntry(&self, index: i32, increment: f64) {
    }

    // Java: void setSubVector(int index, RealVector v)
    pub fn setSubVector(&self, index: i32, v: &dyn std::any::Any) {
    }

    // Java: void setSubVector(int index, double[] v)
    pub fn setSubVector_1(&self, index: i32, v: &[f64]) {
    }

    // Java: void set(double value)
    pub fn set(&self, value: f64) {
        // Java: Arrays.fill(data, value);
    }

    // Java: double[] toArray()
    pub fn toArray(&self) -> Vec<f64> {
        // Java: return data.clone();
        todo!();
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return DEFAULT_FORMAT.format(this);
        todo!();
    }

    // Java: void checkVectorDimensions(RealVector v)
    pub fn checkVectorDimensions(&self, v: &dyn std::any::Any) {
    }

    // Java: void checkVectorDimensions(int n)
    pub fn checkVectorDimensions_1(&self, n: i32) {
    }

    // Java: boolean isNaN()
    pub fn isNaN(&self) -> bool {
        // Java: for (double v : data)
        {
            // Java: if (Double.isNaN(v))
            {
                // Java: return true;
            }
        }
        // Java: return false;
        todo!();
    }

    // Java: boolean isInfinite()
    pub fn isInfinite(&self) -> bool {
        // Java: if (isNaN())
        {
            // Java: return false;
        }
        // Java: for (double v : data)
        {
            // Java: if (Double.isInfinite(v))
            {
                // Java: return true;
            }
        }
        // Java: return false;
        todo!();
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &dyn std::any::Any) -> bool {
        // Java: if (this == other)
        {
            // Java: return true;
        }
        // Java: if (!(other instanceof RealVector))
        {
            // Java: return false;
        }
        // Java: RealVector rhs = (RealVector) other;
        // Java: if (data.length != rhs.getDimension())
        {
            // Java: return false;
        }
        // Java: if (rhs.isNaN())
        {
            // Java: return this.isNaN();
        }
        // Java: for (int i = 0; i < data.length; ++i)
        {
            // Java: if (data[i] != rhs.getEntry(i))
            {
                // Java: return false;
            }
        }
        // Java: return true;
        todo!();
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: if (isNaN())
        {
            // Java: return 9;
        }
        // Java: return MathUtils.hash(data);
        todo!();
    }

    // Java: ArrayRealVector combine(double a, double b, RealVector y)
    pub fn combine(&self, a: f64, b: f64, y: &dyn std::any::Any) -> ArrayRealVector {
        todo!();
    }

    // Java: ArrayRealVector combineToSelf(double a, double b, RealVector y)
    pub fn combineToSelf(&self, a: f64, b: f64, y: &dyn std::any::Any) -> ArrayRealVector {
        todo!();
    }

    // Java: double walkInDefaultOrder(final RealVectorPreservingVisitor visitor)
    pub fn walkInDefaultOrder(&self, visitor: &dyn std::any::Any) -> f64 {
        // Java: visitor.start(data.length, 0, data.length - 1);
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: visitor.visit(i, data[i]);
        }
        // Java: return visitor.end();
        todo!();
    }

    // Java: double walkInDefaultOrder()
    pub fn walkInDefaultOrder_1(&self) -> f64 {
        todo!();
    }

    // Java: double walkInOptimizedOrder(final RealVectorPreservingVisitor visitor)
    pub fn walkInOptimizedOrder(&self, visitor: &dyn std::any::Any) -> f64 {
        // Java: return walkInDefaultOrder(visitor);
        todo!();
    }

    // Java: double walkInOptimizedOrder()
    pub fn walkInOptimizedOrder_1(&self) -> f64 {
        todo!();
    }

    // Java: double walkInDefaultOrder(final RealVectorChangingVisitor visitor)
    pub fn walkInDefaultOrder_2(&self, visitor: &dyn std::any::Any) -> f64 {
        // Java: visitor.start(data.length, 0, data.length - 1);
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: data[i] = visitor.visit(i, data[i]);
        }
        // Java: return visitor.end();
        todo!();
    }

    // Java: double walkInDefaultOrder()
    pub fn walkInDefaultOrder_3(&self) -> f64 {
        todo!();
    }

    // Java: double walkInOptimizedOrder(final RealVectorChangingVisitor visitor)
    pub fn walkInOptimizedOrder_2(&self, visitor: &dyn std::any::Any) -> f64 {
        // Java: return walkInDefaultOrder(visitor);
        todo!();
    }

    // Java: double walkInOptimizedOrder()
    pub fn walkInOptimizedOrder_3(&self) -> f64 {
        todo!();
    }

}

// impl Serializable for ArrayRealVector {
// }
