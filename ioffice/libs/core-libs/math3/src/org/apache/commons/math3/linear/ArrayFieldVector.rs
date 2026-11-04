// Java: import java.io.Serializable
// Java: import java.util.Arrays
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
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID

// Java: public class implements implements the
pub struct implements {
}

// Java: public class ArrayFieldVector extends FieldElement implements FieldVector, Serializable
pub struct ArrayFieldVector {
    // Java field: T[] data
    pub(crate) data: Vec<Box<dyn std::any::Any>>,
    // Java field: Field<T> field
    pub(crate) field: Box<dyn std::any::Any>,
}

// Java: public class implements implements the
impl implements {
}

// Java: public class ArrayFieldVector extends FieldElement implements FieldVector, Serializable
impl ArrayFieldVector {
    // Java: ArrayFieldVector(final Field<T> field)
    pub fn new(field: Box<dyn std::any::Any>) -> Self {
        // Java: this(field, 0);
        ArrayFieldVector { data: Vec::new(), field }
    }

    // Java: ArrayFieldVector(Field<T> field, int size)
    pub fn new_with_size(field: Box<dyn std::any::Any>, _size: i32) -> Self {
        // Java: this.field = field;
        // Java: this.data  = MathArrays.buildArray(field, size);
        ArrayFieldVector { data: Vec::new(), field }
    }

    // Java: ArrayFieldVector(int size, T preset)
    pub fn new_with_preset(_size: i32, _preset: Box<dyn std::any::Any>) -> Self {
        // Java: this(preset.getField(), size);
        // Java: Arrays.fill(data, preset);
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(T[] d)
    pub fn new_from_array(_d: Vec<Box<dyn std::any::Any>>) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(Field<T> field, T[] d)
    pub fn new_from_field_array(field: Box<dyn std::any::Any>, _d: Vec<Box<dyn std::any::Any>>) -> Self {
        ArrayFieldVector { data: Vec::new(), field }
    }

    // Java: ArrayFieldVector(T[] d, boolean copyArray)
    pub fn new_from_array_copy(_d: Vec<Box<dyn std::any::Any>>, _copy_array: bool) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(Field<T> field, T[] d, boolean copyArray)
    pub fn new_from_field_array_copy(field: Box<dyn std::any::Any>, _d: Vec<Box<dyn std::any::Any>>, _copy_array: bool) -> Self {
        ArrayFieldVector { data: Vec::new(), field }
    }

    // Java: ArrayFieldVector(T[] d, int pos, int size)
    pub fn new_from_array_range(_d: Vec<Box<dyn std::any::Any>>, _pos: i32, _size: i32) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(Field<T> field, T[] d, int pos, int size)
    pub fn new_from_field_array_range(field: Box<dyn std::any::Any>, _d: Vec<Box<dyn std::any::Any>>, _pos: i32, _size: i32) -> Self {
        ArrayFieldVector { data: Vec::new(), field }
    }

    // Java: ArrayFieldVector(FieldVector<T> v)
    pub fn new_from_field_vector(_v: &dyn std::any::Any) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(ArrayFieldVector<T> v)
    pub fn new_from_array_field_vector(_v: &ArrayFieldVector) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(ArrayFieldVector<T> v, boolean deep)
    pub fn new_from_array_field_vector_deep(_v: &ArrayFieldVector, _deep: bool) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(ArrayFieldVector<T> v1, ArrayFieldVector<T> v2)
    pub fn new_from_two_array_vectors(_v1: &ArrayFieldVector, _v2: &ArrayFieldVector) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(FieldVector<T> v1, FieldVector<T> v2)
    pub fn new_from_two_field_vectors(_v1: &dyn std::any::Any, _v2: &dyn std::any::Any) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(ArrayFieldVector<T> v1, T[] v2)
    pub fn new_from_vector_and_array(_v1: &ArrayFieldVector, _v2: Vec<Box<dyn std::any::Any>>) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(FieldVector<T> v1, T[] v2)
    pub fn new_from_field_vector_and_array(_v1: &dyn std::any::Any, _v2: Vec<Box<dyn std::any::Any>>) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(T[] v1, ArrayFieldVector<T> v2)
    pub fn new_from_array_and_vector(_v1: Vec<Box<dyn std::any::Any>>, _v2: &ArrayFieldVector) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(T[] v1, FieldVector<T> v2)
    pub fn new_from_array_and_field_vector(_v1: Vec<Box<dyn std::any::Any>>, _v2: &dyn std::any::Any) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(T[] v1, T[] v2)
    pub fn new_from_two_arrays(_v1: Vec<Box<dyn std::any::Any>>, _v2: Vec<Box<dyn std::any::Any>>) -> Self {
        ArrayFieldVector { data: Vec::new(), field: Box::new(()) }
    }

    // Java: ArrayFieldVector(Field<T> field, T[] v1, T[] v2)
    pub fn new_from_field_two_arrays(field: Box<dyn std::any::Any>, _v1: Vec<Box<dyn std::any::Any>>, _v2: Vec<Box<dyn std::any::Any>>) -> Self {
        ArrayFieldVector { data: Vec::new(), field }
    }

    // Java: Field<T> getField()
    pub fn getField(&self) -> &dyn std::any::Any {
        // Java: return field;
        &*self.field
    }

    // Java: FieldVector<T> copy()
    pub fn copy(&self) -> Box<dyn std::any::Any> {
        // Java: return new ArrayFieldVector<T>(this, true);
        todo!("ArrayFieldVector::copy")
    }

    // Java: FieldVector<T> add(FieldVector<T> v)
    pub fn add(&self, _v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::add")
    }

    // Java: ArrayFieldVector<T> add(ArrayFieldVector<T> v)
    pub fn add_array(&self, _v: &ArrayFieldVector) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::add_array")
    }

