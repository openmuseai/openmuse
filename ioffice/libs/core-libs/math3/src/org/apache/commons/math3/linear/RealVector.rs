// Java: import java.util.Iterator
// Java: import java.util.NoSuchElementException
// Java: import org.apache.commons.math3.exception.MathUnsupportedOperationException
use crate::org::apache::commons::math3::exception::MathUnsupportedOperationException;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.analysis.FunctionUtils
use crate::org::apache::commons::math3::analysis::FunctionUtils;
// Java: import org.apache.commons.math3.analysis.function.Add
use crate::org::apache::commons::math3::analysis::function::Add;
// Java: import org.apache.commons.math3.analysis.function.Multiply
use crate::org::apache::commons::math3::analysis::function::Multiply;
// Java: import org.apache.commons.math3.analysis.function.Divide
use crate::org::apache::commons::math3::analysis::function::Divide;
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction::UnivariateFunction;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class RealVector
pub struct RealVector {
    // Java field: throws OutOfRangeException
    // Java field: throws NotPositiveException, OutOfRangeException
    // Java field: throws OutOfRangeException
    // Java field: throws DimensionMismatchException
    // Java field: throws DimensionMismatchException
}

// Java: public class Entry
pub struct Entry {
    // Java field: int index
    pub(crate) index: i32,
}

// Java: public class SparseEntryIterator
pub struct SparseEntryIterator {
    // Java field: int dim
    pub(crate) dim: i32,
    // Java field: Entry current
    pub(crate) current: Box<dyn std::any::Any>,
    // Java field: Entry next
    pub(crate) next: Box<dyn std::any::Any>,
}

// Java: public class RealVector
impl RealVector {
    // Java: abstract int getDimension()
    pub fn getDimension(&self) -> i32 {
        todo!();
    }

    // Java: abstract double getEntry(int index)
    pub fn getEntry(&self, index: i32) -> f64 {
        todo!();
    }

    // Java: abstract void setEntry(int index, double value)
    pub fn setEntry(&self, index: i32, value: f64) {
    }

    // Java: void addToEntry(int index, double increment)
    pub fn addToEntry(&self, index: i32, increment: f64) {
    }

    // Java: abstract RealVector append(RealVector v)
    pub fn append(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!();
    }

    // Java: abstract RealVector append(double d)
    pub fn append_1(&self, d: f64) -> Box<dyn std::any::Any> {
        todo!();
    }

    // Java: abstract RealVector getSubVector(int index, int n)
    pub fn getSubVector(&self, index: i32, n: i32) -> Box<dyn std::any::Any> {
        todo!();
    }

    // Java: abstract void setSubVector(int index, RealVector v)
    pub fn setSubVector(&self, index: i32, v: &dyn std::any::Any) {
    }

    // Java: abstract boolean isNaN()
    pub fn isNaN(&self) -> bool {
        todo!();
    }

    // Java: abstract boolean isInfinite()
    pub fn isInfinite(&self) -> bool {
        todo!();
    }

    // Java: void checkVectorDimensions(RealVector v)
    pub fn checkVectorDimensions(&self, v: &dyn std::any::Any) {
    }

    // Java: void checkVectorDimensions(int n)
    pub fn checkVectorDimensions_1(&self, n: i32) {
    }

    // Java: void checkIndex(final int index)
    pub fn checkIndex(&self, index: i32) {
        // Java: if (index < 0 ||
            // Java: index >= getDimension())
            {
            // Java: throw new OutOfRangeException(LocalizedFormats.INDEX,
                                          // Java: index, 0, getDimension() - 1);
            }
    }

    // Java: void checkIndices(final int start, final int end)
    pub fn checkIndices(&self, start: i32, end: i32) {
    }

    // Java: RealVector add(RealVector v)
    pub fn add(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: checkVectorDimensions(v);
        // Java: RealVector result = v.copy();
        // Java: Iterator<Entry> it = iterator();
        // Java: while (it.hasNext())
        {
            // Java: final Entry e = it.next();
            // Java: final int index = e.getIndex();
            // Java: result.setEntry(index, e.getValue() + result.getEntry(index));
        }
        // Java: return result;
        todo!();
    }

