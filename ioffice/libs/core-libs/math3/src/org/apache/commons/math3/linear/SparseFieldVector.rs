// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.OpenIntToFieldHashMap
use crate::org::apache::commons::math3::util::OpenIntToFieldHashMap;

// Java field: long serialVersionUID

// Java: public class implements implements the
pub struct implements {
}

// Java: public class SparseFieldVector extends FieldElement implements FieldVector, Serializable
pub struct SparseFieldVector {
    // Java field: Field<T> field
    pub(crate) field: Box<dyn std::any::Any>,
    // Java field: OpenIntToFieldHashMap<T> entries
    pub(crate) entries: Box<dyn std::any::Any>,
    // Java field: int virtualSize
    pub(crate) virtualSize: i32,
}

// Java: public class implements implements the
impl implements {
}


// Java: public class SparseFieldVector extends FieldElement implements FieldVector, Serializable
impl SparseFieldVector {
    // Java: SparseFieldVector(Field<T> field)
    pub fn SparseFieldVector(&self) {
        // Java: this(field, 0);
    }

    // Java: SparseFieldVector(Field<T> field, int dimension)
    pub fn SparseFieldVector_2(&self) {
        // Java: this.field = field;
        // Java: virtualSize = dimension;
        // Java: entries = new OpenIntToFieldHashMap<T>(field);
    }

    // Java: SparseFieldVector(SparseFieldVector<T> v, int resize)
    pub fn SparseFieldVector_3(&self) {
        // Java: field = v.field;
        // Java: virtualSize = v.getDimension() + resize;
        // Java: entries = new OpenIntToFieldHashMap<T>(v.entries);
    }

    // Java: SparseFieldVector(Field<T> field, int dimension, int expectedSize)
    pub fn SparseFieldVector_4(&self) {
        // Java: this.field = field;
        // Java: virtualSize = dimension;
        // Java: entries = new OpenIntToFieldHashMap<T>(field,expectedSize);
    }

    // Java: SparseFieldVector(Field<T> field, T[] values)
    pub fn SparseFieldVector_5(&self) {
        // Java: MathUtils.checkNotNull(values);
        // Java: this.field = field;
        // Java: virtualSize = values.length;
        // Java: entries = new OpenIntToFieldHashMap<T>(field);
        // Java: for (int key = 0; key < values.length; key++)
        {
            // Java: T value = values[key];
            // Java: entries.put(key, value);
        }
    }

    // Java: SparseFieldVector(SparseFieldVector<T> v)
    pub fn SparseFieldVector_6(&self) {
        // Java: field = v.field;
        // Java: virtualSize = v.getDimension();
        // Java: entries = new OpenIntToFieldHashMap<T>(v.getEntries());
    }

    // Java: OpenIntToFieldHashMap<T> getEntries()
    pub fn getEntries(&self) -> Box<dyn std::any::Any> {
        // Java: return entries;
        todo!()
    }

