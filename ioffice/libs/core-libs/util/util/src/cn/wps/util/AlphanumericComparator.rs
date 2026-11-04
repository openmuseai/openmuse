#![allow(non_snake_case, dead_code, unused_variables, unused_mut)]

struct CharBuffer {
    chars: Vec<char>,
    position: usize,
    limit: usize,
    capacity: usize,
}

impl CharBuffer {
    fn wrap(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        CharBuffer {
            chars,
            position: 0,
            limit: len,
            capacity: len,
        }
    }

    fn hasRemaining(&self) -> bool {
        self.position < self.limit
    }

    fn position(&self) -> usize {
        self.position
    }

    fn limit(&self) -> usize {
        self.limit
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn get(&self, index: usize) -> char {
        self.chars[index]
    }

    fn charAt(&self, index: usize) -> char {
        self.chars[self.position + index]
    }

    fn remaining(&self) -> usize {
        self.limit - self.position
    }

    fn length(&self) -> usize {
        self.limit - self.position
    }

    fn set_position(&mut self, pos: usize) {
        self.position = pos;
    }

    fn set_limit(&mut self, lim: usize) {
        self.limit = lim;
    }

    fn to_string_range(&self) -> String {
        self.chars[self.position..self.limit].iter().collect()
    }
}

pub struct Collator;

impl Collator {
    pub fn compare(&self, s1: &str, s2: &str) -> i32 {
        match s1.cmp(s2) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

// Java: public class AlphanumericComparator implements Comparator
pub struct AlphanumericComparator {
    // Java field: Collator collator
    collator: Option<Collator>,
}

// Java: public class AlphanumericComparator implements Comparator
impl AlphanumericComparator {
    // Java: AlphanumericComparator(Collator collator)
    pub fn new(collator: Option<Collator>) -> Self {
        // Java: super();
        // Java: this.collator = collator;
        Self { collator }
    }

    // Java: int compare(CharSequence s1, CharSequence s2)
    pub fn compare_str(&self, s1: &str, s2: &str) -> i32 {
        // Java: CharBuffer b1 = CharBuffer.wrap(s1);
        let mut b1 = CharBuffer::wrap(s1);
        // Java: CharBuffer b2 = CharBuffer.wrap(s2);
        let mut b2 = CharBuffer::wrap(s2);
        // Java: while(b1.hasRemaining() && b2.hasRemaining())
        while b1.hasRemaining() && b2.hasRemaining()
        {
            // Java: this.moveWindow(b1);
            self.moveWindow(&mut b1);
            // Java: this.moveWindow(b2);
            self.moveWindow(&mut b2);
            // Java: int result = this.compare(b1, b2);
            let result = self.compare_buffers(&b1, &b2);
            // Java: if (result != 0)
            if result != 0
            {
                // Java: return result;
                return result;
            }
            // Java: this.prepareForNextIteration(b1);
            self.prepareForNextIteration(&mut b1);
            // Java: this.prepareForNextIteration(b2);
            self.prepareForNextIteration(&mut b2);
        }
        // Java: return s1.length() - s2.length();
        return s1.chars().count() as i32 - s2.chars().count() as i32;
    }

    // Java: void moveWindow(CharBuffer buffer)
    fn moveWindow(&self, buffer: &mut CharBuffer) {
        // Java: int start = buffer.position();
        let mut start = buffer.position();
        // Java: int end = buffer.position();
        let mut end = buffer.position();
        // Java: boolean isNumerical = this.isDigit(buffer.get(start));
        let is_numerical = self.isDigit(buffer.get(start));
        // Java: while(end < buffer.limit() && isNumerical == this.isDigit(buffer.get(end)))
        while end < buffer.limit() && is_numerical == self.isDigit(buffer.get(end))
        {
            // Java: ++end;
            end += 1;
            // Java: if (isNumerical && start + 1 < buffer.limit() && this.isZero(buffer.get(start)) && this.isDigit(buffer.get(end)))
            if is_numerical && start + 1 < buffer.limit() && self.isZero(buffer.get(start)) && end < buffer.limit() && self.isDigit(buffer.get(end))
            {
                // Java: ++start;
                start += 1;
            }
        }
        // Java: buffer.position(start).limit(end);
        buffer.set_position(start);
        buffer.set_limit(end);
    }

    // Java: int compare(CharBuffer b1, CharBuffer b2)
    fn compare_buffers(&self, b1: &CharBuffer, b2: &CharBuffer) -> i32 {
        // Java: return this.isNumerical(b1) && this.isNumerical(b2) ? this.compareNumerically(b1, b2) : this.compareAsStrings(b1, b2);
        if self.isNumerical(b1) && self.isNumerical(b2) {
            self.compareNumerically(b1, b2)
        } else {
            self.compareAsStrings(b1, b2)
        }
    }

    // Java: boolean isNumerical(CharBuffer buffer)
    fn isNumerical(&self, buffer: &CharBuffer) -> bool {
        // Java: return this.isDigit(buffer.charAt(0));
        self.isDigit(buffer.charAt(0))
    }

    // Java: boolean isDigit(char c)
    fn isDigit(&self, c: char) -> bool {
        // Java: if (this.collator != null)
        if self.collator.is_some()
        {
            // Java: return Character.isDigit(c);
            return c.is_ascii_digit() || c.is_numeric();
        }
        // Java: else
        else
        {
            // Java: return c >= '0' && c <= '9';
            return c >= '0' && c <= '9';
        }
    }

    // Java: int compareNumerically(CharBuffer b1, CharBuffer b2)
    fn compareNumerically(&self, b1: &CharBuffer, b2: &CharBuffer) -> i32 {
        // Java: int diff = b1.length() - b2.length();
        let diff = b1.length() as i32 - b2.length() as i32;
        // Java: if (diff != 0)
        if diff != 0
        {
            // Java: return diff;
            return diff;
        }
        // Java: else
        else
        {
            // Java: for(int i = 0; i < b1.remaining() && i < b2.remaining(); ++i)
            let mut i = 0;
            while i < b1.remaining() && i < b2.remaining()
            {
                // Java: int result = b1.charAt(i) - b2.charAt(i);
                let result = b1.charAt(i) as i32 - b2.charAt(i) as i32;
                // Java: if (result != 0)
                if result != 0
                {
                    // Java: return result;
                    return result;
                }
                i += 1;
            }
            // Java: return 0;
            return 0;
        }
    }

    // Java: void prepareForNextIteration(CharBuffer buffer)
    fn prepareForNextIteration(&self, buffer: &mut CharBuffer) {
        // Java: buffer.position(buffer.limit()).limit(buffer.capacity());
        let lim = buffer.limit();
        let cap = buffer.capacity();
        buffer.set_position(lim);
        buffer.set_limit(cap);
    }

    // Java: int compareAsStrings(CharBuffer b1, CharBuffer b2)
    fn compareAsStrings(&self, b1: &CharBuffer, b2: &CharBuffer) -> i32 {
        // Java: return this.collator != null ? this.collator.compare(b1.toString(), b2.toString()) : b1.toString().compareTo(b2.toString());
        if let Some(ref collator) = self.collator {
            collator.compare(&b1.to_string_range(), &b2.to_string_range())
        } else {
            let s1 = b1.to_string_range();
            let s2 = b2.to_string_range();
            match s1.cmp(&s2) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            }
        }
    }

    // Java: boolean isZero(char c)
    fn isZero(&self, c: char) -> bool {
        // Java: return c == '0';
        c == '0'
    }
}