    // Java: FieldVector<T> subtract(FieldVector<T> v)
    pub fn subtract(&self, _v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::subtract")
    }

    // Java: ArrayFieldVector<T> subtract(ArrayFieldVector<T> v)
    pub fn subtract_array(&self, _v: &ArrayFieldVector) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::subtract_array")
    }

    // Java: FieldVector<T> mapAdd(T d)
    pub fn mapAdd(&self, _d: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: T[] out = MathArrays.buildArray(field, data.length);
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: out[i] = data[i].add(d);
        }
        // Java: return new ArrayFieldVector<T>(field, out, false);
        todo!("ArrayFieldVector::mapAdd")
    }

    // Java: FieldVector<T> mapAddToSelf(T d)
    pub fn mapAddToSelf(&mut self, _d: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: data[i] = data[i].add(d);
        }
        // Java: return this;
        todo!("ArrayFieldVector::mapAddToSelf")
    }

    // Java: FieldVector<T> mapSubtract(T d)
    pub fn mapSubtract(&self, _d: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: T[] out = MathArrays.buildArray(field, data.length);
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: out[i] = data[i].subtract(d);
        }
        // Java: return new ArrayFieldVector<T>(field, out, false);
        todo!("ArrayFieldVector::mapSubtract")
    }

    // Java: FieldVector<T> mapSubtractToSelf(T d)
    pub fn mapSubtractToSelf(&mut self, _d: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: data[i] = data[i].subtract(d);
        }
        // Java: return this;
        todo!("ArrayFieldVector::mapSubtractToSelf")
    }

    // Java: FieldVector<T> mapMultiply(T d)
    pub fn mapMultiply(&self, _d: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: T[] out = MathArrays.buildArray(field, data.length);
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: out[i] = data[i].multiply(d);
        }
        // Java: return new ArrayFieldVector<T>(field, out, false);
        todo!("ArrayFieldVector::mapMultiply")
    }

    // Java: FieldVector<T> mapMultiplyToSelf(T d)
    pub fn mapMultiplyToSelf(&mut self, _d: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: data[i] = data[i].multiply(d);
        }
        // Java: return this;
        todo!("ArrayFieldVector::mapMultiplyToSelf")
    }

    // Java: FieldVector<T> mapDivide(T d)
    pub fn mapDivide(&self, _d: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::mapDivide")
    }

    // Java: FieldVector<T> mapDivideToSelf(T d)
    pub fn mapDivideToSelf(&mut self, _d: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::mapDivideToSelf")
    }

    // Java: FieldVector<T> mapInv()
    pub fn mapInv(&self) -> Box<dyn std::any::Any> {
        // Java: T[] out = MathArrays.buildArray(field, data.length);
        // Java: final T one = field.getOne();
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: try
            {
                // Java: out[i] = one.divide(data[i]);
            }
            // Java: catch (final MathArithmeticException e)
            {
                // Java: throw new MathArithmeticException(LocalizedFormats.INDEX, i);
            }
        }
        // Java: return new ArrayFieldVector<T>(field, out, false);
        todo!("ArrayFieldVector::mapInv")
    }

    // Java: FieldVector<T> mapInvToSelf()
    pub fn mapInvToSelf(&mut self) -> Box<dyn std::any::Any> {
        // Java: final T one = field.getOne();
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: try
            {
                // Java: data[i] = one.divide(data[i]);
            }
            // Java: catch (final MathArithmeticException e)
            {
                // Java: throw new MathArithmeticException(LocalizedFormats.INDEX, i);
            }
        }
        // Java: return this;
        todo!("ArrayFieldVector::mapInvToSelf")
    }

    // Java: FieldVector<T> ebeMultiply(FieldVector<T> v)
    pub fn ebeMultiply(&self, _v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::ebeMultiply")
    }

    // Java: ArrayFieldVector<T> ebeMultiply(ArrayFieldVector<T> v)
    pub fn ebeMultiply_array(&self, _v: &ArrayFieldVector) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::ebeMultiply_array")
    }

    // Java: FieldVector<T> ebeDivide(FieldVector<T> v)
    pub fn ebeDivide(&self, _v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::ebeDivide")
    }

    // Java: ArrayFieldVector<T> ebeDivide(ArrayFieldVector<T> v)
    pub fn ebeDivide_array(&self, _v: &ArrayFieldVector) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::ebeDivide_array")
    }

    // Java: T[] getData()
    pub fn getData(&self) -> &Vec<Box<dyn std::any::Any>> {
        // Java: return data.clone();
        &self.data
    }

    // Java: T[] getDataRef()
    pub fn getDataRef(&self) -> &Vec<Box<dyn std::any::Any>> {
        // Java: return data;
        &self.data
    }

    // Java: T dotProduct(FieldVector<T> v)
    pub fn dotProduct(&self, _v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::dotProduct")
    }

    // Java: T dotProduct(ArrayFieldVector<T> v)
    pub fn dotProduct_array(&self, _v: &ArrayFieldVector) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::dotProduct_array")
    }

    // Java: FieldVector<T> projection(FieldVector<T> v)
    pub fn projection(&self, _v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::projection")
    }

    // Java: ArrayFieldVector<T> projection(ArrayFieldVector<T> v)
    pub fn projection_array(&self, _v: &ArrayFieldVector) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::projection_array")
    }

    // Java: FieldMatrix<T> outerProduct(FieldVector<T> v)
    pub fn outerProduct(&self, _v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: try
        {
            // Java: return outerProduct((ArrayFieldVector<T>) v);
        }
        // Java: catch (ClassCastException cce)
        {
            // Java: final int m = data.length;
            // Java: final int n = v.getDimension();
            // Java: final FieldMatrix<T> out = new Array2DRowFieldMatrix<T>(field, m, n);
            // Java: for (int i = 0; i < m; i++)
            {
                // Java: for (int j = 0; j < n; j++)
                {
                    // Java: out.setEntry(i, j, data[i].multiply(v.getEntry(j)));
                }
            }
            // Java: return out;
        }
        todo!("ArrayFieldVector::outerProduct")
    }

    // Java: FieldMatrix<T> outerProduct(ArrayFieldVector<T> v)
    pub fn outerProduct_array(&self, _v: &ArrayFieldVector) -> Box<dyn std::any::Any> {
        // Java: final int m = data.length;
        // Java: final int n = v.data.length;
        // Java: final FieldMatrix<T> out = new Array2DRowFieldMatrix<T>(field, m, n);
        // Java: for (int i = 0; i < m; i++)
        {
            // Java: for (int j = 0; j < n; j++)
            {
                // Java: out.setEntry(i, j, data[i].multiply(v.data[j]));
            }
        }
        // Java: return out;
        todo!("ArrayFieldVector::outerProduct_array")
    }

    // Java: T getEntry(int index)
    pub fn getEntry(&self, _index: i32) -> Box<dyn std::any::Any> {
        // Java: return data[index];
        todo!("ArrayFieldVector::getEntry")
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return data.length;
        self.data.len() as i32
    }

    // Java: FieldVector<T> append(FieldVector<T> v)
    pub fn append(&self, _v: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: try
        {
            // Java: return append((ArrayFieldVector<T>) v);
        }
        // Java: catch (ClassCastException cce)
        {
            // Java: return new ArrayFieldVector<T>(this,new ArrayFieldVector<T>(v));
        }
        todo!("ArrayFieldVector::append")
    }

    // Java: ArrayFieldVector<T> append(ArrayFieldVector<T> v)
    pub fn append_array(&self, _v: &ArrayFieldVector) -> Box<dyn std::any::Any> {
        // Java: return new ArrayFieldVector<T>(this, v);
        todo!("ArrayFieldVector::append_array")
    }

    // Java: FieldVector<T> append(T in)
    pub fn append_element(&self, _in_val: Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
        // Java: final T[] out = MathArrays.buildArray(field, data.length + 1);
        // Java: System.arraycopy(data, 0, out, 0, data.length);
        // Java: out[data.length] = in;
        // Java: return new ArrayFieldVector<T>(field, out, false);
        todo!("ArrayFieldVector::append_element")
    }

    // Java: FieldVector<T> getSubVector(int index, int n)
    pub fn getSubVector(&self, _index: i32, _n: i32) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::getSubVector")
    }

    // Java: void setEntry(int index, T value)
    pub fn setEntry(&mut self, _index: i32, _value: Box<dyn std::any::Any>) {
        // Java: try
        {
            // Java: data[index] = value;
        }
        // Java: catch (IndexOutOfBoundsException e)
        {
            // Java: checkIndex(index);
        }
    }

    // Java: void setSubVector(int index, FieldVector<T> v)
    pub fn setSubVector(&mut self, _index: i32, _v: &dyn std::any::Any) {
        // Java: try
        {
            // Java: try
            {
                // Java: set(index, (ArrayFieldVector<T>) v);
            }
            // Java: catch (ClassCastException cce)
            {
                // Java: for (int i = index; i < index + v.getDimension(); ++i)
                {
                    // Java: data[i] = v.getEntry(i-index);
                }
            }
        }
        // Java: catch (IndexOutOfBoundsException e)
        {
            // Java: checkIndex(index);
            // Java: checkIndex(index + v.getDimension() - 1);
        }
    }

    // Java: void set(int index, ArrayFieldVector<T> v)
    pub fn set_array(&mut self, _index: i32, _v: &ArrayFieldVector) {
        // Java: try
        {
            // Java: System.arraycopy(v.data, 0, data, index, v.data.length);
        }
        // Java: catch (IndexOutOfBoundsException e)
        {
            // Java: checkIndex(index);
            // Java: checkIndex(index + v.data.length - 1);
        }
    }

    // Java: void set(T value)
    pub fn set(&mut self, _value: Box<dyn std::any::Any>) {
        // Java: Arrays.fill(data, value);
    }

    // Java: T[] toArray()
    pub fn toArray(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: return data.clone();
        Vec::new()
    }

    // Java: void checkVectorDimensions(FieldVector<T> v)
    pub fn checkVectorDimensions(&self, _v: &dyn std::any::Any) {
    }

    // Java: void checkVectorDimensions(int n)
    pub fn checkVectorDimensions_n(&self, _n: i32) {
    }

    // Java: T walkInDefaultOrder(final FieldVectorPreservingVisitor<T> visitor)
    pub fn walkInDefaultOrder(&self, _visitor: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final int dim = getDimension();
        // Java: visitor.start(dim, 0, dim - 1);
        // Java: for (int i = 0; i < dim; i++)
        {
            // Java: visitor.visit(i, getEntry(i));
        }
        // Java: return visitor.end();
        todo!("ArrayFieldVector::walkInDefaultOrder")
    }

    // Java: T walkInDefaultOrder()
    pub fn walkInDefaultOrder_range(&self, _visitor: &dyn std::any::Any, _start: i32, _end: i32) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::walkInDefaultOrder_range")
    }

    // Java: T walkInOptimizedOrder(final FieldVectorPreservingVisitor<T> visitor)
    pub fn walkInOptimizedOrder(&self, _visitor: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return walkInDefaultOrder(visitor);
        todo!("ArrayFieldVector::walkInOptimizedOrder")
    }

    // Java: T walkInOptimizedOrder()
    pub fn walkInOptimizedOrder_range(&self, _visitor: &dyn std::any::Any, _start: i32, _end: i32) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::walkInOptimizedOrder_range")
    }

    // Java: T walkInDefaultOrder(final FieldVectorChangingVisitor<T> visitor)
    pub fn walkInDefaultOrder_changing(&self, _visitor: &mut dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final int dim = getDimension();
        // Java: visitor.start(dim, 0, dim - 1);
        // Java: for (int i = 0; i < dim; i++)
        {
            // Java: setEntry(i, visitor.visit(i, getEntry(i)));
        }
        // Java: return visitor.end();
        todo!("ArrayFieldVector::walkInDefaultOrder_changing")
    }

    // Java: T walkInDefaultOrder()
    pub fn walkInDefaultOrder_changing_range(&self, _visitor: &mut dyn std::any::Any, _start: i32, _end: i32) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::walkInDefaultOrder_changing_range")
    }

    // Java: T walkInOptimizedOrder(final FieldVectorChangingVisitor<T> visitor)
    pub fn walkInOptimizedOrder_changing(&self, _visitor: &mut dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return walkInDefaultOrder(visitor);
        todo!("ArrayFieldVector::walkInOptimizedOrder_changing")
    }

    // Java: T walkInOptimizedOrder()
    pub fn walkInOptimizedOrder_changing_range(&self, _visitor: &mut dyn std::any::Any, _start: i32, _end: i32) -> Box<dyn std::any::Any> {
        todo!("ArrayFieldVector::walkInOptimizedOrder_changing_range")
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, _other: &dyn std::any::Any) -> bool {
        // Java: if (this == other)
        {
            // Java: return true;
        }
        // Java: if (other == null)
        {
            // Java: return false;
        }
        // Java: try
        {
            // Java: @SuppressWarnings("unchecked") // May fail, but we ignore ClassCastException
                // Java: FieldVector<T> rhs = (FieldVector<T>) other;
            // Java: if (data.length != rhs.getDimension())
            {
                // Java: return false;
            }
            // Java: for (int i = 0; i < data.length; ++i)
            {
                // Java: if (!data[i].equals(rhs.getEntry(i)))
                {
                    // Java: return false;
                }
            }
            // Java: return true;
        }
        // Java: catch (ClassCastException ex)
        {
            // Java: // ignore exception
            // Java: return false;
        }
        false
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int h = 3542;
        let h: i32 = 3542;
        // Java: for (final T a : data)
        for _a in &self.data {
            // Java: h ^= a.hashCode();
        }
        // Java: return h;
        h
    }

    // Java: void checkIndex(final int index)
    pub fn checkIndex(&self, _index: i32) {
        // Java: if (index < 0 || index >= getDimension())
        {
            // Java: throw new OutOfRangeException(LocalizedFormats.INDEX,
                                          // Java: index, 0, getDimension() - 1);
        }
    }

    // Java: void checkIndices(final int start, final int end)
    pub fn checkIndices(&self, _start: i32, _end: i32) {
    }

}
