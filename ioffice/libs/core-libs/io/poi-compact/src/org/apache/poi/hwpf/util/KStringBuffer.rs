// Java: import java.io.IOException

use std::sync::{Arc, Mutex};

// Java: public class KStringBuffer implements Appendable, CharSequence
pub struct KStringBuffer {
    // Java field: int count
    pub(crate) count: i32,
    // Java field: char[] value
    pub(crate) value: Vec<char>,
}

// Java: public class KStringBuffer implements Appendable, CharSequence
impl KStringBuffer {
    pub fn new_empty() -> Self {
        KStringBuffer { count: 0, value: Vec::new() }
    }

    pub fn new_with_capacity(capacity: i32) -> Self {
        KStringBuffer { count: 0, value: Vec::with_capacity(capacity as usize) }
    }

    pub fn new_from_str(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();
        let count = chars.len() as i32;
        KStringBuffer { count, value: chars }
    }

    pub fn length(&self) -> i32 {
        self.count
    }

    pub fn to_string(&self) -> String {
        self.value[..self.count as usize].iter().collect()
    }

    pub fn substring(&self, start: i32, end: i32) -> String {
        self.value[start as usize..end as usize].iter().collect()
    }

    pub fn getDirectBuffer(&self) -> &[char] {
        &self.value[..self.count as usize]
    }

    pub fn delete(&mut self, start: i32, end: i32) {
        let start = start as usize;
        let end = end as usize;
        if end <= self.value.len() && start <= end {
            self.value.drain(start..end);
            self.count = self.value.len() as i32;
        }
    }

    pub fn insert_str(&mut self, offset: i32, s: &str) {
        let chars: Vec<char> = s.chars().collect();
        let offset = offset as usize;
        for (i, c) in chars.iter().enumerate() {
            self.value.insert(offset + i, *c);
        }
        self.count = self.value.len() as i32;
    }

    pub fn append_char(&mut self, c: char) {
        self.value.push(c);
        self.count += 1;
    }

    pub fn append_str(&mut self, s: &str) {
        for c in s.chars() {
            self.value.push(c);
        }
        self.count = self.value.len() as i32;
    }
}
