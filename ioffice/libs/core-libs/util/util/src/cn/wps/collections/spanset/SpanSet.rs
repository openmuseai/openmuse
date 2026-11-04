#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.ListIterator

use super::Span::Span;

// Java: public class SpanSet implements Cloneable, Iterable
#[derive(Clone)]
pub struct SpanSet {
    // Java field: ArrayList<Span> spans
    pub(crate) spans: Vec<Span>,
}

// Java: public class SpanSet implements Cloneable, Iterable
impl SpanSet {
    // Java: SpanSet()
    pub fn new() -> Self {
        // Java: super();
        Self { spans: Vec::new() }
    }

    // Java: SpanSet(Span span)
    pub fn new_with_span(span: Span) -> Self {
        // Java: super();
        let mut s = Self { spans: Vec::new() };
        // Java: if (!span.isEmpty())
        if !span.isEmpty() {
            // Java: this.spans.add(span);
            s.spans.push(span);
        }
        s
    }

    // Java: SpanSet(Span... spans)
    pub fn new_with_spans(spans: &[Span]) -> Self {
        // Java: super();
        let mut s = Self { spans: Vec::new() };
        // Java: if (spans != null && spans.length != 0)
        if spans.is_empty() {
            return s;
        }
        {
            // Java: Span prev_span = spans[0];
            let mut prev_span = spans[0].clone();
            // Java: if (prev_span.isEmpty())
            if prev_span.isEmpty() {
                // Java: throw new IllegalArgumentException("Empty span is not allowed");
                panic!("Empty span is not allowed");
            }
            // Java: else
            else {
                // Java: this.spans.add(prev_span);
                s.spans.push(prev_span.clone());
                // Java: int i = 1;
                let mut i: usize = 1;
                // Java: for(int n = spans.length; i < n; ++i)
                let n = spans.len();
                while i < n {
                    // Java: Span cur_span = spans[i];
                    let cur_span = &spans[i];
                    // Java: if (cur_span.isEmpty())
                    if cur_span.isEmpty() {
                        // Java: throw new IllegalArgumentException("Empty span is not allowed");
                        panic!("Empty span is not allowed");
                    }
                    // Java: if (cur_span.start < prev_span.end)
                    if cur_span.start < prev_span.end {
                        // Java: throw new IllegalArgumentException("One of the span is not ordered correctly");
                        panic!("One of the span is not ordered correctly");
                    }
                    // Java: if (cur_span.start == prev_span.end)
                    if cur_span.start == prev_span.end {
                        // Java: prev_span.end = cur_span.end;
                        let last = s.spans.last_mut().unwrap();
                        last.end = cur_span.end;
                        prev_span.end = cur_span.end;
                    }
                    // Java: else
                    else {
                        // Java: this.spans.add(cur_span);
                        s.spans.push(cur_span.clone());
                        // Java: prev_span = cur_span;
                        prev_span = cur_span.clone();
                    }
                    i += 1;
                }
                // Java: this.check_self();
                s.check_self();
            }
        }
        s
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.spans.isEmpty();
        self.spans.is_empty()
    }

    // Java: void check_self()
    fn check_self(&self) {
        // Java: if (!this.spans.isEmpty())
        if !self.spans.is_empty() {
            // Java: Span prev_span = (Span)this.spans.get(0);
            let prev_span = &self.spans[0];
            // Java: if (prev_span.isEmpty())
            if prev_span.isEmpty() {
                // Java: throw new RuntimeException("One of the spans is empty");
                panic!("One of the spans is empty");
            }
            // Java: else
            else {
                // Java: int i = 1;
                let mut i: usize = 1;
                // Java: for(int n = this.spans.size(); i < n; ++i)
                let n = self.spans.len();
                while i < n {
                    // Java: Span cur_span = (Span)this.spans.get(i);
                    let cur_span = &self.spans[i];
                    // Java: if (cur_span.isEmpty())
                    if cur_span.isEmpty() {
                        // Java: throw new RuntimeException("One of the spans is empty");
                        panic!("One of the spans is empty");
                    }
                    // Java: if (cur_span.start < prev_span.end)
                    if cur_span.start < self.spans[i - 1].end {
                        // Java: throw new RuntimeException("One of the spans is not ordered correctly");
                        panic!("One of the spans is not ordered correctly");
                    }
                    // Java: if (cur_span.start == prev_span.end)
                    if cur_span.start == self.spans[i - 1].end {
                        // Java: throw new RuntimeException("One of the spans must be merged");
                        panic!("One of the spans must be merged");
                    }
                    i += 1;
                }
            }
        }
    }

    // Java: SpanSet(SpanSet spanset)
    pub fn new_from_spanset(spanset: &SpanSet) -> Self {
        // Java: super();
        let mut s = Self { spans: Vec::new() };
        // Java: int i = 0;
        let mut i: usize = 0;
        // Java: for(int n = spanset.spans.size(); i < n; ++i)
        let n = spanset.spans.len();
        while i < n {
            // Java: Span span = (Span)spanset.spans.get(i);
            let span = &spanset.spans[i];
            // Java: this.spans.add(new Span(span.start, span.end));
            s.spans.push(Span::new_with_range(span.start, span.end));
            i += 1;
        }
        // Java: this.check_self();
        s.check_self();
        s
    }

