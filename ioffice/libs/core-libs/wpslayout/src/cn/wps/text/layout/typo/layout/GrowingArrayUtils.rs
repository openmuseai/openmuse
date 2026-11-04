#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: public class GrowingArrayUtils
pub struct GrowingArrayUtils {
}

// Java: public class GrowingArrayUtils
impl GrowingArrayUtils {
    // Java: static public static <T> T[] append(T[] array, int currentSize, T element)
    pub fn append_generic<T: Default + Clone>(mut array: Vec<T>, _currentSize: i32, _element: T) -> Vec<T> {
        // Java: assert currentSize <= array.length;
        assert!(_currentSize <= array.len() as i32);
        // Java: if (currentSize + 1 > array.length)
        if _currentSize as usize + 1 > array.len()
        {
            // Java: T[] newArray = (T[])ArrayUtils.newUnpaddedArray(array.getClass().getComponentType(), growSize(currentSize));
            let new_size = GrowingArrayUtils::growSize(_currentSize) as usize;
            // Java: System.arraycopy(array, 0, newArray, 0, currentSize);
            array.resize(new_size, T::default());
            // Java: array = newArray;
        }
        // Java: array[currentSize] = element;
        if (_currentSize as usize) < array.len() {
            array[_currentSize as usize] = _element;
        } else {
            array.push(_element);
        }
        // Java: return array;
        array
    }

    // Java: static int[] append(int[] array, int currentSize, int element)
    pub fn append_int(mut array: Vec<i32>, _currentSize: i32, _element: i32) -> Vec<i32> {
        // Java: assert currentSize <= array.length;
        assert!(_currentSize <= array.len() as i32);
        // Java: if (currentSize + 1 > array.length)
        if _currentSize as usize + 1 > array.len()
        {
            // Java: int[] newArray = ArrayUtils.newUnpaddedIntArray(growSize(currentSize));
            let new_size = GrowingArrayUtils::growSize(_currentSize) as usize;
            // Java: System.arraycopy(array, 0, newArray, 0, currentSize);
            array.resize(new_size, 0);
            // Java: array = newArray;
        }
        // Java: array[currentSize] = element;
        if (_currentSize as usize) < array.len() {
            array[_currentSize as usize] = _element;
        } else {
            array.push(_element);
        }
        // Java: return array;
        array
    }

    // Java: static long[] append(long[] array, int currentSize, long element)
    pub fn append_long(mut array: Vec<i64>, _currentSize: i32, _element: i64) -> Vec<i64> {
        // Java: assert currentSize <= array.length;
        assert!(_currentSize <= array.len() as i32);
        // Java: if (currentSize + 1 > array.length)
        if _currentSize as usize + 1 > array.len()
        {
            // Java: long[] newArray = ArrayUtils.newUnpaddedLongArray(growSize(currentSize));
            let new_size = GrowingArrayUtils::growSize(_currentSize) as usize;
            // Java: System.arraycopy(array, 0, newArray, 0, currentSize);
            array.resize(new_size, 0);
            // Java: array = newArray;
        }
        // Java: array[currentSize] = element;
        if (_currentSize as usize) < array.len() {
            array[_currentSize as usize] = _element;
        } else {
            array.push(_element);
        }
        // Java: return array;
        array
    }

    // Java: static boolean[] append(boolean[] array, int currentSize, boolean element)
    pub fn append_bool(mut array: Vec<bool>, _currentSize: i32, _element: bool) -> Vec<bool> {
        // Java: assert currentSize <= array.length;
        assert!(_currentSize <= array.len() as i32);
        // Java: if (currentSize + 1 > array.length)
        if _currentSize as usize + 1 > array.len()
        {
            // Java: boolean[] newArray = ArrayUtils.newUnpaddedBooleanArray(growSize(currentSize));
            let new_size = GrowingArrayUtils::growSize(_currentSize) as usize;
            // Java: System.arraycopy(array, 0, newArray, 0, currentSize);
            array.resize(new_size, false);
            // Java: array = newArray;
        }
        // Java: array[currentSize] = element;
        if (_currentSize as usize) < array.len() {
            array[_currentSize as usize] = _element;
        } else {
            array.push(_element);
        }
        // Java: return array;
        array
    }

