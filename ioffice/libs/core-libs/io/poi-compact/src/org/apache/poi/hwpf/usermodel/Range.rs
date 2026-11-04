pub struct Range {
    pub _start: i32,
    pub _end: i32,
}

impl Range {
    pub fn new(_start: i32, _end: i32) -> Self {
        Range { _start, _end }
    }

    pub fn getStartOffset(&self) -> i32 {
        self._start
    }

    pub fn getEndOffset(&self) -> i32 {
        self._end
    }
}
