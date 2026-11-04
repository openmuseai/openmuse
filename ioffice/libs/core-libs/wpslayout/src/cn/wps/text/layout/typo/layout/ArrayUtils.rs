// Java: import java.lang.reflect.Array

// Java field: int CACHE_SIZE
pub(crate) const CACHE_SIZE: i32 = 73;
// Java field: Object[] sCache
static S_CACHE: std::sync::OnceLock<std::sync::Mutex<Vec<Option<Vec<u8>>>>> = std::sync::OnceLock::new();

fn get_s_cache() -> &'static std::sync::Mutex<Vec<Option<Vec<u8>>>> {
    S_CACHE.get_or_init(|| std::sync::Mutex::new(vec![None; CACHE_SIZE as usize]))
}

// Java: public class ArrayUtils
pub struct ArrayUtils;

// Java: public class ArrayUtils
impl ArrayUtils {
    // Java: ArrayUtils()
    pub fn new() -> Self {
      // Java: super();
      ArrayUtils
    }

    // Java: static public static <T> T[] newUnpaddedArray(Class<T> componentType, int minLength)
    pub fn newUnpaddedArray<T: Default + Clone>(min_length: usize) -> Vec<T> {
      // Java: return (T[])((Object[])_newUnpaddedArray(componentType, minLength));
      let size = if (min_length & 1) == 0 { min_length + 1 } else { min_length };
      vec![T::default(); size]
    }

    // Java: static Object _newUnpaddedArray(Class<?> componentType, int minLength)
    pub fn _newUnpaddedArray_int(min_length: i32) -> Vec<i32> {
      // Java: if (!componentType.isPrimitive())
      // (handled by type-specific methods in Rust)
      {
         // Java: int size = (minLength & 1) == 0 ? minLength + 1 : minLength;
         let size = if (min_length & 1) == 0 { min_length + 1 } else { min_length };
         // Java: return Array.newInstance(componentType, size);
         return vec![0i32; size as usize];
      }
      // Java: else if (componentType == Character.TYPE)
      // Java: int bytes = 20 + 2 * minLength;
      // Java: int alignedUpBytes = bytes + 7 & -8;
      // Java: int dataBytes = alignedUpBytes - 20;
      // Java: int size = dataBytes / 2;
      // Java: return new char[size];
      // Java: else if (componentType == Integer.TYPE)
      // Java: int size = (minLength & 1) == 0 ? minLength + 1 : minLength;
      // Java: return new int[size];
      // Java: else if (componentType == Byte.TYPE)
      // Java: int bytes = 20 + minLength;
      // Java: int alignedUpBytes = bytes + 7 & -8;
      // Java: int dataBytes = alignedUpBytes - 20;
      // Java: return new byte[dataBytes];
      // Java: else if (componentType == Boolean.TYPE)
      // Java: int bytes = 20 + minLength;
      // Java: int alignedUpBytes = bytes + 7 & -8;
      // Java: int dataBytes = alignedUpBytes - 20;
      // Java: return new boolean[dataBytes];
      // Java: else if (componentType == Short.TYPE)
      // Java: int bytes = 20 + 2 * minLength;
      // Java: int alignedUpBytes = bytes + 7 & -8;
      // Java: int dataBytes = alignedUpBytes - 20;
      // Java: int size = dataBytes / 2;
      // Java: return new short[size];
      // Java: else if (componentType == Float.TYPE)
      // Java: int size = (minLength & 1) == 0 ? minLength + 1 : minLength;
      // Java: return new float[size];
      // Java: else if (componentType == Long.TYPE)
      // Java: return new long[minLength];
      // Java: else if (componentType == Double.TYPE)
      // Java: return new double[minLength];
      // Java: else
      // Java: assert componentType == Void.TYPE;
      // Java: throw new IllegalArgumentException("Can't allocate an array of void");
    }

    // Java: static Object[] newUnpaddedObjectArray(int minLen)
    pub fn newUnpaddedObjectArray<T: Default + Clone>(min_len: i32) -> Vec<T> {
      // Java: return (Object[])_newUnpaddedArray(Object.class, minLen);
      let size = if (min_len & 1) == 0 { min_len + 1 } else { min_len };
      vec![T::default(); size as usize]
    }