    // Java: RealVector subtract(RealVector v)
    pub fn subtract(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: checkVectorDimensions(v);
        // Java: RealVector result = v.mapMultiply(-1d);
        // Java: Iterator<Entry> it = iterator();
        // Java: while (it.hasNext())
        {
            // Java: final Entry e = it.next();
            // Java: final int index = e.getIndex();
            // Java: result.setEntry(index, e.getValue() + result.getEntry(index));
        }
        // Java: return result;
        todo!();
    }

    // Java: RealVector mapAdd(double d)
    pub fn mapAdd(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: return copy().mapAddToSelf(d);
        todo!();
    }

    // Java: RealVector mapAddToSelf(double d)
    pub fn mapAddToSelf(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: if (d != 0)
        {
            // Java: return mapToSelf(FunctionUtils.fix2ndArgument(new Add(), d));
        }
        // Java: return this;
        todo!();
    }

    // Java: abstract RealVector copy()
    pub fn copy(&self) -> Box<dyn std::any::Any> {
        todo!();
    }

    // Java: double dotProduct(RealVector v)
    pub fn dotProduct(&self, v: &dyn std::any::Any) -> f64 {
        // Java: checkVectorDimensions(v);
        // Java: double d = 0;
        // Java: final int n = getDimension();
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: d += getEntry(i) * v.getEntry(i);
        }
        // Java: return d;
        todo!();
    }

    // Java: double cosine(RealVector v)
    pub fn cosine(&self, v: &dyn std::any::Any) -> f64 {
        todo!();
    }

    // Java: abstract RealVector ebeDivide(RealVector v)
    pub fn ebeDivide(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!();
    }

    // Java: abstract RealVector ebeMultiply(RealVector v)
    pub fn ebeMultiply(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!();
    }

    // Java: double getDistance(RealVector v)
    pub fn getDistance(&self, v: &dyn std::any::Any) -> f64 {
        // Java: checkVectorDimensions(v);
        // Java: double d = 0;
        // Java: Iterator<Entry> it = iterator();
        // Java: while (it.hasNext())
        {
            // Java: final Entry e = it.next();
            // Java: final double diff = e.getValue() - v.getEntry(e.getIndex());
            // Java: d += diff * diff;
        }
        // Java: return FastMath.sqrt(d);
        todo!();
    }

    // Java: double getNorm()
    pub fn getNorm(&self) -> f64 {
        // Java: double sum = 0;
        // Java: Iterator<Entry> it = iterator();
        // Java: while (it.hasNext())
        {
            // Java: final Entry e = it.next();
            // Java: final double value = e.getValue();
            // Java: sum += value * value;
        }
        // Java: return FastMath.sqrt(sum);
        todo!();
    }

    // Java: double getL1Norm()
    pub fn getL1Norm(&self) -> f64 {
        // Java: double norm = 0;
        // Java: Iterator<Entry> it = iterator();
        // Java: while (it.hasNext())
        {
            // Java: final Entry e = it.next();
            // Java: norm += FastMath.abs(e.getValue());
        }
        // Java: return norm;
        todo!();
    }

    // Java: double getLInfNorm()
    pub fn getLInfNorm(&self) -> f64 {
        // Java: double norm = 0;
        // Java: Iterator<Entry> it = iterator();
        // Java: while (it.hasNext())
        {
            // Java: final Entry e = it.next();
            // Java: norm = FastMath.max(norm, FastMath.abs(e.getValue()));
        }
        // Java: return norm;
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

    // Java: int getMinIndex()
    pub fn getMinIndex(&self) -> i32 {
        // Java: int minIndex    = -1;
        // Java: double minValue = Double.POSITIVE_INFINITY;
        // Java: Iterator<Entry> iterator = iterator();
        // Java: while (iterator.hasNext())
        {
            // Java: final Entry entry = iterator.next();
            // Java: if (entry.getValue() <= minValue)
            {
                // Java: minIndex = entry.getIndex();
                // Java: minValue = entry.getValue();
            }
        }
        // Java: return minIndex;
        todo!();
    }

    // Java: double getMinValue()
    pub fn getMinValue(&self) -> f64 {
        // Java: final int minIndex = getMinIndex();
        // Java: return minIndex < 0 ? Double.NaN : getEntry(minIndex);
        todo!();
    }

    // Java: int getMaxIndex()
    pub fn getMaxIndex(&self) -> i32 {
        // Java: int maxIndex    = -1;
        // Java: double maxValue = Double.NEGATIVE_INFINITY;
        // Java: Iterator<Entry> iterator = iterator();
        // Java: while (iterator.hasNext())
        {
            // Java: final Entry entry = iterator.next();
            // Java: if (entry.getValue() >= maxValue)
            {
                // Java: maxIndex = entry.getIndex();
                // Java: maxValue = entry.getValue();
            }
        }
        // Java: return maxIndex;
        todo!();
    }

    // Java: double getMaxValue()
    pub fn getMaxValue(&self) -> f64 {
        // Java: final int maxIndex = getMaxIndex();
        // Java: return maxIndex < 0 ? Double.NaN : getEntry(maxIndex);
        todo!();
    }

    // Java: RealVector mapMultiply(double d)
    pub fn mapMultiply(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: return copy().mapMultiplyToSelf(d);
        todo!();
    }

    // Java: RealVector mapMultiplyToSelf(double d)
    pub fn mapMultiplyToSelf(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: return mapToSelf(FunctionUtils.fix2ndArgument(new Multiply(), d));
        todo!();
    }

    // Java: RealVector mapSubtract(double d)
    pub fn mapSubtract(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: return copy().mapSubtractToSelf(d);
        todo!();
    }

    // Java: RealVector mapSubtractToSelf(double d)
    pub fn mapSubtractToSelf(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: return mapAddToSelf(-d);
        todo!();
    }

    // Java: RealVector mapDivide(double d)
    pub fn mapDivide(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: return copy().mapDivideToSelf(d);
        todo!();
    }

    // Java: RealVector mapDivideToSelf(double d)
    pub fn mapDivideToSelf(&self, d: f64) -> Box<dyn std::any::Any> {
        // Java: return mapToSelf(FunctionUtils.fix2ndArgument(new Divide(), d));
        todo!();
    }

    // Java: RealMatrix outerProduct(RealVector v)
    pub fn outerProduct(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final int m = this.getDimension();
        // Java: final int n = v.getDimension();
        // Java: final RealMatrix product;
        // Java: if (v instanceof SparseRealVector || this instanceof SparseRealVector)
        {
            // Java: product = new OpenMapRealMatrix(m, n);
        }
        // Java: else
        {
            // Java: product = new Array2DRowRealMatrix(m, n);
        }
        // Java: for (int i = 0; i < m; i++)
        {
            // Java: for (int j = 0; j < n; j++)
            {
                // Java: product.setEntry(i, j, this.getEntry(i) * v.getEntry(j));
            }
        }
        // Java: return product;
        todo!();
    }

    // Java: RealVector projection(final RealVector v)
    pub fn projection(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!();
    }

    // Java: void set(double value)
    pub fn set(&self, value: f64) {
        // Java: Iterator<Entry> it = iterator();
        // Java: while (it.hasNext())
        {
            // Java: final Entry e = it.next();
            // Java: e.setValue(value);
        }
    }

    // Java: double[] toArray()
    pub fn toArray(&self) -> Vec<f64> {
        // Java: int dim = getDimension();
        // Java: double[] values = new double[dim];
        // Java: for (int i = 0; i < dim; i++)
        {
            // Java: values[i] = getEntry(i);
        }
        // Java: return values;
        todo!();
    }

    // Java: RealVector unitVector()
    pub fn unitVector(&self) -> Box<dyn std::any::Any> {
        // Java: final double norm = getNorm();
        // Java: if (norm == 0)
        {
            // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NORM);
        }
        // Java: return mapDivide(norm);
        todo!();
    }

    // Java: void unitize()
    pub fn unitize(&self) {
        // Java: final double norm = getNorm();
        // Java: if (norm == 0)
        {
            // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NORM);
        }
        // Java: mapDivideToSelf(getNorm());
    }

    // Java: Iterator<Entry> sparseIterator()
    pub fn sparseIterator(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseEntryIterator();
        todo!();
    }

    // Java: Iterator<Entry> iterator()
    pub fn iterator(&self) -> Box<dyn std::any::Any> {
        // Java: final int dim = getDimension();
        // Java: return new Iterator<Entry>()
        {
            // Java: /** Current index. */
            // Java: private int i = 0;
            // Java: /** Current entry. */
            // Java: private Entry e = new Entry();
            // Java: /** {@inheritDoc} */
            // Java: public boolean hasNext()
            {
                // Java: return i < dim;
            }
            // Java: /** {@inheritDoc} */
            // Java: public Entry next()
            {
                // Java: if (i < dim)
                {
                    // Java: e.setIndex(i++);
                    // Java: return e;
                }
                // Java: else
                {
                    // Java: throw new NoSuchElementException();
                }
            }
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all circumstances.
             // Java: */
            // Java: public void remove() throws MathUnsupportedOperationException
            {
                // Java: throw new MathUnsupportedOperationException();
            }
        }
        todo!();
    }

    // Java: RealVector map(UnivariateFunction function)
    pub fn map(&self, function: &dyn UnivariateFunction) -> Box<dyn std::any::Any> {
        // Java: return copy().mapToSelf(function);
        todo!();
    }

    // Java: RealVector mapToSelf(UnivariateFunction function)
    pub fn mapToSelf(&self, function: &dyn UnivariateFunction) -> Box<dyn std::any::Any> {
        // Java: Iterator<Entry> it = iterator();
        // Java: while (it.hasNext())
        {
            // Java: final Entry e = it.next();
            // Java: e.setValue(function.value(e.getValue()));
        }
        // Java: return this;
        todo!();
    }

    // Java: RealVector combine(double a, double b, RealVector y)
    pub fn combine(&self, a: f64, b: f64, y: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!();
    }

    // Java: RealVector combineToSelf(double a, double b, RealVector y)
    pub fn combineToSelf(&self, a: f64, b: f64, y: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!();
    }

    // Java: double walkInDefaultOrder(final RealVectorPreservingVisitor visitor)
    pub fn walkInDefaultOrder(&self, visitor: &dyn std::any::Any) -> f64 {
        // Java: final int dim = getDimension();
        // Java: visitor.start(dim, 0, dim - 1);
        // Java: for (int i = 0; i < dim; i++)
        {
            // Java: visitor.visit(i, getEntry(i));
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
        // Java: final int dim = getDimension();
        // Java: visitor.start(dim, 0, dim - 1);
        // Java: for (int i = 0; i < dim; i++)
        {
            // Java: setEntry(i, visitor.visit(i, getEntry(i)));
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

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &dyn std::any::Any) -> bool {
        todo!();
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: throw new MathUnsupportedOperationException();
        todo!();
    }

    // Java: static RealVector unmodifiableRealVector(final RealVector v)
    pub fn unmodifiableRealVector(&self, v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: /**
         // Java: * This anonymous class is an implementation of {@link RealVector}
         // Java: * with read-only access.
         // Java: * It wraps any {@link RealVector}, and exposes all methods which
         // Java: * do not modify it. Invoking methods which should normally result
         // Java: * in the modification of the calling {@link RealVector} results in
         // Java: * a {@link MathUnsupportedOperationException}. It should be noted
         // Java: * that {@link UnmodifiableVector} is <em>not</em> immutable.
         // Java: */
        // Java: return new RealVector()
        {
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all circumstances.
             // Java: */
            // Java: @Override
            // Java: public RealVector mapToSelf(UnivariateFunction function)
                // Java: throws MathUnsupportedOperationException
                {
                // Java: throw new MathUnsupportedOperationException();
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector map(UnivariateFunction function)
            {
                // Java: return v.map(function);
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public Iterator<Entry> iterator()
            {
                // Java: final Iterator<Entry> i = v.iterator();
                // Java: return new Iterator<Entry>()
                {
                    // Java: /** The current entry. */
                    // Java: private final UnmodifiableEntry e = new UnmodifiableEntry();
                    // Java: /** {@inheritDoc} */
                    // Java: public boolean hasNext()
                    {
                        // Java: return i.hasNext();
                    }
                    // Java: /** {@inheritDoc} */
                    // Java: public Entry next()
                    {
                        // Java: e.setIndex(i.next().getIndex());
                        // Java: return e;
                    }
                    // Java: /**
                     // Java: * {@inheritDoc}
                     // Java: *
                     // Java: * @throws MathUnsupportedOperationException in all
                     // Java: * circumstances.
                     // Java: */
                    // Java: public void remove() throws MathUnsupportedOperationException
                    {
                        // Java: throw new MathUnsupportedOperationException();
                    }
                }
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public Iterator<Entry> sparseIterator()
            {
                // Java: final Iterator<Entry> i = v.sparseIterator();
                // Java: return new Iterator<Entry>()
                {
                    // Java: /** The current entry. */
                    // Java: private final UnmodifiableEntry e = new UnmodifiableEntry();
                    // Java: /** {@inheritDoc} */
                    // Java: public boolean hasNext()
                    {
                        // Java: return i.hasNext();
                    }
                    // Java: /** {@inheritDoc} */
                    // Java: public Entry next()
                    {
                        // Java: e.setIndex(i.next().getIndex());
                        // Java: return e;
                    }
                    // Java: /**
                     // Java: * {@inheritDoc}
                     // Java: *
                     // Java: * @throws MathUnsupportedOperationException in all
                     // Java: * circumstances.
                     // Java: */
                    // Java: public void remove()
                        // Java: throws MathUnsupportedOperationException
                        {
                        // Java: throw new MathUnsupportedOperationException();
                        }
                }
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector copy()
            {
                // Java: return v.copy();
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector add(RealVector w)
                // Java: throws DimensionMismatchException
                {
                // Java: return v.add(w);
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector subtract(RealVector w)
                // Java: throws DimensionMismatchException
                {
                // Java: return v.subtract(w);
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector mapAdd(double d)
            {
                // Java: return v.mapAdd(d);
            }
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all
             // Java: * circumstances.
             // Java: */
            // Java: @Override
            // Java: public RealVector mapAddToSelf(double d)
                // Java: throws MathUnsupportedOperationException
                {
                // Java: throw new MathUnsupportedOperationException();
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector mapSubtract(double d)
            {
                // Java: return v.mapSubtract(d);
            }
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all
             // Java: * circumstances.
             // Java: */
            // Java: @Override
            // Java: public RealVector mapSubtractToSelf(double d)
                // Java: throws MathUnsupportedOperationException
                {
                // Java: throw new MathUnsupportedOperationException();
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector mapMultiply(double d)
            {
                // Java: return v.mapMultiply(d);
            }
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all
             // Java: * circumstances.
             // Java: */
            // Java: @Override
            // Java: public RealVector mapMultiplyToSelf(double d)
                // Java: throws MathUnsupportedOperationException
                {
                // Java: throw new MathUnsupportedOperationException();
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector mapDivide(double d)
            {
                // Java: return v.mapDivide(d);
            }
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all
             // Java: * circumstances.
             // Java: */
            // Java: @Override
            // Java: public RealVector mapDivideToSelf(double d)
                // Java: throws MathUnsupportedOperationException
                {
                // Java: throw new MathUnsupportedOperationException();
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector ebeMultiply(RealVector w)
                // Java: throws DimensionMismatchException
                {
                // Java: return v.ebeMultiply(w);
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector ebeDivide(RealVector w)
                // Java: throws DimensionMismatchException
                {
                // Java: return v.ebeDivide(w);
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public double dotProduct(RealVector w)
                // Java: throws DimensionMismatchException
                {
                // Java: return v.dotProduct(w);
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public double cosine(RealVector w)
                // Java: throws DimensionMismatchException, MathArithmeticException
                {
                // Java: return v.cosine(w);
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public double getNorm()
            {
                // Java: return v.getNorm();
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public double getL1Norm()
            {
                // Java: return v.getL1Norm();
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public double getLInfNorm()
            {
                // Java: return v.getLInfNorm();
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public double getDistance(RealVector w)
                // Java: throws DimensionMismatchException
                {
                // Java: return v.getDistance(w);
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public double getL1Distance(RealVector w)
                // Java: throws DimensionMismatchException
                {
                // Java: return v.getL1Distance(w);
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public double getLInfDistance(RealVector w)
                // Java: throws DimensionMismatchException
                {
                // Java: return v.getLInfDistance(w);
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector unitVector() throws MathArithmeticException
            {
                // Java: return v.unitVector();
            }
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all
             // Java: * circumstances.
             // Java: */
            // Java: @Override
            // Java: public void unitize() throws MathUnsupportedOperationException
            {
                // Java: throw new MathUnsupportedOperationException();
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealMatrix outerProduct(RealVector w)
            {
                // Java: return v.outerProduct(w);
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public double getEntry(int index) throws OutOfRangeException
            {
                // Java: return v.getEntry(index);
            }
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all
             // Java: * circumstances.
             // Java: */
            // Java: @Override
            // Java: public void setEntry(int index, double value)
                // Java: throws MathUnsupportedOperationException
                {
                // Java: throw new MathUnsupportedOperationException();
                }
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all
             // Java: * circumstances.
             // Java: */
            // Java: @Override
            // Java: public void addToEntry(int index, double value)
                // Java: throws MathUnsupportedOperationException
                {
                // Java: throw new MathUnsupportedOperationException();
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public int getDimension()
            {
                // Java: return v.getDimension();
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector append(RealVector w)
            {
                // Java: return v.append(w);
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector append(double d)
            {
                // Java: return v.append(d);
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector getSubVector(int index, int n)
                // Java: throws OutOfRangeException, NotPositiveException
                {
                // Java: return v.getSubVector(index, n);
                }
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all
             // Java: * circumstances.
             // Java: */
            // Java: @Override
            // Java: public void setSubVector(int index, RealVector w)
                // Java: throws MathUnsupportedOperationException
                {
                // Java: throw new MathUnsupportedOperationException();
                }
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all
             // Java: * circumstances.
             // Java: */
            // Java: @Override
            // Java: public void set(double value)
                // Java: throws MathUnsupportedOperationException
                {
                // Java: throw new MathUnsupportedOperationException();
                }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public double[] toArray()
            {
                // Java: return v.toArray();
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public boolean isNaN()
            {
                // Java: return v.isNaN();
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public boolean isInfinite()
            {
                // Java: return v.isInfinite();
            }
            // Java: /** {@inheritDoc} */
            // Java: @Override
            // Java: public RealVector combine(double a, double b, RealVector y)
                // Java: throws DimensionMismatchException
                {
                // Java: return v.combine(a, b, y);
                }
            // Java: /**
             // Java: * {@inheritDoc}
             // Java: *
             // Java: * @throws MathUnsupportedOperationException in all
             // Java: * circumstances.
             // Java: */
            // Java: @Override
            // Java: public RealVector combineToSelf(double a, double b, RealVector y)
                // Java: throws MathUnsupportedOperationException
                {
                // Java: throw new MathUnsupportedOperationException();
                }
            // Java: /** An entry in the vector. */
            // Java: class UnmodifiableEntry extends Entry
            {
                // Java: /** {@inheritDoc} */
                // Java: @Override
                // Java: public double getValue()
                {
                    // Java: return v.getEntry(getIndex());
                }
                // Java: /**
                 // Java: * {@inheritDoc}
                 // Java: *
                 // Java: * @throws MathUnsupportedOperationException in all
                 // Java: * circumstances.
                 // Java: */
                // Java: @Override
                // Java: public void setValue(double value)
                    // Java: throws MathUnsupportedOperationException
                    {
                    // Java: throw new MathUnsupportedOperationException();
                    }
            }
        }
        todo!();
    }

}

// Java: public class Entry
impl Entry {
    // Java: Entry()
    pub fn Entry(&self) {
            // Java: setIndex(0);
    }

    // Java: double getValue()
    pub fn getValue(&self) -> f64 {
            // Java: return getEntry(getIndex());
        todo!();
    }

    // Java: void setValue(double value)
    pub fn setValue(&self, value: f64) {
            // Java: setEntry(getIndex(), value);
    }

    // Java: int getIndex()
    pub fn getIndex(&self) -> i32 {
            // Java: return index;
        todo!();
    }

    // Java: void setIndex(int index)
    pub fn setIndex(&self, index: i32) {
            // Java: this.index = index;
    }

}

// Java: public class SparseEntryIterator
impl SparseEntryIterator {
    // Java: SparseEntryIterator()
    pub fn SparseEntryIterator(&self) {
            // Java: dim = getDimension();
            // Java: current = new Entry();
            // Java: next = new Entry();
            // Java: if (next.getValue() == 0)
            {
                // Java: advance(next);
            }
    }

    // Java: void advance(Entry e)
    pub fn advance(&self, e: &Entry) {
            // Java: if (e == null)
            {
                // Java: return;
            }
            // Java: do
            {
                // Java: e.setIndex(e.getIndex() + 1);
            }
            // Java: if (e.getIndex() >= dim)
            {
                // Java: e.setIndex(-1);
            }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
            // Java: return next.getIndex() >= 0;
        todo!();
    }

    // Java: Entry next()
    pub fn next(&self) -> Entry {
            // Java: int index = next.getIndex();
            // Java: if (index < 0)
            {
                // Java: throw new NoSuchElementException();
            }
            // Java: current.setIndex(index);
            // Java: advance(next);
            // Java: return current;
        todo!();
    }

    // Java: void remove()
    pub fn remove(&self) {
            // Java: throw new MathUnsupportedOperationException();
    }

}
