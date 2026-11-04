// Java: import java.util.Arrays

// Java: public class UPX
pub struct UPX {
    // Java field: byte[] _upx
    pub(crate) _upx: Vec<u8>,
}

// Java: public class UPX
impl UPX {
    // Java: UPX(byte[] upx)
    pub fn new(upx: Vec<u8>) -> Self {
        // Java: super();
        // Java: this._upx = upx;
        UPX { _upx: upx }
    }

    // Java: byte[] getUPX()
    pub fn getUPX(&self) -> &[u8] {
        // Java: return this._upx;
        &self._upx
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this._upx.length;
        self._upx.len() as i32
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return super.hashCode();
        let mut hash: i32 = 0;
        for &b in &self._upx {
            hash = hash.wrapping_mul(31).wrapping_add(b as i32);
        }
        hash
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&UPX>) -> bool {
        // Java: if (o == null)
        if o.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: UPX upx = (UPX)o;
            let upx = o.unwrap();
            // Java: return Arrays.equals(this._upx, upx._upx);
            self._upx == upx._upx
        }
    }
}