    // Java: static public static <T> T[] emptyArray(Class<T> kind)
    pub fn emptyArray<T>() -> Vec<T> {
      // Java: if (kind == Object.class)
      // Java: return (T[])EmptyArray.OBJECT;
      // Java: else
      // Java: int bucket = (kind.hashCode() & Integer.MAX_VALUE) % 73;
      // Java: Object cache = sCache[bucket];
      // Java: if (cache == null || cache.getClass().getComponentType() != kind)
      // Java: cache = Array.newInstance(kind, 0);
      // Java: sCache[bucket] = cache;
      // Java: return (T[])((Object[])cache);
      Vec::new()
    }

    // Java: static char[] newUnpaddedCharArray(int growSize)
    pub fn newUnpaddedCharArray(grow_size: i32) -> Vec<char> {
      // Java: return (char[])_newUnpaddedArray(Character.TYPE, growSize);
      let min_length = grow_size as usize;
      let bytes = 20 + 2 * min_length;
      let aligned_up_bytes = (bytes + 7) & !7;
      let data_bytes = aligned_up_bytes - 20;
      let size = data_bytes / 2;
      vec!['\0'; size]
    }

    // Java: static int[] newUnpaddedIntArray(int growSize)
    pub fn newUnpaddedIntArray(grow_size: i32) -> Vec<i32> {
      // Java: return (int[])_newUnpaddedArray(Integer.TYPE, growSize);
      let min_length = grow_size as usize;
      let size = if (min_length & 1) == 0 { min_length + 1 } else { min_length };
      vec![0i32; size]
    }

    // Java: static float[] newUnpaddedFloatArray(int growSize)
    pub fn newUnpaddedFloatArray(grow_size: i32) -> Vec<f32> {
      // Java: return (float[])_newUnpaddedArray(Float.TYPE, growSize);
      let min_length = grow_size as usize;
      let size = if (min_length & 1) == 0 { min_length + 1 } else { min_length };
      vec![0.0f32; size]
    }

    // Java: static byte[] newUnpaddedByteArray(int initialCapacity)
    pub fn newUnpaddedByteArray(initial_capacity: i32) -> Vec<u8> {
      // Java: return (byte[])_newUnpaddedArray(Byte.TYPE, initialCapacity);
      let min_length = initial_capacity as usize;
      let bytes = 20 + min_length;
      let aligned_up_bytes = (bytes + 7) & !7;
      let data_bytes = aligned_up_bytes - 20;
      vec![0u8; data_bytes]
    }

    // Java: static int idealByteArraySize(int need)
    pub fn idealByteArraySize(need: i32) -> i32 {
      // Java: for(int i = 4; i < 32; ++i)
      for i in 4..32
      {
         // Java: if (need <= (1 << i) - 12)
         if need <= (1i32 << i) - 12
         {
            // Java: return (1 << i) - 12;
            return (1i32 << i) - 12;
         }
      }
      // Java: return need;
      need
    }

    // Java: static int idealIntArraySize(int need)
    pub fn idealIntArraySize(need: i32) -> i32 {
      // Java: return idealByteArraySize(need * 4) / 4;
      Self::idealByteArraySize(need * 4) / 4
    }

    // Java: static int idealCharArraySize(int need)
    pub fn idealCharArraySize(need: i32) -> i32 {
      // Java: return idealByteArraySize(need * 2) / 2;
      Self::idealByteArraySize(need * 2) / 2
    }

    // Java: static int idealFloatArraySize(int need)
    pub fn idealFloatArraySize(need: i32) -> i32 {
      // Java: return idealByteArraySize(need * 4) / 4;
      Self::idealByteArraySize(need * 4) / 4
    }

    // Java: static long[] newUnpaddedLongArray(int growSize)
    pub fn newUnpaddedLongArray(grow_size: i32) -> Vec<i64> {
      // Java: return (long[])_newUnpaddedArray(Long.TYPE, growSize);
      vec![0i64; grow_size as usize]
    }

    // Java: static boolean[] newUnpaddedBooleanArray(int growSize)
    pub fn newUnpaddedBooleanArray(grow_size: i32) -> Vec<bool> {
      // Java: return (boolean[])_newUnpaddedArray(Boolean.TYPE, growSize);
      let min_length = grow_size as usize;
      let bytes = 20 + min_length;
      let aligned_up_bytes = (bytes + 7) & !7;
      let data_bytes = aligned_up_bytes - 20;
      vec![false; data_bytes]
    }

}
