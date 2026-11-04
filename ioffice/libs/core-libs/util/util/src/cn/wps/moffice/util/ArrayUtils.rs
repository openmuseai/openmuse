use std::collections::HashMap;

// Java field: Object[] EMPTY
// Java field: int CACHE_SIZE
pub(crate) const CACHE_SIZE: i32 = 73;
// Java field: Object[] sCache

// Java: public class ArrayUtils
pub struct ArrayUtils;

// Java: public class ArrayUtils
impl ArrayUtils {
    // Java: ArrayUtils()
    fn new() -> Self {
        // Java: super();
        ArrayUtils
    }

    // Java: static int idealByteArraySize(int need)
    pub fn idealByteArraySize(need: i32) -> i32 {
        // Java: for(int i = 4; i < 32; ++i)
        for i in 4..32 {
            // Java: if (need <= (1 << i) - 12)
            if need <= (1 << i) - 12 {
                // Java: return (1 << i) - 12;
                return (1 << i) - 12;
            }
        }
        // Java: return need;
        need
    }

    // Java: static int idealBooleanArraySize(int need)
    pub fn idealBooleanArraySize(need: i32) -> i32 {
        // Java: return idealByteArraySize(need);
        Self::idealByteArraySize(need)
    }

    // Java: static int idealShortArraySize(int need)
    pub fn idealShortArraySize(need: i32) -> i32 {
        // Java: return idealByteArraySize(need * 2) / 2;
        Self::idealByteArraySize(need * 2) / 2
    }

    // Java: static int idealCharArraySize(int need)
    pub fn idealCharArraySize(need: i32) -> i32 {
        // Java: return idealByteArraySize(need * 2) / 2;
        Self::idealByteArraySize(need * 2) / 2
    }

    // Java: static int idealIntArraySize(int need)
    pub fn idealIntArraySize(need: i32) -> i32 {
        // Java: return idealByteArraySize(need * 4) / 4;
        Self::idealByteArraySize(need * 4) / 4
    }

    // Java: static int idealFloatArraySize(int need)
    pub fn idealFloatArraySize(need: i32) -> i32 {
        // Java: return idealByteArraySize(need * 4) / 4;
        Self::idealByteArraySize(need * 4) / 4
    }

    // Java: static int idealObjectArraySize(int need)
    pub fn idealObjectArraySize(need: i32) -> i32 {
        // Java: return idealByteArraySize(need * 4) / 4;
        Self::idealByteArraySize(need * 4) / 4
    }

    // Java: static int idealLongArraySize(int need)
    pub fn idealLongArraySize(need: i32) -> i32 {
        // Java: return idealByteArraySize(need * 8) / 8;
        Self::idealByteArraySize(need * 8) / 8
    }