    // Java: void add(int start, int end)
    pub fn add_range(&mut self, start: i32, end: i32) {
        // Java: this.add(new Span(start, end));
        self.add_span(Span::new_with_range(start, end));
    }

    // Java: void add(Span span)
    pub fn add_span(&mut self, span: Span) {
        // Java: if (!span.isEmpty())
        if !span.isEmpty() {
            // Java: if (this.spans.isEmpty())
            if self.spans.is_empty() {
                // Java: this.spans.add(span);
                self.spans.push(span);
            }
            // Java: else
            else {
                // Java: this.add_span(span);
                self.add_span_internal(span);
            }
            // Java: this.check_self();
            self.check_self();
        }
    }

    // Java: void add_span(Span span)
    fn add_span_internal(&mut self, span: Span) {
        // Java: int s = this.split(span.start);
        let s = self.split(span.start);
        // Java: int e = this.split(span.end);
        let e = self.split(span.end);
        // Java: int n = e - s;
        let mut n = (e - s) as i32;
        // Java: while(n-- > 0)
        while n > 0 {
            n -= 1;
            // Java: this.spans.remove(s);
            self.spans.remove(s);
        }
        // Java: this.spans.add(s, span);
        self.spans.insert(s, span);
        // Java: if (this.merge(s - 1))
        if s > 0 && self.merge(s - 1) {
            // Java: this.merge(s - 1);
            self.merge(s - 1);
        }
        // Java: else
        else {
            // Java: this.merge(s);
            self.merge(s);
        }
    }

