// java public class SearchUtil
#[derive(Default)]
pub struct SearchUtil {
}

// java public class SearchUtil
impl SearchUtil {
    // java public static int binarySearch(int[] array, int value)
    pub fn binarySearch__intarr_int(array: &Vec<i32>, value: i32) -> i32 {
      // java return binarySearch((int[])array, 0, array.length, (int)value);
      return Self::binarySearch__intarr_int_int_int(array, 0, (array.len() as i32), ((value) as i32));
    }

    // java public static int binarySearch(int[] array, int startIndex, int endIndex, int value)
    pub fn binarySearch__intarr_int_int_int(array: &Vec<i32>, startIndex: i32, endIndex: i32, value: i32) -> i32 {
      // java int lo = startIndex;
      let mut lo: i32 = startIndex;
      // java int hi = endIndex - 1;
      let mut hi: i32 = endIndex - 1;
      // java while(lo <= hi)
      while lo <= hi {
        // java int mid = lo + hi >>> 1;
        let mut mid: i32 = lo + hi >> 1;
        // java int midVal = array[mid];
        let mut midVal: i32 = array[(mid) as usize];
        // java if (midVal < value)
        if midVal < value {
          // java lo = mid + 1;
          lo = mid + 1;
        }
        // java else
        else {
          // java if (midVal <= value)
          if midVal <= value {
            // java return mid;
            return mid;
          }
          // java hi = mid - 1;
          hi = mid - 1;
        }
      }
      // java return ~lo;
      return !lo;
    }

    // java public static int binarySearch(char[] array, char value)
    pub fn binarySearch__chararr_char(array: &Vec<char>, value: char) -> i32 {
      // java return binarySearch((char[])array, 0, array.length, (char)value);
      return Self::binarySearch__chararr_int_int_char(array, 0, (array.len() as i32), value);
    }

    // java public static int binarySearch(char[] array, int startIndex, int endIndex, char value)
    pub fn binarySearch__chararr_int_int_char(array: &Vec<char>, startIndex: i32, endIndex: i32, value: char) -> i32 {
      // java int lo = startIndex;
      let mut lo: i32 = startIndex;
      // java int hi = endIndex - 1;
      let mut hi: i32 = endIndex - 1;
      // java while(lo <= hi)
      while lo <= hi {
        // java int mid = lo + hi >>> 1;
        let mut mid: i32 = lo + hi >> 1;
        // java int midVal = array[mid];
        let mut midVal: i32 = (array[(mid) as usize]) as i32;
        // java if (midVal < value)
        if midVal < (value as i32) {
          // java lo = mid + 1;
          lo = mid + 1;
        }
        // java else
        else {
          // java if (midVal <= value)
          if midVal <= (value as i32) {
            // java return mid;
            return mid;
          }
          // java hi = mid - 1;
          hi = mid - 1;
        }
      }
      // java return ~lo;
      return !lo;
    }

    // java public static int binarySearch(long[] array, int startIndex, int endIndex, long value)
    pub fn binarySearch__longarr_int_int_long(array: &Vec<i64>, startIndex: i32, endIndex: i32, value: i64) -> i32 {
      // java int lo = startIndex;
      let mut lo: i32 = startIndex;
      // java int hi = endIndex - 1;
      let mut hi: i32 = endIndex - 1;
      // java while(lo <= hi)
      while lo <= hi {
        // java int mid = lo + hi >>> 1;
        let mut mid: i32 = lo + hi >> 1;
        // java long midVal = array[mid];
        let mut midVal: i64 = array[(mid) as usize];
        // java if (midVal < value)
        if midVal < value {
          // java lo = mid + 1;
          lo = mid + 1;
        }
        // java else
        else {
          // java if (midVal <= value)
          if midVal <= value {
            // java return mid;
            return mid;
          }
          // java hi = mid - 1;
          hi = mid - 1;
        }
      }
      // java return ~lo;
      return !lo;
    }

    // java public static int binarySearch(long[] arrray, long value)
    pub fn binarySearch__longarr_long(arrray: &Vec<i64>, value: i64) -> i32 {
      // java return binarySearch(arrray, 0, arrray.length, value);
      return Self::binarySearch__longarr_int_int_long(&arrray, 0, (arrray.len() as i32), value);
    }

}
