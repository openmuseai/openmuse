#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]

// Java: import cn.wps.text.layout.typo.layout.style.ReplacementSpan
use crate::cn::wps::text::layout::typo::layout::style::ReplacementSpan;
// Java: import java.lang.reflect.Array
use std::sync::{Mutex, OnceLock};

// Java field: String TAG
pub(crate) const TAG: &str = "TextUtils";
// Java field: char ELLIPSIS_FILLER
pub(crate) const ELLIPSIS_FILLER: char = '\u{feff}';
// Java field: String ELLIPSIS_NORMAL
pub(crate) const ELLIPSIS_NORMAL: &str = "\u{2026}";
// Java field: String ELLIPSIS_TWO_DOTS
pub(crate) const ELLIPSIS_TWO_DOTS: &str = "\u{2025}";
// Java field: Object sLock
static sLock: OnceLock<Mutex<()>> = OnceLock::new();
// Java field: char[] sTemp
static sTemp: OnceLock<Mutex<Option<Vec<char>>>> = OnceLock::new();

// Java: public class TextUtils
pub struct TextUtils {
}

// Java: public enum TruncateAt
pub struct TruncateAt {
    value: i32,
}

// Java: public interface EllipsizeCallback
pub trait EllipsizeCallback {
    // Java: abstract void ellipsized(int var1, int var2)
    fn ellipsized(&self, var1: i32, var2: i32);
}

// Java: public class TextUtils
impl TextUtils {
    // Java: TextUtils()
    pub fn TextUtils() -> Self {
      // Java: super();
      TextUtils {}
    }

    // Java: static char[] obtain(int len)
    pub fn obtain(len: i32) -> Vec<char> {
      // Java: char[] buf;
      let buf: Option<Vec<char>>;
      // Java: synchronized(sLock)
      {
         let mut temp_guard = sTemp.get_or_init(|| Mutex::new(None)).lock().unwrap();
         // Java: buf = sTemp;
         buf = temp_guard.take();
         // Java: sTemp = null;
      }
      // Java: if (buf == null || buf.length < len)
      if buf.is_none() || buf.as_ref().unwrap().len() < len as usize
      {
         // Java: buf = new char[len];
         return vec!['\0'; len as usize];
      }
      // Java: return buf;
      buf.unwrap()
    }

    // Java: static long packRangeInLong(int start, int end)
    pub fn packRangeInLong(start: i32, end: i32) -> i64 {
      // Java: return (long)start << 32 | (long)end;
      ((start as i64) << 32) | ((end as i64) & 0xFFFFFFFF)
    }

    // Java: static int unpackRangeStartFromLong(long range)
    pub fn unpackRangeStartFromLong(range: i64) -> i32 {
      // Java: return (int)(range >>> 32);
      ((range as u64) >> 32) as i32
    }

    // Java: static int unpackRangeEndFromLong(long range)
    pub fn unpackRangeEndFromLong(range: i64) -> i32 {
      // Java: return (int)(range & 4294967295L);
      (range & 0xFFFFFFFF) as i32
    }

    // Java: static int indexOf(CharSequence s, char ch, int start, int end)
    pub fn indexOf(s: &[char], ch: char, start: i32, end: i32) -> i32 {
      // Java: Class<? extends CharSequence> c = s.getClass();
      let _c = 0i32;
      // Java: if (!(s instanceof GetChars) && c != StringBuffer.class && c != StringBuilder.class && c != String.class)
      if false
      {
         // Java: for(int i = start; i < end; ++i)
         for i in start..end
         {
            // Java: if (s.charAt(i) == ch)
            if s[i as usize] == ch
            {
               // Java: return i;
               return i;
            }
         }
         // Java: return -1;
         return -1;
      }
      // Java: else
      else
      {
         // Java: int INDEX_INCREMENT = 500;
         let _INDEX_INCREMENT: i32 = 500;
         // Java: char[] temp;
         let mut temp: Vec<char>;
         // Java: int segend;
         let mut segend: i32;
         // Java: for(temp = obtain(500); start < end; start = segend)
         temp = Self::obtain(500);
         let mut start = start;
         while start < end
         {
            // Java: segend = start + 500;
            segend = start + 500;
            // Java: if (segend > end)
            if segend > end
            {
               // Java: segend = end;
               segend = end;
            }
            // Java: getChars(s, start, segend, temp, 0);
            Self::getChars(s, start, segend, &mut temp, 0);
            // Java: int count = segend - start;
            let count = segend - start;
            // Java: for(int i = 0; i < count; ++i)
            for i in 0..count
            {
               // Java: if (temp[i] == ch)
               if temp[i as usize] == ch
               {
                  // Java: recycle(temp);
                  Self::recycle(temp);
                  // Java: return i + start;
                  return i + start;
               }
            }
            start = segend;
         }
         // Java: recycle(temp);
         Self::recycle(temp);
         // Java: return -1;
         return -1;
      }
    }

