// Java: import cn.wps.text.layout.typo.layout.style.ReplacementSpan
use wpslayout::cn::wps::text::layout::typo::layout::style::ReplacementSpan;
// Java: import java.lang.reflect.Array
// TODO_RUST_CODE use ???; // java.lang.reflect.Array

// Java field: String TAG
pub(crate) const TAG: String = /* TODO */;
// Java field: char ELLIPSIS_FILLER
pub(crate) const ELLIPSIS_FILLER: char = /* TODO */;
// Java field: String ELLIPSIS_NORMAL
pub(crate) const ELLIPSIS_NORMAL: String = /* TODO */;
// Java field: String ELLIPSIS_TWO_DOTS
pub(crate) const ELLIPSIS_TWO_DOTS: String = /* TODO */;
// Java field: Object sLock
static sLock: std::sync::OnceLock<std::sync::Mutex<Object>> = std::sync::OnceLock::new();
// Java field: char[] sTemp
static sTemp: std::sync::OnceLock<std::sync::Mutex<Vec<char>>> = std::sync::OnceLock::new();

// Java: public class TextUtils
pub struct TextUtils {
}

// Java: public enum TruncateAt
pub struct TruncateAt {
}

// Java: public class TextUtils
impl TextUtils {
    // Java: TextUtils()
    pub fn TextUtils(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: static char[] obtain(int len)
    pub fn obtain(&self, int len) -> char[] {
      // Java: char[] buf;
      // TODO_RUST_CODE
      // Java: synchronized(sLock)
        // TODO_RUST_CODE
      {
         // Java: buf = sTemp;
         // TODO_RUST_CODE
         // Java: sTemp = null;
         // TODO_RUST_CODE
      }
      // Java: if (buf == null || buf.length < len)
        // TODO_RUST_CODE
      {
         // Java: buf = new char[len];
         // TODO_RUST_CODE
      }
      // Java: return buf;
      // TODO_RUST_CODE
    }

    // Java: static long packRangeInLong(int start, int end)
    pub fn packRangeInLong(&self, int start, int end) -> i64 {
      // Java: return (long)start << 32 | (long)end;
      // TODO_RUST_CODE
    }

    // Java: static int unpackRangeStartFromLong(long range)
    pub fn unpackRangeStartFromLong(&self, long range) -> i32 {
      // Java: return (int)(range >>> 32);
      // TODO_RUST_CODE
    }

    // Java: static int unpackRangeEndFromLong(long range)
    pub fn unpackRangeEndFromLong(&self, long range) -> i32 {
      // Java: return (int)(range & 4294967295L);
      // TODO_RUST_CODE
    }

    // Java: static int indexOf(CharSequence s, char ch, int start, int end)
    pub fn indexOf(&self, CharSequence s, char ch, int start, int end) -> i32 {
      // Java: Class<? extends CharSequence> c = s.getClass();
      // TODO_RUST_CODE
      // Java: if (!(s instanceof GetChars) && c != StringBuffer.class && c != StringBuilder.class && c != String.class)
        // TODO_RUST_CODE
      {
         // Java: for(int i = start; i < end; ++i)
            // TODO_RUST_CODE
         {
            // Java: if (s.charAt(i) == ch)
                // TODO_RUST_CODE
            {
               // Java: return i;
               // TODO_RUST_CODE
            }
         }
         // Java: return -1;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int INDEX_INCREMENT = 500;
         // TODO_RUST_CODE
         // Java: char[] temp;
         // TODO_RUST_CODE
         // Java: int segend;
         // TODO_RUST_CODE
         // Java: for(temp = obtain(500); start < end; start = segend)
            // TODO_RUST_CODE
         {
            // Java: segend = start + 500;
            // TODO_RUST_CODE
            // Java: if (segend > end)
                // TODO_RUST_CODE
            {
               // Java: segend = end;
               // TODO_RUST_CODE
            }
            // Java: getChars(s, start, segend, temp, 0);
            // TODO_RUST_CODE
            // Java: int count = segend - start;
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < count; ++i)
                // TODO_RUST_CODE
            {
               // Java: if (temp[i] == ch)
                    // TODO_RUST_CODE
               {
                  // Java: recycle(temp);
                  // TODO_RUST_CODE
                  // Java: return i + start;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: recycle(temp);
         // TODO_RUST_CODE
         // Java: return -1;
         // TODO_RUST_CODE
      }
    }

    // Java: static void recycle(char[] temp)
    pub fn recycle(&self, char[] temp) {
      // Java: if (temp.length <= 1000)
        // TODO_RUST_CODE
      {
         // Java: synchronized(sLock)
            // TODO_RUST_CODE
         {
            // Java: sTemp = temp;
            // TODO_RUST_CODE
         }
      }
    }

    // Java: static void getChars(CharSequence s, int start, int end, char[] dest, int destoff)
    pub fn getChars(&self, CharSequence s, int start, int end, char[] dest, int destoff) {
      // Java: Class<? extends CharSequence> c = s.getClass();
      // TODO_RUST_CODE
      // Java: if (c == String.class)
        // TODO_RUST_CODE
      {
         // Java: ((String)s).getChars(start, end, dest, destoff);
         // TODO_RUST_CODE
      }
      // Java: else if (c == StringBuffer.class)
        // TODO_RUST_CODE
      {
         // Java: ((StringBuffer)s).getChars(start, end, dest, destoff);
         // TODO_RUST_CODE
      }
      // Java: else if (c == StringBuilder.class)
        // TODO_RUST_CODE
      {
         // Java: ((StringBuilder)s).getChars(start, end, dest, destoff);
         // TODO_RUST_CODE
      }
      // Java: else if (s instanceof GetChars)
        // TODO_RUST_CODE
      {
         // Java: ((GetChars)s).getChars(start, end, dest, destoff);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: for(int i = start; i < end; ++i)
            // TODO_RUST_CODE
         {
            // Java: dest[destoff++] = s.charAt(i);
            // TODO_RUST_CODE
         }
      }
    }

    // Java: static int getOffsetAfter(CharSequence text, int offset)
    pub fn getOffsetAfter(&self, CharSequence text, int offset) -> i32 {
      // Java: int len = text.length();
      // TODO_RUST_CODE
      // Java: if (offset == len)
        // TODO_RUST_CODE
      {
         // Java: return len;
         // TODO_RUST_CODE
      }
      // Java: else if (offset == len - 1)
        // TODO_RUST_CODE
      {
         // Java: return len;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: char c = text.charAt(offset);
         // TODO_RUST_CODE
         // Java: if (c >= '\ud800' && c <= '\udbff')
            // TODO_RUST_CODE
         {
            // Java: char c1 = text.charAt(offset + 1);
            // TODO_RUST_CODE
            // Java: if (c1 >= '\udc00' && c1 <= '\udfff')
                // TODO_RUST_CODE
            {
               // Java: offset += 2;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: ++offset;
               // TODO_RUST_CODE
            }
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: ++offset;
            // TODO_RUST_CODE
         }
         // Java: if (text instanceof Spanned)
            // TODO_RUST_CODE
         {
            // Java: ReplacementSpan[] spans = (ReplacementSpan[])((Spanned)text).getSpans(offset, offset, ReplacementSpan.class);
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < spans.length; ++i)
                // TODO_RUST_CODE
            {
               // Java: int start = ((Spanned)text).getSpanStart(spans[i]);
               // TODO_RUST_CODE
               // Java: int end = ((Spanned)text).getSpanEnd(spans[i]);
               // TODO_RUST_CODE
               // Java: if (start < offset && end > offset)
                    // TODO_RUST_CODE
               {
                  // Java: offset = end;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: return offset;
         // TODO_RUST_CODE
      }
    }

    // Java: static int getOffsetBefore(CharSequence text, int offset)
    pub fn getOffsetBefore(&self, CharSequence text, int offset) -> i32 {
      // Java: if (offset == 0)
        // TODO_RUST_CODE
      {
         // Java: return 0;
         // TODO_RUST_CODE
      }
      // Java: else if (offset == 1)
        // TODO_RUST_CODE
      {
         // Java: return 0;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: char c = text.charAt(offset - 1);
         // TODO_RUST_CODE
         // Java: if (c >= '\udc00' && c <= '\udfff')
            // TODO_RUST_CODE
         {
            // Java: char c1 = text.charAt(offset - 2);
            // TODO_RUST_CODE
            // Java: if (c1 >= '\ud800' && c1 <= '\udbff')
                // TODO_RUST_CODE
            {
               // Java: offset -= 2;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: --offset;
               // TODO_RUST_CODE
            }
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: --offset;
            // TODO_RUST_CODE
         }
         // Java: if (text instanceof Spanned)
            // TODO_RUST_CODE
         {
            // Java: ReplacementSpan[] spans = (ReplacementSpan[])((Spanned)text).getSpans(offset, offset, ReplacementSpan.class);
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < spans.length; ++i)
                // TODO_RUST_CODE
            {
               // Java: int start = ((Spanned)text).getSpanStart(spans[i]);
               // TODO_RUST_CODE
               // Java: int end = ((Spanned)text).getSpanEnd(spans[i]);
               // TODO_RUST_CODE
               // Java: if (start < offset && end > offset)
                    // TODO_RUST_CODE
               {
                  // Java: offset = start;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: return offset;
         // TODO_RUST_CODE
      }
    }

    // Java: static public static <T> T[] removeEmptySpans(T[] spans, Spanned spanned, Class<T> klass)
    pub fn removeEmptySpans(&self, T[] spans, Spanned spanned, Class<T> klass) -> public static <T> T[] {
      // Java: T[] copy = null;
      // TODO_RUST_CODE
      // Java: int count = 0;
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < spans.length; ++i)
        // TODO_RUST_CODE
      {
         // Java: T span = (T)spans[i];
         // TODO_RUST_CODE
         // Java: int start = spanned.getSpanStart(span);
         // TODO_RUST_CODE
         // Java: int end = spanned.getSpanEnd(span);
         // TODO_RUST_CODE
         // Java: if (start == end)
            // TODO_RUST_CODE
         {
            // Java: if (copy == null)
                // TODO_RUST_CODE
            {
               // Java: copy = (T[])((Object[])Array.newInstance(klass, spans.length - 1));
               // TODO_RUST_CODE
               // Java: System.arraycopy(spans, 0, copy, 0, i);
               // TODO_RUST_CODE
               // Java: count = i;
               // TODO_RUST_CODE
            }
         }
         // Java: else if (copy != null)
            // TODO_RUST_CODE
         {
            // Java: copy[count] = span;
            // TODO_RUST_CODE
            // Java: ++count;
            // TODO_RUST_CODE
         }
      }
      // Java: if (copy != null)
        // TODO_RUST_CODE
      {
         // Java: T[] result = (T[])((Object[])Array.newInstance(klass, count));
         // TODO_RUST_CODE
         // Java: System.arraycopy(copy, 0, result, 0, count);
         // TODO_RUST_CODE
         // Java: return result;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return spans;
         // TODO_RUST_CODE
      }
    }

    // Java: static boolean isEmpty(CharSequence str)
    pub fn isEmpty(&self, CharSequence str) -> bool {
      // Java: return str == null || str.length() == 0;
      // TODO_RUST_CODE
    }

    // Java: static boolean doesNotNeedBidi(char[] text, int start, int len)
    pub fn doesNotNeedBidi(&self, char[] text, int start, int len) -> bool {
      // Java: int end = start + len;
      // TODO_RUST_CODE
      // Java: for(int i = start; i < end; ++i)
        // TODO_RUST_CODE
      {
         // Java: if (couldAffectRtl(text[i]))
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
      }
      // Java: return true;
      // TODO_RUST_CODE
    }

    // Java: static boolean couldAffectRtl(char c)
    pub fn couldAffectRtl(&self, char c) -> bool {
      // Java: return 1424 <= c && c <= 2303 || c == 8206 || c == 8207 || 8234 <= c && c <= 8238 || 8294 <= c && c <= 8297 || '\ud800' <= c && c <= '\udfff' || 'יִ' <= c && c <= '\ufdff' || 'ﹰ' <= c && c <= '\ufefe';
      // TODO_RUST_CODE
    }

    // Java: static String substring(CharSequence source, int start, int end)
    pub fn substring(&self, CharSequence source, int start, int end) -> String {
      // Java: if (source instanceof String)
        // TODO_RUST_CODE
      {
         // Java: return ((String)source).substring(start, end);
         // TODO_RUST_CODE
      }
      // Java: else if (source instanceof StringBuilder)
        // TODO_RUST_CODE
      {
         // Java: return ((StringBuilder)source).substring(start, end);
         // TODO_RUST_CODE
      }
      // Java: else if (source instanceof StringBuffer)
        // TODO_RUST_CODE
      {
         // Java: return ((StringBuffer)source).substring(start, end);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: char[] temp = obtain(end - start);
         // TODO_RUST_CODE
         // Java: getChars(source, start, end, temp, 0);
         // TODO_RUST_CODE
         // Java: String ret = new String(temp, 0, end - start);
         // TODO_RUST_CODE
         // Java: recycle(temp);
         // TODO_RUST_CODE
         // Java: return ret;
         // TODO_RUST_CODE
      }
    }

}

// Java: public enum TruncateAt
impl TruncateAt {
    // Java: TruncateAt()
    pub fn TruncateAt(&self) {
    // TODO_RUST_CODE
    }

}