    // Java: static float[] append(float[] array, int currentSize, float element)
    pub fn append_float(mut array: Vec<f32>, _currentSize: i32, _element: f32) -> Vec<f32> {
        // Java: assert currentSize <= array.length;
        assert!(_currentSize <= array.len() as i32);
        // Java: if (currentSize + 1 > array.length)
        if _currentSize as usize + 1 > array.len()
        {
            // Java: float[] newArray = ArrayUtils.newUnpaddedFloatArray(growSize(currentSize));
            let new_size = GrowingArrayUtils::growSize(_currentSize) as usize;
            // Java: System.arraycopy(array, 0, newArray, 0, currentSize);
            array.resize(new_size, 0.0);
            // Java: array = newArray;
        }
        // Java: array[currentSize] = element;
        if (_currentSize as usize) < array.len() {
            array[_currentSize as usize] = _element;
        } else {
            array.push(_element);
        }
        // Java: return array;
        array
    }

    // Java: static public static <T> T[] insert(T[] array, int currentSize, int index, T element)
    pub fn insert_generic<T: Default + Clone>(mut array: Vec<T>, _currentSize: i32, _index: i32, _element: T) -> Vec<T> {
        // Java: assert currentSize <= array.length;
        assert!(_currentSize <= array.len() as i32);
        // Java: if (currentSize + 1 <= array.length)
        if _currentSize as usize + 1 <= array.len()
        {
            // Java: System.arraycopy(array, index, array, index + 1, currentSize - index);
            let idx = _index as usize;
            let cur = _currentSize as usize;
            for i in (idx..cur).rev() {
                array[i + 1] = array[i].clone();
            }
            // Java: array[index] = element;
            array[idx] = _element;
            // Java: return array;
            return array;
        }
        // Java: else
        else
        {
            // Java: T[] newArray = (T[])ArrayUtils.newUnpaddedArray(array.getClass().getComponentType(), growSize(currentSize));
            let new_size = GrowingArrayUtils::growSize(_currentSize) as usize;
            let mut new_array = vec![T::default(); new_size];
            // Java: System.arraycopy(array, 0, newArray, 0, index);
            let idx = _index as usize;
            for i in 0..idx {
                new_array[i] = array[i].clone();
            }
            // Java: newArray[index] = element;
            new_array[idx] = _element;
            // Java: System.arraycopy(array, index, newArray, index + 1, array.length - index);
            let arr_len = array.len();
            for i in idx..arr_len {
                new_array[i + 1] = array[i].clone();
            }
            // Java: return newArray;
            return new_array;
        }
    }

    // Java: static int[] insert(int[] array, int currentSize, int index, int element)
    pub fn insert_int(mut array: Vec<i32>, _currentSize: i32, _index: i32, _element: i32) -> Vec<i32> {
        // Java: assert currentSize <= array.length;
        assert!(_currentSize <= array.len() as i32);
        // Java: if (currentSize + 1 <= array.length)
        if _currentSize as usize + 1 <= array.len()
        {
            // Java: System.arraycopy(array, index, array, index + 1, currentSize - index);
            let idx = _index as usize;
            let cur = _currentSize as usize;
            for i in (idx..cur).rev() {
                array[i + 1] = array[i];
            }
            // Java: array[index] = element;
            array[idx] = _element;
            // Java: return array;
            return array;
        }
        // Java: else
        else
        {
            // Java: int[] newArray = ArrayUtils.newUnpaddedIntArray(growSize(currentSize));
            let new_size = GrowingArrayUtils::growSize(_currentSize) as usize;
            let mut new_array = vec![0i32; new_size];
            // Java: System.arraycopy(array, 0, newArray, 0, index);
            let idx = _index as usize;
            for i in 0..idx {
                new_array[i] = array[i];
            }
            // Java: newArray[index] = element;
            new_array[idx] = _element;
            // Java: System.arraycopy(array, index, newArray, index + 1, array.length - index);
            let arr_len = array.len();
            for i in idx..arr_len {
                new_array[i + 1] = array[i];
            }
            // Java: return newArray;
            return new_array;
        }
    }

