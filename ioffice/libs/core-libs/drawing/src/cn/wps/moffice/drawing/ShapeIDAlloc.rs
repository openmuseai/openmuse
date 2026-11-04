#![allow(non_snake_case)]

// Java: public class ShapeIDAlloc
pub struct ShapeIDAlloc {
    // Java field: int INTERVAL
    INTERVAL: i32,
    // Java field: int mStartID
    mStartID: i32,
}

// Java: public class Range
pub struct Range {
    // Java field: int start
    pub start: i32,
    // Java field: int end
    pub end: i32,
}

// Java: public class ShapeIDAlloc
impl ShapeIDAlloc {
    // Java: ShapeIDAlloc()
    pub fn new() -> Self {
        // Java: super();
        ShapeIDAlloc {
            INTERVAL: 1024,
            mStartID: 2048,
        }
    }

    // Java: Range getNextRange()
    pub fn getNextRange(&mut self) -> Range {
        // Java: int end = this.mStartID + 1024;
        let end = self.mStartID + 1024;
        // Java: Range range = new Range(this.mStartID, end);
        let range = Range::new(self.mStartID, end);
        // Java: this.mStartID = end;
        self.mStartID = end;
        // Java: return range;
        range
    }

    // Java: Range getMainStartRange()
    pub fn getMainStartRange(&self) -> Range {
        // Java: return new Range(1025, 2048);
        Range::new(1025, 2048)
    }
}

// Java: public class Range
impl Range {
    // Java: Range(int start, int end)
    pub fn new(start: i32, end: i32) -> Self {
        // Java: super();
        // Java: this.start = start;
        // Java: this.end = end;
        Range { start, end }
    }
}
