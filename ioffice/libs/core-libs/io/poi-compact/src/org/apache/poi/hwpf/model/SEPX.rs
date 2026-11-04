// Java: import org.apache.poi.hwpf.sprm.SprmBuffer
use crate::org::apache::poi::hwpf::sprm::SprmBuffer::SprmBuffer;
use super::BytePropertyNode::BytePropertyNode;
use super::SectionDescriptor::SectionDescriptor;
use super::CharIndexTranslator::CharIndexTranslator;

// Java: public class SEPX extends BytePropertyNode
pub struct SEPX {
    pub(crate) base: BytePropertyNode,
    // Java field: SectionDescriptor _sed
    pub(crate) _sed: SectionDescriptor,
}

// Java: public class SEPX extends BytePropertyNode
impl SEPX {
    // Java: SEPX(SectionDescriptor sed, int start, int end, CharIndexTranslator translator, byte[] grpprl)
    pub fn new(sed: SectionDescriptor, start: i32, end: i32, translator: Box<dyn CharIndexTranslator>, grpprl: Vec<u8>) -> Self {
        // Java: super(start, end, translator, grpprl);
        let base = BytePropertyNode::new(start, end, translator, Box::new(grpprl));
        // Java: this._sed = sed;
        SEPX { base, _sed: sed }
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

    // Java: SectionDescriptor getSectionDescriptor()
    pub fn getSectionDescriptor(&self) -> &SectionDescriptor {
        // Java: return this._sed;
        &self._sed
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return super.hashCode();
        0
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: Option<&mut SEPX>) -> bool {
        // Java: if (o == null)
        if o.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: SEPX sepx = (SEPX)o;
            let sepx = o.unwrap();
            // Java: return super.equals(o) ? sepx._sed.equals(this._sed) : false;
            if self.base.base.limitsAreEqual(&mut sepx.base.base) {
                return sepx._sed.equals(Some(&self._sed));
            }
            return false;
        }
    }

    // Java: int getEnd()
    pub fn getEnd(&mut self) -> i32 {
        // Java: return super.getEnd();
        self.base.base.getEnd()
    }
}
