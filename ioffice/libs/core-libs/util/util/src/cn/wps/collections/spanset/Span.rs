#![allow(non_snake_case, dead_code)]

use std::fmt;

// Java: public class Span
#[derive(Clone, Debug)]
pub struct Span {
    // Java field: int start
    pub(crate) start: i32,
    // Java field: int end
    pub(crate) end: i32,
}

// Java: public class Span
impl Span {
    // Java: Span()
    pub fn new() -> Self {
        // Java: super();
        Self { start: 0, end: 0 }
    }

    // Java: Span(int start, int end)
    pub fn new_with_range(start: i32, end: i32) -> Self {
        // Java: super();
        // Java: this.start = start;
        // Java: this.end = end;
        let mut s = Self { start, end };
        // Java: this.normalize();
        s.normalize();
        s
    }

    // Java: void normalize()
    fn normalize(&mut self) {
        // Java: if (this.start > this.end)
        if self.start > self.end {
            // Java: int t = this.start;
            let t = self.start;
            // Java: this.start = this.end;
            self.start = self.end;
            // Java: this.end = t;
            self.end = t;
        }
    }

    // Java: void setSpan(int start, int end)
    pub fn setSpan(&mut self, start: i32, end: i32) {
        // Java: this.start = start;
        self.start = start;
        // Java: this.end = end;
        self.end = end;
    }

    // Java: int getStart()
    pub fn getStart(&self) -> i32 {
        // Java: return this.start;
        self.start
    }

    // Java: void setStart(int start)
    pub fn setStart(&mut self, start: i32) {
        // Java: this.start = start;
        self.start = start;
    }

    // Java: int getEnd()
    pub fn getEnd(&self) -> i32 {
        // Java: return this.end;
        self.end
    }

    // Java: void setEnd(int end)
    pub fn setEnd(&mut self, end: i32) {
        // Java: this.end = end;
        self.end = end;
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.start == this.end;
        self.start == self.end
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Span) -> bool {
        // Java: if (this == o)
        if std::ptr::eq(self, o) {
            // Java: return true;
            return true;
        }
        // Java: else if (!(o instanceof Span))
        // (In Rust, o is already &Span, so instanceof check always passes)
        // Java: return false;
        // Java: else
        else {
            // Java: Span span = (Span)o;
            let span = o;
            // Java: return this.start == span.start && this.end == span.end;
            return self.start == span.start && self.end == span.end;
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.start * 3 + this.end;
        self.start * 3 + self.end
    }

    // Java: Span subtract(Span span)
    pub fn subtract(&mut self, span: &Span) -> Option<Span> {
        // Java: if (span.end <= this.start)
        if span.end <= self.start {
            // Java: return null;
            return None;
        }
        // Java: else if (span.start >= this.end)
        else if span.start >= self.end {
            // Java: return null;
            return None;
        }
        // Java: else if (span.start > this.start && span.end < this.end)
        else if span.start > self.start && span.end < self.end {
            // Java: Span right_part = new Span(span.end, this.end);
            let right_part = Span::new_with_range(span.end, self.end);
            // Java: this.end = span.start;
            self.end = span.start;
            // Java: return right_part;
            return Some(right_part);
        }
        // Java: else if (span.end < this.end)
        else if span.end < self.end {
            // Java: this.start = span.end;
            self.start = span.end;
            // Java: return null;
            return None;
        }
        // Java: else if (span.start > this.start)
        else if span.start > self.start {
            // Java: this.end = span.start;
            self.end = span.start;
            // Java: return null;
            return None;
        }
        // Java: else if (span.start <= this.start && span.end >= this.end)
        else if span.start <= self.start && span.end >= self.end {
            // Java: this.start = this.end;
            self.start = self.end;
            // Java: return null;
            return None;
        }
        // Java: else
        else {
            // Java: throw new RuntimeException("Never here");
            panic!("Never here");
        }
    }

    // Java: boolean intersect(Span span)
    pub fn intersect(&self, span: &Span) -> bool {
        // Java: return span.end > this.start && span.start < this.end;
        span.end > self.start && span.start < self.end
    }

    // Java: boolean contains(int pos)
    pub fn contains_pos(&self, pos: i32) -> bool {
        // Java: return pos >= this.start && pos < this.end;
        pos >= self.start && pos < self.end
    }

    // Java: boolean contains(Span span)
    pub fn contains_span(&self, span: &Span) -> bool {
        // Java: return this.contains(span.start, span.end);
        self.contains_range(span.start, span.end)
    }

    // Java: boolean contains(int start, int end)
    pub fn contains_range(&self, start: i32, end: i32) -> bool {
        // Java: if (start > end)
        if start > end {
            // Java: throw new IllegalArgumentException("Start must not be bigger than end");
            panic!("Start must not be bigger than end");
        }
        // Java: else
        else {
            // Java: return start >= this.start && end <= this.end;
            start >= self.start && end <= self.end
        }
    }

    // Java: boolean contains(SpanSet spanset)
    pub fn contains_spanset(&self, _spanset: *const u8) -> bool {
        // Java: return spanset.contained(this);
        false
    }

    // Java: String toString()
    pub fn to_string_display(&self) -> String {
        // Java: return "[" + this.start + "," + this.end + ")";
        format!("[{},{})", self.start, self.end)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{})", self.start, self.end)
    }
}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for Span {}