    // Java: static boolean equals(_array1: &[u8], _array2: &[u8], int length)
    pub fn equals_bytes(array1: Option<&[u8]>, array2: Option<&[u8]>, length: usize) -> bool {
        // Java: if (array1 == array2)
        if std::ptr::eq(
            array1.map(|a| a.as_ptr()).unwrap_or(std::ptr::null()),
            array2.map(|a| a.as_ptr()).unwrap_or(std::ptr::null()),
        ) {
            // Java: return true;
            return true;
        // Java: else if (array1 != null && array2 != null && array1.length >= length && array2.length >= length)
        } else if let (Some(a1), Some(a2)) = (array1, array2) {
            if a1.len() >= length && a2.len() >= length {
                // Java: for(int i = 0; i < length; ++i)
                for i in 0..length {
                    // Java: if (array1[i] != array2[i])
                    if a1[i] != a2[i] {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: return true;
                return true;
            }
            // Java: else
            // Java: return false;
            false
        } else {
            false
        }
    }

    // Java: static public static <T> T[] emptyArray(Class<T> kind)
    // Java: if (kind == Object.class)
    // Java: return (T[])EMPTY;
    // Java: else
    // Java: int bucket = (System.identityHashCode(kind) / 8 & Integer.MAX_VALUE) % 73;
    // Java: Object cache = sCache[bucket];
    // Java: if (cache == null || cache.getClass().getComponentType() != kind)
    // Java: cache = Array.newInstance(kind, 0);
    // Java: sCache[bucket] = cache;
    // Java: return (T[])((Object[])cache);

    // Java: static public static <T> boolean contains(T[] array, T value)
    pub fn contains_generic<T: PartialEq>(array: &[Option<T>], value: &Option<T>) -> bool {
        // Java: for(T element : array)
        for element in array {
            // Java: if (element == null)
            if element.is_none() {
                // Java: if (value == null)
                if value.is_none() {
                    // Java: return true;
                    return true;
                }
            // Java: else if (value != null && element.equals(value))
            } else if value.is_some() && element == value {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: static boolean contains(_array: &[i32], int value)
    pub fn contains(array: &[i32], value: i32) -> bool {
        // Java: for(int element : array)
        for &element in array {
            // Java: if (element == value)
            if element == value {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: static long total(long[] array)
    pub fn total(array: &[i64]) -> i64 {
        // Java: long total = 0L;
        let mut total: i64 = 0;
        // Java: for(long value : array)
        for &value in array {
            // Java: total += value;
            total += value;
        }
        // Java: return total;
        total
    }

    // Java: static public static <T> T[] appendElement(Class<T> kind, T[] array, T element)
    pub fn appendElement<T: Clone>(array: Option<&[T]>, element: T) -> Vec<T> {
        // Java: T[] result;
        // Java: int end;
        let mut result;
        let end;
        // Java: if (array != null)
        if let Some(arr) = array {
            // Java: end = array.length;
            end = arr.len();
            // Java: result = (T[])((Object[])Array.newInstance(kind, end + 1));
            result = Vec::with_capacity(end + 1);
            // Java: System.arraycopy(array, 0, result, 0, end);
            result.extend_from_slice(arr);
        // Java: else
        } else {
            // Java: end = 0;
            end = 0;
            // Java: result = (T[])((Object[])Array.newInstance(kind, 1));
            result = Vec::with_capacity(1);
        }
        // Java: result[end] = element;
        result.push(element);
        // Java: return result;
        result
    }

    // Java: static public static <T> T[] removeElement(Class<T> kind, T[] array, T element)
    pub fn removeElement<T: Clone + PartialEq>(array: Option<&[T]>, element: &T) -> Option<Vec<T>> {
        // Java: if (array != null)
        if let Some(arr) = array {
            // Java: int length = array.length;
            let length = arr.len();
            // Java: for(int i = 0; i < length; ++i)
            for i in 0..length {
                // Java: if (array[i] == element)
                if &arr[i] == element {
                    // Java: if (length == 1)
                    if length == 1 {
                        // Java: return null;
                        return None;
                    }
                    // Java: T[] result = (T[])((Object[])Array.newInstance(kind, length - 1));
                    let mut result = Vec::with_capacity(length - 1);
                    // Java: System.arraycopy(array, 0, result, 0, i);
                    result.extend_from_slice(&arr[..i]);
                    // Java: System.arraycopy(array, i + 1, result, i, length - i - 1);
                    result.extend_from_slice(&arr[i + 1..]);
                    // Java: return result;
                    return Some(result);
                }
            }
            // Java: return array;
            Some(arr.to_vec())
        } else {
            None
        }
    }

    // Java: static _appendInt: &[i32](_cur: &[i32], int val)
    pub fn appendInt(cur: Option<&[i32]>, val: i32) -> Vec<i32> {
        // Java: if (cur == null)
        if cur.is_none() {
            // Java: return new int[]{val};
            return vec![val];
        // Java: else
        } else {
            let cur = cur.unwrap();
            // Java: int N = cur.length;
            let n = cur.len();
            // Java: for(int i = 0; i < N; ++i)
            for i in 0..n {
                // Java: if (cur[i] == val)
                if cur[i] == val {
                    // Java: return cur;
                    return cur.to_vec();
                }
            }
            // Java: _ret: &[i32] = new int[N + 1];
            let mut ret = Vec::with_capacity(n + 1);
            // Java: System.arraycopy(cur, 0, ret, 0, N);
            ret.extend_from_slice(cur);
            // Java: ret[N] = val;
            ret.push(val);
            // Java: return ret;
            ret
        }
    }

    // Java: static _removeInt: &[i32](_cur: &[i32], int val)
    pub fn removeInt(cur: Option<&[i32]>, val: i32) -> Option<Vec<i32>> {
        // Java: if (cur == null)
        if cur.is_none() {
            // Java: return null;
            return None;
        // Java: else
        } else {
            let cur = cur.unwrap();
            // Java: int N = cur.length;
            let n = cur.len();
            // Java: for(int i = 0; i < N; ++i)
            for i in 0..n {
                // Java: if (cur[i] == val)
                if cur[i] == val {
                    // Java: _ret: &[i32] = new int[N - 1];
                    let mut ret = Vec::with_capacity(n - 1);
                    // Java: if (i > 0)
                    if i > 0 {
                        // Java: System.arraycopy(cur, 0, ret, 0, i);
                        ret.extend_from_slice(&cur[..i]);
                    }
                    // Java: if (i < N - 1)
                    if i < n - 1 {
                        // Java: System.arraycopy(cur, i + 1, ret, i, N - i - 1);
                        ret.extend_from_slice(&cur[i + 1..]);
                    }
                    // Java: return ret;
                    return Some(ret);
                }
            }
            // Java: return cur;
            Some(cur.to_vec())
        }
    }

    // Java: static public static <T> ArrayList<T> subList(List<T> target, int num)
    pub fn subList<T: Clone>(target: &[T], num: usize) -> Vec<T> {
        // Java: if (target.size() <= num)
        if target.len() <= num {
            // Java: return new ArrayList(target);
            return target.to_vec();
        // Java: else
        } else {
            // Java: ArrayList<T> results = new ArrayList(num);
            let mut results = Vec::with_capacity(num);
            // Java: for(int i = 0; i < target.size() && i < num; ++i)
            for i in 0..std::cmp::min(target.len(), num) {
                // Java: results.add(target.get(i));
                results.push(target[i].clone());
            }
            // Java: return results;
            results
        }
    }

    // Java: static _asStringArray: &[String](List<String> target)
    pub fn asStringArray(target: Option<&[String]>) -> Vec<String> {
        // Java: if (target == null)
        if target.is_none() {
            // Java: return new String[0];
            return Vec::new();
        // Java: else
        } else {
            let target = target.unwrap();
            // Java: _os: &[String] = new String[target.size()];
            let mut os = Vec::with_capacity(target.len());
            // Java: for(int i = 0; i < target.size(); ++i)
            for i in 0..target.len() {
                // Java: os[i] = (String)target.get(i);
                os.push(target[i].clone());
            }
            // Java: return os;
            os
        }
    }

    // Java: static public static <T> Map<T, T> createArraySearchMap(List<T> arrays)
    pub fn createArraySearchMap<T: Clone + std::hash::Hash + Eq>(arrays: &[T]) -> HashMap<T, T> {
        // Java: int size = 0;
        // Java: if (arrays != null)
        // Java: size = arrays.size();
        let size = arrays.len();
        // Java: Map<T, T> m = new HashMap(size);
        let mut m = HashMap::with_capacity(size);
        // Java: for(T array : arrays)
        for array in arrays {
            // Java: m.put(array, array);
            m.insert(array.clone(), array.clone());
        }
        // Java: return m;
        m
    }

    // Java: static void arraycopy(_src: &[u8], int src_position, _dst: &[u8], int dst_position, int length)
    pub fn arraycopy(src: &[u8], src_position: usize, dst: &mut [u8], dst_position: usize, length: usize) {
        // Java: if (src_position < 0)
        // Java: throw new IllegalArgumentException("src_position was less than 0.  Actual value " + src_position);
        // Java: else if (src_position >= src.length)
        // Java: throw new IllegalArgumentException("src_position was greater than src array size.  Tried to write starting at position " + src_position + " but the array length is " + src.length);
        // Java: else if (src_position + length > src.length)
        // Java: throw new IllegalArgumentException("src_position + length would overrun the src array.  Expected end at " + (src_position + length) + " actual end at " + src.length);
        // Java: else if (dst_position < 0)
        // Java: throw new IllegalArgumentException("dst_position was less than 0.  Actual value " + dst_position);
        // Java: else if (dst_position >= dst.length)
        // Java: throw new IllegalArgumentException("dst_position was greater than dst array size.  Tried to write starting at position " + dst_position + " but the array length is " + dst.length);
        // Java: else if (dst_position + length > dst.length)
        // Java: throw new IllegalArgumentException("dst_position + length would overrun the dst array.  Expected end at " + (dst_position + length) + " actual end at " + dst.length);
        // Java: else
        assert!(src_position + length <= src.len(), "src overrun");
        assert!(dst_position + length <= dst.len(), "dst overrun");
        // Java: System.arraycopy(src, src_position, dst, dst_position, length);
        dst[dst_position..dst_position + length].copy_from_slice(&src[src_position..src_position + length]);
    }

    // Java: static void arrayMoveWithin(Object[] array, int moveFrom, int moveTo, int numToMove)
    // Java: if (numToMove > 0)
    // Java: if (moveFrom != moveTo)
    // Java: if (moveFrom >= 0 && moveFrom < array.length)
    // Java: if (moveTo >= 0 && moveTo < array.length)
    // Java: if (moveFrom + numToMove > array.length)
    // Java: throw new IllegalArgumentException("Asked to move more entries than the array has");
    // Java: else if (moveTo + numToMove > array.length)
    // Java: throw new IllegalArgumentException("Asked to move to a position that doesn't have enough space");
    // Java: else
    // Java: Object[] toMove = new Object[numToMove];
    // Java: System.arraycopy(array, moveFrom, toMove, 0, numToMove);
    // Java: Object[] toShift;
    // Java: int shiftTo;
    // Java: if (moveFrom > moveTo)
    // Java: toShift = new Object[moveFrom - moveTo];
    // Java: System.arraycopy(array, moveTo, toShift, 0, toShift.length);
    // Java: shiftTo = moveTo + numToMove;
    // Java: else
    // Java: toShift = new Object[moveTo - moveFrom];
    // Java: System.arraycopy(array, moveFrom + numToMove, toShift, 0, toShift.length);
    // Java: shiftTo = moveFrom;
    // Java: System.arraycopy(toMove, 0, array, moveTo, toMove.length);
    // Java: System.arraycopy(toShift, 0, array, shiftTo, toShift.length);
    // Java: else
    // Java: throw new IllegalArgumentException("The moveTo must be a valid array index");
    // Java: else
    // Java: throw new IllegalArgumentException("The moveFrom must be a valid array index");

    // Java: static void checkOffsetAndCount(int arrayLength, int offset, int count)
    pub fn checkOffsetAndCount(arrayLength: i32, offset: i32, count: i32) {
        // Java: if ((offset | count) < 0 || offset > arrayLength || arrayLength - offset < count)
        if (offset | count) < 0 || offset > arrayLength || arrayLength - offset < count {
            // Java: throw new ArrayIndexOutOfBoundsException();
            panic!("ArrayIndexOutOfBoundsException");
        }
    }
}
