#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.util.Arrays
// Java: import java.util.Collection

// Java: public class StringUtils
pub struct StringUtils {
}

// Java: public class StringUtils
impl StringUtils {
    // Java: StringUtils()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: Object clone()
    // Java: throw new CloneNotSupportedException();

    // Java: static String substringBefore(String s, String delim)
    pub fn substringBefore(s: &str, delim: &str) -> Option<String> {
        // Java: int i = s.indexOf(delim);
        let i = s.find(delim);
        // Java: return i == -1 ? null : s.substring(0, i);
        i.map(|idx| s[..idx].to_string())
    }

    // Java: static String substringBefore(String s, char delim)
    pub fn substringBefore_char(s: &str, delim: char) -> Option<String> {
        // Java: int i = s.indexOf(delim);
        let i = s.find(delim);
        // Java: return i == -1 ? null : s.substring(0, i);
        i.map(|idx| s[..idx].to_string())
    }

    // Java: static String substringAfter(String s, String delim)
    pub fn substringAfter(s: &str, delim: &str) -> Option<String> {
        // Java: int i = s.indexOf(delim);
        let i = s.find(delim);
        // Java: return i == -1 ? null : s.substring(i + delim.length());
        i.map(|idx| s[idx + delim.len()..].to_string())
    }

    // Java: static String substringAfter(String s, char delim)
    pub fn substringAfter_char(s: &str, delim: char) -> Option<String> {
        // Java: int i = s.indexOf(delim);
        let i = s.find(delim);
        // Java: return i == -1 ? null : s.substring(i + 1);
        i.map(|idx| s[idx + 1..].to_string())
    }

    // Java: static boolean isEmpty(CharSequence s)
    pub fn isEmpty(s: Option<&str>) -> bool {
        // Java: return s == null || s.length() == 0;
        s.map_or(true, |v| v.is_empty())
    }

    // Java: static boolean isBlank(CharSequence s)
    pub fn isBlank(s: Option<&str>) -> bool {
        // Java: if (isEmpty(s))
        if Self::isEmpty(s)
        {
            // Java: return true;
            return true;
        }
        // Java: else
        {
            let s = s.unwrap();
            // Java: for(int i = 0; i < s.length(); ++i)
            for ch in s.chars()
            {
                // Java: char ch = s.charAt(i);
                // Java: if (!Character.isWhitespace(ch) && isPrintable(ch))
                if !ch.is_whitespace() && Self::isPrintable(ch)
                {
                    // Java: return false;
                    return false;
                }
            }
            // Java: return true;
            true
        }
    }

    // Java: static boolean isPrintable(char ch)
    fn isPrintable(ch: char) -> bool {
        // Java: return ch > ' ';
        ch > ' '
    }