    // Java: static void recycle(char[] temp)
    pub fn recycle(temp: Vec<char>) {
      // Java: if (temp.length <= 1000)
      if temp.len() <= 1000
      {
         // Java: synchronized(sLock)
         {
            let mut temp_guard = sTemp.get_or_init(|| Mutex::new(None)).lock().unwrap();
            // Java: sTemp = temp;
            *temp_guard = Some(temp);
         }
      }
    }

    // Java: static void getChars(CharSequence s, int start, int end, char[] dest, int destoff)
    pub fn getChars(s: &[char], start: i32, end: i32, dest: &mut [char], destoff: i32) {
      // Java: Class<? extends CharSequence> c = s.getClass();
      let _c = 0i32;
      // Java: if (c == String.class)
      {
         // Java: ((String)s).getChars(start, end, dest, destoff);
         let mut d = destoff as usize;
         for i in start..end {
            dest[d] = s[i as usize];
            d += 1;
         }
         return;
      }
      // Java: else if (c == StringBuffer.class)
      {
         // Java: ((StringBuffer)s).getChars(start, end, dest, destoff);
      }
      // Java: else if (c == StringBuilder.class)
      {
         // Java: ((StringBuilder)s).getChars(start, end, dest, destoff);
      }
      // Java: else if (s instanceof GetChars)
      {
         // Java: ((GetChars)s).getChars(start, end, dest, destoff);
      }
      // Java: else
      {
         // Java: for(int i = start; i < end; ++i)
         {
            // Java: dest[destoff++] = s.charAt(i);
         }
      }
    }

    // Java: static int getOffsetAfter(CharSequence text, int offset)
    pub fn getOffsetAfter(text: &[char], offset: i32) -> i32 {
      // Java: int len = text.length();
      let len = text.len() as i32;
      // Java: if (offset == len)
      if offset == len
      {
         // Java: return len;
         return len;
      }
      // Java: else if (offset == len - 1)
      else if offset == len - 1
      {
         // Java: return len;
         return len;
      }
      // Java: else
      else
      {
         // Java: char c = text.charAt(offset);
         let c = text[offset as usize];
         let mut offset = offset;
         // Java: if (c >= '\ud800' && c <= '\udbff')
         if (c as u32) >= 0xd800 && (c as u32) <= 0xdbff
         {
            // Java: char c1 = text.charAt(offset + 1);
            let c1 = text[(offset + 1) as usize];
            // Java: if (c1 >= '\udc00' && c1 <= '\udfff')
            if (c1 as u32) >= 0xdc00 && (c1 as u32) <= 0xdfff
            {
               // Java: offset += 2;
               offset += 2;
            }
            // Java: else
            else
            {
               // Java: ++offset;
               offset += 1;
            }
         }
         // Java: else
         else
         {
            // Java: ++offset;
            offset += 1;
         }
         // Java: if (text instanceof Spanned)
         {
            // Java: ReplacementSpan[] spans = (ReplacementSpan[])((Spanned)text).getSpans(offset, offset, ReplacementSpan.class);
            let spans: Vec<i32> = Vec::new();
            // Java: for(int i = 0; i < spans.length; ++i)
            for i in 0..spans.len()
            {
               // Java: int start = ((Spanned)text).getSpanStart(spans[i]);
               let start = 0i32;
               // Java: int end = ((Spanned)text).getSpanEnd(spans[i]);
               let end = 0i32;
               // Java: if (start < offset && end > offset)
               if start < offset && end > offset
               {
                  // Java: offset = end;
                  offset = end;
               }
            }
         }
         // Java: return offset;
         return offset;
      }
    }

    // Java: static int getOffsetBefore(CharSequence text, int offset)
    pub fn getOffsetBefore(text: &[char], offset: i32) -> i32 {
      // Java: if (offset == 0)
      if offset == 0
      {
         // Java: return 0;
         return 0;
      }
      // Java: else if (offset == 1)
      else if offset == 1
      {
         // Java: return 0;
         return 0;
      }
      // Java: else
      else
      {
         let mut offset = offset;
         // Java: char c = text.charAt(offset - 1);
         let c = text[(offset - 1) as usize];
         // Java: if (c >= '\udc00' && c <= '\udfff')
         if (c as u32) >= 0xdc00 && (c as u32) <= 0xdfff
         {
            // Java: char c1 = text.charAt(offset - 2);
            let c1 = text[(offset - 2) as usize];
            // Java: if (c1 >= '\ud800' && c1 <= '\udbff')
            if (c1 as u32) >= 0xd800 && (c1 as u32) <= 0xdbff
            {
               // Java: offset -= 2;
               offset -= 2;
            }
            // Java: else
            else
            {
               // Java: --offset;
               offset -= 1;
            }
         }
         // Java: else
         else
         {
            // Java: --offset;
            offset -= 1;
         }
         // Java: if (text instanceof Spanned)
         {
            // Java: ReplacementSpan[] spans = (ReplacementSpan[])((Spanned)text).getSpans(offset, offset, ReplacementSpan.class);
            let spans: Vec<i32> = Vec::new();
            // Java: for(int i = 0; i < spans.length; ++i)
            for i in 0..spans.len()
            {
               // Java: int start = ((Spanned)text).getSpanStart(spans[i]);
               let start = 0i32;
               // Java: int end = ((Spanned)text).getSpanEnd(spans[i]);
               let end = 0i32;
               // Java: if (start < offset && end > offset)
               if start < offset && end > offset
               {
                  // Java: offset = start;
                  offset = start;
               }
            }
         }
         // Java: return offset;
         return offset;
      }
    }

