//! Run width cache keyed by (cp, font_id, size). Miss calls the injected measurer.

use std::collections::HashMap;

pub type MeasureFn = Box<dyn FnMut(&str, i32, i32) -> i32>;

pub struct RunWidthCache {
    map: HashMap<(i32, i32, i32), i32>,
    measure: MeasureFn,
    pub measure_count: usize,
}

impl RunWidthCache {
    pub fn with_measurer<F>(f: F) -> Self
    where
        F: FnMut(&str, i32, i32) -> i32 + 'static,
    {
        RunWidthCache {
            map: HashMap::new(),
            measure: Box::new(f),
            measure_count: 0,
        }
    }

    pub fn get_run_width(&mut self, cp: i32, text: &str, font_id: i32, size: i32) -> i32 {
        if text.is_empty() {
            self.map.insert((cp, font_id, size), 0);
            return 0;
        }
        let key = (cp, font_id, size);
        if let Some(&w) = self.map.get(&key) {
            return w;
        }
        self.measure_count += 1;
        let w = (self.measure)(text, font_id, size);
        self.map.insert(key, w);
        w
    }

    /// Same shift rule as TypoRangeBase: entries at/after `cp` move by insert−remove.
    pub fn shift_cp(&mut self, cp: i32, remove_length: i32, insert_length: i32) {
        let mut next = HashMap::new();
        let delta = insert_length - remove_length;
        for ((kcp, font, size), w) in self.map.drain() {
            if kcp < cp {
                next.insert((kcp, font, size), w);
            } else if kcp >= cp + remove_length {
                next.insert((kcp + delta, font, size), w);
            }
        }
        self.map = next;
    }

    pub fn contains(&self, cp: i32, font_id: i32, size: i32) -> bool {
        self.map.contains_key(&(cp, font_id, size))
    }
}