    // Java: static boolean isBlank(_array: &[char], int start, int length)
    pub fn isBlank_array(array: Option<&[char]>, start: usize, length: usize) -> bool {
        // Java: if (array != null && length != 0)
        if let Some(arr) = array {
            if length != 0
            {
                // Java: for(int i = 0; i < length; ++i)
                for i in 0..length
                {
                    // Java: if (!Character.isWhitespace(array[start + i]))
                    if !arr[start + i].is_whitespace()
                    {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: return true;
                return true;
            }
        }
        // Java: else
        {
            // Java: return true;
            true
        }
    }

    // Java: static String unquote(String s)
    pub fn unquote(s: Option<&str>) -> Option<String> {
        // Java: if (s == null)
        if s.is_none()
        {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            let s = s.unwrap();
            // Java: return s.length() <= 1 || (!s.startsWith("'") || !s.endsWith("'")) && (!s.startsWith("\"") || !s.endsWith("\"")) ? s : s.substring(1, s.length() - 1);
            if s.len() <= 1 || ((!s.starts_with('\'') || !s.ends_with('\'')) && (!s.starts_with('"') || !s.ends_with('"'))) {
                Some(s.to_string())
            } else {
                Some(s[1..s.len() - 1].to_string())
            }
        }
    }

    // Java: static boolean isIn(String search, String... choice)
    pub fn isIn(search: &str, choice: &[&str]) -> bool {
        // Java: for(String s : choice)
        for s in choice
        {
            // Java: if (s.equals(search))
            if *s == search
            {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: static boolean isInIgnoreCase(String search, String... choice)
    pub fn isInIgnoreCase(search: &str, choice: &[&str]) -> bool {
        // Java: for(String s : choice)
        for s in choice
        {
            // Java: if (s.equalsIgnoreCase(search))
            if s.eq_ignore_ascii_case(search)
            {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: static boolean startsWith(String search, String... starts)
    pub fn starts_with_any(search: &str, starts: &[&str]) -> bool {
        // Java: for(String s : starts)
        for s in starts
        {
            // Java: if (search.startsWith(s))
            if search.starts_with(s)
            {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: static boolean endsWith(String search, String... ends)
    pub fn ends_with_any(search: &str, ends: &[&str]) -> bool {
        // Java: for(String s : ends)
        for s in ends
        {
            // Java: if (search.endsWith(s))
            if search.ends_with(s)
            {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: static String ljust(String s, int width, char fillchar)
    pub fn ljust(s: &str, width: usize, fillchar: char) -> String {
        // Java: if (s != null && s.length() < width)
        if s.len() < width
        {
            // Java: StringBuilder b = new StringBuilder(width);
            let mut b = String::with_capacity(width);
            // Java: b.append(s);
            b.push_str(s);
            // Java: while(b.length() < width)
            while b.len() < width
            {
                // Java: b.append(fillchar);
                b.push(fillchar);
            }
            // Java: return b.toString();
            b
        }
        // Java: else
        else
        {
            // Java: return s;
            s.to_string()
        }
    }

    // Java: static String ljust(String s, int width)
    pub fn ljust_default(s: &str, width: usize) -> String {
        // Java: return ljust(s, width, ' ');
        Self::ljust(s, width, ' ')
    }

    // Java: static String rjust(String s, int width, char fillchar)
    pub fn rjust(s: &str, width: usize, fillchar: char) -> String {
        // Java: if (s != null && s.length() < width)
        if s.len() < width
        {
            // Java: StringBuilder b = new StringBuilder(width);
            let mut b = String::with_capacity(width);
            // Java: while(b.length() < width - s.length())
            while b.len() < width - s.len()
            {
                // Java: b.append(fillchar);
                b.push(fillchar);
            }
            // Java: b.append(s);
            b.push_str(s);
            // Java: return b.toString();
            b
        }
        // Java: else
        else
        {
            // Java: return s;
            s.to_string()
        }
    }

    // Java: static String rjust(String s, int width)
    pub fn rjust_default(s: &str, width: usize) -> String {
        // Java: return rjust(s, width, ' ');
        Self::rjust(s, width, ' ')
    }

    // Java: static String swapcase(String s)
    pub fn swapcase(s: &str) -> String {
        // Java: StringBuilder b = new StringBuilder(s.length());
        let mut b = String::with_capacity(s.len());
        // Java: for(int i = 0; i < s.length(); ++i)
        for c in s.chars()
        {
            // Java: char c = s.charAt(i);
            // Java: if (Character.isLetter(c))
            if c.is_alphabetic()
            {
                // Java: if (Character.isUpperCase(c))
                if c.is_uppercase()
                {
                    // Java: c = Character.toLowerCase(c);
                    for lc in c.to_lowercase() {
                        b.push(lc);
                    }
                }
                // Java: else
                else
                {
                    // Java: c = Character.toUpperCase(c);
                    for uc in c.to_uppercase() {
                        b.push(uc);
                    }
                }
            } else {
                // Java: b.append(c);
                b.push(c);
            }
        }
        // Java: return b.toString();
        b
    }

    // Java: static String join(Collection<?> c, String sep)
    pub fn join(c: &[&str], sep: &str) -> String {
        // Java: StringBuilder b = new StringBuilder();
        // Java: boolean first = true;
        // Java: for(Object o : c)
        // Java: if (!first)
        // Java: b.append(sep);
        // Java: first = false;
        // Java: b.append(String.valueOf(o));
        // Java: return b.toString();
        c.join(sep)
    }

    // Java: static String join(Collection<?> c)
    pub fn join_default(c: &[&str]) -> String {
        // Java: return join(c, " ");
        Self::join(c, " ")
    }

    // Java: static String join(Object[] c, String sep)
    pub fn join_array(c: &[&str], sep: &str) -> String {
        // Java: return join(Arrays.asList(c), sep);
        Self::join(c, sep)
    }

    // Java: static String join(Object[] c)
    pub fn join_array_default(c: &[&str]) -> String {
        // Java: return join(Arrays.asList(c));
        Self::join_default(c)
    }
}