    // Java: static public static <T> T[] removeEmptySpans(T[] spans, Spanned spanned, Class<T> klass)
    pub fn removeEmptySpans(spans: &[i32], spanned: i32, klass: i32) -> Vec<i32> {
      // Java: T[] copy = null;
      let mut copy: Option<Vec<i32>> = None;
      // Java: int count = 0;
      let mut count: usize = 0;
      // Java: for(int i = 0; i < spans.length; ++i)
      for i in 0..spans.len()
      {
         // Java: T span = (T)spans[i];
         let span = spans[i];
         // Java: int start = spanned.getSpanStart(span);
         let start = 0i32;
         // Java: int end = spanned.getSpanEnd(span);
         let end = 0i32;
         // Java: if (start == end)
         if start == end
         {
            // Java: if (copy == null)
            if copy.is_none()
            {
               // Java: copy = (T[])((Object[])Array.newInstance(klass, spans.length - 1));
               let mut c = vec![0i32; spans.len() - 1];
               // Java: System.arraycopy(spans, 0, copy, 0, i);
               for j in 0..i {
                   c[j] = spans[j];
               }
               // Java: count = i;
               count = i;
               copy = Some(c);
            }
         }
         // Java: else if (copy != null)
         else if copy.is_some()
         {
            // Java: copy[count] = span;
            copy.as_mut().unwrap()[count] = span;
            // Java: ++count;
            count += 1;
         }
      }
      // Java: if (copy != null)
      if copy.is_some()
      {
         // Java: T[] result = (T[])((Object[])Array.newInstance(klass, count));
         let mut result = vec![0i32; count];
         // Java: System.arraycopy(copy, 0, result, 0, count);
         let c = copy.unwrap();
         for j in 0..count {
             result[j] = c[j];
         }
         // Java: return result;
         return result;
      }
      // Java: else
      else
      {
         // Java: return spans;
         return spans.to_vec();
      }
    }

    // Java: static boolean isEmpty(CharSequence str)
    pub fn isEmpty(str: Option<&[char]>) -> bool {
      // Java: return str == null || str.length() == 0;
      str.is_none() || str.unwrap().is_empty()
    }

    // Java: static boolean doesNotNeedBidi(char[] text, int start, int len)
    pub fn doesNotNeedBidi(text: &[char], start: i32, len: i32) -> bool {
      // Java: int end = start + len;
      let end = start + len;
      // Java: for(int i = start; i < end; ++i)
      for i in start..end
      {
         // Java: if (couldAffectRtl(text[i]))
         if TextUtils::couldAffectRtl(text[i as usize])
         {
            // Java: return false;
            return false;
         }
      }
      // Java: return true;
      true
    }

    // Java: static boolean couldAffectRtl(char c)
    pub fn couldAffectRtl(c: char) -> bool {
      // Java: return 1424 <= c && c <= 2303 || c == 8206 || c == 8207 || 8234 <= c && c <= 8238 || 8294 <= c && c <= 8297 || '\ud800' <= c && c <= '\udfff' || 'יִ' <= c && c <= '\ufdff' || 'ﹰ' <= c && c <= '\ufefe';
      let cv = c as u32;
      (1424 <= cv && cv <= 2303) || cv == 8206 || cv == 8207 || (8234 <= cv && cv <= 8238) || (8294 <= cv && cv <= 8297) || (0xd800 <= cv && cv <= 0xdfff) || (0xfb1d <= cv && cv <= 0xfdff) || (0xfe70 <= cv && cv <= 0xfefe)
    }

    // Java: static String substring(CharSequence source, int start, int end)
    pub fn substring(source: &[char], start: i32, end: i32) -> String {
      // Java: if (source instanceof String)
      {
         // Java: return ((String)source).substring(start, end);
         return source[start as usize..end as usize].iter().collect();
      }
      // Java: else if (source instanceof StringBuilder)
      {
         // Java: return ((StringBuilder)source).substring(start, end);
      }
      // Java: else if (source instanceof StringBuffer)
      {
         // Java: return ((StringBuffer)source).substring(start, end);
      }
      // Java: else
      {
         // Java: char[] temp = obtain(end - start);
         let mut temp = Self::obtain(end - start);
         // Java: getChars(source, start, end, temp, 0);
         Self::getChars(source, start, end, &mut temp, 0);
         // Java: String ret = new String(temp, 0, end - start);
         let ret: String = temp[0..(end - start) as usize].iter().collect();
         // Java: recycle(temp);
         Self::recycle(temp);
         // Java: return ret;
         return ret;
      }
    }

}

// Java: public enum TruncateAt
impl TruncateAt {
    // Java: TruncateAt()
    pub fn TruncateAt(value: i32) -> Self {
        TruncateAt { value }
    }

}