    // Java: boolean merge(int index)
    fn merge(&mut self, index: usize) -> bool {
        // Java: if (index >= 0 && index != this.spans.size() - 1)
        if index < self.spans.len() && index != self.spans.len() - 1 {
            // Java: if (((Span)this.spans.get(index)).end == ((Span)this.spans.get(index + 1)).start)
            if self.spans[index].end == self.spans[index + 1].start {
                // Java: ((Span)this.spans.get(index)).end = ((Span)this.spans.get(index + 1)).end;
                self.spans[index].end = self.spans[index + 1].end;
                // Java: this.spans.remove(index + 1);
                self.spans.remove(index + 1);
                // Java: return true;
                return true;
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            return false;
        }
    }

    // Java: int split(int pos)
    fn split(&mut self, pos: i32) -> usize {
        // Java: int index;
        let mut index: usize = 0;
        // Java: for(index = 0; index < this.spans.size(); ++index)
        while index < self.spans.len() {
            // Java: Span span = (Span)this.spans.get(index);
            let span = &self.spans[index];
            // Java: if (span.end > pos)
            if span.end > pos {
                // Java: break;
                break;
            }
            index += 1;
        }
        // Java: if (index == this.spans.size())
        if index == self.spans.len() {
            // Java: return index;
            return index;
        }
        // Java: else if (((Span)this.spans.get(index)).start >= pos)
        else if self.spans[index].start >= pos {
            // Java: return index;
            return index;
        }
        // Java: else
        else {
            // Java: this.spans.add(index + 1, new Span(pos, ((Span)this.spans.get(index)).end));
            let end_val = self.spans[index].end;
            self.spans.insert(index + 1, Span::new_with_range(pos, end_val));
            // Java: ((Span)this.spans.get(index)).end = pos;
            self.spans[index].end = pos;
            // Java: return index + 1;
            return index + 1;
        }
    }

    // Java: void add(Span... spans)
    pub fn add_spans(&mut self, spans: &[Span]) {
        // Java: if (spans != null && spans.length != 0)
        if !spans.is_empty() {
            // Java: for(Span span : spans)
            for span in spans {
                // Java: this.add(span);
                self.add_span(span.clone());
            }
        }
    }

    // Java: void add(SpanSet spanset)
    pub fn add_spanset(&mut self, spanset: &SpanSet) {
        // Java: for(Span span : spanset.spans)
        for span in &spanset.spans {
            // Java: this.add(span.start, span.end);
            self.add_range(span.start, span.end);
        }
    }

    // Java: void add_non_intersect(SpanSet spanset)
    pub fn add_non_intersect(&mut self, spanset: &SpanSet) {
        // Java: this.add(spanset);
        self.add_spanset(spanset);
    }

    // Java: void subtract(int start, int end)
    pub fn subtract_range(&mut self, start: i32, end: i32) {
        // Java: if (start != end)
        if start != end {
            // Java: if (start > end)
            if start > end {
                // Java: throw new IllegalArgumentException("Start must be less than end");
                panic!("Start must be less than end");
            }
            // Java: else
            else {
                // Java: int s = this.split(start);
                let s = self.split(start);
                // Java: int e = this.split(end);
                let e = self.split(end);
                // Java: int n = e - s;
                let mut n = (e - s) as i32;
                // Java: while(n-- > 0)
                while n > 0 {
                    n -= 1;
                    // Java: this.spans.remove(s);
                    self.spans.remove(s);
                }
                // Java: this.check_self();
                self.check_self();
            }
        }
    }

    // Java: void subtract(Span span)
    pub fn subtract_span(&mut self, span: &Span) {
        // Java: this.subtract(span.start, span.end);
        self.subtract_range(span.start, span.end);
    }

    // Java: void subtract(SpanSet spanset)
    pub fn subtract_spanset(&mut self, spanset: &SpanSet) {
        // Java: for(Span span : spanset.spans)
        for span in &spanset.spans.clone() {
            // Java: this.subtract(span.start, span.end);
            self.subtract_range(span.start, span.end);
        }
    }

    // Java: boolean intersect(Span span)
    pub fn intersect_span(&self, span: &Span) -> bool {
        // Java: for(Span s : this.spans)
        for s in &self.spans {
            // Java: if (s.intersect(span))
            if s.intersect(span) {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        return false;
    }

    // Java: boolean intersect(SpanSet spanset)
    pub fn intersect_spanset(&self, spanset: &SpanSet) -> bool {
        // Java: for(Span span : spanset)
        for span in &spanset.spans {
            // Java: if (this.intersect(span))
            if self.intersect_span(span) {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        return false;
    }

    // Java: boolean contains(Span span)
    pub fn contains_span(&self, span: &Span) -> bool {
        // Java: return this.contains(span.start, span.end);
        self.contains_range(span.start, span.end)
    }

    // Java: boolean contains(int start, int end)
    pub fn contains_range(&self, start: i32, end: i32) -> bool {
        // Java: for(Span s : this.spans)
        for s in &self.spans {
            // Java: if (s.contains(start, end))
            if s.contains_range(start, end) {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        return false;
    }

    // Java: boolean contains(SpanSet spanset)
    pub fn contains_spanset(&self, spanset: &SpanSet) -> bool {
        // Java: for(Span span : spanset)
        for span in &spanset.spans {
            // Java: if (!this.contains(span))
            if !self.contains_span(span) {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        return true;
    }

    // Java: boolean contained(Span span)
    pub fn contained(&self, span: &Span) -> bool {
        // Java: for(Span s : this.spans)
        for s in &self.spans {
            // Java: if (!span.contains(s))
            if !span.contains_span(s) {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        return true;
    }

    // Java: boolean equals(Span... o)
    pub fn equals_spans(&self, o: &[Span]) -> bool {
        // Java: if (o == null)
        // (Rust slices can't be null)
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: int n = this.spans.size();
            let n = self.spans.len();
            // Java: if (o.length != n)
            if o.len() != n {
                // Java: return false;
                return false;
            }
            // Java: else
            else {
                // Java: for(int i = 0; i < n; ++i)
                for i in 0..n {
                    // Java: if (!((Span)this.spans.get(i)).equals(o[i]))
                    if !self.spans[i].equals(&o[i]) {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: return true;
                return true;
            }
        }
    }

    // Java: boolean equals(Iterable<Span> o)
    pub fn equals_iter<'a, I: IntoIterator<Item = &'a Span>>(&self, o: I) -> bool {
        // Java: if (o == null)
        // (Rust references can't be null)
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: int n = this.spans.size();
            let n = self.spans.len();
            // Java: int i = 0;
            let mut i: usize = 0;
            // Java: for(Span s : o)
            for s in o {
                // Java: if (i >= n)
                if i >= n {
                    // Java: return false;
                    return false;
                }
                // Java: if (!((Span)this.spans.get(i)).equals(s))
                if !self.spans[i].equals(s) {
                    // Java: return false;
                    return false;
                }
                // Java: ++i;
                i += 1;
            }
            // Java: if (i != n)
            if i != n {
                // Java: return false;
                return false;
            }
            // Java: else
            else {
                // Java: return true;
                return true;
            }
        }
    }

    // Java: boolean equals(Span span)
    pub fn equals_span(&self, span: &Span) -> bool {
        // Java: return this.spans.size() == 1 && ((Span)this.spans.get(0)).equals(span);
        self.spans.len() == 1 && self.spans[0].equals(span)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &SpanSet) -> bool {
        // Java: return o != null && o instanceof SpanSet ? this.equals((Iterable)((SpanSet)o).spans) : false;
        self.equals_iter(&o.spans)
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int n = 0;
        let mut n: i32 = 0;
        // Java: for(Span span : this.spans)
        for span in &self.spans {
            // Java: n += span.hashCode();
            n += span.hashCode();
        }
        // Java: return n;
        return n;
    }

    // Java: Object clone()
    pub fn clone_spanset(&self) -> SpanSet {
        // Java: return new SpanSet(this);
        SpanSet::new_from_spanset(self)
    }

    // Java: Iterator<Span> iterator()
    pub fn iter(&self) -> std::slice::Iter<'_, Span> {
        // Java: return this.spans.iterator();
        self.spans.iter()
    }

    // Java: ListIterator<Span> iterator(int index)
    pub fn iter_from(&self, index: usize) -> std::slice::Iter<'_, Span> {
        // Java: return this.spans.listIterator(index);
        self.spans[index..].iter()
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.spans.size();
        self.spans.len() as i32
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.spans.clear();
        self.spans.clear();
    }
}
