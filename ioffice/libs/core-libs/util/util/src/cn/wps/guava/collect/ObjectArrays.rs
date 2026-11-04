#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;

// Java: public class ObjectArrays
pub struct ObjectArrays;

// Java: public class ObjectArrays
impl ObjectArrays {
    // Java field: Object[] EMPTY_ARRAY
    pub const fn empty_array() -> &'static [Option<Box<dyn Any>>] {
        &[]
    }

    // Java: ObjectArrays()
    // Java: super();

    // Java: static public static <T> T[] newArray(Class<T> type, int length)
    pub fn newArray_typed(length: usize) -> Vec<Option<Box<dyn Any>>> {
        // Java: return (T[])((Object[])Array.newInstance(type, length));
        (0..length).map(|_| None).collect()
    }

    // Java: static public static <T> T[] newArray(T[] reference, int length)
    pub fn newArray(_reference: &[Option<Box<dyn Any>>], length: usize) -> Vec<Option<Box<dyn Any>>> {
        // Java: Class<?> type = reference.getClass().getComponentType();
        // Java: T[] result = (T[])((Object[])Array.newInstance(type, length));
        // Java: return result;
        (0..length).map(|_| None).collect()
    }

    // Java: static public static <T> T[] concat(T[] first, T[] second, Class<T> type)
    pub fn concat_arrays(first: &[Option<Box<dyn Any>>], second: &[Option<Box<dyn Any>>]) -> Vec<Option<Box<dyn Any>>> {
        // Java: T[] result = (T[])newArray(type, first.length + second.length);
        let mut result: Vec<Option<Box<dyn Any>>> = Vec::with_capacity(first.len() + second.len());
        // Java: System.arraycopy(first, 0, result, 0, first.length);
        // Java: System.arraycopy(second, 0, result, first.length, second.length);
        for _ in first { result.push(None); }
        for _ in second { result.push(None); }
        // Java: return result;
        result
    }

    // Java: static public static <T> T[] concat(T element, T[] array)
    pub fn concat_prepend(element: Option<Box<dyn Any>>, array: &[Option<Box<dyn Any>>]) -> Vec<Option<Box<dyn Any>>> {
        // Java: T[] result = (T[])newArray(array, array.length + 1);
        let mut result: Vec<Option<Box<dyn Any>>> = Vec::with_capacity(array.len() + 1);
        // Java: result[0] = element;
        result.push(element);
        // Java: System.arraycopy(array, 0, result, 1, array.length);
        for _ in array { result.push(None); }
        // Java: return result;
        result
    }

    // Java: static public static <T> T[] concat(T[] array, T element)
    pub fn concat_append(array: &[Option<Box<dyn Any>>], element: Option<Box<dyn Any>>) -> Vec<Option<Box<dyn Any>>> {
        // Java: T[] result = (T[])arraysCopyOf(array, array.length + 1);
        let mut result = Self::arraysCopyOf(array, array.len() + 1);
        // Java: result[array.length] = element;
        if array.len() < result.len() {
            result[array.len()] = element;
        }
        // Java: return result;
        result
    }

    // Java: static <T> T[] arraysCopyOf(T[] original, int newLength)
    pub fn arraysCopyOf(original: &[Option<Box<dyn Any>>], new_length: usize) -> Vec<Option<Box<dyn Any>>> {
        // Java: T[] copy = (T[])newArray(original, newLength);
        let mut copy: Vec<Option<Box<dyn Any>>> = (0..new_length).map(|_| None).collect();
        // Java: System.arraycopy(original, 0, copy, 0, Math.min(original.length, newLength));
        let copy_len = original.len().min(new_length);
        for i in 0..copy_len {
            copy[i] = None;
        }
        // Java: return copy;
        copy
    }

    // Java: static <T> T[] toArrayImpl(Collection<?> c, T[] array)
    pub fn toArrayImpl_collection(c_size: usize, array: &mut Vec<Option<Box<dyn Any>>>) {
        // Java: int size = c.size();
        let size = c_size;
        // Java: if (array.length < size)
        if array.len() < size {
            // Java: array = (T[])newArray(array, size);
            array.resize_with(size, || None);
        }
        // Java: fillArray(c, array);
        // Java: if (array.length > size)
        if array.len() > size {
            // Java: array[size] = null;
            array[size] = None;
        }
        // Java: return array;
    }

    // Java: static <T> T[] toArrayImpl(Object[] src, int offset, int len, T[] dst)
    pub fn toArrayImpl_src(src: &[Option<Box<dyn Any>>], offset: usize, len: usize, dst: &mut Vec<Option<Box<dyn Any>>>) {
        // Java: Preconditions.checkPositionIndexes(offset, offset + len, src.length);
        assert!(offset + len <= src.len());
        // Java: if (dst.length < len)
        if dst.len() < len {
            // Java: dst = (T[])newArray(dst, len);
            dst.resize_with(len, || None);
        }
        // Java: else if (dst.length > len)
        else if dst.len() > len {
            // Java: dst[len] = null;
            dst[len] = None;
        }
        // Java: System.arraycopy(src, offset, dst, 0, len);
        // Java: return dst;
    }

    // Java: static Object[] toArrayImpl(Collection<?> c)
    pub fn toArrayImpl_simple(c_size: usize) -> Vec<Option<Box<dyn Any>>> {
        // Java: return fillArray(c, new Object[c.size()]);
        (0..c_size).map(|_| None).collect()
    }

    // Java: static Object[] copyAsObjectArray(Object[] elements, int offset, int length)
    pub fn copyAsObjectArray(elements: &[Option<Box<dyn Any>>], offset: usize, length: usize) -> Vec<Option<Box<dyn Any>>> {
        // Java: Preconditions.checkPositionIndexes(offset, offset + length, elements.length);
        assert!(offset + length <= elements.len());
        // Java: if (length == 0)
        if length == 0 {
            // Java: return EMPTY_ARRAY;
            return Vec::new();
        }
        // Java: else
        {
            // Java: Object[] result = new Object[length];
            // Java: System.arraycopy(elements, offset, result, 0, length);
            // Java: return result;
            (0..length).map(|_| None).collect()
        }
    }

    // Java: static Object[] fillArray(Iterable<?> elements, Object[] array)
    fn fillArray(elements: &[Option<Box<dyn Any>>], array: &mut Vec<Option<Box<dyn Any>>>) {
        // Java: int i = 0;
        let mut i = 0usize;
        // Java: for(Object element : elements)
        for _element in elements.iter() {
            // Java: array[i++] = element;
            if i < array.len() {
                array[i] = None;
            }
            i += 1;
        }
        // Java: return array;
    }

    // Java: static void swap(Object[] array, int i, int j)
    pub fn swap(array: &mut Vec<Option<Box<dyn Any>>>, i: usize, j: usize) {
        // Java: Object temp = array[i];
        // Java: array[i] = array[j];
        // Java: array[j] = temp;
        array.swap(i, j);
    }

    // Java: static Object[] checkElementsNotNull(Object... array)
    pub fn checkElementsNotNull(array: &[Option<Box<dyn Any>>]) {
        // Java: return checkElementsNotNull(array, array.length);
        Self::checkElementsNotNull_len(array, array.len());
    }

    // Java: static Object[] checkElementsNotNull(Object[] array, int length)
    pub fn checkElementsNotNull_len(array: &[Option<Box<dyn Any>>], length: usize) {
        // Java: for(int i = 0; i < length; ++i)
        for i in 0..length {
            // Java: checkElementNotNull(array[i], i);
            Self::checkElementNotNull(array[i].as_deref(), i);
        }
        // Java: return array;
    }

    // Java: static Object checkElementNotNull(Object element, int index)
    pub fn checkElementNotNull(element: Option<&dyn Any>, index: usize) {
        // Java: if (element == null)
        if element.is_none() {
            // Java: throw new NullPointerException("at index " + index);
            panic!("at index {}", index);
        }
        // Java: else
        {
            // Java: return element;
        }
    }
}
