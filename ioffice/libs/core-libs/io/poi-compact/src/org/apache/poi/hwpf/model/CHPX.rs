// Java: import org.apache.poi.hwpf.sprm.SprmBuffer
use crate::org::apache::poi::hwpf::sprm::SprmBuffer::SprmBuffer;
use super::BytePropertyNode::BytePropertyNode;
use super::CharIndexTranslator::CharIndexTranslator;

// Java: public class CHPX extends BytePropertyNode
pub struct CHPX {
    pub(crate) base: BytePropertyNode,
}

// Java: public class CHPX extends BytePropertyNode
impl CHPX {
    // Java: CHPX(int fcStart, int fcEnd, CharIndexTranslator translator, byte[] grpprl)
    pub fn new(fcStart: i32, fcEnd: i32, translator: Box<dyn CharIndexTranslator>, grpprl: Vec<u8>) -> Self {
        // Java: super(fcStart, translator.lookIndexBackward(fcEnd), translator, grpprl);
        let adjusted_end = translator.lookIndexBackward(fcEnd);
        let base = BytePropertyNode::new(fcStart, adjusted_end, translator, Box::new(grpprl));
        CHPX { base }
    }

    // Java: CHPX(int fcStart, int fcEnd, CharIndexTranslator translator, SprmBuffer buf)
    pub fn new_from_sprm(fcStart: i32, fcEnd: i32, translator: Box<dyn CharIndexTranslator>, buf: &SprmBuffer) -> Self {
        // Java: super(fcStart, translator.lookIndexBackward(fcEnd), translator, buf.toByteArray());
        let adjusted_end = translator.lookIndexBackward(fcEnd);
        let bytes = buf._buf.clone();
        let base = BytePropertyNode::new(fcStart, adjusted_end, translator, Box::new(bytes));
        CHPX { base }
    }

    pub fn new_with_grpprl(start: i32, end: i32, grpprl: Vec<u8>) -> Self {
        let base = BytePropertyNode::new_simple(start, end, Box::new(grpprl));
        CHPX { base }
    }

    // Java: byte[] getGrpprl()
    pub fn getGrpprl(&self) -> &[u8] {
        // Java: return (byte[])this._buf;
        self.base.base._buf.downcast_ref::<Vec<u8>>().map(|v| v.as_slice()).unwrap_or(&[])
    }

    // Java: SprmBuffer getSprmBuf()
    pub fn getSprmBuf(&self) -> SprmBuffer {
        // Java: return new SprmBuffer((byte[])this._buf);
        let bytes = self.getGrpprl();
        SprmBuffer { _buf: bytes.to_vec() }
    }

    // Java: String toString()
    pub fn toString(&mut self) -> String {
        // Java: return "CHPX from " + this.getStart() + " to " + this.getEnd() + " (in bytes " + this.getStartBytes() + " to " + this.getEndBytes() + ")";
        let start = self.getStart();
        let end = self.getEnd();
        let sb = self.base.getStartBytes();
        let eb = self.base.getEndBytes();
        format!("CHPX from {} to {} (in bytes {} to {})", start, end, sb, eb)
    }

    // Java: int getStart()
    pub fn getStart(&mut self) -> i32 {
        // Java: if (this.translator != null)
        if self.base.base.translator.is_some() {
            // Java: this.cpStart = this.translator.getCharIndexForNormal(this.cpStart, true);
            self.base.base.cpStart = self.base.base.translator.as_ref().unwrap().getCharIndexForNormal_with_flag(self.base.base.cpStart, true);
            // Java: this.cpEnd = this.translator.getCharIndexForNormal(this.cpEnd);
            self.base.base.cpEnd = self.base.base.translator.as_ref().unwrap().getCharIndexForNormal(self.base.base.cpEnd);
            // Java: this.translator = null;
            self.base.base.translator = None;
        }
        // Java: return this.cpStart;
        self.base.base.cpStart
    }

    // Java: int getEnd()
    pub fn getEnd(&mut self) -> i32 {
        // Java: if (this.translator != null)
        if self.base.base.translator.is_some() {
            // Java: this.cpStart = this.translator.getCharIndexForNormal(this.cpStart, true);
            self.base.base.cpStart = self.base.base.translator.as_ref().unwrap().getCharIndexForNormal_with_flag(self.base.base.cpStart, true);
            // Java: this.cpEnd = this.translator.getCharIndexForNormal(this.cpEnd);
            self.base.base.cpEnd = self.base.base.translator.as_ref().unwrap().getCharIndexForNormal(self.base.base.cpEnd);
            // Java: this.translator = null;
            self.base.base.translator = None;
        }
        // Java: return this.cpEnd;
        self.base.base.cpEnd
    }

    pub fn setStart(&mut self, start: i32) {
        self.base.base.setStart(start);
    }

    pub fn setEnd(&mut self, end: i32) {
        self.base.base.setEnd(end);
    }

    pub fn getStartBytes(&self) -> i32 {
        self.base.getStartBytes()
    }

    pub fn getEndBytes(&self) -> i32 {
        self.base.getEndBytes()
    }
}