    // Java: FieldVector<T> add(SparseFieldVector<T> v)
    pub fn add(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: FieldVector<T> append(SparseFieldVector<T> v)
    pub fn append(&self) -> Box<dyn std::any::Any> {
        // Java: SparseFieldVector<T> res = new SparseFieldVector<T>(this, v.getDimension());
        // Java: OpenIntToFieldHashMap<T>.Iterator iter = v.entries.iterator();
        // Java: while (iter.hasNext())
        {
            // Java: iter.advance();
            // Java: res.setEntry(iter.key() + virtualSize, iter.value());
        }
        // Java: return res;
        todo!()
    }

    // Java: FieldVector<T> append(FieldVector<T> v)
    pub fn append_2(&self) -> Box<dyn std::any::Any> {
        // Java: if (v instanceof SparseFieldVector<?>)
        {
            // Java: return append((SparseFieldVector<T>) v);
        }
        // Java: else
        {
            // Java: final int n = v.getDimension();
            // Java: FieldVector<T> res = new SparseFieldVector<T>(this, n);
            // Java: for (int i = 0; i < n; i++)
            {
                // Java: res.setEntry(i + virtualSize, v.getEntry(i));
            }
            // Java: return res;
        }
        todo!()
    }

    // Java: FieldVector<T> append(T d)
    pub fn append_3(&self) -> Box<dyn std::any::Any> {
        // Java: MathUtils.checkNotNull(d);
        // Java: FieldVector<T> res = new SparseFieldVector<T>(this, 1);
        // Java: res.setEntry(virtualSize, d);
        // Java: return res;
        todo!()
    }

    // Java: FieldVector<T> copy()
    pub fn copy(&self) -> Box<dyn std::any::Any> {
        // Java: return new SparseFieldVector<T>(this);
        todo!()
    }

    // Java: T dotProduct(FieldVector<T> v)
    pub fn dotProduct(&self) -> Box<dyn std::any::Any> {
        // Java: checkVectorDimensions(v.getDimension());
        // Java: T res = field.getZero();
        // Java: OpenIntToFieldHashMap<T>.Iterator iter = entries.iterator();
        // Java: while (iter.hasNext())
        {
            // Java: iter.advance();
            // Java: res = res.add(v.getEntry(iter.key()).multiply(iter.value()));
        }
        // Java: return res;
        todo!()
    }

    // Java: FieldVector<T> ebeDivide(FieldVector<T> v)
    pub fn ebeDivide(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: FieldVector<T> ebeMultiply(FieldVector<T> v)
    pub fn ebeMultiply(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T[] getData()
    pub fn getData(&self) -> Box<dyn std::any::Any> {
        // Java: return toArray();
        todo!()
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return virtualSize;
        todo!()
    }

    // Java: T getEntry(int index)
    pub fn getEntry(&self) -> Box<dyn std::any::Any> {
        // Java: checkIndex(index);
        // Java: return entries.get(index);
        todo!()
    }

    // Java: Field<T> getField()
    pub fn getField(&self) -> Box<dyn std::any::Any> {
        // Java: return field;
        todo!()
    }

    // Java: FieldVector<T> getSubVector(int index, int n)
    pub fn getSubVector(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: FieldVector<T> mapAdd(T d)
    pub fn mapAdd(&self) -> Box<dyn std::any::Any> {
        // Java: return copy().mapAddToSelf(d);
        todo!()
    }

    // Java: FieldVector<T> mapAddToSelf(T d)
    pub fn mapAddToSelf(&self) -> Box<dyn std::any::Any> {
        // Java: for (int i = 0; i < virtualSize; i++)
        {
            // Java: setEntry(i, getEntry(i).add(d));
        }
        // Java: return this;
        todo!()
    }

    // Java: FieldVector<T> mapDivide(T d)
    pub fn mapDivide(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: FieldVector<T> mapDivideToSelf(T d)
    pub fn mapDivideToSelf(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: FieldVector<T> mapInv()
    pub fn mapInv(&self) -> Box<dyn std::any::Any> {
        // Java: return copy().mapInvToSelf();
        todo!()
    }

    // Java: FieldVector<T> mapInvToSelf()
    pub fn mapInvToSelf(&self) -> Box<dyn std::any::Any> {
        // Java: for (int i = 0; i < virtualSize; i++)
        {
            // Java: setEntry(i, field.getOne().divide(getEntry(i)));
        }
        // Java: return this;
        todo!()
    }

    // Java: FieldVector<T> mapMultiply(T d)
    pub fn mapMultiply(&self) -> Box<dyn std::any::Any> {
        // Java: return copy().mapMultiplyToSelf(d);
        todo!()
    }

    // Java: FieldVector<T> mapMultiplyToSelf(T d)
    pub fn mapMultiplyToSelf(&self) -> Box<dyn std::any::Any> {
        // Java: OpenIntToFieldHashMap<T>.Iterator iter = entries.iterator();
        // Java: while (iter.hasNext())
        {
            // Java: iter.advance();
            // Java: entries.put(iter.key(), iter.value().multiply(d));
        }
        // Java: return this;
        todo!()
    }

    // Java: FieldVector<T> mapSubtract(T d)
    pub fn mapSubtract(&self) -> Box<dyn std::any::Any> {
        // Java: return copy().mapSubtractToSelf(d);
        todo!()
    }

    // Java: FieldVector<T> mapSubtractToSelf(T d)
    pub fn mapSubtractToSelf(&self) -> Box<dyn std::any::Any> {
        // Java: return mapAddToSelf(field.getZero().subtract(d));
        todo!()
    }

    // Java: FieldMatrix<T> outerProduct(SparseFieldVector<T> v)
    pub fn outerProduct(&self) -> Box<dyn std::any::Any> {
        // Java: final int n = v.getDimension();
        // Java: SparseFieldMatrix<T> res = new SparseFieldMatrix<T>(field, virtualSize, n);
        // Java: OpenIntToFieldHashMap<T>.Iterator iter = entries.iterator();
        // Java: while (iter.hasNext())
        {
            // Java: iter.advance();
            // Java: OpenIntToFieldHashMap<T>.Iterator iter2 = v.entries.iterator();
            // Java: while (iter2.hasNext())
            {
                // Java: iter2.advance();
                // Java: res.setEntry(iter.key(), iter2.key(), iter.value().multiply(iter2.value()));
            }
        }
        // Java: return res;
        todo!()
    }

    // Java: FieldMatrix<T> outerProduct(FieldVector<T> v)
    pub fn outerProduct_2(&self) -> Box<dyn std::any::Any> {
        // Java: if (v instanceof SparseFieldVector<?>)
        {
            // Java: return outerProduct((SparseFieldVector<T>)v);
        }
        // Java: else
        {
            // Java: final int n = v.getDimension();
            // Java: FieldMatrix<T> res = new SparseFieldMatrix<T>(field, virtualSize, n);
            // Java: OpenIntToFieldHashMap<T>.Iterator iter = entries.iterator();
            // Java: while (iter.hasNext())
            {
                // Java: iter.advance();
                // Java: int row = iter.key();
                // Java: FieldElement<T>value = iter.value();
                // Java: for (int col = 0; col < n; col++)
                {
                    // Java: res.setEntry(row, col, value.multiply(v.getEntry(col)));
                }
            }
            // Java: return res;
        }
        todo!()
    }

    // Java: FieldVector<T> projection(FieldVector<T> v)
    pub fn projection(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void set(T value)
    pub fn set(&self) {
        // Java: MathUtils.checkNotNull(value);
        // Java: for (int i = 0; i < virtualSize; i++)
        {
            // Java: setEntry(i, value);
        }
    }

    // Java: void setEntry(int index, T value)
    pub fn setEntry(&self) {
        // Java: MathUtils.checkNotNull(value);
        // Java: checkIndex(index);
        // Java: entries.put(index, value);
    }

    // Java: void setSubVector(int index, FieldVector<T> v)
    pub fn setSubVector(&self) {
    }

    // Java: SparseFieldVector<T> subtract(SparseFieldVector<T> v)
    pub fn subtract(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: FieldVector<T> subtract(FieldVector<T> v)
    pub fn subtract_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T[] toArray()
    pub fn toArray(&self) -> Box<dyn std::any::Any> {
        // Java: T[] res = MathArrays.buildArray(field, virtualSize);
        // Java: OpenIntToFieldHashMap<T>.Iterator iter = entries.iterator();
        // Java: while (iter.hasNext())
        {
            // Java: iter.advance();
            // Java: res[iter.key()] = iter.value();
        }
        // Java: return res;
        todo!()
    }

    // Java: void checkIndex(final int index)
    pub fn checkIndex(&self) {
        // Java: if (index < 0 || index >= getDimension())
        {
            // Java: throw new OutOfRangeException(index, 0, getDimension() - 1);
        }
    }

    // Java: void checkIndices(final int start, final int end)
    pub fn checkIndices(&self) {
    }

    // Java: void checkVectorDimensions(int n)
    pub fn checkVectorDimensions(&self) {
    }

    // Java: FieldVector<T> add(FieldVector<T> v)
    pub fn add_2(&self) -> Box<dyn std::any::Any> {
        // Java: if (v instanceof SparseFieldVector<?>)
        {
            // Java: return add((SparseFieldVector<T>) v);
        }
        // Java: else
        {
            // Java: final int n = v.getDimension();
            // Java: checkVectorDimensions(n);
            // Java: SparseFieldVector<T> res = new SparseFieldVector<T>(field,
                                                                // Java: getDimension());
            // Java: for (int i = 0; i < n; i++)
            {
                // Java: res.setEntry(i, v.getEntry(i).add(getEntry(i)));
            }
            // Java: return res;
        }
        todo!()
    }

    // Java: T walkInDefaultOrder(final FieldVectorPreservingVisitor<T> visitor)
    pub fn walkInDefaultOrder(&self) -> Box<dyn std::any::Any> {
        // Java: final int dim = getDimension();
        // Java: visitor.start(dim, 0, dim - 1);
        // Java: for (int i = 0; i < dim; i++)
        {
            // Java: visitor.visit(i, getEntry(i));
        }
        // Java: return visitor.end();
        todo!()
    }

    // Java: T walkInDefaultOrder()
    pub fn walkInDefaultOrder_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T walkInOptimizedOrder(final FieldVectorPreservingVisitor<T> visitor)
    pub fn walkInOptimizedOrder(&self) -> Box<dyn std::any::Any> {
        // Java: return walkInDefaultOrder(visitor);
        todo!()
    }

    // Java: T walkInOptimizedOrder()
    pub fn walkInOptimizedOrder_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T walkInDefaultOrder(final FieldVectorChangingVisitor<T> visitor)
    pub fn walkInDefaultOrder_3(&self) -> Box<dyn std::any::Any> {
        // Java: final int dim = getDimension();
        // Java: visitor.start(dim, 0, dim - 1);
        // Java: for (int i = 0; i < dim; i++)
        {
            // Java: setEntry(i, visitor.visit(i, getEntry(i)));
        }
        // Java: return visitor.end();
        todo!()
    }

    // Java: T walkInDefaultOrder()
    pub fn walkInDefaultOrder_4(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T walkInOptimizedOrder(final FieldVectorChangingVisitor<T> visitor)
    pub fn walkInOptimizedOrder_3(&self) -> Box<dyn std::any::Any> {
        // Java: return walkInDefaultOrder(visitor);
        todo!()
    }

    // Java: T walkInOptimizedOrder()
    pub fn walkInOptimizedOrder_4(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: final int prime = 31;
        // Java: int result = 1;
        // Java: result = prime * result + ((field == null) ? 0 : field.hashCode());
        // Java: result = prime * result + virtualSize;
        // Java: OpenIntToFieldHashMap<T>.Iterator iter = entries.iterator();
        // Java: while (iter.hasNext())
        {
            // Java: iter.advance();
            // Java: int temp = iter.value().hashCode();
            // Java: result = prime * result + temp;
        }
        // Java: return result;
        todo!()
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self) -> bool {
        // Java: if (this == obj)
        {
            // Java: return true;
        }
        // Java: if (!(obj instanceof SparseFieldVector<?>))
        {
            // Java: return false;
        }
        // Java: @SuppressWarnings("unchecked") // OK, because "else if" check below ensures that
                                       // Java: // other must be the same type as this
        // Java: SparseFieldVector<T> other = (SparseFieldVector<T>) obj;
        // Java: if (field == null)
        {
            // Java: if (other.field != null)
            {
                // Java: return false;
            }
        }
        // Java: else if (!field.equals(other.field))
        {
            // Java: return false;
        }
        // Java: if (virtualSize != other.virtualSize)
        {
            // Java: return false;
        }
        // Java: OpenIntToFieldHashMap<T>.Iterator iter = entries.iterator();
        // Java: while (iter.hasNext())
        {
            // Java: iter.advance();
            // Java: T test = other.getEntry(iter.key());
            // Java: if (!test.equals(iter.value()))
            {
                // Java: return false;
            }
        }
        // Java: iter = other.getEntries().iterator();
        // Java: while (iter.hasNext())
        {
            // Java: iter.advance();
            // Java: T test = iter.value();
            // Java: if (!test.equals(getEntry(iter.key())))
            {
                // Java: return false;
            }
        }
        // Java: return true;
        todo!()
    }

}