    // Java: static long[] insert(long[] array, int currentSize, int index, long element)
    pub fn insert_long(mut array: Vec<i64>, _currentSize: i32, _index: i32, _element: i64) -> Vec<i64> {
        // Java: assert currentSize <= array.length;
        assert!(_currentSize <= array.len() as i32);
        // Java: if (currentSize + 1 <= array.length)
        if _currentSize as usize + 1 <= array.len()
        {
            // Java: System.arraycopy(array, index, array, index + 1, currentSize - index);
            let idx = _index as usize;
            let cur = _currentSize as usize;
            for i in (idx..cur).rev() {
                array[i + 1] = array[i];
            }
            // Java: array[index] = element;
            array[idx] = _element;
            // Java: return array;
            return array;
        }
        // Java: else
        else
        {
            // Java: long[] newArray = ArrayUtils.newUnpaddedLongArray(growSize(currentSize));
            let new_size = GrowingArrayUtils::growSize(_currentSize) as usize;
            let mut new_array = vec![0i64; new_size];
            // Java: System.arraycopy(array, 0, newArray, 0, index);
            let idx = _index as usize;
            for i in 0..idx {
                new_array[i] = array[i];
            }
            // Java: newArray[index] = element;
            new_array[idx] = _element;
            // Java: System.arraycopy(array, index, newArray, index + 1, array.length - index);
            let arr_len = array.len();
            for i in idx..arr_len {
                new_array[i + 1] = array[i];
            }
            // Java: return newArray;
            return new_array;
        }
    }

    // Java: static boolean[] insert(boolean[] array, int currentSize, int index, boolean element)
    pub fn insert_bool(mut array: Vec<bool>, _currentSize: i32, _index: i32, _element: bool) -> Vec<bool> {
        // Java: assert currentSize <= array.length;
        assert!(_currentSize <= array.len() as i32);
        // Java: if (currentSize + 1 <= array.length)
        if _currentSize as usize + 1 <= array.len()
        {
            // Java: System.arraycopy(array, index, array, index + 1, currentSize - index);
            let idx = _index as usize;
            let cur = _currentSize as usize;
            for i in (idx..cur).rev() {
                array[i + 1] = array[i];
            }
            // Java: array[index] = element;
            array[idx] = _element;
            // Java: return array;
            return array;
        }
        // Java: else
        else
        {
            // Java: boolean[] newArray = ArrayUtils.newUnpaddedBooleanArray(growSize(currentSize));
            let new_size = GrowingArrayUtils::growSize(_currentSize) as usize;
            let mut new_array = vec![false; new_size];
            // Java: System.arraycopy(array, 0, newArray, 0, index);
            let idx = _index as usize;
            for i in 0..idx {
                new_array[i] = array[i];
            }
            // Java: newArray[index] = element;
            new_array[idx] = _element;
            // Java: System.arraycopy(array, index, newArray, index + 1, array.length - index);
            let arr_len = array.len();
            for i in idx..arr_len {
                new_array[i + 1] = array[i];
            }
            // Java: return newArray;
            return new_array;
        }
    }

    // Java: static int growSize(int currentSize)
    pub fn growSize(_currentSize: i32) -> i32 {
        // Java: return currentSize <= 4 ? 8 : currentSize * 2;
        if _currentSize <= 4 { 8 } else { _currentSize * 2 }
    }

    // Java: GrowingArrayUtils()
    pub fn new() -> Self {
        // Java: super();
        GrowingArrayUtils {}
    }
}
