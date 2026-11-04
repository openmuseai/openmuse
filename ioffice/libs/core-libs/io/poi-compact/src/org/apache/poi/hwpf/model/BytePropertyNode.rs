use std::any::Any;
use super::PropertyNode::PropertyNode;
use super::CharIndexTranslator::CharIndexTranslator;

// Java: public class BytePropertyNode extends PropertyNode
pub struct BytePropertyNode {
    pub(crate) base: PropertyNode,
    // Java field: int startBytes
    pub(crate) startBytes: i32,
    // Java field: int endBytes
    pub(crate) endBytes: i32,
}

// Java: public class BytePropertyNode extends PropertyNode
impl BytePropertyNode {
    // Java: BytePropertyNode(int fcStart, int fcEnd, CharIndexTranslator translator, Object buf)
    pub fn new(fcStart: i32, fcEnd: i32, translator: Box<dyn CharIndexTranslator>, buf: Box<dyn Any>) -> Self {
        // Java: super(fcStart, fcEnd, translator, buf);
        let base = PropertyNode::new_with_translator(fcStart, fcEnd, translator, buf);
        // Java: this.startBytes = fcStart;
        let startBytes = fcStart;
        // Java: this.endBytes = fcEnd;
        let endBytes = fcEnd;
        BytePropertyNode { base, startBytes, endBytes }
    }

    pub fn new_simple(fcStart: i32, fcEnd: i32, buf: Box<dyn Any>) -> Self {
        let base = PropertyNode::new(fcStart, fcEnd, buf);
        BytePropertyNode { base, startBytes: fcStart, endBytes: fcEnd }
    }

    // Java: int getStartBytes()
    pub fn getStartBytes(&self) -> i32 {
        // Java: return this.startBytes;
        self.startBytes
    }

    // Java: int getEndBytes()
    pub fn getEndBytes(&self) -> i32 {
        // Java: return this.endBytes;
        self.endBytes
    }
}
