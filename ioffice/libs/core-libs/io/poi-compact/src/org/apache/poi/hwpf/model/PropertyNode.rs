use std::any::Any;
use super::CharIndexTranslator::CharIndexTranslator;

// Java: public class PropertyNode implements Comparable, Cloneable
pub struct PropertyNode {
    // Java field: Object _buf
    pub(crate) _buf: Box<dyn Any>,
    // Java field: CharIndexTranslator translator
    pub(crate) translator: Option<Box<dyn CharIndexTranslator>>,
    // Java field: int cpStart
    pub(crate) cpStart: i32,
    // Java field: int cpEnd
    pub(crate) cpEnd: i32,
}

// Java: public class PropertyNode implements Comparable, Cloneable
impl PropertyNode {
    // Java: PropertyNode(int fcStart, int fcEnd, Object buf)
    pub fn new(fcStart: i32, fcEnd: i32, buf: Box<dyn Any>) -> Self {
        // Java: super();
        // Java: this.cpStart = fcStart;
        let mut cpStart = fcStart;
        // Java: this.cpEnd = fcEnd;
        let cpEnd = fcEnd;
        // Java: this._buf = buf;
        // Java: if (this.cpStart < 0)
        if cpStart < 0 {
            // Java: System.err.println("A property claimed to start before zero, at " + this.cpStart + "! Resetting it to zero, and hoping for the best");
            eprintln!("A property claimed to start before zero, at {}! Resetting it to zero, and hoping for the best", cpStart);
            // Java: this.cpStart = 0;
            cpStart = 0;
        }
        PropertyNode {
            _buf: buf,
            translator: None,
            cpStart,
            cpEnd,
        }
    }

    // Java: PropertyNode(int fcStart, int fcEnd, CharIndexTranslator translator, Object buf)
    pub fn new_with_translator(fcStart: i32, fcEnd: i32, translator: Box<dyn CharIndexTranslator>, buf: Box<dyn Any>) -> Self {
        // Java: super();
        // Java: this.cpStart = fcStart;
        let mut cpStart = fcStart;
        // Java: this.cpEnd = fcEnd;
        let cpEnd = fcEnd;
        // Java: this._buf = buf;
        // Java: if (this.cpStart < 0)
        if cpStart < 0 {
            // Java: System.err.println("A property claimed to start before zero, at " + this.cpStart + "! Resetting it to zero, and hoping for the best");
            eprintln!("A property claimed to start before zero, at {}! Resetting it to zero, and hoping for the best", cpStart);
            // Java: this.cpStart = 0;
            cpStart = 0;
        }
        // Java: this.translator = translator;
        PropertyNode {
            _buf: buf,
            translator: Some(translator),
            cpStart,
            cpEnd,
        }
    }

    // Java: void doGetStartAndEnd()
    fn doGetStartAndEnd(&mut self) {
        // Java: if (this.translator != null)
        if self.translator.is_some() {
            // Java: this.cpStart = this.translator.getCharIndex(this.cpStart, true);
            self.cpStart = self.translator.as_ref().unwrap().getCharIndex_with_flag(self.cpStart, true);
            // Java: this.cpEnd = this.translator.getCharIndex(this.cpEnd);
            self.cpEnd = self.translator.as_ref().unwrap().getCharIndex(self.cpEnd);
            // Java: this.translator = null;
            self.translator = None;
        }
    }

    // Java: int getStart()
    pub fn getStart(&mut self) -> i32 {
        // Java: this.doGetStartAndEnd();
        self.doGetStartAndEnd();
        // Java: return this.cpStart;
        self.cpStart
    }

    // Java: void setStart(int start)
    pub fn setStart(&mut self, start: i32) {
        // Java: this.translator = null;
        self.translator = None;
        // Java: this.cpStart = start;
        self.cpStart = start;
    }

    // Java: int getEnd()
    pub fn getEnd(&mut self) -> i32 {
        // Java: this.doGetStartAndEnd();
        self.doGetStartAndEnd();
        // Java: return this.cpEnd;
        self.cpEnd
    }

    // Java: void setEnd(int end)
    pub fn setEnd(&mut self, end: i32) {
        // Java: this.translator = null;
        self.translator = None;
        // Java: this.cpEnd = end;
        self.cpEnd = end;
    }

    // Java: void adjustForDelete(int start, int length)
    pub fn adjustForDelete(&mut self, start: i32, length: i32) {
        // Java: int end = start + length;
        let end = start + length;
        // Java: if (this.cpEnd > start)
        if self.cpEnd > start {
            // Java: if (this.cpStart < end)
            if self.cpStart < end {
                // Java: this.cpEnd = end >= this.cpEnd ? start : this.cpEnd - length;
                self.cpEnd = if end >= self.cpEnd { start } else { self.cpEnd - length };
                // Java: this.cpStart = Math.min(start, this.cpStart);
                self.cpStart = std::cmp::min(start, self.cpStart);
            }
            // Java: else
            else {
                // Java: this.cpEnd -= length;
                self.cpEnd -= length;
                // Java: this.cpStart -= length;
                self.cpStart -= length;
            }
        }
    }

    // Java: boolean limitsAreEqual(Object o)
    pub fn limitsAreEqual(&mut self, o: &mut PropertyNode) -> bool {
        // Java: return ((PropertyNode)o).getStart() == this.cpStart && ((PropertyNode)o).getEnd() == this.cpEnd;
        o.getStart() == self.cpStart && o.getEnd() == self.cpEnd
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: Option<&mut PropertyNode>) -> bool {
        // Java: if (o == null)
        if o.is_none() {
            // Java: return false;
            return false;
        }
        let other = o.unwrap();
        // Java: else if (this.limitsAreEqual(o))
        if self.limitsAreEqual(other) {
            // Java: Object testBuf = ((PropertyNode)o)._buf;
            // Java: return testBuf instanceof byte[] && this._buf instanceof byte[] ? Arrays.equals((byte[])testBuf, (byte[])this._buf) : this._buf.equals(testBuf);
            if let (Some(self_bytes), Some(other_bytes)) = (self._buf.downcast_ref::<Vec<u8>>(), other._buf.downcast_ref::<Vec<u8>>()) {
                return self_bytes == other_bytes;
            }
            return false;
        }
        // Java: else
        else {
            // Java: return false;
            return false;
        }
    }

    // Java: Object clone()
    // Java: return super.clone();

    // Java: int compareTo(Object o)
    pub fn compareTo(&mut self, o: &mut PropertyNode) -> i32 {
        // Java: int cpEnd = ((PropertyNode)o).getEnd();
        let other_cpEnd = o.getEnd();
        // Java: if (this.cpEnd == cpEnd)
        if self.cpEnd == other_cpEnd {
            // Java: return 0;
            return 0;
        }
        // Java: else
        else {
            // Java: return this.cpEnd < cpEnd ? -1 : 1;
            return if self.cpEnd < other_cpEnd { -1 } else { 1 };
        }
    }
}
